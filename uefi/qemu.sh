#!/bin/sh
make
qemu-system-x86_64 \
    -cpu qemu64 \
    -bios bios64.bin \
    -net none \
    -drive format=raw,file=uefi.cdr,if=ide
