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

use crate::frame_allocator;

#[derive(Clone, Copy)]
pub struct Message {
    pub sender_task_id: u32,
    pub receiver_task_id: u32,
    pub data_addr: u64,
}

impl Message {
    pub fn new() -> Self {
        Self {
            sender_task_id: 0,
            receiver_task_id: 0,
            data_addr: 0,
        }
    }
}

lazy_static! {
    static ref MESSAGES: Mutex<[Message; 32]> = Mutex::new([Message::new(); 32]);
    static ref FIRST_MESSAGE_ID: Mutex<usize> = Mutex::new(32);
}

pub fn send(sender_task_id: u32, receiver_task_id: u32, data: &[u8]) -> Result<(), ()> {
    if data.len() > 4096 {
        return Err(());
    }

    *FIRST_MESSAGE_ID.lock() -= 1;
    let mut message = &mut MESSAGES.lock()[*FIRST_MESSAGE_ID.lock()];

    message.sender_task_id = sender_task_id;
    message.receiver_task_id = receiver_task_id;
    message.data_addr = frame_allocator::alloc(1);

    let message_data = unsafe {
        &mut *core::ptr::slice_from_raw_parts_mut(message.data_addr as *mut u8, data.len())
    };

    message_data.clone_from_slice(data);

    Ok(())
}

pub fn receive(receiver_task_id: u32, data: &mut [u8]) -> Result<(), ()> {
    if *FIRST_MESSAGE_ID.lock() >= 32 {
        return Err(());
    }

    let message = &MESSAGES.lock()[*FIRST_MESSAGE_ID.lock()];

    if message.receiver_task_id == receiver_task_id {
        let message_data = unsafe {
            &*core::ptr::slice_from_raw_parts(message.data_addr as *const u8, data.len())
        };

        data.clone_from_slice(message_data);

        frame_allocator::dealloc(message.data_addr, 1);

        *FIRST_MESSAGE_ID.lock() += 1;
    }

    Ok(())
}
