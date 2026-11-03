# ARM multiplexer and UART lifecycle callers in 1.40.5

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
These routines have no unresolved computed transfers in the fact sheets.
Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/mux-uart-lifecycle-facts.txt) retain all tool sections.

## `i2cMuxInit`

- `i2cMuxInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4f24 to 0xd513c.
  Established. A four-byte assignment writes `0x1` at absolute `0x2adf94` (`str` at 0xd4fa8). The
  unnamed call to `0x1b454` carries `[sp-44]`, `0x1`, `[0x2adf94]`, unknown (`bl` at 0xd4fd4); its
  result is stored as four bytes at absolute `0x2adf90` (`str` at 0xd4fdc). Other four-byte stores
  include `[sp-40] +0x4` retaining `[sp-32]` and `+0x8` retaining `ret(0x1b454)` (`str` at 0xd5048,
  `str` at 0xd5064). Multiplexer allocation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4f34 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd4f38 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd4f3c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd4f40 | load | 4 | `abs 0xd511c` | `-` |
| `ldr` at 0xd4f44 | load | 4 | `abs 0x2adf90` | `-` |
| `ldr` at 0xd4f50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4f5c | load | 4 | `abs 0xd5120` | `-` |
| `str` at 0xd4f60 | store | 4 | `sp -0x38` | `0x135018` |
| `ldr` at 0xd4f64 | load | 4 | `abs 0xd511c` | `-` |
| `ldr` at 0xd4f68 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd4f6c | store | 4 | `sp -0x34` | `[0x2adf90]` |
| `ldr` at 0xd4f70 | load | 4 | `abs 0xd5124` | `-` |
| `ldr` at 0xd4f74 | load | 4 | `abs 0x2adf94` | `-` |
| `str` at 0xd4f78 | store | 4 | `sp -0x30` | `[0x2adf94]` |
| `ldr` at 0xd4f80 | load | 4 | `abs 0xd5128` | `-` |
| `ldr` at 0xd4f90 | load | 4 | `abs 0xd5124` | `-` |
| `ldr` at 0xd4f94 | load | 4 | `abs 0x2adf94` | `-` |
| `ldr` at 0xd4fa0 | load | 4 | `abs 0xd5124` | `-` |
| `str` at 0xd4fa8 | store | 4 | `abs 0x2adf94` | `0x1` |
| `ldr` at 0xd4fac | load | 4 | `abs 0xd5124` | `-` |
| `ldr` at 0xd4fb0 | load | 4 | `abs 0x2adf94` | `-` |
| `str` at 0xd4fc4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd4fc8 | load | 4 | `abs 0xd511c` | `-` |
| `ldr` at 0xd4fcc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4fdc | store | 4 | `abs 0x2adf90` | `ret(0x1b454)` |
| `ldr` at 0xd4fe0 | load | 4 | `abs 0x2adf90` | `-` |
| `ldr` at 0xd4fec | load | 4 | `abs 0xd512c` | `-` |
| `str` at 0xd4ff0 | store | 4 | `sp -0x38` | `0x135068` |
| `ldr` at 0xd4ff4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4ff8 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd5000 | load | 4 | `abs 0xd5128` | `-` |
| `ldr` at 0xd5010 | load | 4 | `abs 0xd511c` | `-` |
| `ldr` at 0xd5014 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5028 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5030 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd5034 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5038 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd503c | store | 4 | `[sp-40] +0x0` | `?` |
| `ldr` at 0xd5040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5044 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd5048 | store | 4 | `[sp-40] +0x4` | `[sp-32]` |
| `ldr` at 0xd504c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5050 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5054 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0xd5064 | store | 4 | `[sp-40] +0x8` | `ret(0x1b454)` |
| `ldr` at 0xd5068 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xd5074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd507c | load | 4 | `abs 0xd5130` | `-` |
| `ldr` at 0xd5080 | load | 4 | `abs 0xd5134` | `-` |
| `str` at 0xd50ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd50b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd50b8 | load | 4 | `abs 0xd5130` | `-` |
| `ldr` at 0xd50bc | load | 4 | `abs 0xd5134` | `-` |
| `ldr` at 0xd50c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd50d8 | load | 4 | `abs 0xd5138` | `-` |
| `str` at 0xd50dc | store | 4 | `(unknown)` | `0x1350b0` |
| `str` at 0xd50e0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd50e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd50e8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd50f0 | load | 4 | `abs 0xd5128` | `-` |
| `ldr` at 0xd5100 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5104 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5108 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0xd5110 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldmdb` at 0xd5118 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5118 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5118 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4f8c | `(unknown callee) 0x21eb8` | `0x9` | `0x13504c` | `0x54` | `0x0` |
| `bl` at 0xd4fd4 | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `[0x2adf94]` | `?` |
| `bl` at 0xd500c | `(unknown callee) 0x21eb8` | `0x9` | `0x13504c` | `0x60` | `0x0` |
| `bl` at 0xd505c | `(unknown callee) 0x1b454` | `[[sp-40]+4]` | `0x1` | `[sp-40]` | `[sp-40]` |
| `bl` at 0xd5084 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5090 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd50c0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd50d0 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd50fc | `(unknown callee) 0x21eb8` | `0x9` | `0x13504c` | `0x6b` | `0x0` |
| `bl` at 0xd5114 | `(unknown callee) 0x1b4cc` | `[[sp-40]+8]` | `0x1` | `[[sp-40]+4]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd4f4c | `[0x2adf90] == 0x0` | `[0x2adf90] != 0x0` |
| `bne` at 0xd4f58 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd4f9c | `[0x2adf94] != 0x0` | `[0x2adf94] == 0x0` |
| `bne` at 0xd4fe8 | `[0x2adf90] != 0x0` | `[0x2adf90] == 0x0` |
| `bne` at 0xd5070 | `[[sp-40]+8] != 0x0` | `[[sp-40]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5118 | `r0` | `ret(0x1b4cc)` |

## `i2cMuxReset`

- `i2cMuxReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd53d0 to 0xd5440.
  Established. The unnamed call to `0xc8348` uses `0x0`, `r1`, `r2`, `r3` (`bl` at 0xd53e4); its
  zero result takes the branch to the arm containing unnamed calls to `0xd57b8` and `0xc8488` (`beq`
  at 0xd53f8, `bl` at 0xd542c, `bl` at 0xd5430). Those calls have candidates `ret(0xc8348)`,
  unknown, unknown, `ret(0xc8348)`, then `ret(0xd57b8)` followed by three unknown candidates (`bl`
  at 0xd542c, `bl` at 0xd5430). Reset and lock effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd53ec | store | 4 | `sp -0x14` | `ret(0xc8348)` |
| `ldr` at 0xd53f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd53fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5408 | load | 4 | `abs 0xd5438` | `-` |
| `str` at 0xd540c | store | 4 | `sp -0x1c` | `0x135144` |
| `str` at 0xd5410 | store | 4 | `sp -0x18` | `ret(0x21a38)` |
| `ldr` at 0xd5418 | load | 4 | `abs 0xd543c` | `-` |
| `ldmdb` at 0xd5434 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5434 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5434 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd53e4 | `(unknown callee) 0xc8348` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xd5400 | `(unknown callee) 0x21a38` | `ret(0xc8348)` | `?` | `?` | `ret(0xc8348)` |
| `bl` at 0xd5424 | `(unknown callee) 0x21eb8` | `0x1` | `0x13504c` | `0xa5` | `0x0` |
| `bl` at 0xd542c | `(unknown callee) 0xd57b8` | `ret(0xc8348)` | `?` | `?` | `ret(0xc8348)` |
| `bl` at 0xd5430 | `(unknown callee) 0xc8488` | `ret(0xd57b8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd53f8 | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5434 | `r0` | `?` |

## `i2cMuxGetCfg`

- `i2cMuxGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5440 to 0xd54a0.
  Established. The absolute four-byte load reads `0x2adf90` (`ldr` at 0xd5470). The later store
  writes four-byte `[[sp-28]]` at `r1 +0x0` (`str` at 0xd5488). A four-byte read at `[sp-28] +0x4`
  supplies return expression `[[sp-28]+4]` (`ldr` at 0xd5490, `ldmdb` at 0xd5498).
  Configuration-record meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5450 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd5454 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd5458 | load | 4 | `abs 0xd549c` | `-` |
| `ldr` at 0xd545c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5470 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5478 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd547c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5484 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd5488 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd548c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5490 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd5498 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5498 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5498 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5498 | `r0` | `[[sp-28]+4]` |

## `i2cMuxCount`

- `i2cMuxCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd54a0 to 0xd54ec.
  Established. A four-byte absolute read targets `0x2adf90` (`ldr` at 0xd54cc). The four-byte read
  at `[sp-24] +0x4` supplies `[[sp-24]+4]` to the exit (`ldr` at 0xd54dc, `ldmdb` at 0xd54e4). A
  count unit is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd54b0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd54b4 | load | 4 | `abs 0xd54e8` | `-` |
| `ldr` at 0xd54b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd54cc | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd54d4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd54d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd54dc | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd54e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd54e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd54e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd54e4 | `r0` | `[[sp-24]+4]` |

## `i2cMuxIsPresent`

- `i2cMuxIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd54ec to 0xd5570.
  Established. The branch takes `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` to `0xd554c` (`blt` at
  0xd553c); its fall-through stores four-byte `0x0` at `sp -0x20` (`str` at 0xd5544). The taken arm
  reads byte `((r1 & 0xff) + [[sp-28]+8]) +0x0` and assigns its expression as four bytes
  at `sp -0x20` (`ldrb` at 0xd555c, `str` at 0xd5560). Presence semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd54fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5504 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd5508 | load | 4 | `abs 0xd556c` | `-` |
| `ldr` at 0xd550c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5520 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5528 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd552c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5534 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd5544 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd554c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd5550 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5554 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd555c | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd5560 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd5564 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd5568 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5568 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5568 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd553c | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5568 | `r0` | `?` |

## `i2cMuxRead`

- `i2cMuxRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd56a4 to 0xd5750.
  Established. The unnamed call to `0xd61d0` carries `r0`, `(r1 & 0xff)`, unknown, `(r1 & 0xff)`
  (`bl` at 0xd56f4). A byte read targets `([sp-21]:1 + [[sp-32]+8]) +0x0` (`ldrb` at 0xd5710); its
  nonzero value takes the branch to `0xd5728` (`bne` at 0xd5718). The other arm stores four-byte
  `0xfffffc11` at `sp -0x28` (`str` at 0xd5720). The unnamed call to `0xd643c` carries `[sp-20]`,
  `ret(0xd61d0)`, `[sp-28]`, `[([sp-21]:1 + [[sp-32]+8])]:1` (`bl` at 0xd5734). Multiplexer read
  effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd56b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd56bc | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xd56c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd56c4 | load | 4 | `abs 0xd5748` | `-` |
| `ldr` at 0xd56c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd56dc | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd56e4 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd56e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd56ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd56fc | store | 4 | `sp -0x24` | `ret(0xd61d0)` |
| `ldr` at 0xd5700 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd5704 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5708 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd5710 | load | 1 | `([sp-21]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd571c | load | 4 | `abs 0xd574c` | `-` |
| `str` at 0xd5720 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd5728 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd572c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5730 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd573c | store | 4 | `sp -0x28` | `ret(0xd643c)` |
| `ldr` at 0xd5740 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd5744 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5744 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd56f4 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5734 | `(unknown callee) 0xd643c` | `[sp-20]` | `ret(0xd61d0)` | `[sp-28]` | `[([sp-21]:1 + [[sp-32]+8])]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5718 | `[([sp-21]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-32]+8])]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5744 | `r0` | `?` |

## `i2cMuxName`

- `i2cMuxName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5750 to 0xd57b8.
  Established. Two four-byte reads at `ret(0xd61d0) +0x4` follow the unnamed call to `0xd61d0` (`bl`
  at 0xd5778, `ldr` at 0xd5788, `ldr` at 0xd5794). The branch takes `[ret(0xd61d0)+4] != 0x0` to
  `0xd57a8` (`bne` at 0xd579c). The zero fall-through assigns four-byte `0x135164` at `sp -0x20`
  (`str` at 0xd57a4). String-pointer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5760 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5768 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd576c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5770 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5780 | store | 4 | `sp -0x1c` | `ret(0xd61d0)` |
| `ldr` at 0xd5784 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5788 | load | 4 | `ret(0xd61d0) +0x4` | `-` |
| `str` at 0xd578c | store | 4 | `sp -0x20` | `[ret(0xd61d0)+4]` |
| `ldr` at 0xd5790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5794 | load | 4 | `ret(0xd61d0) +0x4` | `-` |
| `ldr` at 0xd57a0 | load | 4 | `abs 0xd57b4` | `-` |
| `str` at 0xd57a4 | store | 4 | `sp -0x20` | `0x135164` |
| `ldr` at 0xd57a8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd57b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd57b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd57b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5778 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd579c | `[ret(0xd61d0)+4] != 0x0` | `[ret(0xd61d0)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd57b0 | `r0` | `?` |

## `i2cMuxSetSwitch`

- `i2cMuxSetSwitch`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd60f4 to 0xd61b0.
  Established. The branch takes `(r0 & 0xff) <= 0x9 (unsigned, 32-bit)` to `0xd6124` (`bls` at
  0xd6114). That arm writes byte `0x0` at absolute `0x2adf8c` and `0x2adf8d` (`strb` at 0xd612c,
  `strb` at 0xd6150). Byte assignments at absolute `0x143818`, `0x143819`, `0x14381a` and `0x14381b`
  have unknown values (`strb` at 0xd6138, `strb` at 0xd6144, `strb` at 0xd615c, `strb` at 0xd6168).
  The unnamed call to `0xc82a4` has candidates `[[sp-17]:1+1265678]:1`, `r1`, `0x13500e`,
  `[[sp-17]:1+1265678]:1` (`bl` at 0xd617c). Switch semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xd6108 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xd610c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xd6118 | load | 4 | `abs 0xd6190` | `-` |
| `str` at 0xd611c | store | 4 | `sp -0x18` | `0xfffff573` |
| `ldr` at 0xd6124 | load | 4 | `abs 0xd6194` | `-` |
| `strb` at 0xd612c | store | 1 | `abs 0x2adf8c` | `0x0` |
| `ldr` at 0xd6130 | load | 4 | `abs 0xd6198` | `-` |
| `strb` at 0xd6138 | store | 1 | `abs 0x143818` | `?` |
| `ldr` at 0xd613c | load | 4 | `abs 0xd619c` | `-` |
| `strb` at 0xd6144 | store | 1 | `abs 0x143819` | `?` |
| `ldr` at 0xd6148 | load | 4 | `abs 0xd61a0` | `-` |
| `strb` at 0xd6150 | store | 1 | `abs 0x2adf8d` | `0x0` |
| `ldr` at 0xd6154 | load | 4 | `abs 0xd61a4` | `-` |
| `strb` at 0xd615c | store | 1 | `abs 0x14381a` | `?` |
| `ldr` at 0xd6160 | load | 4 | `abs 0xd61a8` | `-` |
| `strb` at 0xd6168 | store | 1 | `abs 0x14381b` | `?` |
| `ldr` at 0xd616c | load | 4 | `abs 0xd61ac` | `-` |
| `ldrb` at 0xd6170 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xd6174 | load | 1 | `[sp-17]:1 +0x13500e` | `-` |
| `str` at 0xd6184 | store | 4 | `sp -0x18` | `ret(0xc82a4)` |
| `ldr` at 0xd6188 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd618c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd618c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd618c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd617c | `(unknown callee) 0xc82a4` | `[[sp-17]:1+1265678]:1` | `r1` | `0x13500e` | `[[sp-17]:1+1265678]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd6114 | `(r0 & 0xff) <= 0x9 (unsigned, 32-bit)` | `(r0 & 0xff) > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd618c | `r0` | `?` |

## `i2cMuxLastChannel`

- `i2cMuxLastChannel`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd61b0 to 0xd61d0.
  Established. The absolute byte read targets `0x143819` (`ldrb` at 0xd61c0). The exit carries
  `[0x143819]:1` (`ldmdb` at 0xd61c8). A last-channel interpretation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd61bc | load | 4 | `abs 0xd61cc` | `-` |
| `ldrb` at 0xd61c0 | load | 1 | `abs 0x143819` | `-` |
| `ldmdb` at 0xd61c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd61c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd61c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd61c8 | `r0` | `[0x143819]:1` |

## `uartInit`

- `uartInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca358 to 0xca524.
  Established. The four-byte store at absolute `0x2b2f40` retains `[0x2b2f40]+1` (`str` at 0xca40c).
  The unnamed call to `0x1b4cc` has candidates `[sp-32]`, `0x0`, `0x108`, `[0x2b2f40]+1` (`bl` at
  0xca41c). Later four-byte stores retain `[sp-20]`, `[sp-24]` and `[sp-28]` at `[sp-32] +0x8`,
  `+0xc` and `+0x2c` (`str` at 0xca428, `str` at 0xca434, `str` at 0xca440). UART record-field
  meanings and initialization effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca368 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca36c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca370 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xca374 | load | 4 | `abs 0xca508` | `-` |
| `ldr` at 0xca378 | load | 4 | `abs 0x2b2f40` | `-` |
| `ldr` at 0xca384 | load | 4 | `abs 0xca50c` | `-` |
| `str` at 0xca388 | store | 4 | `sp -0x2c` | `0x1341a8` |
| `ldr` at 0xca390 | load | 4 | `abs 0xca510` | `-` |
| `ldr` at 0xca3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca3b4 | load | 4 | `abs 0xca514` | `-` |
| `str` at 0xca3b8 | store | 4 | `sp -0x2c` | `0x1341d4` |
| `ldr` at 0xca3bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca3c0 | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xca3c8 | load | 4 | `abs 0xca510` | `-` |
| `ldr` at 0xca3d8 | load | 4 | `abs 0xca508` | `-` |
| `ldr` at 0xca3dc | load | 4 | `abs 0x2b2f40` | `-` |
| `ldr` at 0xca3f0 | load | 4 | `abs 0xca518` | `-` |
| `str` at 0xca3f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xca3fc | load | 4 | `abs 0xca508` | `-` |
| `ldr` at 0xca400 | load | 4 | `abs 0xca508` | `-` |
| `ldr` at 0xca404 | load | 4 | `abs 0x2b2f40` | `-` |
| `str` at 0xca40c | store | 4 | `abs 0x2b2f40` | `[0x2b2f40]+1` |
| `ldr` at 0xca410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca424 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xca428 | store | 4 | `[sp-32] +0x8` | `[sp-20]` |
| `ldr` at 0xca42c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca430 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca434 | store | 4 | `[sp-32] +0xc` | `[sp-24]` |
| `ldr` at 0xca438 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca43c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca440 | store | 4 | `[sp-32] +0x2c` | `[sp-28]` |
| `ldr` at 0xca444 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca44c | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xca450 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca458 | store | 4 | `[sp-32] +0x4` | `?` |
| `ldr` at 0xca45c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca474 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca478 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca47c | load | 4 | `abs 0xca51c` | `-` |
| `strb` at 0xca48c | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xca494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca4a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca4ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca4b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca4b4 | load | 4 | `abs 0xca520` | `-` |
| `strb` at 0xca4c4 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xca4cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca4d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca4e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca4e8 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xca4f8 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xca4fc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xca504 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca504 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca39c | `(unknown callee) 0x21eb8` | `0x9` | `0x1341cc` | `0x58` | `0x0` |
| `bl` at 0xca3a4 | `(unknown callee) 0xcb0cc` | `?` | `?` | `?` | `?` |
| `bl` at 0xca3d4 | `(unknown callee) 0x21eb8` | `0x9` | `0x1341cc` | `0x5b` | `0x0` |
| `bl` at 0xca41c | `(unknown callee) 0x1b4cc` | `[sp-32]` | `0x0` | `0x108` | `[0x2b2f40]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xca380 | `[0x2b2f40] <= 0x13 (signed, 32-bit)` | `[0x2b2f40] > 0x13 (signed, 32-bit)` |
| `beq` at 0xca3b0 | `ret(0xcb0cc) == 0x0` | `ret(0xcb0cc) != 0x0` |
| `bls` at 0xca464 | `[sp-20] <= 0x4 (unsigned, 32-bit)` | `[sp-20] > 0x4 (unsigned, 32-bit)` |
| `bhi` at 0xca470 | `[sp-20] > 0xc (unsigned, 32-bit)` | `[sp-20] <= 0xc (unsigned, 32-bit)` |
| `bls` at 0xca49c | `[sp-20] <= 0xc (unsigned, 32-bit)` | `[sp-20] > 0xc (unsigned, 32-bit)` |
| `bhi` at 0xca4a8 | `[sp-20] > 0x10 (unsigned, 32-bit)` | `[sp-20] <= 0x10 (unsigned, 32-bit)` |
| `bls` at 0xca4d4 | `[sp-20] <= 0x10 (unsigned, 32-bit)` | `[sp-20] > 0x10 (unsigned, 32-bit)` |
| `bhi` at 0xca4e0 | `[sp-20] > 0x16 (unsigned, 32-bit)` | `[sp-20] <= 0x16 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca504 | `r0` | `[sp-32]` |

## `uartOpen`

- `uartOpen`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca524 to 0xca99c.
  Established. The unnamed call to `0xcb0cc` has candidates `r0`, `r1`, `r2`, `0x0` (`bl` at
  0xca548); its nonzero result takes the branch to `0xca56c` (`bne` at 0xca55c). Later calls to
  unnamed `0x1b49c` carry `[sp-60]`, `0x80047503`, `0x400`, `[sp-60]`, and `[sp-64]`, `0x80047506`,
  `0x1`, `[sp-64]` (`bl` at 0xca8d0, `bl` at 0xca950). Each is followed by a branch taken for a
  signed-nonnegative result (`bge` at 0xca8dc, `bge` at 0xca95c). UART open flags and ioctl meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca534 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xca538 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xca540 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xca544 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca550 | store | 4 | `sp -0x24` | `ret(0xcb0cc)` |
| `ldr` at 0xca554 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca564 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xca56c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca574 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xca578 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xca584 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xca590 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xca5a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xca5b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca5b4 | load | 4 | `ret(0xcb0cc) +0x34` | `-` |
| `ldr` at 0xca5c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca5cc | load | 4 | `ret(0xcb0cc) +0x34` | `-` |
| `str` at 0xca5dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca5e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca5ec | store | 4 | `(unknown)` | `ret(0x22728)` |
| `str` at 0xca5f4 | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xca5fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca600 | load | 4 | `ret(0xcb0cc) +0x38` | `-` |
| `ldr` at 0xca60c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca618 | load | 4 | `ret(0xcb0cc) +0x38` | `-` |
| `str` at 0xca628 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca630 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca638 | store | 4 | `(unknown)` | `ret(0x22728)` |
| `str` at 0xca640 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xca648 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca64c | load | 4 | `ret(0xcb0cc) +0x34` | `-` |
| `ldr` at 0xca658 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca65c | load | 4 | `ret(0xcb0cc) +0x38` | `-` |
| `ldr` at 0xca66c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca678 | load | 4 | `ret(0xcb0cc) +0x34` | `-` |
| `ldr` at 0xca684 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca690 | load | 4 | `[sp-36] +0x38` | `-` |
| `str` at 0xca6a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca6ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca6b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca6b8 | store | 4 | `(unknown)` | `ret(0x22728)` |
| `ldr` at 0xca6bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xca6c0 | store | 4 | `(unknown)` | `?` |
| `str` at 0xca6c8 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xca6cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca6d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca6e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca6e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca6f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca6f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca704 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca708 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca718 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca720 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xca724 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca72c | store | 4 | `[sp-36] +0x4` | `0x1` |
| `ldr` at 0xca730 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca734 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xca744 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca750 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca75c | load | 4 | `abs 0xca984` | `-` |
| `ldr` at 0xca760 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca768 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca774 | store | 4 | `sp -0x30` | `ret(0x1b400)` |
| `ldr` at 0xca778 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca78c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xca7a0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xca7b8 | load | 4 | `abs 0xca988` | `-` |
| `str` at 0xca7bc | store | 4 | `sp -0x50` | `0x1341f8` |
| `ldr` at 0xca7c0 | load | 4 | `abs 0xca98c` | `-` |
| `str` at 0xca7c4 | store | 4 | `sp -0x4c` | `0x134214` |
| `str` at 0xca7c8 | store | 4 | `sp -0x48` | `ret(0x21704)` |
| `ldr` at 0xca7cc | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xca7d0 | store | 4 | `sp -0x44` | `[ret(0x1b148)]` |
| `ldr` at 0xca7d8 | load | 4 | `abs 0xca990` | `-` |
| `str` at 0xca7ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca7f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca804 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xca810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca81c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca82c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca830 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xca834 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xca838 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca844 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca854 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca858 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xca85c | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xca860 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca86c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca870 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca87c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca880 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca890 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca8a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca8a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca8a8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xca8b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca8b4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca8b8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xca8bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xca8c0 | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xca8c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca8c8 | load | 4 | `abs 0xca994` | `-` |
| `ldr` at 0xca8e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca8e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca8f0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca8f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca900 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xca904 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xca910 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca920 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca924 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca928 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xca930 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca934 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca938 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xca93c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xca940 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xca944 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca948 | load | 4 | `abs 0xca998` | `-` |
| `ldr` at 0xca960 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca964 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca970 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca974 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xca978 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xca97c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xca980 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xca980 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xca980 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca980 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca980 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca548 | `(unknown callee) 0xcb0cc` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xca5c4 | `(unknown callee) 0x22728` | `ret(0xcb0cc)` | `?` | `?` | `[ret(0xcb0cc)+52]` |
| `bl` at 0xca5e8 | `(unknown callee) 0x22728` | `?` | `?` | `?` | `[ret(0xcb0cc)+52]` |
| `bl` at 0xca610 | `(unknown callee) 0x22728` | `ret(0xcb0cc)` | `?` | `?` | `[ret(0xcb0cc)+56]` |
| `bl` at 0xca634 | `(unknown callee) 0x22728` | `?` | `?` | `?` | `[ret(0xcb0cc)+56]` |
| `bl` at 0xca670 | `(unknown callee) 0x22728` | `ret(0xcb0cc)` | `?` | `?` | `?` |
| `bl` at 0xca688 | `(unknown callee) 0x22728` | `ret(0x22728)` | `?` | `ret(0x22728)` | `[ret(0xcb0cc)+52]` |
| `bl` at 0xca6b4 | `(unknown callee) 0x22728` | `?` | `?` | `?` | `?` |
| `bl` at 0xca73c | `(unknown callee) 0xcb1c8` | `[[sp-36]]` | `0xf` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xca76c | `(unknown callee) 0x1b400` | `?` | `[sp-32]` | `0x143710` | `[sp-28]` |
| `bl` at 0xca784 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xca798 | `(unknown callee) 0x1b148` | `ret(0x1b148)` | `?` | `?` | `[ret(0x1b148)]` |
| `bl` at 0xca7a4 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xca7ac | `(unknown callee) 0x1b148` | `ret(0x21704)` | `?` | `?` | `?` |
| `bl` at 0xca7e4 | `(unknown callee) 0x21eb8` | `0x3` | `0x1341cc` | `0xcf` | `0x0` |
| `bl` at 0xca7fc | `(unknown callee) 0x1b148` | `?` | `?` | `?` | `?` |
| `bl` at 0xca8d0 | `(unknown callee) 0x1b49c` | `[sp-60]` | `0x80047503` | `0x400` | `[sp-60]` |
| `bl` at 0xca8e8 | `(unknown callee) 0xca99c` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x1b49c)` |
| `bl` at 0xca950 | `(unknown callee) 0x1b49c` | `[sp-64]` | `0x80047506` | `0x1` | `[sp-64]` |
| `bl` at 0xca968 | `(unknown callee) 0xca99c` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x1b49c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca55c | `ret(0xcb0cc) != 0x0` | `ret(0xcb0cc) == 0x0` |
| `beq` at 0xca580 | `[sp-56] == 0x1` | `[sp-56] != 0x1` |
| `bgt` at 0xca58c | `[sp-56] > 0x1 (signed, 32-bit)` | `[sp-56] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xca598 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0xca5a8 | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xca5bc | `[ret(0xcb0cc)+52] == 0x0` | `[ret(0xcb0cc)+52] != 0x0` |
| `beq` at 0xca5d4 | `[ret(0xcb0cc)+52] == ret(0x22728)` | `[ret(0xcb0cc)+52] != ret(0x22728)` |
| `beq` at 0xca608 | `[ret(0xcb0cc)+56] == 0x0` | `[ret(0xcb0cc)+56] != 0x0` |
| `beq` at 0xca620 | `[ret(0xcb0cc)+56] == ret(0x22728)` | `[ret(0xcb0cc)+56] != ret(0x22728)` |
| `bne` at 0xca654 | `[ret(0xcb0cc)+52] != 0x0` | `[ret(0xcb0cc)+52] == 0x0` |
| `bne` at 0xca664 | `[ret(0xcb0cc)+56] != 0x0` | `[ret(0xcb0cc)+56] == 0x0` |
| `bne` at 0xca680 | `[ret(0xcb0cc)+52] != ret(0x22728)` | `[ret(0xcb0cc)+52] == ret(0x22728)` |
| `bne` at 0xca698 | `[[sp-36]+56] != ret(0x22728)` | `[[sp-36]+56] == ret(0x22728)` |
| `beq` at 0xca6d4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xca6e0 | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `bge` at 0xca6f0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xca700 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca710 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xca74c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca758 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bge` at 0xca780 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `beq` at 0xca794 | `[ret(0x1b148)] == 0x4` | `[ret(0x1b148)] != 0x4` |
| `bge` at 0xca7f8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xca80c | `[ret(0x1b148)] == 0x4` | `[ret(0x1b148)] != 0x4` |
| `beq` at 0xca818 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xca828 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xca840 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xca850 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xca868 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca878 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca888 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca89c | `? == 0x0` | `? != 0x0` |
| `bge` at 0xca8dc | `ret(0x1b49c) >= 0x0 (signed, 32-bit)` | `ret(0x1b49c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xca8fc | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca90c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca91c | `? == 0x0` | `? != 0x0` |
| `bge` at 0xca95c | `ret(0x1b49c) >= 0x0 (signed, 32-bit)` | `ret(0x1b49c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca980 | `r0` | `?` |

## `uartClose`

- `uartClose`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca99c to 0xcab94.
  Established. Four-byte stores at `[sp-24] +0x34` and `+0x38` write `0x0` in the paired-store arm
  (`str` at 0xcaac8, `str` at 0xcaad4). Included unnamed calls to `0x1b094` have four unknown
  candidates (`bl` at 0xcab50, `bl` at 0xcab78). The later four-byte stores at `[sp-24] +0x0` and
  `+0x4` have unknown values (`str` at 0xcab5c, `str` at 0xcab84). The final return is unknown
  (`ldmdb` at 0xcab90). UART ownership and close semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca9ac | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xca9b0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xca9b8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xca9c0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xca9c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca9cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca9d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca9dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca9e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca9f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcaa08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaa14 | load | 4 | `r0 +0x34` | `-` |
| `str` at 0xcaa24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcaa2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcaa34 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `str` at 0xcaa3c | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xcaa44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaa50 | load | 4 | `r0 +0x38` | `-` |
| `str` at 0xcaa60 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcaa68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcaa70 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xcaa78 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xcaa80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaa8c | load | 4 | `r0 +0x34` | `-` |
| `ldr` at 0xcaa98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaaa4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0xcaab8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xcaac0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcaac8 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `ldr` at 0xcaacc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcaad4 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xcaadc | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xcaae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaae4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcaaf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaaf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcab00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcab10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcab1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcab28 | store | 4 | `[sp-24] +0x4` | `?` |
| `str` at 0xcab30 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xcab38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcab48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcab54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcab5c | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xcab60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcab70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcab74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcab7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcab84 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xcab88 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcab90 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcab90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcab90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcab90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcaa0c | `(unknown callee) 0x22728` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xcaa48 | `(unknown callee) 0x22728` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xcaa84 | `(unknown callee) 0x22728` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xcaa9c | `(unknown callee) 0x22728` | `ret(0x22728)` | `?` | `ret(0x22728)` | `[r0+52]` |
| `bl` at 0xcab18 | `(unknown callee) 0xcb30c` | `?` | `?` | `?` | `?` |
| `bl` at 0xcab50 | `(unknown callee) 0x1b094` | `?` | `?` | `?` | `?` |
| `bl` at 0xcab78 | `(unknown callee) 0x1b094` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xca9d8 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `bgt` at 0xca9e4 | `[sp-48] > 0x1 (signed, 32-bit)` | `[sp-48] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xca9f0 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xcaa00 | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `beq` at 0xcaa1c | `[r0+52] == ret(0x22728)` | `[r0+52] != ret(0x22728)` |
| `beq` at 0xcaa58 | `[r0+56] == ret(0x22728)` | `[r0+56] != ret(0x22728)` |
| `bne` at 0xcaa94 | `[r0+52] != ret(0x22728)` | `[r0+52] == ret(0x22728)` |
| `bne` at 0xcaaac | `[[sp-24]+56] != ret(0x22728)` | `[[sp-24]+56] == ret(0x22728)` |
| `bne` at 0xcaaec | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xcaafc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xcab0c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xcab44 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xcab6c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcab90 | `r0` | `?` |

## `uartRead`

- `uartRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcab94 to 0xcad54.
  Established. A signed-positive entry `r2` takes the branch to `0xcabc8` (`bgt` at 0xcabb8). The
  other arm assigns four-byte `0x0` at `sp -0xac` (`str` at 0xcabc0). The unnamed call to `0x1b0f4`
  carries `[[sp-20]]+1`, `sp-160`, `0x0`, `0x0` (`bl` at 0xcac88). A later unnamed call to `0x1b58c`
  has candidates `[[sp-20]]`, `[sp-24]`, `[sp-28]`, `[sp-20]` (`bl` at 0xcacc4); its return is
  retained as four bytes at `sp -0x20` (`str` at 0xcaccc). Read system-call semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcaba4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcaba8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcabac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcabb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcabc0 | store | 4 | `sp -0xac` | `0x0` |
| `str` at 0xcabcc | store | 4 | `sp -0xa8` | `sp-160` |
| `str` at 0xcabd4 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0xcabd8 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xcabe8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xcabec | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xcabf4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xcabf8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xcac00 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xcac08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcac44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac48 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xcac58 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcac60 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xcac64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcac68 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xcac78 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0xcaca0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcaca8 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xcacb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcacb8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xcacbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcacc0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcaccc | store | 4 | `sp -0x20` | `ret(0x1b58c)` |
| `ldr` at 0xcacd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcace4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcacf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcacf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcacfc | load | 4 | `abs 0xcad50` | `-` |
| `ldr` at 0xcad00 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xcad04 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xcad10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcad1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcad30 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcad38 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xcad40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcad44 | store | 4 | `sp -0xac` | `[sp-32]` |
| `ldr` at 0xcad48 | load | 4 | `sp -0xac` | `-` |
| `ldmdb` at 0xcad4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcad4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcad4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcac88 | `(unknown callee) 0x1b0f4` | `[[sp-20]]+1` | `sp-160` | `0x0` | `0x0` |
| `bl` at 0xcac98 | `(unknown callee) 0x1b148` | `ret(0x1b0f4)` | `?` | `?` | `ret(0x1b0f4)` |
| `bl` at 0xcacc4 | `(unknown callee) 0x1b58c` | `[[sp-20]]` | `[sp-24]` | `[sp-28]` | `[sp-20]` |
| `bl` at 0xcacdc | `(unknown callee) 0x1b148` | `ret(0x1b58c)` | `?` | `?` | `ret(0x1b58c)` |
| `bl` at 0xcad08 | `(unknown callee) 0x2079c` | `0x134220` | `[[sp-20]+8]` | `[[sp-20]]` | `[sp-20]` |
| `bl` at 0xcad24 | `(unknown callee) 0xcaeb0` | `[sp-20]` | `0x1` | `?` | `[sp-32]` |
| `bl` at 0xcad28 | `(unknown callee) 0x1b148` | `ret(0xcaeb0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xcabb8 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bls` at 0xcabe0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bge` at 0xcac94 | `ret(0x1b0f4) >= 0x0 (signed, 32-bit)` | `ret(0x1b0f4) < 0x0 (signed, 32-bit)` |
| `bne` at 0xcacd8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xcacec | `[ret(0x1b148)] == 0x23` | `[ret(0x1b148)] != 0x23` |
| `bge` at 0xcad18 | `[sp-32] >= 0x0 (signed, 32-bit)` | `[sp-32] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcad4c | `r0` | `?` |

## `i2cLock`

- `i2cLock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8348 to 0xc8488.
  Established. The unnamed call to `0x1b310` has four unknown candidates; its result is assigned as
  four bytes at absolute `0x2adf04` (`bl` at 0xc83b0, `str` at 0xc83bc). A zero `[sp-17]:1` takes
  the branch past the callback arm (`beq` at 0xc83c8). The unknown-target call in that arm carries
  `[sp-17]:1`, unknown, `[sp-17]:1`, `[0x1436d4]` (`mov` at 0xc8414). A later byte store at absolute
  `0x2adf08` retains `([sp-17]:1 & 0xff)` (`strb` at 0xc8450). Lock ownership and callback effects
  are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc835c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8360 | load | 4 | `abs 0xc8464` | `-` |
| `str` at 0xc836c | store | 4 | `sp -0x18` | `ret(0x1b280)` |
| `ldr` at 0xc8370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8384 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xc8390 | load | 4 | `abs 0xc8468` | `-` |
| `str` at 0xc8394 | store | 4 | `sp -0x24` | `0x133b60` |
| `str` at 0xc8398 | store | 4 | `sp -0x20` | `ret(0x21704)` |
| `ldr` at 0xc83a0 | load | 4 | `abs 0xc846c` | `-` |
| `ldr` at 0xc83a4 | load | 4 | `abs 0xc8470` | `-` |
| `ldr` at 0xc83b8 | load | 4 | `abs 0xc8474` | `-` |
| `str` at 0xc83bc | store | 4 | `abs 0x2adf04` | `ret(0x1b310)` |
| `ldrb` at 0xc83c0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc83cc | load | 4 | `abs 0xc8478` | `-` |
| `ldr` at 0xc83d0 | load | 4 | `abs 0x1436d4` | `-` |
| `ldr` at 0xc83dc | load | 4 | `abs 0xc847c` | `-` |
| `str` at 0xc83e0 | store | 4 | `sp -0x24` | `0x133b7c` |
| `ldrb` at 0xc83e4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc83e8 | store | 4 | `sp -0x20` | `[sp-17]:1` |
| `ldr` at 0xc83f0 | load | 4 | `abs 0xc846c` | `-` |
| `ldr` at 0xc83f4 | load | 4 | `abs 0xc8480` | `-` |
| `ldr` at 0xc8400 | load | 4 | `abs 0xc8478` | `-` |
| `ldrb` at 0xc8404 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8408 | load | 4 | `abs 0x1436d4` | `-` |
| `str` at 0xc841c | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xc8420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8438 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xc8440 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc8448 | load | 4 | `abs 0xc8484` | `-` |
| `ldrb` at 0xc844c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xc8450 | store | 1 | `abs 0x2adf08` | `([sp-17]:1 & 0xff)` |
| `str` at 0xc8458 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc845c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8364 | `(unknown callee) 0x1b280` | `0x2adee8` | `r1` | `r2` | `r0` |
| `bl` at 0xc837c | `(unknown callee) 0x1b148` | `ret(0x1b280)` | `?` | `?` | `ret(0x1b280)` |
| `bl` at 0xc8388 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xc83ac | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x141` | `0x0` |
| `bl` at 0xc83b0 | `(unknown callee) 0x1b310` | `?` | `?` | `?` | `?` |
| `bl` at 0xc83fc | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x14a` | `0x0` |
| `mov` at 0xc8414 | `(unknown callee) (unknown target)` | `[sp-17]:1` | `?` | `[sp-17]:1` | `[0x1436d4]` |
| `bl` at 0xc842c | `(unknown callee) 0xc8488` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xc8430 | `(unknown callee) 0x1b148` | `ret(0xc8488)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc8378 | `ret(0x1b280) >= 0x0 (signed, 32-bit)` | `ret(0x1b280) < 0x0 (signed, 32-bit)` |
| `beq` at 0xc83c8 | `[sp-17]:1 == 0x0` | `[sp-17]:1 != 0x0` |
| `bne` at 0xc83d8 | `[0x1436d4] != 0x0` | `[0x1436d4] == 0x0` |
| `beq` at 0xc8428 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8460 | `r0` | `?` |

## `i2cUnlock`

- `i2cUnlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8488 to 0xc857c.
  Established. The four-byte assignment at absolute `0x2adf04` writes `0x0` (`str` at 0xc84a0). A
  byte read at absolute `0x2adf08` supplies the branch taken for `[0x2adf08]:1 == 0x0` (`ldrb` at
  0xc84a8, `beq` at 0xc84b0). The other arm includes an unknown-target call with `[0x2adf08]:1`,
  `r1`, `[0x2adf00]`, `[0x2adf08]:1` (`mov` at 0xc84dc). A later byte-zero store targets absolute
  `0x2adf08` (`strb` at 0xc8504). The unnamed call to `0x1b4fc` uses `0x2adee8` and three unknown
  candidates (`bl` at 0xc850c). Effects on the lock remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc8498 | load | 4 | `abs 0xc855c` | `-` |
| `str` at 0xc84a0 | store | 4 | `abs 0x2adf04` | `0x0` |
| `ldr` at 0xc84a4 | load | 4 | `abs 0xc8560` | `-` |
| `ldrb` at 0xc84a8 | load | 1 | `abs 0x2adf08` | `-` |
| `ldr` at 0xc84b4 | load | 4 | `abs 0xc8564` | `-` |
| `ldr` at 0xc84b8 | load | 4 | `abs 0x2adf00` | `-` |
| `ldr` at 0xc84c4 | load | 4 | `abs 0xc8564` | `-` |
| `ldr` at 0xc84c8 | load | 4 | `abs 0xc8560` | `-` |
| `ldrb` at 0xc84cc | load | 1 | `abs 0x2adf08` | `-` |
| `ldr` at 0xc84d0 | load | 4 | `abs 0x2adf00` | `-` |
| `str` at 0xc84e4 | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0xc84e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc84f4 | load | 4 | `abs 0xc8568` | `-` |
| `ldr` at 0xc84fc | load | 4 | `abs 0xc8560` | `-` |
| `strb` at 0xc8504 | store | 1 | `abs 0x2adf08` | `0x0` |
| `ldr` at 0xc8508 | load | 4 | `abs 0xc856c` | `-` |
| `str` at 0xc8514 | store | 4 | `sp -0x14` | `ret(0x1b4fc)` |
| `ldr` at 0xc8518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc852c | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xc8538 | load | 4 | `abs 0xc8570` | `-` |
| `str` at 0xc853c | store | 4 | `sp -0x1c` | `0x133bac` |
| `str` at 0xc8540 | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xc8548 | load | 4 | `abs 0xc8574` | `-` |
| `ldr` at 0xc854c | load | 4 | `abs 0xc8578` | `-` |
| `ldmdb` at 0xc8558 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8558 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xc84dc | `(unknown callee) (unknown target)` | `[0x2adf08]:1` | `r1` | `[0x2adf00]` | `[0x2adf08]:1` |
| `bl` at 0xc84f8 | `(unknown callee) 0x202d0` | `0x133b94` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xc850c | `(unknown callee) 0x1b4fc` | `0x2adee8` | `?` | `?` | `?` |
| `bl` at 0xc8524 | `(unknown callee) 0x1b148` | `ret(0x1b4fc)` | `?` | `?` | `ret(0x1b4fc)` |
| `bl` at 0xc8530 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xc8554 | `(unknown callee) 0x21eb8` | `0x9` | `0x133a6c` | `0x175` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc84b0 | `[0x2adf08]:1 == 0x0` | `[0x2adf08]:1 != 0x0` |
| `beq` at 0xc84c0 | `[0x2adf00] == 0x0` | `[0x2adf00] != 0x0` |
| `beq` at 0xc84f0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bge` at 0xc8520 | `ret(0x1b4fc) >= 0x0 (signed, 32-bit)` | `ret(0x1b4fc) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8558 | `r0` | `?` |
