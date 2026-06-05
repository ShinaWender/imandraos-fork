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

#![no_main]
#![no_std]
#![no_builtins]

mod arch;
mod console;
mod frame_allocator;
mod hardware_detection;
mod ipc;
mod memory_management;
mod scheduler;
mod smp;
mod syscall;
mod timer;

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use smp::SmpInterface;
use spin::Mutex;

use crate::{
    arch::{memory_management::Paging, smp::Smp, timer::Timer},
    console::Console,
    memory_management::{PAGE_EXEC_FLAG, PAGE_READ_FLAG, PAGE_WRITE_FLAG, PagingInterface},
    timer::TimerInterface,
};

lazy_static! {
    pub static ref CONSOLE: Mutex<Console> = Mutex::new(Console {});
    pub static ref PAGING: Mutex<Paging> = Mutex::new(Paging::from_page_table(0));
}

macro_rules! print {
    ($($arg:tt)*) => ({
        use core::fmt::Write;
        let mut console = $crate::CONSOLE.lock();
        console.write_fmt(format_args!($($arg)*)).expect("IO error");
    });
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

#[inline(never)]
#[panic_handler]
fn panic(panic_info: &PanicInfo) -> ! {
    println!("{}", panic_info);
    loop {}
}

#[unsafe(no_mangle)]
extern "C" fn main(is_main_cpu: bool, cpu_id: usize, device_tree_blob: *mut u8) -> ! {
    if is_main_cpu {
        let harddet = hardware_detection::HardwareDetector::new(device_tree_blob);

        *CONSOLE.lock() = Console::new();

        println!("RAM starts at 0x{:x}", harddet.ram_begin);
        println!(
            "Available {} bytes or {}M",
            harddet.ram_size,
            harddet.ram_size / 1024 / 1024
        );

        frame_allocator::init(
            unsafe { arch::HEAP_BEGIN },
            harddet.ram_size - (unsafe { arch::HEAP_BEGIN as usize } - harddet.ram_begin as usize),
        );
        println!("Frame allocator initialized");

        *PAGING.lock() = Paging::new();
        {
            let paging = PAGING.lock();

            paging
                .map(
                    0x8000_0000,
                    0x8000_0000,
                    2,
                    PAGE_EXEC_FLAG | PAGE_READ_FLAG | PAGE_WRITE_FLAG,
                )
                .expect("Kernel mapping error");

            paging.enable();
        }
        println!("Virtual memory initialized");

        Timer::init();
        println!("Timer initialized");

        println!("Pryvitanne svet!");

        Smp::init();
        (0..harddet.cpu_count).for_each(|cpu_id_to_run| {
            if cpu_id_to_run != cpu_id {
                Smp::start_cpu(cpu_id_to_run as u64).unwrap();
            }
        });

        println!("cpu{} enabled", cpu_id);

        scheduler::init();
        scheduler::add_task(include_bytes!("../../ipc_test1/app"));
        scheduler::add_task(include_bytes!("../../ipc_test2/app"));
        scheduler::switch();
    } else {
        PAGING.lock().enable();

        println!("cpu{} enabled", cpu_id);
    }

    loop {}
}
