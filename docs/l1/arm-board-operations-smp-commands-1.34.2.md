# ARM board operations and SMP commands in 1.34.2

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
SMPCmdCheckBrickType, smpcmd_mgn and smpcmd_tst have unresolved computed selectors. Accesses and return values cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/board-operations-smp-commands-facts.txt) retain all tool sections.

## `charioPort`

- `charioPort`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20d5c to 0x20d7c.
  Established. A word read uses absolute `0x1170e4` (`ldr` at 0x20d6c). The loaded
  word reaches the exit as `[0x1170e4]` in r0 (`ldmdb` at 0x20d74). No calls or conditional branches
  appear. The returned word's port representation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x20d68 | load | 4 | `abs 0x20d78` | `-` |
| `ldr` at 0x20d6c | load | 4 | `abs 0x1170e4` | `-` |
| `ldmdb` at 0x20d74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20d74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20d74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20d74 | `r0` | `[0x1170e4]` |

## `OpusTio_BrickOpsInit`

- `OpusTio_BrickOpsInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x371e0 to 0x3738c.
  Established. Word `0x37188` is stored at `r0 +0x0` (`str` at 0x37204). The following word and byte
  stores include the targets recorded in the access table. Entry r0 remains the exit value (`ldmdb`
  at 0x3734c). This body has no recorded calls or conditional branches, and its board-operation
  field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x371f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x371f4 | load | 4 | `abs 0x37350` | `-` |
| `str` at 0x371f8 | store | 4 | `sp -0x18` | `0x1155c4` |
| `ldr` at 0x371fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37200 | load | 4 | `abs 0x37354` | `-` |
| `str` at 0x37204 | store | 4 | `r0 +0x0` | `0x37188` |
| `ldr` at 0x37208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3720c | load | 4 | `abs 0x37358` | `-` |
| `str` at 0x37210 | store | 4 | `[sp-20] +0x4` | `0x36f48` |
| `ldr` at 0x37214 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x3721c | store | 1 | `[sp-20] +0xc` | `0x52` |
| `ldr` at 0x37220 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37228 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x3722c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37234 | store | 4 | `[sp-20] +0x14` | `0x11` |
| `ldr` at 0x37238 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37240 | store | 4 | `[sp-20] +0x18` | `0xff` |
| `ldr` at 0x37244 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3724c | store | 4 | `[sp-20] +0x1c` | `0xff` |
| `ldr` at 0x37250 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37258 | store | 4 | `[sp-20] +0x20` | `0xff` |
| `ldr` at 0x3725c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37260 | load | 4 | `abs 0x3735c` | `-` |
| `str` at 0x37264 | store | 4 | `[sp-20] +0x28` | `0xe00ac` |
| `ldr` at 0x37268 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3726c | load | 4 | `abs 0x37360` | `-` |
| `str` at 0x37270 | store | 4 | `[sp-20] +0x24` | `0xe54ec` |
| `ldr` at 0x37274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37278 | load | 4 | `abs 0x37364` | `-` |
| `str` at 0x3727c | store | 4 | `[sp-20] +0x90` | `0x361a4` |
| `ldr` at 0x37280 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37288 | store | 4 | `[sp-20] +0x7c` | `0x1e` |
| `ldr` at 0x3728c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37294 | store | 4 | `[sp-20] +0x80` | `0x2d` |
| `ldr` at 0x37298 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x372a0 | store | 4 | `[sp-20] +0x84` | `0x32` |
| `ldr` at 0x372a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x372ac | store | 4 | `[sp-20] +0x88` | `0x37` |
| `ldr` at 0x372b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x372b8 | store | 4 | `[sp-20] +0x8c` | `0x17` |
| `ldr` at 0x372bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x372c4 | store | 4 | `[sp-20] +0xa0` | `0x1` |
| `ldr` at 0x372c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x372d0 | store | 4 | `[sp-20] +0x9c` | `0x2` |
| `ldr` at 0x372d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x372d8 | load | 4 | `abs 0x37368` | `-` |
| `str` at 0x372dc | store | 4 | `[sp-20] +0xa4` | `0x4d1d4` |
| `ldr` at 0x372e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x372e4 | load | 4 | `abs 0x3736c` | `-` |
| `str` at 0x372e8 | store | 4 | `[sp-20] +0xa8` | `0x4d2bc` |
| `ldr` at 0x372ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x372f0 | load | 4 | `abs 0x37370` | `-` |
| `str` at 0x372f4 | store | 4 | `[sp-20] +0xac` | `0x4d24c` |
| `ldr` at 0x372f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37300 | store | 4 | `[sp-20] +0xb0` | `0x1` |
| `ldr` at 0x37304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37308 | load | 4 | `abs 0x37374` | `-` |
| `str` at 0x3730c | store | 4 | `[sp-20] +0xc0` | `0x35eb8` |
| `ldr` at 0x37310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37314 | load | 4 | `abs 0x37378` | `-` |
| `str` at 0x37318 | store | 4 | `[sp-20] +0xc4` | `0x35f74` |
| `ldr` at 0x3731c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37320 | load | 4 | `abs 0x3737c` | `-` |
| `str` at 0x37324 | store | 4 | `[sp-20] +0xcc` | `0x3602c` |
| `ldr` at 0x37328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3732c | load | 4 | `abs 0x37380` | `-` |
| `str` at 0x37330 | store | 4 | `[sp-20] +0xd0` | `0x3607c` |
| `ldr` at 0x37334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37338 | load | 4 | `abs 0x37384` | `-` |
| `str` at 0x3733c | store | 4 | `[sp-20] +0xe8` | `0x36758` |
| `ldr` at 0x37340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37344 | load | 4 | `abs 0x37388` | `-` |
| `str` at 0x37348 | store | 4 | `[sp-20] +0xec` | `0x3667c` |
| `ldmdb` at 0x3734c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3734c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3734c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3734c | `r0` | `r0` |

## `TIOTest_SetSystemAceMode`

- `TIOTest_SetSystemAceMode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37b88 to 0x37c48.
  Established. The first call targets `0x1c79c` with candidates `0x99`, entry r1, entry r2 and entry
  r3 (`bl` at 0x37b9c). Zero from that call takes the branch and nonzero falls through (`beq` at
  0x37bb4). Another call targets `0xb79a0` with `0xb`, `0x80`, `[sp-32]` and unknown as candidates
  (`bl` at 0x37bd8). The r0 exit value is `[sp-28]` (`ldmdb` at 0x37c44). The SystemACE signal
  interpretation remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37ba0 | store | 4 | `sp -0x14` | `ret(0x1c79c)` |
| `str` at 0x37ba8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x37bac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37bbc | store | 4 | `sp -0x20` | `0x80` |
| `str` at 0x37bc8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x37bd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x37be0 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x37be4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37bf4 | store | 4 | `sp -0x24` | `0x10` |
| `str` at 0x37c00 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x37c0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x37c18 | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x37c1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x37c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x37c34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x37c38 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x37c3c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x37c44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37c44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37c44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37b9c | `(unknown callee) 0x1c79c` | `0x99` | `r1` | `r2` | `r3` |
| `bl` at 0x37bd8 | `(unknown callee) 0xb79a0` | `0xb` | `0x80` | `[sp-32]` | `?` |
| `bl` at 0x37c10 | `(unknown callee) 0xb79a0` | `0xc` | `0x38` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x37bb4 | `ret(0x1c79c) == 0x0` | `ret(0x1c79c) != 0x0` |
| `beq` at 0x37bec | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `beq` at 0x37c24 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `bne` at 0x37c30 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37c44 | `r0` | `[sp-28]` |

## `TIOTest_BrickOpsInit`

- `TIOTest_BrickOpsInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37c48 to 0x37d90.
  Established. A word store places `0x37b44` at `r0 +0x0` (`str` at 0x37c6c). Entry r0 also supplies
  the recorded exit value (`ldmdb` at 0x37d6c). All explicit stores are retained in the table. The
  body has no calls or conditional branches, and its TIO test configuration role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37c58 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x37c5c | load | 4 | `abs 0x37d70` | `-` |
| `str` at 0x37c60 | store | 4 | `sp -0x18` | `0x1155cc` |
| `ldr` at 0x37c64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37c68 | load | 4 | `abs 0x37d74` | `-` |
| `str` at 0x37c6c | store | 4 | `r0 +0x0` | `0x37b44` |
| `ldr` at 0x37c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37c74 | load | 4 | `abs 0x37d78` | `-` |
| `str` at 0x37c78 | store | 4 | `[sp-20] +0x4` | `0x37994` |
| `ldr` at 0x37c7c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x37c84 | store | 1 | `[sp-20] +0xc` | `0x0` |
| `ldr` at 0x37c88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37c90 | store | 4 | `[sp-20] +0x10` | `0x0` |
| `ldr` at 0x37c94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37c9c | store | 4 | `[sp-20] +0x14` | `0xff` |
| `ldr` at 0x37ca0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37ca8 | store | 4 | `[sp-20] +0x18` | `0xff` |
| `ldr` at 0x37cac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37cb4 | store | 4 | `[sp-20] +0x1c` | `0xff` |
| `ldr` at 0x37cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37cc0 | store | 4 | `[sp-20] +0x20` | `0xff` |
| `ldr` at 0x37cc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37cc8 | load | 4 | `abs 0x37d7c` | `-` |
| `str` at 0x37ccc | store | 4 | `[sp-20] +0x28` | `0xe07b4` |
| `ldr` at 0x37cd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37cd8 | store | 4 | `[sp-20] +0x9c` | `0x0` |
| `ldr` at 0x37cdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37ce4 | store | 4 | `[sp-20] +0xa4` | `0x0` |
| `ldr` at 0x37ce8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37cf0 | store | 4 | `[sp-20] +0xa8` | `0x0` |
| `ldr` at 0x37cf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37cfc | store | 4 | `[sp-20] +0xac` | `0x0` |
| `ldr` at 0x37d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37d04 | load | 4 | `abs 0x37d80` | `-` |
| `str` at 0x37d08 | store | 4 | `[sp-20] +0xc0` | `0x457c8` |
| `ldr` at 0x37d0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37d10 | load | 4 | `abs 0x37d84` | `-` |
| `str` at 0x37d14 | store | 4 | `[sp-20] +0xc4` | `0x45994` |
| `ldr` at 0x37d18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37d20 | store | 4 | `[sp-20] +0xcc` | `0x0` |
| `ldr` at 0x37d24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37d2c | store | 4 | `[sp-20] +0xd0` | `0x0` |
| `ldr` at 0x37d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37d34 | load | 4 | `abs 0x37d88` | `-` |
| `str` at 0x37d38 | store | 4 | `[sp-20] +0xe8` | `0x375d8` |
| `ldr` at 0x37d3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x37d40 | load | 4 | `abs 0x37d8c` | `-` |
| `str` at 0x37d44 | store | 4 | `[sp-20] +0xec` | `0x45be0` |
| `ldr` at 0x37d48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37d50 | store | 4 | `[sp-20] +0x2c` | `0x5` |
| `ldr` at 0x37d54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37d5c | store | 4 | `[sp-20] +0x30` | `0x5` |
| `ldr` at 0x37d60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x37d68 | store | 4 | `[sp-20] +0x34` | `0x8` |
| `ldmdb` at 0x37d6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37d6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37d6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37d6c | `r0` | `r0` |

## `promIceEnabled`

- `promIceEnabled`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41534 to 0x41644.
  Established. The call to `0x41944` has candidates `(r0 & 0xff)`, `sp-24`, `sp-24` and `(r0 &
  0xff)` (`bl` at 0x4155c); its nonzero result takes the branch and zero falls through (`bne` at
  0x41570). Later byte reads use `[sp-24] +0x0` and `[sp-24] +0x1` (`ldrb` at 0x41590, `ldrb` at
  0x41598). PROM enable polarity and the meaning of those bytes are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41548 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4154c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x41564 | store | 4 | `sp -0x1c` | `ret(0x41944)` |
| `ldr` at 0x41568 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41574 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4158c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41590 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x41594 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41598 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0x415ac | store | 4 | `sp -0x1c` | `ret(0xb7a78)` |
| `ldr` at 0x415b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x415b4 | store | 4 | `sp -0x24` | `ret(0xb7a78)` |
| `ldr` at 0x415b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x415c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x415d8 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x415e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x415ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x415f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41604 | load | 4 | `abs 0x4163c` | `-` |
| `str` at 0x41608 | store | 4 | `sp -0x30` | `0xe23e4` |
| `ldrb` at 0x4160c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x41610 | store | 4 | `sp -0x2c` | `[sp-17]:1` |
| `str` at 0x41614 | store | 4 | `sp -0x28` | `ret(0x1ea50)` |
| `ldr` at 0x4161c | load | 4 | `abs 0x41640` | `-` |
| `str` at 0x41630 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x41634 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x41638 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41638 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41638 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4155c | `(unknown callee) 0x41944` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4157c | `(unknown callee) 0x414d4` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x415a4 | `(unknown callee) 0xb7a78` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `[[sp-24]]:1` | `[[sp-24]+1]:1` |
| `bl` at 0x415fc | `(unknown callee) 0x1ea50` | `?` | `?` | `?` | `?` |
| `bl` at 0x41628 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2390` | `0xa7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x41570 | `ret(0x41944) != 0x0` | `ret(0x41944) == 0x0` |
| `beq` at 0x41588 | `ret(0x414d4) == 0x0` | `ret(0x414d4) != 0x0` |
| `beq` at 0x415c0 | `ret(0xb7a78) == 0x0` | `ret(0xb7a78) != 0x0` |
| `beq` at 0x415cc | `ret(0xb7a78) == 0x1` | `ret(0xb7a78) != 0x1` |
| `beq` at 0x415f4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41638 | `r0` | `?` |

## `SMPCmdCheckBrickType`

- `SMPCmdCheckBrickType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7573c to 0x75a08.
  Established. A byte read uses `[sp-24] +0x0` (`ldrb` at 0x75798). The corresponding comparison
  takes the branch for `[[sp-24]]:1 != 0x7e` and falls through on equality (`bne` at 0x757a0). The
  opening call to `0x2b31c` carries entry r0 through r3 as candidates (`bl` at 0x75754). The
  computed transfer has unresolved edges (`ldrls` at 0x757fc), so brick-type acceptance and the
  recorded accesses cover only known paths.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7574c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x75750 | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x75758 | store | 1 | `sp -0x1d` | `(ret(0x2b31c) & 0xff)` |
| `strb` at 0x75760 | store | 1 | `sp -0x1e` | `0x0` |
| `str` at 0x75768 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x75770 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x75778 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x7577c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7578c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x75794 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x75798 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x757a8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x757ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x757b4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrb` at 0x757b8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x757bc | store | 4 | `sp -0x34` | `(ret(0x2b31c) & 0xff)` |
| `ldr` at 0x757c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x757cc | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x757e0 | store | 1 | `sp -0x1d` | `0x58` |
| `strb` at 0x757ec | store | 1 | `sp -0x1d` | `0x4e` |
| `ldrb` at 0x757f0 | load | 1 | `sp -0x1d` | `-` |
| `ldrls` at 0x757fc | load | 4 | `(unknown)` | `-` |
| `strb` at 0x75914 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x75918 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x7591c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x75938 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x75940 | store | 4 | `sp -0x38` | `0x0` |
| `ldrb` at 0x75944 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x75950 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x75954 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x75970 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x75974 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x75978 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7597c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x75980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7598c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75990 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7599c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x759a0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x759ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x759b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x759b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x759c0 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x759c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x759d4 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x759d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x759dc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x759e0 | load | 4 | `abs 0x75a04` | `-` |
| `str` at 0x759ec | store | 4 | `sp -0x30` | `?` |
| `str` at 0x759f8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x759fc | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x75a00 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x75a00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75a00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75a00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75754 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x75924 | `(unknown callee) 0x18770` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x7595c | `(unknown callee) 0x18770` | `[sp-24]` | `[sp-30]:1` | `?` | `[sp-30]:1` |
| `bl` at 0x759b4 | `(unknown callee) 0x18be4` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x759cc | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x759e4 | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0xf133c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x75784 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x757a0 | `[[sp-24]]:1 != 0x7e` | `[[sp-24]]:1 == 0x7e` |
| `beq` at 0x757c8 | `(ret(0x2b31c) & 0xff) == 0x26` | `(ret(0x2b31c) & 0xff) != 0x26` |
| `beq` at 0x757d4 | `(ret(0x2b31c) & 0xff) == 0x59` | `(ret(0x2b31c) & 0xff) != 0x59` |
| `ldrls` at 0x757fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x7594c | `[sp-30]:1 == 0x0` | `[sp-30]:1 != 0x0` |
| `beq` at 0x75968 | `ret(0x18770) == 0x0` | `ret(0x18770) != 0x0` |
| `beq` at 0x75988 | `[sp-36] == [sp-44]` | `[sp-36] != [sp-44]` |
| `beq` at 0x75998 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x759a8 | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75a00 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x757fc).

## `SMPCmdNoResponse`

- `SMPCmdNoResponse`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75ea0 to 0x75eec.
  Established. The call to `0x187e8` supplies entry r0, `0xebab8`, entry r2 and entry r3 as
  candidates (`bl` at 0x75ebc). Nonzero takes the branch and zero falls through (`bne` at 0x75ec8).
  The recorded r0 exit value is `[sp-24]` (`ldmdb` at 0x75ee4). The literal's protocol meaning
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75eb0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x75eb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75eb8 | load | 4 | `abs 0x75ee8` | `-` |
| `str` at 0x75ed0 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x75edc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x75ee0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x75ee4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75ee4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75ee4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75ebc | `(unknown callee) 0x187e8` | `r0` | `0xebab8` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x75ec8 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75ee4 | `r0` | `[sp-24]` |

## `SMPCmdPostProcess`

- `SMPCmdPostProcess`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75eec to 0x75fac.
  Established. A word at `r0 +0x0` is read before the call (`ldr` at 0x75f10). That call targets
  `0x18bcc` with candidates `[r0]`, `0xf14d0`, `sp-56` and `sp-88` (`bl` at 0x75f1c). A result
  unequal to `0x2` takes the branch and equality falls through (`bne` at 0x75f28). The next word
  test takes `[0x11b2ec] == 0x0` and falls through for nonzero (`beq` at 0x75f38). Response-field
  meaning is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75efc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75f00 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x75f04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75f10 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0x75f14 | load | 4 | `abs 0x75f94` | `-` |
| `ldr` at 0x75f2c | load | 4 | `abs 0x75f98` | `-` |
| `ldr` at 0x75f30 | load | 4 | `abs 0x11b2ec` | `-` |
| `ldr` at 0x75f44 | load | 4 | `abs 0x75f9c` | `-` |
| `ldr` at 0x75f60 | load | 4 | `abs 0x75fa0` | `-` |
| `ldr` at 0x75f74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75f78 | load | 4 | `abs 0x75fa4` | `-` |
| `ldr` at 0x75f7c | load | 4 | `abs 0x11b304` | `-` |
| `str` at 0x75f80 | store | 4 | `[sp-24] +0x410` | `[0x11b304]` |
| `ldr` at 0x75f84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75f88 | load | 4 | `abs 0x75fa8` | `-` |
| `str` at 0x75f8c | store | 4 | `[sp-20] +0x0` | `0xf14e8` |
| `ldmdb` at 0x75f90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75f90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75f90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75f1c | `(unknown callee) 0x18bcc` | `[r0]` | `0xf14d0` | `sp-56` | `sp-88` |
| `bl` at 0x75f48 | `(unknown callee) 0x187e8` | `sp-56` | `0xf14dc` | `?` | `sp-56` |
| `bl` at 0x75f64 | `(unknown callee) 0x187e8` | `sp-88` | `0xf14e0` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x75f28 | `ret(0x18bcc) != 0x2` | `ret(0x18bcc) == 0x2` |
| `beq` at 0x75f38 | `[0x11b2ec] == 0x0` | `[0x11b2ec] != 0x0` |
| `bne` at 0x75f54 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |
| `bne` at 0x75f70 | `ret(0x187e8) != 0x0` | `ret(0x187e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75f90 | `r0` | `?` |

## `SMPCmdInit`

- `SMPCmdInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x80e24 to 0x81090.
  Established. The first branch takes `[0x11b140] == 0x0` and falls through for nonzero (`beq` at
  0x80e40). Calls include `0x9a7e0` with candidates `0x282040`, `0x3`, `0x0` and unknown (`bl` at
  0x80e6c). The exit retains `ret(0x9fd1c)` (`ldmdb` at 0x81028). These operands do not establish
  which hardware requests are registered.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x80e34 | load | 4 | `abs 0x8102c` | `-` |
| `ldr` at 0x80e38 | load | 4 | `abs 0x11b140` | `-` |
| `ldr` at 0x80e44 | load | 4 | `abs 0x81030` | `-` |
| `str` at 0x80e48 | store | 4 | `sp -0x28` | `0xf44b0` |
| `ldr` at 0x80e50 | load | 4 | `abs 0x81034` | `-` |
| `ldr` at 0x80e54 | load | 4 | `abs 0x81038` | `-` |
| `ldr` at 0x80e60 | load | 4 | `abs 0x8103c` | `-` |
| `ldr` at 0x80e70 | load | 4 | `abs 0x81040` | `-` |
| `ldr` at 0x80e74 | load | 4 | `abs 0x81044` | `-` |
| `str` at 0x80e84 | store | 4 | `sp -0x14` | `ret(0x9a8f8)` |
| `ldr` at 0x80e88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80ea0 | load | 4 | `abs 0x81048` | `-` |
| `str` at 0x80ea4 | store | 4 | `sp -0x28` | `0xf44f8` |
| `str` at 0x80ea8 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0x80eb0 | load | 4 | `abs 0x81034` | `-` |
| `ldr` at 0x80eb4 | load | 4 | `abs 0x8104c` | `-` |
| `ldr` at 0x80ec0 | load | 4 | `abs 0x8103c` | `-` |
| `ldr` at 0x80ec4 | load | 4 | `abs 0x81050` | `-` |
| `ldr` at 0x80ecc | load | 4 | `abs 0x81040` | `-` |
| `str` at 0x80ed8 | store | 4 | `sp -0x14` | `ret(0x99520)` |
| `ldr` at 0x80edc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80ef4 | load | 4 | `abs 0x81054` | `-` |
| `str` at 0x80ef8 | store | 4 | `sp -0x28` | `0xf451c` |
| `str` at 0x80efc | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0x80f04 | load | 4 | `abs 0x81034` | `-` |
| `ldr` at 0x80f08 | load | 4 | `abs 0x81058` | `-` |
| `ldr` at 0x80f14 | load | 4 | `abs 0x8105c` | `-` |
| `ldr` at 0x80f18 | load | 4 | `abs 0x81060` | `-` |
| `str` at 0x80f28 | store | 4 | `sp -0x14` | `ret(0x9a8f8)` |
| `ldr` at 0x80f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80f38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80f44 | load | 4 | `abs 0x81064` | `-` |
| `str` at 0x80f48 | store | 4 | `sp -0x28` | `0xf454c` |
| `str` at 0x80f4c | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0x80f54 | load | 4 | `abs 0x81034` | `-` |
| `ldr` at 0x80f58 | load | 4 | `abs 0x81068` | `-` |
| `ldr` at 0x80f64 | load | 4 | `abs 0x8103c` | `-` |
| `ldr` at 0x80f68 | load | 4 | `abs 0x81050` | `-` |
| `ldr` at 0x80f70 | load | 4 | `abs 0x8105c` | `-` |
| `str` at 0x80f7c | store | 4 | `sp -0x14` | `ret(0x99520)` |
| `ldr` at 0x80f80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80f8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80f98 | load | 4 | `abs 0x8106c` | `-` |
| `str` at 0x80f9c | store | 4 | `sp -0x28` | `0xf4570` |
| `str` at 0x80fa0 | store | 4 | `sp -0x24` | `ret(0x1e71c)` |
| `ldr` at 0x80fa8 | load | 4 | `abs 0x81034` | `-` |
| `ldr` at 0x80fac | load | 4 | `abs 0x81070` | `-` |
| `ldr` at 0x80fb8 | load | 4 | `abs 0x81074` | `-` |
| `ldr` at 0x80fbc | load | 4 | `abs 0x81078` | `-` |
| `str` at 0x80fd0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x80fd8 | store | 4 | `sp -0x24` | `0x1000` |
| `str` at 0x80fe0 | store | 4 | `sp -0x20` | `0x14` |
| `str` at 0x80fe8 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x80ff0 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0x80ff4 | load | 4 | `abs 0x8107c` | `-` |
| `ldr` at 0x80ff8 | load | 4 | `abs 0x81080` | `-` |
| `ldr` at 0x80ffc | load | 4 | `abs 0x81084` | `-` |
| `ldr` at 0x81008 | load | 4 | `abs 0x81088` | `-` |
| `ldr` at 0x8100c | load | 4 | `abs 0x8108c` | `-` |
| `ldr` at 0x81014 | load | 4 | `abs 0x8102c` | `-` |
| `ldr` at 0x81018 | load | 4 | `abs 0x8102c` | `-` |
| `ldr` at 0x8101c | load | 4 | `abs 0x11b140` | `-` |
| `str` at 0x81024 | store | 4 | `abs 0x11b140` | `[0x11b140]+1` |
| `ldmdb` at 0x81028 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x81028 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x81028 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x80e5c | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x1812` | `0x0` |
| `bl` at 0x80e6c | `(unknown callee) 0x9a7e0` | `0x282040` | `0x3` | `0x0` | `?` |
| `bl` at 0x80e7c | `(unknown callee) 0x9a8f8` | `0x282048` | `0xf44ec` | `0xa` | `?` |
| `bl` at 0x80e98 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x80ebc | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x181f` | `0x0` |
| `bl` at 0x80ed0 | `(unknown callee) 0x99520` | `0x282040` | `0xffff003` | `0x1` | `0x282048` |
| `bl` at 0x80eec | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x80f10 | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x1827` | `0x0` |
| `bl` at 0x80f20 | `(unknown callee) 0x9a8f8` | `0x2820dc` | `0xf4540` | `0xa` | `?` |
| `bl` at 0x80f3c | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0x80f60 | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x182e` | `0x0` |
| `bl` at 0x80f74 | `(unknown callee) 0x99520` | `0x282040` | `0xffff003` | `0x2` | `0x2820dc` |
| `bl` at 0x80f90 | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0x80fb4 | `(unknown callee) 0x1eed0` | `0x9` | `0xf44d4` | `0x1836` | `0x0` |
| `bl` at 0x80fc8 | `(unknown callee) 0x1f9e8` | `0x282174` | `0xf4598` | `0x1` | `0x9` |
| `bl` at 0x81004 | `(unknown callee) 0x1f498` | `0x282170` | `0xf45a4` | `0x82334` | `0x0` |
| `bl` at 0x81010 | `(unknown callee) 0x9fd1c` | `0xec348` | `0x12d` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x80e40 | `[0x11b140] == 0x0` | `[0x11b140] != 0x0` |
| `beq` at 0x80e90 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x80ee4 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |
| `beq` at 0x80f34 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0x80f88 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x81028 | `r0` | `ret(0x9fd1c)` |

## `SMPCmd_2m_Init`

- `SMPCmd_2m_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x89adc to 0x89b24.
  Established. A call to `0x9fd1c` carries `0xf6390`, `0x94`, entry r2 and entry r3 as candidates
  (`bl` at 0x89af0). A word store places `0x871b0` at absolute `0x11b144` (`str` at 0x89afc). The
  exit value is `ret(0x9fd1c)` (`ldmdb` at 0x89b0c). The stored word's use by a later caller is not
  proved here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x89ae8 | load | 4 | `abs 0x89b10` | `-` |
| `ldr` at 0x89af4 | load | 4 | `abs 0x89b14` | `-` |
| `ldr` at 0x89af8 | load | 4 | `abs 0x89b18` | `-` |
| `str` at 0x89afc | store | 4 | `abs 0x11b144` | `0x871b0` |
| `ldr` at 0x89b00 | load | 4 | `abs 0x89b1c` | `-` |
| `ldr` at 0x89b04 | load | 4 | `abs 0x89b20` | `-` |
| `str` at 0x89b08 | store | 4 | `abs 0x11b148` | `0x8778c` |
| `ldmdb` at 0x89b0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x89b0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x89b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x89af0 | `(unknown callee) 0x9fd1c` | `0xf6390` | `0x94` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x89b0c | `r0` | `ret(0x9fd1c)` |

## `smpcmd_ioexp`

- `smpcmd_ioexp`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x97054 to 0x97214.
  Established. A word store writes `ret(0x18920)` at `[sp-28] +0x0` (`str` at 0x970a0). The opening
  call targets `0xb7894` with entry r0 through r3 as candidates (`bl` at 0x9706c). The later result
  decision takes the branch for `ret(0xb78b4) != 0x0` and falls through for zero (`bne` at 0x97140).
  The r0 exit is `0x0` (`ldmdb` at 0x971fc). I/O expander register identities remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x97064 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x97068 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x97074 | store | 4 | `sp -0x2c` | `ret(0xb7894)` |
| `ldr` at 0x97078 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9707c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x970a0 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x970a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x970a8 | store | 4 | `sp -0x28` | `[[sp-28]]` |
| `ldr` at 0x970ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x970b0 | load | 4 | `abs 0x97200` | `-` |
| `ldr` at 0x970b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x970c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x970d0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `str` at 0x970d8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x970dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x970e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x970f0 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x97100 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97104 | load | 4 | `abs 0x97204` | `-` |
| `ldr` at 0x97108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x97110 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97120 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x97128 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldrb` at 0x9712c | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x97144 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97148 | load | 4 | `abs 0x97208` | `-` |
| `ldrb` at 0x97154 | load | 1 | `sp -0x24` | `-` |
| `str` at 0x9716c | store | 4 | `sp -0x20` | `ret(0xb7af8)` |
| `ldr` at 0x97170 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9717c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x97188 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9718c | load | 4 | `abs 0x9720c` | `-` |
| `ldrb` at 0x9719c | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0x971b0 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0x971b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x971b8 | load | 4 | `abs 0x97210` | `-` |
| `ldr` at 0x971c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x971d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x971e0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x971e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x971ec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x971fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x971fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x971fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x971fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9706c | `(unknown callee) 0xb7894` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x97098 | `(unknown callee) 0x18920` | `?` | `?` | `ret(0xb7894)` | `?` |
| `bl` at 0x970b4 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0xfc67c` | `?` | `[[sp-28]]` |
| `bl` at 0x970bc | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x970f8 | `(unknown callee) 0xb8268` | `[sp-36]:1` | `?` | `?` | `[sp-36]:1` |
| `bl` at 0x9710c | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc6c8` | `[sp-36]` | `ret(0xb8268)` |
| `bl` at 0x97114 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x97134 | `(unknown callee) 0xb78b4` | `[sp-36]:1` | `?` | `ret(0x18a1c)` | `[sp-36]:1` |
| `bl` at 0x9714c | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc6d8` | `?` | `ret(0xb78b4)` |
| `bl` at 0x97164 | `(unknown callee) 0xb7af8` | `[sp-36]:1` | `sp-45` | `sp-45` | `[sp-36]:1` |
| `bl` at 0x97180 | `(unknown callee) 0x1ea50` | `ret(0xb7af8)` | `?` | `?` | `ret(0xb7af8)` |
| `bl` at 0x97194 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc6e8` | `ret(0x1ea50)` | `ret(0x1ea50)` |
| `bl` at 0x971a4 | `(unknown callee) 0xb7bf0` | `[sp-36]:1` | `?` | `?` | `[sp-36]:1` |
| `bl` at 0x971c4 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc704` | `[sp-45]:1` | `?` |
| `bl` at 0x971cc | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x970e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x97140 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |
| `beq` at 0x97178 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x971fc | `r0` | `0x0` |

## `smpcmd_intr_count`

- `smpcmd_intr_count`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x97214 to 0x974f4.
  Established. Calls first target `0xaff4c` with entry r0 through r3, then `0xb7894` with
  `ret(0xaff4c)`, unknown, unknown and `ret(0xaff4c)` as candidates (`bl` at 0x9722c, `bl` at
  0x97238). A word store writes `ret(0x18920)` at `[sp-28] +0x0` (`str` at 0x97274). The exit
  records r0 `0x0` (`ldmdb` at 0x974d8). Interrupt counter sources and units have not been
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x97224 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x97228 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x97234 | store | 4 | `sp -0x20` | `ret(0xaff4c)` |
| `str` at 0x97240 | store | 4 | `sp -0x24` | `ret(0xb7894)` |
| `ldr` at 0x97244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x97248 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9724c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x97274 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x97278 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x9727c | store | 4 | `sp -0x28` | `[[sp-28]]` |
| `ldr` at 0x97280 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97284 | load | 4 | `abs 0x974dc` | `-` |
| `ldr` at 0x9728c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9729c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x972a4 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `str` at 0x972ac | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x972b0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x972c0 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x972d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x972d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x972dc | load | 4 | `abs 0x974e0` | `-` |
| `ldr` at 0x972ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x972fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x97304 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x97308 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x97310 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x9731c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x97320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x97324 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x97334 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x9734c | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x9735c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97360 | load | 4 | `abs 0x974e4` | `-` |
| `ldr` at 0x97370 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x9737c | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x97398 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x973a8 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x973b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x973bc | load | 4 | `abs 0x974e8` | `-` |
| `ldr` at 0x973c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x973d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x973e0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x973e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x973ec | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x973f8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x973fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x97400 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x97410 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x97428 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x97438 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9743c | load | 4 | `abs 0x974ec` | `-` |
| `ldr` at 0x9744c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x97458 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x97474 | load | 1 | `sp -0x2c` | `-` |
| `ldrb` at 0x97484 | load | 1 | `sp -0x2c` | `-` |
| `ldr` at 0x97494 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x97498 | load | 4 | `abs 0x974f0` | `-` |
| `ldr` at 0x974a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x974b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x974bc | store | 4 | `sp -0x28` | `([sp-40] + ret(0x18a1c))` |
| `ldr` at 0x974c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x974c8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x974d8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x974d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x974d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x974d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9722c | `(unknown callee) 0xaff4c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x97238 | `(unknown callee) 0xb7894` | `ret(0xaff4c)` | `?` | `?` | `ret(0xaff4c)` |
| `bl` at 0x9726c | `(unknown callee) 0x18920` | `?` | `?` | `([sp-32] + ret(0xb7894))` | `?` |
| `bl` at 0x97288 | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0xfc718` | `?` | `[[sp-28]]` |
| `bl` at 0x97290 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x972c8 | `(unknown callee) 0xac3ec` | `[sp-44]:1` | `?` | `ret(0x18a1c)` | `[sp-44]:1` |
| `bl` at 0x972e8 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc72c` | `[sp-44]+65` | `ret(0xac3ec)` |
| `bl` at 0x972f0 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x9733c | `(unknown callee) 0xaff6c` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x97354 | `(unknown callee) 0xb14ac` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x97368 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc73c` | `ret(0xb14ac)` | `ret(0xb14ac)` |
| `bl` at 0x97384 | `(unknown callee) 0xb115c` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x973a0 | `(unknown callee) 0xb14ac` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x973b0 | `(unknown callee) 0xb115c` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x973c4 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc75c` | `ret(0xb14ac)` | `ret(0xb115c)` |
| `bl` at 0x973cc | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x97418 | `(unknown callee) 0xb78b4` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x97430 | `(unknown callee) 0xb8268` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x97444 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc774` | `ret(0xb8268)` | `ret(0xb8268)` |
| `bl` at 0x97460 | `(unknown callee) 0xb7bb4` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x9747c | `(unknown callee) 0xb8268` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x9748c | `(unknown callee) 0xb7bb4` | `[sp-44]:1` | `?` | `?` | `[sp-44]:1` |
| `bl` at 0x974a0 | `(unknown callee) 0x18c74` | `[sp-40]` | `0xfc794` | `ret(0xb8268)` | `ret(0xb7bb4)` |
| `bl` at 0x974a8 | `(unknown callee) 0x18a1c` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x972b8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x9732c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x97348 | `ret(0xaff6c) != 0x0` | `ret(0xaff6c) == 0x0` |
| `bne` at 0x97378 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x97390 | `ret(0xb115c) != 0x0` | `ret(0xb115c) == 0x0` |
| `blt` at 0x97408 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x97424 | `ret(0xb78b4) != 0x0` | `ret(0xb78b4) == 0x0` |
| `bne` at 0x97454 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x9746c | `ret(0xb7bb4) != 0x0` | `ret(0xb7bb4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x974d8 | `r0` | `0x0` |

## `SMPLibrary`

- `SMPLibrary`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x523e8 to 0x52c78.
  Established. The call to `0x1fb00` carries `0x281580`, unknown, entry r2 and `0x0` as candidates
  (`bl` at 0x52418); its zero result takes the branch, while nonzero falls through (`beq` at
  0x5242c). The next tested stack value takes a branch for `[sp-32] == 0x0` and falls through
  otherwise (`beq` at 0x52464). The final r0 value is unknown (`ldmdb` at 0x52c0c). The SMP command
  grammar is not assigned from these tests.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x523f8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x523fc | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x52404 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x5240c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x52410 | load | 4 | `abs 0x52c10` | `-` |
| `str` at 0x52420 | store | 4 | `sp -0x28` | `ret(0x1fb00)` |
| `ldr` at 0x52424 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52430 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5243c | load | 4 | `abs 0x52c14` | `-` |
| `str` at 0x52440 | store | 4 | `sp -0x90` | `0xe5d00` |
| `str` at 0x52444 | store | 4 | `sp -0x8c` | `ret(0x1e71c)` |
| `ldr` at 0x5244c | load | 4 | `abs 0x52c18` | `-` |
| `ldr` at 0x5245c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52468 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5246c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x52470 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x52478 | store | 4 | `abs 0x281ef4` | `0x51` |
| `str` at 0x52488 | store | 4 | `[sp-32] +0x0` | `ret(0x18920)` |
| `ldr` at 0x5248c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x52490 | store | 4 | `abs 0x281eec` | `[[sp-32]]` |
| `ldr` at 0x52494 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52498 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x524a0 | store | 4 | `abs 0x281ef0` | `0x0` |
| `ldr` at 0x524a4 | load | 4 | `abs 0x281eec` | `-` |
| `str` at 0x524ac | store | 4 | `[0x281eec] +0x0` | `0x0` |
| `ldr` at 0x524b4 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x524bc | store | 4 | `abs 0x281eec` | `0x0` |
| `ldr` at 0x524c0 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x524c8 | store | 4 | `abs 0x281ef0` | `0x0` |
| `ldr` at 0x524cc | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x524d4 | store | 4 | `abs 0x281ef4` | `0x0` |
| `ldr` at 0x524d8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x524dc | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x524e8 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x524ec | load | 4 | `abs 0x2815e0` | `-` |
| `ldr` at 0x524f8 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x524fc | load | 4 | `abs 0x52c20` | `-` |
| `ldr` at 0x52514 | load | 4 | `abs 0x52c10` | `-` |
| `str` at 0x52520 | store | 4 | `sp -0x28` | `ret(0x1fe64)` |
| `ldr` at 0x52524 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52530 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5253c | load | 4 | `abs 0x52c24` | `-` |
| `str` at 0x52540 | store | 4 | `sp -0x90` | `0xe5d28` |
| `str` at 0x52544 | store | 4 | `sp -0x8c` | `ret(0x1e71c)` |
| `ldr` at 0x5254c | load | 4 | `abs 0x52c18` | `-` |
| `str` at 0x52560 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x52568 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5256c | load | 4 | `abs 0x281368` | `-` |
| `ldr` at 0x52570 | load | 4 | `abs 0x52c28` | `-` |
| `ldr` at 0x52578 | load | 4 | `abs 0x52c28` | `-` |
| `strb` at 0x52580 | store | 1 | `abs 0x11a05d` | `0x1` |
| `ldr` at 0x52584 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52588 | load | 4 | `abs 0x281368` | `-` |
| `ldr` at 0x5258c | load | 4 | `abs 0x52c28` | `-` |
| `ldr` at 0x52594 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x5259c | store | 4 | `abs 0x2815e0` | `0x1` |
| `ldr` at 0x525a0 | load | 4 | `abs 0x52c10` | `-` |
| `str` at 0x525ac | store | 4 | `sp -0x28` | `ret(0x1fe64)` |
| `ldr` at 0x525b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x525bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x525c8 | load | 4 | `abs 0x52c24` | `-` |
| `str` at 0x525cc | store | 4 | `sp -0x90` | `0xe5d28` |
| `str` at 0x525d0 | store | 4 | `sp -0x8c` | `ret(0x1e71c)` |
| `ldr` at 0x525d8 | load | 4 | `abs 0x52c18` | `-` |
| `ldr` at 0x525dc | load | 4 | `abs 0x52c2c` | `-` |
| `str` at 0x525ec | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x525f4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x525f8 | load | 4 | `abs 0x2815e4` | `-` |
| `str` at 0x525fc | store | 4 | `sp -0x88` | `[0x2815e4]` |
| `ldr` at 0x52600 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x5260c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x52618 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x52628 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5262c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x52638 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x52640 | store | 4 | `abs 0x2815e4` | `0x2` |
| `ldr` at 0x52644 | load | 4 | `abs 0x52c30` | `-` |
| `ldr` at 0x5264c | load | 4 | `abs 0x52c34` | `-` |
| `ldr` at 0x52654 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52658 | load | 4 | `abs 0x52c38` | `-` |
| `str` at 0x5265c | store | 4 | `abs 0x281638` | `0x2815e8` |
| `ldr` at 0x52664 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x52670 | store | 4 | `sp -0x24` | `ret(0x5342c)` |
| `ldr` at 0x52678 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5267c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x52688 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5268c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52698 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5269c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526a8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526ac | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526b8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526bc | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526c8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526cc | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526d8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526dc | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526e8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526ec | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x526f8 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x526fc | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52708 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5270c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52718 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5271c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52728 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5272c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52738 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5273c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52748 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5274c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52758 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5275c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52768 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5276c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52778 | load | 4 | `abs 0x52c3c` | `-` |
| `ldr` at 0x5277c | load | 4 | `abs 0x119fd8` | `-` |
| `ldr` at 0x52788 | load | 4 | `abs 0x52c40` | `-` |
| `ldr` at 0x527a4 | load | 4 | `abs 0x52c44` | `-` |
| `ldr` at 0x527ac | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x527b0 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x527bc | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x527c0 | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x527d4 | load | 4 | `abs 0x52c48` | `-` |
| `ldr` at 0x527e8 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x527f0 | store | 4 | `abs 0x2815e4` | `0x1` |
| `ldr` at 0x527f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x527fc | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x52808 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5280c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x5281c | load | 4 | `abs 0x52c38` | `-` |
| `ldrb` at 0x52820 | load | 1 | `abs 0x2815e8` | `-` |
| `ldr` at 0x5282c | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52830 | load | 4 | `abs 0x281c9c` | `-` |
| `ldr` at 0x52844 | load | 4 | `abs 0x52c4c` | `-` |
| `ldr` at 0x52850 | load | 4 | `abs 0x52c38` | `-` |
| `ldr` at 0x52858 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5285c | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52860 | load | 4 | `abs 0x281c9c` | `-` |
| `ldr` at 0x52868 | load | 4 | `abs 0x52c50` | `-` |
| `str` at 0x52890 | store | 4 | `abs 0x281c9c` | `?` |
| `ldr` at 0x52894 | load | 4 | `abs 0x52c54` | `-` |
| `ldr` at 0x52898 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5289c | load | 4 | `abs 0x281c9c` | `-` |
| `str` at 0x528a0 | store | 4 | `abs 0x119fb0` | `[0x281c9c]` |
| `ldr` at 0x528a4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x528a8 | load | 4 | `abs 0x281ca0` | `-` |
| `ldr` at 0x528b4 | load | 4 | `abs 0x52c58` | `-` |
| `ldr` at 0x528b8 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x528bc | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x528c0 | load | 4 | `abs 0x281ca0` | `-` |
| `str` at 0x528c8 | store | 4 | `abs 0x281ca0` | `[0x281ca0]+1` |
| `str` at 0x528cc | store | 4 | `abs 0x119fb4` | `[0x281ca0]+1` |
| `ldr` at 0x528d4 | load | 4 | `abs 0x52c58` | `-` |
| `str` at 0x528dc | store | 4 | `abs 0x119fb4` | `0x14` |
| `ldr` at 0x528e0 | load | 4 | `abs 0x52c5c` | `-` |
| `str` at 0x528f0 | store | 4 | `sp -0x24` | `ret(0x53554)` |
| `ldr` at 0x528f4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x528f8 | load | 4 | `abs 0x52c38` | `-` |
| `str` at 0x528fc | store | 4 | `abs 0x281638` | `0x2815e8` |
| `ldr` at 0x52900 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52904 | load | 4 | `abs 0x281638` | `-` |
| `strb` at 0x5290c | store | 1 | `[0x281638] +0x0` | `0x0` |
| `ldr` at 0x52910 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52914 | load | 4 | `abs 0x2815e4` | `-` |
| `ldr` at 0x52930 | load | 4 | `abs 0x52c44` | `-` |
| `ldr` at 0x52938 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x5293c | load | 4 | `abs 0x281ca4` | `-` |
| `ldr` at 0x52950 | load | 4 | `abs 0x52c60` | `-` |
| `ldr` at 0x52958 | load | 4 | `abs 0x52c1c` | `-` |
| `str` at 0x52960 | store | 4 | `abs 0x2815e4` | `0x1` |
| `ldr` at 0x52974 | load | 4 | `abs 0x52c34` | `-` |
| `ldr` at 0x52980 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52984 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x52990 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52994 | load | 4 | `abs 0x281638` | `-` |
| `ldr` at 0x52998 | load | 4 | `abs 0x52c38` | `-` |
| `ldr` at 0x529a4 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x529a8 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x529ac | load | 4 | `abs 0x281638` | `-` |
| `str` at 0x529b8 | store | 4 | `abs 0x281638` | `[0x281638]-1` |
| `strb` at 0x529c0 | store | 1 | `[0x281638] -0x1` | `0x0` |
| `ldr` at 0x529c4 | load | 4 | `abs 0x52c64` | `-` |
| `ldr` at 0x529d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x529d4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x529e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x529e4 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x529f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x529f8 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x52a08 | store | 4 | `sp -0x7c` | `?` |
| `str` at 0x52a10 | store | 4 | `sp -0x80` | `0x0` |
| `str` at 0x52a1c | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0x52a20 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52a24 | load | 4 | `abs 0x281ca0` | `-` |
| `str` at 0x52a28 | store | 4 | `sp -0x80` | `[0x281ca0]` |
| `ldr` at 0x52a2c | load | 4 | `abs 0x52c58` | `-` |
| `ldr` at 0x52a30 | load | 4 | `abs 0x119fb4` | `-` |
| `ldr` at 0x52a34 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x52a48 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52a4c | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52a50 | load | 4 | `abs 0x281638` | `-` |
| `str` at 0x52a5c | store | 4 | `abs 0x281638` | `[0x281638]-1` |
| `ldr` at 0x52a60 | load | 4 | `abs 0x52c68` | `-` |
| `ldr` at 0x52a70 | load | 4 | `abs 0x52c64` | `-` |
| `ldr` at 0x52a7c | load | 4 | `abs 0x52c54` | `-` |
| `ldr` at 0x52a80 | load | 4 | `abs 0x52c54` | `-` |
| `ldr` at 0x52a84 | load | 4 | `abs 0x119fb0` | `-` |
| `ldr` at 0x52a88 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x52a94 | load | 4 | `abs 0x52c50` | `-` |
| `str` at 0x52abc | store | 4 | `abs 0x119fb0` | `?` |
| `ldr` at 0x52ac0 | load | 4 | `abs 0x52c58` | `-` |
| `ldr` at 0x52ac4 | load | 4 | `abs 0x52c58` | `-` |
| `ldr` at 0x52ac8 | load | 4 | `abs 0x119fb4` | `-` |
| `ldr` at 0x52acc | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x52ad4 | store | 4 | `abs 0x119fb4` | `([0x119fb4] + [sp-124])` |
| `ldr` at 0x52ad8 | load | 4 | `abs 0x52c54` | `-` |
| `ldr` at 0x52adc | load | 4 | `abs 0x119fb0` | `-` |
| `ldr` at 0x52af0 | load | 4 | `abs 0x52c4c` | `-` |
| `ldr` at 0x52af8 | load | 4 | `abs 0x52c38` | `-` |
| `ldr` at 0x52b04 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b08 | load | 4 | `abs 0x52c38` | `-` |
| `ldr` at 0x52b18 | load | 4 | `abs 0x52c38` | `-` |
| `str` at 0x52b20 | store | 4 | `abs 0x281638` | `ret(0x18a1c)+0x2815e8` |
| `ldr` at 0x52b24 | load | 4 | `abs 0x52c38` | `-` |
| `ldr` at 0x52b30 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b34 | load | 4 | `abs 0x281638` | `-` |
| `ldr` at 0x52b38 | load | 4 | `abs 0x52c6c` | `-` |
| `ldr` at 0x52b44 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b4c | load | 4 | `abs 0x281638` | `-` |
| `ldr` at 0x52b50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52b58 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x52b5c | store | 1 | `[0x281638] +0x0` | `([[sp-28]]:1 & 0xff)` |
| `str` at 0x52b64 | store | 4 | `abs 0x281638` | `[0x281638]+1` |
| `ldr` at 0x52b68 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b6c | load | 4 | `abs 0x281638` | `-` |
| `strb` at 0x52b74 | store | 1 | `[0x281638] +0x0` | `0x0` |
| `ldr` at 0x52b7c | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b84 | load | 4 | `abs 0x281638` | `-` |
| `ldr` at 0x52b8c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x52b90 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0x52b94 | store | 1 | `(unknown)` | `([[sp-28]]:1 & 0xff)` |
| `ldr` at 0x52b98 | load | 4 | `abs 0x52c1c` | `-` |
| `ldr` at 0x52b9c | load | 4 | `abs 0x281638` | `-` |
| `strb` at 0x52ba4 | store | 1 | `[0x281638] +0x0` | `0x0` |
| `ldr` at 0x52ba8 | load | 4 | `abs 0x52c70` | `-` |
| `ldr` at 0x52bb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x52bb8 | load | 4 | `abs 0x52c10` | `-` |
| `str` at 0x52bc4 | store | 4 | `sp -0x28` | `ret(0x1fe64)` |
| `ldr` at 0x52bc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x52be0 | load | 4 | `abs 0x52c24` | `-` |
| `str` at 0x52be4 | store | 4 | `sp -0x90` | `0xe5d28` |
| `str` at 0x52be8 | store | 4 | `sp -0x8c` | `ret(0x1e71c)` |
| `ldr` at 0x52bf0 | load | 4 | `abs 0x52c18` | `-` |
| `ldr` at 0x52bf4 | load | 4 | `abs 0x52c74` | `-` |
| `ldr` at 0x52c00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x52c04 | store | 4 | `sp -0x84` | `[sp-36]` |
| `ldr` at 0x52c08 | load | 4 | `sp -0x84` | `-` |
| `ldmdb` at 0x52c0c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x52c0c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x52c0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x52c0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x52c0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x52418 | `(unknown callee) 0x1fb00` | `0x281580` | `?` | `r2` | `0x0` |
| `bl` at 0x52434 | `(unknown callee) 0x1e71c` | `ret(0x1fb00)` | `?` | `?` | `ret(0x1fb00)` |
| `bl` at 0x52458 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x288` | `0x0` |
| `bl` at 0x52480 | `(unknown callee) 0x18920` | `0x144` | `?` | `0x281360` | `0x51` |
| `bl` at 0x52504 | `(unknown callee) 0x537a0` | `0x281360` | `0xe5d24` | `0x1` | `[0x2815e0]` |
| `bl` at 0x52518 | `(unknown callee) 0x1fe64` | `0x281580` | `?` | `?` | `ret(0x537a0)` |
| `bl` at 0x52534 | `(unknown callee) 0x1e71c` | `ret(0x1fe64)` | `?` | `?` | `ret(0x1fe64)` |
| `bl` at 0x52558 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x2a0` | `0x0` |
| `bl` at 0x52574 | `(unknown callee) 0xadf60` | `[0x281368]` | `0x11a048` | `?` | `0x281360` |
| `bl` at 0x52590 | `(unknown callee) 0xadf9c` | `[0x281368]` | `0x11a048` | `0x11a048` | `0x281360` |
| `bl` at 0x525a4 | `(unknown callee) 0x1fe64` | `0x281580` | `?` | `0x281360` | `0x1` |
| `bl` at 0x525c0 | `(unknown callee) 0x1e71c` | `ret(0x1fe64)` | `?` | `?` | `ret(0x1fe64)` |
| `bl` at 0x525e4 | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x2b5` | `0x0` |
| `bl` at 0x52648 | `(unknown callee) 0x539b4` | `0xe5d4c` | `?` | `0x281360` | `0x2` |
| `bl` at 0x52650 | `(unknown callee) 0x539b4` | `0x28163c` | `?` | `?` | `?` |
| `bl` at 0x52668 | `(unknown callee) 0x5342c` | `[sp-28]` | `?` | `[0x2815e4]` | `[[sp-28]]:1` |
| `bl` at 0x5278c | `(unknown callee) 0x539b4` | `0xe5d70` | `?` | `[0x2815e4]` | `[0x119fd8]` |
| `bl` at 0x52794 | `(unknown callee) 0x3a8c0` | `?` | `?` | `[0x2815e4]` | `[0x119fd8]` |
| `bl` at 0x527a8 | `(unknown callee) 0x539b4` | `0xe5d94` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x527c4 | `(unknown callee) 0x54938` | `[0x281ca4]` | `?` | `?` | `0x281360` |
| `bl` at 0x527d8 | `(unknown callee) 0x18c74` | `sp-120` | `0xe5dc8` | `ret(0x54938)` | `sp-120` |
| `bl` at 0x527e4 | `(unknown callee) 0x539b4` | `sp-120` | `?` | `?` | `sp-120` |
| `bl` at 0x52854 | `(unknown callee) 0x18cd4` | `?` | `0x2815e8` | `?` | `?` |
| `bl` at 0x528e4 | `(unknown callee) 0x539b4` | `0xe5e00` | `?` | `?` | `?` |
| `bl` at 0x528e8 | `(unknown callee) 0x53554` | `ret(0x539b4)` | `?` | `?` | `?` |
| `bl` at 0x52920 | `(unknown callee) 0x3a8c0` | `ret(0x53554)` | `?` | `[0x281638]` | `[0x2815e4]` |
| `bl` at 0x52934 | `(unknown callee) 0x539b4` | `0xe5d94` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x52940 | `(unknown callee) 0x54938` | `[0x281ca4]` | `?` | `?` | `0x281360` |
| `bl` at 0x52954 | `(unknown callee) 0x18c74` | `sp-120` | `0xe5e04` | `ret(0x54938)` | `sp-120` |
| `bl` at 0x5296c | `(unknown callee) 0x539b4` | `sp-120` | `?` | `0x281360` | `sp-120` |
| `bl` at 0x52978 | `(unknown callee) 0x539b4` | `0x28163c` | `?` | `[0x281638]` | `[0x2815e4]` |
| `bl` at 0x529c8 | `(unknown callee) 0x539b4` | `0xe5e40` | `0x281360` | `[0x281638]-1` | `0x0` |
| `bl` at 0x52a74 | `(unknown callee) 0x539b4` | `0xe5e40` | `0x281360` | `[0x281638]-1` | `0x2815e7` |
| `bl` at 0x52b00 | `(unknown callee) 0x18cd4` | `0x2815e8` | `?` | `?` | `?` |
| `bl` at 0x52b0c | `(unknown callee) 0x18a1c` | `0x2815e8` | `?` | `?` | `?` |
| `bl` at 0x52b28 | `(unknown callee) 0x539b4` | `0x2815e8` | `?` | `ret(0x18a1c)` | `ret(0x18a1c)+0x2815e8` |
| `bl` at 0x52bac | `(unknown callee) 0x539b4` | `0xe5e44` | `?` | `[0x281638]` | `0x0` |
| `bl` at 0x52bb4 | `(unknown callee) 0x539b4` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x52bbc | `(unknown callee) 0x1fe64` | `0x281580` | `?` | `?` | `?` |
| `bl` at 0x52bd8 | `(unknown callee) 0x1e71c` | `ret(0x1fe64)` | `?` | `?` | `ret(0x1fe64)` |
| `bl` at 0x52bfc | `(unknown callee) 0x1eed0` | `0x9` | `0xe5aa0` | `0x3ab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5242c | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x52464 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x524e4 | `[[sp-28]]:1 != 0x18` | `[[sp-28]]:1 == 0x18` |
| `bne` at 0x524f4 | `[0x2815e0] != 0x0` | `[0x2815e0] == 0x0` |
| `beq` at 0x52510 | `ret(0x537a0) == 0x1` | `ret(0x537a0) != 0x1` |
| `beq` at 0x5252c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x525b8 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |
| `beq` at 0x52608 | `[0x2815e4] == 0x1` | `[0x2815e4] != 0x1` |
| `blo` at 0x52614 | `[0x2815e4] < 0x1 (unsigned, 32-bit)` | `[0x2815e4] >= 0x1 (unsigned, 32-bit)` |
| `bhi` at 0x52620 | `[0x2815e4] > 0x3 (unsigned, 32-bit)` | `[0x2815e4] <= 0x3 (unsigned, 32-bit)` |
| `bne` at 0x52634 | `[[sp-28]]:1 != 0x14` | `[[sp-28]]:1 == 0x14` |
| `bne` at 0x52684 | `[[sp-28]]:1 != 0x4` | `[[sp-28]]:1 == 0x4` |
| `beq` at 0x52694 | `[0x119fd8] == 0x43` | `[0x119fd8] != 0x43` |
| `beq` at 0x526a4 | `[0x119fd8] == 0x33` | `[0x119fd8] != 0x33` |
| `beq` at 0x526b4 | `[0x119fd8] == 0x35` | `[0x119fd8] != 0x35` |
| `beq` at 0x526c4 | `[0x119fd8] == 0x34` | `[0x119fd8] != 0x34` |
| `beq` at 0x526d4 | `[0x119fd8] == 0x41` | `[0x119fd8] != 0x41` |
| `beq` at 0x526e4 | `[0x119fd8] == 0x36` | `[0x119fd8] != 0x36` |
| `beq` at 0x526f4 | `[0x119fd8] == 0x39` | `[0x119fd8] != 0x39` |
| `beq` at 0x52704 | `[0x119fd8] == 0x37` | `[0x119fd8] != 0x37` |
| `beq` at 0x52714 | `[0x119fd8] == 0x38` | `[0x119fd8] != 0x38` |
| `beq` at 0x52724 | `[0x119fd8] == 0x30` | `[0x119fd8] != 0x30` |
| `beq` at 0x52734 | `[0x119fd8] == 0x51` | `[0x119fd8] != 0x51` |
| `beq` at 0x52744 | `[0x119fd8] == 0x40` | `[0x119fd8] != 0x40` |
| `beq` at 0x52754 | `[0x119fd8] == 0x4f` | `[0x119fd8] != 0x4f` |
| `beq` at 0x52764 | `[0x119fd8] == 0x42` | `[0x119fd8] != 0x42` |
| `beq` at 0x52774 | `[0x119fd8] == 0x46` | `[0x119fd8] != 0x46` |
| `beq` at 0x52784 | `[0x119fd8] == 0x4c` | `[0x119fd8] != 0x4c` |
| `beq` at 0x527a0 | `ret(0x3a8c0) == 0x1` | `ret(0x3a8c0) != 0x1` |
| `bne` at 0x527b8 | `[0x2815e4] != 0x3` | `[0x2815e4] == 0x3` |
| `beq` at 0x52804 | `[[sp-28]]:1 == 0xd` | `[[sp-28]]:1 != 0xd` |
| `beq` at 0x52814 | `[[sp-28]]:1 == 0xa` | `[[sp-28]]:1 != 0xa` |
| `beq` at 0x52828 | `[0x2815e8]:1 == 0x0` | `[0x2815e8]:1 != 0x0` |
| `bgt` at 0x528b0 | `[0x281ca0] > 0x13 (signed, 32-bit)` | `[0x281ca0] <= 0x13 (signed, 32-bit)` |
| `bne` at 0x5291c | `[0x2815e4] != 0x2` | `[0x2815e4] == 0x2` |
| `beq` at 0x5292c | `ret(0x3a8c0) == 0x1` | `ret(0x3a8c0) != 0x1` |
| `bne` at 0x5298c | `[[sp-28]]:1 != 0x8` | `[[sp-28]]:1 == 0x8` |
| `bls` at 0x529a0 | `[0x281638] <= 0x2815e8 (unsigned, 32-bit)` | `[0x281638] > 0x2815e8 (unsigned, 32-bit)` |
| `beq` at 0x529dc | `[[sp-28]]:1 == 0x10` | `[[sp-28]]:1 != 0x10` |
| `beq` at 0x529ec | `[[sp-28]]:1 == 0xe` | `[[sp-28]]:1 != 0xe` |
| `bne` at 0x52a00 | `[[sp-28]]:1 != 0x10` | `[[sp-28]]:1 == 0x10` |
| `bne` at 0x52a3c | `[0x119fb4] != [sp-128]` | `[0x119fb4] == [sp-128]` |
| `bhi` at 0x52a68 | `[0x281638]-1 > 0x2815e7 (unsigned, 32-bit)` | `[0x281638]-1 <= 0x2815e7 (unsigned, 32-bit)` |
| `bhs` at 0x52b40 | `[0x281638] >= 0x281637 (unsigned, 32-bit)` | `[0x281638] < 0x281637 (unsigned, 32-bit)` |
| `beq` at 0x52bd0 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x52c0c | `r0` | `?` |

## `smpcmd_mgn`

- `smpcmd_mgn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7dc4c to 0x7e144.
  Established. Word reads include `[sp-24] +0x70` and `[[sp-24]+112] +0x0` (`ldr` at 0x7dc84, `ldr`
  at 0x7dc88). The call to `0x7573c` has candidates `[[[sp-24]+112]]`, `[sp-28]`, unknown and
  `[[sp-24]+112]` (`bl` at 0x7dc90). A zero result takes the branch and nonzero falls through (`beq`
  at 0x7dc9c). The computed selector has unresolved edges (`ldrls` at 0x7de3c). Effects on its unresolved taken
  edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7dc5c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7dc60 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x7dc68 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x7dc70 | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0x7dc7c | store | 1 | `sp -0x36` | `(ret(0x2b31c) & 0xff)` |
| `ldr` at 0x7dc80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7dc84 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x7dc88 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0x7dc8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7dca4 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x7dcb8 | store | 4 | `sp -0x2c` | `ret(0x39994)` |
| `ldr` at 0x7dcbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7dcc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7dcd8 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x7dcdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7dce0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x7dce4 | load | 4 | `abs 0x7e12c` | `-` |
| `str` at 0x7dcf0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x7dcf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7dd00 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x7dd08 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `ldr` at 0x7dd14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7dd1c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x7dd24 | load | 4 | `[[sp-24]+112] +0x8` | `-` |
| `str` at 0x7dd28 | store | 4 | `sp -0x24` | `[[[sp-24]+112]+8]` |
| `str` at 0x7dd30 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x7dd3c | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x7dd44 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x7dd48 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7dd4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7dd5c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7dd70 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7dd78 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7dd88 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x7dd94 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0x7dd98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7dd9c | load | 4 | `[sp-24] +0x28` | `-` |
| `ldr` at 0x7dda8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x7ddac | store | 4 | `sp -0x24` | `[sp-68]` |
| `ldr` at 0x7ddb4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x7ddbc | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x7ddc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x7ddc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7ddd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7dde4 | store | 4 | `[sp-28] +0x0` | `ret(0x18920)` |
| `ldr` at 0x7dde8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7ddec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7ddf0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x7ddf4 | load | 4 | `abs 0x7e130` | `-` |
| `ldr` at 0x7ddf8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0x7de04 | store | 4 | `sp -0x4c` | `?` |
| `str` at 0x7de10 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x7de14 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7de20 | store | 4 | `sp -0x34` | `ret(0x3d108)` |
| `ldr` at 0x7de24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7de2c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x7de34 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldrls` at 0x7de3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x7e0bc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x7e114 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x7e118 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7e118 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7e118 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7e118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7dc74 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x7dc90 | `(unknown callee) 0x7573c` | `[[[sp-24]+112]]` | `[sp-28]` | `?` | `[[sp-24]+112]` |
| `bl` at 0x7dcb4 | `(unknown callee) 0x39994` | `sp-40` | `?` | `?` | `sp-40` |
| `bl` at 0x7dcd0 | `(unknown callee) 0x18920` | `0x50` | `?` | `?` | `ret(0x39994)` |
| `bl` at 0x7dce8 | `(unknown callee) 0x18cd4` | `[[sp-28]]` | `0xf31f0` | `?` | `[sp-28]` |
| `bl` at 0x7dddc | `(unknown callee) 0x18920` | `0x50` | `?` | `[sp-68]` | `[sp-44]` |
| `bl` at 0x7ddfc | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0xf34dc` | `[[sp-24]+40]` | `[sp-28]` |
| `bl` at 0x7de18 | `(unknown callee) 0x3d108` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7dc9c | `ret(0x7573c) == 0x0` | `ret(0x7573c) != 0x0` |
| `bne` at 0x7dcc4 | `ret(0x39994) != 0x0` | `ret(0x39994) == 0x0` |
| `bne` at 0x7dd10 | `[[[sp-24]+112]+8] != 0x1a` | `[[[sp-24]+112]+8] == 0x1a` |
| `blo` at 0x7dd54 | `[sp-68] < [sp-44] (unsigned, 32-bit)` | `[sp-68] >= [sp-44] (unsigned, 32-bit)` |
| `bne` at 0x7dd80 | `? != 0xff` | `? == 0xff` |
| `bne` at 0x7dda4 | `[sp-72]+1 != [[sp-24]+40]` | `[sp-72]+1 == [[sp-24]+40]` |
| `bne` at 0x7ddd0 | `[sp-68] != [sp-44]` | `[sp-68] == [sp-44]` |
| `ldrls` at 0x7de3c | `[[[sp-24]+112]+4] <= 0x3 (unsigned, 32-bit)` | `[[[sp-24]+112]+4] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7e118 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x7de3c).

## `smpcmd_tst`

- `smpcmd_tst`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x96af4 to 0x97054.
  Established. The call to `0x18920` receives `0x50`, entry r1, entry r2 and `0x1` as candidates
  (`bl` at 0x96b1c); its result is stored as a word at `r1 +0x0` (`str` at 0x96b24). A later byte
  store clears `[[sp-28]] +0x0` (`strb` at 0x96b34). The computed transfer has an unresolved taken
  edge for unsigned `[[[sp-24]+112]] <= 0x8`, and values above `0x8` fall through (`ldrls` at 0x96b48).
  Hardware test actions behind that selector remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x96b04 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x96b08 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x96b10 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x96b14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x96b24 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0x96b28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x96b2c | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x96b34 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x96b38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x96b3c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0x96b40 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldrls` at 0x96b48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x96ffc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x9700c | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x97010 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x97010 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x97010 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x97010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x96b1c | `(unknown callee) 0x18920` | `0x50` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x96b48 | `[[[sp-24]+112]] <= 0x8 (unsigned, 32-bit)` | `[[[sp-24]+112]] > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x97010 | `r0` | `[sp-76]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x96b48).
