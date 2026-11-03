# ARM serial, reset and service entry paths in 1.34.2

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
The first three routines report unresolved computed transfers. Their unknown destinations are excluded from access, call and return summaries. Other routines report no unresolved computed transfers. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/serial-reset-service-facts.txt) retain all tool sections.

## `brickGetSerialSlab`

- `brickGetSerialSlab`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b424 to 0x2b63c.
  Established. The call to `0xb45d0` has candidates `sp-36`, entry r1, `0x119c14`, `sp-36` (`bl` at
  0x2b45c); its nonzero result takes the continuation while zero falls through to word `0x0` at
  `sp -0x28` (`bne` at 0x2b468, `str` at 0x2b470). Two computed transfers have unresolved taken
  edges for unsigned `[sp-28] <= 0x4` and `[sp-28] <= 0x5`, with respective fall-through predicates
  above those bounds (`ldrls` at 0x2b4ac, `ldrls` at 0x2b50c). A reached word store at an unknown
  target writes `ret(0x18788)` (`str` at 0x2b610). Selection destinations and serial-data
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2b434 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x2b43c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2b440 | load | 4 | `abs 0x2b62c` | `-` |
| `ldr` at 0x2b444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2b448 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2b470 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x2b484 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x2b488 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2b494 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2b4a4 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x2b4ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2b504 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x2b50c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2b574 | load | 4 | `abs 0x2b630` | `-` |
| `str` at 0x2b578 | store | 4 | `sp -0x30` | `0xdb7dc` |
| `ldr` at 0x2b580 | load | 4 | `abs 0x2b634` | `-` |
| `ldr` at 0x2b584 | load | 4 | `abs 0x2b638` | `-` |
| `ldr` at 0x2b590 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2b5a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2b5ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2b5b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2b5cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2b5d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2b5e0 | load | 4 | `abs 0x2b62c` | `-` |
| `ldr` at 0x2b5e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2b5e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2b5fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2b604 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2b610 | store | 4 | `(unknown)` | `ret(0x18788)` |
| `ldr` at 0x2b614 | load | 4 | `abs 0x2b62c` | `-` |
| `ldr` at 0x2b618 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2b61c | load | 4 | `(unknown)` | `-` |
| `str` at 0x2b620 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x2b624 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x2b628 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2b628 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2b628 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2b628 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2b628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2b45c | `(unknown callee) 0xb45d0` | `sp-36` | `r1` | `0x119c14` | `sp-36` |
| `bl` at 0x2b478 | `(unknown callee) 0x2b31c` | `ret(0xb45d0)` | `?` | `?` | `ret(0xb45d0)` |
| `bl` at 0x2b58c | `(unknown callee) 0x1eed0` | `0x9` | `0xdb814` | `0x1d9` | `0x0` |
| `bl` at 0x2b608 | `(unknown callee) 0x18788` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2b450 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x2b468 | `ret(0xb45d0) != 0x0` | `ret(0xb45d0) == 0x0` |
| `beq` at 0x2b490 | `[sp-44] == 0x36` | `[sp-44] != 0x36` |
| `beq` at 0x2b49c | `[sp-44] == 0x40` | `[sp-44] != 0x40` |
| `ldrls` at 0x2b4ac | `[sp-28] <= 0x4 (unsigned, 32-bit)` | `[sp-28] > 0x4 (unsigned, 32-bit)` |
| `ldrls` at 0x2b50c | `[sp-28] <= 0x5 (unsigned, 32-bit)` | `[sp-28] > 0x5 (unsigned, 32-bit)` |
| `bne` at 0x2b5b4 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x2b5dc | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2b628 | `r0` | `[sp-40]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x2b4ac).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x2b50c).

## `brickGetSerial`

- `brickGetSerial`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b63c to 0x2bb18.
  Established. A nonzero word at absolute `0x119c10` takes the edge to the final load, while
  zero falls through (`bne` at 0x2b660). The initial call to `0xb45d0` carries `sp-32`, entry r1,
  entry r2, `sp-32` (`bl` at 0x2b66c). A computed transfer takes an unresolved edge for
  `[0x115468]:1-35 <= 0x36` unsigned and falls through above that bound (`ldrls` at 0x2b698). On a
  known path, a word store writes `ret(0x18788)` at absolute `0x119c10` (`str` at 0x2ba9c). Excluded
  destinations’ calls, stores and serial construction are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2b650 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2b654 | load | 4 | `abs 0x2bab4` | `-` |
| `ldr` at 0x2b658 | load | 4 | `abs 0x119c10` | `-` |
| `str` at 0x2b680 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x2b688 | load | 4 | `abs 0x2bab8` | `-` |
| `ldrb` at 0x2b68c | load | 1 | `abs 0x115468` | `-` |
| `ldrls` at 0x2b698 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2ba1c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x2ba20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ba34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2ba3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2ba48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ba5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2ba64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x2ba70 | load | 4 | `abs 0x2bab4` | `-` |
| `ldr` at 0x2ba74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2ba88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2ba90 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2ba9c | store | 4 | `abs 0x119c10` | `ret(0x18788)` |
| `ldr` at 0x2baa0 | load | 4 | `abs 0x2bab4` | `-` |
| `ldr` at 0x2baa4 | load | 4 | `abs 0x119c10` | `-` |
| `str` at 0x2baa8 | store | 4 | `sp -0x24` | `[0x119c10]` |
| `ldr` at 0x2baac | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x2bab0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x2bab0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bab0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bab0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2b66c | `(unknown callee) 0xb45d0` | `sp-32` | `r1` | `r2` | `sp-32` |
| `bl` at 0x2ba04 | `(unknown callee) 0x2b31c` | `ret(0xb45d0)` | `?` | `?` | `[0x115468]:1-35` |
| `bl` at 0x2ba94 | `(unknown callee) 0x18788` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2b660 | `[0x119c10] != 0x0` | `[0x119c10] == 0x0` |
| `bne` at 0x2b678 | `ret(0xb45d0) != 0x0` | `ret(0xb45d0) == 0x0` |
| `ldrls` at 0x2b698 | `[0x115468]:1-35 <= 0x36 (unsigned, 32-bit)` | `[0x115468]:1-35 > 0x36 (unsigned, 32-bit)` |
| `bne` at 0x2ba14 | `? != 0x36` | `? == 0x36` |
| `bne` at 0x2ba44 | `? != 0x1` | `? == 0x1` |
| `beq` at 0x2ba6c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bab0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x2b698).

## `networkSerial`

- `networkSerial`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2fc9c to 0x2fedc.
  Established. The only call on known edges targets `0x2b31c` with all four entry register
  candidates (`bl` at 0x2fcac). The computed transfer takes an unresolved edge for `? <= 0x35`
  unsigned and falls through for `? > 0x35` (`ldrls` at 0x2fcc0). The known fall-through return
  reports `ret(0x2b31c)` (`ldmdb` at 0x2fecc). No explicit stores are reported on known edges. The
  excluded destinations’ network and serial effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldrls` at 0x2fcc0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x2fecc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2fecc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2fecc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2fcac | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x2fcc0 | `? <= 0x35 (unsigned, 32-bit)` | `? > 0x35 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2fecc | `r0` | `ret(0x2b31c)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x2fcc0).

## `PBaySdrGetCardSerial`

- `PBaySdrGetCardSerial`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x92b44 to 0x93010.
  Established. A halfword store clears `sp -0x2c2`, and a byte store writes `0x8` at `sp -0x2c3`
  (`strh` at 0x92c08, `strb` at 0x92c18). The first call to `0x937d8` carries `[sp-20]`, `sp-696`,
  `[sp-288]`, `[sp-706]:2` (`bl` at 0x92d58); its low-byte result being nonzero takes the
  continuation, while zero falls through to the later return path (`bne` at 0x92d7c). Two
  subsequent byte stores at unknown targets write `0xa` and `0x0` (`strb` at 0x92f74, `strb` at
  0x92f9c). The absolute word `0x1156e0` is cleared near the end (`str` at 0x92fd4). The card-data
  encoding and callee effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x92b54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x92b58 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x92b5c | load | 4 | `abs 0x92fe8` | `-` |
| `str` at 0x92b60 | store | 4 | `sp -0x120` | `0x115694` |
| `ldr` at 0x92b64 | load | 4 | `sp -0x120` | `-` |
| `ldrb` at 0x92b68 | load | 1 | `abs 0x115694` | `-` |
| `ldr` at 0x92b74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x92b7c | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0x92b84 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x92b8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x92ba4 | store | 4 | `sp -0x2cc` | `ret(0x18920)` |
| `ldr` at 0x92ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x92bac | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x92bb0 | store | 4 | `[sp-24] +0x0` | `ret(0x18920)` |
| `ldr` at 0x92bb4 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0x92bc4 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x92bcc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x92bd0 | load | 4 | `abs 0x92fec` | `-` |
| `ldr` at 0x92bd8 | load | 4 | `abs 0x92ff0` | `-` |
| `ldr` at 0x92bdc | load | 4 | `abs 0x1156e0` | `-` |
| `str` at 0x92be0 | store | 4 | `sp -0x2dc` | `[0x1156e0]` |
| `ldr` at 0x92be8 | load | 4 | `abs 0x92fec` | `-` |
| `str` at 0x92bec | store | 4 | `sp -0x2cc` | `0x1156b4` |
| `ldr` at 0x92bf0 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0x92bf8 | store | 1 | `abs 0x1156b4` | `0x0` |
| `ldr` at 0x92bfc | load | 4 | `abs 0x92ff4` | `-` |
| `strh` at 0x92c08 | store | 2 | `sp -0x2c2` | `0x0` |
| `ldr` at 0x92c0c | load | 4 | `abs 0x92ff8` | `-` |
| `strb` at 0x92c18 | store | 1 | `sp -0x2c3` | `0x8` |
| `ldr` at 0x92c20 | load | 4 | `abs 0x92ff4` | `-` |
| `ldrh` at 0x92c28 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0x92c2c | load | 4 | `abs 0x92ff8` | `-` |
| `ldrb` at 0x92c34 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0x92c38 | store | 4 | `sp -0x2e4` | `0x8` |
| `ldr` at 0x92c3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92c44 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x92c54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x92c64 | store | 4 | `sp -0x2bc` | `ret(0x90884)` |
| `ldr` at 0x92c68 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0x92c74 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x92c78 | load | 4 | `abs 0x92ffc` | `-` |
| `ldr` at 0x92c80 | load | 4 | `abs 0x92ff0` | `-` |
| `str` at 0x92c88 | store | 4 | `abs 0x1156e0` | `?` |
| `str` at 0x92c90 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x92c98 | load | 4 | `sp -0x130` | `-` |
| `str` at 0x92ca0 | store | 4 | `sp -0x2d0` | `ret(0x94484)` |
| `ldr` at 0x92ca8 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0x92cc0 | load | 4 | `abs 0x93000` | `-` |
| `ldrb` at 0x92ccc | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x92cd8 | load | 4 | `abs 0x93000` | `-` |
| `ldrb` at 0x92ce4 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0x92cf0 | load | 4 | `abs 0x92ff4` | `-` |
| `ldr` at 0x92cf4 | load | 4 | `abs 0x93000` | `-` |
| `ldrb` at 0x92d00 | load | 1 | `sp -0x2d4` | `-` |
| `strh` at 0x92d0c | store | 2 | `sp -0x2c2` | `?` |
| `ldr` at 0x92d10 | load | 4 | `abs 0x92ff8` | `-` |
| `strb` at 0x92d1c | store | 1 | `sp -0x2c3` | `0x3` |
| `ldr` at 0x92d24 | load | 4 | `abs 0x92ff4` | `-` |
| `ldrh` at 0x92d2c | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0x92d30 | load | 4 | `abs 0x92ff8` | `-` |
| `ldrb` at 0x92d38 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0x92d3c | store | 4 | `sp -0x2e4` | `0x3` |
| `str` at 0x92d44 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0x92d48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92d50 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x92d60 | load | 4 | `abs 0x93004` | `-` |
| `strb` at 0x92d68 | store | 1 | `sp -0x2c4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x92d6c | load | 4 | `abs 0x93004` | `-` |
| `ldrb` at 0x92d74 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0x92d80 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x92d84 | load | 4 | `abs 0x92ffc` | `-` |
| `ldr` at 0x92d8c | load | 4 | `abs 0x92ff0` | `-` |
| `str` at 0x92d94 | store | 4 | `abs 0x1156e0` | `?` |
| `str` at 0x92d9c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0x92da4 | load | 4 | `abs 0x92ff8` | `-` |
| `ldr` at 0x92da8 | load | 4 | `abs 0x93008` | `-` |
| `ldrb` at 0x92db0 | load | 1 | `sp -0x117` | `-` |
| `strb` at 0x92dbc | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0x92dc0 | load | 4 | `abs 0x92ff8` | `-` |
| `ldr` at 0x92dc4 | load | 4 | `abs 0x92ff8` | `-` |
| `ldr` at 0x92dc8 | load | 4 | `abs 0x93004` | `-` |
| `ldrb` at 0x92dd0 | load | 1 | `sp -0x2c3` | `-` |
| `ldrb` at 0x92dd8 | load | 1 | `sp -0x2c4` | `-` |
| `strb` at 0x92de4 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0x92de8 | load | 4 | `abs 0x92ff4` | `-` |
| `ldr` at 0x92dec | load | 4 | `abs 0x92ff4` | `-` |
| `ldrh` at 0x92df4 | load | 2 | `sp -0x2c2` | `-` |
| `strh` at 0x92e00 | store | 2 | `sp -0x2c2` | `([sp-706]:2+3 & 0xffff)` |
| `ldr` at 0x92e08 | load | 4 | `abs 0x92ff4` | `-` |
| `ldrh` at 0x92e10 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0x92e14 | load | 4 | `abs 0x92ff8` | `-` |
| `ldrb` at 0x92e1c | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0x92e20 | store | 4 | `sp -0x2e4` | `[sp-707]:1` |
| `str` at 0x92e28 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0x92e2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x92e34 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x92e44 | load | 4 | `abs 0x93004` | `-` |
| `strb` at 0x92e4c | store | 1 | `sp -0x2c4` | `(ret(0x937d8) & 0xff)` |
| `ldr` at 0x92e50 | load | 4 | `abs 0x93004` | `-` |
| `ldrb` at 0x92e58 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0x92e64 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0x92e68 | load | 4 | `abs 0x92ffc` | `-` |
| `ldr` at 0x92e70 | load | 4 | `abs 0x92ff0` | `-` |
| `str` at 0x92e78 | store | 4 | `abs 0x1156e0` | `?` |
| `str` at 0x92e80 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0x92e8c | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0x92e94 | store | 4 | `sp -0x2c0` | `0x0` |
| `ldr` at 0x92e98 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0x92ea8 | load | 4 | `abs 0x9300c` | `-` |
| `ldr` at 0x92eac | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x92ebc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92ed0 | load | 4 | `abs 0x9300c` | `-` |
| `ldr` at 0x92ed4 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x92ee4 | load | 1 | `(sp-16 + [sp-284]) -0x108` | `-` |
| `strb` at 0x92ef0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x92ef4 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x92efc | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldr` at 0x92f00 | load | 4 | `sp -0x2c0` | `-` |
| `ldrb` at 0x92f14 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x92f2c | store | 1 | `(unknown)` | `0x28` |
| `ldr` at 0x92f34 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x92f44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92f48 | load | 4 | `sp -0x2cc` | `-` |
| `ldrb` at 0x92f60 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92f68 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0x92f74 | store | 1 | `(unknown)` | `0xa` |
| `ldrb` at 0x92f80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92f8c | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0x92f9c | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x92fac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x92fb0 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x92fb8 | store | 4 | `sp -0x11c` | `?` |
| `ldr` at 0x92fbc | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0x92fc4 | store | 4 | `sp -0x2c0` | `[sp-704]+1` |
| `ldr` at 0x92fcc | load | 4 | `abs 0x92ff0` | `-` |
| `str` at 0x92fd4 | store | 4 | `abs 0x1156e0` | `0x0` |
| `str` at 0x92fdc | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0x92fe0 | load | 4 | `sp -0x2dc` | `-` |
| `ldmdb` at 0x92fe4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x92fe4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x92fe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x92b9c | `(unknown callee) 0x18920` | `0x2a` | `r1` | `r2` | `r1` |
| `bl` at 0x92bd4 | `(unknown callee) 0x18cd4` | `[sp-716]` | `0x1156b4` | `[sp-24]` | `[sp-716]` |
| `bl` at 0x92c4c | `(unknown callee) 0x936bc` | `[sp-20]` | `sp-696` | `[sp-288]` | `0x0` |
| `bl` at 0x92c5c | `(unknown callee) 0x90884` | `[sp-20]` | `sp-696` | `?` | `sp-696` |
| `bl` at 0x92c7c | `(unknown callee) 0x18ab8` | `[sp-716]` | `0xfbf84` | `?` | `ret(0x90884)` |
| `bl` at 0x92c9c | `(unknown callee) 0x94484` | `[sp-304]` | `?` | `?` | `ret(0x90884)` |
| `bl` at 0x92cbc | `(unknown callee) 0x18ae8` | `sp-728` | `ret(0x94484)+1` | `0x8` | `ret(0x94484)+1` |
| `bl` at 0x92d58 | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-696` | `[sp-288]` | `[sp-706]:2` |
| `bl` at 0x92d88 | `(unknown callee) 0x18ab8` | `[sp-716]` | `0xfbf84` | `ret(0x937d8)` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x92e3c | `(unknown callee) 0x937d8` | `[sp-20]` | `sp-696` | `[sp-288]` | `([sp-706]:2+3 & 0xffff)` |
| `bl` at 0x92e6c | `(unknown callee) 0x18ab8` | `[sp-716]` | `0xfbf84` | `sp-16` | `(ret(0x937d8) & 0xff)` |
| `bl` at 0x92f54 | `(unknown callee) 0x18ae8` | `[sp-716]` | `(sp-280 + [sp-284])` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x92b70 | `[0x115694]:1 != 0xff` | `[0x115694]:1 == 0xff` |
| `beq` at 0x92b94 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x92bbc | `[sp-716] != 0x0` | `[sp-716] == 0x0` |
| `bne` at 0x92c70 | `ret(0x90884) != 0x0` | `ret(0x90884) == 0x0` |
| `beq` at 0x92cd4 | `[sp-724]:1 == 0x0` | `[sp-724]:1 != 0x0` |
| `beq` at 0x92cec | `[sp-724]:1 == 0xff` | `[sp-724]:1 != 0xff` |
| `bne` at 0x92d7c | `(ret(0x937d8) & 0xff) != 0x0` | `(ret(0x937d8) & 0xff) == 0x0` |
| `bne` at 0x92e60 | `(ret(0x937d8) & 0xff) != 0x0` | `(ret(0x937d8) & 0xff) == 0x0` |
| `ble` at 0x92ea0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x92ec4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x92f08 | `[sp-704] != 0x4` | `[sp-704] == 0x4` |
| `bls` at 0x92f1c | `? <= 0x28 (unsigned, 32-bit)` | `? > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x92fe4 | `r0` | `?` |

## `PBaySdrWarmReset`

- `PBaySdrWarmReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x93eb4 to 0x93f98.
  Established. The halfword store at `sp -0x1ac` writes `([r0+6]:2 & 0xffff)` (`strh` at 0x93ed8).
  Stack words include `0x20` at `sp -0x1a8` and `0x6` at `sp -0x108` (`str` at 0x93ee0, `str` at
  0x93f10). The call to `0x90884` carries entry r0, `sp-428`, `sp-260`, `sp-428` (`bl` at 0x93f34);
  its zero result takes the path storing `0xfc128` at `sp -0x1b4`, while nonzero falls through
  to `0xfc124` there (`beq` at 0x93f48, `str` at 0x93f5c, `str` at 0x93f50). A physical
  warm-reset operation is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x93ec4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x93ec8 | load | 4 | `abs 0x93f88` | `-` |
| `ldr` at 0x93ecc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x93ed0 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0x93ed8 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0x93ee0 | store | 4 | `sp -0x1a8` | `0x20` |
| `str` at 0x93ee8 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0x93ef0 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0x93ef8 | store | 4 | `sp -0x110` | `0x3` |
| `ldr` at 0x93efc | load | 4 | `abs 0x93f8c` | `-` |
| `ldrb` at 0x93f00 | load | 1 | `abs 0x11b35b` | `-` |
| `strb` at 0x93f08 | store | 1 | `sp -0x10c` | `([0x11b35b]:1 & 0xff)` |
| `str` at 0x93f10 | store | 4 | `sp -0x108` | `0x6` |
| `strb` at 0x93f1c | store | 1 | `sp -0x104` | `0x0` |
| `str` at 0x93f24 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x93f2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x93f3c | store | 4 | `sp -0x1b0` | `ret(0x90884)` |
| `ldr` at 0x93f40 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0x93f4c | load | 4 | `abs 0x93f90` | `-` |
| `str` at 0x93f50 | store | 4 | `sp -0x1b4` | `0xfc124` |
| `ldr` at 0x93f58 | load | 4 | `abs 0x93f94` | `-` |
| `str` at 0x93f5c | store | 4 | `sp -0x1b4` | `0xfc128` |
| `str` at 0x93f6c | store | 4 | `sp -0x1b8` | `ret(0x18920)` |
| `ldr` at 0x93f70 | load | 4 | `sp -0x1b8` | `-` |
| `ldr` at 0x93f74 | load | 4 | `sp -0x1b4` | `-` |
| `ldr` at 0x93f7c | load | 4 | `sp -0x1b8` | `-` |
| `ldmdb` at 0x93f84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x93f84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x93f84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x93f34 | `(unknown callee) 0x90884` | `r0` | `sp-428` | `sp-260` | `sp-428` |
| `bl` at 0x93f64 | `(unknown callee) 0x18920` | `0xc` | `?` | `?` | `?` |
| `bl` at 0x93f78 | `(unknown callee) 0x18cd4` | `ret(0x18920)` | `[sp-436]` | `?` | `ret(0x18920)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x93f48 | `ret(0x90884) == 0x0` | `ret(0x90884) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x93f84 | `r0` | `[sp-440]` |

## `L1FRU_2m_Reset`

- `L1FRU_2m_Reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8ca9c to 0x8cb00.
  Established. Two absolute word stores clear `0x282300` and `0x282304` (`str` at 0x8cacc, `str` at
  0x8caf4). Each prior zero test takes its bypass on equality and falls through on nonzero
  (`beq` at 0x8cab4, `beq` at 0x8cadc). The corresponding calls to `0x18be4` have first candidates
  `[0x282300]` and `[0x282304]` (`bl` at 0x8cac0, `bl` at 0x8cae8). The joined r0 return is unknown
  (`ldmdb` at 0x8caf8). The cleared words’ relationship to a physical reset is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8caa8 | load | 4 | `abs 0x8cafc` | `-` |
| `ldr` at 0x8caac | load | 4 | `abs 0x282300` | `-` |
| `ldr` at 0x8cab8 | load | 4 | `abs 0x8cafc` | `-` |
| `ldr` at 0x8cabc | load | 4 | `abs 0x282300` | `-` |
| `ldr` at 0x8cac4 | load | 4 | `abs 0x8cafc` | `-` |
| `str` at 0x8cacc | store | 4 | `abs 0x282300` | `0x0` |
| `ldr` at 0x8cad0 | load | 4 | `abs 0x8cafc` | `-` |
| `ldr` at 0x8cad4 | load | 4 | `abs 0x282304` | `-` |
| `ldr` at 0x8cae0 | load | 4 | `abs 0x8cafc` | `-` |
| `ldr` at 0x8cae4 | load | 4 | `abs 0x282304` | `-` |
| `ldr` at 0x8caec | load | 4 | `abs 0x8cafc` | `-` |
| `str` at 0x8caf4 | store | 4 | `abs 0x282304` | `0x0` |
| `ldmdb` at 0x8caf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8caf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8caf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cac0 | `(unknown callee) 0x18be4` | `[0x282300]` | `r1` | `r2` | `0x2822e0` |
| `bl` at 0x8cae8 | `(unknown callee) 0x18be4` | `[0x282304]` | `?` | `0x2822e0` | `0x2822e0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8cab4 | `[0x282300] == 0x0` | `[0x282300] != 0x0` |
| `beq` at 0x8cadc | `[0x282304] == 0x0` | `[0x282304] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8caf8 | `r0` | `?` |

## `routerExitServiceMode`

- `routerExitServiceMode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaae48 to 0xaaf6c.
  Established. The first three calls to `0x1abd0` use first/second candidate pairs `0x50`/`0x0`,
  `0x3e`/`0x0`, `0x3f`/`0x0` (`bl` at 0xaae78, `bl` at 0xaae84, `bl` at 0xaae90). A branch takes its
  longer path for `[sp-20] != 0x1` and falls through on equality (`bne` at 0xaae9c). That longer
  path includes word `0x1` at absolute `0x119e70` (`str` at 0xaaebc). The last call targets
  `0x1d6d8` with candidates `0x107b4c` followed by three unknown values (`bl` at 0xaaf58); its
  result reaches the return, with the service-mode interpretation unknown (`ldmdb` at 0xaaf5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaae58 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xaae5c | load | 4 | `abs 0xaaf60` | `-` |
| `ldm` at 0xaae64 | load | 4 | `abs 0x107b40` | `-` |
| `ldm` at 0xaae64 | load | 4 | `abs 0x107b44` | `-` |
| `ldm` at 0xaae64 | load | 4 | `abs 0x107b48` | `-` |
| `stm` at 0xaae68 | store | 4 | `sp -0x20` | `[0x107b40]` |
| `stm` at 0xaae68 | store | 4 | `sp -0x1c` | `[0x107b44]` |
| `strb` at 0xaae6c | store | 1 | `sp -0x18` | `([0x107b48] & 0xff)` |
| `ldr` at 0xaae94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaaeb4 | load | 4 | `abs 0xaaf64` | `-` |
| `str` at 0xaaebc | store | 4 | `abs 0x119e70` | `0x1` |
| `ldr` at 0xaaf54 | load | 4 | `abs 0xaaf68` | `-` |
| `ldmdb` at 0xaaf5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaaf5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaaf5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaae78 | `(unknown callee) 0x1abd0` | `0x50` | `0x0` | `[0x107b48]` | `0x107b40` |
| `bl` at 0xaae84 | `(unknown callee) 0x1abd0` | `0x3e` | `0x0` | `?` | `?` |
| `bl` at 0xaae90 | `(unknown callee) 0x1abd0` | `0x3f` | `0x0` | `?` | `?` |
| `bl` at 0xaaeac | `(unknown callee) 0x1abd0` | `0xd` | `sp-32` | `?` | `sp-32` |
| `bl` at 0xaaecc | `(unknown callee) 0x1abd0` | `0xd` | `sp-32` | `0x119e70` | `sp-32` |
| `bl` at 0xaaed8 | `(unknown callee) 0x1abd0` | `0x3d` | `0x2` | `?` | `?` |
| `bl` at 0xaaee4 | `(unknown callee) 0x1abd0` | `0x7b` | `0x1` | `?` | `?` |
| `bl` at 0xaaee8 | `(unknown callee) 0x2b31c` | `ret(0x1abd0)` | `?` | `?` | `?` |
| `bl` at 0xaaefc | `(unknown callee) 0x2b31c` | `ret(0x2b31c)` | `?` | `?` | `?` |
| `bl` at 0xaaf10 | `(unknown callee) 0x2b31c` | `ret(0x2b31c)` | `?` | `?` | `?` |
| `bl` at 0xaaf24 | `(unknown callee) 0x2b31c` | `ret(0x2b31c)` | `?` | `?` | `?` |
| `bl` at 0xaaf44 | `(unknown callee) 0x3efcc` | `0x6` | `0x2` | `?` | `?` |
| `bl` at 0xaaf50 | `(unknown callee) 0x3efcc` | `0x7` | `0x2` | `?` | `?` |
| `bl` at 0xaaf58 | `(unknown callee) 0x1d6d8` | `0x107b4c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaae9c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0xaaef8 | `? == 0x52` | `? != 0x52` |
| `beq` at 0xaaf0c | `? == 0x44` | `? != 0x44` |
| `beq` at 0xaaf20 | `? == 0x36` | `? != 0x36` |
| `beq` at 0xaaf34 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaaf5c | `r0` | `ret(0x1d6d8)` |

## `MInfoReset`

- `MInfoReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa454 to 0xaa4f4.
  Established. A nonzero word at absolute `0x11b794` takes the longer path, while zero falls through
  to the return (`bne` at 0xaa470). The longer path clears words at `0x11b77c` and `0x11b780`
  (`str` at 0xaa480, `str` at 0xaa48c). Calls to `0x18c20` include `0x14b798`, `0x0`, `0x66664`,
  `0x0` and later `0x11b798`, `0x0`, `0x30000`, unknown (`bl` at 0xaa49c, `bl` at 0xaa4cc). These
  candidates do not establish the callees’ memory-operation contracts. The physical role and joined
  return are unknown (`ldmdb` at 0xaa4d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xaa464 | load | 4 | `abs 0xaa4d4` | `-` |
| `ldr` at 0xaa468 | load | 4 | `abs 0x11b794` | `-` |
| `ldr` at 0xaa478 | load | 4 | `abs 0xaa4d8` | `-` |
| `str` at 0xaa480 | store | 4 | `abs 0x11b77c` | `0x0` |
| `ldr` at 0xaa484 | load | 4 | `abs 0xaa4dc` | `-` |
| `str` at 0xaa48c | store | 4 | `abs 0x11b780` | `0x0` |
| `ldr` at 0xaa490 | load | 4 | `abs 0xaa4e0` | `-` |
| `ldr` at 0xaa498 | load | 4 | `abs 0xaa4e4` | `-` |
| `ldr` at 0xaa4a0 | load | 4 | `abs 0xaa4e8` | `-` |
| `ldr` at 0xaa4a8 | load | 4 | `abs 0xaa4e4` | `-` |
| `ldr` at 0xaa4b0 | load | 4 | `abs 0xaa4ec` | `-` |
| `ldr` at 0xaa4b8 | load | 4 | `abs 0xaa4e4` | `-` |
| `ldr` at 0xaa4c0 | load | 4 | `abs 0xaa4f0` | `-` |
| `ldmdb` at 0xaa4d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa4d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa4d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa49c | `(unknown callee) 0x18c20` | `0x14b798` | `0x0` | `0x66664` | `0x0` |
| `bl` at 0xaa4ac | `(unknown callee) 0x18c20` | `0x1b1dfc` | `0x0` | `0x66664` | `?` |
| `bl` at 0xaa4bc | `(unknown callee) 0x18c20` | `0x218460` | `0x0` | `0x66664` | `?` |
| `bl` at 0xaa4cc | `(unknown callee) 0x18c20` | `0x11b798` | `0x0` | `0x30000` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xaa470 | `[0x11b794] != 0x0` | `[0x11b794] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa4d0 | `r0` | `?` |

## `exceptionIsSoftReset`

- `exceptionIsSoftReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e0f8 to 0x1e174.
  Established. A nonzero word at absolute `0x1169d8` takes the edge to a stack word of `0x1`,
  while zero falls through to the first call (`bne` at 0x1e114, `str` at 0x1e150, `bl` at
  0x1e11c). That call targets `0x1880c` with `0xd6fa4`, entry r1, entry r2, `[0x1169d8]` (`bl` at
  0x1e11c). The later call to `0x187e8` carries `ret(0x1880c)`, `0xd6fb4`, unknown, `ret(0x1880c)`
  (`bl` at 0x1e13c); its nonzero result takes the path to word `0x0` at `sp -0x18`, while zero
  falls through to `0x1` there (`bne` at 0x1e148, `str` at 0x1e15c, `str` at 0x1e150). The
  returned values’ reset classification is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1e108 | load | 4 | `abs 0x1e168` | `-` |
| `ldr` at 0x1e10c | load | 4 | `abs 0x1169d8` | `-` |
| `ldr` at 0x1e118 | load | 4 | `abs 0x1e16c` | `-` |
| `str` at 0x1e124 | store | 4 | `sp -0x14` | `ret(0x1880c)` |
| `ldr` at 0x1e128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e134 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e138 | load | 4 | `abs 0x1e170` | `-` |
| `str` at 0x1e150 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x1e15c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1e160 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x1e164 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1e164 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1e164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e11c | `(unknown callee) 0x1880c` | `0xd6fa4` | `r1` | `r2` | `[0x1169d8]` |
| `bl` at 0x1e13c | `(unknown callee) 0x187e8` | `ret(0x1880c)` | `0xd6fb4` | `?` | `ret(0x1880c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1e114 | `[0x1169d8] != 0x0` | `[0x1169d8] == 0x0` |
| `beq` at 0x1e130 | `ret(0x1880c) == 0x0` | `ret(0x1880c) != 0x0` |
| `bne` at 0x1e148 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1e164 | `r0` | `?` |

## `AsterixEEPROMInit_stub`

- `AsterixEEPROMInit_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bb18 to 0x2bb30.
  Established. The body selects `0x0`, moves it to r0 and returns that value (`mov` at 0x2bb24,
  `mov` at 0x2bb28, `ldmdb` at 0x2bb2c); its only stores are the implicit register saves in the
  prologue (`push` at 0x2bb1c). No calls or conditional branches are reported. An EEPROM
  initialization operation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x2bb2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bb2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bb2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bb2c | `r0` | `0x0` |

## `AsterixLedOn_stub`

- `AsterixLedOn_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bb30 to 0x2bb50.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0x2bb40). The subsequent moves
  select `0x0` for r0 before the return (`mov` at 0x2bb44, `mov` at 0x2bb48, `ldmdb` at 0x2bb4c).
  There are no reported calls or conditional branches. The symbol’s physical light-on implication is
  unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bb40 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0x2bb4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bb4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bb4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bb4c | `r0` | `0x0` |

## `AsterixLedOff_stub`

- `AsterixLedOff_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bb50 to 0x2bb70.
  Established. Entry r0 is saved at `sp -0x14` as a word (`str` at 0x2bb60). The routine then
  selects a zero value for r0 and returns it (`mov` at 0x2bb64, `mov` at 0x2bb68, `ldmdb` at
  0x2bb6c). No call or conditional branch is reported. A light-off operation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bb60 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0x2bb6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bb6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bb6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bb6c | `r0` | `0x0` |

## `AsterixLedBlink_stub`

- `AsterixLedBlink_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2bb70 to 0x2bb94.
  Established. The two explicit word stores save entry r0 and r1 at `sp -0x14` and `sp -0x18` (`str`
  at 0x2bb80, `str` at 0x2bb84). r0 receives `0x0` and returns it (`mov` at 0x2bb88, `mov` at
  0x2bb8c, `ldmdb` at 0x2bb90). The fact sheet reports no calls or conditional branches. Blink
  behavior is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2bb80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2bb84 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0x2bb90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2bb90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2bb90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2bb90 | `r0` | `0x0` |

## `smpcmd_scan_imp`

- `smpcmd_scan_imp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8568 to 0xc85bc.
  Established. The call to `0x18920` has candidates `0xa0`, entry r1, entry r2 and entry r3 (`bl` at
  0xc8588). A word store places its result at entry `r1 +0x0` (`str` at 0xc8590). The next call
  targets `0x18c74` with `[[sp-28]]`, `0x10a67c`, unknown, `[sp-28]` (`bl` at 0xc85a8). Stack word
  `sp -0x2c` receives `0x0` (`str` at 0xc8598). The return reports `0x0`, with scan-command
  semantics unknown (`ldmdb` at 0xc85b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8578 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc857c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc8580 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc8590 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `str` at 0xc8598 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc859c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc85a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc85a4 | load | 4 | `abs 0xc85b8` | `-` |
| `ldmdb` at 0xc85b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc85b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc85b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc85b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8588 | `(unknown callee) 0x18920` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xc85a8 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a67c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc85b4 | `r0` | `0x0` |

## `smpcmd_scan_test`

- `smpcmd_scan_test`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc85bc to 0xc8610.
  Established. A word store puts `ret(0x18920)` at entry `r1 +0x0` (`str` at 0xc85e4). The call
  producing that value uses `0xa0`, entry r1, entry r2 and entry r3 (`bl` at 0xc85dc). A separate
  stack word at `sp -0x20` is cleared (`str` at 0xc85ec). The final call to `0x18c74` carries
  `[[sp-28]]`, `0x10a6b4`, unknown, `[sp-28]` (`bl` at 0xc85fc). The body sets r0 to `0x0` and
  returns it (`mov` at 0xc8600, `mov` at 0xc8604, `ldmdb` at 0xc8608). A physical scan test is
  unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc85cc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc85d0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc85d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc85e4 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `str` at 0xc85ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc85f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc85f4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xc85f8 | load | 4 | `abs 0xc860c` | `-` |
| `ldmdb` at 0xc8608 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc8608 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8608 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8608 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc85dc | `(unknown callee) 0x18920` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xc85fc | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0x10a6b4` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8608 | `r0` | `0x0` |
