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

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    static ref HEAP_BEGIN_ADDR: Mutex<u64> = Mutex::new(0);
    static ref HEAP_SIZE: Mutex<usize> = Mutex::new(0);
    static ref FRAME_COUNTER: Mutex<usize> = Mutex::new(0);
}

const FRAME_SIZE: usize = 4096;

pub fn init(heap_begin_addr: u64, heap_size: usize) {
    *HEAP_BEGIN_ADDR.lock() = heap_begin_addr;
    *HEAP_SIZE.lock() = heap_size;
    *FRAME_COUNTER.lock() = 0;
}

pub fn alloc(frame_count: usize) -> u64 {
    let addr = *HEAP_BEGIN_ADDR.lock()
        + (*FRAME_COUNTER.lock() * FRAME_SIZE) as u64
        + (frame_count * FRAME_SIZE) as u64;
    *FRAME_COUNTER.lock() += frame_count;

    addr
}

pub fn dealloc(frame_addr: u64, frame_count: usize) {}
