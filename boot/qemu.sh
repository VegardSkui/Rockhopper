#!/bin/sh
make
qemu-system-x86_64 -drive format=raw,file=diskimage.dd
