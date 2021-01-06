KERNEL_OFFSET = 0xfffffffff8000000;

SECTIONS {
    . = KERNEL_OFFSET;

    .text : {
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }
}
