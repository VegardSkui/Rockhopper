ORG 0x7C00
BITS 16

start:
    xor ax, ax
    mov ds, ax
    mov ss, ax
    mov sp, start       ; Stack lives just underneath the bootsector and grows downwards

    ; Store the drive index from DL, we assume the BIOS set it correctly
    mov [drive_index], dl

    cli                 ; Disable interrupts

    ; Verify the A20 line is enabled
    ;xor ax, ax         ; AX = 0x0000 (set above, must be uncommented if moved)
    mov fs, ax          ; FS = 0x0000
    not ax
    mov gs, ax          ; GS = 0xFFFF
    mov di, 0x7DFE
    mov si, 0x7E0E
    mov ax, [fs:di]     ; Should be our boot signature 0x55AA (at 0000:7DFE)
    mov bx, [gs:si]     ; Memory 1 MiB up (at FFFF:7E0E)
    cmp ax, bx          ; If different, A20 must be enabled
    je a20_disabled     ; If A20 is disabled, print an error and hang

    ; Check whether the CPU has CPUID available
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
    je cpuid_not_available

    ; Check if the CPU supports long mode (64 bit)
    mov eax, 0x80000000 ; Get highest extended function implemented
    cpuid
    cmp eax, 0x80000001 ; If less than, long mode is not supported
    jb no_long_mode

    push edx            ; Store EDX on the stack (it's overwritten by CPUID)
    mov eax, 0x80000001 ; Get extended processor info and feature bits
    cpuid
    test edx, 1 << 29   ; Test if the long mode (LM) bit is set
    pop edx
    jz no_long_mode

    ; Retrieve the drive index back into DL for the next operations
    mov dl, [drive_index]

    ; Check if BIOS INT 13h extensions are present
    mov ah, 0x41
    mov bx, 0x55AA
    int 0x13
    jc no_extensions    ; Carry flag will be set if the extensions are missing

    ; Check the drive sector size, we only support 512 bytes per sector
    mov ah, 0x48
    mov si, drive_parameter_result_buffer
    int 0x13
    cmp word [drive_parameter_result_buffer.bytes_per_sector], 0x0200
    jne unsupported_sector_size

    ; Load the second stage
    mov si, stage2_dap  ; Point SI to the Disk Address Packet
    mov ah, 0x42
    int 0x13
    jc read_failed      ; Carry flag is set if an error occured

    jmp 0x7E00          ; Jump to the start of the second stage

; Print the error string in SI and hang
error:
    call bios_print_error

hang:
    hlt
    jmp hang

a20_disabled:
    mov si, .error
    call error
.error db "A20 Disabled", 13, 10, 0

cpuid_not_available:
    mov si, .error
    call error
.error db "CPUID not available", 13, 10, 0

no_long_mode:
    mov si, .error
    call error
.error db "CPU does not support long mode"

no_extensions:
    mov si, .error
    call error
.error db "BIOS INT 13h Extensions not supported", 13, 10, 0

unsupported_sector_size:
    mov si, .error
    call error
.error db "Unsupported sector size", 13, 10, 0

read_failed:
    mov si, .error
    call error
.error db "Read failed", 13, 10, 0

%include "bios_print.inc"

drive_index db 0

drive_parameter_result_buffer:
    .result_buffer_size dw 0x1E
    .information_flags dw 0
    .physical_cylinder_count dd 0
    .physical_head_count dd 0
    .physical_sectors_per_track dd 0
    .absolute_sector_count1 dd 0
    .absolute_sector_count2 dd 0
    .bytes_per_sector dw 0
    .edd_configuration_parameters dd 0

stage2_dap:
    db 0x10     ; Disk address packet (DAP) size
    db 0        ; Unused, always zero
    dw 16       ; Numbers of sectors to read
    dw 0x7E00   ; Result memory buffer offset
    dw 0        ; Result memory buffer segment
    dd 1024     ; Logical Block Address (LBA) rightmost bits
    dd 0        ; LBA leftmost bits

times 510-($-$$) db 0
dw 0xAA55
