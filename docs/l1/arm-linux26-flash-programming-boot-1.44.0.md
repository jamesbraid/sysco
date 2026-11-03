# ARM Linux 2.6 flash and boot candidates in 1.44.0

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
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
An indirect transfer in FlashIRouter remains unresolved; its tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-flash-programming-boot-facts.txt) retain all tool sections.

## `flash_init`

- `flash_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110180 to 0x1102a0.
  Established. A word store preserves entry r0 at entry sp minus 0x18 (`str` at 0x110190).
  The result from unnamed 0x204e4 is stored as a word at absolute address 0x1a05dc (`bl` at 0x1101f8, `str` at 0x110204).
  Another word store writes 1 to absolute address 0x314f74 (`str` at 0x110270).
  Alternative word stores at entry sp minus 0x20 write an unknown value or 0 (`str` at 0x110260, `str` at 0x110278).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110190 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x110194 | load | 4 | `abs 0x110288` | `-` |
| `ldr` at 0x110198 | load | 4 | `abs 0x314f74` | `-` |
| `str` at 0x1101a8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x1101b4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x1101b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1101c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1101cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1101d4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x1101dc | load | 4 | `abs 0x11028c` | `-` |
| `ldr` at 0x1101e0 | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x1101ec | load | 4 | `abs 0x110290` | `-` |
| `ldr` at 0x1101f4 | load | 4 | `abs 0x110294` | `-` |
| `ldr` at 0x110200 | load | 4 | `abs 0x11028c` | `-` |
| `str` at 0x110204 | store | 4 | `abs 0x1a05dc` | `ret(0x204e4)` |
| `ldr` at 0x110208 | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x110228 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110234 | load | 4 | `abs 0x110298` | `-` |
| `str` at 0x110238 | store | 4 | `sp -0x2c` | `0x18dbe4` |
| `ldr` at 0x11023c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x110240 | store | 4 | `sp -0x28` | `[ret(0x20184)]` |
| `str` at 0x110244 | store | 4 | `sp -0x24` | `ret(0x20154)` |
| `ldr` at 0x11024c | load | 4 | `abs 0x11029c` | `-` |
| `str` at 0x110260 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110268 | load | 4 | `abs 0x110288` | `-` |
| `str` at 0x110270 | store | 4 | `abs 0x314f74` | `0x1` |
| `str` at 0x110278 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x11027c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x110284 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x110284 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110284 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110284 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1101c8 | `(unknown callee) 0x10fce4` | `?` | `?` | `?` | `?` |
| `bl` at 0x1101f8 | `(unknown callee) 0x204e4` | `0x124f0e5` | `0x1` | `0x3b6` | `[0x1a05dc]` |
| `bl` at 0x110214 | `(unknown callee) 0x20184` | `ret(0x204e4)` | `?` | `ret(0x204e4)` | `[0x1a05dc]` |
| `bl` at 0x110220 | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x11022c | `(unknown callee) 0x20154` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110258 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0xde` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1101a0 | `[0x314f74] == 0x0` | `[0x314f74] != 0x0` |
| `bgt` at 0x1101c0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x1101e8 | `[0x1a05dc] >= 0x0 (signed, 32-bit)` | `[0x1a05dc] < 0x0 (signed, 32-bit)` |
| `bge` at 0x110210 | `[0x1a05dc] >= 0x0 (signed, 32-bit)` | `[0x1a05dc] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110284 | `r0` | `?` |

## `flash_write_addr`

- `flash_write_addr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110654 to 0x1106f4.
  Established. The result from unnamed 0x110410 is stored as a word at entry sp minus 0x1c (`bl` at 0x11066c, `str` at 0x110674).
  A distinct result from unnamed 0x1104ac is stored as a word at entry sp minus 0x18 (`bl` at 0x110694, `str` at 0x11069c).
  An indexed word load retains an unknown effective address (`ldr` at 0x1106d0).
  The following computed word store at entry sp minus 0x20 has an unknown value (`str` at 0x1106dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110664 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x110668 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110674 | store | 4 | `sp -0x1c` | `ret(0x110410)` |
| `ldr` at 0x110678 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x110688 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110690 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11069c | store | 4 | `sp -0x18` | `ret(0x1104ac)` |
| `ldr` at 0x1106a0 | load | 4 | `abs 0x1106ec` | `-` |
| `ldr` at 0x1106a4 | load | 4 | `abs 0x31e428` | `-` |
| `ldr` at 0x1106b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1106c0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x1106c8 | load | 4 | `abs 0x1106f0` | `-` |
| `ldr` at 0x1106cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1106d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1106d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1106dc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x1106e0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1106e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1106e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1106e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11066c | `(unknown callee) 0x110410` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x110694 | `(unknown callee) 0x1104ac` | `[sp-20]` | `?` | `?` | `ret(0x110410)` |
| `bl` at 0x1106a8 | `(unknown callee) 0x110410` | `[0x31e428]` | `?` | `?` | `0x31e418` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x110680 | `ret(0x110410) >= 0x0 (signed, 32-bit)` | `ret(0x110410) < 0x0 (signed, 32-bit)` |
| `bne` at 0x1106b8 | `ret(0x110410) != [sp-28]` | `ret(0x110410) == [sp-28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1106e8 | `r0` | `[sp-32]` |

## `flash_read`

- `flash_read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1106f4 to 0x110788.
  Established. Word stores preserve entry r0, r1 and r2 at entry sp minus 0x14, 0x18 and 0x1c respectively (`str` at 0x110704, `str` at 0x110708, `str` at 0x11070c).
  The result from unnamed 0x110180 is stored as a word at entry sp minus 0x24 (`bl` at 0x110724, `str` at 0x11072c).
  A call to unnamed 0x203f4 has printed r0-r3 candidates [sp-20], [sp-32], [sp-28] and unknown (`bl` at 0x110768).
  A later word store writes 0 at entry sp minus 0x28 (`str` at 0x110774).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110704 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x110708 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x11070c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x110710 | load | 4 | `abs 0x110784` | `-` |
| `ldr` at 0x110714 | load | 4 | `abs 0x314f74` | `-` |
| `str` at 0x11072c | store | 4 | `sp -0x24` | `ret(0x110180)` |
| `ldr` at 0x110730 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11073c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x110740 | store | 4 | `sp -0x28` | `ret(0x110180)` |
| `ldr` at 0x110748 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x110754 | store | 4 | `sp -0x20` | `ret(0x11053c)` |
| `ldr` at 0x11075c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x110760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x110764 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x110774 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x110778 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x110780 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110780 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110780 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110724 | `(unknown callee) 0x110180` | `0x0` | `r1` | `r2` | `[0x314f74]` |
| `bl` at 0x11074c | `(unknown callee) 0x11053c` | `?` | `?` | `?` | `?` |
| `bl` at 0x110758 | `(unknown callee) 0x1102a0` | `ret(0x11053c)` | `?` | `?` | `ret(0x11053c)` |
| `bl` at 0x110768 | `(unknown callee) 0x203f4` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0x11076c | `(unknown callee) 0x110358` | `ret(0x203f4)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11071c | `[0x314f74] != 0x0` | `[0x314f74] == 0x0` |
| `bge` at 0x110738 | `ret(0x110180) >= 0x0 (signed, 32-bit)` | `ret(0x110180) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110780 | `r0` | `?` |

## `flash_erase`

- `flash_erase`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110788 to 0x1107bc.
  Established. Entry r0, r1 and r2 are stored as words at entry sp minus 0x14, 0x18 and 0x1c respectively (`str` at 0x110798, `str` at 0x11079c, `str` at 0x1107a0).
  A further word store writes 0 at entry sp minus 0x24 (`str` at 0x1107a8).
  The recorded return value in r0 is 0 (`ldm` at 0x1107b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110798 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x11079c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1107a0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x1107a8 | store | 4 | `sp -0x24` | `0x0` |
| `ldm` at 0x1107b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1107b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1107b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1107b8 | `r0` | `0x0` |

## `flash_program`

- `flash_program`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1107bc to 0x110c10.
  Established. The result from unnamed 0x110410 is stored as a word at entry sp minus 0x28 (`bl` at 0x110824, `str` at 0x11082c).
  A later result from unnamed 0x2007c is stored as a word at entry sp minus 0x2c (`bl` at 0x110a54, `str` at 0x110a5c).
  Separate word stores at that latter location write constants 0xfffffc7c and 0xfffffc7a (`str` at 0x110860, `str` at 0x110b78).
  A word store writes [sp-20] to memory at [sp-32] (`str` at 0x110bb4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1107cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1107d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1107d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x1107d8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x1107e0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x1107e4 | load | 4 | `abs 0x110bd0` | `-` |
| `ldr` at 0x1107e8 | load | 4 | `abs 0x314f74` | `-` |
| `str` at 0x110800 | store | 4 | `sp -0x40` | `ret(0x110180)` |
| `ldr` at 0x110804 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x110810 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x110814 | store | 4 | `sp -0x60` | `ret(0x110180)` |
| `ldr` at 0x110820 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11082c | store | 4 | `sp -0x28` | `ret(0x110410)` |
| `ldr` at 0x110830 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11083c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110848 | store | 4 | `sp -0x30` | `ret(0x110654)` |
| `ldr` at 0x11084c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11085c | load | 4 | `abs 0x110bd4` | `-` |
| `str` at 0x110860 | store | 4 | `sp -0x2c` | `0xfffffc7c` |
| `ldr` at 0x110868 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110874 | store | 4 | `sp -0x34` | `ret(0x1104ac)` |
| `ldr` at 0x110878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x110884 | load | 4 | `abs 0x110bd8` | `-` |
| `ldr` at 0x110888 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x110890 | load | 4 | `abs 0x31e438` | `-` |
| `ldr` at 0x1108a4 | load | 4 | `abs 0x110bdc` | `-` |
| `str` at 0x1108a8 | store | 4 | `sp -0x70` | `0x18dcdc` |
| `ldr` at 0x1108ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1108b0 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0x1108b8 | load | 4 | `abs 0x110be0` | `-` |
| `str` at 0x1108cc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x1108d8 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x1108dc | load | 4 | `abs 0x110bd8` | `-` |
| `ldr` at 0x1108e0 | load | 4 | `abs 0x31e438` | `-` |
| `ldr` at 0x1108e4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1108ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1108fc | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0x110904 | store | 4 | `sp -0x58` | `0x10` |
| `str` at 0x11090c | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0x110910 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x110914 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x110918 | load | 4 | `abs 0x110be4` | `-` |
| `ldr` at 0x11091c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x110924 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x110928 | load | 4 | `abs 0x110be8` | `-` |
| `ldr` at 0x110948 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110954 | load | 4 | `abs 0x110bec` | `-` |
| `str` at 0x110958 | store | 4 | `sp -0x70` | `0x18dd18` |
| `ldr` at 0x11095c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x110960 | store | 4 | `sp -0x6c` | `[sp-64]` |
| `str` at 0x110964 | store | 4 | `sp -0x68` | `ret(0x219a4)` |
| `ldr` at 0x11096c | load | 4 | `abs 0x110be0` | `-` |
| `ldr` at 0x110970 | load | 4 | `abs 0x110bf0` | `-` |
| `str` at 0x110980 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x110988 | load | 1 | `sp -0x4b` | `-` |
| `ldr` at 0x110994 | load | 4 | `abs 0x110bd8` | `-` |
| `ldr` at 0x110998 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11099c | load | 4 | `abs 0x31e438` | `-` |
| `str` at 0x1109a4 | store | 4 | `sp -0x34` | `([sp-52] + [0x31e438])` |
| `ldr` at 0x1109a8 | load | 4 | `abs 0x110bd8` | `-` |
| `ldr` at 0x1109ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1109b0 | load | 4 | `abs 0x31e438` | `-` |
| `str` at 0x1109b8 | store | 4 | `sp -0x30` | `([sp-48] + [0x31e438])` |
| `ldr` at 0x1109bc | load | 4 | `abs 0x110bd8` | `-` |
| `ldr` at 0x1109c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1109c4 | load | 4 | `abs 0x31e438` | `-` |
| `str` at 0x1109cc | store | 4 | `sp -0x40` | `([sp-64] + [0x31e438])` |
| `ldr` at 0x1109d4 | load | 4 | `abs 0x110bf4` | `-` |
| `ldr` at 0x1109d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1109dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1109e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x110a00 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110a0c | load | 4 | `abs 0x110bf8` | `-` |
| `str` at 0x110a10 | store | 4 | `sp -0x70` | `0x18dd4c` |
| `ldr` at 0x110a14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110a18 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0x110a1c | store | 4 | `sp -0x68` | `ret(0x219a4)` |
| `ldr` at 0x110a24 | load | 4 | `abs 0x110be0` | `-` |
| `str` at 0x110a38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x110a40 | load | 4 | `abs 0x110bf4` | `-` |
| `ldr` at 0x110a44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x110a48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x110a4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x110a50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x110a5c | store | 4 | `sp -0x2c` | `ret(0x2007c)` |
| `ldr` at 0x110a60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x110a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110a78 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110a84 | load | 4 | `abs 0x110bfc` | `-` |
| `str` at 0x110a88 | store | 4 | `sp -0x70` | `0x18dd78` |
| `ldr` at 0x110a8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110a90 | store | 4 | `sp -0x6c` | `[sp-20]` |
| `ldr` at 0x110a94 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x110a98 | store | 4 | `sp -0x68` | `[sp-44]` |
| `str` at 0x110a9c | store | 4 | `sp -0x64` | `ret(0x219a4)` |
| `ldr` at 0x110aa4 | load | 4 | `abs 0x110be0` | `-` |
| `ldr` at 0x110aa8 | load | 4 | `abs 0x110c00` | `-` |
| `str` at 0x110ab8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x110ac0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x110ac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110ae4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110af0 | load | 4 | `abs 0x110c04` | `-` |
| `str` at 0x110af4 | store | 4 | `sp -0x70` | `0x18dda8` |
| `ldr` at 0x110af8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110afc | store | 4 | `sp -0x6c` | `[sp-20]` |
| `str` at 0x110b00 | store | 4 | `sp -0x68` | `ret(0x219a4)` |
| `ldr` at 0x110b08 | load | 4 | `abs 0x110be0` | `-` |
| `ldr` at 0x110b0c | load | 4 | `abs 0x110c08` | `-` |
| `str` at 0x110b1c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x110b28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x110b2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x110b30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110b3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x110b40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x110b48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x110b4c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x110b54 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x110b58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x110b64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x110b68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x110b70 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x110b74 | load | 4 | `abs 0x110c0c` | `-` |
| `str` at 0x110b78 | store | 4 | `sp -0x2c` | `0xfffffc7a` |
| `ldr` at 0x110b80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x110b88 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x110b98 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x110ba0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x110bac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x110bb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110bb4 | store | 4 | `[sp-32] +0x0` | `[sp-20]` |
| `ldr` at 0x110bbc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x110bc0 | store | 4 | `sp -0x60` | `[sp-44]` |
| `ldr` at 0x110bc4 | load | 4 | `sp -0x60` | `-` |
| `ldm` at 0x110bcc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110bcc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110bcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1107f8 | `(unknown callee) 0x110180` | `0x0` | `r1` | `r2` | `[0x314f74]` |
| `bl` at 0x11081c | `(unknown callee) 0x1102a0` | `?` | `?` | `?` | `?` |
| `bl` at 0x110824 | `(unknown callee) 0x110410` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x110840 | `(unknown callee) 0x110654` | `[sp-20]` | `?` | `?` | `ret(0x110410)` |
| `bl` at 0x11086c | `(unknown callee) 0x1104ac` | `[sp-20]` | `?` | `?` | `ret(0x110654)` |
| `bl` at 0x110894 | `(unknown callee) 0x14ff8c` | `ret(0x1104ac)` | `[0x31e438]` | `ret(0x1104ac)` | `0x31e418` |
| `bl` at 0x1108c4 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x198` | `0x0` |
| `bl` at 0x110930 | `(unknown callee) 0x20520` | `?` | `0xc00c4d04` | `sp-92` | `[sp-40]` |
| `bl` at 0x110940 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x11094c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110978 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1a3` | `0x0` |
| `bl` at 0x1109e8 | `(unknown callee) 0x203dc` | `?` | `?` | `0x0` | `0x1a05ec` |
| `bl` at 0x1109f8 | `(unknown callee) 0x20184` | `ret(0x203dc)` | `?` | `?` | `ret(0x203dc)` |
| `bl` at 0x110a04 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110a30 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1b4` | `0x0` |
| `bl` at 0x110a54 | `(unknown callee) 0x2007c` | `?` | `[sp-24]` | `[sp-28]` | `[sp-40]` |
| `bl` at 0x110a70 | `(unknown callee) 0x20184` | `ret(0x2007c)` | `?` | `ret(0x2007c)` | `[sp-28]` |
| `bl` at 0x110a7c | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110ab0 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1ba` | `0x0` |
| `bl` at 0x110acc | `(unknown callee) 0x20298` | `[sp-48]` | `[sp-28]` | `0x6` | `[sp-28]` |
| `bl` at 0x110adc | `(unknown callee) 0x20184` | `ret(0x20298)` | `?` | `?` | `ret(0x20298)` |
| `bl` at 0x110ae8 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110b14 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1c2` | `0x0` |
| `bl` at 0x110b90 | `(unknown callee) 0x110358` | `ret(0x20298)` | `?` | `?` | `[sp-28]` |
| `bl` at 0x110bb8 | `(unknown callee) 0x110358` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1107f0 | `[0x314f74] != 0x0` | `[0x314f74] == 0x0` |
| `bge` at 0x11080c | `ret(0x110180) >= 0x0 (signed, 32-bit)` | `ret(0x110180) < 0x0 (signed, 32-bit)` |
| `beq` at 0x110838 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x110854 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x110880 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x1108a0 | `ret(0x14ff8c) == 0x0` | `ret(0x14ff8c) != 0x0` |
| `bls` at 0x1108f4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x11093c | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `beq` at 0x110990 | `[sp-75]:1 == 0xff` | `[sp-75]:1 != 0xff` |
| `bge` at 0x1109f4 | `ret(0x203dc) >= 0x0 (signed, 32-bit)` | `ret(0x203dc) < 0x0 (signed, 32-bit)` |
| `beq` at 0x110a6c | `ret(0x2007c) == [sp-28]` | `ret(0x2007c) != [sp-28]` |
| `bge` at 0x110ad8 | `ret(0x20298) >= 0x0 (signed, 32-bit)` | `ret(0x20298) < 0x0 (signed, 32-bit)` |
| `bhs` at 0x110b38 | `C == 1` | `not (C == 1)` |
| `beq` at 0x110b60 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x110ba8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110bcc | `r0` | `?` |

## `flash_set_boot_image`

- `flash_set_boot_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110c10 to 0x110e38.
  Established. Separate paths write byte values 0x41 and 0x42 at entry sp minus 0x15 (`strb` at 0x110cc4, `strb` at 0x110ce4).
  The result from unnamed 0x2046c is stored as a word at entry sp minus 0x1c (`bl` at 0x110d24, `str` at 0x110d2c).
  A call to unnamed 0x22070 has printed r0-r3 candidates 3, 0x18dac4, 0x202 and 0 (`bl` at 0x110ddc).
  A later word store writes 0 at entry sp minus 0x20 (`str` at 0x110df8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110c20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x110c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x110c30 | load | 4 | `abs 0x110e08` | `-` |
| `ldr` at 0x110c4c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110c60 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110c6c | load | 4 | `abs 0x110e0c` | `-` |
| `str` at 0x110c70 | store | 4 | `sp -0x2c` | `0x18dde8` |
| `ldr` at 0x110c74 | load | 4 | `abs 0x110e08` | `-` |
| `str` at 0x110c78 | store | 4 | `sp -0x28` | `0x18ddd4` |
| `str` at 0x110c7c | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x110c84 | load | 4 | `abs 0x110e10` | `-` |
| `ldr` at 0x110c88 | load | 4 | `abs 0x110e14` | `-` |
| `str` at 0x110c98 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x110ca4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x110cac | load | 4 | `abs 0x110e18` | `-` |
| `ldr` at 0x110cb0 | load | 4 | `abs 0x31e420` | `-` |
| `ldr` at 0x110cb4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x110cc4 | store | 1 | `sp -0x15` | `0x41` |
| `ldr` at 0x110ccc | load | 4 | `abs 0x110e18` | `-` |
| `ldr` at 0x110cd0 | load | 4 | `abs 0x31e424` | `-` |
| `ldr` at 0x110cd4 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x110ce4 | store | 1 | `sp -0x15` | `0x42` |
| `ldr` at 0x110cec | load | 4 | `abs 0x110e1c` | `-` |
| `str` at 0x110cf0 | store | 4 | `sp -0x2c` | `0x18de14` |
| `ldr` at 0x110cf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110cf8 | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0x110d00 | load | 4 | `abs 0x110e10` | `-` |
| `ldr` at 0x110d04 | load | 4 | `abs 0x110e20` | `-` |
| `str` at 0x110d14 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110d1c | load | 4 | `abs 0x110e08` | `-` |
| `ldr` at 0x110d20 | load | 4 | `abs 0x110e24` | `-` |
| `str` at 0x110d2c | store | 4 | `sp -0x1c` | `ret(0x2046c)` |
| `ldr` at 0x110d30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110d44 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110d50 | load | 4 | `abs 0x110e28` | `-` |
| `str` at 0x110d54 | store | 4 | `sp -0x2c` | `0x18de3c` |
| `ldr` at 0x110d58 | load | 4 | `abs 0x110e2c` | `-` |
| `str` at 0x110d5c | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x110d60 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x110d68 | load | 4 | `abs 0x110e10` | `-` |
| `str` at 0x110d7c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110d88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110dac | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110db8 | load | 4 | `abs 0x110e30` | `-` |
| `str` at 0x110dbc | store | 4 | `sp -0x2c` | `0x18de80` |
| `ldr` at 0x110dc0 | load | 4 | `abs 0x110e08` | `-` |
| `str` at 0x110dc4 | store | 4 | `sp -0x28` | `0x18ddd4` |
| `str` at 0x110dc8 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x110dd0 | load | 4 | `abs 0x110e10` | `-` |
| `ldr` at 0x110dd4 | load | 4 | `abs 0x110e34` | `-` |
| `str` at 0x110de4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110dec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x110df8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x110dfc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x110e04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110e04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110c34 | `(unknown callee) 0x20100` | `0x18ddd4` | `r1` | `r2` | `r0` |
| `bl` at 0x110c44 | `(unknown callee) 0x20184` | `ret(0x20100)` | `?` | `?` | `ret(0x20100)` |
| `bl` at 0x110c58 | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `[ret(0x20184)]` |
| `bl` at 0x110c64 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110c90 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1ed` | `0x0` |
| `bl` at 0x110d0c | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1f7` | `0x0` |
| `bl` at 0x110d24 | `(unknown callee) 0x2046c` | `0x18ddd4` | `0x241` | `?` | `?` |
| `bl` at 0x110d3c | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0x110d48 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110d74 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x1fc` | `0x0` |
| `bl` at 0x110d94 | `(unknown callee) 0x2007c` | `ret(0x2046c)` | `sp-21` | `0x1` | `sp-21` |
| `bl` at 0x110da4 | `(unknown callee) 0x20184` | `ret(0x2007c)` | `?` | `?` | `ret(0x2007c)` |
| `bl` at 0x110db0 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110ddc | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x202` | `0x0` |
| `bl` at 0x110df0 | `(unknown callee) 0x200b8` | `[sp-28]` | `?` | `?` | `ret(0x2007c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x110c2c | `r0 != 0x0` | `r0 == 0x0` |
| `bge` at 0x110c40 | `ret(0x20100) >= 0x0 (signed, 32-bit)` | `ret(0x20100) < 0x0 (signed, 32-bit)` |
| `beq` at 0x110c54 | `[ret(0x20184)] == 0x2` | `[ret(0x20184)] != 0x2` |
| `bne` at 0x110cbc | `[0x31e420] != r0` | `[0x31e420] == r0` |
| `bne` at 0x110cdc | `[0x31e424] != r0` | `[0x31e424] == r0` |
| `bge` at 0x110d38 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x110da0 | `ret(0x2007c) == 0x1` | `ret(0x2007c) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110e04 | `r0` | `?` |

## `flash_set_default_image`

- `flash_set_default_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110e38 to 0x1110ec.
  Established. Word stores to memory at [0x31e434] and [0x31e434] plus 4 have unknown values (`str` at 0x110e64, `str` at 0x110e90).
  The result from unnamed 0x2046c is stored as a word at entry sp minus 0x18 (`bl` at 0x110f94, `str` at 0x110f9c).
  Byte stores on separate paths write 0x41 and 0x42 at entry sp minus 0x19 (`strb` at 0x110f34, `strb` at 0x110f54).
  The later computed word store retains both an unknown effective address and an unknown value (`str` at 0x11109c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110e48 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x110e4c | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110e50 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x110e54 | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110e58 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x110e5c | load | 4 | `[0x31e434] +0x0` | `-` |
| `str` at 0x110e64 | store | 4 | `[0x31e434] +0x0` | `?` |
| `ldr` at 0x110e68 | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110e70 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x110e78 | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110e80 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x110e88 | load | 4 | `[0x31e434] +0x4` | `-` |
| `str` at 0x110e90 | store | 4 | `[0x31e434] +0x4` | `?` |
| `ldr` at 0x110e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x110ea0 | load | 4 | `abs 0x1110c0` | `-` |
| `ldr` at 0x110ebc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110ed0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110edc | load | 4 | `abs 0x1110c4` | `-` |
| `str` at 0x110ee0 | store | 4 | `sp -0x2c` | `0x18deac` |
| `ldr` at 0x110ee4 | load | 4 | `abs 0x1110c0` | `-` |
| `str` at 0x110ee8 | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x110eec | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x110ef4 | load | 4 | `abs 0x1110c8` | `-` |
| `ldr` at 0x110ef8 | load | 4 | `abs 0x1110cc` | `-` |
| `str` at 0x110f08 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x110f14 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x110f1c | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110f20 | load | 4 | `abs 0x31e420` | `-` |
| `ldr` at 0x110f24 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x110f34 | store | 1 | `sp -0x19` | `0x41` |
| `ldr` at 0x110f3c | load | 4 | `abs 0x1110bc` | `-` |
| `ldr` at 0x110f40 | load | 4 | `abs 0x31e424` | `-` |
| `ldr` at 0x110f44 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x110f54 | store | 1 | `sp -0x19` | `0x42` |
| `ldr` at 0x110f5c | load | 4 | `abs 0x1110d0` | `-` |
| `str` at 0x110f60 | store | 4 | `sp -0x2c` | `0x18dedc` |
| `ldr` at 0x110f64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110f68 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0x110f70 | load | 4 | `abs 0x1110c8` | `-` |
| `str` at 0x110f84 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110f8c | load | 4 | `abs 0x1110c0` | `-` |
| `ldr` at 0x110f90 | load | 4 | `abs 0x1110d4` | `-` |
| `str` at 0x110f9c | store | 4 | `sp -0x18` | `ret(0x2046c)` |
| `ldr` at 0x110fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x110fb4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110fc0 | load | 4 | `abs 0x1110d8` | `-` |
| `str` at 0x110fc4 | store | 4 | `sp -0x2c` | `0x18de3c` |
| `ldr` at 0x110fc8 | load | 4 | `abs 0x1110c0` | `-` |
| `str` at 0x110fcc | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x110fd0 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x110fd8 | load | 4 | `abs 0x1110c8` | `-` |
| `ldr` at 0x110fdc | load | 4 | `abs 0x1110dc` | `-` |
| `str` at 0x110fec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110ff8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11101c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x111028 | load | 4 | `abs 0x1110e0` | `-` |
| `str` at 0x11102c | store | 4 | `sp -0x2c` | `0x18df08` |
| `ldr` at 0x111030 | load | 4 | `abs 0x1110c0` | `-` |
| `str` at 0x111034 | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x111038 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x111040 | load | 4 | `abs 0x1110c8` | `-` |
| `ldr` at 0x111044 | load | 4 | `abs 0x1110e4` | `-` |
| `str` at 0x111054 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x11105c | load | 4 | `abs 0x1110bc` | `-` |
| `ldrb` at 0x111060 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x111064 | load | 4 | `abs 0x1110e8` | `-` |
| `ldr` at 0x11106c | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x111078 | load | 4 | `abs 0x1110bc` | `-` |
| `ldrb` at 0x11107c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x111080 | load | 4 | `abs 0x1110e8` | `-` |
| `ldr` at 0x111088 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x111094 | load | 4 | `(unknown)` | `-` |
| `str` at 0x11109c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1110a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1110ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1110b0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1110b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1110b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1110b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110ea4 | `(unknown callee) 0x20100` | `0x18de68` | `[0x31e434]+4` | `0x4` | `[sp-20]` |
| `bl` at 0x110eb4 | `(unknown callee) 0x20184` | `ret(0x20100)` | `?` | `?` | `ret(0x20100)` |
| `bl` at 0x110ec8 | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `[ret(0x20184)]` |
| `bl` at 0x110ed4 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110f00 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x21a` | `0x0` |
| `bl` at 0x110f7c | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x224` | `0x0` |
| `bl` at 0x110f94 | `(unknown callee) 0x2046c` | `0x18de68` | `0x241` | `?` | `?` |
| `bl` at 0x110fac | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0x110fb8 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110fe4 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x229` | `0x0` |
| `bl` at 0x111004 | `(unknown callee) 0x2007c` | `ret(0x2046c)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0x111014 | `(unknown callee) 0x20184` | `ret(0x2007c)` | `?` | `?` | `ret(0x2007c)` |
| `bl` at 0x111020 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x11104c | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x22f` | `0x0` |
| `bl` at 0x1110a4 | `(unknown callee) 0x200b8` | `[sp-24]` | `0x31e418` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x110e9c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bge` at 0x110eb0 | `ret(0x20100) >= 0x0 (signed, 32-bit)` | `ret(0x20100) < 0x0 (signed, 32-bit)` |
| `beq` at 0x110ec4 | `[ret(0x20184)] == 0x2` | `[ret(0x20184)] != 0x2` |
| `bne` at 0x110f2c | `[0x31e420] != [sp-20]` | `[0x31e420] == [sp-20]` |
| `bne` at 0x110f4c | `[0x31e424] != [sp-20]` | `[0x31e424] == [sp-20]` |
| `bge` at 0x110fa8 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `beq` at 0x111010 | `ret(0x2007c) == 0x1` | `ret(0x2007c) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1110b8 | `r0` | `?` |

## `flash_get_default_image`

- `flash_get_default_image`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1110ec to 0x111324.
  Established. Word stores at memory targets [0x31e434] and [0x31e434] plus 4 retain unknown values (`str` at 0x111114, `str` at 0x111140).
  The result from unnamed 0x2046c is stored as a word at entry sp minus 0x14 (`bl` at 0x11114c, `str` at 0x111154).
  Distinct paths load words from absolute addresses 0x31e420 and 0x31e424 and store the respective values at entry sp minus 0x18 (`ldr` at 0x111244, `str` at 0x111248, `ldr` at 0x111260, `str` at 0x111264).
  A computed word store later has an unknown target and value (`str` at 0x1112dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1110fc | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111100 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x111104 | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111108 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x11110c | load | 4 | `[0x31e434] +0x0` | `-` |
| `str` at 0x111114 | store | 4 | `[0x31e434] +0x0` | `?` |
| `ldr` at 0x111118 | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111120 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x111128 | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111130 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x111138 | load | 4 | `[0x31e434] +0x4` | `-` |
| `str` at 0x111140 | store | 4 | `[0x31e434] +0x4` | `?` |
| `ldr` at 0x111144 | load | 4 | `abs 0x111300` | `-` |
| `str` at 0x111154 | store | 4 | `sp -0x14` | `ret(0x2046c)` |
| `ldr` at 0x111158 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11116c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x11117c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x11118c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x111198 | load | 4 | `abs 0x111304` | `-` |
| `str` at 0x11119c | store | 4 | `sp -0x2c` | `0x18df34` |
| `ldr` at 0x1111a0 | load | 4 | `abs 0x111300` | `-` |
| `str` at 0x1111a4 | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x1111a8 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x1111b0 | load | 4 | `abs 0x111308` | `-` |
| `ldr` at 0x1111b4 | load | 4 | `abs 0x11130c` | `-` |
| `str` at 0x1111c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x1111d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1111f4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x111200 | load | 4 | `abs 0x111310` | `-` |
| `str` at 0x111204 | store | 4 | `sp -0x2c` | `0x18df64` |
| `ldr` at 0x111208 | load | 4 | `abs 0x111300` | `-` |
| `str` at 0x11120c | store | 4 | `sp -0x28` | `0x18de68` |
| `str` at 0x111210 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0x111218 | load | 4 | `abs 0x111308` | `-` |
| `ldr` at 0x11121c | load | 4 | `abs 0x111314` | `-` |
| `str` at 0x11122c | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x111234 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x111240 | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111244 | load | 4 | `abs 0x31e420` | `-` |
| `str` at 0x111248 | store | 4 | `sp -0x18` | `[0x31e420]` |
| `ldrb` at 0x111250 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x11125c | load | 4 | `abs 0x1112fc` | `-` |
| `ldr` at 0x111260 | load | 4 | `abs 0x31e424` | `-` |
| `str` at 0x111264 | store | 4 | `sp -0x18` | `[0x31e424]` |
| `ldr` at 0x11126c | load | 4 | `abs 0x111318` | `-` |
| `str` at 0x111270 | store | 4 | `sp -0x2c` | `0x18df98` |
| `ldrb` at 0x111274 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x111278 | store | 4 | `sp -0x28` | `[sp-25]:1` |
| `ldr` at 0x111280 | load | 4 | `abs 0x111308` | `-` |
| `ldr` at 0x111284 | load | 4 | `abs 0x11131c` | `-` |
| `str` at 0x111294 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x11129c | load | 4 | `abs 0x1112fc` | `-` |
| `ldrb` at 0x1112a0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1112a4 | load | 4 | `abs 0x111320` | `-` |
| `ldr` at 0x1112ac | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x1112b8 | load | 4 | `abs 0x1112fc` | `-` |
| `ldrb` at 0x1112bc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1112c0 | load | 4 | `abs 0x111320` | `-` |
| `ldr` at 0x1112c8 | load | 4 | `abs 0x31e434` | `-` |
| `ldr` at 0x1112d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1112dc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1112e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1112e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1112ec | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x1112f0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1112f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1112f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1112f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11114c | `(unknown callee) 0x2046c` | `0x18de68` | `0x0` | `0x4` | `?` |
| `bl` at 0x111164 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `?` | `ret(0x2046c)` |
| `bl` at 0x111184 | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `[ret(0x20184)]` |
| `bl` at 0x111190 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x1111bc | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x24d` | `0x0` |
| `bl` at 0x1111dc | `(unknown callee) 0x20628` | `ret(0x2046c)` | `sp-25` | `0x1` | `sp-25` |
| `bl` at 0x1111ec | `(unknown callee) 0x20184` | `ret(0x20628)` | `?` | `?` | `ret(0x20628)` |
| `bl` at 0x1111f8 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x111224 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x253` | `0x0` |
| `bl` at 0x11128c | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x25d` | `0x0` |
| `bl` at 0x1112e4 | `(unknown callee) 0x200b8` | `[sp-20]` | `0x31e418` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x111160 | `ret(0x2046c) >= 0x0 (signed, 32-bit)` | `ret(0x2046c) < 0x0 (signed, 32-bit)` |
| `bne` at 0x111174 | `[ret(0x20184)] != 0x2` | `[ret(0x20184)] == 0x2` |
| `beq` at 0x1111e8 | `ret(0x20628) == 0x1` | `ret(0x20628) != 0x1` |
| `bne` at 0x11123c | `[sp-25]:1 != 0x41` | `[sp-25]:1 == 0x41` |
| `bne` at 0x111258 | `[sp-25]:1 != 0x42` | `[sp-25]:1 == 0x42` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1112f8 | `r0` | `?` |

## `FlashValidateImage`

- `FlashValidateImage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3680 to 0xe3b08.
  Established. A byte store writes 0 to absolute address 0x1b0e2c (`strb` at 0xe36b8).
  A result from unnamed 0x20010 is stored as a word at entry sp minus 0x2c (`bl` at 0xe36e4, `str` at 0xe36ec).
  The word store at entry sp minus 0x3c writes the sum [sp-44] plus [sp-36] (`str` at 0xe3764).
  Two word stores write 0x1b0e2c to memory at [sp] on their respective paths (`str` at 0xe3a78, `str` at 0xe3abc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3690 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3694 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3698 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe369c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe36a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe36a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe36ac | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe36b0 | load | 4 | `abs 0xe3ad4` | `-` |
| `strb` at 0xe36b8 | store | 1 | `abs 0x1b0e2c` | `0x0` |
| `ldr` at 0xe36bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe36c8 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe36cc | load | 4 | `abs 0xe3ad8` | `-` |
| `ldr` at 0xe36d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe36dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe36e0 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe36ec | store | 4 | `sp -0x2c` | `ret(0x20010)` |
| `ldr` at 0xe36f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe36f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3704 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe370c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe371c | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3720 | load | 4 | `abs 0xe3adc` | `-` |
| `ldr` at 0xe3724 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3730 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3744 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3748 | load | 4 | `abs 0xe3ae0` | `-` |
| `ldr` at 0xe374c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3758 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe375c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3764 | store | 4 | `sp -0x3c` | `([sp-44] + [sp-36])` |
| `ldr` at 0xe3768 | load | 4 | `sp -0x3c` | `-` |
| `ldrh` at 0xe376c | load | 2 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0xe3784 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe3788 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe3794 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3798 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe37a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe37a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe37b8 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe37bc | load | 4 | `abs 0xe3ae4` | `-` |
| `ldr` at 0xe37c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe37cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe37d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe37ec | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe37f0 | load | 4 | `abs 0xe3ae8` | `-` |
| `ldr` at 0xe37f4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3804 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe380c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe3810 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe3814 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xe3818 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe381c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe3828 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe382c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe383c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3844 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x20010))` |
| `ldr` at 0xe3848 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3850 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xe3854 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe385c | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xe3864 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3870 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3874 | load | 4 | `abs 0xe3aec` | `-` |
| `ldr` at 0xe3880 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe388c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xe3890 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe3894 | store | 4 | `[sp+4] +0x0` | `[sp-60]` |
| `ldr` at 0xe3898 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe389c | load | 4 | `[sp-60] +0x18` | `-` |
| `str` at 0xe38a8 | store | 4 | `sp -0x2c` | `ret(0x20010)` |
| `ldr` at 0xe38ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe38b8 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe38bc | load | 4 | `abs 0xe3af0` | `-` |
| `ldr` at 0xe38c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe38cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe38d4 | store | 4 | `sp -0x40` | `(ret(0x20010) + [sp-36])` |
| `ldr` at 0xe38d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe38dc | store | 4 | `sp -0x30` | `[sp-20]` |
| `str` at 0xe38e4 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe38ec | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe38f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe38f4 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0xe3904 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe3910 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3914 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xe3924 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe392c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x20010))` |
| `ldr` at 0xe3930 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3938 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xe393c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3944 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xe394c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3950 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xe3960 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe396c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3970 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0xe3984 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3988 | load | 4 | `abs 0xe3af4` | `-` |
| `str` at 0xe3994 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xe399c | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe39a0 | load | 4 | `abs 0xe3af8` | `-` |
| `ldr` at 0xe39ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe39b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe39bc | load | 4 | `[sp-64] +0x4` | `-` |
| `str` at 0xe39c8 | store | 4 | `sp -0x34` | `ret(0x20010)` |
| `str` at 0xe39d0 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe39d8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe39dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe39e0 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xe39e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe39e8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe39f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe39f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3a08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3a10 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x20010))` |
| `ldr` at 0xe3a14 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3a1c | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0xe3a20 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3a28 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0xe3a30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3a3c | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3a40 | load | 4 | `abs 0xe3afc` | `-` |
| `ldr` at 0xe3a4c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe3a58 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe3a5c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe3a60 | store | 4 | `[sp+8] +0x0` | `[sp-64]` |
| `ldr` at 0xe3a64 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3a70 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3a74 | load | 4 | `abs 0xe3ad4` | `-` |
| `str` at 0xe3a78 | store | 4 | `[sp] +0x0` | `0x1b0e2c` |
| `str` at 0xe3a80 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xe3a88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3a94 | load | 4 | `abs 0xe3b00` | `-` |
| `ldr` at 0xe3a98 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xe3a9c | load | 4 | `abs 0xe3b04` | `-` |
| `ldr` at 0xe3aa0 | load | 4 | `abs 0xe3ad4` | `-` |
| `ldr` at 0xe3aa8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3ab4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3ab8 | load | 4 | `abs 0xe3ad4` | `-` |
| `str` at 0xe3abc | store | 4 | `[sp] +0x0` | `0x1b0e2c` |
| `str` at 0xe3ac4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe3ac8 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xe3ad0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3ad0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3ad0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe36d4 | `(unknown callee) 0x205bc` | `0x1b0e2c` | `0x183f74` | `[sp-20]` | `[sp-20]` |
| `bl` at 0xe36e4 | `(unknown callee) 0x20010` | `[[sp-20]]` | `r1` | `0x1b0e2c` | `[sp-20]` |
| `bl` at 0xe3728 | `(unknown callee) 0x205bc` | `0x1b0e2c` | `0x183f94` | `ret(0x20010)` | `?` |
| `bl` at 0xe3734 | `(unknown callee) 0x11053c` | `ret(0x20010)` | `?` | `([sp-24] + [sp-28])` | `ret(0x20010)` |
| `bl` at 0xe3750 | `(unknown callee) 0x205bc` | `0x1b0e2c` | `0x183fb4` | `[sp-44]` | `ret(0x11053c)` |
| `bl` at 0xe3774 | `(unknown callee) 0x202ec` | `[([sp-44] + [sp-36])+2]:2` | `?` | `[sp-44]` | `[([sp-44] + [sp-36])+2]:2` |
| `bl` at 0xe37c4 | `(unknown callee) 0x205bc` | `0x1b0e2c` | `0x183ff0` | `[sp-52]` | `?` |
| `bl` at 0xe37dc | `(unknown callee) 0x11053c` | `([sp-44] + [sp-52])` | `([sp-44] + [sp-52])` | `[sp-44]` | `([sp-44] + [sp-52])` |
| `bl` at 0xe37f8 | `(unknown callee) 0x205bc` | `0x1b0e2c` | `0x184014` | `[sp-52]` | `ret(0x11053c)` |
| `bl` at 0xe3830 | `(unknown callee) 0x20010` | `?` | `?` | `?` | `?` |
| `bl` at 0xe3878 | `(unknown callee) 0x20634` | `0x1b0e2c` | `0x18405c` | `?` | `?` |
| `bl` at 0xe38a0 | `(unknown callee) 0x20010` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0xe38c0 | `(unknown callee) 0x20634` | `0x1b0e2c` | `0x184078` | `?` | `ret(0x20010)` |
| `bl` at 0xe38f8 | `(unknown callee) 0x20010` | `[[sp-60]+12]` | `?` | `ret(0x20010)` | `[sp-60]` |
| `bl` at 0xe3918 | `(unknown callee) 0x20010` | `[[sp-48]]` | `?` | `ret(0x20010)` | `[sp-48]` |
| `bl` at 0xe3954 | `(unknown callee) 0x20010` | `[[sp-60]+32]` | `?` | `ret(0x20010)` | `[sp-60]` |
| `bl` at 0xe3974 | `(unknown callee) 0x20010` | `[[sp-60]+32]` | `?` | `ret(0x20010)` | `[sp-60]` |
| `bl` at 0xe398c | `(unknown callee) 0x20634` | `0x1b0e2c` | `0x184094` | `?` | `ret(0x20010)` |
| `bl` at 0xe39a4 | `(unknown callee) 0x20634` | `0x1b0e2c` | `0x1840a8` | `?` | `ret(0x20010)` |
| `bl` at 0xe39c0 | `(unknown callee) 0x20010` | `[[sp-64]+4]` | `?` | `?` | `[sp-64]` |
| `bl` at 0xe39fc | `(unknown callee) 0x20010` | `?` | `?` | `?` | `?` |
| `bl` at 0xe3a44 | `(unknown callee) 0x20634` | `0x1b0e2c` | `0x1840bc` | `?` | `?` |
| `bl` at 0xe3aa4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x1840dc` | `0x1b0e2c` | `0x1a288c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe36c4 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xe36fc | `ret(0x20010) < [sp-24] (unsigned, 32-bit)` | `ret(0x20010) >= [sp-24] (unsigned, 32-bit)` |
| `blo` at 0xe3714 | `([sp-24] + [sp-28]) < ret(0x20010) (unsigned, 32-bit)` | `([sp-24] + [sp-28]) >= ret(0x20010) (unsigned, 32-bit)` |
| `bne` at 0xe3740 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xe3790 | `[sp-52] <= 0x1f (unsigned, 32-bit)` | `[sp-52] > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xe37b0 | `([sp-44] + [sp-52]) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `([sp-44] + [sp-52]) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0xe37e8 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xe3824 | `C == 1` | `not (C == 1)` |
| `beq` at 0xe386c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe3888 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `bne` at 0xe38b4 | `ret(0x20010) != 0x0` | `ret(0x20010) == 0x0` |
| `bhs` at 0xe390c | `[sp-56] >= ret(0x20010) (unsigned, 32-bit)` | `[sp-56] < ret(0x20010) (unsigned, 32-bit)` |
| `beq` at 0xe3968 | `ret(0x20010) == [sp-40]` | `ret(0x20010) != [sp-40]` |
| `bne` at 0xe3980 | `ret(0x20010) != 0x0` | `ret(0x20010) == 0x0` |
| `beq` at 0xe39b4 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `bhs` at 0xe39f0 | `C == 1` | `not (C == 1)` |
| `beq` at 0xe3a38 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe3a54 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xe3a6c | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0xe3a90 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xe3ab0 | `[sp] == 0x0` | `[sp] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3ad0 | `r0` | `[sp-68]` |

## `FlashInit`

- `FlashInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3b08 to 0xe3c9c.
  Established. A word store updates absolute address 0x1b0e24 with its separately loaded value plus 1 (`ldr` at 0xe3b4c, `str` at 0xe3b54).
  Words 0x19 and 0x14 are stored at entry sp minus 0x18 and 0x1c respectively (`str` at 0xe3b6c, `str` at 0xe3b74).
  The result from unnamed 0xfb6ec is stored as a word at entry sp minus 0x14 (`bl` at 0xe3b84, `str` at 0xe3b8c).
  The final call to unnamed 0x22540 has printed r0-r3 candidates 0x31b890, 0x184174, 0xe3c9c and 0 (`bl` at 0xe3c58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe3b18 | load | 4 | `abs 0xe3c64` | `-` |
| `ldr` at 0xe3b1c | load | 4 | `abs 0x1b0e24` | `-` |
| `ldr` at 0xe3b28 | load | 4 | `abs 0xe3c68` | `-` |
| `str` at 0xe3b2c | store | 4 | `sp -0x30` | `0x1840f0` |
| `ldr` at 0xe3b34 | load | 4 | `abs 0xe3c6c` | `-` |
| `ldr` at 0xe3b38 | load | 4 | `abs 0xe3c70` | `-` |
| `ldr` at 0xe3b44 | load | 4 | `abs 0xe3c64` | `-` |
| `ldr` at 0xe3b48 | load | 4 | `abs 0xe3c64` | `-` |
| `ldr` at 0xe3b4c | load | 4 | `abs 0x1b0e24` | `-` |
| `str` at 0xe3b54 | store | 4 | `abs 0x1b0e24` | `[0x1b0e24]+1` |
| `ldr` at 0xe3b58 | load | 4 | `abs 0xe3c74` | `-` |
| `str` at 0xe3b6c | store | 4 | `sp -0x18` | `0x19` |
| `str` at 0xe3b74 | store | 4 | `sp -0x1c` | `0x14` |
| `ldr` at 0xe3b78 | load | 4 | `abs 0xe3c78` | `-` |
| `ldr` at 0xe3b7c | load | 4 | `abs 0xe3c7c` | `-` |
| `str` at 0xe3b8c | store | 4 | `sp -0x14` | `ret(0xfb6ec)` |
| `ldr` at 0xe3b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3b9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3ba8 | load | 4 | `abs 0xe3c80` | `-` |
| `str` at 0xe3bac | store | 4 | `sp -0x30` | `0x18412c` |
| `str` at 0xe3bb0 | store | 4 | `sp -0x2c` | `ret(0x219a4)` |
| `ldr` at 0xe3bb8 | load | 4 | `abs 0xe3c6c` | `-` |
| `ldr` at 0xe3bbc | load | 4 | `abs 0xe3c84` | `-` |
| `ldr` at 0xe3bc8 | load | 4 | `abs 0xe3c74` | `-` |
| `ldr` at 0xe3bcc | load | 4 | `abs 0xe3c74` | `-` |
| `ldr` at 0xe3bd0 | load | 4 | `abs 0x31b888` | `-` |
| `ldr` at 0xe3bd8 | load | 4 | `abs 0xe3c78` | `-` |
| `str` at 0xe3be4 | store | 4 | `sp -0x14` | `ret(0xfa264)` |
| `ldr` at 0xe3be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3c00 | load | 4 | `abs 0xe3c88` | `-` |
| `str` at 0xe3c04 | store | 4 | `sp -0x30` | `0x184150` |
| `str` at 0xe3c08 | store | 4 | `sp -0x2c` | `ret(0x219a4)` |
| `ldr` at 0xe3c10 | load | 4 | `abs 0xe3c6c` | `-` |
| `ldr` at 0xe3c14 | load | 4 | `abs 0xe3c8c` | `-` |
| `str` at 0xe3c24 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xe3c2c | store | 4 | `sp -0x2c` | `0x400` |
| `ldr` at 0xe3c30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3c34 | store | 4 | `sp -0x28` | `[sp-24]` |
| `ldr` at 0xe3c38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3c3c | store | 4 | `sp -0x24` | `[sp-28]` |
| `str` at 0xe3c44 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xe3c48 | load | 4 | `abs 0xe3c90` | `-` |
| `ldr` at 0xe3c4c | load | 4 | `abs 0xe3c94` | `-` |
| `ldr` at 0xe3c50 | load | 4 | `abs 0xe3c98` | `-` |
| `ldm` at 0xe3c60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe3c60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe3c60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3b40 | `(unknown callee) 0x22070` | `0x9` | `0x184108` | `0xd8c` | `0x0` |
| `bl` at 0xe3b64 | `(unknown callee) 0xfb5cc` | `0x31b888` | `0x10` | `0x0` | `[0x1b0e24]+1` |
| `bl` at 0xe3b84 | `(unknown callee) 0xfb6ec` | `0x31b894` | `0x184124` | `0x5` | `0x14` |
| `bl` at 0xe3ba0 | `(unknown callee) 0x219a4` | `ret(0xfb6ec)` | `?` | `?` | `ret(0xfb6ec)` |
| `bl` at 0xe3bc4 | `(unknown callee) 0x22070` | `0x9` | `0x184108` | `0xda4` | `0x0` |
| `bl` at 0xe3bdc | `(unknown callee) 0xfa264` | `0x31b888` | `[0x31b888]` | `0x1` | `0x31b894` |
| `bl` at 0xe3bf8 | `(unknown callee) 0x219a4` | `ret(0xfa264)` | `?` | `?` | `ret(0xfa264)` |
| `bl` at 0xe3c1c | `(unknown callee) 0x22070` | `0x9` | `0x184108` | `0xdad` | `0x0` |
| `bl` at 0xe3c58 | `(unknown callee) 0x22540` | `0x31b890` | `0x184174` | `0xe3c9c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3b24 | `[0x1b0e24] == 0x0` | `[0x1b0e24] != 0x0` |
| `beq` at 0xe3b98 | `ret(0xfb6ec) == 0x0` | `ret(0xfb6ec) != 0x0` |
| `beq` at 0xe3bf0 | `ret(0xfa264) == 0x0` | `ret(0xfa264) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe3c60 | `r0` | `ret(0x22540)` |

## `FlashTask`

- `FlashTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3c9c to 0xe3d70.
  Established. The result from unnamed 0xfb734 is stored as a word at entry sp minus 0x1c (`bl` at 0xe3cc4, `str` at 0xe3ccc).
  A word store updates absolute address 0x1b0e28 with its separately loaded value minus 1 (`ldr` at 0xe3cf8, `str` at 0xe3d00).
  A byte loaded from memory at [sp-32] plus 0x16 supplies the nonzero test selecting the branch to 0xe3d50 (`ldrb` at 0xe3d0c, `bne` at 0xe3d14).
  The final unconditional transfer returns control to 0xe3cb4 (`b` at 0xe3d58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3cac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3cb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe3cbc | load | 4 | `abs 0xe3d5c` | `-` |
| `str` at 0xe3ccc | store | 4 | `sp -0x1c` | `ret(0xfb734)` |
| `ldr` at 0xe3cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3ce0 | load | 4 | `abs 0xe3d60` | `-` |
| `ldr` at 0xe3ce4 | load | 4 | `abs 0x1b0e28` | `-` |
| `ldr` at 0xe3cf0 | load | 4 | `abs 0xe3d60` | `-` |
| `ldr` at 0xe3cf4 | load | 4 | `abs 0xe3d60` | `-` |
| `ldr` at 0xe3cf8 | load | 4 | `abs 0x1b0e28` | `-` |
| `str` at 0xe3d00 | store | 4 | `abs 0x1b0e28` | `[0x1b0e28]-1` |
| `ldr` at 0xe3d08 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe3d0c | load | 1 | `[sp-32] +0x16` | `-` |
| `ldr` at 0xe3d18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3d24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3d30 | load | 4 | `abs 0xe3d64` | `-` |
| `str` at 0xe3d34 | store | 4 | `sp -0x28` | `0x18417c` |
| `str` at 0xe3d38 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0xe3d40 | load | 4 | `abs 0xe3d68` | `-` |
| `ldr` at 0xe3d44 | load | 4 | `abs 0xe3d6c` | `-` |
| `ldr` at 0xe3d50 | load | 4 | `sp -0x20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3cc4 | `(unknown callee) 0xfb734` | `sp-32` | `0x31b894` | `?` | `sp-32` |
| `bl` at 0xe3d1c | `(unknown callee) 0xe3d98` | `[sp-32]` | `?` | `?` | `[[sp-32]+22]:1` |
| `bl` at 0xe3d28 | `(unknown callee) 0x219a4` | `ret(0xfb734)` | `?` | `?` | `ret(0xfb734)` |
| `bl` at 0xe3d4c | `(unknown callee) 0x22070` | `0x3` | `0x184108` | `0xdd9` | `0x0` |
| `bl` at 0xe3d54 | `(unknown callee) 0xfb794` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3cd8 | `ret(0xfb734) == 0x0` | `ret(0xfb734) != 0x0` |
| `beq` at 0xe3cec | `[0x1b0e28] == 0x0` | `[0x1b0e28] != 0x0` |
| `bne` at 0xe3d14 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `FlashIRouter`

- `FlashIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3d98 to 0xe4b24.
  Established. The result from unnamed 0x6dc0c is stored as a word at entry sp minus 0x1c (`bl` at 0xe3db4, `str` at 0xe3dbc).
  A halfword loaded from memory at [sp-20] plus 0x22 is stored as a word at entry sp minus 0x20 (`ldrh` at 0xe3dc4, `str` at 0xe3dc8).
  The indirect transfer at 0xe3df4 has an unknown taken target, and omitted effects remain unknown (`ldrls` at 0xe3df4).
  On the known fallthrough, a word store writes 0x18419c at entry sp minus 0x19c (`str` at 0xe4ad4).
  The subsequent call to unnamed 0x22070 has printed r0-r3 candidates 4, 0x184108, 0xf20 and 0 (`bl` at 0xe4af0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3da8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3db0 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0xe3dbc | store | 4 | `sp -0x1c` | `ret(0x6dc0c)` |
| `ldr` at 0xe3dc0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xe3dc4 | load | 2 | `[sp-20] +0x22` | `-` |
| `str` at 0xe3dc8 | store | 4 | `sp -0x20` | `[[sp-20]+34]:2` |
| `str` at 0xe3dd0 | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0xe3dd4 | load | 4 | `abs 0xe4afc` | `-` |
| `ldr` at 0xe3dd8 | load | 4 | `abs 0x1b0e90` | `-` |
| `ldr` at 0xe3de8 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xe3df4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4ad0 | load | 4 | `abs 0xe4b1c` | `-` |
| `str` at 0xe4ad4 | store | 4 | `sp -0x19c` | `0x18419c` |
| `ldr` at 0xe4ad8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4adc | store | 4 | `sp -0x198` | `[sp-32]` |
| `ldr` at 0xe4ae4 | load | 4 | `abs 0xe4b20` | `-` |
| `ldm` at 0xe4af8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe4af8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe4af8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3db4 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0xe3de4 | `(unknown callee) 0xe3d70` | `ret(0x6dc0c)` | `?` | `?` | `[0x1b0e90]` |
| `bl` at 0xe4af0 | `(unknown callee) 0x22070` | `0x4` | `0x184108` | `0xf20` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3de0 | `[0x1b0e90] != 0x0` | `[0x1b0e90] == 0x0` |
| `ldrls` at 0xe3df4 | `[sp-32]-114 <= 0x4 (unsigned, 32-bit)` | `[sp-32]-114 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe4af8 | `r0` | `ret(0x22070)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xe3df4).

## `bootAddressIsValid`

- `bootAddressIsValid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x108300 to 0x108420.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x108310).
  One word loaded from absolute address 0x31e418 is compared unsigned with entry r0, selecting the branch to 0x108368 when the loaded value is greater (`ldr` at 0x108334, `bhi` at 0x108340).
  A separate load from that address supplies the comparison selecting 0x108368 when its value plus 0x80000 is at most entry r0 (`ldr` at 0x108348, `bls` at 0x108358).
  Word stores on distinct paths place 1 or 0 at entry sp minus 0x1c (`str` at 0x108360, `str` at 0x10840c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x108310 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x108314 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x108328 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x108330 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x108334 | load | 4 | `abs 0x31e418` | `-` |
| `ldr` at 0x108338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x108344 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x108348 | load | 4 | `abs 0x31e418` | `-` |
| `ldr` at 0x108350 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x108360 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x108368 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x10836c | load | 4 | `abs 0x31e420` | `-` |
| `ldr` at 0x108378 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x10837c | load | 4 | `abs 0x31e420` | `-` |
| `ldr` at 0x108380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10838c | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x108390 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x108394 | load | 4 | `abs 0x31e420` | `-` |
| `ldr` at 0x108398 | load | 4 | `abs 0x31e430` | `-` |
| `ldr` at 0x1083a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1083b0 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x1083b8 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x1083bc | load | 4 | `abs 0x31e424` | `-` |
| `ldr` at 0x1083c8 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x1083cc | load | 4 | `abs 0x31e424` | `-` |
| `ldr` at 0x1083d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1083dc | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x1083e0 | load | 4 | `abs 0x10841c` | `-` |
| `ldr` at 0x1083e4 | load | 4 | `abs 0x31e424` | `-` |
| `ldr` at 0x1083e8 | load | 4 | `abs 0x31e430` | `-` |
| `ldr` at 0x1083f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x108400 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x10840c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x108410 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x108418 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x108418 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x108418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x108320 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x108340 | `[0x31e418] > r0 (unsigned, 32-bit)` | `[0x31e418] <= r0 (unsigned, 32-bit)` |
| `bls` at 0x108358 | `[0x31e418]+524288 <= r0 (unsigned, 32-bit)` | `[0x31e418]+524288 > r0 (unsigned, 32-bit)` |
| `beq` at 0x108374 | `[0x31e420] == 0x0` | `[0x31e420] != 0x0` |
| `bhi` at 0x108388 | `[0x31e420] > r0 (unsigned, 32-bit)` | `[0x31e420] <= r0 (unsigned, 32-bit)` |
| `bls` at 0x1083a8 | `([0x31e420] + [0x31e430]) <= r0 (unsigned, 32-bit)` | `([0x31e420] + [0x31e430]) > r0 (unsigned, 32-bit)` |
| `beq` at 0x1083c4 | `[0x31e424] == 0x0` | `[0x31e424] != 0x0` |
| `bhi` at 0x1083d8 | `[0x31e424] > r0 (unsigned, 32-bit)` | `[0x31e424] <= r0 (unsigned, 32-bit)` |
| `bls` at 0x1083f8 | `([0x31e424] + [0x31e430]) <= r0 (unsigned, 32-bit)` | `([0x31e424] + [0x31e430]) > r0 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x108418 | `r0` | `?` |

## `bootAddressNormalize`

- `bootAddressNormalize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x108420 to 0x108488.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x108430, `str` at 0x108434).
  The computed word stored at entry sp minus 0x1c has an unknown value (`str` at 0x108440).
  One load from that location selects the branch to 0x108460 when its value is 1 (`ldr` at 0x108444, `beq` at 0x10844c).
  A separate load selects the same destination when its value is 2 (`ldr` at 0x108450, `beq` at 0x108458).
  The later word store at entry sp minus 0x14 also has an unknown value (`str` at 0x108474).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x108430 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x108434 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x108438 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x108440 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x108444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108450 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x108460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x108468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x108474 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x108478 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x108484 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x108484 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x108484 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10844c | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `beq` at 0x108458 | `[sp-28] == 0x2` | `[sp-28] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x108484 | `r0` | `?` |

## `bootValidate`

- `bootValidate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x108488 to 0x1085dc.
  Established. The result from unnamed 0x1086c8 is stored as a word at entry sp minus 0x28 (`bl` at 0x1084bc, `str` at 0x1084c4).
  One halfword load from memory at [sp-40] plus 2 selects 0x108514 for an unsigned value at most 0x1f (`ldrh` at 0x1084f4, `bls` at 0x1084fc).
  A separate halfword load from that target selects 0x108514 for an unsigned value greater than 0x400 (`ldrh` at 0x108504, `bhi` at 0x10850c).
  Separate paths store the result from unnamed 0x1085dc plus 4 or the unchanged result at entry sp minus 0x20 (`bl` at 0x1084e4, `str` at 0x108538, `str` at 0x108544).
  A later word store at entry sp minus 0x1c writes [sp-28] plus the result from unnamed 0x20010 (`bl` at 0x10858c, `str` at 0x1085a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x108498 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10849c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1084a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1084b0 | store | 4 | `sp -0x18` | `sp-92` |
| `ldr` at 0x1084b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1084b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1084c4 | store | 4 | `sp -0x28` | `ret(0x1086c8)` |
| `ldr` at 0x1084c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1084d8 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x1084e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1084ec | store | 4 | `sp -0x2c` | `ret(0x1085dc)` |
| `ldr` at 0x1084f0 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x1084f4 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x108500 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x108504 | load | 2 | `[sp-40] +0x2` | `-` |
| `str` at 0x108518 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x108520 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x108524 | load | 2 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x108530 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x108538 | store | 4 | `sp -0x20` | `ret(0x1085dc)+4` |
| `ldr` at 0x108540 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x108544 | store | 4 | `sp -0x20` | `ret(0x1085dc)` |
| `ldr` at 0x108548 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x10854c | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x108550 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x108558 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x108560 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x108564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x108568 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x108578 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10857c | load | 4 | `(unknown)` | `-` |
| `str` at 0x108584 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x108598 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1085a0 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x20010))` |
| `ldr` at 0x1085a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1085b8 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0x1085c4 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x1085c8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x1085cc | store | 4 | `sp -0x64` | `[sp-104]` |
| `ldr` at 0x1085d0 | load | 4 | `sp -0x64` | `-` |
| `ldm` at 0x1085d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1085d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1085d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1084bc | `(unknown callee) 0x1086c8` | `r0` | `?` | `r2` | `?` |
| `bl` at 0x1084e4 | `(unknown callee) 0x1085dc` | `[sp-20]` | `?` | `?` | `ret(0x1086c8)` |
| `bl` at 0x10858c | `(unknown callee) 0x20010` | `?` | `?` | `sp-32` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1084a8 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0x1084d0 | `ret(0x1086c8) != 0x0` | `ret(0x1086c8) == 0x0` |
| `bls` at 0x1084fc | `[[sp-40]+2]:2 <= 0x1f (unsigned, 32-bit)` | `[[sp-40]+2]:2 > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0x10850c | `[[sp-40]+2]:2 > 0x400 (unsigned, 32-bit)` | `[[sp-40]+2]:2 <= 0x400 (unsigned, 32-bit)` |
| `bhi` at 0x10852c | `[[sp-40]]:2 > 0x2 (unsigned, 32-bit)` | `[[sp-40]]:2 <= 0x2 (unsigned, 32-bit)` |
| `bhs` at 0x108570 | `C == 1` | `not (C == 1)` |
| `bne` at 0x1085b0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1085d8 | `r0` | `[sp-100]` |
