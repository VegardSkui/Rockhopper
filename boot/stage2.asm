ORG 0x7E00
BITS 16

    ; Zero out a 12 KiB buffer for paging tables starting at 0x1000 (4 KiB)
    push di
    mov edi, 0x1000
    mov ecx, 0x0C00
    xor eax, eax
    cld
    rep stosd
    pop di

    mov dword [0x1000], 0x2003  ; Page Map Level 4
    mov dword [0x2000], 0x3003  ; Page Directory Pointer Table
    mov dword [0x3000], 0x0083  ; Page Directory (identity map first 2 MiB with huge page)

    ; Set the PAE and PGE bits
    mov eax, 0b10100000
    mov cr4, eax

    ; Point CR3 to the physical address of PML4
    mov edx, 0x1000
    mov cr3, edx

    ; Read the Extended Feature Enable Register (EFER) Model Specific Register (MSR)
    mov ecx, 0xC0000080
    rdmsr

    ; Set the Long Mode Enable (LME) bit and write the EFER back
    or eax, 0x00000100
    wrmsr

    ; Set the Paging (PG) and Protected Mode Enable (PE) bit to activate long mode
    mov ebx, cr0
    or ebx, 0x80000001
    mov cr0, ebx

    ; Load the Global Descriptor Table (GDT) for 64 bit defined below
    lgdt [gdt.pointer]

    ; Load CS with a 64 bit segment
    jmp gdt.code:long_mode_start

gdt:
    dq 0
.code: equ $ - gdt
    dq (1 << 43) | (1 << 44) | (1 << 47) | (1 << 53)
.pointer:
    dw $ - gdt - 1
    dq gdt

%include "bios_print.inc"

%include "long_mode_start.inc"
