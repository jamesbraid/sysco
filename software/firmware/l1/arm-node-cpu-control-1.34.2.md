# ARM node, CPU and interrupt paths in 1.34.2

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/node-cpu-control-facts.txt) retain all tool sections.

## `IP57_Cfg_Init`

- `IP57_Cfg_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c4c8 to 0x4c6cc.
  Established. Absolute stores write byte `0x1` at `0x119eb0`, word `[sp-28]` at `0x119eb4`, and
  word `[sp-24]` at `0x119eb8` (`strb` at 0x4c51c, `str` at 0x4c528, `str` at 0x4c534). The result
  of the call to `0x18920` is stored as a word at `0x281200` (`bl` at 0x4c548, `str` at 0x4c550).
  Later writes include word `0x200` at `[sp-48] +0x4` and byte `0x0` at `[sp-48] +0x8` (`str` at
  0x4c618, `strb` at 0x4c624). The final call to `0x4c6cc` carries `[sp]`, `[sp+4]`, unknown,
  unknown, and supplies the reported r0 return (`bl` at 0x4c6a0, `ldmdb` at 0x4c6a4).
  Configuration-field meanings and physical initialization are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c4d8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x4c4dc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x4c4e0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x4c4e4 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x4c4e8 | load | 4 | `abs 0x4c6a8` | `-` |
| `ldrb` at 0x4c4ec | load | 1 | `abs 0x119eb0` | `-` |
| `ldr` at 0x4c4f8 | load | 4 | `abs 0x4c6ac` | `-` |
| `str` at 0x4c4fc | store | 4 | `sp -0x44` | `0xe3914` |
| `ldr` at 0x4c504 | load | 4 | `abs 0x4c6b0` | `-` |
| `ldr` at 0x4c514 | load | 4 | `abs 0x4c6a8` | `-` |
| `strb` at 0x4c51c | store | 1 | `abs 0x119eb0` | `0x1` |
| `ldr` at 0x4c520 | load | 4 | `abs 0x4c6b4` | `-` |
| `ldr` at 0x4c524 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4c528 | store | 4 | `abs 0x119eb4` | `[sp-28]` |
| `ldr` at 0x4c52c | load | 4 | `abs 0x4c6b8` | `-` |
| `ldr` at 0x4c530 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c534 | store | 4 | `abs 0x119eb8` | `[sp-24]` |
| `ldr` at 0x4c538 | load | 4 | `abs 0x4c6bc` | `-` |
| `ldr` at 0x4c53c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4c550 | store | 4 | `abs 0x281200` | `ret(0x18920)` |
| `ldr` at 0x4c554 | load | 4 | `abs 0x281200` | `-` |
| `ldr` at 0x4c560 | load | 4 | `abs 0x4c6c0` | `-` |
| `str` at 0x4c564 | store | 4 | `sp -0x44` | `0xe3944` |
| `ldr` at 0x4c56c | load | 4 | `abs 0x4c6b0` | `-` |
| `str` at 0x4c580 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c584 | load | 4 | `abs 0x4c6b4` | `-` |
| `ldr` at 0x4c588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4c58c | load | 4 | `abs 0x119eb4` | `-` |
| `ldr` at 0x4c59c | load | 4 | `abs 0x4c6b8` | `-` |
| `ldr` at 0x4c5a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4c5b4 | load | 4 | `abs 0x119eb8` | `-` |
| `str` at 0x4c5bc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4c5c0 | load | 4 | `abs 0x4c6bc` | `-` |
| `ldr` at 0x4c5c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4c5cc | load | 4 | `abs 0x281200` | `-` |
| `str` at 0x4c5d4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x4c5d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4c5dc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4c5e0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x4c5e8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x4c5ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4c5f0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4c5f4 | load | 1 | `[sp-44] +0x3` | `-` |
| `strb` at 0x4c5fc | store | 1 | `[sp-48] +0x1` | `([[sp-44]+3]:1+5 & 0xff)` |
| `ldr` at 0x4c600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4c604 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x4c608 | load | 1 | `[sp-44] +0xc` | `-` |
| `strb` at 0x4c60c | store | 1 | `[sp-48] +0x2` | `([[sp-44]+12]:1 & 0xff)` |
| `ldr` at 0x4c610 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x4c618 | store | 4 | `[sp-48] +0x4` | `0x200` |
| `ldr` at 0x4c61c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x4c624 | store | 1 | `[sp-48] +0x8` | `0x0` |
| `ldr` at 0x4c628 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x4c62c | load | 4 | `abs 0x4c6c4` | `-` |
| `ldr` at 0x4c630 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4c634 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c638 | store | 4 | `[sp-48] +0xc` | `?` |
| `ldr` at 0x4c63c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4c644 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x4c64c | load | 4 | `abs 0x4c6c8` | `-` |
| `ldr` at 0x4c654 | load | 4 | `abs 0x4c6bc` | `-` |
| `ldr` at 0x4c658 | load | 4 | `abs 0x4c6b4` | `-` |
| `str` at 0x4c660 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x4c664 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4c668 | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0x4c66c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4c670 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x4c674 | load | 4 | `abs 0x4c6c8` | `-` |
| `str` at 0x4c678 | store | 4 | `sp -0x38` | `0x119ec4` |
| `str` at 0x4c680 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x4c684 | load | 4 | `abs 0x281200` | `-` |
| `ldr` at 0x4c688 | load | 4 | `abs 0x119eb4` | `-` |
| `ldr` at 0x4c698 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x4c69c | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0x4c6a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4c6a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c6a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c6a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4c510 | `(unknown callee) 0x1eed0` | `0x9` | `0xe3930` | `0x8d` | `0x0` |
| `bl` at 0x4c548 | `(unknown callee) 0x18920` | `?` | `?` | `0x119eb8` | `?` |
| `bl` at 0x4c578 | `(unknown callee) 0x1eed0` | `0x9` | `0xe3930` | `0x98` | `0x0` |
| `bl` at 0x4c650 | `(unknown callee) 0x4c78c` | `0x119ec4` | `?` | `?` | `[0x119eb4]` |
| `bl` at 0x4c694 | `(unknown callee) 0x24250` | `[0x281200]` | `[0x119eb4]` | `0xc` | `0x0` |
| `bl` at 0x4c6a0 | `(unknown callee) 0x4c6cc` | `[sp]` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4c4f4 | `[0x119eb0]:1 == 0x0` | `[0x119eb0]:1 != 0x0` |
| `bne` at 0x4c55c | `[0x281200] != 0x0` | `[0x281200] == 0x0` |
| `blt` at 0x4c594 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c6a4 | `r0` | `ret(0x4c6cc)` |

## `IP57_Set_MemCfg`

- `IP57_Set_MemCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c6cc to 0x4c78c.
  Established. The first branch takes its edge for entry `r0 == 0x0` and falls through on inequality
  (`beq` at 0x4c6ec). The next takes for entry `r1 != 0x0` and falls through on equality (`bne` at
  0x4c6f8). Another equality edge selects entry `r0 == 0x64`, after which stack words receive
  `0xe3814` at `sp -0x14` and `0x6` at `sp -0x18` (`beq` at 0x4c704, `str` at 0x4c710, `str` at
  0x4c718). The final call to `0x2470c` has four unknown candidates and supplies the r0 return (`bl`
  at 0x4c774, `ldmdb` at 0x4c778). The configuration format and hardware effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c6dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c6e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4c6e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c6f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c6fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c70c | load | 4 | `abs 0x4c77c` | `-` |
| `str` at 0x4c710 | store | 4 | `sp -0x14` | `0xe3814` |
| `str` at 0x4c718 | store | 4 | `sp -0x18` | `0x6` |
| `ldr` at 0x4c720 | load | 4 | `abs 0x4c780` | `-` |
| `str` at 0x4c724 | store | 4 | `sp -0x20` | `0xe3980` |
| `ldr` at 0x4c728 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c72c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x4c734 | load | 4 | `abs 0x4c784` | `-` |
| `ldr` at 0x4c744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c75c | load | 4 | `abs 0x4c788` | `-` |
| `str` at 0x4c760 | store | 4 | `sp -0x14` | `0xe3874` |
| `str` at 0x4c768 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x4c76c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c770 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4c778 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c778 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4c740 | `(unknown callee) 0x1eed0` | `0x9` | `0xe3930` | `0xc8` | `0x0` |
| `bl` at 0x4c774 | `(unknown callee) 0x2470c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4c6ec | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x4c6f8 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x4c704 | `r0 == 0x64` | `r0 != 0x64` |
| `beq` at 0x4c74c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x4c758 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c778 | `r0` | `ret(0x2470c)` |

## `IP57_Probe`

- `IP57_Probe`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c848 to 0x4caac.
  Established. The initial call targets `0x4e270` with `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 &
  0xff)` (`bl` at 0x4c878); its zero result takes the continuation and nonzero falls through to
  an early zero stack result (`beq` at 0x4c88c, `str` at 0x4c894). Indexed byte stores write `0x0`
  at `[sp-17]:1 +0x119ec0` and `0x2` at `[sp-17]:1 +0x2811e0` (`strb` at 0x4c8a8, `strb` at
  0x4c8b8). Later stores at those same respective targets write unknown and `0x1` (`strb` at
  0x4c9e4, `strb` at 0x4ca48). The merged r0 return and probing effects are unknown (`ldmdb` at
  0x4ca94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4c85c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4c864 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x4c868 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4c880 | store | 4 | `sp -0x20` | `ret(0x4e270)` |
| `ldr` at 0x4c884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4c894 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c89c | load | 4 | `abs 0x4ca98` | `-` |
| `ldrb` at 0x4c8a0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4c8a8 | store | 1 | `[sp-17]:1 +0x119ec0` | `0x0` |
| `ldr` at 0x4c8ac | load | 4 | `abs 0x4ca9c` | `-` |
| `ldrb` at 0x4c8b0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4c8b8 | store | 1 | `[sp-17]:1 +0x2811e0` | `0x2` |
| `ldr` at 0x4c8bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c8c0 | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x4c8c4 | store | 1 | `sp -0x21` | `([[sp-24]+3]:1 & 0xff)` |
| `ldrb` at 0x4c8c8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4c8cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c8d0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x4c8e4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x4c8f0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x4c8f8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `strb` at 0x4c904 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x4c908 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4c918 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c91c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x4c920 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x4c944 | store | 4 | `sp -0x20` | `ret(0xb7af8)` |
| `ldr` at 0x4c948 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4c954 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x4c968 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c974 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c980 | load | 4 | `abs 0x4caa0` | `-` |
| `str` at 0x4c984 | store | 4 | `sp -0x34` | `0xe39bc` |
| `ldrb` at 0x4c988 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x4c98c | store | 4 | `sp -0x30` | `[sp-33]:1` |
| `str` at 0x4c990 | store | 4 | `sp -0x2c` | `ret(0x1ea50)` |
| `ldr` at 0x4c998 | load | 4 | `abs 0x4caa4` | `-` |
| `ldr` at 0x4c99c | load | 4 | `abs 0x4caa8` | `-` |
| `ldrb` at 0x4c9a8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4c9ac | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4c9c0 | load | 4 | `abs 0x4ca98` | `-` |
| `ldrb` at 0x4c9c4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4c9c8 | load | 4 | `abs 0x4ca98` | `-` |
| `ldrb` at 0x4c9cc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4c9d0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x4c9dc | load | 1 | `[sp-17]:1 +0x119ec0` | `-` |
| `strb` at 0x4c9e4 | store | 1 | `[sp-17]:1 +0x119ec0` | `?` |
| `ldrb` at 0x4c9e8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x4c9f8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4ca04 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x4ca24 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4ca28 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x4ca3c | load | 4 | `abs 0x4ca9c` | `-` |
| `ldrb` at 0x4ca40 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4ca48 | store | 1 | `[sp-17]:1 +0x2811e0` | `0x1` |
| `ldrb` at 0x4ca4c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x4ca54 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x4ca5c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4ca74 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4ca8c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x4ca90 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4ca94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ca94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ca94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4c878 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4c8ec | `(unknown callee) 0xb415c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4c93c | `(unknown callee) 0xb7af8` | `?` | `sp-25` | `sp-25` | `?` |
| `bl` at 0x4c978 | `(unknown callee) 0x1ea50` | `ret(0xb7af8)` | `?` | `?` | `ret(0xb7af8)` |
| `bl` at 0x4c9a4 | `(unknown callee) 0x1eed0` | `0x1` | `0xe3930` | `0x11b` | `0x0` |
| `bl` at 0x4c9b8 | `(unknown callee) 0x4d414` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x4ca14 | `(unknown callee) 0x2e8fc` | `?` | `[sp-17]:1` | `?` | `?` |
| `bl` at 0x4ca34 | `(unknown callee) 0x4d32c` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x4ca64 | `(unknown callee) 0x2ef84` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4ca84 | `(unknown callee) 0x4cd84` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4c88c | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `ble` at 0x4c8dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x4c910 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x4c950 | `ret(0xb7af8) != 0x0` | `ret(0xb7af8) == 0x0` |
| `bne` at 0x4c960 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4c970 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `beq` at 0x4c9f4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x4ca20 | `ret(0x2e8fc) != 0x0` | `ret(0x2e8fc) == 0x0` |
| `bne` at 0x4ca70 | `ret(0x2ef84) != 0x0` | `ret(0x2ef84) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ca94 | `r0` | `?` |

## `IP57_Init`

- `IP57_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4caac to 0x4cb2c.
  Established. The call to `0x2ef00` carries `(r0 & 0xff)`, entry r1, entry r2, `(r0 & 0xff)` (`bl`
  at 0x4cad4); its zero result takes the bypass, while nonzero falls through to `0x2ef84` (`beq`
  at 0x4cae0, `bl` at 0x4caec). The second result takes that bypass when nonzero and falls through
  when zero to a call to `0x4cd84` with `[sp-17]:1`, `0x0`, `0x1`, `[sp-17]:1` (`bne` at 0x4caf8,
  `bl` at 0x4cb0c). The two arms store word `0x0` and word `0x1` at `sp -0x28`, respectively (`str`
  at 0x4cb14, `str` at 0x4cb20). The joined return and physical initialization role remain unproven
  (`ldmdb` at 0x4cb28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4cac0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4cac8 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x4cacc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4cae4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4cafc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cb14 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4cb20 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x4cb24 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4cb28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4cb28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4cb28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cad4 | `(unknown callee) 0x2ef00` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x4caec | `(unknown callee) 0x2ef84` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x4cb0c | `(unknown callee) 0x4cd84` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4cae0 | `ret(0x2ef00) == 0x0` | `ret(0x2ef00) != 0x0` |
| `bne` at 0x4caf8 | `ret(0x2ef84) != 0x0` | `ret(0x2ef84) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4cb28 | `r0` | `?` |

## `IP57_NodeRemove`

- `IP57_NodeRemove`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4cb2c to 0x4cc38.
  Established. A call to `0x4e270` uses `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)` (`bl` at
  0x4cb54); its zero result takes the continuing edge, while nonzero falls through and saves
  `ret(0x4e270)` as a word at `sp -0x24` (`beq` at 0x4cb68, `str` at 0x4cb70). The continuation
  calls `0x4cd84` with `[sp-17]:1`, `0x1`, `0x1`, `[sp-17]:1` (`bl` at 0x4cb88). After two loops, a
  byte store writes `0xf` at `[sp-17]:1 +0x2811e0` (`strb` at 0x4cc20). Node-removal semantics and
  the merged return remain unknown (`ldmdb` at 0x4cc30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4cb40 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4cb44 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4cb5c | store | 4 | `sp -0x1c` | `ret(0x4e270)` |
| `ldr` at 0x4cb60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4cb6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4cb70 | store | 4 | `sp -0x24` | `ret(0x4e270)` |
| `ldrb` at 0x4cb78 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4cb8c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4cb90 | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x4cb94 | store | 1 | `sp -0x1d` | `([[sp-24]+3]:1 & 0xff)` |
| `ldrb` at 0x4cb98 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4cb9c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4cba0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x4cbb4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x4cbc0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4cbc8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x4cbd0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4cbd4 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x4cbd8 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x4cbdc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x4cbe0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4cbe4 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x4cbf8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x4cc04 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4cc0c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x4cc14 | load | 4 | `abs 0x4cc34` | `-` |
| `ldrb` at 0x4cc18 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4cc20 | store | 1 | `[sp-17]:1 +0x2811e0` | `0xf` |
| `str` at 0x4cc28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4cc2c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4cc30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4cc30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4cc30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cb54 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4cb88 | `(unknown callee) 0x4cd84` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x4cbbc | `(unknown callee) 0xb40dc` | `?` | `?` | `?` | `?` |
| `bl` at 0x4cc00 | `(unknown callee) 0xb62c4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4cb68 | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `blt` at 0x4cbac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x4cbf0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4cc30 | `r0` | `?` |

## `IP57_NodeDisable`

- `IP57_NodeDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4cc38 to 0x4cc70.
  Established. The only call reaches `0x4cd84` with `(r0 & 0xff)`, `0x0`, `0x1`, `(r0 & 0xff)` (`bl`
  at 0x4cc60); its value reaches the r0 return as `ret(0x4cd84)` (`ldmdb` at 0x4cc6c). The only
  explicit store saves byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4cc4c). The wrapper has no
  reported conditional branches, and physical disable behavior is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4cc4c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4cc50 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x4cc6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4cc6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4cc6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cc60 | `(unknown callee) 0x4cd84` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4cc6c | `r0` | `ret(0x4cd84)` |

## `IP57_NodeEnable`

- `IP57_NodeEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4cc70 to 0x4cca4.
  Established. A byte store retains `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4cc84). The call to
  `0x4cf38` carries `(r0 & 0xff)`, `0x0`, entry r2, `(r0 & 0xff)` (`bl` at 0x4cc94); its result is
  the reported r0 return (`ldmdb` at 0x4cca0). No conditional branch is reported, and enabling a
  node is not established by the wrapper alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4cc84 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4cc88 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x4cca0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4cca0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4cca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4cc94 | `(unknown callee) 0x4cf38` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4cca0 | `r0` | `ret(0x4cf38)` |

## `IP57_NodePresent`

- `IP57_NodePresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d198 to 0x4d1d4.
  Established. An indexed byte read addresses `(r0 & 0xff) +0x2811e0` (`ldrb` at 0x4d1b8). The only
  explicit store saves byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4d1ac). The facts report no
  calls or conditional branches, and the predicated result remains unknown at the return (`ldmdb` at
  0x4d1cc). A physical node-presence interpretation is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d1ac | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4d1b0 | load | 4 | `abs 0x4d1d0` | `-` |
| `ldrb` at 0x4d1b4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d1b8 | load | 1 | `(r0 & 0xff) +0x2811e0` | `-` |
| `ldmdb` at 0x4d1cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d1cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d1cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d1cc | `r0` | `?` |

## `IP57_CpuPresent`

- `IP57_CpuPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d1d4 to 0x4d24c.
  Established. The call to `0x1c79c` has `0x7b`, entry r1, entry r2, unknown (`bl` at 0x4d208); its
  result takes the continuation for `ret(0x1c79c) != 0x1` and falls through on equality (`bne` at
  0x4d214). The equality arm stores word `0x0` at `sp -0x18` (`str` at 0x4d21c). The other arm reads
  a byte at `[sp-18]:1 +0x119ec0` (`ldrb` at 0x4d22c). The joined r0 value and CPU-presence meaning
  are unknown (`ldmdb` at 0x4d244).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d1e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4d1ec | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4d1f4 | store | 1 | `sp -0x12` | `?` |
| `ldrb` at 0x4d1f8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4d200 | store | 1 | `sp -0x13` | `?` |
| `str` at 0x4d21c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d224 | load | 4 | `abs 0x4d248` | `-` |
| `ldrb` at 0x4d228 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x4d22c | load | 1 | `[sp-18]:1 +0x119ec0` | `-` |
| `ldrb` at 0x4d230 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x4d23c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d240 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4d244 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d244 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d244 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d208 | `(unknown callee) 0x1c79c` | `0x7b` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4d214 | `ret(0x1c79c) != 0x1` | `ret(0x1c79c) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d244 | `r0` | `?` |

## `IP57_CpuDisable`

- `IP57_CpuDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d24c to 0x4d2bc.
  Established. A call to `0x4d1d4` carries `(r0 & 0xff)`, entry r1, entry r2, `(r0 & 0xff)` (`bl` at
  0x4d26c). A zero result takes the edge that stores word `0xfffff50e` at `sp -0x18`, while nonzero
  falls through to `0x4d32c` with four unknown candidates (`beq` at 0x4d278, `str` at 0x4d2ac, `bl`
  at 0x4d298). That call result is saved as a word at the same stack target (`str` at 0x4d2a0). The
  merged r0 return and physical CPU-disable action remain unknown (`ldmdb` at 0x4d2b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d260 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4d264 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d27c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d288 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d2a0 | store | 4 | `sp -0x18` | `ret(0x4d32c)` |
| `ldr` at 0x4d2a8 | load | 4 | `abs 0x4d2b8` | `-` |
| `str` at 0x4d2ac | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x4d2b0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4d2b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d2b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d2b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d26c | `(unknown callee) 0x4d1d4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x4d298 | `(unknown callee) 0x4d32c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d278 | `ret(0x4d1d4) == 0x0` | `ret(0x4d1d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d2b4 | `r0` | `?` |

## `IP57_CpuEnable`

- `IP57_CpuEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d2bc to 0x4d32c.
  Established. The first call targets `0x4d1d4` with `(r0 & 0xff)`, entry r1, entry r2, `(r0 &
  0xff)` (`bl` at 0x4d2dc); its zero result takes the fallback edge and nonzero falls through to
  `0x4d4d0`, whose four candidates are unknown (`beq` at 0x4d2e8, `bl` at 0x4d308). The call arm
  stores `ret(0x4d4d0)` and the fallback stores `0xfffff50e`, both as words at `sp -0x18` (`str` at
  0x4d310, `str` at 0x4d31c). The return is unknown after the join, and CPU-enable semantics are not
  proven (`ldmdb` at 0x4d324).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d2d0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4d2d4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d2ec | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d2f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d310 | store | 4 | `sp -0x18` | `ret(0x4d4d0)` |
| `ldr` at 0x4d318 | load | 4 | `abs 0x4d328` | `-` |
| `str` at 0x4d31c | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x4d320 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4d324 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d324 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d324 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d2dc | `(unknown callee) 0x4d1d4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x4d308 | `(unknown callee) 0x4d4d0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d2e8 | `ret(0x4d1d4) == 0x0` | `ret(0x4d1d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d324 | `r0` | `?` |

## `IP57_Nmi`

- `IP57_Nmi`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4dea0 to 0x4df74.
  Established. The call to `0xb7958` carries `([[sp-24]+2]:1+2 & 0xff)`, `0x10`, unknown,
  `([[sp-24]+2]:1+2 & 0xff)` (`bl` at 0x4df08); its zero result takes the continuing edge and
  nonzero falls through to save that result as a word at `sp -0x24` (`beq` at 0x4df1c, `str` at
  0x4df24). On continuation, a call to `0x4e310` has `0xa`, unknown, unknown, `ret(0xb7958)` (`bl`
  at 0x4df30). The following call to `0xb790c` uses `[sp-25]:1`, `0x10`, unknown, `[sp-25]:1` (`bl`
  at 0x4df40). Pulse timing, physical NMI effects and the merged return remain unknown (`ldmdb` at
  0x4df70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4deb4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x4deb8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4ded0 | store | 4 | `sp -0x20` | `ret(0x4e270)` |
| `ldr` at 0x4ded4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4dee0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4dee4 | store | 4 | `sp -0x24` | `ret(0x4e270)` |
| `ldr` at 0x4deec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4def0 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x4def8 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x4defc | load | 1 | `sp -0x19` | `-` |
| `str` at 0x4df10 | store | 4 | `sp -0x20` | `ret(0xb7958)` |
| `ldr` at 0x4df14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4df20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4df24 | store | 4 | `sp -0x24` | `ret(0xb7958)` |
| `ldrb` at 0x4df34 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x4df48 | store | 4 | `sp -0x20` | `ret(0xb790c)` |
| `ldr` at 0x4df4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4df58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4df5c | store | 4 | `sp -0x24` | `ret(0xb790c)` |
| `str` at 0x4df68 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4df6c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4df70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4df70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4df70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dec8 | `(unknown callee) 0x4e270` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x4df08 | `(unknown callee) 0xb7958` | `([[sp-24]+2]:1+2 & 0xff)` | `0x10` | `?` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x4df30 | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `ret(0xb7958)` |
| `bl` at 0x4df40 | `(unknown callee) 0xb790c` | `[sp-25]:1` | `0x10` | `?` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4dedc | `ret(0x4e270) == 0x0` | `ret(0x4e270) != 0x0` |
| `beq` at 0x4df1c | `ret(0xb7958) == 0x0` | `ret(0xb7958) != 0x0` |
| `beq` at 0x4df54 | `ret(0xb790c) == 0x0` | `ret(0xb790c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4df70 | `r0` | `?` |

## `IP57_Int`

- `IP57_Int`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4df74 to 0x4e1e0.
  Established. Six entry bytes are saved at `sp -0x15` through `sp -0x1a`, with values `(r0 &
  0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r3 & 0xff)`, `([sp] & 0xff)`, `([sp+4] & 0xff)` (`strb` at
  0x4df94, `strb` at 0x4df9c, `strb` at 0x4dfa4, `strb` at 0x4dfac, `strb` at 0x4dfb4, `strb` at
  0x4dfbc). The opening call to `0x2e538` carries `[sp-24]:1`, `[sp-25]:1`, `[sp-26]:1`, `[sp-24]:1`
  (`bl` at 0x4dfec). A signed test takes its bypass for `? >= 0x0` and falls through below zero
  (`bge` at 0x4dff8). A later call to `0xb7af8` has `[sp-24]:1`, `sp-26`, `sp-26`, `[sp-24]:1`, and
  its zero result takes the next continuation while nonzero falls through (`bl` at 0x4e014, `beq` at
  0x4e028). The returned value and interrupt-handling interpretation remain unknown (`ldmdb` at
  0x4e1ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4df88 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x4df8c | load | 4 | `sp +0x4` | `-` |
| `strb` at 0x4df94 | store | 1 | `sp -0x15` | `(r0 & 0xff)` |
| `strb` at 0x4df9c | store | 1 | `sp -0x16` | `(r1 & 0xff)` |
| `strb` at 0x4dfa4 | store | 1 | `sp -0x17` | `(r2 & 0xff)` |
| `strb` at 0x4dfac | store | 1 | `sp -0x18` | `(r3 & 0xff)` |
| `strb` at 0x4dfb4 | store | 1 | `sp -0x19` | `([sp] & 0xff)` |
| `strb` at 0x4dfbc | store | 1 | `sp -0x1a` | `([sp+4] & 0xff)` |
| `ldrb` at 0x4dfc0 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x4dfc8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x4dfd0 | store | 1 | `sp -0x1b` | `?` |
| `ldrb` at 0x4dfd4 | load | 1 | `sp -0x18` | `-` |
| `ldrb` at 0x4dfd8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x4dfdc | load | 1 | `sp -0x1a` | `-` |
| `ldrsb` at 0x4dff0 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0x4e004 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x4e01c | store | 4 | `sp -0x20` | `ret(0xb7af8)` |
| `ldr` at 0x4e020 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4e02c | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x4e03c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e048 | load | 4 | `abs 0x4e1c0` | `-` |
| `str` at 0x4e04c | store | 4 | `sp -0x38` | `0xe3a1c` |
| `str` at 0x4e050 | store | 4 | `sp -0x34` | `ret(0xb8268)` |
| `str` at 0x4e054 | store | 4 | `sp -0x30` | `ret(0x1ea50)` |
| `ldr` at 0x4e05c | load | 4 | `abs 0x4e1c4` | `-` |
| `ldr` at 0x4e060 | load | 4 | `abs 0x4e1c8` | `-` |
| `ldrb` at 0x4e070 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0x4e084 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x4e08c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x4e094 | load | 4 | `abs 0x4e1cc` | `-` |
| `ldrb` at 0x4e0a8 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x4e0c0 | store | 4 | `sp -0x20` | `ret(0x2d888)` |
| `ldr` at 0x4e0c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x4e0d0 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x4e0e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e0ec | load | 4 | `abs 0x4e1d0` | `-` |
| `str` at 0x4e0f0 | store | 4 | `sp -0x38` | `0xe3a84` |
| `str` at 0x4e0f4 | store | 4 | `sp -0x34` | `ret(0x2d92c)` |
| `str` at 0x4e0f8 | store | 4 | `sp -0x30` | `ret(0x1ea50)` |
| `ldr` at 0x4e100 | load | 4 | `abs 0x4e1c4` | `-` |
| `ldr` at 0x4e104 | load | 4 | `abs 0x4e1d4` | `-` |
| `ldr` at 0x4e114 | load | 4 | `abs 0x4e1d8` | `-` |
| `str` at 0x4e118 | store | 4 | `sp -0x38` | `0xe3aa8` |
| `ldrb` at 0x4e11c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x4e124 | store | 4 | `sp -0x34` | `[sp-21]:1+48` |
| `ldrb` at 0x4e128 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x4e130 | store | 4 | `sp -0x30` | `[sp-22]:1+65` |
| `ldr` at 0x4e134 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4e138 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x4e13c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4e158 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4e158 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4e174 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4e174 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4e194 | store | 4 | `sp -0x28` | `ret(0xd61ec)` |
| `ldr` at 0x4e19c | load | 4 | `abs 0x4e1c4` | `-` |
| `ldr` at 0x4e1a0 | load | 4 | `abs 0x4e1dc` | `-` |
| `ldmdb` at 0x4e1ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4e1ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e1ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e1ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dfec | `(unknown callee) 0x2e538` | `[sp-24]:1` | `[sp-25]:1` | `[sp-26]:1` | `[sp-24]:1` |
| `bl` at 0x4e000 | `(unknown callee) 0x4e310` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x4e014 | `(unknown callee) 0xb7af8` | `[sp-24]:1` | `sp-26` | `sp-26` | `[sp-24]:1` |
| `bl` at 0x4e034 | `(unknown callee) 0xb8268` | `[sp-24]:1` | `?` | `?` | `[sp-24]:1` |
| `bl` at 0x4e040 | `(unknown callee) 0x1ea50` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x4e068 | `(unknown callee) 0x1eed0` | `0x3` | `0xe3930` | `0x44f` | `0x0` |
| `bl` at 0x4e0a0 | `(unknown callee) 0x1dba4` | `0xe3a58` | `[sp-21]:1+48` | `[sp-22]:1+65` | `[sp-22]:1+65` |
| `bl` at 0x4e0b8 | `(unknown callee) 0x2d888` | `[sp-23]:1` | `sp-36` | `sp-36` | `[sp-23]:1` |
| `bl` at 0x4e0d8 | `(unknown callee) 0x2d92c` | `[sp-23]:1` | `?` | `?` | `[sp-23]:1` |
| `bl` at 0x4e0e4 | `(unknown callee) 0x1ea50` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x4e10c | `(unknown callee) 0x1eed0` | `0x3` | `0xe3930` | `0x459` | `0x0` |
| `bl` at 0x4e140 | `(unknown callee) 0xd5c0c` | `[sp-36]` | `?` | `?` | `[sp-36]` |
| `bl` at 0x4e15c | `(unknown callee) 0xd5c84` | `ret(0xd5c0c)` | `?` | `?` | `?` |
| `bl` at 0x4e178 | `(unknown callee) 0xd5900` | `ret(0xd5c84)` | `?` | `?` | `?` |
| `bl` at 0x4e18c | `(unknown callee) 0xd61ec` | `ret(0xd5900)` | `?` | `?` | `ret(0xd5900)` |
| `bl` at 0x4e1a8 | `(unknown callee) 0x1eed0` | `0x1` | `0xe3930` | `0x45c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x4dff8 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `beq` at 0x4e028 | `ret(0xb7af8) == 0x0` | `ret(0xb7af8) != 0x0` |
| `bne` at 0x4e080 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x4e0cc | `ret(0x2d888) == 0x0` | `ret(0x2d888) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e1ac | `r0` | `?` |

## `IP57_Get_Cfg`

- `IP57_Get_Cfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e1e0 to 0x4e240.
  Established. The absolute word at `0x119eb8` is read before a branch that takes the continuation
  for nonzero and falls through on zero (`ldr` at 0x4e1f8, `bne` at 0x4e200). The zero arm stores
  word `0xfffff510` at `sp -0x18` (`str` at 0x4e208). The nonzero arm stores word `[0x119eb8]` at
  entry `r0 +0x0`, then saves `[0x119eb4]` as a word at `sp -0x18` (`str` at 0x4e21c, `str` at
  0x4e228). No calls are reported, and the joined return and configuration-object meaning remain
  unknown (`ldmdb` at 0x4e230).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e1f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e1f4 | load | 4 | `abs 0x4e234` | `-` |
| `ldr` at 0x4e1f8 | load | 4 | `abs 0x119eb8` | `-` |
| `ldr` at 0x4e204 | load | 4 | `abs 0x4e238` | `-` |
| `str` at 0x4e208 | store | 4 | `sp -0x18` | `0xfffff510` |
| `ldr` at 0x4e210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e214 | load | 4 | `abs 0x4e234` | `-` |
| `ldr` at 0x4e218 | load | 4 | `abs 0x119eb8` | `-` |
| `str` at 0x4e21c | store | 4 | `r0 +0x0` | `[0x119eb8]` |
| `ldr` at 0x4e220 | load | 4 | `abs 0x4e23c` | `-` |
| `ldr` at 0x4e224 | load | 4 | `abs 0x119eb4` | `-` |
| `str` at 0x4e228 | store | 4 | `sp -0x18` | `[0x119eb4]` |
| `ldr` at 0x4e22c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4e230 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e230 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e230 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4e200 | `[0x119eb8] != 0x0` | `[0x119eb8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e230 | `r0` | `?` |

## `IP57_GetCpuMask`

- `IP57_GetCpuMask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e240 to 0x4e270.
  Established. The explicit byte store retains `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x4e254). A
  byte read addresses `(r0 & 0xff) +0x119ec0` and supplies the r0 return `[(r0 & 0xff)+1154752]:1`
  (`ldrb` at 0x4e260, `ldmdb` at 0x4e268). Calls and conditional branches are absent. The CPU-mask
  encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4e254 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x4e258 | load | 4 | `abs 0x4e26c` | `-` |
| `ldrb` at 0x4e25c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4e260 | load | 1 | `(r0 & 0xff) +0x119ec0` | `-` |
| `ldmdb` at 0x4e268 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e268 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e268 | `r0` | `[(r0 & 0xff)+1154752]:1` |
