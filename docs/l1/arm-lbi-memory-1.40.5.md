# ARM LBI memory and JChip transfer routines in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
These routines have no unresolved computed transfers. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/lbi-memory-facts.txt) retain all tool sections.

## `LBIInit`

- `LBIInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38878 to 0x38a54.
  Established. The unnamed `0xf61cc` call has `0x0`, `sp-148`, `sp-144`, `sp-148` (`bl` at 0x38934);
  its nonzero result takes the branch to `0x38970` (`bne` at 0x38940). A later unnamed `0xf6550`
  call has `[sp-20]`, `0x1081c8`, `0x100`, `sp-144` (`bl` at 0x389c8). Included four-byte stack
  assignments at `sp -0x9c` retain `0xfffff060` and `ret(0x38a54)` (`str` at 0x388c8, `str` at
  0x38a38). Initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38888 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3888c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x38894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x388a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x388ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x388b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x388c4 | load | 4 | `abs 0x38a44` | `-` |
| `str` at 0x388c8 | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x388d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x388ec | load | 4 | `abs 0x38a44` | `-` |
| `str` at 0x388f0 | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x388f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38904 | store | 4 | `sp -0x98` | `ret(0xe99c4)` |
| `ldr` at 0x38908 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x38914 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x38918 | store | 4 | `sp -0x9c` | `ret(0xe99c4)` |
| `ldr` at 0x38948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38954 | load | 4 | `abs 0x38a48` | `-` |
| `ldr` at 0x3895c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x38968 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x38974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38980 | load | 4 | `abs 0x38a4c` | `-` |
| `ldr` at 0x3898c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x389a8 | load | 4 | `abs 0x38a44` | `-` |
| `str` at 0x389ac | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x389bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x389c0 | load | 4 | `abs 0x38a50` | `-` |
| `str` at 0x389e0 | store | 4 | `sp -0x9c` | `?` |
| `ldr` at 0x389ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38a08 | load | 4 | `abs 0x38a44` | `-` |
| `str` at 0x38a0c | store | 4 | `sp -0x9c` | `0xfffff060` |
| `ldr` at 0x38a28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x38a38 | store | 4 | `sp -0x9c` | `ret(0x38a54)` |
| `ldr` at 0x38a3c | load | 4 | `sp -0x9c` | `-` |
| `ldmdb` at 0x38a40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38a40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38a40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38890 | `(unknown callee) 0xd53d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x388a8 | `(unknown callee) 0xd6bb0` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x388b4 | `(unknown callee) 0xf6004` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x388dc | `(unknown callee) 0xf612c` | `[sp-20]` | `0x0` | `0x0` | `ret(0xf6004)` |
| `bl` at 0x388fc | `(unknown callee) 0xe99c4` | `[sp-20]` | `?` | `?` | `ret(0xf612c)` |
| `bl` at 0x38920 | `(unknown callee) 0xe3cf8` | `ret(0xe99c4)` | `?` | `?` | `ret(0xe99c4)` |
| `bl` at 0x38934 | `(unknown callee) 0xf61cc` | `0x0` | `sp-148` | `sp-144` | `sp-148` |
| `bl` at 0x38944 | `(unknown callee) 0xe3d64` | `ret(0xf61cc)` | `?` | `?` | `ret(0xf61cc)` |
| `bl` at 0x3894c | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38960 | `(unknown callee) 0x2079c` | `0x108180` | `ret(0xeaab4)` | `[sp-148]` | `ret(0xeaab4)` |
| `bl` at 0x38970 | `(unknown callee) 0xe3d64` | `ret(0xf61cc)` | `?` | `?` | `ret(0xf61cc)` |
| `bl` at 0x38978 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38988 | `(unknown callee) 0x2079c` | `0x1081a4` | `ret(0xeaab4)` | `?` | `ret(0xeaab4)` |
| `bl` at 0x38998 | `(unknown callee) 0xf612c` | `[sp-20]` | `0x1` | `0x1` | `?` |
| `bl` at 0x389b4 | `(unknown callee) 0xe3cf8` | `ret(0xf612c)` | `?` | `?` | `ret(0xf612c)` |
| `bl` at 0x389c8 | `(unknown callee) 0xf6550` | `[sp-20]` | `0x1081c8` | `0x100` | `sp-144` |
| `bl` at 0x389d8 | `(unknown callee) 0xe3d64` | `ret(0xf6550)` | `?` | `?` | `ret(0xf6550)` |
| `bl` at 0x389e8 | `(unknown callee) 0xe3d64` | `ret(0xf6550)` | `?` | `?` | `ret(0xf6550)` |
| `bl` at 0x389f8 | `(unknown callee) 0xf612c` | `[sp-20]` | `0x0` | `0x0` | `?` |
| `bl` at 0x38a24 | `(unknown callee) 0xdd33c` | `0x0` | `0x1` | `sp-144` | `sp-144` |
| `bl` at 0x38a30 | `(unknown callee) 0x38a54` | `[sp-20]` | `0x2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x388c0 | `ret(0xf6004) == 0x0` | `ret(0xf6004) != 0x0` |
| `beq` at 0x388e8 | `ret(0xf612c) == 0x0` | `ret(0xf612c) != 0x0` |
| `beq` at 0x38910 | `ret(0xe99c4) == 0x0` | `ret(0xe99c4) != 0x0` |
| `bne` at 0x38940 | `ret(0xf61cc) != 0x0` | `ret(0xf61cc) == 0x0` |
| `beq` at 0x389a4 | `ret(0xf612c) == 0x0` | `ret(0xf612c) != 0x0` |
| `beq` at 0x389d4 | `ret(0xf6550) == 0x0` | `ret(0xf6550) != 0x0` |
| `beq` at 0x38a04 | `ret(0xf612c) == 0x0` | `ret(0xf612c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38a40 | `r0` | `?` |

## `LBIMemInit`

- `LBIMemInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38a54 to 0x38d44.
  Established. Two initial four-byte zero stores have unknown effective targets (`str` at 0x38a80,
  `str` at 0x38a94). Included unnamed `0xeb5ec` calls carry `[sp-20]`, `0x16`, `sp-32`, `sp-32`, and
  `[sp-20]`, `0x10`, `sp-32`, `0x0` (`bl` at 0x38abc, `bl` at 0x38cb8). The latter zero result takes
  the branch to `0x38d04` (`beq` at 0x38cc4). Memory initialization commands and external effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38a64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38a68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x38a80 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x38a94 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x38aa4 | store | 4 | `sp -0x2c` | `0x39` |
| `ldr` at 0x38aa8 | load | 4 | `abs 0x38d20` | `-` |
| `str` at 0x38aac | store | 4 | `sp -0x28` | `0x1081e4` |
| `ldr` at 0x38ab0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38ad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38adc | load | 4 | `abs 0x38d24` | `-` |
| `str` at 0x38ae0 | store | 4 | `sp -0x2c` | `0x1081f4` |
| `str` at 0x38ae4 | store | 4 | `sp -0x28` | `ret(0xeaab4)` |
| `ldr` at 0x38aec | load | 4 | `abs 0x38d28` | `-` |
| `ldr` at 0x38afc | load | 4 | `abs 0x38d2c` | `-` |
| `str` at 0x38b00 | store | 4 | `sp -0x24` | `0xfffff05e` |
| `ldr` at 0x38b14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38b18 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x38b34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38b38 | load | 4 | `abs 0x38d30` | `-` |
| `ldr` at 0x38b50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38b58 | load | 4 | `abs 0x38d30` | `-` |
| `str` at 0x38b64 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38b80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38b84 | load | 4 | `abs 0x38d34` | `-` |
| `ldr` at 0x38b9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38ba4 | load | 4 | `abs 0x38d34` | `-` |
| `str` at 0x38bb0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38bcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38bd0 | load | 4 | `abs 0x38d38` | `-` |
| `ldr` at 0x38be8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38bf0 | load | 4 | `abs 0x38d38` | `-` |
| `str` at 0x38bfc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38c18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38c34 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38c48 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x38c64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x38c80 | load | 4 | `(unknown)` | `-` |
| `str` at 0x38c94 | store | 4 | `(unknown)` | `?` |
| `str` at 0x38ca0 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0x38ca4 | load | 4 | `abs 0x38d3c` | `-` |
| `str` at 0x38ca8 | store | 4 | `sp -0x28` | `0x108234` |
| `ldr` at 0x38cac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38cd8 | load | 4 | `abs 0x38d40` | `-` |
| `str` at 0x38cdc | store | 4 | `sp -0x2c` | `0x108244` |
| `str` at 0x38ce0 | store | 4 | `sp -0x28` | `ret(0xeaab4)` |
| `ldr` at 0x38ce8 | load | 4 | `abs 0x38d28` | `-` |
| `ldr` at 0x38cf8 | load | 4 | `abs 0x38d2c` | `-` |
| `str` at 0x38cfc | store | 4 | `sp -0x24` | `0xfffff05e` |
| `str` at 0x38d14 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x38d18 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x38d1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38d1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38d1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38a6c | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x38abc | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x16` | `sp-32` | `sp-32` |
| `bl` at 0x38acc | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x38ad4 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38af8 | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0xa3` | `0x0` |
| `bl` at 0x38cb8 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x10` | `sp-32` | `0x0` |
| `bl` at 0x38cc8 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x38cd0 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38cf4 | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0xac` | `0x0` |
| `bl` at 0x38d08 | `(unknown callee) 0x3a780` | `0x12c` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x38d0c | `(unknown callee) 0xe3d64` | `ret(0x3a780)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38ac8 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x38cc4 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38d1c | `r0` | `?` |

## `LBIMemRead`

- `LBIMemRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38d44 to 0x391c8.
  Established. The unnamed `0xeb5ec` call has `[sp-20]`, `0x11`, `sp-36`, `0x0` (`bl` at 0x38e78). A
  later call to that numeric target has `[sp-20]`, `0x14`, `0x0`, `[sp-52]` (`bl` at 0x38f10). The
  branch takes `[sp-32] > 0x3`, signed 32-bit, to `0x3918c` (`bgt` at 0x390b4); its fall-through
  includes a halfword store through `[sp-28] +0x0` with `([sp-36]:2 & 0xffff)` (`strh` at 0x39188).
  Read layout and memory addressing remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38d54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x38d58 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x38d5c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x38d60 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x38d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38d74 | load | 4 | `abs 0x3919c` | `-` |
| `str` at 0x38d78 | store | 4 | `sp -0x30` | `0xfffff05d` |
| `ldr` at 0x38d84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x38d8c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38d90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38d94 | load | 4 | `abs 0x391a0` | `-` |
| `ldr` at 0x38da0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38da8 | load | 4 | `abs 0x391a0` | `-` |
| `str` at 0x38db4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38db8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38dbc | load | 4 | `abs 0x391a4` | `-` |
| `ldr` at 0x38dc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38dd0 | load | 4 | `abs 0x391a4` | `-` |
| `str` at 0x38ddc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38de0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38de4 | load | 4 | `abs 0x391a8` | `-` |
| `ldr` at 0x38df0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38df8 | load | 4 | `abs 0x391a8` | `-` |
| `str` at 0x38e04 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38e18 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38e2c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x38e30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x38e40 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x38e54 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x38e60 | store | 4 | `sp -0x3c` | `0x18` |
| `ldr` at 0x38e64 | load | 4 | `abs 0x391ac` | `-` |
| `str` at 0x38e68 | store | 4 | `sp -0x38` | `0x108270` |
| `ldr` at 0x38e6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38e98 | load | 4 | `abs 0x391b0` | `-` |
| `str` at 0x38e9c | store | 4 | `sp -0x3c` | `0x108280` |
| `str` at 0x38ea0 | store | 4 | `sp -0x38` | `ret(0xeaab4)` |
| `ldr` at 0x38ea8 | load | 4 | `abs 0x391b4` | `-` |
| `ldr` at 0x38eb8 | load | 4 | `abs 0x391b8` | `-` |
| `str` at 0x38ebc | store | 4 | `sp -0x30` | `0xfffff05c` |
| `ldr` at 0x38ec4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38ed4 | store | 4 | `sp -0x34` | `sp-36` |
| `ldr` at 0x38edc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x38ee0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x38ee4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x38ee8 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0x38eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x38ef4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x38ef8 | load | 4 | `abs 0x391bc` | `-` |
| `str` at 0x38efc | store | 4 | `sp -0x38` | `0x1082ac` |
| `ldr` at 0x38f00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38f0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38f24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x38f30 | load | 4 | `abs 0x391c0` | `-` |
| `str` at 0x38f34 | store | 4 | `sp -0x3c` | `0x1082bc` |
| `str` at 0x38f38 | store | 4 | `sp -0x38` | `ret(0xeaab4)` |
| `ldr` at 0x38f40 | load | 4 | `abs 0x391b4` | `-` |
| `ldr` at 0x38f50 | load | 4 | `abs 0x391c4` | `-` |
| `str` at 0x38f54 | store | 4 | `sp -0x30` | `0xfffff05b` |
| `str` at 0x38f64 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x38f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x38f7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x38f8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38f90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38f94 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38f98 | load | 4 | `abs 0x391a0` | `-` |
| `ldr` at 0x38fa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38fa8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38fb0 | load | 4 | `abs 0x391a0` | `-` |
| `str` at 0x38fbc | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38fc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38fc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38fc8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38fcc | load | 4 | `abs 0x391a4` | `-` |
| `ldr` at 0x38fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38fdc | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x38fe4 | load | 4 | `abs 0x391a4` | `-` |
| `str` at 0x38ff0 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x38ff4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38ff8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x38ffc | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x39000 | load | 4 | `abs 0x391a8` | `-` |
| `ldr` at 0x3900c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39010 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x39018 | load | 4 | `abs 0x391a8` | `-` |
| `str` at 0x39024 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x39028 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3902c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39030 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x39040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39044 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x39058 | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x3905c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39060 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39064 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x39074 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39078 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x3908c | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0x39090 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39098 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3909c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x390a4 | store | 4 | `sp -0x28` | `[sp-40]+4` |
| `ldr` at 0x390ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x390b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x390bc | load | 4 | `abs 0x391a0` | `-` |
| `ldr` at 0x390c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x390d0 | load | 4 | `abs 0x391a0` | `-` |
| `str` at 0x390dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x390e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x390e4 | load | 4 | `abs 0x391a4` | `-` |
| `ldr` at 0x390f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x390f8 | load | 4 | `abs 0x391a4` | `-` |
| `str` at 0x39104 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3910c | load | 4 | `abs 0x391a8` | `-` |
| `ldr` at 0x39118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39120 | load | 4 | `abs 0x391a8` | `-` |
| `str` at 0x3912c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39130 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39140 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x39154 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39158 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39168 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3917c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39180 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x39184 | load | 2 | `sp -0x24` | `-` |
| `strh` at 0x39188 | store | 2 | `[sp-28] +0x0` | `([sp-36]:2 & 0xffff)` |
| `str` at 0x39190 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x39194 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x39198 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39198 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39198 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38d80 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x38e78 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x11` | `sp-36` | `0x0` |
| `bl` at 0x38e88 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x38e90 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38eb4 | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0xc9` | `0x0` |
| `bl` at 0x38f10 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x14` | `0x0` | `[sp-52]` |
| `bl` at 0x38f20 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x38f28 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x38f4c | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0xd4` | `0x0` |
| `bl` at 0x38f5c | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38d70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x38e84 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `bgt` at 0x38ecc | `[sp-32] > 0x3 (signed, 32-bit)` | `[sp-32] <= 0x3 (signed, 32-bit)` |
| `beq` at 0x38f1c | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `blt` at 0x38f84 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x390b4 | `[sp-32] > 0x3 (signed, 32-bit)` | `[sp-32] <= 0x3 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39198 | `r0` | `?` |

## `LBIMemWrite`

- `LBIMemWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x391c8 to 0x396f4.
  Established. The branch takes entry `r3 > 0x3`, signed 32-bit, to `0x3921c` (`bgt` at 0x3920c).
  Included halfword operations read an unknown target and store at `sp -0x2c` with unknown value
  (`ldrh` at 0x39430, `strh` at 0x39434). The unnamed `0xeb5ec` call carries `[sp-20]`, `0x13`,
  `[sp-36]`, `[sp-36]` (`bl` at 0x39668). Write data encoding and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x391d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x391dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x391e0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x391e4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x391e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x391f8 | load | 4 | `abs 0x396c8` | `-` |
| `str` at 0x391fc | store | 4 | `sp -0x34` | `0xfffff05d` |
| `ldr` at 0x39204 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x39214 | store | 4 | `sp -0x38` | `sp-44` |
| `ldr` at 0x3921c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39220 | store | 4 | `sp -0x38` | `r2` |
| `ldr` at 0x39224 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x39228 | store | 4 | `sp -0x24` | `[sp-56]` |
| `str` at 0x39230 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x39234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39248 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39258 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39260 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39268 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39270 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39278 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3927c | load | 4 | `abs 0x396cc` | `-` |
| `ldr` at 0x39288 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39290 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39298 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x392a0 | load | 4 | `abs 0x396cc` | `-` |
| `str` at 0x392ac | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x392b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x392b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x392c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x392c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x392d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x392d4 | load | 4 | `abs 0x396d0` | `-` |
| `ldr` at 0x392e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x392e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x392f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x392f8 | load | 4 | `abs 0x396d0` | `-` |
| `str` at 0x39304 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x39308 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39310 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39318 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39320 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3932c | load | 4 | `abs 0x396d4` | `-` |
| `ldr` at 0x39338 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39340 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39348 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39350 | load | 4 | `abs 0x396d4` | `-` |
| `str` at 0x3935c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x39360 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39368 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39370 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39378 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39380 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x39390 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39398 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x393a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x393b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x393b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x393c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x393c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x393d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x393d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x393e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x393f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x393f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3940c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x39410 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x39418 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x39420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3942c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x39430 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x39434 | store | 2 | `sp -0x2c` | `?` |
| `ldr` at 0x39438 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3943c | load | 4 | `abs 0x396cc` | `-` |
| `ldr` at 0x39448 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39450 | load | 4 | `abs 0x396cc` | `-` |
| `str` at 0x3945c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39460 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39464 | load | 4 | `abs 0x396d0` | `-` |
| `ldr` at 0x39470 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39478 | load | 4 | `abs 0x396d0` | `-` |
| `str` at 0x39484 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39488 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3948c | load | 4 | `abs 0x396d4` | `-` |
| `ldr` at 0x39498 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x394a0 | load | 4 | `abs 0x396d4` | `-` |
| `str` at 0x394ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x394b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x394c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x394d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x394d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x394e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x394fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x39504 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3950c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39510 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39514 | load | 4 | `abs 0x396cc` | `-` |
| `ldr` at 0x39520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39528 | load | 4 | `abs 0x396cc` | `-` |
| `str` at 0x39534 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39538 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3953c | load | 4 | `abs 0x396d0` | `-` |
| `ldr` at 0x39548 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39550 | load | 4 | `abs 0x396d0` | `-` |
| `str` at 0x3955c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39560 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39564 | load | 4 | `abs 0x396d4` | `-` |
| `ldr` at 0x39570 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39578 | load | 4 | `abs 0x396d4` | `-` |
| `str` at 0x39584 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39598 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x395ac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x395b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x395c0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x395d4 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x395e0 | store | 4 | `sp -0x40` | `0x18` |
| `ldr` at 0x395e4 | load | 4 | `abs 0x396d8` | `-` |
| `str` at 0x395e8 | store | 4 | `sp -0x3c` | `0x108270` |
| `ldr` at 0x395ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3960c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39618 | load | 4 | `abs 0x396dc` | `-` |
| `str` at 0x3961c | store | 4 | `sp -0x40` | `0x108280` |
| `str` at 0x39620 | store | 4 | `sp -0x3c` | `ret(0xeaab4)` |
| `ldr` at 0x39628 | load | 4 | `abs 0x396e0` | `-` |
| `ldr` at 0x39638 | load | 4 | `abs 0x396e4` | `-` |
| `str` at 0x3963c | store | 4 | `sp -0x34` | `0xfffff05c` |
| `ldr` at 0x39644 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3964c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x39650 | load | 4 | `abs 0x396e8` | `-` |
| `str` at 0x39654 | store | 4 | `sp -0x3c` | `0x1082ec` |
| `ldr` at 0x39658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39660 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39664 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3967c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39688 | load | 4 | `abs 0x396ec` | `-` |
| `str` at 0x3968c | store | 4 | `sp -0x40` | `0x1082fc` |
| `str` at 0x39690 | store | 4 | `sp -0x3c` | `ret(0xeaab4)` |
| `ldr` at 0x39698 | load | 4 | `abs 0x396e0` | `-` |
| `ldr` at 0x396a8 | load | 4 | `abs 0x396f0` | `-` |
| `str` at 0x396ac | store | 4 | `sp -0x34` | `0xfffff05a` |
| `str` at 0x396bc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x396c0 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x396c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x396c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x396c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39500 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x395f8 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x11` | `sp-40` | `0x0` |
| `bl` at 0x39608 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x39610 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x39634 | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0x10c` | `0x0` |
| `bl` at 0x39668 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x13` | `[sp-36]` | `[sp-36]` |
| `bl` at 0x39678 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x39680 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x396a4 | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0x118` | `0x0` |
| `bl` at 0x396b4 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x391f4 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x3920c | `r3 > 0x3 (signed, 32-bit)` | `r3 <= 0x3 (signed, 32-bit)` |
| `blt` at 0x39250 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0x39428 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x39604 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x39674 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x396c4 | `r0` | `?` |

## `LBIMemReadToFile`

- `LBIMemReadToFile`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x396f4 to 0x39898.
  Established. The unnamed `0x1b400` call has `r3`, `0x242`, `0x1a4`, `0x1000` (`bl` at 0x39730);
  its nonnegative signed result takes the branch to `0x39754` (`bge` at 0x39744). The unnamed
  `0x38d44` call has `[sp-20]`, `[sp-24]`, unknown, unknown (`bl` at 0x397ec). Included four-byte
  stack assignments retain `0x1000` at `sp -0x34` and `0xc` at `sp -0x3c` (`str` at 0x39720, `str`
  at 0x3977c). File transfer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39704 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39708 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3970c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x39710 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x39718 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x39720 | store | 4 | `sp -0x34` | `0x1000` |
| `ldr` at 0x39724 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39728 | load | 4 | `abs 0x39890` | `-` |
| `str` at 0x39738 | store | 4 | `sp -0x28` | `ret(0x1b400)` |
| `ldr` at 0x3973c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39748 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3974c | store | 4 | `sp -0x3c` | `ret(0x1b400)` |
| `ldr` at 0x39754 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39760 | store | 4 | `sp -0x38` | `ret(0x1b1b4)` |
| `ldr` at 0x39764 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39770 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3977c | store | 4 | `sp -0x3c` | `0xc` |
| `str` at 0x39788 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3978c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x397a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x397a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x397ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x397b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x397bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x397c4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x397cc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x397d0 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0x397d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x397d8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x397dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x397e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x397e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x397e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x397f4 | store | 4 | `sp -0x2c` | `ret(0x38d44)` |
| `ldr` at 0x397f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39808 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3980c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39810 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3981c | store | 4 | `sp -0x2c` | `ret(0x1b058)` |
| `ldr` at 0x39820 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39824 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39830 | load | 4 | `abs 0x39894` | `-` |
| `ldr` at 0x39834 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39838 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39848 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3984c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39850 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39858 | store | 4 | `sp -0x24` | `([sp-36] + [sp-52])` |
| `ldr` at 0x3985c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39860 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39868 | store | 4 | `sp -0x18` | `([sp-24] + [sp-52])` |
| `ldr` at 0x39870 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39878 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39880 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39884 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x39888 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x3988c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3988c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3988c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39730 | `(unknown callee) 0x1b400` | `r3` | `0x242` | `0x1a4` | `0x1000` |
| `bl` at 0x39758 | `(unknown callee) 0x1b1b4` | `[sp-52]` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0x39774 | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0x397ec | `(unknown callee) 0x38d44` | `[sp-20]` | `[sp-24]` | `?` | `?` |
| `bl` at 0x39814 | `(unknown callee) 0x1b058` | `[sp-40]` | `[sp-56]` | `[sp-48]` | `ret(0x38d44)` |
| `bl` at 0x3983c | `(unknown callee) 0x23fbc` | `0x10832c` | `[sp-48]` | `ret(0x1b058)` | `[sp-48]` |
| `bl` at 0x39874 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0x3987c | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x39744 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3976c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `blt` at 0x39798 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x397b4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x39800 | `ret(0x38d44) == 0x0` | `ret(0x38d44) != 0x0` |
| `beq` at 0x3982c | `ret(0x1b058) == [sp-48]` | `ret(0x1b058) != [sp-48]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3988c | `r0` | `?` |

## `LBIMemWriteFromFile`

- `LBIMemWriteFromFile`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39898 to 0x39a88.
  Established. A four-byte stack assignment at `sp -0x34` retains `0x1000` (`str` at 0x398c4). The
  unnamed `0xfc570` call has `[sp-40]`, `sp-144`, unknown, `sp-144` (`bl` at 0x3993c); its
  nonnegative signed result takes the branch to `0x39968` (`bge` at 0x39950). A later unnamed
  `0x391c8` call has `[sp-20]`, `[sp-24]`, `[sp-56]`, unknown (`bl` at 0x39a24). File-derived write
  layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x398a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x398ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x398b0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x398b4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x398bc | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x398c4 | store | 4 | `sp -0x34` | `0x1000` |
| `ldr` at 0x398c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x398d8 | store | 4 | `sp -0x28` | `ret(0x1b400)` |
| `ldr` at 0x398dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x398e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x398ec | store | 4 | `sp -0x94` | `ret(0x1b400)` |
| `ldr` at 0x398f4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39900 | store | 4 | `sp -0x38` | `ret(0x1b1b4)` |
| `ldr` at 0x39904 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39910 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3991c | store | 4 | `sp -0x94` | `0xc` |
| `ldr` at 0x39924 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39934 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39944 | store | 4 | `sp -0x2c` | `ret(0xfc570)` |
| `ldr` at 0x39948 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3995c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0x39960 | store | 4 | `sp -0x2c` | `[ret(0x1b148)]` |
| `ldr` at 0x39968 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x3996c | store | 4 | `sp -0x1c` | `[sp-100]` |
| `str` at 0x39974 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x39978 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3997c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3998c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39990 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x39998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x399a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x399a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x399b0 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x399b8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x399bc | store | 4 | `sp -0x98` | `[sp-52]` |
| `ldr` at 0x399c0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x399c4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x399c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x399cc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x399d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x399dc | store | 4 | `sp -0x2c` | `ret(0x1b58c)` |
| `ldr` at 0x399e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x399e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x399f0 | load | 4 | `abs 0x39a84` | `-` |
| `ldr` at 0x399f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x399f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39a08 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x39a0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x39a18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39a1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39a20 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x39a2c | store | 4 | `sp -0x2c` | `ret(0x391c8)` |
| `ldr` at 0x39a30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x39a40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39a44 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39a4c | store | 4 | `sp -0x24` | `([sp-36] + [sp-52])` |
| `ldr` at 0x39a50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39a54 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x39a5c | store | 4 | `sp -0x18` | `([sp-24] + [sp-52])` |
| `ldr` at 0x39a64 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x39a6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39a74 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x39a78 | store | 4 | `sp -0x94` | `[sp-44]` |
| `ldr` at 0x39a7c | load | 4 | `sp -0x94` | `-` |
| `ldmdb` at 0x39a80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39a80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39a80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x398d0 | `(unknown callee) 0x1b400` | `r3` | `0x0` | `r2` | `0x1000` |
| `bl` at 0x398f8 | `(unknown callee) 0x1b1b4` | `[sp-52]` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0x39914 | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0x3993c | `(unknown callee) 0xfc570` | `[sp-40]` | `sp-144` | `?` | `sp-144` |
| `bl` at 0x39954 | `(unknown callee) 0x1b148` | `ret(0xfc570)` | `?` | `?` | `ret(0xfc570)` |
| `bl` at 0x399d4 | `(unknown callee) 0x1b58c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x399fc | `(unknown callee) 0x23fbc` | `0x108348` | `[sp-48]` | `ret(0x1b58c)` | `[sp-48]` |
| `bl` at 0x39a24 | `(unknown callee) 0x391c8` | `[sp-20]` | `[sp-24]` | `[sp-56]` | `?` |
| `bl` at 0x39a68 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0x39a70 | `(unknown callee) 0x1b094` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x398e4 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0x3990c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0x3992c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bge` at 0x39950 | `ret(0xfc570) >= 0x0 (signed, 32-bit)` | `ret(0xfc570) < 0x0 (signed, 32-bit)` |
| `blt` at 0x39984 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x399a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x399ec | `ret(0x1b58c) == [sp-48]` | `ret(0x1b58c) != [sp-48]` |
| `beq` at 0x39a38 | `ret(0x391c8) == 0x0` | `ret(0x391c8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39a80 | `r0` | `?` |

## `LBILoadPROM`

- `LBILoadPROM`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39a88 to 0x39c80.
  Established. Included four-byte assignments write `0xff000000` at `sp -0x80` and `0x2000000` at
  `sp -0x88` (`str` at 0x39aac, `str` at 0x39b14). The unnamed `0x39898` call has `[sp-20]`,
  `[sp-132]`, `[sp-76]`, `[sp-24]` (`bl` at 0x39bf4); its zero result takes the branch to `0x39c40`
  (`beq` at 0x39c08). PROM addressing and load effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39a98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39a9c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39aa0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x39aa4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x39aac | store | 4 | `sp -0x80` | `0xff000000` |
| `ldr` at 0x39ab4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39acc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39ae0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0x39aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39af0 | load | 4 | `abs 0x39c70` | `-` |
| `ldr` at 0x39b04 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0x39b08 | store | 4 | `sp -0xfc` | `[ret(0x1b148)]` |
| `str` at 0x39b14 | store | 4 | `sp -0x88` | `0x2000000` |
| `ldr` at 0x39b18 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x39b1c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x39b24 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x39b28 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x39b2c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x39b38 | store | 4 | `sp -0x8c` | `([sp-132] + [sp-128])+0xffffffffff000000` |
| `ldr` at 0x39b3c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x39b40 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x39b4c | store | 4 | `sp -0x90` | `([sp-136] + [sp-128])+0xffffffffff000000` |
| `ldr` at 0x39b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39b5c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x39b80 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0x39b88 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x39b90 | store | 4 | `sp -0x104` | `([sp-136] + [sp-128])+0xfffffffffeffffff` |
| `ldr` at 0x39b94 | load | 4 | `sp -0xf8` | `-` |
| `str` at 0x39b98 | store | 4 | `sp -0x100` | `[sp-248]` |
| `ldr` at 0x39ba0 | load | 4 | `abs 0x39c74` | `-` |
| `ldr` at 0x39ba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39ba8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x39bb0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x39bdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39be8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x39bec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x39bf0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x39bfc | store | 4 | `sp -0x7c` | `ret(0x39898)` |
| `ldr` at 0x39c00 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x39c0c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x39c18 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x39c20 | store | 4 | `sp -0x104` | `[sp-144]-1` |
| `str` at 0x39c24 | store | 4 | `sp -0x100` | `ret(0x21704)` |
| `ldr` at 0x39c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39c2c | load | 4 | `abs 0x39c78` | `-` |
| `ldr` at 0x39c30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39c34 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x39c40 | load | 4 | `sp -0x90` | `-` |
| `str` at 0x39c48 | store | 4 | `sp -0x104` | `[sp-144]-1` |
| `ldr` at 0x39c4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39c50 | load | 4 | `abs 0x39c7c` | `-` |
| `ldr` at 0x39c54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39c58 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x39c60 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x39c64 | store | 4 | `sp -0xfc` | `[sp-124]` |
| `ldr` at 0x39c68 | load | 4 | `sp -0xfc` | `-` |
| `ldmdb` at 0x39c6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39c6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39c6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39abc | `(unknown callee) 0xfc554` | `r1` | `sp-120` | `r2` | `sp-120` |
| `bl` at 0x39ad8 | `(unknown callee) 0x1b148` | `ret(0xfc554)` | `?` | `?` | `[sp-28]` |
| `bl` at 0x39ae4 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0x39af8 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x108364` | `ret(0x21704)` | `ret(0x21704)` |
| `bl` at 0x39afc | `(unknown callee) 0x1b148` | `?` | `?` | `?` | `?` |
| `bl` at 0x39bac | `(unknown callee) 0x1b52c` | `sp-244` | `0x108384` | `[sp-24]` | `([sp-132] + [sp-128])+0xffffffffff000000` |
| `bl` at 0x39be0 | `(unknown callee) 0x8ded4` | `?` | `sp-244` | `[sp-32]` | `?` |
| `bl` at 0x39bf4 | `(unknown callee) 0x39898` | `[sp-20]` | `[sp-132]` | `[sp-76]` | `[sp-24]` |
| `bl` at 0x39c10 | `(unknown callee) 0x21704` | `ret(0x39898)` | `?` | `?` | `ret(0x39898)` |
| `bl` at 0x39c38 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x1083c8` | `[sp-24]` | `[sp-140]` |
| `bl` at 0x39c5c | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x1083f8` | `[sp-24]` | `[sp-140]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x39ac8 | `ret(0xfc554) >= 0x0 (signed, 32-bit)` | `ret(0xfc554) < 0x0 (signed, 32-bit)` |
| `beq` at 0x39ad4 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x39b58 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x39c08 | `ret(0x39898) == 0x0` | `ret(0x39898) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39c6c | `r0` | `?` |

## `LBIMemPattern`

- `LBIMemPattern`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39c80 to 0x39de0.
  Established. The unnamed `0x3a10c` call has `[sp-32]`, `sp-4128`, `[sp-8236]`, `0x0` (`bl` at
  0x39d24). A later unnamed `0x391c8` call uses `[sp-20]`, `([sp-24] + [sp-8228])`, `sp-4128`,
  `[sp-8236]` (`bl` at 0x39d60); its zero result takes the branch to `0x39d9c`, followed by a
  four-byte assignment at `sp -0x2024` retaining `[sp-8228]+4096` (`beq` at 0x39d80, `str` at
  0x39db4). Pattern contents and memory effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39c94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39c98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39c9c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x39ca0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x39ca4 | load | 4 | `abs 0x39dd4` | `-` |
| `str` at 0x39cb0 | store | 4 | `sp -0x2024` | `0x0` |
| `ldr` at 0x39cb4 | load | 4 | `abs 0x39dd4` | `-` |
| `ldr` at 0x39cbc | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x39cc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x39cd0 | load | 4 | `abs 0x39dd8` | `-` |
| `ldr` at 0x39cd4 | load | 4 | `abs 0x39dd4` | `-` |
| `ldr` at 0x39cdc | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x39ce0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x39cf4 | store | 4 | `sp -0x202c` | `?` |
| `ldr` at 0x39d04 | load | 4 | `abs 0x39dd8` | `-` |
| `str` at 0x39d0c | store | 4 | `sp -0x2034` | `0x0` |
| `ldr` at 0x39d10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x39d1c | load | 4 | `sp -0x202c` | `-` |
| `ldr` at 0x39d28 | load | 4 | `abs 0x39dd4` | `-` |
| `ldr` at 0x39d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x39d34 | load | 4 | `sp -0x2024` | `-` |
| `ldr` at 0x39d48 | load | 4 | `abs 0x39dd8` | `-` |
| `ldr` at 0x39d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39d5c | load | 4 | `sp -0x202c` | `-` |
| `ldr` at 0x39d68 | load | 4 | `abs 0x39ddc` | `-` |
| `str` at 0x39d70 | store | 4 | `sp -0x2028` | `ret(0x391c8)` |
| `ldr` at 0x39d78 | load | 4 | `sp -0x2028` | `-` |
| `ldr` at 0x39d84 | load | 4 | `abs 0x39ddc` | `-` |
| `ldr` at 0x39d8c | load | 4 | `sp -0x2028` | `-` |
| `str` at 0x39d94 | store | 4 | `sp -0x2030` | `ret(0x391c8)` |
| `ldr` at 0x39d9c | load | 4 | `abs 0x39dd4` | `-` |
| `ldr` at 0x39da0 | load | 4 | `abs 0x39dd4` | `-` |
| `ldr` at 0x39da8 | load | 4 | `sp -0x2024` | `-` |
| `str` at 0x39db4 | store | 4 | `sp -0x2024` | `[sp-8228]+4096` |
| `str` at 0x39dc4 | store | 4 | `sp -0x2030` | `0x0` |
| `ldr` at 0x39dcc | load | 4 | `sp -0x2030` | `-` |
| `ldmdb` at 0x39dd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x39dd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x39dd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39d24 | `(unknown callee) 0x3a10c` | `[sp-32]` | `sp-4128` | `[sp-8236]` | `0x0` |
| `bl` at 0x39d60 | `(unknown callee) 0x391c8` | `[sp-20]` | `([sp-24] + [sp-8228])` | `sp-4128` | `[sp-8236]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0x39cc8 | `C == 0` | `not (C == 0)` |
| `beq` at 0x39d80 | `ret(0x391c8) == 0x0` | `ret(0x391c8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x39dd0 | `r0` | `?` |

## `LBIReadLEDS`

- `LBIReadLEDS`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x39de0 to 0x3a10c.
  Established. The unnamed `0xeb5ec` call carries `[sp-20]`, `0x19`, `0x0`, `sp-32` (`bl` at
  0x39e20); its zero result takes the branch to `0x39e6c` (`beq` at 0x39e2c). Included byte stores
  through `[sp-24] +0x0` and `[sp-24] +0x1` have unknown values (`strb` at 0x3a0a8, `strb` at
  0x3a0d0). LED bit meanings and read result semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x39df0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x39df4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x39e04 | store | 4 | `sp -0x38` | `0x28` |
| `ldr` at 0x39e08 | load | 4 | `abs 0x3a0ec` | `-` |
| `str` at 0x39e0c | store | 4 | `sp -0x34` | `0x10841c` |
| `ldr` at 0x39e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39e34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x39e40 | load | 4 | `abs 0x3a0f0` | `-` |
| `str` at 0x39e44 | store | 4 | `sp -0x38` | `0x10842c` |
| `str` at 0x39e48 | store | 4 | `sp -0x34` | `ret(0xeaab4)` |
| `ldr` at 0x39e50 | load | 4 | `abs 0x3a0f4` | `-` |
| `ldr` at 0x39e54 | load | 4 | `abs 0x3a0f8` | `-` |
| `ldr` at 0x39e60 | load | 4 | `abs 0x3a0fc` | `-` |
| `str` at 0x39e64 | store | 4 | `sp -0x30` | `0xfffff059` |
| `str` at 0x39e7c | store | 4 | `sp -0x38` | `0x20` |
| `ldr` at 0x39e94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39e98 | load | 4 | `abs 0x3a100` | `-` |
| `ldr` at 0x39ea4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39eac | load | 4 | `abs 0x3a100` | `-` |
| `str` at 0x39eb8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39ebc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39ec0 | load | 4 | `abs 0x3a104` | `-` |
| `ldr` at 0x39ecc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39ed4 | load | 4 | `abs 0x3a104` | `-` |
| `str` at 0x39ee0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39ee4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39ee8 | load | 4 | `abs 0x3a108` | `-` |
| `ldr` at 0x39ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39efc | load | 4 | `abs 0x3a108` | `-` |
| `str` at 0x39f08 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39f0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39f1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39f30 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x39f34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x39f44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x39f58 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x39f68 | store | 4 | `sp -0x38` | `0x8` |
| `ldr` at 0x39f80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39f84 | load | 4 | `abs 0x3a100` | `-` |
| `ldr` at 0x39f90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39f98 | load | 4 | `abs 0x3a100` | `-` |
| `str` at 0x39fa4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39fa8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39fac | load | 4 | `abs 0x3a104` | `-` |
| `ldr` at 0x39fb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39fc0 | load | 4 | `abs 0x3a104` | `-` |
| `str` at 0x39fcc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39fd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39fd4 | load | 4 | `abs 0x3a108` | `-` |
| `ldr` at 0x39fe0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x39fe8 | load | 4 | `abs 0x3a108` | `-` |
| `str` at 0x39ff4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x39ff8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a008 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3a01c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3a020 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a030 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3a044 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x3a04c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3a050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a064 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a06c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a074 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x3a07c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x3a080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a084 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a088 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a094 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3a0a0 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x3a0a8 | store | 1 | `[sp-24] +0x0` | `?` |
| `ldr` at 0x3a0ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a0b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a0b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a0bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3a0c8 | load | 1 | `[sp-24] +0x1` | `-` |
| `strb` at 0x3a0d0 | store | 1 | `[sp-24] +0x1` | `?` |
| `ldr` at 0x3a0d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3a0dc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x3a0e4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3a0e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a0e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a0e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x39df8 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x39e20 | `(unknown callee) 0xeb5ec` | `[sp-20]` | `0x19` | `0x0` | `sp-32` |
| `bl` at 0x39e30 | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x39e38 | `(unknown callee) 0xeaab4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x39e5c | `(unknown callee) 0x21eb8` | `0x3` | `0x108220` | `0x1dd` | `0x0` |
| `bl` at 0x39e6c | `(unknown callee) 0xe3d64` | `ret(0xeb5ec)` | `?` | `?` | `ret(0xeb5ec)` |
| `bl` at 0x39e90 | `(unknown callee) 0xe2b10` | `0x0` | `sp-32` | `0x0` | `sp-40` |
| `bl` at 0x39f7c | `(unknown callee) 0xe2b10` | `0x20` | `sp-32` | `0x0` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x39e2c | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `ble` at 0x3a058 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a0e8 | `r0` | `?` |

## `Daytona_JTAG_Sel`

- `Daytona_JTAG_Sel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3628c to 0x36380.
  Established. The two unnamed `0xd73b0` calls carry `0x0`, `0x8`, `0x80`, `[sp-17]:1`, and `0x0`,
  `0xc`, `0x80`, `[sp-18]:1` (`bl` at 0x36310, `bl` at 0x36334). A zero result from the second call
  takes the branch to `0x36358` (`beq` at 0x36340). The exit carries `[sp-19]:1` (`ldmdb` at
  0x3637c). Port selection encoding and physical effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x362a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x362ac | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x362b0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x362c0 | store | 1 | `sp -0x15` | `?` |
| `strb` at 0x362cc | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x362d0 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x362d4 | store | 1 | `sp -0x11` | `([sp-21]:1 & 0xff)` |
| `ldrb` at 0x362d8 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x362e8 | store | 1 | `sp -0x16` | `?` |
| `strb` at 0x362f4 | store | 1 | `sp -0x16` | `0x0` |
| `ldrb` at 0x362f8 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x362fc | store | 1 | `sp -0x12` | `([sp-22]:1 & 0xff)` |
| `ldrb` at 0x36300 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x36318 | store | 1 | `sp -0x13` | `(ret(0xd73b0) & 0xff)` |
| `strb` at 0x36320 | store | 1 | `sp -0x17` | `0x0` |
| `ldrb` at 0x36324 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x36344 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x36354 | store | 1 | `sp -0x17` | `0x1` |
| `ldrb` at 0x36358 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x3635c | store | 1 | `sp -0x14` | `([sp-23]:1 & 0xff)` |
| `ldrb` at 0x3636c | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x36370 | store | 1 | `sp -0x13` | `(([sp-23]:1 & 0xff) & 0xff)` |
| `ldrb` at 0x36374 | load | 1 | `sp -0x13` | `-` |
| `ldmdb` at 0x3637c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3637c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3637c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36310 | `(unknown callee) 0xd73b0` | `0x0` | `0x8` | `0x80` | `[sp-17]:1` |
| `bl` at 0x36334 | `(unknown callee) 0xd73b0` | `0x0` | `0xc` | `0x80` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x362b8 | `(r0 & 0xff) == 0x0` | `(r0 & 0xff) != 0x0` |
| `beq` at 0x362e0 | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0x36340 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bne` at 0x3634c | `[sp-19]:1 != 0x0` | `[sp-19]:1 == 0x0` |
| `beq` at 0x36368 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3637c | `r0` | `[sp-19]:1` |

## `Daytona_JTAG_Sel_Query`

- `Daytona_JTAG_Sel_Query`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x361c0 to 0x3628c.
  Established. Included unnamed `0xd75c0` calls have `0x0`, `0x8`, `r2`, `r3`, and `0x0`, `0xc`,
  unknown, `ret(0xd75c0)` (`bl` at 0x361dc, `bl` at 0x361f0). Byte assignments at `sp -0x15` and
  `sp -0x16` each retain `(ret(0xd75c0) & 0xff)` (`strb` at 0x361e4, `strb` at 0x361f8). The later
  four-byte stack assignment at `sp -0x14` has `([sp-20] + ret(0x1b2bc))` (`str` at 0x36274). Query
  status meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x361d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x361e4 | store | 1 | `sp -0x15` | `(ret(0xd75c0) & 0xff)` |
| `strb` at 0x361f8 | store | 1 | `sp -0x16` | `(ret(0xd75c0) & 0xff)` |
| `ldrsb` at 0x361fc | load | 1 | `sp -0x15` | `-` |
| `ldrsb` at 0x36208 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x36214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36218 | load | 4 | `abs 0x3627c` | `-` |
| `ldr` at 0x36224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36228 | load | 4 | `abs 0x36280` | `-` |
| `ldrsb` at 0x36234 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x36240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36244 | load | 4 | `abs 0x36284` | `-` |
| `ldr` at 0x36250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x36254 | load | 4 | `abs 0x36288` | `-` |
| `ldr` at 0x3625c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3626c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x36274 | store | 4 | `sp -0x14` | `([sp-20] + ret(0x1b2bc))` |
| `ldmdb` at 0x36278 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36278 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36278 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x361dc | `(unknown callee) 0xd75c0` | `0x0` | `0x8` | `r2` | `r3` |
| `bl` at 0x361f0 | `(unknown callee) 0xd75c0` | `0x0` | `0xc` | `?` | `ret(0xd75c0)` |
| `bl` at 0x3621c | `(unknown callee) 0x1b52c` | `[sp-20]` | `0x107508` | `?` | `?` |
| `bl` at 0x3622c | `(unknown callee) 0x1b52c` | `[sp-20]` | `0x107520` | `?` | `?` |
| `bl` at 0x36248 | `(unknown callee) 0x1b52c` | `[sp-20]` | `0x107534` | `?` | `?` |
| `bl` at 0x36258 | `(unknown callee) 0x1b52c` | `[sp-20]` | `0x10754c` | `?` | `?` |
| `bl` at 0x36260 | `(unknown callee) 0x1b2bc` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x36204 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x36210 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `bge` at 0x3623c | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36278 | `r0` | `ret(0x1b2bc)` |

## `SysComJTagReadReady`

- `SysComJTagReadReady`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84094 to 0x84198.
  Established. A four-byte read targets absolute `0x2b24b0` (`ldr` at 0x840b0). The signed
  comparison takes entry `r0 >= [0x2b24b0]` to the exit at `0x84184` (`bge` at 0x840b8). An included
  unnamed `0x22e50` call has unknown, `[sp-20]`, unknown, unknown (`bl` at 0x84140). A later
  four-byte stack store at `sp -0x1c` retains its result (`str` at 0x8416c). Readiness and
  notification meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x840a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x840a8 | load | 4 | `abs 0x84188` | `-` |
| `ldr` at 0x840ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x840b0 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0x840bc | load | 4 | `abs 0x84188` | `-` |
| `ldr` at 0x840c0 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x840cc | load | 4 | `abs 0x84188` | `-` |
| `ldr` at 0x840d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x840f4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x840fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84108 | load | 4 | `abs 0x84188` | `-` |
| `ldr` at 0x8410c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84130 | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x84148 | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0x8414c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x84158 | load | 4 | `abs 0x8418c` | `-` |
| `str` at 0x8415c | store | 4 | `sp -0x24` | `0x1127e8` |
| `ldr` at 0x84160 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x84164 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x84168 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8416c | store | 4 | `sp -0x1c` | `ret(0x22e50)` |
| `ldr` at 0x84174 | load | 4 | `abs 0x84190` | `-` |
| `ldr` at 0x84178 | load | 4 | `abs 0x84194` | `-` |
| `ldmdb` at 0x84184 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x84184 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x84184 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84140 | `(unknown callee) 0x22e50` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x84180 | `(unknown callee) 0x21eb8` | `0x9` | `0x112380` | `0x283` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x840b8 | `r0 >= [0x2b24b0] (signed, 32-bit)` | `r0 < [0x2b24b0] (signed, 32-bit)` |
| `beq` at 0x840c8 | `[0x2b24b4] == 0x0` | `[0x2b24b4] != 0x0` |
| `beq` at 0x84104 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x84154 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x84184 | `r0` | `?` |

## `JChip_tdr_load`

- `JChip_tdr_load`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed170 to 0xedaec.
  Established. A halfword read targets `[[sp-60]] +0xe` (`ldrh` at 0xed248). Included four-byte
  global assignments at `0x2ae004` write `0x0` and `[0x2ae004]+1` (`str` at 0xed404, `str` at
  0xed44c). The unnamed `0xdd8e8` call carries `0x1`, `0x1`, `0x6`, `[sp-208]` (`bl` at 0xed4bc). A
  later unsigned comparison takes `[sp-48] >= [sp-44]` to `0xeda60` (`bhs` at 0xeda0c). Scan-record
  loading and result-field meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed180 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xed184 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xed188 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xed18c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xed194 | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0xed198 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed19c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed1a8 | load | 4 | `abs 0xeda74` | `-` |
| `ldr` at 0xed1ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed1b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed1b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed1bc | load | 4 | `abs 0xeda78` | `-` |
| `str` at 0xed1c0 | store | 4 | `sp -0x48` | `0x2ae008` |
| `ldr` at 0xed1c4 | load | 4 | `abs 0xeda7c` | `-` |
| `str` at 0xed1c8 | store | 4 | `sp -0x4c` | `0x2ae088` |
| `ldr` at 0xed1cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed1d8 | store | 4 | `sp -0x3c` | `ret(0xebc18)` |
| `ldr` at 0xed1dc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed1e8 | load | 4 | `abs 0xeda80` | `-` |
| `ldr` at 0xed1ec | load | 4 | `abs 0xeda84` | `-` |
| `ldr` at 0xed1f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed1f8 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed1fc | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed204 | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed208 | load | 4 | `abs 0xeda8c` | `-` |
| `ldr` at 0xed214 | load | 4 | `abs 0xeda90` | `-` |
| `str` at 0xed218 | store | 4 | `sp -0xd4` | `0xfffff7c6` |
| `ldr` at 0xed220 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed234 | load | 4 | `abs 0xeda94` | `-` |
| `str` at 0xed238 | store | 4 | `sp -0xd4` | `0xfffff7c5` |
| `ldr` at 0xed240 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed244 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldrh` at 0xed248 | load | 2 | `[[sp-60]] +0xe` | `-` |
| `str` at 0xed25c | store | 4 | `sp -0xd0` | `0x1` |
| `str` at 0xed264 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xed26c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xed274 | load | 4 | `sp -0x3c` | `-` |
| `ldrsb` at 0xed278 | load | 1 | `[sp-60] +0x21` | `-` |
| `str` at 0xed27c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xed280 | load | 4 | `abs 0xeda98` | `-` |
| `ldr` at 0xed284 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xed288 | load | 1 | `[sp-60] +0x19` | `-` |
| `ldr` at 0xed2a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed2ac | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xed2b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed2b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed2bc | load | 4 | `[sp-60] +0x28` | `-` |
| `ldr` at 0xed2c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed2c8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xed2cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed2e0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed2e4 | load | 4 | `abs 0xeda80` | `-` |
| `ldr` at 0xed2f8 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed2fc | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed304 | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed308 | load | 4 | `abs 0xeda9c` | `-` |
| `ldr` at 0xed314 | load | 4 | `abs 0xedaa0` | `-` |
| `str` at 0xed318 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xed320 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed324 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed330 | load | 4 | `abs 0xedaa4` | `-` |
| `ldr` at 0xed334 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed33c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed34c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed354 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xed360 | store | 4 | `sp -0x44` | `ret(0xecd30)` |
| `ldr` at 0xed364 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xed370 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed374 | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed37c | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed380 | load | 4 | `abs 0xedaa8` | `-` |
| `ldr` at 0xed38c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xed390 | store | 4 | `sp -0xd4` | `[sp-68]` |
| `ldr` at 0xed398 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed39c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed3ac | load | 4 | `abs 0xedaac` | `-` |
| `ldr` at 0xed3b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed3bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed3c0 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xed3c4 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `ldr` at 0xed3c8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed3d4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xed3d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed3ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed3f8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xed3fc | load | 4 | `abs 0xedab0` | `-` |
| `str` at 0xed404 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xed408 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed40c | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed410 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed420 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed424 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed42c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xed438 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xed43c | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed440 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed444 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed44c | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed454 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed458 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed45c | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xed460 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xed464 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xed46c | load | 4 | `[sp-64] +0x10` | `-` |
| `ldr` at 0xed470 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed474 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed47c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed48c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed490 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xed494 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xed498 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xed49c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xed4a0 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xed4a4 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed4a8 | store | 4 | `sp -0xdc` | `0x2b47f8` |
| `ldr` at 0xed4b8 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed4cc | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed4d0 | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed4d8 | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed4dc | load | 4 | `abs 0xedab4` | `-` |
| `ldr` at 0xed4e8 | load | 4 | `abs 0xedaa0` | `-` |
| `str` at 0xed4ec | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xed4f4 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed4f8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed504 | load | 4 | `abs 0xedab8` | `-` |
| `ldr` at 0xed508 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed50c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed518 | load | 4 | `abs 0xedabc` | `-` |
| `ldr` at 0xed51c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed520 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed52c | load | 4 | `abs 0xedac0` | `-` |
| `ldr` at 0xed530 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed534 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed53c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed540 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xed544 | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xed548 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xed54c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed550 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed554 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed558 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed560 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed564 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed568 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed580 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed584 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed588 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldrh` at 0xed58c | load | 2 | `[[sp-64]+8] +0x0` | `-` |
| `str` at 0xed590 | store | 4 | `sp -0xe8` | `[[[sp-64]+8]]:2` |
| `ldr` at 0xed594 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed598 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed5a0 | load | 4 | `[sp-64] +0x10` | `-` |
| `str` at 0xed5ac | store | 4 | `sp -0x28` | `0x6` |
| `str` at 0xed5b8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xed5bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed5c0 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xed5c4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xed5c8 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xed5cc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xed5d0 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xed5d4 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed5d8 | store | 4 | `sp -0xdc` | `0x2b47f8` |
| `ldr` at 0xed5e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xed5e8 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed5fc | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed600 | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed608 | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed618 | load | 4 | `abs 0xedaa0` | `-` |
| `str` at 0xed61c | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xed624 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed628 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed634 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xed640 | load | 4 | `abs 0xedac4` | `-` |
| `str` at 0xed644 | store | 4 | `sp -0xd8` | `0x137778` |
| `ldr` at 0xed64c | load | 4 | `abs 0xedac8` | `-` |
| `str` at 0xed650 | store | 4 | `sp -0xd8` | `0x137788` |
| `ldr` at 0xed654 | load | 4 | `abs 0xedacc` | `-` |
| `ldr` at 0xed658 | load | 4 | `sp -0xd8` | `-` |
| `ldr` at 0xed65c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed660 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed66c | load | 4 | `abs 0xedad0` | `-` |
| `ldr` at 0xed670 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed674 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed680 | load | 4 | `abs 0xedad4` | `-` |
| `ldr` at 0xed684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed688 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed690 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed6a4 | load | 4 | `abs 0xedad8` | `-` |
| `ldr` at 0xed6a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed6b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed6c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed6c8 | store | 4 | `sp -0xe8` | `[sp-36]` |
| `ldr` at 0xed6cc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xed6d0 | store | 4 | `sp -0xe4` | `[sp-72]` |
| `ldr` at 0xed6d4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xed6d8 | store | 4 | `sp -0xe0` | `[sp-76]` |
| `ldr` at 0xed6dc | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed6e0 | store | 4 | `sp -0xdc` | `0x2b47f8` |
| `ldr` at 0xed6f0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed704 | load | 4 | `abs 0xeda80` | `-` |
| `str` at 0xed708 | store | 4 | `sp -0xe8` | `0x2b47f8` |
| `ldr` at 0xed710 | load | 4 | `abs 0xeda88` | `-` |
| `ldr` at 0xed714 | load | 4 | `abs 0xedadc` | `-` |
| `ldr` at 0xed720 | load | 4 | `abs 0xedaa0` | `-` |
| `str` at 0xed724 | store | 4 | `sp -0xd4` | `0xfffff7ca` |
| `ldr` at 0xed72c | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed730 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed73c | load | 4 | `abs 0xedae0` | `-` |
| `ldr` at 0xed740 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0xed744 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed750 | load | 4 | `abs 0xedad0` | `-` |
| `ldr` at 0xed754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed758 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed764 | load | 4 | `abs 0xedad4` | `-` |
| `ldr` at 0xed768 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed76c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed780 | load | 4 | `abs 0xedab0` | `-` |
| `str` at 0xed788 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xed78c | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed790 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed794 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed7a4 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed7a8 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed7c4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xed7c8 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed7cc | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed7d0 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed7d8 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed7e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed7ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xed7f4 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed7f8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed80c | load | 4 | `abs 0xedae4` | `-` |
| `ldr` at 0xed810 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed818 | load | 4 | `abs 0xedab0` | `-` |
| `str` at 0xed820 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xed824 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed828 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed82c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed83c | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed840 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed848 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed850 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed854 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed85c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed864 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed868 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed884 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed88c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xed890 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed894 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed898 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed8a0 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed8a8 | load | 4 | `abs 0xedab0` | `-` |
| `str` at 0xed8b0 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xed8b4 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed8b8 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed8bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed8cc | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed8d0 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed8d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed8e0 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed8e4 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed8ec | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed8f4 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed8f8 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed914 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed91c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xed920 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed924 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed928 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed930 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed938 | load | 4 | `abs 0xedab0` | `-` |
| `str` at 0xed940 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xed944 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed948 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed94c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xed95c | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed960 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed978 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed97c | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed984 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xed98c | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed990 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xed998 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xed9a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed9a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xed9ac | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xed9b0 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed9b4 | load | 4 | `abs 0xedab0` | `-` |
| `ldr` at 0xed9b8 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xed9c0 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed9c8 | load | 4 | `abs 0xeda70` | `-` |
| `ldr` at 0xed9cc | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed9e0 | load | 4 | `abs 0xedae8` | `-` |
| `ldr` at 0xed9e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xed9f0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xed9f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xed9fc | store | 4 | `[sp-32] +0x0` | `0x0` |
| `ldr` at 0xeda00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeda04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeda10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeda14 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xeda20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeda38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeda44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeda4c | store | 4 | `[sp-32] +0x0` | `0x1` |
| `ldr` at 0xeda50 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xeda58 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xeda64 | store | 4 | `sp -0xd4` | `0x0` |
| `ldr` at 0xeda68 | load | 4 | `sp -0xd4` | `-` |
| `ldmdb` at 0xeda6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeda6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeda6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed1b8 | `(unknown callee) 0x23fbc` | `0x13769c` | `r0` | `r1` | `r2` |
| `bl` at 0xed1d0 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `0x2ae088` |
| `bl` at 0xed1f4 | `(unknown callee) 0x1b52c` | `0x2b47f8` | `0x1375c0` | `[sp-20]` | `ret(0xebc18)` |
| `bl` at 0xed210 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x5c9` | `0x0` |
| `bl` at 0xed224 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `bl` at 0xed2e8 | `(unknown callee) 0xdd3f8` | `0x1` | `?` | `0x2b47f8` | `?` |
| `bl` at 0xed310 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x5e5` | `0x0` |
| `bl` at 0xed338 | `(unknown callee) 0x23fbc` | `0x1375f0` | `[sp-208]` | `?` | `[0x2adfd8]` |
| `bl` at 0xed358 | `(unknown callee) 0xecd30` | `[sp-20]` | `0x0` | `[sp-28]` | `?` |
| `bl` at 0xed388 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x5f1` | `0x0` |
| `bl` at 0xed3b8 | `(unknown callee) 0xf1394` | `0x0` | `0x1376d8` | `[sp-64]` | `0x4` |
| `bl` at 0xed478 | `(unknown callee) 0xe2b10` | `0x0` | `[[sp-64]+16]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xed4bc | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x6` | `[sp-208]` |
| `bl` at 0xed4e4 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x604` | `0x0` |
| `bl` at 0xed510 | `(unknown callee) 0x23fbc` | `0x1376f4` | `[sp-208]` | `[sp-36]` | `[0x2adfd8]` |
| `bl` at 0xed524 | `(unknown callee) 0xf81f4` | `0x0` | `0x137750` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xed538 | `(unknown callee) 0xf81f4` | `0x0` | `0x137764` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xed55c | `(unknown callee) 0xe2b10` | `[sp-52]` | `[sp-76]` | `[sp-52]` | `[sp-72]` |
| `bl` at 0xed56c | `(unknown callee) 0xf20b0` | `[sp-64]` | `[sp-52]` | `[sp-72]` | `?` |
| `bl` at 0xed5a4 | `(unknown callee) 0xe2b10` | `[sp-52]` | `[sp-72]` | `0x0` | `[[sp-64]+16]` |
| `bl` at 0xed5ec | `(unknown callee) 0xdd8e8` | `0x1` | `?` | `0x1` | `[sp-208]` |
| `bl` at 0xed614 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x620` | `0x0` |
| `bl` at 0xed664 | `(unknown callee) 0x23fbc` | `0x137794` | `[sp-216]` | `[sp-208]` | `[sp-36]` |
| `bl` at 0xed678 | `(unknown callee) 0xf81f4` | `0x0` | `0x1377e8` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xed68c | `(unknown callee) 0xf81f4` | `0x0` | `0x1377f4` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xed6b0 | `(unknown callee) 0xf1394` | `0x0` | `0x137800` | `[sp-64]` | `0x4` |
| `bl` at 0xed6f4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-208]` |
| `bl` at 0xed71c | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x633` | `0x0` |
| `bl` at 0xed748 | `(unknown callee) 0x23fbc` | `0x13781c` | `[sp-208]` | `[sp-36]` | `[0x2adfd8]` |
| `bl` at 0xed75c | `(unknown callee) 0xf81f4` | `0x0` | `0x1377e8` | `[sp-44]` | `[sp-72]` |
| `bl` at 0xed770 | `(unknown callee) 0xf81f4` | `0x0` | `0x1377f4` | `[sp-44]` | `[sp-76]` |
| `bl` at 0xed7f0 | `(unknown callee) 0xf21bc` | `[sp-64]` | `0x4` | `[sp-52]` | `sp-204` |
| `bl` at 0xed814 | `(unknown callee) 0xf81f4` | `0x0` | `0x137878` | `[sp-44]` | `sp-204` |
| `bl` at 0xed9e8 | `(unknown callee) 0xf81f4` | `0x0` | `0x137888` | `[sp-44]` | `sp-204` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xed1a4 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xed1e4 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xed230 | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0xed254 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xed2d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed2f4 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bls` at 0xed32c | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xed348 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xed36c | `ret(0xecd30) >= 0x0 (signed, 32-bit)` | `ret(0xecd30) < 0x0 (signed, 32-bit)` |
| `bls` at 0xed3a4 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `blo` at 0xed418 | `[0x2ae004] < [sp-44] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-44] (unsigned, 32-bit)` |
| `beq` at 0xed488 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed4c8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xed500 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xed57c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed5f8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xed630 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xed63c | `[sp-40] != 0x6` | `[sp-40] == 0x6` |
| `beq` at 0xed69c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xed6c0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xed700 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xed738 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xed77c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `blo` at 0xed79c | `[0x2ae004] < [sp-44] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xed800 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xed834 | `[0x2ae004] < [sp-44] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xed8c4 | `[0x2ae004] < [sp-44] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xed954 | `[0x2ae004] < [sp-44] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-44] (unsigned, 32-bit)` |
| `bls` at 0xed9d4 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `bhs` at 0xeda0c | `[sp-48] >= [sp-44] (unsigned, 32-bit)` | `[sp-48] < [sp-44] (unsigned, 32-bit)` |
| `bne` at 0xeda1c | `[[sp-32]] != 0x0` | `[[sp-32]] == 0x0` |
| `beq` at 0xeda40 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeda6c | `r0` | `?` |

## `JChip_find_tdr`

- `JChip_find_tdr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xecb70 to 0xecc58.
  Established. The nonzero result from unnamed `0xebc18` takes the branch to `0xecbd0` (`bl` at
  0xecbac, `bne` at 0xecbc0). Included four-byte stack assignments at `sp -0x24` retain `0xfffff7c6`
  and `[sp-32]` at separate sites (`str` at 0xecbc8, `str` at 0xecc24). The unnamed `0x1b088` call
  has `[sp-24]` followed by three unknown candidates (`bl` at 0xecc10). TDR lookup criteria and
  return interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xecb80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xecb84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xecb88 | load | 4 | `abs 0xecc4c` | `-` |
| `ldr` at 0xecb8c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xecb98 | load | 4 | `abs 0xecc50` | `-` |
| `ldr` at 0xecb9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xecba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecba8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xecbb4 | store | 4 | `sp -0x1c` | `ret(0xebc18)` |
| `ldr` at 0xecbb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecbc4 | load | 4 | `abs 0xecc54` | `-` |
| `str` at 0xecbc8 | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `str` at 0xecbd4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xecbd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecbdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xecbe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecbf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecbf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xecbfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecc04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecc08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecc0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecc20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xecc24 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xecc2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xecc34 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xecc3c | load | 4 | `abs 0xecc54` | `-` |
| `str` at 0xecc40 | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `ldr` at 0xecc44 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xecc48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xecc48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xecc48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xecba4 | `(unknown callee) 0x23fbc` | `0x137528` | `r0` | `r1` | `[0x2adfd8]` |
| `bl` at 0xecbac | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xecc10 | `(unknown callee) 0x1b088` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xecb94 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xecbc0 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `blo` at 0xecbe8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xecc1c | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xecc48 | `r0` | `?` |

## `JChip_find_tdr_field`

- `JChip_find_tdr_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xecc58 to 0xecd30.
  Established. The four-byte load at `ret(0xebc18) +0x28` precedes two loads with unknown targets
  (`ldr` at 0xecccc, `ldr` at 0xeccd4, `ldr` at 0xeccd8). The unnamed `0xf10c8` call has `[sp-40]`,
  `[sp-28]`, unknown, unknown (`bl` at 0xecce8). The unsigned comparison takes
  `ret(0xf10c8) < [[sp-40]+2]:2` to `0xecd14` (`blo` at 0xecd04). Field lookup semantics and return meaning remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xecc68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xecc6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xecc70 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xecc74 | load | 4 | `abs 0xecd24` | `-` |
| `ldr` at 0xecc78 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xecc84 | load | 4 | `abs 0xecd28` | `-` |
| `ldr` at 0xecc88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xecc8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecc90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecc98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xecca4 | store | 4 | `sp -0x20` | `ret(0xebc18)` |
| `ldr` at 0xecca8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeccb4 | load | 4 | `abs 0xecd2c` | `-` |
| `str` at 0xeccb8 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xeccc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeccc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecccc | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xeccd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeccd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeccdc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xecce0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xecce4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeccf0 | store | 4 | `sp -0x24` | `ret(0xf10c8)` |
| `ldr` at 0xeccf4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xeccf8 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xeccfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xecd08 | load | 4 | `abs 0xecd2c` | `-` |
| `str` at 0xecd0c | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xecd14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xecd18 | store | 4 | `sp -0x2c` | `ret(0xf10c8)` |
| `ldr` at 0xecd1c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xecd20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xecd20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xecd20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xecc94 | `(unknown callee) 0x23fbc` | `0x137554` | `r0` | `r1` | `r2` |
| `bl` at 0xecc9c | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xecce8 | `(unknown callee) 0xf10c8` | `[sp-40]` | `[sp-28]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xecc80 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeccb0 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `blo` at 0xecd04 | `ret(0xf10c8) < [[sp-40]+2]:2 (unsigned, 32-bit)` | `ret(0xf10c8) >= [[sp-40]+2]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xecd20 | `r0` | `?` |
