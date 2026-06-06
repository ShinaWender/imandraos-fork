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

use crate::ipc;
use crate::scheduler;

pub const SYSCALL_EXIT: u64 = 0;
pub const SYSCALL_GTID: u64 = 1;
pub const SYSCALL_SEND: u64 = 2;
pub const SYSCALL_RECV: u64 = 3;

#[unsafe(no_mangle)]
extern "C" fn syscall_handler(syscall: u64, a0: u64, a1: u64) -> u64 {
    match syscall {
        SYSCALL_EXIT => {
            let task_id = scheduler::get_current_task_id();

            scheduler::delete_task(task_id);

            0
        }
        SYSCALL_GTID => scheduler::get_current_task_id() as u64,
        SYSCALL_SEND => {
            let sender_task_id = scheduler::get_current_task_id();
            let receiver_task_id = a0 as u32;
            let data = unsafe { &*core::ptr::slice_from_raw_parts(a1 as *const u8, 4096) };

            match ipc::send(sender_task_id, receiver_task_id, data) {
                Ok(()) => 0,
                Err(()) => 1,
            }
        }
        SYSCALL_RECV => {
            let receiver_task_id = scheduler::get_current_task_id();
            let data = unsafe { &mut *core::ptr::slice_from_raw_parts_mut(a0 as *mut u8, 4096) };

            match ipc::receive(receiver_task_id, data) {
                Ok(()) => 0,
                Err(()) => 1,
            }
        }
        _ => 0,
    }
}
