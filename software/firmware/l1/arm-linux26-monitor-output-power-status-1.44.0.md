# ARM Linux 2.6 monitor output and power status in 1.44.0

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
Computed transfers in monitorReadOutput, monitorWriteOutput, monitorVoltage, monitorInt,
monitorTest and monitorPSMICfgStatus leave unresolved edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-monitor-output-power-status-facts.txt) retain all tool sections.

## `monitorReadOutput`

- `monitorReadOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x113a3c to 0x113d20.
  Established. The candidate r0 values at the two calls are entry r0 and the saved word at sp-0x14 respectively (`bl` at 0x113a70, `bl` at 0x113a88).
  Known-edge word stores put 0xfffffc11 and 0xfffff830 at sp-0x34 (`str` at 0x113aa8, `str` at 0x113cc0).
  The computed transfer is unresolved (`ldrls` at 0x113ac0), so omitted effects and the output-read mechanism remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x113a4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x113a50 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x113a58 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x113a60 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x113a64 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113a68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113a78 | store | 4 | `sp -0x20` | `ret(0x114714)` |
| `ldrb` at 0x113a7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113a80 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113a90 | store | 4 | `sp -0x24` | `ret(0x114838)` |
| `ldr` at 0x113a94 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x113a98 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x113aa4 | load | 4 | `abs 0x113d14` | `-` |
| `str` at 0x113aa8 | store | 4 | `sp -0x34` | `0xfffffc11` |
| `ldr` at 0x113ab0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x113ab4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0x113ac0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113cbc | load | 4 | `abs 0x113d1c` | `-` |
| `str` at 0x113cc0 | store | 4 | `sp -0x34` | `0xfffff830` |
| `ldr` at 0x113d08 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x113d10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x113d10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x113d10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x113a70 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x113a88 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x113aa0 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x113ac0 | `[[sp-32]]:1-1 <= 0x7 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x113d10 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x113ac0).

## `monitorWriteOutput`

- `monitorWriteOutput`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11366c to 0x113a3c.
  Established. Entry r1, r2 and r3 are reduced to bytes and saved in stack fields (`strb` at 0x113688, `strb` at 0x113690, `strb` at 0x113698).
  Word stores put 0xfffffc11 and 0xfffff830 at sp-0x2c (`str` at 0x1136e0, `str` at 0x1139d4).
  The transfer is unresolved (`ldrls` at 0x1136f8), leaving omitted effects and any output-write destination unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11367c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x113688 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x113690 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x113698 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x11369c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1136a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1136b0 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x1136b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1136b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1136c8 | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x1136cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1136d0 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x1136dc | load | 4 | `abs 0x113a30` | `-` |
| `str` at 0x1136e0 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0x1136e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1136ec | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x1136f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1139d0 | load | 4 | `abs 0x113a38` | `-` |
| `str` at 0x1139d4 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0x113a24 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x113a2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x113a2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x113a2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1136a8 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x1136c0 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1136d8 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x1136f8 | `[[sp-28]]:1-1 <= 0x7 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x7 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x113a2c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1136f8).

## `monitorVoltage`

- `monitorVoltage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x113d20 to 0x113fd0.
  Established. Byte loads use the result of unnamed 0x114714 at offsets zero and three (`ldrb` at 0x113d60, `ldrb` at 0x113e18).
  Known-edge word stores put 0x3b0ffb8b and 0xc2c80000 at sp-0x20, followed by a coprocessor load from that stack location whose width and result are unknown (`str` at 0x113e38, `str` at 0x113f74, `ldc` at 0x113f78).
  Transfers at (`ldrls` at 0x113da8, `ldrls` at 0x113e48, `ldrls` at 0x113ebc) remain unresolved, so omitted effects and physical units are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x113d30 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x113d38 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x113d40 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x113d44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113d48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x113d58 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldr` at 0x113d5c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x113d60 | load | 1 | `ret(0x114714) +0x0` | `-` |
| `str` at 0x113d64 | store | 4 | `sp -0x24` | `[ret(0x114714)]:1` |
| `ldr` at 0x113d68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x113d74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x113d80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x113d90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x113da0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x113da8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113e14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x113e18 | load | 1 | `ret(0x114714) +0x3` | `-` |
| `ldrb` at 0x113e28 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x113e34 | load | 4 | `abs 0x113f98` | `-` |
| `str` at 0x113e38 | store | 4 | `sp -0x20` | `0x3b0ffb8b` |
| `ldrb` at 0x113e40 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x113e48 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x113eb0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0x113ebc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x113f70 | load | 4 | `abs 0x113fcc` | `-` |
| `str` at 0x113f74 | store | 4 | `sp -0x20` | `0xc2c80000` |
| `ldc` at 0x113f78 | load | unknown | `sp -0x20` | `-` |
| `ldm` at 0x113f80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x113f80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x113f80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x113d50 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x113d70 | `[ret(0x114714)]:1 == 0x2` | `[ret(0x114714)]:1 != 0x2` |
| `bgt` at 0x113d7c | `[ret(0x114714)]:1 > 0x2 (signed, 32-bit)` | `[ret(0x114714)]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0x113d88 | `[ret(0x114714)]:1 == 0x1` | `[ret(0x114714)]:1 != 0x1` |
| `beq` at 0x113d98 | `[ret(0x114714)]:1 == 0x7` | `[ret(0x114714)]:1 != 0x7` |
| `ldrls` at 0x113da8 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `beq` at 0x113e24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x113e30 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `ldrls` at 0x113e48 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |
| `ldrls` at 0x113ebc | `[sp-22]:1-7 <= 0xc (unsigned, 32-bit)` | `[sp-22]:1-7 > 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x113f80 | `r0` | `ret(0x114714)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x113da8).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x113e48).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x113ebc).

## `monitorVoltageLimit`

- `monitorVoltageLimit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x113fd0 to 0x114088.
  Established. The halfword store at sp-0x18 initially receives entry r2 masked to 16 bits (`strh` at 0x113fec).
  Later halfword stores can replace that field with 0xfe or one, and the recorded return is the word loaded from sp-0x20 (`strh` at 0x114054, `strh` at 0x11406c, `ldm` at 0x114084).
  The transformed values do not establish voltage units or a physical limit register.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x113fe0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x113fe8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0x113fec | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `ldrb` at 0x113ff0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x113ff4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x114004 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldr` at 0x114008 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11400c | load | 1 | `ret(0x114714) +0x0` | `-` |
| `ldrh` at 0x114018 | load | 2 | `sp -0x18` | `-` |
| `str` at 0x114020 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x114028 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11402c | load | 1 | `ret(0x114714) +0x0` | `-` |
| `ldrh` at 0x114038 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0x114040 | store | 2 | `sp -0x18` | `?` |
| `ldrh` at 0x114044 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0x114054 | store | 2 | `sp -0x18` | `0xfe` |
| `ldrh` at 0x11405c | load | 2 | `sp -0x18` | `-` |
| `strh` at 0x11406c | store | 2 | `sp -0x18` | `0x1` |
| `ldrh` at 0x114070 | load | 2 | `sp -0x18` | `-` |
| `str` at 0x114078 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x11407c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x114084 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x114084 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x114084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x113ffc | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x114014 | `[ret(0x114714)]:1 != 0x9` | `[ret(0x114714)]:1 == 0x9` |
| `bne` at 0x114034 | `[ret(0x114714)]:1 != 0x2` | `[ret(0x114714)]:1 == 0x2` |
| `bls` at 0x11404c | `[sp-24]:2 <= 0xfe (unsigned, 32-bit)` | `[sp-24]:2 > 0xfe (unsigned, 32-bit)` |
| `bne` at 0x114064 | `[sp-24]:2 != 0x0` | `[sp-24]:2 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x114084 | `r0` | `[sp-32]` |

## `monitorIntCount`

- `monitorIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x114088 to 0x1140d0.
  Established. The call to unnamed 0x114838 precedes a word load eight bytes beyond its result (`bl` at 0x1140b0, `ldr` at 0x1140c0).
  The recorded return carries that loaded word (`ldm` at 0x1140cc);
  its count interpretation and the object's hardware relationship remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x114098 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1140a0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x1140a4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1140a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1140b8 | store | 4 | `sp -0x1c` | `ret(0x114838)` |
| `ldr` at 0x1140bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1140c0 | load | 4 | `ret(0x114838) +0x8` | `-` |
| `ldm` at 0x1140cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1140cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1140cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1140b0 | `(unknown callee) 0x114838` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1140cc | `r0` | `[ret(0x114838)+8]` |

## `monitorIntDisable`

- `monitorIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1140d0 to 0x11411c.
  Established. The call to unnamed 0x114f34 has candidate values entry r0, entry r1 masked to a byte, entry r2 masked to a byte and zero (`bl` at 0x114108).
  The recorded return carries that call result (`ldm` at 0x114118).
  The callee's effects and any interrupt-disable mechanism are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1140e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1140e8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x1140f0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x1140f4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x1140f8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1140fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x114118 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x114118 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x114118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x114108 | `(unknown callee) 0x114f34` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x114118 | `r0` | `ret(0x114f34)` |

## `monitorIntEnable`

- `monitorIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11411c to 0x114168.
  Established. The candidate registers at the call to unnamed 0x114f34 contain entry r0, the low bytes of entry r1 and r2, and one (`bl` at 0x114154).
  The recorded exit retains the call result (`ldm` at 0x114164).
  These values do not identify the controlled interrupt or establish an enable register.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11412c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x114134 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x11413c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x114140 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x114144 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x114148 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x114164 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x114164 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x114164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x114154 | `(unknown callee) 0x114f34` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x114164 | `r0` | `ret(0x114f34)` |

## `monitorInt`

- `monitorInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x114238 to 0x1144ec.
  Established. Known-edge word stores add the fields loaded from sp-0x28 and sp-0x2c, then increment the word at offset eight of the pointer loaded from sp-0x20 (`str` at 0x1144a8, `str` at 0x1144bc).
  A byte store advances the field at sp-0x21 modulo 256 (`strb` at 0x1144c8).
  The transfer is unresolved (`ldrls` at 0x1142cc), leaving omitted effects unknown, and the updated fields alone do not establish physical interrupt accounting.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x114248 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x114250 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x114258 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x114260 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0x114264 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x114268 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0x11426c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x114270 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x11427c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x114280 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x114290 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x114294 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x114298 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1142a8 | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x1142ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1142b0 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x1142bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1142c0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x1142cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1143f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11440c | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x114410 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x114420 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x114424 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11444c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x114450 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x114454 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x114458 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x114464 | load | 4 | `abs 0x1144e0` | `-` |
| `str` at 0x114468 | store | 4 | `(unknown)` | `0x18e0d0` |
| `str` at 0x11446c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x114470 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x114474 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x114478 | store | 4 | `(unknown)` | `[[sp-28]]:1` |
| `ldr` at 0x114480 | load | 4 | `abs 0x1144e4` | `-` |
| `ldr` at 0x114484 | load | 4 | `abs 0x1144e8` | `-` |
| `ldr` at 0x114490 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11449c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1144a0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1144a8 | store | 4 | `sp -0x28` | `([sp-40] + [sp-44])` |
| `ldr` at 0x1144ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1144b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1144b4 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0x1144bc | store | 4 | `[sp-32] +0x8` | `[[sp-32]+8]+1` |
| `ldrb` at 0x1144c0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x1144c8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x1144d0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x1144dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1144dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1144dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x114288 | `(unknown callee) 0x114714` | `?` | `?` | `?` | `?` |
| `bl` at 0x1142a0 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0x114408 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x114414 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11445c | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x11448c | `(unknown callee) 0x22070` | `0x9` | `0x18dff0` | `0x8fb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x114278 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x1142b8 | `[ret(0x114838)]:1 == 0x0` | `[ret(0x114838)]:1 != 0x0` |
| `ldrls` at 0x1142cc | `[[sp-28]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x8 (unsigned, 32-bit)` |
| `beq` at 0x114498 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1144dc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1142cc).

## `monitorName`

- `monitorName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1144ec to 0x114558.
  Established. A word load reads offset eight of the result of unnamed 0x114714 and saves the value in a stack field (`ldr` at 0x114524, `str` at 0x114528).
  The second word load from that address supplies the zero test (`ldr` at 0x114530).
  When the branch falls through because that loaded word equals zero, a word store puts 0x18e140 at sp-0x20 (`bne` at 0x114538, `str` at 0x114540).
  The recorded return is unknown, so the returned name and the referenced storage's contents remain unestablished (`ldm` at 0x114550).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1144fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x114504 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x114508 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11450c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11451c | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldr` at 0x114520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x114524 | load | 4 | `ret(0x114714) +0x8` | `-` |
| `str` at 0x114528 | store | 4 | `sp -0x20` | `[ret(0x114714)+8]` |
| `ldr` at 0x11452c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x114530 | load | 4 | `ret(0x114714) +0x8` | `-` |
| `ldr` at 0x11453c | load | 4 | `abs 0x114554` | `-` |
| `str` at 0x114540 | store | 4 | `sp -0x20` | `0x18e140` |
| `ldr` at 0x114544 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x114550 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x114550 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x114550 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x114514 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x114538 | `[ret(0x114714)+8] != 0x0` | `[ret(0x114714)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x114550 | `r0` | `?` |

## `monitorTest`

- `monitorTest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x114558 to 0x114714.
  Established. The known-edge word stores at sp-0x24 receive 0xfffffc11 and 0xfffff830 (`str` at 0x1145b8, `str` at 0x1146e8);
  its recorded return is unknown (`ldm` at 0x1146f4).
  The computed transfer is unresolved (`ldrls` at 0x1145d0), leaving omitted effects and the actual test operation unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x114568 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x114570 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x114574 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x114578 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x114588 | store | 4 | `sp -0x1c` | `ret(0x114714)` |
| `ldrb` at 0x11458c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x114590 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1145a0 | store | 4 | `sp -0x20` | `ret(0x114838)` |
| `ldr` at 0x1145a4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x1145a8 | load | 1 | `ret(0x114838) +0x0` | `-` |
| `ldr` at 0x1145b4 | load | 4 | `abs 0x1146f8` | `-` |
| `str` at 0x1145b8 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x1145c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1145c4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x1145d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1146e4 | load | 4 | `abs 0x114710` | `-` |
| `str` at 0x1146e8 | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0x1146ec | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x1146f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1146f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1146f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x114580 | `(unknown callee) 0x114714` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x114598 | `(unknown callee) 0x114838` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1145b0 | `[ret(0x114838)]:1 != 0x0` | `[ret(0x114838)]:1 == 0x0` |
| `ldrls` at 0x1145d0 | `[[sp-28]]:1-1 <= 0x8 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1146f4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1145d0).

## `monitorVSC055Int`

- `monitorVSC055Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1180dc to 0x118314.
  Established. Two computed byte loads have unknown effective addresses (`ldrb` at 0x1181a8, `ldrb` at 0x1181d8).
  The call to unnamed 0xac7e4 has candidate values unknown, the bytes loaded from sp-0x1d and sp-0x1e, and unknown (`bl` at 0x1182e8).
  A following word store puts one at sp-0x34, but the recorded exit reads that field and does not establish the physical interrupt source (`str` at 0x1182f0, `ldm` at 0x1182fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1180ec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1180f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x118100 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x118108 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x118110 | store | 1 | `sp -0x1d` | `?` |
| `strb` at 0x118118 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x11811c | load | 4 | `abs 0x118300` | `-` |
| `ldr` at 0x118120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x118124 | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x118130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x118138 | load | 4 | `abs 0x118304` | `-` |
| `ldr` at 0x11813c | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x118144 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x118154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11815c | load | 4 | `abs 0x118304` | `-` |
| `ldr` at 0x118160 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x118168 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x118170 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x118174 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x118178 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x11817c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x118188 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11818c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1181a0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x1181a8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x1181ac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1181b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1181bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1181d0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x1181d8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x1181dc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1181e8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x1181ec | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0x1181f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1181f8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x118200 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11820c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x118220 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x118224 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x118234 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x118238 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x118260 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x118264 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x118268 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11826c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x118278 | load | 4 | `abs 0x118308` | `-` |
| `str` at 0x11827c | store | 4 | `(unknown)` | `0x18e2cc` |
| `str` at 0x118280 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x118284 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x118288 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0x11828c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x118290 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0x118294 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x118298 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0x1182a0 | load | 4 | `abs 0x11830c` | `-` |
| `ldr` at 0x1182a4 | load | 4 | `abs 0x118310` | `-` |
| `ldrb` at 0x1182b0 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x1182c8 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x1182cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1182d4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x1182d8 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x1182f0 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x1182f4 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x1182fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1182fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1182fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11821c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x118228 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x118270 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x1182ac | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x10d1` | `0x0` |
| `bl` at 0x1182e8 | `(unknown callee) 0xac7e4` | `?` | `[sp-29]:1` | `[sp-30]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x11812c | `[sp-20] >= [0x314fa4] (signed, 32-bit)` | `[sp-20] < [0x314fa4] (signed, 32-bit)` |
| `bne` at 0x11814c | `? != 0x0` | `? == 0x0` |
| `ble` at 0x118184 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x1181b4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1181e4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x118208 | `[sp-29]:1 != 0xff` | `[sp-29]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1182fc | `r0` | `[sp-52]` |

## `monitorPSMIDump`

- `monitorPSMIDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11a154 to 0x11a448.
  Established. Byte stores select 0x3e and 0x41 or 0x5e and 0x63 in the fields at sp-0x1f and sp-0x20 (`strb` at 0x11a18c, `strb` at 0x11a194, `strb` at 0x11a1d4, `strb` at 0x11a1dc).
  A halfword load from sp-0x22 is saved as a word at sp-0x7c (`ldrh` at 0x11a2f4, `str` at 0x11a2f8).
  The later call to unnamed 0x243d4 has candidate values 0x18e784, a byte at offset two of the pointer loaded from sp-0x1c, its word at offset eight and the byte at sp-0x1e (`bl` at 0x11a404).
  Physical status-bit meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11a164 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11a168 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x11a170 | store | 1 | `sp -0x1d` | `(r2 & 0xff)` |
| `ldrb` at 0x11a174 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11a180 | load | 4 | `abs 0x11a42c` | `-` |
| `strb` at 0x11a18c | store | 1 | `sp -0x1f` | `0x3e` |
| `strb` at 0x11a194 | store | 1 | `sp -0x20` | `0x41` |
| `ldrb` at 0x11a198 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11a1a4 | load | 4 | `abs 0x11a430` | `-` |
| `strb` at 0x11a1b0 | store | 1 | `sp -0x1f` | `0x0` |
| `strb` at 0x11a1b8 | store | 1 | `sp -0x20` | `?` |
| `ldrb` at 0x11a1bc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11a1c8 | load | 4 | `abs 0x11a434` | `-` |
| `strb` at 0x11a1d4 | store | 1 | `sp -0x1f` | `0x5e` |
| `strb` at 0x11a1dc | store | 1 | `sp -0x20` | `0x63` |
| `ldrb` at 0x11a1e0 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x11a1e4 | store | 1 | `sp -0x1e` | `([sp-31]:1 & 0xff)` |
| `ldrb` at 0x11a1e8 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x11a1ec | load | 1 | `sp -0x20` | `-` |
| `ldrb` at 0x11a1f8 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x11a200 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11a204 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11a218 | store | 4 | `sp -0x28` | `ret(0x11f61c)` |
| `ldr` at 0x11a21c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11a228 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11a23c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x11a240 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x11a250 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x11a254 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x11a27c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x11a280 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11a284 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11a288 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11a294 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11a2a0 | load | 4 | `abs 0x11a438` | `-` |
| `str` at 0x11a2a4 | store | 4 | `(unknown)` | `0x18e1b8` |
| `ldrb` at 0x11a2a8 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x11a2ac | store | 4 | `(unknown)` | `[sp-30]:1` |
| `str` at 0x11a2b0 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11a2b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11a2b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11a2bc | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11a2c0 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11a2c8 | load | 4 | `abs 0x11a43c` | `-` |
| `ldr` at 0x11a2cc | load | 4 | `abs 0x11a440` | `-` |
| `ldr` at 0x11a2d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x11a2dc | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x11a2e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11a2e8 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldr` at 0x11a2ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11a2f0 | load | 1 | `sp -0x1e` | `-` |
| `ldrh` at 0x11a2f4 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a2f8 | store | 4 | `sp -0x7c` | `[sp-34]:2` |
| `ldrsh` at 0x11a2fc | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a304 | store | 4 | `sp -0x78` | `?` |
| `ldrh` at 0x11a308 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a314 | store | 4 | `sp -0x74` | `?` |
| `ldrh` at 0x11a318 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a324 | store | 4 | `sp -0x70` | `?` |
| `ldrh` at 0x11a328 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a334 | store | 4 | `sp -0x6c` | `?` |
| `ldrh` at 0x11a338 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a344 | store | 4 | `sp -0x68` | `?` |
| `ldrh` at 0x11a348 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a354 | store | 4 | `sp -0x64` | `?` |
| `ldrh` at 0x11a358 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a364 | store | 4 | `sp -0x60` | `?` |
| `ldrh` at 0x11a368 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a374 | store | 4 | `sp -0x5c` | `?` |
| `ldrh` at 0x11a378 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a384 | store | 4 | `sp -0x58` | `?` |
| `ldrh` at 0x11a388 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a394 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x11a398 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3a4 | store | 4 | `sp -0x50` | `?` |
| `ldrh` at 0x11a3a8 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3b4 | store | 4 | `sp -0x4c` | `?` |
| `ldrh` at 0x11a3b8 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3c4 | store | 4 | `sp -0x48` | `?` |
| `ldrh` at 0x11a3c8 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3d4 | store | 4 | `sp -0x44` | `?` |
| `ldrh` at 0x11a3d8 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3e4 | store | 4 | `sp -0x40` | `?` |
| `ldrh` at 0x11a3e8 | load | 2 | `sp -0x22` | `-` |
| `str` at 0x11a3f0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x11a3f4 | load | 4 | `abs 0x11a444` | `-` |
| `ldr` at 0x11a3fc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x11a408 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x11a410 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `str` at 0x11a41c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x11a420 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x11a428 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11a428 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11a428 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11a428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11a184 | `(unknown callee) 0x243d4` | `0x18e720` | `r1` | `r2` | `(r2 & 0xff)` |
| `bl` at 0x11a1a8 | `(unknown callee) 0x243d4` | `0x18e744` | `?` | `?` | `?` |
| `bl` at 0x11a1cc | `(unknown callee) 0x243d4` | `0x18e764` | `?` | `?` | `?` |
| `bl` at 0x11a210 | `(unknown callee) 0x11f61c` | `?` | `?` | `?` | `sp-34` |
| `bl` at 0x11a238 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11a244 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11a28c | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x11a298 | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x11a2d4 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x150a` | `0x0` |
| `bl` at 0x11a404 | `(unknown callee) 0x243d4` | `0x18e784` | `[[sp-28]+2]:1` | `[[sp-28]+8]` | `[sp-30]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11a17c | `(r2 & 0xff) != 0x0` | `(r2 & 0xff) == 0x0` |
| `bne` at 0x11a1a0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x11a1c4 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x11a1f4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x11a224 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11a428 | `r0` | `?` |

## `monitorPSMICfgStatus`

- `monitorPSMICfgStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11a448 to 0x11b65c.
  Established. Byte stores put 0x5e and 0x63 at sp-0x22 and sp-0x23 (`strb` at 0x11a468, `strb` at 0x11a470).
  The call to unnamed 0x11f61c has candidate values the words at sp-0x18 and sp-0x1c, zero and the address sp-0x26 (`bl` at 0x11a524).
  The computed transfer is unresolved (`ldrls` at 0x11a784), so omitted effects and later configuration-status interpretations remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11a458 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11a45c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11a460 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0x11a468 | store | 1 | `sp -0x22` | `0x5e` |
| `strb` at 0x11a470 | store | 1 | `sp -0x23` | `0x63` |
| `ldr` at 0x11a474 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11a488 | store | 4 | `sp -0x80` | `ret(0x6e1b0)` |
| `ldr` at 0x11a48c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x11a49c | store | 4 | `sp -0x84` | `ret(0x2031c)+1` |
| `ldr` at 0x11a4a0 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x11a4c8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x11a4cc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x11a4d0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x11a4d4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x11a4e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11a4e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11a4e8 | load | 4 | `abs 0x11b4b8` | `-` |
| `ldr` at 0x11a4f0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x11a4f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11a508 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11a510 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11a518 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11a51c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11a52c | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11a530 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11a53c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11a548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11a54c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11a550 | load | 4 | `abs 0x11b4bc` | `-` |
| `ldr` at 0x11a554 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11a564 | store | 4 | `sp -0x8c` | `0x0` |
| `ldrb` at 0x11a56c | load | 1 | `sp -0x22` | `-` |
| `strb` at 0x11a570 | store | 1 | `sp -0x21` | `([sp-34]:1 & 0xff)` |
| `ldrb` at 0x11a574 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x11a578 | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0x11a584 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11a58c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11a590 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11a5a4 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11a5a8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11a5b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11a5c8 | store | 4 | `sp -0x88` | `ret(0x6e1b0)` |
| `ldr` at 0x11a5cc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x11a5dc | store | 4 | `sp -0x84` | `ret(0x2031c)+1` |
| `ldr` at 0x11a5e0 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x11a608 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0x11a60c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x11a610 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x11a614 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x11a620 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11a62c | load | 4 | `abs 0x11b4c0` | `-` |
| `str` at 0x11a630 | store | 4 | `(unknown)` | `0x18e1b8` |
| `ldrb` at 0x11a634 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11a638 | store | 4 | `(unknown)` | `[sp-33]:1` |
| `str` at 0x11a63c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11a640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11a644 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11a648 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11a64c | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11a654 | load | 4 | `abs 0x11b4c4` | `-` |
| `ldr` at 0x11a658 | load | 4 | `abs 0x11b4c8` | `-` |
| `ldr` at 0x11a664 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11a668 | store | 4 | `sp -0x8c` | `[sp-124]` |
| `ldrsh` at 0x11a674 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11a67c | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11a688 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a694 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a698 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6a4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6a8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6b4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6b8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6c4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6c8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6d4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6d8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6e4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6e8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a6f4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a6f8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a704 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a708 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a714 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a718 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a724 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a728 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a734 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a738 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a744 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a748 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a754 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11a758 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11a760 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11a768 | load | 4 | `abs 0x11b4cc` | `-` |
| `ldrb` at 0x11a778 | load | 1 | `sp -0x21` | `-` |
| `ldrls` at 0x11a784 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x11b640 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11b648 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11b650 | load | 4 | `sp -0x8c` | `-` |
| `ldm` at 0x11b658 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11b658 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11b658 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11b658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11a484 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11a490 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11a4d8 | `(unknown callee) 0x203f4` | `[sp-136]` | `[sp-128]` | `[sp-132]` | `?` |
| `bl` at 0x11a4f4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18e7e4` | `ret(0x203f4)` | `[[sp-28]+8]` |
| `bl` at 0x11a4fc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11a524 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x0` | `sp-38` |
| `bl` at 0x11a540 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `ret(0x11f61c)` |
| `bl` at 0x11a55c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18e7ec` | `[[sp-28]+8]` | `ret(0x219a4)` |
| `bl` at 0x11a59c | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11a5c4 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11a5d0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11a618 | `(unknown callee) 0x203f4` | `[sp-128]` | `[sp-136]` | `[sp-132]` | `?` |
| `bl` at 0x11a624 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11a660 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x1544` | `0x0` |
| `bl` at 0x11a774 | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11a538 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x11a580 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x11a5b0 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `ldrls` at 0x11a784 | `[sp-33]:1-94 <= 0x5 (unsigned, 32-bit)` | `[sp-33]:1-94 > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11b658 | `r0` | `[sp-140]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x11a784).

## `monitorPSMIPwrStatus`

- `monitorPSMIPwrStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11b65c to 0x11d0fc.
  Established. Known-edge word stores put four at sp-0x80 and sp-0x84, and ten at sp-0x88 and sp-0x8c (`str` at 0x11b8a4, `str` at 0x11b8ac, `str` at 0x11b9d4, `str` at 0x11b9dc).
  The call to unnamed 0x11f61c has candidate values the words at sp-0x18 and sp-0x1c, 0x32 and the address sp-0x26 (`bl` at 0x11c3a0).
  Coprocessor stores at resolved stack locations have unknown widths and values, preventing a supported numeric or physical power interpretation (`stc` at 0x11bc98, `stc` at 0x11bca8, `stc` at 0x11bccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11b66c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11b670 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11b674 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x11b678 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11b68c | store | 4 | `sp -0xb4` | `ret(0x6e1b0)` |
| `ldr` at 0x11b690 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x11b6a0 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11b6a4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11b6cc | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x11b6d0 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11b6d4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11b6d8 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11b6e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11b6e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11b6ec | load | 4 | `abs 0x11c00c` | `-` |
| `ldr` at 0x11b6f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x11b6fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11b70c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11b714 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11b71c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11b720 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11b730 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11b734 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b740 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b74c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11b750 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11b754 | load | 4 | `abs 0x11c010` | `-` |
| `ldr` at 0x11b758 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11b768 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x11b774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11b778 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11b788 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11b78c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b798 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b7a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11b7a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11b7ac | load | 4 | `abs 0x11c010` | `-` |
| `ldr` at 0x11b7b0 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11b7c0 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x11b7cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11b7d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11b7e0 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11b7e4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b7f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11b804 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11b808 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11b818 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11b81c | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11b844 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11b848 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11b84c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11b850 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11b85c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b868 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11b86c | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11b874 | store | 4 | `(unknown)` | `0x6` |
| `str` at 0x11b878 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11b87c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11b880 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11b884 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11b888 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11b890 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11b894 | load | 4 | `abs 0x11c014` | `-` |
| `str` at 0x11b8a4 | store | 4 | `sp -0x80` | `0x4` |
| `str` at 0x11b8ac | store | 4 | `sp -0x84` | `0x4` |
| `ldrh` at 0x11b8b4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11b8c0 | store | 4 | `sp -0x80` | `?` |
| `ldrh` at 0x11b8c4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11b8cc | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x11b8d0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x11b8e0 | store | 4 | `sp -0x80` | `0x4` |
| `ldr` at 0x11b8e4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x11b8f4 | store | 4 | `sp -0x84` | `0x4` |
| `ldr` at 0x11b8fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11b900 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11b910 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11b914 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b920 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11b934 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11b938 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11b948 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11b94c | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11b974 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11b978 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11b97c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11b980 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11b98c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11b998 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11b99c | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11b9a4 | store | 4 | `(unknown)` | `0x12` |
| `str` at 0x11b9a8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11b9ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11b9b0 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11b9b4 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11b9b8 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11b9c0 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11b9c4 | load | 4 | `abs 0x11c018` | `-` |
| `str` at 0x11b9d4 | store | 4 | `sp -0x88` | `0xa` |
| `str` at 0x11b9dc | store | 4 | `sp -0x8c` | `0xa` |
| `ldrh` at 0x11b9e4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11b9ec | store | 4 | `sp -0x88` | `?` |
| `ldrh` at 0x11b9f0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11b9fc | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x11ba00 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x11ba10 | store | 4 | `sp -0x88` | `0xa` |
| `ldr` at 0x11ba14 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x11ba24 | store | 4 | `sp -0x8c` | `0xa` |
| `ldr` at 0x11ba28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ba2c | load | 4 | `abs 0x11c01c` | `-` |
| `ldr` at 0x11ba34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ba44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11ba4c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11ba50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ba54 | load | 4 | `abs 0x11c044` | `-` |
| `ldr` at 0x11ba5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ba6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11ba74 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `strb` at 0x11ba7c | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x11ba80 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11ba84 | load | 4 | `sp -0x84` | `-` |
| `ldrb` at 0x11ba90 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11ba98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ba9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11bab0 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11bab4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11bac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11bad4 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11bad8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11bae8 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11baec | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11bb14 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11bb18 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11bb1c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11bb20 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11bb2c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11bb38 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11bb3c | store | 4 | `(unknown)` | `0x18ee38` |
| `ldrb` at 0x11bb40 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11bb44 | store | 4 | `(unknown)` | `[sp-33]:1` |
| `str` at 0x11bb48 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11bb4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11bb50 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11bb54 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11bb58 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11bb60 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11bb64 | load | 4 | `abs 0x11c020` | `-` |
| `ldr` at 0x11bb70 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11bb74 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11bb80 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11bb88 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11bb94 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bba0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bba4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bbb0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bbb4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bbc0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bbc4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bbd0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bbd4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bbe0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bbe4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bbf0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bbf4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc00 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc04 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc10 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc14 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc20 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc24 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc30 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc34 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc40 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc44 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc50 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc54 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc60 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bc64 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bc6c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11bc74 | load | 4 | `abs 0x11c024` | `-` |
| `ldrh` at 0x11bc84 | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0x11bc88 | load | 4 | `abs 0x11c028` | `-` |
| `stc` at 0x11bc98 | store | unknown | `sp -0x90` | `?` |
| `ldrh` at 0x11bc9c | load | 2 | `sp -0x26` | `-` |
| `stc` at 0x11bca8 | store | unknown | `sp -0x94` | `?` |
| `ldc` at 0x11bcac | load | unknown | `sp -0x90` | `-` |
| `ldc` at 0x11bcb4 | load | unknown | `sp -0x94` | `-` |
| `ldc` at 0x11bcbc | load | unknown | `abs 0x11c004` | `-` |
| `stc` at 0x11bccc | store | unknown | `sp -0x98` | `?` |
| `ldrb` at 0x11bcd0 | load | 1 | `sp -0x21` | `-` |
| `ldc` at 0x11bcd8 | load | unknown | `sp -0x98` | `-` |
| `stc` at 0x11bce0 | store | unknown | `(unknown)` | `?` |
| `ldrb` at 0x11bce8 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11bcec | store | 4 | `(unknown)` | `[sp-33]:1` |
| `str` at 0x11bcf4 | store | 4 | `(unknown)` | `sp-120` |
| `str` at 0x11bcfc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11bd04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd08 | load | 4 | `abs 0x11c02c` | `-` |
| `ldr` at 0x11bd14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11bd2c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11bd30 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11bd38 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11bd40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd44 | load | 4 | `abs 0x11c030` | `-` |
| `ldr` at 0x11bd4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11bd64 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11bd68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd6c | load | 4 | `abs 0x11c044` | `-` |
| `ldr` at 0x11bd74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bd84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11bd8c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `strb` at 0x11bd94 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x11bd98 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11bd9c | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0x11bda8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11bdb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11bdbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11bdd0 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11bdd4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11bde0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11bdf4 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11bdf8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11be08 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11be0c | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11be34 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11be38 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11be3c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11be40 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11be4c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11be58 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11be5c | store | 4 | `(unknown)` | `0x18ee38` |
| `ldrb` at 0x11be60 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11be68 | store | 4 | `(unknown)` | `[sp-33]:1+32` |
| `str` at 0x11be6c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11be70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11be74 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11be78 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11be7c | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11be84 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11be88 | load | 4 | `abs 0x11c034` | `-` |
| `ldr` at 0x11be94 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11be98 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11bea4 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11beac | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11beb8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bec4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bec8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bed4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bed8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bee4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bee8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bef4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bef8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf04 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf08 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf14 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf18 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf24 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf28 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf34 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf38 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf44 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf48 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf54 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf58 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf64 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf68 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf74 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf78 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf84 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11bf88 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11bf90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11bf98 | load | 4 | `abs 0x11c038` | `-` |
| `ldrb` at 0x11bfa8 | load | 1 | `sp -0x21` | `-` |
| `ldrh` at 0x11bfb0 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0x11bfb4 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11bfbc | store | 4 | `(unknown)` | `[sp-33]:1+32` |
| `str` at 0x11bfc4 | store | 4 | `(unknown)` | `sp-120` |
| `ldr` at 0x11bfc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bfcc | load | 4 | `abs 0x11c03c` | `-` |
| `ldr` at 0x11bfd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11bfe8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11bff0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11bff4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11bffc | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11c06c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c070 | load | 4 | `abs 0x11c040` | `-` |
| `ldr` at 0x11c078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c088 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c090 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11c094 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c098 | load | 4 | `abs 0x11c044` | `-` |
| `ldr` at 0x11c0a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c0b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c0b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `strb` at 0x11c0c0 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x11c0c4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11c0c8 | load | 4 | `sp -0x88` | `-` |
| `ldrb` at 0x11c0d4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11c0e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11c0e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11c0fc | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11c100 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c10c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11c120 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11c124 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11c134 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11c138 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11c160 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11c164 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11c168 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11c16c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11c178 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c184 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11c188 | store | 4 | `(unknown)` | `0x18ee38` |
| `ldrb` at 0x11c18c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11c194 | store | 4 | `(unknown)` | `[sp-33]:1+40` |
| `str` at 0x11c198 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11c19c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11c1a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11c1a4 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11c1a8 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11c1b0 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11c1b4 | load | 4 | `abs 0x11c048` | `-` |
| `ldr` at 0x11c1c0 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11c1c4 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11c1d0 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c1d8 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c1e4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c1f0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c1f4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c200 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c204 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c210 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c214 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c220 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c224 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c230 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c234 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c240 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c244 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c250 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c254 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c260 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c264 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c270 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c274 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c280 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c284 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c290 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c294 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c2a0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c2a4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c2b0 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c2b4 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c2bc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11c2c4 | load | 4 | `abs 0x11c05c` | `-` |
| `ldrh` at 0x11c2d4 | load | 2 | `sp -0x26` | `-` |
| `stc` at 0x11c2e4 | store | unknown | `sp -0xa4` | `?` |
| `ldrh` at 0x11c2e8 | load | 2 | `sp -0x26` | `-` |
| `stc` at 0x11c2f4 | store | unknown | `sp -0xa8` | `?` |
| `ldc` at 0x11c2f8 | load | unknown | `sp -0xa4` | `-` |
| `ldc` at 0x11c300 | load | unknown | `sp -0xa8` | `-` |
| `ldc` at 0x11c308 | load | unknown | `abs 0x11c064` | `-` |
| `stc` at 0x11c318 | store | unknown | `sp -0xac` | `?` |
| `ldrb` at 0x11c31c | load | 1 | `sp -0x21` | `-` |
| `ldc` at 0x11c324 | load | unknown | `sp -0xac` | `-` |
| `stc` at 0x11c32c | store | unknown | `(unknown)` | `?` |
| `ldrb` at 0x11c334 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11c33c | store | 4 | `(unknown)` | `[sp-33]:1+40` |
| `str` at 0x11c344 | store | 4 | `(unknown)` | `sp-120` |
| `str` at 0x11c34c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11c354 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c358 | load | 4 | `abs 0x11c04c` | `-` |
| `ldr` at 0x11c364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c374 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c37c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11c380 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11c388 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11c394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11c398 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11c3a8 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11c3ac | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c3b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11c3cc | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11c3d0 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11c3e0 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11c3e4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11c40c | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11c410 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11c414 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11c418 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11c424 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c430 | load | 4 | `abs 0x11c050` | `-` |
| `str` at 0x11c434 | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11c43c | store | 4 | `(unknown)` | `0x32` |
| `str` at 0x11c440 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11c444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11c448 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11c44c | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11c450 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11c458 | load | 4 | `abs 0x11c054` | `-` |
| `ldr` at 0x11c45c | load | 4 | `abs 0x11c058` | `-` |
| `ldr` at 0x11c468 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11c46c | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11c478 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c480 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c48c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c498 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c49c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4a8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4ac | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4b8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4bc | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4c8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4cc | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4d8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4dc | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4e8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4ec | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c4f8 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c4fc | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c508 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c50c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c518 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c51c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c528 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c52c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c538 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c53c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c548 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c54c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c558 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c55c | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c564 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11c56c | load | 4 | `abs 0x11c05c` | `-` |
| `ldrh` at 0x11c57c | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0x11c580 | load | 4 | `abs 0x11c060` | `-` |
| `stc` at 0x11c590 | store | unknown | `sp -0xa4` | `?` |
| `ldrh` at 0x11c594 | load | 2 | `sp -0x26` | `-` |
| `stc` at 0x11c5a0 | store | unknown | `sp -0xa8` | `?` |
| `ldc` at 0x11c5a4 | load | unknown | `sp -0xa4` | `-` |
| `ldc` at 0x11c5ac | load | unknown | `sp -0xa8` | `-` |
| `ldc` at 0x11c5b4 | load | unknown | `abs 0x11c904` | `-` |
| `stc` at 0x11c5c4 | store | unknown | `sp -0xac` | `?` |
| `ldc` at 0x11c5c8 | load | unknown | `sp -0xac` | `-` |
| `str` at 0x11c5d4 | store | 4 | `(unknown)` | `0x32` |
| `str` at 0x11c5dc | store | 4 | `(unknown)` | `sp-120` |
| `ldr` at 0x11c5e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c5e4 | load | 4 | `abs 0x11c90c` | `-` |
| `stc` at 0x11c5e8 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11c5f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c604 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c60c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11c610 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c614 | load | 4 | `abs 0x11c910` | `-` |
| `ldr` at 0x11c61c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c62c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c634 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11c638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c63c | load | 4 | `abs 0x11c928` | `-` |
| `ldr` at 0x11c644 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c654 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c65c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `strb` at 0x11c664 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x11c668 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11c66c | load | 4 | `sp -0x8c` | `-` |
| `ldrb` at 0x11c678 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11c688 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11c68c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11c6a0 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11c6a4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c6b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11c6c4 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11c6c8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11c6d8 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11c6dc | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11c704 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11c708 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11c70c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11c710 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11c71c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c728 | load | 4 | `abs 0x11c92c` | `-` |
| `str` at 0x11c72c | store | 4 | `(unknown)` | `0x18ee38` |
| `ldrb` at 0x11c730 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11c738 | store | 4 | `(unknown)` | `[sp-33]:1+51` |
| `str` at 0x11c73c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11c740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11c744 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11c748 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11c74c | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11c754 | load | 4 | `abs 0x11c930` | `-` |
| `ldr` at 0x11c758 | load | 4 | `abs 0x11c914` | `-` |
| `ldr` at 0x11c764 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11c768 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11c774 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c77c | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11c788 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c794 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c798 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7a4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7a8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7b4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7b8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7c4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7c8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7d4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7d8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7e4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7e8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c7f4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c7f8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c804 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c808 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c814 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c818 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c824 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c828 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c834 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c838 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c844 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c848 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c854 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11c858 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11c860 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11c868 | load | 4 | `abs 0x11c938` | `-` |
| `ldrh` at 0x11c878 | load | 2 | `sp -0x26` | `-` |
| `ldc` at 0x11c880 | load | unknown | `abs 0x11c93c` | `-` |
| `stc` at 0x11c88c | store | unknown | `sp -0xb0` | `?` |
| `ldrb` at 0x11c890 | load | 1 | `sp -0x21` | `-` |
| `ldc` at 0x11c898 | load | unknown | `sp -0xb0` | `-` |
| `stc` at 0x11c8a0 | store | unknown | `(unknown)` | `?` |
| `ldrb` at 0x11c8a8 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11c8b0 | store | 4 | `(unknown)` | `[sp-33]:1+51` |
| `str` at 0x11c8b8 | store | 4 | `(unknown)` | `sp-120` |
| `str` at 0x11c8c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11c8c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c8cc | load | 4 | `abs 0x11c918` | `-` |
| `ldr` at 0x11c8d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11c8e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11c8f0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11c8f4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11c8fc | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11c948 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11c94c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11c95c | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11c960 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c96c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11c980 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11c984 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11c994 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11c998 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11c9c0 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11c9c4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11c9c8 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11c9cc | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11c9d8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11c9e4 | load | 4 | `abs 0x11c92c` | `-` |
| `str` at 0x11c9e8 | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11c9f0 | store | 4 | `(unknown)` | `0x3d` |
| `str` at 0x11c9f4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11c9f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11c9fc | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11ca00 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11ca04 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11ca0c | load | 4 | `abs 0x11c930` | `-` |
| `ldr` at 0x11ca10 | load | 4 | `abs 0x11c91c` | `-` |
| `ldr` at 0x11ca1c | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11ca20 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11ca2c | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11ca34 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11ca40 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca4c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11ca50 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca5c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11ca60 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca6c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11ca70 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca7c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11ca80 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca8c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11ca90 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11ca9c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11caa0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11caac | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cab0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cabc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cac0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cacc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cad0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cadc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cae0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11caec | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11caf0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cafc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cb00 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cb0c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cb10 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cb18 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11cb20 | load | 4 | `abs 0x11c938` | `-` |
| `ldrh` at 0x11cb30 | load | 2 | `sp -0x26` | `-` |
| `ldc` at 0x11cb38 | load | unknown | `abs 0x11c93c` | `-` |
| `stc` at 0x11cb44 | store | unknown | `sp -0xb0` | `?` |
| `ldc` at 0x11cb48 | load | unknown | `sp -0xb0` | `-` |
| `str` at 0x11cb54 | store | 4 | `(unknown)` | `0x3d` |
| `str` at 0x11cb5c | store | 4 | `(unknown)` | `sp-120` |
| `ldr` at 0x11cb60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cb64 | load | 4 | `abs 0x11c920` | `-` |
| `stc` at 0x11cb68 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11cb74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cb84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11cb8c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11cb90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cb94 | load | 4 | `abs 0x11c924` | `-` |
| `ldr` at 0x11cb9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cbac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11cbb4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11cbb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cbbc | load | 4 | `abs 0x11c928` | `-` |
| `ldr` at 0x11cbc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11cbd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11cbdc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `strb` at 0x11cbe4 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x11cbe8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11cbec | load | 4 | `sp -0x8c` | `-` |
| `ldrb` at 0x11cbf8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11cc08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11cc0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11cc20 | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11cc24 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11cc30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11cc44 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11cc48 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11cc58 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11cc5c | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11cc84 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11cc88 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11cc8c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11cc90 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11cc9c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11cca8 | load | 4 | `abs 0x11c92c` | `-` |
| `str` at 0x11ccac | store | 4 | `(unknown)` | `0x18ee38` |
| `ldrb` at 0x11ccb0 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11ccb8 | store | 4 | `(unknown)` | `[sp-33]:1+83` |
| `str` at 0x11ccbc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11ccc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ccc4 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11ccc8 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11cccc | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11ccd4 | load | 4 | `abs 0x11c930` | `-` |
| `ldr` at 0x11ccd8 | load | 4 | `abs 0x11c934` | `-` |
| `ldr` at 0x11cce4 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11cce8 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11ccf4 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11ccfc | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11cd08 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd14 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd18 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd24 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd28 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd34 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd38 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd44 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd48 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd54 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd58 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd64 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd68 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd74 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd78 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd84 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd88 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cd94 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cd98 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cda4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cda8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cdb4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cdb8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cdc4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cdc8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cdd4 | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cdd8 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cde0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11cde8 | load | 4 | `abs 0x11c938` | `-` |
| `ldrh` at 0x11cdf8 | load | 2 | `sp -0x26` | `-` |
| `ldc` at 0x11ce00 | load | unknown | `abs 0x11d0dc` | `-` |
| `stc` at 0x11ce0c | store | unknown | `sp -0xb0` | `?` |
| `ldrb` at 0x11ce10 | load | 1 | `sp -0x21` | `-` |
| `ldc` at 0x11ce18 | load | unknown | `sp -0xb0` | `-` |
| `stc` at 0x11ce20 | store | unknown | `(unknown)` | `?` |
| `ldrb` at 0x11ce28 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x11ce30 | store | 4 | `(unknown)` | `[sp-33]:1+83` |
| `str` at 0x11ce38 | store | 4 | `(unknown)` | `sp-120` |
| `str` at 0x11ce40 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11ce48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ce4c | load | 4 | `abs 0x11d0e4` | `-` |
| `ldr` at 0x11ce58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ce68 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11ce70 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11ce74 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11ce7c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11ce88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ce8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11ce9c | store | 4 | `sp -0x7c` | `ret(0x11f61c)` |
| `ldr` at 0x11cea0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11ceac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11cec0 | store | 4 | `sp -0xbc` | `ret(0x6e1b0)` |
| `ldr` at 0x11cec4 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x11ced4 | store | 4 | `sp -0xb8` | `ret(0x2031c)+1` |
| `ldr` at 0x11ced8 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11cf00 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x11cf04 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11cf08 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x11cf0c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11cf18 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11cf24 | load | 4 | `abs 0x11d0e8` | `-` |
| `str` at 0x11cf28 | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11cf30 | store | 4 | `(unknown)` | `0x5d` |
| `str` at 0x11cf34 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11cf38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11cf3c | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11cf40 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11cf44 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11cf4c | load | 4 | `abs 0x11d0ec` | `-` |
| `ldr` at 0x11cf50 | load | 4 | `abs 0x11d0f0` | `-` |
| `ldr` at 0x11cf5c | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x11cf60 | store | 4 | `sp -0xc0` | `[sp-124]` |
| `ldrsh` at 0x11cf6c | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11cf74 | load | 2 | `sp -0x26` | `-` |
| `ldrh` at 0x11cf80 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cf8c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cf90 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cf9c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cfa0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cfac | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cfb0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cfbc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cfc0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cfcc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cfd0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cfdc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cfe0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cfec | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11cff0 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11cffc | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d000 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d00c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d010 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d01c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d020 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d02c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d030 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d03c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d040 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d04c | store | 4 | `(unknown)` | `?` |
| `ldrh` at 0x11d050 | load | 2 | `sp -0x26` | `-` |
| `str` at 0x11d058 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x11d060 | load | 4 | `abs 0x11d0f4` | `-` |
| `ldrh` at 0x11d070 | load | 2 | `sp -0x26` | `-` |
| `ldc` at 0x11d078 | load | unknown | `abs 0x11d0dc` | `-` |
| `stc` at 0x11d084 | store | unknown | `sp -0xb0` | `?` |
| `ldc` at 0x11d088 | load | unknown | `sp -0xb0` | `-` |
| `str` at 0x11d094 | store | 4 | `(unknown)` | `0x5d` |
| `str` at 0x11d09c | store | 4 | `(unknown)` | `sp-120` |
| `ldr` at 0x11d0a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11d0a4 | load | 4 | `abs 0x11d0f8` | `-` |
| `stc` at 0x11d0a8 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11d0b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11d0c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11d0cc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11d0d0 | load | 4 | `sp -0xc0` | `-` |
| `ldm` at 0x11d0d8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11d0d8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11d0d8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11d0d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11b688 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11b694 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11b6dc | `(unknown callee) 0x203f4` | `[sp-188]` | `[sp-180]` | `[sp-184]` | `?` |
| `bl` at 0x11b6f8 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18e7e4` | `ret(0x203f4)` | `[[sp-28]+8]` |
| `bl` at 0x11b700 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11b728 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x0` | `sp-38` |
| `bl` at 0x11b744 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `ret(0x11f61c)` |
| `bl` at 0x11b760 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18e7ec` | `[[sp-28]+8]` | `ret(0x219a4)` |
| `bl` at 0x11b780 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x0` | `sp-38` |
| `bl` at 0x11b79c | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `ret(0x11f61c)` |
| `bl` at 0x11b7b8 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18e7ec` | `[[sp-28]+8]` | `ret(0x219a4)` |
| `bl` at 0x11b7d8 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x6` | `sp-38` |
| `bl` at 0x11b800 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11b80c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11b854 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11b860 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11b89c | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x1633` | `0x0` |
| `bl` at 0x11b908 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x12` | `sp-38` |
| `bl` at 0x11b930 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11b93c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11b984 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11b990 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11b9cc | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x1642` | `0x0` |
| `bl` at 0x11ba30 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ed54` | `?` | `?` |
| `bl` at 0x11ba38 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11ba58 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18e880` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11ba60 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11baa8 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11bad0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11badc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11bb24 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11bb30 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11bb6c | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x1657` | `0x0` |
| `bl` at 0x11bc80 | `(unknown callee) 0x205bc` | `sp-120` | `0x18ed6c` | `?` | `?` |
| `bl` at 0x11bd10 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18eda0` | `[sp-33]:1+1` | `?` |
| `bl` at 0x11bd18 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11bd48 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18edc4` | `?` | `[sp-132]` |
| `bl` at 0x11bd50 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11bd70 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18e880` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11bd78 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11bdc8 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11bdf0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11bdfc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11be44 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11be50 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11be90 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x1681` | `0x0` |
| `bl` at 0x11bfa4 | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |
| `bl` at 0x11bfd4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18edd4` | `[sp-33]:1+1` | `[sp-38]:2` |
| `bl` at 0x11bfdc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c074 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18edf8` | `?` | `[sp-128]` |
| `bl` at 0x11c07c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c09c | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18e880` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11c0a4 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c0f4 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11c11c | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11c128 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11c170 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11c17c | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11c1bc | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x16a6` | `0x0` |
| `bl` at 0x11c2d0 | `(unknown callee) 0x205bc` | `sp-120` | `0x18ee68` | `?` | `?` |
| `bl` at 0x11c360 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ee0c` | `[sp-33]:1+1` | `?` |
| `bl` at 0x11c368 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c3a0 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x32` | `sp-38` |
| `bl` at 0x11c3c8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11c3d4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11c41c | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11c428 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11c464 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x16ca` | `0x0` |
| `bl` at 0x11c578 | `(unknown callee) 0x205bc` | `sp-120` | `0x18ee68` | `?` | `?` |
| `bl` at 0x11c5f0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ee9c` | `?` | `?` |
| `bl` at 0x11c5f8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c618 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18eec8` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11c620 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c640 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18e880` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11c648 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c698 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11c6c0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11c6cc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11c714 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11c720 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11c760 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x16f2` | `0x0` |
| `bl` at 0x11c874 | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |
| `bl` at 0x11c8d4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18eedc` | `[sp-33]:1+1` | `?` |
| `bl` at 0x11c8dc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11c954 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x3d` | `sp-38` |
| `bl` at 0x11c97c | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11c988 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11c9d0 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11c9dc | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11ca18 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x1713` | `0x0` |
| `bl` at 0x11cb2c | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |
| `bl` at 0x11cb70 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ef08` | `?` | `?` |
| `bl` at 0x11cb78 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11cb98 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18ef34` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11cba0 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11cbc0 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18e880` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11cbc8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11cc18 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `?` | `sp-38` |
| `bl` at 0x11cc40 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11cc4c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11cc94 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11cca0 | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11cce0 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x1739` | `0x0` |
| `bl` at 0x11cdf4 | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |
| `bl` at 0x11ce54 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18eedc` | `[sp-33]:1+1` | `?` |
| `bl` at 0x11ce5c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11ce94 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x5d` | `sp-38` |
| `bl` at 0x11cebc | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11cec8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11cf10 | `(unknown callee) 0x203f4` | `[sp-180]` | `[sp-188]` | `[sp-184]` | `?` |
| `bl` at 0x11cf1c | `(unknown callee) 0x219a4` | `[sp-124]` | `?` | `?` | `?` |
| `bl` at 0x11cf58 | `(unknown callee) 0x22070` | `0x3` | `0x18dff0` | `0x175a` | `0x0` |
| `bl` at 0x11d06c | `(unknown callee) 0x205bc` | `sp-120` | `0x18e810` | `?` | `?` |
| `bl` at 0x11d0b0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ef08` | `?` | `?` |
| `bl` at 0x11d0b8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11b73c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x11b794 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x11b7ec | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `ble` at 0x11b8d8 | `[sp-128] <= 0x4 (signed, 32-bit)` | `[sp-128] > 0x4 (signed, 32-bit)` |
| `ble` at 0x11b8ec | `[sp-132] <= 0x4 (signed, 32-bit)` | `[sp-132] > 0x4 (signed, 32-bit)` |
| `beq` at 0x11b91c | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `ble` at 0x11ba08 | `[sp-136] <= 0xa (signed, 32-bit)` | `[sp-136] > 0xa (signed, 32-bit)` |
| `ble` at 0x11ba1c | `[sp-140] <= 0xa (signed, 32-bit)` | `[sp-140] > 0xa (signed, 32-bit)` |
| `bge` at 0x11ba8c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11babc | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11bda4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11bddc | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11c0d0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11c108 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11c3b4 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11c674 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11c6ac | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11c968 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11cbf4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11cc2c | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11cea8 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11d0d8 | `r0` | `?` |

## `monitorPSMIPwrStatusTbl`

- `monitorPSMIPwrStatusTbl`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11d0fc to 0x11ea34.
  Established. Four word loads from absolute addresses 0x18ef50 through 0x18ef5c are copied to sp-0x6f8 through sp-0x6ec (`ldm` at 0x11d15c, `stm` at 0x11d160).
  The call to unnamed 0x20550 has candidate values sp-0x6e4, zero, 0x3c and sp-0x6e4 (`bl` at 0x11d17c).
  Coprocessor stores at sp-0x710 and sp-0x714 have unknown widths and values (`stc` at 0x11d86c, `stc` at 0x11d890).
  The recorded return is the sum of the word at sp-0x20 and a result of unnamed 0x2031c (`ldm` at 0x11ea30).
  These software data movements do not establish a physical power-status table layout.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11d110 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11d114 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11d118 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x11d120 | store | 4 | `sp -0x690` | `sp-364` |
| `str` at 0x11d128 | store | 4 | `sp -0x694` | `sp-684` |
| `str` at 0x11d130 | store | 4 | `sp -0x698` | `sp-1004` |
| `str` at 0x11d13c | store | 4 | `sp -0x69c` | `sp-1324` |
| `str` at 0x11d148 | store | 4 | `sp -0x6a0` | `sp-1644` |
| `ldr` at 0x11d14c | load | 4 | `abs 0x11db4c` | `-` |
| `ldm` at 0x11d15c | load | 4 | `abs 0x18ef50` | `-` |
| `ldm` at 0x11d15c | load | 4 | `abs 0x18ef54` | `-` |
| `ldm` at 0x11d15c | load | 4 | `abs 0x18ef58` | `-` |
| `ldm` at 0x11d15c | load | 4 | `abs 0x18ef5c` | `-` |
| `stm` at 0x11d160 | store | 4 | `sp -0x6f8` | `[0x18ef50]` |
| `stm` at 0x11d160 | store | 4 | `sp -0x6f4` | `[0x18ef54]` |
| `stm` at 0x11d160 | store | 4 | `sp -0x6f0` | `[0x18ef58]` |
| `stm` at 0x11d160 | store | 4 | `sp -0x6ec` | `[0x18ef5c]` |
| `ldr` at 0x11d164 | load | 4 | `abs 0x18ef60` | `-` |
| `str` at 0x11d168 | store | 4 | `sp -0x6e8` | `[0x18ef60]` |
| `str` at 0x11d188 | store | 4 | `sp -0x18` | `ret(0x6e190)` |
| `ldr` at 0x11d190 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d1a0 | store | 4 | `sp -0x24` | `ret(0x111db8)` |
| `str` at 0x11d1a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x11d1ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d1b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11d1bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d1d0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x11d1d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x11d1e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d1ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11d204 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d218 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d228 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d23c | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11d240 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d24c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d260 | store | 4 | `sp -0x734` | `ret(0x6e1b0)` |
| `ldr` at 0x11d264 | load | 4 | `sp -0x734` | `-` |
| `str` at 0x11d274 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11d278 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11d2a0 | store | 4 | `sp -0x73c` | `?` |
| `ldr` at 0x11d2a4 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11d2a8 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11d2ac | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11d2c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d2d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d2e0 | load | 4 | `abs 0x11dba0` | `-` |
| `ldr` at 0x11d2e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11d2f0 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d304 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d308 | load | 4 | `abs 0x11db50` | `-` |
| `ldr` at 0x11d318 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11d31c | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d324 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11d334 | load | 4 | `sp -0x694` | `-` |
| `str` at 0x11d33c | store | 4 | `sp -0x694` | `([sp-1684] + ret(0x2031c))` |
| `ldr` at 0x11d340 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11d344 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d34c | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11d35c | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11d364 | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11d368 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11d36c | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d374 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11d384 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11d38c | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11d390 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11d394 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d39c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11d3ac | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0x11d3b4 | store | 4 | `sp -0x6a0` | `([sp-1696] + ret(0x2031c))` |
| `ldr` at 0x11d3b8 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d3c8 | load | 4 | `sp -0x690` | `-` |
| `str` at 0x11d3d0 | store | 4 | `sp -0x690` | `([sp-1680] + ret(0x2031c))` |
| `ldr` at 0x11d3d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d3ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d3fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d410 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11d414 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d420 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d434 | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11d438 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11d448 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11d44c | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11d474 | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11d478 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11d47c | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11d480 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11d48c | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d498 | load | 4 | `abs 0x11db54` | `-` |
| `str` at 0x11d49c | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11d4a4 | store | 4 | `(unknown)` | `0x6` |
| `str` at 0x11d4a8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11d4ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d4c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d4c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x11d4cc | store | 4 | `(unknown)` | `?` |
| `str` at 0x11d4d0 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11d4d8 | load | 4 | `abs 0x11db58` | `-` |
| `ldr` at 0x11d4dc | load | 4 | `abs 0x11db5c` | `-` |
| `str` at 0x11d4ec | store | 4 | `sp -0x700` | `0x4` |
| `str` at 0x11d4f4 | store | 4 | `sp -0x704` | `0x4` |
| `ldr` at 0x11d4fc | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d504 | load | 2 | `sp -0x6a6` | `-` |
| `str` at 0x11d510 | store | 4 | `sp -0x700` | `?` |
| `ldr` at 0x11d514 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d51c | load | 2 | `sp -0x6a6` | `-` |
| `str` at 0x11d524 | store | 4 | `sp -0x704` | `?` |
| `ldr` at 0x11d528 | load | 4 | `sp -0x700` | `-` |
| `str` at 0x11d538 | store | 4 | `sp -0x700` | `0x4` |
| `ldr` at 0x11d53c | load | 4 | `sp -0x704` | `-` |
| `str` at 0x11d54c | store | 4 | `sp -0x704` | `0x4` |
| `ldr` at 0x11d550 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d564 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d574 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d588 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11d58c | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d598 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d5ac | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11d5b0 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11d5c0 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11d5c4 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11d5ec | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11d5f0 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11d5f4 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11d5f8 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11d604 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d610 | load | 4 | `abs 0x11db54` | `-` |
| `str` at 0x11d614 | store | 4 | `(unknown)` | `0x18ee38` |
| `str` at 0x11d61c | store | 4 | `(unknown)` | `0x12` |
| `str` at 0x11d620 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11d624 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d638 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d640 | load | 4 | `(unknown)` | `-` |
| `str` at 0x11d644 | store | 4 | `(unknown)` | `?` |
| `str` at 0x11d648 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11d650 | load | 4 | `abs 0x11db58` | `-` |
| `ldr` at 0x11d654 | load | 4 | `abs 0x11db60` | `-` |
| `str` at 0x11d664 | store | 4 | `sp -0x708` | `0xa` |
| `str` at 0x11d66c | store | 4 | `sp -0x70c` | `0xa` |
| `ldr` at 0x11d674 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d67c | load | 2 | `sp -0x6a6` | `-` |
| `str` at 0x11d684 | store | 4 | `sp -0x708` | `?` |
| `ldr` at 0x11d688 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d690 | load | 2 | `sp -0x6a6` | `-` |
| `str` at 0x11d69c | store | 4 | `sp -0x70c` | `?` |
| `ldr` at 0x11d6a0 | load | 4 | `sp -0x708` | `-` |
| `str` at 0x11d6b0 | store | 4 | `sp -0x708` | `0xa` |
| `ldr` at 0x11d6b4 | load | 4 | `sp -0x70c` | `-` |
| `str` at 0x11d6c4 | store | 4 | `sp -0x70c` | `0xa` |
| `ldr` at 0x11d6c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d6dc | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11d6e0 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11d6f0 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11d6f4 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11d71c | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11d720 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11d724 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11d728 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11d73c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d750 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d75c | load | 4 | `abs 0x11dba0` | `-` |
| `ldr` at 0x11d764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11d774 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d778 | load | 4 | `abs 0x11dba4` | `-` |
| `ldr` at 0x11d784 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d794 | load | 4 | `sp -0x690` | `-` |
| `str` at 0x11d79c | store | 4 | `sp -0x690` | `([sp-1680] + ret(0x2031c))` |
| `ldr` at 0x11d7a0 | load | 4 | `abs 0x11db98` | `-` |
| `strb` at 0x11d7ac | store | 1 | `sp -0x6a1` | `0x0` |
| `ldr` at 0x11d7b0 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11d7b8 | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11d7bc | load | 4 | `sp -0x704` | `-` |
| `ldr` at 0x11d7c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d7dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d7e4 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11d7ec | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11d7f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d804 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11d808 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11d814 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d818 | load | 4 | `abs 0x11db64` | `-` |
| `ldr` at 0x11d81c | load | 4 | `abs 0x11dbac` | `-` |
| `ldr` at 0x11d824 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d834 | load | 4 | `sp -0x690` | `-` |
| `str` at 0x11d83c | store | 4 | `sp -0x690` | `([sp-1680] + ret(0x2031c))` |
| `ldr` at 0x11d844 | load | 4 | `abs 0x11db68` | `-` |
| `ldr` at 0x11d850 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d858 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11d85c | load | 4 | `abs 0x11db6c` | `-` |
| `stc` at 0x11d86c | store | unknown | `sp -0x710` | `?` |
| `ldr` at 0x11d870 | load | 4 | `abs 0x11db70` | `-` |
| `ldr` at 0x11d87c | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11d884 | load | 2 | `sp -0x6a6` | `-` |
| `stc` at 0x11d890 | store | unknown | `sp -0x714` | `?` |
| `ldr` at 0x11d894 | load | 4 | `abs 0x11db74` | `-` |
| `ldr` at 0x11d8a0 | load | 4 | `abs 0x11db68` | `-` |
| `ldc` at 0x11d8ac | load | unknown | `sp -0x710` | `-` |
| `ldr` at 0x11d8b4 | load | 4 | `abs 0x11db70` | `-` |
| `ldc` at 0x11d8c0 | load | unknown | `sp -0x714` | `-` |
| `ldc` at 0x11d8c8 | load | unknown | `abs 0x11db44` | `-` |
| `stc` at 0x11d8d8 | store | unknown | `sp -0x718` | `?` |
| `ldr` at 0x11d8dc | load | 4 | `abs 0x11db74` | `-` |
| `ldc` at 0x11d8e8 | load | unknown | `sp -0x718` | `-` |
| `ldr` at 0x11d8f0 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d8f4 | load | 4 | `abs 0x11db78` | `-` |
| `stc` at 0x11d8f8 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11d904 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d914 | load | 4 | `sp -0x690` | `-` |
| `str` at 0x11d91c | store | 4 | `sp -0x690` | `([sp-1680] + ret(0x2031c))` |
| `ldr` at 0x11d920 | load | 4 | `abs 0x11db98` | `-` |
| `ldr` at 0x11d924 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11d92c | load | 1 | `sp -0x6a1` | `-` |
| `strb` at 0x11d938 | store | 1 | `sp -0x6a1` | `([sp-1697]:1+1 & 0xff)` |
| `ldr` at 0x11d940 | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d944 | load | 4 | `abs 0x11db9c` | `-` |
| `ldr` at 0x11d94c | load | 4 | `sp -0x690` | `-` |
| `ldr` at 0x11d95c | load | 4 | `sp -0x690` | `-` |
| `str` at 0x11d964 | store | 4 | `sp -0x690` | `([sp-1680] + ret(0x2031c))` |
| `ldr` at 0x11d968 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11d97c | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11d980 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11d990 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11d994 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11d9bc | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11d9c0 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11d9c4 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11d9c8 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11d9dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11d9f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11d9fc | load | 4 | `abs 0x11dba0` | `-` |
| `ldr` at 0x11da04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11da14 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11da18 | load | 4 | `abs 0x11dba4` | `-` |
| `ldr` at 0x11da24 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11da34 | load | 4 | `sp -0x694` | `-` |
| `str` at 0x11da3c | store | 4 | `sp -0x694` | `([sp-1684] + ret(0x2031c))` |
| `ldr` at 0x11da40 | load | 4 | `abs 0x11db98` | `-` |
| `strb` at 0x11da4c | store | 1 | `sp -0x6a1` | `0x0` |
| `ldr` at 0x11da50 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11da58 | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11da5c | load | 4 | `sp -0x700` | `-` |
| `ldr` at 0x11da68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11da7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11da84 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11da8c | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11daa0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11daac | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11dab0 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11dabc | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11dac0 | load | 4 | `abs 0x11db7c` | `-` |
| `ldr` at 0x11dac4 | load | 4 | `abs 0x11dbac` | `-` |
| `ldr` at 0x11dacc | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11dadc | load | 4 | `sp -0x694` | `-` |
| `str` at 0x11dae4 | store | 4 | `sp -0x694` | `([sp-1684] + ret(0x2031c))` |
| `ldr` at 0x11daec | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11daf4 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11daf8 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11dafc | load | 4 | `abs 0x11db80` | `-` |
| `ldr` at 0x11db08 | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11db18 | load | 4 | `sp -0x694` | `-` |
| `str` at 0x11db20 | store | 4 | `sp -0x694` | `([sp-1684] + ret(0x2031c))` |
| `ldr` at 0x11db24 | load | 4 | `abs 0x11db98` | `-` |
| `ldr` at 0x11db28 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11db30 | load | 1 | `sp -0x6a1` | `-` |
| `strb` at 0x11db3c | store | 1 | `sp -0x6a1` | `([sp-1697]:1+1 & 0xff)` |
| `ldr` at 0x11dbc0 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11dbc8 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11dbcc | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11dbd0 | load | 4 | `abs 0x11db9c` | `-` |
| `ldr` at 0x11dbdc | load | 4 | `sp -0x694` | `-` |
| `ldr` at 0x11dbec | load | 4 | `sp -0x694` | `-` |
| `str` at 0x11dbf4 | store | 4 | `sp -0x694` | `([sp-1684] + ret(0x2031c))` |
| `ldr` at 0x11dbf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11dc0c | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11dc10 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11dc20 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11dc24 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11dc4c | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11dc50 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11dc54 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11dc58 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11dc6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11dc80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11dc8c | load | 4 | `abs 0x11dba0` | `-` |
| `ldr` at 0x11dc94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11dca4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dca8 | load | 4 | `abs 0x11dba4` | `-` |
| `ldr` at 0x11dcb4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dcc4 | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11dccc | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11dcd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11dce4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11dcf4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11dd08 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11dd0c | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11dd18 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dd1c | load | 4 | `abs 0x11dba8` | `-` |
| `ldr` at 0x11dd20 | load | 4 | `abs 0x11dbac` | `-` |
| `ldr` at 0x11dd28 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dd38 | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11dd40 | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11dd48 | load | 4 | `abs 0x11db88` | `-` |
| `ldr` at 0x11dd54 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11dd5c | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11dd60 | load | 4 | `abs 0x11db84` | `-` |
| `stc` at 0x11dd70 | store | unknown | `sp -0x724` | `?` |
| `ldr` at 0x11dd74 | load | 4 | `abs 0x11db8c` | `-` |
| `ldr` at 0x11dd80 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11dd88 | load | 2 | `sp -0x6a6` | `-` |
| `stc` at 0x11dd94 | store | unknown | `sp -0x728` | `?` |
| `ldr` at 0x11dd98 | load | 4 | `abs 0x11db90` | `-` |
| `ldr` at 0x11dda4 | load | 4 | `abs 0x11db88` | `-` |
| `ldc` at 0x11ddb0 | load | unknown | `sp -0x724` | `-` |
| `ldr` at 0x11ddb8 | load | 4 | `abs 0x11db8c` | `-` |
| `ldc` at 0x11ddc4 | load | unknown | `sp -0x728` | `-` |
| `ldc` at 0x11ddcc | load | unknown | `abs 0x11dbb8` | `-` |
| `stc` at 0x11dddc | store | unknown | `sp -0x72c` | `?` |
| `ldr` at 0x11dde0 | load | 4 | `abs 0x11db90` | `-` |
| `ldc` at 0x11ddec | load | unknown | `sp -0x72c` | `-` |
| `ldr` at 0x11ddf4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11ddf8 | load | 4 | `abs 0x11db94` | `-` |
| `stc` at 0x11ddfc | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11de08 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11de18 | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11de20 | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11de24 | load | 4 | `abs 0x11db98` | `-` |
| `strb` at 0x11de30 | store | 1 | `sp -0x6a1` | `0x0` |
| `ldr` at 0x11de34 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11de3c | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11de40 | load | 4 | `sp -0x708` | `-` |
| `ldr` at 0x11de4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11de60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11de68 | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11de70 | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11de84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11de90 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11de94 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11dea0 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dea4 | load | 4 | `abs 0x11dba8` | `-` |
| `ldr` at 0x11dea8 | load | 4 | `abs 0x11dbac` | `-` |
| `ldr` at 0x11deb0 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dec0 | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11dec8 | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11ded0 | load | 4 | `abs 0x11db88` | `-` |
| `ldr` at 0x11dedc | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11dee4 | load | 2 | `sp -0x6a6` | `-` |
| `stc` at 0x11def4 | store | unknown | `sp -0x724` | `?` |
| `ldr` at 0x11def8 | load | 4 | `abs 0x11db8c` | `-` |
| `ldr` at 0x11df04 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11df0c | load | 2 | `sp -0x6a6` | `-` |
| `stc` at 0x11df18 | store | unknown | `sp -0x728` | `?` |
| `ldr` at 0x11df1c | load | 4 | `abs 0x11db90` | `-` |
| `ldr` at 0x11df28 | load | 4 | `abs 0x11db88` | `-` |
| `ldc` at 0x11df34 | load | unknown | `sp -0x724` | `-` |
| `ldr` at 0x11df3c | load | 4 | `abs 0x11db8c` | `-` |
| `ldc` at 0x11df48 | load | unknown | `sp -0x728` | `-` |
| `ldc` at 0x11df50 | load | unknown | `abs 0x11dbb8` | `-` |
| `stc` at 0x11df60 | store | unknown | `sp -0x72c` | `?` |
| `ldr` at 0x11df64 | load | 4 | `abs 0x11db90` | `-` |
| `ldc` at 0x11df70 | load | unknown | `sp -0x72c` | `-` |
| `ldr` at 0x11df78 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11df7c | load | 4 | `abs 0x11db94` | `-` |
| `stc` at 0x11df80 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11df8c | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11df9c | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11dfa4 | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11dfa8 | load | 4 | `abs 0x11db98` | `-` |
| `ldr` at 0x11dfac | load | 4 | `abs 0x11db98` | `-` |
| `ldrb` at 0x11dfb4 | load | 1 | `sp -0x6a1` | `-` |
| `strb` at 0x11dfc0 | store | 1 | `sp -0x6a1` | `([sp-1697]:1+1 & 0xff)` |
| `ldr` at 0x11dfc8 | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11dfd0 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11dfd4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dfd8 | load | 4 | `abs 0x11db9c` | `-` |
| `ldr` at 0x11dfe4 | load | 4 | `sp -0x698` | `-` |
| `ldr` at 0x11dff4 | load | 4 | `sp -0x698` | `-` |
| `str` at 0x11dffc | store | 4 | `sp -0x698` | `([sp-1688] + ret(0x2031c))` |
| `ldr` at 0x11e000 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e014 | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11e018 | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11e028 | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11e02c | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11e054 | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11e058 | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11e05c | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11e060 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11e074 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e088 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e094 | load | 4 | `abs 0x11dba0` | `-` |
| `ldr` at 0x11e09c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11e0ac | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e0b0 | load | 4 | `abs 0x11dba4` | `-` |
| `ldr` at 0x11e0bc | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e0cc | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e0d4 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e0d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e0ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e0fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e110 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11e114 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11e120 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e124 | load | 4 | `abs 0x11dba8` | `-` |
| `ldr` at 0x11e128 | load | 4 | `abs 0x11dbac` | `-` |
| `ldr` at 0x11e130 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e140 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e148 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e150 | load | 4 | `abs 0x11dbb0` | `-` |
| `ldr` at 0x11e15c | load | 4 | `abs 0x11dbb4` | `-` |
| `ldrh` at 0x11e164 | load | 2 | `sp -0x6a6` | `-` |
| `ldc` at 0x11e16c | load | unknown | `abs 0x11e4dc` | `-` |
| `stc` at 0x11e178 | store | unknown | `sp -0x730` | `?` |
| `ldr` at 0x11e17c | load | 4 | `abs 0x11e494` | `-` |
| `ldc` at 0x11e188 | load | unknown | `sp -0x730` | `-` |
| `ldr` at 0x11e190 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e194 | load | 4 | `abs 0x11e498` | `-` |
| `stc` at 0x11e198 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11e1a4 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e1b4 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e1bc | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e1c0 | load | 4 | `abs 0x11e49c` | `-` |
| `strb` at 0x11e1cc | store | 1 | `sp -0x6a1` | `0x0` |
| `ldr` at 0x11e1d0 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e1d8 | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11e1dc | load | 4 | `sp -0x70c` | `-` |
| `ldr` at 0x11e1e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e1fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e204 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e20c | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11e220 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e22c | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11e230 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11e23c | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e240 | load | 4 | `abs 0x11e48c` | `-` |
| `ldr` at 0x11e244 | load | 4 | `abs 0x11e490` | `-` |
| `ldr` at 0x11e24c | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e25c | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e264 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e26c | load | 4 | `abs 0x11e494` | `-` |
| `ldr` at 0x11e278 | load | 4 | `abs 0x11e4a0` | `-` |
| `ldrh` at 0x11e280 | load | 2 | `sp -0x6a6` | `-` |
| `ldc` at 0x11e288 | load | unknown | `abs 0x11e4dc` | `-` |
| `stc` at 0x11e294 | store | unknown | `sp -0x730` | `?` |
| `ldr` at 0x11e298 | load | 4 | `abs 0x11e494` | `-` |
| `ldc` at 0x11e2a4 | load | unknown | `sp -0x730` | `-` |
| `ldr` at 0x11e2ac | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e2b0 | load | 4 | `abs 0x11e498` | `-` |
| `stc` at 0x11e2b4 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11e2c0 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e2d0 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e2d8 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e2dc | load | 4 | `abs 0x11e49c` | `-` |
| `ldr` at 0x11e2e0 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e2e8 | load | 1 | `sp -0x6a1` | `-` |
| `strb` at 0x11e2f4 | store | 1 | `sp -0x6a1` | `([sp-1697]:1+1 & 0xff)` |
| `ldr` at 0x11e2fc | load | 4 | `abs 0x11e4a0` | `-` |
| `ldrh` at 0x11e304 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11e308 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e30c | load | 4 | `abs 0x11e4a4` | `-` |
| `ldr` at 0x11e318 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e328 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e330 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e334 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e348 | store | 4 | `sp -0x73c` | `ret(0x6e1b0)` |
| `ldr` at 0x11e34c | load | 4 | `sp -0x73c` | `-` |
| `str` at 0x11e35c | store | 4 | `sp -0x738` | `ret(0x2031c)+1` |
| `ldr` at 0x11e360 | load | 4 | `sp -0x738` | `-` |
| `str` at 0x11e388 | store | 4 | `sp -0x734` | `?` |
| `ldr` at 0x11e38c | load | 4 | `sp -0x734` | `-` |
| `ldr` at 0x11e390 | load | 4 | `sp -0x73c` | `-` |
| `ldr` at 0x11e394 | load | 4 | `sp -0x738` | `-` |
| `ldr` at 0x11e3a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e3bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e3c8 | load | 4 | `abs 0x11e484` | `-` |
| `ldr` at 0x11e3d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11e3e0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e3e4 | load | 4 | `abs 0x11e488` | `-` |
| `ldr` at 0x11e3f0 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e400 | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0x11e408 | store | 4 | `sp -0x6a0` | `([sp-1696] + ret(0x2031c))` |
| `ldr` at 0x11e40c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e420 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e430 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e444 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11e448 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11e454 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e458 | load | 4 | `abs 0x11e48c` | `-` |
| `ldr` at 0x11e45c | load | 4 | `abs 0x11e490` | `-` |
| `ldr` at 0x11e464 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e474 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e47c | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e4e4 | load | 4 | `abs 0x11e494` | `-` |
| `ldr` at 0x11e4f0 | load | 4 | `abs 0x11e4a0` | `-` |
| `ldrh` at 0x11e4f8 | load | 2 | `sp -0x6a6` | `-` |
| `ldc` at 0x11e500 | load | unknown | `abs 0x11e4dc` | `-` |
| `stc` at 0x11e50c | store | unknown | `sp -0x730` | `?` |
| `ldr` at 0x11e510 | load | 4 | `abs 0x11e494` | `-` |
| `ldc` at 0x11e51c | load | unknown | `sp -0x730` | `-` |
| `ldr` at 0x11e524 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e528 | load | 4 | `abs 0x11e498` | `-` |
| `stc` at 0x11e52c | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11e538 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e548 | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0x11e550 | store | 4 | `sp -0x6a0` | `([sp-1696] + ret(0x2031c))` |
| `ldr` at 0x11e554 | load | 4 | `abs 0x11e49c` | `-` |
| `strb` at 0x11e560 | store | 1 | `sp -0x6a1` | `0x0` |
| `ldr` at 0x11e564 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e56c | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11e570 | load | 4 | `sp -0x70c` | `-` |
| `ldr` at 0x11e57c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11e590 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11e598 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e5a0 | load | 1 | `sp -0x6a1` | `-` |
| `ldr` at 0x11e5b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11e5c0 | store | 4 | `sp -0x6fc` | `ret(0x11f61c)` |
| `ldr` at 0x11e5c4 | load | 4 | `sp -0x6fc` | `-` |
| `ldr` at 0x11e5d0 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e5d4 | load | 4 | `abs 0x11e48c` | `-` |
| `ldr` at 0x11e5d8 | load | 4 | `abs 0x11e490` | `-` |
| `ldr` at 0x11e5e0 | load | 4 | `sp -0x69c` | `-` |
| `ldr` at 0x11e5f0 | load | 4 | `sp -0x69c` | `-` |
| `str` at 0x11e5f8 | store | 4 | `sp -0x69c` | `([sp-1692] + ret(0x2031c))` |
| `ldr` at 0x11e600 | load | 4 | `abs 0x11e494` | `-` |
| `ldr` at 0x11e60c | load | 4 | `abs 0x11e4a0` | `-` |
| `ldrh` at 0x11e614 | load | 2 | `sp -0x6a6` | `-` |
| `ldc` at 0x11e61c | load | unknown | `abs 0x11e4dc` | `-` |
| `stc` at 0x11e628 | store | unknown | `sp -0x730` | `?` |
| `ldr` at 0x11e62c | load | 4 | `abs 0x11e494` | `-` |
| `ldc` at 0x11e638 | load | unknown | `sp -0x730` | `-` |
| `ldr` at 0x11e640 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e644 | load | 4 | `abs 0x11e498` | `-` |
| `stc` at 0x11e648 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x11e654 | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e664 | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0x11e66c | store | 4 | `sp -0x6a0` | `([sp-1696] + ret(0x2031c))` |
| `ldr` at 0x11e670 | load | 4 | `abs 0x11e49c` | `-` |
| `ldr` at 0x11e674 | load | 4 | `abs 0x11e49c` | `-` |
| `ldrb` at 0x11e67c | load | 1 | `sp -0x6a1` | `-` |
| `strb` at 0x11e688 | store | 1 | `sp -0x6a1` | `([sp-1697]:1+1 & 0xff)` |
| `ldr` at 0x11e690 | load | 4 | `abs 0x11e4a0` | `-` |
| `ldrh` at 0x11e698 | load | 2 | `sp -0x6a6` | `-` |
| `ldr` at 0x11e69c | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e6a0 | load | 4 | `abs 0x11e4a4` | `-` |
| `ldr` at 0x11e6ac | load | 4 | `sp -0x6a0` | `-` |
| `ldr` at 0x11e6bc | load | 4 | `sp -0x6a0` | `-` |
| `str` at 0x11e6c4 | store | 4 | `sp -0x6a0` | `([sp-1696] + ret(0x2031c))` |
| `ldr` at 0x11e6c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x11e6d0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x11e6d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e6dc | load | 4 | `abs 0x11e4a8` | `-` |
| `ldr` at 0x11e6e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e6f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e6fc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e704 | load | 4 | `abs 0x11e4ac` | `-` |
| `ldr` at 0x11e70c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e71c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e724 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e728 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e72c | load | 4 | `abs 0x11e4b0` | `-` |
| `ldr` at 0x11e734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e744 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e74c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e758 | load | 4 | `abs 0x11e4d8` | `-` |
| `ldr` at 0x11e764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e774 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e77c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e780 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e784 | load | 4 | `abs 0x11e4b4` | `-` |
| `ldr` at 0x11e78c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e79c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e7a4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e7a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e7ac | load | 4 | `abs 0x11e4b8` | `-` |
| `ldr` at 0x11e7b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e7c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e7cc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e7d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e7d4 | load | 4 | `abs 0x11e4bc` | `-` |
| `ldr` at 0x11e7dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e7ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e7f4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e7fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e800 | load | 4 | `abs 0x11e4d8` | `-` |
| `ldr` at 0x11e80c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e81c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e824 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e82c | load | 4 | `abs 0x11e4c0` | `-` |
| `ldr` at 0x11e834 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e844 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e84c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e850 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e854 | load | 4 | `abs 0x11e4c4` | `-` |
| `ldr` at 0x11e85c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e86c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e874 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e878 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e87c | load | 4 | `abs 0x11e4d4` | `-` |
| `ldr` at 0x11e884 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e894 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e89c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e8a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e8a8 | load | 4 | `abs 0x11e4d8` | `-` |
| `ldr` at 0x11e8b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e8c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e8cc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e8d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e8d4 | load | 4 | `abs 0x11e4c8` | `-` |
| `ldr` at 0x11e8dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e8ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e8f4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e8f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e8fc | load | 4 | `abs 0x11e4d0` | `-` |
| `ldr` at 0x11e904 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e914 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e91c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e920 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e924 | load | 4 | `abs 0x11e4d4` | `-` |
| `ldr` at 0x11e92c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e93c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e944 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e954 | load | 4 | `abs 0x11e4d8` | `-` |
| `ldr` at 0x11e960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e970 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e978 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e97c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e980 | load | 4 | `abs 0x11e4cc` | `-` |
| `ldr` at 0x11e988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e998 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e9a0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e9a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e9a8 | load | 4 | `abs 0x11e4d0` | `-` |
| `ldr` at 0x11e9b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e9c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e9c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e9cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e9d0 | load | 4 | `abs 0x11e4d4` | `-` |
| `ldr` at 0x11e9d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11e9e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11e9f0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11e9fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ea00 | load | 4 | `abs 0x11e4d8` | `-` |
| `ldr` at 0x11ea0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ea1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11ea24 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldm` at 0x11ea30 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11ea30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11ea30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11ea30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11d17c | `(unknown callee) 0x20550` | `sp-1764` | `0x0` | `0x3c` | `sp-1764` |
| `bl` at 0x11d180 | `(unknown callee) 0x6e190` | `ret(0x20550)` | `?` | `?` | `?` |
| `bl` at 0x11d198 | `(unknown callee) 0x111db8` | `ret(0x6e190)` | `sp-44` | `?` | `sp-44` |
| `bl` at 0x11d1f4 | `(unknown callee) 0x111978` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x11d234 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x0` | `sp-1702` |
| `bl` at 0x11d25c | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11d268 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11d2b0 | `(unknown callee) 0x203f4` | `[sp-1852]` | `[sp-1844]` | `[sp-1848]` | `?` |
| `bl` at 0x11d2ec | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11d2f4 | `(unknown callee) 0x219a4` | `[sp-1788]` | `?` | `?` | `?` |
| `bl` at 0x11d314 | `(unknown callee) 0x205bc` | `[sp-1680]` | `0x18efa0` | `sp-1676` | `ret(0x219a4)` |
| `bl` at 0x11d320 | `(unknown callee) 0x20634` | `[sp-1684]` | `[sp-1680]` | `?` | `?` |
| `bl` at 0x11d328 | `(unknown callee) 0x2031c` | `[sp-1684]` | `?` | `?` | `?` |
| `bl` at 0x11d348 | `(unknown callee) 0x20634` | `[sp-1688]` | `[sp-1680]` | `ret(0x2031c)` | `([sp-1684] + ret(0x2031c))` |
| `bl` at 0x11d350 | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11d370 | `(unknown callee) 0x20634` | `[sp-1692]` | `[sp-1680]` | `ret(0x2031c)` | `([sp-1688] + ret(0x2031c))` |
| `bl` at 0x11d378 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11d398 | `(unknown callee) 0x20634` | `[sp-1696]` | `[sp-1680]` | `ret(0x2031c)` | `([sp-1692] + ret(0x2031c))` |
| `bl` at 0x11d3a0 | `(unknown callee) 0x2031c` | `[sp-1696]` | `?` | `?` | `?` |
| `bl` at 0x11d3bc | `(unknown callee) 0x2031c` | `[sp-1680]` | `?` | `ret(0x2031c)` | `([sp-1696] + ret(0x2031c))` |
| `bl` at 0x11d408 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x6` | `sp-1702` |
| `bl` at 0x11d430 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11d43c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11d484 | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11d490 | `(unknown callee) 0x219a4` | `[sp-1788]` | `?` | `?` | `?` |
| `bl` at 0x11d4e4 | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x17b4` | `0x0` |
| `bl` at 0x11d580 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x12` | `sp-1702` |
| `bl` at 0x11d5a8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11d5b4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11d5fc | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11d608 | `(unknown callee) 0x219a4` | `[sp-1788]` | `?` | `?` | `?` |
| `bl` at 0x11d65c | `(unknown callee) 0x22070` | `0x1` | `0x18dff0` | `0x17c3` | `0x0` |
| `bl` at 0x11d6d8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11d6e4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11d72c | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11d768 | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11d780 | `(unknown callee) 0x205bc` | `[sp-1680]` | `0x18efdc` | `sp-1676` | `sp-1676` |
| `bl` at 0x11d788 | `(unknown callee) 0x2031c` | `[sp-1680]` | `?` | `?` | `?` |
| `bl` at 0x11d7fc | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `?` | `sp-1702` |
| `bl` at 0x11d820 | `(unknown callee) 0x205bc` | `[sp-1680]` | `0x18efb0` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11d828 | `(unknown callee) 0x2031c` | `[sp-1680]` | `?` | `?` | `?` |
| `bl` at 0x11d900 | `(unknown callee) 0x205bc` | `[sp-1680]` | `0x18efb8` | `?` | `?` |
| `bl` at 0x11d908 | `(unknown callee) 0x2031c` | `[sp-1680]` | `?` | `?` | `?` |
| `bl` at 0x11d948 | `(unknown callee) 0x205bc` | `([sp-1680] + ret(0x2031c))` | `0x18efd8` | `?` | `[sp-1796]` |
| `bl` at 0x11d950 | `(unknown callee) 0x2031c` | `[sp-1680]` | `?` | `?` | `?` |
| `bl` at 0x11d978 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11d984 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11d9cc | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11da08 | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11da20 | `(unknown callee) 0x205bc` | `[sp-1684]` | `0x18efdc` | `sp-1676` | `sp-1676` |
| `bl` at 0x11da28 | `(unknown callee) 0x2031c` | `[sp-1684]` | `?` | `?` | `?` |
| `bl` at 0x11daa4 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `?` | `sp-1702` |
| `bl` at 0x11dac8 | `(unknown callee) 0x205bc` | `[sp-1684]` | `0x18efc0` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11dad0 | `(unknown callee) 0x2031c` | `[sp-1684]` | `?` | `?` | `?` |
| `bl` at 0x11db04 | `(unknown callee) 0x205bc` | `[sp-1684]` | `0x18efc8` | `[sp-1702]:2` | `[sp-1702]:2` |
| `bl` at 0x11db0c | `(unknown callee) 0x2031c` | `[sp-1684]` | `?` | `?` | `?` |
| `bl` at 0x11dbd8 | `(unknown callee) 0x205bc` | `([sp-1684] + ret(0x2031c))` | `0x18efd8` | `[sp-1702]:2` | `[sp-1702]:2` |
| `bl` at 0x11dbe0 | `(unknown callee) 0x2031c` | `[sp-1684]` | `?` | `?` | `?` |
| `bl` at 0x11dc08 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11dc14 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11dc5c | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11dc98 | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11dcb0 | `(unknown callee) 0x205bc` | `[sp-1688]` | `0x18efdc` | `sp-1676` | `sp-1676` |
| `bl` at 0x11dcb8 | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11dd00 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x32` | `sp-1702` |
| `bl` at 0x11dd24 | `(unknown callee) 0x205bc` | `[sp-1688]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11dd2c | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11de04 | `(unknown callee) 0x205bc` | `[sp-1688]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11de0c | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11de88 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `?` | `sp-1702` |
| `bl` at 0x11deac | `(unknown callee) 0x205bc` | `[sp-1688]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11deb4 | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11df88 | `(unknown callee) 0x205bc` | `[sp-1688]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11df90 | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11dfe0 | `(unknown callee) 0x205bc` | `([sp-1688] + ret(0x2031c))` | `0x18efd8` | `[sp-1702]:2` | `[sp-1702]:2` |
| `bl` at 0x11dfe8 | `(unknown callee) 0x2031c` | `[sp-1688]` | `?` | `?` | `?` |
| `bl` at 0x11e010 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11e01c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11e064 | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11e0a0 | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11e0b8 | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efdc` | `sp-1676` | `sp-1676` |
| `bl` at 0x11e0c0 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e108 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x3d` | `sp-1702` |
| `bl` at 0x11e12c | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11e134 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e1a0 | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11e1a8 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e224 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `?` | `sp-1702` |
| `bl` at 0x11e248 | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11e250 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e2bc | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11e2c4 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e314 | `(unknown callee) 0x205bc` | `([sp-1692] + ret(0x2031c))` | `0x18efd8` | `[sp-1702]:2` | `[sp-1702]:2` |
| `bl` at 0x11e31c | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e344 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11e350 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11e398 | `(unknown callee) 0x203f4` | `[sp-1844]` | `[sp-1852]` | `[sp-1848]` | `?` |
| `bl` at 0x11e3d4 | `(unknown callee) 0x205bc` | `sp-1676` | `0x18e5f0` | `ret(0x203f4)` | `?` |
| `bl` at 0x11e3ec | `(unknown callee) 0x205bc` | `[sp-1696]` | `0x18efdc` | `sp-1676` | `sp-1676` |
| `bl` at 0x11e3f4 | `(unknown callee) 0x2031c` | `[sp-1696]` | `?` | `?` | `?` |
| `bl` at 0x11e43c | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `0x5d` | `sp-1702` |
| `bl` at 0x11e460 | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11e468 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e534 | `(unknown callee) 0x205bc` | `[sp-1696]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11e53c | `(unknown callee) 0x2031c` | `[sp-1696]` | `?` | `?` | `?` |
| `bl` at 0x11e5b8 | `(unknown callee) 0x11f61c` | `[sp-24]` | `?` | `?` | `sp-1702` |
| `bl` at 0x11e5dc | `(unknown callee) 0x205bc` | `[sp-1692]` | `0x18efe4` | `0x18efec` | `ret(0x11f61c)` |
| `bl` at 0x11e5e4 | `(unknown callee) 0x2031c` | `[sp-1692]` | `?` | `?` | `?` |
| `bl` at 0x11e650 | `(unknown callee) 0x205bc` | `[sp-1696]` | `0x18efd0` | `?` | `?` |
| `bl` at 0x11e658 | `(unknown callee) 0x2031c` | `[sp-1696]` | `?` | `?` | `?` |
| `bl` at 0x11e6a8 | `(unknown callee) 0x205bc` | `[sp-1696]` | `0x18efd8` | `[sp-1702]:2` | `[sp-1702]:2` |
| `bl` at 0x11e6b0 | `(unknown callee) 0x2031c` | `[sp-1696]` | `?` | `?` | `?` |
| `bl` at 0x11e6e0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18ed54` | `?` | `?` |
| `bl` at 0x11e6e8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e708 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18eff0` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e710 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e730 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f00c` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e738 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e760 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f0c0` | `sp-364` | `sp-364` |
| `bl` at 0x11e768 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e788 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18edc4` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e790 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e7b0 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f028` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e7b8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e7d8 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f040` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e7e0 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e808 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f0c0` | `sp-684` | `sp-684` |
| `bl` at 0x11e810 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e830 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18edf8` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e838 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e858 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f058` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e860 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e880 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f098` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e888 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e8b0 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f0c0` | `sp-1004` | `sp-1004` |
| `bl` at 0x11e8b8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e8d8 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18eec8` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e8e0 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e900 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f078` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e908 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e928 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f098` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e930 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e95c | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f0c0` | `sp-1324` | `sp-1324` |
| `bl` at 0x11e964 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e984 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18ef34` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e98c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e9ac | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f078` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e9b4 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11e9d4 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f098` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0x11e9dc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11ea08 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x18f0c0` | `sp-1644` | `sp-1644` |
| `bl` at 0x11ea10 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x11d1b8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x11d1e0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x11d200 | `ret(0x111978) == 0x0` | `ret(0x111978) != 0x0` |
| `bne` at 0x11d248 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x11d41c | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `ble` at 0x11d530 | `[sp-1792] <= 0x4 (signed, 32-bit)` | `[sp-1792] > 0x4 (signed, 32-bit)` |
| `ble` at 0x11d544 | `[sp-1796] <= 0x4 (signed, 32-bit)` | `[sp-1796] > 0x4 (signed, 32-bit)` |
| `beq` at 0x11d594 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `ble` at 0x11d6a8 | `[sp-1800] <= 0xa (signed, 32-bit)` | `[sp-1800] > 0xa (signed, 32-bit)` |
| `ble` at 0x11d6bc | `[sp-1804] <= 0xa (signed, 32-bit)` | `[sp-1804] > 0xa (signed, 32-bit)` |
| `bge` at 0x11d7c4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11d810 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11da64 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11dab8 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11dd14 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11de48 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11de9c | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11e11c | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11e1e4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11e238 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11e450 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `bge` at 0x11e578 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x11e5cc | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11ea30 | `r0` | `([sp-32] + ret(0x2031c))` |
