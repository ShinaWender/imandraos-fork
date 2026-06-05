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

pub struct HardwareDetector {
    pub cpu_count: usize,
    pub ram_begin: u64,
    pub ram_size: usize,
}

impl HardwareDetector {
    pub fn new(device_tree_blob: *const u8) -> Self {
        let fdt = unsafe { fdt::Fdt::from_ptr(device_tree_blob).expect("invalid fdt") };

        let ram = fdt
            .memory()
            .regions()
            .find(|memory_region| memory_region.starting_address as usize == 0x8000_0000)
            .expect("ram not found");

        let ram_size = ram.size.expect("failed to get ram size");

        Self {
            cpu_count: fdt.cpus().count(),
            ram_begin: 0x8000_0000,
            ram_size: ram_size,
        }
    }
}
