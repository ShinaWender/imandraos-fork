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

#![allow(dead_code)]
#![allow(unused_variables)]

pub const RHR: u8 = 0b000;
pub const THR: u8 = 0b000;
pub const IER: u8 = 0b001;
pub const ISR: u8 = 0b010;
pub const FCR: u8 = 0b010;
pub const LCR: u8 = 0b011;
pub const MCR: u8 = 0b100;
pub const LSR: u8 = 0b101;
pub const MSR: u8 = 0b110;
pub const SPR: u8 = 0b111;
pub const DLL: u8 = 0b000;
pub const DLM: u8 = 0b001;
pub const PSD: u8 = 0b101;

pub const RECEIVER_LINE_STATUS_INTR: u8 = 0b0110;
pub const RECEIVED_DATA_READY_INTR: u8 = 0b0100;
pub const RECEPTION_TIMEOUT_INTR: u8 = 0b1100;
pub const THR_EMPTY_INTR: u8 = 0b0010;
pub const MODEM_STATUS_INTR: u8 = 0b0000;
pub const DMA_RECEPTION_END_INTR: u8 = 0b1110;
pub const DMA_TRANSMISSION_END_INTR: u8 = 0b1010;

pub const LSR_FIFO_ERROR: u8 = 0b10000000;
pub const LSR_TRANSMITTER_EMPTY: u8 = 0b01000000;
pub const LSR_THR_EMPTY: u8 = 0b00100000;
pub const LSR_BREAK_INTERRUPT: u8 = 0b00010000;
pub const LSR_FRAMING_ERROR: u8 = 0b00001000;
pub const LSR_PARITY_ERROR: u8 = 0b00000100;
pub const LSR_OVERRUN_ERROR: u8 = 0b00000010;
pub const LSR_DATA_READY: u8 = 0b00000001;

use core::ptr;

pub struct Ns16550a {
    uart_base: usize,
}

impl Ns16550a {
    pub fn new(uart_base: usize) -> Self {
        Self {
            uart_base: uart_base,
        }
    }

    pub fn init(&self) {
        self.write_register(IER, 0);
        self.write_register(LCR, 0b10000000);
        self.write_register(DLL, 0x3);
        self.write_register(DLM, 0x0);
        self.write_register(LCR, 0b00000011);
        self.write_register(FCR, 0b00000110);
    }

    pub fn write_register(&self, register: u8, value: u8) {
        let uart =
            unsafe { &mut *(ptr::null_mut::<u8>().with_addr(self.uart_base + register as usize)) };
        *uart = value;
    }

    pub fn read_register(&self, register: u8) -> u8 {
        let uart = unsafe { &*(ptr::null::<u8>().with_addr(self.uart_base + register as usize)) };
        *uart
    }

    pub fn write_byte_slice(&self, register: u8, bytes: &[u8]) {
        bytes
            .iter()
            .for_each(|&byte| self.write_register(register, byte));
    }

    pub fn read_byte_slice(&self, register: u8, bytes: &mut [u8]) {
        bytes
            .iter_mut()
            .for_each(|byte| *byte = self.read_register(register));
    }
}

use core::fmt;

impl fmt::Write for Ns16550a {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for &byte in s.as_bytes() {
            if byte == b'\n' {
                self.write_register(THR, b'\n');
                self.write_register(THR, b'\r');
            } else {
                self.write_register(THR, byte);
            }
        }

        Ok(())
    }
}
