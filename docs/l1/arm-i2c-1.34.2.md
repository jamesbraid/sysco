# ARM L1 I2C entry paths in 1.34.2

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
values, and brackets denote loaded values. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Effects and return paths beyond unresolved computed transfers are unknown.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/i2c-facts.txt) retain all tool sections.

## `i2cInit`

- `i2cInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab234 to 0xab3d8.
  Established. The routine tests the word at 0x115708 (`ldr` at 0xab24c, `cmp` at 0xab250). `blt` takes the negative path past the first
  call to 0x1eed0 (`ldr` at 0xab24c, `blt` at 0xab254, `bl` at 0xab270). It stores the result of the
  call to 0x1889c at 0x282980 (`bl` at 0xab2f0, `str` at 0xab2f4). Later it stores a fresh load from
  that location plus 36 back there (`ldr` at 0xab344, `str` at 0xab34c), then writes the result of
  the later call to 0x18b54 at 0x115708 (`bl` at 0xab358, `str` at 0xab364). The last visible call
  targets 0xab908 (`bl` at 0xab3a8). The device mapping and error-reporting roles of the numeric
  callees remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab244 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xab248 | load | 4 | `abs 0xab3b0` | `-` |
| `ldr` at 0xab24c | load | 4 | `abs 0x115708` | `-` |
| `ldr` at 0xab258 | load | 4 | `abs 0xab3b4` | `-` |
| `str` at 0xab25c | store | 4 | `sp -0x24` | `0x107ba4` |
| `ldr` at 0xab264 | load | 4 | `abs 0xab3b8` | `-` |
| `ldr` at 0xab274 | load | 4 | `abs 0xab3bc` | `-` |
| `ldr` at 0xab278 | load | 4 | `abs 0xab3c0` | `-` |
| `str` at 0xab284 | store | 4 | `sp -0x1c` | `ret(0x18b54)` |
| `ldr` at 0xab288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab2a0 | load | 4 | `abs 0xab3c4` | `-` |
| `str` at 0xab2a4 | store | 4 | `sp -0x24` | `0x107bd0` |
| `ldr` at 0xab2a8 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xab2ac | store | 4 | `sp -0x20` | `[ret(0x188b4)]` |
| `ldr` at 0xab2b4 | load | 4 | `abs 0xab3b8` | `-` |
| `ldr` at 0xab2cc | load | 4 | `abs 0xab3c8` | `-` |
| `ldr` at 0xab2d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab2d4 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xab2dc | store | 4 | `sp -0x20` | `0x56000000` |
| `str` at 0xab2f4 | store | 4 | `abs 0x282980` | `ret(0x1889c)` |
| `ldr` at 0xab2f8 | load | 4 | `abs 0x282980` | `-` |
| `ldr` at 0xab310 | load | 4 | `abs 0xab3cc` | `-` |
| `str` at 0xab314 | store | 4 | `sp -0x24` | `0x107bf0` |
| `ldr` at 0xab318 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `str` at 0xab31c | store | 4 | `sp -0x20` | `[ret(0x188b4)]` |
| `ldr` at 0xab324 | load | 4 | `abs 0xab3b8` | `-` |
| `ldr` at 0xab33c | load | 4 | `abs 0xab3c8` | `-` |
| `ldr` at 0xab340 | load | 4 | `abs 0xab3c8` | `-` |
| `ldr` at 0xab344 | load | 4 | `abs 0x282980` | `-` |
| `str` at 0xab34c | store | 4 | `abs 0x282980` | `[0x282980]+36` |
| `ldr` at 0xab350 | load | 4 | `abs 0xab3d0` | `-` |
| `ldr` at 0xab360 | load | 4 | `abs 0xab3b0` | `-` |
| `str` at 0xab364 | store | 4 | `abs 0x115708` | `ret(0x18b54)` |
| `ldr` at 0xab368 | load | 4 | `abs 0x115708` | `-` |
| `ldr` at 0xab37c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xab388 | load | 4 | `abs 0xab3d4` | `-` |
| `str` at 0xab38c | store | 4 | `sp -0x24` | `0x107c28` |
| `str` at 0xab390 | store | 4 | `sp -0x20` | `ret(0x1e71c)` |
| `ldr` at 0xab398 | load | 4 | `abs 0xab3b8` | `-` |
| `ldmdb` at 0xab3ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xab3ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab3ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab3ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab270 | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x4a` | `0x0` |
| `bl` at 0xab27c | `(unknown callee) 0x18b54` | `0x107bc4` | `0x1002` | `?` | `?` |
| `bl` at 0xab294 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `?` | `ret(0x18b54)` |
| `bl` at 0xab2c0 | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x50` | `0x0` |
| `bl` at 0xab2c8 | `(unknown callee) 0x18b84` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xab2f0 | `(unknown callee) 0x1889c` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xab304 | `(unknown callee) 0x188b4` | `ret(0x1889c)` | `?` | `?` | `[0x282980]` |
| `bl` at 0xab330 | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x58` | `0x0` |
| `bl` at 0xab338 | `(unknown callee) 0x18b84` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xab358 | `(unknown callee) 0x18b54` | `0x107c1c` | `0x2` | `0x282980` | `[0x282980]+36` |
| `bl` at 0xab374 | `(unknown callee) 0x188b4` | `ret(0x18b54)` | `?` | `ret(0x18b54)` | `[0x115708]` |
| `bl` at 0xab380 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xab3a4 | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x62` | `0x0` |
| `bl` at 0xab3a8 | `(unknown callee) 0xab908` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xab254 | `[0x115708] < 0x0 (signed, 32-bit)` | `[0x115708] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xab290 | `ret(0x18b54) >= 0x0 (signed, 32-bit)` | `ret(0x18b54) < 0x0 (signed, 32-bit)` |
| `bne` at 0xab300 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xab370 | `[0x115708] >= 0x0 (signed, 32-bit)` | `[0x115708] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab3ac | `r0` | `ret(0xab908)` |

## `i2cRead`

- `i2cRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab9f4 to 0xaba40.
  Established. The byte save at sp -0x11 contains `(r0 & 0xff)`, and the halfword save at sp -0x1a
  contains `(r2 & 0xffff)` (`strb` at 0xaba0c, `strh` at 0xaba10). The latter is copied into the
  four-byte outgoing stack slot at sp -0x20 (`str` at 0xaba1c). The call to 0xabce0 has r0 candidate
  0x1 and r1 candidate `(r0 & 0xff)` (`bl` at 0xaba30); its r2 candidate remains unknown (`bl` at 0xaba30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaba08 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xaba0c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xaba10 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xaba14 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xaba18 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xaba1c | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xaba2c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaba3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaba3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaba3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaba30 | `(unknown callee) 0xabce0` | `0x1` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaba3c | `r0` | `ret(0xabce0)` |

## `i2cWrite`

- `i2cWrite`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab94c to 0xab998.
  Established. The call to 0xabce0 supplies r0 candidate 0x0 and r1 candidate `(r0 & 0xff)` (`bl` at
  0xab988). Before that call, the routine places `(r2 & 0xffff)` in the four-byte slot at sp -0x20
  (`str` at 0xab974); its result reaches the return in r0 (`ldmdb` at 0xab994). The transaction
  semantics of the numeric target are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab960 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xab964 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xab968 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xab96c | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xab970 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xab974 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xab984 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xab994 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab994 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab988 | `(unknown callee) 0xabce0` | `0x0` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab994 | `r0` | `ret(0xabce0)` |

## `i2cReadOffset`

- `i2cReadOffset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaba40 to 0xaba9c.
  Established. Two byte saves retain the low bytes of entry r0 and r1 (`strb` at 0xaba5c, `strb` at
  0xaba64). The call to 0xabce0 receives candidates 0x1, `(r0 & 0xff)`, `(r1 & 0xff)` and entry r2
  in r0 through r3 (`bl` at 0xaba8c). The four-byte outgoing slot at sp -0x20 separately contains
  `(r3 & 0xffff)` (`str` at 0xaba78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaba50 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xaba5c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xaba64 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xaba68 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xaba6c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xaba70 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xaba74 | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xaba78 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xaba88 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaba98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaba98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaba98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaba8c | `(unknown callee) 0xabce0` | `0x1` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaba98 | `r0` | `ret(0xabce0)` |

## `i2cWriteOffset`

- `i2cWriteOffset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab998 to 0xab9f4.
  Established. The halfword save retains `(r3 & 0xffff)` at sp -0x1a (`strh` at 0xab9c0), then the
  outgoing four-byte slot receives that expression at sp -0x20 (`str` at 0xab9d0). At the call to
  0xabce0, the four candidates are 0x0, `(r0 & 0xff)`, `(r1 & 0xff)` and entry r2 (`bl` at 0xab9e4).
  The target's parameter meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab9a8 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xab9b4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xab9bc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xab9c0 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xab9c4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xab9c8 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xab9cc | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xab9d0 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xab9e0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xab9f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab9f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab9f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab9e4 | `(unknown callee) 0xabce0` | `0x0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab9f0 | `r0` | `ret(0xabce0)` |

## `i2cReadByte`

- `i2cReadByte`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabbc8 to 0xabc4c.
  Established. The first call targets 0xab648 with r0 candidate `(r0 & 0xff)` (`bl` at 0xabbf4).
  `beq` takes the zero-result path to the call to 0xab9f4 (`beq` at 0xabc08, `bl` at 0xabc28); its fall-through path copies the
  first result to sp -0x20 and bypasses that later call (`beq` at 0xabc08, `str` at 0xabc10, `b` at
  0xabc14, `bl` at 0xabc28). The zero-result path also calls 0xab788 with r0 candidate 0x0 (`bl` at
  0xabc38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabbdc | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xabbe0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xabbe8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0xabbec | load | 1 | `sp -0x11` | `-` |
| `str` at 0xabbfc | store | 4 | `sp -0x1c` | `ret(0xab648)` |
| `ldr` at 0xabc00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabc0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabc10 | store | 4 | `sp -0x20` | `ret(0xab648)` |
| `ldrb` at 0xabc18 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xabc20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabc30 | store | 4 | `sp -0x1c` | `ret(0xab9f4)` |
| `ldr` at 0xabc3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabc40 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xabc44 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xabc48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabc48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabc48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabbf4 | `(unknown callee) 0xab648` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xabc28 | `(unknown callee) 0xab9f4` | `[sp-18]:1` | `[sp-24]` | `0x1` | `[sp-18]:1` |
| `bl` at 0xabc38 | `(unknown callee) 0xab788` | `0x0` | `?` | `?` | `ret(0xab9f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabc08 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabc48 | `r0` | `?` |

## `i2cWriteByte`

- `i2cWriteByte`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaba9c to 0xabb28.
  Established. Three byte stores preserve masked entry values at sp -0x11, sp -0x12 and sp -0x13
  (`strb` at 0xabab0, `strb` at 0xabab8, `strb` at 0xabac0). After the call to 0xab648, `beq` takes
  its zero-result path to 0xab94c (`bl` at 0xabacc, `beq` at 0xabae0, `bl` at 0xabb04). That
  call's r1 candidate is sp-19 and its r2 candidate is 0x1 (`bl` at 0xabb04). The same path then
  calls 0xab788 with r0 candidate 0x0 (`bl` at 0xabb14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xabab0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xabab8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xabac0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xabac4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xabad4 | store | 4 | `sp -0x18` | `ret(0xab648)` |
| `ldr` at 0xabad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabae4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabae8 | store | 4 | `sp -0x1c` | `ret(0xab648)` |
| `ldrb` at 0xabaf0 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xabb0c | store | 4 | `sp -0x18` | `ret(0xab94c)` |
| `ldr` at 0xabb18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabb1c | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xabb20 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xabb24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabb24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabb24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabacc | `(unknown callee) 0xab648` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xabb04 | `(unknown callee) 0xab94c` | `[sp-18]:1` | `sp-19` | `0x1` | `[sp-18]:1` |
| `bl` at 0xabb14 | `(unknown callee) 0xab788` | `0x0` | `?` | `?` | `ret(0xab94c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabae0 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabb24 | `r0` | `?` |

## `i2cReadByteOffset`

- `i2cReadByteOffset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabc4c to 0xabce0.
  Established. The routine first saves entry r3 as a word at sp -0x18 and masks entry r0 through r2
  into separate byte slots (`str` at 0xabc5c, `strb` at 0xabc64, `strb` at 0xabc6c, `strb` at
  0xabc74). `beq` takes the zero-result path after the call to 0xab648 (`bl` at 0xabc80, `beq` at
  0xabc94). On that path the call to 0xaba40 has candidates `[sp-18]:1`, `[sp-19]:1`, `[sp-24]` and
  0x1 (`bl` at 0xabcbc). It is followed by a call to 0xab788 (`bl` at 0xabccc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabc5c | store | 4 | `sp -0x18` | `r3` |
| `strb` at 0xabc64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xabc6c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xabc74 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0xabc78 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xabc88 | store | 4 | `sp -0x1c` | `ret(0xab648)` |
| `ldr` at 0xabc8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabc98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabc9c | store | 4 | `sp -0x20` | `ret(0xab648)` |
| `ldrb` at 0xabca4 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xabca8 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0xabcb4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabcc4 | store | 4 | `sp -0x1c` | `ret(0xaba40)` |
| `ldr` at 0xabcd0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabcd4 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xabcd8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xabcdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabcdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabcdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabc80 | `(unknown callee) 0xab648` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xabcbc | `(unknown callee) 0xaba40` | `[sp-18]:1` | `[sp-19]:1` | `[sp-24]` | `0x1` |
| `bl` at 0xabccc | `(unknown callee) 0xab788` | `0x0` | `?` | `?` | `ret(0xaba40)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabc94 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabcdc | `r0` | `?` |

## `i2cWriteByteOffset`

- `i2cWriteByteOffset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabb28 to 0xabbc8.
  Established. The four byte stores retain masked entry r0 through r3 at sp -0x11 through sp -0x14
  (`strb` at 0xabb40, `strb` at 0xabb48, `strb` at 0xabb50, `strb` at 0xabb58). The call to 0xab648
  is followed by `beq`, whose taken zero-result path reaches the call to 0xab998 (`bl` at 0xabb64,
  `beq` at 0xabb78, `bl` at 0xabba4). That later call's r2 candidate is sp-20 and its r3 candidate
  is 0x1 (`bl` at 0xabba4). The nonzero fall-through path copies the first call result and jumps
  past the later calls (`str` at 0xabb80, `b` at 0xabb84).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xabb40 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xabb48 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0xabb50 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0xabb58 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `ldrb` at 0xabb5c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xabb6c | store | 4 | `sp -0x18` | `ret(0xab648)` |
| `ldr` at 0xabb70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabb7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabb80 | store | 4 | `sp -0x1c` | `ret(0xab648)` |
| `ldrb` at 0xabb88 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xabb8c | load | 1 | `sp -0x13` | `-` |
| `str` at 0xabbac | store | 4 | `sp -0x18` | `ret(0xab998)` |
| `ldr` at 0xabbb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabbbc | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xabbc0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xabbc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabbc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabbc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabb64 | `(unknown callee) 0xab648` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xabba4 | `(unknown callee) 0xab998` | `[sp-18]:1` | `[sp-19]:1` | `sp-20` | `0x1` |
| `bl` at 0xabbb4 | `(unknown callee) 0xab788` | `0x0` | `?` | `?` | `ret(0xab998)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabb78 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabbc4 | `r0` | `?` |

## `i2cReset`

- `i2cReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab908 to 0xab94c.
  Established. A word at sp -0x14 is initialized to 0x0 (`str` at 0xab91c). The loop reads that slot
  and a literal at 0xab948 before `ble` (`ldr` at 0xab920, `ldr` at 0xab924, `ble` at 0xab92c). The tool leaves the compared operands unknown (`ldr` at
  0xab920, `ldr` at 0xab924, `ble` at 0xab92c); its taken arm reaches the add-and-store sequence,
  whose stored value remains unknown (`add` at 0xab938, `str` at 0xab93c). The hardware-reset role
  suggested by the symbol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab91c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xab920 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab924 | load | 4 | `abs 0xab948` | `-` |
| `ldr` at 0xab934 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab93c | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0xab944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xab92c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab944 | `r0` | `r0` |

## `i2cResetBusSelect`

- `i2cResetBusSelect`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab428 to 0xab48c.
  Established. The word read from 0x11570c decides `beq` (`ldr` at 0xab43c, `beq` at 0xab444); its taken zero path skips both calls
  (`ldr` at 0xab43c, `beq` at 0xab444). On fall-through, the call to 0xab648 receives r0 candidate
  0x1 (`bl` at 0xab44c). A second `beq` takes that call's zero-result path to 0xab788 with r0
  candidate 0x0 (`beq` at 0xab460, `bl` at 0xab474). The nonzero result instead reaches the
  saved-result store at sp -0x18 (`str` at 0xab468).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xab438 | load | 4 | `abs 0xab488` | `-` |
| `ldr` at 0xab43c | load | 4 | `abs 0x11570c` | `-` |
| `str` at 0xab454 | store | 4 | `sp -0x14` | `ret(0xab648)` |
| `ldr` at 0xab458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab464 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab468 | store | 4 | `sp -0x18` | `ret(0xab648)` |
| `str` at 0xab47c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xab480 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xab484 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab484 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab484 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab44c | `(unknown callee) 0xab648` | `0x1` | `r1` | `r2` | `[0x11570c]` |
| `bl` at 0xab474 | `(unknown callee) 0xab788` | `0x0` | `?` | `?` | `ret(0xab648)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab444 | `[0x11570c] == 0x0` | `[0x11570c] != 0x0` |
| `beq` at 0xab460 | `ret(0xab648) == 0x0` | `ret(0xab648) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab484 | `r0` | `?` |

## `i2cSelectBus`

- `i2cSelectBus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab87c to 0xab908.
  Established. `beq` takes the zero path from the word loaded at 0x11570c to the call to 0x1eed0
  (`ldr` at 0xab898, `beq` at 0xab8a0, `bl` at 0xab8e8); its nonzero fall-through path reloads
  0x11570c and reaches an unresolved computed transfer (`ldr` at 0xab8ac, `mov` at 0xab8b8). The
  known zero path stores 0x0 at sp -0x18 (`str` at 0xab8f0). The return table covers that known path
  only (`ldmdb` at 0xab8f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xab890 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xab894 | load | 4 | `abs 0xab8fc` | `-` |
| `ldr` at 0xab898 | load | 4 | `abs 0x11570c` | `-` |
| `ldr` at 0xab8a4 | load | 4 | `abs 0xab8fc` | `-` |
| `ldrb` at 0xab8a8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xab8ac | load | 4 | `abs 0x11570c` | `-` |
| `ldr` at 0xab8c8 | load | 4 | `abs 0xab900` | `-` |
| `str` at 0xab8cc | store | 4 | `sp -0x20` | `0x107d18` |
| `ldrb` at 0xab8d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xab8d4 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xab8dc | load | 4 | `abs 0xab904` | `-` |
| `str` at 0xab8f0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xab8f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xab8f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab8f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab8f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab8e8 | `(unknown callee) 0x1eed0` | `0x1` | `0x107bbc` | `0x168` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab8a0 | `[0x11570c] == 0x0` | `[0x11570c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab8f8 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0xab8b8).

## `i2cSetBusSelect`

- `i2cSetBusSelect`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab3d8 to 0xab400.
  Established. Entry r0 is saved at sp -0x14, reloaded and stored as a word at 0x11570c (`str` at
  0xab3e8, `ldr` at 0xab3f0, `str` at 0xab3f4). The return preserves entry r0 (`ldmdb` at 0xab3f8).
  The receiving global's wider role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab3e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xab3ec | load | 4 | `abs 0xab3fc` | `-` |
| `ldr` at 0xab3f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab3f4 | store | 4 | `abs 0x11570c` | `r0` |
| `ldmdb` at 0xab3f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab3f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab3f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab3f8 | `r0` | `r0` |

## `i2cSetBusDeselect`

- `i2cSetBusDeselect`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab400 to 0xab428.
  Established. The saved entry r0 at sp -0x14 supplies a four-byte store to 0x27eae8 (`str` at
  0xab410, `ldr` at 0xab418, `str` at 0xab41c); its return also carries entry r0 (`ldmdb` at
  0xab420). The callback semantics implied by the symbol are not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab410 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xab414 | load | 4 | `abs 0xab424` | `-` |
| `ldr` at 0xab418 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab41c | store | 4 | `abs 0x27eae8` | `r0` |
| `ldmdb` at 0xab420 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab420 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab420 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab420 | `r0` | `r0` |

## `i2cSetSwitch`

- `i2cSetSwitch`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab5d4 to 0xab60c.
  Established. The byte save retains `(r0 & 0xff)` at sp -0x11 (`strb` at 0xab5e8). A word read from
  0x282980 supplies the effective target `[0x282980] +0x0` for a four-byte store of that masked
  value (`ldr` at 0xab5f0, `str` at 0xab5f8). The return has r0 value 0x0 (`ldmdb` at 0xab604). The
  pointee's hardware identity remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xab5e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xab5ec | load | 4 | `abs 0xab608` | `-` |
| `ldr` at 0xab5f0 | load | 4 | `abs 0x282980` | `-` |
| `ldrb` at 0xab5f4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xab5f8 | store | 4 | `[0x282980] +0x0` | `(r0 & 0xff)` |
| `ldmdb` at 0xab604 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab604 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab604 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab604 | `r0` | `0x0` |
