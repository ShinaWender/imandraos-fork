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

#[derive(Clone, Copy)]
pub struct Message {
    pub sender_task_id: u64,
    pub receiver_task_id: u64,
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
}

pub fn send(sender_task_id: u64, receiver_task_id: u64, data: &[u8]) -> Result<(), ()> {
    Ok(())
}

pub fn receive(receiver_task_id: u64) -> Message {
    Message::new()
}
