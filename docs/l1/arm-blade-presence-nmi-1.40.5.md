# ARM blade presence and NMI paths in 1.40.5

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
No unresolved computed transfers are reported. Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/blade-presence-nmi-facts.txt) retain all tool sections.

## `ip63_cpu_disable`

- `ip63_cpu_disable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x384e0 to 0x3850c.
  Established. The byte assignment at `sp -0x11` writes `(r0 & 0xff)` (`strb` at 0x384f4). The
  unnamed `0x202d0` call carries `0x108108`, `r1`, `r2`, `r0` (`bl` at 0x384fc). CPU disable effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x384f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x384f8 | load | 4 | `abs 0x38508` | `-` |
| `ldmdb` at 0x38504 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38504 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x384fc | `(unknown callee) 0x202d0` | `0x108108` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38504 | `r0` | `?` |

## `ip63_cpu_enable`

- `ip63_cpu_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3850c to 0x38538.
  Established. The unnamed `0x202d0` call carries `0x108120`, `r1`, `r2`, `r0` (`bl` at 0x38528).
  The return value is unknown (`ldmdb` at 0x38530). CPU enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38520 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38524 | load | 4 | `abs 0x38534` | `-` |
| `ldmdb` at 0x38530 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38530 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38530 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38528 | `(unknown callee) 0x202d0` | `0x108120` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38530 | `r0` | `?` |

## `ip63_cpu_present`

- `ip63_cpu_present`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38538 to 0x38594.
  Established. The absolute byte at `0x2b1408` is read (`ldrb` at 0x3855c); its zero value takes
  `0x38580`, and nonzero falls through (`beq` at 0x38564). A second absolute byte read uses
  `0x2b1468` (`ldrb` at 0x3856c). CPU presence policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3854c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x38554 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x38558 | load | 4 | `abs 0x3858c` | `-` |
| `ldrb` at 0x3855c | load | 1 | `abs 0x2b1408` | `-` |
| `ldr` at 0x38568 | load | 4 | `abs 0x38590` | `-` |
| `ldrb` at 0x3856c | load | 1 | `abs 0x2b1468` | `-` |
| `str` at 0x3857c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x38580 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x38588 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38588 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38588 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38564 | `[0x2b1408]:1 == 0x0` | `[0x2b1408]:1 != 0x0` |
| `beq` at 0x38574 | `[0x2b1468]:1 == 0x0` | `[0x2b1468]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38588 | `r0` | `?` |

## `ip63Int`

- `ip63Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38594 to 0x385e0.
  Established. The unnamed `0x2079c` call carries `0x108134`, `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)` (`bl` at 0x385d4). The return reports `ret(0x2079c)` (`ldmdb` at 0x385d8). Interrupt
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x385a8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x385b0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x385b8 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x385bc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x385c0 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x385c4 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x385c8 | load | 4 | `abs 0x385dc` | `-` |
| `ldmdb` at 0x385d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x385d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x385d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x385d4 | `(unknown callee) 0x2079c` | `0x108134` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x385d8 | `r0` | `ret(0x2079c)` |

## `ip63BladePresent`

- `ip63BladePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x385e0 to 0x38610.
  Established. The byte assignment at `(r0 & 0xff) +0x2b1408` writes `0x1` (`strb` at 0x38604). The
  reported return value is entry `r0` (`ldmdb` at 0x38608). Blade-state meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x385f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x385f8 | load | 4 | `abs 0x3860c` | `-` |
| `ldrb` at 0x385fc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x38604 | store | 1 | `(r0 & 0xff) +0x2b1408` | `0x1` |
| `ldmdb` at 0x38608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38608 | `r0` | `r0` |

## `ip63BladeNotPresent`

- `ip63BladeNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38610 to 0x38640.
  Established. The byte at `(r0 & 0xff) +0x2b1408` receives `0x0` (`strb` at 0x38634). The return
  reports entry `r0` (`ldmdb` at 0x38638). Removal signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38624 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38628 | load | 4 | `abs 0x3863c` | `-` |
| `ldrb` at 0x3862c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x38634 | store | 1 | `(r0 & 0xff) +0x2b1408` | `0x0` |
| `ldmdb` at 0x38638 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38638 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38638 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38638 | `r0` | `r0` |

## `ip63BladeIsPresent`

- `ip63BladeIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38640 to 0x38670.
  Established. The byte read uses `(r0 & 0xff) +0x2b1408` (`ldrb` at 0x38660); its expression `[(r0 & 0xff)+2823176]:1` is the reported return (`ldmdb` at 0x38668). Presence-state interpretation
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38654 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38658 | load | 4 | `abs 0x3866c` | `-` |
| `ldrb` at 0x3865c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x38660 | load | 1 | `(r0 & 0xff) +0x2b1408` | `-` |
| `ldmdb` at 0x38668 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38668 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38668 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38668 | `r0` | `[(r0 & 0xff)+2823176]:1` |

## `IP63_Nmi`

- `IP63_Nmi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38698 to 0x386c0.
  Established. The unnamed `0x202d0` call uses `0x108170`, `r1`, `r2`, `r3` (`bl` at 0x386b0); its
  surrounding body stores entry `r0` as a word at `sp -0x14` (`str` at 0x386a8). NMI effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x386a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x386ac | load | 4 | `abs 0x386bc` | `-` |
| `ldmdb` at 0x386b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x386b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x386b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x386b0 | `(unknown callee) 0x202d0` | `0x108170` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x386b8 | `r0` | `?` |

## `daytonaBladePresent`

- `daytonaBladePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36380 to 0x363b0.
  Established. A byte at `(r0 & 0xff) +0x2b1408` receives `0x1` (`strb` at 0x363a4). The entry byte
  expression `(r0 & 0xff)` is also saved at `sp -0x11` (`strb` at 0x36394). Blade activation
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x36394 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x36398 | load | 4 | `abs 0x363ac` | `-` |
| `ldrb` at 0x3639c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x363a4 | store | 1 | `(r0 & 0xff) +0x2b1408` | `0x1` |
| `ldmdb` at 0x363a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x363a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x363a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x363a8 | `r0` | `r0` |

## `daytonaBladeNotPresent`

- `daytonaBladeNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x363b0 to 0x363e0.
  Established. The assignment at `(r0 & 0xff) +0x2b1408` writes byte `0x0` (`strb` at 0x363d4). The
  saved byte at `sp -0x11` contains `(r0 & 0xff)` (`strb` at 0x363c4). Blade deactivation semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x363c4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x363c8 | load | 4 | `abs 0x363dc` | `-` |
| `ldrb` at 0x363cc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x363d4 | store | 1 | `(r0 & 0xff) +0x2b1408` | `0x0` |
| `ldmdb` at 0x363d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x363d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x363d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x363d8 | `r0` | `r0` |

## `daytonaBladeIsPresent`

- `daytonaBladeIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x363e0 to 0x36410.
  Established. A byte is read from `(r0 & 0xff) +0x2b1408` (`ldrb` at 0x36400). The reported return
  expression is `[(r0 & 0xff)+2823176]:1` (`ldmdb` at 0x36408). Blade presence encoding remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x363f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x363f8 | load | 4 | `abs 0x3640c` | `-` |
| `ldrb` at 0x363fc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x36400 | load | 1 | `(r0 & 0xff) +0x2b1408` | `-` |
| `ldmdb` at 0x36408 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36408 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36408 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36408 | `r0` | `[(r0 & 0xff)+2823176]:1` |

## `daytonaInt`

- `daytonaInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x36410 to 0x3645c.
  Established. The unnamed `0x2079c` call carries `0x107564`, `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)` (`bl` at 0x36450); its result is reported at the return (`ldmdb` at 0x36454). Interrupt
  delivery remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x36424 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x3642c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x36434 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x36438 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3643c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x36440 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x36444 | load | 4 | `abs 0x36458` | `-` |
| `ldmdb` at 0x36454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x36450 | `(unknown callee) 0x2079c` | `0x107564` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36454 | `r0` | `ret(0x2079c)` |

## `nmi`

- `nmi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e918 to 0x4e998.
  Established. The absolute word at `0x2af354` is read (`ldr` at 0x4e950). Equality with zero takes
  `0x4e980`, while nonzero falls through (`beq` at 0x4e958). The indirect call has an unknown target
  and candidates `[sp-21]:1`, unknown, `[sp-21]:1`, `[0x2af354]` (`mov` at 0x4e970). NMI delivery
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e928 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4e930 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x4e934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e940 | load | 4 | `abs 0x4e990` | `-` |
| `ldr` at 0x4e94c | load | 4 | `abs 0x4e994` | `-` |
| `ldr` at 0x4e950 | load | 4 | `abs 0x2af354` | `-` |
| `ldr` at 0x4e95c | load | 4 | `abs 0x4e994` | `-` |
| `ldrb` at 0x4e960 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4e964 | load | 4 | `abs 0x2af354` | `-` |
| `str` at 0x4e978 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e984 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e988 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x4e98c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e98c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e98c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e938 | `(unknown callee) 0x4ea90` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x4e948 | `(unknown callee) 0x2079c` | `0x10b0ec` | `ret(0x4ea90)` | `?` | `ret(0x4ea90)` |
| `mov` at 0x4e970 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-21]:1` | `[0x2af354]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e958 | `[0x2af354] == 0x0` | `[0x2af354] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e98c | `r0` | `?` |

## `nmiMaskClr`

- `nmiMaskClr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ea1c to 0x4ea4c.
  Established. The absolute word assignments at `0x148f74` and `0x148f78` each write `0x0` (`str` at
  0x4ea30, `str` at 0x4ea3c). The return reports entry `r0` (`ldmdb` at 0x4ea40). Mask hardware
  meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4ea28 | load | 4 | `abs 0x4ea44` | `-` |
| `str` at 0x4ea30 | store | 4 | `abs 0x148f74` | `0x0` |
| `ldr` at 0x4ea34 | load | 4 | `abs 0x4ea48` | `-` |
| `str` at 0x4ea3c | store | 4 | `abs 0x148f78` | `0x0` |
| `ldmdb` at 0x4ea40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ea40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ea40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ea40 | `r0` | `r0` |

## `nmiMaskGet`

- `nmiMaskGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ea4c to 0x4ea90.
  Established. The absolute word read uses `0x148f74` (`ldr` at 0x4ea6c). The assignment through `r0 +0x0` writes `[0x148f74]` (`str` at 0x4ea70). A second word assignment at `[sp-24] +0x0` writes
  `[0x148f78]` (`str` at 0x4ea80). Mask field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ea5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4ea60 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4ea64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ea68 | load | 4 | `abs 0x4ea88` | `-` |
| `ldr` at 0x4ea6c | load | 4 | `abs 0x148f74` | `-` |
| `str` at 0x4ea70 | store | 4 | `r0 +0x0` | `[0x148f74]` |
| `ldr` at 0x4ea74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4ea78 | load | 4 | `abs 0x4ea8c` | `-` |
| `ldr` at 0x4ea7c | load | 4 | `abs 0x148f78` | `-` |
| `str` at 0x4ea80 | store | 4 | `[sp-24] +0x0` | `[0x148f78]` |
| `ldmdb` at 0x4ea84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ea84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ea84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ea84 | `r0` | `r0` |
