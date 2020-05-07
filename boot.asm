ORG 0x7C00
BITS 16

    xor ax, ax
    mov ds, ax

    mov ax, 0xABCD
    call bios_print_hex16

    mov si, greeting    ; Print greeting
    call bios_print

    call check_cpu      ; Check if long mode is supported
    jc .no_long_mode    ; If not, print an error

    jmp hang

.no_long_mode:
    mov si, no_long_mode    ; Print an error message and proceed to hang
    call bios_print

hang:
    hlt
    jmp hang

; Checks wether the CPU supports long mode
; Returns with the carry flag set, if not supported
check_cpu:
    ; Try flipping bit 21 (ID) in the EFLAGS register
    pushfd
    pop eax
    mov ecx, eax        ; Store original in ECX
    xor eax, 1 << 21    ; Flip the 21st bit in EAX
    push eax            ; Push EAX back on the stack
    popfd               ; and into the EFLAGS register

    pushfd
    pop eax             ; Read EFLAGS into EAX again
    test eax, ecx       ; If EAX == ECX, CPUID isn't available (the 21st bit should differ)
    je .no_long_mode

    mov eax, 0x80000000 ; Get highest extended function implemented
    cpuid
    cmp eax, 0x80000001 ; If less than, long mode is not supported
    jb .no_long_mode

    mov eax, 0x80000001 ; Get extended processor info and feature bits
    cpuid
    test edx, 1 << 29   ; Test if the long mode (LM) bit is set
    jz .no_long_mode

    ret

.no_long_mode:
    stc     ; Set the carry flag
    ret

%include "bios_print.asm"

greeting db "Hello World!", 13, 10, "- Rockhopper", 13, 10, 0
no_long_mode db "ERROR: CPU doesn't support long mode", 13, 10, 0

times 510-($-$$) db 0   ; Pad with zeros
dw 0xAA55               ; Boot signature
