# ARM L1 monitor interrupts and power paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/monitor-interrupt-power-facts.txt) retain all tool sections.

## `monitorIntDisable`

- `monitorIntDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1198 to 0xb11e0.
  Established. The two byte saves retain `(r0 & 0xff)` at sp -0x11 and `(r1 & 0xff)` at sp -0x12
  (`strb` at 0xb11b0, `strb` at 0xb11b8). The call to 0xb1b84 uses those expressions as its first
  two candidates, followed by 0x0 and `(r0 & 0xff)` (`bl` at 0xb11d0). The return carries
  `ret(0xb1b84)` in r0 (`ldmdb` at 0xb11dc). The physical interrupt-disable effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb11b0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb11b8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb11bc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb11c0 | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb11dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb11dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb11dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb11d0 | `(unknown callee) 0xb1b84` | `(r0 & 0xff)` | `(r1 & 0xff)` | `0x0` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb11dc | `r0` | `ret(0xb1b84)` |

## `monitorIntEnable`

- `monitorIntEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb11e0 to 0xb1228.
  Established. The call to 0xb1b84 has candidates `(r0 & 0xff)`, `(r1 & 0xff)`, 0x1 and `(r0 &
  0xff)` (`bl` at 0xb1218). Earlier byte stores retain the masked entry values at sp -0x11 and sp
  -0x12 (`strb` at 0xb11f8, `strb` at 0xb1200); its result reaches r0 at return (`ldmdb` at
  0xb1224). No interrupt-enable hardware contract is established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb11f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb1200 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb1204 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb1208 | load | 1 | `sp -0x12` | `-` |
| `ldmdb` at 0xb1224 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1224 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1218 | `(unknown callee) 0xb1b84` | `(r0 & 0xff)` | `(r1 & 0xff)` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1224 | `r0` | `ret(0xb1b84)` |

## `monitorInt`

- `monitorInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb12dc to 0xb14ac.
  Established. A word at sp -0x24 begins at zero before the loop, whose `bls` predicate has unknown
  operands (`str` at 0xb1304, `bls` at 0xb131c). A later `beq` takes the zero-byte
  `[ret(0xb16d0)]:1` path past a computed transfer (`beq` at 0xb1358, `ldrls` at 0xb136c). That
  `ldrls` has taken condition `[[sp-24]]:1-1 <= 0x5 (unsigned, 32-bit)` and an unresolved taken edge
  (`ldrls` at 0xb136c). Known later stores include `([sp-36] + [sp-40])` at sp -0x24 and
  `[[sp-28]+4]+1` at `[sp-28] +0x4` (`str` at 0xb146c, `str` at 0xb1480). The aggregate meaning and
  additional dispatch effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb12f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb12fc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0xb1304 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xb1308 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb130c | store | 1 | `sp -0x1d` | `((r0 & 0xff) & 0xff)` |
| `ldrb` at 0xb1310 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xb1314 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb1324 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xb1334 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldrb` at 0xb1338 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xb1348 | store | 4 | `sp -0x1c` | `ret(0xb16d0)` |
| `ldr` at 0xb134c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1350 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb135c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb1360 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xb136c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb142c | load | 4 | `abs 0xb14a0` | `-` |
| `str` at 0xb1430 | store | 4 | `sp -0x30` | `0x1089f0` |
| `ldr` at 0xb1434 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb1438 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xb143c | store | 4 | `sp -0x2c` | `[[sp-24]]:1` |
| `ldr` at 0xb1444 | load | 4 | `abs 0xb14a4` | `-` |
| `ldr` at 0xb1448 | load | 4 | `abs 0xb14a8` | `-` |
| `ldr` at 0xb1454 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1464 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb146c | store | 4 | `sp -0x24` | `([sp-36] + [sp-40])` |
| `ldr` at 0xb1470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1478 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xb1480 | store | 4 | `[sp-28] +0x4` | `[[sp-28]+4]+1` |
| `ldrb` at 0xb1484 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xb148c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0xb1494 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb149c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb149c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb149c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb132c | `(unknown callee) 0xb1638` | `?` | `?` | `?` | `?` |
| `bl` at 0xb1340 | `(unknown callee) 0xb16d0` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0xb1450 | `(unknown callee) 0x1eed0` | `0x9` | `0x10897c` | `0x4e4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb131c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xb1358 | `[ret(0xb16d0)]:1 == 0x0` | `[ret(0xb16d0)]:1 != 0x0` |
| `ldrls` at 0xb136c | `[[sp-24]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xb145c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb149c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb136c).

## `monitorName`

- `monitorName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb14ac to 0xb150c.
  Established. The first word read at `ret(0xb1638) +0x8` is saved at sp -0x1c (`ldr` at 0xb14dc,
  `str` at 0xb14e0). A second load from that location decides `bne`, whose nonzero taken path skips
  the fallback store of 0x108a5c (`ldr` at 0xb14e8, `bne` at 0xb14f0, `str` at 0xb14f8). The two
  observations need not agree. The returned value's interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb14c0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb14c4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb14d4 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldr` at 0xb14d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb14dc | load | 4 | `ret(0xb1638) +0x8` | `-` |
| `str` at 0xb14e0 | store | 4 | `sp -0x1c` | `[ret(0xb1638)+8]` |
| `ldr` at 0xb14e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb14e8 | load | 4 | `ret(0xb1638) +0x8` | `-` |
| `ldr` at 0xb14f4 | load | 4 | `abs 0xb1508` | `-` |
| `str` at 0xb14f8 | store | 4 | `sp -0x1c` | `0x108a5c` |
| `ldr` at 0xb14fc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb1504 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1504 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb14cc | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb14f0 | `[ret(0xb1638)+8] != 0x0` | `[ret(0xb1638)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1504 | `r0` | `?` |

## `monitorTest`

- `monitorTest`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb150c to 0xb1638.
  Established. Calls first target 0xb1638 and 0xb16d0 (`bl` at 0xb152c, `bl` at 0xb1540). `bne`
  takes the nonzero-byte path from `[ret(0xb16d0)]:1` to a conditional computed transfer (`bne`
  at 0xb1558, `ldrls` at 0xb1578). The `ldrls` taken condition is `[[sp-24]]:1-1 <= 0x5 (unsigned,
  32-bit)`, and its taken edge is unresolved (`ldrls` at 0xb1578). Known fall-through saves
  0xfffff830 at sp -0x20 (`str` at 0xb1618). The test behavior and additional transfer effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb1520 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb1524 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb1534 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldrb` at 0xb1538 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb1548 | store | 4 | `sp -0x1c` | `ret(0xb16d0)` |
| `ldr` at 0xb154c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1550 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb155c | load | 4 | `abs 0xb1624` | `-` |
| `str` at 0xb1560 | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0xb1568 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb156c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xb1578 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1614 | load | 4 | `abs 0xb1634` | `-` |
| `str` at 0xb1618 | store | 4 | `sp -0x20` | `0xfffff830` |
| `ldr` at 0xb161c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb1620 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1620 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1620 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb152c | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb1540 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1558 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `ldrls` at 0xb1578 | `[[sp-24]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1620 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb1578).

## `monitorVSC055Int`

- `monitorVSC055Int`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb3e68 to 0xb3fc8.
  Established. Two initial byte stores at sp -0x19 and sp -0x1a have unknown values (`strb` at
  0xb3e94, `strb` at 0xb3e9c). The loop's `blt` predicate has unknown operands, and its two `bne`
  sites retain only Z == 0 (`blt` at 0xb3eb8, `bne` at 0xb3eec, `bne` at 0xb3f1c). After the loop,
  `bne` takes the `[sp-25]:1 != 0xff` path past 0x1eed0 (`bne` at 0xb3f40, `bl` at 0xb3f74). The
  later call to 0x6712c uses candidates `[sp-25]:1`, `[sp-26]:1`, `[sp-26]:1` and `[sp-25]:1` (`bl`
  at 0xb3fa4). The return has r0 value 0x1 (`ldmdb` at 0xb3fb0). The physical interrupt meaning is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb3e7c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb3e84 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb3e8c | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xb3e94 | store | 1 | `sp -0x19` | `?` |
| `strb` at 0xb3e9c | store | 1 | `sp -0x1a` | `?` |
| `str` at 0xb3ea4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb3ea8 | load | 4 | `abs 0xb3fb4` | `-` |
| `ldr` at 0xb3eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3eb0 | load | 4 | `abs 0x2840e0` | `-` |
| `ldr` at 0xb3ec0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3ed4 | load | 4 | `abs 0xb3fb8` | `-` |
| `ldr` at 0xb3ed8 | load | 4 | `abs 0x2840c0` | `-` |
| `ldrb` at 0xb3ee0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb3ee4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb3ef0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb3f04 | load | 4 | `abs 0xb3fb8` | `-` |
| `ldr` at 0xb3f08 | load | 4 | `abs 0x2840c0` | `-` |
| `ldrb` at 0xb3f10 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xb3f14 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb3f20 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xb3f24 | store | 1 | `sp -0x19` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xb3f28 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb3f30 | store | 4 | `sp -0x18` | `?` |
| `ldrb` at 0xb3f38 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xb3f44 | load | 4 | `abs 0xb3fbc` | `-` |
| `str` at 0xb3f48 | store | 4 | `sp -0x2c` | `0x108bac` |
| `ldrb` at 0xb3f4c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb3f50 | store | 4 | `sp -0x28` | `[sp-17]:1` |
| `ldrb` at 0xb3f54 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb3f58 | store | 4 | `sp -0x24` | `[sp-18]:1` |
| `ldrb` at 0xb3f5c | load | 1 | `sp -0x13` | `-` |
| `str` at 0xb3f60 | store | 4 | `sp -0x20` | `[sp-19]:1` |
| `ldr` at 0xb3f68 | load | 4 | `abs 0xb3fc0` | `-` |
| `ldr` at 0xb3f6c | load | 4 | `abs 0xb3fc4` | `-` |
| `ldrb` at 0xb3f78 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0xb3f90 | store | 1 | `sp -0x1a` | `?` |
| `ldrb` at 0xb3f94 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xb3f98 | load | 1 | `sp -0x1a` | `-` |
| `ldmdb` at 0xb3fb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb3fb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb3fb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb3f74 | `(unknown callee) 0x1eed0` | `0x1` | `0x10897c` | `0xb43` | `0x0` |
| `bl` at 0xb3fa4 | `(unknown callee) 0x6712c` | `[sp-25]:1` | `[sp-26]:1` | `[sp-26]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb3eb8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb3eec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb3f1c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb3f40 | `[sp-25]:1 != 0xff` | `[sp-25]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb3fb0 | `r0` | `0x1` |

## `powerState`

- `powerState`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ab58 to 0x3ad74.
  Established. The first call targets 0x399d0, followed by `bne`, whose taken nonzero-result path
  skips the save of 0xfffff9b9 at sp -0x34 (`bl` at 0x3ab70, `bne` at 0x3ab7c, `str` at 0x3ab84). A
  later `bne` takes the non-0xff byte path from `[[sp-40]+4]:1` past a save of 0x2 to that slot
  (`bne` at 0x3ac48, `str` at 0x3ac50). The signed `bgt` on `ret(0xd6148) > 0x0` and `blt` on
  `ret(0xd6150) < 0x0` both take paths to a save of 0x1 (`bgt` at 0x3acdc, `blt` at 0x3ad38, `str`
  at 0x3ad44). The alternate path saves 0x2 (`str` at 0x3ad50). The physical state encoding is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ab68 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x3ab6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ab80 | load | 4 | `abs 0x3ad6c` | `-` |
| `str` at 0x3ab84 | store | 4 | `sp -0x34` | `0xfffff9b9` |
| `ldr` at 0x3ab8c | load | 4 | `abs 0x3ad70` | `-` |
| `ldr` at 0x3ab90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3aba4 | load | 4 | `abs 0x281120` | `-` |
| `str` at 0x3abac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3abb0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3abb4 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x3abc0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3abc4 | load | 1 | `[sp-40] +0x2` | `-` |
| `ldr` at 0x3abd4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3abd8 | load | 1 | `[sp-40] +0x3` | `-` |
| `str` at 0x3abe4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3abe8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3abfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ac0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ac34 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3ac3c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3ac40 | load | 1 | `[sp-40] +0x4` | `-` |
| `str` at 0x3ac50 | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0x3ac58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ac80 | store | 4 | `sp -0x2c` | `ret(0x3d538)` |
| `ldr` at 0x3ac84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3ac94 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x3acb0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3acb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3ace0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3acf0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x3ad0c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3ad0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3ad44 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0x3ad50 | store | 4 | `sp -0x34` | `0x2` |
| `ldr` at 0x3ad54 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ad58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ab70 | `(unknown callee) 0x399d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3abcc | `(unknown callee) 0xb7b78` | `[[sp-40]+2]:1` | `0x281120` | `?` | `[[sp-40]+2]:1` |
| `bl` at 0x3abec | `(unknown callee) 0x3dba8` | `[sp-40]` | `?` | `ret(0xb7b78)` | `?` |
| `bl` at 0x3ac10 | `(unknown callee) 0x3dba8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x3ac5c | `(unknown callee) 0x3da54` | `[sp-40]` | `?` | `?` | `[[sp-40]+4]:1` |
| `bl` at 0x3ac70 | `(unknown callee) 0x3cd7c` | `ret(0x3da54)` | `0x0` | `sp-48` | `ret(0x3da54)` |
| `bl` at 0x3ac78 | `(unknown callee) 0x3d538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x3ac88 | `(unknown callee) 0xd5c38` | `[sp-48]` | `?` | `?` | `ret(0x3d538)` |
| `bl` at 0x3ac98 | `(unknown callee) 0xd5c38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3acb4 | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `?` | `?` |
| `bl` at 0x3acd0 | `(unknown callee) 0xd6148` | `ret(0xd5c38)` | `?` | `ret(0xd5c84)` | `?` |
| `bl` at 0x3ace4 | `(unknown callee) 0xd5c38` | `[sp-48]` | `?` | `?` | `ret(0xd6148)` |
| `bl` at 0x3acf4 | `(unknown callee) 0xd5c38` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3ad10 | `(unknown callee) 0xd5c84` | `ret(0xd5c38)` | `?` | `?` | `?` |
| `bl` at 0x3ad2c | `(unknown callee) 0xd6150` | `ret(0xd5c38)` | `?` | `ret(0xd5c84)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ab7c | `ret(0x399d0) != 0x0` | `ret(0x399d0) == 0x0` |
| `beq` at 0x3abbc | `[[sp-40]+2]:1 == 0xff` | `[[sp-40]+2]:1 != 0xff` |
| `beq` at 0x3abf8 | `ret(0x3dba8) == 0x0` | `ret(0x3dba8) != 0x0` |
| `bne` at 0x3ac04 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x3ac1c | `ret(0x3dba8) != 0x0` | `ret(0x3dba8) == 0x0` |
| `bne` at 0x3ac28 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x3ac48 | `[[sp-40]+4]:1 != 0xff` | `[[sp-40]+4]:1 == 0xff` |
| `bgt` at 0x3acdc | `ret(0xd6148) > 0x0 (signed, 32-bit)` | `ret(0xd6148) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x3ad38 | `ret(0xd6150) < 0x0 (signed, 32-bit)` | `ret(0xd6150) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ad58 | `r0` | `?` |

## `powerSetMargin`

- `powerSetMargin`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ad74 to 0x3ae7c.
  Established. The call to 0x3dea4 has first candidate `ret(0x3dae8)` and second candidate
  `[sp-21]:1` (`bl` at 0x3ade8). `beq` takes its zero-result path to the word store of 0x4 at
  0x1155e0, followed by 0x5ee50 with first candidate 0x4 (`beq` at 0x3adfc, `str` at 0x3ae28, `bl`
  at 0x3ae30). Nonzero fall-through instead writes 0x5 at that global and calls 0x5ee50 with first
  candidate 0x5 (`str` at 0x3ae08, `bl` at 0x3ae10). Later calls to 0x1abd0 include first candidates
  `[[sp-28]+16]:1+20` and `[[sp-28]+16]:1+130` (`bl` at 0x3ae48, `bl` at 0x3ae64). The margin and
  notification meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ad84 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3ad8c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x3ad90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ada8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3adb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3adbc | store | 4 | `sp -0x1c` | `ret(0x3dae8)` |
| `ldr` at 0x3adc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3add4 | store | 4 | `sp -0x24` | `ret(0x3db60)` |
| `ldrb` at 0x3addc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3ade0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3adf0 | store | 4 | `sp -0x20` | `ret(0x3dea4)` |
| `ldr` at 0x3adf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ae00 | load | 4 | `abs 0x3ae78` | `-` |
| `str` at 0x3ae08 | store | 4 | `abs 0x1155e0` | `0x5` |
| `ldr` at 0x3ae14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ae18 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x3ae20 | load | 4 | `abs 0x3ae78` | `-` |
| `str` at 0x3ae28 | store | 4 | `abs 0x1155e0` | `0x4` |
| `ldr` at 0x3ae34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ae38 | load | 1 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x3ae4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3ae50 | load | 1 | `[sp-28] +0x10` | `-` |
| `ldrb` at 0x3ae58 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x3ae6c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3ae70 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3ae74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ae74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ae74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ad94 | `(unknown callee) 0x399d0` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x3adb4 | `(unknown callee) 0x3dae8` | `[sp-20]` | `?` | `?` | `ret(0x399d0)` |
| `bl` at 0x3adcc | `(unknown callee) 0x3db60` | `ret(0x3dae8)` | `?` | `?` | `ret(0x3dae8)` |
| `bl` at 0x3ade8 | `(unknown callee) 0x3dea4` | `ret(0x3dae8)` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x3ae10 | `(unknown callee) 0x5ee50` | `0x5` | `?` | `0x5` | `0x1155e0` |
| `bl` at 0x3ae30 | `(unknown callee) 0x5ee50` | `0x4` | `?` | `0x1155e0` | `0x4` |
| `bl` at 0x3ae48 | `(unknown callee) 0x1abd0` | `[[sp-28]+16]:1+20` | `0x4` | `?` | `[[sp-28]+16]:1+20` |
| `bl` at 0x3ae64 | `(unknown callee) 0x1abd0` | `[[sp-28]+16]:1+130` | `[sp-21]:1` | `[sp-21]:1` | `[[sp-28]+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3ada0 | `ret(0x399d0) != 0x0` | `ret(0x399d0) == 0x0` |
| `bne` at 0x3adc8 | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `beq` at 0x3adfc | `ret(0x3dea4) == 0x0` | `ret(0x3dea4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ae74 | `r0` | `?` |

## `powerMarginName`

- `powerMarginName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d1d0 to 0x3d2e0.
  Established. After the call to 0x3dae8, `bne` takes the nonzero-result path past the save of
  0xe1b28 at sp -0x1c (`bl` at 0x3d1e8, `bne` at 0x3d1fc, `str` at 0x3d204). The later call to
  0x1c79c has first candidate `[ret(0x3dae8)+16]:1+20` (`bl` at 0x3d22c). The following `ldrls`
  taken condition is `ret(0x1c79c) <= 0x5 (unsigned, 32-bit)`, with an unresolved taken edge
  (`ldrls` at 0x3d238). Known fall-through saves 0xe1b58 at sp -0x1c (`str` at 0x3d2a4).
  Returned-value interpretation and additional dispatch effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d1e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d1e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d1f0 | store | 4 | `sp -0x18` | `ret(0x3dae8)` |
| `ldr` at 0x3d1f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d200 | load | 4 | `abs 0x3d2bc` | `-` |
| `str` at 0x3d204 | store | 4 | `sp -0x1c` | `0xe1b28` |
| `ldr` at 0x3d20c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3d210 | load | 1 | `ret(0x3dae8) +0x6` | `-` |
| `ldr` at 0x3d21c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3d220 | load | 1 | `ret(0x3dae8) +0x10` | `-` |
| `ldrls` at 0x3d238 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d2a0 | load | 4 | `abs 0x3d2d8` | `-` |
| `str` at 0x3d2a4 | store | 4 | `sp -0x1c` | `0xe1b58` |
| `ldr` at 0x3d2ac | load | 4 | `abs 0x3d2dc` | `-` |
| `str` at 0x3d2b0 | store | 4 | `sp -0x1c` | `0xe1b60` |
| `ldr` at 0x3d2b4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d2b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d2b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d2b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d1e8 | `(unknown callee) 0x3dae8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3d22c | `(unknown callee) 0x1c79c` | `[ret(0x3dae8)+16]:1+20` | `?` | `?` | `[ret(0x3dae8)+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3d1fc | `ret(0x3dae8) != 0x0` | `ret(0x3dae8) == 0x0` |
| `beq` at 0x3d218 | `[ret(0x3dae8)+6]:1 == 0xff` | `[ret(0x3dae8)+6]:1 != 0xff` |
| `ldrls` at 0x3d238 | `ret(0x1c79c) <= 0x5 (unsigned, 32-bit)` | `ret(0x1c79c) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d2b8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d238).

## `powerGetName`

- `powerGetName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d108 to 0x3d1d0.
  Established. The routine reads the word at 0x281120 at three separate sites (`ldr` at 0x3d134,
  `ldr` at 0x3d160, `ldr` at 0x3d190); its `beq` tests an unknown expression against zero (`beq` at
  0x3d144). The two paths save unknown word values at sp -0x18 (`str` at 0x3d16c, `str` at 0x3d1bc).
  The return retains `[sp-24]` (`ldmdb` at 0x3d1c4). The name meaning suggested by the symbol
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d118 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d11c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d130 | load | 4 | `abs 0x3d1c8` | `-` |
| `ldr` at 0x3d134 | load | 4 | `abs 0x281120` | `-` |
| `ldr` at 0x3d13c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d15c | load | 4 | `abs 0x3d1c8` | `-` |
| `ldr` at 0x3d160 | load | 4 | `abs 0x281120` | `-` |
| `ldr` at 0x3d168 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d16c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3d174 | load | 4 | `abs 0x3d1cc` | `-` |
| `ldr` at 0x3d178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d18c | load | 4 | `abs 0x3d1c8` | `-` |
| `ldr` at 0x3d190 | load | 4 | `abs 0x281120` | `-` |
| `ldrb` at 0x3d198 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3d1b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3d1bc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3d1c0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3d1c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d1c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d1c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d144 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d1c4 | `r0` | `[sp-24]` |

## `powerUpInProgress`

- `powerUpInProgress`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dda4 to 0x3ddc4.
  Established. A word read at 0x119de0 supplies r0 expression `[0x119de0]` at return (`ldr` at
  0x3ddb4, `ldmdb` at 0x3ddbc). The word's power-progress meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3ddb0 | load | 4 | `abs 0x3ddc0` | `-` |
| `ldr` at 0x3ddb4 | load | 4 | `abs 0x119de0` | `-` |
| `ldmdb` at 0x3ddbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ddbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ddbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ddbc | `r0` | `[0x119de0]` |

## `powerUpFirstReset`

- `powerUpFirstReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ddc4 to 0x3dde4.
  Established. The word loaded from 0x119ddc reaches the return in r0 as `[0x119ddc]` (`ldr` at
  0x3ddd4, `ldmdb` at 0x3dddc). This body does not establish the returned word's reset meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x3ddd0 | load | 4 | `abs 0x3dde0` | `-` |
| `ldr` at 0x3ddd4 | load | 4 | `abs 0x119ddc` | `-` |
| `ldmdb` at 0x3dddc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dddc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dddc | `r0` | `[0x119ddc]` |

## `pcixPowerUp`

- `pcixPowerUp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49b2c to 0x49bd8.
  Established. Two word slots at sp -0x14 and sp -0x1c begin at zero (`str` at 0x49b40, `str` at
  0x49b48). The loop's `blt` has unknown operands (`blt` at 0x49b5c). The call to 0x4a694 uses first
  candidate unknown and second candidate 0x119e98 (`bl` at 0x49b88); its following `beq` takes the
  zero-result path past the result save at sp -0x14, while `bne` also skips that save for nonzero
  `[sp-20]` (`beq` at 0x49b9c, `bne` at 0x49ba8, `str` at 0x49bb0). The iteration count and physical
  power-up sequence are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49b40 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x49b48 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49b4c | load | 4 | `abs 0x49bd0` | `-` |
| `ldr` at 0x49b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49b54 | load | 4 | `abs 0x119e9c` | `-` |
| `ldr` at 0x49b64 | load | 4 | `abs 0x49bd4` | `-` |
| `ldr` at 0x49b68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49b7c | load | 4 | `abs 0x119e98` | `-` |
| `str` at 0x49b90 | store | 4 | `sp -0x18` | `ret(0x4a694)` |
| `ldr` at 0x49b94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49ba0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49bac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49bb0 | store | 4 | `sp -0x14` | `ret(0x4a694)` |
| `ldr` at 0x49bb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49bbc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x49bc4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x49bcc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49bcc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49bcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49b88 | `(unknown callee) 0x4a694` | `?` | `0x119e98` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x49b5c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49b9c | `ret(0x4a694) == 0x0` | `ret(0x4a694) != 0x0` |
| `bne` at 0x49ba8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49bcc | `r0` | `?` |

## `pcixPowerDown`

- `pcixPowerDown`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49bd8 to 0x49c84.
  Established. Before the loop test, word slots at sp -0x14 and sp -0x1c are initialized to 0x0
  (`str` at 0x49bec, `str` at 0x49bf4). The loop test uses a `blt` predicate with unknown operands
  (`blt` at 0x49c08). The call
  to 0x4acb4 has first candidate unknown and second candidate 0x119e98 (`bl` at 0x49c34). The later
  `beq` takes the zero-result path past the save of `ret(0x4acb4)` at sp -0x14 (`beq` at 0x49c48,
  `str` at 0x49c5c). The back-edge follows a word store of `[sp-28]+1` at sp -0x1c (`str` at
  0x49c68, `b` at 0x49c6c). The physical power-down sequence remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x49bec | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x49bf4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49bf8 | load | 4 | `abs 0x49c7c` | `-` |
| `ldr` at 0x49bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49c00 | load | 4 | `abs 0x119e9c` | `-` |
| `ldr` at 0x49c10 | load | 4 | `abs 0x49c80` | `-` |
| `ldr` at 0x49c14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49c28 | load | 4 | `abs 0x119e98` | `-` |
| `str` at 0x49c3c | store | 4 | `sp -0x18` | `ret(0x4acb4)` |
| `ldr` at 0x49c40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49c4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49c58 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49c5c | store | 4 | `sp -0x14` | `ret(0x4acb4)` |
| `ldr` at 0x49c60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49c68 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x49c70 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x49c78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49c78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49c78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49c34 | `(unknown callee) 0x4acb4` | `?` | `0x119e98` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x49c08 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49c48 | `ret(0x4acb4) == 0x0` | `ret(0x4acb4) != 0x0` |
| `bne` at 0x49c54 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49c78 | `r0` | `?` |

## `eeprom_free_pirom_stub`

- `eeprom_free_pirom_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac084 to 0xac09c.
  Established. A word store at sp -0x14 retains entry r0 (`str` at 0xac094). The return also carries
  entry r0 (`ldmdb` at 0xac098). The PIROM-release behavior suggested by the symbol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac094 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0xac098 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac098 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac098 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac098 | `r0` | `r0` |
