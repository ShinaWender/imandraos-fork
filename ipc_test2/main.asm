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

recv:
        li a0, 3
        mv a1, t6
        la a2, MESSAGE_BUFFER
        ecall

        beqz a0, success
        j recv

success:
        la a0, MESSAGE_BUFFER
        call print_string

        li a0, 0
        ecall

print_string:
        li t1, 0xe000
        lb t0, 0(a0)
        beqz t0, finish
        sb t0, 0(t1)
        addi a0, a0, 1
        j print_string

finish:
        ret
