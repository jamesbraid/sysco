# ARM monitor interrupts and environment events in 1.40.5

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
Computed transfers remain unresolved (`ldrls` at 0xce33c, 0xce3dc, 0xce748, 0xce9d8, 0x826ec). Known-edge tables exclude effects reached only through unknown destinations. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/monitor-environment-events-facts.txt) retain all tool sections.

## `monitorReadOutput`

- `monitorReadOutput`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce0b0 to 0xce2d0.
  Established. The unnamed calls to `0xc8998` carry unknown, `[[sp-32]+2]:1`, `[sp-37]:1`, `[sp-28]`
  (`bl` at 0xce244, `bl` at 0xce2b0). After the first call, the byte assignment at `[sp-28] +0x0`
  has unknown value (`strb` at 0xce260). Included four-byte stores at `sp -0x30` retain
  `0xfffffc11`, `0xfffff82e` and `0xfffff830` in separate paths (`str` at 0xce11c, `str` at 0xce178,
  `str` at 0xce274). Device-output meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce0c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xce0c4 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xce0cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xce0d4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xce0d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce0dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce0ec | store | 4 | `sp -0x20` | `ret(0xceaac)` |
| `ldrb` at 0xce0f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce0f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce104 | store | 4 | `sp -0x24` | `ret(0xcebcc)` |
| `ldr` at 0xce108 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xce10c | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xce118 | load | 4 | `abs 0xce2c4` | `-` |
| `str` at 0xce11c | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xce124 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce128 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xce12c | store | 4 | `sp -0x38` | `[[sp-32]]:1` |
| `ldr` at 0xce130 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xce13c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xce148 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xce158 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xce168 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xce174 | load | 4 | `abs 0xce2c8` | `-` |
| `str` at 0xce178 | store | 4 | `sp -0x30` | `0xfffff82e` |
| `strb` at 0xce184 | store | 1 | `sp -0x25` | `0x19` |
| `ldrb` at 0xce18c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xce190 | store | 4 | `sp -0x34` | `[sp-22]:1` |
| `ldr` at 0xce194 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce1a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce1ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xce1bc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xce1d0 | store | 1 | `sp -0x25` | `0x30` |
| `strb` at 0xce1dc | store | 1 | `sp -0x25` | `0x31` |
| `strb` at 0xce1e8 | store | 1 | `sp -0x25` | `0x32` |
| `ldr` at 0xce1f0 | load | 4 | `abs 0xce2c8` | `-` |
| `str` at 0xce1f4 | store | 4 | `sp -0x30` | `0xfffff82e` |
| `ldrb` at 0xce204 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xce20c | store | 1 | `sp -0x25` | `?` |
| `ldr` at 0xce210 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce214 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xce218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce22c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce230 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xce234 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xce240 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce24c | store | 4 | `sp -0x2c` | `ret(0xc8998)` |
| `ldr` at 0xce250 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce254 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce258 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xce260 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xce264 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xce268 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xce270 | load | 4 | `abs 0xce2cc` | `-` |
| `str` at 0xce274 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xce27c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce280 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldr` at 0xce284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce298 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce29c | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xce2a0 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xce2ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xce2b8 | store | 4 | `sp -0x30` | `ret(0xc8998)` |
| `ldr` at 0xce2bc | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xce2c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce2c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce2c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce0e4 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xce0fc | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xce220 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xce244 | `(unknown callee) 0xc8998` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |
| `bl` at 0xce28c | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-32]+1]:1` | `?` | `[[sp-32]+1]:1` |
| `bl` at 0xce2b0 | `(unknown callee) 0xc8998` | `?` | `[[sp-32]+2]:1` | `[sp-37]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce114 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `beq` at 0xce138 | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `bgt` at 0xce144 | `[[sp-32]]:1 > 0x2 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xce150 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xce160 | `[[sp-32]]:1 == 0x6` | `[[sp-32]]:1 != 0x6` |
| `beq` at 0xce170 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xce19c | `[sp-22]:1 == 0x1` | `[sp-22]:1 != 0x1` |
| `bgt` at 0xce1a8 | `[sp-22]:1 > 0x1 (signed, 32-bit)` | `[sp-22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xce1b4 | `[sp-22]:1 == 0x0` | `[sp-22]:1 != 0x0` |
| `beq` at 0xce1c4 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce2c0 | `r0` | `?` |

## `monitorVoltage`

- `monitorVoltage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce2d0 to 0xce484.
  Established. The unnamed call to `0xceaac` has candidates `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)`
  (`bl` at 0xce300); its return supplies a byte read at `ret(0xceaac) +0x0` (`ldrb` at 0xce310). Two
  conditional computed transfers are taken for `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` and `<= 0x8
  (unsigned, 32-bit)`, with unresolved destinations (`ldrls` at 0xce33c, `ldrls` at 0xce3dc). A
  known-edge four-byte assignment at `sp -0x20` retains `0xc2c80000` (`str` at 0xce448). Voltage
  units and selector-only effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce2e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce2e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xce2f0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xce2f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce2f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce308 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldr` at 0xce30c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce310 | load | 1 | `ret(0xceaac) +0x0` | `-` |
| `str` at 0xce314 | store | 4 | `sp -0x24` | `[ret(0xceaac)]:1` |
| `ldr` at 0xce318 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xce324 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xce334 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xce33c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce3ac | load | 1 | `ret(0xceaac) +0x3` | `-` |
| `ldrb` at 0xce3bc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xce3c8 | load | 4 | `abs 0xce468` | `-` |
| `str` at 0xce3cc | store | 4 | `sp -0x20` | `0x3b0ffb8b` |
| `ldrb` at 0xce3d4 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xce3dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce444 | load | 4 | `abs 0xce480` | `-` |
| `str` at 0xce448 | store | 4 | `sp -0x20` | `0xc2c80000` |
| `ldr` at 0xce44c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xce450 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce450 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce450 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce300 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xce320 | `[ret(0xceaac)]:1 == 0x1` | `[ret(0xceaac)]:1 != 0x1` |
| `beq` at 0xce32c | `[ret(0xceaac)]:1 == 0x2` | `[ret(0xceaac)]:1 != 0x2` |
| `ldrls` at 0xce33c | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `beq` at 0xce3b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xce3c4 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `ldrls` at 0xce3dc | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce450 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xce33c).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xce3dc).

## `monitorVoltageLimit`

- `monitorVoltageLimit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce484 to 0xce510.
  Established. The branch takes `[sp-24]:2 <= 0xfe (unsigned, 32-bit)` to `0xce4f0` (`bls` at
  0xce4e0); its fall-through stores halfword `0xfe` at `sp -0x18` (`strh` at 0xce4e8). The later
  zero fall-through of the branch for `[sp-24]:2 != 0x0` writes halfword `0x1` there (`bne` at
  0xce4f8, `strh` at 0xce500). The exit carries `[sp-24]:1` (`ldmdb` at 0xce50c). Voltage-limit
  units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce494 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce49c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xce4a0 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `ldrb` at 0xce4a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce4a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce4b8 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldr` at 0xce4bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce4c0 | load | 1 | `ret(0xceaac) +0x0` | `-` |
| `ldrh` at 0xce4cc | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xce4d4 | store | 2 | `sp -0x18` | `?` |
| `ldrh` at 0xce4d8 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xce4e8 | store | 2 | `sp -0x18` | `0xfe` |
| `ldrh` at 0xce4f0 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xce500 | store | 2 | `sp -0x18` | `0x1` |
| `ldrb` at 0xce504 | load | 1 | `sp -0x18` | `-` |
| `ldmdb` at 0xce50c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce50c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce50c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce4b0 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce4c8 | `[ret(0xceaac)]:1 != 0x2` | `[ret(0xceaac)]:1 == 0x2` |
| `bls` at 0xce4e0 | `[sp-24]:2 <= 0xfe (unsigned, 32-bit)` | `[sp-24]:2 > 0xfe (unsigned, 32-bit)` |
| `bne` at 0xce4f8 | `[sp-24]:2 != 0x0` | `[sp-24]:2 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce50c | `r0` | `[sp-24]:1` |

## `monitorIntCount`

- `monitorIntCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce510 to 0xce554.
  Established. The unnamed call to `0xcebcc` carries `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl`
  at 0xce538). A four-byte read at `ret(0xcebcc) +0x4` supplies return expression `[ret(0xcebcc)+4]`
  (`ldr` at 0xce548, `ldmdb` at 0xce550). The counted event is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce520 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce528 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xce52c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce530 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce540 | store | 4 | `sp -0x1c` | `ret(0xcebcc)` |
| `ldr` at 0xce544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce548 | load | 4 | `ret(0xcebcc) +0x4` | `-` |
| `ldmdb` at 0xce550 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce550 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce538 | `(unknown callee) 0xcebcc` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce550 | `r0` | `[ret(0xcebcc)+4]` |

## `monitorIntDisable`

- `monitorIntDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce554 to 0xce59c.
  Established. A call to unnamed `0xcf1c4` has candidates `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0`
  (`bl` at 0xce58c). The explicit byte stack stores retain `(r1 & 0xff)` and `(r2 & 0xff)` at `sp
  -0x15` and `-0x16` (`strb` at 0xce56c, `strb` at 0xce574). Interrupt-disable effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce564 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce56c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xce574 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xce578 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xce57c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xce580 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xce598 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce598 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce598 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce58c | `(unknown callee) 0xcf1c4` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce598 | `r0` | `ret(0xcf1c4)` |

## `monitorIntEnable`

- `monitorIntEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce59c to 0xce5e4.
  Established. The byte stack stores retain `(r1 & 0xff)` and `(r2 & 0xff)` at `sp -0x15` and
  `-0x16` (`strb` at 0xce5b4, `strb` at 0xce5bc). The unnamed call to `0xcf1c4` carries `r0`, `(r1 &
  0xff)`, `(r2 & 0xff)`, `0x1` (`bl` at 0xce5d4). The exit preserves `ret(0xcf1c4)` (`ldmdb` at
  0xce5e0). Interrupt-enable semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce5ac | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce5b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xce5bc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xce5c0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xce5c4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xce5c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xce5e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce5e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce5e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce5d4 | `(unknown callee) 0xcf1c4` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce5e0 | `r0` | `ret(0xcf1c4)` |

## `monitorInt`

- `monitorInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce6b0 to 0xce8f8.
  Established. The unnamed call to `0xcebcc` has candidates `[sp-20]`, `[sp-33]:1`, unknown,
  `[sp-33]:1` (`bl` at 0xce71c). The branch taken for `[ret(0xcebcc)]:1 == 0x0` reaches `0xce8c8`
  (`beq` at 0xce734). The conditional computed transfer is taken for `[[sp-28]]:1-1 <= 0x5
  (unsigned, 32-bit)` with unresolved destination (`ldrls` at 0xce748). Later four-byte assignments
  retain `([sp-40] + [sp-44])` at `sp -0x28` and `[[sp-32]+4]+1` at `[sp-32] +0x4` (`str` at
  0xce8b0, `str` at 0xce8c4). Interrupt-record and selector-only meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce6c0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce6c8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xce6d0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0xce6d8 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xce6dc | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xce6e0 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0xce6e4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xce6e8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xce6f8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xce6fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce70c | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldrb` at 0xce710 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xce714 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce724 | store | 4 | `sp -0x20` | `ret(0xcebcc)` |
| `ldr` at 0xce728 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce72c | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xce738 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce73c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xce748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce810 | load | 4 | `abs 0xce8e4` | `-` |
| `ldr` at 0xce814 | load | 4 | `abs 0xce8e8` | `-` |
| `str` at 0xce840 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xce844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xce84c | load | 4 | `abs 0xce8e4` | `-` |
| `ldr` at 0xce850 | load | 4 | `abs 0xce8e8` | `-` |
| `ldr` at 0xce85c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xce86c | load | 4 | `abs 0xce8ec` | `-` |
| `str` at 0xce870 | store | 4 | `(unknown)` | `0x1349b8` |
| `str` at 0xce874 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xce878 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce87c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xce880 | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0xce888 | load | 4 | `abs 0xce8f0` | `-` |
| `ldr` at 0xce88c | load | 4 | `abs 0xce8f4` | `-` |
| `ldr` at 0xce898 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xce8a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xce8a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xce8b0 | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0xce8b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce8b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xce8bc | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xce8c4 | store | 4 | `[sp-32] +0x4` | `[[sp-32]+4]+1` |
| `ldrb` at 0xce8c8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xce8d0 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0xce8d8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xce8e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce8e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce8e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce704 | `(unknown callee) 0xceaac` | `?` | `?` | `?` | `?` |
| `bl` at 0xce71c | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xce818 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xce824 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xce854 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xce864 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xce894 | `(unknown callee) 0x21eb8` | `0x9` | `0x1348cc` | `0x531` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xce6f0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xce734 | `[ret(0xcebcc)]:1 == 0x0` | `[ret(0xcebcc)]:1 != 0x0` |
| `ldrls` at 0xce748 | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xce8a0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce8e0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xce748).

## `monitorName`

- `monitorName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce8f8 to 0xce960.
  Established. The four-byte reads target `ret(0xceaac) +0x8` (`ldr` at 0xce930, `ldr` at 0xce93c).
  The branch takes `[ret(0xceaac)+8] != 0x0` to `0xce950` (`bne` at 0xce944). The zero fall-through
  stores four-byte `0x134a28` at `sp -0x20` (`str` at 0xce94c). The final return is unknown (`ldmdb`
  at 0xce958). Name-string semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce908 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce910 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xce914 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce918 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce928 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldr` at 0xce92c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce930 | load | 4 | `ret(0xceaac) +0x8` | `-` |
| `str` at 0xce934 | store | 4 | `sp -0x20` | `[ret(0xceaac)+8]` |
| `ldr` at 0xce938 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xce93c | load | 4 | `ret(0xceaac) +0x8` | `-` |
| `ldr` at 0xce948 | load | 4 | `abs 0xce95c` | `-` |
| `str` at 0xce94c | store | 4 | `sp -0x20` | `0x134a28` |
| `ldr` at 0xce950 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xce958 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce958 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce958 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce920 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce944 | `[ret(0xceaac)+8] != 0x0` | `[ret(0xceaac)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce958 | `r0` | `?` |

## `monitorTest`

- `monitorTest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xce960 to 0xceaac.
  Established. The byte read at `ret(0xcebcc) +0x0` precedes the branch taken for `[ret(0xcebcc)]:1
  != 0x0` (`ldrb` at 0xce9b0, `bne` at 0xce9b8). The other arm stores four-byte `0xfffffc11` at `sp
  -0x24` (`str` at 0xce9c0). A conditional computed transfer is taken for `[[sp-28]]:1-1 <= 0x5
  (unsigned, 32-bit)` with unknown destination (`ldrls` at 0xce9d8). Another known-edge four-byte
  assignment there retains `0xfffff830` (`str` at 0xcea8c). Test coverage and selector-only effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xce970 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xce978 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xce97c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce980 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce990 | store | 4 | `sp -0x1c` | `ret(0xceaac)` |
| `ldrb` at 0xce994 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce998 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce9a8 | store | 4 | `sp -0x20` | `ret(0xcebcc)` |
| `ldr` at 0xce9ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce9b0 | load | 1 | `ret(0xcebcc) +0x0` | `-` |
| `ldr` at 0xce9bc | load | 4 | `abs 0xcea98` | `-` |
| `str` at 0xce9c0 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xce9c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce9cc | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xce9d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcea88 | load | 4 | `abs 0xceaa8` | `-` |
| `str` at 0xcea8c | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0xcea90 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcea94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcea94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcea94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xce988 | `(unknown callee) 0xceaac` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xce9a0 | `(unknown callee) 0xcebcc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce9b8 | `[ret(0xcebcc)]:1 != 0x0` | `[ret(0xcebcc)]:1 == 0x0` |
| `ldrls` at 0xce9d8 | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcea94 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xce9d8).

## `monitorVSC055Int`

- `monitorVSC055Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd1d2c to 0xd1f18.
  Established. The byte stack assignments at `sp -0x21` and `sp -0x22` initially have unknown values
  (`strb` at 0xd1d78, `strb` at 0xd1d80). A later branch takes `[sp-33]:1 != 0xff` to `0xd1ec4`
  (`bne` at 0xd1e24). The unnamed call to `0x7e1d0` carries `[sp-20]`, `[sp-33]:1`, `[sp-34]:1`,
  `[sp-33]:1` (`bl` at 0xd1ef0). The exit carries `0x1` (`ldmdb` at 0xd1efc). Fan-interrupt routing
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd1d3c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd1d48 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd1d50 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xd1d58 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xd1d5c | load | 4 | `abs 0xd1f00` | `-` |
| `ldr` at 0xd1d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1d68 | load | 4 | `abs 0x2adf7c` | `-` |
| `str` at 0xd1d70 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xd1d78 | store | 1 | `sp -0x21` | `?` |
| `strb` at 0xd1d80 | store | 1 | `sp -0x22` | `?` |
| `str` at 0xd1d88 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd1d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1d90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1d94 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd1da4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1da8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1dbc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xd1dc4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd1dc8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd1dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd1dd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd1dec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xd1df4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd1df8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd1e04 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xd1e08 | store | 1 | `sp -0x21` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xd1e0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd1e14 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd1e1c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd1e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e30 | load | 4 | `abs 0xd1f04` | `-` |
| `ldr` at 0xd1e34 | load | 4 | `abs 0xd1f08` | `-` |
| `str` at 0xd1e60 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd1e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd1e6c | load | 4 | `abs 0xd1f04` | `-` |
| `ldr` at 0xd1e70 | load | 4 | `abs 0xd1f08` | `-` |
| `ldr` at 0xd1e7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd1e8c | load | 4 | `abs 0xd1f0c` | `-` |
| `str` at 0xd1e90 | store | 4 | `(unknown)` | `0x134bb4` |
| `str` at 0xd1e94 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd1e98 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd1e9c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xd1ea0 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd1ea4 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xd1ea8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0xd1eac | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xd1eb4 | load | 4 | `abs 0xd1f10` | `-` |
| `ldr` at 0xd1eb8 | load | 4 | `abs 0xd1f14` | `-` |
| `ldrb` at 0xd1ec4 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xd1edc | store | 1 | `sp -0x22` | `?` |
| `ldrb` at 0xd1ee0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd1ee4 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xd1ee8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd1efc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd1efc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd1efc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd1e38 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xd1e44 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd1e74 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x134924` | `0x13492c` |
| `bl` at 0xd1e84 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd1ec0 | `(unknown callee) 0x21eb8` | `0x1` | `0x1348cc` | `0xbc9` | `0x0` |
| `bl` at 0xd1ef0 | `(unknown callee) 0x7e1d0` | `[sp-20]` | `[sp-33]:1` | `[sp-34]:1` | `[sp-33]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd1d9c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd1dd0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd1e00 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd1e24 | `[sp-33]:1 != 0xff` | `[sp-33]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd1efc | `r0` | `0x1` |

## `EnvInit`

- `EnvInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7b1b0 to 0x7b6e8.
  Established. The four-byte absolute assignment at `0x149b6c` retains `[0x149b6c]+1` (`str` at
  0x7b264). A zero `[0x2af37c]` takes the branch past an unknown-target call with candidates
  `[sp-32]`, `0x149e64`, unknown, `[0x2af37c]` (`beq` at 0x7b498, `mov` at 0x7b4b0). Later four-byte
  stores at absolute `0x149e74`, `0x149e78`, `0x149e7c` and `0x149e8c` retain `ret(0x3cc10)`,
  `ret(0x46174)`, `ret(0x535b0)` and `ret(0xcd15c)` (`str` at 0x7b4d4, `str` at 0x7b534, `str` at
  0x7b5a0, `str` at 0x7b600). Environmental initialization and allocation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7b1c4 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x7b1d0 | load | 4 | `abs 0x7b60c` | `-` |
| `str` at 0x7b1e0 | store | 4 | `sp -0x2c` | `0x960` |
| `ldr` at 0x7b1e8 | load | 4 | `abs 0x7b610` | `-` |
| `str` at 0x7b1ec | store | 4 | `sp -0x2c` | `0x5dc` |
| `ldr` at 0x7b1f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7b1f4 | store | 4 | `sp -0x24` | `[sp-44]` |
| `ldr` at 0x7b200 | load | 4 | `abs 0x7b60c` | `-` |
| `str` at 0x7b210 | store | 4 | `sp -0x30` | `0x960` |
| `ldr` at 0x7b218 | load | 4 | `abs 0x7b610` | `-` |
| `str` at 0x7b21c | store | 4 | `sp -0x30` | `0x5dc` |
| `ldr` at 0x7b220 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7b224 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0x7b228 | load | 4 | `abs 0x7b614` | `-` |
| `ldr` at 0x7b22c | load | 4 | `abs 0x149b6c` | `-` |
| `ldr` at 0x7b238 | load | 4 | `abs 0x7b618` | `-` |
| `str` at 0x7b23c | store | 4 | `sp -0x44` | `0x111288` |
| `ldr` at 0x7b244 | load | 4 | `abs 0x7b61c` | `-` |
| `ldr` at 0x7b254 | load | 4 | `abs 0x7b614` | `-` |
| `ldr` at 0x7b258 | load | 4 | `abs 0x7b614` | `-` |
| `ldr` at 0x7b25c | load | 4 | `abs 0x149b6c` | `-` |
| `str` at 0x7b264 | store | 4 | `abs 0x149b6c` | `[0x149b6c]+1` |
| `ldr` at 0x7b268 | load | 4 | `abs 0x7b620` | `-` |
| `str` at 0x7b270 | store | 4 | `abs 0x149b84` | `0x0` |
| `ldr` at 0x7b274 | load | 4 | `abs 0x7b624` | `-` |
| `ldr` at 0x7b284 | load | 4 | `abs 0x7b628` | `-` |
| `ldr` at 0x7b288 | load | 4 | `abs 0x7b62c` | `-` |
| `ldr` at 0x7b298 | load | 4 | `abs 0x7b630` | `-` |
| `ldr` at 0x7b29c | load | 4 | `abs 0x7b634` | `-` |
| `ldr` at 0x7b2ac | load | 4 | `abs 0x7b638` | `-` |
| `ldr` at 0x7b2b0 | load | 4 | `abs 0x7b63c` | `-` |
| `ldr` at 0x7b2c0 | load | 4 | `abs 0x7b640` | `-` |
| `ldr` at 0x7b2c4 | load | 4 | `abs 0x7b644` | `-` |
| `ldr` at 0x7b2d4 | load | 4 | `abs 0x7b648` | `-` |
| `ldr` at 0x7b2d8 | load | 4 | `abs 0x7b64c` | `-` |
| `ldr` at 0x7b2e8 | load | 4 | `abs 0x7b650` | `-` |
| `ldr` at 0x7b2ec | load | 4 | `abs 0x7b654` | `-` |
| `ldr` at 0x7b2fc | load | 4 | `abs 0x7b658` | `-` |
| `ldr` at 0x7b300 | load | 4 | `abs 0x7b65c` | `-` |
| `ldr` at 0x7b310 | load | 4 | `abs 0x7b660` | `-` |
| `ldr` at 0x7b314 | load | 4 | `abs 0x7b664` | `-` |
| `str` at 0x7b328 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b32c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7b330 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7b338 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b340 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b348 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b34c | load | 4 | `abs 0x7b668` | `-` |
| `ldr` at 0x7b350 | load | 4 | `abs 0x7b66c` | `-` |
| `ldr` at 0x7b354 | load | 4 | `abs 0x7b670` | `-` |
| `ldr` at 0x7b358 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b364 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b368 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7b36c | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7b374 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b37c | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b384 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b388 | load | 4 | `abs 0x7b674` | `-` |
| `ldr` at 0x7b38c | load | 4 | `abs 0x7b678` | `-` |
| `ldr` at 0x7b390 | load | 4 | `abs 0x7b67c` | `-` |
| `ldr` at 0x7b394 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b3a0 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b3a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7b3a8 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7b3b0 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b3b8 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b3c0 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b3c4 | load | 4 | `abs 0x7b680` | `-` |
| `ldr` at 0x7b3c8 | load | 4 | `abs 0x7b684` | `-` |
| `ldr` at 0x7b3cc | load | 4 | `abs 0x7b688` | `-` |
| `ldr` at 0x7b3d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b3dc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b3e0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7b3e4 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7b3ec | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b3f4 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b3fc | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b400 | load | 4 | `abs 0x7b68c` | `-` |
| `ldr` at 0x7b404 | load | 4 | `abs 0x7b690` | `-` |
| `ldr` at 0x7b408 | load | 4 | `abs 0x7b694` | `-` |
| `ldr` at 0x7b40c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b418 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b41c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7b420 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7b428 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b430 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b438 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b43c | load | 4 | `abs 0x7b698` | `-` |
| `ldr` at 0x7b440 | load | 4 | `abs 0x7b69c` | `-` |
| `ldr` at 0x7b444 | load | 4 | `abs 0x7b6a0` | `-` |
| `ldr` at 0x7b448 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7b454 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7b458 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7b45c | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7b464 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7b46c | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7b474 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7b478 | load | 4 | `abs 0x7b6a4` | `-` |
| `ldr` at 0x7b47c | load | 4 | `abs 0x7b6a8` | `-` |
| `ldr` at 0x7b480 | load | 4 | `abs 0x7b6ac` | `-` |
| `ldr` at 0x7b484 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7b48c | load | 4 | `abs 0x7b6b0` | `-` |
| `ldr` at 0x7b490 | load | 4 | `abs 0x2af37c` | `-` |
| `ldr` at 0x7b49c | load | 4 | `abs 0x7b6b0` | `-` |
| `ldr` at 0x7b4a0 | load | 4 | `abs 0x2af37c` | `-` |
| `ldr` at 0x7b4a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7b4a8 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b4b4 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b4c4 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b4cc | load | 4 | `abs 0x7b6b8` | `-` |
| `str` at 0x7b4d4 | store | 4 | `abs 0x149e74` | `ret(0x3cc10)` |
| `ldr` at 0x7b4d8 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b4dc | load | 4 | `abs 0x149e74` | `-` |
| `ldr` at 0x7b4e8 | load | 4 | `abs 0x7b6bc` | `-` |
| `str` at 0x7b4ec | store | 4 | `sp -0x44` | `0x11134c` |
| `ldr` at 0x7b4f4 | load | 4 | `abs 0x7b61c` | `-` |
| `ldr` at 0x7b4f8 | load | 4 | `abs 0x7b6c0` | `-` |
| `ldr` at 0x7b508 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b50c | load | 4 | `abs 0x149e74` | `-` |
| `ldr` at 0x7b514 | load | 4 | `abs 0x7b6c4` | `-` |
| `ldr` at 0x7b524 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b52c | load | 4 | `abs 0x7b6c8` | `-` |
| `str` at 0x7b534 | store | 4 | `abs 0x149e78` | `ret(0x46174)` |
| `ldr` at 0x7b538 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b53c | load | 4 | `abs 0x149e78` | `-` |
| `ldr` at 0x7b548 | load | 4 | `abs 0x7b6cc` | `-` |
| `str` at 0x7b54c | store | 4 | `sp -0x44` | `0x111370` |
| `ldr` at 0x7b554 | load | 4 | `abs 0x7b61c` | `-` |
| `ldr` at 0x7b558 | load | 4 | `abs 0x7b6d0` | `-` |
| `ldr` at 0x7b568 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b56c | load | 4 | `abs 0x149e78` | `-` |
| `ldr` at 0x7b580 | load | 4 | `abs 0x7b6d4` | `-` |
| `ldr` at 0x7b590 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b598 | load | 4 | `abs 0x7b6d8` | `-` |
| `str` at 0x7b5a0 | store | 4 | `abs 0x149e7c` | `ret(0x535b0)` |
| `ldr` at 0x7b5a4 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b5a8 | load | 4 | `abs 0x149e7c` | `-` |
| `ldr` at 0x7b5b4 | load | 4 | `abs 0x7b6dc` | `-` |
| `str` at 0x7b5b8 | store | 4 | `sp -0x44` | `0x11138c` |
| `ldr` at 0x7b5c0 | load | 4 | `abs 0x7b61c` | `-` |
| `ldr` at 0x7b5d4 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b5d8 | load | 4 | `abs 0x149e7c` | `-` |
| `ldr` at 0x7b5e0 | load | 4 | `abs 0x7b6e0` | `-` |
| `ldr` at 0x7b5f0 | load | 4 | `abs 0x7b6b4` | `-` |
| `ldr` at 0x7b5f8 | load | 4 | `abs 0x7b6e4` | `-` |
| `str` at 0x7b600 | store | 4 | `abs 0x149e8c` | `ret(0xcd15c)` |
| `ldmdb` at 0x7b608 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7b608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7b608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7b608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7b1c8 | `(unknown callee) 0x82e7c` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x7b1f8 | `(unknown callee) 0x82e7c` | `ret(0x82e7c)` | `?` | `ret(0x82e7c)` | `[sp-44]` |
| `bl` at 0x7b250 | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0x164` | `0x0` |
| `bl` at 0x7b280 | `(unknown callee) 0xb5e34` | `0x149ba4` | `0x4` | `0x0` | `0x0` |
| `bl` at 0x7b294 | `(unknown callee) 0x229b0` | `0x149bac` | `0x1112a4` | `0x0` | `0x9` |
| `bl` at 0x7b2a8 | `(unknown callee) 0x229b0` | `0x149c00` | `0x1112b0` | `0x0` | `0x9` |
| `bl` at 0x7b2bc | `(unknown callee) 0x229b0` | `0x149c54` | `0x1112bc` | `0x0` | `0x9` |
| `bl` at 0x7b2d0 | `(unknown callee) 0x229b0` | `0x149ca8` | `0x1112c8` | `0x0` | `0x9` |
| `bl` at 0x7b2e4 | `(unknown callee) 0x229b0` | `0x149cfc` | `0x1112d4` | `0x0` | `0x9` |
| `bl` at 0x7b2f8 | `(unknown callee) 0x229b0` | `0x149d50` | `0x1112e0` | `0x0` | `0x9` |
| `bl` at 0x7b30c | `(unknown callee) 0x229b0` | `0x149da4` | `0x1112ec` | `0x1` | `0x9` |
| `bl` at 0x7b320 | `(unknown callee) 0x229b0` | `0x149df8` | `0x1112f8` | `0x1` | `0x9` |
| `bl` at 0x7b35c | `(unknown callee) 0x22480` | `0x149e4c` | `0x111304` | `0x7e8e0` | `[sp-32]` |
| `bl` at 0x7b398 | `(unknown callee) 0x22480` | `0x149e50` | `0x111310` | `0x7f288` | `[sp-32]` |
| `bl` at 0x7b3d4 | `(unknown callee) 0x22480` | `0x149e54` | `0x11131c` | `0x7f7bc` | `[sp-32]` |
| `bl` at 0x7b410 | `(unknown callee) 0x22480` | `0x149e58` | `0x111328` | `0x80080` | `[sp-32]` |
| `bl` at 0x7b44c | `(unknown callee) 0x22480` | `0x149e5c` | `0x111334` | `0x8038c` | `[sp-32]` |
| `bl` at 0x7b488 | `(unknown callee) 0x22480` | `0x149e60` | `0x111340` | `0x806d0` | `[sp-32]` |
| `mov` at 0x7b4b0 | `(unknown callee) (unknown target)` | `[sp-32]` | `0x149e64` | `?` | `[0x2af37c]` |
| `bl` at 0x7b4c0 | `(unknown callee) 0x1b4cc` | `0x149e64` | `0x0` | `0x4c0` | `?` |
| `bl` at 0x7b4d0 | `(unknown callee) 0x3cc10` | `0x0` | `0x149e80` | `?` | `?` |
| `bl` at 0x7b500 | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0x1a2` | `0x0` |
| `bl` at 0x7b520 | `(unknown callee) 0x1b4cc` | `0x149e94` | `0x0` | `?` | `?` |
| `bl` at 0x7b530 | `(unknown callee) 0x46174` | `0x0` | `0x149e84` | `?` | `?` |
| `bl` at 0x7b560 | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0x1ad` | `0x0` |
| `bl` at 0x7b58c | `(unknown callee) 0x1b4cc` | `0x14a000` | `0x0` | `?` | `?` |
| `bl` at 0x7b59c | `(unknown callee) 0x535b0` | `0x0` | `0x149e88` | `?` | `?` |
| `bl` at 0x7b5cc | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0x1b8` | `0x0` |
| `bl` at 0x7b5ec | `(unknown callee) 0x1b4cc` | `0x14a2d4` | `0x0` | `?` | `?` |
| `bl` at 0x7b5fc | `(unknown callee) 0xcd15c` | `0x0` | `0x149e90` | `?` | `?` |
| `bl` at 0x7b604 | `(unknown callee) 0x829f0` | `ret(0xcd15c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x7b1d8 | `ret(0x82e7c) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x82e7c) > 0x7ffff (unsigned, 32-bit)` |
| `bls` at 0x7b208 | `ret(0x82e7c) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x82e7c) > 0x7ffff (unsigned, 32-bit)` |
| `beq` at 0x7b234 | `[0x149b6c] == 0x0` | `[0x149b6c] != 0x0` |
| `beq` at 0x7b498 | `[0x2af37c] == 0x0` | `[0x2af37c] != 0x0` |
| `ble` at 0x7b4e4 | `[0x149e74] <= 0x2a (signed, 32-bit)` | `[0x149e74] > 0x2a (signed, 32-bit)` |
| `ble` at 0x7b544 | `[0x149e78] <= 0x14 (signed, 32-bit)` | `[0x149e78] > 0x14 (signed, 32-bit)` |
| `ble` at 0x7b5b0 | `[0x149e7c] <= 0x14 (signed, 32-bit)` | `[0x149e7c] > 0x14 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7b608 | `r0` | `ret(0x829f0)` |

## `EnvEvent`

- `EnvEvent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7aad8 to 0x7b1b0.
  Established. The unnamed call to `0x22ac8` carries `0x149da4`, unknown, unknown, `ret(0x4f4d0)`
  (`bl` at 0x7ab0c); its zero result takes the branch to `0x7ab48` (`beq` at 0x7ab20). Included
  four-byte stores at absolute `0x149b74` have unknown values (`str` at 0x7ace4, `str` at 0x7adbc).
  A later unnamed call to `0x22e50` uses `0x149da4` and three unknown candidates (`bl` at 0x7ae80).
  The final call to unnamed `0x75afc` carries `[sp-244]` and three unknown candidates (`bl` at
  0x7b170). Event protocol and status-bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7aae8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x7aaec | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x7aaf0 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x7aaf4 | store | 4 | `sp -0x28` | `r3` |
| `strb` at 0x7ab00 | store | 1 | `sp -0xf5` | `(ret(0x4f4d0) & 0xff)` |
| `ldr` at 0x7ab04 | load | 4 | `abs 0x7b178` | `-` |
| `str` at 0x7ab14 | store | 4 | `sp -0xf0` | `ret(0x22ac8)` |
| `ldr` at 0x7ab18 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x7ab24 | load | 4 | `abs 0x7b17c` | `-` |
| `str` at 0x7ab28 | store | 4 | `sp -0x104` | `0x111224` |
| `ldr` at 0x7ab2c | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x7ab30 | store | 4 | `sp -0x100` | `ret(0x22ac8)` |
| `ldr` at 0x7ab38 | load | 4 | `abs 0x7b180` | `-` |
| `ldr` at 0x7ab48 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x7ab54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ab64 | load | 4 | `abs 0x7b184` | `-` |
| `ldrb` at 0x7ab9c | load | 1 | `sp -0xf5` | `-` |
| `ldr` at 0x7abbc | load | 4 | `abs 0x7b188` | `-` |
| `ldr` at 0x7abcc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7abdc | load | 4 | `abs 0x7b184` | `-` |
| `ldr` at 0x7abe4 | load | 4 | `abs 0x7b18c` | `-` |
| `ldr` at 0x7abe8 | load | 4 | `abs 0x149b78` | `-` |
| `ldr` at 0x7abf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7ac14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7ac2c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7ac58 | store | 4 | `sp -0x104` | `0x1` |
| `str` at 0x7ac74 | store | 4 | `sp -0x104` | `0x0` |
| `ldr` at 0x7ac78 | load | 4 | `abs 0x7b190` | `-` |
| `ldr` at 0x7ac80 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x7ac90 | store | 4 | `sp -0xf0` | `ret(0xb764c)` |
| `ldr` at 0x7ac94 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7acb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7acc0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7acd0 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7acd4 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7acd8 | load | 4 | `abs 0x149b74` | `-` |
| `ldr` at 0x7acdc | load | 4 | `sp +0x4` | `-` |
| `str` at 0x7ace4 | store | 4 | `abs 0x149b74` | `?` |
| `ldr` at 0x7ace8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ad04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ad10 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ad20 | load | 4 | `abs 0x7b198` | `-` |
| `ldr` at 0x7ad24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ad30 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ad40 | load | 4 | `abs 0x7b19c` | `-` |
| `ldr` at 0x7ad44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ad50 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ad60 | load | 4 | `abs 0x7b1a0` | `-` |
| `ldr` at 0x7ad64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ad70 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ad80 | load | 4 | `abs 0x7b1a4` | `-` |
| `ldr` at 0x7ad84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ad90 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7ada0 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7ada4 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7ada8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7adb4 | load | 4 | `abs 0x149b74` | `-` |
| `str` at 0x7adbc | store | 4 | `abs 0x149b74` | `?` |
| `ldr` at 0x7adc4 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7adc8 | load | 4 | `abs 0x149b74` | `-` |
| `ldr` at 0x7addc | load | 4 | `abs 0x7b198` | `-` |
| `ldr` at 0x7adec | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7adf0 | load | 4 | `abs 0x149b74` | `-` |
| `ldr` at 0x7ae04 | load | 4 | `abs 0x7b19c` | `-` |
| `ldr` at 0x7ae14 | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7ae18 | load | 4 | `abs 0x149b74` | `-` |
| `ldr` at 0x7ae2c | load | 4 | `abs 0x7b1a0` | `-` |
| `ldr` at 0x7ae3c | load | 4 | `abs 0x7b194` | `-` |
| `ldr` at 0x7ae40 | load | 4 | `abs 0x149b74` | `-` |
| `ldr` at 0x7ae54 | load | 4 | `abs 0x7b1a4` | `-` |
| `ldr` at 0x7ae70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ae7c | load | 4 | `abs 0x7b178` | `-` |
| `str` at 0x7ae88 | store | 4 | `sp -0xf0` | `ret(0x22e50)` |
| `ldr` at 0x7ae8c | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x7ae98 | load | 4 | `abs 0x7b1a8` | `-` |
| `str` at 0x7ae9c | store | 4 | `sp -0x104` | `0x111264` |
| `ldr` at 0x7aea0 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x7aea4 | store | 4 | `sp -0x100` | `ret(0x22e50)` |
| `ldr` at 0x7aeac | load | 4 | `abs 0x7b180` | `-` |
| `ldr` at 0x7aeb0 | load | 4 | `abs 0x7b1ac` | `-` |
| `ldr` at 0x7aec8 | load | 4 | `ret(0x75194) +0x5c` | `-` |
| `str` at 0x7aecc | store | 4 | `sp -0xf4` | `[ret(0x75194)+92]` |
| `ldr` at 0x7aed0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7aed8 | store | 4 | `sp -0xfc` | `?` |
| `ldr` at 0x7aedc | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7aee8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7aef4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af00 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af0c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af18 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af24 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af30 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af40 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af4c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af5c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af68 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af74 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af80 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af90 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7af9c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afac | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afb8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afc4 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afd0 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afdc | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7afe8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7aff8 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b004 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b014 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b020 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b02c | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b038 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b048 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b054 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b060 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b070 | load | 4 | `sp -0xfc` | `-` |
| `ldr` at 0x7b080 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b088 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b090 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b098 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0a0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0a8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0b0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0b8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0c0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0c8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0d0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0d8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0e0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0e8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b0f0 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b0f8 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b100 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b108 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b110 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b118 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b120 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b128 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b130 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b138 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b140 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b148 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b150 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b158 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b160 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7b168 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7b16c | load | 4 | `sp -0xf4` | `-` |
| `ldmdb` at 0x7b174 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7b174 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7b174 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7b174 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7b174 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7aaf8 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x7ab0c | `(unknown callee) 0x22ac8` | `0x149da4` | `?` | `?` | `ret(0x4f4d0)` |
| `bl` at 0x7ab44 | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0xb1` | `0x0` |
| `bl` at 0x7ab58 | `(unknown callee) 0x202d0` | `[sp-40]` | `?` | `?` | `[sp]` |
| `bl` at 0x7ab68 | `(unknown callee) 0x1b598` | `sp-236` | `0x111258` | `?` | `sp-236` |
| `bl` at 0x7ab78 | `(unknown callee) 0x1b2bc` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x7ab88 | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `ret(0x1b2bc)` |
| `bl` at 0x7ab94 | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x7abb0 | `(unknown callee) 0x6a5c8` | `(sp-236 + ret(0x1b2bc))` | `ret(0x1f26c)` | `[sp-245]:1` | `ret(0x1f26c)` |
| `bl` at 0x7abc0 | `(unknown callee) 0x1b364` | `sp-236` | `0x11125c` | `?` | `sp-236` |
| `bl` at 0x7abd0 | `(unknown callee) 0x1b364` | `sp-236` | `[sp-40]` | `?` | `sp-236` |
| `bl` at 0x7abe0 | `(unknown callee) 0x1b364` | `sp-236` | `0x111258` | `?` | `sp-236` |
| `bl` at 0x7ac18 | `(unknown callee) 0x69908` | `sp-84` | `sp-80` | `[sp-32]` | `sp-84` |
| `bl` at 0x7ac30 | `(unknown callee) 0x69964` | `sp-84` | `sp-80` | `[sp-36]` | `sp-84` |
| `bl` at 0x7ac3c | `(unknown callee) 0x1b2bc` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x7ac68 | `(unknown callee) 0x69b84` | `sp-84` | `sp-80` | `sp-236` | `ret(0x1b2bc)` |
| `bl` at 0x7ac88 | `(unknown callee) 0xb764c` | `0x149ba4` | `0x0` | `[sp-84]` | `sp-80` |
| `bl` at 0x7aca8 | `(unknown callee) 0x52048` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x7acb8 | `(unknown callee) 0x51b28` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x7acfc | `(unknown callee) 0x52048` | `0x1` | `0x149b74` | `[0x149b74]` | `?` |
| `bl` at 0x7ad0c | `(unknown callee) 0x51b28` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x7ad28 | `(unknown callee) 0x1b598` | `0x14a324` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7ad48 | `(unknown callee) 0x1b598` | `0x14a374` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7ad68 | `(unknown callee) 0x1b598` | `0x14a3c4` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7ad88 | `(unknown callee) 0x1b598` | `0x14a414` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7adc0 | `(unknown callee) 0x52014` | `0x149b74` | `0x149b74` | `?` | `?` |
| `bl` at 0x7ade4 | `(unknown callee) 0x51b28` | `0x32` | `0x14a324` | `0x2` | `?` |
| `bl` at 0x7ae0c | `(unknown callee) 0x51b28` | `0x32` | `0x14a374` | `0x2` | `?` |
| `bl` at 0x7ae34 | `(unknown callee) 0x51b28` | `0x32` | `0x14a3c4` | `0x2` | `?` |
| `bl` at 0x7ae5c | `(unknown callee) 0x51b28` | `0x32` | `0x14a414` | `0x2` | `?` |
| `bl` at 0x7ae68 | `(unknown callee) 0x52154` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x7ae78 | `(unknown callee) 0x51b28` | `0x32` | `[sp-40]` | `0x1` | `?` |
| `bl` at 0x7ae80 | `(unknown callee) 0x22e50` | `0x149da4` | `?` | `?` | `?` |
| `bl` at 0x7aeb8 | `(unknown callee) 0x21eb8` | `0x9` | `0x111244` | `0x119` | `0x0` |
| `bl` at 0x7aec0 | `(unknown callee) 0x75194` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7b170 | `(unknown callee) 0x75afc` | `[sp-244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7ab20 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0x7ab50 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x7abf0 | `[0x149b78] == 0x0` | `[0x149b78] != 0x0` |
| `bgt` at 0x7abfc | `[sp-28] > 0x1 (signed, 32-bit)` | `[sp-28] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x7aca0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7accc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7acf4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ad1c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ad3c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ad5c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ad7c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ad9c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7add4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7adfc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ae24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ae4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7ae94 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0x7aee4 | `[sp-252] == 0x3300` | `[sp-252] != 0x3300` |
| `bhi` at 0x7aef0 | `[sp-252] > 0x3300 (unsigned, 32-bit)` | `[sp-252] <= 0x3300 (unsigned, 32-bit)` |
| `beq` at 0x7aefc | `[sp-252] == 0x2000` | `[sp-252] != 0x2000` |
| `bhi` at 0x7af08 | `[sp-252] > 0x2000 (unsigned, 32-bit)` | `[sp-252] <= 0x2000 (unsigned, 32-bit)` |
| `beq` at 0x7af14 | `[sp-252] == 0x1200` | `[sp-252] != 0x1200` |
| `bhi` at 0x7af20 | `[sp-252] > 0x1200 (unsigned, 32-bit)` | `[sp-252] <= 0x1200 (unsigned, 32-bit)` |
| `beq` at 0x7af2c | `[sp-252] == 0x1000` | `[sp-252] != 0x1000` |
| `beq` at 0x7af38 | `[sp-252] == 0x1100` | `[sp-252] != 0x1100` |
| `beq` at 0x7af48 | `[sp-252] == 0x1300` | `[sp-252] != 0x1300` |
| `beq` at 0x7af54 | `[sp-252] == 0x1400` | `[sp-252] != 0x1400` |
| `beq` at 0x7af64 | `[sp-252] == 0x3000` | `[sp-252] != 0x3000` |
| `bhi` at 0x7af70 | `[sp-252] > 0x3000 (unsigned, 32-bit)` | `[sp-252] <= 0x3000 (unsigned, 32-bit)` |
| `beq` at 0x7af7c | `[sp-252] == 0x2100` | `[sp-252] != 0x2100` |
| `beq` at 0x7af88 | `[sp-252] == 0x2200` | `[sp-252] != 0x2200` |
| `beq` at 0x7af98 | `[sp-252] == 0x3100` | `[sp-252] != 0x3100` |
| `beq` at 0x7afa4 | `[sp-252] == 0x3200` | `[sp-252] != 0x3200` |
| `beq` at 0x7afb4 | `[sp-252] == 0x6000` | `[sp-252] != 0x6000` |
| `bhi` at 0x7afc0 | `[sp-252] > 0x6000 (unsigned, 32-bit)` | `[sp-252] <= 0x6000 (unsigned, 32-bit)` |
| `beq` at 0x7afcc | `[sp-252] == 0x4000` | `[sp-252] != 0x4000` |
| `bhi` at 0x7afd8 | `[sp-252] > 0x4000 (unsigned, 32-bit)` | `[sp-252] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x7afe4 | `[sp-252] == 0x3400` | `[sp-252] != 0x3400` |
| `beq` at 0x7aff0 | `[sp-252] == 0x3500` | `[sp-252] != 0x3500` |
| `beq` at 0x7b000 | `[sp-252] == 0x4100` | `[sp-252] != 0x4100` |
| `beq` at 0x7b00c | `[sp-252] == 0x5000` | `[sp-252] != 0x5000` |
| `beq` at 0x7b01c | `[sp-252] == 0x7200` | `[sp-252] != 0x7200` |
| `bhi` at 0x7b028 | `[sp-252] > 0x7200 (unsigned, 32-bit)` | `[sp-252] <= 0x7200 (unsigned, 32-bit)` |
| `beq` at 0x7b034 | `[sp-252] == 0x7000` | `[sp-252] != 0x7000` |
| `beq` at 0x7b040 | `[sp-252] == 0x7100` | `[sp-252] != 0x7100` |
| `beq` at 0x7b050 | `[sp-252] == 0x7400` | `[sp-252] != 0x7400` |
| `bhi` at 0x7b05c | `[sp-252] > 0x7400 (unsigned, 32-bit)` | `[sp-252] <= 0x7400 (unsigned, 32-bit)` |
| `beq` at 0x7b068 | `[sp-252] == 0x7300` | `[sp-252] != 0x7300` |
| `beq` at 0x7b078 | `[sp-252] == 0x7500` | `[sp-252] != 0x7500` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7b174 | `r0` | `ret(0x75afc)` |

## `EnvState`

- `EnvState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8107c to 0x81164.
  Established. Four-byte stores at absolute `0x149e68` include `0x1` and `0x0` in separate arms
  (`str` at 0x810f8, `str` at 0x81118). Assignments at absolute `0x149e6c` include `0x0` and `0x1`
  in those arms (`str` at 0x81104, `str` at 0x81124). Each arm contains a call to unnamed `0x829f0`
  with first two candidates unknown and final two `0x149e64`, `0x0` or `0x149e64`, `0x1`,
  respectively (`bl` at 0x81108, `bl` at 0x81128). State labels and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8108c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x81090 | load | 4 | `abs 0x81158` | `-` |
| `ldr` at 0x81094 | load | 4 | `abs 0x149b70` | `-` |
| `ldr` at 0x810a4 | load | 4 | `abs 0x8115c` | `-` |
| `ldr` at 0x810a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x810b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x810b4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x810b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x810c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x810d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x810e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x810f0 | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x810f8 | store | 4 | `abs 0x149e68` | `0x1` |
| `ldr` at 0x810fc | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x81104 | store | 4 | `abs 0x149e6c` | `0x0` |
| `ldr` at 0x81110 | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x81118 | store | 4 | `abs 0x149e68` | `0x0` |
| `ldr` at 0x8111c | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x81124 | store | 4 | `abs 0x149e6c` | `0x1` |
| `ldr` at 0x81130 | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x81138 | store | 4 | `abs 0x149e68` | `0x0` |
| `ldr` at 0x81140 | load | 4 | `abs 0x81160` | `-` |
| `str` at 0x81148 | store | 4 | `abs 0x149e6c` | `0x0` |
| `ldmdb` at 0x81154 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x81154 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x81154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x810ac | `(unknown callee) 0x23fbc` | `0x111afc` | `r0` | `r2` | `?` |
| `bl` at 0x81108 | `(unknown callee) 0x829f0` | `?` | `?` | `0x149e64` | `0x0` |
| `bl` at 0x81128 | `(unknown callee) 0x829f0` | `?` | `?` | `0x149e64` | `0x1` |
| `bl` at 0x8113c | `(unknown callee) 0x829f0` | `?` | `?` | `0x149e64` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x810a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x810c0 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x810cc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x810d8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x810e8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x81154 | `r0` | `0x0` |

## `EnvQuery`

- `EnvQuery`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82548 to 0x828fc.
  Established. Four-byte reads target absolute `0x149e74`, `0x149e78` and `0x149e7c` (`ldr` at
  0x825a4, `ldr` at 0x825a8, `ldr` at 0x825b4). The four-byte assignment at `[sp-24] +0x0` retains
  `ret(0x1b1b4)` (`str` at 0x825dc). The branch takes `[[sp-24]] != 0x0` to `0x82608` (`bne` at
  0x825e8). A conditional computed transfer is taken for `[0x149e70] <= 0x3 (unsigned, 32-bit)` with
  unresolved destination (`ldrls` at 0x826ec). Query-format and selector-only effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82558 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8255c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x82578 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x82594 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x82598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8259c | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x825a0 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x825a4 | load | 4 | `abs 0x149e74` | `-` |
| `ldr` at 0x825a8 | load | 4 | `abs 0x149e78` | `-` |
| `ldr` at 0x825b0 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x825b4 | load | 4 | `abs 0x149e7c` | `-` |
| `str` at 0x825dc | store | 4 | `[sp-24] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0x825e0 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x825ec | load | 4 | `abs 0x828bc` | `-` |
| `ldr` at 0x825fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82600 | store | 4 | `[sp-24] +0x0` | `ret(0x1b034)` |
| `ldr` at 0x82608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8260c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x82610 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0x82614 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x8261c | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x82620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8262c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82630 | load | 4 | `abs 0x828c0` | `-` |
| `ldr` at 0x82638 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8263c | load | 4 | `abs 0x828c4` | `-` |
| `ldr` at 0x82644 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82648 | load | 4 | `abs 0x828c8` | `-` |
| `ldr` at 0x82650 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8265c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82660 | load | 4 | `abs 0x828cc` | `-` |
| `ldr` at 0x82668 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8266c | load | 4 | `abs 0x828d0` | `-` |
| `ldr` at 0x82674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82678 | load | 4 | `abs 0x828d4` | `-` |
| `ldr` at 0x82680 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82684 | load | 4 | `abs 0x828d8` | `-` |
| `ldr` at 0x82690 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82694 | load | 4 | `abs 0x828dc` | `-` |
| `ldr` at 0x8269c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x826ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x826b4 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x826b8 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x826bc | load | 4 | `abs 0x149e64` | `-` |
| `str` at 0x826c0 | store | 4 | `sp -0x2c` | `[0x149e64]` |
| `ldr` at 0x826c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x826d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x826e0 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x826e4 | load | 4 | `abs 0x149e70` | `-` |
| `ldrls` at 0x826ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x82744 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x82748 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8274c | load | 4 | `abs 0x828f0` | `-` |
| `ldr` at 0x82750 | load | 4 | `abs 0x149e70` | `-` |
| `ldr` at 0x8275c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82760 | load | 4 | `abs 0x828f4` | `-` |
| `ldr` at 0x8276c | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x82770 | load | 4 | `abs 0x828b8` | `-` |
| `ldr` at 0x82774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82778 | load | 4 | `abs 0x828f8` | `-` |
| `ldr` at 0x8277c | load | 4 | `abs 0x149e64` | `-` |
| `ldr` at 0x82780 | load | 4 | `abs 0x149e70` | `-` |
| `ldr` at 0x82788 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82798 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x827a0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x827a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x827b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x827b8 | load | 4 | `abs 0x828e0` | `-` |
| `ldr` at 0x827c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x827d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x827d8 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x827dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x827e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x827f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x827fc | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x82800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82808 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82818 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x82820 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x82824 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8282c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8283c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x82844 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0x82848 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x82854 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82858 | load | 4 | `abs 0x828c0` | `-` |
| `ldr` at 0x82860 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82864 | load | 4 | `abs 0x828c4` | `-` |
| `ldr` at 0x8286c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82870 | load | 4 | `abs 0x828c8` | `-` |
| `ldr` at 0x82878 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x82884 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82888 | load | 4 | `abs 0x828cc` | `-` |
| `ldr` at 0x82890 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x82894 | load | 4 | `abs 0x828d0` | `-` |
| `ldr` at 0x8289c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x828a0 | load | 4 | `abs 0x828d4` | `-` |
| `ldr` at 0x828a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x828ac | load | 4 | `abs 0x828d8` | `-` |
| `ldmdb` at 0x828b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x828b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x828b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x828b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82564 | `(unknown callee) 0x1f26c` | `0x6` | `r1` | `r2` | `r3` |
| `bl` at 0x82580 | `(unknown callee) 0x1f26c` | `0x7` | `?` | `?` | `?` |
| `bl` at 0x825d4 | `(unknown callee) 0x1b1b4` | `?` | `[0x149e74]` | `(([0x149e74] + [0x149e78]) + [0x149e7c])` | `?` |
| `bl` at 0x825f0 | `(unknown callee) 0x1b034` | `0x111f54` | `?` | `?` | `[[sp-24]]` |
| `bl` at 0x82634 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x111f84` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x82640 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x111fb8` | `?` | `?` |
| `bl` at 0x8264c | `(unknown callee) 0x1b364` | `[sp-40]` | `0x111fec` | `?` | `?` |
| `bl` at 0x82664 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112020` | `?` | `[sp-36]` |
| `bl` at 0x82670 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112054` | `?` | `?` |
| `bl` at 0x8267c | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112088` | `?` | `?` |
| `bl` at 0x82688 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x1120b8` | `?` | `?` |
| `bl` at 0x82698 | `(unknown callee) 0x1b598` | `[sp-40]` | `0x1120ec` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x826a0 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x82754 | `(unknown callee) 0x1b52c` | `([sp-40] + ret(0x1b2bc))` | `0x112190` | `[0x149e70]` | `0x149e64` |
| `bl` at 0x82764 | `(unknown callee) 0x1b598` | `([sp-40] + ret(0x1b2bc))` | `0x1121c0` | `ret(0x1b2bc)` | `[0x149e64]` |
| `bl` at 0x82784 | `(unknown callee) 0x1b52c` | `([sp-40] + ret(0x1b2bc))` | `0x1121d0` | `[0x149e64]` | `[0x149e70]` |
| `bl` at 0x8278c | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x827bc | `(unknown callee) 0x1b364` | `([sp-40] + ret(0x1b2bc))` | `0x111c5c` | `ret(0x1b2bc)` | `[sp-28]` |
| `bl` at 0x827c4 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x827e0 | `(unknown callee) 0x81164` | `([sp-40] + ret(0x1b2bc))` | `?` | `ret(0x1b2bc)` | `([sp-40] + ret(0x1b2bc))` |
| `bl` at 0x827e8 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x82804 | `(unknown callee) 0x81668` | `([sp-40] + ret(0x1b2bc))` | `?` | `ret(0x1b2bc)` | `([sp-40] + ret(0x1b2bc))` |
| `bl` at 0x8280c | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x82828 | `(unknown callee) 0x81920` | `([sp-40] + ret(0x1b2bc))` | `?` | `ret(0x1b2bc)` | `([sp-40] + ret(0x1b2bc))` |
| `bl` at 0x82830 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x8285c | `(unknown callee) 0x1b364` | `([sp-40] + ret(0x1b2bc))` | `0x111f84` | `ret(0x1b2bc)` | `[sp-32]` |
| `bl` at 0x82868 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x111fb8` | `?` | `?` |
| `bl` at 0x82874 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x111fec` | `?` | `?` |
| `bl` at 0x8288c | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112020` | `?` | `[sp-36]` |
| `bl` at 0x82898 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112054` | `?` | `?` |
| `bl` at 0x828a4 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x112088` | `?` | `?` |
| `bl` at 0x828b0 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x1120b8` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x825e8 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0x82628 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x82658 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x826cc | `[0x149e64] == 0x0` | `[0x149e64] != 0x0` |
| `beq` at 0x826d8 | `[0x149e64] == 0x1` | `[0x149e64] != 0x1` |
| `ldrls` at 0x826ec | `[0x149e70] <= 0x3 (unsigned, 32-bit)` | `[0x149e70] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x827ac | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x82850 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x82880 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x828b4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x826ec).

## `EnvMessages`

- `EnvMessages`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82db4 to 0x82e7c.
  Established. The branch takes entry `r0 == 0x1` to `0x82e30` (`beq` at 0x82dd8), reaching a
  four-byte `0x1` assignment at absolute `0x149b78` (`str` at 0x82e38). The later equality branch
  takes `r0 == 0x2` to `0x82e48` (`beq` at 0x82e00), reaching a four-byte `0x0` store there (`str`
  at 0x82e50). The exit carries `[sp-24]` (`ldmdb` at 0x82e64). Message-enable and result-string
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82dc4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82dc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x82dcc | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x82dd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82ddc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82de8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82df8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82e08 | load | 4 | `abs 0x82e68` | `-` |
| `ldr` at 0x82e0c | load | 4 | `abs 0x149b78` | `-` |
| `ldr` at 0x82e18 | load | 4 | `abs 0x82e6c` | `-` |
| `str` at 0x82e1c | store | 4 | `sp -0x18` | `0x1122ac` |
| `ldr` at 0x82e24 | load | 4 | `abs 0x82e70` | `-` |
| `str` at 0x82e28 | store | 4 | `sp -0x18` | `0x1122d0` |
| `ldr` at 0x82e30 | load | 4 | `abs 0x82e68` | `-` |
| `str` at 0x82e38 | store | 4 | `abs 0x149b78` | `0x1` |
| `ldr` at 0x82e3c | load | 4 | `abs 0x82e74` | `-` |
| `str` at 0x82e40 | store | 4 | `sp -0x18` | `0x1122f4` |
| `ldr` at 0x82e48 | load | 4 | `abs 0x82e68` | `-` |
| `str` at 0x82e50 | store | 4 | `abs 0x149b78` | `0x0` |
| `ldr` at 0x82e54 | load | 4 | `abs 0x82e78` | `-` |
| `str` at 0x82e58 | store | 4 | `sp -0x18` | `0x112320` |
| `ldr` at 0x82e60 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x82e64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82e64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x82dd8 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0x82de4 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x82df0 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x82e00 | `r0 == 0x2` | `r0 != 0x2` |
| `beq` at 0x82e14 | `[0x149b78] == 0x0` | `[0x149b78] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82e64 | `r0` | `[sp-24]` |
