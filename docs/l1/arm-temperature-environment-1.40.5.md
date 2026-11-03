# ARM temperature and environment callers in 1.40.5

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
These routines have no unresolved computed transfers in the fact sheets.
Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/temperature-environment-facts.txt) retain all tool sections.

## `tempInit`

- `tempInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53488 to 0x535b0.
  Established. A four-byte store writes `0x1` at absolute `0x1490c4` (`str` at 0x5350c). The unnamed
  call to `0x1b454` carries `[sp-40]`, `0x1`, `0x1490c4`, unknown, and its result is stored at
  absolute `0x1490c0` (`bl` at 0x5352c, `str` at 0x53534). Later stores at `[sp-36] +0x0` and `+0x4`
  retain an unknown four-byte value and byte `([sp-32]:1 & 0xff)` respectively (`str` at 0x53588,
  `strb` at 0x53594). Temperature-record roles and allocation effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53498 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5349c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x534a0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x534a4 | load | 4 | `abs 0x5359c` | `-` |
| `ldr` at 0x534a8 | load | 4 | `abs 0x1490c0` | `-` |
| `ldr` at 0x534b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x534c0 | load | 4 | `abs 0x535a0` | `-` |
| `str` at 0x534c4 | store | 4 | `sp -0x34` | `0x10bd0c` |
| `ldr` at 0x534c8 | load | 4 | `abs 0x5359c` | `-` |
| `ldr` at 0x534cc | load | 4 | `abs 0x1490c0` | `-` |
| `str` at 0x534d0 | store | 4 | `sp -0x30` | `[0x1490c0]` |
| `ldr` at 0x534d4 | load | 4 | `abs 0x535a4` | `-` |
| `ldr` at 0x534d8 | load | 4 | `abs 0x1490c4` | `-` |
| `str` at 0x534dc | store | 4 | `sp -0x2c` | `[0x1490c4]` |
| `ldr` at 0x534e4 | load | 4 | `abs 0x535a8` | `-` |
| `ldr` at 0x534f4 | load | 4 | `abs 0x535a4` | `-` |
| `ldr` at 0x534f8 | load | 4 | `abs 0x1490c4` | `-` |
| `ldr` at 0x53504 | load | 4 | `abs 0x535a4` | `-` |
| `str` at 0x5350c | store | 4 | `abs 0x1490c4` | `0x1` |
| `ldr` at 0x53510 | load | 4 | `abs 0x535a4` | `-` |
| `ldr` at 0x53514 | load | 4 | `abs 0x1490c4` | `-` |
| `str` at 0x5351c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x53520 | load | 4 | `abs 0x5359c` | `-` |
| `ldr` at 0x53524 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x53534 | store | 4 | `abs 0x1490c0` | `ret(0x1b454)` |
| `ldr` at 0x53538 | load | 4 | `abs 0x1490c0` | `-` |
| `ldr` at 0x53544 | load | 4 | `abs 0x535ac` | `-` |
| `str` at 0x53548 | store | 4 | `sp -0x34` | `0x10bd50` |
| `ldr` at 0x5354c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x53550 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x53558 | load | 4 | `abs 0x535a8` | `-` |
| `ldr` at 0x53568 | load | 4 | `abs 0x5359c` | `-` |
| `ldr` at 0x5356c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53574 | load | 4 | `abs 0x1490c0` | `-` |
| `str` at 0x5357c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x53580 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53584 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53588 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x5358c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x53590 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x53594 | store | 1 | `[sp-36] +0x4` | `([sp-32]:1 & 0xff)` |
| `ldmdb` at 0x53598 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x53598 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53598 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x534f0 | `(unknown callee) 0x21eb8` | `0x9` | `0x10bd3c` | `0x3a` | `0x0` |
| `bl` at 0x5352c | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `0x1490c4` | `?` |
| `bl` at 0x53564 | `(unknown callee) 0x21eb8` | `0x9` | `0x10bd3c` | `0x46` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x534b0 | `[0x1490c0] == 0x0` | `[0x1490c0] != 0x0` |
| `bne` at 0x534bc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x53500 | `[0x1490c4] != 0x0` | `[0x1490c4] == 0x0` |
| `bne` at 0x53540 | `[0x1490c0] != 0x0` | `[0x1490c0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53598 | `r0` | `?` |

## `tempGetCfg`

- `tempGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x535b0 to 0x53604.
  Established. The four-byte read at absolute `0x1490c0` precedes a four-byte store of `[[sp-28]]`
  at `r1 +0x0` (`ldr` at 0x535d4, `str` at 0x535ec). A byte load at `[sp-28] +0x4` supplies return
  expression `[[sp-28]+4]:1` (`ldrb` at 0x535f4, `ldmdb` at 0x535fc). Configuration-field meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x535c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x535c4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x535c8 | load | 4 | `abs 0x53600` | `-` |
| `ldr` at 0x535cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x535d4 | load | 4 | `abs 0x1490c0` | `-` |
| `str` at 0x535dc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x535e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x535e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x535e8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x535ec | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x535f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x535f4 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0x535fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x535fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x535fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x535fc | `r0` | `[[sp-28]+4]:1` |

## `tempCount`

- `tempCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53604 to 0x53644.
  Established. A four-byte load reads absolute `0x1490c0` (`ldr` at 0x53624). The byte read at
  `[sp-24] +0x4` supplies `[[sp-24]+4]:1` to the exit (`ldrb` at 0x53634, `ldmdb` at 0x5363c). The
  counted object is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53614 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x53618 | load | 4 | `abs 0x53640` | `-` |
| `ldr` at 0x5361c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x53624 | load | 4 | `abs 0x1490c0` | `-` |
| `str` at 0x5362c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x53630 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x53634 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0x5363c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5363c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5363c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5363c | `r0` | `[[sp-24]+4]:1` |

## `tempIsPresent`

- `tempIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53644 to 0x53690.
  Established. The unnamed call to `0x539a4` uses `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl` at
  0x5366c); its return supplies a byte load at `ret(0x539a4) +0x0` (`ldrb` at 0x53674). The
  following unnamed call to `0xcce28` carries `[sp-20]`, `[ret(0x539a4)]:1`, unknown,
  `[ret(0x539a4)]:1` (`bl` at 0x53680). Physical-presence detection is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53654 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5365c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x53660 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x53664 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x53674 | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x53678 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5368c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5368c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5368c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5366c | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x53680 | `(unknown callee) 0xcce28` | `[sp-20]` | `[ret(0x539a4)]:1` | `?` | `[ret(0x539a4)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5368c | `r0` | `ret(0xcce28)` |

## `tempSetLimits`

- `tempSetLimits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53690 to 0x5370c.
  Established. The unnamed call to `0xcd4bc` carries `[sp-20]`, `[ret(0x539a4)]:1`,
  `[ret(0x539a4)+1]:1`, `[sp-22]:1` (`bl` at 0x536fc). Before that call, a four-byte stack store
  retains `[sp-23]:1` at `sp -0x20` (`str` at 0x536f0). The exit carries `ret(0xcd4bc)` (`ldmdb` at
  0x53708). Limit units and the callee's effect remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x536a0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x536ac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x536b4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x536bc | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x536c0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x536c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x536d4 | store | 4 | `sp -0x1c` | `ret(0x539a4)` |
| `ldr` at 0x536d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x536dc | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x536e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x536e4 | load | 1 | `ret(0x539a4) +0x1` | `-` |
| `ldrb` at 0x536e8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x536ec | load | 1 | `sp -0x17` | `-` |
| `str` at 0x536f0 | store | 4 | `sp -0x20` | `[sp-23]:1` |
| `ldr` at 0x536f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x53708 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53708 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53708 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x536cc | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x536fc | `(unknown callee) 0xcd4bc` | `[sp-20]` | `[ret(0x539a4)]:1` | `[ret(0x539a4)+1]:1` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53708 | `r0` | `ret(0xcd4bc)` |

## `tempIntDisable`

- `tempIntDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5370c to 0x5376c.
  Established. The byte loads read `ret(0x539a4) +0x0` and `+0x1` (`ldrb` at 0x53744, `ldrb` at
  0x5374c). The unnamed call to `0xce554` has candidates `[sp-20]`, `[ret(0x539a4)]:1`,
  `[ret(0x539a4)+1]:1`, `[ret(0x539a4)+1]:1` (`bl` at 0x5375c); its result remains `ret(0xce554)` at
  the exit (`ldmdb` at 0x53768). Interrupt-disable signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5371c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53724 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x53728 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5372c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5373c | store | 4 | `sp -0x1c` | `ret(0x539a4)` |
| `ldr` at 0x53740 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53744 | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x53748 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5374c | load | 1 | `ret(0x539a4) +0x1` | `-` |
| `ldr` at 0x53750 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x53768 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53768 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53768 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53734 | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x5375c | `(unknown callee) 0xce554` | `[sp-20]` | `[ret(0x539a4)]:1` | `[ret(0x539a4)+1]:1` | `[ret(0x539a4)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53768 | `r0` | `ret(0xce554)` |

## `tempIntEnable`

- `tempIntEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5376c to 0x537cc.
  Established. Following the unnamed call to `0x539a4`, two byte reads target `ret(0x539a4) +0x0`
  and `+0x1` (`bl` at 0x53794, `ldrb` at 0x537a4, `ldrb` at 0x537ac). The unnamed call to `0xce59c`
  carries `[sp-20]`, `[ret(0x539a4)]:1`, `[ret(0x539a4)+1]:1`, `[ret(0x539a4)+1]:1` (`bl` at
  0x537bc). The exit preserves `ret(0xce59c)` (`ldmdb` at 0x537c8). Interrupt-enable effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5377c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53784 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x53788 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5378c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5379c | store | 4 | `sp -0x1c` | `ret(0x539a4)` |
| `ldr` at 0x537a0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x537a4 | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x537a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x537ac | load | 1 | `ret(0x539a4) +0x1` | `-` |
| `ldr` at 0x537b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x537c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x537c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x537c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53794 | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x537bc | `(unknown callee) 0xce59c` | `[sp-20]` | `[ret(0x539a4)]:1` | `[ret(0x539a4)+1]:1` | `[ret(0x539a4)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x537c8 | `r0` | `ret(0xce59c)` |

## `tempRead`

- `tempRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x537cc to 0x53874.
  Established. The unnamed call to `0xcdac4` has candidates `[sp-20]`, `[ret(0x539a4)]:1`,
  `[ret(0x539a4)+1]:1`, `sp-33` (`bl` at 0x53828); its zero result takes the branch to `0x53858`
  (`beq` at 0x5383c). The fall-through assigns four-byte `0x0` at `[sp-28] +0x0` (`str` at 0x53848).
  The other arm reads a byte at `sp -0x21` and stores an unknown four-byte value at `[sp-28] +0x0`
  (`ldrsb` at 0x5385c, `str` at 0x53860). Temperature units and sensor effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x537dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x537e4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x537e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x537ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x537f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53800 | store | 4 | `sp -0x20` | `ret(0x539a4)` |
| `ldr` at 0x53804 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x53808 | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x5380c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x53810 | load | 1 | `ret(0x539a4) +0x1` | `-` |
| `ldr` at 0x53818 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53830 | store | 4 | `sp -0x28` | `ret(0xcdac4)` |
| `ldr` at 0x53834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x53840 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53848 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x5384c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x53850 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x53858 | load | 4 | `sp -0x1c` | `-` |
| `ldrsb` at 0x5385c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x53860 | store | 4 | `[sp-28] +0x0` | `?` |
| `str` at 0x53868 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x5386c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x53870 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53870 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53870 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x537f8 | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x53828 | `(unknown callee) 0xcdac4` | `[sp-20]` | `[ret(0x539a4)]:1` | `[ret(0x539a4)+1]:1` | `sp-33` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5383c | `ret(0xcdac4) == 0x0` | `ret(0xcdac4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53870 | `r0` | `[sp-44]` |

## `tempLocation`

- `tempLocation`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53874 to 0x538e8.
  Established. Two four-byte loads read `ret(0x539a4) +0x4` (`ldr` at 0x538ac, `ldr` at 0x538b8).
  The branch takes `[ret(0x539a4)+4] != 0x0` to `0x538dc` (`bne` at 0x538c0). On the zero fall-through, the
  unnamed call to `0xce8f8` carries `[sp-20]`, `[ret(0x539a4)]:1`, unknown, `[ret(0x539a4)]:1` (`bl`
  at 0x538d4). The merged return is unknown (`ldmdb` at 0x538e4). Location-string semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53884 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5388c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x53890 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x53894 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x538a4 | store | 4 | `sp -0x1c` | `ret(0x539a4)` |
| `ldr` at 0x538a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x538ac | load | 4 | `ret(0x539a4) +0x4` | `-` |
| `str` at 0x538b0 | store | 4 | `sp -0x20` | `[ret(0x539a4)+4]` |
| `ldr` at 0x538b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x538b8 | load | 4 | `ret(0x539a4) +0x4` | `-` |
| `ldr` at 0x538c4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x538c8 | load | 1 | `ret(0x539a4) +0x0` | `-` |
| `ldr` at 0x538cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x538d8 | store | 4 | `sp -0x20` | `ret(0xce8f8)` |
| `ldr` at 0x538dc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x538e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x538e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x538e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5389c | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x538d4 | `(unknown callee) 0xce8f8` | `[sp-20]` | `[ret(0x539a4)]:1` | `?` | `[ret(0x539a4)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x538c0 | `[ret(0x539a4)+4] != 0x0` | `[ret(0x539a4)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x538e4 | `r0` | `?` |

## `tempType`

- `tempType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x538e8 to 0x539a4.
  Established. The byte read at `ret(0x539a4) +0x2` precedes the branch taken when
  `[ret(0x539a4)+2]:1 == 0x0` (`ldrb` at 0x53920, `beq` at 0x53928). That arm includes an unnamed
  call to `0xcd15c` with `[sp-20]`, `sp-32`, unknown, `sp-32` (`bl` at 0x53948). A later branch
  takes `[[sp-28]]:1 >= ret(0xcd15c) (signed, 32-bit)` to `0x53994` (`bge` at 0x53964); its
  destination stores four-byte `0x0` at `sp -0x28` (`str` at 0x53998). Device-type numbering is
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x538f8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53900 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x53904 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x53908 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53918 | store | 4 | `sp -0x1c` | `ret(0x539a4)` |
| `ldr` at 0x5391c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53920 | load | 1 | `ret(0x539a4) +0x2` | `-` |
| `ldr` at 0x5392c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53930 | load | 1 | `ret(0x539a4) +0x2` | `-` |
| `str` at 0x53934 | store | 4 | `sp -0x28` | `[ret(0x539a4)+2]:1` |
| `ldr` at 0x53940 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x53950 | store | 4 | `sp -0x24` | `ret(0xcd15c)` |
| `ldr` at 0x53954 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53958 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5395c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53968 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5396c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x53980 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x53988 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5398c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x53998 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5399c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x539a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x539a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x539a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53910 | `(unknown callee) 0x539a4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x53948 | `(unknown callee) 0xcd15c` | `[sp-20]` | `sp-32` | `?` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53928 | `[ret(0x539a4)+2]:1 == 0x0` | `[ret(0x539a4)+2]:1 != 0x0` |
| `bge` at 0x53964 | `[[sp-28]]:1 >= ret(0xcd15c) (signed, 32-bit)` | `[[sp-28]]:1 < ret(0xcd15c) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x539a0 | `r0` | `?` |

## `EnvIgnoreLowPower`

- `EnvIgnoreLowPower`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82d60 to 0x82d94.
  Established. The four-byte assignment at absolute `0x149b7c` has unknown value (`str` at 0x82d88).
  An earlier four-byte stack store retains entry `r0` at `sp -0x14` (`str` at 0x82d70). The exit
  carries entry `r0` (`ldmdb` at 0x82d8c). Low-power policy semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82d70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82d74 | load | 4 | `abs 0x82d90` | `-` |
| `ldr` at 0x82d78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x82d88 | store | 4 | `abs 0x149b7c` | `?` |
| `ldmdb` at 0x82d8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82d8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82d8c | `r0` | `r0` |

## `EnvAltitude`

- `EnvAltitude`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x829c8 to 0x829f0.
  Established. The unnamed call to `0x1d41c` uses `0xc`, `r0`, `r2`, `r3` (`bl` at 0x829e4); its
  return becomes the first candidate of the unnamed call to `0x829f0`, whose other three candidates
  are unknown (`bl` at 0x829e8). The exit preserves `ret(0x829f0)` (`ldmdb` at 0x829ec). Altitude
  units and update effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x829d8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x829e0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x829ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x829ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x829ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x829e4 | `(unknown callee) 0x1d41c` | `0xc` | `r0` | `r2` | `r3` |
| `bl` at 0x829e8 | `(unknown callee) 0x829f0` | `ret(0x1d41c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x829ec | `r0` | `ret(0x829f0)` |

## `EnvAuto`

- `EnvAuto`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8295c to 0x829c8.
  Established. A zero entry `r0` takes the branch to `0x829b4` (`beq` at 0x82978). The fall-through
  includes two unnamed calls to `0x1d41c`, with candidate prefixes `0x7`, unknown, `r2`, unknown,
  then `0x7d` and three unknown candidates (`bl` at 0x82994, `bl` at 0x829b0). The call at the
  branch destination has `0x6` followed by three unknown candidates (`bl` at 0x829bc).
  Automatic-policy semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8296c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8297c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82998 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x829b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x829c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x829c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x829c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82994 | `(unknown callee) 0x1d41c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x829b0 | `(unknown callee) 0x1d41c` | `0x7d` | `?` | `?` | `?` |
| `bl` at 0x829bc | `(unknown callee) 0x1d41c` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x829c0 | `(unknown callee) 0x829f0` | `ret(0x1d41c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x82978 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x829c4 | `r0` | `ret(0x829f0)` |

## `EnvGetStatus`

- `EnvGetStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82cf8 to 0x82d34.
  Established. The four-byte read at absolute `0x143590` supplies the four-byte store `[0x143590]`
  at `r0 +0x0` (`ldr` at 0x82d14, `str` at 0x82d18). Another four-byte read targets absolute
  `0x149e7c` (`ldr` at 0x82d20). The exit carries `[0x149e7c]` (`ldmdb` at 0x82d28).
  Environmental-status field meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82d08 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82d0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82d10 | load | 4 | `abs 0x82d2c` | `-` |
| `ldr` at 0x82d14 | load | 4 | `abs 0x143590` | `-` |
| `str` at 0x82d18 | store | 4 | `r0 +0x0` | `[0x143590]` |
| `ldr` at 0x82d1c | load | 4 | `abs 0x82d30` | `-` |
| `ldr` at 0x82d20 | load | 4 | `abs 0x149e7c` | `-` |
| `ldmdb` at 0x82d28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82d28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82d28 | `r0` | `[0x149e7c]` |

## `EnvReset`

- `EnvReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x829f0 to 0x82ab4.
  Established. The branch takes `[0x149b6c] != 0x0` to `0x82a14` (`bne` at 0x82a0c). The later
  four-byte store at absolute `0x149e64` has unknown value (`str` at 0x82a34). A zero `[0x2af304]`
  takes the branch past an unknown-target call whose candidates are `ret(0x7c5cc)`, unknown,
  unknown, `[0x2af304]` (`beq` at 0x82a48, `mov` at 0x82a58). Four-byte assignments at `sp -0x18`
  include `0x6`, `0x2` and `0x1` in separate arms (`str` at 0x82a80, `str` at 0x82a8c, `str` at
  0x82a98). Reset-state meanings and callback effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x82a00 | load | 4 | `abs 0x82aa8` | `-` |
| `ldr` at 0x82a04 | load | 4 | `abs 0x149b6c` | `-` |
| `ldr` at 0x82a18 | load | 4 | `abs 0x82aac` | `-` |
| `str` at 0x82a34 | store | 4 | `abs 0x149e64` | `?` |
| `ldr` at 0x82a3c | load | 4 | `abs 0x82ab0` | `-` |
| `ldr` at 0x82a40 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x82a4c | load | 4 | `abs 0x82ab0` | `-` |
| `ldr` at 0x82a50 | load | 4 | `abs 0x2af304` | `-` |
| `ldr` at 0x82a5c | load | 4 | `abs 0x82aac` | `-` |
| `ldr` at 0x82a60 | load | 4 | `abs 0x149e64` | `-` |
| `ldr` at 0x82a6c | load | 4 | `abs 0x82aac` | `-` |
| `ldr` at 0x82a70 | load | 4 | `abs 0x149e70` | `-` |
| `str` at 0x82a80 | store | 4 | `sp -0x18` | `0x6` |
| `str` at 0x82a8c | store | 4 | `sp -0x18` | `0x2` |
| `str` at 0x82a98 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x82a9c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x82aa4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82aa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82aa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82aa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82a14 | `(unknown callee) 0x7b6e8` | `r0` | `r1` | `r2` | `[0x149b6c]` |
| `bl` at 0x82a20 | `(unknown callee) 0x1f26c` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x82a38 | `(unknown callee) 0x7c5cc` | `ret(0x1f26c)` | `?` | `?` | `?` |
| `mov` at 0x82a58 | `(unknown callee) (unknown target)` | `ret(0x7c5cc)` | `?` | `?` | `[0x2af304]` |
| `bl` at 0x82aa0 | `(unknown callee) 0x75afc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x82a0c | `[0x149b6c] != 0x0` | `[0x149b6c] == 0x0` |
| `beq` at 0x82a48 | `[0x2af304] == 0x0` | `[0x2af304] != 0x0` |
| `bne` at 0x82a68 | `[0x149e64] != 0x0` | `[0x149e64] == 0x0` |
| `beq` at 0x82a78 | `[0x149e70] == 0x0` | `[0x149e70] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82aa4 | `r0` | `?` |
