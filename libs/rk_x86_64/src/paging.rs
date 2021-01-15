/// A virtual representation of a page table. Does not represent a table
/// directly and it's memory can thus not be used as the page table itself.
pub struct PageTable {
    addr: u64,
}

impl PageTable {
    pub unsafe fn new(addr: u64) -> Self {
        Self { addr }
    }

    /// Returns the full address for the entry at a given index.
    #[inline]
    fn entry_addr(&self, i: u16) -> u64 {
        self.addr + 0x8 * i as u64
    }

    /// Returns the entry in the table at the given index.
    pub fn entry(&self, i: u16) -> PageTableEntry {
        assert!(i < 512, "Invalid index");

        // Safety: The user guaranteed every entry to be valid when creating the page
        // table.
        unsafe { PageTableEntry::read(self.entry_addr(i) as *const u64) }
    }

    /// Set an entry of the page table, overwriting any existing entry at the
    /// same index.
    ///
    /// # Safety
    /// Changing page table entries can break memory safety if done incorrectly.
    pub unsafe fn set_entry(&self, i: u16, entry: PageTableEntry) {
        assert!(i < 512, "Invalid index");

        core::ptr::write(self.entry_addr(i) as *mut u64, entry.data())
    }
}

/// An entry in a page table.
#[derive(Debug)]
pub struct PageTableEntry(u64);

impl PageTableEntry {
    /// Creates a new page table entry with the given address and flags.
    pub fn new(addr: u64, flags: u64) -> Self {
        // TODO: Make sure the address doesn't interfere with the flags and vice versa.
        Self(addr | flags)
    }

    /// Reads a page table entry from the specified address.
    ///
    /// # Safety
    /// The address must point to a page table entry.
    pub unsafe fn read(addr: *const u64) -> Self {
        Self(core::ptr::read(addr))
    }

    #[inline]
    pub fn is_present(&self) -> bool {
        self.0 & 1 == 1
    }

    #[inline]
    pub fn addr(&self) -> u64 {
        // Remove the 12 least significant bits (the options)
        self.0 >> 12 << 12
    }

    /// Returns the raw bit data of the entry.
    #[inline]
    pub fn data(&self) -> u64 {
        self.0
    }
}
