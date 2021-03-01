.PHONY: all disk qemu fmt clean

LIBS = $(shell find libs/*/src -type f)

all: disk

disk/EFI/BOOT/BOOTX64.EFI: $(shell find bootloader/src -type f) $(LIBS)
	mkdir -p disk/EFI/BOOT
	cd bootloader && cargo build
	cp bootloader/target/x86_64-unknown-uefi/debug/rk_bootloader.efi disk/EFI/BOOT/BOOTX64.EFI

disk/RK_KERNEL.ELF: $(shell find kernel/src -type f) $(LIBS) kernel/font.o
	mkdir -p disk
	cd kernel && cargo build
	cp kernel/target/x86_64-rockhopper/debug/rk_kernel disk/RK_KERNEL.ELF

disk/TEST.TXT:
	mkdir -p disk
	cp test.txt disk/TEST.TXT

kernel/font.o: vendor/terminus-font/ter-116n.psf
	cp vendor/terminus-font/ter-116n.psf kernel/font.psf
	cd kernel && llvm-objcopy --input-target binary --output-target elf64-x86-64 font.psf font.o

disk: disk/EFI/BOOT/BOOTX64.EFI disk/RK_KERNEL.ELF disk/TEST.TXT

qemu: disk
	qemu-system-x86_64 \
		-nodefaults \
		-cpu qemu64 \
		-drive if=pflash,format=raw,file=OVMF_CODE-pure-efi.fd,readonly=on \
		-drive if=pflash,format=raw,file=OVMF_VARS-pure-efi.fd,readonly=on \
		-drive format=raw,file=fat:rw:disk \
		-serial file:serial \
		-vga std \
		-monitor stdio \
		-d int -no-reboot -no-shutdown

fmt:
	cd bootloader && cargo fmt
	cd kernel && cargo fmt
	cd libs/rk_elf64 && cargo fmt
	cd libs/rk_uefi && cargo fmt
	cd libs/rk_x86_64 && cargo fmt

clean:
	rm -rf disk
	#rm -rf bootloader/target
	#rm -rf kernel/target
	rm -f kernel/font.psf
	rm -f kernel/font.o
	rm -f serial
