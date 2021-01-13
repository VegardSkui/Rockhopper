//
//                            x86-64 Memory Map
//
//        Lower Half
// (0)    0x0000_0000_0000_0000 - 0x0000_7FFF_FFFF_FFFF    (128 TiB)
//
//        Higher Half
// (1)    0xFFFF_8000_0000_0000 - 0xFFFF_807F_FFFF_FFFF    (512 GiB)
// (2)    0xFFFF_8080_0000_0000 - 0xFFFF_FF7F_FFFF_FFFF    (127 TiB)
// (3)    0xFFFF_FF80_0000_0000 - 0xFFFF_FFFF_FFFF_FFFF    (512 GiB)
//
// (0)    Free
// (1)    Physical Memory Mapping
// (2)    Free
// (3)    Kernel                       (Last entry of PML4)
//
