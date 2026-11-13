# ARM scan packet commands and link control in 1.40.5

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
`JTAG_LLP_SC_f` has an unresolved computed transfer (`ldrls` at 0xf4ce4);
its tables and return cover known edges only. Indirect call targets and numeric callee
names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-packet-facts.txt) retain all tool sections.

## `scancmd_create`

- `scancmd_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcbc0 to 0xdcc84.
  Established. The unnamed `0x1b1b4` call has `[sp-32]`, `r1`, `r2`, unknown, with its result stored
  as four bytes at `sp -0x18` (`bl` at 0xdcc24, `str` at 0xdcc2c). Nonzero result takes the branch
  to `0xdcc48` (`bne` at 0xdcc38). The following unnamed `0xdcc84` call has `[sp-20]`,
  `ret(0x1b1b4)`, unknown, `ret(0x1b1b4)` (`bl` at 0xdcc50). Command allocation and layout remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcbd0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdcbd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcbe4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdcbec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdcbf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdcbfc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdcc04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdcc0c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdcc10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdcc1c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdcc20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdcc2c | store | 4 | `sp -0x18` | `ret(0x1b1b4)` |
| `ldr` at 0xdcc30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdcc40 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdcc48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcc4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcc60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdcc6c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdcc74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdcc78 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xdcc7c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdcc80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcc80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcc80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcc24 | `(unknown callee) 0x1b1b4` | `[sp-32]` | `r1` | `r2` | `?` |
| `bl` at 0xdcc50 | `(unknown callee) 0xdcc84` | `[sp-20]` | `ret(0x1b1b4)` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdcc64 | `(unknown callee) 0x1b490` | `[sp-24]` | `?` | `?` | `ret(0xdcc84)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcbe0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdcc38 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xdcc5c | `ret(0xdcc84) != 0x0` | `ret(0xdcc84) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcc80 | `r0` | `?` |

## `scancmd_init`

- `scancmd_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcc84 to 0xdcd20.
  Established. Nonzero entry `r1` takes the branch to `0xdccb4` (`bne` at 0xdcca4). The unnamed
  `0x1b4cc` call has `r1`, `0x0`, `[sp-36]`, unknown (`bl` at 0xdcd0c). Four-byte assignments
  at `sp -0x20` retain `0x0` and `0x1` on separate paths (`str` at 0xdccac, `str` at 0xdcd14).
  Initialization extent and command layout are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcc94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdcc98 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdcc9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdccac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdccb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdccc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdcccc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdccd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdccdc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdcce4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdccec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdccf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdccfc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdcd00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcd08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdcd14 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xdcd18 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdcd1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcd1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcd1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcd0c | `(unknown callee) 0x1b4cc` | `r1` | `0x0` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcca4 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xdccc0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcd1c | `r0` | `?` |

## `scancmd_dump`

- `scancmd_dump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcd50 to 0xdd300.
  Established. Entry `r3 != 0x0` takes the branch to `0xdcd80` (`bne` at 0xdcd78). A later four-byte
  load reads `[sp-32] +0xc`, while an included four-byte stack assignment writes `[sp-32]+16`
  at `sp -0x3c` (`ldr` at 0xdd078, `str` at 0xdd09c). The unnamed `0x240b8` call carries `[sp-20]`,
  `0x135d9c`, `[[sp-32]+12]`, `[sp-32]+12` (`bl` at 0xdd044). Dump field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcd60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdcd64 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdcd68 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdcd6c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdcd70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcd80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdcd84 | store | 4 | `sp -0x40` | `r0` |
| `ldr` at 0xdcd88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcd94 | load | 4 | `abs 0xdd29c` | `-` |
| `ldr` at 0xdcd98 | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xdcd9c | store | 4 | `sp -0x40` | `[0x145ae4]` |
| `ldr` at 0xdcda0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdcda4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xdcda8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcdb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcdb8 | load | 4 | `abs 0xdd2a0` | `-` |
| `ldr` at 0xdcdbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdcdc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcdcc | load | 4 | `abs 0xdd2a4` | `-` |
| `ldr` at 0xdcdd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcdd8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdcde4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcde8 | load | 4 | `abs 0xdd2a8` | `-` |
| `ldr` at 0xdcdf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce00 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdce0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdce10 | load | 4 | `abs 0xdd2ac` | `-` |
| `ldr` at 0xdce1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce20 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdce2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdce30 | load | 4 | `abs 0xdd2b0` | `-` |
| `ldr` at 0xdce40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce48 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdce54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdce58 | load | 4 | `abs 0xdd2b4` | `-` |
| `ldr` at 0xdce64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce68 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdce74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdce78 | load | 4 | `abs 0xdd2b8` | `-` |
| `ldr` at 0xdce88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdce90 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdce9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcea0 | load | 4 | `abs 0xdd2bc` | `-` |
| `ldr` at 0xdceac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdceb0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdceb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcebc | load | 4 | `abs 0xdd2c0` | `-` |
| `ldr` at 0xdcecc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdced4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdcee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcee4 | load | 4 | `abs 0xdd2c4` | `-` |
| `ldr` at 0xdcef4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcefc | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdcf08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcf0c | load | 4 | `abs 0xdd2c8` | `-` |
| `ldr` at 0xdcf1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf24 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdcf30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcf34 | load | 4 | `abs 0xdd2cc` | `-` |
| `ldr` at 0xdcf44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf4c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdcf58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcf5c | load | 4 | `abs 0xdd2d0` | `-` |
| `ldr` at 0xdcf6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf74 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdcf80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcf84 | load | 4 | `abs 0xdd2d4` | `-` |
| `ldr` at 0xdcf94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcf9c | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdcfa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcfac | load | 4 | `abs 0xdd2d8` | `-` |
| `ldr` at 0xdcfbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcfc4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdcfd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcfd4 | load | 4 | `abs 0xdd2dc` | `-` |
| `ldr` at 0xdcfe4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdcfec | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdcff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcff8 | load | 4 | `abs 0xdd2e0` | `-` |
| `ldr` at 0xdd008 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd010 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdd01c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd020 | load | 4 | `abs 0xdd2e4` | `-` |
| `ldr` at 0xdd030 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd03c | load | 4 | `abs 0xdd2e8` | `-` |
| `ldr` at 0xdd040 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdd04c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd054 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdd05c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd060 | load | 4 | `abs 0xdd2ec` | `-` |
| `ldr` at 0xdd070 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd078 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdd084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd094 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd09c | store | 4 | `sp -0x3c` | `[sp-32]+16` |
| `ldr` at 0xdd0a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd0ac | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdd0c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd0cc | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0xdd0d8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdd0dc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd0e4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdd0e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd0f0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xdd0f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd0f8 | load | 4 | `abs 0xdd2f0` | `-` |
| `str` at 0xdd104 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdd108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd10c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd11c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd124 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd12c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd138 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd144 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd150 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd15c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd160 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xdd164 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd170 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd17c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd180 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xdd184 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd190 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd19c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd1a0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xdd1a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd1bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd1c0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xdd1c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1d0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd1dc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd1e0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xdd1e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd1f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd1fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd200 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xdd204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd208 | load | 4 | `abs 0xdd2f4` | `-` |
| `ldr` at 0xdd20c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd210 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd218 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd220 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xdd228 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdd238 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdd23c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd240 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd250 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd264 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdd26c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd270 | load | 4 | `abs 0xdd2f8` | `-` |
| `ldr` at 0xdd274 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdd27c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd284 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xdd28c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd290 | load | 4 | `abs 0xdd2fc` | `-` |
| `ldmdb` at 0xdd298 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd298 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd298 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcdc0 | `(unknown callee) 0x240b8` | `?` | `0x135c18` | `r1` | `r1` |
| `bl` at 0xdcdd0 | `(unknown callee) 0x240b8` | `?` | `0x135c1c` | `r2` | `r1` |
| `bl` at 0xdcdf0 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135c38` | `?` | `?` |
| `bl` at 0xdce18 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135c54` | `?` | `?` |
| `bl` at 0xdce38 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135c64` | `?` | `?` |
| `bl` at 0xdce60 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135c80` | `?` | `?` |
| `bl` at 0xdce80 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135c90` | `?` | `?` |
| `bl` at 0xdcea8 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135cac` | `?` | `?` |
| `bl` at 0xdcec4 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135cbc` | `?` | `?` |
| `bl` at 0xdceec | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135cd8` | `?` | `?` |
| `bl` at 0xdcf14 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135ce8` | `?` | `?` |
| `bl` at 0xdcf3c | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d04` | `?` | `?` |
| `bl` at 0xdcf64 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d14` | `?` | `?` |
| `bl` at 0xdcf8c | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d30` | `?` | `?` |
| `bl` at 0xdcfb4 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d40` | `?` | `?` |
| `bl` at 0xdcfdc | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d5c` | `?` | `?` |
| `bl` at 0xdd000 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d6c` | `?` | `?` |
| `bl` at 0xdd028 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d88` | `?` | `?` |
| `bl` at 0xdd044 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135d9c` | `[[sp-32]+12]` | `[sp-32]+12` |
| `bl` at 0xdd068 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135db4` | `?` | `?` |
| `bl` at 0xdd0fc | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135dc8` | `?` | `?` |
| `bl` at 0xdd214 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135dec` | `?` | `?` |
| `bl` at 0xdd278 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135e20` | `?` | `?` |
| `bl` at 0xdd294 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x135e28` | `?` | `[sp-56]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcd78 | `r3 != 0x0` | `r3 == 0x0` |
| `bne` at 0xdcd90 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xdcdb0 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xdd080 | `[[sp-32]+12] == 0x0` | `[[sp-32]+12] != 0x0` |
| `bne` at 0xdd08c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blo` at 0xdd114 | `C == 0` | `not (C == 0)` |
| `beq` at 0xdd230 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xdd248 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd298 | `r0` | `?` |

## `scancmd_destroy`

- `scancmd_destroy`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdcd20 to 0xdcd50.
  Established. Entry `r0 != 0x0` takes the branch to `0xdcd44`, which includes the unnamed `0x1b490`
  call with `r0`, `r1`, `r2`, `r0` (`bne` at 0xdcd3c, `bl` at 0xdcd48). The exit value is unknown
  (`ldmdb` at 0xdcd4c). Object ownership and destruction effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdcd30 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdcd34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdcd44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdcd4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdcd4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdcd4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdcd48 | `(unknown callee) 0x1b490` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdcd3c | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdcd4c | `r0` | `?` |

## `smpcmd_scan_sel`

- `smpcmd_scan_sel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8734 to 0xe8918.
  Established. A four-byte store through `r1 +0x0` retains `ret(0x1b1b4)` (`str` at 0xe875c). The
  unsigned-lower-or-same branch takes `[[sp-24]+44] <= 0xffff` to `0xe8800` (`bls` at 0xe87e0). The
  unnamed `0xdde78` call has unknown, `[[sp-24]+40]`, `[[sp-24]+44]`, `0x0` (`bl` at 0xe88ac).
  Selection packet fields and external effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8744 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8748 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe874c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe875c | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe8760 | load | 4 | `abs 0xe8900` | `-` |
| `ldr` at 0xe8764 | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe8770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8774 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8778 | load | 4 | `abs 0xe8904` | `-` |
| `str` at 0xe8784 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xe8790 | store | 4 | `sp -0x20` | `0x58` |
| `ldr` at 0xe8794 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8798 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe879c | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xe87a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe87ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe87b4 | load | 4 | `abs 0xe8908` | `-` |
| `str` at 0xe87c0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe87c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe87cc | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xe87d0 | store | 4 | `sp -0x2c` | `[[sp-24]+44]` |
| `ldr` at 0xe87d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe87d8 | load | 4 | `abs 0xe890c` | `-` |
| `ldr` at 0xe87e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe87e8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe87ec | load | 4 | `abs 0xe8910` | `-` |
| `str` at 0xe87f8 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xe8804 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xe880c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe8810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8814 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8818 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8828 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8830 | store | 4 | `sp -0x20` | `0x5a` |
| `ldr` at 0xe8834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8838 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xe883c | store | 4 | `sp -0x30` | `[[sp-24]+48]` |
| `ldr` at 0xe8840 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8844 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8848 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8858 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8860 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe8864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8868 | load | 4 | `[sp-24] +0x34` | `-` |
| `str` at 0xe886c | store | 4 | `sp -0x34` | `[[sp-24]+52]` |
| `ldr` at 0xe8870 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8874 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0xe8878 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe887c | store | 4 | `sp -0xe8` | `?` |
| `str` at 0xe8884 | store | 4 | `sp -0xe4` | `sp-56` |
| `str` at 0xe888c | store | 4 | `sp -0xe0` | `sp-40` |
| `ldr` at 0xe8890 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8894 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe8898 | store | 4 | `sp -0xdc` | `[[sp-28]]` |
| `ldr` at 0xe889c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe88a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe88a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe88bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe88c0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe88c4 | load | 4 | `abs 0xe8904` | `-` |
| `str` at 0xe88d0 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe88d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe88dc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe88e0 | load | 4 | `abs 0xe8914` | `-` |
| `ldr` at 0xe88e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe88e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe88f4 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xe88f8 | load | 4 | `sp -0xd8` | `-` |
| `ldmdb` at 0xe88fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe88fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe88fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe88fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8754 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe877c | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe87b8 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136898` | `?` | `[sp-28]` |
| `bl` at 0xe87f0 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x1368cc` | `[[sp-24]+44]` | `[sp-28]` |
| `bl` at 0xe88ac | `(unknown callee) 0xdde78` | `?` | `[[sp-24]+40]` | `[[sp-24]+44]` | `0x0` |
| `bl` at 0xe88c8 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe88ec | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x1368f4` | `[sp-56]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe876c | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `bls` at 0xe87a8 | `[[sp-24]+40] <= 0xff (unsigned, 32-bit)` | `[[sp-24]+40] > 0xff (unsigned, 32-bit)` |
| `bls` at 0xe87e0 | `[[sp-24]+44] <= 0xffff (unsigned, 32-bit)` | `[[sp-24]+44] > 0xffff (unsigned, 32-bit)` |
| `beq` at 0xe8824 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe8854 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe88b8 | `ret(0xdde78) != 0x0` | `ret(0xdde78) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe88fc | `r0` | `0x0` |

## `smpcmd_scan_config`

- `smpcmd_scan_config`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8918 to 0xe89ac.
  Established. An included four-byte assignment through `r1 +0x0` writes `ret(0x1b1b4)` (`str` at
  0xe8940). Nonzero `[0x2adfd4]` takes the branch to `0xe8970` (`bne` at 0xe8950). The
  unknown-target call has `ret(0x1b1b4)`, unknown, unknown, `[0x143860]` (`mov` at 0xe897c).
  Configuration display effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8928 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe892c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8930 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8940 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe8944 | load | 4 | `abs 0xe89a0` | `-` |
| `ldr` at 0xe8948 | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe8954 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8958 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe895c | load | 4 | `abs 0xe89a4` | `-` |
| `str` at 0xe8968 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8970 | load | 4 | `abs 0xe89a8` | `-` |
| `ldr` at 0xe8974 | load | 4 | `abs 0x143860` | `-` |
| `ldr` at 0xe8980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8984 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8988 | load | 4 | `abs 0xe89a4` | `-` |
| `str` at 0xe8994 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8998 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe899c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe899c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe899c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe899c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8938 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8960 | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `mov` at 0xe897c | `(unknown callee) (unknown target)` | `ret(0x1b1b4)` | `?` | `?` | `[0x143860]` |
| `bl` at 0xe898c | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8950 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe899c | `r0` | `0x0` |

## `smpcmd_scan_reset`

- `smpcmd_scan_reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe85f4 to 0xe8734.
  Established. The two unnamed `0xdd33c` calls carry `0x0`, `[sp-32]`, `[[sp-28]]`, `[sp-28]`, and
  `0x1`, `[sp-32]`, `[[sp-28]]`, `[sp-28]` (`bl` at 0xe8688, `bl` at 0xe86dc). Their respective
  nonzero results take branches to `0xe86b4` and `0xe8708` (`bne` at 0xe8694, `bne` at 0xe86e8). The
  exit carries `0x0` (`ldmdb` at 0xe8724). Reset signaling and packet-field meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8604 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8608 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe860c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe861c | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe8620 | load | 4 | `abs 0xe8728` | `-` |
| `ldr` at 0xe8624 | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe8630 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8634 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8638 | load | 4 | `abs 0xe872c` | `-` |
| `str` at 0xe8644 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe865c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe8660 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8664 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8668 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8678 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8680 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8684 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8698 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe869c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe86a0 | load | 4 | `abs 0xe872c` | `-` |
| `str` at 0xe86ac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe86b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe86b8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe86bc | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe86cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe86d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe86d8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe86ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe86f0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe86f4 | load | 4 | `abs 0xe872c` | `-` |
| `str` at 0xe8700 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8708 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe870c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8710 | load | 4 | `abs 0xe8730` | `-` |
| `str` at 0xe871c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8720 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe8724 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8724 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8724 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8724 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8614 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe863c | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe864c | `(unknown callee) 0xe623c` | `ret(0x1b1b4)` | `?` | `?` | `[0x2adfd4]` |
| `bl` at 0xe8688 | `(unknown callee) 0xdd33c` | `0x0` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe86a4 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe86dc | `(unknown callee) 0xdd33c` | `0x1` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe86f8 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe8714 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136874` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe862c | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `beq` at 0xe8674 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe8694 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `beq` at 0xe86c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe86e8 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8724 | `r0` | `0x0` |

## `smpcmd_scan_debug`

- `smpcmd_scan_debug`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8fc4 to 0xe9068.
  Established. A four-byte global assignment at `0x2adfd8` retains `[[sp-24]+40]` (`str` at
  0xe9034). The unsigned-lower-or-same branch takes `[[sp-24]+40] <= 0x4` to `0xe902c` (`bls` at
  0xe9018). The unnamed `0x1b52c` call has `[[sp-28]]`, `0x136b04`, `[0x2adfd8]`, `[sp-28]` (`bl` at
  0xe904c). Debug level meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8fd4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8fd8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8fdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8fec | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe8ff0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8ff4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8ff8 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe9004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9008 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe900c | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe9010 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe901c | load | 4 | `abs 0xe905c` | `-` |
| `ldr` at 0xe902c | load | 4 | `abs 0xe9060` | `-` |
| `ldr` at 0xe9030 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9034 | store | 4 | `abs 0x2adfd8` | `[[sp-24]+40]` |
| `ldr` at 0xe9038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe903c | load | 4 | `abs 0xe9060` | `-` |
| `ldr` at 0xe9040 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe9044 | load | 4 | `abs 0xe9064` | `-` |
| `ldr` at 0xe9048 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldmdb` at 0xe9058 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9058 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9058 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8fe4 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe9024 | `(unknown callee) 0x23fbc` | `0x136ac8` | `0x4` | `?` | `[[sp-24]+40]` |
| `bl` at 0xe904c | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136b04` | `[0x2adfd8]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9000 | `[[[sp-24]+112]] != 0x1` | `[[[sp-24]+112]] == 0x1` |
| `bls` at 0xe9018 | `[[sp-24]+40] <= 0x4 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9058 | `r0` | `0x0` |

## `smpcmd_scan_imp`

- `smpcmd_scan_imp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9160 to 0xe91b4.
  Established. The unnamed `0x1b1b4` call carries `0xa0`, `r1`, `r2`, `r3` (`bl` at 0xe9180). A
  four-byte assignment through `r1 +0x0` retains its result (`str` at 0xe9188). The following
  unnamed `0x1b52c` call has `[[sp-28]]`, `0x136b1c`, unknown, `[sp-28]` (`bl` at 0xe91a0). The exit
  carries `0x0` (`ldmdb` at 0xe91ac). The command role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9170 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe9174 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe9178 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9188 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `str` at 0xe9190 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe9194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9198 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe919c | load | 4 | `abs 0xe91b0` | `-` |
| `ldmdb` at 0xe91ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe91ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe91ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe91ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9180 | `(unknown callee) 0x1b1b4` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xe91a0 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136b1c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe91ac | `r0` | `0x0` |

## `smpcmd_scan_port`

- `smpcmd_scan_port`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe89ac to 0xe8ac4.
  Established. A four-byte load reads `[[sp-24]+112] +0x0` (`ldr` at 0xe8a0c); its zero value takes
  the branch to `0xe8a7c` (`beq` at 0xe8a14). The included unknown-target call has `[sp-32]`,
  `[sp-36]`, unknown, `[0x14385c]` (`mov` at 0xe8a78). The unnamed `0x1b52c` call uses `[[sp-28]]`,
  `0x136924`, `[0x2adfe4]`, `[0x2adfe8]` (`bl` at 0xe8a98). Port packet encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe89bc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe89c0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe89c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe89d4 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe89d8 | load | 4 | `abs 0xe8aac` | `-` |
| `ldr` at 0xe89dc | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe89e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe89ec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe89f0 | load | 4 | `abs 0xe8ab0` | `-` |
| `str` at 0xe89fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8a04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a08 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8a0c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8a18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a1c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe8a20 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe8a2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a30 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8a34 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe8a38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a3c | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xe8a40 | store | 4 | `sp -0x24` | `[[sp-24]+44]` |
| `ldr` at 0xe8a48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8a4c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8a50 | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xe8a58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8a60 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe8a64 | load | 4 | `abs 0xe8ab4` | `-` |
| `ldr` at 0xe8a68 | load | 4 | `abs 0x14385c` | `-` |
| `ldr` at 0xe8a6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8a70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8a7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8a80 | load | 4 | `abs 0xe8ab8` | `-` |
| `ldr` at 0xe8a84 | load | 4 | `abs 0xe8abc` | `-` |
| `ldr` at 0xe8a88 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8a8c | load | 4 | `abs 0xe8ac0` | `-` |
| `ldr` at 0xe8a90 | load | 4 | `abs 0x2adfe4` | `-` |
| `ldr` at 0xe8a94 | load | 4 | `abs 0x2adfe8` | `-` |
| `str` at 0xe8aa0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe8aa4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe8aa8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8aa8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8aa8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8aa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe89cc | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe89f4 | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `mov` at 0xe8a78 | `(unknown callee) (unknown target)` | `[sp-32]` | `[sp-36]` | `?` | `[0x14385c]` |
| `bl` at 0xe8a98 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136924` | `[0x2adfe4]` | `[0x2adfe8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe89e4 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `beq` at 0xe8a14 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `bne` at 0xe8a28 | `[[[sp-24]+112]] != 0x2` | `[[[sp-24]+112]] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8aa8 | `r0` | `0x0` |

## `smpcmd_scan_ids`

- `smpcmd_scan_ids`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8ac4 to 0xe8fc4.
  Established. The unsigned-lower-or-same branch takes `[[sp-24]+40] <= 0x40` to `0xe8b54` (`bls` at
  0xe8b30). Two included four-byte stores clear unknown effective targets (`str` at 0xe8c64, `str`
  at 0xe8c7c). The unnamed `0xdd8e8` call has `0x1`, `0x0`, `0x1`, `[sp-40]` (`bl` at 0xe8d24).
  Later byte and halfword reads target `sp -0x30` and `sp -0x2e` (`ldrb` at 0xe8ee4, `ldrh` at
  0xe8efc). ID decoding and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8ad4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe8ad8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe8adc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe8aec | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe8af0 | load | 4 | `abs 0xe8f88` | `-` |
| `ldr` at 0xe8af4 | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe8b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8b04 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8b08 | load | 4 | `abs 0xe8f8c` | `-` |
| `str` at 0xe8b14 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8b1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8b20 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xe8b24 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xe8b28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8b38 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8b3c | load | 4 | `abs 0xe8f90` | `-` |
| `str` at 0xe8b4c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8b54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8b60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8b64 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8b68 | load | 4 | `abs 0xe8f94` | `-` |
| `str` at 0xe8b74 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8b7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8b84 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe8b88 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe8b9c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe8ba8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe8bac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8bc0 | store | 4 | `sp -0x40` | `ret(0x1b1b4)` |
| `ldr` at 0xe8bc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8bd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8bd4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8bd8 | load | 4 | `abs 0xe8f98` | `-` |
| `str` at 0xe8be4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8bec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8c00 | store | 4 | `sp -0x44` | `ret(0x1b1b4)` |
| `ldr` at 0xe8c04 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8c10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8c14 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8c18 | load | 4 | `abs 0xe8f98` | `-` |
| `ldr` at 0xe8c20 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8c2c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xe8c38 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe8c3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8c40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8c50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8c58 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8c64 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe8c68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8c70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8c7c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe8c80 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8c88 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe8ca0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8cac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8cb0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8cc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8cc8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8ccc | load | 4 | `abs 0xe8f8c` | `-` |
| `ldr` at 0xe8cd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8cdc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8ce8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8cf0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe8cf4 | store | 4 | `sp -0x58` | `[sp-56]` |
| `ldr` at 0xe8cf8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe8cfc | store | 4 | `sp -0x54` | `[sp-64]` |
| `ldr` at 0xe8d00 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8d04 | store | 4 | `sp -0x50` | `[sp-68]` |
| `ldr` at 0xe8d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8d0c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe8d10 | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0xe8d20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8d34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8d38 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8d3c | load | 4 | `abs 0xe8f8c` | `-` |
| `ldr` at 0xe8d44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8d4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8d58 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8d60 | load | 4 | `abs 0xe8f9c` | `-` |
| `ldr` at 0xe8d68 | load | 4 | `abs 0xe8fa0` | `-` |
| `ldr` at 0xe8d70 | load | 4 | `abs 0xe8fa4` | `-` |
| `ldr` at 0xe8d78 | load | 4 | `abs 0xe8fa8` | `-` |
| `str` at 0xe8d84 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xe8d8c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe8d90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8d94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8da4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe8da8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8dbc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe8dc4 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xe8dc8 | load | 4 | `abs 0xe8fac` | `-` |
| `ldr` at 0xe8dcc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8ddc | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xe8de8 | store | 4 | `sp -0x58` | `0x20` |
| `ldr` at 0xe8dec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe8df0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe8e00 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe8e08 | store | 4 | `sp -0x3c` | `[sp-60]+32` |
| `ldr` at 0xe8e0c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e10 | load | 4 | `abs 0xe8fb0` | `-` |
| `ldr` at 0xe8e1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e24 | load | 4 | `abs 0xe8fb0` | `-` |
| `str` at 0xe8e30 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8e34 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e38 | load | 4 | `abs 0xe8fb4` | `-` |
| `ldr` at 0xe8e44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e4c | load | 4 | `abs 0xe8fb4` | `-` |
| `str` at 0xe8e58 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8e5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e60 | load | 4 | `abs 0xe8fb8` | `-` |
| `ldr` at 0xe8e6c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e74 | load | 4 | `abs 0xe8fb8` | `-` |
| `str` at 0xe8e80 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8e84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8e94 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8ea8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8eac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe8ebc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8ed0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe8ed8 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xe8ee4 | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0xe8eec | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0xe8efc | load | 2 | `sp -0x2e` | `-` |
| `str` at 0xe8f14 | store | 4 | `sp -0x58` | `?` |
| `ldrb` at 0xe8f18 | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xe8f24 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xe8f28 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe8f2c | store | 4 | `sp -0x50` | `[sp-52]` |
| `ldr` at 0xe8f30 | load | 4 | `abs 0xe8fbc` | `-` |
| `ldr` at 0xe8f34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8f40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe8f48 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe8f50 | load | 4 | `abs 0xe8f8c` | `-` |
| `ldr` at 0xe8f58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8f5c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe8f60 | load | 4 | `abs 0xe8fc0` | `-` |
| `ldr` at 0xe8f68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe8f70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe8f7c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe8f80 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xe8f84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe8f84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8f84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8f84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8ae4 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe8b0c | `(unknown callee) 0x1b598` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe8b44 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136944` | `0x40` | `[sp-28]` |
| `bl` at 0xe8b6c | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x13696c` | `?` | `[sp-28]` |
| `bl` at 0xe8bb8 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xe8bdc | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136990` | `?` | `[sp-28]` |
| `bl` at 0xe8bf8 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xe8c1c | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136990` | `?` | `[sp-28]` |
| `bl` at 0xe8c24 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe8c90 | `(unknown callee) 0xe623c` | `ret(0x1b1b4)` | `?` | `?` | `[sp-36]` |
| `bl` at 0xe8cb4 | `(unknown callee) 0xdd33c` | `0x1` | `[sp-40]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xe8cd0 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe8cd8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe8ce0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe8d24 | `(unknown callee) 0xdd8e8` | `0x1` | `0x0` | `0x1` | `[sp-40]` |
| `bl` at 0xe8d40 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe8d48 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe8d50 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe8d64 | `(unknown callee) 0x23fbc` | `0x1369bc` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe8d6c | `(unknown callee) 0x23fbc` | `0x1369c0` | `?` | `?` | `?` |
| `bl` at 0xe8d74 | `(unknown callee) 0x23fbc` | `0x1369d8` | `?` | `?` | `?` |
| `bl` at 0xe8d7c | `(unknown callee) 0x23fbc` | `0x136a08` | `?` | `?` | `?` |
| `bl` at 0xe8dac | `(unknown callee) 0xe2688` | `?` | `[sp-68]` | `?` | `[sp-32]` |
| `bl` at 0xe8dd0 | `(unknown callee) 0x23fbc` | `0x136a38` | `[sp-44]` | `?` | `[sp-60]+1` |
| `bl` at 0xe8dfc | `(unknown callee) 0xe2b10` | `[sp-60]` | `[sp-68]` | `0x0` | `sp-52` |
| `bl` at 0xe8ee0 | `(unknown callee) 0xe9208` | `[sp-52]` | `sp-48` | `?` | `sp-48` |
| `bl` at 0xe8f3c | `(unknown callee) 0x23fbc` | `0x136a64` | `[sp-44]` | `?` | `?` |
| `bl` at 0xe8f54 | `(unknown callee) 0x23fbc` | `0x136870` | `?` | `?` | `[sp-32]` |
| `bl` at 0xe8f64 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136a9c` | `?` | `[sp-28]` |
| `bl` at 0xe8f6c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe8f74 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8afc | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `bls` at 0xe8b30 | `[[sp-24]+40] <= 0x40 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x40 (unsigned, 32-bit)` |
| `bne` at 0xe8b5c | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bne` at 0xe8bcc | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xe8c0c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `blo` at 0xe8c48 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe8cc0 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xe8d30 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe8d9c | `C == 0` | `not (C == 0)` |
| `bne` at 0xe8db8 | `ret(0xe2688) != 0x0` | `ret(0xe2688) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8f84 | `r0` | `0x0` |

## `smpcmd_scan_set`

- `smpcmd_scan_set`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9068 to 0xe9160.
  Established. The nested four-byte load at `[[sp-24]+112] +0x4` supplies the zero test that takes
  the branch to `0xe9118` (`ldr` at 0xe90f0, `beq` at 0xe90f8). The unnamed `0xe6590` call carries
  `[sp-36]` followed by three unknown candidates (`bl` at 0xe9138). The final unnamed `0xe6360` call
  has `[[sp-28]]`, unknown, unknown, `[sp-28]` (`bl` at 0xe9144). Signal-setting semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9078 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe907c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe9080 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9090 | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xe9094 | load | 4 | `abs 0xe9158` | `-` |
| `ldr` at 0xe9098 | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe90a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe90a8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe90ac | load | 4 | `abs 0xe915c` | `-` |
| `str` at 0xe90b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe90c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe90c4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe90c8 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xe90dc | store | 4 | `sp -0x24` | `ret(0xe623c)` |
| `ldr` at 0xe90e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe90e8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe90f0 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldr` at 0xe90fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9100 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe9104 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9108 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xe9110 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9118 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe911c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xe9120 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xe9128 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe9130 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe913c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9140 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xe914c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe9150 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe9154 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9154 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9154 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9088 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xe90b0 | `(unknown callee) 0x1b364` | `[[sp-28]]` | `0x136870` | `?` | `[sp-28]` |
| `bl` at 0xe90d4 | `(unknown callee) 0xe623c` | `ret(0x1b1b4)` | `?` | `?` | `[[[sp-24]+112]]` |
| `bl` at 0xe9138 | `(unknown callee) 0xe6590` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xe9144 | `(unknown callee) 0xe6360` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe90a0 | `[0x2adfd4] != 0x0` | `[0x2adfd4] == 0x0` |
| `beq` at 0xe90d0 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `beq` at 0xe90f8 | `[[[sp-24]+112]+4] == 0x0` | `[[[sp-24]+112]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9154 | `r0` | `0x0` |

## `smpcmd_scan_test`

- `smpcmd_scan_test`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe91b4 to 0xe9208.
  Established. A four-byte assignment through `r1 +0x0` retains `ret(0x1b1b4)` (`str` at 0xe91dc).
  An explicit four-byte stack store clears `sp -0x20` (`str` at 0xe91e4). The unnamed `0x1b52c` call
  has `[[sp-28]]`, `0x136b54`, unknown, `[sp-28]` (`bl` at 0xe91f4). No conditional branches appear
  in this span. Test effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe91c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe91c8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe91cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe91dc | store | 4 | `r1 +0x0` | `ret(0x1b1b4)` |
| `str` at 0xe91e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe91e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe91ec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xe91f0 | load | 4 | `abs 0xe9204` | `-` |
| `ldmdb` at 0xe9200 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9200 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9200 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9200 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe91d4 | `(unknown callee) 0x1b1b4` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xe91f4 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x136b54` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9200 | `r0` | `0x0` |

## `scanpktInit`

- `scanpktInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe931c to 0xe94a8.
  Established. Zero `[0x2adff8]` takes the branch to `0xe935c`, followed by a four-byte global
  assignment of `[0x2adff8]+1` at `0x2adff8` (`beq` at 0xe9338, `str` at 0xe936c). The unnamed
  `0x22480` call has `0x2b4708`, `0x136bf8`, `0xe94a8`, `0x0` (`bl` at 0xe940c). Included four-byte
  stack assignments retain `0x800`, `0x19`, `0xa`, `0x4` at distinct sites (`str` at 0xe93e0, `str`
  at 0xe93e8, `str` at 0xe93f0, `str` at 0xe93f8). Packet task setup and external effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe932c | load | 4 | `abs 0xe947c` | `-` |
| `ldr` at 0xe9330 | load | 4 | `abs 0x2adff8` | `-` |
| `ldr` at 0xe933c | load | 4 | `abs 0xe9480` | `-` |
| `str` at 0xe9340 | store | 4 | `sp -0x2c` | `0x136b80` |
| `ldr` at 0xe9348 | load | 4 | `abs 0xe9484` | `-` |
| `ldr` at 0xe935c | load | 4 | `abs 0xe947c` | `-` |
| `ldr` at 0xe9360 | load | 4 | `abs 0xe947c` | `-` |
| `ldr` at 0xe9364 | load | 4 | `abs 0x2adff8` | `-` |
| `str` at 0xe936c | store | 4 | `abs 0x2adff8` | `[0x2adff8]+1` |
| `ldr` at 0xe9370 | load | 4 | `abs 0xe9488` | `-` |
| `ldr` at 0xe9374 | load | 4 | `abs 0xe948c` | `-` |
| `str` at 0xe9384 | store | 4 | `sp -0x14` | `ret(0xb5f4c)` |
| `ldr` at 0xe9388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe93a0 | load | 4 | `abs 0xe9490` | `-` |
| `str` at 0xe93a4 | store | 4 | `sp -0x2c` | `0x136bd0` |
| `str` at 0xe93a8 | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0xe93b0 | load | 4 | `abs 0xe9484` | `-` |
| `ldr` at 0xe93c4 | load | 4 | `abs 0xe9494` | `-` |
| `str` at 0xe93d8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xe93e0 | store | 4 | `sp -0x28` | `0x800` |
| `str` at 0xe93e8 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xe93f0 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xe93f8 | store | 4 | `sp -0x1c` | `0x4` |
| `ldr` at 0xe93fc | load | 4 | `abs 0xe9498` | `-` |
| `ldr` at 0xe9400 | load | 4 | `abs 0xe949c` | `-` |
| `ldr` at 0xe9404 | load | 4 | `abs 0xe94a0` | `-` |
| `ldr` at 0xe9420 | load | 4 | `abs 0xe9494` | `-` |
| `ldr` at 0xe9424 | load | 4 | `abs 0xe9494` | `-` |
| `ldr` at 0xe9428 | load | 4 | `abs 0x2b4700` | `-` |
| `ldr` at 0xe9430 | load | 4 | `abs 0xe9488` | `-` |
| `str` at 0xe943c | store | 4 | `sp -0x14` | `ret(0xb4b18)` |
| `ldr` at 0xe9440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe944c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9458 | load | 4 | `abs 0xe94a4` | `-` |
| `str` at 0xe945c | store | 4 | `sp -0x2c` | `0x136c04` |
| `str` at 0xe9460 | store | 4 | `sp -0x28` | `ret(0x21704)` |
| `ldr` at 0xe9468 | load | 4 | `abs 0xe9484` | `-` |
| `ldmdb` at 0xe9478 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9478 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9354 | `(unknown callee) 0x21eb8` | `0x4` | `0x136bac` | `0x42` | `0x0` |
| `bl` at 0xe937c | `(unknown callee) 0xb5f4c` | `0x2b470c` | `0x136bc4` | `0x14` | `[0x2adff8]+1` |
| `bl` at 0xe9398 | `(unknown callee) 0x21704` | `ret(0xb5f4c)` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0xe93bc | `(unknown callee) 0x21eb8` | `0x4` | `0x136bac` | `0x5a` | `0x0` |
| `bl` at 0xe93d0 | `(unknown callee) 0xb5e34` | `0x2b4700` | `0xf` | `0x0` | `ret(0xb5f4c)` |
| `bl` at 0xe940c | `(unknown callee) 0x22480` | `0x2b4708` | `0x136bf8` | `0xe94a8` | `0x0` |
| `bl` at 0xe9434 | `(unknown callee) 0xb4b18` | `0x2b4700` | `[0x2b4700]` | `0x1` | `0x2b470c` |
| `bl` at 0xe9450 | `(unknown callee) 0x21704` | `ret(0xb4b18)` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0xe9474 | `(unknown callee) 0x21eb8` | `0x4` | `0x136bac` | `0x74` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe9338 | `[0x2adff8] == 0x0` | `[0x2adff8] != 0x0` |
| `beq` at 0xe9390 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0xe9418 | `ret(0x22480) == 0x0` | `ret(0x22480) != 0x0` |
| `beq` at 0xe9448 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9478 | `r0` | `?` |

## `JTAG_LLP_SC_f`

- `JTAG_LLP_SC_f`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf4bec to 0xf562c.
  Established. Two four-byte reads target `r0 +0x4` (`ldr` at 0xf4c0c, `ldr` at 0xf4c24). The
  unnamed `0xebe18` call has unknown, unknown, `0x4`, unknown (`bl` at 0xf4c38). The computed
  transfer takes an unresolved edge for `? <= 0x5`, unsigned 32-bit, and falls through for `? > 0x5`
  (`ldrls` at 0xf4ce4); its known fall-through includes a four-byte assignment of `0xfffff7c0` at
  `sp -0x48` (`str` at 0xf4da8). Selector-specific link operations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf4bfc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf4c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4c0c | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xf4c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4c24 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xf4c40 | store | 4 | `sp -0x18` | `ret(0xebe18)` |
| `ldr` at 0xf4c44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4c50 | load | 4 | `abs 0xf5608` | `-` |
| `str` at 0xf4c54 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xf4c5c | load | 4 | `abs 0xf560c` | `-` |
| `ldr` at 0xf4c60 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xf4c6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4c78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4c7c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xf4c88 | load | 4 | `abs 0xf5610` | `-` |
| `ldr` at 0xf4c90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf4c98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf4ca0 | store | 4 | `sp -0x2c` | `[sp-20]+16` |
| `ldr` at 0xf4ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf4ca8 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xf4cb4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf4cb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf4cbc | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xf4ccc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf4cd0 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xf4ce4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf4da4 | load | 4 | `abs 0xf5614` | `-` |
| `str` at 0xf4da8 | store | 4 | `sp -0x48` | `0xfffff7c0` |
| `ldr` at 0xf5600 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xf5604 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5604 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5604 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf4c38 | `(unknown callee) 0xebe18` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xf4c70 | `(unknown callee) 0xeaab4` | `[sp-24]` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0xf4c94 | `(unknown callee) 0x23fbc` | `0x13a3fc` | `ret(0xeaab4)` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf4c4c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xf4c68 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0xf4ce4 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5604 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xf4ce4).
