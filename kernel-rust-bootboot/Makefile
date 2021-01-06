.PHONY: qemu clean

rockhopper.x86_64.elf: src/**
	cargo build
	cp target/x86_64-rockhopper/debug/kernel-rust-bootboot rockhopper.x86_64.elf

disk/BOOTBOOT/INITRD: rockhopper.x86_64.elf
	mkdir -p disk/BOOTBOOT
	tar -c rockhopper.x86_64.elf > disk/BOOTBOOT/INITRD

qemu: disk/BOOTBOOT/INITRD
	qemu-system-x86_64 \
		-nodefaults \
		-cpu qemu64 \
		-drive if=pflash,format=raw,file=OVMF_CODE-pure-efi.fd,readonly=on \
		-drive if=pflash,format=raw,file=OVMF_VARS-pure-efi.fd,readonly=on \
		-drive format=raw,file=fat:rw:disk \
		-serial stdio \
		-vga std

clean:
	#rm -rf target
	rm -f rockhopper.x86_64.elf
	rm -f disk/BOOTBOOT/INITRD
