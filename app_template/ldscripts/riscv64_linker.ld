OUTPUT_ARCH("riscv")

ENTRY(start)

PAGE_SIZE = 0x1000;

SECTIONS {
    /DISCARD/ : {
        *(.eh_frame);
    }
    
    . = 0x90100000;

    .text ALIGN(PAGE_SIZE) : {
        *(.text.init);
        *(.text .text*);
    }

    .rodata ALIGN(PAGE_SIZE) : {
        *(.rodata .rodata*);
    }
    
    .data ALIGN(PAGE_SIZE) : { 
        *(.data .data.*);
        *(.sdata .sdata.*);
    }
    
    .bss ALIGN(PAGE_SIZE) : { 
        *(.bss .bss*);
        *(.sbss .sbss*);
        *(COMMON);
    }
}

