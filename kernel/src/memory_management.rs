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

pub const PAGE_READ_FLAG: u8 = 0b0001;
pub const PAGE_WRITE_FLAG: u8 = 0b0010;
pub const PAGE_EXEC_FLAG: u8 = 0b0100;
pub const PAGE_USER_FLAG: u8 = 0b1000;

pub trait PagingInterface {
    fn new() -> Self;

    fn from_page_table(page_table_addr: u64) -> Self;

    fn empty() -> Self;

    fn map(
        &self,
        physical_address: u64,
        virtual_address: u64,
        level: u32,
        flags: u8,
    ) -> Result<(), ()>;

    fn unmap(&self, virtual_address: u64, level: u32) -> Result<(), ()>;

    fn map_region(
        &self,
        physical_address: u64,
        virtual_address: u64,
        region_size: usize,
        flags: u8,
    ) -> Result<(), ()>;

    fn unmap_region(&self, virtual_address: u64, region_size: usize) -> Result<(), ()>;

    fn enable(&self);

    fn update(&self);
}
