#!/bin/sh
cargo build
rm -rf disk
mkdir -p disk/EFI/BOOT
cp target/x86_64-unknown-uefi/debug/uefi-rust.efi disk/EFI/BOOT/BOOTX64.EFI
qemu-system-x86_64 \
    -nodefaults \
    -cpu qemu64 \
    -bios bios64.bin \
    -drive format=raw,file=fat:rw:disk \
    -serial stdio \
    -vga std
