# ARM JTAG master and communication routines in 1.40.5

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
The computed transfers in jmstr_move_to_idle (`ldrls` at 0xdd600) and jmstr_scan
(`ldrls` at 0xdd950) have unresolved targets. Their tables and returns cover known edges only.
Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/jtag-master-facts.txt) retain all tool sections.

## `jmstr_set_functions`

- `jmstr_set_functions`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd300 to 0xdd33c.
  Established. Two absolute four-byte assignments write `r0` at `0x2adfc4` and `[sp-24]` at
  `0x2adfc8` (`str` at 0xdd320, `str` at 0xdd32c). The exit carries entry `r0` (`ldmdb` at 0xdd330).
  This span has no calls or conditional branches. Stored-pointer and callback meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd310 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdd314 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdd318 | load | 4 | `abs 0xdd334` | `-` |
| `ldr` at 0xdd31c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdd320 | store | 4 | `abs 0x2adfc4` | `r0` |
| `ldr` at 0xdd324 | load | 4 | `abs 0xdd338` | `-` |
| `ldr` at 0xdd328 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd32c | store | 4 | `abs 0x2adfc8` | `[sp-24]` |
| `ldmdb` at 0xdd330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd330 | `r0` | `r0` |

## `jmstr_reset`

- `jmstr_reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd33c to 0xdd3f8.
  Established. An unknown-target call has unknown, `0x20`, `sp-40`, `sp-40` (`mov` at 0xdd3b8); its
  nonzero result takes the branch to `0xdd3e0` (`bne` at 0xdd3c4). Included four-byte stack
  assignments at `sp -0x24` retain `0x20`, `0x28` and unknown at distinct sites (`str` at 0xdd364,
  `str` at 0xdd37c, `str` at 0xdd394). Reset signaling and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd34c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdd350 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdd354 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdd35c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdd364 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdd368 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd374 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd37c | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xdd380 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd38c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd394 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdd398 | load | 4 | `abs 0xdd3f0` | `-` |
| `ldr` at 0xdd3a4 | load | 4 | `abs 0x2adfc8` | `-` |
| `ldr` at 0xdd3a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd3c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd3cc | load | 4 | `abs 0xdd3f4` | `-` |
| `str` at 0xdd3d8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdd3e4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xdd3e8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xdd3ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdd3ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd3ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd3ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdd3b8 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdd3d0 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135e6c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd370 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdd388 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xdd3c4 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd3ec | `r0` | `?` |

## `jmstr_reset_and_idle`

- `jmstr_reset_and_idle`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd3f8 to 0xdd51c.
  Established. Two unknown-target calls each have unknown, `0x20`, `sp-40`, `sp-40` (`mov` at
  0xdd474, `mov` at 0xdd4dc). The first nonzero result takes the branch to `0xdd49c`, followed by
  four-byte `0x10` at `sp -0x24` (`bne` at 0xdd480, `str` at 0xdd4a0). A later four-byte store at
  that stack location retains `0x18` (`str` at 0xdd4b8). Reset-to-idle transitions and physical
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd408 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdd40c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdd410 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdd418 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdd420 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdd424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd430 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd438 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xdd43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd448 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd450 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdd454 | load | 4 | `abs 0xdd514` | `-` |
| `ldr` at 0xdd460 | load | 4 | `abs 0x2adfc8` | `-` |
| `ldr` at 0xdd464 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd484 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd488 | load | 4 | `abs 0xdd518` | `-` |
| `str` at 0xdd494 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdd4a0 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xdd4a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd4b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd4b8 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xdd4bc | load | 4 | `abs 0xdd514` | `-` |
| `ldr` at 0xdd4c8 | load | 4 | `abs 0x2adfc8` | `-` |
| `ldr` at 0xdd4cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd4ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd4f0 | load | 4 | `abs 0xdd518` | `-` |
| `str` at 0xdd4fc | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdd508 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xdd50c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xdd510 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdd510 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd510 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd510 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdd474 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdd48c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135e6c` | `?` | `ret(unknown)` |
| `mov` at 0xdd4dc | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xdd4f4 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135e6c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd42c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdd444 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xdd480 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdd4ac | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xdd4e8 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd510 | `r0` | `?` |

## `jmstr_move_to_idle`

- `jmstr_move_to_idle`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd51c to 0xdd7a0.
  Established. The computed transfer has an unsigned-lower-or-same flags predicate with unknown
  operands and an unresolved target (`ldrls` at 0xdd600); its known fall-through includes four-byte
  `0xff000000` at `sp -0x2c` and `0x5` at `sp -0x38` (`str` at 0xdd694, `str` at 0xdd69c). The
  unknown-target call has unknown, `0x5`, `sp-60`, `sp-44` (`mov` at 0xdd6d8). Selector-specific
  state transitions and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd52c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdd530 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdd534 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xdd538 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xdd540 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xdd548 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xdd54c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd558 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd56c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd584 | store | 4 | `sp -0x30` | `ret(0x1b1b4)` |
| `ldr` at 0xdd588 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdd594 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd598 | load | 4 | `abs 0xdd790` | `-` |
| `str` at 0xdd5a4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdd5ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd5c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd5cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd5dc | store | 4 | `sp -0x28` | `0x10` |
| `ldr` at 0xdd5e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd5ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdd5f4 | store | 4 | `sp -0x28` | `0x18` |
| `ldr` at 0xdd5f8 | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0xdd600 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdd694 | store | 4 | `sp -0x2c` | `0xff000000` |
| `str` at 0xdd69c | store | 4 | `sp -0x38` | `0x5` |
| `ldr` at 0xdd6a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd6ac | load | 4 | `abs 0xdd794` | `-` |
| `str` at 0xdd6bc | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0xdd6c0 | load | 4 | `abs 0x2adfc4` | `-` |
| `ldr` at 0xdd6c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd6c8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdd6e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd6ec | load | 4 | `abs 0xdd798` | `-` |
| `ldr` at 0xdd6f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdd700 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd70c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdd714 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd720 | load | 4 | `abs 0xdd79c` | `-` |
| `ldr` at 0xdd724 | load | 4 | `abs 0x2adfc8` | `-` |
| `ldr` at 0xdd728 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd72c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd730 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdd734 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdd74c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd750 | load | 4 | `abs 0xdd798` | `-` |
| `ldr` at 0xdd758 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd764 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xdd76c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd778 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdd784 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0xdd788 | load | 4 | `sp -0x40` | `-` |
| `ldmdb` at 0xdd78c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdd78c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd78c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd78c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd57c | `(unknown callee) 0x1b1b4` | `?` | `r1` | `?` | `?` |
| `bl` at 0xdd59c | `(unknown callee) 0x1b52c` | `[sp-36]` | `0x135ea0` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdd5d4 | `(unknown callee) 0x1b4cc` | `ret(0x1b1b4)` | `0x0` | `?` | `?` |
| `mov` at 0xdd6d8 | `(unknown callee) (unknown target)` | `?` | `0x5` | `sp-60` | `sp-44` |
| `bl` at 0xdd6f0 | `(unknown callee) 0x1b52c` | `[sp-36]` | `0x135e6c` | `?` | `ret(unknown)` |
| `bl` at 0xdd704 | `(unknown callee) 0x1b490` | `[sp-48]` | `?` | `?` | `[sp-48]` |
| `mov` at 0xdd73c | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0xdd754 | `(unknown callee) 0x1b52c` | `[sp-36]` | `0x135e6c` | `?` | `ret(unknown)` |
| `bl` at 0xdd75c | `(unknown callee) 0x1b490` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xdd77c | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd554 | `r2 == 0x0` | `r2 != 0x0` |
| `bne` at 0xdd590 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `beq` at 0xdd5e8 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xdd600 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xdd6a8 | `0x5 == 0x0` | `0x5 != 0x0` |
| `bne` at 0xdd6e4 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdd6fc | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xdd71c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xdd748 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xdd774 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd78c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xdd600).

## `jmstr_run_tck`

- `jmstr_run_tck`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd7a0 to 0xdd844.
  Established. Zero entry `r1` takes the branch to `0xdd82c` (`beq` at 0xdd7e4). The other path
  includes an unknown-target call with unknown, `r1`, `0x0`, `0x0` (`mov` at 0xdd804); its nonzero
  result takes a later branch to `0xdd82c` (`bne` at 0xdd810). Four-byte stack assignments at `sp -0x20` retain `0x10` and `0x18` at separate sites (`str` at 0xdd7c0, `str` at 0xdd7d8).
  Clock-generation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd7b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdd7b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdd7b8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdd7c0 | store | 4 | `sp -0x20` | `0x10` |
| `ldr` at 0xdd7c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd7d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdd7d8 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0xdd7dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd7e8 | load | 4 | `abs 0xdd83c` | `-` |
| `ldr` at 0xdd7ec | load | 4 | `abs 0x2adfc8` | `-` |
| `ldr` at 0xdd7f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd818 | load | 4 | `abs 0xdd840` | `-` |
| `str` at 0xdd824 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xdd830 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xdd834 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xdd838 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd838 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdd804 | `(unknown callee) (unknown target)` | `?` | `r1` | `0x0` | `0x0` |
| `bl` at 0xdd81c | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x135e6c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd7cc | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xdd7e4 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xdd810 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd838 | `r0` | `?` |

## `jmstr_custom_seq`

- `jmstr_custom_seq`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd844 to 0xdd8e8.
  Established. A four-byte load reads entry `sp +0x0`, with that value stored at `sp -0x2c` (`ldr`
  at 0xdd888, `str` at 0xdd88c). The unknown-target call has unknown, `r1`, `r2`, `r3` (`mov` at
  0xdd8a8). Nonzero result takes the branch to `0xdd8d0`, where a four-byte stack assignment retains
  `0x1` at `sp -0x28` (`bne` at 0xdd8b4, `str` at 0xdd8d4). Custom scan-sequence effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd854 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdd858 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdd85c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdd860 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xdd868 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xdd86c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd878 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd880 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xdd884 | load | 4 | `abs 0xdd8e0` | `-` |
| `ldr` at 0xdd888 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdd88c | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0xdd890 | load | 4 | `abs 0x2adfc4` | `-` |
| `ldr` at 0xdd894 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdd898 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdd89c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdd8a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdd8b8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xdd8bc | load | 4 | `abs 0xdd8e4` | `-` |
| `str` at 0xdd8c8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xdd8d4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xdd8d8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xdd8dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdd8dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdd8dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xdd8a8 | `(unknown callee) (unknown target)` | `?` | `r1` | `r2` | `r3` |
| `bl` at 0xdd8c0 | `(unknown callee) 0x1b52c` | `[sp+4]` | `0x135e6c` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdd874 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xdd8b4 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdd8dc | `r0` | `?` |

## `jmstr_scan`

- `jmstr_scan`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd8e8 to 0xdde78.
  Established. Nonzero `[sp]` takes the branch to `0xdd928`, while zero falls through (`bne` at
  0xdd918). The computed transfer takes an unresolved edge for entry `r1 <= 0xd`, unsigned 32-bit,
  and falls through for `r1 > 0xd` (`ldrls` at 0xdd950). The known fall-through calls unnamed
  `0x1b598` with `[sp+12]`, `0x135ec4`, `r2`, `r1` (`bl` at 0xddab0). Selector-specific scan actions
  and state meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd8f8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xdd8fc | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xdd900 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xdd904 | store | 4 | `sp -0x28` | `r3` |
| `str` at 0xdd90c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdd910 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xdd920 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xdd92c | store | 4 | `sp -0x34` | `0x10` |
| `ldr` at 0xdd930 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdd93c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdd944 | store | 4 | `sp -0x34` | `0x18` |
| `ldr` at 0xdd948 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xdd950 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xddaa8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xddaac | load | 4 | `abs 0xdde68` | `-` |
| `str` at 0xddab8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdde60 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xdde64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xdde64 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdde64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdde64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdde64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xddab0 | `(unknown callee) 0x1b598` | `[sp+12]` | `0x135ec4` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdd918 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xdd938 | `r3 == 0x0` | `r3 != 0x0` |
| `ldrls` at 0xdd950 | `r1 <= 0xd (unsigned, 32-bit)` | `r1 > 0xd (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdde64 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xdd950).

## `jmstr_sel_sic`

- `jmstr_sel_sic`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdde78 to 0xdfd48.
  Established. Two unnamed `0x1b1b4` calls each have first candidate `0x60`, with their results
  assigned as four bytes at `sp -0x40` and `sp -0x44` (`bl` at 0xdde9c, `bl` at 0xdded0, `str` at
  0xddea4, `str` at 0xdded8). Nonzero first result takes the branch to `0xddecc` (`bne` at 0xddeb0).
  A later equality branch takes `[[sp-68]] == 0x54007ff5` to `0xdeaa4` (`beq` at 0xdea70).
  SIC-selection effects and the comparison value meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdde88 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdde8c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdde90 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdde94 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xddea4 | store | 4 | `sp -0x40` | `ret(0x1b1b4)` |
| `ldr` at 0xddea8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddeb4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xddeb8 | load | 4 | `abs 0xdee58` | `-` |
| `str` at 0xddec4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdded8 | store | 4 | `sp -0x44` | `ret(0x1b1b4)` |
| `ldr` at 0xddedc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddee8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xddeec | load | 4 | `abs 0xdee58` | `-` |
| `ldr` at 0xddef4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddf00 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xddf08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xddf18 | load | 4 | `abs 0xdee5c` | `-` |
| `str` at 0xddf1c | store | 4 | `sp -0x28` | `0xff800000` |
| `str` at 0xddf24 | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0xddf2c | load | 4 | `abs 0xdee60` | `-` |
| `str` at 0xddf30 | store | 4 | `sp -0x28` | `0xfff00000` |
| `ldr` at 0xddf34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xddf38 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xddf3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddf40 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xddf4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddf54 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xddf60 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xddf64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddf68 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xddf74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddf7c | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xddf88 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xddf8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddf90 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xddf9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddfa4 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xddfb0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xddfb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddfc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xddfd8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xddfdc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xddfec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xde000 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xde004 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde010 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde018 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xde020 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xde024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde038 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde03c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde050 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde058 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde064 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde070 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde074 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde088 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde090 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde09c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde0a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde0b8 | store | 4 | `sp -0x24` | `0x9` |
| `ldr` at 0xde0bc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde0c4 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `str` at 0xde0d0 | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xde0d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde0dc | store | 4 | `[sp-64] +0x0` | `0x4` |
| `ldr` at 0xde0e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde0e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde0e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde0ec | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xde0f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde0fc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde104 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xde110 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde114 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde118 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde11c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde120 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xde12c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde130 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde138 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xde144 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde148 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde14c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde150 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde154 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xde160 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde164 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde16c | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xde178 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde17c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde180 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde184 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde194 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde198 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde1ac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde1b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1b8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde1c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1cc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde1e0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde1e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde1ec | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde1f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde1f4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde1f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde1fc | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde200 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde204 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde208 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde20c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde21c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde230 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde238 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde244 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde24c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde250 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde254 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde260 | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xde268 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xde26c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde270 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xde274 | store | 4 | `[sp-64] +0x0` | `[sp-24]` |
| `ldr` at 0xde278 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde280 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde284 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde288 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde28c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde290 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xde29c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2a0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde2a8 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xde2b4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde2b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde2c4 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xde2d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2d4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde2dc | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xde2e8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde2ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde2f8 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xde304 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde308 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde310 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xde31c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde324 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde328 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde33c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde350 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde354 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde358 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde35c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde36c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde370 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde384 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde388 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde38c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde390 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde394 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde398 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde39c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde3a0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde3a4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde3b4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde3c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde3dc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde3e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde3e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde3ec | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde3f4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde3f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xde408 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde41c | store | 4 | `sp -0x4c` | `0x9` |
| `str` at 0xde428 | store | 4 | `sp -0x4c` | `0xc` |
| `ldr` at 0xde42c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xde430 | store | 4 | `sp -0x24` | `[sp-76]` |
| `ldr` at 0xde434 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde43c | store | 4 | `[sp-64] +0x0` | `0xe` |
| `ldr` at 0xde440 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde448 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde44c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde450 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde454 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde458 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xde464 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde468 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde470 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xde47c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde480 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde484 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde488 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde48c | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xde498 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde49c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde4a4 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xde4b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde4b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde4b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde4bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde4c0 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xde4cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde4d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde4d8 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xde4e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde4e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde4ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde4f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde500 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde504 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde518 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde51c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde520 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde524 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde534 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde538 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde54c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde550 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde554 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde558 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde55c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde560 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde564 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde568 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde56c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde57c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde590 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde598 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde5a4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde5ac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde5b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde5b4 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde5c0 | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xde5c8 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xde5cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5d0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xde5d4 | store | 4 | `[sp-64] +0x0` | `[sp]` |
| `ldr` at 0xde5d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde5e4 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xde5f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde5fc | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xde608 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde60c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde610 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde614 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde618 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xde624 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde628 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde630 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xde63c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde640 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde644 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde648 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde64c | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xde658 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde65c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde664 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xde670 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde674 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde678 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde67c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde68c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde690 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde6a4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde6a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde6ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde6b0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde6c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde6c4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde6d8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde6e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde6ec | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xde6f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde6f8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde700 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde70c | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xde710 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde714 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde718 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde71c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde720 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde724 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde728 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde72c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde73c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde750 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde758 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde764 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde76c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xde778 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde77c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde780 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde788 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde790 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde79c | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xde7a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde7b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde7bc | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xde7c8 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xde7cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde7e0 | store | 4 | `sp -0x50` | `0x9` |
| `str` at 0xde7ec | store | 4 | `sp -0x50` | `0xc` |
| `ldr` at 0xde7f0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xde7f4 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0xde7f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7fc | load | 4 | `abs 0xdee64` | `-` |
| `str` at 0xde800 | store | 4 | `[sp-64] +0x0` | `0xfff` |
| `ldr` at 0xde804 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde80c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde810 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde814 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde818 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde81c | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xde828 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde82c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde834 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xde840 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde844 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde848 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde84c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde850 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xde85c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde860 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde868 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xde874 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde878 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde87c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde880 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde884 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xde890 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde894 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde89c | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xde8a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde8ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde8c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde8dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde8e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde8f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8fc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde910 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde914 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde918 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde91c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde920 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde924 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde928 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde92c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde930 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde940 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde954 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde95c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde968 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde970 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde974 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde978 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde984 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde988 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde98c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde990 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde994 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde99c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde9a0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde9a4 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde9a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde9b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde9b8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde9bc | load | 4 | `abs 0xdee74` | `-` |
| `ldr` at 0xde9c0 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde9c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde9d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde9dc | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xde9e8 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xde9ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde9f0 | load | 4 | `abs 0xdee68` | `-` |
| `str` at 0xde9f4 | store | 4 | `[sp-64] +0x0` | `0xa800ffea` |
| `ldr` at 0xde9f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdea00 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdea04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdea08 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdea0c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdea10 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdea14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdea18 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdea1c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdea20 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdea30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdea44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdea4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdea58 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdea60 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea64 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdea68 | load | 4 | `abs 0xdee6c` | `-` |
| `ldr` at 0xdea74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea78 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdea7c | load | 4 | `abs 0xdee70` | `-` |
| `ldr` at 0xdea80 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdea88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdea90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdea9c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdeaa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdeab8 | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xdeabc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdeac4 | store | 4 | `[sp-64] +0x0` | `0x9` |
| `ldr` at 0xdeac8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdead0 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdead4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdead8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeadc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeae0 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdeaec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeaf0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeaf8 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdeb04 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb10 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb14 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdeb20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb24 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb2c | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdeb38 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb40 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb44 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb48 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdeb54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb58 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb60 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdeb6c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb78 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb8c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdeba0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeba4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeba8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdebac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdebbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdebc0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdebd4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdebd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdebdc | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdebe0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdebe4 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdebe8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdebec | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdebf0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdebf4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdec04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdec18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec20 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdec2c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdec34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec38 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec3c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdec48 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdec4c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec50 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec54 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdec58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdec60 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdec64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec68 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdec6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdec78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec7c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdec80 | load | 4 | `abs 0xdee74` | `-` |
| `ldr` at 0xdec84 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdec8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec94 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdeca0 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdecac | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xdecb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecb4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xdecb8 | store | 4 | `[sp-64] +0x0` | `[sp+4]` |
| `ldr` at 0xdecbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecc4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdecc8 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdecd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecd8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdece0 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdecec | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdecf0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecf4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecf8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdecfc | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xded08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded0c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded14 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xded20 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xded24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded2c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded30 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xded3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded40 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded48 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xded54 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xded58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded5c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded60 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded74 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xded88 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xded8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded94 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeda4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeda8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdedbc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdedc0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdedc8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdedcc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdedd0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdedd4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdedd8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdeddc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdede0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdede4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdede8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdedf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdee0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdee14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdee20 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdee28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee2c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee30 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdee38 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdee3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdee50 | store | 4 | `sp -0x54` | `0x9` |
| `str` at 0xdee90 | store | 4 | `sp -0x54` | `0xc` |
| `ldr` at 0xdee94 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xdee98 | store | 4 | `sp -0x24` | `[sp-84]` |
| `ldr` at 0xdee9c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdeea4 | store | 4 | `[sp-64] +0x0` | `0x8` |
| `ldr` at 0xdeea8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdeeb0 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdeeb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeeb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeebc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeec0 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdeecc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeed0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeed8 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdeee4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeee8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeeec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeef0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeef4 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdef00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef04 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdef0c | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdef18 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdef1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef24 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdef28 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdef34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef38 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdef40 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdef4c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdef50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef58 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdef68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef6c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdef80 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdef84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef8c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdef9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdefa0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdefb4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdefb8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdefbc | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdefc0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdefc4 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdefc8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdefcc | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdefd0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdefd4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdefe4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdeff8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf000 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf00c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf014 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf018 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf01c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf028 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf02c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf030 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf034 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf038 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdf040 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf044 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf048 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf04c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf058 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf05c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf060 | load | 4 | `abs 0xdee74` | `-` |
| `ldr` at 0xdf064 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf06c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf074 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf080 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf088 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf09c | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xdf0a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf0ac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0b8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf0bc | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdf0c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0cc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf0d4 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdf0e0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf0e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf0ec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf0f0 | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdf0fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf100 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf108 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdf114 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf118 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf11c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf120 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf124 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdf130 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf134 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf13c | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdf148 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf14c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf150 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf154 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf164 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf168 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf17c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf180 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf184 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf188 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf198 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf19c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf1b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf1b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf1c4 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xdf1c8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf1d0 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdf1d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf1e4 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `str` at 0xdf1f0 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xdf1f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf204 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf208 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf210 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf218 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf21c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdf228 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdf234 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf23c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf248 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xdf24c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf254 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdf25c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf268 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xdf26c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf270 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf274 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf278 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdf284 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf288 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf290 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdf29c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf2a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf2ac | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdf2b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf2c4 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdf2d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf2d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf2e0 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdf2ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf2f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf2f8 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdf304 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf308 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf30c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf310 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf324 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf338 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf33c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf340 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf344 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf354 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf358 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf36c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf370 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf374 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdf378 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf37c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdf380 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf384 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdf388 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdf38c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdf39c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf3b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf3b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf3c4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf3cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdf3d8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf3dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf3e0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf3e8 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf3f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf400 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf408 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xdf414 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xdf418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf428 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdf43c | store | 4 | `sp -0x58` | `0x9` |
| `str` at 0xdf448 | store | 4 | `sp -0x58` | `0xc` |
| `ldr` at 0xdf44c | load | 4 | `sp -0x58` | `-` |
| `str` at 0xdf450 | store | 4 | `sp -0x24` | `[sp-88]` |
| `ldr` at 0xdf454 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf45c | store | 4 | `[sp-64] +0x0` | `0x1` |
| `ldr` at 0xdf460 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf468 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdf46c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf470 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf474 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf478 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdf484 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf488 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf490 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdf49c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf4a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf4ac | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdf4b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf4c4 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdf4d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf4d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf4e0 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdf4ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf4f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf4f8 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdf504 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf508 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf50c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf510 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf520 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf524 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf538 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf53c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf540 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf544 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf554 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf558 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf56c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf570 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf574 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdf578 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf57c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdf580 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf584 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdf588 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdf58c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdf59c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf5b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf5b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf5c4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf5cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf5d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf5d4 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf5d8 | store | 4 | `[sp-68] +0x0` | `[[sp-68]]` |
| `str` at 0xdf5e0 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdf5e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf5ec | store | 4 | `[sp-64] +0x0` | `0x0` |
| `ldr` at 0xdf5f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf5f8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdf5fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf600 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf604 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf608 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdf614 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf618 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf620 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdf62c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf630 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf634 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf638 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf63c | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdf648 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf64c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf654 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdf660 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf664 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf668 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf66c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf670 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdf67c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf680 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf688 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdf694 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf698 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf69c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf6a0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf6b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf6b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf6c8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf6cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf6d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf6d4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdf6e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf6e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdf6fc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdf700 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf704 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdf708 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf70c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdf710 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf714 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdf718 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdf71c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdf72c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf740 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf748 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf754 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf75c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf760 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf764 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf768 | load | 4 | `abs 0xdee78` | `-` |
| `ldr` at 0xdf774 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf778 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf780 | load | 4 | `abs 0xdee78` | `-` |
| `str` at 0xdf78c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf790 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf794 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf798 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf79c | load | 4 | `abs 0xdee7c` | `-` |
| `ldr` at 0xdf7a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf7ac | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf7b4 | load | 4 | `abs 0xdee7c` | `-` |
| `str` at 0xdf7c0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf7c4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf7c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf7cc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf7d0 | load | 4 | `abs 0xdee80` | `-` |
| `ldr` at 0xdf7dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf7e0 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf7e8 | load | 4 | `abs 0xdee80` | `-` |
| `str` at 0xdf7f4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf7f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf7fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf800 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf810 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf814 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf828 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf82c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf830 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf834 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdf844 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf848 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf85c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdf860 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xdf864 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf868 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdf86c | store | 4 | `[sp+8] +0x0` | `[[sp-68]]` |
| `str` at 0xdf874 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xdf878 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf88c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf890 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf8a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf8ac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf8b8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf8c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf8e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf8f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf8fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf910 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf918 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf924 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf92c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf93c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf954 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf95c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf970 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf978 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf984 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf98c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf99c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf9a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf9b0 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xdf9bc | store | 4 | `sp -0x38` | `0x100` |
| `ldr` at 0xdf9c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf9d0 | store | 4 | `sp -0x3c` | `0x300` |
| `ldr` at 0xdf9d4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdf9e8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf9f4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdf9fc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdfa00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfa04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfa14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfa1c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfa28 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdfa2c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfa34 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfa40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfa48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfa58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfa60 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfa6c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfa70 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfa78 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfa84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfa94 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfaa4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfaac | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfab8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdfabc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfac4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfacc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfad0 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xdfad4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfad8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdfadc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfae0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdfae4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdfae8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdfaf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdfb0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfb14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfb20 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdfb28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdfb2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfb30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdfb34 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xdfb48 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdfb4c | load | 4 | `abs 0xdee84` | `-` |
| `ldr` at 0xdfb54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfb5c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfb68 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdfb74 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0xdfb78 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdfb88 | store | 4 | `sp -0x3c` | `0xc0` |
| `ldr` at 0xdfb8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdfba0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdfbac | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdfbb4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdfbb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfbbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfbcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfbd4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfbe0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdfbe4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfbec | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfbf8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfc00 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfc10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfc18 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfc24 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfc28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfc30 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfc3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfc4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfc5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfc64 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfc70 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdfc74 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdfc7c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdfc84 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfc88 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xdfc8c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfc90 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdfc94 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfc98 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdfc9c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdfca0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdfcb0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdfcc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfccc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfcd8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdfce0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdfce4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfce8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdfcec | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xdfd00 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdfd04 | load | 4 | `abs 0xdee88` | `-` |
| `ldr` at 0xdfd0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfd14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfd20 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdfd28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfd30 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfd3c | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xdfd40 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xdfd44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdfd44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdfd44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdde9c | `(unknown callee) 0x1b1b4` | `0x60` | `r1` | `r2` | `r3` |
| `bl` at 0xddebc | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f38` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdded0 | `(unknown callee) 0x1b1b4` | `0x60` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xddef0 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f38` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xddef8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde040 | `(unknown callee) 0xdd33c` | `0x0` | `0x1` | `[sp+16]` | `?` |
| `bl` at 0xde054 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xde05c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde078 | `(unknown callee) 0xdd33c` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xde08c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xde094 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde220 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xde234 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xde23c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde3b8 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde3cc | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xde3d4 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde580 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde594 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xde59c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde740 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde754 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xde75c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde944 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde958 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xde960 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde9c4 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f9c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xde9cc | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde9d4 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdea34 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdea48 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdea50 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdea84 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f64` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xdea8c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdea94 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdec08 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdec1c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdec24 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdec88 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f9c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xdec90 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdec98 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdedfc | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdee10 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdee18 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdefe8 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdeffc | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdf004 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf068 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135f9c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xdf070 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf078 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf3a0 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdf3b4 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdf3bc | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf5a0 | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xdf5b4 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdf5bc | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf730 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdf744 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdf74c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf894 | `(unknown callee) 0xdd33c` | `0x0` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xdf8a8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xdf8b0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf900 | `(unknown callee) 0xdd33c` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xdf914 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xdf91c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf960 | `(unknown callee) 0xdd51c` | `0x0` | `[sp-52]` | `0x0` | `[sp+16]` |
| `bl` at 0xdf974 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd51c)` |
| `bl` at 0xdf97c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf9a0 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf9a8 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfafc | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdfb10 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdfb18 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfb38 | `(unknown callee) 0xe1604` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xdfb50 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135fc8` | `?` | `ret(0xe1604)` |
| `bl` at 0xdfb58 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfb60 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfcb4 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdfcc8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xdfcd0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfcf0 | `(unknown callee) 0xe1604` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+12]` |
| `bl` at 0xdfd08 | `(unknown callee) 0x1b52c` | `[sp+16]` | `0x135ff0` | `?` | `ret(0xe1604)` |
| `bl` at 0xdfd10 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfd18 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfd2c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `ret(0xe1604)` |
| `bl` at 0xdfd34 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xddeb0 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xddee4 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `beq` at 0xddf14 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde030 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde04c | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xde084 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `beq` at 0xde0b0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde22c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xde3c4 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xde404 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde414 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde58c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xde74c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xde774 | `[sp-36] != 0x18` | `[sp-36] == 0x18` |
| `beq` at 0xde7d8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde950 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xde9b0 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xdea40 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xdea70 | `[[sp-68]] == 0x54007ff5` | `[[sp-68]] != 0x54007ff5` |
| `beq` at 0xdeab0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdec14 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xdec74 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xdee08 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xdee48 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdeff4 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xdf054 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `beq` at 0xdf094 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdf200 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdf3ac | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xdf3d4 | `[sp-36] != 0x8` | `[sp-36] == 0x8` |
| `beq` at 0xdf424 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdf434 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdf5ac | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xdf73c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xdf884 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdf8a0 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xdf8cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdf8dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdf8ec | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdf90c | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xdf938 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdf948 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdf96c | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xdf998 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xdfa0c | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfa50 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfa9c | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfb08 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xdfb44 | `ret(0xe1604) != 0x0` | `ret(0xe1604) == 0x0` |
| `blo` at 0xdfbc4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfc08 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfc54 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfcc0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xdfcfc | `ret(0xe1604) != 0x0` | `ret(0xe1604) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdfd44 | `r0` | `?` |

## `jmstr_sel_sic_multi`

- `jmstr_sel_sic_multi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdfd48 to 0xe1604.
  Established. A four-byte load reads `r0 +0x10` (`ldr` at 0xdfd74). Zero `[r0+16]` takes the branch
  to `0xdfd98`, as does a value above `0x8` unsigned 32-bit at the next test (`beq` at 0xdfd84,
  `bhi` at 0xdfd90). The unnamed `0x1b52c` call has `r3`, `0x136020`, `[r0+16]`,
  `[r0+16]` (`bl` at 0xdfda4). Included four-byte stores through `[sp-24] +0x0` and `[sp-28] +0x0`
  each have `[sp-40]` (`str` at 0xe13f8, `str` at 0xe15d8). Multi-device scan roles and data
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdfd58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdfd5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdfd60 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdfd64 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdfd6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfd74 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xdfd78 | store | 4 | `sp -0x68` | `[r0+16]` |
| `ldr` at 0xdfd7c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfd88 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfd98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfd9c | load | 4 | `abs 0xe0cdc` | `-` |
| `ldr` at 0xdfda0 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xdfdac | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfdc0 | store | 4 | `sp -0x3c` | `ret(0x1b1b4)` |
| `ldr` at 0xdfdc4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfdd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfdd4 | load | 4 | `abs 0xe0ce0` | `-` |
| `str` at 0xdfde0 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfdf4 | store | 4 | `sp -0x40` | `ret(0x1b1b4)` |
| `ldr` at 0xdfdf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfe04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfe08 | load | 4 | `abs 0xe0ce0` | `-` |
| `ldr` at 0xdfe10 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfe1c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfe30 | store | 4 | `sp -0x44` | `ret(0x1b1b4)` |
| `ldr` at 0xdfe34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdfe40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfe44 | load | 4 | `abs 0xe0ce0` | `-` |
| `ldr` at 0xdfe4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfe54 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfe60 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfe6c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfe70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfe74 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfe84 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfe98 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdfe9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfeb4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdfec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfecc | load | 4 | `(unknown)` | `-` |
| `str` at 0xdfed0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdfed4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfedc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdfee8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfeec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfef0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdff00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdff14 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdff18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdff30 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdff3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdff48 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdff5c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdff60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdff68 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdff7c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdff80 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdff8c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xdff90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xdffa4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdffa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdffbc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdffc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdffcc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdffd8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdffe4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdffe8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdfffc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0004 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe000c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0018 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0020 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xe0028 | store | 4 | `sp -0x94` | `[sp-104]-1` |
| `ldr` at 0xe002c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0040 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0044 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0048 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0058 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0060 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe006c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0070 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0078 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0084 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe0094 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe009c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe00a4 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe00a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe00ac | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe00c0 | load | 4 | `abs 0xe0ce4` | `-` |
| `ldr` at 0xe00c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe00d0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe00d8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe00e0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe00e4 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe00e8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe00f0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xe00fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0100 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe0104 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0108 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe010c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0110 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0114 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0118 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0128 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe013c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0144 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe014c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0158 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0160 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0164 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe0168 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe016c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0170 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0174 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0178 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe017c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe018c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe01a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe01a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe01b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe01bc | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe01c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe01cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe01d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe01e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe01e8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe01f4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe01f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0200 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe020c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe0210 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0224 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xe0228 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xe0234 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0240 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0244 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe024c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0254 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe025c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0264 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe0268 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe026c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0288 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0290 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0298 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe02a0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe02a4 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe02a8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe02ac | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe02b8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe02bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe02c0 | store | 4 | `sp -0xb0` | `[sp-36]` |
| `ldr` at 0xe02c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe02c8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe02cc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe02d0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe02d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe02d8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe02e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe02fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0304 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe030c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0318 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0320 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xe0338 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe034c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0350 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0354 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0364 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe036c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0378 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe037c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0384 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0390 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe03a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe03a8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe03b0 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe03b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe03b8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe03cc | load | 4 | `abs 0xe0ce4` | `-` |
| `ldr` at 0xe03d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe03dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe03e4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe03ec | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe03f0 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe03f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe03f8 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe03fc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0400 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0404 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0408 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe040c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0410 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0420 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0434 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe043c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0444 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0450 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe045c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0460 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0464 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0474 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe047c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0488 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe048c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0494 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe04a0 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe04a4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe04b8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xe04bc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xe04c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe04d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe04d8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe04e0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe04e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe04f0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe04f8 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe04fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0500 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe051c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0524 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe052c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0534 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe0538 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe053c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0540 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe0544 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0548 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe054c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0550 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0554 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0558 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe057c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0584 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe058c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0598 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe05a0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xe05b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe05cc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe05d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe05d4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe05e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe05ec | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe05f8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe05fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0604 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0610 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe0620 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0628 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0630 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe0634 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0638 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe064c | load | 4 | `abs 0xe0ce4` | `-` |
| `ldr` at 0xe0654 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe065c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0664 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe066c | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe0670 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe0674 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0678 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe067c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0680 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0684 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0688 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe068c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0690 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe06a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe06b4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe06bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06c4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe06d0 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe06dc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe06e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe06e4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe06f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe06fc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0708 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe070c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0714 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0720 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe0724 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0738 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xe073c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xe0748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0754 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0758 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0760 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0768 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0770 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0778 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xe077c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0780 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe079c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe07a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe07ac | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe07b4 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe07b8 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xe07bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe07c0 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xe07c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe07c8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe07cc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe07d0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe07d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe07d8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe07e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe07fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0804 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe080c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0818 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe0824 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0828 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe082c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe083c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0844 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0850 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0854 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe085c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0868 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xe086c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0870 | store | 4 | `sp -0x2c` | `[sp-148]` |
| `ldr` at 0xe0874 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0878 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe088c | load | 4 | `abs 0xe0ce4` | `-` |
| `ldr` at 0xe0894 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe089c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe08a4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe08ac | load | 4 | `sp -0x98` | `-` |
| `str` at 0xe08b0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe08b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe08b8 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe08bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe08c0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe08c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe08c8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe08cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe08d0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe08e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe08f4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe08fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0904 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0910 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0918 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xe0920 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `str` at 0xe092c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0930 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0934 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0944 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe095c | load | 4 | `abs 0xe0ce8` | `-` |
| `str` at 0xe0960 | store | 4 | `(unknown)` | `0x800ffe0` |
| `ldr` at 0xe0964 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe096c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0978 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe097c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0984 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0990 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0994 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe099c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe09b0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe09b4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe09bc | store | 4 | `sp -0x24` | `ret(0xe17d4)` |
| `ldr` at 0xe09c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe09c4 | store | 4 | `sp -0xb0` | `ret(0xe17d4)` |
| `ldr` at 0xe09c8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe09cc | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe09d0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe09d4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe09d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe09dc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe09ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0a00 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0a08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0a10 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0a1c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0a24 | load | 4 | `abs 0xe0ce8` | `-` |
| `str` at 0xe0a28 | store | 4 | `sp -0x88` | `0x800ffe0` |
| `ldr` at 0xe0a2c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a30 | load | 4 | `abs 0xe0cf0` | `-` |
| `ldr` at 0xe0a3c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a44 | load | 4 | `abs 0xe0cf0` | `-` |
| `str` at 0xe0a50 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0a54 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a58 | load | 4 | `abs 0xe0cf4` | `-` |
| `ldr` at 0xe0a64 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a6c | load | 4 | `abs 0xe0cf4` | `-` |
| `str` at 0xe0a78 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0a7c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a80 | load | 4 | `abs 0xe0cf8` | `-` |
| `ldr` at 0xe0a8c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0a94 | load | 4 | `abs 0xe0cf8` | `-` |
| `str` at 0xe0aa0 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0aa4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0ab4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xe0ac8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0acc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0adc | load | 4 | `sp -0x88` | `-` |
| `str` at 0xe0af0 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0af8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xe0b00 | store | 4 | `sp -0x9c` | `?` |
| `str` at 0xe0b08 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe0b0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b10 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0b20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b38 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0b3c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0b44 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0xe0b4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0b50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b68 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b80 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0b88 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0b90 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xe0b9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0ba4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0ba8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0bb0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0bb8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0bbc | load | 4 | `sp -0x68` | `-` |
| `str` at 0xe0bc4 | store | 4 | `sp -0x88` | `?` |
| `str` at 0xe0bcc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0bd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0bd4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0be4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0bec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0bf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0c0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0c10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0c1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0c20 | load | 4 | `abs 0xe0cec` | `-` |
| `ldr` at 0xe0c28 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0c30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0c38 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0c44 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0c4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0c54 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe0c60 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xe0c64 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe0c68 | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xe0c6c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0c70 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xe0c74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0c78 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0c7c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0c9c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0ca4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0cac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0cb8 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe0cc4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0cc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0ccc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0d04 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0d18 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xe0d1c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xe0d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0d34 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0d38 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xe0d3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0d54 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xe0d60 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xe0d6c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0d70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0d78 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0d84 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0d88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0d90 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0d9c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0da0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0da8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0dbc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0dc0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0dc8 | store | 4 | `sp -0x24` | `ret(0xe17d4)` |
| `ldr` at 0xe0dcc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0dd0 | store | 4 | `sp -0xb0` | `ret(0xe17d4)` |
| `ldr` at 0xe0dd4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0dd8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0ddc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0de0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0de4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0de8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0df8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0e0c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0e14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0e1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0e28 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0e30 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe0e44 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xe0e48 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe0e4c | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xe0e50 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0e54 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xe0e58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0e5c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0e60 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0e80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0e88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0e90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0e9c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe0ea8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0eac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0eb0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0ec0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0edc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0ee0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0ee8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0ef4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0ef8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0f00 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0f0c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0f10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0f18 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0f2c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0f30 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0f38 | store | 4 | `sp -0x24` | `ret(0xe17d4)` |
| `ldr` at 0xe0f3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0f40 | store | 4 | `sp -0xb0` | `ret(0xe17d4)` |
| `ldr` at 0xe0f44 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0f48 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0f4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0f50 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0f54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0f58 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0f68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0f84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0f8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0f98 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0fa0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0fa4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0fa8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0fac | load | 4 | `abs 0xe0cf0` | `-` |
| `ldr` at 0xe0fb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0fbc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0fc4 | load | 4 | `abs 0xe0cf0` | `-` |
| `str` at 0xe0fd0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe0fd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0fd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0fdc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0fe0 | load | 4 | `abs 0xe0cf4` | `-` |
| `ldr` at 0xe0fec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0ff0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0ff8 | load | 4 | `abs 0xe0cf4` | `-` |
| `str` at 0xe1004 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe1008 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe100c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1010 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe1014 | load | 4 | `abs 0xe0cf8` | `-` |
| `ldr` at 0xe1020 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1024 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe102c | load | 4 | `abs 0xe0cf8` | `-` |
| `str` at 0xe1038 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe103c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1040 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1044 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe1054 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1058 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe106c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe1070 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1074 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1078 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe1088 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe108c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe10a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe10a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe10a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe10ac | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe10b0 | store | 4 | `[sp-24] +0x0` | `[[sp-64]]` |
| `str` at 0xe10b8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe10bc | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe10d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe10d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe10e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe10f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe10f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe1104 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe110c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe111c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe112c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe1144 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe115c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1164 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe116c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe1178 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe1180 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe1190 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe11a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe11b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe11c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe11cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe11d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe11e0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe11e8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe11f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1200 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1208 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe1214 | store | 4 | `sp -0xa0` | `0x1` |
| `str` at 0xe1220 | store | 4 | `sp -0x34` | `0x100` |
| `ldr` at 0xe1224 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1234 | store | 4 | `sp -0x38` | `0x300` |
| `ldr` at 0xe1238 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe124c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1258 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xe1260 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe1264 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1268 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1278 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1280 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe128c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1290 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1298 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe12a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe12ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe12bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe12c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe12d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe12d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe12dc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe12e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe12f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1308 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1310 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe131c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1320 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1328 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe1330 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1334 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe1338 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe133c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe1340 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe1344 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe1348 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe134c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe135c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1370 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1378 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1380 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe138c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe1398 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe139c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe13a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe13b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe13b8 | load | 4 | `abs 0xe0cfc` | `-` |
| `ldr` at 0xe13c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe13c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe13d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe13dc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe13e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe13f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe13f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe13f8 | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `str` at 0xe1400 | store | 4 | `sp -0x34` | `0x40` |
| `ldr` at 0xe1404 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1414 | store | 4 | `sp -0x38` | `0xc0` |
| `ldr` at 0xe1418 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe142c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1438 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xe1440 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe1444 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1448 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe1460 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe146c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1470 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1478 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe1484 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe148c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe149c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe14a4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe14b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe14b4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe14bc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe14c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe14d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe14e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe14f0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe14fc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1500 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe1508 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe1510 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe1514 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe1518 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe151c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe1520 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe1524 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe1528 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe152c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe153c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1550 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe1558 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1560 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe156c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe1578 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe157c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe1580 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe1594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1598 | load | 4 | `abs 0xe0d00` | `-` |
| `ldr` at 0xe15a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe15a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe15b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe15bc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe15c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe15d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe15d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe15d8 | store | 4 | `[sp-28] +0x0` | `[sp-40]` |
| `ldr` at 0xe15dc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe15e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe15ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe15f8 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0xe15fc | load | 4 | `sp -0xa0` | `-` |
| `ldmdb` at 0xe1600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe1600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe1600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdfda4 | `(unknown callee) 0x1b52c` | `r3` | `0x136020` | `[r0+16]` | `[r0+16]` |
| `bl` at 0xdfdb8 | `(unknown callee) 0x1b1b4` | `0x60` | `r1` | `0x10` | `[r0+16]` |
| `bl` at 0xdfdd8 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135f38` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdfdec | `(unknown callee) 0x1b1b4` | `0x60` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdfe0c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135f38` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdfe14 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xdfe28 | `(unknown callee) 0x1b1b4` | `0x60` | `?` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdfe48 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135f38` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xdfe50 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xdfe58 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdff84 | `(unknown callee) 0xe17d4` | `sp-136` | `0xc` | `[sp-104]` | `?` |
| `bl` at 0xdffac | `(unknown callee) 0xdd33c` | `0x0` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0xdffc0 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xdffc8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdffd0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdffec | `(unknown callee) 0xdd33c` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xe0000 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xe0008 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0010 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0098 | `(unknown callee) 0xe1a30` | `0x4` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe00cc | `(unknown callee) 0xe1a30` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe012c | `(unknown callee) 0xdd8e8` | `0x0` | `0x0` | `0x1` | `[sp-48]` |
| `bl` at 0xe0140 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0148 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0150 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0190 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe01a4 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe01ac | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe01b4 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0258 | `(unknown callee) 0xe1a30` | `[sp-136]` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe028c | `(unknown callee) 0xe1a30` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xe02ec | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0300 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0308 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0310 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe03a4 | `(unknown callee) 0xe1a30` | `0xe` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe03d8 | `(unknown callee) 0xe1a30` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe0424 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0438 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0440 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0448 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe04ec | `(unknown callee) 0xe1a30` | `[sp-136]` | `0x18` | `[sp-60]` | `sp-152` |
| `bl` at 0xe0520 | `(unknown callee) 0xe1a30` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xe056c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0580 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0588 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0590 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0624 | `(unknown callee) 0xe1a30` | `0x9` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe0658 | `(unknown callee) 0xe1a30` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe06a4 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe06b8 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe06c0 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe06c8 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe076c | `(unknown callee) 0xe1a30` | `[sp-136]` | `0x8` | `[sp-60]` | `sp-152` |
| `bl` at 0xe07a0 | `(unknown callee) 0xe1a30` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xe07ec | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0800 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0808 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0810 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0898 | `(unknown callee) 0xe1a30` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xe08e4 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe08f8 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0900 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0908 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe09b8 | `(unknown callee) 0xe17d4` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe09f0 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0a04 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0a0c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0a14 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0c24 | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x13603c` | `?` | `?` |
| `bl` at 0xe0c2c | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0c34 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0c3c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0c8c | `(unknown callee) 0xe1c84` | `[sp-104]` | `0x8` | `0xc` | `0x1` |
| `bl` at 0xe0ca0 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xe1c84)` |
| `bl` at 0xe0ca8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0cb0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0dc4 | `(unknown callee) 0xe17d4` | `sp-136` | `0x8` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe0dfc | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0e10 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0e18 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0e20 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0e70 | `(unknown callee) 0xe1c84` | `[sp-104]` | `0x1` | `0xc` | `0x1` |
| `bl` at 0xe0e84 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xe1c84)` |
| `bl` at 0xe0e8c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0e94 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0f34 | `(unknown callee) 0xe17d4` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe0f6c | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0f80 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe0f88 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0f90 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe10d8 | `(unknown callee) 0xdd33c` | `0x0` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xe10ec | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xe10f4 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe10fc | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe114c | `(unknown callee) 0xdd33c` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xe1160 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd33c)` |
| `bl` at 0xe1168 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe1170 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe11b4 | `(unknown callee) 0xdd51c` | `0x0` | `[sp-48]` | `0x0` | `[sp-32]` |
| `bl` at 0xe11c8 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd51c)` |
| `bl` at 0xe11d0 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe11d8 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe11fc | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe1204 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe120c | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe1360 | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe1374 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe137c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe1384 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe13a4 | `(unknown callee) 0xe1604` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xe13bc | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135fc8` | `?` | `ret(0xe1604)` |
| `bl` at 0xe13c4 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe13cc | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe13d4 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe1540 | `(unknown callee) 0xdd8e8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe1554 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `ret(0xdd8e8)` |
| `bl` at 0xe155c | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe1564 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe1584 | `(unknown callee) 0xe1604` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xe159c | `(unknown callee) 0x1b52c` | `[sp-32]` | `0x135ff0` | `?` | `ret(0xe1604)` |
| `bl` at 0xe15a4 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe15ac | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe15b4 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe15e0 | `(unknown callee) 0x1b490` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe15e8 | `(unknown callee) 0x1b490` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe15f0 | `(unknown callee) 0x1b490` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdfd84 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bhi` at 0xdfd90 | `[r0+16] > 0x8 (unsigned, 32-bit)` | `[r0+16] <= 0x8 (unsigned, 32-bit)` |
| `bne` at 0xdfdcc | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xdfe00 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xdfe3c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `blo` at 0xdfe7c | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfef8 | `C == 0` | `not (C == 0)` |
| `beq` at 0xdff9c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdffb8 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xdfff8 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bge` at 0xe0034 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xe0050 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe00b4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe00f8 | `[sp-148] != [sp-104]-1` | `[sp-148] == [sp-104]-1` |
| `bne` at 0xe0138 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xe019c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe01d8 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0274 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe02f8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xe0344 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xe035c | `C == 0` | `not (C == 0)` |
| `blo` at 0xe03c0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0430 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe046c | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0508 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0578 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xe05c4 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xe05dc | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0640 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe06b0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe06ec | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0788 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe07f8 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe0834 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0880 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe08f0 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe093c | `C == 0` | `not (C == 0)` |
| `bne` at 0xe09fc | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `blo` at 0xe0b18 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0bdc | `C == 0` | `not (C == 0)` |
| `beq` at 0xe0c18 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe0c98 | `ret(0xe1c84) != 0x0` | `ret(0xe1c84) == 0x0` |
| `blo` at 0xe0cd4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0e08 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xe0e3c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe0e7c | `ret(0xe1c84) != 0x0` | `ret(0xe1c84) == 0x0` |
| `blo` at 0xe0eb8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0f78 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `beq` at 0xe10c8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe10e4 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xe1118 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe1128 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe1138 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe1158 | `ret(0xdd33c) != 0x0` | `ret(0xdd33c) == 0x0` |
| `bne` at 0xe118c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe119c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe11c0 | `ret(0xdd51c) != 0x0` | `ret(0xdd51c) == 0x0` |
| `bne` at 0xe11f4 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe1270 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe12b4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe1300 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe136c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xe13b0 | `ret(0xe1604) != 0x0` | `ret(0xe1604) == 0x0` |
| `beq` at 0xe13ec | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blo` at 0xe1450 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe1494 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe14e0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe154c | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bne` at 0xe1590 | `ret(0xe1604) != 0x0` | `ret(0xe1604) == 0x0` |
| `beq` at 0xe15cc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe1600 | `r0` | `?` |

## `JTAG_Init`

- `JTAG_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3ac8 to 0xe3c98.
  Established. A four-byte assignment at absolute `0x2b45c0` retains `ret(0x1b130)` (`str` at
  0xe3b98). Another writes `[0x2b45c0]+16` at absolute `0x2b4520` (`str` at 0xe3bec). Four halfword
  stores through `[0x2b4520] +0x0` have `0x800`, `0x1000`, `0x800`, `0x800`, respectively (`strh` at
  0xe3bfc, `strh` at 0xe3c0c, `strh` at 0xe3c1c, `strh` at 0xe3c2c). The branch takes `Z == 0` to `0xe3bd0` (`bne` at 0xe3ba4). Hardware bit
  meanings and initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe3ad8 | load | 4 | `abs 0xe3c54` | `-` |
| `ldr` at 0xe3adc | load | 4 | `abs 0x2adfd4` | `-` |
| `ldr` at 0xe3ae8 | load | 4 | `abs 0xe3c58` | `-` |
| `str` at 0xe3aec | store | 4 | `sp -0x24` | `0x136150` |
| `ldr` at 0xe3af4 | load | 4 | `abs 0xe3c5c` | `-` |
| `ldr` at 0xe3af8 | load | 4 | `abs 0xe3c60` | `-` |
| `ldr` at 0xe3b04 | load | 4 | `abs 0xe3c64` | `-` |
| `ldr` at 0xe3b08 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xe3b14 | load | 4 | `abs 0xe3c68` | `-` |
| `ldr` at 0xe3b1c | load | 4 | `abs 0xe3c6c` | `-` |
| `ldr` at 0xe3b20 | load | 4 | `abs 0xe3c70` | `-` |
| `ldr` at 0xe3b28 | load | 4 | `abs 0xe3c74` | `-` |
| `ldr` at 0xe3b2c | load | 4 | `abs 0xe3c78` | `-` |
| `str` at 0xe3b38 | store | 4 | `sp -0x1c` | `ret(0x1b400)` |
| `ldr` at 0xe3b3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3b54 | load | 4 | `abs 0xe3c7c` | `-` |
| `ldr` at 0xe3b58 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xe3b5c | load | 4 | `abs 0xe3c80` | `-` |
| `ldr` at 0xe3b60 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xe3b70 | load | 4 | `abs 0xe3c84` | `-` |
| `ldr` at 0xe3b74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3b78 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe3b80 | store | 4 | `sp -0x20` | `0x10000000` |
| `str` at 0xe3b98 | store | 4 | `abs 0x2b45c0` | `ret(0x1b130)` |
| `ldr` at 0xe3b9c | load | 4 | `abs 0x2b45c0` | `-` |
| `ldr` at 0xe3bb4 | load | 4 | `abs 0xe3c7c` | `-` |
| `ldr` at 0xe3bb8 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xe3bbc | load | 4 | `abs 0xe3c88` | `-` |
| `ldr` at 0xe3bc0 | load | 4 | `ret(0x1b148) +0x0` | `-` |
| `ldr` at 0xe3bdc | load | 4 | `abs 0xe3c8c` | `-` |
| `ldr` at 0xe3be0 | load | 4 | `abs 0xe3c84` | `-` |
| `ldr` at 0xe3be4 | load | 4 | `abs 0x2b45c0` | `-` |
| `str` at 0xe3bec | store | 4 | `abs 0x2b4520` | `[0x2b45c0]+16` |
| `ldr` at 0xe3bf0 | load | 4 | `abs 0xe3c8c` | `-` |
| `ldr` at 0xe3bf4 | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xe3bfc | store | 2 | `[0x2b4520] +0x0` | `0x800` |
| `ldr` at 0xe3c00 | load | 4 | `abs 0xe3c8c` | `-` |
| `ldr` at 0xe3c04 | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xe3c0c | store | 2 | `[0x2b4520] +0x0` | `0x1000` |
| `ldr` at 0xe3c10 | load | 4 | `abs 0xe3c8c` | `-` |
| `ldr` at 0xe3c14 | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xe3c1c | store | 2 | `[0x2b4520] +0x0` | `0x800` |
| `ldr` at 0xe3c20 | load | 4 | `abs 0xe3c8c` | `-` |
| `ldr` at 0xe3c24 | load | 4 | `abs 0x2b4520` | `-` |
| `strh` at 0xe3c2c | store | 2 | `[0x2b4520] +0x0` | `0x800` |
| `ldr` at 0xe3c30 | load | 4 | `abs 0xe3c90` | `-` |
| `ldr` at 0xe3c34 | load | 4 | `abs 0xe3c94` | `-` |
| `ldr` at 0xe3c44 | load | 4 | `abs 0xe3c54` | `-` |
| `str` at 0xe3c4c | store | 4 | `abs 0x2adfd4` | `0x1` |
| `ldmdb` at 0xe3c50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe3c50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3c50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3c50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3b00 | `(unknown callee) 0x21eb8` | `0x9` | `0x136168` | `0x102` | `0x0` |
| `bl` at 0xe3b18 | `(unknown callee) 0x23fbc` | `0x136180` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0xe3b24 | `(unknown callee) 0xdd300` | `0xe688c` | `0xe6628` | `?` | `?` |
| `bl` at 0xe3b30 | `(unknown callee) 0x1b400` | `0x1361b4` | `0x1002` | `?` | `?` |
| `bl` at 0xe3b48 | `(unknown callee) 0x1b148` | `ret(0x1b400)` | `?` | `?` | `ret(0x1b400)` |
| `bl` at 0xe3b64 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1361c0` | `[ret(0x1b148)]` | `0x145ae8` |
| `bl` at 0xe3b6c | `(unknown callee) 0x1b43c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xe3b94 | `(unknown callee) 0x1b130` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xe3ba8 | `(unknown callee) 0x1b148` | `ret(0x1b130)` | `?` | `?` | `[0x2b45c0]` |
| `bl` at 0xe3bc4 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x1361e0` | `[ret(0x1b148)]` | `0x145ae8` |
| `bl` at 0xe3bcc | `(unknown callee) 0x1b43c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xe3bd8 | `(unknown callee) 0xe3c98` | `0x1` | `0x0` | `?` | `?` |
| `bl` at 0xe3c40 | `(unknown callee) 0x229b0` | `0x2b45e0` | `0x13620c` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3ae4 | `[0x2adfd4] != 0x1` | `[0x2adfd4] == 0x1` |
| `beq` at 0xe3b10 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bge` at 0xe3b44 | `ret(0x1b400) >= 0x0 (signed, 32-bit)` | `ret(0x1b400) < 0x0 (signed, 32-bit)` |
| `bne` at 0xe3ba4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3c50 | `r0` | `ret(0x229b0)` |

## `JTAG_Lock`

- `JTAG_Lock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3cf8 to 0xe3d64.
  Established. The unnamed `0x22ac8` call has `0x2b45e0`, unknown, `r2`, `r3` (`bl` at 0xe3d10); its
  zero result takes the branch to `0xe3d54` (`beq` at 0xe3d24). The other path includes a four-byte
  assignment of `ret(0x21704)` at `sp -0x18` (`str` at 0xe3d3c). The merged exit value and lock
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe3d08 | load | 4 | `abs 0xe3d58` | `-` |
| `str` at 0xe3d18 | store | 4 | `sp -0x14` | `ret(0x22ac8)` |
| `ldr` at 0xe3d1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3d34 | load | 4 | `abs 0xe3d5c` | `-` |
| `str` at 0xe3d38 | store | 4 | `sp -0x1c` | `0x136218` |
| `str` at 0xe3d3c | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xe3d44 | load | 4 | `abs 0xe3d60` | `-` |
| `ldmdb` at 0xe3d54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3d54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3d54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3d10 | `(unknown callee) 0x22ac8` | `0x2b45e0` | `?` | `r2` | `r3` |
| `bl` at 0xe3d2c | `(unknown callee) 0x21704` | `ret(0x22ac8)` | `?` | `?` | `ret(0x22ac8)` |
| `bl` at 0xe3d50 | `(unknown callee) 0x21eb8` | `0x9` | `0x136168` | `0x170` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3d24 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3d54 | `r0` | `?` |

## `JTAG_Unlock`

- `JTAG_Unlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3d64 to 0xe3dcc.
  Established. The unnamed `0x22e50` call carries `0x2b45e0`, `r1`, `r2`, `r3` (`bl` at 0xe3d78). A
  zero result takes the branch to `0xe3dbc`, while nonzero falls through (`beq` at 0xe3d8c). The
  latter path includes a four-byte stack assignment of `0x136238` at `sp -0x1c` (`str` at 0xe3da0).
  Unlock effects and return value are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe3d74 | load | 4 | `abs 0xe3dc0` | `-` |
| `str` at 0xe3d80 | store | 4 | `sp -0x14` | `ret(0x22e50)` |
| `ldr` at 0xe3d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3d90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3d9c | load | 4 | `abs 0xe3dc4` | `-` |
| `str` at 0xe3da0 | store | 4 | `sp -0x1c` | `0x136238` |
| `str` at 0xe3da4 | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xe3dac | load | 4 | `abs 0xe3dc8` | `-` |
| `ldmdb` at 0xe3dbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3dbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3dbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe3d78 | `(unknown callee) 0x22e50` | `0x2b45e0` | `r1` | `r2` | `r3` |
| `bl` at 0xe3d94 | `(unknown callee) 0x21704` | `ret(0x22e50)` | `?` | `?` | `ret(0x22e50)` |
| `bl` at 0xe3db8 | `(unknown callee) 0x21eb8` | `0x9` | `0x136168` | `0x180` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe3d8c | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3dbc | `r0` | `?` |

## `JTAG_Comm_read`

- `JTAG_Comm_read`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe81e4 to 0xe837c.
  Established. The unnamed `0xeb5ec` call uses `r0`, `[sp-48]`, `sp-44`, `sp-44` (`bl` at 0xe824c);
  its zero result takes the branch to `0xe8298`, followed by an unnamed `0xe2cf8` call with `0x0`,
  `0x40`, `sp-44`, `sp-44` (`beq` at 0xe8260, `bl` at 0xe82a8). An included byte store through
  `([sp-24] + [sp-32]) +0x0` has unknown value (`strb` at 0xe832c). Read payload layout and external
  transaction semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe81f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe81f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe81fc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe8200 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe8218 | store | 4 | `sp -0x30` | `0xc` |
| `str` at 0xe8224 | store | 4 | `sp -0x30` | `0x11` |
| `str` at 0xe8234 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xe8238 | load | 4 | `abs 0xe8374` | `-` |
| `str` at 0xe823c | store | 4 | `sp -0x38` | `0x1367f0` |
| `ldr` at 0xe8240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe8244 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8254 | store | 4 | `sp -0x24` | `ret(0xeb5ec)` |
| `ldr` at 0xe8258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe8264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8270 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe827c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe8280 | load | 4 | `abs 0xe8378` | `-` |
| `ldr` at 0xe828c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8290 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe82b0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe82b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe82c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe82cc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe82e4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe82e8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xe8304 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe830c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8314 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe8328 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xe832c | store | 1 | `([sp-24] + [sp-32]) +0x0` | `?` |
| `ldr` at 0xe8348 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8350 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8354 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe835c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe8368 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe836c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe8370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe824c | `(unknown callee) 0xeb5ec` | `r0` | `[sp-48]` | `sp-44` | `sp-44` |
| `bl` at 0xe8274 | `(unknown callee) 0x21704` | `ret(0xeb5ec)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xe8288 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x136800` | `ret(0x21704)` | `ret(0x21704)` |
| `bl` at 0xe82a8 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-44` | `sp-44` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe8210 | `? != 0x6` | `? == 0x6` |
| `beq` at 0xe8260 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0xe826c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `ble` at 0xe82bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8370 | `r0` | `?` |

## `JTAG_Comm_write`

- `JTAG_Comm_write`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe837c to 0xe8510.
  Established. Two four-byte zero stores have unknown effective targets (`stm` at 0xe83ac). Included
  byte loads read `([sp-24] + [sp-32]) +0x4` and the same base `+0x0` (`ldrb` at 0xe8424, `ldrb` at 0xe845c). The unnamed `0xeb5ec` call carries `[sp-20]`, `[sp-48]`, `sp-44`, `0x0` (`bl` at
  0xe84ac); its zero result takes the branch to `0xe84f8` (`beq` at 0xe84c0). The packed write
  format and external effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe838c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe8390 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe8394 | store | 4 | `sp -0x1c` | `r2` |
| `stm` at 0xe83ac | store | 4 | `(unknown)` | `0x0` |
| `stm` at 0xe83ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe83b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe83c8 | store | 4 | `sp -0x30` | `0xd` |
| `str` at 0xe83d4 | store | 4 | `sp -0x30` | `0x12` |
| `str` at 0xe83dc | store | 4 | `sp -0x20` | `0x3` |
| `ldr` at 0xe83e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe8408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8414 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8418 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe8424 | load | 1 | `([sp-24] + [sp-32]) +0x4` | `-` |
| `str` at 0xe842c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe8454 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xe845c | load | 1 | `([sp-24] + [sp-32]) +0x0` | `-` |
| `str` at 0xe8464 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8468 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe8470 | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `str` at 0xe8494 | store | 4 | `sp -0x3c` | `0x40` |
| `ldr` at 0xe8498 | load | 4 | `abs 0xe8508` | `-` |
| `str` at 0xe849c | store | 4 | `sp -0x38` | `0x13682c` |
| `ldr` at 0xe84a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe84a4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe84b4 | store | 4 | `sp -0x24` | `ret(0xeb5ec)` |
| `ldr` at 0xe84b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe84c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe84dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe84e0 | load | 4 | `abs 0xe850c` | `-` |
| `ldr` at 0xe84ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe84f0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe84fc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe8500 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe8504 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe8504 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe8504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe8488 | `(unknown callee) 0xe2cf8` | `0x0` | `0x40` | `sp-44` | `sp-44` |
| `bl` at 0xe84ac | `(unknown callee) 0xeb5ec` | `[sp-20]` | `[sp-48]` | `sp-44` | `0x0` |
| `bl` at 0xe84d4 | `(unknown callee) 0x21704` | `ret(0xeb5ec)` | `?` | `?` | `[sp-28]` |
| `bl` at 0xe84e8 | `(unknown callee) 0x1b52c` | `[sp-28]` | `0x13683c` | `ret(0x21704)` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe83c0 | `? != 0x6` | `? == 0x6` |
| `bge` at 0xe83e8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xe84c0 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0xe84cc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe8504 | `r0` | `?` |

## `TIOJTAGInit`

- `TIOJTAGInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53d08 to 0x53ed4.
  Established. A zero result from unnamed `0x3dce4` takes the branch to the exit at `0x53eb0` (`bl`
  at 0x53d18, `beq` at 0x53d24). Included byte assignments at absolute `0x1490d4` write `0x1`,
  `0x0`, `0x0` at separate sites (`strb` at 0x53d38, `strb` at 0x53de8, `strb` at 0x53e08). The
  unnamed `0xd7530` call has `0x0`, unknown, `sp-17`, unknown (`bl` at 0x53d94). Record meanings and
  TIO JTAG initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53d2c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x53d30 | load | 4 | `abs 0x53eb4` | `-` |
| `strb` at 0x53d38 | store | 1 | `abs 0x1490d4` | `0x1` |
| `ldr` at 0x53d3c | load | 4 | `abs 0x53eb8` | `-` |
| `ldrb` at 0x53d40 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x53d44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53d50 | load | 4 | `abs 0x53eb4` | `-` |
| `ldrb` at 0x53d54 | load | 1 | `abs 0x1490d4` | `-` |
| `ldr` at 0x53d64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53d78 | load | 4 | `abs 0x53ebc` | `-` |
| `ldr` at 0x53d7c | load | 4 | `abs 0x1490d0` | `-` |
| `ldrb` at 0x53d84 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53d9c | store | 4 | `sp -0x1c` | `ret(0xd7530)` |
| `ldr` at 0x53da0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53dac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53db8 | load | 4 | `abs 0x53ec0` | `-` |
| `str` at 0x53dbc | store | 4 | `sp -0x28` | `0x10be20` |
| `ldr` at 0x53dc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53dc4 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x53dc8 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x53dd0 | load | 4 | `abs 0x53ec4` | `-` |
| `ldr` at 0x53dd4 | load | 4 | `abs 0x53ec8` | `-` |
| `ldr` at 0x53de0 | load | 4 | `abs 0x53eb4` | `-` |
| `strb` at 0x53de8 | store | 1 | `abs 0x1490d4` | `0x0` |
| `ldrb` at 0x53df0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x53e00 | load | 4 | `abs 0x53eb4` | `-` |
| `strb` at 0x53e08 | store | 1 | `abs 0x1490d4` | `0x0` |
| `ldr` at 0x53e0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53e14 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x53e1c | load | 4 | `abs 0x53eb4` | `-` |
| `ldrb` at 0x53e20 | load | 1 | `abs 0x1490d4` | `-` |
| `str` at 0x53e30 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x53e34 | load | 4 | `abs 0x53eb8` | `-` |
| `ldrb` at 0x53e38 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x53e3c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x53e4c | load | 1 | `sp -0x18` | `-` |
| `str` at 0x53e5c | store | 4 | `sp -0x1c` | `ret(0x54e78)` |
| `ldr` at 0x53e60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53e6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53e78 | load | 4 | `abs 0x53ecc` | `-` |
| `str` at 0x53e7c | store | 4 | `sp -0x28` | `0x10be40` |
| `ldr` at 0x53e80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53e84 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x53e88 | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x53e90 | load | 4 | `abs 0x53ec4` | `-` |
| `ldr` at 0x53e94 | load | 4 | `abs 0x53ed0` | `-` |
| `ldr` at 0x53ea0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53ea8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x53eb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53eb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53eb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53d18 | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x53d94 | `(unknown callee) 0xd7530` | `0x0` | `?` | `sp-17` | `?` |
| `bl` at 0x53db0 | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0x53ddc | `(unknown callee) 0x21eb8` | `0x1` | `0x10bdd4` | `0x107` | `0x0` |
| `bl` at 0x53e54 | `(unknown callee) 0x54e78` | `[sp-24]:1` | `?` | `[0x2b1620]:1` | `[sp-24]:1` |
| `bl` at 0x53e70 | `(unknown callee) 0x21a38` | `ret(0x54e78)` | `?` | `?` | `ret(0x54e78)` |
| `bl` at 0x53e9c | `(unknown callee) 0x21eb8` | `0x1` | `0x10bdd4` | `0x113` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53d24 | `ret(0x3dce4) == 0x0` | `ret(0x3dce4) != 0x0` |
| `bge` at 0x53d4c | `[sp-24] >= [0x2b1620]:1 (signed, 32-bit)` | `[sp-24] < [0x2b1620]:1 (signed, 32-bit)` |
| `bne` at 0x53d5c | `[0x1490d4]:1 != 0x0` | `[0x1490d4]:1 == 0x0` |
| `beq` at 0x53da8 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `bne` at 0x53dfc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x53e28 | `[0x1490d4]:1 == 0x0` | `[0x1490d4]:1 != 0x0` |
| `blt` at 0x53e44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x53e68 | `ret(0x54e78) == 0x0` | `ret(0x54e78) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53eb0 | `r0` | `?` |
