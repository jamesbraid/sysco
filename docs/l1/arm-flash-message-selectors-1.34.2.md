# ARM flash commands and message selectors in 1.34.2

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
cmdFlashBoot, cmdFlashSetDefault and SdrDecode each report an unresolved computed transfer. Their excluded destinations do not supply summary facts. Other routines report no unresolved computed transfers. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/flash-message-selectors-facts.txt) retain all tool sections.

## `IcmbMsgGetSeqNbr`

- `IcmbMsgGetSeqNbr`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9483c to 0x9486c.
  Established. A byte load reads entry `r0 +0x0` (`ldrb` at 0x94854). A later byte store at `sp
  -0x15` has unknown value (`strb` at 0x9485c). The return reports `[sp-21]:1` (`ldmdb` at 0x94868).
  No calls or conditional branches are reported, and sequence-number encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9484c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94850 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x94854 | load | 1 | `r0 +0x0` | `-` |
| `strb` at 0x9485c | store | 1 | `sp -0x15` | `?` |
| `ldrb` at 0x94860 | load | 1 | `sp -0x15` | `-` |
| `ldmdb` at 0x94868 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x94868 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x94868 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x94868 | `r0` | `[sp-21]:1` |

## `IcmbMsgGetCmd`

- `IcmbMsgGetCmd`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x948b8 to 0x948ec.
  Established. The byte read at entry `r0 +0x1` supplies word `[r0+1]:1` to `sp -0x18` (`ldrb` at
  0x948d8, `str` at 0x948dc). That value is moved to r0 and reaches the return (`mov` at 0x948e4,
  `ldmdb` at 0x948e8). The fact sheet reports no calls or conditional branches. Command-code
  semantics are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x948c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x948d0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x948d8 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0x948dc | store | 4 | `sp -0x18` | `[r0+1]:1` |
| `ldr` at 0x948e0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x948e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x948e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x948e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x948e8 | `r0` | `[r0+1]:1` |

## `IcmbMsgGetMsgType`

- `IcmbMsgGetMsgType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x947b0 to 0x947e8.
  Established. A byte load addresses entry `r0 +0x0` (`ldrb` at 0x947c8). The later word store at
  `sp -0x18` has unknown value (`str` at 0x947d8). r0 returns `[sp-24]` (`ldmdb` at 0x947e4). No
  calls are reported. The message-type meaning suggested by the name is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x947c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x947c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x947c8 | load | 1 | `r0 +0x0` | `-` |
| `str` at 0x947d8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x947dc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x947e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x947e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x947e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x947e4 | `r0` | `[sp-24]` |

## `IcmbMsgGetData`

- `IcmbMsgGetData`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x94950 to 0x949a8.
  Established. The only call targets `0x947b0` with all four entry register candidates (`bl` at
  0x94968); its nonzero result takes the path to word `[sp-20]+3` at `sp -0x1c`, while zero falls
  through to `[sp-20]+2` there (`bne` at 0x9497c, `str` at 0x94998, `str` at 0x94988). The
  return reports `[sp-28]` (`ldmdb` at 0x949a4). The selected data’s protocol role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x94960 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94964 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x94970 | store | 4 | `sp -0x18` | `ret(0x947b0)` |
| `ldr` at 0x94974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x94980 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x94988 | store | 4 | `sp -0x1c` | `[sp-20]+2` |
| `ldr` at 0x94990 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x94998 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0x9499c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x949a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x949a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x949a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x94968 | `(unknown callee) 0x947b0` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9497c | `ret(0x947b0) != 0x0` | `ret(0x947b0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x949a4 | `r0` | `[sp-28]` |

## `IcmbMsgGetCmpCode`

- `IcmbMsgGetCmpCode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9491c to 0x94950.
  Established. The byte at entry `r0 +0x2` is loaded and saved as word `[r0+2]:1` at `sp -0x18`
  (`ldrb` at 0x9493c, `str` at 0x94940). The return reports `[r0+2]:1` (`ldmdb` at 0x9494c). Calls
  and conditional branches are absent from the facts. Completion-status interpretation is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9492c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x94934 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9493c | load | 1 | `r0 +0x2` | `-` |
| `str` at 0x94940 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0x94944 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x9494c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9494c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9494c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9494c | `r0` | `[r0+2]:1` |

## `brickGetFlashType`

- `brickGetFlashType`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2b404 to 0x2b424.
  Established. A byte load reads absolute `0x119c0d` (`ldrb` at 0x2b414); its value moves to r0 and
  is returned as `[0x119c0d]:1` (`mov` at 0x2b418, `ldmdb` at 0x2b41c). There are no reported calls
  or explicit stores. The byte’s flash-type mapping is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2b410 | load | 4 | `abs 0x2b420` | `-` |
| `ldrb` at 0x2b414 | load | 1 | `abs 0x119c0d` | `-` |
| `ldmdb` at 0x2b41c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2b41c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2b41c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2b41c | `r0` | `[0x119c0d]:1` |

## `SHUB_MMR_Read`

- `SHUB_MMR_Read`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x247f4 to 0x24858.
  Established. The call to `0xc97a8` carries unknown, entry r1, entry r2 and entry r3 (`bl` at
  0x2482c); its nonzero result takes the path to word `0x0` at `sp -0x24`, while zero falls through
  to word `0x1` there (`bne` at 0x24838, `str` at 0x2484c, `str` at 0x24840). The return reports
  `[sp-36]` (`ldmdb` at 0x24854). Physical register-read semantics and status meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24804 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24808 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2480c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x24810 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x24814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24828 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x24840 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x2484c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x24850 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x24854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2482c | `(unknown callee) 0xc97a8` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x24838 | `ret(0xc97a8) != 0x0` | `ret(0xc97a8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24854 | `r0` | `[sp-36]` |

## `SHUB_MMR_Write`

- `SHUB_MMR_Write`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24790 to 0x247f4.
  Established. All four entry registers are saved as words at `sp -0x14`, `sp -0x18`, `sp -0x1c`,
  `sp -0x20` (`str` at 0x247a0, `str` at 0x247a4, `str` at 0x247a8, `str` at 0x247ac). The only call
  targets `0xc98ac` with unknown, entry r1, entry r2, entry r3 (`bl` at 0x247c8). A nonzero result
  takes the path to word `0x0` at `sp -0x24` and zero falls through to `0x1` (`bne` at
  0x247d4, `str` at 0x247e8, `str` at 0x247dc). Physical write behavior remains unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x247a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x247a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x247a8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x247ac | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x247b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x247bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x247c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x247c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x247dc | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x247e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x247ec | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x247f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x247f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x247f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x247c8 | `(unknown callee) 0xc98ac` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x247d4 | `ret(0xc98ac) != 0x0` | `ret(0xc98ac) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x247f0 | `r0` | `[sp-36]` |

## `ipmiInit`

- `ipmiInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ed58 to 0x1ed68.
  Established. The prologue saves registers and the return restores them (`push` at 0x1ed5c, `ldmdb`
  at 0x1ed64). The reported r0 return retains entry r0 (`ldmdb` at 0x1ed64). No calls, explicit
  stores or conditional branches are reported. An initialization effect on a controller is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x1ed64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x1ed64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x1ed64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x1ed64 | `r0` | `r0` |

## `cmdFlashVersion`

- `cmdFlashVersion`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8c398 to 0x8c5b4.
  Established. The comparison takes its edge for `[0x284070] != [0x284068]` and falls through on
  equality (`bne` at 0x8c478); its paths store `[0x28406c]` or `[0x284068]` at `sp -0x6c`,
  respectively (`str` at 0x8c494, `str` at 0x8c484). The call to `0xa913c` uses `[sp-48]`, `sp-96`,
  unknown, `sp-96` (`bl` at 0x8c4c0). A later comparison with the same predicates selects word
  `0x42` or `0x41` at `sp -0x74` (`bne` at 0x8c514, `str` at 0x8c528, `str` at 0x8c51c). The return
  reports `0x0`, with version-field interpretation unknown (`ldmdb` at 0x8c59c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8c3a8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8c3ac | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x8c3b0 | load | 4 | `abs 0x8c5a0` | `-` |
| `ldrb` at 0x8c3b4 | load | 1 | `abs 0xfb1f0` | `-` |
| `strb` at 0x8c3b8 | store | 1 | `sp -0x2c` | `([0xfb1f0]:1 & 0xff)` |
| `strb` at 0x8c3c4 | store | 1 | `sp -0x2b` | `0x0` |
| `strb` at 0x8c3d0 | store | 1 | `sp -0x2a` | `0x0` |
| `strb` at 0x8c3dc | store | 1 | `sp -0x29` | `0x0` |
| `strb` at 0x8c3e8 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0x8c3f4 | store | 1 | `sp -0x27` | `0x0` |
| `strb` at 0x8c400 | store | 1 | `sp -0x26` | `0x0` |
| `strb` at 0x8c40c | store | 1 | `sp -0x25` | `0x0` |
| `strb` at 0x8c418 | store | 1 | `sp -0x24` | `0x0` |
| `strb` at 0x8c424 | store | 1 | `sp -0x23` | `0x0` |
| `strb` at 0x8c430 | store | 1 | `sp -0x22` | `0x0` |
| `strb` at 0x8c43c | store | 1 | `sp -0x21` | `0x0` |
| `strb` at 0x8c448 | store | 1 | `sp -0x20` | `0x0` |
| `strb` at 0x8c454 | store | 1 | `sp -0x1f` | `0x0` |
| `strb` at 0x8c460 | store | 1 | `sp -0x1e` | `0x0` |
| `ldr` at 0x8c464 | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c468 | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c46c | load | 4 | `abs 0x284070` | `-` |
| `ldr` at 0x8c470 | load | 4 | `abs 0x284068` | `-` |
| `ldr` at 0x8c47c | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c480 | load | 4 | `abs 0x284068` | `-` |
| `str` at 0x8c484 | store | 4 | `sp -0x6c` | `[0x284068]` |
| `ldr` at 0x8c48c | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c490 | load | 4 | `abs 0x28406c` | `-` |
| `str` at 0x8c494 | store | 4 | `sp -0x6c` | `[0x28406c]` |
| `ldr` at 0x8c498 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x8c49c | store | 4 | `sp -0x30` | `[sp-108]` |
| `ldr` at 0x8c4a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8c4b0 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0x8c4b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8c4d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8c4d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8c4d8 | load | 4 | `abs 0x8c5a8` | `-` |
| `ldr` at 0x8c4dc | load | 4 | `abs 0x8c5ac` | `-` |
| `str` at 0x8c4e8 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x8c4f0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x8c4f4 | store | 4 | `sp -0x64` | `[sp-80]` |
| `ldr` at 0x8c4f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8c4fc | store | 4 | `sp -0x70` | `[sp-28]` |
| `ldr` at 0x8c500 | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c504 | load | 4 | `abs 0x8c5a4` | `-` |
| `ldr` at 0x8c508 | load | 4 | `abs 0x284070` | `-` |
| `ldr` at 0x8c50c | load | 4 | `abs 0x284068` | `-` |
| `str` at 0x8c51c | store | 4 | `sp -0x74` | `0x41` |
| `str` at 0x8c528 | store | 4 | `sp -0x74` | `0x42` |
| `ldr` at 0x8c52c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c534 | store | 4 | `sp -0x90` | `[sp-80]+4` |
| `ldr` at 0x8c538 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c540 | store | 4 | `sp -0x8c` | `[sp-80]+6` |
| `ldr` at 0x8c544 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c548 | store | 4 | `sp -0x88` | `[sp-80]` |
| `ldr` at 0x8c54c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c554 | store | 4 | `sp -0x84` | `[sp-80]+8` |
| `ldr` at 0x8c558 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c560 | store | 4 | `sp -0x80` | `[sp-80]+10` |
| `ldr` at 0x8c564 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8c56c | store | 4 | `sp -0x7c` | `[sp-80]+12` |
| `ldr` at 0x8c570 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8c574 | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x8c578 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x8c57c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8c580 | load | 4 | `abs 0x8c5b0` | `-` |
| `ldr` at 0x8c584 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8c588 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x8c594 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x8c598 | load | 4 | `sp -0x68` | `-` |
| `ldmdb` at 0x8c59c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8c59c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8c59c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8c59c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8c4a8 | `(unknown callee) 0x18920` | `0x80` | `[0x284070]` | `0x284060` | `[sp-108]` |
| `bl` at 0x8c4c0 | `(unknown callee) 0xa913c` | `[sp-48]` | `sp-96` | `?` | `sp-96` |
| `bl` at 0x8c4e0 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0xfb350` | `0xfb384` | `[sp-28]` |
| `bl` at 0x8c58c | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0xfb38c` | `[sp-76]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8c478 | `[0x284070] != [0x284068]` | `[0x284070] == [0x284068]` |
| `beq` at 0x8c4cc | `ret(0xa913c) == 0x0` | `ret(0xa913c) != 0x0` |
| `bne` at 0x8c514 | `[0x284070] != [0x284068]` | `[0x284070] == [0x284068]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8c59c | `r0` | `0x0` |

## `cmdFlashBoot`

- `cmdFlashBoot`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8bc3c to 0x8c398.
  Established. The opening call to `0x18920` carries `0x300`, entry r1, entry r2, `[[r0+112]]` (`bl`
  at 0x8bc6c); its result is stored as a word at entry `r1 +0x0`, and a byte store then clears
  `[[sp-28]] +0x0` (`str` at 0x8bc74, `strb` at 0x8bc84). The later call to `0x8c7fc` has `0x64`,
  unknown, `[[sp-28]]`, `[sp-36]` (`bl` at 0x8bc98). The computed transfer takes an unresolved edge
  for `[sp-36]+3 <= 0xa` unsigned and falls through above it (`ldrls` at 0x8bca8). Excluded
  destinations’ boot actions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8bc4c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8bc50 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x8bc54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8bc58 | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0x8bc5c | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0x8bc60 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `ldr` at 0x8bc64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8bc74 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0x8bc78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8bc7c | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x8bc84 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8bc88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8bc9c | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0x8bca8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x8c340 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8c340 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8c340 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8c340 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8bc6c | `(unknown callee) 0x18920` | `0x300` | `r1` | `r2` | `[[r0+112]]` |
| `bl` at 0x8bc98 | `(unknown callee) 0x8c7fc` | `0x64` | `?` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x8bc90 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ldrls` at 0x8bca8 | `[sp-36]+3 <= 0xa (unsigned, 32-bit)` | `[sp-36]+3 > 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8c340 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8bca8).

## `cmdFlashSetDefault`

- `cmdFlashSetDefault`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8c5b4 to 0x8c7fc.
  Established. A call to `0x18920` has `0x80`, entry r1, entry r2, `0x0` (`bl` at 0x8c5ec). The
  computed transfer takes an unresolved edge for `[sp-36]+1 <= 0x5` unsigned and falls through above
  that bound (`ldrls` at 0x8c614). The later known-edge call to `0xaf620` uses `[sp-40]`, unknown,
  `[[sp-28]]`, `[sp-36]+1` (`bl` at 0x8c790). A zero result takes the continuation and nonzero falls
  through to `0x1e71c` (`beq` at 0x8c7a4, `bl` at 0x8c7ac). Default-selection and physical flash
  effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8c5c4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8c5c8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x8c5cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8c5d0 | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0x8c5d4 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0x8c5d8 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `str` at 0x8c5e0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8c5e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8c5f4 | store | 4 | `r1 +0x0` | `ret(0x18920)` |
| `ldr` at 0x8c5f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8c5fc | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x8c604 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x8c608 | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0x8c614 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8c78c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8c798 | store | 4 | `sp -0x20` | `ret(0xaf620)` |
| `ldr` at 0x8c79c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8c7a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8c7b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8c7b8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8c7bc | load | 4 | `abs 0x8c7f8` | `-` |
| `str` at 0x8c7c8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x8c7d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8c7d4 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x8c7dc | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `str` at 0x8c7e4 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x8c7e8 | load | 4 | `sp -0x90` | `-` |
| `ldmdb` at 0x8c7ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8c7ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8c7ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8c7ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8c5ec | `(unknown callee) 0x18920` | `0x80` | `r1` | `r2` | `0x0` |
| `bl` at 0x8c790 | `(unknown callee) 0xaf620` | `[sp-40]` | `?` | `[[sp-28]]` | `[sp-36]+1` |
| `bl` at 0x8c7ac | `(unknown callee) 0x1e71c` | `ret(0xaf620)` | `?` | `?` | `ret(0xaf620)` |
| `bl` at 0x8c7c0 | `(unknown callee) 0x18c74` | `[[sp-28]]` | `0xfb3fc` | `ret(0x1e71c)` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x8c614 | `[sp-36]+1 <= 0x5 (unsigned, 32-bit)` | `[sp-36]+1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0x8c7a4 | `ret(0xaf620) == 0x0` | `ret(0xaf620) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8c7ec | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8c614).

## `FlashValidateImage`

- `FlashValidateImage`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8acc8 to 0x8b12c.
  Established. An absolute byte store clears `0x11b270` (`strb` at 0x8ad00). The first unsigned
  range comparison takes its edge for `ret(0x1874c) < [sp-24]` and falls through for
  greater-or-equal (`blo` at 0x8ad40). A second takes its edge for `ret(0x1874c) > ([sp-24] +
  [sp-28])` and falls through otherwise (`bhi` at 0x8ad58). Later word stores include `[sp-60]` at
  `[sp+4] +0x0`, `[sp-64]` at `[sp+8] +0x0`, and `0x11b270` at `[sp] +0x0` (`str` at 0x8aecc, `str`
  at 0x8b088, `str` at 0x8b0a0). The call purposes, accepted image format and physical validation
  effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8acd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8acdc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8ace0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x8ace4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x8ace8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8acec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8acf4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8acf8 | load | 4 | `abs 0x8b0f8` | `-` |
| `strb` at 0x8ad00 | store | 1 | `abs 0x11b270` | `0x0` |
| `ldr` at 0x8ad04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ad10 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8ad14 | load | 4 | `abs 0x8b0fc` | `-` |
| `ldr` at 0x8ad18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ad24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ad28 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x8ad30 | store | 4 | `sp -0x2c` | `ret(0x1874c)` |
| `ldr` at 0x8ad34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ad38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ad44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ad48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ad50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ad60 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8ad64 | load | 4 | `abs 0x8b100` | `-` |
| `ldr` at 0x8ad68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ad74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ad88 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8ad8c | load | 4 | `abs 0x8b104` | `-` |
| `ldr` at 0x8ad90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ad9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ada0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8ada8 | store | 4 | `sp -0x3c` | `([sp-44] + [sp-36])` |
| `ldr` at 0x8adac | load | 4 | `sp -0x3c` | `-` |
| `ldrh` at 0x8adb0 | load | 2 | `([sp-44] + [sp-36]) +0x2` | `-` |
| `str` at 0x8adbc | store | 4 | `sp -0x34` | `ret(0x189f8)` |
| `ldr` at 0x8adc0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8adcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8add0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8add8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8addc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8adf0 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8adf4 | load | 4 | `abs 0x8b108` | `-` |
| `ldr` at 0x8adf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8ae04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ae08 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8ae24 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8ae28 | load | 4 | `abs 0x8b10c` | `-` |
| `ldr` at 0x8ae2c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8ae3c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x8ae44 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8ae48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8ae4c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0x8ae50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8ae54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8ae64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8ae68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ae74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8ae7c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1874c))` |
| `ldr` at 0x8ae80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8ae88 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x8ae8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8ae94 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x8ae9c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8aea8 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8aeac | load | 4 | `abs 0x8b110` | `-` |
| `ldr` at 0x8aeb8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x8aec4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x8aec8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8aecc | store | 4 | `[sp+4] +0x0` | `[sp-60]` |
| `ldr` at 0x8aed0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8aed4 | load | 4 | `[sp-60] +0x18` | `-` |
| `str` at 0x8aedc | store | 4 | `sp -0x2c` | `ret(0x1874c)` |
| `ldr` at 0x8aee0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8aeec | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8aef0 | load | 4 | `abs 0x8b114` | `-` |
| `ldr` at 0x8aefc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8af00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8af08 | store | 4 | `sp -0x40` | `(ret(0x1874c) + [sp-36])` |
| `ldr` at 0x8af0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8af10 | store | 4 | `sp -0x30` | `[sp-20]` |
| `str` at 0x8af18 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x8af20 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8af24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8af28 | load | 4 | `[sp-60] +0xc` | `-` |
| `ldr` at 0x8af34 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8af44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8af48 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x8af54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8af5c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1874c))` |
| `ldr` at 0x8af60 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8af68 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x8af6c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8af74 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x8af7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8af80 | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0x8af8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8af98 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8af9c | load | 4 | `[sp-60] +0x20` | `-` |
| `ldr` at 0x8afb0 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8afb4 | load | 4 | `abs 0x8b118` | `-` |
| `str` at 0x8afc0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x8afc8 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8afcc | load | 4 | `abs 0x8b11c` | `-` |
| `ldr` at 0x8afd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8afe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8afe8 | load | 4 | `[sp-64] +0x4` | `-` |
| `str` at 0x8aff0 | store | 4 | `sp -0x34` | `ret(0x1874c)` |
| `str` at 0x8aff8 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x8b000 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8b004 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8b008 | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0x8b00c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8b010 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8b020 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8b024 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8b030 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8b038 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1874c))` |
| `ldr` at 0x8b03c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8b044 | store | 4 | `sp -0x38` | `[sp-56]+4` |
| `ldr` at 0x8b048 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8b050 | store | 4 | `sp -0x30` | `[sp-48]+4` |
| `ldr` at 0x8b058 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8b064 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8b068 | load | 4 | `abs 0x8b120` | `-` |
| `ldr` at 0x8b074 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x8b080 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x8b084 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8b088 | store | 4 | `[sp+8] +0x0` | `[sp-64]` |
| `ldr` at 0x8b08c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x8b098 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x8b09c | load | 4 | `abs 0x8b0f8` | `-` |
| `str` at 0x8b0a0 | store | 4 | `[sp] +0x0` | `0x11b270` |
| `str` at 0x8b0a8 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x8b0b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8b0bc | load | 4 | `abs 0x8b124` | `-` |
| `ldr` at 0x8b0c0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x8b0c4 | load | 4 | `abs 0x8b128` | `-` |
| `ldr` at 0x8b0c8 | load | 4 | `abs 0x8b0f8` | `-` |
| `ldr` at 0x8b0d0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x8b0dc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x8b0e0 | load | 4 | `abs 0x8b0f8` | `-` |
| `str` at 0x8b0e4 | store | 4 | `[sp] +0x0` | `0x11b270` |
| `str` at 0x8b0ec | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x8b0f0 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0x8b0f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8b0f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8b0f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8ad1c | `(unknown callee) 0x18c74` | `0x11b270` | `0xfae94` | `[sp-20]` | `[sp-20]` |
| `bl` at 0x8ad2c | `(unknown callee) 0x1874c` | `[[sp-20]]` | `r1` | `0x11b270` | `[sp-20]` |
| `bl` at 0x8ad6c | `(unknown callee) 0x18c74` | `0x11b270` | `0xfaeb4` | `ret(0x1874c)` | `?` |
| `bl` at 0x8ad78 | `(unknown callee) 0xaed2c` | `ret(0x1874c)` | `?` | `([sp-24] + [sp-28])` | `ret(0x1874c)` |
| `bl` at 0x8ad94 | `(unknown callee) 0x18c74` | `0x11b270` | `0xfaed4` | `[sp-44]` | `ret(0xaed2c)` |
| `bl` at 0x8adb8 | `(unknown callee) 0x189f8` | `[([sp-44] + [sp-36])+2]:2` | `?` | `[sp-44]` | `[([sp-44] + [sp-36])+2]:2` |
| `bl` at 0x8adfc | `(unknown callee) 0x18c74` | `0x11b270` | `0xfaf10` | `ret(0x189f8)` | `?` |
| `bl` at 0x8ae14 | `(unknown callee) 0xaed2c` | `([sp-44] + ret(0x189f8))` | `([sp-44] + ret(0x189f8))` | `[sp-44]` | `([sp-44] + ret(0x189f8))` |
| `bl` at 0x8ae30 | `(unknown callee) 0x18c74` | `0x11b270` | `0xfaf34` | `[sp-52]` | `ret(0xaed2c)` |
| `bl` at 0x8ae6c | `(unknown callee) 0x1874c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8aeb0 | `(unknown callee) 0x18cd4` | `0x11b270` | `0xfaf7c` | `?` | `?` |
| `bl` at 0x8aed8 | `(unknown callee) 0x1874c` | `[[sp-60]+24]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x8aef4 | `(unknown callee) 0x18cd4` | `0x11b270` | `0xfaf98` | `?` | `ret(0x1874c)` |
| `bl` at 0x8af2c | `(unknown callee) 0x1874c` | `[[sp-60]+12]` | `?` | `ret(0x1874c)` | `[sp-60]` |
| `bl` at 0x8af4c | `(unknown callee) 0x1874c` | `[[sp-48]]` | `?` | `ret(0x1874c)` | `[sp-48]` |
| `bl` at 0x8af84 | `(unknown callee) 0x1874c` | `[[sp-60]+32]` | `?` | `ret(0x1874c)` | `[sp-60]` |
| `bl` at 0x8afa0 | `(unknown callee) 0x1874c` | `[[sp-60]+32]` | `?` | `ret(0x1874c)` | `[sp-60]` |
| `bl` at 0x8afb8 | `(unknown callee) 0x18cd4` | `0x11b270` | `0xfafb4` | `?` | `ret(0x1874c)` |
| `bl` at 0x8afd0 | `(unknown callee) 0x18cd4` | `0x11b270` | `0xfafc8` | `?` | `ret(0x1874c)` |
| `bl` at 0x8afec | `(unknown callee) 0x1874c` | `[[sp-64]+4]` | `?` | `?` | `[sp-64]` |
| `bl` at 0x8b028 | `(unknown callee) 0x1874c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8b06c | `(unknown callee) 0x18cd4` | `0x11b270` | `0xfafdc` | `?` | `?` |
| `bl` at 0x8b0cc | `(unknown callee) 0x21010` | `[0x116928]` | `0xfaffc` | `0x11b270` | `0x116928` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8ad0c | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x8ad40 | `ret(0x1874c) < [sp-24] (unsigned, 32-bit)` | `ret(0x1874c) >= [sp-24] (unsigned, 32-bit)` |
| `bhi` at 0x8ad58 | `ret(0x1874c) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `ret(0x1874c) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0x8ad84 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x8adc8 | `ret(0x189f8) <= 0x1f (unsigned, 32-bit)` | `ret(0x189f8) > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0x8ade8 | `([sp-44] + ret(0x189f8)) > ([sp-24] + [sp-28]) (unsigned, 32-bit)` | `([sp-44] + ret(0x189f8)) <= ([sp-24] + [sp-28]) (unsigned, 32-bit)` |
| `bne` at 0x8ae20 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0x8ae5c | `C == 0` | `not (C == 0)` |
| `beq` at 0x8aea4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8aec0 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `bne` at 0x8aee8 | `ret(0x1874c) != 0x0` | `ret(0x1874c) == 0x0` |
| `blt` at 0x8af3c | `[sp-56] < ret(0x1874c) (signed, 32-bit)` | `[sp-56] >= ret(0x1874c) (signed, 32-bit)` |
| `beq` at 0x8af94 | `[sp-40] == ret(0x1874c)` | `[sp-40] != ret(0x1874c)` |
| `bne` at 0x8afac | `ret(0x1874c) != 0x0` | `ret(0x1874c) == 0x0` |
| `beq` at 0x8afe0 | `[sp-64] == 0x0` | `[sp-64] != 0x0` |
| `blo` at 0x8b018 | `C == 0` | `not (C == 0)` |
| `beq` at 0x8b060 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8b07c | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0x8b094 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x8b0b8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0x8b0d8 | `[sp] == 0x0` | `[sp] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8b0f4 | `r0` | `[sp-68]` |

## `SdrDecode`

- `SdrDecode`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x93010 to 0x930c4.
  Established. A byte load reads entry `r0 +0x3`, with `[r0+3]:1` saved as a word at `sp -0x1c`
  (`ldrb` at 0x93030, `str` at 0x93034). The computed transfer takes an unresolved edge for
  `[r0+3]:1-1 <= 0x11` unsigned and falls through for values above it (`ldrls` at 0x93044). The
  known fall-through return retains entry r0 (`ldmdb` at 0x930c0). No calls are reported on known
  edges. Decoding destinations and their data roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x93020 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x93024 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x93028 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x9302c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x93030 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0x93034 | store | 4 | `sp -0x1c` | `[r0+3]:1` |
| `ldr` at 0x93038 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x93044 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x930c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x930c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x930c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x93044 | `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)` | `[r0+3]:1-1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x930c0 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x93044).

## `SdrSetChassisFru`

- `SdrSetChassisFru`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x93e60 to 0x93eb4.
  Established. The initial branch takes its bypass for `[0x115694]:1 != 0xff` and falls through on
  equality (`bne` at 0x93e84). The equality path writes byte `((r0 & 0xff) & 0xff)` at absolute
  `0x115694`, then zeros bytes `0x115695` and `0x115696` (`strb` at 0x93e90, `strb` at 0x93e9c,
  `strb` at 0x93ea8). Entry r0 remains the reported return value (`ldmdb` at 0x93eac). No calls are
  reported, and the three bytes’ chassis-record meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x93e74 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x93e78 | load | 4 | `abs 0x93eb0` | `-` |
| `ldrb` at 0x93e7c | load | 1 | `abs 0x115694` | `-` |
| `ldr` at 0x93e88 | load | 4 | `abs 0x93eb0` | `-` |
| `ldrb` at 0x93e8c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x93e90 | store | 1 | `abs 0x115694` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0x93e94 | load | 4 | `abs 0x93eb0` | `-` |
| `strb` at 0x93e9c | store | 1 | `abs 0x115695` | `0x0` |
| `ldr` at 0x93ea0 | load | 4 | `abs 0x93eb0` | `-` |
| `strb` at 0x93ea8 | store | 1 | `abs 0x115696` | `0x0` |
| `ldmdb` at 0x93eac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x93eac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x93eac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x93e84 | `[0x115694]:1 != 0xff` | `[0x115694]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x93eac | `r0` | `r0` |
