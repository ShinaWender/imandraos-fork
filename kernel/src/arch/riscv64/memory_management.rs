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

use crate::arch::set_satp;
use crate::arch::sfence_vma;
use crate::frame_allocator;
use crate::memory_management::{
    PAGE_EXEC_FLAG, PAGE_READ_FLAG, PAGE_USER_FLAG, PAGE_WRITE_FLAG, PagingInterface,
};

pub struct Paging {
    page_table: u64,
}

const PTE_VALID_FLAG: u64 = 0b1;
const PTE_READ_FLAG: u64 = 0b10;
const PTE_WRITE_FLAG: u64 = 0b100;
const PTE_EXEC_FLAG: u64 = 0b1000;
const PTE_USER_FLAG: u64 = 0b10000;
const PTE_DIRTY_FLAG: u64 = 0b1000000;
const PTE_ACCESS_FLAG: u64 = 0b10000000;

const PAGE_SIZE_L0: usize = 0x1000;
const PAGE_SIZE_L1: usize = 0x20_0000;
const PAGE_SIZE_L2: usize = 0x4000_0000;

struct Pte {
    value: u64,
}

impl Pte {
    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn is_valid(&self) -> bool {
        (self.value & PTE_VALID_FLAG) != 0
    }

    pub fn set(&mut self, ppn: u64, flags: u64) {
        self.value = ((ppn >> 12) << 10) | flags;
    }

    pub fn get_ppn(&self) -> u64 {
        (self.value >> 10) << 12
    }
}

struct Page {
    data: [Pte; 4096 / 8],
}

impl PagingInterface for Paging {
    fn new() -> Self {
        let page_table_addr = frame_allocator::alloc(1);
        let page_table = unsafe { &mut *(page_table_addr as *mut Page) };
        page_table.data.iter_mut().for_each(|pte| pte.value = 0);

        Self {
            page_table: page_table_addr,
        }
    }

    fn from_page_table(page_table_addr: u64) -> Self {
        Self {
            page_table: page_table_addr,
        }
    }

    fn empty() -> Self {
        Self { page_table: 0 }
    }

    fn map(
        &self,
        physical_address: u64,
        virtual_address: u64,
        level: u32,
        flags: u8,
    ) -> Result<(), ()> {
        let vpn: [u64; 3] = [
            (virtual_address >> 12) & 0x1ff,
            (virtual_address >> 21) & 0x1ff,
            (virtual_address >> 30) & 0x1ff,
        ];

        let mut page_table = unsafe { &mut *(self.page_table as *mut Page) };

        for i in (level as usize..=2).rev() {
            if i == level as usize {
                let mut pte_flags = PTE_VALID_FLAG;

                if flags & PAGE_EXEC_FLAG != 0 {
                    pte_flags |= PTE_EXEC_FLAG;
                }
                if flags & PAGE_READ_FLAG != 0 {
                    pte_flags |= PTE_READ_FLAG;
                }
                if flags & PAGE_WRITE_FLAG != 0 {
                    pte_flags |= PTE_WRITE_FLAG;
                }
                if flags & PAGE_USER_FLAG != 0 {
                    pte_flags |= PTE_USER_FLAG;
                }

                page_table.data[vpn[i] as usize].set(physical_address, pte_flags);
            } else {
                if !page_table.data[vpn[i] as usize].is_valid() {
                    let new_page_table_addr = frame_allocator::alloc(1);
                    page_table.data[vpn[i] as usize].set(new_page_table_addr, PTE_VALID_FLAG);

                    page_table = unsafe { &mut *(new_page_table_addr as *mut Page) };
                    page_table.data.iter_mut().for_each(|pte| pte.value = 0);
                } else {
                    let new_page_table_addr = page_table.data[vpn[i] as usize].get_ppn();
                    page_table = unsafe { &mut *(new_page_table_addr as *mut Page) };
                }
            }
        }

        Ok(())
    }

    fn unmap(&self, virtual_address: u64, level: u32) -> Result<(), ()> {
        Ok(())
    }

    fn map_region(
        &self,
        physical_address: u64,
        virtual_address: u64,
        region_size: usize,
        flags: u8,
    ) -> Result<(), ()> {
        let pages_to_map_count =
            region_size / PAGE_SIZE_L0 + (region_size % PAGE_SIZE_L0 != 0) as usize;

        (0..pages_to_map_count).for_each(|page_num| {
            self.map(
                physical_address + page_num as u64 * PAGE_SIZE_L0 as u64,
                virtual_address + page_num as u64 * PAGE_SIZE_L0 as u64,
                0,
                flags,
            )
            .unwrap()
        });

        Ok(())
    }

    fn unmap_region(&self, virtual_address: u64, region_size: usize) -> Result<(), ()> {
        Ok(())
    }

    fn enable(&self) {
        unsafe {
            set_satp(self.page_table >> 12 | (0x8 << 60));
        }
    }

    fn update(&self) {
        unsafe {
            sfence_vma();
        }
    }
}
