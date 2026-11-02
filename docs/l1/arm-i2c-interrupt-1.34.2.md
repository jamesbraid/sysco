# ARM L1 I2C interrupt and locking paths in 1.34.2

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
register locations r0 through r3 with unknown arity. Stack stores share the access tables.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Unresolved computed transfers exclude additional effects and return paths.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/i2c-interrupt-facts.txt) retain all tool sections.

## `i2cGetSwitch`

- `i2cGetSwitch`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab60c to 0xab648.
  Established. A word read from 0x282980 supplies the target `[0x282980] +0x0` for a byte load
  (`ldr` at 0xab628, `ldrb` at 0xab62c). The subsequent byte store targets entry r0 +0x0, with an
  unknown stored value (`strb` at 0xab634). The return carries r0 value 0x0 (`ldmdb` at 0xab640).
  The pointed location's identity is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab61c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xab620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab624 | load | 4 | `abs 0xab644` | `-` |
| `ldr` at 0xab628 | load | 4 | `abs 0x282980` | `-` |
| `ldrb` at 0xab62c | load | 1 | `[0x282980] +0x0` | `-` |
| `strb` at 0xab634 | store | 1 | `r0 +0x0` | `?` |
| `ldmdb` at 0xab640 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab640 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab640 | `r0` | `0x0` |

## `i2cLock`

- `i2cLock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab648 to 0xab788.
  Established. The call to 0x189e0 has r0 candidate 0x27ead0 (`bl` at 0xab664). The later call to
  0x18a64 supplies the value stored at 0x27eaec (`bl` at 0xab6b0, `str` at 0xab6bc). `beq` takes the
  zero-byte path from `[sp-17]:1` to the zero-result store (`beq` at 0xab6c8, `str` at 0xab758); its
  nonzero path can reach the unresolved computed transfer at 0xab714 (`mov` at 0xab714). Additional
  effects beyond that transfer remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xab65c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xab660 | load | 4 | `abs 0xab764` | `-` |
| `str` at 0xab66c | store | 4 | `sp -0x18` | `ret(0x189e0)` |
| `ldr` at 0xab670 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab684 | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xab690 | load | 4 | `abs 0xab768` | `-` |
| `str` at 0xab694 | store | 4 | `sp -0x24` | `0x107cb0` |
| `str` at 0xab698 | store | 4 | `sp -0x20` | `ret(0x1e71c)` |
| `ldr` at 0xab6a0 | load | 4 | `abs 0xab76c` | `-` |
| `ldr` at 0xab6a4 | load | 4 | `abs 0xab770` | `-` |
| `ldr` at 0xab6b8 | load | 4 | `abs 0xab774` | `-` |
| `str` at 0xab6bc | store | 4 | `abs 0x27eaec` | `ret(0x18a64)` |
| `ldrb` at 0xab6c0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xab6cc | load | 4 | `abs 0xab778` | `-` |
| `ldr` at 0xab6d0 | load | 4 | `abs 0x11570c` | `-` |
| `ldr` at 0xab6dc | load | 4 | `abs 0xab77c` | `-` |
| `str` at 0xab6e0 | store | 4 | `sp -0x24` | `0x107ccc` |
| `ldrb` at 0xab6e4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xab6e8 | store | 4 | `sp -0x20` | `[sp-17]:1` |
| `ldr` at 0xab6f0 | load | 4 | `abs 0xab76c` | `-` |
| `ldr` at 0xab6f4 | load | 4 | `abs 0xab780` | `-` |
| `ldr` at 0xab700 | load | 4 | `abs 0xab778` | `-` |
| `ldrb` at 0xab704 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xab708 | load | 4 | `abs 0x11570c` | `-` |
| `str` at 0xab758 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xab75c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xab760 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab760 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab664 | `(unknown callee) 0x189e0` | `0x27ead0` | `r1` | `r2` | `r0` |
| `bl` at 0xab67c | `(unknown callee) 0x188b4` | `ret(0x189e0)` | `?` | `?` | `ret(0x189e0)` |
| `bl` at 0xab688 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xab6ac | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x122` | `0x0` |
| `bl` at 0xab6b0 | `(unknown callee) 0x18a64` | `?` | `?` | `?` | `?` |
| `bl` at 0xab6fc | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x12b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xab678 | `ret(0x189e0) >= 0x0 (signed, 32-bit)` | `ret(0x189e0) < 0x0 (signed, 32-bit)` |
| `beq` at 0xab6c8 | `[sp-17]:1 == 0x0` | `[sp-17]:1 != 0x0` |
| `bne` at 0xab6d8 | `[0x11570c] != 0x0` | `[0x11570c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab760 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0xab714).

## `i2cUnlock`

- `i2cUnlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab788 to 0xab87c.
  Established. The routine clears the word at 0x27eaec (`str` at 0xab7a0). The byte at 0x27eaf0
  decides `beq`, whose taken zero path skips the second global test (`ldrb` at 0xab7a8, `beq` at
  0xab7b0). With a nonzero byte, a second `beq` takes the zero-word path from 0x27eae8 to the
  byte-clear at 0x27eaf0 (`ldr` at 0xab7b8, `beq` at 0xab7c0, `strb` at 0xab804); its nonzero
  fall-through reaches an unresolved computed transfer (`mov` at 0xab7dc). The known continuation
  calls 0x18c50 with r0 candidate 0x27ead0 (`bl` at 0xab80c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xab798 | load | 4 | `abs 0xab85c` | `-` |
| `str` at 0xab7a0 | store | 4 | `abs 0x27eaec` | `0x0` |
| `ldr` at 0xab7a4 | load | 4 | `abs 0xab860` | `-` |
| `ldrb` at 0xab7a8 | load | 1 | `abs 0x27eaf0` | `-` |
| `ldr` at 0xab7b4 | load | 4 | `abs 0xab864` | `-` |
| `ldr` at 0xab7b8 | load | 4 | `abs 0x27eae8` | `-` |
| `ldr` at 0xab7c4 | load | 4 | `abs 0xab864` | `-` |
| `ldr` at 0xab7c8 | load | 4 | `abs 0xab860` | `-` |
| `ldrb` at 0xab7cc | load | 1 | `abs 0x27eaf0` | `-` |
| `ldr` at 0xab7d0 | load | 4 | `abs 0x27eae8` | `-` |
| `ldr` at 0xab7fc | load | 4 | `abs 0xab860` | `-` |
| `strb` at 0xab804 | store | 1 | `abs 0x27eaf0` | `0x0` |
| `ldr` at 0xab808 | load | 4 | `abs 0xab86c` | `-` |
| `str` at 0xab814 | store | 4 | `sp -0x14` | `ret(0x18c50)` |
| `ldr` at 0xab818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab82c | load | 4 | `ret(0x188b4) +0x0` | `-` |
| `ldr` at 0xab838 | load | 4 | `abs 0xab870` | `-` |
| `str` at 0xab83c | store | 4 | `sp -0x1c` | `0x107cfc` |
| `str` at 0xab840 | store | 4 | `sp -0x18` | `ret(0x1e71c)` |
| `ldr` at 0xab848 | load | 4 | `abs 0xab874` | `-` |
| `ldr` at 0xab84c | load | 4 | `abs 0xab878` | `-` |
| `ldmdb` at 0xab858 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab858 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab80c | `(unknown callee) 0x18c50` | `0x27ead0` | `r1` | `?` | `?` |
| `bl` at 0xab824 | `(unknown callee) 0x188b4` | `ret(0x18c50)` | `?` | `?` | `ret(0x18c50)` |
| `bl` at 0xab830 | `(unknown callee) 0x1e71c` | `[ret(0x188b4)]` | `?` | `?` | `ret(0x188b4)` |
| `bl` at 0xab854 | `(unknown callee) 0x1eed0` | `0x9` | `0x107bbc` | `0x156` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xab7b0 | `[0x27eaf0]:1 == 0x0` | `[0x27eaf0]:1 != 0x0` |
| `beq` at 0xab7c0 | `[0x27eae8] == 0x0` | `[0x27eae8] != 0x0` |
| `bge` at 0xab820 | `ret(0x18c50) >= 0x0 (signed, 32-bit)` | `ret(0x18c50) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab858 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`mov` at 0xab7dc).

## `i2cIntInit`

- `i2cIntInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac0b4 to 0xac10c.
  Established. The byte read at 0x282b68 decides `beq`, whose taken zero path reaches the return
  without the call (`ldrb` at 0xac0c8, `beq` at 0xac0d0, `ldmdb` at 0xac0fc). The fall-through
  reloads that byte and stores the fresh expression `[0x282b68]:1` at sp -0x18 (`ldrb` at 0xac0e0,
  `str` at 0xac0e4). It then calls 0x1eed0 with candidates 0x9, 0x107d98, 0x61 and 0x0 (`bl` at
  0xac0f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xac0c4 | load | 4 | `abs 0xac100` | `-` |
| `ldrb` at 0xac0c8 | load | 1 | `abs 0x282b68` | `-` |
| `ldr` at 0xac0d4 | load | 4 | `abs 0xac104` | `-` |
| `str` at 0xac0d8 | store | 4 | `sp -0x1c` | `0x107d70` |
| `ldr` at 0xac0dc | load | 4 | `abs 0xac100` | `-` |
| `ldrb` at 0xac0e0 | load | 1 | `abs 0x282b68` | `-` |
| `str` at 0xac0e4 | store | 4 | `sp -0x18` | `[0x282b68]:1` |
| `ldr` at 0xac0ec | load | 4 | `abs 0xac108` | `-` |
| `ldmdb` at 0xac0fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac0fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac0fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac0f8 | `(unknown callee) 0x1eed0` | `0x9` | `0x107d98` | `0x61` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xac0d0 | `[0x282b68]:1 == 0x0` | `[0x282b68]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac0fc | `r0` | `?` |

## `i2cIntHighRegister`

- `i2cIntHighRegister`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac10c to 0xac14c.
  Established. The routine saves entry r0 as a word and the masked low bytes of entry r1 and r2 in
  adjacent stack slots (`str` at 0xac11c, `strb` at 0xac124, `strb` at 0xac12c). It reads the word
  at 0x116928 (`ldr` at 0xac134). At the call to 0x21010, r0 candidate is `[0x116928]`, r1 is
  0x107da4, r2 is entry r2 and r3 is 0x116928 (`bl` at 0xac13c). The registration role implied by
  the name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac11c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xac124 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xac12c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xac130 | load | 4 | `abs 0xac144` | `-` |
| `ldr` at 0xac134 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xac138 | load | 4 | `abs 0xac148` | `-` |
| `ldmdb` at 0xac140 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac140 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac140 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac13c | `(unknown callee) 0x21010` | `[0x116928]` | `0x107da4` | `r2` | `0x116928` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac140 | `r0` | `ret(0x21010)` |

## `i2cIntLowRegister`

- `i2cIntLowRegister`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac14c to 0xac18c.
  Established. Two byte saves retain `(r1 & 0xff)` at sp -0x15 and `(r2 & 0xff)` at sp -0x16 (`strb`
  at 0xac164, `strb` at 0xac16c). The call to 0x21010 has candidates `[0x116928]`, 0x107e04, entry
  r2 and 0x116928 (`bl` at 0xac17c); its result is the return's r0 value (`ldmdb` at 0xac180). The
  body's device role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac15c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xac164 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xac16c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xac170 | load | 4 | `abs 0xac184` | `-` |
| `ldr` at 0xac174 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0xac178 | load | 4 | `abs 0xac188` | `-` |
| `ldmdb` at 0xac180 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac180 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac17c | `(unknown callee) 0x21010` | `[0x116928]` | `0x107e04` | `r2` | `0x116928` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac180 | `r0` | `ret(0x21010)` |

## `i2cIntRegister`

- `i2cIntRegister`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac18c to 0xac2dc.
  Established. The first `bls` takes the path past a call to 0x1eed0 when `(r1 & 0xff)` is at most
  0x5 unsigned (`bls` at 0xac1c4, `bl` at 0xac1e8). The second `bls` tests the byte expression
  `[[sp-36]+68]:1` against 0x2 unsigned (`bls` at 0xac220). Later stores include an unknown word at
  `[sp-28] +0x0`, byte expressions `([sp-22]:1 & 0xff)` and `([sp-23]:1 & 0xff)` at that loaded base
  plus 4 and 5, and `([[sp-36]+68]:1+1 & 0xff)` at `[sp-36] +0x44` (`str` at 0xac298, `strb` at
  0xac2a4, `strb` at 0xac2b0, `strb` at 0xac2c4). The stored word's meaning and the record's identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac19c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xac1a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xac1b0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xac1b8 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0xac1bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac1c8 | load | 4 | `abs 0xac2cc` | `-` |
| `str` at 0xac1cc | store | 4 | `sp -0x2c` | `0x107e64` |
| `ldrb` at 0xac1d0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xac1d4 | store | 4 | `sp -0x28` | `(r1 & 0xff)` |
| `ldr` at 0xac1dc | load | 4 | `abs 0xac2d0` | `-` |
| `ldrb` at 0xac1ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xac208 | load | 4 | `abs 0xac2d4` | `-` |
| `str` at 0xac210 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac214 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac218 | load | 1 | `[sp-36] +0x44` | `-` |
| `ldr` at 0xac224 | load | 4 | `abs 0xac2d8` | `-` |
| `str` at 0xac228 | store | 4 | `sp -0x2c` | `0x107ea0` |
| `ldr` at 0xac22c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac230 | load | 1 | `[sp-36] +0x44` | `-` |
| `str` at 0xac234 | store | 4 | `sp -0x28` | `[[sp-36]+68]:1` |
| `ldr` at 0xac23c | load | 4 | `abs 0xac2d0` | `-` |
| `ldr` at 0xac24c | load | 4 | `abs 0xac2d4` | `-` |
| `ldrb` at 0xac250 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xac278 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac280 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac28c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xac290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac294 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac298 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0xac29c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac2a0 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0xac2a4 | store | 1 | `[sp-28] +0x4` | `([sp-22]:1 & 0xff)` |
| `ldr` at 0xac2a8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac2ac | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xac2b0 | store | 1 | `[sp-28] +0x5` | `([sp-23]:1 & 0xff)` |
| `ldr` at 0xac2b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac2b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac2bc | load | 1 | `[sp-36] +0x44` | `-` |
| `strb` at 0xac2c4 | store | 1 | `[sp-36] +0x44` | `([[sp-36]+68]:1+1 & 0xff)` |
| `ldmdb` at 0xac2c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac2c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac2c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac1e8 | `(unknown callee) 0x1eed0` | `0x9` | `0x107d98` | `0x87` | `0x0` |
| `bl` at 0xac248 | `(unknown callee) 0x1eed0` | `0x9` | `0x107d98` | `0x8f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xac1c4 | `(r1 & 0xff) <= 0x5 (unsigned, 32-bit)` | `(r1 & 0xff) > 0x5 (unsigned, 32-bit)` |
| `bls` at 0xac220 | `[[sp-36]+68]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-36]+68]:1 > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac2c8 | `r0` | `0x44` |

## `i2cIntEnable`

- `i2cIntEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac2dc to 0xac3ec.
  Established. The routine initializes a word at sp -0x14 to zero (`str` at 0xac2f0). The `ble`
  predicate has unknown operands (`ble` at 0xac2fc). Inside the known loop body, `bne` takes the
  nonzero-byte path from `[[sp-24]+68]:1` to the later two calls (`bne` at 0xac354, `bl` at
  0xac374, `bl` at 0xac3c8). The outgoing store at sp -0x30 carries sp-36 (`str` at 0xac3b8). The
  call to 0xac67c has candidates unknown, `[sp-20]`, unknown and 0xa (`bl` at 0xac3c8). The loop's
  full iteration count is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac2f0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xac2f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac320 | load | 4 | `abs 0xac3e0` | `-` |
| `str` at 0xac328 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xac32c | load | 4 | `abs 0xac3e4` | `-` |
| `ldr` at 0xac348 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xac34c | load | 1 | `[sp-24] +0x44` | `-` |
| `ldr` at 0xac360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac36c | load | 4 | `abs 0xac3e8` | `-` |
| `ldr` at 0xac378 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac394 | load | 4 | `abs 0xac3e0` | `-` |
| `ldr` at 0xac3a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xac3b0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xac3b8 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xac3c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac3cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac3d4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac3dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac344 | `(unknown callee) 0x18ae8` | `sp-44` | `0x107ecf` | `0x6` | `0x107ecf` |
| `bl` at 0xac374 | `(unknown callee) 0x18c74` | `sp-36` | `0x107ed8` | `[sp-20]+65` | `[sp-20]+65` |
| `bl` at 0xac3c8 | `(unknown callee) 0xac67c` | `?` | `[sp-20]` | `?` | `0xa` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac2fc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xac354 | `[[sp-24]+68]:1 != 0x0` | `[[sp-24]+68]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac3dc | `r0` | `?` |

## `i2cIntCount`

- `i2cIntCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac3ec to 0xac478.
  Established. The low byte of entry r0 is saved at sp -0x11 (`strb` at 0xac400). `bls` takes the
  path past the call to 0x1eed0 for `(r0 & 0xff) <= 0x5` unsigned (`bls` at 0xac40c, `bl` at
  0xac430). The final four-byte load has an unknown effective target and supplies an unknown return
  value (`ldr` at 0xac460, `ldmdb` at 0xac468). The count-record layout is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xac400 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xac404 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xac410 | load | 4 | `abs 0xac46c` | `-` |
| `str` at 0xac414 | store | 4 | `sp -0x1c` | `0x107ee0` |
| `ldrb` at 0xac418 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xac41c | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xac424 | load | 4 | `abs 0xac470` | `-` |
| `ldr` at 0xac434 | load | 4 | `abs 0xac474` | `-` |
| `ldrb` at 0xac438 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xac460 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xac468 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac468 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac468 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac430 | `(unknown callee) 0x1eed0` | `0x9` | `0x107d98` | `0xb7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xac40c | `(r0 & 0xff) <= 0x5 (unsigned, 32-bit)` | `(r0 & 0xff) > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac468 | `r0` | `?` |

## `i2cIntPollingMask`

- `i2cIntPollingMask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac478 to 0xac50c.
  Established. Two four-byte stack slots at sp -0x14 and sp -0x18 begin at zero (`str` at 0xac48c,
  `str` at 0xac494). `ble` selects the loop body using unknown operands (`ble` at 0xac4a0). The body
  includes a byte load with unknown effective target, followed later by unknown-valued word stores
  to those two slots (`ldrb` at 0xac4d4, `str` at 0xac4e8, `str` at 0xac4f4); its final r0 value
  remains unknown (`ldmdb` at 0xac504).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac48c | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0xac494 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xac498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac4a8 | load | 4 | `abs 0xac508` | `-` |
| `ldr` at 0xac4ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xac4d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac4d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac4e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac4e8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xac4ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac4f4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xac4fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac504 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac504 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac4a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac504 | `r0` | `?` |

## `i2cIntLock`

- `i2cIntLock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac50c to 0xac5f0.
  Established. The loop-control word at sp -0x14 begins at 0x0 (`str` at 0xac520). `ble` retains an
  unknown comparison, and the inner `bne` retains only the predicate Z == 0 (`ble` at 0xac52c, `bne`
  at 0xac568). The call to 0x189e0 has four unknown candidates (`bl` at 0xac57c). `beq` takes its
  zero-result path past the calls to 0x1f75c and 0x1eed0 (`beq` at 0xac590, `bl` at 0xac5a0, `bl` at
  0xac5cc). The locking semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac520 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xac524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac550 | load | 4 | `abs 0xac5e4` | `-` |
| `str` at 0xac558 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xac55c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac560 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac570 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac584 | store | 4 | `sp -0x18` | `ret(0x189e0)` |
| `ldr` at 0xac588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac594 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac5a8 | load | 4 | `abs 0xac5e8` | `-` |
| `str` at 0xac5ac | store | 4 | `sp -0x28` | `0x107f1c` |
| `str` at 0xac5b0 | store | 4 | `sp -0x24` | `ret(0x1f75c)` |
| `ldr` at 0xac5b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac5b8 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0xac5c0 | load | 4 | `abs 0xac5ec` | `-` |
| `ldr` at 0xac5d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac5d8 | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0xac5e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac5e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac5e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac57c | `(unknown callee) 0x189e0` | `?` | `?` | `?` | `?` |
| `bl` at 0xac5a0 | `(unknown callee) 0x1f75c` | `[sp-28]+16` | `?` | `?` | `[sp-28]+16` |
| `bl` at 0xac5cc | `(unknown callee) 0x1eed0` | `0x9` | `0x107d98` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac52c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xac568 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xac590 | `ret(0x189e0) == 0x0` | `ret(0x189e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac5e0 | `r0` | `?` |

## `i2cIntUnlock`

- `i2cIntUnlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac5f0 to 0xac67c.
  Established. A zero word is stored at sp -0x14 before the loop (`str` at 0xac604). The tool leaves
  the `ble` operands unknown and records only Z == 0 for the inner `bne` (`ble` at 0xac610, `bne` at
  0xac64c). That inner taken path reaches 0x18c50 with four unknown call candidates (`bl` at
  0xac660). The back-edge follows an unknown-valued word store to sp -0x14 (`str` at 0xac66c, `b` at
  0xac670).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac604 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xac608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac634 | load | 4 | `abs 0xac678` | `-` |
| `str` at 0xac63c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xac640 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xac644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac664 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac66c | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0xac674 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac674 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac660 | `(unknown callee) 0x18c50` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xac610 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xac64c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac674 | `r0` | `?` |

## `i2c_int_init`

- `i2c_int_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac67c to 0xac790.
  Established. The first explicit record store writes entry r1 at entry r0 +0x4 (`str` at 0xac6a4).
  Later stores use the reloaded base `[sp-20]`, writing 0x1 at +0x8 and `[sp-28]` at +0xc (`str` at
  0xac6b0, `str` at 0xac6bc). `bls` takes the path that saves 0x5dc at sp -0x34 when `ret(0xacd8c)
  <= 0x7ffff` unsigned, while fall-through saves 0x1000 there (`bls` at 0xac6d0, `str` at 0xac6e4,
  `str` at 0xac6d8). The final byte store at 0x282b68 writes `([0x282b68]:1+1 & 0xff)` (`strb` at
  0xac778). The record's device identity is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac68c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac690 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac694 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xac698 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xac69c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac6a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac6a4 | store | 4 | `r0 +0x4` | `r1` |
| `ldr` at 0xac6a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac6b0 | store | 4 | `[sp-20] +0x8` | `0x1` |
| `ldr` at 0xac6b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac6b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac6bc | store | 4 | `[sp-20] +0xc` | `[sp-28]` |
| `ldr` at 0xac6c8 | load | 4 | `abs 0xac780` | `-` |
| `str` at 0xac6d8 | store | 4 | `sp -0x34` | `0x1000` |
| `ldr` at 0xac6e0 | load | 4 | `abs 0xac784` | `-` |
| `str` at 0xac6e4 | store | 4 | `sp -0x34` | `0x5dc` |
| `ldr` at 0xac6e8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xac6ec | store | 4 | `sp -0x2c` | `[sp-52]` |
| `ldr` at 0xac6f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac6f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac6fc | store | 4 | `sp -0x48` | `[sp-20]` |
| `ldr` at 0xac700 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xac704 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xac708 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac70c | store | 4 | `sp -0x40` | `[sp-32]` |
| `str` at 0xac714 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0xac71c | store | 4 | `sp -0x38` | `0x9` |
| `ldr` at 0xac724 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xac728 | load | 4 | `abs 0xac788` | `-` |
| `ldr` at 0xac750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac768 | load | 4 | `abs 0xac78c` | `-` |
| `ldr` at 0xac76c | load | 4 | `abs 0xac78c` | `-` |
| `ldrb` at 0xac770 | load | 1 | `abs 0x282b68` | `-` |
| `strb` at 0xac778 | store | 1 | `abs 0x282b68` | `([0x282b68]:1+1 & 0xff)` |
| `ldmdb` at 0xac77c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac77c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac77c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac6c0 | `(unknown callee) 0xacd8c` | `r0` | `r1` | `[sp-20]` | `[sp-28]` |
| `bl` at 0xac730 | `(unknown callee) 0x1f498` | `[sp-20]+16` | `[sp]` | `0xac790` | `0x1` |
| `bl` at 0xac73c | `(unknown callee) 0x189d4` | `sp-48` | `?` | `?` | `sp-48` |
| `bl` at 0xac74c | `(unknown callee) 0x187a0` | `sp-48` | `0x1` | `?` | `sp-48` |
| `bl` at 0xac764 | `(unknown callee) 0x1892c` | `[sp-20]+20` | `sp-48` | `sp-48` | `[sp-20]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xac6d0 | `ret(0xacd8c) <= 0x7ffff (unsigned, 32-bit)` | `ret(0xacd8c) > 0x7ffff (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac77c | `r0` | `ret(0x1892c)` |

## `i2cInitialize`

- `i2cInitialize`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e2bc to 0x3e300.
  Established. The first calls target 0xb6428 and 0x3e4dc (`bl` at 0x3e2c8, `bl` at 0x3e2cc). `beq`
  takes the second call's zero-result path past the call to 0xb747c (`beq` at 0x3e2d8, `bl` at
  0x3e2dc). The remaining call sequence includes 0x397e0, 0x41d40 and finally 0x2be78 (`bl` at
  0x3e2e8, `bl` at 0x3e2f0, `bl` at 0x3e2f8). The last call's result reaches the return in r0
  (`ldmdb` at 0x3e2fc). Individual device roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x3e2fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3e2fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3e2fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e2c8 | `(unknown callee) 0xb6428` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3e2cc | `(unknown callee) 0x3e4dc` | `ret(0xb6428)` | `?` | `?` | `?` |
| `bl` at 0x3e2dc | `(unknown callee) 0xb747c` | `ret(0x3e4dc)` | `?` | `?` | `ret(0x3e4dc)` |
| `bl` at 0x3e2e0 | `(unknown callee) 0xb4254` | `?` | `?` | `?` | `?` |
| `bl` at 0x3e2e4 | `(unknown callee) 0xafd30` | `ret(0xb4254)` | `?` | `?` | `?` |
| `bl` at 0x3e2e8 | `(unknown callee) 0x397e0` | `ret(0xafd30)` | `?` | `?` | `?` |
| `bl` at 0x3e2ec | `(unknown callee) 0x2dc8c` | `ret(0x397e0)` | `?` | `?` | `?` |
| `bl` at 0x3e2f0 | `(unknown callee) 0x41d40` | `ret(0x2dc8c)` | `?` | `?` | `?` |
| `bl` at 0x3e2f4 | `(unknown callee) 0x3eb60` | `ret(0x41d40)` | `?` | `?` | `?` |
| `bl` at 0x3e2f8 | `(unknown callee) 0x2be78` | `ret(0x3eb60)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3e2d8 | `ret(0x3e4dc) == 0x0` | `ret(0x3e4dc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3e2fc | `r0` | `ret(0x2be78)` |

## `Unknown_i2cConfig`

- `Unknown_i2cConfig`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bb94 to 0x2bbf0.
  Established. The call to 0xb3fc8 receives r0 candidate 0xdb8d4 and r1 candidate 0x9 (`bl` at
  0x2bbac). A byte at sp -0x11 starts at zero (`strb` at 0x2bbb4). The `bls` predicate has unknown
  operands, so an iteration count is unknown (`bls` at 0x2bbc0). The loop body calls 0xb415c with
  unknown candidates and stores `([sp-17]:1+1 & 0xff)` back to that byte (`bl` at 0x2bbd0, `strb` at
  0x2bbdc). A later call targets 0x3e2bc (`bl` at 0x2bbe4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2bba4 | load | 4 | `abs 0x2bbec` | `-` |
| `strb` at 0x2bbb4 | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0x2bbb8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2bbc8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2bbd4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x2bbdc | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldmdb` at 0x2bbe8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bbe8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bbe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2bbac | `(unknown callee) 0xb3fc8` | `0xdb8d4` | `0x9` | `r2` | `r3` |
| `bl` at 0x2bbd0 | `(unknown callee) 0xb415c` | `?` | `?` | `?` | `?` |
| `bl` at 0x2bbe4 | `(unknown callee) 0x3e2bc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x2bbc0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bbe8 | `r0` | `ret(0x3e2bc)` |
