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

all: main.efi uefi.cdr

main.o: main.c
	clang $(CFLAGS) -c -o main.o main.c

main.efi: main.o
	clang $(LDFLAGS) -o MAIN.EFI main.o

uefi.cdr: main.efi
	mkdir -p diskimage/EFI/BOOT
	cp main.efi diskimage/EFI/MAIN.EFI
	hdiutil create -fs fat32 -ov -size 48m -volname ROCKHOPPER -format UDTO -srcfolder diskimage uefi.cdr

clean:
	rm -f *.o
	rm -f *.efi
	rm -rf diskimage
	rm -f uefi.cdr
