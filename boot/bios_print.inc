; Print string from SI using the BIOS
bios_print:
    cld             ; Clear direction flag (to make sure SI is incremented)
.loop:
    lodsb           ; Load byte (character) at address DS:SI into AL
    or al, al       ; Zero means we're at the end of the string
    jz .done
    mov ah, 0x0E    ; Select teletype output function (write character)
    mov bh, 0       ; Page zero
    int 0x10        ; BIOS Interrupt
    jmp .loop       ; Repeat for the next character
.done:
    ret

; Print error string from SI using the BIOS
bios_print_error:
    push si         ; Store the current value of SI on the stack
    mov si, .error  ; Point SI to the error prefix string
    call bios_print ; Print the error prefix
    pop si          ; Restore the old value of SI
    call bios_print ; Print the string pointed to by SI
    ret
.error db "ERROR: ", 0

; Print 16 bit hex from AX using the BIOS
bios_print_hex16:
    mov di, .outhex16   ; Point DI to the beginning of the output string
    mov si, .hexstr     ; Point SI to the beginning of the hex lookup string
    mov cx, 4           ; We'll need to repeat 4 times, one for each nibble
.loop:
    rol ax, 4           ; Rotate AX by one nibble
    mov bx, ax
    and bx, 0x000F      ; Copy the last nibble of AX to BX
    mov bl, [si + bx]   ; Set BL to the ASCII character of the hex digit it now represents
    mov [di], bl        ; Set the character in the output string
    inc di              ; Point DI to the next character
    loop .loop

    mov si, .outhex16   ; Point SI to the output string
    call bios_print     ; and print
    ret

.hexstr db "0123456789ABCDEF"
.outhex16 db "0000", 13, 10, 0
