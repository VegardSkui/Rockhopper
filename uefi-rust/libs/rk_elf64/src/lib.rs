#![no_std]

/// A 64-bit ELF file header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct FileHeader {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    /// Offset to the start of the program header table (usually follows
    /// immediately after the file header).
    pub e_phoff: u64,
    /// Offset to the start of the section header table.
    pub e_shoff: u64,
    pub e_flags: u32,
    /// The size of the header, normally 64 bytes.
    pub e_ehsize: u16,
    /// The size of a program header table entry.
    pub e_phentsize: u16,
    /// The number of entries in the program header table.
    pub e_phnum: u16,
    /// The size of a section header table entry.
    pub e_shentsize: u16,
    /// The number of entries in the section header table.
    pub e_shnum: u16,
    /// The index of the section header table entry that contains the section
    /// names.
    pub e_shstrndx: u16,
}

/// A 64-bit ELF program header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    /// Virtual address of the segment in memory.
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

pub struct ProgramType(u32);

impl ProgramType {
    pub const PT_LOAD: u32 = 0x0000_0001;
}
