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

use core::arch::naked_asm;

pub const SUCCESS: i64 = 0;
pub const FAILED: i64 = -1;
pub const NOT_SUPPORTED: i64 = -2;
pub const INVALID_PARAMETER: i64 = -3;
pub const DENIED: i64 = -4;
pub const INVALID_ADDRESS: i64 = -5;
pub const ALREADY_AVAILABLE: i64 = -6;

#[repr(C)]
pub struct Sbiret {
    pub error: i64,
    pub value: i64,
}

#[unsafe(naked)]
extern "C" fn sbi_call(
    a0: u64,
    a1: u64,
    a2: u64,
    a3: u64,
    a4: u64,
    a5: u64,
    a6: u64,
    a7: u64,
) -> Sbiret {
    naked_asm!(
        "
        ecall
        ret
        "
    );
}

pub fn hart_start(hart_id: u64, start_addr: u64, privilege: u64) -> Sbiret {
    sbi_call(hart_id, start_addr, privilege, 0, 0, 0, 0, 0x48534d)
}

pub fn set_timer(time_value: u64) -> Sbiret {
    sbi_call(time_value, 0, 0, 0, 0, 0, 0, 0x5449_4d45)
}

pub fn put_char(ch: u8) -> Sbiret {
    sbi_call(ch as u64, 0, 0, 0, 0, 0, 0, 1)
}
