ORG 0x7C00
BITS 16

    xor ax, ax
    mov ds, ax

    mov si, greeting
    call bios_print

hang:
    hlt
    jmp hang

; Print string from SI using the BIOS
bios_print:
    cld             ; Clear direction flag (to make sure SI is incremented)
.loop
    lodsb           ; Load byte (character) at address DS:SI into AL
    or al, al       ; Zero means we're at the end of the string
    jz .done
    mov ah, 0x0E    ; Select teletype output function (write character)
    mov bh, 0       ; Page zero
    int 0x10        ; BIOS Interrupt
    jmp .loop       ; Repeat for the next character
.done:
    ret

greeting db "Hello World!", 13, 10, "- Rockhopper", 13, 10, 0

times 510-($-$$) db 0   ; Pad with zeros
dw 0xAA55               ; Boot signature
