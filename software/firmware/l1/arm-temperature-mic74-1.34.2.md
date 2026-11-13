# ARM L1 temperature and MIC74 entry paths in 1.34.2

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
No unresolved computed transfers were reported for these routines.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/temperature-mic74-facts.txt) retain all tool sections.

## `tempInit`

- `tempInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d664 to 0x2d6d4.
  Established. The word read at 0x281020 decides `beq`, whose taken zero path skips the call to
  0x1eed0 (`ldr` at 0x2d680, `beq` at 0x2d688, `bl` at 0x2d6a4). Later word stores write an unknown
  value at 0x281020 and `[sp-24]` at 0x281040 (`str` at 0x2d6b0, `str` at 0x2d6bc). Their
  temperature-configuration meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d674 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2d678 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2d67c | load | 4 | `abs 0x2d6c4` | `-` |
| `ldr` at 0x2d680 | load | 4 | `abs 0x281020` | `-` |
| `ldr` at 0x2d68c | load | 4 | `abs 0x2d6c8` | `-` |
| `str` at 0x2d690 | store | 4 | `sp -0x1c` | `0xdbc14` |
| `ldr` at 0x2d698 | load | 4 | `abs 0x2d6cc` | `-` |
| `ldr` at 0x2d6a8 | load | 4 | `abs 0x2d6c4` | `-` |
| `ldr` at 0x2d6ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2d6b0 | store | 4 | `abs 0x281020` | `?` |
| `ldr` at 0x2d6b4 | load | 4 | `abs 0x2d6d0` | `-` |
| `ldr` at 0x2d6b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2d6bc | store | 4 | `abs 0x281040` | `[sp-24]` |
| `ldmdb` at 0x2d6c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d6c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d6c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d6a4 | `(unknown callee) 0x1eed0` | `0x9` | `0xdbc2c` | `0x2e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2d688 | `[0x281020] == 0x0` | `[0x281020] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d6c0 | `r0` | `?` |

## `tempGetCfg`

- `tempGetCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d6d4 to 0x2d710.
  Established. The word loaded at 0x281020 is stored as `[0x281020]` at entry r0 +0x0 (`ldr` at
  0x2d6f0, `str` at 0x2d6f4). The separate word load from 0x281040 supplies return expression
  `[0x281040]` (`ldr` at 0x2d6fc, `ldmdb` at 0x2d704). The stored and returned words'
  interpretations are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d6e4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x2d6e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2d6ec | load | 4 | `abs 0x2d708` | `-` |
| `ldr` at 0x2d6f0 | load | 4 | `abs 0x281020` | `-` |
| `str` at 0x2d6f4 | store | 4 | `r0 +0x0` | `[0x281020]` |
| `ldr` at 0x2d6f8 | load | 4 | `abs 0x2d70c` | `-` |
| `ldr` at 0x2d6fc | load | 4 | `abs 0x281040` | `-` |
| `ldmdb` at 0x2d704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d704 | `r0` | `[0x281040]` |

## `tempCount`

- `tempCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d710 to 0x2d730.
  Established. A word read from 0x281040 becomes r0 expression `[0x281040]` at return (`ldr` at
  0x2d720, `ldmdb` at 0x2d728). This body does not establish the returned word's meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2d71c | load | 4 | `abs 0x2d72c` | `-` |
| `ldr` at 0x2d720 | load | 4 | `abs 0x281040` | `-` |
| `ldmdb` at 0x2d728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d728 | `r0` | `[0x281040]` |

## `tempPresent`

- `tempPresent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d730 to 0x2d770.
  Established. The first call targets 0x2da24 with r0 candidate `(r0 & 0xff)` (`bl` at 0x2d750). A
  byte read at that call result +0x0 supplies the first candidate `[ret(0x2da24)]:1` for 0xaff6c
  (`ldrb` at 0x2d758, `bl` at 0x2d760). The return carries `ret(0xaff6c)` (`ldmdb` at 0x2d76c). The
  physical presence test remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d744 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d748 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x2d758 | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `ldmdb` at 0x2d76c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d76c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d76c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d750 | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d760 | `(unknown callee) 0xaff6c` | `[ret(0x2da24)]:1` | `?` | `?` | `[ret(0x2da24)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d76c | `r0` | `ret(0xaff6c)` |

## `tempSetLimits`

- `tempSetLimits`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d770 to 0x2d7e0.
  Established. Three byte saves retain masked entry r0, r1 and r2 at sp -0x11, sp -0x12 and sp -0x13
  (`strb` at 0x2d784, `strb` at 0x2d78c, `strb` at 0x2d794). The call to 0xb023c receives candidates
  `[ret(0x2da24)]:1`, `[ret(0x2da24)+1]:1`, `[sp-18]:1` and `[sp-19]:1` (`bl` at 0x2d7d0). The limit
  units and device effect are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d784 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x2d78c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x2d794 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x2d798 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d7a8 | store | 4 | `sp -0x18` | `ret(0x2da24)` |
| `ldr` at 0x2d7ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d7b0 | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `ldr` at 0x2d7b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d7b8 | load | 1 | `ret(0x2da24) +0x1` | `-` |
| `ldrb` at 0x2d7bc | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x2d7c0 | load | 1 | `sp -0x13` | `-` |
| `ldmdb` at 0x2d7dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d7dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d7dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d7a0 | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d7d0 | `(unknown callee) 0xb023c` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` | `[sp-18]:1` | `[sp-19]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d7dc | `r0` | `ret(0xb023c)` |

## `tempIntDisable`

- `tempIntDisable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d7e0 to 0x2d834.
  Established. The word save at sp -0x18 retains the result of the call to 0x2da24 (`bl` at 0x2d800,
  `str` at 0x2d808). The following byte loads read that result +0x0 and +0x1 (`ldrb` at 0x2d810,
  `ldrb` at 0x2d818). The call to 0xb1198 has candidates `[ret(0x2da24)]:1`, `[ret(0x2da24)+1]:1`,
  `[ret(0x2da24)]:1` and `[ret(0x2da24)+1]:1` (`bl` at 0x2d824); its interrupt-disable semantics are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d7f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d7f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d808 | store | 4 | `sp -0x18` | `ret(0x2da24)` |
| `ldr` at 0x2d80c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d810 | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `ldr` at 0x2d814 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d818 | load | 1 | `ret(0x2da24) +0x1` | `-` |
| `ldmdb` at 0x2d830 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d830 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d800 | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d824 | `(unknown callee) 0xb1198` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d830 | `r0` | `ret(0xb1198)` |

## `tempIntEnable`

- `tempIntEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d834 to 0x2d888.
  Established. The first call to 0x2da24 uses r0 candidate `(r0 & 0xff)` (`bl` at 0x2d854). A later
  call targets 0xb11e0 with first candidates `[ret(0x2da24)]:1` and `[ret(0x2da24)+1]:1` (`bl` at
  0x2d878). The return retains `ret(0xb11e0)` in r0 (`ldmdb` at 0x2d884). The physical
  interrupt-enable effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d848 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d84c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d85c | store | 4 | `sp -0x18` | `ret(0x2da24)` |
| `ldr` at 0x2d860 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d864 | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `ldr` at 0x2d868 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d86c | load | 1 | `ret(0x2da24) +0x1` | `-` |
| `ldmdb` at 0x2d884 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d884 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d884 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d854 | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d878 | `(unknown callee) 0xb11e0` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d884 | `r0` | `ret(0xb11e0)` |

## `tempRead`

- `tempRead`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d888 to 0x2d92c.
  Established. The call to 0xb07d4 uses candidates `[ret(0x2da24)]:1`, `[ret(0x2da24)+1]:1`, sp-29
  and `[ret(0x2da24)+1]:1` (`bl` at 0x2d8d4). `beq` takes its zero-result path past the word-clear
  at `[sp-24] +0x0` (`beq` at 0x2d8e8, `str` at 0x2d8f4). That taken path includes signed byte loads
  at sp -0x1d and `[sp-28] +0x2`, followed by an unknown-valued word store at `[sp-24] +0x0`
  (`ldrsb` at 0x2d908, `ldrsb` at 0x2d910, `str` at 0x2d918). The temperature units and conversion
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2d89c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x2d8a0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d8a4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d8b4 | store | 4 | `sp -0x1c` | `ret(0x2da24)` |
| `ldr` at 0x2d8b8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2d8bc | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `ldr` at 0x2d8c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2d8c4 | load | 1 | `ret(0x2da24) +0x1` | `-` |
| `str` at 0x2d8dc | store | 4 | `sp -0x24` | `ret(0xb07d4)` |
| `ldr` at 0x2d8e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2d8ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2d8f4 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x2d8f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x2d8fc | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x2d904 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x2d908 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x2d90c | load | 4 | `sp -0x1c` | `-` |
| `ldrsb` at 0x2d910 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x2d918 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0x2d920 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x2d924 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x2d928 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d928 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d8ac | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d8d4 | `(unknown callee) 0xb07d4` | `[ret(0x2da24)]:1` | `[ret(0x2da24)+1]:1` | `sp-29` | `[ret(0x2da24)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2d8e8 | `ret(0xb07d4) == 0x0` | `ret(0xb07d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d928 | `r0` | `?` |

## `tempLocation`

- `tempLocation`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d92c to 0x2d994.
  Established. The first word read at `ret(0x2da24) +0x4` is saved at sp -0x1c (`ldr` at 0x2d95c,
  `str` at 0x2d960). A second load from that location decides `bne`, whose nonzero taken path skips
  0xb14ac (`ldr` at 0x2d968, `bne` at 0x2d970, `bl` at 0x2d980). Fall-through calls 0xb14ac with
  first candidate `[ret(0x2da24)]:1` and saves its result at sp -0x1c (`bl` at 0x2d980, `str` at
  0x2d984). The loads need not agree, and the returned value's meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d940 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d944 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d954 | store | 4 | `sp -0x18` | `ret(0x2da24)` |
| `ldr` at 0x2d958 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2d95c | load | 4 | `ret(0x2da24) +0x4` | `-` |
| `str` at 0x2d960 | store | 4 | `sp -0x1c` | `[ret(0x2da24)+4]` |
| `ldr` at 0x2d964 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2d968 | load | 4 | `ret(0x2da24) +0x4` | `-` |
| `ldr` at 0x2d974 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d978 | load | 1 | `ret(0x2da24) +0x0` | `-` |
| `str` at 0x2d984 | store | 4 | `sp -0x1c` | `ret(0xb14ac)` |
| `ldr` at 0x2d988 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x2d990 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2d990 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2d990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d94c | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d980 | `(unknown callee) 0xb14ac` | `[ret(0x2da24)]:1` | `?` | `?` | `[ret(0x2da24)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2d970 | `[ret(0x2da24)+4] != 0x0` | `[ret(0x2da24)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2d990 | `r0` | `?` |

## `tempType`

- `tempType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d994 to 0x2da24.
  Established. The calls to 0x2da24 and 0xaff10 precede a signed `bge` test of `[[sp-24]]:1 >=
  ret(0xaff10)` (`bl` at 0x2d9b4, `bl` at 0x2d9c8, `bge` at 0x2d9e4); its taken path stores 0x0 at
  sp -0x24 (`str` at 0x2da18). Fall-through instead reaches a byte load with unknown effective
  target, followed by an unknown-valued word store to that slot (`ldrb` at 0x2da08, `str` at
  0x2da0c). The temperature-type encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x2d9a8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x2d9ac | load | 1 | `sp -0x11` | `-` |
| `str` at 0x2d9bc | store | 4 | `sp -0x18` | `ret(0x2da24)` |
| `str` at 0x2d9d0 | store | 4 | `sp -0x20` | `ret(0xaff10)` |
| `ldr` at 0x2d9d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d9d8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x2d9dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x2d9e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2d9ec | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x2da00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2da08 | load | 1 | `(unknown)` | `-` |
| `str` at 0x2da0c | store | 4 | `sp -0x24` | `?` |
| `str` at 0x2da18 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x2da1c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x2da20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2da20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2da20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d9b4 | `(unknown callee) 0x2da24` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x2d9c8 | `(unknown callee) 0xaff10` | `sp-28` | `?` | `?` | `sp-28` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x2d9e4 | `[[sp-24]]:1 >= ret(0xaff10) (signed, 32-bit)` | `[[sp-24]]:1 < ret(0xaff10) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2da20 | `r0` | `?` |

## `mic74_Reset`

- `mic74_Reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9f44 to 0xba17c.
  Established. The first call to 0xabb28 has r2 candidate 0x4 and r3 candidate 0x0 (`bl` at
  0xb9f88). Later calls to that target include r2 candidates 0x0, 0x2, 0x5, 0x1 and 0x6 at separate
  sites (`bl` at 0xb9fe4, `bl` at 0xba028, `bl` at 0xba06c, `bl` at 0xba0b8, `bl` at 0xba0fc). The
  final call to 0xabc4c has r2 candidate 0x3 and r3 candidate sp-23 (`bl` at 0xba144). After that
  call, `beq` skips the result replacement when the call result is zero, and `bne` skips it for
  nonzero `[sp-28]` (`beq` at 0xba158, `bne` at 0xba164, `str` at 0xba16c). The register meanings
  and physical reset sequence are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9f54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb9f58 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9f5c | load | 1 | `r0 +0x1` | `-` |
| `strb` at 0xb9f60 | store | 1 | `sp -0x15` | `([r0+1]:1 & 0xff)` |
| `ldr` at 0xb9f64 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9f68 | load | 1 | `r0 +0x2` | `-` |
| `strb` at 0xb9f6c | store | 1 | `sp -0x16` | `([r0+2]:1 & 0xff)` |
| `ldrb` at 0xb9f70 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xb9f74 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xb9f90 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xb9f94 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xb9f98 | store | 4 | `sp -0x24` | `[sp-21]:1` |
| `ldrb` at 0xb9f9c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xb9fa0 | store | 4 | `sp -0x28` | `[sp-22]:1` |
| `str` at 0xb9fa8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb9fac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9fb0 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb9fbc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb9fc0 | load | 1 | `[sp-20] +0x4` | `-` |
| `str` at 0xb9fd0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xb9fd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9fd8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb9fe0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb9fec | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xb9ff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb9ffc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba008 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba00c | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xba010 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xba014 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xba018 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba01c | load | 1 | `[sp-20] +0x5` | `-` |
| `str` at 0xba030 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xba034 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba040 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba04c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba050 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xba054 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xba058 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xba05c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba060 | load | 1 | `[sp-20] +0x6` | `-` |
| `str` at 0xba074 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xba078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba090 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba094 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xba098 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xba09c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xba0a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba0a4 | load | 1 | `[sp-20] +0x4` | `-` |
| `str` at 0xba0c0 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xba0c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba0d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba0dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba0e0 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xba0e4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xba0e8 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xba104 | store | 4 | `sp -0x20` | `ret(0xabb28)` |
| `ldr` at 0xba108 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba114 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba120 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba124 | store | 4 | `sp -0x1c` | `ret(0xabb28)` |
| `ldrb` at 0xba128 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xba12c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xba14c | store | 4 | `sp -0x20` | `ret(0xabc4c)` |
| `ldr` at 0xba150 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xba15c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xba168 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba16c | store | 4 | `sp -0x1c` | `ret(0xabc4c)` |
| `ldr` at 0xba170 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xba178 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba178 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba178 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb9f88 | `(unknown callee) 0xabb28` | `([r0+1]:1 & 0xff)` | `([r0+2]:1 & 0xff)` | `0x4` | `0x0` |
| `bl` at 0xb9fe4 | `(unknown callee) 0xabb28` | `[sp-21]:1` | `[sp-22]:1` | `0x0` | `?` |
| `bl` at 0xba028 | `(unknown callee) 0xabb28` | `[sp-21]:1` | `[sp-22]:1` | `0x2` | `[[sp-20]+5]:1` |
| `bl` at 0xba06c | `(unknown callee) 0xabb28` | `[sp-21]:1` | `[sp-22]:1` | `0x5` | `[[sp-20]+6]:1` |
| `bl` at 0xba0b8 | `(unknown callee) 0xabb28` | `[sp-21]:1` | `[sp-22]:1` | `0x1` | `?` |
| `bl` at 0xba0fc | `(unknown callee) 0xabb28` | `[sp-21]:1` | `[sp-22]:1` | `0x6` | `0x0` |
| `bl` at 0xba144 | `(unknown callee) 0xabc4c` | `[sp-21]:1` | `[sp-22]:1` | `0x3` | `sp-23` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb9fb8 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `beq` at 0xb9fc8 | `[[sp-20]+4]:1 == 0x0` | `[[sp-20]+4]:1 != 0x0` |
| `beq` at 0xb9ff8 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xba004 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba03c | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xba048 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba080 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xba08c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba0cc | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xba0d8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba110 | `ret(0xabb28) == 0x0` | `ret(0xabb28) != 0x0` |
| `bne` at 0xba11c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xba158 | `ret(0xabc4c) == 0x0` | `ret(0xabc4c) != 0x0` |
| `bne` at 0xba164 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba178 | `r0` | `[sp-28]` |

## `mic74_IntSet`

- `mic74_IntSet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba17c to 0xba1cc.
  Established. Byte reads at entry r0 +0x1 and +0x2 supply the first two candidates for 0xabb28
  (`ldrb` at 0xba19c, `ldrb` at 0xba1a4, `bl` at 0xba1bc). The next candidates are 0x4 and `(r1 &
  0xff)` (`bl` at 0xba1bc); its result reaches r0 at return (`ldmdb` at 0xba1c8). The interrupt-set
  hardware effect is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba18c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xba194 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xba198 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba19c | load | 1 | `r0 +0x1` | `-` |
| `ldr` at 0xba1a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba1a4 | load | 1 | `r0 +0x2` | `-` |
| `ldrb` at 0xba1a8 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba1c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba1bc | `(unknown callee) 0xabb28` | `[r0+1]:1` | `[r0+2]:1` | `0x4` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba1c8 | `r0` | `ret(0xabb28)` |

## `mic74_IntGet`

- `mic74_IntGet`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba1cc to 0xba214.
  Established. Entry r1 is saved as a word at sp -0x18 (`str` at 0xba1e0). The call to 0xabc4c uses
  candidates `[r0+1]:1`, `[r0+2]:1`, 0x4 and entry r1 (`bl` at 0xba204). The return carries
  `ret(0xabc4c)` (`ldmdb` at 0xba210). The addressed register and interrupt-status format remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba1dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba1e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xba1e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba1e8 | load | 1 | `r0 +0x1` | `-` |
| `ldr` at 0xba1ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba1f0 | load | 1 | `r0 +0x2` | `-` |
| `ldr` at 0xba200 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xba210 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba210 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba210 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba204 | `(unknown callee) 0xabc4c` | `[r0+1]:1` | `[r0+2]:1` | `0x4` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba210 | `r0` | `ret(0xabc4c)` |

## `mic74_Int`

- `mic74_Int`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba214 to 0xba394.
  Established. Two word stores initialize sp -0x20 and sp -0x28 to 0x0 (`str` at 0xba238, `str` at
  0xba240). The loop's `ble` has unknown operands (`ble` at 0xba24c); its call to 0xabc4c has first
  two candidates unknown, followed by 0x3 and sp-26 (`bl` at 0xba278). A later `beq` tests an
  unknown expression against zero. Nonzero fall-through reaches 0xb8014, whose result is included in
  the word store `([sp-32] + ret(0xb8014))` at sp -0x20 (`beq` at 0xba2ec, `bl` at 0xba308, `str` at
  0xba318). The iteration count and physical interrupt behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba224 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba228 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xba230 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `str` at 0xba238 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xba240 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xba244 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xba254 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba258 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xba25c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xba260 | load | 1 | `(unknown)` | `-` |
| `str` at 0xba280 | store | 4 | `sp -0x24` | `ret(0xabc4c)` |
| `ldr` at 0xba284 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xba290 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xba29c | load | 4 | `abs 0xba388` | `-` |
| `str` at 0xba2a0 | store | 4 | `sp -0x3c` | `0x109698` |
| `ldr` at 0xba2a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba2a8 | load | 4 | `[sp-20] +0xc` | `-` |
| `str` at 0xba2ac | store | 4 | `sp -0x38` | `[[sp-20]+12]` |
| `str` at 0xba2b0 | store | 4 | `sp -0x34` | `ret(0x1ea50)` |
| `ldr` at 0xba2b8 | load | 4 | `abs 0xba38c` | `-` |
| `ldr` at 0xba2c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba2cc | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xba2d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xba2d8 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xba2dc | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xba2f0 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xba2f4 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xba2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba2fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xba310 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba318 | store | 4 | `sp -0x20` | `([sp-32] + ret(0xb8014))` |
| `ldr` at 0xba320 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba324 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xba32c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xba334 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xba33c | load | 4 | `abs 0xba390` | `-` |
| `str` at 0xba340 | store | 4 | `sp -0x3c` | `0x1096dc` |
| `ldr` at 0xba344 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba348 | load | 4 | `(unknown)` | `-` |
| `str` at 0xba34c | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0xba350 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xba354 | store | 4 | `sp -0x34` | `[sp-26]:1` |
| `ldr` at 0xba358 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xba35c | load | 1 | `(unknown)` | `-` |
| `str` at 0xba360 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xba368 | load | 4 | `abs 0xba38c` | `-` |
| `ldr` at 0xba378 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xba37c | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xba380 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xba384 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba384 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba384 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba278 | `(unknown callee) 0xabc4c` | `?` | `?` | `0x3` | `sp-26` |
| `bl` at 0xba294 | `(unknown callee) 0x1ea50` | `ret(0xabc4c)` | `?` | `?` | `ret(0xabc4c)` |
| `bl` at 0xba2c4 | `(unknown callee) 0x1eed0` | `0x3` | `0x1096c0` | `0x68` | `0x0` |
| `bl` at 0xba308 | `(unknown callee) 0xb8014` | `[sp-20]` | `[sp-24]` | `[sp-25]:1` | `[sp-26]:1` |
| `bl` at 0xba374 | `(unknown callee) 0x1eed0` | `0x3` | `0x1096c0` | `0x73` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xba24c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xba28c | `ret(0xabc4c) == 0x0` | `ret(0xabc4c) != 0x0` |
| `beq` at 0xba2ec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba384 | `r0` | `[sp-32]` |

## `eeprom_read_pirom_stub`

- `eeprom_read_pirom_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac05c to 0xac084.
  Established. A word store at sp -0x18 preserves entry r1, and a byte store at sp -0x11 preserves
  `(r0 & 0xff)` (`str` at 0xac070, `strb` at 0xac074). The analyzer leaves the final r0 value
  unknown (`ldmdb` at 0xac080). The PIROM-read role suggested by the symbol is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac070 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xac074 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0xac080 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac080 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac080 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac080 | `r0` | `?` |
