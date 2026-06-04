/*
    ImandraOS the microkernel-based operating system
    Copyright (C) 2026  Yuriy Alekseyevich Zhelyazko

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use crate::{
    arch::{
        HEAP_BEGIN, TEXT_BEGIN, defs, jump_to_user_space, memory_management::Paging, timer::Timer,
    },
    frame_allocator,
    memory_management::{
        PAGE_EXEC_FLAG, PAGE_READ_FLAG, PAGE_USER_FLAG, PAGE_WRITE_FLAG, PagingInterface,
    },
    timer::TimerInterface,
};
use lazy_static::lazy_static;
use spin::Mutex;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    None = 0,
    Runnable = 1,
}

#[derive(Clone, Copy)]
pub struct Task {
    pub id: u32,
    pub parent_task_id: u32,
    pub status: TaskStatus,
    pub pc: u64,
    pub prog_addr: u64,
    pub prog_size: usize,
    pub kernel_stack: u64,
    pub page_table: u64,
}

impl Task {
    pub fn new() -> Self {
        Self {
            id: 0,
            parent_task_id: 0,
            status: TaskStatus::None,
            pc: 0,
            prog_addr: 0,
            prog_size: 0,
            kernel_stack: 0,
            page_table: 0,
        }
    }
}

const MAX_TASK_COUNT: usize = 32;

lazy_static! {
    pub static ref TASKS: Mutex<[Task; MAX_TASK_COUNT]> = Mutex::new([Task::new(); MAX_TASK_COUNT]);
    pub static ref CURRENT_TASK_ID: Mutex<u32> = Mutex::new(0);
}

pub fn init() {
    (0..MAX_TASK_COUNT).for_each(|task_id| {
        TASKS.lock()[task_id].id = task_id as u32;
        TASKS.lock()[task_id].status = TaskStatus::None;
    });
}

pub fn add_task(prog: &[u8]) {
    let task_id = TASKS
        .lock()
        .iter()
        .find(|task| task.status == TaskStatus::None)
        .expect("Not founded free task id")
        .id;

    let task = &mut TASKS.lock()[task_id as usize];

    let prog_size_in_pages =
        prog.len() / defs::PAGE_SIZE + (prog.len() % defs::PAGE_SIZE > 0) as usize + 200;

    task.parent_task_id = 0;
    task.status = TaskStatus::Runnable;
    task.pc = defs::PROGRAM_BEGIN;
    task.prog_addr = frame_allocator::alloc(prog_size_in_pages);
    task.prog_size = prog_size_in_pages * defs::PAGE_SIZE;
    task.kernel_stack = frame_allocator::alloc(1);
    task.page_table = frame_allocator::alloc(1);

    let prog_mem =
        unsafe { core::slice::from_raw_parts_mut(task.prog_addr as *mut u8, prog.len()) };
    prog_mem.copy_from_slice(prog);

    let page_table =
        unsafe { core::slice::from_raw_parts_mut(task.page_table as *mut u32, defs::PAGE_SIZE) };
    page_table.fill(0);
    let task_paging = Paging::from_page_table(task.page_table);

    task_paging
        .map(
            0x8000_0000,
            0x8000_0000,
            2,
            PAGE_READ_FLAG | PAGE_WRITE_FLAG | PAGE_EXEC_FLAG,
        )
        .expect("Kernel mapping error");

    task_paging
        .map(
            0x1000_0000,
            defs::GENERIC_UART_BASE,
            0,
            PAGE_READ_FLAG | PAGE_WRITE_FLAG | PAGE_USER_FLAG,
        )
        .expect("UART mapping error");

    task_paging
        .map(
            task.kernel_stack,
            defs::KSTACK_BEGIN,
            0,
            PAGE_READ_FLAG | PAGE_WRITE_FLAG,
        )
        .expect("Kernel stack mapping error");

    task_paging
        .map_region(
            task.prog_addr,
            task.pc,
            task.prog_size,
            PAGE_READ_FLAG | PAGE_WRITE_FLAG | PAGE_EXEC_FLAG | PAGE_USER_FLAG,
        )
        .expect("User program mapping error");
}

pub fn delete_task(task_id: u32) {
    let mut tasks = TASKS.lock();

    if tasks[task_id as usize].status != TaskStatus::Runnable {
        return;
    }

    tasks[task_id as usize].status = TaskStatus::None;

    let task_paging = Paging::from_page_table(tasks[task_id as usize].page_table);

    unsafe {
        task_paging
            .unmap_region(TEXT_BEGIN, HEAP_BEGIN as usize - TEXT_BEGIN as usize)
            .expect("Kernel unmapping error");
    }
    task_paging
        .unmap(defs::GENERIC_UART_BASE, 0)
        .expect("UART unmapping error");
    task_paging
        .unmap(defs::KSTACK_BEGIN, 0)
        .expect("Kernel stack unmapping error");
    task_paging
        .unmap_region(defs::PROGRAM_BEGIN, tasks[task_id as usize].prog_size)
        .expect("User program unmapping error");

    let prog_len = tasks[task_id as usize].prog_size;
    let prog_size_in_pages = prog_len / defs::PAGE_SIZE + (prog_len % defs::PAGE_SIZE > 0) as usize;

    frame_allocator::dealloc(tasks[task_id as usize].prog_addr, prog_size_in_pages);
    frame_allocator::dealloc(tasks[task_id as usize].kernel_stack, 1);
    frame_allocator::dealloc(tasks[task_id as usize].page_table, 1);
}

pub fn switch() -> ! {
    let pc = {
        let tasks = TASKS.lock();

        let mut next_task_id = *CURRENT_TASK_ID.lock() + 1;

        while tasks[next_task_id as usize].status != TaskStatus::Runnable {
            next_task_id += 1;
            if next_task_id == MAX_TASK_COUNT as u32 {
                next_task_id = 0;
            }
        }

        let task = &tasks[next_task_id as usize];
        *CURRENT_TASK_ID.lock() = next_task_id;

        let paging = Paging::from_page_table(task.page_table);
        paging.enable();

        task.pc
    };

    Timer::set_timer(1000000).expect("Timer error");

    unsafe {
        jump_to_user_space(pc);
    }
}

pub fn update_pc_for_current_task(new_program_counter: u64) {
    TASKS.lock()[*CURRENT_TASK_ID.lock() as usize].pc = new_program_counter;
}

pub fn get_current_task_id() -> u32 {
    *CURRENT_TASK_ID.lock()
}
