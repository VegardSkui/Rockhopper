ORG 0x7C00
BITS 16

start:
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov sp, start       ; Stack lives just underneath the bootsector and grows downwards

    mov ax, 0xABCD
    call bios_print_hex16

    call verify_a20

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

; Verifies the A20 line is enabled, if not, it prints an error message and hangs
verify_a20:
    cli
    xor ax, ax
    mov fs, ax          ; FS = 0x0000
    not ax
    mov gs, ax          ; GS = 0xFFFF
    mov di, 0x7DFE
    mov si, 0x7E0E
    mov ax, [fs:di]     ; Should be our boot signature 0x55AA (at 0000:7DFE)
    mov bx, [gs:si]     ; Memory 1 MiB up (at FFFF:7E0E)
    cmp ax, bx          ; If different, A20 must be enabled
    jne .done

    mov si, .disabled   ; If A20 is disabled, print an error and hang
    call bios_print
    jmp hang
.done:
    ret
.disabled db "ERROR: A20 Disabled", 13, 10, 0

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
