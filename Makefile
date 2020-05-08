.PHONY: all clean

all: boot.bin

%.bin: %.asm
	nasm -fbin $< -o $@

clean:
	rm -f boot.bin
