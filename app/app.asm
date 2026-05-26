.section .text

.global start
start:
        li a0, 0x10000000
        li t0, 0x61
        li t1, 0x7a
        
write_byte:
        sb t0, 0(a0)
        addi t0, t0, 1
        beq t0, t1, repeat_again
        j write_byte
        
repeat_again:
        li t0, 0x61
        j write_byte
        
