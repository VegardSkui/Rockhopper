#![no_std]
#![no_main]
#![feature(asm)]

use core::convert::TryInto;
use core::panic::PanicInfo;
use rk_uefi::data_types::{
    Char16, EfiAllocateType, EfiHandle, EfiMemoryDescriptor, EfiMemoryType, EfiPhysicalAddress,
    EfiStatus,
};
use rk_uefi::guid::{
    EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, EFI_LOADED_IMAGE_PROTOCOL_GUID,
    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
};
use rk_uefi::protocol::{
    EfiFileProtocol, EfiFileSystemInfo, EfiGraphicsOutputProtocol, EfiLoadedImageProtocol,
    EfiSimpleFileSystemProtocol, EFI_FILE_HIDDEN, EFI_FILE_MODE_READ, EFI_FILE_READ_ONLY,
    EFI_FILE_SYSTEM, EFI_FILE_SYSTEM_INFO_ID,
};
use rk_uefi::table::EfiSystemTable;
use rk_uefi::{print, println, system_table};

/// The main entry point for the UEFI application.
#[no_mangle]
fn efi_main(image_handle: EfiHandle, system_table: &'static mut EfiSystemTable) -> EfiStatus {
    rk_uefi::init(image_handle, &mut *system_table);

    rk_uefi::system_table().con_out().reset(false);

    println!("Hello World!");

    // Print the firmware vendor and revision
    print!("Firmware: ");
    rk_uefi::system_table()
        .con_out()
        .output_string(rk_uefi::system_table().firmware_vendor());
    println!(
        ", rev. {:#010x}",
        rk_uefi::system_table().firmware_revision()
    );

    // Print the UEFI revision
    let revision = rk_uefi::system_table().revision();
    println!("UEFI v{}.{}\n", (revision >> 16) as u16, revision as u16);

    // Print volume label
    let root = unsafe { get_volume_root(image_handle).as_ref().unwrap() };
    let mut buffer_size: usize = 1024; // This should be more than enough, and wasteful
    let buffer = rk_uefi::system_table()
        .boot_services()
        .allocate_pool(EfiMemoryType::EfiLoaderData, buffer_size)
        .expect("Could not allocate buffer");
    let status = root.get_info(&EFI_FILE_SYSTEM_INFO_ID, &mut buffer_size, unsafe {
        &mut *buffer
    });
    if status.is_error() {
        panic!("ERROR! {:?}", status);
    }
    let file_info = unsafe { &*(buffer as *const EfiFileSystemInfo) };
    print!("Volume Label: ");
    rk_uefi::system_table()
        .con_out()
        .output_string(&file_info.volume_label[0]);
    println!("");

    // Print info about the current graphics mode
    let mut ptr = core::ptr::null_mut();
    let status = rk_uefi::system_table().boot_services().locate_protocol(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        core::ptr::null_mut(),
        &mut ptr,
    );
    if status.is_error() {
        panic!("Unable to locate GOP");
    }
    let gop = unsafe { &*(ptr as *mut EfiGraphicsOutputProtocol) };
    let gop_mode = gop.mode();
    let gop_mode_info = unsafe { *gop.mode().info };
    println!(
        "Mode {}, width {}, height {}, fb base {:#x}",
        gop_mode.mode,
        gop_mode_info.horizontal_resolution,
        gop_mode_info.vertical_resolution,
        gop_mode.frame_buffer_base.0
    );

    // Load the kernel ELF
    let kernel_elf_addr = load_kernel_elf(image_handle);

    let kernel_elf_file_header = unsafe { *(kernel_elf_addr.0 as *const rk_elf64::FileHeader) };
    // TODO: Make sure we actually have a correct ELF file header (check magic
    // number and machine type at least).
    if kernel_elf_file_header.e_phentsize != 0x38 {
        panic!("Unexpected program header table entry size");
    }

    let mut kernel_size: u64 = 0;
    let mut kernel_addr = EfiPhysicalAddress(0);
    let kernel_entry = kernel_elf_file_header.e_entry;

    // Go through each program header
    let kernel_elf_program_header_table =
        (kernel_elf_addr.0 + kernel_elf_file_header.e_phoff) as *const rk_elf64::ProgramHeader;
    for i in 0..kernel_elf_file_header.e_phnum {
        let ph = unsafe { *kernel_elf_program_header_table.offset(i as isize) };
        // Look for the section containing our kernel code
        if ph.p_type == rk_elf64::ProgramType::PT_LOAD && ph.p_vaddr == 0xffff_ffff_f800_0000 {
            kernel_size = ph.p_filesz;

            // Allocate memory for the kernel
            let pages = (kernel_size as usize + 4095) / 4096;
            kernel_addr = rk_uefi::system_table()
                .boot_services()
                .allocate_pages(
                    EfiAllocateType::AllocateAnyPages,
                    EfiMemoryType::EfiLoaderData,
                    pages,
                )
                .expect("Could not allocate memory for the kernel");

            // Copy the loadable kernel section into the allocated space
            let destination = kernel_addr.0 as *mut core::ffi::c_void;
            let source = (kernel_elf_addr.0 + ph.p_offset) as *mut core::ffi::c_void;
            rk_uefi::system_table().boot_services().copy_mem(
                destination,
                source,
                kernel_size as usize,
            );

            break;
        }
    }
    if kernel_size == 0 {
        panic!("Could not find kernel");
    }
    if kernel_size > 2 * 1024 * 1024 {
        panic!("Kernel cannot be larger than 2 MiB");
    }
    println!("kernel_size = {} bytes", kernel_size);
    println!("kernel_addr = {:#x} (phys)", kernel_addr.0);
    println!("kernel_entry = {:#x} (virt)", kernel_entry);

    // Get the memory map
    // We don't actually pass or use it yet, but we need the map key to exit boot
    // services.
    let mut memory_map_size: usize = 0;
    let mut memory_map = core::ptr::null_mut();
    let mut map_key: usize = 0;
    let mut descriptor_size: usize = 0;
    let mut descriptor_version: u32 = 0;
    let status = rk_uefi::system_table().boot_services().get_memory_map(
        &mut memory_map_size,
        memory_map,
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    if status != EfiStatus::EFI_BUFFER_TOO_SMALL {
        panic!("Unexpected status while getting memory map size, got {:?}, expected EFI_BUFFER_TOO_SMALL", status);
    }
    // Calculate how many pages are needed to fit the memory map. Adding space for
    // two extra descriptors since the memory map may need to be expanded while
    // allocating space for itself. Also adding 4095 bytes (1 byte less than the
    // page size) to make sure any rounding is upwards.
    memory_map_size += 2 * descriptor_size;
    let pages = (memory_map_size + 4095) / 4096;
    // Allocate pages for the memory map
    let memory_map_addr = rk_uefi::system_table()
        .boot_services()
        .allocate_pages(
            EfiAllocateType::AllocateAnyPages,
            EfiMemoryType::EfiLoaderData,
            pages,
        )
        .expect("Could not allocate pages for the memory map");
    memory_map = memory_map_addr.0 as *mut EfiMemoryDescriptor;
    // Finally, actually get the memory map
    let status = rk_uefi::system_table().boot_services().get_memory_map(
        &mut memory_map_size,
        memory_map,
        &mut map_key,
        &mut descriptor_size,
        &mut descriptor_version,
    );
    if status.is_error() {
        panic!("Could not get memory map: {:?}", status);
    }

    // Notify the firmware that we're taking over 😎
    rk_uefi::system_table()
        .boot_services()
        .exit_boot_services(image_handle, map_key);

    // Create new page tables for our higher half kernel
    unsafe {
        // Zero out space for 5 paging tables
        core::ptr::write_bytes(0x70000 as *mut u8, 0, 5 * 4096);

        // PML4
        core::ptr::write(0x70000 as *mut u64, 0x71000 | 0b11);
        core::ptr::write((0x70000 + 511 * 8) as *mut u64, 0x72000 | 0b11);

        // Identity map the first 4 GiB using three PDP huge pages
        core::ptr::write(0x71000 as *mut u64, 0b1000_0011);
        core::ptr::write((0x71000 + 1 * 8) as *mut u64, 0x4000_0000 | 0b1000_0011);
        core::ptr::write((0x71000 + 2 * 8) as *mut u64, 0x8000_0000 | 0b1000_0011);
        core::ptr::write((0x71000 + 3 * 8) as *mut u64, 0xc000_0000 | 0b1000_0011);

        // PDP
        core::ptr::write((0x72000 + 511 * 8) as *mut u64, 0x73000 | 0b11);

        // PD
        // We can't just use a single huge page to map the 2 MiB because we cannot
        // guarantee that the memory UEFI allocated to us for the kernel is 2
        // MiB-aligned, we can only guarantee the normal 4096 byte page-alignment.
        core::ptr::write((0x73000 + (0o700 * 8)) as *mut u64, 0x74000 | 0b11);

        // PT
        // Map each entry in the page table for a total of 2 MiB.
        for i in 0..512 {
            core::ptr::write(
                (0x74000 + i * 8) as *mut u64,
                (kernel_addr.0 + 0x1000 * i) | 0b11,
            );
        }
    }
    let pml4_addr: u64 = 0x70000;

    // Make sure PSE and PAE is enabled (PSE is always enabled when PAE is enabled
    // regardless of the PSE bit, but we set it anyways, just in case)
    let cr4 = rk_x86_64::register::cr4::read();
    rk_x86_64::register::cr4::write(cr4 | 1 << 4 | 1 << 5);

    // Write the address of our new PML4 into the CR3 register
    rk_x86_64::register::cr3::write(pml4_addr);

    // Write some pixels to the frame buffer
    let fb = 0x8000_0000 as *mut u32;
    for i in 0..40_000 {
        unsafe {
            fb.offset(i).write(0x00ff00ff);
        }
    }

    // Jump into the kernel
    unsafe {
        asm!("jmp {}", in(reg) kernel_entry);
    }

    // We should never get here because of the jump, but Rust doesn't know that and
    // we have to show that we have a diverging function
    rk_x86_64::hang()
}

/// Loads the kernel ELF and returns the physical address.
///
/// The kernel is always loaded in a 2 MiB contiguous page block.
fn load_kernel_elf(image: EfiHandle) -> EfiPhysicalAddress {
    let root = unsafe { get_volume_root(image).as_ref().unwrap() };

    // RK_KERNEL.ELF
    const FILE_NAME: [Char16; 14] = [
        Char16(0x0052),
        Char16(0x004b),
        Char16(0x005f),
        Char16(0x004b),
        Char16(0x0045),
        Char16(0x0052),
        Char16(0x004e),
        Char16(0x0045),
        Char16(0x004c),
        Char16(0x002e),
        Char16(0x0045),
        Char16(0x004c),
        Char16(0x0046),
        Char16(0),
    ];

    // Get a file handle for the kernel
    let mut ptr = core::ptr::null_mut();
    let status = root.open(
        &mut ptr,
        &FILE_NAME[0],
        EFI_FILE_MODE_READ,
        EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM,
    );
    if status.is_error() {
        panic!("Could not open RK_KERNEL.ELF: {:?}", status);
    }
    let file_handle = unsafe { &*(ptr as *const EfiFileProtocol) };

    let file_size = file_handle.file_size().expect("Could not get kernel size");

    println!("Kernel ELF Size = {} bytes", file_size);

    // Panic if the kernel ELF is larger than 2 MiB
    if file_size > 2 * 1024 * 1024 {
        panic!("Kernel ELF too big, max 2 MiB");
    }

    // Allocate pages for a 2 MiB contiguous block. Recall that each UEFI page is
    // always 4096 KiB, so we need 512 pages.
    let kernel_addr = system_table()
        .boot_services()
        .allocate_pages(
            EfiAllocateType::AllocateAnyPages,
            EfiMemoryType::EfiLoaderData,
            512,
        )
        .expect("Could not allocate page memory for the kernel ELF");

    let buffer = unsafe { &mut *(kernel_addr.0 as *mut core::ffi::c_void) };
    let mut size: usize = 2 * 1024 * 1024;
    let status = file_handle.read(&mut size, buffer);
    if status.is_error() {
        panic!("Could not read kernel ELF");
    }

    kernel_addr
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);

    rk_x86_64::hang()
}

fn get_volume_root(image: EfiHandle) -> *mut EfiFileProtocol {
    let mut ptr1 = core::ptr::null_mut();
    system_table().boot_services().handle_protocol(
        image,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        &mut ptr1,
    );
    let loaded_image = unsafe { &*(ptr1 as *mut EfiLoadedImageProtocol) };

    let mut ptr2 = core::ptr::null_mut();
    system_table().boot_services().handle_protocol(
        loaded_image.device_handle(),
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        &mut ptr2,
    );
    let volume = unsafe { &*(ptr2 as *mut EfiSimpleFileSystemProtocol) };

    let mut ptr3 = core::ptr::null_mut();
    volume.open_volume(&mut ptr3);
    ptr3
}

#[allow(dead_code)]
fn read_test_file(image: EfiHandle) {
    let root = unsafe { get_volume_root(image).as_ref().unwrap() };

    const TEST_FILE_NAME: [Char16; 9] = [
        Char16(0x0054),
        Char16(0x0045),
        Char16(0x0053),
        Char16(0x0054),
        Char16(0x002e),
        Char16(0x0054),
        Char16(0x0058),
        Char16(0x0054),
        Char16(0x0),
    ];

    // Print the name of the file
    print!("\nLoading File ");
    system_table().con_out().output_string(&TEST_FILE_NAME[0]);
    print!("\n");

    let mut ptr = core::ptr::null_mut();
    let s1 = root.open(
        &mut ptr,
        &TEST_FILE_NAME[0],
        EFI_FILE_MODE_READ,
        EFI_FILE_READ_ONLY | EFI_FILE_HIDDEN | EFI_FILE_SYSTEM,
    );
    if s1.is_error() {
        panic!("Could not open file: {:?}", s1);
    }
    let file_handle = unsafe { &*(ptr as *const EfiFileProtocol) };

    let file_size = file_handle.file_size().expect("Could not get file size");
    println!("File Size = {} bytes", file_size);

    // Read
    let mut size: usize = file_size.try_into().unwrap();
    let buffer = system_table()
        .boot_services()
        .allocate_pool(EfiMemoryType::EfiLoaderData, size)
        .expect("Could not allocate read buffer");
    let rs = file_handle.read(&mut size, unsafe { &mut *buffer });
    println!("Read status = {:?}", rs);
    println!("Read size = {}", size);

    // Close
    let cs = file_handle.close();
    println!("Close status = {:?}", cs);

    // Print content
    print!("PRINTING FILE CONTENTS ==");
    system_table()
        .con_out()
        .output_string(unsafe { &*(buffer as *const Char16) });
    println!("==");

    let fs = system_table().boot_services().free_pool(buffer);
    println!("Free buffer status = {:?}", fs);
}

#[allow(dead_code)]
fn print_available_graphics_modes() {
    let mut ptr = core::ptr::null_mut();
    let r1 = system_table().boot_services().locate_protocol(
        &EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID,
        core::ptr::null_mut(),
        &mut ptr,
    );
    if r1.is_error() {
        panic!("Unable to locate GOP");
    } else {
        println!("Located GOP");
    }

    // This should be safe as we got a success status code
    let gop = unsafe { &*(ptr as *mut EfiGraphicsOutputProtocol) };

    println!("Current mode: {}", gop.mode().mode);

    // Query info about each available graphics output mode and print it
    for i in 0..gop.mode().max_mode {
        let info = gop.query_mode(i).expect("Cannot get info");
        println!(
            "Mode {}, width {}, height {}",
            i, info.horizontal_resolution, info.vertical_resolution
        );
    }
}
