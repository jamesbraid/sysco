# ARM TIO link and reset paths in 1.40.5

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
The selected spans have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/tio-link-reset-facts.txt) retain all tool sections.

## `OpusTio_BrickOpsInit`

- `OpusTio_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bf40 to 0x2c0ec.
  Established. Included four-byte assignments at `r0 +0x0` and `[sp-20] +0x4` write `0x2bee8` and
  `0x2bc40` respectively (`str` at 0x2bf64, `str` at 0x2bf70). A byte assignment at `[sp-20] +0xc`
  writes `0x52` (`strb` at 0x2bf7c). A later four-byte assignment at `[sp-20] +0xf0` writes
  `0x2b338` (`str` at 0x2c0a8). Stored pointers and board-operation fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bf50 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2bf54 | load | 4 | `abs 0x2c0b0` | `-` |
| `str` at 0x2bf58 | store | 4 | `sp -0x18` | `0x1432a4` |
| `ldr` at 0x2bf5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf60 | load | 4 | `abs 0x2c0b4` | `-` |
| `str` at 0x2bf64 | store | 4 | `r0 +0x0` | `0x2bee8` |
| `ldr` at 0x2bf68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bf6c | load | 4 | `abs 0x2c0b8` | `-` |
| `str` at 0x2bf70 | store | 4 | `[sp-20] +0x4` | `0x2bc40` |
| `ldr` at 0x2bf74 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2bf7c | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x2bf80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf88 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2bf8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bf94 | store | 4 | `[sp-20] +0xa0` | `0x11` |
| `ldr` at 0x2bf98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfa0 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2bfa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfac | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2bfb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfb8 | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2bfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfc0 | load | 4 | `abs 0x2c0bc` | `-` |
| `str` at 0x2bfc4 | store | 4 | `[sp-20] +0x14` | `0x10380c` |
| `ldr` at 0x2bfc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfcc | load | 4 | `abs 0x2c0c0` | `-` |
| `str` at 0x2bfd0 | store | 4 | `[sp-20] +0xb0` | `0x10eee4` |
| `ldr` at 0x2bfd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2bfd8 | load | 4 | `abs 0x2c0c4` | `-` |
| `str` at 0x2bfdc | store | 4 | `[sp-20] +0x7c` | `0x2ae54` |
| `ldr` at 0x2bfe0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bfe8 | store | 4 | `[sp-20] +0x68` | `0x1e` |
| `ldr` at 0x2bfec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2bff4 | store | 4 | `[sp-20] +0x6c` | `0x2d` |
| `ldr` at 0x2bff8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c000 | store | 4 | `[sp-20] +0x70` | `0x32` |
| `ldr` at 0x2c004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c00c | store | 4 | `[sp-20] +0x74` | `0x37` |
| `ldr` at 0x2c010 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c018 | store | 4 | `[sp-20] +0x78` | `0x17` |
| `ldr` at 0x2c01c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c024 | store | 4 | `[sp-20] +0x8c` | `0x1` |
| `ldr` at 0x2c028 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c030 | store | 4 | `[sp-20] +0x88` | `0x2` |
| `ldr` at 0x2c034 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c038 | load | 4 | `abs 0x2c0c8` | `-` |
| `str` at 0x2c03c | store | 4 | `[sp-20] +0x90` | `0x60688` |
| `ldr` at 0x2c040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c044 | load | 4 | `abs 0x2c0cc` | `-` |
| `str` at 0x2c048 | store | 4 | `[sp-20] +0x94` | `0x60770` |
| `ldr` at 0x2c04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c050 | load | 4 | `abs 0x2c0d0` | `-` |
| `str` at 0x2c054 | store | 4 | `[sp-20] +0x98` | `0x60700` |
| `ldr` at 0x2c058 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c060 | store | 4 | `[sp-20] +0xb4` | `0x1` |
| `ldr` at 0x2c064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c068 | load | 4 | `abs 0x2c0d4` | `-` |
| `str` at 0x2c06c | store | 4 | `[sp-20] +0xc4` | `0x2ab5c` |
| `ldr` at 0x2c070 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c074 | load | 4 | `abs 0x2c0d8` | `-` |
| `str` at 0x2c078 | store | 4 | `[sp-20] +0xc8` | `0x2ac18` |
| `ldr` at 0x2c07c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c080 | load | 4 | `abs 0x2c0dc` | `-` |
| `str` at 0x2c084 | store | 4 | `[sp-20] +0xd0` | `0x2acd0` |
| `ldr` at 0x2c088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c08c | load | 4 | `abs 0x2c0e0` | `-` |
| `str` at 0x2c090 | store | 4 | `[sp-20] +0xd4` | `0x2ad20` |
| `ldr` at 0x2c094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c098 | load | 4 | `abs 0x2c0e4` | `-` |
| `str` at 0x2c09c | store | 4 | `[sp-20] +0xec` | `0x2b424` |
| `ldr` at 0x2c0a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c0a4 | load | 4 | `abs 0x2c0e8` | `-` |
| `str` at 0x2c0a8 | store | 4 | `[sp-20] +0xf0` | `0x2b338` |
| `ldmdb` at 0x2c0ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c0ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c0ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c0ac | `r0` | `r0` |

## `tio_test_pci_reset_int`

- `tio_test_pci_reset_int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c260 to 0x2c2cc.
  Established. The branch takes `(r1 & 0xff) != 0xc` to `0x2c2c8` and falls through for equality
  (`bne` at 0x2c298). The unnamed `0x5469c` call carries `0x0`, `(r1 & 0xff)`, `0x2`, `(r3 & 0xff)`
  (`bl` at 0x2c2c4). Reset-interrupt selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c270 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x2c27c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x2c284 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x2c28c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x2c290 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2c29c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x2c2ac | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2c2b0 | load | 1 | `sp -0x17` | `-` |
| `ldmdb` at 0x2c2c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c2c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c2c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c2c4 | `(unknown callee) 0x5469c` | `0x0` | `(r1 & 0xff)` | `0x2` | `(r3 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c298 | `(r1 & 0xff) != 0xc` | `(r1 & 0xff) == 0xc` |
| `beq` at 0x2c2a8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c2c8 | `r0` | `?` |

## `TIOTest_SetSystemAceMode`

- `TIOTest_SetSystemAceMode`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2ce4c to 0x2cf14.
  Established. The unnamed `0x1f26c` call carries `0x99`, `r1`, `r2`, `r3` (`bl` at 0x2ce60); its
  zero result takes the branch to `0x2ce88` (`beq` at 0x2ce78). Two unnamed `0xd73b0` calls have
  `0x0`, `0xb`, `0x80`, `[sp-32]` and `0x0`, `0xc`, `0x38`, `[sp-36]` respectively (`bl` at 0x2cea0,
  `bl` at 0x2cedc). Mode selector and register meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2ce64 | store | 4 | `sp -0x14` | `ret(0x1f26c)` |
| `str` at 0x2ce6c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2ce70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2ce80 | store | 4 | `sp -0x20` | `0x80` |
| `str` at 0x2ce8c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2ce9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2cea8 | store | 4 | `sp -0x1c` | `ret(0xd73b0)` |
| `ldr` at 0x2ceac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2cebc | store | 4 | `sp -0x24` | `0x10` |
| `str` at 0x2cec8 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x2ced8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2cee4 | store | 4 | `sp -0x18` | `ret(0xd73b0)` |
| `ldr` at 0x2cee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2cef4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2cf00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2cf04 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x2cf08 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2cf10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2cf10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2cf10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2ce60 | `(unknown callee) 0x1f26c` | `0x99` | `r1` | `r2` | `r3` |
| `bl` at 0x2cea0 | `(unknown callee) 0xd73b0` | `0x0` | `0xb` | `0x80` | `[sp-32]` |
| `bl` at 0x2cedc | `(unknown callee) 0xd73b0` | `0x0` | `0xc` | `0x38` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2ce78 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `beq` at 0x2ceb4 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `beq` at 0x2cef0 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |
| `bne` at 0x2cefc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2cf10 | `r0` | `[sp-28]` |

## `TIOTest_BrickOpsInit`

- `TIOTest_BrickOpsInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d024 to 0x2d16c.
  Established. A byte assignment at `[sp-20] +0xc` writes `0x0` (`strb` at 0x2d060). Included
  four-byte assignments at `[sp-20] +0x18`, `[sp-20] +0x1c` and `[sp-20] +0x20` write `0x5`, `0x5`
  and `0x8` respectively (`str` at 0x2d12c, `str` at 0x2d138, `str` at 0x2d144). The stored
  `0x5442c` value occupies `[sp-20] +0xf0` as four bytes (`str` at 0x2d120). Board-operation layout
  and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d034 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d038 | load | 4 | `abs 0x2d14c` | `-` |
| `str` at 0x2d03c | store | 4 | `sp -0x18` | `0x1432f4` |
| `ldr` at 0x2d040 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d044 | load | 4 | `abs 0x2d150` | `-` |
| `str` at 0x2d048 | store | 4 | `r0 +0x0` | `0x2ce08` |
| `ldr` at 0x2d04c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d050 | load | 4 | `abs 0x2d154` | `-` |
| `str` at 0x2d054 | store | 4 | `[sp-20] +0x4` | `0x2c9e4` |
| `ldr` at 0x2d058 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x2d060 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x2d064 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d06c | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x2d070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d078 | store | 4 | `[sp-20] +0xa0` | `0xff` |
| `ldr` at 0x2d07c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d084 | store | 4 | `[sp-20] +0xa4` | `0xff` |
| `ldr` at 0x2d088 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d090 | store | 4 | `[sp-20] +0xa8` | `0xff` |
| `ldr` at 0x2d094 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d09c | store | 4 | `[sp-20] +0xac` | `0xff` |
| `ldr` at 0x2d0a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d0a4 | load | 4 | `abs 0x2d158` | `-` |
| `str` at 0x2d0a8 | store | 4 | `[sp-20] +0x14` | `0x104328` |
| `ldr` at 0x2d0ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0b4 | store | 4 | `[sp-20] +0x88` | `0x0` |
| `ldr` at 0x2d0b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0c0 | store | 4 | `[sp-20] +0x90` | `0x0` |
| `ldr` at 0x2d0c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0cc | store | 4 | `[sp-20] +0x94` | `0x0` |
| `ldr` at 0x2d0d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0d8 | store | 4 | `[sp-20] +0x98` | `0x0` |
| `ldr` at 0x2d0dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d0e0 | load | 4 | `abs 0x2d15c` | `-` |
| `str` at 0x2d0e4 | store | 4 | `[sp-20] +0xc4` | `0x53f1c` |
| `ldr` at 0x2d0e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d0ec | load | 4 | `abs 0x2d160` | `-` |
| `str` at 0x2d0f0 | store | 4 | `[sp-20] +0xc8` | `0x54128` |
| `ldr` at 0x2d0f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d0fc | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x2d100 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d108 | store | 4 | `[sp-20] +0xd4` | `0x0` |
| `ldr` at 0x2d10c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d110 | load | 4 | `abs 0x2d164` | `-` |
| `str` at 0x2d114 | store | 4 | `[sp-20] +0xec` | `0x2c444` |
| `ldr` at 0x2d118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d11c | load | 4 | `abs 0x2d168` | `-` |
| `str` at 0x2d120 | store | 4 | `[sp-20] +0xf0` | `0x5442c` |
| `ldr` at 0x2d124 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d12c | store | 4 | `[sp-20] +0x18` | `0x5` |
| `ldr` at 0x2d130 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d138 | store | 4 | `[sp-20] +0x1c` | `0x5` |
| `ldr` at 0x2d13c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d144 | store | 4 | `[sp-20] +0x20` | `0x8` |
| `ldmdb` at 0x2d148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d148 | `r0` | `r0` |

## `IP63_SoftReset`

- `IP63_SoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38670 to 0x38698.
  Established. The unnamed `0x202d0` call has `0x10815c`, `r1`, `r2`, `r3` (`bl` at 0x38688). The
  reported return is unknown (`ldmdb` at 0x38690). This body does not establish a reset effect, so
  its role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38680 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x38684 | load | 4 | `abs 0x38694` | `-` |
| `ldmdb` at 0x38690 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38690 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38690 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38688 | `(unknown callee) 0x202d0` | `0x10815c` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38690 | `r0` | `?` |

## `promIceReset`

- `promIceReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44f7c to 0x450ac.
  Established. A byte read targets `[sp-20] +0x1` (`ldrb` at 0x44ffc). The unnamed `0xd73b0` call
  has `0x0`, `[[sp-20]]:1`, `[[sp-20]+1]:1`, unknown (`bl` at 0x4503c); its zero result takes the
  branch to `0x45088` (`beq` at 0x45050). The later byte stack assignment at `sp -0x19` writes
  `([sp-25]:1+1 & 0xff)` (`strb` at 0x45090). Reset selection and external effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44f90 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x44f94 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44f98 | load | 4 | `abs 0x4509c` | `-` |
| `ldr` at 0x44f9c | load | 4 | `abs 0x2b1480` | `-` |
| `ldrb` at 0x44fac | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44fc4 | load | 4 | `abs 0x450a0` | `-` |
| `ldrb` at 0x44fc8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44fd0 | load | 4 | `abs 0x2b14a0` | `-` |
| `str` at 0x44fd8 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x44fdc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x44ff8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x44ffc | load | 1 | `[sp-20] +0x1` | `-` |
| `strb` at 0x45000 | store | 1 | `sp -0x1b` | `([[sp-20]+1]:1 & 0xff)` |
| `strb` at 0x4500c | store | 1 | `sp -0x1b` | `0x0` |
| `ldrb` at 0x45010 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x45014 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x45018 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4501c | load | 1 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x45020 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x45024 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldrb` at 0x45028 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x45044 | store | 4 | `sp -0x18` | `ret(0xd73b0)` |
| `ldr` at 0x45048 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45054 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45060 | load | 4 | `abs 0x450a4` | `-` |
| `str` at 0x45064 | store | 4 | `sp -0x28` | `0x109c50` |
| `ldrb` at 0x45068 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x4506c | store | 4 | `sp -0x24` | `[sp-25]:1` |
| `str` at 0x45070 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x45078 | load | 4 | `abs 0x450a8` | `-` |
| `ldrb` at 0x45088 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x45090 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0x45098 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45098 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45098 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44fb4 | `(unknown callee) 0x45108` | `?` | `?` | `[0x2b1480]` | `?` |
| `bl` at 0x44fe8 | `(unknown callee) 0x1f26c` | `[sp-25]:1+108` | `0x2b14a0` | `?` | `[sp-25]:1+108` |
| `bl` at 0x4503c | `(unknown callee) 0xd73b0` | `0x0` | `[[sp-20]]:1` | `[[sp-20]+1]:1` | `?` |
| `bl` at 0x45058 | `(unknown callee) 0x21a38` | `ret(0xd73b0)` | `?` | `?` | `ret(0xd73b0)` |
| `bl` at 0x45084 | `(unknown callee) 0x21eb8` | `0x1` | `0x109c1c` | `0x68` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x44fa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x44fc0 | `ret(0x45108) == 0x0` | `ret(0x45108) != 0x0` |
| `bne` at 0x44ff4 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0x45050 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45098 | `r0` | `?` |

## `TIOInit`

- `TIOInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53ab4 to 0x53d08.
  Established. A four-byte absolute assignment at `0x1490d0` retains `[sp-24]` (`str` at 0x53c68).
  Byte assignments at absolute `0x2b1620` and `0x2b1618` write `([sp-28]:1 & 0xff)` and `([sp-32]:1 & 0xff)` respectively (`strb` at 0x53c74, `strb` at 0x53c80). The unnamed `0x58180` call carries
  `0x1`, `0x54404`, `0x56840`, `0x56920` (`bl` at 0x53b28). TIO registration and stored record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53ac4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x53ac8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x53acc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x53ad0 | load | 4 | `abs 0x53cd4` | `-` |
| `ldr` at 0x53ad4 | load | 4 | `abs 0x1490d0` | `-` |
| `ldr` at 0x53ae0 | load | 4 | `abs 0x53cd8` | `-` |
| `str` at 0x53ae4 | store | 4 | `sp -0x38` | `0x10bdb8` |
| `ldr` at 0x53ae8 | load | 4 | `abs 0x53cdc` | `-` |
| `ldrb` at 0x53aec | load | 1 | `abs 0x2b1620` | `-` |
| `str` at 0x53af0 | store | 4 | `sp -0x34` | `[0x2b1620]:1` |
| `ldr` at 0x53af8 | load | 4 | `abs 0x53ce0` | `-` |
| `ldr` at 0x53b08 | load | 4 | `abs 0x53ce4` | `-` |
| `str` at 0x53b0c | store | 4 | `sp -0x38` | `0x5482c` |
| `ldr` at 0x53b10 | load | 4 | `abs 0x53ce8` | `-` |
| `str` at 0x53b14 | store | 4 | `sp -0x34` | `0x54b64` |
| `ldr` at 0x53b1c | load | 4 | `abs 0x53cec` | `-` |
| `ldr` at 0x53b20 | load | 4 | `abs 0x53cf0` | `-` |
| `ldr` at 0x53b24 | load | 4 | `abs 0x53cf4` | `-` |
| `str` at 0x53b30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x53b34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53b38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53b48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53b64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53b78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53b80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x53b84 | load | 4 | `(unknown)` | `-` |
| `str` at 0x53b90 | store | 4 | `sp -0x28` | `ret(0xe9c50)` |
| `ldr` at 0x53b94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x53ba0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x53bac | load | 4 | `abs 0x53cf8` | `-` |
| `str` at 0x53bb0 | store | 4 | `sp -0x38` | `0x10bde8` |
| `ldr` at 0x53bb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53bc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53bd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x53bd4 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x53bd8 | store | 4 | `sp -0x30` | `ret(0x21704)` |
| `ldr` at 0x53be0 | load | 4 | `abs 0x53ce0` | `-` |
| `ldr` at 0x53bf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x53bf8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x53c00 | load | 4 | `abs 0x53cfc` | `-` |
| `ldr` at 0x53c04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53c18 | store | 4 | `abs 0x1490d8` | `ret(0x1b1b4)` |
| `ldr` at 0x53c1c | load | 4 | `abs 0x1490d8` | `-` |
| `ldr` at 0x53c28 | load | 4 | `abs 0x53d00` | `-` |
| `str` at 0x53c2c | store | 4 | `sp -0x38` | `0x10be04` |
| `ldr` at 0x53c34 | load | 4 | `abs 0x53ce0` | `-` |
| `ldr` at 0x53c44 | load | 4 | `abs 0x53cfc` | `-` |
| `ldr` at 0x53c48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53c50 | load | 4 | `abs 0x1490d8` | `-` |
| `ldr` at 0x53c60 | load | 4 | `abs 0x53cd4` | `-` |
| `ldr` at 0x53c64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53c68 | store | 4 | `abs 0x1490d0` | `[sp-24]` |
| `ldr` at 0x53c6c | load | 4 | `abs 0x53cdc` | `-` |
| `ldrb` at 0x53c70 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x53c74 | store | 1 | `abs 0x2b1620` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x53c78 | load | 4 | `abs 0x53d04` | `-` |
| `ldrb` at 0x53c7c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x53c80 | store | 1 | `abs 0x2b1618` | `([sp-32]:1 & 0xff)` |
| `str` at 0x53c88 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x53c8c | load | 4 | `abs 0x53cdc` | `-` |
| `ldrb` at 0x53c90 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x53c94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53ca4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53cac | load | 4 | `abs 0x53cfc` | `-` |
| `ldr` at 0x53cb0 | load | 4 | `abs 0x1490d8` | `-` |
| `strb` at 0x53cbc | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x53cc0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x53cc8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x53cd0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x53cd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53cd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53cd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53b04 | `(unknown callee) 0x21eb8` | `0x9` | `0x10bdd4` | `0xd5` | `0x0` |
| `bl` at 0x53b28 | `(unknown callee) 0x58180` | `0x1` | `0x54404` | `0x56840` | `0x56920` |
| `bl` at 0x53b88 | `(unknown callee) 0xe9c50` | `?` | `?` | `?` | `?` |
| `bl` at 0x53ba4 | `(unknown callee) 0x21704` | `ret(0xe9c50)` | `?` | `?` | `ret(0xe9c50)` |
| `bl` at 0x53bec | `(unknown callee) 0x21eb8` | `0x9` | `0x10bdd4` | `0xe2` | `0x0` |
| `bl` at 0x53c10 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x53c40 | `(unknown callee) 0x21eb8` | `0x9` | `0x10bdd4` | `0xea` | `0x0` |
| `bl` at 0x53c5c | `(unknown callee) 0x1b4cc` | `[0x1490d8]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53adc | `[0x1490d0] == 0x0` | `[0x1490d0] != 0x0` |
| `blt` at 0x53b40 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x53b9c | `ret(0xe9c50) == 0x0` | `ret(0xe9c50) != 0x0` |
| `bne` at 0x53c24 | `[0x1490d8] != 0x0` | `[0x1490d8] == 0x0` |
| `blt` at 0x53c9c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53cd0 | `r0` | `ret(0x1b4cc)` |

## `TIOTarget`

- `TIOTarget`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53ed4 to 0x53f1c.
  Established. A four-byte absolute read targets `0x1490d0` (`ldr` at 0x53f04). The following
  four-byte read has an unknown effective target (`ldr` at 0x53f0c). Target indexing and the
  reported unknown return remain Unknown (`ldmdb` at 0x53f14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x53ee8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x53eec | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x53f00 | load | 4 | `abs 0x53f18` | `-` |
| `ldr` at 0x53f04 | load | 4 | `abs 0x1490d0` | `-` |
| `ldr` at 0x53f0c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x53f14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53f14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53f14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53f14 | `r0` | `?` |

## `TIOActive`

- `TIOActive`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54404 to 0x5442c.
  Established. A byte absolute read targets `0x1490d4` (`ldrb` at 0x5441c). The reported result is
  `[0x1490d4]:1` (`ldmdb` at 0x54424). Active-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54414 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x54418 | load | 4 | `abs 0x54428` | `-` |
| `ldrb` at 0x5441c | load | 1 | `abs 0x1490d4` | `-` |
| `ldmdb` at 0x54424 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x54424 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x54424 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x54424 | `r0` | `[0x1490d4]:1` |

## `TIOSetLinkEnable`

- `TIOSetLinkEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5442c to 0x544b0.
  Established. The branch takes `[0x1490d4]:1 != 0x0` to `0x54484` and falls through for equality
  (`bne` at 0x54474). The unnamed `0x544b0` call carries `r0`, `r1`, `r2`, `[0x1490d4]:1` (`bl` at
  0x54490). Link selection and enable semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5443c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x54440 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x54444 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x54448 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54460 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x54468 | load | 4 | `abs 0x544ac` | `-` |
| `ldrb` at 0x5446c | load | 1 | `abs 0x1490d4` | `-` |
| `str` at 0x5447c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x54484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5448c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54498 | store | 4 | `sp -0x20` | `ret(0x544b0)` |
| `ldr` at 0x5449c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x544a0 | store | 4 | `sp -0x24` | `ret(0x544b0)` |
| `ldr` at 0x544a4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x544a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x544a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x544a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54490 | `(unknown callee) 0x544b0` | `r0` | `r1` | `r2` | `[0x1490d4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54458 | `? == 0x4` | `? != 0x4` |
| `bne` at 0x54474 | `[0x1490d4]:1 != 0x0` | `[0x1490d4]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x544a8 | `r0` | `?` |

## `tio_set_link_enable`

- `tio_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x544b0 to 0x54598.
  Established. A four-byte stack assignment at `sp -0x20` writes `0x90060780` (`str` at 0x54550).
  The unnamed `0xeaf08` call carries `[sp-20]`, `[sp-32]`, `0x0`, `0x1` (`bl` at 0x54568). The
  earlier branch takes `[0x2adfd8] == 0x0` to `0x544f0` (`beq` at 0x544d8). Link access mechanism
  and address interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x544c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x544c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x544c8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x544cc | load | 4 | `abs 0x54580` | `-` |
| `ldr` at 0x544d0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x544dc | load | 4 | `abs 0x54584` | `-` |
| `ldr` at 0x544e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x544e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x544e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x544f0 | load | 4 | `abs 0x54588` | `-` |
| `ldr` at 0x544f4 | load | 4 | `abs 0x14928c` | `-` |
| `ldr` at 0x54500 | load | 4 | `abs 0x54584` | `-` |
| `str` at 0x54504 | store | 4 | `sp -0x34` | `0x10be64` |
| `ldr` at 0x54508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5450c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x54510 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54514 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x54518 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5451c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x54524 | load | 4 | `abs 0x5458c` | `-` |
| `ldr` at 0x54528 | load | 4 | `abs 0x54590` | `-` |
| `ldr` at 0x54534 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54544 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5454c | load | 4 | `abs 0x54594` | `-` |
| `str` at 0x54550 | store | 4 | `sp -0x20` | `0x90060780` |
| `ldr` at 0x54558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5455c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54574 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x54578 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5457c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5457c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5457c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x544ec | `(unknown callee) 0x23fbc` | `0x10be64` | `r0` | `r1` | `r2` |
| `bl` at 0x54530 | `(unknown callee) 0x21eb8` | `0x7` | `0x10bdd4` | `0x216` | `0x0` |
| `bl` at 0x54554 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `0x90060780` |
| `bl` at 0x54568 | `(unknown callee) 0xeaf08` | `[sp-20]` | `[sp-32]` | `0x0` | `0x1` |
| `bl` at 0x5456c | `(unknown callee) 0xe3d64` | `ret(0xeaf08)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x544d8 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x544fc | `[0x14928c] == 0x0` | `[0x14928c] != 0x0` |
| `bne` at 0x5453c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5457c | `r0` | `0x0` |

## `TIOPciResetInt`

- `TIOPciResetInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5469c to 0x547b4.
  Established. A byte absolute read targets `0x2b1620` (`ldrb` at 0x546d8). Included later byte
  reads have unknown effective targets (`ldrb` at 0x5474c, `ldrb` at 0x54770, `ldrb` at 0x54790).
  Two calls to unnamed `0x56a70` have four unknown candidates (`bl` at 0x54778, `bl` at 0x54798).
  PCI reset-interrupt mapping and call effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x546ac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x546b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x546c0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x546c8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x546d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x546d4 | load | 4 | `abs 0x547a0` | `-` |
| `ldrb` at 0x546d8 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x546dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x546e8 | load | 4 | `abs 0x547a4` | `-` |
| `str` at 0x546ec | store | 4 | `sp -0x28` | `0x10bec0` |
| `ldrb` at 0x546f0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x546f4 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x546fc | load | 4 | `abs 0x547a8` | `-` |
| `ldr` at 0x54700 | load | 4 | `abs 0x547ac` | `-` |
| `ldr` at 0x54710 | load | 4 | `abs 0x547b0` | `-` |
| `ldr` at 0x54718 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54734 | load | 4 | `abs 0x1490d0` | `-` |
| `str` at 0x5473c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x54744 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x54748 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5474c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x54750 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5475c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5476c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x54770 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5477c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5478c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x54790 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x5479c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5479c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5479c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54708 | `(unknown callee) 0x21eb8` | `0x1` | `0x10bdd4` | `0x265` | `0x0` |
| `bl` at 0x54778 | `(unknown callee) 0x56a70` | `?` | `?` | `?` | `?` |
| `bl` at 0x54798 | `(unknown callee) 0x56a70` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x546e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x54758 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x54768 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x54788 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5479c | `r0` | `?` |

## `TIOPresent`

- `TIOPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x547b4 to 0x547f0.
  Established. A byte store with unknown effective target writes `0x1` (`strb` at 0x547e4). The
  earlier four-byte absolute read targets `0x1490d8` (`ldr` at 0x547d8). The reported return
  preserves entry `r0` (`ldmdb` at 0x547e8). Presence record layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x547c8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x547cc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x547d4 | load | 4 | `abs 0x547ec` | `-` |
| `ldr` at 0x547d8 | load | 4 | `abs 0x1490d8` | `-` |
| `strb` at 0x547e4 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x547e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x547e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x547e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x547e8 | `r0` | `r0` |

## `TIONotPresent`

- `TIONotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x547f0 to 0x5482c.
  Established. The byte assignment has an unknown effective target and writes `0x0` (`strb` at
  0x54820). A four-byte absolute read targets `0x1490d8` (`ldr` at 0x54814). The reported result
  preserves entry `r0` (`ldmdb` at 0x54824). Absence record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54804 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x54808 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x54810 | load | 4 | `abs 0x54828` | `-` |
| `ldr` at 0x54814 | load | 4 | `abs 0x1490d8` | `-` |
| `strb` at 0x54820 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x54824 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x54824 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x54824 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x54824 | `r0` | `r0` |

## `TIO_Issue_CX_Probe`

- `TIO_Issue_CX_Probe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5566c to 0x55d7c.
  Established. The unnamed `0xeae04` call carries unknown, `0x90060e80`, `sp-44`, `sp-48` (`bl` at
  0x55744). Included unnamed `0xeaf08` calls have `[sp-20]`, `0xb000b100`, `0x0`, `0x8` and
  `[sp-20]`, `0xb000b100`, `0x0`, `0x0` respectively (`bl` at 0x55a88, `bl` at 0x55abc). The earlier
  branch takes `(r1 & 0xff) != 0x1` to `0x5572c` and falls through for equality (`bne` at 0x55698).
  Probe target addresses and external access semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5567c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x55684 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x5568c | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x55690 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5569c | load | 4 | `abs 0x55d50` | `-` |
| `ldr` at 0x556a0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x556ac | load | 4 | `abs 0x55d54` | `-` |
| `ldr` at 0x556b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x556c8 | store | 4 | `sp -0x20` | `ret(0xecd30)` |
| `ldr` at 0x556cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x556d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x556dc | store | 4 | `sp -0x6c` | `ret(0xecd30)` |
| `ldr` at 0x556e4 | load | 4 | `abs 0x55d50` | `-` |
| `ldr` at 0x556e8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x556f4 | load | 4 | `abs 0x55d58` | `-` |
| `ldr` at 0x556fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55710 | store | 4 | `sp -0x20` | `ret(0xecd30)` |
| `ldr` at 0x55714 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55720 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55724 | store | 4 | `sp -0x6c` | `ret(0xecd30)` |
| `ldr` at 0x55734 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55738 | load | 4 | `abs 0x55d5c` | `-` |
| `str` at 0x5574c | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x55750 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5575c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55760 | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `ldr` at 0x55768 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x55774 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x55778 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x55784 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x55790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55794 | load | 4 | `abs 0x55d60` | `-` |
| `str` at 0x557a8 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x557ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x557b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x557bc | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `ldr` at 0x557cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x557d0 | load | 4 | `abs 0x55d64` | `-` |
| `str` at 0x557e4 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x557e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x557f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x557f8 | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `ldr` at 0x55808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5580c | load | 4 | `abs 0x55d68` | `-` |
| `str` at 0x55820 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x55824 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55830 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55834 | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `ldr` at 0x5583c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x55844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55848 | load | 4 | `abs 0x55d60` | `-` |
| `ldr` at 0x55850 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x5585c | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55860 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5586c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55870 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5587c | load | 4 | `abs 0x55d68` | `-` |
| `str` at 0x55890 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55894 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x558a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x558a4 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x558ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x558b0 | load | 4 | `abs 0x55d64` | `-` |
| `str` at 0x558c4 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x558c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x558d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x558d8 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `str` at 0x558e4 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x558e8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x558fc | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x55904 | store | 4 | `sp -0x78` | `0x30000000` |
| `str` at 0x5590c | store | 4 | `sp -0x74` | `sp-44` |
| `str` at 0x55914 | store | 4 | `sp -0x70` | `sp-48` |
| `ldr` at 0x55918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5591c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x55920 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x55930 | store | 4 | `sp -0x1c` | `ret(0x642c8)` |
| `ldr` at 0x55934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x55940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55944 | load | 4 | `abs 0x55d60` | `-` |
| `ldr` at 0x55948 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5594c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x55958 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x5595c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55968 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5596c | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55978 | load | 4 | `abs 0x55d64` | `-` |
| `ldr` at 0x5597c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55980 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x5598c | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55990 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5599c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x559a0 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x559a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x559ac | load | 4 | `abs 0x55d68` | `-` |
| `ldr` at 0x559b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x559b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x559c0 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x559c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x559d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x559d4 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `str` at 0x559e0 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x559e8 | load | 4 | `abs 0x55d50` | `-` |
| `ldr` at 0x559ec | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x559f8 | load | 4 | `abs 0x55d54` | `-` |
| `ldr` at 0x55a00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55a14 | store | 4 | `sp -0x20` | `ret(0xecd30)` |
| `ldr` at 0x55a18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55a24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55a28 | store | 4 | `sp -0x6c` | `ret(0xecd30)` |
| `ldr` at 0x55a30 | load | 4 | `abs 0x55d50` | `-` |
| `ldr` at 0x55a34 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x55a40 | load | 4 | `abs 0x55d58` | `-` |
| `ldr` at 0x55a48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55a5c | store | 4 | `sp -0x20` | `ret(0xecd30)` |
| `ldr` at 0x55a60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55a6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55a70 | store | 4 | `sp -0x6c` | `ret(0xecd30)` |
| `ldr` at 0x55a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55a7c | load | 4 | `abs 0x55d6c` | `-` |
| `str` at 0x55a90 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55a94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55aa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55aa4 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55aac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55ab0 | load | 4 | `abs 0x55d6c` | `-` |
| `str` at 0x55ac4 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55ac8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55ad4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55ad8 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55aec | load | 4 | `abs 0x55d70` | `-` |
| `str` at 0x55b00 | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x55b04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55b10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55b14 | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `str` at 0x55b20 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x55b24 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55b38 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55b40 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x55b50 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55b68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55b7c | store | 4 | `sp -0x20` | `ret(0xeae04)` |
| `ldr` at 0x55b80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55b8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55b90 | store | 4 | `sp -0x6c` | `ret(0xeae04)` |
| `ldr` at 0x55b98 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x55ba0 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x55ba4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55bb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55bbc | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x55bc0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x55bcc | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55bd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55bdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55be0 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55be8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x55bf0 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x55bf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55c10 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55c14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55c20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55c24 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55c2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55c30 | load | 4 | `abs 0x55d74` | `-` |
| `str` at 0x55c44 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55c48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55c54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55c58 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55c60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55c64 | load | 4 | `abs 0x55d78` | `-` |
| `str` at 0x55c78 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55c7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55c88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55c8c | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55c94 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x55c9c | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0x55ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55ca8 | load | 4 | `abs 0x55d60` | `-` |
| `ldr` at 0x55cac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x55cb0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x55cbc | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55cc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55ccc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55cd0 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55cd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55cdc | load | 4 | `abs 0x55d64` | `-` |
| `ldr` at 0x55ce0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x55ce4 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x55cf0 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55cf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55d00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55d04 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55d0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55d10 | load | 4 | `abs 0x55d68` | `-` |
| `ldr` at 0x55d14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x55d18 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x55d24 | store | 4 | `sp -0x20` | `ret(0xeaf08)` |
| `ldr` at 0x55d28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55d34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55d38 | store | 4 | `sp -0x6c` | `ret(0xeaf08)` |
| `ldr` at 0x55d40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55d44 | store | 4 | `sp -0x6c` | `[sp-28]` |
| `ldr` at 0x55d48 | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0x55d4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x55d4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x55d4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x556b0 | `(unknown callee) 0x23fbc` | `0x10c000` | `r1` | `r2` | `[0x2adfd8]` |
| `bl` at 0x556c0 | `(unknown callee) 0xecd30` | `?` | `0x0` | `0x1` | `?` |
| `bl` at 0x556f8 | `(unknown callee) 0x23fbc` | `0x10c020` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x55708 | `(unknown callee) 0xecd30` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x55744 | `(unknown callee) 0xeae04` | `?` | `0x90060e80` | `sp-44` | `sp-48` |
| `bl` at 0x557a0 | `(unknown callee) 0xeae04` | `[sp-20]` | `0xb0000028` | `sp-52` | `sp-56` |
| `bl` at 0x557dc | `(unknown callee) 0xeae04` | `[sp-20]` | `0xb0000100` | `sp-68` | `sp-72` |
| `bl` at 0x55818 | `(unknown callee) 0xeae04` | `[sp-20]` | `0xb0000108` | `sp-60` | `sp-64` |
| `bl` at 0x55854 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000028` | `?` | `[sp-56]` |
| `bl` at 0x55888 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000108` | `0x0` | `0x0` |
| `bl` at 0x558bc | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000100` | `0x0` | `0xc8` |
| `bl` at 0x55928 | `(unknown callee) 0x642c8` | `[sp-20]` | `[sp-36]` | `[sp-40]` | `0xff` |
| `bl` at 0x55950 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x55984 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x559b8 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x559fc | `(unknown callee) 0x23fbc` | `0x10c000` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x55a0c | `(unknown callee) 0xecd30` | `[sp-20]` | `0x0` | `0x1` | `?` |
| `bl` at 0x55a44 | `(unknown callee) 0x23fbc` | `0x10c020` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x55a54 | `(unknown callee) 0xecd30` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x55a88 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb000b100` | `0x0` | `0x8` |
| `bl` at 0x55abc | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb000b100` | `0x0` | `0x0` |
| `bl` at 0x55af8 | `(unknown callee) 0xeae04` | `[sp-20]` | `0xb0008118` | `sp-84` | `sp-88` |
| `bl` at 0x55b74 | `(unknown callee) 0xeae04` | `[sp-20]` | `?` | `sp-92` | `sp-96` |
| `bl` at 0x55bc4 | `(unknown callee) 0xeaf08` | `[sp-20]` | `?` | `[sp-92]` | `[sp-96]` |
| `bl` at 0x55c08 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000000` | `0x0` | `0x0` |
| `bl` at 0x55c3c | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0008000` | `0x0` | `0x0` |
| `bl` at 0x55c70 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0008060` | `0x0` | `0x0` |
| `bl` at 0x55cb4 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x55ce8 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x55d1c | `(unknown callee) 0xeaf08` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x55698 | `(r1 & 0xff) != 0x1` | `(r1 & 0xff) == 0x1` |
| `bls` at 0x556a8 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x556d4 | `ret(0xecd30) >= 0x0 (signed, 32-bit)` | `ret(0xecd30) < 0x0 (signed, 32-bit)` |
| `bls` at 0x556f0 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x5571c | `ret(0xecd30) == 0x0` | `ret(0xecd30) != 0x0` |
| `beq` at 0x55758 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x557b4 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x557f0 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x5582c | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x55868 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x5589c | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x558d0 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `bls` at 0x558f0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x5593c | `ret(0x642c8) != 0x0` | `ret(0x642c8) == 0x0` |
| `beq` at 0x55964 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55998 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x559cc | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `bls` at 0x559f4 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x55a20 | `ret(0xecd30) >= 0x0 (signed, 32-bit)` | `ret(0xecd30) < 0x0 (signed, 32-bit)` |
| `bls` at 0x55a3c | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x55a68 | `ret(0xecd30) == 0x0` | `ret(0xecd30) != 0x0` |
| `beq` at 0x55a9c | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55ad0 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55b0c | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `bls` at 0x55b2c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x55b4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x55b88 | `ret(0xeae04) == 0x0` | `ret(0xeae04) != 0x0` |
| `beq` at 0x55bd8 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55c1c | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55c50 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55c84 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55cc8 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55cfc | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |
| `beq` at 0x55d30 | `ret(0xeaf08) == 0x0` | `ret(0xeaf08) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x55d4c | `r0` | `[sp-108]` |
