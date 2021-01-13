use rk_x86_64::gdt;

// Create a custom type for the GDT such that the size doesn't have to be
// updated twice (in the declaration and in the load call) when changed later.
type GlobalDescriptorTable = [u64; 3];

static GDT: GlobalDescriptorTable = [
    gdt::null(),
    gdt::kernel_code_segment(),
    gdt::kernel_data_segment(),
];

pub fn init() {
    unsafe {
        // Load the GDT, this is safe because the GDT is static and will exists for as
        // long as the kernel is running.
        gdt::load(
            core::mem::size_of::<GlobalDescriptorTable>(),
            GDT.as_ptr() as u64,
        );

        // Load the code segment register with the kernel code segment (offset 0x8 in
        // the GDT)
        rk_x86_64::register::cs::write(0x8);

        // Load the data segment registers with the kernel data segment (offset
        // 0x10 in the GDT)
        asm!(
            "mov ds, ax",
            "mov es, ax",
            "mov fs, ax",
            "mov gs, ax",
            "mov ss, ax",
            in("ax") 0x10
        );
    }
}
