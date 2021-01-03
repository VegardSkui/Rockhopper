.PHONY: all clean

CFLAGS = \
	-target x86_64-unknown-windows \
	-ffreestanding \
	-fshort-wchar \
	-mno-red-zone \
	-Ignu-efi/inc \
	-Ignu-efi/inc/x86_64

LDFLAGS = \
	-target x86_64-unknown-windows \
	-nostdlib \
	-Wl,-entry:efi_main \
	-Wl,-subsystem:efi_application \
	-fuse-ld=lld-link

all: uefi.cdr

%.o: %.c
	clang $(CFLAGS) -c -o $@ $<

hello.efi: hello.o
	clang $(LDFLAGS) -o $@ $<

main.efi: main.o utils.o
	clang $(LDFLAGS) -o main.efi main.o utils.o

uefi.cdr: hello.efi main.efi
	mkdir -p diskimage/EFI/BOOT
	mkdir -p diskimage/ROCKHOPPER
	cp main.efi diskimage/EFI/BOOT/BOOTX64.EFI
	cp hello.efi diskimage/ROCKHOPPER/HELLO.EFI
	hdiutil create -fs fat32 -ov -size 48m -volname ROCKHOPPER -format UDTO -srcfolder diskimage uefi.cdr

clean:
	rm -f *.o
	rm -f *.efi
	rm -rf diskimage
	rm -f uefi.cdr
