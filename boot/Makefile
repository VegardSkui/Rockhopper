.PHONY: all clean

all: boot.bin stage2.bin diskimage.dd

diskimage.dd: boot.bin stage2.bin
	dd if=/dev/zero of=diskimage.dd bs=1024 count=0 seek=1024
	dd if=boot.bin of=diskimage.dd conv=notrunc bs=512 count=1
	dd if=stage2.bin of=diskimage.dd conv=notrunc bs=512 seek=1024

%.bin: %.asm
	nasm -MD -fbin $< -o $@

clean:
	rm -f *.bin
	rm -f *.bin.d
	rm -f diskimage.dd

-include *.d
