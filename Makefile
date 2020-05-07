.PHONY: all clean

all:
	nasm boot.asm -o boot.bin

clean:
	rm -f boot.bin
