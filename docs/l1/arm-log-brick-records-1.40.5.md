# ARM logs and brick identity records in 1.40.5

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
`IRFmtAddr`, `UtilFmtXTalk`, `UtilDspBrick`, `brickConvertType`, `brickConvertString` and
`brickTypeToChar` have unresolved computed transfers. Their access, call and return lists cover
known edges only. Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/log-brick-records-facts.txt) retain all tool sections.

## `ramLogInit`

- `ramLogInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba0a4 to 0xba184.
  Established. The unnamed `0x1b1b4` call at `0xba0c0` carries `0x18`, entry `r1`, `r2`, `r3` (`bl` at 0xba0c0); its nonzero result takes `0xba0e4`, while zero falls through (`bne` at 0xba0d4). The
  word at `[sp-32] +0xc` receives `([[sp-32]+4] + [sp-24])` (`str` at 0xba150). RAM log record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba0b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xba0b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xba0c8 | store | 4 | `sp -0x20` | `ret(0x1b1b4)` |
| `ldr` at 0xba0cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba0dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xba0e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba0e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba0f4 | store | 4 | `ret(0x1b1b4) +0x4` | `ret(0x1b1b4)` |
| `ldr` at 0xba0f8 | load | 4 | `ret(0x1b1b4) +0x4` | `-` |
| `ldr` at 0xba104 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba110 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xba118 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xba120 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba124 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba128 | store | 4 | `[sp-32] +0x0` | `[sp-28]` |
| `ldr` at 0xba12c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba134 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xba138 | store | 4 | `[sp-32] +0x8` | `[[sp-32]+4]` |
| `ldr` at 0xba13c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba144 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xba148 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba150 | store | 4 | `[sp-32] +0xc` | `([[sp-32]+4] + [sp-24])` |
| `ldr` at 0xba154 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba158 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba15c | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xba160 | store | 4 | `[sp-32] +0x10` | `[[sp-32]+4]` |
| `ldr` at 0xba164 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba168 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba16c | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xba170 | store | 4 | `[sp-32] +0x14` | `[[sp-32]+4]` |
| `ldr` at 0xba174 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba178 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xba17c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xba180 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xba180 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba180 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba0c0 | `(unknown callee) 0x1b1b4` | `0x18` | `r1` | `r2` | `r3` |
| `bl` at 0xba0ec | `(unknown callee) 0x1b1b4` | `[sp-24]` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xba108 | `(unknown callee) 0x1b490` | `[sp-32]` | `?` | `?` | `[ret(0x1b1b4)+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba0d4 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xba100 | `[ret(0x1b1b4)+4] != 0x0` | `[ret(0x1b1b4)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba180 | `r0` | `?` |

## `ramLogReset`

- `ramLogReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba184 to 0xba1cc.
  Established. Entry `r0 != 0x0` takes `0xba1a8`, while zero falls through (`bne` at 0xba1a0). The
  word at `r0 +0x10` receives `[r0+8]` (`str` at 0xba1b4). The word at `[sp-20] +0x14` receives
  `[[sp-20]+8]` (`str` at 0xba1c4). Log reset semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba194 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xba198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba1a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba1ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba1b0 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xba1b4 | store | 4 | `r0 +0x10` | `[r0+8]` |
| `ldr` at 0xba1b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba1bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba1c0 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba1c4 | store | 4 | `[sp-20] +0x14` | `[[sp-20]+8]` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba1a0 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba1c8 | `r0` | `r0` |

## `ramLogInsert`

- `ramLogInsert`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba1cc to 0xba658.
  Established. The unnamed `0x1b274` call carries `0x0`, entry `r1`, `r2`, `r3` (`bl` at 0xba1ec).
  The byte assignment through `[sp-60] +0x0` writes `0xa` on an included path (`strb` at 0xba32c).
  The word at `[sp-20] +0x10` receives `([[sp-20]+16] + [sp-44])` (`str` at 0xba450). Log insertion
  policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba1dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba1e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xba1e4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xba1f4 | store | 4 | `sp -0x24` | `ret(0x1b274)` |
| `str` at 0xba1fc | store | 4 | `sp -0x28` | `sp-36` |
| `str` at 0xba204 | store | 4 | `sp -0x2c` | `0x5` |
| `ldr` at 0xba208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba20c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba210 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba214 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba21c | store | 4 | `sp -0x30` | `?` |
| `str` at 0xba224 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xba228 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba23c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xba248 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba260 | store | 4 | `sp -0x1c` | `ret(0x1b2bc)` |
| `ldr` at 0xba264 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba278 | store | 4 | `sp -0x34` | `ret(0x1b1b4)` |
| `ldr` at 0xba27c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xba280 | store | 4 | `sp -0x3c` | `ret(0x1b1b4)` |
| `ldr` at 0xba284 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba288 | store | 4 | `sp -0x38` | `[sp-24]` |
| `ldr` at 0xba28c | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xba290 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xba29c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba2b8 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xba2bc | load | 1 | `[sp-56] +0x0` | `-` |
| `ldr` at 0xba2c4 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xba2cc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xba2e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba2e4 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xba2ec | load | 1 | `[sp-56] +0x0` | `-` |
| `strb` at 0xba2f0 | store | 1 | `[sp-60] +0x0` | `([[sp-56]]:1 & 0xff)` |
| `str` at 0xba2f8 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xba2fc | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xba304 | store | 1 | `[sp-60] +0x1` | `0x0` |
| `ldr` at 0xba30c | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xba310 | load | 1 | `[sp-56] +0x0` | `-` |
| `ldr` at 0xba320 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xba32c | store | 1 | `[sp-60] +0x0` | `0xa` |
| `str` at 0xba334 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xba338 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xba340 | store | 1 | `[sp-60] +0x1` | `0x0` |
| `ldr` at 0xba344 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xba34c | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xba350 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba358 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xba360 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xba364 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xba368 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xba374 | store | 4 | `sp -0x1c` | `ret(0x1b2bc)` |
| `ldr` at 0xba37c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xba380 | store | 4 | `sp -0x3c` | `[sp-24]` |
| `ldr` at 0xba384 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba38c | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba390 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba39c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3a4 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba3a8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xba3b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3b8 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba3bc | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0xba3c8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba3d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3d8 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba3dc | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba3e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba3f0 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba3f4 | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0xba3fc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba400 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba404 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xba410 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba414 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xba41c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xba42c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xba430 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xba438 | store | 4 | `sp -0x30` | `([sp-48] + [sp-44])` |
| `ldr` at 0xba43c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba444 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba448 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xba450 | store | 4 | `[sp-20] +0x10` | `([[sp-20]+16] + [sp-44])` |
| `ldr` at 0xba454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba458 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldrb` at 0xba45c | load | 1 | `[[sp-20]+16] +0x0` | `-` |
| `ldr` at 0xba46c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xba474 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xba478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba47c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba480 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xba488 | store | 4 | `[sp-20] +0x10` | `[[sp-20]+16]+1` |
| `ldr` at 0xba48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba490 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba494 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba498 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba4a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba4a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba4ac | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba4b0 | store | 4 | `[sp-20] +0x10` | `[[sp-20]+8]` |
| `ldr` at 0xba4bc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xba4c4 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xba4d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xba4e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba4e8 | load | 4 | `[sp-20] +0x14` | `-` |
| `strb` at 0xba4f4 | store | 1 | `[[sp-20]+20] +0x0` | `?` |
| `str` at 0xba4fc | store | 4 | `[sp-20] +0x14` | `[[sp-20]+20]+1` |
| `ldr` at 0xba504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba50c | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba514 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xba520 | load | 1 | `[sp-40] +0x0` | `-` |
| `strb` at 0xba524 | store | 1 | `[[sp-20]+20] +0x0` | `([[sp-40]]:1 & 0xff)` |
| `str` at 0xba52c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0xba534 | store | 4 | `[sp-20] +0x14` | `[[sp-20]+20]+1` |
| `ldr` at 0xba538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba53c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba540 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba544 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba558 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba55c | store | 4 | `[sp-20] +0x14` | `[[sp-20]+8]` |
| `ldr` at 0xba564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xba578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba57c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba580 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xba588 | store | 4 | `[sp-20] +0x10` | `[[sp-20]+16]+1` |
| `ldr` at 0xba58c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xba594 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xba598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba59c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba5a0 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xba5a4 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba5b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba5b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba5b8 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba5bc | store | 4 | `[sp-20] +0x10` | `[[sp-20]+8]` |
| `ldr` at 0xba5c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba5d0 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldr` at 0xba5e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba5e8 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba5f0 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0xba5fc | load | 1 | `[sp-60] +0x0` | `-` |
| `strb` at 0xba600 | store | 1 | `[[sp-20]+20] +0x0` | `([[sp-60]]:1 & 0xff)` |
| `str` at 0xba608 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `str` at 0xba610 | store | 4 | `[sp-20] +0x14` | `[[sp-20]+20]+1` |
| `ldr` at 0xba614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba61c | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xba620 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xba62c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba634 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xba638 | store | 4 | `[sp-20] +0x14` | `[[sp-20]+8]` |
| `ldr` at 0xba640 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xba64c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xba654 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba654 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba654 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba1ec | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xba258 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `[[sp-20]+12]` | `[sp-20]` | `[sp-28]` |
| `bl` at 0xba270 | `(unknown callee) 0x1b1b4` | `[sp-28]+1` | `?` | `?` | `[sp-28]+1` |
| `bl` at 0xba2ac | `(unknown callee) 0x1b55c` | `?` | `?` | `?` | `[sp-28]` |
| `bl` at 0xba36c | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xba650 | `(unknown callee) 0x1b490` | `[sp-52]` | `?` | `?` | `[sp-52]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba230 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xba244 | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |
| `bne` at 0xba250 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba298 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xba2a4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba2d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xba318 | `[[sp-56]]:1 != 0xd` | `[[sp-56]]:1 == 0xd` |
| `bls` at 0xba398 | `[[sp-20]+20] <= [[sp-20]+16] (unsigned, 32-bit)` | `[[sp-20]+20] > [[sp-20]+16] (unsigned, 32-bit)` |
| `bhs` at 0xba3e4 | `[[sp-20]+20] >= [[sp-20]+16] (unsigned, 32-bit)` | `[[sp-20]+20] < [[sp-20]+16] (unsigned, 32-bit)` |
| `beq` at 0xba40c | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |
| `bge` at 0xba424 | `([sp-28] + [sp-44]) >= [sp-48] (signed, 32-bit)` | `([sp-28] + [sp-44]) < [sp-48] (signed, 32-bit)` |
| `bne` at 0xba464 | `[[[sp-20]+16]]:1 != 0xf0` | `[[[sp-20]+16]]:1 == 0xf0` |
| `blo` at 0xba4a0 | `[[sp-20]+16] < [[sp-20]+12] (unsigned, 32-bit)` | `[[sp-20]+16] >= [[sp-20]+12] (unsigned, 32-bit)` |
| `bne` at 0xba4cc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xba4dc | `[sp-44] != 0x4` | `[sp-44] == 0x4` |
| `blo` at 0xba54c | `[[sp-20]+20] < [[sp-20]+12] (unsigned, 32-bit)` | `[[sp-20]+20] >= [[sp-20]+12] (unsigned, 32-bit)` |
| `bge` at 0xba570 | `[sp-28] >= [sp-48] (signed, 32-bit)` | `[sp-28] < [sp-48] (signed, 32-bit)` |
| `blo` at 0xba5ac | `[[sp-20]+16] < [[sp-20]+12] (unsigned, 32-bit)` | `[[sp-20]+16] >= [[sp-20]+12] (unsigned, 32-bit)` |
| `bne` at 0xba5d8 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xba628 | `[[sp-20]+20] < [[sp-20]+12] (unsigned, 32-bit)` | `[[sp-20]+20] >= [[sp-20]+12] (unsigned, 32-bit)` |
| `beq` at 0xba648 | `[sp-52] == 0x0` | `[sp-52] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba654 | `r0` | `?` |

## `ramLogFetch`

- `ramLogFetch`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba658 to 0xbac2c.
  Established. Entry `r0 != 0x0` takes `0xba6ac`, while zero falls through (`bne` at 0xba680). The
  unnamed `0x1b1b4` call at `0xba688` carries `0x50`, entry `r1`, entry `r2`, entry `r0` (`bl` at 0xba688). Included byte assignments at unknown targets write `0xa` and `0x0` (`strb` at 0xbabe0,
  `strb` at 0xbabfc). Log fetch encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba668 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba66c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xba674 | store | 4 | `sp -0x4c` | `0x1` |
| `ldr` at 0xba678 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xba690 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xba694 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba698 | load | 4 | `abs 0xbac10` | `-` |
| `ldr` at 0xba6a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba6a4 | store | 4 | `sp -0x50` | `[sp-28]` |
| `ldr` at 0xba6ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6b4 | load | 4 | `r0 +0x14` | `-` |
| `ldr` at 0xba6b8 | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xba6c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6c8 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xba6cc | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xba6d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6d4 | load | 4 | `r0 +0x14` | `-` |
| `str` at 0xba6d8 | store | 4 | `sp -0x24` | `[r0+20]` |
| `str` at 0xba6e0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xba6e8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xba6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba6f8 | load | 4 | `r0 +0x14` | `-` |
| `ldr` at 0xba6fc | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xba708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba70c | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xba710 | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xba714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba718 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xba71c | store | 4 | `sp -0x24` | `[r0+12]` |
| `ldr` at 0xba720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba724 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xba728 | store | 4 | `sp -0x28` | `[r0+8]` |
| `ldr` at 0xba72c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba730 | load | 4 | `r0 +0x14` | `-` |
| `str` at 0xba734 | store | 4 | `sp -0x2c` | `[r0+20]` |
| `str` at 0xba748 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xba74c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba75c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xba764 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba768 | load | 4 | `abs 0xbac14` | `-` |
| `ldr` at 0xba770 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba774 | store | 4 | `sp -0x50` | `[sp-28]` |
| `ldr` at 0xba77c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba780 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xba790 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xba794 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba798 | store | 4 | `sp -0x34` | `[r0+16]` |
| `ldr` at 0xba79c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xba7a0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xba7a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xba7b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xba7b8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba7bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xba7c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xba7d0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba7dc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xba7e4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xba7ec | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xba7f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xba7fc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba804 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0xba80c | store | 4 | `sp -0x3c` | `0x4` |
| `ldr` at 0xba814 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba81c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xba820 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xba828 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba830 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xba834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xba844 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xba848 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xba84c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xba850 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xba854 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xba858 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xba85c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xba860 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xba868 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xba874 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba87c | store | 4 | `sp -0x54` | `?` |
| `str` at 0xba888 | store | 4 | `sp -0x54` | `0x64` |
| `ldr` at 0xba88c | load | 4 | `sp -0x54` | `-` |
| `str` at 0xba890 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xba894 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xba8a0 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xba8a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba8b4 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xba8c0 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xba8c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xba8c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba8d0 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba8dc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xba8e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xba8e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xba8ec | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba8f8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xba8fc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xba90c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xba928 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xba92c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xba938 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xba93c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xba94c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xba958 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xba960 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xba964 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba968 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0xba96c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xba970 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xba974 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xba984 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xba988 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba98c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xba990 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xba9a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba9a4 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xba9b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xba9c0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xba9c4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xba9cc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xba9d0 | store | 1 | `[sp-68] +0x0` | `?` |
| `str` at 0xba9d8 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0xba9dc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xba9e4 | store | 4 | `sp -0x3c` | `[sp-60]-1` |
| `ldr` at 0xba9f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaa0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbaa10 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbaa20 | load | 4 | `abs 0xbac18` | `-` |
| `ldr` at 0xbaa2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbaa30 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbaa48 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbaa50 | store | 4 | `sp -0x4c` | `([sp-76] + ret(0x1b2bc))` |
| `ldr` at 0xbaa58 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbaa5c | load | 1 | `(unknown)` | `-` |
| `str` at 0xbaa6c | store | 4 | `sp -0x44` | `sp-64` |
| `str` at 0xbaa74 | store | 4 | `sp -0x3c` | `0x4` |
| `ldr` at 0xbaa80 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbaa88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbaa90 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbaa94 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbaa98 | store | 1 | `(unknown)` | `?` |
| `str` at 0xbaaa0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xbaab4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbaab8 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xbaac0 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0xbaac8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xbaad8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbaadc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbaae4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbaae8 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xbaaf0 | load | 4 | `abs 0xbac1c` | `-` |
| `ldr` at 0xbab00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbab04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbab0c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xbab10 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xbab18 | load | 4 | `abs 0xbac20` | `-` |
| `ldr` at 0xbab24 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbab2c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xbab30 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbab40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbab44 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbab50 | load | 4 | `abs 0xbac24` | `-` |
| `ldr` at 0xbab5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbab60 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbab6c | load | 4 | `abs 0xbac28` | `-` |
| `ldr` at 0xbab74 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbab7c | store | 4 | `sp -0x4c` | `[sp-76]+3` |
| `ldr` at 0xbab80 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbab88 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbab90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbab94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbaba4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbaba8 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0xbabac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbabb0 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xbabb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xbabb8 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0xbabbc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xbabc0 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xbabcc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbabd4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xbabe0 | store | 1 | `(unknown)` | `0xa` |
| `str` at 0xbabe8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xbabec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbabf0 | load | 4 | `sp -0x4c` | `-` |
| `strb` at 0xbabfc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xbac00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbac04 | store | 4 | `sp -0x50` | `[sp-28]` |
| `ldr` at 0xbac08 | load | 4 | `sp -0x50` | `-` |
| `ldmdb` at 0xbac0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbac0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbac0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba688 | `(unknown callee) 0x1b1b4` | `0x50` | `r1` | `r2` | `r0` |
| `bl` at 0xba69c | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `0x129220` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xba740 | `(unknown callee) 0x1b1b4` | `0x50` | `[r0+20]` | `r0` | `[r0+16]` |
| `bl` at 0xba76c | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `0x129234` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xba898 | `(unknown callee) 0x1b1b4` | `?` | `[r0+20]` | `?` | `?` |
| `bl` at 0xba930 | `(unknown callee) 0x1b1b4` | `[sp-72]` | `[sp-76]` | `?` | `?` |
| `bl` at 0xbaa04 | `(unknown callee) 0x1b070` | `sp-64` | `[sp-68]` | `[sp-68]+1` | `sp-64` |
| `bl` at 0xbaa28 | `(unknown callee) 0x1b1cc` | `([sp-28] + [sp-76])` | `0x13` | `0x129248` | `ret(0x1b070)` |
| `bl` at 0xbaa3c | `(unknown callee) 0x1b2bc` | `([sp-28] + [sp-76])` | `?` | `[sp-28]` | `([sp-28] + [sp-76])` |
| `bl` at 0xbaaa8 | `(unknown callee) 0x1b55c` | `?` | `?` | `?` | `[[sp-20]]` |
| `bl` at 0xbaaf8 | `(unknown callee) 0x1b52c` | `([sp-28] + [sp-76])` | `0x129258` | `[[sp-48]]:1` | `[[sp-48]]:1` |
| `bl` at 0xbab20 | `(unknown callee) 0x1b52c` | `([sp-28] + [sp-76])` | `0x12925c` | `[[sp-48]]:1` | `[[sp-48]]:1` |
| `bl` at 0xbab54 | `(unknown callee) 0x1b364` | `([sp-28] + [sp-76])` | `0x129264` | `[sp-76]` | `([sp-28] + [sp-76])` |
| `bl` at 0xbab70 | `(unknown callee) 0x1b364` | `([sp-28] + [sp-76])` | `0x129268` | `[sp-76]` | `([sp-28] + [sp-76])` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba680 | `r0 != 0x0` | `r0 == 0x0` |
| `bls` at 0xba6c0 | `[r0+20] <= [r0+16] (unsigned, 32-bit)` | `[r0+20] > [r0+16] (unsigned, 32-bit)` |
| `bhs` at 0xba704 | `[r0+20] >= [r0+16] (unsigned, 32-bit)` | `[r0+20] < [r0+16] (unsigned, 32-bit)` |
| `bne` at 0xba754 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `beq` at 0xba788 | `[r0] == 0x0` | `[r0] != 0x0` |
| `bne` at 0xba7ac | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xba7c8 | `C == 0` | `not (C == 0)` |
| `beq` at 0xba7d8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xba7f8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xba83c | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xba870 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xba8ac | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xba944 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xba97c | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xba998 | `C == 0` | `not (C == 0)` |
| `beq` at 0xba9ac | `[[sp-20]] == 0x0` | `[[sp-20]] != 0x0` |
| `beq` at 0xba9b8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xba9ec | `[sp-60]-1 != 0x0` | `[sp-60]-1 == 0x0` |
| `beq` at 0xba9f8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xbaa64 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xbaad4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xbab3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbab9c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbac0c | `r0` | `?` |

## `IRLogFrame`

- `IRLogFrame`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb89f4 to 0xb8b04.
  Established. The halfword through `r1 +0x10` is read (`ldrh` at 0xb8a40). The byte through `r1 +0x16` is read (`ldrb` at 0xb8a90). The unnamed `0x1b52c` call carries `sp-152`, `0x128f8c`, entry
  `r0`, entry `r1` (`bl` at 0xb8ae4). Frame log field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8a04 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8a08 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8a0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8a18 | load | 4 | `abs 0xb8af8` | `-` |
| `ldr` at 0xb8a20 | load | 4 | `abs 0xb8afc` | `-` |
| `ldr` at 0xb8a30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8a34 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xb8a38 | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xb8a3c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8a40 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xb8a44 | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xb8a48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8a4c | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xb8a50 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xb8a54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8a58 | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xb8a5c | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xb8a60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8a64 | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xb8a68 | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xb8a6c | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xb8a70 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8a78 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb8a7c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8a80 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8a88 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xb8a8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8a90 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb8a94 | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xb8a98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8a9c | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xb8aa0 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xb8aa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8aa8 | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xb8aac | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xb8ab0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8ab4 | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xb8ab8 | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xb8abc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8ac0 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xb8ac4 | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xb8ac8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8acc | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xb8ad0 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xb8ad8 | load | 4 | `abs 0xb8b00` | `-` |
| `ldr` at 0xb8adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8ae0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8af4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8af4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8a1c | `(unknown callee) 0x202d0` | `0x128eec` | `r1` | `r2` | `r1` |
| `bl` at 0xb8a24 | `(unknown callee) 0x202d0` | `0x128f3c` | `?` | `?` | `?` |
| `bl` at 0xb8ae4 | `(unknown callee) 0x1b52c` | `sp-152` | `0x128f8c` | `r0` | `r1` |
| `bl` at 0xb8af0 | `(unknown callee) 0x202d0` | `sp-152` | `?` | `?` | `sp-152` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8a14 | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8af4 | `r0` | `ret(0x202d0)` |

## `IRPrintFrame`

- `IRPrintFrame`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8b04 to 0xb8c1c.
  Established. The absolute word at `0x145ae8` is read (`ldr` at 0xb8b2c). The unnamed `0x240b8`
  call at `0xb8c04` carries `[0x145ae8]`, `0x12907c`, entry `r0`, entry `r1` (`bl` at 0xb8c04); its
  result is reported by the return (`ldmdb` at 0xb8c08). Frame display interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8b14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8b18 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8b1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8b28 | load | 4 | `abs 0xb8c0c` | `-` |
| `ldr` at 0xb8b2c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb8b30 | load | 4 | `abs 0xb8c10` | `-` |
| `ldr` at 0xb8b38 | load | 4 | `abs 0xb8c0c` | `-` |
| `ldr` at 0xb8b3c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb8b40 | load | 4 | `abs 0xb8c14` | `-` |
| `ldr` at 0xb8b4c | load | 4 | `abs 0xb8c0c` | `-` |
| `ldr` at 0xb8b50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8b54 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xb8b58 | store | 4 | `sp -0xcc` | `[r1]` |
| `ldr` at 0xb8b5c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8b60 | load | 2 | `r1 +0x10` | `-` |
| `str` at 0xb8b64 | store | 4 | `sp -0xc8` | `[r1+16]:2` |
| `ldr` at 0xb8b68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8b6c | load | 1 | `r1 +0x12` | `-` |
| `str` at 0xb8b70 | store | 4 | `sp -0xc4` | `[r1+18]:1` |
| `ldr` at 0xb8b74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8b78 | load | 1 | `r1 +0x13` | `-` |
| `str` at 0xb8b7c | store | 4 | `sp -0xc0` | `[r1+19]:1` |
| `ldr` at 0xb8b80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8b84 | load | 1 | `r1 +0x14` | `-` |
| `str` at 0xb8b88 | store | 4 | `sp -0xbc` | `[r1+20]:1` |
| `ldr` at 0xb8b8c | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0xb8b90 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8b98 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0xb8b9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8ba0 | load | 1 | `r1 +0x15` | `-` |
| `str` at 0xb8ba8 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xb8bac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8bb0 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb8bb4 | store | 4 | `sp -0xb0` | `[r1+22]:1` |
| `ldr` at 0xb8bb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8bbc | load | 4 | `r1 +0x18` | `-` |
| `str` at 0xb8bc0 | store | 4 | `sp -0xac` | `[r1+24]` |
| `ldr` at 0xb8bc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8bc8 | load | 4 | `r1 +0x1c` | `-` |
| `str` at 0xb8bcc | store | 4 | `sp -0xa8` | `[r1+28]` |
| `ldr` at 0xb8bd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8bd4 | load | 1 | `r1 +0x20` | `-` |
| `str` at 0xb8bd8 | store | 4 | `sp -0xa4` | `[r1+32]:1` |
| `ldr` at 0xb8bdc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb8be0 | load | 1 | `r1 +0x21` | `-` |
| `str` at 0xb8be4 | store | 4 | `sp -0xa0` | `[r1+33]:1` |
| `ldr` at 0xb8be8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb8bec | load | 2 | `r1 +0x22` | `-` |
| `str` at 0xb8bf0 | store | 4 | `sp -0x9c` | `[r1+34]:2` |
| `ldr` at 0xb8bf4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb8bf8 | load | 4 | `abs 0xb8c18` | `-` |
| `ldr` at 0xb8bfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb8c08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb8c08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb8c08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8b34 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128fdc` | `r2` | `0x145ae8` |
| `bl` at 0xb8b44 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x12902c` | `?` | `0x145ae8` |
| `bl` at 0xb8c04 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x12907c` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8b24 | `r1 != 0x0` | `r1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb8c08 | `r0` | `ret(0x240b8)` |

## `IRFmtAddr`

- `IRFmtAddr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8654 to 0xb8928.
  Established. Signed entry `r0 >= 0x0` takes `0xb86a8`, while negative values fall through (`bge` at 0xb867c). The unnamed `0x1b52c` call at `0xb8894` carries `[sp-24]`, `0x128ea8`, `[sp-32]`,
  `[sp-32]` (`bl` at 0xb8894). The known word assignment at `sp -0x18` writes `([sp-24] + ret(0x1b2bc))` (`str` at 0xb88ac). Address formatting beyond known edges remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8664 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8668 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb866c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8670 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb8674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8680 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8694 | store | 4 | `sp -0x38` | `0xe` |
| `str` at 0xb86a0 | store | 4 | `sp -0x38` | `0xf` |
| `ldr` at 0xb86a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb86b0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb86b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb86bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb86c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb86c4 | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xb86c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb86d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb86dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb86e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb86ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb86f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8700 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb8704 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb870c | store | 4 | `sp -0x3c` | `?` |
| `str` at 0xb8718 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb871c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb8720 | store | 4 | `sp -0x2c` | `[sp-60]` |
| `ldr` at 0xb8724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8738 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb873c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb8744 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb874c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8754 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xb8758 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb875c | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xb8760 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xb8768 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb888c | load | 4 | `abs 0xb8920` | `-` |
| `ldr` at 0xb8890 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb88a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb88ac | store | 4 | `sp -0x18` | `([sp-24] + ret(0x1b2bc))` |
| `ldr` at 0xb88b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb88b4 | store | 4 | `sp -0x4c` | `[sp-44]` |
| `ldr` at 0xb88b8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb88bc | store | 4 | `sp -0x48` | `[sp-48]` |
| `ldr` at 0xb88c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb88c4 | load | 4 | `abs 0xb8924` | `-` |
| `ldr` at 0xb88c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb88cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb88d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb88e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb88e8 | store | 4 | `sp -0x18` | `([sp-24] + ret(0x1b2bc))` |
| `ldr` at 0xb88ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb88f0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0xb88f4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xb88f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb88f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb88f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8894 | `(unknown callee) 0x1b52c` | `[sp-24]` | `0x128ea8` | `[sp-32]` | `[sp-32]` |
| `bl` at 0xb889c | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xb88d0 | `(unknown callee) 0x1b52c` | `([sp-24] + ret(0x1b2bc))` | `0x128eb0` | `[sp-36]` | `[sp-40]` |
| `bl` at 0xb88d8 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb867c | `r0 >= 0x0 (signed, 32-bit)` | `r0 < 0x0 (signed, 32-bit)` |
| `bne` at 0xb868c | `? != 0x2` | `? == 0x2` |
| `blt` at 0xb86f4 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0xb872c | `[sp-20] >= 0x0 (signed, 32-bit)` | `[sp-20] < 0x0 (signed, 32-bit)` |
| `ldrls` at 0xb8768 | `[sp-32] <= 0xf (unsigned, 32-bit)` | `[sp-32] > 0xf (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb88f8 | `r0` | `[sp-28]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb8768).

## `UtilFmtBrick`

- `UtilFmtBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a5c8 to 0x6a65c.
  Established. The unnamed `0xc5870` call carries entry `r2`, `r1`, `r2`, `r3` (`bl` at 0x6a5ec).
  The unnamed `0x1b4cc` call at `0x6a62c` carries `[sp-20]`, `0x2d`, `0x3`, `[sp-24]` (`bl` at 0x6a62c). A later unnamed `0x1b4cc` call carries `[sp-20]+4`, `0x2d`, `0x2`, `[sp-20]+4` (`bl` at 0x6a650). Brick formatting policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a5d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6a5dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6a5e0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6a5e4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6a5e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6a5f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6a5fc | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x6a600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a604 | load | 4 | `abs 0x6a658` | `-` |
| `ldr` at 0x6a608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6a630 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6a63c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6a654 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6a654 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6a654 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a5ec | `(unknown callee) 0xc5870` | `r2` | `r1` | `r2` | `r3` |
| `bl` at 0x6a610 | `(unknown callee) 0x1b52c` | `[sp-20]` | `0x10fb3c` | `[sp-24]` | `?` |
| `bl` at 0x6a62c | `(unknown callee) 0x1b4cc` | `[sp-20]` | `0x2d` | `0x3` | `[sp-24]` |
| `bl` at 0x6a650 | `(unknown callee) 0x1b4cc` | `[sp-20]+4` | `0x2d` | `0x2` | `[sp-20]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6a61c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x6a638 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6a654 | `r0` | `?` |

## `UtilFmtXTalk`

- `UtilFmtXTalk`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a65c to 0x6a6d8.
  Established. Unsigned entry `r0 <= 0x3` takes an unresolved computed destination, while larger
  values fall through (`ldrls` at 0x6a678). The known return reports `[sp-24]` (`ldmdb` at 0x6a6c4).
  XTalk label selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a66c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6a670 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x6a678 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6a6c0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6a6c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6a6c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6a6c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x6a678 | `r0 <= 0x3 (unsigned, 32-bit)` | `r0 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6a6c4 | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6a678).

## `UtilDspBrick`

- `UtilDspBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a384 to 0x6a5c8.
  Established. The unnamed `0x75194` call carries `0x0`, entry `r1`, `r2`, `r3` (`bl` at 0x6a398).
  The subsequent unnamed `0x6a5c8` call carries `sp-36`, `[ret(0x75194)+8]`, `[ret(0x75194)+16]`,
  `[ret(0x75194)+12]` (`bl` at 0x6a3c4). The unnamed `0x51b28` call carries `0x31`, `sp-36`, `0x0`,
  `sp-36` (`bl` at 0x6a5b8). Brick display effects on unresolved edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6a3a0 | store | 4 | `sp -0x28` | `ret(0x75194)` |
| `ldr` at 0x6a3a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6a3ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6a3b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6a3b8 | load | 4 | `ret(0x75194) +0x8` | `-` |
| `ldr` at 0x6a3bc | load | 4 | `ret(0x75194) +0x10` | `-` |
| `ldr` at 0x6a3c0 | load | 4 | `ret(0x75194) +0xc` | `-` |
| `ldr` at 0x6a3c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6a3cc | load | 4 | `[sp-40] +0x10` | `-` |
| `ldrls` at 0x6a3d8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x6a5bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6a5bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6a5bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6a5bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6a398 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x6a3c4 | `(unknown callee) 0x6a5c8` | `sp-36` | `[ret(0x75194)+8]` | `[ret(0x75194)+16]` | `[ret(0x75194)+12]` |
| `bl` at 0x6a5b8 | `(unknown callee) 0x51b28` | `0x31` | `sp-36` | `0x0` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x6a3d8 | `[[sp-40]+16]-35 <= 0x59 (unsigned, 32-bit)` | `[[sp-40]+16]-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6a5bc | `r0` | `ret(0x51b28)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6a3d8).

## `brickConvertType`

- `brickConvertType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5428 to 0xc5870.
  Established. Unsigned `r0-35 <= 0x59` takes an unresolved computed destination, while larger
  values fall through (`ldrls` at 0xc5448). The known unnamed `0x1b52c` call carries `0x14ab90`,
  `0x133830`, entry `r0`, `r0-35` (`bl` at 0xc57d4). The known return reports `0x14ab90` (`ldmdb` at 0xc57e0). Brick type naming remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5438 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc543c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc5448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc57c8 | load | 4 | `abs 0xc57e4` | `-` |
| `ldr` at 0xc57cc | load | 4 | `abs 0xc586c` | `-` |
| `ldr` at 0xc57d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc57d8 | load | 4 | `abs 0xc57e4` | `-` |
| `ldmdb` at 0xc57e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc57e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc57e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc57d4 | `(unknown callee) 0x1b52c` | `0x14ab90` | `0x133830` | `r0` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc5448 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc57e0 | `r0` | `0x14ab90` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc5448).

## `brickConvertString`

- `brickConvertString`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5058 to 0xc5428.
  Established. The word at `sp -0x18` receives `0x3f` (`str` at 0xc5070). The unnamed `0x1b2bc` call
  carries entry `r0`, `r1`, `r2`, `0x3f` (`bl` at 0xc5078); its result unequal to `0x1` takes
  `0xc51a0`, while equality falls through (`bne` at 0xc5084). String-to-brick mapping beyond known
  edges remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5068 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5070 | store | 4 | `sp -0x18` | `0x3f` |
| `ldr` at 0xc5074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5088 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc508c | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrls` at 0xc50a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc51a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc51a4 | load | 4 | `abs 0xc53e8` | `-` |
| `str` at 0xc51bc | store | 4 | `sp -0x18` | `0x43` |
| `ldr` at 0xc51c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc51c8 | load | 4 | `abs 0xc53ec` | `-` |
| `str` at 0xc51e0 | store | 4 | `sp -0x18` | `0x33` |
| `ldr` at 0xc51e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc51ec | load | 4 | `abs 0xc53f0` | `-` |
| `str` at 0xc5204 | store | 4 | `sp -0x18` | `0x34` |
| `ldr` at 0xc520c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5210 | load | 4 | `abs 0xc53f4` | `-` |
| `str` at 0xc5228 | store | 4 | `sp -0x18` | `0x35` |
| `ldr` at 0xc5230 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5234 | load | 4 | `abs 0xc53f8` | `-` |
| `str` at 0xc524c | store | 4 | `sp -0x18` | `0x36` |
| `ldr` at 0xc5254 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5258 | load | 4 | `abs 0xc53fc` | `-` |
| `str` at 0xc5270 | store | 4 | `sp -0x18` | `0x40` |
| `ldr` at 0xc5278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc527c | load | 4 | `abs 0xc5400` | `-` |
| `str` at 0xc5294 | store | 4 | `sp -0x18` | `0x4f` |
| `ldr` at 0xc529c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52a0 | load | 4 | `abs 0xc5404` | `-` |
| `str` at 0xc52b8 | store | 4 | `sp -0x18` | `0x42` |
| `ldr` at 0xc52c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52c4 | load | 4 | `abs 0xc5408` | `-` |
| `str` at 0xc52dc | store | 4 | `sp -0x18` | `0x4c` |
| `ldr` at 0xc52e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52e8 | load | 4 | `abs 0xc540c` | `-` |
| `str` at 0xc5300 | store | 4 | `sp -0x18` | `0x5a` |
| `ldr` at 0xc5308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc530c | load | 4 | `abs 0xc5410` | `-` |
| `str` at 0xc5324 | store | 4 | `sp -0x18` | `0x7c` |
| `ldr` at 0xc532c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5330 | load | 4 | `abs 0xc5414` | `-` |
| `str` at 0xc5348 | store | 4 | `sp -0x18` | `0x44` |
| `ldr` at 0xc5350 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5354 | load | 4 | `abs 0xc5418` | `-` |
| `str` at 0xc536c | store | 4 | `sp -0x18` | `0x53` |
| `ldr` at 0xc5374 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5378 | load | 4 | `abs 0xc541c` | `-` |
| `str` at 0xc5390 | store | 4 | `sp -0x18` | `0x31` |
| `ldr` at 0xc5398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc539c | load | 4 | `abs 0xc5420` | `-` |
| `str` at 0xc53b4 | store | 4 | `sp -0x18` | `0x32` |
| `ldr` at 0xc53bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc53c0 | load | 4 | `abs 0xc5424` | `-` |
| `str` at 0xc53d8 | store | 4 | `sp -0x18` | `0x54` |
| `ldr` at 0xc53dc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc53e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc53e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc53e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5078 | `(unknown callee) 0x1b2bc` | `r0` | `r1` | `r2` | `0x3f` |
| `bl` at 0xc5094 | `(unknown callee) 0x1b34c` | `[[sp-20]]:1` | `?` | `?` | `[[sp-20]]:1` |
| `bl` at 0xc51a8 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133658` | `?` | `ret(0x1b2bc)` |
| `bl` at 0xc51cc | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133660` | `?` | `ret(0x1b088)` |
| `bl` at 0xc51f0 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133668` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5214 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133670` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5238 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133678` | `?` | `ret(0x1b088)` |
| `bl` at 0xc525c | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133680` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5280 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133688` | `?` | `ret(0x1b088)` |
| `bl` at 0xc52a4 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133690` | `?` | `ret(0x1b088)` |
| `bl` at 0xc52c8 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x133698` | `?` | `ret(0x1b088)` |
| `bl` at 0xc52ec | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336a0` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5310 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336a8` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5334 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336b0` | `?` | `ret(0x1b088)` |
| `bl` at 0xc5358 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336b8` | `?` | `ret(0x1b088)` |
| `bl` at 0xc537c | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336c0` | `?` | `ret(0x1b088)` |
| `bl` at 0xc53a0 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336c4` | `?` | `ret(0x1b088)` |
| `bl` at 0xc53c4 | `(unknown callee) 0x1b088` | `[sp-20]` | `0x1336c8` | `?` | `ret(0x1b088)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc5084 | `ret(0x1b2bc) != 0x1` | `ret(0x1b2bc) == 0x1` |
| `ldrls` at 0xc50a4 | `ret(0x1b34c)-35 <= 0x36 (unsigned, 32-bit)` | `ret(0x1b34c)-35 > 0x36 (unsigned, 32-bit)` |
| `bne` at 0xc51b4 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc51d8 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc51fc | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5220 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5244 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5268 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc528c | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc52b0 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc52d4 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc52f8 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc531c | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5340 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5364 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc5388 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc53ac | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xc53d0 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc53e4 | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc50a4).

## `brickIsCompute`

- `brickIsCompute`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5bbc to 0xc5cd4.
  Established. Entry `r0 == 0x43` takes `0xc5cc0`, while inequality falls through (`beq` at 0xc5be0). Another included equality `r0 == 0x7c` takes that destination, while inequality falls
  through (`beq` at 0xc5cb8). The word at `sp -0x18` receives `0x1` there (`str` at 0xc5cc4).
  Compute-brick classification meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5bcc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5bd4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc5bd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5bfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5cb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5cc4 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xc5cc8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc5cd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5cd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5cd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5be0 | `r0 == 0x43` | `r0 != 0x43` |
| `beq` at 0xc5bec | `r0 == 0x33` | `r0 != 0x33` |
| `beq` at 0xc5bf8 | `r0 == 0x35` | `r0 != 0x35` |
| `beq` at 0xc5c04 | `r0 == 0x34` | `r0 != 0x34` |
| `beq` at 0xc5c10 | `r0 == 0x41` | `r0 != 0x41` |
| `beq` at 0xc5c1c | `r0 == 0x36` | `r0 != 0x36` |
| `beq` at 0xc5c28 | `r0 == 0x39` | `r0 != 0x39` |
| `beq` at 0xc5c34 | `r0 == 0x37` | `r0 != 0x37` |
| `beq` at 0xc5c40 | `r0 == 0x38` | `r0 != 0x38` |
| `beq` at 0xc5c4c | `r0 == 0x30` | `r0 != 0x30` |
| `beq` at 0xc5c58 | `r0 == 0x51` | `r0 != 0x51` |
| `beq` at 0xc5c64 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0xc5c70 | `r0 == 0x4f` | `r0 != 0x4f` |
| `beq` at 0xc5c7c | `r0 == 0x42` | `r0 != 0x42` |
| `beq` at 0xc5c88 | `r0 == 0x46` | `r0 != 0x46` |
| `beq` at 0xc5c94 | `r0 == 0x48` | `r0 != 0x48` |
| `beq` at 0xc5ca0 | `r0 == 0x4c` | `r0 != 0x4c` |
| `beq` at 0xc5cac | `r0 == 0x4d` | `r0 != 0x4d` |
| `beq` at 0xc5cb8 | `r0 == 0x7c` | `r0 != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5cd0 | `r0` | `?` |

## `brickTypeToChar`

- `brickTypeToChar`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5870 to 0xc5bbc.
  Established. Unsigned `r0-35 <= 0x59` takes an unresolved computed destination, while larger
  values fall through (`ldrls` at 0xc5890). The known unnamed `0x1b178` call carries entry `r0`,
  `r1`, `r2`, `r0-35` (`bl` at 0xc5ba8). Brick character mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5880 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5884 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc5890 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc5ba4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5bb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5bb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5bb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5ba8 | `(unknown callee) 0x1b178` | `r0` | `r1` | `r2` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc5890 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5bb8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc5890).

## `MemUsageBuf`

- `MemUsageBuf`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbac2c to 0xbae6c.
  Established. The word at `sp -0x3c` receives `0x14a9b0` (`str` at 0xbac4c). The unnamed `0x1b52c`
  call at `0xbac5c` carries `0x14a9b0`, `0x12926c`, `[sp-56]`, `0x14a9b0` (`bl` at 0xbac5c). The
  return reports `0x14a9b0` (`ldmdb` at 0xbae38). Memory report field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbac48 | load | 4 | `abs 0xbae3c` | `-` |
| `str` at 0xbac4c | store | 4 | `sp -0x3c` | `0x14a9b0` |
| `ldr` at 0xbac50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbac54 | load | 4 | `abs 0xbae40` | `-` |
| `ldr` at 0xbac58 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbac60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbac70 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbac78 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbac7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbac80 | load | 4 | `abs 0xbae44` | `-` |
| `ldr` at 0xbac84 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbac8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbac9c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbaca4 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbaca8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbacac | load | 4 | `abs 0xbae48` | `-` |
| `ldr` at 0xbacb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xbacb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbacc8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbacd0 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbacd4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbacd8 | load | 4 | `abs 0xbae4c` | `-` |
| `ldr` at 0xbacdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbace4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbacf4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbacfc | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbad00 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad04 | load | 4 | `abs 0xbae50` | `-` |
| `ldr` at 0xbad08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbad10 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad20 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbad28 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbad2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad30 | load | 4 | `abs 0xbae54` | `-` |
| `ldr` at 0xbad34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbad3c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad4c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbad54 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbad58 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad5c | load | 4 | `abs 0xbae58` | `-` |
| `ldr` at 0xbad60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbad68 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad78 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbad80 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbad84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbad88 | load | 4 | `abs 0xbae5c` | `-` |
| `ldr` at 0xbad8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbad94 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbada4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbadac | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbadb0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbadb4 | load | 4 | `abs 0xbae60` | `-` |
| `ldr` at 0xbadb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbadc0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbadd0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbadd8 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbaddc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbade0 | load | 4 | `abs 0xbae64` | `-` |
| `ldr` at 0xbade4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbadec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbadfc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbae04 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbae08 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbae0c | load | 4 | `abs 0xbae68` | `-` |
| `ldr` at 0xbae14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbae24 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbae2c | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x1b2bc))` |
| `ldr` at 0xbae30 | load | 4 | `abs 0xbae3c` | `-` |
| `ldmdb` at 0xbae38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbae38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbae38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbac44 | `(unknown callee) 0x1b208` | `sp-56` | `r1` | `r2` | `sp-56` |
| `bl` at 0xbac5c | `(unknown callee) 0x1b52c` | `0x14a9b0` | `0x12926c` | `[sp-56]` | `0x14a9b0` |
| `bl` at 0xbac64 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbac88 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x129280` | `[sp-52]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbac90 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbacb4 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x129294` | `[sp-48]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbacbc | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbace0 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x1292a8` | `[sp-44]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbace8 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbad0c | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x1292bc` | `[sp-40]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbad14 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbad38 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x1292d0` | `[sp-36]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbad40 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbad64 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x1292e4` | `[sp-32]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbad6c | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbad90 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x1292f8` | `[sp-28]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbad98 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbadbc | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x12930c` | `[sp-24]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbadc4 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbade8 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x129320` | `[sp-20]` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbadf0 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xbae10 | `(unknown callee) 0x1b52c` | `([sp-60] + ret(0x1b2bc))` | `0x129334` | `ret(0x1b2bc)` | `([sp-60] + ret(0x1b2bc))` |
| `bl` at 0xbae18 | `(unknown callee) 0x1b2bc` | `[sp-60]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbae38 | `r0` | `0x14a9b0` |
