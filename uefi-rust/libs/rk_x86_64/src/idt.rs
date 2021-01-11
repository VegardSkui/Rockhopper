use core::ops::{Index, IndexMut};

#[repr(C)]
pub struct InterruptDescriptorTable([Descriptor; 256]);

impl InterruptDescriptorTable {
    /// Creates a new IDT with all entries marked as not present.
    pub const fn new() -> Self {
        Self {
            0: [Descriptor::not_present(); 256],
        }
    }

    /// Loads the IDT using the `lidt` instruction.
    ///
    /// # Safety
    /// `self` must not be destroyed for as long as it's the active IDT.
    pub fn load(&self) {
        let pointer = InterruptTablePointer {
            size: (core::mem::size_of::<Self>() - 1) as u16,
            offset: self.0.as_ptr() as u64,
        };

        unsafe { asm!("lidt [{}]", in(reg) &pointer) };
    }
}

impl Index<usize> for InterruptDescriptorTable {
    type Output = Descriptor;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for InterruptDescriptorTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// An IDT entry.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Descriptor {
    offset_1: u16,
    selector: u16,
    options: u16,
    offset_2: u16,
    offset_3: u32,
    zero: u32,
}

impl Descriptor {
    /// Returns an entry with all bits set to 0, except the bits which must be
    /// 1.
    pub const fn not_present() -> Self {
        Self {
            offset_1: 0,
            selector: 0,
            options: 0b111 << 9,
            offset_2: 0,
            offset_3: 0,
            zero: 0,
        }
    }

    /// Set the handler function.
    pub fn set_handler(&mut self, handler: Handler) {
        let addr: u64 = handler as u64;

        self.offset_1 = addr as u16;
        self.offset_2 = (addr >> 16) as u16;
        self.offset_3 = (addr >> 32) as u32;

        // TODO: Don't hardcode the selector, read and use the current code selector
        // instead.
        self.selector = 0x38;

        // Set the present bit
        self.options |= 1 << 15;
    }
}

pub type Handler = extern "x86-interrupt" fn(&InterruptFrame);

#[derive(Debug)]
#[repr(C)]
pub struct InterruptFrame {
    ip: u64,
    cs: u64,
    flags: u64,
    sp: u64,
    ss: u64,
}

// Use packed representation to stop Rust from adding padding and thus breaking
// the representation.
#[repr(C, packed)]
struct InterruptTablePointer {
    /// The size of the IDT - 1.
    size: u16,
    /// Pointer to the IDT.
    offset: u64,
}
