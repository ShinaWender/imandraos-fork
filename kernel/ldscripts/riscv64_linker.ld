OUTPUT_ARCH("riscv")

ENTRY(start)

PAGE_SIZE = 0x1000;

SECTIONS {
    /DISCARD/ : {
        *(.eh_frame);
    }

    _KERNEL_SPACE_BEGIN = 0x80200000;
    . = _KERNEL_SPACE_BEGIN;

    .text ALIGN(PAGE_SIZE) : {
        _TEXT_BEGIN = .;
        *(.text.init);
        *(.text .text*);
        _TEXT_END = .;
    }

    .rodata ALIGN(PAGE_SIZE) : {
        _RODATA_BEGIN = .;
        *(.rodata .rodata*);
        _RODATA_END = .;
    }
    
    .data ALIGN(PAGE_SIZE) : { 
        _DATA_BEGIN = .;
        *(.data .data.*);
        *(.sdata .sdata.*);
        _DATA_END = .;
    }
    
    .bss ALIGN(PAGE_SIZE) : { 
        _BSS_BEGIN = .;
        *(.bss .bss*);
        *(.sbss .sbss*);
        *(COMMON);
        _BSS_END = .;
    }

    . = ALIGN(PAGE_SIZE);
    _HEAP_BEGIN = .;
}

