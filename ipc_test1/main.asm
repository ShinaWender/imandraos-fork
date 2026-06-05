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

.section .data

string: .ascii "Hello, world! If you see this message than IPC works!\0"

.section .bss

.align 16
MESSAGE_BUFFER:
.rep 4096
.byte 0
.endr

.section .text

.global start
start:
        li a0, 1
        ecall
        mv t6, a0

        la a0, string
        la a1, MESSAGE_BUFFER
        
next:
        lb t0, 0(a0)
        sb t0, 0(a1)
        
        addi a0, a0, 1
        addi a1, a1, 1
        
        beqz t0, finish

        j next    

finish:
        li a0, 2
        mv a1, t6
        addi a2, t6, 1
        la a3, MESSAGE_BUFFER
        ecall
        
        j .
