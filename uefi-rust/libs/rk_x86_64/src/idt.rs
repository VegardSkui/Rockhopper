use core::marker::PhantomData;

/// An IDT with 256 entries.
///
/// The first 32 entries are the CPU exceptions and are named in order to
/// enforce correct handler types. The remaining 224 interrupts are available
/// through the `interrupts` array.
#[repr(C)]
pub struct InterruptDescriptorTable {
    pub divide_by_zero_error: Descriptor<InterruptHandler>,
    pub debug: Descriptor<InterruptHandler>,
    pub non_maskable_interrupt: Descriptor<InterruptHandler>,
    pub breakpoint: Descriptor<InterruptHandler>,
    pub overflow: Descriptor<InterruptHandler>,
    pub bound_range_exceeded: Descriptor<InterruptHandler>,
    pub invalid_opcode: Descriptor<InterruptHandler>,
    pub device_not_available: Descriptor<InterruptHandler>,

    /// Double Fault is an Abort type CPU exception, and is not recoverable.
    ///
    /// The error code is always zero and can be ignored.
    pub double_fault: Descriptor<DivergingInterruptHandlerWithErrorCode>,

    // LEGACY
    coprocessor_segment_overrun: Descriptor<InterruptHandler>,

    pub invalid_tss: Descriptor<InterruptHandlerWithErrorCode>,
    pub segment_not_present: Descriptor<InterruptHandlerWithErrorCode>,
    pub stack_segment_fault: Descriptor<InterruptHandlerWithErrorCode>,
    pub general_protection_fault: Descriptor<InterruptHandlerWithErrorCode>,
    pub page_fault: Descriptor<InterruptHandlerWithErrorCode>,

    reserved15: Descriptor<InterruptHandler>,

    pub x87_floating_point_exception: Descriptor<InterruptHandler>,
    pub alignment_check: Descriptor<InterruptHandlerWithErrorCode>,

    /// Machine Check is an abort type CPU exception and is not recoverable.
    pub machine_check: Descriptor<DivergingInterruptHandler>,

    pub simd_floating_point_exception: Descriptor<InterruptHandler>,
    pub virtualization_exception: Descriptor<InterruptHandler>,

    reserved21_29: [Descriptor<InterruptHandler>; 9],

    pub security_exception: Descriptor<InterruptHandlerWithErrorCode>,

    reserved31: Descriptor<InterruptHandler>,

    /// General interrupts, remember that the first index refers to the 32nd
    /// interrupt.
    pub interrupts: [Descriptor<InterruptHandler>; 224],
}

impl InterruptDescriptorTable {
    /// Creates a new IDT with all entries marked as not present.
    pub const fn new() -> Self {
        Self {
            divide_by_zero_error: Descriptor::not_present(),
            debug: Descriptor::not_present(),
            non_maskable_interrupt: Descriptor::not_present(),
            breakpoint: Descriptor::not_present(),
            overflow: Descriptor::not_present(),
            bound_range_exceeded: Descriptor::not_present(),
            invalid_opcode: Descriptor::not_present(),
            device_not_available: Descriptor::not_present(),
            double_fault: Descriptor::not_present(),
            coprocessor_segment_overrun: Descriptor::not_present(),
            invalid_tss: Descriptor::not_present(),
            segment_not_present: Descriptor::not_present(),
            stack_segment_fault: Descriptor::not_present(),
            general_protection_fault: Descriptor::not_present(),
            page_fault: Descriptor::not_present(),
            reserved15: Descriptor::not_present(),
            x87_floating_point_exception: Descriptor::not_present(),
            alignment_check: Descriptor::not_present(),
            machine_check: Descriptor::not_present(),
            simd_floating_point_exception: Descriptor::not_present(),
            virtualization_exception: Descriptor::not_present(),
            reserved21_29: [Descriptor::not_present(); 9],
            security_exception: Descriptor::not_present(),
            reserved31: Descriptor::not_present(),
            interrupts: [Descriptor::not_present(); 224],
        }
    }

    /// Loads the IDT using the `lidt` instruction.
    ///
    /// # Safety
    /// `self` must not be destroyed for as long as it's the active IDT.
    pub fn load(&self) {
        let pointer = InterruptTablePointer {
            size: (core::mem::size_of::<Self>() - 1) as u16,
            offset: self as *const _ as u64,
        };

        unsafe { asm!("lidt [{}]", in(reg) &pointer) };
    }
}

/// An IDT entry.
///
/// The generic type should be one of the interrupt handler types.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Descriptor<F> {
    offset_1: u16,
    selector: u16,
    options: u16,
    offset_2: u16,
    offset_3: u32,
    zero: u32,
    handler_phantom: PhantomData<F>,
}

impl<F> Descriptor<F> {
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
            handler_phantom: PhantomData,
        }
    }

    /// Sets the handler function using the address.
    fn set_handler_internal(&mut self, addr: u64) {
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

impl Descriptor<InterruptHandler> {
    /// Sets the handler function.
    pub fn set_handler(&mut self, handler: InterruptHandler) {
        self.set_handler_internal(handler as u64);
    }
}

impl Descriptor<DivergingInterruptHandler> {
    /// Sets the handler function.
    pub fn set_handler(&mut self, handler: DivergingInterruptHandler) {
        self.set_handler_internal(handler as u64);
    }
}

impl Descriptor<InterruptHandlerWithErrorCode> {
    /// Sets the handler function.
    pub fn set_handler(&mut self, handler: InterruptHandlerWithErrorCode) {
        self.set_handler_internal(handler as u64);
    }
}

impl Descriptor<DivergingInterruptHandlerWithErrorCode> {
    /// Sets the handler function.
    pub fn set_handler(&mut self, handler: DivergingInterruptHandlerWithErrorCode) {
        self.set_handler_internal(handler as u64);
    }
}

pub type InterruptHandler = extern "x86-interrupt" fn(&InterruptFrame);
pub type DivergingInterruptHandler = extern "x86-interrupt" fn(&InterruptFrame) -> !;
pub type InterruptHandlerWithErrorCode = extern "x86-interrupt" fn(&InterruptFrame, u64);
pub type DivergingInterruptHandlerWithErrorCode =
    extern "x86-interrupt" fn(&InterruptFrame, u64) -> !;

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
