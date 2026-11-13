# ARM display and LED entry paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates. No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/display-led-paths-facts.txt) retain all tool sections.

## `displayLedOn`

- `displayLedOn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c4a4 to 0x2c5e4.
  Established. Three separate selection paths call `0xb7958` with first candidate `[0x115474]:1` and second candidates
  `0x40`, `0x10`, `0x20`, respectively (`bl` at 0x2c550, `bl` at 0x2c570, `bl` at 0x2c590). Each has
  unknown third candidate and `[0x115474]:1` fourth (`bl` at 0x2c550, `bl` at 0x2c570, `bl` at
  0x2c590). The first branch takes its edge to `0x2c4d4` for `[0x119c2c]:1 != 0x0` and falls through on
  zero (`bne` at 0x2c4c4). The zero path stores word `0xfffff8f7` at `sp -0x18` (`str` at 0x2c4cc).
  The physical outputs selected by the numeric values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c4b4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2c4b8 | load | 4 | `abs 0x2c5cc` | `-` |
| `ldrb` at 0x2c4bc | load | 1 | `abs 0x119c2c` | `-` |
| `ldr` at 0x2c4c8 | load | 4 | `abs 0x2c5d0` | `-` |
| `str` at 0x2c4cc | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x2c4e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c4f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c510 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c514 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0x2c518 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c524 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c540 | load | 4 | `abs 0x2c5d4` | `-` |
| `ldrb` at 0x2c544 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c558 | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x2c560 | load | 4 | `abs 0x2c5d4` | `-` |
| `ldrb` at 0x2c564 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c578 | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x2c580 | load | 4 | `abs 0x2c5d4` | `-` |
| `ldrb` at 0x2c584 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c598 | store | 4 | `sp -0x18` | `ret(0xb7958)` |
| `ldr` at 0x2c5a0 | load | 4 | `abs 0x2c5d8` | `-` |
| `str` at 0x2c5a4 | store | 4 | `sp -0x24` | `0xdba14` |
| `ldr` at 0x2c5a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c5ac | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x2c5b4 | load | 4 | `abs 0x2c5dc` | `-` |
| `ldr` at 0x2c5b8 | load | 4 | `abs 0x2c5e0` | `-` |
| `ldr` at 0x2c5c4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2c5c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c5c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c5c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c4d4 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `[0x119c2c]:1` |
| `bl` at 0x2c50c | `(unknown callee) 0x2d164` | `0x8` | `0x1` | `?` | `[sp-20]` |
| `bl` at 0x2c550 | `(unknown callee) 0xb7958` | `[0x115474]:1` | `0x40` | `?` | `[0x115474]:1` |
| `bl` at 0x2c570 | `(unknown callee) 0xb7958` | `[0x115474]:1` | `0x10` | `?` | `[0x115474]:1` |
| `bl` at 0x2c590 | `(unknown callee) 0xb7958` | `[0x115474]:1` | `0x20` | `?` | `[0x115474]:1` |
| `bl` at 0x2c5c0 | `(unknown callee) 0x1eed0` | `0x9` | `0xdb960` | `0x1db` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c4c4 | `[0x119c2c]:1 != 0x0` | `[0x119c2c]:1 == 0x0` |
| `bne` at 0x2c4e4 | `? != 0x39` | `? == 0x39` |
| `beq` at 0x2c4f0 | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x2c4fc | `[sp-20] == 0x2` | `[sp-20] != 0x2` |
| `beq` at 0x2c520 | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `blo` at 0x2c52c | `[sp-20] < 0x1 (unsigned, 32-bit)` | `[sp-20] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x2c538 | `[sp-20] == 0x2` | `[sp-20] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c5c8 | `r0` | `?` |

## `displayLedOff`

- `displayLedOff`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c5e4 to 0x2c71c.
  Established. Separate selection paths call `0xb790c` with `[0x115474]:1`, `0x40`, unknown,
  `[0x115474]:1`, or the same candidates with second values `0x10` and `0x20` (`bl` at 0x2c68c, `bl` at 0x2c6ac, `bl` at
  0x2c6cc). The branch comparing `[sp-20]` to `0x1` takes its edge for unsigned less-than and falls
  through otherwise (`blo` at 0x2c668). The call to `0x2d1d0` carries `0x8`, unknown, unknown,
  `[sp-20]` (`bl` at 0x2c648). The joined return and physical light state are unknown (`ldmdb` at
  0x2c704).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c5f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2c5f8 | load | 4 | `abs 0x2c708` | `-` |
| `ldrb` at 0x2c5fc | load | 1 | `abs 0x119c2c` | `-` |
| `ldr` at 0x2c608 | load | 4 | `abs 0x2c70c` | `-` |
| `str` at 0x2c60c | store | 4 | `sp -0x18` | `0xfffff8f7` |
| `ldr` at 0x2c628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c64c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c650 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0x2c654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c660 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c66c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c67c | load | 4 | `abs 0x2c710` | `-` |
| `ldrb` at 0x2c680 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c694 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2c69c | load | 4 | `abs 0x2c710` | `-` |
| `ldrb` at 0x2c6a0 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c6b4 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2c6bc | load | 4 | `abs 0x2c710` | `-` |
| `ldrb` at 0x2c6c0 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c6d4 | store | 4 | `sp -0x18` | `ret(0xb790c)` |
| `ldr` at 0x2c6dc | load | 4 | `abs 0x2c714` | `-` |
| `str` at 0x2c6e0 | store | 4 | `sp -0x24` | `0xdba14` |
| `ldr` at 0x2c6e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c6e8 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x2c6f0 | load | 4 | `abs 0x2c718` | `-` |
| `ldr` at 0x2c700 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2c704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c614 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `[0x119c2c]:1` |
| `bl` at 0x2c648 | `(unknown callee) 0x2d1d0` | `0x8` | `?` | `?` | `[sp-20]` |
| `bl` at 0x2c68c | `(unknown callee) 0xb790c` | `[0x115474]:1` | `0x40` | `?` | `[0x115474]:1` |
| `bl` at 0x2c6ac | `(unknown callee) 0xb790c` | `[0x115474]:1` | `0x10` | `?` | `[0x115474]:1` |
| `bl` at 0x2c6cc | `(unknown callee) 0xb790c` | `[0x115474]:1` | `0x20` | `?` | `[0x115474]:1` |
| `bl` at 0x2c6fc | `(unknown callee) 0x1eed0` | `0x9` | `0xdb960` | `0x1fc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c604 | `[0x119c2c]:1 != 0x0` | `[0x119c2c]:1 == 0x0` |
| `bne` at 0x2c624 | `? != 0x39` | `? == 0x39` |
| `beq` at 0x2c630 | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x2c63c | `[sp-20] == 0x2` | `[sp-20] != 0x2` |
| `beq` at 0x2c65c | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `blo` at 0x2c668 | `[sp-20] < 0x1 (unsigned, 32-bit)` | `[sp-20] >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x2c674 | `[sp-20] == 0x2` | `[sp-20] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c704 | `r0` | `?` |

## `displayLedStatus`

- `displayLedStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c71c to 0x2c810.
  Established. Separate paths store bytes `0x40`, `0x10`, `0x20` at `sp -0x15` (`strb` at 0x2c780,
  `strb` at 0x2c78c, `strb` at 0x2c798). Entry r0 equal to `0x1` or `0x2` takes the corresponding
  selection edges, while inequality falls through (`beq` at 0x2c75c, `beq` at 0x2c774). The call to
  `0xb7b78` has candidates `[0x115474]:1`, unknown, unknown, `[0x115474]:1` (`bl` at 0x2c7d0). The
  final word store at `sp -0x1c` has unknown value (`str` at 0x2c7f0). The status encoding and
  merged return remain unknown (`ldmdb` at 0x2c7f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c72c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2c730 | load | 4 | `abs 0x2c7fc` | `-` |
| `ldrb` at 0x2c734 | load | 1 | `abs 0x115474` | `-` |
| `ldr` at 0x2c740 | load | 4 | `abs 0x2c800` | `-` |
| `str` at 0x2c744 | store | 4 | `sp -0x1c` | `0xfffff8f8` |
| `ldr` at 0x2c74c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c750 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x2c754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2c760 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2c76c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x2c780 | store | 1 | `sp -0x15` | `0x40` |
| `strb` at 0x2c78c | store | 1 | `sp -0x15` | `0x10` |
| `strb` at 0x2c798 | store | 1 | `sp -0x15` | `0x20` |
| `ldr` at 0x2c7a0 | load | 4 | `abs 0x2c804` | `-` |
| `str` at 0x2c7a4 | store | 4 | `sp -0x28` | `0xdba14` |
| `ldr` at 0x2c7a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2c7ac | store | 4 | `sp -0x24` | `r0` |
| `ldr` at 0x2c7b4 | load | 4 | `abs 0x2c808` | `-` |
| `ldr` at 0x2c7b8 | load | 4 | `abs 0x2c80c` | `-` |
| `ldr` at 0x2c7c4 | load | 4 | `abs 0x2c7fc` | `-` |
| `ldrb` at 0x2c7c8 | load | 1 | `abs 0x115474` | `-` |
| `ldrb` at 0x2c7d8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x2c7f0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2c7f4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2c7f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c7f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c7f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c7c0 | `(unknown callee) 0x1eed0` | `0x9` | `0xdb960` | `0x21f` | `0x0` |
| `bl` at 0x2c7d0 | `(unknown callee) 0xb7b78` | `[0x115474]:1` | `?` | `?` | `[0x115474]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c73c | `[0x115474]:1 != 0xff` | `[0x115474]:1 == 0xff` |
| `beq` at 0x2c75c | `r0 == 0x1` | `r0 != 0x1` |
| `blo` at 0x2c768 | `r0 < 0x1 (unsigned, 32-bit)` | `r0 >= 0x1 (unsigned, 32-bit)` |
| `beq` at 0x2c774 | `r0 == 0x2` | `r0 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c7f8 | `r0` | `?` |

## `displayReset`

- `displayReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2c810 to 0x2ca0c.
  Established. Absolute byte stores set `0x119cae` to `0x1` and later `0x0` (`strb` at 0x2c89c,
  `strb` at 0x2c93c). Two indirect calls have unknown targets, with candidates `[0x115474]:1`,
  unknown, `[0x119c40]`, unknown and `0x119caf`, `0xdb940`, `0x4`, `[0x119c4c]` (`mov` at 0x2c8d4,
  `mov` at 0x2c930). A later call to `0xb79a0` uses `[0x115474]:1`, `0x70`, `[sp-21]:1`,
  `[0x115474]:1` (`bl` at 0x2c958); its zero result takes the continuation, while nonzero falls
  through to the return (`beq` at 0x2c96c). A physical reset effect is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2c820 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2c824 | load | 4 | `abs 0x2c9dc` | `-` |
| `ldrb` at 0x2c828 | load | 1 | `abs 0x119c2c` | `-` |
| `ldr` at 0x2c834 | load | 4 | `abs 0x2c9e0` | `-` |
| `str` at 0x2c838 | store | 4 | `sp -0x20` | `0xfffff8f7` |
| `ldr` at 0x2c840 | load | 4 | `abs 0x2c9e4` | `-` |
| `ldrb` at 0x2c844 | load | 1 | `abs 0x115474` | `-` |
| `strb` at 0x2c858 | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0x2c85c | load | 4 | `abs 0x2c9e4` | `-` |
| `ldrb` at 0x2c860 | load | 1 | `abs 0x115474` | `-` |
| `str` at 0x2c878 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x2c87c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c888 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2c88c | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `ldr` at 0x2c894 | load | 4 | `abs 0x2c9e8` | `-` |
| `strb` at 0x2c89c | store | 1 | `abs 0x119cae` | `0x1` |
| `ldr` at 0x2c8a0 | load | 4 | `abs 0x2c9ec` | `-` |
| `ldr` at 0x2c8a4 | load | 4 | `abs 0x2c9e4` | `-` |
| `ldrb` at 0x2c8a8 | load | 1 | `abs 0x115474` | `-` |
| `ldr` at 0x2c8ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2c8c8 | load | 4 | `abs 0x119c40` | `-` |
| `str` at 0x2c8dc | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x2c8e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c8ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c8f8 | load | 4 | `abs 0x2c9f0` | `-` |
| `str` at 0x2c8fc | store | 4 | `sp -0x28` | `0xdb9d8` |
| `str` at 0x2c900 | store | 4 | `sp -0x24` | `ret(0x1ea50)` |
| `ldr` at 0x2c908 | load | 4 | `abs 0x2c9f4` | `-` |
| `ldr` at 0x2c90c | load | 4 | `abs 0x2c9f8` | `-` |
| `ldr` at 0x2c918 | load | 4 | `abs 0x2c9ec` | `-` |
| `ldr` at 0x2c91c | load | 4 | `abs 0x119c4c` | `-` |
| `ldr` at 0x2c920 | load | 4 | `abs 0x2c9fc` | `-` |
| `ldr` at 0x2c924 | load | 4 | `abs 0x2ca00` | `-` |
| `ldr` at 0x2c934 | load | 4 | `abs 0x2c9e8` | `-` |
| `strb` at 0x2c93c | store | 1 | `abs 0x119cae` | `0x0` |
| `ldr` at 0x2c944 | load | 4 | `abs 0x2c9e4` | `-` |
| `ldrb` at 0x2c948 | load | 1 | `abs 0x115474` | `-` |
| `ldrb` at 0x2c94c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x2c960 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x2c964 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c970 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2c974 | store | 4 | `sp -0x20` | `ret(0xb79a0)` |
| `ldr` at 0x2c97c | load | 4 | `abs 0x2ca04` | `-` |
| `ldrb` at 0x2c980 | load | 1 | `abs 0x119c54` | `-` |
| `ldr` at 0x2c990 | load | 4 | `abs 0x2ca04` | `-` |
| `str` at 0x2c9a0 | store | 4 | `sp -0x1c` | `ret(0x2bfac)` |
| `ldr` at 0x2c9a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2c9b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2c9b4 | store | 4 | `sp -0x20` | `ret(0x2bfac)` |
| `ldr` at 0x2c9c0 | load | 4 | `abs 0x2ca08` | `-` |
| `str` at 0x2c9d0 | store | 4 | `sp -0x20` | `ret(0x2bfac)` |
| `ldr` at 0x2c9d4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x2c9d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2c9d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2c9d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2c84c | `(unknown callee) 0xb7b78` | `[0x115474]:1` | `r1` | `r2` | `[0x115474]:1` |
| `bl` at 0x2c870 | `(unknown callee) 0xb79a0` | `[0x115474]:1` | `0x70` | `0x0` | `[0x115474]:1` |
| `mov` at 0x2c8d4 | `(unknown callee) (unknown target)` | `[0x115474]:1` | `?` | `[0x119c40]` | `?` |
| `bl` at 0x2c8f0 | `(unknown callee) 0x1ea50` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0x2c914 | `(unknown callee) 0x1eed0` | `0x3` | `0xdb960` | `0x246` | `0x0` |
| `mov` at 0x2c930 | `(unknown callee) (unknown target)` | `0x119caf` | `0xdb940` | `0x4` | `[0x119c4c]` |
| `bl` at 0x2c940 | `(unknown callee) 0x2c470` | `ret(unknown)` | `?` | `0x119cae` | `0x0` |
| `bl` at 0x2c958 | `(unknown callee) 0xb79a0` | `[0x115474]:1` | `0x70` | `[sp-21]:1` | `[0x115474]:1` |
| `bl` at 0x2c998 | `(unknown callee) 0x2bfac` | `0x31` | `0x119c54` | `0x0` | `[0x119c54]:1` |
| `bl` at 0x2c9c8 | `(unknown callee) 0x2bfac` | `0x32` | `0xdba34` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2c830 | `[0x119c2c]:1 != 0x0` | `[0x119c2c]:1 == 0x0` |
| `beq` at 0x2c884 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x2c8e8 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x2c96c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |
| `beq` at 0x2c988 | `[0x119c54]:1 == 0x0` | `[0x119c54]:1 != 0x0` |
| `beq` at 0x2c9ac | `ret(0x2bfac) == 0x0` | `ret(0x2bfac) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2c9d8 | `r0` | `?` |

## `frontpanelLEDInit`

- `frontpanelLEDInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d0dc to 0x2d164.
  Established. Word stores address absolute `0x119cc0` with unknown value and `0x119cc4` with
  `[sp-24]` (`str` at 0x2d140, `str` at 0x2d14c). Entry r1 was saved at `sp -0x18` (`str` at
  0x2d0f0). A zero value at `[0x119cc0]` takes the bypass and nonzero falls through to the only
  call (`beq` at 0x2d100). That call targets `0x1eed0` with `0x9`, `0xdbbfc`, `0x3a`, `0x0` (`bl` at
  0x2d134). The slots’ physical interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d0ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d0f0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2d0f4 | load | 4 | `abs 0x2d154` | `-` |
| `ldr` at 0x2d0f8 | load | 4 | `abs 0x119cc0` | `-` |
| `ldr` at 0x2d104 | load | 4 | `abs 0x2d158` | `-` |
| `str` at 0x2d108 | store | 4 | `sp -0x24` | `0xdbbc8` |
| `ldr` at 0x2d10c | load | 4 | `abs 0x2d154` | `-` |
| `ldr` at 0x2d110 | load | 4 | `abs 0x119cc0` | `-` |
| `str` at 0x2d114 | store | 4 | `sp -0x20` | `[0x119cc0]` |
| `ldr` at 0x2d118 | load | 4 | `abs 0x2d15c` | `-` |
| `ldr` at 0x2d11c | load | 4 | `abs 0x119cc4` | `-` |
| `str` at 0x2d120 | store | 4 | `sp -0x1c` | `[0x119cc4]` |
| `ldr` at 0x2d128 | load | 4 | `abs 0x2d160` | `-` |
| `ldr` at 0x2d138 | load | 4 | `abs 0x2d154` | `-` |
| `ldr` at 0x2d13c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d140 | store | 4 | `abs 0x119cc0` | `?` |
| `ldr` at 0x2d144 | load | 4 | `abs 0x2d15c` | `-` |
| `ldr` at 0x2d148 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2d14c | store | 4 | `abs 0x119cc4` | `[sp-24]` |
| `ldmdb` at 0x2d150 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d150 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d150 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d134 | `(unknown callee) 0x1eed0` | `0x9` | `0xdbbfc` | `0x3a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2d100 | `[0x119cc0] == 0x0` | `[0x119cc0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d150 | `r0` | `?` |

## `frontpanelLEDOn`

- `frontpanelLEDOn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d164 to 0x2d1d0.
  Established. When entry r0 is not equal to `0x4`, the branch takes the longer path, while equality
  falls through to word `0x0` at `sp -0x1c` (`bne` at 0x2d184, `str` at 0x2d18c). The longer
  path clears absolute word `0x119cb4` (`str` at 0x2d19c); its calls include `0x2d1d0` with `0xff`,
  entry r1, `0x0`, `[sp-24]` and `0x2d460` with `[sp-20]` followed by three unknown candidates (`bl`
  at 0x2d1b0, `bl` at 0x2d1b8). Output polarity and the joined return are unknown (`ldmdb` at
  0x2d1c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d174 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d178 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2d17c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d18c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x2d194 | load | 4 | `abs 0x2d1cc` | `-` |
| `str` at 0x2d19c | store | 4 | `abs 0x119cb4` | `0x0` |
| `ldr` at 0x2d1a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2d1b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1c0 | store | 4 | `sp -0x1c` | `ret(0x2d460)` |
| `ldr` at 0x2d1c4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2d1c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d1c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d1c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d1b0 | `(unknown callee) 0x2d1d0` | `0xff` | `r1` | `0x0` | `[sp-24]` |
| `bl` at 0x2d1b8 | `(unknown callee) 0x2d460` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2d184 | `r0 != 0x4` | `r0 == 0x4` |
| `bne` at 0x2d1a8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d1c8 | `r0` | `?` |

## `frontpanelLEDOff`

- `frontpanelLEDOff`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d1d0 to 0x2d224.
  Established. The first branch takes its longer path for entry `r0 != 0x4` and falls through on
  equality (`bne` at 0x2d1ec). The longer path clears word `0x119cb4` before calling `0x2d568`
  (`str` at 0x2d204, `bl` at 0x2d20c). That call carries `[sp-20]`, entry r1, `0x0`, `0x119cb4`
  (`bl` at 0x2d20c). The result is stored at `sp -0x18`, but the joined r0 return is unknown (`str`
  at 0x2d214, `ldmdb` at 0x2d21c). A physical off state is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d1e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d1e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d1f4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2d1fc | load | 4 | `abs 0x2d220` | `-` |
| `str` at 0x2d204 | store | 4 | `abs 0x119cb4` | `0x0` |
| `ldr` at 0x2d208 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d214 | store | 4 | `sp -0x18` | `ret(0x2d568)` |
| `ldr` at 0x2d218 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2d21c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d21c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d21c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d20c | `(unknown callee) 0x2d568` | `[sp-20]` | `r1` | `0x0` | `0x119cb4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2d1ec | `r0 != 0x4` | `r0 == 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d21c | `r0` | `?` |

## `frontpanelLEDBlink`

- `frontpanelLEDBlink`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d224 to 0x2d30c.
  Established. The absolute word `0x119cb4` receives `0x1`, and absolute byte `0x119cc8` receives
  `([sp-20]:1 & 0xff)` (`str` at 0x2d25c, `strb` at 0x2d268). Later paths write byte `0x1` or `0x0`
  at `0x119cb9` (`strb` at 0x2d298, `strb` at 0x2d2d4). The signed test takes its edge for `[sp-24]
  > 0x0` and falls through otherwise (`bgt` at 0x2d2a4). A byte store at `0x119cb8` has unknown
  value (`strb` at 0x2d2bc). The return reports `0x0`, with blink timing and physical behavior
  unknown (`ldmdb` at 0x2d2f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d234 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d238 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2d23c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x2d240 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2d254 | load | 4 | `abs 0x2d2f4` | `-` |
| `str` at 0x2d25c | store | 4 | `abs 0x119cb4` | `0x1` |
| `ldr` at 0x2d260 | load | 4 | `abs 0x2d2f8` | `-` |
| `ldrb` at 0x2d264 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x2d268 | store | 1 | `abs 0x119cc8` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x2d26c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d27c | load | 4 | `abs 0x2d2fc` | `-` |
| `ldr` at 0x2d280 | load | 4 | `abs 0x2d300` | `-` |
| `ldr` at 0x2d284 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d288 | load | 1 | `[0x119cc0] +0x2` | `-` |
| `strb` at 0x2d28c | store | 1 | `abs 0x119cc9` | `([[0x119cc0]+2]:1 & 0xff)` |
| `ldr` at 0x2d290 | load | 4 | `abs 0x2d304` | `-` |
| `strb` at 0x2d298 | store | 1 | `abs 0x119cb9` | `0x1` |
| `ldr` at 0x2d29c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2d2ac | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x2d2b0 | load | 4 | `abs 0x2d308` | `-` |
| `ldr` at 0x2d2b4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x2d2bc | store | 1 | `abs 0x119cb8` | `?` |
| `str` at 0x2d2c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2d2cc | load | 4 | `abs 0x2d304` | `-` |
| `strb` at 0x2d2d4 | store | 1 | `abs 0x119cb9` | `0x0` |
| `ldr` at 0x2d2d8 | load | 4 | `abs 0x2d2fc` | `-` |
| `ldrb` at 0x2d2dc | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x2d2e0 | store | 1 | `abs 0x119cc9` | `([sp-20]:1 & 0xff)` |
| `str` at 0x2d2e8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2d2ec | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x2d2f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d2f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d2f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d250 | `(unknown callee) 0x2d1d0` | `0xff` | `r1` | `r2` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2d248 | `r2 != 0x0` | `r2 == 0x0` |
| `beq` at 0x2d278 | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x2d2a4 | `[sp-24] > 0x0 (signed, 32-bit)` | `[sp-24] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d2f0 | `r0` | `0x0` |

## `frontpanelLEDBlinkTask`

- `frontpanelLEDBlinkTask`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d30c to 0x2d460.
  Established. Two byte stores at absolute `0x119cc9` write `([[0x119cc0]+6]:1 & 0xff)` and
  `([[0x119cc0]+2]:1 & 0xff)` on separate paths (`strb` at 0x2d390, `strb` at 0x2d3a8). Their
  deciding branch takes its edge for `[0x119cc9]:1 != [[0x119cc0]+2]:1` and falls through on
  equality (`bne` at 0x2d374). Word stores at `0x1154c4` include `0x3` and `[0x1154c4]-1` (`str` at
  0x2d420, `str` at 0x2d438). Calls target `0x2d568` or `0x2d460` with candidates `[0x119cc9]:1`,
  `[0x119cca]:1`, `0x119cc9`, `[0x119cc9]:1` (`bl` at 0x2d400, `bl` at 0x2d414). Their physical
  actions and the timing unit are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2d318 | load | 4 | `abs 0x2d440` | `-` |
| `ldr` at 0x2d31c | load | 4 | `abs 0x119cb4` | `-` |
| `ldr` at 0x2d32c | load | 4 | `abs 0x2d444` | `-` |
| `ldr` at 0x2d330 | load | 4 | `abs 0x1154c4` | `-` |
| `ldr` at 0x2d33c | load | 4 | `abs 0x2d448` | `-` |
| `ldrb` at 0x2d340 | load | 1 | `abs 0x119cb9` | `-` |
| `ldr` at 0x2d34c | load | 4 | `abs 0x2d44c` | `-` |
| `ldr` at 0x2d350 | load | 4 | `abs 0x119cbc` | `-` |
| `ldr` at 0x2d35c | load | 4 | `abs 0x2d450` | `-` |
| `ldr` at 0x2d360 | load | 4 | `abs 0x2d454` | `-` |
| `ldr` at 0x2d364 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d368 | load | 1 | `abs 0x119cc9` | `-` |
| `ldrb` at 0x2d36c | load | 1 | `[0x119cc0] +0x2` | `-` |
| `ldr` at 0x2d378 | load | 4 | `abs 0x2d450` | `-` |
| `ldr` at 0x2d380 | load | 4 | `abs 0x2d454` | `-` |
| `ldr` at 0x2d384 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d38c | load | 1 | `[0x119cc0] +0x6` | `-` |
| `strb` at 0x2d390 | store | 1 | `abs 0x119cc9` | `([[0x119cc0]+6]:1 & 0xff)` |
| `ldr` at 0x2d398 | load | 4 | `abs 0x2d450` | `-` |
| `ldr` at 0x2d39c | load | 4 | `abs 0x2d454` | `-` |
| `ldr` at 0x2d3a0 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d3a4 | load | 1 | `[0x119cc0] +0x2` | `-` |
| `strb` at 0x2d3a8 | store | 1 | `abs 0x119cc9` | `([[0x119cc0]+2]:1 & 0xff)` |
| `ldr` at 0x2d3ac | load | 4 | `abs 0x2d44c` | `-` |
| `ldr` at 0x2d3b0 | load | 4 | `abs 0x2d458` | `-` |
| `ldrb` at 0x2d3b4 | load | 1 | `abs 0x119cb8` | `-` |
| `str` at 0x2d3b8 | store | 4 | `abs 0x119cbc` | `[0x119cb8]:1` |
| `ldr` at 0x2d3c0 | load | 4 | `abs 0x2d44c` | `-` |
| `ldr` at 0x2d3c4 | load | 4 | `abs 0x2d44c` | `-` |
| `ldr` at 0x2d3c8 | load | 4 | `abs 0x119cbc` | `-` |
| `str` at 0x2d3d0 | store | 4 | `abs 0x119cbc` | `[0x119cbc]-1` |
| `ldr` at 0x2d3d4 | load | 4 | `abs 0x2d45c` | `-` |
| `ldr` at 0x2d3d8 | load | 4 | `abs 0x2d450` | `-` |
| `ldrb` at 0x2d3dc | load | 1 | `abs 0x119cca` | `-` |
| `ldrb` at 0x2d3e0 | load | 1 | `abs 0x119cc9` | `-` |
| `ldr` at 0x2d3f4 | load | 4 | `abs 0x2d450` | `-` |
| `ldrb` at 0x2d3f8 | load | 1 | `abs 0x119cc9` | `-` |
| `ldr` at 0x2d408 | load | 4 | `abs 0x2d450` | `-` |
| `ldrb` at 0x2d40c | load | 1 | `abs 0x119cc9` | `-` |
| `ldr` at 0x2d418 | load | 4 | `abs 0x2d444` | `-` |
| `str` at 0x2d420 | store | 4 | `abs 0x1154c4` | `0x3` |
| `ldr` at 0x2d428 | load | 4 | `abs 0x2d444` | `-` |
| `ldr` at 0x2d42c | load | 4 | `abs 0x2d444` | `-` |
| `ldr` at 0x2d430 | load | 4 | `abs 0x1154c4` | `-` |
| `str` at 0x2d438 | store | 4 | `abs 0x1154c4` | `[0x1154c4]-1` |
| `ldmdb` at 0x2d43c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d43c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d43c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d400 | `(unknown callee) 0x2d568` | `[0x119cc9]:1` | `[0x119cca]:1` | `0x119cc9` | `[0x119cc9]:1` |
| `bl` at 0x2d414 | `(unknown callee) 0x2d460` | `[0x119cc9]:1` | `[0x119cca]:1` | `0x119cc9` | `[0x119cc9]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2d324 | `[0x119cb4] != 0x0` | `[0x119cb4] == 0x0` |
| `bge` at 0x2d338 | `[0x1154c4] >= 0x0 (signed, 32-bit)` | `[0x1154c4] < 0x0 (signed, 32-bit)` |
| `beq` at 0x2d348 | `[0x119cb9]:1 == 0x0` | `[0x119cb9]:1 != 0x0` |
| `bgt` at 0x2d358 | `[0x119cbc] > 0x1 (signed, 32-bit)` | `[0x119cbc] <= 0x1 (signed, 32-bit)` |
| `bne` at 0x2d374 | `[0x119cc9]:1 != [[0x119cc0]+2]:1` | `[0x119cc9]:1 == [[0x119cc0]+2]:1` |
| `beq` at 0x2d3f0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d43c | `r0` | `?` |

## `fp_led_on_raw`

- `fp_led_on_raw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d460 to 0x2d568.
  Established. The only call targets `0xb79a0` with four unknown candidates (`bl` at 0x2d508); its
  zero result takes the bypass and nonzero falls through to storing `ret(0xb79a0)` at `sp -0x18`
  (`beq` at 0x2d51c, `str` at 0x2d524). A subsequent byte store at absolute `0x119cca` has unknown
  value (`strb` at 0x2d53c). The return reports `[sp-24]` (`ldmdb` at 0x2d558). The physical signal
  mapping remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d470 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d478 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2d47c | load | 4 | `abs 0x2d55c` | `-` |
| `ldr` at 0x2d480 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d484 | load | 4 | `abs 0x119cc4` | `-` |
| `ldr` at 0x2d494 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d49c | load | 4 | `abs 0x2d560` | `-` |
| `ldr` at 0x2d4a0 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d4a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2d4ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d4bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d4c4 | load | 4 | `abs 0x2d560` | `-` |
| `ldr` at 0x2d4c8 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d4d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2d4d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d4dc | load | 4 | `abs 0x2d560` | `-` |
| `ldr` at 0x2d4e0 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d4e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2d4ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d4f4 | load | 4 | `abs 0x2d560` | `-` |
| `ldr` at 0x2d4f8 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d500 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2d510 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x2d514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2d520 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2d524 | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x2d528 | load | 4 | `abs 0x2d564` | `-` |
| `ldr` at 0x2d52c | load | 4 | `abs 0x2d564` | `-` |
| `ldrb` at 0x2d530 | load | 1 | `abs 0x119cca` | `-` |
| `ldrb` at 0x2d534 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x2d53c | store | 1 | `abs 0x119cca` | `?` |
| `ldr` at 0x2d540 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2d548 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2d550 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2d558 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d558 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d508 | `(unknown callee) 0xb79a0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2d48c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x2d4b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2d51c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d558 | `r0` | `[sp-24]` |

## `fp_led_off_raw`

- `fp_led_off_raw`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d568 to 0x2d664.
  Established. The call to `0xb79a0` carries unknown, unknown, `0x0`, unknown (`bl` at 0x2d600); its
  nonzero result falls through to word `ret(0xb79a0)` at `sp -0x18`, while zero takes the bypass
  (`beq` at 0x2d614, `str` at 0x2d61c). A byte store at absolute `0x119cca` has unknown value
  (`strb` at 0x2d638). The reported r0 return is `[sp-24]` (`ldmdb` at 0x2d654). Clearing a
  particular light is not established by the numeric callee.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d578 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d580 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2d584 | load | 4 | `abs 0x2d658` | `-` |
| `ldr` at 0x2d588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d58c | load | 4 | `abs 0x119cc4` | `-` |
| `ldr` at 0x2d59c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d5a4 | load | 4 | `abs 0x2d65c` | `-` |
| `ldr` at 0x2d5a8 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d5b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2d5b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d5c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d5cc | load | 4 | `abs 0x2d65c` | `-` |
| `ldr` at 0x2d5d0 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d5d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x2d5dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d5e4 | load | 4 | `abs 0x2d65c` | `-` |
| `ldr` at 0x2d5e8 | load | 4 | `abs 0x119cc0` | `-` |
| `ldrb` at 0x2d5f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2d608 | store | 4 | `sp -0x1c` | `ret(0xb79a0)` |
| `ldr` at 0x2d60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2d618 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2d61c | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x2d620 | load | 4 | `abs 0x2d660` | `-` |
| `ldr` at 0x2d624 | load | 4 | `abs 0x2d660` | `-` |
| `ldrb` at 0x2d628 | load | 1 | `sp -0x14` | `-` |
| `ldrb` at 0x2d630 | load | 1 | `abs 0x119cca` | `-` |
| `strb` at 0x2d638 | store | 1 | `abs 0x119cca` | `?` |
| `ldr` at 0x2d63c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2d644 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x2d64c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x2d654 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d654 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d654 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d600 | `(unknown callee) 0xb79a0` | `?` | `?` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x2d594 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x2d5c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x2d614 | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d654 | `r0` | `[sp-24]` |

## `pwrStatusLEDInit`

- `pwrStatusLEDInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaaf6c to 0xaaff8.
  Established. The absolute word stores write unknown value at `0x27eac4` and `[sp-24]` at
  `0x27eac8` (`str` at 0xaafd0, `str` at 0xaafdc). Zero `[0x27eac4]` takes the bypass and nonzero
  falls through to `0x1eed0` (`beq` at 0xaaf90, `bl` at 0xaafc4). The last call targets
  `0xaaff8` with candidates unknown, unknown, `0x27eac8`, `[sp-24]` (`bl` at 0xaafe0); its result
  reaches the return (`ldmdb` at 0xaafe4). The saved words’ hardware role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaaf7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaaf80 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xaaf84 | load | 4 | `abs 0xaafe8` | `-` |
| `ldr` at 0xaaf88 | load | 4 | `abs 0x27eac4` | `-` |
| `ldr` at 0xaaf94 | load | 4 | `abs 0xaafec` | `-` |
| `str` at 0xaaf98 | store | 4 | `sp -0x24` | `0x107b64` |
| `ldr` at 0xaaf9c | load | 4 | `abs 0xaafe8` | `-` |
| `ldr` at 0xaafa0 | load | 4 | `abs 0x27eac4` | `-` |
| `str` at 0xaafa4 | store | 4 | `sp -0x20` | `[0x27eac4]` |
| `ldr` at 0xaafa8 | load | 4 | `abs 0xaaff0` | `-` |
| `ldr` at 0xaafac | load | 4 | `abs 0x27eac8` | `-` |
| `str` at 0xaafb0 | store | 4 | `sp -0x1c` | `[0x27eac8]` |
| `ldr` at 0xaafb8 | load | 4 | `abs 0xaaff4` | `-` |
| `ldr` at 0xaafc8 | load | 4 | `abs 0xaafe8` | `-` |
| `ldr` at 0xaafcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaafd0 | store | 4 | `abs 0x27eac4` | `?` |
| `ldr` at 0xaafd4 | load | 4 | `abs 0xaaff0` | `-` |
| `ldr` at 0xaafd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xaafdc | store | 4 | `abs 0x27eac8` | `[sp-24]` |
| `ldmdb` at 0xaafe4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaafe4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaafe4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaafc4 | `(unknown callee) 0x1eed0` | `0x9` | `0x107b8c` | `0x2c` | `0x0` |
| `bl` at 0xaafe0 | `(unknown callee) 0xaaff8` | `?` | `?` | `0x27eac8` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaaf90 | `[0x27eac4] == 0x0` | `[0x27eac4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaafe4 | `r0` | `ret(0xaaff8)` |

## `pwrStatusLEDUpdate`

- `pwrStatusLEDUpdate`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaaff8 to 0xab234.
  Established. The call to `0x3ab58` has four unknown candidates (`bl` at 0xab080). Two later tests
  take their edges when its result equals `0x2` or `0xfffff9b9`, respectively, and fall through on
  inequality (`beq` at 0xab094, `beq` at 0xab0a4). Calls to `0xb790c` and `0xb7958` each retain four
  unknown candidates (`bl` at 0xab128, `bl` at 0xab168, `bl` at 0xab1d0, `bl` at 0xab210). Stack
  word `sp -0x1c` can receive `[sp-28]+1` (`str` at 0xab0b0). The physical status update and merged
  r0 return remain unknown (`ldmdb` at 0xab224).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab00c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xab014 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xab018 | load | 4 | `abs 0xab228` | `-` |
| `ldr` at 0xab01c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab020 | load | 4 | `abs 0x27eac8` | `-` |
| `str` at 0xab034 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xab038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab040 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab044 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab04c | load | 1 | `(unknown)` | `-` |
| `str` at 0xab050 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xab054 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab05c | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab060 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab068 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab07c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab088 | store | 4 | `sp -0x20` | `ret(0x3ab58)` |
| `ldr` at 0xab08c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab098 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab09c | load | 4 | `abs 0xab230` | `-` |
| `ldr` at 0xab0a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab0b0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xab0b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab0bc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xab0c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab0d8 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab0dc | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab0e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab0f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab0f8 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab0fc | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab104 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab110 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab114 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab11c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab138 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab13c | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab144 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab150 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab154 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab15c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab16c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab180 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab184 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab18c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab198 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab1a0 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab1a4 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab1ac | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab1b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab1b8 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab1bc | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab1c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab1d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab1e0 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab1e4 | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab1ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab1f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab1f8 | load | 4 | `abs 0xab22c` | `-` |
| `ldr` at 0xab1fc | load | 4 | `abs 0x27eac4` | `-` |
| `ldrb` at 0xab204 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab214 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab21c | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0xab224 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab224 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab080 | `(unknown callee) 0x3ab58` | `?` | `?` | `?` | `?` |
| `bl` at 0xab128 | `(unknown callee) 0xb790c` | `?` | `?` | `?` | `?` |
| `bl` at 0xab168 | `(unknown callee) 0xb7958` | `?` | `?` | `?` | `?` |
| `bl` at 0xab1d0 | `(unknown callee) 0xb7958` | `?` | `?` | `?` | `?` |
| `bl` at 0xab210 | `(unknown callee) 0xb790c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xab028 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0xab074 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xab094 | `ret(0x3ab58) == 0x2` | `ret(0x3ab58) != 0x2` |
| `beq` at 0xab0a4 | `ret(0x3ab58) == 0xfffff9b9` | `ret(0x3ab58) != 0xfffff9b9` |
| `bne` at 0xab0cc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xab0ec | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xab174 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xab194 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab224 | `r0` | `?` |

## `pcixLedOn`

- `pcixLedOn`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x494e0 to 0x496d8.
  Established. Calls to `0x4c2b4` include unknown, `0x1`, unknown, unknown and later `ret(0x4b89c)`,
  `0x1`, unknown, `ret(0x4b89c)` (`bl` at 0x49554, `bl` at 0x496b8). The initial branch takes its
  edge for `(r0 & 0xff) != 0xff` and falls through on equality (`bne` at 0x49514). A later call to
  `0x4b89c` carries `(r0 & 0xff)`, `(r1 & 0xff)`, `(r1 & 0xff)`, `(r0 & 0xff)` (`bl` at 0x49678).
  The physical output selection and merged return remain unknown (`ldmdb` at 0x496c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x494f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x49500 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x49508 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x4950c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4951c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49520 | load | 4 | `abs 0x496cc` | `-` |
| `ldr` at 0x49524 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49528 | load | 4 | `abs 0x119ea8` | `-` |
| `ldr` at 0x49538 | load | 4 | `abs 0x496d0` | `-` |
| `ldr` at 0x4953c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49544 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x4955c | store | 4 | `sp -0x24` | `ret(0x4c2b4)` |
| `ldr` at 0x49560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4956c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49578 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4957c | store | 4 | `sp -0x20` | `ret(0x4c2b4)` |
| `ldr` at 0x49580 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49588 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49590 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49594 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x4959c | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x495a8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x495b8 | store | 4 | `sp -0x18` | `ret(0x4b7e4)` |
| `ldr` at 0x495bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x495c8 | load | 4 | `abs 0x496d4` | `-` |
| `str` at 0x495cc | store | 4 | `sp -0x2c` | `0xfffff703` |
| `ldr` at 0x495d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x495d8 | load | 1 | `ret(0x4b7e4) +0x1` | `-` |
| `str` at 0x495dc | store | 4 | `sp -0x28` | `[ret(0x4b7e4)+1]:1` |
| `ldr` at 0x495e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x495e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x495e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x495ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x495f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49604 | load | 4 | `abs 0x496d0` | `-` |
| `ldr` at 0x49608 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49610 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49628 | store | 4 | `sp -0x24` | `ret(0x4c2b4)` |
| `ldr` at 0x4962c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49648 | store | 4 | `sp -0x20` | `ret(0x4c2b4)` |
| `ldr` at 0x4964c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49654 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x4965c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49660 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldrb` at 0x49668 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4966c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x49680 | store | 4 | `sp -0x1c` | `ret(0x4b89c)` |
| `ldr` at 0x49684 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x49690 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x49694 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x496a8 | store | 4 | `sp -0x2c` | `ret(0x4ba38)` |
| `ldr` at 0x496b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x496c0 | store | 4 | `sp -0x2c` | `ret(0x4c2b4)` |
| `ldr` at 0x496c4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x496c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x496c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x496c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49554 | `(unknown callee) 0x4c2b4` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x495b0 | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x49620 | `(unknown callee) 0x4c2b4` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x49678 | `(unknown callee) 0x4b89c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x496a0 | `(unknown callee) 0x4ba38` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0x496b8 | `(unknown callee) 0x4c2b4` | `ret(0x4b89c)` | `0x1` | `?` | `ret(0x4b89c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x49514 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x49530 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49568 | `ret(0x4c2b4) == 0x0` | `ret(0x4c2b4) != 0x0` |
| `bne` at 0x49574 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x495a4 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x495c4 | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `blt` at 0x495fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49634 | `ret(0x4c2b4) == 0x0` | `ret(0x4c2b4) != 0x0` |
| `bne` at 0x49640 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x4968c | `ret(0x4b89c) != 0x0` | `ret(0x4b89c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x496c8 | `r0` | `?` |

## `pcixLedOff`

- `pcixLedOff`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x496d8 to 0x498d0.
  Established. The first and last calls to `0x4c2b4` have second candidate `0x0`, while the middle
  call has `0x1` (`bl` at 0x4974c, `bl` at 0x49818, `bl` at 0x498b0). A zero result from `0x4b89c` falls through to `0x4ba38`, while
  nonzero takes the path to the last `0x4c2b4` call (`bne` at 0x49884, `bl` at 0x49898, `bl` at
  0x498b0). Stack word `sp -0x2c` can receive `0xfffff703` (`str` at 0x497c4). The differing second
  candidates’ physical meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x496f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x496f8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x49700 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x49704 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x49714 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49718 | load | 4 | `abs 0x498c4` | `-` |
| `ldr` at 0x4971c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49720 | load | 4 | `abs 0x119ea8` | `-` |
| `ldr` at 0x49730 | load | 4 | `abs 0x498c8` | `-` |
| `ldr` at 0x49734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4973c | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49754 | store | 4 | `sp -0x24` | `ret(0x4c2b4)` |
| `ldr` at 0x49758 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49764 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49770 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49774 | store | 4 | `sp -0x20` | `ret(0x4c2b4)` |
| `ldr` at 0x49778 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49780 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49788 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4978c | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x49794 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x497a0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x497b0 | store | 4 | `sp -0x18` | `ret(0x4b7e4)` |
| `ldr` at 0x497b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x497c0 | load | 4 | `abs 0x498cc` | `-` |
| `str` at 0x497c4 | store | 4 | `sp -0x2c` | `0xfffff703` |
| `ldr` at 0x497cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x497d0 | load | 1 | `ret(0x4b7e4) +0x1` | `-` |
| `str` at 0x497d4 | store | 4 | `sp -0x28` | `[ret(0x4b7e4)+1]:1` |
| `ldr` at 0x497d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x497dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x497e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x497e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x497ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x497fc | load | 4 | `abs 0x498c8` | `-` |
| `ldr` at 0x49800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49808 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49820 | store | 4 | `sp -0x24` | `ret(0x4c2b4)` |
| `ldr` at 0x49824 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49830 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4983c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49840 | store | 4 | `sp -0x20` | `ret(0x4c2b4)` |
| `ldr` at 0x49844 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4984c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49854 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49858 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldrb` at 0x49860 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x49864 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x49878 | store | 4 | `sp -0x1c` | `ret(0x4b89c)` |
| `ldr` at 0x4987c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x49888 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4988c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x498a0 | store | 4 | `sp -0x2c` | `ret(0x4ba38)` |
| `ldr` at 0x498a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x498b8 | store | 4 | `sp -0x2c` | `ret(0x4c2b4)` |
| `ldr` at 0x498bc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x498c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x498c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x498c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4974c | `(unknown callee) 0x4c2b4` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x497a8 | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x49818 | `(unknown callee) 0x4c2b4` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x49870 | `(unknown callee) 0x4b89c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x49898 | `(unknown callee) 0x4ba38` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0x498b0 | `(unknown callee) 0x4c2b4` | `ret(0x4b89c)` | `0x0` | `?` | `ret(0x4b89c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4970c | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x49728 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49760 | `ret(0x4c2b4) == 0x0` | `ret(0x4c2b4) != 0x0` |
| `bne` at 0x4976c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x4979c | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x497bc | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `blt` at 0x497f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x4982c | `ret(0x4c2b4) == 0x0` | `ret(0x4c2b4) != 0x0` |
| `bne` at 0x49838 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x49884 | `ret(0x4b89c) != 0x0` | `ret(0x4b89c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x498c0 | `r0` | `?` |
