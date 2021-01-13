KERNEL_OFFSET = 0xfffffffff8000000;

PHDRS {
    a PT_LOAD;
}

SECTIONS {
    . = KERNEL_OFFSET;

    entry_data = .;
    . += 4096;

    .text : {
        *(.text*)
        . = ALIGN(4096);
    }

    .rodata : {
        *(.rodata*)
        . = ALIGN(4096);
    }

    .data : {
        *(.data*)
        . = ALIGN(4096);
        *(.bss*)
        . = ALIGN(4096);
    }
}
