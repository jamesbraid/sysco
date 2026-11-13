# ARM L1 I2C mux paths in 1.34.2

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
values, and brackets denote loaded values. Numbers without a `0x` prefix in bracket
expressions are decimal, and `:1` marks a byte load. Repeated loads can share an expression
without establishing that their values agree. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. An unresolved computed transfer can leave additional effects and return paths unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/i2c-mux-facts.txt) retain all tool sections.

## `i2cMuxInit`

- `i2cMuxInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb61f4 to 0xb62c4.
  Established. The word at 0x27eb6c decides `beq`, whose taken zero path skips the first call to
  0x1eed0 (`ldr` at 0xb6210, `beq` at 0xb6218, `bl` at 0xb6234). The call to 0x18ba8 supplies the
  value stored at 0x284140 (`bl` at 0xb6244, `str` at 0xb624c). Later word stores write `[sp-24]` at
  0x284100 and `[sp-28]` at 0x284120 (`str` at 0xb6294, `str` at 0xb62a0). The meanings of those stored words are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6204 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb6208 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb620c | load | 4 | `abs 0xb62a8` | `-` |
| `ldr` at 0xb6210 | load | 4 | `abs 0x27eb6c` | `-` |
| `ldr` at 0xb621c | load | 4 | `abs 0xb62ac` | `-` |
| `str` at 0xb6220 | store | 4 | `sp -0x24` | `0x108fc0` |
| `ldr` at 0xb6228 | load | 4 | `abs 0xb62b0` | `-` |
| `ldr` at 0xb6238 | load | 4 | `abs 0xb62b4` | `-` |
| `ldr` at 0xb623c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb624c | store | 4 | `abs 0x284140` | `ret(0x18ba8)` |
| `ldr` at 0xb6250 | load | 4 | `abs 0x284140` | `-` |
| `ldr` at 0xb625c | load | 4 | `abs 0xb62b8` | `-` |
| `str` at 0xb6260 | store | 4 | `sp -0x24` | `0x108ff8` |
| `ldr` at 0xb6268 | load | 4 | `abs 0xb62b0` | `-` |
| `ldr` at 0xb6278 | load | 4 | `abs 0xb62b4` | `-` |
| `ldr` at 0xb627c | load | 4 | `abs 0x284140` | `-` |
| `ldr` at 0xb6284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb628c | load | 4 | `abs 0xb62bc` | `-` |
| `ldr` at 0xb6290 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6294 | store | 4 | `abs 0x284100` | `[sp-24]` |
| `ldr` at 0xb6298 | load | 4 | `abs 0xb62c0` | `-` |
| `ldr` at 0xb629c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb62a0 | store | 4 | `abs 0x284120` | `[sp-28]` |
| `ldmdb` at 0xb62a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb62a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb62a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb62a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6234 | `(unknown callee) 0x1eed0` | `0x9` | `0x108fdc` | `0x4d` | `0x0` |
| `bl` at 0xb6244 | `(unknown callee) 0x18ba8` | `?` | `0x1` | `?` | `?` |
| `bl` at 0xb6274 | `(unknown callee) 0x1eed0` | `0x9` | `0x108fdc` | `0x54` | `0x0` |
| `bl` at 0xb6288 | `(unknown callee) 0x18c20` | `[0x284140]` | `0x1` | `[sp-28]` | `0x284140` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6218 | `[0x27eb6c] == 0x0` | `[0x27eb6c] != 0x0` |
| `bne` at 0xb6258 | `[0x284140] != 0x0` | `[0x284140] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb62a4 | `r0` | `ret(0x18c20)` |

## `i2cMuxDisable`

- `i2cMuxDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb62c4 to 0xb6344.
  Established. `bge` takes the path to 0x1eed0 when `(r0 & 0xff) >= [0x284120]` signed (`bge` at
  0xb62ec, `bl` at 0xb632c). Fall-through instead stores byte 0x0 at `((r0 & 0xff) + [0x284140])
  +0x0` (`strb` at 0xb6304). The physical device role of that loaded base remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb62d8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb62dc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb62e0 | load | 4 | `abs 0xb6334` | `-` |
| `ldr` at 0xb62e4 | load | 4 | `abs 0x284120` | `-` |
| `ldrb` at 0xb62f0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb62f4 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb62f8 | load | 4 | `abs 0x284140` | `-` |
| `strb` at 0xb6304 | store | 1 | `((r0 & 0xff) + [0x284140]) +0x0` | `0x0` |
| `ldr` at 0xb630c | load | 4 | `abs 0xb633c` | `-` |
| `str` at 0xb6310 | store | 4 | `sp -0x1c` | `0x109018` |
| `ldrb` at 0xb6314 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb6318 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xb6320 | load | 4 | `abs 0xb6340` | `-` |
| `ldmdb` at 0xb6330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb632c | `(unknown callee) 0x1eed0` | `0x6` | `0x108fdc` | `0x69` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb62ec | `(r0 & 0xff) >= [0x284120] (signed, 32-bit)` | `(r0 & 0xff) < [0x284120] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6330 | `r0` | `?` |

## `i2cMuxProbe`

- `i2cMuxProbe`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6344 to 0xb6428.
  Established. The first `bge` takes the path past the calls when `(r0 & 0xff) >= [0x284120]` signed
  (`bge` at 0xb636c). After the call to 0xb6554, `beq` takes the zero-result path to a word store of
  0x1 at sp -0x24 (`bl` at 0xb637c, `beq` at 0xb6390, `str` at 0xb6400). A different path clears a
  byte at `([sp-21]:1 + [0x284140]) +0x0` (`strb` at 0xb63f4). The later `beq` at 0xb639c retains
  only the predicate Z == 1 (`beq` at 0xb639c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6358 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `ldrb` at 0xb635c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb6360 | load | 4 | `abs 0xb6418` | `-` |
| `ldr` at 0xb6364 | load | 4 | `abs 0x284120` | `-` |
| `ldrb` at 0xb6370 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb6384 | store | 4 | `sp -0x1c` | `ret(0xb6554)` |
| `ldr` at 0xb6388 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6394 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb63a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb63b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb63bc | load | 4 | `abs 0xb641c` | `-` |
| `str` at 0xb63c0 | store | 4 | `sp -0x30` | `0x10903c` |
| `str` at 0xb63c4 | store | 4 | `sp -0x2c` | `ret(0xb66c8)` |
| `str` at 0xb63c8 | store | 4 | `sp -0x28` | `ret(0x1ea50)` |
| `ldr` at 0xb63d0 | load | 4 | `abs 0xb6420` | `-` |
| `ldrb` at 0xb63e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb63e4 | load | 4 | `abs 0xb6424` | `-` |
| `ldr` at 0xb63e8 | load | 4 | `abs 0x284140` | `-` |
| `strb` at 0xb63f4 | store | 1 | `([sp-21]:1 + [0x284140]) +0x0` | `0x0` |
| `str` at 0xb6400 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0xb640c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb6410 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb6414 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb6414 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6414 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6414 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb637c | `(unknown callee) 0xb6554` | `(r0 & 0xff)` | `0x0` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0xb63a8 | `(unknown callee) 0xb66c8` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |
| `bl` at 0xb63b4 | `(unknown callee) 0x1ea50` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb63dc | `(unknown callee) 0x1eed0` | `0x1` | `0x108fdc` | `0x79` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb636c | `(r0 & 0xff) >= [0x284120] (signed, 32-bit)` | `(r0 & 0xff) < [0x284120] (signed, 32-bit)` |
| `beq` at 0xb6390 | `ret(0xb6554) == 0x0` | `ret(0xb6554) != 0x0` |
| `beq` at 0xb639c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6414 | `r0` | `?` |

## `i2cMuxReset`

- `i2cMuxReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6428 to 0xb6498.
  Established. The call to 0xab648 supplies r0 candidate 0x0 (`bl` at 0xb643c). `beq` takes that
  call's zero-result path to calls at 0xb6728 and 0xab788 (`beq` at 0xb6450, `bl` at 0xb6484, `bl`
  at 0xb6488). Fall-through instead calls 0x1ea50, then 0x1eed0 with candidates 0x1, 0x108fdc, 0x92
  and 0x0 (`bl` at 0xb6458, `bl` at 0xb647c). The reset semantics of the numeric callees are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6444 | store | 4 | `sp -0x14` | `ret(0xab648)` |
| `ldr` at 0xb6448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6460 | load | 4 | `abs 0xb6490` | `-` |
| `str` at 0xb6464 | store | 4 | `sp -0x1c` | `0x109068` |
| `str` at 0xb6468 | store | 4 | `sp -0x18` | `ret(0x1ea50)` |
| `ldr` at 0xb6470 | load | 4 | `abs 0xb6494` | `-` |
| `ldmdb` at 0xb648c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb648c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb648c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb643c | `(unknown callee) 0xab648` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb6458 | `(unknown callee) 0x1ea50` | `ret(0xab648)` | `?` | `?` | `ret(0xab648)` |
| `bl` at 0xb647c | `(unknown callee) 0x1eed0` | `0x1` | `0x108fdc` | `0x92` | `0x0` |
| `bl` at 0xb6484 | `(unknown callee) 0xb6728` | `ret(0xab648)` | `?` | `?` | `ret(0xab648)` |
| `bl` at 0xb6488 | `(unknown callee) 0xab788` | `ret(0xb6728)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6450 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb648c | `r0` | `?` |

## `i2cMuxGetCfg`

- `i2cMuxGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6498 to 0xb64d4.
  Established. The word read at 0x284100 is written to entry r0 +0x0 (`ldr` at 0xb64b4, `str` at
  0xb64b8). A separate load from 0x284120 supplies the return's r0 value `[0x284120]` (`ldr` at
  0xb64c0, `ldmdb` at 0xb64c8). The meanings of the two loaded words remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb64a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb64ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb64b0 | load | 4 | `abs 0xb64cc` | `-` |
| `ldr` at 0xb64b4 | load | 4 | `abs 0x284100` | `-` |
| `str` at 0xb64b8 | store | 4 | `r0 +0x0` | `[0x284100]` |
| `ldr` at 0xb64bc | load | 4 | `abs 0xb64d0` | `-` |
| `ldr` at 0xb64c0 | load | 4 | `abs 0x284120` | `-` |
| `ldmdb` at 0xb64c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb64c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb64c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb64c8 | `r0` | `[0x284120]` |

## `i2cMuxCount`

- `i2cMuxCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb64d4 to 0xb64f4.
  Established. A word load from 0x284120 supplies r0 value `[0x284120]` at the return (`ldr` at
  0xb64e4, `ldmdb` at 0xb64ec). The body does not establish what that word counts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb64e0 | load | 4 | `abs 0xb64f0` | `-` |
| `ldr` at 0xb64e4 | load | 4 | `abs 0x284120` | `-` |
| `ldmdb` at 0xb64ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb64ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb64ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb64ec | `r0` | `[0x284120]` |

## `i2cMuxPresent`

- `i2cMuxPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb64f4 to 0xb6554.
  Established. `blt` takes the indexed-byte path for `(r0 & 0xff) < [0x284120]` signed (`blt` at
  0xb651c). On that path a byte read from `((r0 & 0xff) + [0x284140]) +0x0` is saved as a word at sp
  -0x18 (`ldrb` at 0xb653c, `str` at 0xb6540). Fall-through saves 0x0 to that slot and jumps to the
  join (`str` at 0xb6524, `b` at 0xb6528). The merged return value is unknown (`ldmdb` at 0xb6548).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6508 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb650c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6510 | load | 4 | `abs 0xb654c` | `-` |
| `ldr` at 0xb6514 | load | 4 | `abs 0x284120` | `-` |
| `str` at 0xb6524 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0xb652c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6530 | load | 4 | `abs 0xb6550` | `-` |
| `ldr` at 0xb6534 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb653c | load | 1 | `((r0 & 0xff) + [0x284140]) +0x0` | `-` |
| `str` at 0xb6540 | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x284140])]:1` |
| `ldr` at 0xb6544 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb6548 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6548 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6548 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb651c | `(r0 & 0xff) < [0x284120] (signed, 32-bit)` | `(r0 & 0xff) >= [0x284120] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6548 | `r0` | `?` |

## `i2cMuxSelect`

- `i2cMuxSelect`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6554 to 0xb664c.
  Established. The first call targets 0xb6e08 with r0 candidate `(r0 & 0xff)` (`bl` at 0xb6580). The
  following byte test's `bne` takes its nonzero path to 0xab648, while the zero fall-through
  saves 0xfffffc11 at sp -0x20 (`bne` at 0xb65a4, `bl` at 0xb65c0, `str` at 0xb65ac). After a zero
  result from 0xab648, another `bne` selects between calls to 0xb6ae0 on its taken nonzero-byte path
  and 0xb6968 on fall-through (`beq` at 0xb65d4, `bne` at 0xb65f0, `bl` at 0xb6624, `bl` at
  0xb6604). The selected path subsequently calls 0xab788 (`bl` at 0xb6630).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb656c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb6574 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb6578 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb6588 | store | 4 | `sp -0x18` | `ret(0xb6e08)` |
| `ldrb` at 0xb658c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6590 | load | 4 | `abs 0xb6644` | `-` |
| `ldr` at 0xb6594 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb659c | load | 1 | `([sp-17]:1 + [0x284140]) +0x0` | `-` |
| `ldr` at 0xb65a8 | load | 4 | `abs 0xb6648` | `-` |
| `str` at 0xb65ac | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0xb65b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65b8 | load | 1 | `ret(0xb6e08) +0x1` | `-` |
| `str` at 0xb65c8 | store | 4 | `sp -0x1c` | `ret(0xab648)` |
| `ldr` at 0xb65cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb65d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb65dc | store | 4 | `sp -0x20` | `ret(0xab648)` |
| `ldr` at 0xb65e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb65e8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldrb` at 0xb65f4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb65f8 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb660c | store | 4 | `sp -0x1c` | `ret(0xb6968)` |
| `ldrb` at 0xb6614 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb6618 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xb662c | store | 4 | `sp -0x1c` | `ret(0xb6ae0)` |
| `ldr` at 0xb6634 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6638 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xb663c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb6640 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6640 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6580 | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0xb65c0 | `(unknown callee) 0xab648` | `[ret(0xb6e08)+1]:1` | `?` | `[sp-17]:1` | `[ret(0xb6e08)+1]:1` |
| `bl` at 0xb6604 | `(unknown callee) 0xb6968` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0xb6624 | `(unknown callee) 0xb6ae0` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0xb6630 | `(unknown callee) 0xab788` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb65a4 | `[([sp-17]:1 + [0x284140])]:1 != 0x0` | `[([sp-17]:1 + [0x284140])]:1 == 0x0` |
| `beq` at 0xb65d4 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |
| `bne` at 0xb65f0 | `[[sp-24]+1]:1 != 0x0` | `[[sp-24]+1]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6640 | `r0` | `?` |

## `i2cMuxRead`

- `i2cMuxRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb664c to 0xb66c8.
  Established. The byte store at sp -0x11 retains `(r0 & 0xff)` (`strb` at 0xb6664). The call to
  0xb6e08 uses that expression as its first candidate (`bl` at 0xb6670). `bne` takes the
  nonzero-byte path to 0xb6fe4 with r0 candidate `ret(0xb6e08)` and r1 candidate `[sp-24]` (`bne` at
  0xb6694, `bl` at 0xb66ac). The zero fall-through stores 0xfffffc11 at sp -0x20 (`str` at 0xb669c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6660 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xb6664 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb6668 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb6678 | store | 4 | `sp -0x1c` | `ret(0xb6e08)` |
| `ldrb` at 0xb667c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6680 | load | 4 | `abs 0xb66c0` | `-` |
| `ldr` at 0xb6684 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb668c | load | 1 | `([sp-17]:1 + [0x284140]) +0x0` | `-` |
| `ldr` at 0xb6698 | load | 4 | `abs 0xb66c4` | `-` |
| `str` at 0xb669c | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0xb66a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb66a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb66b4 | store | 4 | `sp -0x20` | `ret(0xb6fe4)` |
| `ldr` at 0xb66b8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb66bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb66bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb66bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6670 | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb66ac | `(unknown callee) 0xb6fe4` | `ret(0xb6e08)` | `[sp-24]` | `[sp-17]:1` | `[([sp-17]:1 + [0x284140])]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb6694 | `[([sp-17]:1 + [0x284140])]:1 != 0x0` | `[([sp-17]:1 + [0x284140])]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb66bc | `r0` | `?` |

## `i2cMuxName`

- `i2cMuxName`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb66c8 to 0xb6728.
  Established. After the call to 0xb6e08, a word read at its result +0x4 is copied to sp -0x1c (`bl`
  at 0xb66e8, `ldr` at 0xb66f8, `str` at 0xb66fc). The routine reads the same effective location
  again before `bne` (`ldr` at 0xb6704, `bne` at 0xb670c). That branch takes its nonzero path past
  the fallback store of 0x109088 to sp -0x1c (`bne` at 0xb670c, `str` at 0xb6714). The two loads
  need not return the same value, and the pointer's string meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb66dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb66e0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb66f0 | store | 4 | `sp -0x18` | `ret(0xb6e08)` |
| `ldr` at 0xb66f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb66f8 | load | 4 | `ret(0xb6e08) +0x4` | `-` |
| `str` at 0xb66fc | store | 4 | `sp -0x1c` | `[ret(0xb6e08)+4]` |
| `ldr` at 0xb6700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6704 | load | 4 | `ret(0xb6e08) +0x4` | `-` |
| `ldr` at 0xb6710 | load | 4 | `abs 0xb6724` | `-` |
| `str` at 0xb6714 | store | 4 | `sp -0x1c` | `0x109088` |
| `ldr` at 0xb6718 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb6720 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6720 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6720 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb66e8 | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb670c | `[ret(0xb6e08)+4] != 0x0` | `[ret(0xb6e08)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6720 | `r0` | `?` |

## `i2cMuxSelectRaw`

- `i2cMuxSelectRaw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6968 to 0xb6ae0.
  Established. Byte stores with unknown values target 0x11582a and 0x11582b before the call to
  0xb6e88 (`strb` at 0xb6a34, `strb` at 0xb6a40, `bl` at 0xb6a50). `beq` takes the call's
  zero-result path to byte stores of `([sp-17]:1 & 0xff)` at 0x115828 and `([sp-18]:1 & 0xff)` at
  0x115829 (`beq` at 0xb6a64, `strb` at 0xb6aa4, `strb` at 0xb6ab0). The nonzero fall-through
  instead calls 0x1e71c and 0x1dba4 (`bl` at 0xb6a6c, `bl` at 0xb6a8c). The globals' hardware
  meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6980 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb6988 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb698c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb699c | store | 4 | `sp -0x18` | `ret(0xb6e08)` |
| `ldrb` at 0xb69a0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb69a4 | load | 4 | `abs 0xb6ac4` | `-` |
| `ldr` at 0xb69a8 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb69b0 | load | 1 | `([sp-17]:1 + [0x284140]) +0x0` | `-` |
| `ldr` at 0xb69bc | load | 4 | `abs 0xb6ac8` | `-` |
| `str` at 0xb69c0 | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0xb69c8 | load | 4 | `abs 0xb6acc` | `-` |
| `ldrb` at 0xb69cc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb69d0 | load | 1 | `abs 0x115828` | `-` |
| `ldr` at 0xb69dc | load | 4 | `abs 0xb6ad0` | `-` |
| `ldrb` at 0xb69e0 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb69e4 | load | 1 | `abs 0x115829` | `-` |
| `str` at 0xb69f4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb69fc | load | 4 | `abs 0xb6acc` | `-` |
| `ldrb` at 0xb6a00 | load | 1 | `abs 0x115828` | `-` |
| `ldr` at 0xb6a0c | load | 4 | `abs 0xb6ad0` | `-` |
| `ldrb` at 0xb6a10 | load | 1 | `abs 0x115829` | `-` |
| `ldr` at 0xb6a1c | load | 4 | `abs 0xb6acc` | `-` |
| `ldrb` at 0xb6a20 | load | 1 | `abs 0x115828` | `-` |
| `ldr` at 0xb6a2c | load | 4 | `abs 0xb6ad4` | `-` |
| `strb` at 0xb6a34 | store | 1 | `abs 0x11582a` | `?` |
| `ldr` at 0xb6a38 | load | 4 | `abs 0xb6ad8` | `-` |
| `strb` at 0xb6a40 | store | 1 | `abs 0x11582b` | `?` |
| `ldrb` at 0xb6a44 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb6a48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6a58 | store | 4 | `sp -0x1c` | `ret(0xb6e88)` |
| `ldr` at 0xb6a5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6a68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6a74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6a78 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb6a7c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb6a80 | load | 4 | `abs 0xb6adc` | `-` |
| `ldr` at 0xb6a90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6a94 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xb6a9c | load | 4 | `abs 0xb6acc` | `-` |
| `ldrb` at 0xb6aa0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb6aa4 | store | 1 | `abs 0x115828` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0xb6aa8 | load | 4 | `abs 0xb6ad0` | `-` |
| `ldrb` at 0xb6aac | load | 1 | `sp -0x12` | `-` |
| `strb` at 0xb6ab0 | store | 1 | `abs 0x115829` | `([sp-18]:1 & 0xff)` |
| `str` at 0xb6ab8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb6abc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb6ac0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6ac0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6ac0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6994 | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0xb6a28 | `(unknown callee) 0xb6c08` | `[0x115828]:1` | `?` | `[sp-17]:1` | `[0x115828]:1` |
| `bl` at 0xb6a50 | `(unknown callee) 0xb6e88` | `[sp-24]` | `[sp-18]:1` | `0x11582b` | `[sp-18]:1` |
| `bl` at 0xb6a6c | `(unknown callee) 0x1e71c` | `ret(0xb6e88)` | `?` | `?` | `ret(0xb6e88)` |
| `bl` at 0xb6a8c | `(unknown callee) 0x1dba4` | `0x109090` | `[[sp-24]+2]:1` | `[sp-18]:1` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb69b8 | `[([sp-17]:1 + [0x284140])]:1 != 0x0` | `[([sp-17]:1 + [0x284140])]:1 == 0x0` |
| `bne` at 0xb69d8 | `[sp-17]:1 != [0x115828]:1` | `[sp-17]:1 == [0x115828]:1` |
| `bne` at 0xb69ec | `[sp-18]:1 != [0x115829]:1` | `[sp-18]:1 == [0x115829]:1` |
| `beq` at 0xb6a08 | `[0x115828]:1 == 0xfe` | `[0x115828]:1 != 0xfe` |
| `beq` at 0xb6a18 | `[0x115829]:1 == 0x0` | `[0x115829]:1 != 0x0` |
| `beq` at 0xb6a64 | `ret(0xb6e88) == 0x0` | `ret(0xb6e88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6ac0 | `r0` | `?` |

## `i2cMuxSelectSubRaw`

- `i2cMuxSelectSubRaw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6ae0 to 0xb6c08.
  Established. The routine compares saved byte `[sp-17]:1` with the byte loaded at 0x11582a, then
  uses `bne` to take the unequal path to 0xb6e88 (`ldrb` at 0xb6b48, `bne` at 0xb6b50, `bl` at
  0xb6b80). On equality, a second `bne` tests `[sp-18]:1` against `[0x11582b]:1`; its equal
  fall-through saves a zero result (`bne` at 0xb6b64, `str` at 0xb6b6c). After the call to 0xb6e88,
  `beq` takes its zero-result path to stores at 0x11582a and 0x11582b containing `([sp-17]:1 &
  0xff)` and `([sp-18]:1 & 0xff)` (`beq` at 0xb6b94, `strb` at 0xb6bd4, `strb` at 0xb6be0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6af8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xb6b00 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xb6b04 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xb6b14 | store | 4 | `sp -0x18` | `ret(0xb6e08)` |
| `ldrb` at 0xb6b18 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6b1c | load | 4 | `abs 0xb6bf4` | `-` |
| `ldr` at 0xb6b20 | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb6b28 | load | 1 | `([sp-17]:1 + [0x284140]) +0x0` | `-` |
| `ldr` at 0xb6b34 | load | 4 | `abs 0xb6bf8` | `-` |
| `str` at 0xb6b38 | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0xb6b40 | load | 4 | `abs 0xb6bfc` | `-` |
| `ldrb` at 0xb6b44 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb6b48 | load | 1 | `abs 0x11582a` | `-` |
| `ldr` at 0xb6b54 | load | 4 | `abs 0xb6c00` | `-` |
| `ldrb` at 0xb6b58 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xb6b5c | load | 1 | `abs 0x11582b` | `-` |
| `str` at 0xb6b6c | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0xb6b74 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb6b78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6b88 | store | 4 | `sp -0x1c` | `ret(0xb6e88)` |
| `ldr` at 0xb6b8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6ba4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb6ba8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xb6bac | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xb6bb0 | load | 4 | `abs 0xb6c04` | `-` |
| `ldr` at 0xb6bc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6bc4 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xb6bcc | load | 4 | `abs 0xb6bfc` | `-` |
| `ldrb` at 0xb6bd0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xb6bd4 | store | 1 | `abs 0x11582a` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0xb6bd8 | load | 4 | `abs 0xb6c00` | `-` |
| `ldrb` at 0xb6bdc | load | 1 | `sp -0x12` | `-` |
| `strb` at 0xb6be0 | store | 1 | `abs 0x11582b` | `([sp-18]:1 & 0xff)` |
| `str` at 0xb6be8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb6bec | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xb6bf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6bf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6bf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6b0c | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0xb6b80 | `(unknown callee) 0xb6e88` | `ret(0xb6e08)` | `[sp-18]:1` | `?` | `[sp-18]:1` |
| `bl` at 0xb6b9c | `(unknown callee) 0x1e71c` | `ret(0xb6e88)` | `?` | `?` | `ret(0xb6e88)` |
| `bl` at 0xb6bbc | `(unknown callee) 0x1dba4` | `0x1090c4` | `[[sp-24]+2]:1` | `[sp-18]:1` | `ret(0x1e71c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb6b30 | `[([sp-17]:1 + [0x284140])]:1 != 0x0` | `[([sp-17]:1 + [0x284140])]:1 == 0x0` |
| `bne` at 0xb6b50 | `[sp-17]:1 != [0x11582a]:1` | `[sp-17]:1 == [0x11582a]:1` |
| `bne` at 0xb6b64 | `[sp-18]:1 != [0x11582b]:1` | `[sp-18]:1 == [0x11582b]:1` |
| `beq` at 0xb6b94 | `ret(0xb6e88) == 0x0` | `ret(0xb6e88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6bf0 | `r0` | `?` |

## `i2cMuxDeselectRaw`

- `i2cMuxDeselectRaw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6c08 to 0xb6d4c.
  Established. The call to 0xb6e88 has r0 candidate `ret(0xb6e08)` and r1 candidate 0x0 (`bl` at
  0xb6c64). `beq` takes the call's zero-result path past three calls (`beq` at 0xb6c78, `bl`
  at 0xb6c84, `bl` at 0xb6c90, `bl` at 0xb6cb8). Later byte stores include unknown values at
  0x115828, 0x115829, 0x11582a and 0x11582b (`strb` at 0xb6cf4, `strb` at 0xb6d00, `strb` at
  0xb6d0c, `strb` at 0xb6d18). The first pair is bypassed by the taken nonzero-byte path of `bne` at
  0xb6cd4 or the taken unequal-byte path of `bne` at 0xb6ce8 (`bne` at 0xb6cd4, `bne` at 0xb6ce8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6c1c | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `ldrb` at 0xb6c20 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb6c30 | store | 4 | `sp -0x1c` | `ret(0xb6e08)` |
| `ldrb` at 0xb6c34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb6c38 | load | 4 | `abs 0xb6d2c` | `-` |
| `ldr` at 0xb6c3c | load | 4 | `abs 0x284140` | `-` |
| `ldrb` at 0xb6c44 | load | 1 | `([sp-21]:1 + [0x284140]) +0x0` | `-` |
| `ldr` at 0xb6c50 | load | 4 | `abs 0xb6d30` | `-` |
| `str` at 0xb6c54 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xb6c5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6c6c | store | 4 | `sp -0x20` | `ret(0xb6e88)` |
| `ldr` at 0xb6c70 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb6c7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb6c8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6c98 | load | 4 | `abs 0xb6d34` | `-` |
| `str` at 0xb6c9c | store | 4 | `sp -0x30` | `0x1090fc` |
| `str` at 0xb6ca0 | store | 4 | `sp -0x2c` | `ret(0xb66c8)` |
| `str` at 0xb6ca4 | store | 4 | `sp -0x28` | `ret(0x1e71c)` |
| `ldr` at 0xb6cac | load | 4 | `abs 0xb6d38` | `-` |
| `ldr` at 0xb6cbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6cc0 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xb6cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb6ccc | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0xb6cd8 | load | 4 | `abs 0xb6d3c` | `-` |
| `ldrb` at 0xb6cdc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xb6ce0 | load | 1 | `abs 0x115828` | `-` |
| `ldr` at 0xb6cec | load | 4 | `abs 0xb6d3c` | `-` |
| `strb` at 0xb6cf4 | store | 1 | `abs 0x115828` | `?` |
| `ldr` at 0xb6cf8 | load | 4 | `abs 0xb6d40` | `-` |
| `strb` at 0xb6d00 | store | 1 | `abs 0x115829` | `?` |
| `ldr` at 0xb6d04 | load | 4 | `abs 0xb6d44` | `-` |
| `strb` at 0xb6d0c | store | 1 | `abs 0x11582a` | `?` |
| `ldr` at 0xb6d10 | load | 4 | `abs 0xb6d48` | `-` |
| `strb` at 0xb6d18 | store | 1 | `abs 0x11582b` | `?` |
| `str` at 0xb6d20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb6d24 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb6d28 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb6d28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6d28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6d28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6c28 | `(unknown callee) 0xb6e08` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xb6c64 | `(unknown callee) 0xb6e88` | `ret(0xb6e08)` | `0x0` | `[sp-21]:1` | `[([sp-21]:1 + [0x284140])]:1` |
| `bl` at 0xb6c84 | `(unknown callee) 0xb66c8` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |
| `bl` at 0xb6c90 | `(unknown callee) 0x1e71c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xb6cb8 | `(unknown callee) 0x1eed0` | `0x1` | `0x108fdc` | `0x1d8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb6c4c | `[([sp-21]:1 + [0x284140])]:1 != 0x0` | `[([sp-21]:1 + [0x284140])]:1 == 0x0` |
| `beq` at 0xb6c78 | `ret(0xb6e88) == 0x0` | `ret(0xb6e88) != 0x0` |
| `bne` at 0xb6cd4 | `[[sp-28]+1]:1 != 0x0` | `[[sp-28]+1]:1 == 0x0` |
| `bne` at 0xb6ce8 | `[sp-21]:1 != [0x115828]:1` | `[sp-21]:1 == [0x115828]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6d28 | `r0` | `?` |

## `i2cMuxSetSwitch`

- `i2cMuxSetSwitch`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6d4c to 0xb6de8.
  Established. `bls` takes the path beyond the 0xfffff573 result store for `(r0 & 0xff) <= 0x9`
  unsigned (`bls` at 0xb6d6c, `str` at 0xb6d74). That path includes unknown-valued byte stores at
  0x115828, 0x115829, 0x11582a and 0x11582b (`strb` at 0xb6d84, `strb` at 0xb6d90, `strb` at
  0xb6d9c, `strb` at 0xb6da8). The later byte load has effective target `[sp-17]:1 +0x108fb6`
  (`ldrb` at 0xb6db4). The call to 0xab5d4 has first candidate `[[sp-17]:1+1085366]:1` (`bl` at
  0xb6dbc). The switch's physical identity remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xb6d60 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xb6d64 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xb6d70 | load | 4 | `abs 0xb6dd0` | `-` |
| `str` at 0xb6d74 | store | 4 | `sp -0x18` | `0xfffff573` |
| `ldr` at 0xb6d7c | load | 4 | `abs 0xb6dd4` | `-` |
| `strb` at 0xb6d84 | store | 1 | `abs 0x115828` | `?` |
| `ldr` at 0xb6d88 | load | 4 | `abs 0xb6dd8` | `-` |
| `strb` at 0xb6d90 | store | 1 | `abs 0x115829` | `?` |
| `ldr` at 0xb6d94 | load | 4 | `abs 0xb6ddc` | `-` |
| `strb` at 0xb6d9c | store | 1 | `abs 0x11582a` | `?` |
| `ldr` at 0xb6da0 | load | 4 | `abs 0xb6de0` | `-` |
| `strb` at 0xb6da8 | store | 1 | `abs 0x11582b` | `?` |
| `ldr` at 0xb6dac | load | 4 | `abs 0xb6de4` | `-` |
| `ldrb` at 0xb6db0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xb6db4 | load | 1 | `[sp-17]:1 +0x108fb6` | `-` |
| `str` at 0xb6dc4 | store | 4 | `sp -0x18` | `ret(0xab5d4)` |
| `ldr` at 0xb6dc8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb6dcc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6dcc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6dcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6dbc | `(unknown callee) 0xab5d4` | `[[sp-17]:1+1085366]:1` | `r1` | `0x108fb6` | `[[sp-17]:1+1085366]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xb6d6c | `(r0 & 0xff) <= 0x9 (unsigned, 32-bit)` | `(r0 & 0xff) > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6dcc | `r0` | `?` |

## `i2cMuxLastChannel`

- `i2cMuxLastChannel`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6de8 to 0xb6e08.
  Established. The byte loaded at 0x115829 supplies the return expression `[0x115829]:1` (`ldrb` at
  0xb6df8, `ldmdb` at 0xb6e00). This body alone does not establish the channel numbering represented
  by that byte.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb6df4 | load | 4 | `abs 0xb6e04` | `-` |
| `ldrb` at 0xb6df8 | load | 1 | `abs 0x115829` | `-` |
| `ldmdb` at 0xb6e00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6e00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6e00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6e00 | `r0` | `[0x115829]:1` |
