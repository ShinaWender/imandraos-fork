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
.global jump_to_user_space
jump_to_user_space:
   csrr t0, sstatus
   li t1, 1
   slli t1, t1, 18
   or t0, t0, t1
   csrw sstatus, t0

   csrw sepc, a0
   li a0, 0x90000000

   sd sp, 248(a0)

   ld x1, 0(a0)
   ld x2, 8(a0)
   ld x3, 16(a0)
   ld x4, 24(a0)
   ld x5, 32(a0)
   ld x6, 40(a0)
   ld x7, 48(a0)
   ld x8, 56(a0)
   ld x9, 64(a0)
   ld x11, 80(a0)
   ld x12, 88(a0)
   ld x13, 96(a0)
   ld x14, 104(a0)
   ld x15, 112(a0)
   ld x16, 120(a0)
   ld x17, 128(a0)
   ld x18, 136(a0)
   ld x19, 144(a0)
   ld x20, 152(a0)
   ld x21, 160(a0)
   ld x22, 168(a0)
   ld x23, 176(a0)
   ld x24, 184(a0)
   ld x25, 192(a0)
   ld x26, 200(a0)
   ld x27, 208(a0)
   ld x28, 216(a0)
   ld x29, 224(a0)
   ld x30, 232(a0)
   ld x31, 240(a0)
   ld x10, 72(a0)

   sret
   
