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

/* Allocate 4096 bytes for each hart stack */

.align 16
STACK:
.rep (4096 * 64)
.byte 0
.endr

.section .data

.align 16
FIRST_HART_ID:
.short 0xffff

.section .text.init

/* Hart id is stored in a0 register,
 * Pointer to the device tree blob is stored in a1 register.
 */
.global start
start:
        la t0, trap_handler_table
        ori t0, t0, 1
        csrw stvec, t0
        li t0, 1
        csrw sie, t0

        li t0, 32
        csrs sie, t0

        la sp, STACK
        li t1, 4096
        mul t0, a0, t1
        add t0, t0, t1
        add sp, sp, t0

        li t0, 0xffff
        la t2, FIRST_HART_ID
        ld t1, 0(t2)
        beq t0, t1, main_hart
        j not_main_hart

main_hart:
        sd a0, 0(t2)
        mv a2, a1 # device_tree_blob = a1 register value
        mv a1, a0 # cpu_id = a0 register value
        li a0, 1 # is_main_cpu = true
        j main

not_main_hart:
        mv a2, a1 # device_tree_blob = a1 register value
        mv a1, a0 # cpu_id = a0 register value
        li a0, 0 # is_main_cpu = false
        j main

        j .
