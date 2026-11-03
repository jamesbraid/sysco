# ARM Linux 2.6 PIO and scan record routines in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions are unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-pio-scan-records-facts.txt) retain all tool sections.

## `PIOWrite_Common`

- `PIOWrite_Common`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1415e8 to 0x141d8c.
  Established. Word stores place 0xd0000000 and 0x50000000 at entry sp minus 0x40 and 0x44 respectively (`str` at 0x14172c, `str` at 0x141734).
  A later word store writes 0 to absolute address 0x3151c4 (`str` at 0x141ba8).
  A separate load from that absolute address is incremented by 1 and stored there as a word (`ldr` at 0x141be8, `add` at 0x141bec, `str` at 0x141bf0).
  The computed word store at 0x141bdc writes 0 to an unknown effective target (`str` at 0x141bdc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1415f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1415fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x141600 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x141604 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x141608 | load | 4 | `abs 0x141d54` | `-` |
| `ldr` at 0x14160c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14162c | load | 4 | `(unknown)` | `-` |
| `str` at 0x141630 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x141634 | load | 4 | `abs 0x141d54` | `-` |
| `ldr` at 0x141638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141658 | load | 4 | `(unknown)` | `-` |
| `str` at 0x14165c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x141660 | load | 4 | `abs 0x141d54` | `-` |
| `ldr` at 0x141664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14168c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x141694 | load | 1 | `(unknown)` | `-` |
| `str` at 0x141698 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x14169c | load | 4 | `abs 0x141d54` | `-` |
| `ldr` at 0x1416a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1416c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1416c8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x1416d0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x1416d4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x1416e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x1416e8 | load | 4 | `abs 0x141d54` | `-` |
| `ldr` at 0x1416ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14170c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141714 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x14171c | load | 1 | `(unknown)` | `-` |
| `str` at 0x14172c | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0x141734 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0x141740 | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0x141748 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0x14174c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x141750 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0x141758 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x14175c | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0x141760 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141764 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x141768 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x141774 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x141778 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x14177c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141780 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141784 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141788 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x14178c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141790 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x1417b8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x1417c0 | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x1417c4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1417d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1417d4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x1417d8 | load | 4 | `abs 0x141d5c` | `-` |
| `ldr` at 0x1417dc | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x1417e0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1417e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1417ec | load | 4 | `sp +0x4` | `-` |
| `str` at 0x1417f0 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0x1417f4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x1417f8 | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0x141818 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x14181c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141820 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141824 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x141828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141838 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14183c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x141840 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141844 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141848 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14184c | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x141850 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141854 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x14187c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141884 | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x141888 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141894 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141898 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x14189c | load | 4 | `abs 0x141d64` | `-` |
| `ldr` at 0x1418a0 | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x1418a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1418a8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1418b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1418b4 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0x1418bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1418c0 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0x1418c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1418c8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x1418cc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x1418d8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x1418dc | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x1418e0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1418e4 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x1418e8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1418ec | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x1418f0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1418f4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x14191c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141924 | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x141928 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141934 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141938 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x14193c | load | 4 | `abs 0x141d68` | `-` |
| `ldr` at 0x141940 | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x141944 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141948 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x141950 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x14195c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141960 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141968 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x14196c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141974 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141978 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x14197c | load | 4 | `abs 0x141d6c` | `-` |
| `ldr` at 0x141988 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141990 | load | 4 | `abs 0x141d6c` | `-` |
| `str` at 0x14199c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1419a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1419a4 | load | 4 | `abs 0x141d70` | `-` |
| `ldr` at 0x1419b0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1419b8 | load | 4 | `abs 0x141d70` | `-` |
| `str` at 0x1419c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1419c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1419cc | load | 4 | `abs 0x141d74` | `-` |
| `ldr` at 0x1419d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1419e0 | load | 4 | `abs 0x141d74` | `-` |
| `str` at 0x1419ec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1419f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141a00 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141a14 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141a18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141a28 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141a3c | store | 4 | `sp -0x38` | `?` |
| `str` at 0x141a48 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x141a4c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141a50 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141a54 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x141a58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141a68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x141a6c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x141a70 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141a74 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141a78 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141a7c | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x141a80 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141a84 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x141aac | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141ab4 | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x141ab8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141ac4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141ac8 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x141acc | load | 4 | `abs 0x141d78` | `-` |
| `ldr` at 0x141ad0 | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x141ad4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x141ad8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x141ae0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x141af0 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x141af8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x141afc | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0x141b04 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141b08 | store | 4 | `sp -0x68` | `[sp+8]` |
| `ldr` at 0x141b0c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141b10 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x141b14 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x141b20 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x141b24 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x141b28 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141b2c | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141b30 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141b34 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x141b38 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141b3c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x141b64 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141b6c | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x141b70 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141b7c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141b80 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x141b84 | load | 4 | `abs 0x141d5c` | `-` |
| `ldr` at 0x141b88 | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x141b8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141b90 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141b9c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x141ba0 | load | 4 | `abs 0x141d7c` | `-` |
| `str` at 0x141ba8 | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x141bac | load | 4 | `abs 0x141d7c` | `-` |
| `ldr` at 0x141bb0 | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x141bbc | load | 4 | `abs 0x141d7c` | `-` |
| `ldr` at 0x141bc0 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x141bdc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x141be0 | load | 4 | `abs 0x141d7c` | `-` |
| `ldr` at 0x141be4 | load | 4 | `abs 0x141d7c` | `-` |
| `ldr` at 0x141be8 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x141bf0 | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x141c00 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x141c04 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141c08 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141c0c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x141c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x141c28 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x141c34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x141c38 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x141c3c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141c40 | store | 4 | `sp -0x64` | `[sp+8]` |
| `ldr` at 0x141c44 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141c48 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x141c4c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141c50 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `str` at 0x141c78 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141c80 | load | 4 | `abs 0x141d58` | `-` |
| `ldr` at 0x141c84 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141c90 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141c94 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x141c98 | load | 4 | `abs 0x141d80` | `-` |
| `ldr` at 0x141c9c | load | 4 | `abs 0x141d60` | `-` |
| `ldr` at 0x141ca0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x141ca4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x141cac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141cb8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x141ccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141cd8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x141cec | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x141cf0 | load | 4 | `abs 0x141d84` | `-` |
| `str` at 0x141cfc | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x141d08 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x141d0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x141d14 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0x141d1c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x141d28 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x141d2c | load | 4 | `abs 0x141d88` | `-` |
| `str` at 0x141d38 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x141d44 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x141d48 | load | 4 | `sp -0x58` | `-` |
| `ldm` at 0x141d50 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x141d50 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x141d50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1416e4 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0x141770 | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x1417a4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x1417e8 | `(unknown callee) 0x13e868` | `0x19272c` | `0x19273c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0x14180c | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x141834 | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0x141868 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1418ac | `(unknown callee) 0x13e868` | `0x1927d4` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x1418d4 | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x141908 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x14194c | `(unknown callee) 0x13e868` | `0x1927e4` | `0x19273c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0x141a64 | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0x141a98 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x141adc | `(unknown callee) 0x13e868` | `0x1927c4` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x141b1c | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x141b50 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x141b94 | `(unknown callee) 0x13e868` | `0x19272c` | `0x19273c` | `[sp-76]` | `[sp+8]` |
| `bl` at 0x141c18 | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0x141c64 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x141ca8 | `(unknown callee) 0x13e868` | `0x192740` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x141cbc | `(unknown callee) 0x131fb0` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0x141cdc | `(unknown callee) 0x131fb0` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0x141cf4 | `(unknown callee) 0x20634` | `[sp+16]` | `0x192758` | `?` | `ret(0x131fb0)` |
| `bl` at 0x141d30 | `(unknown callee) 0x20634` | `[sp+16]` | `0x19278c` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x141724 | `? != 0x6` | `? == 0x6` |
| `bne` at 0x1417b0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1417cc | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x141874 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141890 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x141914 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141930 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x141aa4 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141ac0 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x141ae8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x141b5c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141b78 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x141bb8 | `[0x3151c4] > 0x2 (unsigned, 32-bit)` | `[0x3151c4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0x141c24 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0x141c30 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0x141c70 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141c8c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x141cc8 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x141ce8 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x141d24 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x141d50 | `r0` | `?` |

## `PIORead_Common`

- `PIORead_Common`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x141d8c to 0x142420.
  Established. Separate word stores write 0 to the addresses held in [sp+4] and [sp] (`str` at 0x141efc, `str` at 0x141f04).
  A later word store writes [sp-68] to the address held in [sp+4] (`str` at 0x14238c).
  Another writes [sp-72] as a word to the address held in [sp] (`str` at 0x142398).
  The return record retains an unknown value in r0 (`ldm` at 0x1423e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x141d9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x141da0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x141da4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x141da8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x141dac | load | 4 | `abs 0x1423ec` | `-` |
| `ldr` at 0x141db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141dd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x141dd4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x141dd8 | load | 4 | `abs 0x1423ec` | `-` |
| `ldr` at 0x141ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141dfc | load | 4 | `(unknown)` | `-` |
| `str` at 0x141e00 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x141e04 | load | 4 | `abs 0x1423ec` | `-` |
| `ldr` at 0x141e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141e30 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x141e38 | load | 1 | `(unknown)` | `-` |
| `str` at 0x141e3c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x141e40 | load | 4 | `abs 0x1423ec` | `-` |
| `ldr` at 0x141e44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141e64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141e6c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x141e74 | load | 2 | `(unknown)` | `-` |
| `str` at 0x141e78 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x141e84 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x141e8c | load | 4 | `abs 0x1423ec` | `-` |
| `ldr` at 0x141e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141eb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x141eb8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x141ec0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x141ed0 | store | 4 | `sp -0x50` | `0xd0000000` |
| `str` at 0x141ed8 | store | 4 | `sp -0x54` | `0x50000000` |
| `str` at 0x141ee4 | store | 4 | `sp -0x50` | `0x8000000` |
| `str` at 0x141eec | store | 4 | `sp -0x54` | `0xf0000000` |
| `ldr` at 0x141ef0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x141ef4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x141efc | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0x141f04 | store | 4 | `[sp] +0x0` | `0x0` |
| `ldr` at 0x141f08 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x141f0c | store | 4 | `sp -0x38` | `[sp-84]` |
| `ldr` at 0x141f14 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141f18 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0x141f1c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x141f20 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x141f24 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x141f30 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x141f34 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0x141f38 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x141f3c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x141f40 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141f44 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x141f48 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141f4c | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0x141f74 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x141f7c | load | 4 | `abs 0x1423f0` | `-` |
| `ldr` at 0x141f80 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141f8c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141f90 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0x141f94 | load | 4 | `abs 0x1423f4` | `-` |
| `ldr` at 0x141f98 | load | 4 | `abs 0x1423f8` | `-` |
| `ldr` at 0x141f9c | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x141fa0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x141fa8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x141fb4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141fb8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141fc0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141fc4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141fc8 | load | 4 | `abs 0x1423fc` | `-` |
| `ldr` at 0x141fd4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141fdc | load | 4 | `abs 0x1423fc` | `-` |
| `str` at 0x141fe8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141fec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141ff0 | load | 4 | `abs 0x142400` | `-` |
| `ldr` at 0x141ffc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x142004 | load | 4 | `abs 0x142400` | `-` |
| `str` at 0x142010 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x142014 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x142018 | load | 4 | `abs 0x142404` | `-` |
| `ldr` at 0x142024 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x14202c | load | 4 | `abs 0x142404` | `-` |
| `str` at 0x142038 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x14203c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x14204c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x142060 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x142064 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x142074 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x142088 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x142094 | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0x142098 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x14209c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x1420a0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x1420a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1420b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1420b8 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0x1420bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1420c0 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x1420c4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1420c8 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x1420cc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1420d0 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0x1420f8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x142100 | load | 4 | `abs 0x1423f0` | `-` |
| `ldr` at 0x142104 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142110 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142114 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0x142118 | load | 4 | `abs 0x142408` | `-` |
| `ldr` at 0x14211c | load | 4 | `abs 0x1423f8` | `-` |
| `ldr` at 0x142120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x142124 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x14212c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x14213c | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0x142144 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x142148 | store | 4 | `sp -0x38` | `[sp-80]` |
| `ldr` at 0x142150 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142154 | store | 4 | `sp -0x78` | `[sp+8]` |
| `ldr` at 0x142158 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x14215c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x142160 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x14216c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x142170 | store | 4 | `sp -0x78` | `[sp-92]` |
| `ldr` at 0x142174 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142178 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x14217c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142180 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142184 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142188 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0x1421b0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x1421b8 | load | 4 | `abs 0x1423f0` | `-` |
| `ldr` at 0x1421bc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1421c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1421cc | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0x1421d0 | load | 4 | `abs 0x14240c` | `-` |
| `ldr` at 0x1421d4 | load | 4 | `abs 0x1423f8` | `-` |
| `ldr` at 0x1421d8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x1421dc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1421e8 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x1421ec | load | 4 | `abs 0x142410` | `-` |
| `str` at 0x1421f4 | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x1421f8 | load | 4 | `abs 0x142410` | `-` |
| `ldr` at 0x1421fc | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x142208 | load | 4 | `abs 0x142410` | `-` |
| `ldr` at 0x14220c | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x142228 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x14222c | load | 4 | `abs 0x142410` | `-` |
| `ldr` at 0x142230 | load | 4 | `abs 0x142410` | `-` |
| `ldr` at 0x142234 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x14223c | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x14224c | store | 4 | `sp -0x78` | `sp-36` |
| `ldr` at 0x142250 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142254 | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x142258 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x14225c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142268 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x142274 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x142280 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x142284 | store | 4 | `sp -0x78` | `[sp-36]` |
| `ldr` at 0x142288 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x14228c | store | 4 | `sp -0x74` | `[sp+8]` |
| `ldr` at 0x142290 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142294 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142298 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x14229c | store | 4 | `sp -0x6c` | `[sp+16]` |
| `str` at 0x1422c4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x1422cc | load | 4 | `abs 0x1423f0` | `-` |
| `ldr` at 0x1422d0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1422dc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1422e0 | store | 4 | `sp -0x78` | `[sp+12]` |
| `ldr` at 0x1422e4 | load | 4 | `abs 0x142414` | `-` |
| `ldr` at 0x1422e8 | load | 4 | `abs 0x1423f8` | `-` |
| `ldr` at 0x1422ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1422f0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1422f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142304 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x142318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142324 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x142338 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x14233c | load | 4 | `abs 0x142418` | `-` |
| `str` at 0x142348 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x142358 | store | 4 | `sp -0x78` | `0x42` |
| `ldr` at 0x14235c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142360 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x142384 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x142388 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x14238c | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0x142390 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x142394 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x142398 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0x1423a0 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x1423a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1423ac | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0x1423b4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1423c0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x1423c4 | load | 4 | `abs 0x14241c` | `-` |
| `str` at 0x1423d0 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x1423dc | store | 4 | `sp -0x68` | `0x1` |
| `ldr` at 0x1423e0 | load | 4 | `sp -0x68` | `-` |
| `ldm` at 0x1423e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1423e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1423e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x141e88 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0x141f2c | `(unknown callee) 0x13e4f0` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0x141f60 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x141fa4 | `(unknown callee) 0x13e868` | `0x1927f4` | `0x19273c` | `[sp-92]` | `[sp+8]` |
| `bl` at 0x1420b0 | `(unknown callee) 0x13e778` | `[sp-88]` | `[sp-24]` | `0x1e` | `sp-56` |
| `bl` at 0x1420e4 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142128 | `(unknown callee) 0x13e868` | `0x192804` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x142168 | `(unknown callee) 0x13e4f0` | `[sp-92]` | `[sp-100]` | `[sp-96]` | `sp-56` |
| `bl` at 0x14219c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1421e0 | `(unknown callee) 0x13e868` | `0x192814` | `0x19273c` | `[sp-92]` | `[sp+8]` |
| `bl` at 0x142264 | `(unknown callee) 0x13e778` | `[sp-88]` | `[sp-24]` | `0x42` | `sp-56` |
| `bl` at 0x1422b0 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1422f4 | `(unknown callee) 0x13e868` | `0x192824` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x142308 | `(unknown callee) 0x131fb0` | `[sp-24]+64` | `[sp+12]` | `?` | `[sp-24]+64` |
| `bl` at 0x142328 | `(unknown callee) 0x131fb0` | `[sp-24]+65` | `[sp+12]` | `?` | `[sp-24]+65` |
| `bl` at 0x142340 | `(unknown callee) 0x20634` | `[sp+16]` | `0x192758` | `?` | `ret(0x131fb0)` |
| `bl` at 0x14236c | `(unknown callee) 0x132444` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0x142380 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0x1423c8 | `(unknown callee) 0x20634` | `[sp+16]` | `0x19283c` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x141ec8 | `? != 0x6` | `? == 0x6` |
| `bne` at 0x141f6c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141f88 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x1420f0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x14210c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142134 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x1421a8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1421c4 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x142204 | `[0x3151c4] > 0x2 (unsigned, 32-bit)` | `[0x3151c4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0x142270 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0x14227c | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0x1422bc | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1422d8 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x142314 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x142334 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x1423bc | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1423e8 | `r0` | `?` |

## `PIOWrite_PIC`

- `PIOWrite_PIC`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x142420 to 0x142b38.
  Established. A word store writes 0x48000000 at entry sp minus 0x38 (`str` at 0x142524).
  A later word store writes 0x28000000 at that same destination (`str` at 0x142688).
  Two subsequent stores there write 0 and 0x01000000 as words (`str` at 0x142728, `str` at 0x142734).
  Another word store writes 0x08000000 at entry sp minus 0x38 (`str` at 0x1427f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x142430 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x142434 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x142438 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x14243c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x142440 | load | 4 | `abs 0x142b08` | `-` |
| `ldr` at 0x142444 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142464 | load | 4 | `(unknown)` | `-` |
| `str` at 0x142468 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x14246c | load | 4 | `abs 0x142b08` | `-` |
| `ldr` at 0x142470 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142490 | load | 4 | `(unknown)` | `-` |
| `str` at 0x142494 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x142498 | load | 4 | `abs 0x142b08` | `-` |
| `ldr` at 0x14249c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1424bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1424c4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x1424cc | load | 1 | `(unknown)` | `-` |
| `str` at 0x1424d0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x1424d4 | load | 4 | `abs 0x142b08` | `-` |
| `ldr` at 0x1424d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1424f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142500 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x142508 | load | 2 | `(unknown)` | `-` |
| `str` at 0x14250c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x142518 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142524 | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0x14252c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142530 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0x142534 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x142538 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x14253c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x142548 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x14254c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x142550 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142554 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x142558 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14255c | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x142560 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142564 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x14258c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x142594 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x142598 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1425a4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1425a8 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x1425ac | load | 4 | `abs 0x142b10` | `-` |
| `ldr` at 0x1425b0 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x1425b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1425b8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1425c0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x1425c4 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0x1425c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x1425cc | store | 4 | `sp -0x38` | `[sp]` |
| `str` at 0x1425ec | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0x1425f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1425f4 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x1425f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1425fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14260c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x142610 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0x142614 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142618 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x14261c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142620 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x142624 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142628 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x142650 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x142658 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x14265c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142668 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14266c | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x142670 | load | 4 | `abs 0x142b18` | `-` |
| `ldr` at 0x142674 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x142678 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x14267c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142688 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0x142690 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142694 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0x142698 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x14269c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1426a0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x1426ac | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1426b0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x1426b4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1426b8 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x1426bc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1426c0 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x1426c4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1426c8 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x1426f0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x1426f8 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x1426fc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142708 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14270c | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x142710 | load | 4 | `abs 0x142b1c` | `-` |
| `ldr` at 0x142714 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x142718 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x14271c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142728 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x14272c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x142734 | store | 4 | `sp -0x38` | `0x1000000` |
| `ldr` at 0x142738 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x14273c | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0x14275c | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0x142760 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142764 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x142768 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x14276c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14277c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x142780 | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0x142784 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142788 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x14278c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142790 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x142794 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142798 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x1427c0 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x1427c8 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x1427cc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1427d8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1427dc | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x1427e0 | load | 4 | `abs 0x142b20` | `-` |
| `ldr` at 0x1427e4 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x1427e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1427ec | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1427f8 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0x142800 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142804 | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0x142808 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x14280c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x142810 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x14281c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x142820 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x142824 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142828 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x14282c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142830 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x142834 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142838 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x142860 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x142868 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x14286c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142878 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14287c | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x142880 | load | 4 | `abs 0x142b24` | `-` |
| `ldr` at 0x142884 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x142888 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x14288c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x142894 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1428a4 | store | 4 | `sp -0x50` | `0x1` |
| `str` at 0x1428b0 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0x1428b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1428bc | store | 4 | `sp -0x60` | `[sp+8]` |
| `ldr` at 0x1428c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x1428c4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1428c8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x1428d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1428d8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x1428dc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1428e0 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x1428e4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1428e8 | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x1428ec | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1428f0 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x142918 | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x142920 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x142924 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142930 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142934 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x142938 | load | 4 | `abs 0x142b1c` | `-` |
| `ldr` at 0x14293c | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x142940 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x142944 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142950 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x142954 | load | 4 | `abs 0x142b28` | `-` |
| `str` at 0x14295c | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x142960 | load | 4 | `abs 0x142b28` | `-` |
| `ldr` at 0x142964 | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x142970 | load | 4 | `abs 0x142b28` | `-` |
| `ldr` at 0x142974 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x142990 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x142994 | load | 4 | `abs 0x142b28` | `-` |
| `ldr` at 0x142998 | load | 4 | `abs 0x142b28` | `-` |
| `ldr` at 0x14299c | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x1429a4 | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x1429b4 | store | 4 | `sp -0x60` | `sp-36` |
| `ldr` at 0x1429b8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1429bc | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x1429c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1429c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1429d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1429dc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1429e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1429ec | store | 4 | `sp -0x60` | `[sp-36]` |
| `ldr` at 0x1429f0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1429f4 | store | 4 | `sp -0x5c` | `[sp+8]` |
| `ldr` at 0x1429f8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1429fc | store | 4 | `sp -0x58` | `[sp+12]` |
| `ldr` at 0x142a00 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142a04 | store | 4 | `sp -0x54` | `[sp+16]` |
| `str` at 0x142a2c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0x142a34 | load | 4 | `abs 0x142b0c` | `-` |
| `ldr` at 0x142a38 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142a44 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142a48 | store | 4 | `sp -0x60` | `[sp+12]` |
| `ldr` at 0x142a4c | load | 4 | `abs 0x142b2c` | `-` |
| `ldr` at 0x142a50 | load | 4 | `abs 0x142b14` | `-` |
| `ldr` at 0x142a54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x142a58 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x142a60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142a6c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x142a80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142a8c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x142aa0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x142aa4 | load | 4 | `abs 0x142b30` | `-` |
| `str` at 0x142ab0 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x142abc | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x142ac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x142ac8 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0x142ad0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x142adc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x142ae0 | load | 4 | `abs 0x142b34` | `-` |
| `str` at 0x142aec | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x142af8 | store | 4 | `sp -0x50` | `0x1` |
| `ldr` at 0x142afc | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0x142b04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x142b04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x142b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x14251c | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0x142544 | `(unknown callee) 0x13e4f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0x142578 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x1425bc | `(unknown callee) 0x13e868` | `0x192874` | `0x19273c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0x1425e0 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x142608 | `(unknown callee) 0x13e778` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x14263c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142680 | `(unknown callee) 0x13e868` | `0x192888` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x1426a8 | `(unknown callee) 0x13e4f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0x1426dc | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142720 | `(unknown callee) 0x13e868` | `0x19289c` | `0x19273c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0x142750 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x142778 | `(unknown callee) 0x13e778` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x1427ac | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1427f0 | `(unknown callee) 0x13e868` | `0x1928b0` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x142818 | `(unknown callee) 0x13e4f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0x14284c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142890 | `(unknown callee) 0x13e868` | `0x1928c4` | `0x19273c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0x1428d0 | `(unknown callee) 0x13e4f0` | `[sp-68]` | `[sp-76]` | `[sp-72]` | `sp-56` |
| `bl` at 0x142904 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142948 | `(unknown callee) 0x13e868` | `0x19289c` | `0x19273c` | `[sp-68]` | `[sp+8]` |
| `bl` at 0x1429cc | `(unknown callee) 0x13e778` | `[sp-64]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x142a18 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142a5c | `(unknown callee) 0x13e868` | `0x1928d8` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x142a70 | `(unknown callee) 0x131fb0` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0x142a90 | `(unknown callee) 0x131fb0` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0x142aa8 | `(unknown callee) 0x20634` | `[sp+16]` | `0x1928f4` | `?` | `ret(0x131fb0)` |
| `bl` at 0x142ae4 | `(unknown callee) 0x20634` | `[sp+16]` | `0x19292c` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x142584 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1425a0 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142648 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142664 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x1426e8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142704 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x1427b8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1427d4 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142858 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142874 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x14289c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x142910 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x14292c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x14296c | `[0x3151c4] > 0x1 (unsigned, 32-bit)` | `[0x3151c4] <= 0x1 (unsigned, 32-bit)` |
| `ble` at 0x1429d8 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0x1429e4 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0x142a24 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142a40 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x142a7c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x142a9c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x142ad8 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x142b04 | `r0` | `?` |

## `PIORead_PIC`

- `PIORead_PIC`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x142b38 to 0x143298.
  Established. A word store writes 0 to the address held in [sp+4] (`str` at 0x142c44).
  A separate word store writes 0 to the address held in [sp] (`str` at 0x142c4c).
  Later stores write [sp-68] to the address held in [sp+4] and [sp-72] to the address held in [sp], both as words (`str` at 0x143208, `str` at 0x143214).
  Another word store writes [sp-28] minus 1 at entry sp minus 0x1c (`str` at 0x143228).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x142b48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x142b4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x142b50 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x142b54 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x142b58 | load | 4 | `abs 0x143268` | `-` |
| `ldr` at 0x142b5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142b7c | load | 4 | `(unknown)` | `-` |
| `str` at 0x142b80 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x142b84 | load | 4 | `abs 0x143268` | `-` |
| `ldr` at 0x142b88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142ba8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x142bac | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x142bb0 | load | 4 | `abs 0x143268` | `-` |
| `ldr` at 0x142bb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142bdc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x142be4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x142be8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x142bec | load | 4 | `abs 0x143268` | `-` |
| `ldr` at 0x142bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x142c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142c18 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x142c20 | load | 2 | `(unknown)` | `-` |
| `str` at 0x142c24 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x142c30 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x142c38 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x142c3c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x142c44 | store | 4 | `[sp+4] +0x0` | `0x0` |
| `str` at 0x142c4c | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0x142c54 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0x142c5c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142c60 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0x142c64 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142c68 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x142c6c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x142c78 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x142c7c | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0x142c80 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142c84 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142c88 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142c8c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x142c90 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142c94 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x142cbc | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x142cc4 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x142cc8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142cd4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142cd8 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142cdc | load | 4 | `abs 0x143270` | `-` |
| `ldr` at 0x142ce0 | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x142ce4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142ce8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142cf4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x142cf8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x142cfc | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0x142d1c | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0x142d20 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142d24 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142d28 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x142d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142d3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x142d40 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0x142d44 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142d48 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142d4c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142d50 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x142d54 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142d58 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x142d80 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x142d88 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x142d8c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142d98 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142d9c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142da0 | load | 4 | `abs 0x143278` | `-` |
| `ldr` at 0x142da4 | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x142da8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x142dac | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142db8 | store | 4 | `sp -0x38` | `0x8000000` |
| `ldr` at 0x142dc0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142dc4 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0x142dc8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142dcc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x142dd0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x142ddc | load | 4 | `sp -0x54` | `-` |
| `str` at 0x142de0 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0x142de4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142de8 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142dec | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142df0 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x142df4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142df8 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x142e20 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x142e28 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x142e2c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142e38 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142e3c | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142e40 | load | 4 | `abs 0x14327c` | `-` |
| `ldr` at 0x142e44 | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x142e48 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142e4c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x142e54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x142e64 | store | 4 | `sp -0x60` | `0x1` |
| `str` at 0x142e70 | store | 4 | `sp -0x38` | `0x28000000` |
| `ldr` at 0x142e78 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142e7c | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0x142e80 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142e84 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x142e88 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x142e94 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x142e98 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0x142e9c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142ea0 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142ea4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142ea8 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x142eac | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142eb0 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x142ed8 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x142ee0 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x142ee4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x142ef0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142ef4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x142ef8 | load | 4 | `abs 0x143270` | `-` |
| `ldr` at 0x142efc | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x142f00 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x142f04 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142f10 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x142f14 | load | 4 | `abs 0x143280` | `-` |
| `str` at 0x142f1c | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x142f20 | load | 4 | `abs 0x143280` | `-` |
| `ldr` at 0x142f24 | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x142f30 | load | 4 | `abs 0x143280` | `-` |
| `ldr` at 0x142f34 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x142f50 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x142f54 | load | 4 | `abs 0x143280` | `-` |
| `ldr` at 0x142f58 | load | 4 | `abs 0x143280` | `-` |
| `ldr` at 0x142f5c | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x142f64 | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x142f74 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0x142f78 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142f7c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142f80 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x142f84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x142f90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x142f9c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x142fa8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x142fac | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0x142fb0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x142fb4 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x142fb8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x142fbc | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x142fc0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x142fc4 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x142fec | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x142ff4 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x142ff8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x143004 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x143008 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x14300c | load | 4 | `abs 0x143284` | `-` |
| `ldr` at 0x143010 | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x143014 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x143018 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x143020 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14302c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x143040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14304c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x143060 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x143064 | load | 4 | `abs 0x143288` | `-` |
| `str` at 0x143070 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x14307c | store | 4 | `sp -0x38` | `0x48000000` |
| `ldr` at 0x143084 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x143088 | store | 4 | `sp -0x70` | `[sp+8]` |
| `ldr` at 0x14308c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x143090 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x143094 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x1430a0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x1430a4 | store | 4 | `sp -0x70` | `[sp-84]` |
| `ldr` at 0x1430a8 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1430ac | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x1430b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1430b4 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x1430b8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1430bc | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x1430e4 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x1430ec | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x1430f0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1430fc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x143100 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x143104 | load | 4 | `abs 0x14328c` | `-` |
| `ldr` at 0x143108 | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x14310c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x143110 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x143134 | store | 4 | `sp -0x70` | `sp-36` |
| `ldr` at 0x143138 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x14313c | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x143140 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x143144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143154 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x143158 | store | 4 | `sp -0x70` | `[sp-36]` |
| `ldr` at 0x14315c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x143160 | store | 4 | `sp -0x6c` | `[sp+8]` |
| `ldr` at 0x143164 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x143168 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x14316c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x143170 | store | 4 | `sp -0x64` | `[sp+16]` |
| `str` at 0x143198 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x1431a0 | load | 4 | `abs 0x14326c` | `-` |
| `ldr` at 0x1431a4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1431b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1431b4 | store | 4 | `sp -0x70` | `[sp+12]` |
| `ldr` at 0x1431b8 | load | 4 | `abs 0x143290` | `-` |
| `ldr` at 0x1431bc | load | 4 | `abs 0x143274` | `-` |
| `ldr` at 0x1431c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1431c4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1431d4 | store | 4 | `sp -0x70` | `0x40` |
| `ldr` at 0x1431d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1431dc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x143200 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x143204 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x143208 | store | 4 | `[sp+4] +0x0` | `[sp-68]` |
| `ldr` at 0x14320c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x143210 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x143214 | store | 4 | `[sp] +0x0` | `[sp-72]` |
| `str` at 0x14321c | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0x143220 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x143228 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0x143230 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x14323c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x143240 | load | 4 | `abs 0x143294` | `-` |
| `str` at 0x14324c | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x143258 | store | 4 | `sp -0x60` | `0x1` |
| `ldr` at 0x14325c | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0x143264 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x143264 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x143264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x142c34 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0x142c74 | `(unknown callee) 0x13e4f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0x142ca8 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x142cec | `(unknown callee) 0x13e868` | `0x192968` | `0x19273c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0x142d10 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x142d38 | `(unknown callee) 0x13e778` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x142d6c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142db0 | `(unknown callee) 0x13e868` | `0x19297c` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x142dd8 | `(unknown callee) 0x13e4f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0x142e0c | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x142e50 | `(unknown callee) 0x13e868` | `0x192990` | `0x19273c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0x142e90 | `(unknown callee) 0x13e4f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0x142ec4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x142f08 | `(unknown callee) 0x13e868` | `0x192968` | `0x19273c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0x142f8c | `(unknown callee) 0x13e778` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x142fd8 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x14301c | `(unknown callee) 0x13e868` | `0x1929a4` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x143030 | `(unknown callee) 0x131fb0` | `[sp-24]+57` | `[sp+12]` | `?` | `[sp-24]+57` |
| `bl` at 0x143050 | `(unknown callee) 0x131fb0` | `[sp-24]+58` | `[sp+12]` | `?` | `[sp-24]+58` |
| `bl` at 0x143068 | `(unknown callee) 0x20634` | `[sp+16]` | `0x1928f4` | `?` | `ret(0x131fb0)` |
| `bl` at 0x14309c | `(unknown callee) 0x13e4f0` | `[sp-84]` | `[sp-92]` | `[sp-88]` | `sp-56` |
| `bl` at 0x1430d0 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x143114 | `(unknown callee) 0x13e868` | `0x1929c0` | `0x19273c` | `[sp-84]` | `[sp+8]` |
| `bl` at 0x143128 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x143150 | `(unknown callee) 0x13e778` | `[sp-80]` | `[sp-24]` | `0x40` | `sp-56` |
| `bl` at 0x143184 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1431c8 | `(unknown callee) 0x13e868` | `0x1929d4` | `0x19273c` | `[sp-36]` | `[sp+8]` |
| `bl` at 0x1431e8 | `(unknown callee) 0x132444` | `[sp-24]` | `[sp+12]` | `0x0` | `sp-72` |
| `bl` at 0x1431fc | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-72` | `sp-72` |
| `bl` at 0x143244 | `(unknown callee) 0x20634` | `[sp+16]` | `0x1929e8` | `?` | `[sp-76]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x142cb4 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142cd0 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142d78 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142d94 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142e18 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142e34 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x142e5c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x142ed0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x142eec | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x142f2c | `[0x3151c4] > 0x1 (unsigned, 32-bit)` | `[0x3151c4] <= 0x1 (unsigned, 32-bit)` |
| `ble` at 0x142f98 | `[sp-28] <= 0x0 (signed, 32-bit)` | `[sp-28] > 0x0 (signed, 32-bit)` |
| `bne` at 0x142fa4 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |
| `bne` at 0x142fe4 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x143000 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x14303c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x14305c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x1430dc | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1430f8 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x143190 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1431ac | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x143238 | `[sp-76] != 0x0` | `[sp-76] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x143264 | `r0` | `?` |

## `PIOWrite_Special`

- `PIOWrite_Special`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x140eac to 0x1415e8.
  Established. Word stores write 0xd0000000 and 0x50000000 at entry sp minus 0x40 and 0x44 respectively (`str` at 0x140ffc, `str` at 0x141004).
  A later word store writes 1 at entry sp minus 0x58 (`str` at 0x1411ac).
  A computed word store writes 0 to an unknown effective target (`str` at 0x141440).
  The word store at entry sp minus 0x20 writes [sp-32] minus 1 (`str` at 0x141578).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x140ebc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x140ec0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x140ec4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x140ec8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x140ecc | load | 4 | `abs 0x1415b8` | `-` |
| `ldr` at 0x140ed0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x140ef0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x140ef4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x140ef8 | load | 4 | `abs 0x1415b8` | `-` |
| `ldr` at 0x140efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x140f1c | load | 4 | `(unknown)` | `-` |
| `str` at 0x140f20 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x140f24 | load | 4 | `abs 0x1415b8` | `-` |
| `ldr` at 0x140f28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x140f48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x140f50 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x140f58 | load | 1 | `(unknown)` | `-` |
| `str` at 0x140f5c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x140f60 | load | 4 | `abs 0x1415b8` | `-` |
| `ldr` at 0x140f64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x140f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x140f8c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x140f94 | load | 2 | `(unknown)` | `-` |
| `str` at 0x140f98 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x140f9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x140fb0 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x140fb8 | load | 4 | `abs 0x1415b8` | `-` |
| `ldr` at 0x140fbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x140fdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x140fe4 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x140fec | load | 1 | `(unknown)` | `-` |
| `str` at 0x140ffc | store | 4 | `sp -0x40` | `0xd0000000` |
| `str` at 0x141004 | store | 4 | `sp -0x44` | `0x50000000` |
| `str` at 0x141010 | store | 4 | `sp -0x40` | `0x8000000` |
| `str` at 0x141018 | store | 4 | `sp -0x44` | `0xf0000000` |
| `ldr` at 0x14101c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x141028 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x14102c | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0x141034 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141038 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x14103c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141040 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x141044 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x141050 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x141054 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x141058 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14105c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x141060 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141064 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x141068 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x14106c | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x141094 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x14109c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x1410a0 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0x1410a4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x1410a8 | store | 4 | `sp -0x38` | `[sp+4]` |
| `str` at 0x1410c8 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x1410cc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1410d0 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x1410d4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x1410d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1410e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1410ec | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x1410f0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1410f4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x1410f8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1410fc | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x141100 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x141104 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x14112c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141134 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x141138 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0x141140 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141144 | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x141148 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x14114c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x141150 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x14115c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x141160 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x141164 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141168 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x14116c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141170 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x141174 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x141178 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x1411a0 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x1411ac | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x1411b4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x1411c0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1411c4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1411cc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1411d0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1411d8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x1411dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1411e0 | load | 4 | `abs 0x1415bc` | `-` |
| `ldr` at 0x1411ec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1411f4 | load | 4 | `abs 0x1415bc` | `-` |
| `str` at 0x141200 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141204 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141208 | load | 4 | `abs 0x1415c0` | `-` |
| `ldr` at 0x141214 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x14121c | load | 4 | `abs 0x1415c0` | `-` |
| `str` at 0x141228 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x14122c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141230 | load | 4 | `abs 0x1415c4` | `-` |
| `ldr` at 0x14123c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141244 | load | 4 | `abs 0x1415c4` | `-` |
| `str` at 0x141250 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x141254 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x141264 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x141278 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x14127c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x14128c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1412a0 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x1412ac | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x1412b0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1412b4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x1412b8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x1412bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1412cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1412d0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x1412d4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1412d8 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x1412dc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1412e0 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x1412e4 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x1412e8 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x141310 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x141318 | load | 4 | `abs 0x1415c8` | `-` |
| `ldr` at 0x14131c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x141328 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x14132c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x141330 | load | 4 | `abs 0x1415cc` | `-` |
| `ldr` at 0x141334 | load | 4 | `abs 0x1415d0` | `-` |
| `ldr` at 0x141338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x14133c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x141344 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x141354 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x14135c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x141360 | store | 4 | `sp -0x38` | `[sp-64]` |
| `ldr` at 0x141368 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14136c | store | 4 | `sp -0x68` | `[sp+12]` |
| `ldr` at 0x141370 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x141374 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x141378 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x141384 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x141388 | store | 4 | `sp -0x68` | `[sp-76]` |
| `ldr` at 0x14138c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141390 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x141394 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x141398 | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x14139c | load | 4 | `sp +0x14` | `-` |
| `str` at 0x1413a0 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x1413c8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x1413d0 | load | 4 | `abs 0x1415c8` | `-` |
| `ldr` at 0x1413d4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1413e0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1413e4 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x1413e8 | load | 4 | `abs 0x1415d4` | `-` |
| `ldr` at 0x1413ec | load | 4 | `abs 0x1415d0` | `-` |
| `ldr` at 0x1413f0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x1413f4 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x141400 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x141404 | load | 4 | `abs 0x1415d8` | `-` |
| `str` at 0x14140c | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x141410 | load | 4 | `abs 0x1415d8` | `-` |
| `ldr` at 0x141414 | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x141420 | load | 4 | `abs 0x1415d8` | `-` |
| `ldr` at 0x141424 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x141440 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x141444 | load | 4 | `abs 0x1415d8` | `-` |
| `ldr` at 0x141448 | load | 4 | `abs 0x1415d8` | `-` |
| `ldr` at 0x14144c | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x141454 | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x141464 | store | 4 | `sp -0x68` | `sp-36` |
| `ldr` at 0x141468 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x14146c | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x141470 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x141474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x141480 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x14148c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x141498 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x14149c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x1414a0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x1414a4 | store | 4 | `sp -0x64` | `[sp+12]` |
| `ldr` at 0x1414a8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1414ac | store | 4 | `sp -0x60` | `[sp+16]` |
| `ldr` at 0x1414b0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x1414b4 | store | 4 | `sp -0x5c` | `[sp+20]` |
| `str` at 0x1414dc | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0x1414e4 | load | 4 | `abs 0x1415c8` | `-` |
| `ldr` at 0x1414e8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1414f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1414f8 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x1414fc | load | 4 | `abs 0x1415dc` | `-` |
| `ldr` at 0x141500 | load | 4 | `abs 0x1415d0` | `-` |
| `ldr` at 0x141504 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x141508 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x141510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x14151c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x141530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x14153c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x141550 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x141554 | load | 4 | `abs 0x1415e0` | `-` |
| `str` at 0x141560 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x14156c | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x141570 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x141578 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0x141580 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x14158c | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x141590 | load | 4 | `abs 0x1415e4` | `-` |
| `str` at 0x14159c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0x1415a8 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x1415ac | load | 4 | `sp -0x58` | `-` |
| `ldm` at 0x1415b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1415b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1415b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x140fb4 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+20]` | `[sp-20]` |
| `bl` at 0x14104c | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x141080 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `0x0` |
| `bl` at 0x1410bc | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x1410e4 | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0x141118 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x141158 | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x14118c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1412c8 | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-28]` | `0x1e` | `sp-56` |
| `bl` at 0x1412fc | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x141340 | `(unknown callee) 0x13e868` | `0x1927c4` | `0x19273c` | `[sp-36]` | `[sp+12]` |
| `bl` at 0x141380 | `(unknown callee) 0x13e4f0` | `[sp-76]` | `[sp-84]` | `[sp-80]` | `sp-56` |
| `bl` at 0x1413b4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1413f8 | `(unknown callee) 0x13e868` | `0x19272c` | `0x19273c` | `[sp-76]` | `[sp+12]` |
| `bl` at 0x14147c | `(unknown callee) 0x13e778` | `[sp-72]` | `[sp-28]` | `0x42` | `sp-56` |
| `bl` at 0x1414c8 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x14150c | `(unknown callee) 0x13e868` | `0x192740` | `0x19273c` | `[sp-36]` | `[sp+12]` |
| `bl` at 0x141520 | `(unknown callee) 0x131fb0` | `[sp-28]+64` | `[sp+16]` | `?` | `[sp-28]+64` |
| `bl` at 0x141540 | `(unknown callee) 0x131fb0` | `[sp-28]+65` | `[sp+16]` | `?` | `[sp-28]+65` |
| `bl` at 0x141558 | `(unknown callee) 0x20634` | `[sp+20]` | `0x192758` | `?` | `ret(0x131fb0)` |
| `bl` at 0x141594 | `(unknown callee) 0x20634` | `[sp+20]` | `0x19278c` | `?` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x140fa4 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0x140ff4 | `? != 0x6` | `? == 0x6` |
| `bne` at 0x141024 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0x14108c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x141124 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x141198 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x141308 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x141324 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x14134c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x1413c0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1413dc | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x14141c | `[0x3151c4] > 0x2 (unsigned, 32-bit)` | `[0x3151c4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0x141488 | `[sp-32] <= 0x0 (signed, 32-bit)` | `[sp-32] > 0x0 (signed, 32-bit)` |
| `bne` at 0x141494 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `bne` at 0x1414d4 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x1414f0 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x14152c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x14154c | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x141588 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1415b4 | `r0` | `?` |

## `PIOWrite_Multi`

- `PIOWrite_Multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x140744 to 0x140eac.
  Established. The first conditional branch takes its target at 0x140860 when an unknown operand equals 0, and falls through when it does not (`beq` at 0x140840).
  Separate word stores place 0 and 1 at entry sp minus 0x3c on their respective paths (`str` at 0x140858, `str` at 0x140864).
  A later word store writes [sp] minus 1 at entry sp (`str` at 0x140e40).
  The return record retains an unknown value in r0 (`ldm` at 0x140e7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x140754 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x140758 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x14075c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x140760 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x140764 | load | 4 | `abs 0x140e80` | `-` |
| `ldr` at 0x140768 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x140788 | load | 4 | `(unknown)` | `-` |
| `str` at 0x14078c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x140790 | load | 4 | `abs 0x140e80` | `-` |
| `ldr` at 0x140794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1407b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1407b8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x1407bc | load | 4 | `abs 0x140e80` | `-` |
| `ldr` at 0x1407c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1407e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1407e8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x1407f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x1407f4 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x1407f8 | load | 4 | `abs 0x140e80` | `-` |
| `ldr` at 0x1407fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x14081c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140824 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x14082c | load | 2 | `(unknown)` | `-` |
| `str` at 0x140830 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x140834 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14084c | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140858 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x140864 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x140868 | load | 4 | `abs 0x140e80` | `-` |
| `ldr` at 0x14086c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x14088c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140894 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x14089c | load | 1 | `(unknown)` | `-` |
| `str` at 0x1408ac | store | 4 | `sp -0x44` | `0xd0000000` |
| `str` at 0x1408b4 | store | 4 | `sp -0x48` | `0x50000000` |
| `str` at 0x1408c0 | store | 4 | `sp -0x44` | `0x8000000` |
| `str` at 0x1408c8 | store | 4 | `sp -0x48` | `0xf0000000` |
| `ldr` at 0x1408cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1408d0 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0x1408d8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1408dc | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0x1408e0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x1408e4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x1408e8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x1408f4 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x1408f8 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0x1408fc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140900 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140904 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140908 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x14090c | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140910 | store | 4 | `sp -0x60` | `[sp+24]` |
| `ldr` at 0x140918 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x140938 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140940 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x140944 | store | 4 | `sp -0x34` | `[sp+12]` |
| `ldr` at 0x140948 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x14094c | store | 4 | `sp -0x38` | `[sp+8]` |
| `str` at 0x14096c | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0x140970 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140974 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140978 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x14097c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x14098c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x140990 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0x140994 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140998 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x14099c | load | 4 | `sp +0x14` | `-` |
| `str` at 0x1409a0 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x1409a4 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x1409a8 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x1409d0 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x1409d8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x1409dc | store | 4 | `sp -0x38` | `[sp-72]` |
| `ldr` at 0x1409e4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x1409e8 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0x1409ec | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x1409f0 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x1409f4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x140a00 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x140a04 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0x140a08 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140a0c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140a10 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140a14 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x140a18 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140a1c | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x140a44 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140a4c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x140a58 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140a5c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x140a64 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140a68 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x140a70 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140a74 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140a78 | load | 4 | `abs 0x140e84` | `-` |
| `ldr` at 0x140a84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140a8c | load | 4 | `abs 0x140e84` | `-` |
| `str` at 0x140a98 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140a9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140aa0 | load | 4 | `abs 0x140e88` | `-` |
| `ldr` at 0x140aac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140ab4 | load | 4 | `abs 0x140e88` | `-` |
| `str` at 0x140ac0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140ac4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140ac8 | load | 4 | `abs 0x140e8c` | `-` |
| `ldr` at 0x140ad4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140adc | load | 4 | `abs 0x140e8c` | `-` |
| `str` at 0x140ae8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140aec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140afc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x140b10 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x140b14 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x140b24 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x140b38 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x140b44 | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0x140b48 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140b4c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140b50 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x140b54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140b64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x140b68 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0x140b6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140b70 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140b74 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140b78 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x140b7c | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140b80 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x140ba8 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x140bc0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x140bc4 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0x140bc8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140bcc | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140bd0 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140bd4 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x140bd8 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140bdc | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x140c04 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140c0c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x140c1c | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0x140c24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x140c28 | store | 4 | `sp -0x38` | `[sp-68]` |
| `ldr` at 0x140c30 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140c34 | store | 4 | `sp -0x6c` | `[sp+16]` |
| `ldr` at 0x140c38 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x140c3c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x140c40 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x140c4c | load | 4 | `sp -0x50` | `-` |
| `str` at 0x140c50 | store | 4 | `sp -0x6c` | `[sp-80]` |
| `ldr` at 0x140c54 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140c58 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140c5c | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140c60 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x140c64 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140c68 | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x140c90 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140c98 | load | 4 | `abs 0x140e90` | `-` |
| `ldr` at 0x140c9c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x140ca8 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140cac | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0x140cb0 | load | 4 | `abs 0x140e94` | `-` |
| `ldr` at 0x140cb4 | load | 4 | `abs 0x140e98` | `-` |
| `ldr` at 0x140cb8 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x140cbc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140cc8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x140ccc | load | 4 | `abs 0x140e9c` | `-` |
| `str` at 0x140cd4 | store | 4 | `abs 0x3151c4` | `0x0` |
| `ldr` at 0x140cd8 | load | 4 | `abs 0x140e9c` | `-` |
| `ldr` at 0x140cdc | load | 4 | `abs 0x3151c4` | `-` |
| `ldr` at 0x140ce8 | load | 4 | `abs 0x140e9c` | `-` |
| `ldr` at 0x140cec | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x140d08 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x140d0c | load | 4 | `abs 0x140e9c` | `-` |
| `ldr` at 0x140d10 | load | 4 | `abs 0x140e9c` | `-` |
| `ldr` at 0x140d14 | load | 4 | `abs 0x3151c4` | `-` |
| `str` at 0x140d1c | store | 4 | `abs 0x3151c4` | `[0x3151c4]+1` |
| `str` at 0x140d2c | store | 4 | `sp -0x6c` | `sp-36` |
| `ldr` at 0x140d30 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140d34 | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140d38 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x140d3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140d48 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x140d54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x140d60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x140d64 | store | 4 | `sp -0x6c` | `[sp-36]` |
| `ldr` at 0x140d68 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x140d6c | store | 4 | `sp -0x68` | `[sp+16]` |
| `ldr` at 0x140d70 | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140d74 | store | 4 | `sp -0x64` | `[sp+20]` |
| `ldr` at 0x140d78 | load | 4 | `sp +0x18` | `-` |
| `str` at 0x140d7c | store | 4 | `sp -0x60` | `[sp+24]` |
| `str` at 0x140da4 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x140dac | load | 4 | `abs 0x140e90` | `-` |
| `ldr` at 0x140db0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x140dbc | load | 4 | `sp +0x14` | `-` |
| `str` at 0x140dc0 | store | 4 | `sp -0x6c` | `[sp+20]` |
| `ldr` at 0x140dc4 | load | 4 | `abs 0x140ea0` | `-` |
| `ldr` at 0x140dc8 | load | 4 | `abs 0x140e98` | `-` |
| `ldr` at 0x140dcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x140dd0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x140dd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140de4 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x140df8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x140e04 | load | 4 | `sp +0x14` | `-` |
| `ldr` at 0x140e18 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x140e1c | load | 4 | `abs 0x140ea4` | `-` |
| `str` at 0x140e28 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x140e34 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0x140e38 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x140e40 | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0x140e48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x140e54 | load | 4 | `sp +0x18` | `-` |
| `ldr` at 0x140e58 | load | 4 | `abs 0x140ea8` | `-` |
| `str` at 0x140e64 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x140e70 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0x140e74 | load | 4 | `sp -0x5c` | `-` |
| `ldm` at 0x140e7c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x140e7c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x140e7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x140850 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `[sp+24]` | `?` |
| `bl` at 0x1408f0 | `(unknown callee) 0x13e4f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0x140924 | `(unknown callee) 0x12d2b8` | `0x0` | `[sp-60]` | `0x1` | `0x0` |
| `bl` at 0x140960 | `(unknown callee) 0x13262c` | `0x0` | `0x40` | `sp-56` | `sp-56` |
| `bl` at 0x140988 | `(unknown callee) 0x13e778` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0x1409bc | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x1409fc | `(unknown callee) 0x13e4f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0x140a30 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x140b60 | `(unknown callee) 0x13e778` | `[sp-76]` | `[sp-32]` | `0x1e` | `sp-56` |
| `bl` at 0x140b94 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x140bf0 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x140c48 | `(unknown callee) 0x13e4f0` | `[sp-80]` | `[sp-88]` | `[sp-84]` | `sp-56` |
| `bl` at 0x140c7c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x140cc0 | `(unknown callee) 0x13e868` | `0x19272c` | `0x19273c` | `[sp-80]` | `[sp+16]` |
| `bl` at 0x140d44 | `(unknown callee) 0x13e778` | `[sp-76]` | `[sp-32]` | `0x42` | `sp-56` |
| `bl` at 0x140d90 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `0x0` |
| `bl` at 0x140dd4 | `(unknown callee) 0x13e868` | `0x192740` | `0x19273c` | `[sp-36]` | `[sp+16]` |
| `bl` at 0x140de8 | `(unknown callee) 0x131fb0` | `[sp-32]+64` | `[sp+20]` | `?` | `[sp-32]+64` |
| `bl` at 0x140e08 | `(unknown callee) 0x131fb0` | `[sp-32]+65` | `[sp+20]` | `?` | `[sp-32]+65` |
| `bl` at 0x140e20 | `(unknown callee) 0x20634` | `[sp+24]` | `0x192758` | `?` | `ret(0x131fb0)` |
| `bl` at 0x140e5c | `(unknown callee) 0x20634` | `[sp+24]` | `0x19278c` | `?` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x140840 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1408a4 | `? != 0x6` | `? == 0x6` |
| `bne` at 0x140930 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x1409c8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x140a3c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x140ba0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x140bbc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x140bfc | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x140c14 | `[sp] != 0x0` | `[sp] == 0x0` |
| `bne` at 0x140c88 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x140ca4 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x140ce4 | `[0x3151c4] > 0x2 (unsigned, 32-bit)` | `[0x3151c4] <= 0x2 (unsigned, 32-bit)` |
| `ble` at 0x140d50 | `[sp] <= 0x0 (signed, 32-bit)` | `[sp] > 0x0 (signed, 32-bit)` |
| `bne` at 0x140d5c | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0x140d9c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bls` at 0x140db8 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x140df4 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `beq` at 0x140e14 | `ret(0x131fb0) == 0x0` | `ret(0x131fb0) != 0x0` |
| `bne` at 0x140e50 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x140e7c | `r0` | `?` |

## `scancmd_init`

- `scancmd_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12c5b4 to 0x12c654.
  Established. The first conditional branch takes its target at 0x12c5e4 when entry r1 is nonzero and falls through when entry r1 is zero (`bne` at 0x12c5d4).
  The zero path writes 0 as a word at entry sp minus 0x20 (`str` at 0x12c5dc).
  The call to unnamed 0x20550 has printed r0-r3 candidates entry r1, 0, [sp-36] and unknown (`bl` at 0x12c63c).
  A later word store writes 1 at entry sp minus 0x20 (`str` at 0x12c644).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12c5c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12c5c8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x12c5cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12c5dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x12c5e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c5f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12c5fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12c600 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12c60c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12c614 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12c61c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12c620 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12c62c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12c630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12c638 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12c644 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x12c648 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x12c650 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12c650 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12c650 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12c63c | `(unknown callee) 0x20550` | `r1` | `0x0` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12c5d4 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0x12c5f0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12c650 | `r0` | `?` |

## `scancmd_create`

- `scancmd_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12c4f0 to 0x12c5b4.
  Established. The result from unnamed 0x201fc is stored as a word at entry sp minus 0x18 (`bl` at 0x12c554, `str` at 0x12c558);
  its zero result falls through to a word store of 0 at entry sp minus 0x1c (`bl` at 0x12c554, `bne` at 0x12c564, `str` at 0x12c56c).
  A separate call to unnamed 0x12c5b4 is followed by a branch that takes 0x12c5a0 when its result is nonzero and falls through when it is zero (`bl` at 0x12c57c, `bne` at 0x12c588).
  The word store at entry sp minus 0x1c on the nonzero path writes [sp-24] (`str` at 0x12c5a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12c500 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x12c504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12c51c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x12c520 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12c52c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x12c534 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12c53c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x12c540 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12c54c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x12c550 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12c558 | store | 4 | `sp -0x18` | `ret(0x201fc)` |
| `ldr` at 0x12c55c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12c56c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x12c574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12c58c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12c598 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x12c5a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12c5a4 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0x12c5a8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x12c5b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12c5b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12c5b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12c554 | `(unknown callee) 0x201fc` | `[sp-32]` | `r1` | `r2` | `?` |
| `bl` at 0x12c57c | `(unknown callee) 0x12c5b4` | `[sp-20]` | `ret(0x201fc)` | `?` | `ret(0x201fc)` |
| `bl` at 0x12c590 | `(unknown callee) 0x20514` | `[sp-24]` | `?` | `?` | `ret(0x12c5b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12c510 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12c564 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x12c588 | `ret(0x12c5b4) != 0x0` | `ret(0x12c5b4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12c5b0 | `r0` | `?` |

## `scancmd_destroy`

- `scancmd_destroy`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12c654 to 0x12c684.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x12c664).
  The branch takes its target at 0x12c678 when entry r0 is nonzero and falls through when it is zero (`bne` at 0x12c670).
  The call to unnamed 0x20514 has printed r0-r3 candidates entry r0, entry r1, entry r2 and entry r0 (`bl` at 0x12c67c).
  The return record retains an unknown value in r0 (`ldm` at 0x12c680).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12c664 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x12c668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c678 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12c680 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12c680 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12c680 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12c680 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12c67c | `(unknown callee) 0x20514` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12c670 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12c680 | `r0` | `?` |

## `scancmd_dump`

- `scancmd_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12c684 to 0x12cc4c.
  Established. A word loaded from absolute address 0x1a2888 is stored at entry sp minus 0x40 (`ldr` at 0x12c6cc, `str` at 0x12c6d0).
  A later word store writes [sp-32] plus 16 at entry sp minus 0x3c (`str` at 0x12c9ec).
  One word store writes [sp-36] plus 1 at entry sp minus 0x24 (`str` at 0x12cb6c).
  A separate later word store writes [sp-36] plus 1 at that same destination (`str` at 0x12cbcc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12c694 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12c698 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12c69c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x12c6a0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x12c6a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c6b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12c6b8 | store | 4 | `sp -0x40` | `r0` |
| `ldr` at 0x12c6bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c6c8 | load | 4 | `abs 0x12cbe8` | `-` |
| `ldr` at 0x12c6cc | load | 4 | `abs 0x1a2888` | `-` |
| `str` at 0x12c6d0 | store | 4 | `sp -0x40` | `[0x1a2888]` |
| `ldr` at 0x12c6d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12c6d8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x12c6dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12c6e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c6ec | load | 4 | `abs 0x12cbec` | `-` |
| `ldr` at 0x12c6f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12c6fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c700 | load | 4 | `abs 0x12cbf0` | `-` |
| `ldr` at 0x12c708 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c70c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x12c718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c71c | load | 4 | `abs 0x12cbf4` | `-` |
| `ldr` at 0x12c72c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c734 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c744 | load | 4 | `abs 0x12cbf8` | `-` |
| `ldr` at 0x12c750 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c754 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x12c760 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c764 | load | 4 | `abs 0x12cbfc` | `-` |
| `ldr` at 0x12c774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c77c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c788 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c78c | load | 4 | `abs 0x12cc00` | `-` |
| `ldr` at 0x12c798 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c79c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x12c7a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c7ac | load | 4 | `abs 0x12cc04` | `-` |
| `ldr` at 0x12c7bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c7c4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c7d4 | load | 4 | `abs 0x12cc08` | `-` |
| `ldr` at 0x12c7e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c7e4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x12c7ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c7f0 | load | 4 | `abs 0x12cc0c` | `-` |
| `ldr` at 0x12c800 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c808 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c818 | load | 4 | `abs 0x12cc10` | `-` |
| `ldr` at 0x12c828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c830 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c840 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c848 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x12c858 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c85c | load | 4 | `abs 0x12cc14` | `-` |
| `ldr` at 0x12c86c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c874 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c884 | load | 4 | `abs 0x12cc18` | `-` |
| `ldr` at 0x12c894 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c89c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x12c8a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c8ac | load | 4 | `abs 0x12cc1c` | `-` |
| `ldr` at 0x12c8bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c8c4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c8d4 | load | 4 | `abs 0x12cc20` | `-` |
| `ldr` at 0x12c8e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c8ec | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x12c8f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c8fc | load | 4 | `abs 0x12cc24` | `-` |
| `ldr` at 0x12c90c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c914 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c920 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c924 | load | 4 | `abs 0x12cc28` | `-` |
| `ldr` at 0x12c934 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c93c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x12c944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c948 | load | 4 | `abs 0x12cc2c` | `-` |
| `ldr` at 0x12c958 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c960 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c96c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c970 | load | 4 | `abs 0x12cc30` | `-` |
| `ldr` at 0x12c980 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c98c | load | 4 | `abs 0x12cc34` | `-` |
| `ldr` at 0x12c990 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0x12c99c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c9a4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x12c9ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12c9b0 | load | 4 | `abs 0x12cc38` | `-` |
| `ldr` at 0x12c9c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c9c8 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0x12c9d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12c9e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12c9ec | store | 4 | `sp -0x3c` | `[sp-32]+16` |
| `ldr` at 0x12c9f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12c9fc | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0x12ca14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12ca1c | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x12ca28 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x12ca2c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12ca34 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x12ca38 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12ca40 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x12ca44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12ca48 | load | 4 | `abs 0x12cc3c` | `-` |
| `str` at 0x12ca54 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x12ca58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ca5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12ca68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ca70 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12ca78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ca84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12ca90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ca9c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12caa8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12caac | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x12cab0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cabc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cac8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12cacc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x12cad0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cadc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cae8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12caec | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x12caf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cafc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cb08 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12cb0c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x12cb10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cb1c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cb28 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12cb2c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x12cb30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cb3c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cb48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12cb4c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x12cb50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12cb54 | load | 4 | `abs 0x12cc40` | `-` |
| `ldr` at 0x12cb58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12cb5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12cb64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12cb6c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x12cb74 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x12cb84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x12cb88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cb8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12cb98 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12cba0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cbac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12cbb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12cbb8 | load | 4 | `abs 0x12cc44` | `-` |
| `ldr` at 0x12cbbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12cbc4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12cbcc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x12cbd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12cbd8 | load | 4 | `abs 0x12cc48` | `-` |
| `ldm` at 0x12cbe4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12cbe4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12cbe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12c6f4 | `(unknown callee) 0x244d4` | `?` | `0x190554` | `r1` | `r1` |
| `bl` at 0x12c704 | `(unknown callee) 0x244d4` | `?` | `0x190558` | `r2` | `r1` |
| `bl` at 0x12c724 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190574` | `?` | `?` |
| `bl` at 0x12c74c | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190590` | `?` | `?` |
| `bl` at 0x12c76c | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1905a0` | `?` | `?` |
| `bl` at 0x12c794 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1905bc` | `?` | `?` |
| `bl` at 0x12c7b4 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1905cc` | `?` | `?` |
| `bl` at 0x12c7dc | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1905e8` | `?` | `?` |
| `bl` at 0x12c7f8 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1905f8` | `?` | `?` |
| `bl` at 0x12c820 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190614` | `?` | `?` |
| `bl` at 0x12c864 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190624` | `?` | `?` |
| `bl` at 0x12c88c | `(unknown callee) 0x244d4` | `[sp-20]` | `0x19063c` | `?` | `?` |
| `bl` at 0x12c8b4 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x19064c` | `?` | `?` |
| `bl` at 0x12c8dc | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190668` | `?` | `?` |
| `bl` at 0x12c904 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190678` | `?` | `?` |
| `bl` at 0x12c92c | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190694` | `?` | `?` |
| `bl` at 0x12c950 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1906a4` | `?` | `?` |
| `bl` at 0x12c978 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1906c0` | `?` | `?` |
| `bl` at 0x12c994 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1906d4` | `[[sp-32]+12]` | `[sp-32]+12` |
| `bl` at 0x12c9b8 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x1906ec` | `?` | `?` |
| `bl` at 0x12ca4c | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190700` | `?` | `?` |
| `bl` at 0x12cb60 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190724` | `?` | `?` |
| `bl` at 0x12cbc0 | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190758` | `?` | `?` |
| `bl` at 0x12cbdc | `(unknown callee) 0x244d4` | `[sp-20]` | `0x190760` | `?` | `[sp-56]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12c6ac | `r3 != 0x0` | `r3 == 0x0` |
| `bne` at 0x12c6c4 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0x12c6e4 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0x12c9d0 | `[[sp-32]+12] == 0x0` | `[[sp-32]+12] != 0x0` |
| `bne` at 0x12c9dc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0x12ca64 | `C == 1` | `not (C == 1)` |
| `beq` at 0x12cb7c | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bhs` at 0x12cb94 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12cbe4 | `r0` | `?` |

## `scan_multi_sic_ir`

- `scan_multi_sic_ir`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x131598 to 0x13196c.
  Established. The result from unnamed 0x1310f0 is stored as a word at entry sp minus 0x28 and then at entry sp minus 0x168 (`bl` at 0x131658, `str` at 0x131660, `str` at 0x131668).
  The earlier computed word store at 0x1315ec has an unknown value and unknown effective target (`str` at 0x1315ec).
  Separate computed word stores write 0 to unknown effective targets (`str` at 0x13160c, `str` at 0x13162c).
  The return record retains an unknown value in r0 (`ldm` at 0x131950).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1315a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1315ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1315b0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x1315b4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x1315bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x1315c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1315c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1315d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1315d4 | load | 4 | `abs 0x131954` | `-` |
| `ldr` at 0x1315e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1315ec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1315f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13160c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x131610 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13162c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x131630 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x131638 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x13164c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x131650 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x131660 | store | 4 | `sp -0x28` | `ret(0x1310f0)` |
| `ldr` at 0x131664 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x131668 | store | 4 | `sp -0x168` | `ret(0x1310f0)` |
| `str` at 0x131670 | store | 4 | `sp -0x164` | `sp-136` |
| `str` at 0x131678 | store | 4 | `sp -0x160` | `sp-232` |
| `ldr` at 0x13167c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x131680 | store | 4 | `sp -0x15c` | `[sp+12]` |
| `ldr` at 0x131688 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13168c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x131690 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x1316a8 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0x1316b0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1316bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1316c4 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0x1316c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1316d0 | store | 4 | `sp -0x150` | `?` |
| `str` at 0x1316d8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x1316dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1316e0 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0x1316ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x131704 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13170c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x131714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x131718 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x131724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13172c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x131734 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x13174c | load | 4 | `(unknown)` | `-` |
| `str` at 0x131750 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0x131754 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x131758 | load | 4 | `abs 0x131958` | `-` |
| `ldr` at 0x13175c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x131760 | load | 4 | `(unknown)` | `-` |
| `str` at 0x13176c | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0x131774 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x13177c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x131788 | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0x131790 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x131794 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x131798 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0x1317a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1317b0 | load | 4 | `sp -0x154` | `-` |
| `str` at 0x1317b8 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x1317bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1317c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x1317cc | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1317d0 | load | 4 | `abs 0x13195c` | `-` |
| `ldr` at 0x1317dc | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1317e4 | load | 4 | `abs 0x13195c` | `-` |
| `str` at 0x1317f0 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x1317f4 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1317f8 | load | 4 | `abs 0x131960` | `-` |
| `ldr` at 0x131804 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x13180c | load | 4 | `abs 0x131960` | `-` |
| `str` at 0x131818 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x13181c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x131820 | load | 4 | `abs 0x131964` | `-` |
| `ldr` at 0x13182c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x131834 | load | 4 | `abs 0x131964` | `-` |
| `str` at 0x131840 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x131844 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x131854 | load | 4 | `sp -0x154` | `-` |
| `str` at 0x131868 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x13186c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x13187c | load | 4 | `sp -0x154` | `-` |
| `str` at 0x131890 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0x131894 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0x1318ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1318b0 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1318b8 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0x1318c0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1318c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1318cc | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1318dc | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0x1318e4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x1318ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1318f0 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0x1318f8 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0x131910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x131914 | load | 4 | `sp -0x154` | `-` |
| `str` at 0x13191c | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0x131920 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x131924 | load | 4 | `abs 0x131968` | `-` |
| `ldr` at 0x131928 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0x131938 | store | 4 | `sp -0x158` | `0x0` |
| `str` at 0x131944 | store | 4 | `sp -0x158` | `0x1` |
| `ldr` at 0x131948 | load | 4 | `sp -0x158` | `-` |
| `ldm` at 0x131950 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x131950 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x131950 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x131658 | `(unknown callee) 0x1310f0` | `sp-328` | `?` | `?` | `sp-136` |
| `bl` at 0x131694 | `(unknown callee) 0x12d2b8` | `0x0` | `[sp-32]` | `[sp]` | `[sp+4]` |
| `bl` at 0x131764 | `(unknown callee) 0x205bc` | `[sp+12]` | `0x1909a4` | `[sp-36]` | `?` |
| `bl` at 0x131930 | `(unknown callee) 0x205bc` | `[sp+12]` | `0x1909f4` | `[sp-332]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x1315cc | `C == 1` | `not (C == 1)` |
| `bne` at 0x1316a0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x1316b8 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `bhs` at 0x1316e8 | `C == 1` | `not (C == 1)` |
| `beq` at 0x131720 | `? == ?` | `? != ?` |
| `bhs` at 0x1317a0 | `C == 1` | `not (C == 1)` |
| `beq` at 0x1318d8 | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x131950 | `r0` | `?` |

## `tdrdata_create`

- `tdrdata_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x143db8 to 0x143e48.
  Established. A halfword store preserves the low 16 bits of entry r1 at entry sp minus 0x16 (`strh` at 0x143dd4).
  The result from unnamed 0x201fc is stored as a word at entry sp minus 0x20 (`bl` at 0x143ddc, `str` at 0x143de0).
  The call to unnamed 0x143e48 has printed r0-r3 candidates that earlier result, [sp-20], [sp-22]:2 and [sp-28] (`bl` at 0x143ddc, `bl` at 0x143e10).
  The return record retains an unknown value in r0 (`ldm` at 0x143e44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x143dc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x143dd0 | store | 4 | `sp -0x1c` | `r2` |
| `strh` at 0x143dd4 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `str` at 0x143de0 | store | 4 | `sp -0x20` | `ret(0x201fc)` |
| `ldr` at 0x143de4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x143df4 | store | 4 | `sp -0x24` | `0x0` |
| `ldrh` at 0x143dfc | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0x143e00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x143e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x143e0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x143e20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x143e2c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x143e34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x143e38 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x143e3c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x143e44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x143e44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x143e44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x143ddc | `(unknown callee) 0x201fc` | `0x14` | `r1` | `r2` | `r1` |
| `bl` at 0x143e10 | `(unknown callee) 0x143e48` | `ret(0x201fc)` | `[sp-20]` | `[sp-22]:2` | `[sp-28]` |
| `bl` at 0x143e24 | `(unknown callee) 0x20514` | `[sp-32]` | `?` | `?` | `ret(0x143e48)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x143dec | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x143e1c | `ret(0x143e48) != 0x0` | `ret(0x143e48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x143e44 | `r0` | `?` |

## `tdrdata_init`

- `tdrdata_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x143e48 to 0x143fec.
  Established. A halfword store writes 0 to the address formed by [sp-24] plus 6 (`strh` at 0x143ec0).
  A word store writes [sp-36] to the address formed by [sp-24] plus 8 (`str` at 0x143ecc).
  The result from one call to unnamed 0x201fc is stored as a word at the address formed by [sp-24] plus 0x10 (`bl` at 0x143f1c, `str` at 0x143f20).
  A separate call to that unnamed address has its result stored as a word at the address formed by [sp-24] plus 0xc (`bl` at 0x143f70, `str` at 0x143f74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x143e58 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x143e5c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x143e60 | store | 4 | `sp -0x24` | `r3` |
| `strh` at 0x143e64 | store | 2 | `sp -0x1e` | `(r2 & 0xffff)` |
| `ldr` at 0x143e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x143e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143e78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x143e80 | store | 4 | `r0 +0x0` | `ret(0x20058)` |
| `ldr` at 0x143e84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143e88 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x143e98 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x143ea0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x143ea8 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0x143eac | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x143eb0 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0x143eb4 | store | 2 | `[sp-24] +0x4` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0x143eb8 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x143ec0 | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0x143ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143ec8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x143ecc | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0x143ed0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x143ed4 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x143ef4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x143ef8 | load | 2 | `[sp-36] +0x0` | `-` |
| `str` at 0x143f10 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x143f14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f18 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x143f20 | store | 4 | `[sp-24] +0x10` | `ret(0x201fc)` |
| `ldr` at 0x143f24 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0x143f30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f34 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x143f40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f44 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x143f50 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x143f58 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x143f5c | load | 2 | `[sp-36] +0x2` | `-` |
| `str` at 0x143f64 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x143f68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x143f74 | store | 4 | `[sp-24] +0xc` | `ret(0x201fc)` |
| `ldr` at 0x143f78 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x143f84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f88 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x143f94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143f98 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x143fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143fa4 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0x143fb0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x143fb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x143fbc | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x143fc0 | load | 2 | `[sp-36] +0x2` | `-` |
| `ldr` at 0x143fc4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x143fd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x143fdc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x143fe0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x143fe8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x143fe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x143fe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x143fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x143e7c | `(unknown callee) 0x20058` | `r1` | `r1` | `r2` | `r1` |
| `bl` at 0x143f1c | `(unknown callee) 0x201fc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x143f48 | `(unknown callee) 0x20514` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x143f70 | `(unknown callee) 0x201fc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x143f9c | `(unknown callee) 0x20514` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x143fa8 | `(unknown callee) 0x20514` | `[[sp-24]+16]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x143fcc | `(unknown callee) 0x20550` | `[[sp-24]+12]` | `0x0` | `[[sp-36]+2]:2` | `[sp-24]` |
| `bl` at 0x143fd4 | `(unknown callee) 0x144ac4` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x143e70 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x143e90 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0x143f2c | `[[sp-24]+16] != 0x0` | `[[sp-24]+16] == 0x0` |
| `beq` at 0x143f3c | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |
| `bne` at 0x143f80 | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `beq` at 0x143f90 | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x143fe8 | `r0` | `?` |

## `tdrdata_destroy`

- `tdrdata_destroy`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x143fec to 0x144090.
  Established. A word store writes 0 at entry sp minus 0x1c (`str` at 0x144008).
  A word loaded from [sp-32] plus 0xc supplies the printed r0 candidate for a call to unnamed 0x20514 (`ldr` at 0x144060, `bl` at 0x144064).
  A later word store writes [sp-28] plus 1 at entry sp minus 0x1c (`str` at 0x144078).
  The return record preserves the result of the final call to unnamed 0x20514 (`bl` at 0x144084, `ldm` at 0x14408c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x143ffc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x144000 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x144008 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x14400c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x144010 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x14401c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x144024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14402c | load | 4 | `(unknown)` | `-` |
| `str` at 0x144030 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x144034 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x144038 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x144044 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x144048 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x144050 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x144054 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x14405c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x144060 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0x144068 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x144070 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x144078 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x144080 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x14408c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x14408c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x14408c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x14404c | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0x144058 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0x144064 | `(unknown callee) 0x20514` | `[[sp-32]+12]` | `?` | `?` | `[sp-32]` |
| `bl` at 0x14406c | `(unknown callee) 0x20514` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x144084 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x144018 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x144040 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x14408c | `r0` | `ret(0x20514)` |

## `tdrdata_reset`

- `tdrdata_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x144ac4 to 0x144ca4.
  Established. The word loaded from entry r0 plus 8 is tested by a branch that takes 0x144ba8 when the loaded value is zero and falls through when it is nonzero (`ldr` at 0x144adc, `beq` at 0x144ae4).
  A computed word store has an unknown value and unknown effective target (`str` at 0x144b94).
  A later computed word store writes 0 to an unknown effective target (`str` at 0x144c3c).
  A computed halfword store also writes 0 to an unknown effective target (`strh` at 0x144c88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x144ad4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x144ad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144adc | load | 4 | `r0 +0x8` | `-` |
| `str` at 0x144aec | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x144af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144af4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144af8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144b14 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144b18 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144b3c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144b40 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144b54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144b64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144b68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144b70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x144b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144b7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x144b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144b88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x144b90 | load | 4 | `(unknown)` | `-` |
| `str` at 0x144b94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x144b98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x144ba0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x144bac | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x144bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144bb4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144bb8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144bd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144bd4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144bd8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144be8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144bfc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144c00 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144c30 | load | 4 | `(unknown)` | `-` |
| `str` at 0x144c3c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x144c40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x144c48 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x144c54 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x144c58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144c5c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x144c60 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x144c64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x144c74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x144c7c | load | 4 | `(unknown)` | `-` |
| `strh` at 0x144c88 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x144c8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x144c94 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x144ca0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x144ca0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x144ca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x144ae4 | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `bne` at 0x144b0c | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x144b30 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0x144b5c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x144bcc | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x144bf0 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0x144c1c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0x144c6c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x144ca0 | `r0` | `?` |
