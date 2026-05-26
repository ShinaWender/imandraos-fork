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

use crate::arch::opensbi;

pub struct Smp;

impl crate::smp::SmpInterface for Smp {
    fn new() -> Self {
        Self {}
    }

    fn start_cpu(&self, cpu_id: u64) -> Result<(), &str> {
        let result = opensbi::hart_start(cpu_id, 0x8020_0000, 1);

        match result.error {
            opensbi::SUCCESS => return Ok(()),
            opensbi::FAILED => return Err("opensbi error: failed"),
            opensbi::NOT_SUPPORTED => return Err("opensbi error: not supported"),
            opensbi::INVALID_PARAMETER => return Err("opensbi error: invalid parameter"),
            opensbi::DENIED => return Err("opensbi error: denied"),
            opensbi::INVALID_ADDRESS => return Err("opensbi error: invalid address"),
            opensbi::ALREADY_AVAILABLE => return Err("opensbi error: already available"),
            _ => return Err("opensbi error: unknown error code"),
        }
    }
}
