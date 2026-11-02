# ARM L1 monitor entry paths in 1.34.2

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

[Fact sheets and full listings](disasm/arm-1.34.2-package3/monitor-facts.txt) retain all tool sections.

## `monitorReset`

- `monitorReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafd30 to 0xaff10.
  Established. A byte store initializes sp -0x19 to 0x0 before the loop test, whose `blt` predicate
  has unknown operands (`strb` at 0xafd44, `blt` at 0xafd58). A later `bne` takes the non-0x63 byte path
  past two byte-clear stores, one at an unknown target and one at `[sp-24] +0x0` (`bne` at 0xafd94,
  `strb` at 0xafdb0, `strb` at 0xafdbc). The `ldrls` taken condition is `[[sp-20]]:1-1 <= 0x5
  (unsigned, 32-bit)`, with an unresolved taken edge (`ldrls` at 0xafde0); its known fall-through
  reaches 0x1eed0 with candidates 0x9, 0x10897c, 0x14c and 0x0 (`bl` at 0xafe9c). Additional
  dispatch effects and physical reset behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xafd44 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xafd48 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafd4c | load | 4 | `abs 0xafef8` | `-` |
| `ldr` at 0xafd50 | load | 4 | `abs 0x2840e0` | `-` |
| `ldrb` at 0xafd60 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xafd70 | store | 4 | `sp -0x14` | `ret(0xb1638)` |
| `ldrb` at 0xafd74 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xafd84 | store | 4 | `sp -0x18` | `ret(0xb16d0)` |
| `ldr` at 0xafd88 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xafd8c | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xafd98 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xafda0 | load | 4 | `abs 0xafefc` | `-` |
| `ldr` at 0xafda4 | load | 4 | `abs 0x2840a0` | `-` |
| `strb` at 0xafdb0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xafdb4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xafdbc | store | 1 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0xafdc0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xafdc4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xafdd0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xafdd4 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrls` at 0xafde0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafe78 | load | 4 | `abs 0xaff00` | `-` |
| `str` at 0xafe7c | store | 4 | `sp -0x2c` | `0x1089f0` |
| `ldr` at 0xafe80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xafe84 | load | 1 | `[sp-20] +0x0` | `-` |
| `str` at 0xafe88 | store | 4 | `sp -0x28` | `[[sp-20]]:1` |
| `ldr` at 0xafe90 | load | 4 | `abs 0xaff04` | `-` |
| `ldr` at 0xafea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xafeac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xafeb8 | load | 4 | `abs 0xaff08` | `-` |
| `str` at 0xafebc | store | 4 | `sp -0x2c` | `0x108a04` |
| `ldr` at 0xafec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafec4 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xafec8 | store | 4 | `sp -0x28` | `[[sp-20]+8]` |
| `str` at 0xafecc | store | 4 | `sp -0x24` | `ret(0x1ea50)` |
| `ldr` at 0xafed4 | load | 4 | `abs 0xaff04` | `-` |
| `ldr` at 0xafed8 | load | 4 | `abs 0xaff0c` | `-` |
| `ldrb` at 0xafee4 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xafeec | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0xafef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafd68 | `(unknown callee) 0xb1638` | `?` | `?` | `[0x2840e0]` | `?` |
| `bl` at 0xafd7c | `(unknown callee) 0xb16d0` | `[sp-25]:1` | `?` | `?` | `[sp-25]:1` |
| `bl` at 0xafe9c | `(unknown callee) 0x1eed0` | `0x9` | `0x10897c` | `0x14c` | `0x0` |
| `bl` at 0xafeb0 | `(unknown callee) 0x1ea50` | `[sp-32]` | `?` | `?` | `[sp-32]` |
| `bl` at 0xafee0 | `(unknown callee) 0x1eed0` | `0x1` | `0x10897c` | `0x151` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xafd58 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xafd94 | `[[sp-20]]:1 != 0x63` | `[[sp-20]]:1 == 0x63` |
| `beq` at 0xafdcc | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xafde0 | `[[sp-20]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-20]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xafea8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafef4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xafde0).

## `monitorGetCfg`

- `monitorGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaff10 to 0xaff4c.
  Established. The word read at 0x2840c0 supplies the store of `[0x2840c0]` at entry r0 +0x0 (`ldr`
  at 0xaff2c, `str` at 0xaff30). The separate word load from 0x2840e0 supplies the return expression
  `[0x2840e0]` (`ldr` at 0xaff38, `ldmdb` at 0xaff40). The words' configuration interpretations
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaff20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaff24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff28 | load | 4 | `abs 0xaff44` | `-` |
| `ldr` at 0xaff2c | load | 4 | `abs 0x2840c0` | `-` |
| `str` at 0xaff30 | store | 4 | `r0 +0x0` | `[0x2840c0]` |
| `ldr` at 0xaff34 | load | 4 | `abs 0xaff48` | `-` |
| `ldr` at 0xaff38 | load | 4 | `abs 0x2840e0` | `-` |
| `ldmdb` at 0xaff40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaff40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaff40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaff40 | `r0` | `[0x2840e0]` |

## `monitorCount`

- `monitorCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaff4c to 0xaff6c.
  Established. The word loaded from 0x2840e0 becomes r0 expression `[0x2840e0]` at return (`ldr` at
  0xaff5c, `ldmdb` at 0xaff64). The returned word's meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaff58 | load | 4 | `abs 0xaff68` | `-` |
| `ldr` at 0xaff5c | load | 4 | `abs 0x2840e0` | `-` |
| `ldmdb` at 0xaff64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaff64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaff64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaff64 | `r0` | `[0x2840e0]` |

## `monitorPresent`

- `monitorPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaff6c to 0xaffa8.
  Established. A call to 0xb16d0 uses first candidate `(r0 & 0xff)` (`bl` at 0xaff8c). The byte load
  at its result +0x0 supplies `[ret(0xb16d0)]:1` at return (`ldrb` at 0xaff9c, `ldmdb` at 0xaffa4).
  The physical presence semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xaff80 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xaff84 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xaff94 | store | 4 | `sp -0x18` | `ret(0xb16d0)` |
| `ldr` at 0xaff98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xaff9c | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldmdb` at 0xaffa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaffa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaffa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaff8c | `(unknown callee) 0xb16d0` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaffa4 | `r0` | `[ret(0xb16d0)]:1` |

## `monitorSetVoltageLimits`

- `monitorSetVoltageLimits`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaffa8 to 0xb023c.
  Established. The first two calls target 0xb1638 and 0xb16d0 (`bl` at 0xaffe4, `bl` at 0xafff8).
  The following `bne` takes the nonzero-byte path from `[ret(0xb16d0)]:1` (`bne` at 0xb0010).
  Zero fall-through saves 0xfffffc11 at sp -0x28 (`str` at 0xb0018). Two later conditional computed
  transfers have taken conditions `[sp-18]:1 <= 0x9` and `[sp-18]:1 <= 0x8`, both unsigned 32-bit,
  with unresolved taken edges (`ldrls` at 0xb0050, `ldrls` at 0xb010c). Another known path saves
  0xfffff830 at sp -0x28 (`str` at 0xb01ac). The voltage-limit register selection and additional
  dispatch effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xaffc0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xaffc8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xaffd0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xaffd8 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xaffdc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xaffec | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldrb` at 0xafff0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0000 | store | 4 | `sp -0x1c` | `ret(0xb16d0)` |
| `ldr` at 0xb0004 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0008 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb0014 | load | 4 | `abs 0xb0234` | `-` |
| `str` at 0xb0018 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xb0020 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0024 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xb0028 | store | 4 | `sp -0x2c` | `[[sp-24]]:1` |
| `ldr` at 0xb002c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0038 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb0048 | load | 1 | `sp -0x12` | `-` |
| `ldrls` at 0xb0050 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb00fc | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xb0104 | load | 1 | `sp -0x12` | `-` |
| `ldrls` at 0xb010c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb01a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb01a8 | load | 4 | `abs 0xb0238` | `-` |
| `str` at 0xb01ac | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xb022c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb0230 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0230 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0230 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaffe4 | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xafff8 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0010 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb0034 | `[[sp-24]]:1 == 0x1` | `[[sp-24]]:1 != 0x1` |
| `beq` at 0xb0040 | `[[sp-24]]:1 == 0x2` | `[[sp-24]]:1 != 0x2` |
| `ldrls` at 0xb0050 | `[sp-18]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-18]:1 > 0x9 (unsigned, 32-bit)` |
| `ldrls` at 0xb010c | `[sp-18]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-18]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0230 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb0050).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb010c).

## `monitorSetTempLimits`

- `monitorSetTempLimits`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb023c to 0xb04f0.
  Established. Four byte saves retain masked entry r0 through r3 at sp -0x11 through sp -0x14
  (`strb` at 0xb0254, `strb` at 0xb025c, `strb` at 0xb0264, `strb` at 0xb026c). After the two calls
  to 0xb1638 and 0xb16d0, `bne` takes the nonzero-byte path from `[ret(0xb16d0)]:1` to a
  computed transfer (`bl` at 0xb0278, `bl` at 0xb028c, `bne` at 0xb02a4, `ldrls` at 0xb02c4). The
  `ldrls` taken condition is `[[sp-24]]:1-1 <= 0x4 (unsigned, 32-bit)`, and its taken edge is
  unresolved (`ldrls` at 0xb02c4). Known fall-through saves 0xfffff830 at sp -0x28 (`str` at
  0xb0460). The temperature-limit units and further transfer effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb0254 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb025c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb0264 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xb026c | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xb0270 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0280 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldrb` at 0xb0284 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0294 | store | 4 | `sp -0x1c` | `ret(0xb16d0)` |
| `ldr` at 0xb0298 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb029c | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb02a8 | load | 4 | `abs 0xb04e8` | `-` |
| `str` at 0xb02ac | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xb02b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb02b8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xb02c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb045c | load | 4 | `abs 0xb04ec` | `-` |
| `str` at 0xb0460 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xb04e0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb04e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb04e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb04e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0278 | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb028c | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb02a4 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `ldrls` at 0xb02c4 | `[[sp-24]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb04e4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb02c4).

## `monitorSetTachLimit`

- `monitorSetTachLimit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb04f0 to 0xb0614.
  Established. After the call to 0xb16d0, `bne` takes the nonzero-byte path to tests of
  `[[sp-28]]:1` (`bl` at 0xb0530, `bne` at 0xb0548). The `beq` sites select paths for that byte
  equal to 0x2, 0x1 and 0x6 (`beq` at 0xb056c, `beq` at 0xb0584, `beq` at 0xb0594). Those paths
  reach calls to 0xb2d8c, 0xb2368 and 0xb3d6c respectively, each with candidates `[sp-28]`,
  `[sp-18]:1`, `[sp-24]` and `[sp-18]:1` (`bl` at 0xb05cc, `bl` at 0xb05ac, `bl` at 0xb05ec). The
  physical tachometer-limit behavior and units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0504 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb0508 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb0510 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb0514 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0524 | store | 4 | `sp -0x1c` | `ret(0xb1638)` |
| `ldrb` at 0xb0528 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0538 | store | 4 | `sp -0x20` | `ret(0xb16d0)` |
| `ldr` at 0xb053c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0540 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb054c | load | 4 | `abs 0xb060c` | `-` |
| `str` at 0xb0550 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xb0558 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb055c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xb0560 | store | 4 | `sp -0x28` | `[[sp-28]]:1` |
| `ldr` at 0xb0564 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0570 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb057c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb058c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb059c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb05a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb05a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb05b4 | store | 4 | `sp -0x24` | `ret(0xb2368)` |
| `ldrb` at 0xb05bc | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb05c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb05c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb05d4 | store | 4 | `sp -0x24` | `ret(0xb2d8c)` |
| `ldrb` at 0xb05dc | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb05e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb05e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb05f4 | store | 4 | `sp -0x24` | `ret(0xb3d6c)` |
| `ldr` at 0xb05fc | load | 4 | `abs 0xb0610` | `-` |
| `str` at 0xb0600 | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0xb0604 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb0608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb051c | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb0530 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb05ac | `(unknown callee) 0xb2368` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |
| `bl` at 0xb05cc | `(unknown callee) 0xb2d8c` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |
| `bl` at 0xb05ec | `(unknown callee) 0xb3d6c` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0548 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb056c | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xb0578 | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb0584 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xb0594 | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0608 | `r0` | `?` |

## `monitorReadVoltage`

- `monitorReadVoltage`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0614 to 0xb07d4.
  Established. The routine first writes halfword 0x0 at entry r2 +0x0 (`strh` at 0xb0640). A later
  `beq` takes the byte-equal-to-0x2 path to 0xb29ec, with candidates `[sp-28]`, `[sp-18]:1`,
  `[sp-24]` and `[sp-18]:1` (`beq` at 0xb06a8, `bl` at 0xb074c). Another path reaches a conditional
  computed transfer with taken condition `[sp-18]:1 <= 0x9 (unsigned, 32-bit)` and an unresolved
  taken edge (`ldrls` at 0xb06b8). Known paths save 0xfffffc11 or 0xfffff830 at sp -0x2c (`str` at
  0xb0680, `str` at 0xb0760). The physical voltage-reading units and additional dispatch effects are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0628 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb062c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb0634 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xb0638 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xb0640 | store | 2 | `r2 +0x0` | `0x0` |
| `ldrb` at 0xb0644 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0654 | store | 4 | `sp -0x1c` | `ret(0xb1638)` |
| `ldrb` at 0xb0658 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0668 | store | 4 | `sp -0x20` | `ret(0xb16d0)` |
| `ldr` at 0xb066c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0670 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb067c | load | 4 | `abs 0xb07cc` | `-` |
| `str` at 0xb0680 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xb0688 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb068c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xb0690 | store | 4 | `sp -0x30` | `[[sp-28]]:1` |
| `ldr` at 0xb0694 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb06a0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xb06b0 | load | 1 | `sp -0x12` | `-` |
| `ldrls` at 0xb06b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0734 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xb073c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0740 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0748 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0754 | store | 4 | `sp -0x2c` | `ret(0xb29ec)` |
| `ldr` at 0xb075c | load | 4 | `abs 0xb07d0` | `-` |
| `str` at 0xb0760 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xb07c4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb07c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb07c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb07c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb064c | `(unknown callee) 0xb1638` | `[sp-17]:1` | `r1` | `r2` | `[sp-17]:1` |
| `bl` at 0xb0660 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb074c | `(unknown callee) 0xb29ec` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0678 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb069c | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xb06a8 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `ldrls` at 0xb06b8 | `[sp-18]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-18]:1 > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb07c8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb06b8).

## `monitorReadTemp`

- `monitorReadTemp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb07d4 to 0xb09b4.
  Established. A byte store clears entry r2 +0x0 before the calls to 0xb1638 and 0xb16d0 (`strb` at
  0xb0800, `bl` at 0xb080c, `bl` at 0xb0820). The later `ldrls` taken condition is `[[sp-28]]:1-1 <=
  0x4 (unsigned, 32-bit)`, with an unresolved taken edge (`ldrls` at 0xb0858); its known
  fall-through saves 0xfffff830 at sp -0x28 (`str` at 0xb0970). The temperature-read protocol, units
  and additional transfer effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb07e8 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb07ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb07f4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xb07f8 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xb0800 | store | 1 | `r2 +0x0` | `0x0` |
| `ldrb` at 0xb0804 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0814 | store | 4 | `sp -0x1c` | `ret(0xb1638)` |
| `ldrb` at 0xb0818 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0828 | store | 4 | `sp -0x20` | `ret(0xb16d0)` |
| `ldr` at 0xb082c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0830 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb083c | load | 4 | `abs 0xb09ac` | `-` |
| `str` at 0xb0840 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xb0848 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb084c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xb0858 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb096c | load | 4 | `abs 0xb09b0` | `-` |
| `str` at 0xb0970 | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xb09a4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb09a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb09a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb09a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb080c | `(unknown callee) 0xb1638` | `[sp-17]:1` | `r1` | `r2` | `[sp-17]:1` |
| `bl` at 0xb0820 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0838 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `ldrls` at 0xb0858 | `[[sp-28]]:1-1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb09a8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb0858).

## `monitorReadTach`

- `monitorReadTach`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb09b4 to 0xb0ae4.
  Established. The initial word store clears entry r2 +0x0 (`str` at 0xb09e0). The `beq` sites later
  select paths for `[[sp-28]]:1` equal to 0x2, 0x1 and 0x6 (`beq` at 0xb0a3c, `beq` at 0xb0a54,
  `beq` at 0xb0a64). Those paths call 0xb2c00, 0xb2258 and 0xb3c94 respectively, with candidates
  `[sp-28]`, `[sp-18]:1`, `[sp-24]` and `[sp-18]:1` at each call (`bl` at 0xb0a9c, `bl` at 0xb0a7c,
  `bl` at 0xb0abc). The measured tachometer quantity and units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb09c8 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb09cc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb09d4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xb09d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb09e0 | store | 4 | `r2 +0x0` | `0x0` |
| `ldrb` at 0xb09e4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb09f4 | store | 4 | `sp -0x1c` | `ret(0xb1638)` |
| `ldrb` at 0xb09f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0a08 | store | 4 | `sp -0x20` | `ret(0xb16d0)` |
| `ldr` at 0xb0a0c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0a10 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb0a1c | load | 4 | `abs 0xb0adc` | `-` |
| `str` at 0xb0a20 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xb0a28 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0a2c | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xb0a30 | store | 4 | `sp -0x28` | `[[sp-28]]:1` |
| `ldr` at 0xb0a34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb0a5c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb0a6c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0a70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0a78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0a84 | store | 4 | `sp -0x24` | `ret(0xb2258)` |
| `ldrb` at 0xb0a8c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0a98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0aa4 | store | 4 | `sp -0x24` | `ret(0xb2c00)` |
| `ldrb` at 0xb0aac | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0ab8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0ac4 | store | 4 | `sp -0x24` | `ret(0xb3c94)` |
| `ldr` at 0xb0acc | load | 4 | `abs 0xb0ae0` | `-` |
| `str` at 0xb0ad0 | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0xb0ad4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb0ad8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0ad8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0ad8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb09ec | `(unknown callee) 0xb1638` | `[sp-17]:1` | `r1` | `r2` | `[sp-17]:1` |
| `bl` at 0xb0a00 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb0a7c | `(unknown callee) 0xb2258` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |
| `bl` at 0xb0a9c | `(unknown callee) 0xb2c00` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |
| `bl` at 0xb0abc | `(unknown callee) 0xb3c94` | `[sp-28]` | `[sp-18]:1` | `[sp-24]` | `[sp-18]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0a18 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb0a3c | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xb0a48 | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb0a54 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xb0a64 | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0ad8 | `r0` | `?` |

## `monitorWriteOutput`

- `monitorWriteOutput`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0ae4 to 0xb0d38.
  Established. A `beq` takes the saved-byte zero path to the byte store of 0x30 at sp -0x1d and 0x5c
  at sp -0x1e (`beq` at 0xb0be0, `strb` at 0xb0bfc, `strb` at 0xb0c04). Separate `beq` paths for 0x1
  and 0x2 select the store pairs 0x31/0x5d and 0x32/0x5e at those respective slots (`beq` at
  0xb0bc8, `beq` at 0xb0bf0, `strb` at 0xb0c10, `strb` at 0xb0c18, `strb` at 0xb0c24, `strb` at
  0xb0c2c). The following call to 0xabb28 has fourth candidate 0xe2 (`bl` at 0xb0c60). A later call
  to that target has third candidate unknown and fourth candidate `[sp-19]:1` (`bl` at 0xb0d18). The
  output-register meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb0af8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb0b00 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xb0b08 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xb0b0c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0b1c | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldrb` at 0xb0b20 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0b30 | store | 4 | `sp -0x1c` | `ret(0xb16d0)` |
| `ldr` at 0xb0b34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0b38 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb0b44 | load | 4 | `abs 0xb0d2c` | `-` |
| `str` at 0xb0b48 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xb0b50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0b54 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0xb0b58 | store | 4 | `sp -0x30` | `[[sp-24]]:1` |
| `ldr` at 0xb0b5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0b68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0b74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0b84 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xb0b94 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0ba0 | load | 4 | `abs 0xb0d30` | `-` |
| `str` at 0xb0ba4 | store | 4 | `sp -0x28` | `0xfffff82e` |
| `strb` at 0xb0bb0 | store | 1 | `sp -0x1d` | `0x19` |
| `ldrb` at 0xb0bb8 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb0bbc | store | 4 | `sp -0x2c` | `[sp-18]:1` |
| `ldr` at 0xb0bc0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0bcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb0be8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xb0bfc | store | 1 | `sp -0x1d` | `0x30` |
| `strb` at 0xb0c04 | store | 1 | `sp -0x1e` | `0x5c` |
| `strb` at 0xb0c10 | store | 1 | `sp -0x1d` | `0x31` |
| `strb` at 0xb0c18 | store | 1 | `sp -0x1e` | `0x5d` |
| `strb` at 0xb0c24 | store | 1 | `sp -0x1d` | `0x32` |
| `strb` at 0xb0c2c | store | 1 | `sp -0x1e` | `0x5e` |
| `ldr` at 0xb0c34 | load | 4 | `abs 0xb0d30` | `-` |
| `str` at 0xb0c38 | store | 4 | `sp -0x28` | `0xfffff82e` |
| `ldr` at 0xb0c40 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0c44 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb0c48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0c4c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb0c50 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xb0c68 | store | 4 | `sp -0x24` | `ret(0xabb28)` |
| `ldr` at 0xb0c6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0c78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb0c7c | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldrb` at 0xb0c8c | load | 1 | `sp -0x12` | `-` |
| `strb` at 0xb0c94 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xb0c98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0c9c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb0ca0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0ca4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb0ca8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xb0cc4 | store | 4 | `sp -0x24` | `ret(0xabc4c)` |
| `ldrb` at 0xb0cc8 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xb0cd0 | store | 1 | `sp -0x1f` | `?` |
| `ldrb` at 0xb0cd4 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0xb0cd8 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0xb0ce0 | store | 1 | `sp -0x13` | `?` |
| `ldr` at 0xb0ce8 | load | 4 | `abs 0xb0d34` | `-` |
| `str` at 0xb0cec | store | 4 | `sp -0x28` | `0xfffff830` |
| `ldr` at 0xb0cf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0cf8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xb0cfc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0d00 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb0d04 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xb0d08 | load | 1 | `sp -0x13` | `-` |
| `str` at 0xb0d20 | store | 4 | `sp -0x28` | `ret(0xabb28)` |
| `ldr` at 0xb0d24 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb0d28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0d28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0b14 | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb0b28 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb0c60 | `(unknown callee) 0xabb28` | `[[sp-24]+1]:1` | `[[sp-24]+2]:1` | `?` | `0xe2` |
| `bl` at 0xb0cbc | `(unknown callee) 0xabc4c` | `[[sp-24]+1]:1` | `[[sp-24]+2]:1` | `[sp-29]:1` | `sp-31` |
| `bl` at 0xb0d18 | `(unknown callee) 0xabb28` | `[[sp-24]+1]:1` | `[[sp-24]+2]:1` | `?` | `[sp-19]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0b40 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb0b64 | `[[sp-24]]:1 == 0x2` | `[[sp-24]]:1 != 0x2` |
| `bgt` at 0xb0b70 | `[[sp-24]]:1 > 0x2 (signed, 32-bit)` | `[[sp-24]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb0b7c | `[[sp-24]]:1 == 0x1` | `[[sp-24]]:1 != 0x1` |
| `beq` at 0xb0b8c | `[[sp-24]]:1 == 0x6` | `[[sp-24]]:1 != 0x6` |
| `beq` at 0xb0b9c | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0xb0bc8 | `[sp-18]:1 == 0x1` | `[sp-18]:1 != 0x1` |
| `bgt` at 0xb0bd4 | `[sp-18]:1 > 0x1 (signed, 32-bit)` | `[sp-18]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb0be0 | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0xb0bf0 | `[sp-18]:1 == 0x2` | `[sp-18]:1 != 0x2` |
| `beq` at 0xb0c74 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0d28 | `r0` | `?` |

## `monitorReadOutput`

- `monitorReadOutput`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0d38 to 0xb0f24.
  Established. The byte saved at sp -0x12 supplies `beq` paths for 0x0, 0x1 and 0x2, which reach
  byte stores of 0x30, 0x31 and 0x32 at sp -0x21 respectively (`beq` at 0xb0e30, `beq` at 0xb0e18,
  `beq` at 0xb0e40, `strb` at 0xb0e4c, `strb` at 0xb0e58, `strb` at 0xb0e64). One call to 0xabc4c
  has candidates `[[sp-28]+1]:1`, `[[sp-28]+2]:1`, `[sp-33]:1` and `[sp-24]` (`bl` at 0xb0eac). A
  subsequent byte store at `[sp-24] +0x0` has an unknown value (`strb` at 0xb0ec8). The later call
  to 0xabc4c has an unknown third candidate (`bl` at 0xb0f04). The read-output format remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb0d4c | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xb0d50 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb0d58 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb0d5c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0d6c | store | 4 | `sp -0x1c` | `ret(0xb1638)` |
| `ldrb` at 0xb0d70 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0d80 | store | 4 | `sp -0x20` | `ret(0xb16d0)` |
| `ldr` at 0xb0d84 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0d88 | load | 1 | `ret(0xb16d0) +0x0` | `-` |
| `ldr` at 0xb0d94 | load | 4 | `abs 0xb0f18` | `-` |
| `str` at 0xb0d98 | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xb0da0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0da4 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xb0da8 | store | 4 | `sp -0x34` | `[[sp-28]]:1` |
| `ldr` at 0xb0dac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb0db8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb0dc4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb0dd4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xb0de4 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb0df0 | load | 4 | `abs 0xb0f1c` | `-` |
| `str` at 0xb0df4 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `strb` at 0xb0e00 | store | 1 | `sp -0x21` | `0x19` |
| `ldrb` at 0xb0e08 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb0e0c | store | 4 | `sp -0x30` | `[sp-18]:1` |
| `ldr` at 0xb0e10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0e1c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0e28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb0e38 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0xb0e4c | store | 1 | `sp -0x21` | `0x30` |
| `strb` at 0xb0e58 | store | 1 | `sp -0x21` | `0x31` |
| `strb` at 0xb0e64 | store | 1 | `sp -0x21` | `0x32` |
| `ldr` at 0xb0e6c | load | 4 | `abs 0xb0f1c` | `-` |
| `str` at 0xb0e70 | store | 4 | `sp -0x2c` | `0xfffff82e` |
| `ldrb` at 0xb0e80 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0xb0e88 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xb0e8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0e90 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xb0e94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0e98 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xb0e9c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb0ea8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0eb4 | store | 4 | `sp -0x28` | `ret(0xabc4c)` |
| `ldr` at 0xb0eb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0ebc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0ec0 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0xb0ec8 | store | 1 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xb0ecc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb0ed0 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xb0ed8 | load | 4 | `abs 0xb0f20` | `-` |
| `str` at 0xb0edc | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xb0ee4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0ee8 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xb0eec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb0ef0 | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0xb0ef4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xb0f00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb0f0c | store | 4 | `sp -0x2c` | `ret(0xabc4c)` |
| `ldr` at 0xb0f10 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb0f14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0f14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0f14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0d64 | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb0d78 | `(unknown callee) 0xb16d0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0xb0eac | `(unknown callee) 0xabc4c` | `[[sp-28]+1]:1` | `[[sp-28]+2]:1` | `[sp-33]:1` | `[sp-24]` |
| `bl` at 0xb0f04 | `(unknown callee) 0xabc4c` | `[[sp-28]+1]:1` | `[[sp-28]+2]:1` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb0d90 | `[ret(0xb16d0)]:1 != 0x0` | `[ret(0xb16d0)]:1 == 0x0` |
| `beq` at 0xb0db4 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `bgt` at 0xb0dc0 | `[[sp-28]]:1 > 0x2 (signed, 32-bit)` | `[[sp-28]]:1 <= 0x2 (signed, 32-bit)` |
| `beq` at 0xb0dcc | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xb0ddc | `[[sp-28]]:1 == 0x6` | `[[sp-28]]:1 != 0x6` |
| `beq` at 0xb0dec | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0xb0e18 | `[sp-18]:1 == 0x1` | `[sp-18]:1 != 0x1` |
| `bgt` at 0xb0e24 | `[sp-18]:1 > 0x1 (signed, 32-bit)` | `[sp-18]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb0e30 | `[sp-18]:1 == 0x0` | `[sp-18]:1 != 0x0` |
| `beq` at 0xb0e40 | `[sp-18]:1 == 0x2` | `[sp-18]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0f14 | `r0` | `?` |

## `monitorVoltage`

- `monitorVoltage`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb0f24 to 0xb10d4.
  Established. The first two `beq` sites select paths for the byte loaded from `ret(0xb1638) +0x0`
  equal to 0x1 or 0x2 (`ldrb` at 0xb0f60, `beq` at 0xb0f70, `beq` at 0xb0f7c). Two conditional
  computed transfers have taken conditions `[sp-18]:1 <= 0x9` and `[sp-18]:1 <= 0x8`, both unsigned
  32-bit, and unresolved taken edges (`ldrls` at 0xb0f8c, `ldrls` at 0xb102c). Known paths include
  word stores of 0x3b0ffb8b and 0xc2c80000 at sp -0x1c (`str` at 0xb101c, `str` at 0xb1098). The
  constant meanings, voltage conversion and additional dispatch effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb0f3c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb0f44 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb0f48 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb0f58 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldr` at 0xb0f5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0f60 | load | 1 | `ret(0xb1638) +0x0` | `-` |
| `str` at 0xb0f64 | store | 4 | `sp -0x20` | `[ret(0xb1638)]:1` |
| `ldr` at 0xb0f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb0f74 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb0f84 | load | 1 | `sp -0x12` | `-` |
| `ldrls` at 0xb0f8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0ff8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb0ffc | load | 1 | `ret(0xb1638) +0x3` | `-` |
| `ldrb` at 0xb100c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb1018 | load | 4 | `abs 0xb10b8` | `-` |
| `str` at 0xb101c | store | 4 | `sp -0x1c` | `0x3b0ffb8b` |
| `ldrb` at 0xb1024 | load | 1 | `sp -0x12` | `-` |
| `ldrls` at 0xb102c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1094 | load | 4 | `abs 0xb10d0` | `-` |
| `str` at 0xb1098 | store | 4 | `sp -0x1c` | `0xc2c80000` |
| `ldr` at 0xb109c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb10a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb10a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb10a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0f50 | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb0f70 | `[ret(0xb1638)]:1 == 0x1` | `[ret(0xb1638)]:1 != 0x1` |
| `beq` at 0xb0f7c | `[ret(0xb1638)]:1 == 0x2` | `[ret(0xb1638)]:1 != 0x2` |
| `ldrls` at 0xb0f8c | `[sp-18]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-18]:1 > 0x9 (unsigned, 32-bit)` |
| `beq` at 0xb1008 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb1014 | `[sp-18]:1 == 0x2` | `[sp-18]:1 != 0x2` |
| `ldrls` at 0xb102c | `[sp-18]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-18]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb10a0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb0f8c).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb102c).

## `monitorVoltageLimit`

- `monitorVoltageLimit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb10d4 to 0xb115c.
  Established. A halfword save at sp -0x14 retains `(r1 & 0xffff)` (`strh` at 0xb10f0). `bne` takes
  the path past an unknown-valued overwrite of that slot when `[ret(0xb1638)]:1 != 0x2` (`bne` at
  0xb1114, `strh` at 0xb1120). The later `bls` takes the path past the halfword store of 0xfe when
  `[sp-20]:2 <= 0xfe` unsigned (`bls` at 0xb112c, `strh` at 0xb1134). A final `bne` takes the
  nonzero-halfword path past a store of 0x1 to that same slot (`bne` at 0xb1144, `strh` at 0xb114c).
  The return retains its byte expression `[sp-20]:1` (`ldmdb` at 0xb1158). The physical voltage limit
  meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb10ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xb10f0 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `ldrb` at 0xb10f4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb1104 | store | 4 | `sp -0x18` | `ret(0xb1638)` |
| `ldr` at 0xb1108 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb110c | load | 1 | `ret(0xb1638) +0x0` | `-` |
| `ldrh` at 0xb1118 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0xb1120 | store | 2 | `sp -0x14` | `?` |
| `ldrh` at 0xb1124 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0xb1134 | store | 2 | `sp -0x14` | `0xfe` |
| `ldrh` at 0xb113c | load | 2 | `sp -0x14` | `-` |
| `strh` at 0xb114c | store | 2 | `sp -0x14` | `0x1` |
| `ldrb` at 0xb1150 | load | 1 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1158 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1158 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1158 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb10fc | `(unknown callee) 0xb1638` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1114 | `[ret(0xb1638)]:1 != 0x2` | `[ret(0xb1638)]:1 == 0x2` |
| `bls` at 0xb112c | `[sp-20]:2 <= 0xfe (unsigned, 32-bit)` | `[sp-20]:2 > 0xfe (unsigned, 32-bit)` |
| `bne` at 0xb1144 | `[sp-20]:2 != 0x0` | `[sp-20]:2 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1158 | `r0` | `[sp-20]:1` |

## `monitorIntCount`

- `monitorIntCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb115c to 0xb1198.
  Established. The call to 0xb16d0 has first candidate `(r0 & 0xff)` (`bl` at 0xb117c). A word load
  at that result +0x4 supplies the return expression `[ret(0xb16d0)+4]` (`ldr` at 0xb118c, `ldmdb`
  at 0xb1194). The returned word's interrupt meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb1170 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb1174 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb1184 | store | 4 | `sp -0x18` | `ret(0xb16d0)` |
| `ldr` at 0xb1188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb118c | load | 4 | `ret(0xb16d0) +0x4` | `-` |
| `ldmdb` at 0xb1194 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1194 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1194 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb117c | `(unknown callee) 0xb16d0` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1194 | `r0` | `[ret(0xb16d0)+4]` |
