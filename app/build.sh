#!/bin/sh

riscv64-unknown-none-elf-as app.asm -o app.o
riscv64-unknown-none-elf-ld -T linker.ld -o app app.o
riscv64-unknown-none-elf-objcopy -O binary app
