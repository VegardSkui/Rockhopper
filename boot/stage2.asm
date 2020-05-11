ORG 0x7E00
BITS 16

    mov si, greeting
    call bios_print
hang:
    hlt
    jmp hang

%include "bios_print.inc"

greeting db "Hello World!", 13, 10, " - Second Stage", 13, 10, 0
