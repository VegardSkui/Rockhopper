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
    }

    .rodata : {
        *(.rodata*)
    }
}
