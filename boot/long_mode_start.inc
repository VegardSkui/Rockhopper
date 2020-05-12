BITS 64
long_mode_start:
    ; Set all segment registers (except code) to zero
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ; Empty the screen
    mov ax, 0x1020      ; Space with blue background
    mov rcx, 2000       ; Repeat for 80*25 characters
    mov rdi, 0xB8000    ; Begin at the start of the text buffer
    rep stosw

hang:
    hlt
    jmp hang
