.section .text

.global start
start:
        li t0, 'b'
        li t2, '\n'
        li a0, 0x10000000
        li t1, 0xfffffff

write_byte:
        sd t0, 0(a0)
        sd t2, 0(a0)
        
sleep:
        addi t1, t1, -1
        bne t1, zero, sleep
        li t1, 0xfffffff
        j write_byte
