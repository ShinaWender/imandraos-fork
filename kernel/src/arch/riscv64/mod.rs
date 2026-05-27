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

use core::arch::global_asm;

pub mod memory_management;
pub mod opensbi;
pub mod smp;
pub mod timer;

global_asm!(include_str!("asm/init.asm"));
global_asm!(include_str!("asm/trap.asm"));
global_asm!(include_str!("asm/constants.asm"));
global_asm!(include_str!("asm/satp.asm"));
global_asm!(include_str!("asm/sfence_vma.asm"));
global_asm!(include_str!("asm/jump_to_user_space.asm"));
global_asm!(include_str!("asm/rdtime.asm"));
global_asm!(include_str!("asm/init_timer.asm"));

unsafe extern "C" {
    pub static TEXT_BEGIN: u64;
    pub static TEXT_END: u64;

    pub static RODATA_BEGIN: u64;
    pub static RODATA_END: u64;

    pub static DATA_BEGIN: u64;
    pub static DATA_END: u64;

    pub static BSS_BEGIN: u64;
    pub static BSS_END: u64;

    pub static HEAP_BEGIN: u64;

    pub fn set_satp(val: u64);
    pub fn sfence_vma();
    pub fn jump_to_user_space(program_counter: u64) -> !;
    pub fn rdtime() -> u64;
    pub fn init_timer();
}

use crate::PAGING;
use crate::memory_management::PagingInterface;
use crate::scheduler;

#[unsafe(no_mangle)]
extern "C" fn _switch(program_counter: u64) -> ! {
    PAGING.lock().enable();
    scheduler::update_pc_for_current_task(program_counter + 4);
    scheduler::switch();
}
