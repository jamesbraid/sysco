# ARM multiplexer, UART and NVRAM callers in 1.40.5

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
The fact sheets report no unresolved computed transfers. Indirect call targets and callee
effects remain unknown. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/i2c-uart-nvram-facts.txt) retain all tool sections.

Four undecodable words lie inside exact ELF function spans. Each consists of raw
bytes `11 fc ff ff`, retained after the rendered fact sheets:

| Routine | Raw start | Exclusive end |
|---|---|---|
| `i2cMuxSelect` | `0xd56a0` | `0xd56a4` |
| `i2cMuxSelectRaw` | `0xd5cc8` | `0xd5ccc` |
| `i2cMuxSelectSubRaw` | `0xd5ebc` | `0xd5ec0` |
| `i2cMuxDeselectRaw` | `0xd60c4` | `0xd60c8` |

Their presence does not establish an error-code meaning. The tables retain the
tracked store values and unknowns without interpreting those words as instructions.

## `i2cMuxNotPresent`

- `i2cMuxNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd513c to 0xd5250.
  Established. The signed branch takes its longer arm when `(r1 & 0xff) >= [[sp-28]+4]` (`bge` at
  0xd518c). The fall-through stores one-byte zero at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`strb` at
  0xd51a4). Two unnamed calls to `0x4f9a4` carry candidate prefixes `r0`, `0x0`, `0x1350a4`,
  `0x1350ac` and `[sp-20]`, `0x0`, `0x1350a4`, `0x1350ac` (`bl` at 0xd51bc, `bl` at 0xd51f8).
  Multiplexer presence and record fields are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd514c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5154 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd5158 | load | 4 | `abs 0xd523c` | `-` |
| `ldr` at 0xd515c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5170 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5178 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd517c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5180 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5184 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd5190 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd5194 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd5198 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd51a4 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd51ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd51b4 | load | 4 | `abs 0xd5240` | `-` |
| `ldr` at 0xd51b8 | load | 4 | `abs 0xd5244` | `-` |
| `str` at 0xd51e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd51e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd51f0 | load | 4 | `abs 0xd5240` | `-` |
| `ldr` at 0xd51f4 | load | 4 | `abs 0xd5244` | `-` |
| `ldr` at 0xd5200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd5210 | load | 4 | `abs 0xd5248` | `-` |
| `str` at 0xd5214 | store | 4 | `(unknown)` | `0x1350f0` |
| `str` at 0xd5218 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd521c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd5220 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd5228 | load | 4 | `abs 0xd524c` | `-` |
| `ldmdb` at 0xd5238 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5238 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5238 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd51bc | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd51c8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd51f8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5208 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5234 | `(unknown callee) 0x21eb8` | `0x6` | `0x13504c` | `0x7a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd518c | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5238 | `r0` | `?` |

## `i2cMuxProbe`

- `i2cMuxProbe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5250 to 0xd53d0.
  Established. The signed entry comparison takes the outer bypass arm for `(r1 & 0xff) >=
  [[sp-36]+4]` (`bge` at 0xd52a0). The other arm calls unnamed target `0xd5570` with candidates
  `r0`, `(r1 & 0xff)`, `0x0`, `(r1 & 0xff)` (`bl` at 0xd52b4). A zero result takes the branch to a
  four-byte `0x1` assignment at `sp -0x34` (`beq` at 0xd52c8, `str` at 0xd53a4). A later store writes
  one-byte zero at `([sp-29]:1 + [[sp-36]+8]) +0x0` (`strb` at 0xd5398). Probe and device-presence
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5260 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd5268 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd526c | load | 4 | `abs 0xd53bc` | `-` |
| `ldr` at 0xd5270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5284 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd528c | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xd5290 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5294 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5298 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0xd52a4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd52a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd52bc | store | 4 | `sp -0x28` | `ret(0xd5570)` |
| `ldr` at 0xd52c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd52cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd52d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd52e0 | load | 4 | `abs 0xd53c0` | `-` |
| `ldr` at 0xd52e4 | load | 4 | `abs 0xd53c4` | `-` |
| `str` at 0xd5310 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd5314 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd531c | load | 4 | `abs 0xd53c0` | `-` |
| `ldr` at 0xd5320 | load | 4 | `abs 0xd53c4` | `-` |
| `ldr` at 0xd532c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xd533c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5350 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd535c | load | 4 | `abs 0xd53c8` | `-` |
| `str` at 0xd5360 | store | 4 | `(unknown)` | `0x135118` |
| `str` at 0xd5364 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0xd5368 | store | 4 | `(unknown)` | `ret(0xd5750)` |
| `str` at 0xd536c | store | 4 | `(unknown)` | `ret(0x21a38)` |
| `ldr` at 0xd5374 | load | 4 | `abs 0xd53cc` | `-` |
| `ldr` at 0xd5384 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd5388 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd538c | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd5398 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `str` at 0xd53a4 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xd53b0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd53b4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd53b8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd53b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd53b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd53b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd53b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd52b4 | `(unknown callee) 0xd5570` | `r0` | `(r1 & 0xff)` | `0x0` | `(r1 & 0xff)` |
| `bl` at 0xd52e8 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd52f4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5324 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5334 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5348 | `(unknown callee) 0xd5750` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd5354 | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd5380 | `(unknown callee) 0x21eb8` | `0x1` | `0x13504c` | `0x8c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd52a0 | `(r1 & 0xff) >= [[sp-36]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-36]+4] (signed, 32-bit)` |
| `beq` at 0xd52c8 | `ret(0xd5570) == 0x0` | `ret(0xd5570) != 0x0` |
| `beq` at 0xd52d4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd53b8 | `r0` | `?` |

## `i2cMuxSelect`

- `i2cMuxSelect`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5570 to 0xd56a4.
  Established. A one-byte load reads `([sp-21]:1 + [[sp-28]+8]) +0x0` (`ldrb` at 0xd55e0); its
  nonzero value takes the branch to the longer arm (`bne` at 0xd55e8). The fall-through stores
  four-byte `0xfffffc11` at `sp -0x28` (`str` at 0xd55f0). Two later unnamed calls to `0xd5a9c` and
  `0xd5cf0` have the same candidate list `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`, `[sp-21]:1` (`bl` at
  0xd565c, `bl` at 0xd567c). The branch deciding between them takes its later arm when
  `[[sp-32]+1]:1 != 0x0` (`bne` at 0xd5648). Bus and selector meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5580 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd5588 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd5590 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xd5594 | load | 4 | `abs 0xd569c` | `-` |
| `ldr` at 0xd5598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd55ac | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd55b4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd55b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd55bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd55cc | store | 4 | `sp -0x20` | `ret(0xd61d0)` |
| `ldr` at 0xd55d0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd55d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd55d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd55e0 | load | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `-` |
| `ldr` at 0xd55ec | load | 4 | `abs 0xd56a0` | `-` |
| `str` at 0xd55f0 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd55f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd55fc | load | 1 | `ret(0xd61d0) +0x1` | `-` |
| `ldr` at 0xd5600 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5620 | store | 4 | `sp -0x24` | `ret(0xc8348)` |
| `ldr` at 0xd5624 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd5630 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5634 | store | 4 | `sp -0x28` | `ret(0xc8348)` |
| `ldr` at 0xd563c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd5640 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd564c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd5650 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd5654 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5664 | store | 4 | `sp -0x24` | `ret(0xd5a9c)` |
| `ldrb` at 0xd566c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd5670 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd5674 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd5684 | store | 4 | `sp -0x24` | `ret(0xd5cf0)` |
| `ldr` at 0xd568c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5690 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xd5694 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd5698 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5698 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5698 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd55c4 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5608 | `(unknown callee) 0xc807c` | `[sp-20]` | `[ret(0xd61d0)+1]:1` | `[sp-21]:1` | `[ret(0xd61d0)+1]:1` |
| `bl` at 0xd5618 | `(unknown callee) 0xc8348` | `?` | `?` | `?` | `?` |
| `bl` at 0xd565c | `(unknown callee) 0xd5a9c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xd567c | `(unknown callee) 0xd5cf0` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xd5688 | `(unknown callee) 0xc8488` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd55e8 | `[([sp-21]:1 + [[sp-28]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-28]+8])]:1 == 0x0` |
| `beq` at 0xd562c | `ret(0xc8348) == 0x0` | `ret(0xc8348) != 0x0` |
| `bne` at 0xd5648 | `[[sp-32]+1]:1 != 0x0` | `[[sp-32]+1]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5698 | `r0` | `?` |

## `i2cMuxResetRaw`

- `i2cMuxResetRaw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd57b8 to 0xd5a9c.
  Established. The entry branch takes its longer arm when `[0x2adf94] != 0x0` (`bne` at 0xd57e8).
  One-byte zeros are stored at absolute `0x2adf8c` and `0x2adf8d` (`strb` at 0xd58a4, `strb` at
  0xd59fc). Four other one-byte stores at `0x143818`, `0x143819`, `0x14381a`, `0x14381b` retain
  unknown values (`strb` at 0xd58b0, `strb` at 0xd58bc, `strb` at 0xd5a08, `strb` at 0xd5a14). The
  late branch takes the skip arm when `[0x143818]:1 == 0xfe` (`beq` at 0xd5a24). The subsequent
  unnamed call to `0xd5ed8` has candidates `[0x2adf8c]:1`, `[0x143818]:1`, `[0x2adf8c]:1`,
  `[0x143818]:1` (`bl` at 0xd5a40). Reset sequencing and byte-field roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd57c8 | load | 4 | `abs 0xd5a7c` | `-` |
| `ldr` at 0xd57cc | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd57d0 | store | 4 | `sp -0x14` | `[0x2adf90]` |
| `str` at 0xd57d8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd57dc | load | 4 | `abs 0xd5a80` | `-` |
| `ldr` at 0xd57e0 | load | 4 | `abs 0x2adf94` | `-` |
| `str` at 0xd57f0 | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0xd57fc | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd5800 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5804 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd5808 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd5818 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd581c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5820 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd5828 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd5834 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd5838 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5840 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd5848 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd5854 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd5868 | store | 4 | `sp -0x1c` | `ret(0xd5ed8)` |
| `ldr` at 0xd586c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5888 | store | 4 | `sp -0x18` | `ret(0xd5ed8)` |
| `ldrb` at 0xd588c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd5894 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xd589c | load | 4 | `abs 0xd5a84` | `-` |
| `strb` at 0xd58a4 | store | 1 | `abs 0x2adf8c` | `0x0` |
| `ldr` at 0xd58a8 | load | 4 | `abs 0xd5a88` | `-` |
| `strb` at 0xd58b0 | store | 1 | `abs 0x143818` | `?` |
| `ldr` at 0xd58b4 | load | 4 | `abs 0xd5a8c` | `-` |
| `strb` at 0xd58bc | store | 1 | `abs 0x143819` | `?` |
| `strb` at 0xd58c4 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0xd58c8 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd58cc | load | 4 | `abs 0xd5a80` | `-` |
| `ldr` at 0xd58d0 | load | 4 | `abs 0x2adf94` | `-` |
| `strb` at 0xd58e4 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd58e8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd58ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd58f0 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xd5900 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd5904 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5908 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrb` at 0xd5910 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd591c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd5920 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5928 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xd5930 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd593c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd5940 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd5944 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd594c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xd5954 | load | 1 | `(unknown)` | `-` |
| `str` at 0xd5974 | store | 4 | `sp -0x1c` | `ret(0xc857c)` |
| `ldr` at 0xd5978 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5990 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5994 | store | 4 | `sp -0x18` | `ret(0xc857c)` |
| `ldrb` at 0xd5998 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0xd599c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd59b0 | store | 4 | `sp -0x1c` | `ret(0xd5ed8)` |
| `ldr` at 0xd59b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd59c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd59cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd59d0 | store | 4 | `sp -0x18` | `ret(0xd5ed8)` |
| `ldrb` at 0xd59d4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd59dc | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0xd59e4 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xd59ec | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd59f4 | load | 4 | `abs 0xd5a90` | `-` |
| `strb` at 0xd59fc | store | 1 | `abs 0x2adf8d` | `0x0` |
| `ldr` at 0xd5a00 | load | 4 | `abs 0xd5a94` | `-` |
| `strb` at 0xd5a08 | store | 1 | `abs 0x14381a` | `?` |
| `ldr` at 0xd5a0c | load | 4 | `abs 0xd5a98` | `-` |
| `strb` at 0xd5a14 | store | 1 | `abs 0x14381b` | `?` |
| `ldr` at 0xd5a18 | load | 4 | `abs 0xd5a88` | `-` |
| `ldrb` at 0xd5a1c | load | 1 | `abs 0x143818` | `-` |
| `ldr` at 0xd5a28 | load | 4 | `abs 0xd5a84` | `-` |
| `ldrb` at 0xd5a2c | load | 1 | `abs 0x2adf8c` | `-` |
| `ldr` at 0xd5a30 | load | 4 | `abs 0xd5a88` | `-` |
| `ldrb` at 0xd5a34 | load | 1 | `abs 0x143818` | `-` |
| `str` at 0xd5a48 | store | 4 | `sp -0x1c` | `ret(0xd5ed8)` |
| `ldr` at 0xd5a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5a58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5a64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5a68 | store | 4 | `sp -0x18` | `ret(0xd5ed8)` |
| `ldr` at 0xd5a6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5a70 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0xd5a74 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd5a78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5a78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5860 | `(unknown callee) 0xd5ed8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xd595c | `(unknown callee) 0xc807c` | `?` | `?` | `?` | `?` |
| `bl` at 0xd596c | `(unknown callee) 0xc857c` | `?` | `?` | `?` | `?` |
| `bl` at 0xd59a8 | `(unknown callee) 0xd5ed8` | `[sp-30]:1` | `[sp-29]:1` | `[sp-29]:1` | `[sp-30]:1` |
| `bl` at 0xd5a40 | `(unknown callee) 0xd5ed8` | `[0x2adf8c]:1` | `[0x143818]:1` | `[0x2adf8c]:1` | `[0x143818]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd57e8 | `[0x2adf94] != 0x0` | `[0x2adf94] == 0x0` |
| `blt` at 0xd5810 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd5830 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xd5850 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xd5874 | `ret(0xd5ed8) == 0x0` | `ret(0xd5ed8) != 0x0` |
| `bne` at 0xd5880 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0xd58d8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xd58f8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd5918 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xd5938 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xd5980 | `ret(0xc857c) == 0x0` | `ret(0xc857c) != 0x0` |
| `bne` at 0xd598c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xd59bc | `ret(0xd5ed8) == 0x0` | `ret(0xd5ed8) != 0x0` |
| `bne` at 0xd59c8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xd5a24 | `[0x143818]:1 == 0xfe` | `[0x143818]:1 != 0xfe` |
| `beq` at 0xd5a54 | `ret(0xd5ed8) == 0x0` | `ret(0xd5ed8) != 0x0` |
| `bne` at 0xd5a60 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5a78 | `r0` | `?` |

## `i2cMuxSelectRaw`

- `i2cMuxSelectRaw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5a9c to 0xd5cf0.
  Established. The branch takes its long arm when `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` (`bne` at
  0xd5b14). Another branch takes a later arm when `ret(0xd62e0) == 0x0` (`beq` at 0xd5bec). That arm
  writes one-byte `([sp-24]:1 & 0xff)` to absolute `0x2adf8c`, `([sp-25]:1 & 0xff)` to `0x143818`,
  and `([sp-26]:1 & 0xff)` to `0x143819` (`strb` at 0xd5c98, `strb` at 0xd5ca4, `strb` at 0xd5cb0).
  The earlier three byte stores write zero at `0x2adf8d` and unknown values at `0x14381a`,
  `0x14381b` (`strb` at 0xd5bb0, `strb` at 0xd5bbc, `strb` at 0xd5bc8). Selector and pointee
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5aac | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xd5ab4 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xd5abc | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xd5ac0 | load | 4 | `abs 0xd5cc4` | `-` |
| `ldr` at 0xd5ac4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5ad8 | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5ae0 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd5ae4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5ae8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5af8 | store | 4 | `sp -0x24` | `ret(0xd61d0)` |
| `ldr` at 0xd5afc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd5b00 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5b04 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd5b0c | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd5b18 | load | 4 | `abs 0xd5cc8` | `-` |
| `str` at 0xd5b1c | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xd5b24 | load | 4 | `abs 0xd5ccc` | `-` |
| `ldrb` at 0xd5b28 | load | 1 | `abs 0x2adf8c` | `-` |
| `ldr` at 0xd5b2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5b38 | load | 4 | `abs 0xd5cd0` | `-` |
| `ldrb` at 0xd5b3c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd5b40 | load | 1 | `abs 0x143818` | `-` |
| `ldr` at 0xd5b4c | load | 4 | `abs 0xd5cd4` | `-` |
| `ldrb` at 0xd5b50 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd5b54 | load | 1 | `abs 0x143819` | `-` |
| `str` at 0xd5b64 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd5b6c | load | 4 | `abs 0xd5cd0` | `-` |
| `ldrb` at 0xd5b70 | load | 1 | `abs 0x143818` | `-` |
| `ldr` at 0xd5b7c | load | 4 | `abs 0xd5cd4` | `-` |
| `ldrb` at 0xd5b80 | load | 1 | `abs 0x143819` | `-` |
| `ldr` at 0xd5b8c | load | 4 | `abs 0xd5ccc` | `-` |
| `ldrb` at 0xd5b90 | load | 1 | `abs 0x2adf8c` | `-` |
| `ldr` at 0xd5b94 | load | 4 | `abs 0xd5cd0` | `-` |
| `ldrb` at 0xd5b98 | load | 1 | `abs 0x143818` | `-` |
| `ldr` at 0xd5ba8 | load | 4 | `abs 0xd5cd8` | `-` |
| `strb` at 0xd5bb0 | store | 1 | `abs 0x2adf8d` | `0x0` |
| `ldr` at 0xd5bb4 | load | 4 | `abs 0xd5cdc` | `-` |
| `strb` at 0xd5bbc | store | 1 | `abs 0x14381a` | `?` |
| `ldr` at 0xd5bc0 | load | 4 | `abs 0xd5ce0` | `-` |
| `strb` at 0xd5bc8 | store | 1 | `abs 0x14381b` | `?` |
| `ldrb` at 0xd5bcc | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd5bd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5be0 | store | 4 | `sp -0x28` | `ret(0xd62e0)` |
| `ldr` at 0xd5be4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5bf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5bf8 | load | 4 | `abs 0xd5ce4` | `-` |
| `ldr` at 0xd5bfc | load | 4 | `abs 0xd5ce8` | `-` |
| `str` at 0xd5c28 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd5c2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5c34 | load | 4 | `abs 0xd5ce4` | `-` |
| `ldr` at 0xd5c38 | load | 4 | `abs 0xd5ce8` | `-` |
| `ldr` at 0xd5c44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5c54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5c60 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd5c64 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd5c68 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd5c6c | store | 4 | `(unknown)` | `ret(0x21704)` |
| `ldr` at 0xd5c70 | load | 4 | `abs 0xd5cec` | `-` |
| `ldr` at 0xd5c84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd5c88 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd5c90 | load | 4 | `abs 0xd5ccc` | `-` |
| `ldrb` at 0xd5c94 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd5c98 | store | 1 | `abs 0x2adf8c` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xd5c9c | load | 4 | `abs 0xd5cd0` | `-` |
| `ldrb` at 0xd5ca0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd5ca4 | store | 1 | `abs 0x143818` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xd5ca8 | load | 4 | `abs 0xd5cd4` | `-` |
| `ldrb` at 0xd5cac | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd5cb0 | store | 1 | `abs 0x143819` | `([sp-26]:1 & 0xff)` |
| `str` at 0xd5cb8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd5cbc | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd5cc0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5cc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5cc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5cc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5af0 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5ba4 | `(unknown callee) 0xd5ed8` | `[0x2adf8c]:1` | `[0x143818]:1` | `[0x2adf8c]:1` | `[0x143818]:1` |
| `bl` at 0xd5bd8 | `(unknown callee) 0xd62e0` | `[sp-36]` | `[sp-26]:1` | `0x14381b` | `[sp-26]:1` |
| `bl` at 0xd5c00 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5c0c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5c3c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5c4c | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5c58 | `(unknown callee) 0x21704` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd5c80 | `(unknown callee) 0x2079c` | `0x13516c` | `ret(0x1b598)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5b14 | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xd5b34 | `[sp-24] != [0x2adf8c]:1` | `[sp-24] == [0x2adf8c]:1` |
| `bne` at 0xd5b48 | `[sp-25]:1 != [0x143818]:1` | `[sp-25]:1 == [0x143818]:1` |
| `bne` at 0xd5b5c | `[sp-26]:1 != [0x143819]:1` | `[sp-26]:1 == [0x143819]:1` |
| `beq` at 0xd5b78 | `[0x143818]:1 == 0xfe` | `[0x143818]:1 != 0xfe` |
| `beq` at 0xd5b88 | `[0x143819]:1 == 0x0` | `[0x143819]:1 != 0x0` |
| `beq` at 0xd5bec | `ret(0xd62e0) == 0x0` | `ret(0xd62e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5cc0 | `r0` | `?` |

## `i2cMuxSelectSubRaw`

- `i2cMuxSelectSubRaw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5cf0 to 0xd5ed8.
  Established. The first unnamed call to `0xd61d0` has candidates `r0`, `(r1 & 0xff)`, unknown, `(r1
  & 0xff)` (`bl` at 0xd5d44). A later unnamed call to `0xd62e0` carries `ret(0xd61d0)`, `[sp-26]:1`,
  unknown, `[sp-26]:1` (`bl` at 0xd5dcc); its zero result takes the branch to byte assignments
  (`beq` at 0xd5de0). These store `([sp-24]:1 & 0xff)` at absolute `0x2adf8d`, `([sp-25]:1 & 0xff)`
  at `0x14381a`, and `([sp-26]:1 & 0xff)` at `0x14381b` (`strb` at 0xd5e8c, `strb` at 0xd5e98,
  `strb` at 0xd5ea4). The subchannel-selection role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5d00 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xd5d08 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xd5d10 | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xd5d14 | load | 4 | `abs 0xd5eb8` | `-` |
| `ldr` at 0xd5d18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5d2c | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5d34 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd5d38 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5d3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5d4c | store | 4 | `sp -0x24` | `ret(0xd61d0)` |
| `ldr` at 0xd5d50 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd5d54 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5d58 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd5d60 | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd5d6c | load | 4 | `abs 0xd5ebc` | `-` |
| `str` at 0xd5d70 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xd5d78 | load | 4 | `abs 0xd5ec0` | `-` |
| `ldrb` at 0xd5d7c | load | 1 | `abs 0x2adf8d` | `-` |
| `ldr` at 0xd5d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5d8c | load | 4 | `abs 0xd5ec4` | `-` |
| `ldrb` at 0xd5d90 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd5d94 | load | 1 | `abs 0x14381a` | `-` |
| `ldr` at 0xd5da0 | load | 4 | `abs 0xd5ec8` | `-` |
| `ldrb` at 0xd5da4 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd5da8 | load | 1 | `abs 0x14381b` | `-` |
| `str` at 0xd5db8 | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0xd5dc0 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd5dc4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5dd4 | store | 4 | `sp -0x28` | `ret(0xd62e0)` |
| `ldr` at 0xd5dd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5dec | load | 4 | `abs 0xd5ecc` | `-` |
| `ldr` at 0xd5df0 | load | 4 | `abs 0xd5ed0` | `-` |
| `str` at 0xd5e1c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd5e20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5e28 | load | 4 | `abs 0xd5ecc` | `-` |
| `ldr` at 0xd5e2c | load | 4 | `abs 0xd5ed0` | `-` |
| `ldr` at 0xd5e38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5e48 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5e54 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd5e58 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd5e5c | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd5e60 | store | 4 | `(unknown)` | `ret(0x21704)` |
| `ldr` at 0xd5e64 | load | 4 | `abs 0xd5ed4` | `-` |
| `ldr` at 0xd5e78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd5e7c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd5e84 | load | 4 | `abs 0xd5ec0` | `-` |
| `ldrb` at 0xd5e88 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd5e8c | store | 1 | `abs 0x2adf8d` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xd5e90 | load | 4 | `abs 0xd5ec4` | `-` |
| `ldrb` at 0xd5e94 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd5e98 | store | 1 | `abs 0x14381a` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xd5e9c | load | 4 | `abs 0xd5ec8` | `-` |
| `ldrb` at 0xd5ea0 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd5ea4 | store | 1 | `abs 0x14381b` | `([sp-26]:1 & 0xff)` |
| `str` at 0xd5eac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd5eb0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd5eb4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5eb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5eb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5eb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5d44 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5dcc | `(unknown callee) 0xd62e0` | `ret(0xd61d0)` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xd5df4 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5e00 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5e30 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5e40 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5e4c | `(unknown callee) 0x21704` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd5e74 | `(unknown callee) 0x2079c` | `0x1351a0` | `ret(0x1b598)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5d68 | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xd5d88 | `[sp-24] != [0x2adf8d]:1` | `[sp-24] == [0x2adf8d]:1` |
| `bne` at 0xd5d9c | `[sp-25]:1 != [0x14381a]:1` | `[sp-25]:1 == [0x14381a]:1` |
| `bne` at 0xd5db0 | `[sp-26]:1 != [0x14381b]:1` | `[sp-26]:1 == [0x14381b]:1` |
| `beq` at 0xd5de0 | `ret(0xd62e0) == 0x0` | `ret(0xd62e0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5eb4 | `r0` | `?` |

## `i2cMuxDeselectRaw`

- `i2cMuxDeselectRaw`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5ed8 to 0xd60f4.
  Established. The unnamed call to `0xd62e0` has candidates `ret(0xd61d0)`, `0x0`, `[sp-29]:1`,
  `[([sp-29]:1 + [[sp-36]+8])]:1` (`bl` at 0xd5f60); its zero result takes the branch to the later
  byte-test arm (`beq` at 0xd5f74). One-byte stores include zero at absolute `0x2adf8c`, zero at
  `0x2adf8d`, and unknown values at `0x143818`, `0x143819`, `0x14381a`, `0x14381b` (`strb` at
  0xd6070, `strb` at 0xd6094, `strb` at 0xd607c, `strb` at 0xd6088, `strb` at 0xd60a0, `strb` at
  0xd60ac). The first pair of unknown-byte stores requires the fall-through when `[[sp-40]+1]:1 ==
  0x0` (`bne` at 0xd603c). Deselection and reset meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5ee8 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd5ef0 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd5ef4 | load | 4 | `abs 0xd60c0` | `-` |
| `ldr` at 0xd5ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5f0c | load | 4 | `abs 0x2adf90` | `-` |
| `str` at 0xd5f14 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xd5f18 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5f1c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5f2c | store | 4 | `sp -0x28` | `ret(0xd61d0)` |
| `ldr` at 0xd5f30 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd5f34 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5f38 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0xd5f40 | load | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0xd5f4c | load | 4 | `abs 0xd60c4` | `-` |
| `str` at 0xd5f50 | store | 4 | `sp -0x34` | `0xfffffc11` |
| `ldr` at 0xd5f58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd5f68 | store | 4 | `sp -0x2c` | `ret(0xd62e0)` |
| `ldr` at 0xd5f6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5f80 | load | 4 | `abs 0xd60c8` | `-` |
| `ldr` at 0xd5f84 | load | 4 | `abs 0xd60cc` | `-` |
| `str` at 0xd5fb0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd5fb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5fbc | load | 4 | `abs 0xd60c8` | `-` |
| `ldr` at 0xd5fc0 | load | 4 | `abs 0xd60cc` | `-` |
| `ldr` at 0xd5fcc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xd5fdc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5ff0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5ffc | load | 4 | `abs 0xd60d0` | `-` |
| `str` at 0xd6000 | store | 4 | `(unknown)` | `0x1351d8` |
| `str` at 0xd6004 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `str` at 0xd6008 | store | 4 | `(unknown)` | `ret(0xd5750)` |
| `str` at 0xd600c | store | 4 | `(unknown)` | `ret(0x21704)` |
| `ldr` at 0xd6014 | load | 4 | `abs 0xd60d4` | `-` |
| `ldr` at 0xd6018 | load | 4 | `abs 0xd60d8` | `-` |
| `ldr` at 0xd6024 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd6028 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd6030 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xd6034 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0xd6040 | load | 4 | `abs 0xd60dc` | `-` |
| `ldrb` at 0xd6044 | load | 1 | `abs 0x2adf8c` | `-` |
| `ldr` at 0xd6048 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6054 | load | 4 | `abs 0xd60e0` | `-` |
| `ldrb` at 0xd6058 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd605c | load | 1 | `abs 0x143818` | `-` |
| `ldr` at 0xd6068 | load | 4 | `abs 0xd60dc` | `-` |
| `strb` at 0xd6070 | store | 1 | `abs 0x2adf8c` | `0x0` |
| `ldr` at 0xd6074 | load | 4 | `abs 0xd60e0` | `-` |
| `strb` at 0xd607c | store | 1 | `abs 0x143818` | `?` |
| `ldr` at 0xd6080 | load | 4 | `abs 0xd60e4` | `-` |
| `strb` at 0xd6088 | store | 1 | `abs 0x143819` | `?` |
| `ldr` at 0xd608c | load | 4 | `abs 0xd60e8` | `-` |
| `strb` at 0xd6094 | store | 1 | `abs 0x2adf8d` | `0x0` |
| `ldr` at 0xd6098 | load | 4 | `abs 0xd60ec` | `-` |
| `strb` at 0xd60a0 | store | 1 | `abs 0x14381a` | `?` |
| `ldr` at 0xd60a4 | load | 4 | `abs 0xd60f0` | `-` |
| `strb` at 0xd60ac | store | 1 | `abs 0x14381b` | `?` |
| `str` at 0xd60b4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd60b8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd60bc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd60bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd60bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd60bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd60bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5f24 | `(unknown callee) 0xd61d0` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5f60 | `(unknown callee) 0xd62e0` | `ret(0xd61d0)` | `0x0` | `[sp-29]:1` | `[([sp-29]:1 + [[sp-36]+8])]:1` |
| `bl` at 0xd5f88 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5f94 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5fc4 | `(unknown callee) 0x4f9a4` | `[sp-28]` | `0x0` | `0x1350a4` | `0x1350ac` |
| `bl` at 0xd5fd4 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd5fe8 | `(unknown callee) 0xd5750` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd5ff4 | `(unknown callee) 0x21704` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xd6020 | `(unknown callee) 0x21eb8` | `0x1` | `0x13504c` | `0x1f7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5f48 | `[([sp-29]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-29]:1 + [[sp-36]+8])]:1 == 0x0` |
| `beq` at 0xd5f74 | `ret(0xd62e0) == 0x0` | `ret(0xd62e0) != 0x0` |
| `bne` at 0xd603c | `[[sp-40]+1]:1 != 0x0` | `[[sp-40]+1]:1 == 0x0` |
| `bne` at 0xd6050 | `[sp-28] != [0x2adf8c]:1` | `[sp-28] == [0x2adf8c]:1` |
| `bne` at 0xd6064 | `[sp-29]:1 != [0x143818]:1` | `[sp-29]:1 == [0x143818]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd60bc | `r0` | `?` |

## `uartWrite`

- `uartWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcad54 to 0xcade4.
  Established. The entry branch takes its call arm when `r2 > 0x0 (signed, 32-bit)` (`bgt` at
  0xcad78). The unnamed target `0x1b058` receives candidates `[r0+4]`, `r1`, `r2`, `r0` (`bl` at
  0xcad98). A nonnegative result takes the branch to its local-result arm (`bge` at 0xcadac). The
  other arm includes an unnamed call to `0xcaeb0` with `[sp-20]`, `0x2`, unknown, `ret(0x1b058)`
  (`bl` at 0xcadb8). The merged return is unknown (`ldmdb` at 0xcade0). Byte extent and UART
  transport effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcad64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcad68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xcad6c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xcad70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcad80 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xcad88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcad8c | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xcad90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcad94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcada0 | store | 4 | `sp -0x20` | `ret(0x1b058)` |
| `ldr` at 0xcada4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcadb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcadc4 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `str` at 0xcadcc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xcadd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcadd8 | store | 4 | `sp -0x24` | `ret(0x1b058)` |
| `ldr` at 0xcaddc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xcade0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcade0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcade0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcad98 | `(unknown callee) 0x1b058` | `[r0+4]` | `r1` | `r2` | `r0` |
| `bl` at 0xcadb8 | `(unknown callee) 0xcaeb0` | `[sp-20]` | `0x2` | `?` | `ret(0x1b058)` |
| `bl` at 0xcadbc | `(unknown callee) 0x1b148` | `ret(0xcaeb0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xcad78 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bge` at 0xcadac | `ret(0x1b058) >= 0x0 (signed, 32-bit)` | `ret(0x1b058) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcade0 | `r0` | `?` |

## `uartSetConfig`

- `uartSetConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcae20 to 0xcaeb0.
  Established. The first byte comparison falls through when `[r1+22]:1 == 0x1` (`bne` at 0xcae44),
  reaching a byte store of `0x2` at `r1 +0x16` (`strb` at 0xcae50). Two later byte stores also write
  `0x2` but have unresolved targets (`strb` at 0xcae6c, `strb` at 0xcae88). Seven four-byte stores
  at the two multiple-store sites retain unknown targets and values (`stm` at 0xcaea0, `stm` at
  0xcaea8). Configuration fields and transfer extent are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcae30 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcae34 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcae38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcae3c | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0xcae48 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcae50 | store | 1 | `r1 +0x16` | `0x2` |
| `ldr` at 0xcae54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcae58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcae64 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcae6c | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xcae70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcae74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcae80 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xcae88 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xcae8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcae90 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xcae9c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcae9c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcae9c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcae9c | load | 4 | `(unknown)` | `-` |
| `stm` at 0xcaea0 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xcaea0 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xcaea0 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xcaea0 | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xcaea4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcaea4 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xcaea4 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xcaea8 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xcaea8 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xcaea8 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xcaeac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcaeac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcaeac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcae44 | `[r1+22]:1 != 0x1` | `[r1+22]:1 == 0x1` |
| `bne` at 0xcae60 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xcae7c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcaeac | `r0` | `?` |

## `uartGetConfig`

- `uartGetConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcade4 to 0xcae20.
  Established. The first multiple load reads four-byte values at `r0 +0x10`, `+0x14`, `+0x18`,
  `+0x1c` (`ldm` at 0xcae0c). The next multiple load reads `r0 +0x20`, `+0x24`, `+0x28` (`ldm` at
  0xcae14). Four-byte stores place the corresponding expressions at `r1 +0x0`, `+0x4`, `+0x8`,
  `+0xc` (`stm` at 0xcae10), then `+0x10`, `+0x14`, `+0x18` (`stm` at 0xcae18). The return expression is `[r0+32]` (`ldmdb`
  at 0xcae1c). The copied fields have Unknown roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcadf4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcadf8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcadfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcae00 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xcae0c | load | 4 | `r0 +0x10` | `-` |
| `ldm` at 0xcae0c | load | 4 | `r0 +0x14` | `-` |
| `ldm` at 0xcae0c | load | 4 | `r0 +0x18` | `-` |
| `ldm` at 0xcae0c | load | 4 | `r0 +0x1c` | `-` |
| `stm` at 0xcae10 | store | 4 | `r1 +0x0` | `[r0+16]` |
| `stm` at 0xcae10 | store | 4 | `r1 +0x4` | `[r0+20]` |
| `stm` at 0xcae10 | store | 4 | `r1 +0x8` | `[r0+24]` |
| `stm` at 0xcae10 | store | 4 | `r1 +0xc` | `[r0+28]` |
| `ldm` at 0xcae14 | load | 4 | `r0 +0x20` | `-` |
| `ldm` at 0xcae14 | load | 4 | `r0 +0x24` | `-` |
| `ldm` at 0xcae14 | load | 4 | `r0 +0x28` | `-` |
| `stm` at 0xcae18 | store | 4 | `r1 +0x10` | `[r0+32]` |
| `stm` at 0xcae18 | store | 4 | `r1 +0x14` | `[r0+36]` |
| `stm` at 0xcae18 | store | 4 | `r1 +0x18` | `[r0+40]` |
| `ldmdb` at 0xcae1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcae1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcae1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcae1c | `r0` | `[r0+32]` |

## `uartFlush`

- `uartFlush`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcaeb0 to 0xcaf58.
  Established. The first branch takes its alternate arm when `? == 0x0` (`beq` at 0xcaed4). The two
  arms store `[r0+4]` or `[r0]` at `sp -0x20` (`str` at 0xcaee0, `str` at 0xcaef0). The unnamed call
  to `0x1b49c` then has candidates `[sp-32]`, `0x80047502`, `r1`, `[sp-32]` (`bl` at 0xcaf08); its
  nonnegative result takes the exit branch (`bge` at 0xcaf14). Flush semantics and command
  identities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcaec0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcaec4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcaec8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcaedc | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xcaee0 | store | 4 | `sp -0x20` | `[r0+4]` |
| `ldr` at 0xcaee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcaeec | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xcaef0 | store | 4 | `sp -0x20` | `[r0]` |
| `ldr` at 0xcaef4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcaef8 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xcaefc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcaf00 | load | 4 | `abs 0xcaf4c` | `-` |
| `ldr` at 0xcaf04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcaf20 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcaf2c | load | 4 | `abs 0xcaf50` | `-` |
| `ldr` at 0xcaf30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcaf34 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xcaf38 | load | 4 | `abs 0xcaf54` | `-` |
| `ldr` at 0xcaf3c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xcaf48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcaf48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcaf48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcaf08 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x80047502` | `r1` | `[sp-32]` |
| `bl` at 0xcaf18 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xcaf24 | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcaf44 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x134244` | `[[sp-20]+8]` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcaed4 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xcaf14 | `ret(0x1b49c) >= 0x0 (signed, 32-bit)` | `ret(0x1b49c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcaf48 | `r0` | `?` |

## `uartSendBreak`

- `uartSendBreak`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcaf58 to 0xcafd0.
  Established. A four-byte load reads `r0 +0x4` (`ldr` at 0xcaf74). The unnamed call to `0x1b49c`
  has candidates `[r0+4]`, `0x80047504`, `0x0`, `r0` (`bl` at 0xcaf80); its result at least zero
  takes the exit arm (`bge` at 0xcaf8c). The other arm includes an unnamed call to `0x240b8` with
  `[0x145ae8]`, `0x13426c`, `[[sp-20]+8]`, `ret(0x21704)` (`bl` at 0xcafbc). A break signal and
  duration are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcaf68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xcaf6c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xcaf70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcaf74 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xcaf78 | load | 4 | `abs 0xcafc4` | `-` |
| `ldr` at 0xcaf98 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xcafa4 | load | 4 | `abs 0xcafc8` | `-` |
| `ldr` at 0xcafa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcafac | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xcafb0 | load | 4 | `abs 0xcafcc` | `-` |
| `ldr` at 0xcafb4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xcafc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcafc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcafc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcaf80 | `(unknown callee) 0x1b49c` | `[r0+4]` | `0x80047504` | `0x0` | `r0` |
| `bl` at 0xcaf90 | `(unknown callee) 0x1b148` | `ret(0x1b49c)` | `?` | `?` | `ret(0x1b49c)` |
| `bl` at 0xcaf9c | `(unknown callee) 0x21704` | `[ret(0x1b148)]` | `?` | `?` | `ret(0x1b148)` |
| `bl` at 0xcafbc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x13426c` | `[[sp-20]+8]` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xcaf8c | `ret(0x1b49c) >= 0x0 (signed, 32-bit)` | `ret(0x1b49c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcafc0 | `r0` | `?` |

## `uartShowStatus`

- `uartShowStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcafd0 to 0xcb0cc.
  Established. The first unnamed call to `0xcb0cc` has candidates `r0`, `r1`, `r2`, `r3` (`bl` at
  0xcaff0); its nonzero result takes the longer arm (`bne` at 0xcb004). That arm reads four bytes at
  `[sp-44] +0x98` and `+0x100`, plus two bytes at `+0x9c`, `+0x9e`, `+0xa0`, `+0x104` (`ldr` at
  0xcb02c, `ldr` at 0xcb050, `ldrh` at 0xcb06c, `ldrh` at 0xcb078, `ldrh` at 0xcb084, `ldrh` at
  0xcb09c); its final unnamed call has candidates `[sp-40]`, `0x134298`, `[sp-36]`, `[[sp-44]+44]`
  (`bl` at 0xcb0b4). Status-field meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcafe0 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xcafe4 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xcafe8 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xcafec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xcaff8 | store | 4 | `sp -0x2c` | `ret(0xcb0cc)` |
| `ldr` at 0xcaffc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcb00c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcb014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb028 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb02c | load | 4 | `[sp-44] +0x98` | `-` |
| `ldr` at 0xcb038 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb04c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcb050 | load | 4 | `[sp-44] +0x100` | `-` |
| `ldr` at 0xcb05c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xcb060 | store | 4 | `sp -0x50` | `ret(0xcb180)` |
| `str` at 0xcb064 | store | 4 | `sp -0x4c` | `ret(0xcb1a4)` |
| `ldr` at 0xcb068 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb06c | load | 2 | `[sp-44] +0x9c` | `-` |
| `str` at 0xcb070 | store | 4 | `sp -0x48` | `[[sp-44]+156]:2` |
| `ldr` at 0xcb074 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb078 | load | 2 | `[sp-44] +0x9e` | `-` |
| `str` at 0xcb07c | store | 4 | `sp -0x44` | `[[sp-44]+158]:2` |
| `ldr` at 0xcb080 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb084 | load | 2 | `[sp-44] +0xa0` | `-` |
| `str` at 0xcb088 | store | 4 | `sp -0x40` | `[[sp-44]+160]:2` |
| `str` at 0xcb08c | store | 4 | `sp -0x3c` | `ret(0xcb180)` |
| `str` at 0xcb090 | store | 4 | `sp -0x38` | `ret(0xcb1a4)` |
| `ldr` at 0xcb094 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xcb09c | load | 2 | `[sp-44] +0x104` | `-` |
| `str` at 0xcb0a0 | store | 4 | `sp -0x34` | `[[sp-44]+260]:2` |
| `ldr` at 0xcb0a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xcb0a8 | load | 4 | `abs 0xcb0c8` | `-` |
| `ldr` at 0xcb0ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcb0b0 | load | 4 | `[sp-44] +0x2c` | `-` |
| `str` at 0xcb0bc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xcb0c0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcb0c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcaff0 | `(unknown callee) 0xcb0cc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xcb020 | `(unknown callee) 0xcb180` | `ret(0xcb0cc)+60` | `?` | `?` | `ret(0xcb0cc)+60` |
| `bl` at 0xcb030 | `(unknown callee) 0xcb1a4` | `[[sp-44]+152]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xcb044 | `(unknown callee) 0xcb180` | `[sp-44]+164` | `?` | `?` | `[sp-44]+164` |
| `bl` at 0xcb054 | `(unknown callee) 0xcb1a4` | `[[sp-44]+256]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xcb0b4 | `(unknown callee) 0x1b52c` | `[sp-40]` | `0x134298` | `[sp-36]` | `[[sp-44]+44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcb004 | `ret(0xcb0cc) != 0x0` | `ret(0xcb0cc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcb0c4 | `r0` | `[sp-48]` |

## `nvramRead`

- `nvramRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20bc8 to 0x20c38.
  Established. The signed branch takes its later arm when `(r0 + r2) <= 0x1fff` (`ble` at 0x20bf8).
  The other arm writes an unknown four-byte value at `sp -0x1c` (`str` at 0x20c04). The visible
  unnamed call to `0x1b394` has all four candidate slots unknown (`bl` at 0x20c20). The return
  expression is `[sp-28]` (`ldmdb` at 0x20c2c). NVRAM address and read extent are not established by
  the tracked call arguments.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20bd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20bdc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x20be0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x20be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20be8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20bf0 | load | 4 | `abs 0x20c30` | `-` |
| `ldr` at 0x20bfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20c04 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x20c08 | load | 4 | `abs 0x20c34` | `-` |
| `ldr` at 0x20c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20c24 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x20c2c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20c2c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20c2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20c20 | `(unknown callee) 0x1b394` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x20bf8 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20c2c | `r0` | `[sp-28]` |

## `nvramWrite`

- `nvramWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20c38 to 0x20ca8.
  Established. The initial three four-byte stores preserve `r0`, `r1`, `r2` at `sp -0x14`, `-0x18`,
  `-0x1c` (`str` at 0x20c48, `str` at 0x20c4c, `str` at 0x20c50); its branch takes the later arm
  when `(r0 + r2) <= 0x1fff (signed, 32-bit)` (`ble` at 0x20c68). The unnamed call to `0x1b394` has
  four unknown candidate slots (`bl` at 0x20c90), and `[sp-28]` reaches the return (`ldmdb` at
  0x20c9c). A persistent-store effect is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20c48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20c4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x20c50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x20c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20c58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20c60 | load | 4 | `abs 0x20ca0` | `-` |
| `ldr` at 0x20c6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20c74 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x20c78 | load | 4 | `abs 0x20ca4` | `-` |
| `ldr` at 0x20c7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20c88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20c8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20c94 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x20c9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20c9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20c9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20c90 | `(unknown callee) 0x1b394` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x20c68 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20c9c | `r0` | `[sp-28]` |
