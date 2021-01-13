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

/// A 64-bit ELF section header.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct SectionHeader {
    /// The offset into the symbol header string table for the name of this
    /// section.
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    /// Offset of the section from the beginning of the file, may be zero.
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    /// For sections that contain fixed size entry, the size of each entry in
    /// bytes, otherwise zero.
    pub sh_entsize: u64,
}

/// A 64-bit ELF symbol.
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct Symbol {
    /// The offset of the symbol's name in the string table. A value of zero
    /// indicates that the symbol is unnamed.
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    /// The index of the related section in the section header table.
    pub st_shndx: u16,
    /// The value of the associated symbol.
    pub st_value: u64,
    /// The size of the associated symbol if applicable. Zero if the symbol has
    /// no size or an unknown size.
    pub st_size: u64,
}

/// Tries to find the section header for the section with the given name.
///
/// Returns `Err` if no section with the given name can be found.
pub fn find_section_header(elf_addr: u64, name: &str) -> Result<SectionHeader, ()> {
    let file_header = unsafe { *(elf_addr as *const FileHeader) };

    // Make sure section header table entries are the size of one section header
    assert_eq!(
        file_header.e_shentsize as usize,
        core::mem::size_of::<SectionHeader>()
    );

    let section_header_table = (elf_addr + file_header.e_shoff) as *const SectionHeader;
    let section_header_string_table_header =
        unsafe { section_header_table.offset(file_header.e_shstrndx as isize) };
    let section_header_string_table_ptr =
        unsafe { (elf_addr + (*section_header_string_table_header).sh_offset) as *const u8 };

    // Check for each section if it the name matches
    for i in 0..file_header.e_shnum {
        let section_header = unsafe { *section_header_table.offset(i as isize) };
        let name_ptr =
            unsafe { section_header_string_table_ptr.offset(section_header.sh_name as isize) };
        if memeq(name_ptr, name.as_ptr(), name.len()) {
            return Ok(section_header);
        }
    }

    Err(())
}

/// Tries to find the symbol with the given name.
///
/// Returns `Err` if no symbol with the given name can be found.
pub fn find_symbol(elf_addr: u64, name: &str) -> Result<Symbol, ()> {
    // Find the symbol and string tables
    let symtab_sh = find_section_header(elf_addr, ".symtab").expect(".symtab not found");
    let strtab_sh = find_section_header(elf_addr, ".strtab").expect(".strtab not found");

    let symtab = (elf_addr + symtab_sh.sh_offset) as *const Symbol;
    let strtab_ptr = (elf_addr + strtab_sh.sh_offset) as *const u8;

    // Make sure symbol table entries are the size of one symbol
    assert_eq!(
        symtab_sh.sh_entsize as usize,
        core::mem::size_of::<Symbol>()
    );

    // Calculate how many symbols there are
    let symbol_count = symtab_sh.sh_size / symtab_sh.sh_entsize;

    // Check for each symbol if it the name matches
    for i in 0..symbol_count {
        let symbol = unsafe { *symtab.offset(i as isize) };

        // Skip the symbol if it's unnamed
        if symbol.st_name == 0 {
            continue;
        }

        let name_ptr = unsafe { strtab_ptr.offset(symbol.st_name as isize) };
        if memeq(name_ptr, name.as_ptr(), name.len()) {
            return Ok(symbol);
        }
    }

    Err(())
}

/// Checks if two blocks of memory are equal.
fn memeq(s1: *const u8, s2: *const u8, n: usize) -> bool {
    // Check each byte for equality, return early if they differ
    for i in 0..n {
        unsafe {
            if *s1.offset(i as isize) != *s2.offset(i as isize) {
                return false;
            }
        }
    }
    return true;
}
