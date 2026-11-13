# ARM FRU capture and status paths in 1.40.5

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
`L1FRU_2m_Init` has an unresolved computed transfer; its access, call and return lists cover
known edges only. Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/fru-capture-status-facts.txt) retain all tool sections.

## `PBaySdrWarmReset`

- `PBaySdrWarmReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae704 to 0xae7e8.
  Established. The unnamed `0xab0d4` call carries `r0`, `sp-428`, `sp-260`, `sp-428` (`bl` at 0xae784); its zero result takes `0xae7a8`, while nonzero falls through (`beq` at 0xae798). The
  absolute byte at `0x14a77b` is read (`ldrb` at 0xae750). Warm-reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae714 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae718 | load | 4 | `abs 0xae7d8` | `-` |
| `ldr` at 0xae71c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xae720 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xae728 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xae730 | store | 4 | `sp -0x1a8` | `0x20` |
| `str` at 0xae738 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xae740 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xae748 | store | 4 | `sp -0x110` | `0x3` |
| `ldr` at 0xae74c | load | 4 | `abs 0xae7dc` | `-` |
| `ldrb` at 0xae750 | load | 1 | `abs 0x14a77b` | `-` |
| `strb` at 0xae758 | store | 1 | `sp -0x10c` | `([0x14a77b]:1 & 0xff)` |
| `str` at 0xae760 | store | 4 | `sp -0x108` | `0x6` |
| `strb` at 0xae76c | store | 1 | `sp -0x104` | `0x0` |
| `str` at 0xae774 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0xae77c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae78c | store | 4 | `sp -0x1b0` | `ret(0xab0d4)` |
| `ldr` at 0xae790 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xae79c | load | 4 | `abs 0xae7e0` | `-` |
| `str` at 0xae7a0 | store | 4 | `sp -0x1b4` | `0x127e54` |
| `ldr` at 0xae7a8 | load | 4 | `abs 0xae7e4` | `-` |
| `str` at 0xae7ac | store | 4 | `sp -0x1b4` | `0x127e58` |
| `str` at 0xae7bc | store | 4 | `sp -0x1b8` | `ret(0x1b1b4)` |
| `ldr` at 0xae7c0 | load | 4 | `sp -0x1b8` | `-` |
| `ldr` at 0xae7c4 | load | 4 | `sp -0x1b4` | `-` |
| `ldr` at 0xae7cc | load | 4 | `sp -0x1b8` | `-` |
| `ldmdb` at 0xae7d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae7d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae7d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae784 | `(unknown callee) 0xab0d4` | `r0` | `sp-428` | `sp-260` | `sp-428` |
| `bl` at 0xae7b4 | `(unknown callee) 0x1b1b4` | `0xc` | `?` | `?` | `?` |
| `bl` at 0xae7c8 | `(unknown callee) 0x1b598` | `ret(0x1b1b4)` | `[sp-436]` | `?` | `ret(0x1b1b4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae798 | `ret(0xab0d4) == 0x0` | `ret(0xab0d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae7d4 | `r0` | `[sp-440]` |

## `SdrSetChassisFru`

- `SdrSetChassisFru`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae6b0 to 0xae704.
  Established. The byte at `0x14365c` is read (`ldrb` at 0xae6cc). A value unequal to `0xff` takes
  `0xae6fc`, while equality falls through (`bne` at 0xae6d4). The absolute byte assignment at
  `0x14365c` writes `((r0 & 0xff) & 0xff)` (`strb` at 0xae6e0). Chassis-FRU encoding remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xae6c4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xae6c8 | load | 4 | `abs 0xae700` | `-` |
| `ldrb` at 0xae6cc | load | 1 | `abs 0x14365c` | `-` |
| `ldr` at 0xae6d8 | load | 4 | `abs 0xae700` | `-` |
| `ldrb` at 0xae6dc | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xae6e0 | store | 1 | `abs 0x14365c` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xae6e4 | load | 4 | `abs 0xae700` | `-` |
| `strb` at 0xae6ec | store | 1 | `abs 0x14365d` | `0x0` |
| `ldr` at 0xae6f0 | load | 4 | `abs 0xae700` | `-` |
| `strb` at 0xae6f8 | store | 1 | `abs 0x14365e` | `0x0` |
| `ldmdb` at 0xae6fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae6fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae6fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xae6d4 | `[0x14365c]:1 != 0xff` | `[0x14365c]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae6fc | `r0` | `r0` |

## `L1FRU_2m_Reset`

- `L1FRU_2m_Reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa72b0 to 0xa7314.
  Established. The absolute word assignments at `0x2b2880` and `0x2b2884` write `0x0` (`str` at
  0xa72e0, `str` at 0xa7308). The unnamed `0x1b490` call at `0xa72d4` carries `[0x2b2880]`, `r1`,
  `r2`, `0x2b2860` (`bl` at 0xa72d4). FRU-reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa72bc | load | 4 | `abs 0xa7310` | `-` |
| `ldr` at 0xa72c0 | load | 4 | `abs 0x2b2880` | `-` |
| `ldr` at 0xa72cc | load | 4 | `abs 0xa7310` | `-` |
| `ldr` at 0xa72d0 | load | 4 | `abs 0x2b2880` | `-` |
| `ldr` at 0xa72d8 | load | 4 | `abs 0xa7310` | `-` |
| `str` at 0xa72e0 | store | 4 | `abs 0x2b2880` | `0x0` |
| `ldr` at 0xa72e4 | load | 4 | `abs 0xa7310` | `-` |
| `ldr` at 0xa72e8 | load | 4 | `abs 0x2b2884` | `-` |
| `ldr` at 0xa72f4 | load | 4 | `abs 0xa7310` | `-` |
| `ldr` at 0xa72f8 | load | 4 | `abs 0x2b2884` | `-` |
| `ldr` at 0xa7300 | load | 4 | `abs 0xa7310` | `-` |
| `str` at 0xa7308 | store | 4 | `abs 0x2b2884` | `0x0` |
| `ldmdb` at 0xa730c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa730c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa730c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa72d4 | `(unknown callee) 0x1b490` | `[0x2b2880]` | `r1` | `r2` | `0x2b2860` |
| `bl` at 0xa72fc | `(unknown callee) 0x1b490` | `[0x2b2884]` | `?` | `0x2b2860` | `0x2b2860` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa72c8 | `[0x2b2880] == 0x0` | `[0x2b2880] != 0x0` |
| `beq` at 0xa72f0 | `[0x2b2884] == 0x0` | `[0x2b2884] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa730c | `r0` | `?` |

## `L1FRU_2m_Dump`

- `L1FRU_2m_Dump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa75f0 to 0xa761c.
  Established. The unnamed `0xc4df0` call carries `r0`, `0x0`, `r1`, `r3` (`bl` at 0xa7614). The
  return reports `ret(0xc4df0)` (`ldmdb` at 0xa7618). Dump contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7600 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7604 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa7608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa7610 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa7618 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7618 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7618 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7614 | `(unknown callee) 0xc4df0` | `r0` | `0x0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7618 | `r0` | `ret(0xc4df0)` |

## `L1FRU_2m_EnvStatus`

- `L1FRU_2m_EnvStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8a60 to 0xa8a80.
  Established. The unnamed `0x82548` call uses `0x2b2884`, `0x1`, `r2`, `r3` (`bl` at 0xa8a74); its
  result is reported by the return (`ldmdb` at 0xa8a78). Environment-status meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8a6c | load | 4 | `abs 0xa8a7c` | `-` |
| `ldmdb` at 0xa8a78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8a78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8a74 | `(unknown callee) 0x82548` | `0x2b2884` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8a78 | `r0` | `ret(0x82548)` |

## `L1FRU_2m_MgnStatus`

- `L1FRU_2m_MgnStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8a80 to 0xa8aa0.
  Established. The unnamed `0x94eb8` call carries `0x2b2888`, `0x1`, `r2`, `r3` (`bl` at 0xa8a94).
  The return reports `ret(0x94eb8)` (`ldmdb` at 0xa8a98). Margin-status interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8a8c | load | 4 | `abs 0xa8a9c` | `-` |
| `ldmdb` at 0xa8a98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8a98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8a98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8a94 | `(unknown callee) 0x94eb8` | `0x2b2888` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8a98 | `r0` | `ret(0x94eb8)` |

## `L1FRU_2m_Deadlock`

- `L1FRU_2m_Deadlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7524 to 0xa75f0.
  Established. The absolute word read uses `0x2b2874` (`ldr` at 0xa754c). Equality `[sp-32] == 0x1`
  takes `0xa75a0`, while inequality falls through (`beq` at 0xa758c). The unnamed `0xa902c` call has
  unknown first and fourth candidates with `[sp-20]` and `[sp-24]` between them (`bl` at 0xa75cc).
  Deadlock handling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7534 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa7538 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa7540 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa7544 | load | 4 | `abs 0xa75ec` | `-` |
| `ldr` at 0xa7548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa754c | load | 4 | `abs 0x2b2874` | `-` |
| `ldr` at 0xa755c | load | 4 | `abs 0xa75ec` | `-` |
| `ldr` at 0xa7560 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7574 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa757c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa7580 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7584 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa7590 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa75a0 | load | 4 | `abs 0xa75ec` | `-` |
| `ldr` at 0xa75a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa75b8 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa75c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa75c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa75c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa75d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa75d8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0xa75e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa75e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa75e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa75cc | `(unknown callee) 0xa902c` | `?` | `[sp-20]` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa7554 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa758c | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `blt` at 0xa7598 | `[sp-32] < 0x1 (signed, 32-bit)` | `[sp-32] >= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa75e8 | `r0` | `0x0` |

## `L1FRU_2m_Capture`

- `L1FRU_2m_Capture`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7314 to 0xa7524.
  Established. The absolute word assignment at `0x2b286c` writes `ret(0x1b274)` (`str` at 0xa7334).
  A nonzero word at `0x2b2880` takes `0xa735c`, while zero falls through (`bne` at 0xa7344). The
  absolute word assignment at `0x2b2868` writes `0x0` (`str` at 0xa73c4). Capture semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa7324 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa7328 | load | 4 | `abs 0xa7520` | `-` |
| `str` at 0xa7334 | store | 4 | `abs 0x2b286c` | `ret(0x1b274)` |
| `ldr` at 0xa7338 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa733c | load | 4 | `abs 0x2b2880` | `-` |
| `ldr` at 0xa7348 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa734c | load | 4 | `abs 0x2b2884` | `-` |
| `str` at 0xa7360 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa7368 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa7388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa73a0 | load | 4 | `abs 0xa7520` | `-` |
| `str` at 0xa73a8 | store | 4 | `abs 0x2b2870` | `0x1` |
| `ldr` at 0xa73b0 | load | 4 | `abs 0xa7520` | `-` |
| `str` at 0xa73b8 | store | 4 | `abs 0x2b2870` | `0x2` |
| `ldr` at 0xa73bc | load | 4 | `abs 0xa7520` | `-` |
| `str` at 0xa73c4 | store | 4 | `abs 0x2b2868` | `0x0` |
| `str` at 0xa73cc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa73d0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa73d4 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa73e0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa73e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa73e8 | load | 4 | `abs 0x2b2874` | `-` |
| `ldr` at 0xa73f8 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa73fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7410 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7418 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa741c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa7420 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa742c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa7438 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa744c | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa7450 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7464 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa7478 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa747c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa7490 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa74a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa74a8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa74b0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa74b4 | load | 4 | `abs 0x2b2880` | `-` |
| `ldr` at 0xa74c0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa74c4 | load | 4 | `abs 0x2b2880` | `-` |
| `ldr` at 0xa74d0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa74d4 | load | 4 | `abs 0x2b2884` | `-` |
| `ldr` at 0xa74e0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa74e4 | load | 4 | `abs 0x2b2884` | `-` |
| `ldr` at 0xa74f0 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa74f4 | load | 4 | `abs 0x2b2888` | `-` |
| `ldr` at 0xa7500 | load | 4 | `abs 0xa7520` | `-` |
| `ldr` at 0xa7504 | load | 4 | `abs 0x2b2888` | `-` |
| `str` at 0xa7514 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa7518 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xa751c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa751c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa751c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa751c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7330 | `(unknown callee) 0x1b274` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa7470 | `(unknown callee) 0xa8aa0` | `?` | `0x2b2860` | `?` | `?` |
| `bl` at 0xa749c | `(unknown callee) 0xa9bb4` | `?` | `0x2b2860` | `?` | `?` |
| `bl` at 0xa74c8 | `(unknown callee) 0x1b490` | `[0x2b2880]` | `?` | `?` | `0x2b2860` |
| `bl` at 0xa74cc | `(unknown callee) 0xa8a40` | `?` | `?` | `?` | `?` |
| `bl` at 0xa74e8 | `(unknown callee) 0x1b490` | `[0x2b2884]` | `?` | `?` | `0x2b2860` |
| `bl` at 0xa74ec | `(unknown callee) 0xa8a60` | `?` | `?` | `?` | `?` |
| `bl` at 0xa7508 | `(unknown callee) 0x1b490` | `[0x2b2888]` | `?` | `?` | `0x2b2860` |
| `bl` at 0xa750c | `(unknown callee) 0xa8a80` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa7344 | `[0x2b2880] != 0x0` | `[0x2b2880] == 0x0` |
| `bne` at 0xa7354 | `[0x2b2884] != 0x0` | `[0x2b2884] == 0x0` |
| `blt` at 0xa7370 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0xa7384 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa739c | `? != 0x5` | `? == 0x5` |
| `bne` at 0xa73dc | `[0x2b2868] != 0x0` | `[0x2b2868] == 0x0` |
| `blt` at 0xa73f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa7428 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0xa7434 | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xa7440 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0xa74bc | `[0x2b2880] == 0x0` | `[0x2b2880] != 0x0` |
| `beq` at 0xa74dc | `[0x2b2884] == 0x0` | `[0x2b2884] != 0x0` |
| `beq` at 0xa74fc | `[0x2b2888] == 0x0` | `[0x2b2888] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa751c | `r0` | `[sp-32]` |

## `L1FRU_2m_Attach`

- `L1FRU_2m_Attach`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa88a0 to 0xa8a40.
  Established. The unnamed `0xc25e4` call carries `[sp-48]`, `[sp-40]`, `[sp-44]`, `[0x2b2868]`
  (`bl` at 0xa8974). The unnamed `0x69ca4` call carries `0x3`, `sp-24`, `[sp-28]`, `[sp-32]` (`bl` at 0xa8994). Attachment record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa88b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa88b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa88b8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa88c4 | store | 4 | `sp -0x28` | `ret(0x1f26c)` |
| `str` at 0xa88d0 | store | 4 | `sp -0x2c` | `ret(0x1f26c)` |
| `str` at 0xa88e0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xa88e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa88f0 | load | 4 | `abs 0xa8a34` | `-` |
| `ldr` at 0xa88f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa88fc | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8900 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8904 | load | 4 | `abs 0x2b286c` | `-` |
| `str` at 0xa8908 | store | 4 | `sp -0x58` | `[0x2b286c]` |
| `ldr` at 0xa890c | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8910 | load | 4 | `abs 0x2b2870` | `-` |
| `str` at 0xa8914 | store | 4 | `sp -0x54` | `[0x2b2870]` |
| `ldr` at 0xa8918 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa891c | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa8920 | store | 4 | `sp -0x50` | `[0x2b2874]` |
| `ldr` at 0xa8924 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8928 | load | 4 | `abs 0x2b2878` | `-` |
| `str` at 0xa892c | store | 4 | `sp -0x4c` | `[0x2b2878]` |
| `ldr` at 0xa8930 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8934 | load | 4 | `abs 0x2b2880` | `-` |
| `str` at 0xa8938 | store | 4 | `sp -0x48` | `[0x2b2880]` |
| `ldr` at 0xa893c | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa8940 | load | 4 | `abs 0x2b2884` | `-` |
| `str` at 0xa8944 | store | 4 | `sp -0x44` | `[0x2b2884]` |
| `ldr` at 0xa8948 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa894c | load | 4 | `abs 0x2b2888` | `-` |
| `str` at 0xa8950 | store | 4 | `sp -0x40` | `[0x2b2888]` |
| `str` at 0xa8958 | store | 4 | `sp -0x3c` | `sp-32` |
| `str` at 0xa8960 | store | 4 | `sp -0x38` | `sp-36` |
| `ldr` at 0xa8964 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa8968 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa896c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8970 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa897c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa8980 | store | 4 | `sp -0x58` | `[sp-36]` |
| `ldr` at 0xa898c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8990 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa899c | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0xa89a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa89b0 | load | 4 | `abs 0xa8a3c` | `-` |
| `ldr` at 0xa89b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa89bc | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa89c0 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa89c4 | load | 4 | `abs 0x2b286c` | `-` |
| `str` at 0xa89c8 | store | 4 | `sp -0x58` | `[0x2b286c]` |
| `ldr` at 0xa89cc | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa89d0 | load | 4 | `abs 0x2b2870` | `-` |
| `str` at 0xa89d4 | store | 4 | `sp -0x54` | `[0x2b2870]` |
| `ldr` at 0xa89d8 | load | 4 | `abs 0xa8a38` | `-` |
| `ldr` at 0xa89dc | load | 4 | `abs 0x2b287c` | `-` |
| `str` at 0xa89e0 | store | 4 | `sp -0x50` | `[0x2b287c]` |
| `str` at 0xa89e8 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0xa89f0 | store | 4 | `sp -0x48` | `sp-36` |
| `ldr` at 0xa89f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa89f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa89fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8a00 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa8a10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8a14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8a18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8a20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8a24 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0xa8a2c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xa8a30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8a30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa88c0 | `(unknown callee) 0x1f26c` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xa88cc | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xa88d4 | `(unknown callee) 0x4f4d0` | `ret(0x1f26c)` | `?` | `?` | `?` |
| `bl` at 0xa88f8 | `(unknown callee) 0x2079c` | `0x1272dc` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0xa8974 | `(unknown callee) 0xc25e4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2b2868]` |
| `bl` at 0xa8994 | `(unknown callee) 0x69ca4` | `0x3` | `sp-24` | `[sp-28]` | `[sp-32]` |
| `bl` at 0xa89b8 | `(unknown callee) 0x2079c` | `0x1272e8` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0xa8a04 | `(unknown callee) 0xc330c` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2b2868]` |
| `bl` at 0xa8a1c | `(unknown callee) 0x69a08` | `sp-24` | `[sp-28]` | `[sp-32]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa88ec | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0xa89ac | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8a30 | `r0` | `[sp-52]` |

## `L1FRU_2m_Init`

- `L1FRU_2m_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa7038 to 0xa72b0.
  Established. The absolute word at `0x14a710` is read (`ldr` at 0xa704c). Nonzero takes `0xa71e4`,
  while zero falls through (`bne` at 0xa7054). The unnamed `0x1b4cc` call carries `0x2b2860`, `0x0`,
  `0x2c`, `[0x14a710]` (`bl` at 0xa7064). Initialization effects beyond known edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa7048 | load | 4 | `abs 0xa725c` | `-` |
| `ldr` at 0xa704c | load | 4 | `abs 0x14a710` | `-` |
| `ldr` at 0xa7058 | load | 4 | `abs 0xa7260` | `-` |
| `ldrls` at 0xa707c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa71e4 | load | 4 | `abs 0xa72a4` | `-` |
| `ldr` at 0xa71e8 | load | 4 | `abs 0x14a708` | `-` |
| `ldr` at 0xa71f4 | load | 4 | `abs 0xa72a8` | `-` |
| `str` at 0xa71f8 | store | 4 | `sp -0x1c` | `0x127164` |
| `ldr` at 0xa7200 | load | 4 | `abs 0xa72ac` | `-` |
| `ldr` at 0xa7210 | load | 4 | `abs 0xa72a4` | `-` |
| `ldr` at 0xa7214 | load | 4 | `abs 0xa72a4` | `-` |
| `ldr` at 0xa7218 | load | 4 | `abs 0x14a708` | `-` |
| `str` at 0xa7220 | store | 4 | `abs 0x14a708` | `[0x14a708]+1` |
| `ldr` at 0xa7224 | load | 4 | `abs 0xa7260` | `-` |
| `ldr` at 0xa7228 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa7234 | load | 4 | `abs 0xa7260` | `-` |
| `str` at 0xa723c | store | 4 | `abs 0x2b2868` | `0x3` |
| `ldr` at 0xa7244 | load | 4 | `abs 0xa72a4` | `-` |
| `ldr` at 0xa7248 | load | 4 | `abs 0xa72a4` | `-` |
| `ldr` at 0xa724c | load | 4 | `abs 0x14a708` | `-` |
| `str` at 0xa7254 | store | 4 | `abs 0x14a708` | `[0x14a708]+1` |
| `ldmdb` at 0xa7258 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa7258 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa7258 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa7258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa7064 | `(unknown callee) 0x1b4cc` | `0x2b2860` | `0x0` | `0x2c` | `[0x14a710]` |
| `bl` at 0xa7068 | `(unknown callee) 0x4f4d0` | `ret(0x1b4cc)` | `?` | `?` | `?` |
| `bl` at 0xa720c | `(unknown callee) 0x21eb8` | `0x9` | `0x127184` | `0x7e` | `0x0` |
| `bl` at 0xa7240 | `(unknown callee) 0xbdc28` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa7054 | `[0x14a710] != 0x0` | `[0x14a710] == 0x0` |
| `ldrls` at 0xa707c | `? <= 0x1a (unsigned, 32-bit)` | `? > 0x1a (unsigned, 32-bit)` |
| `beq` at 0xa71f0 | `[0x14a708] == 0x0` | `[0x14a708] != 0x0` |
| `bne` at 0xa7230 | `[0x2b2868] != 0x0` | `[0x2b2868] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa7258 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa707c).

## `L1FRU_2m_Examine`

- `L1FRU_2m_Examine`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa81f8 to 0xa88a0.
  Established. The unnamed `0xc428c` call carries `[sp-180]`, `sp-36`, `sp-40`, `sp-44` (`bl` at 0xa8404). The absolute word at `0x14a6fc` is read (`ldr` at 0xa8840); its expression `[0x14a6fc]`
  is assigned as a word at `[sp-24] +0x0` (`str` at 0xa8844). Examination result meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8208 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa820c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa8214 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0xa821c | store | 4 | `sp -0x48` | `?` |
| `str` at 0xa8224 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xa822c | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xa8234 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xa8244 | store | 4 | `sp -0xac` | `ret(0x75194)` |
| `ldr` at 0xa824c | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8250 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa8260 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8264 | load | 4 | `abs 0x2b2870` | `-` |
| `ldr` at 0xa8270 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa8278 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xa8280 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8284 | load | 4 | `abs 0x2b2870` | `-` |
| `ldr` at 0xa8290 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa8298 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xa829c | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa82a0 | load | 4 | `abs 0x2b286c` | `-` |
| `str` at 0xa82a4 | store | 4 | `sp -0x48` | `[0x2b286c]` |
| `ldr` at 0xa82a8 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa82ac | load | 4 | `abs 0x2b286c` | `-` |
| `str` at 0xa82b0 | store | 4 | `sp -0x4c` | `[0x2b286c]` |
| `ldr` at 0xa82b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa82bc | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0xa82c4 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa82c8 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa82d8 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xa82dc | load | 4 | `sp -0xac` | `-` |
| `str` at 0xa82e0 | store | 4 | `sp -0xcc` | `ret(0xc4804)` |
| `ldr` at 0xa82e8 | load | 4 | `abs 0xa8864` | `-` |
| `ldr` at 0xa82ec | load | 4 | `[sp-172] +0x8` | `-` |
| `ldr` at 0xa82f0 | load | 4 | `[sp-172] +0xc` | `-` |
| `str` at 0xa8310 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa8314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8324 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8328 | load | 4 | `[sp-20] +0x414` | `-` |
| `ldr` at 0xa8338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa833c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa8340 | load | 4 | `abs 0xa8868` | `-` |
| `ldr` at 0xa8350 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa8354 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa8358 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa835c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa836c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa8370 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa8384 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xa8388 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa8398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa839c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa83a0 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0xa83a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa83b4 | store | 4 | `sp -0xb4` | `ret(0xb823c)` |
| `str` at 0xa83c8 | store | 4 | `sp -0xcc` | `sp-48` |
| `str` at 0xa83d0 | store | 4 | `sp -0xc8` | `sp-52` |
| `str` at 0xa83d8 | store | 4 | `sp -0xc4` | `sp-56` |
| `str` at 0xa83e0 | store | 4 | `sp -0xc0` | `sp-60` |
| `str` at 0xa83e8 | store | 4 | `sp -0xbc` | `sp-64` |
| `str` at 0xa83f0 | store | 4 | `sp -0xb8` | `sp-168` |
| `ldr` at 0xa83f4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0xa8408 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8414 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa8428 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0xa8438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa843c | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0xa8448 | store | 4 | `sp -0xcc` | `ret(0xc4804)` |
| `ldr` at 0xa844c | load | 4 | `abs 0xa8864` | `-` |
| `ldr` at 0xa8468 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa8478 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa8480 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0xa8484 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8488 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8494 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa8498 | store | 4 | `sp -0x48` | `[sp-40]` |
| `ldr` at 0xa849c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa84a0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa84ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa84b0 | store | 4 | `sp -0x4c` | `[sp-40]` |
| `ldr` at 0xa84b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa84c0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xa84c8 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xa84d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa84dc | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa84e4 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xa84e8 | load | 4 | `abs 0xa886c` | `-` |
| `ldr` at 0xa84ec | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xa84f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa84fc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa8504 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8508 | load | 4 | `abs 0x2b287c` | `-` |
| `ldr` at 0xa8510 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8518 | load | 4 | `abs 0xa8870` | `-` |
| `ldr` at 0xa8520 | load | 4 | `abs 0x2b287c` | `-` |
| `ldr` at 0xa8528 | load | 4 | `abs 0xa8860` | `-` |
| `ldr` at 0xa8530 | load | 4 | `abs 0xa8870` | `-` |
| `ldr` at 0xa8538 | load | 4 | `abs 0x2b287c` | `-` |
| `ldr` at 0xa8540 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa8554 | store | 4 | `sp -0xb0` | `ret(0x1b274)` |
| `ldr` at 0xa8558 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa8564 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa8574 | load | 4 | `abs 0xa8874` | `-` |
| `ldr` at 0xa8584 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa8594 | load | 4 | `abs 0xa8878` | `-` |
| `ldr` at 0xa85a8 | load | 4 | `abs 0xa887c` | `-` |
| `ldr` at 0xa85d0 | load | 4 | `abs 0xa8880` | `-` |
| `ldr` at 0xa860c | load | 4 | `abs 0xa8884` | `-` |
| `ldr` at 0xa8630 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8634 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa863c | load | 4 | `abs 0xa8888` | `-` |
| `ldr` at 0xa8654 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8658 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8660 | load | 4 | `abs 0xa888c` | `-` |
| `ldr` at 0xa8678 | load | 4 | `abs 0xa888c` | `-` |
| `ldr` at 0xa86a4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa86a8 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa86b0 | load | 4 | `abs 0xa888c` | `-` |
| `str` at 0xa86dc | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0xa86e0 | load | 4 | `abs 0xa8890` | `-` |
| `ldr` at 0xa8704 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8708 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa8714 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8718 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa872c | load | 4 | `sp -0xb0` | `-` |
| `str` at 0xa8730 | store | 4 | `sp -0xcc` | `[sp-176]` |
| `ldr` at 0xa8738 | load | 4 | `abs 0xa8894` | `-` |
| `ldr` at 0xa873c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa8740 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8764 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa8768 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8770 | load | 4 | `abs 0xa8888` | `-` |
| `ldr` at 0xa8788 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa878c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa8794 | load | 4 | `abs 0xa888c` | `-` |
| `ldr` at 0xa87ac | load | 4 | `abs 0xa888c` | `-` |
| `ldr` at 0xa87d8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa87dc | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xa87e4 | load | 4 | `abs 0xa888c` | `-` |
| `str` at 0xa8810 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0xa8814 | load | 4 | `abs 0xa8898` | `-` |
| `ldr` at 0xa8838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa883c | load | 4 | `abs 0xa889c` | `-` |
| `ldr` at 0xa8840 | load | 4 | `abs 0x14a6fc` | `-` |
| `str` at 0xa8844 | store | 4 | `[sp-24] +0x0` | `[0x14a6fc]` |
| `ldr` at 0xa8848 | load | 4 | `abs 0xa889c` | `-` |
| `str` at 0xa8850 | store | 4 | `abs 0x14a6fc` | `0x0` |
| `ldr` at 0xa8854 | load | 4 | `abs 0xa886c` | `-` |
| `ldmdb` at 0xa885c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa885c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa885c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa823c | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0xa8248 | `(unknown callee) 0xa80a8` | `ret(0x75194)` | `?` | `?` | `ret(0x75194)` |
| `bl` at 0xa82cc | `(unknown callee) 0xc4804` | `[0x2b2868]` | `?` | `?` | `0x2b2860` |
| `bl` at 0xa82f4 | `(unknown callee) 0x1b52c` | `sp-164` | `0x1271a0` | `[[sp-172]+8]` | `[[sp-172]+12]` |
| `bl` at 0xa8308 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa83b0 | `(unknown callee) 0xb823c` | `?` | `0x1` | `0x418` | `?` |
| `bl` at 0xa8404 | `(unknown callee) 0xc428c` | `[sp-180]` | `sp-36` | `sp-40` | `sp-44` |
| `bl` at 0xa8418 | `(unknown callee) 0xc4804` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0xa8450 | `(unknown callee) 0x1b52c` | `sp-164` | `0x1271a0` | `?` | `?` |
| `bl` at 0xa8464 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa84f0 | `(unknown callee) 0xc1174` | `0x2b287c` | `[sp-168]` | `[sp-40]` | `?` |
| `bl` at 0xa850c | `(unknown callee) 0xc1374` | `[0x2b287c]` | `?` | `?` | `0x2b2860` |
| `bl` at 0xa8524 | `(unknown callee) 0xc166c` | `0x0` | `0xa80f0` | `0x0` | `[0x2b287c]` |
| `bl` at 0xa853c | `(unknown callee) 0xc166c` | `0x2` | `0xa80f0` | `0x0` | `[0x2b287c]` |
| `bl` at 0xa8550 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `[sp-68]` |
| `bl` at 0xa857c | `(unknown callee) 0xa80f0` | `0x0` | `0x1271b8` | `?` | `[sp-84]` |
| `bl` at 0xa859c | `(unknown callee) 0xa80f0` | `0x0` | `0x1271f0` | `?` | `[sp-80]` |
| `bl` at 0xa85b0 | `(unknown callee) 0xa80f0` | `0x0` | `0x127218` | `?` | `?` |
| `bl` at 0xa85bc | `(unknown callee) 0x1b070` | `sp-72` | `?` | `?` | `sp-72` |
| `bl` at 0xa85d8 | `(unknown callee) 0x1b1cc` | `sp-164` | `0x3c` | `0x127240` | `ret(0x1b070)` |
| `bl` at 0xa85ec | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa85f8 | `(unknown callee) 0x1b070` | `sp-76` | `?` | `?` | `sp-76` |
| `bl` at 0xa8614 | `(unknown callee) 0x1b1cc` | `sp-164` | `0x3c` | `0x127260` | `ret(0x1b070)` |
| `bl` at 0xa8628 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa86ec | `(unknown callee) 0x1b52c` | `sp-164` | `0x127280` | `?` | `?` |
| `bl` at 0xa8700 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa8744 | `(unknown callee) 0x1b52c` | `sp-164` | `0x12729c` | `[sp-72]` | `[sp-76]` |
| `bl` at 0xa8758 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa8820 | `(unknown callee) 0x1b52c` | `sp-164` | `0x1272c0` | `?` | `?` |
| `bl` at 0xa8834 | `(unknown callee) 0xa80f0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0xa8858 | `(unknown callee) 0xc1008` | `0x2b287c` | `?` | `0x14a6fc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa8258 | `[0x2b2868] == 0x0` | `[0x2b2868] != 0x0` |
| `bne` at 0xa826c | `[0x2b2870] != 0x1` | `[0x2b2870] == 0x1` |
| `bne` at 0xa828c | `[0x2b2870] != 0x2` | `[0x2b2870] == 0x2` |
| `beq` at 0xa831c | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `blt` at 0xa8330 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa8364 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa837c | `? == 0x1` | `? != 0x1` |
| `bne` at 0xa8394 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa8410 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0xa8470 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bge` at 0xa8490 | `[sp-40] >= [sp-72] (signed, 32-bit)` | `[sp-40] < [sp-72] (signed, 32-bit)` |
| `ble` at 0xa84a8 | `[sp-40] <= [sp-76] (signed, 32-bit)` | `[sp-40] > [sp-76] (signed, 32-bit)` |
| `bne` at 0xa84bc | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `bne` at 0xa84d8 | `[sp-44] != 0x2` | `[sp-44] == 0x2` |
| `beq` at 0xa8548 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0xa8560 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `beq` at 0xa856c | `[sp-84] == 0x0` | `[sp-84] != 0x0` |
| `beq` at 0xa858c | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `bgt` at 0xa8710 | `[sp-76] > [sp-176] (signed, 32-bit)` | `[sp-76] <= [sp-176] (signed, 32-bit)` |
| `bgt` at 0xa8720 | `[sp-72] > [sp-176] (signed, 32-bit)` | `[sp-72] <= [sp-176] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa885c | `r0` | `ret(0xc1008)` |

## `L1FRUSHUBInit`

- `L1FRUSHUBInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa998c to 0xa9b8c.
  Established. The word assignment at an unknown effective target writes `0x1` (`str` at 0xa9a00).
  The unnamed `0x1b1b4` call carries `[sp-28]`, `0x2b2860`, unknown, `[0x2b2878]` (`bl` at 0xa9b08).
  The absolute word assignment at `0x2b2874` writes `[0x2b2874]+1` (`str` at 0xa9b54). SHUB record
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa999c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa99a0 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa99a4 | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa99a8 | store | 4 | `sp -0x24` | `[0x2b2874]` |
| `ldr` at 0xa99ac | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa99b0 | load | 4 | `abs 0x2b2878` | `-` |
| `str` at 0xa99c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa99c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa99cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa99dc | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa99e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa99f4 | load | 4 | `abs 0x2b2878` | `-` |
| `str` at 0xa9a00 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xa9a04 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9a08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9a1c | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9a24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9a28 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xa9a2c | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9a30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9a44 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9a4c | load | 4 | `abs 0xa9b7c` | `-` |
| `ldr` at 0xa9a50 | load | 4 | `abs 0x1436c0` | `-` |
| `str` at 0xa9a54 | store | 4 | `(unknown)` | `[0x1436c0]` |
| `ldr` at 0xa9a58 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9a5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9a70 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9a78 | load | 4 | `abs 0xa9b80` | `-` |
| `str` at 0xa9a7c | store | 4 | `(unknown)` | `0x12c140` |
| `ldr` at 0xa9a80 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9a84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9a98 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9aa0 | load | 4 | `abs 0xa9b84` | `-` |
| `ldr` at 0xa9aa4 | load | 4 | `abs 0x1436c4` | `-` |
| `str` at 0xa9aa8 | store | 4 | `(unknown)` | `[0x1436c4]` |
| `ldr` at 0xa9aac | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9ab0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9ac4 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9acc | load | 4 | `abs 0xa9b88` | `-` |
| `str` at 0xa9ad0 | store | 4 | `(unknown)` | `0x12f650` |
| `ldr` at 0xa9ad4 | load | 4 | `abs 0xa9b7c` | `-` |
| `ldr` at 0xa9ad8 | load | 4 | `abs 0x1436c0` | `-` |
| `str` at 0xa9ae0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa9ae4 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9ae8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9afc | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9b04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa9b10 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xa9b14 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9b18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9b2c | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9b34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9b3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9b44 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9b48 | load | 4 | `abs 0xa9b78` | `-` |
| `ldr` at 0xa9b4c | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa9b54 | store | 4 | `abs 0x2b2874` | `[0x2b2874]+1` |
| `ldr` at 0xa9b58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9b60 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa9b64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9b6c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xa9b74 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa9b74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9b74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9b74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9b08 | `(unknown callee) 0x1b1b4` | `[sp-28]` | `0x2b2860` | `?` | `[0x2b2878]` |
| `bl` at 0xa9b40 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa99b8 | `[0x2b2878] != 0x0` | `[0x2b2878] == 0x0` |
| `blt` at 0xa99d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9b74 | `r0` | `?` |

## `L1FRUNL4RInit`

- `L1FRUNL4RInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9e2c to 0xaa02c.
  Established. The word assignment at an unknown target writes `0x2` (`str` at 0xa9ea0). The unnamed
  `0x1b1b4` call carries `[sp-28]`, `0x2b2860`, unknown, `[0x2b2878]` (`bl` at 0xa9fa8). The
  absolute word assignment at `0x2b2874` writes `[0x2b2874]+1` (`str` at 0xa9ff4). Link record
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9e3c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa9e40 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9e44 | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa9e48 | store | 4 | `sp -0x24` | `[0x2b2874]` |
| `ldr` at 0xa9e4c | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9e50 | load | 4 | `abs 0x2b2878` | `-` |
| `str` at 0xa9e64 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa9e68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9e7c | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9e80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9e94 | load | 4 | `abs 0x2b2878` | `-` |
| `str` at 0xa9ea0 | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0xa9ea4 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9ea8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9ebc | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9ec4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9ec8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xa9ecc | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9ed0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9ee4 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9eec | load | 4 | `abs 0xaa01c` | `-` |
| `ldr` at 0xa9ef0 | load | 4 | `abs 0x1436c8` | `-` |
| `str` at 0xa9ef4 | store | 4 | `(unknown)` | `[0x1436c8]` |
| `ldr` at 0xa9ef8 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9efc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9f10 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9f18 | load | 4 | `abs 0xaa020` | `-` |
| `str` at 0xa9f1c | store | 4 | `(unknown)` | `0x131948` |
| `ldr` at 0xa9f20 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9f24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9f38 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9f40 | load | 4 | `abs 0xaa024` | `-` |
| `ldr` at 0xa9f44 | load | 4 | `abs 0x1436cc` | `-` |
| `str` at 0xa9f48 | store | 4 | `(unknown)` | `[0x1436cc]` |
| `ldr` at 0xa9f4c | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9f50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9f64 | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9f6c | load | 4 | `abs 0xaa028` | `-` |
| `str` at 0xa9f70 | store | 4 | `(unknown)` | `0x1324f0` |
| `ldr` at 0xa9f74 | load | 4 | `abs 0xaa01c` | `-` |
| `ldr` at 0xa9f78 | load | 4 | `abs 0x1436c8` | `-` |
| `str` at 0xa9f80 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa9f84 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9f88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9f9c | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9fa4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa9fb0 | store | 4 | `(unknown)` | `ret(0x1b1b4)` |
| `ldr` at 0xa9fb4 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9fb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9fcc | load | 4 | `abs 0x2b2878` | `-` |
| `ldr` at 0xa9fd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa9fdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9fe4 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9fe8 | load | 4 | `abs 0xaa018` | `-` |
| `ldr` at 0xa9fec | load | 4 | `abs 0x2b2874` | `-` |
| `str` at 0xa9ff4 | store | 4 | `abs 0x2b2874` | `[0x2b2874]+1` |
| `ldr` at 0xa9ff8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xaa000 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xaa004 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa00c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xaa014 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaa014 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa014 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9fa8 | `(unknown callee) 0x1b1b4` | `[sp-28]` | `0x2b2860` | `?` | `[0x2b2878]` |
| `bl` at 0xa9fe0 | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa9e58 | `[0x2b2878] != 0x0` | `[0x2b2878] == 0x0` |
| `blt` at 0xa9e74 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa014 | `r0` | `?` |

## `L1FRUSHUBCapture`

- `L1FRUSHUBCapture`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8aa0 to 0xa8d80.
  Established. The absolute word assignment at `0x2b2868` writes `0x1` (`str` at 0xa8acc). The
  unnamed `0xeae04` call carries `ret(0x4869c)`, `[sp-40]`, `sp-48`, `sp-52` (`bl` at 0xa8c10). A
  later assignment at `0x2b2868` writes `0x4` (`str` at 0xa8d38). SHUB capture effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa8ab0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa8ab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ab8 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xa8ac4 | load | 4 | `abs 0xa8d78` | `-` |
| `str` at 0xa8acc | store | 4 | `abs 0x2b2868` | `0x1` |
| `ldr` at 0xa8ad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8adc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8ae0 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa8ae8 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa8afc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa8b00 | load | 4 | `abs 0xa8d78` | `-` |
| `ldr` at 0xa8b04 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa8b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8b18 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xa8b2c | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa8b30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8b48 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xa8b50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa8b54 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa8b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8b70 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0xa8b78 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa8b80 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa8b84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa8b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8b94 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0xa8ba4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xa8ba8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa8bac | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0xa8bb4 | store | 4 | `sp -0x3c` | `sp-48` |
| `str` at 0xa8bbc | store | 4 | `sp -0x38` | `sp-52` |
| `str` at 0xa8be4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8bf0 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa8c08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa8c24 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa8c28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa8c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8c40 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8c48 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8c50 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8c60 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8c68 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8c70 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8c74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8c80 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8c88 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa8c90 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8c94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8c98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8ca0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa8ca8 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xa8cac | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xa8cb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8cbc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8cc4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa8ccc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8cdc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8ce4 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa8cec | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8cf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8cf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8cfc | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa8d04 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa8d0c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa8d10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8d1c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa8d24 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa8d28 | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa8d30 | load | 4 | `abs 0xa8d78` | `-` |
| `str` at 0xa8d38 | store | 4 | `abs 0x2b2868` | `0x4` |
| `ldr` at 0xa8d3c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa8d40 | store | 4 | `sp -0x44` | `[sp-44]` |
| `ldr` at 0xa8d44 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa8d48 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xa8d4c | load | 4 | `abs 0xa8d7c` | `-` |
| `ldr` at 0xa8d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa8d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa8d58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa8d60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa8d68 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xa8d74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8d74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8d74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa8ad4 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0xa8af4 | `(unknown callee) 0x1b4cc` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0xa8b98 | `(unknown callee) 0x4869c` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa8bd0 | `(unknown callee) 0x642c8` | `ret(0x4869c)` | `?` | `?` | `?` |
| `bl` at 0xa8bf4 | `(unknown callee) 0x4869c` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa8c10 | `(unknown callee) 0xeae04` | `ret(0x4869c)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0xa8d5c | `(unknown callee) 0x2079c` | `0x1272f8` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0xa8d70 | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa8ac0 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0xa8b0c | `[0x2b2868] != 0x0` | `[0x2b2868] == 0x0` |
| `blt` at 0xa8b20 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xa8b8c | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `beq` at 0xa8c30 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8d74 | `r0` | `?` |

## `L1FRUNL4RCapture`

- `L1FRUNL4RCapture`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9bb4 to 0xa9e2c.
  Established. The unnamed `0xeae04` call carries `ret(0x279d0)`, `[sp-40]`, `sp-48`, `sp-52` (`bl` at 0xa9cbc). The absolute word assignment at `0x2b2868` writes `0x4` (`str` at 0xa9de4). Link
  capture effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9bc4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa9bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9bcc | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0xa9bd8 | load | 4 | `abs 0xa9e24` | `-` |
| `str` at 0xa9be0 | store | 4 | `abs 0x2b2868` | `0x1` |
| `ldr` at 0xa9bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9bf4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xa9bfc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0xa9c10 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa9c14 | load | 4 | `abs 0xa9e24` | `-` |
| `ldr` at 0xa9c18 | load | 4 | `abs 0x2b2868` | `-` |
| `ldr` at 0xa9c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9c28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9c2c | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xa9c40 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xa9c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9c48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9c5c | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xa9c64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa9c68 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa9c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9c84 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0xa9c8c | load | 1 | `(unknown)` | `-` |
| `str` at 0xa9c94 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa9c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9c9c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xa9cb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa9cd0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa9cd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa9ce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9ce4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9cec | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9cf4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9cfc | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d0c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9d14 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9d1c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9d24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d2c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9d34 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xa9d3c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9d40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d4c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa9d54 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0xa9d58 | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0xa9d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9d60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d68 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9d70 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa9d78 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9d88 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9d90 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa9d98 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9d9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9da8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xa9db0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xa9db8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xa9dbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9dc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9dc8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0xa9dd0 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xa9dd4 | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xa9ddc | load | 4 | `abs 0xa9e24` | `-` |
| `str` at 0xa9de4 | store | 4 | `abs 0x2b2868` | `0x4` |
| `ldr` at 0xa9de8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa9dec | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0xa9df0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9df4 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xa9df8 | load | 4 | `abs 0xa9e28` | `-` |
| `ldr` at 0xa9dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9e00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9e04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa9e0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa9e14 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xa9e20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9e20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9e20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9be8 | `(unknown callee) 0xe3cf8` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0xa9c08 | `(unknown callee) 0x1b4cc` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0xa9ca0 | `(unknown callee) 0x279d0` | `[[sp-20]+4]` | `[sp-20]` | `?` | `[sp-20]` |
| `bl` at 0xa9cbc | `(unknown callee) 0xeae04` | `ret(0x279d0)` | `[sp-40]` | `sp-48` | `sp-52` |
| `bl` at 0xa9e08 | `(unknown callee) 0x2079c` | `0x127a08` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0xa9e1c | `(unknown callee) 0xe3d64` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa9bd4 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0xa9c20 | `[0x2b2868] != 0x0` | `[0x2b2868] == 0x0` |
| `blt` at 0xa9c34 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa9cdc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9e20 | `r0` | `?` |
