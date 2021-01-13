#[repr(C)]
pub enum EfiAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType,
}

#[repr(C)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiMaxMemoryType,
}

#[repr(transparent)]
pub struct EfiPhysicalAddress(pub u64);

#[repr(C)]
pub struct EfiMemoryDescriptor {
    type1: u32,
    physical_start: EfiPhysicalAddress,
    virtual_start: EfiVirtualAddress,
    number_of_pages: u64,
    attribute: u64,
}

#[repr(transparent)]
pub struct EfiVirtualAddress(u64);
