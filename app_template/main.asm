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
MESSAGE_BUFFER2:
.rep 4096
.byte 0
.endr

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

        li t0, 0xe000
        addi a0, a0, '0'
        sd a0, 0(t0)

        la t0, MESSAGE_BUFFER
        li t1, 'A'
        sd t1, 0(t0)
        sd t1, 1(t0)
        sd t1, 3(t0)

        li a0, 2
        mv a1, t6
        mv a2, t6
        la a3, MESSAGE_BUFFER
        ecall

        li a0, 3
        mv a1, t6
        la a2, MESSAGE_BUFFER2
        ecall

        li t0, 0xe000

        beqz a0, success
        j fail
        
success:
        li a0, 's'
        sd a0, 0(t0)

        la a1, MESSAGE_BUFFER2
        ld a0, 0(a1)
        sd a0, 0(t0)
        
        j exit
        
fail:
        li a0, 'f'
        sd a0, 0(t0)
        j exit

exit:
        li a0, 0
        ecall
