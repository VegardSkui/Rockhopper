#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;
use rk_uefi::data_types::{EfiHandle, EfiMemoryType, EfiStatus};
use rk_uefi::guid::{
    EFI_GRAPHICS_OUTPUT_PROTOCOL_GUID, EFI_LOADED_IMAGE_PROTOCOL_GUID,
    EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
};
use rk_uefi::protocol::{
    EfiFileProtocol, EfiFileSystemInfo, EfiGraphicsOutputProtocol, EfiLoadedImageProtocol,
    EfiSimpleFileSystemProtocol, EFI_FILE_SYSTEM_INFO_ID,
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

    println!("\nStalling for 1 second");
    rk_uefi::system_table().boot_services().stall(1_000_000);
    println!("Done!");

    hang()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print!("{}", info);

    hang()
}

/// Halt the CPU forever.
#[inline]
pub fn hang() -> ! {
    loop {
        unsafe { asm!("hlt") }
    }
}

fn get_volume_root(image: EfiHandle) -> *mut EfiFileProtocol {
    let mut ptr1 = core::ptr::null_mut();
    let s1 = system_table().boot_services().handle_protocol(
        image,
        &EFI_LOADED_IMAGE_PROTOCOL_GUID,
        &mut ptr1,
    );
    let loaded_image = unsafe { &*(ptr1 as *mut EfiLoadedImageProtocol) };
    println!("LOADED IMAGE STATUS = {:?}", s1);

    let mut ptr2 = core::ptr::null_mut();
    let s2 = system_table().boot_services().handle_protocol(
        loaded_image.device_handle(),
        &EFI_SIMPLE_FILE_SYSTEM_PROTOCOL_GUID,
        &mut ptr2,
    );
    let volume = unsafe { &*(ptr2 as *mut EfiSimpleFileSystemProtocol) };
    println!("SIMPLE FILE SYSTEM STATUS = {:?}", s2);

    let mut ptr3 = core::ptr::null_mut();
    let s3 = volume.open_volume(&mut ptr3);
    println!("OPEN VOLUME STATUS = {:?}", s3);

    ptr3
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
