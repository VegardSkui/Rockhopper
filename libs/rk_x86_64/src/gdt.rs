use crate::DescriptorTablePointer;

/// Loads the GDT with the given size and address using the `lgdt` instruction.
///
/// # Safety
/// The size and address must point to a valid GDT which cannot be destroyed for
/// as long as it's the active GDT.
pub unsafe fn load(size: usize, addr: u64) {
    let pointer = DescriptorTablePointer {
        size: (size - 1) as u16,
        offset: addr,
    };

    asm!("lgdt [{}]", in(reg) &pointer);
}

/// Returns a null descriptor.
pub const fn null() -> u64 {
    0
}

/// Returns a kernel code segment descriptor.
pub const fn kernel_code_segment() -> u64 {
    // Create a segment with the Granularity, Long, Present, Type (which signifies a
    // code segment when set), Readable, and Accessed bits set. The segment limit is
    // also set to its maximum value.
    0x00af_9b00_0000_ffff
}

/// Returns kernel data segment descriptor.
pub const fn kernel_data_segment() -> u64 {
    // Create a segment with the Granularity, Long, Present, Writable, and Accessed
    // bits set. The segment limit is also set to its maximum value. The Type bit
    // indicates a data segment when clear, and is therefore not set.
    0x00af_9300_0000_ffff
}
