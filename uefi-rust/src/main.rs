#![no_std]
#![no_main]
#![feature(asm)]

use core::convert::TryInto;
use core::panic::PanicInfo;
use rk_uefi::data_types::{Char16, EfiHandle, EfiMemoryType, EfiStatus};
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

    read_test_file(image_handle);

    println!("\nStalling for 1 second");
    rk_uefi::system_table().boot_services().stall(1_000_000);
    println!("Done!");

    rk_x86_64::hang()
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
