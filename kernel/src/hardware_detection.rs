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

use crate::ns16550a::Ns16550a;

pub struct HardwareDetector {
    pub cpu_count: usize,
    pub ram_begin: u64,
    pub ram_size: usize,
    pub ns16550a: Option<Ns16550a>,
}

impl HardwareDetector {
    fn get_ns16550a(device_tree: fdt::Fdt) -> Ns16550a {
        let node_name = device_tree
            .chosen()
            .stdout()
            .expect("chosen node not found")
            .name;

        let str_begin = node_name
            .as_bytes()
            .iter()
            .position(|&ch| ch == b'@')
            .expect("invalid node name")
            + 1;
        let str_end = node_name.len();

        let addr_str = &node_name[str_begin..str_end];
        let addr = usize::from_str_radix(addr_str, 16).expect("from_str_radix() failed");

        let uart = Ns16550a::new(addr);
        uart
    }

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
            ns16550a: Option::from(Self::get_ns16550a(fdt)),
        }
    }
}
