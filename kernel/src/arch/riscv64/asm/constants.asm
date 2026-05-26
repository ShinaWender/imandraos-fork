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

.section .rodata

.align 16

.global TEXT_BEGIN
TEXT_BEGIN: .quad _TEXT_BEGIN

.global TEXT_END
TEXT_END: .quad _TEXT_END

.global RODATA_BEGIN
RODATA_BEGIN: .quad _RODATA_BEGIN

.global RODATA_END
RODATA_END: .quad _RODATA_END

.global DATA_BEGIN
DATA_BEGIN: .quad _DATA_BEGIN

.global DATA_END
DATA_END: .quad _DATA_END

.global BSS_BEGIN
BSS_BEGIN: .quad _BSS_BEGIN

.global BSS_END
BSS_END: .quad _BSS_END

.global HEAP_BEGIN
HEAP_BEGIN: .quad _HEAP_BEGIN
