BITS 64
long_mode_start:
    ; Set all segment registers (except code) to zero
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

hang:
    hlt
    jmp hang
