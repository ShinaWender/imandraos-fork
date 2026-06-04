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

.section .text

.align 16
.global trap_handler_table
trap_handler_table:
.option exact
        j env_call_asm
        j .
        j .
        j .
        j .
        j timer_interrupt_asm
        j .
        j .
        j .
        j .
        j .
        j .
        j .
        j .
        j .
        j .
.option noexact
        
.align 16
.global timer_interrupt_asm
timer_interrupt_asm:
        csrw sscratch, a0
        li a0, 0x90000000

        sd x1, 0(a0)
        sd x2, 8(a0)
        sd x3, 16(a0)
        sd x4, 24(a0)
        sd x5, 32(a0)
        sd x6, 40(a0)
        sd x7, 48(a0)
        sd x8, 56(a0)
        sd x9, 64(a0)
        sd x11, 80(a0)
        sd x12, 88(a0)
        sd x13, 96(a0)
        sd x14, 104(a0)
        sd x15, 112(a0)
        sd x16, 120(a0)
        sd x17, 128(a0)
        sd x18, 136(a0)
        sd x19, 144(a0)
        sd x20, 152(a0)
        sd x21, 160(a0)
        sd x22, 168(a0)
        sd x23, 176(a0)
        sd x24, 184(a0)
        sd x25, 192(a0)
        sd x26, 200(a0)
        sd x27, 208(a0)
        sd x28, 216(a0)
        sd x29, 224(a0)
        sd x30, 232(a0)
        sd x31, 240(a0)

        ld sp, 248(a0)

        csrr t0, sscratch
        sd t0, 72(a0)

        li t0, 32
        csrc sip, t0

        csrr a0, sepc
        j _switch

.align 16
.global env_call_asm
env_call_asm:
        csrw sscratch, a0
        li a0, 0x90000000

        sd x1, 0(a0)
        sd x2, 8(a0)
        sd x3, 16(a0)
        sd x4, 24(a0)
        sd x5, 32(a0)
        sd x6, 40(a0)
        sd x7, 48(a0)
        sd x8, 56(a0)
        sd x9, 64(a0)
        sd x11, 80(a0)
        sd x12, 88(a0)
        sd x13, 96(a0)
        sd x14, 104(a0)
        sd x15, 112(a0)
        sd x16, 120(a0)
        sd x17, 128(a0)
        sd x18, 136(a0)
        sd x19, 144(a0)
        sd x20, 152(a0)
        sd x21, 160(a0)
        sd x22, 168(a0)
        sd x23, 176(a0)
        sd x24, 184(a0)
        sd x25, 192(a0)
        sd x26, 200(a0)
        sd x27, 208(a0)
        sd x28, 216(a0)
        sd x29, 224(a0)
        sd x30, 232(a0)
        sd x31, 240(a0)

        ld sp, 248(a0)

        csrr t0, sscratch
        sd t0, 72(a0)
        mv a0, t0
        
        call syscall_handler

        li t0, 0x90000000
        sd a0, 72(t0)

        csrr a0, sepc
        j _switch
