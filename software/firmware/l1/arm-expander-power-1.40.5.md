# ARM expander and power control helpers in 1.40.5

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
ioExpPreConfig has an unresolved computed transfer (`ldrls` at 0xd6c3c); its tables and return cover known edges only. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/expander-power-facts.txt) retain all tool sections.

## `ioExpInit`

- `ioExpInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd64f4 to 0xd6798.
  Established. Included four-byte global assignments write `0x1` at `0x2adf9c` and `ret(0x1b454)` at
  `0x2adf98` (`str` at 0xd6578, `str` at 0xd65ac). Another four-byte store through `[sp-36] +0x8`
  retains `ret(0x1b454)` (`str` at 0xd6634). Included byte assignments at unknown targets write
  `0x0` and `0x1` at separate sites (`strb` at 0xd6734, `strb` at 0xd6760). Expander record layout
  and initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6504 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd6508 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd650c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd6510 | load | 4 | `abs 0xd6778` | `-` |
| `ldr` at 0xd6514 | load | 4 | `abs 0x2adf98` | `-` |
| `ldr` at 0xd6520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd652c | load | 4 | `abs 0xd677c` | `-` |
| `str` at 0xd6530 | store | 4 | `sp -0x38` | `0x13522c` |
| `ldr` at 0xd6534 | load | 4 | `abs 0xd6778` | `-` |
| `ldr` at 0xd6538 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd653c | store | 4 | `sp -0x34` | `[0x2adf98]` |
| `ldr` at 0xd6540 | load | 4 | `abs 0xd6780` | `-` |
| `ldr` at 0xd6544 | load | 4 | `abs 0x2adf9c` | `-` |
| `str` at 0xd6548 | store | 4 | `sp -0x30` | `[0x2adf9c]` |
| `ldr` at 0xd6550 | load | 4 | `abs 0xd6784` | `-` |
| `ldr` at 0xd6560 | load | 4 | `abs 0xd6780` | `-` |
| `ldr` at 0xd6564 | load | 4 | `abs 0x2adf9c` | `-` |
| `ldr` at 0xd6570 | load | 4 | `abs 0xd6780` | `-` |
| `str` at 0xd6578 | store | 4 | `abs 0x2adf9c` | `0x1` |
| `ldr` at 0xd657c | load | 4 | `abs 0xd6780` | `-` |
| `ldr` at 0xd6580 | load | 4 | `abs 0x2adf9c` | `-` |
| `str` at 0xd6594 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd6598 | load | 4 | `abs 0xd6778` | `-` |
| `ldr` at 0xd659c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd65ac | store | 4 | `abs 0x2adf98` | `ret(0x1b454)` |
| `ldr` at 0xd65b0 | load | 4 | `abs 0x2adf98` | `-` |
| `ldr` at 0xd65bc | load | 4 | `abs 0xd6788` | `-` |
| `str` at 0xd65c0 | store | 4 | `sp -0x38` | `0x135278` |
| `ldr` at 0xd65c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd65c8 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd65d0 | load | 4 | `abs 0xd6784` | `-` |
| `ldr` at 0xd65e0 | load | 4 | `abs 0xd6778` | `-` |
| `ldr` at 0xd65e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd65f8 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd6600 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd6604 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6608 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd660c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xd6610 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6614 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd6618 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xd661c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6620 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6624 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xd6634 | store | 4 | `[sp-36] +0x8` | `ret(0x1b454)` |
| `ldr` at 0xd6638 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd6644 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd664c | load | 4 | `abs 0xd678c` | `-` |
| `ldr` at 0xd6650 | load | 4 | `abs 0xd6790` | `-` |
| `str` at 0xd667c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd6680 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6688 | load | 4 | `abs 0xd678c` | `-` |
| `ldr` at 0xd668c | load | 4 | `abs 0xd6790` | `-` |
| `ldr` at 0xd6698 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd66a8 | load | 4 | `abs 0xd6794` | `-` |
| `str` at 0xd66ac | store | 4 | `(unknown)` | `0x1352bc` |
| `str` at 0xd66b0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd66b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd66b8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd66c0 | load | 4 | `abs 0xd6784` | `-` |
| `str` at 0xd66d4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd66d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd66dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd66e0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xd66f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd66f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd66fc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0xd6704 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd6710 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6714 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6728 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd6734 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xd673c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6740 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6754 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd6760 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xd6764 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd676c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0xd6774 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd6774 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6774 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6774 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd655c | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x54` | `0x0` |
| `bl` at 0xd65a4 | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `[0x2adf9c]` | `?` |
| `bl` at 0xd65dc | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x60` | `0x0` |
| `bl` at 0xd662c | `(unknown callee) 0x1b454` | `[[sp-36]+4]` | `0xc` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xd6654 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6660 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd6690 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd66a0 | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd66cc | `(unknown callee) 0x21eb8` | `0x9` | `0x13525c` | `0x6c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd651c | `[0x2adf98] == 0x0` | `[0x2adf98] != 0x0` |
| `bne` at 0xd6528 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd656c | `[0x2adf9c] != 0x0` | `[0x2adf9c] == 0x0` |
| `bne` at 0xd65b8 | `[0x2adf98] != 0x0` | `[0x2adf98] == 0x0` |
| `bne` at 0xd6640 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `blt` at 0xd66e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd670c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6774 | `r0` | `?` |

## `ioExpPresent`

- `ioExpPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6798 to 0xd68bc.
  Established. The signed comparison takes `(r1 & 0xff) >= [[sp-28]+4]` to `0xd6818` (`bge` at
  0xd67e8); its fall-through includes a byte store with unknown target and value `0x1` (`strb` at
  0xd6810). The later unnamed `0x21eb8` call carries `0x6`, `0x13525c`, `0x82`, `0x0` (`bl` at
  0xd68a0). Presence-record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd67a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd67b0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd67b4 | load | 4 | `abs 0xd68a8` | `-` |
| `ldr` at 0xd67b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd67cc | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd67d4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd67d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd67dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd67e0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd67ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd67f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6804 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd6810 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xd6818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6820 | load | 4 | `abs 0xd68ac` | `-` |
| `ldr` at 0xd6824 | load | 4 | `abs 0xd68b0` | `-` |
| `str` at 0xd6850 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd6854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd685c | load | 4 | `abs 0xd68ac` | `-` |
| `ldr` at 0xd6860 | load | 4 | `abs 0xd68b0` | `-` |
| `ldr` at 0xd686c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd687c | load | 4 | `abs 0xd68b4` | `-` |
| `str` at 0xd6880 | store | 4 | `(unknown)` | `0x1352f4` |
| `str` at 0xd6884 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd6888 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd688c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd6894 | load | 4 | `abs 0xd68b8` | `-` |
| `ldmdb` at 0xd68a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd68a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd68a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6828 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6834 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd6864 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6874 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd68a0 | `(unknown callee) 0x21eb8` | `0x6` | `0x13525c` | `0x82` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd67e8 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd68a4 | `r0` | `?` |

## `ioExpNotPresent`

- `ioExpNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd68bc to 0xd69e0.
  Established. The deciding branch takes `(r1 & 0xff) >= [[sp-28]+4]`, signed 32-bit, to `0xd693c`
  (`bge` at 0xd690c). The other arm clears one byte at an unknown target (`strb` at 0xd6934).
  Included four-byte stores at unknown targets retain `0x135318` and `[sp-21]:1` (`str` at 0xd69a4,
  `str` at 0xd69b0). Absence-record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd68cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd68d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd68d8 | load | 4 | `abs 0xd69cc` | `-` |
| `ldr` at 0xd68dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd68f0 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd68f8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd68fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6900 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6904 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd6910 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6914 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6928 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd6934 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xd693c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6944 | load | 4 | `abs 0xd69d0` | `-` |
| `ldr` at 0xd6948 | load | 4 | `abs 0xd69d4` | `-` |
| `str` at 0xd6974 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd6978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6980 | load | 4 | `abs 0xd69d0` | `-` |
| `ldr` at 0xd6984 | load | 4 | `abs 0xd69d4` | `-` |
| `ldr` at 0xd6990 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd69a0 | load | 4 | `abs 0xd69d8` | `-` |
| `str` at 0xd69a4 | store | 4 | `(unknown)` | `0x135318` |
| `str` at 0xd69a8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd69ac | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd69b0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd69b8 | load | 4 | `abs 0xd69dc` | `-` |
| `ldmdb` at 0xd69c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd69c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd69c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd694c | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6958 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd6988 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6998 | `(unknown callee) 0x1b598` | `[sp-32]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd69c4 | `(unknown callee) 0x21eb8` | `0x6` | `0x13525c` | `0x91` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd690c | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd69c8 | `r0` | `?` |

## `ioExpPreConfig`

- `ioExpPreConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6bb0 to 0xd6cd4.
  Established. A byte read through `ret(0xd8068) +0x0` supplies the zero test that takes the branch
  to `0xd6cc4` (`ldrb` at 0xd6c08, `beq` at 0xd6c10). A later byte store through `ret(0xd8068) +0x1`
  has unknown value (`strb` at 0xd6c2c). The computed transfer takes an unresolved edge for
  `[[sp-28]]:1 <= 0x4`, unsigned 32-bit, and falls through for `[[sp-28]]:1 > 0x4` (`ldrls` at
  0xd6c3c). Selector-specific preconfiguration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6bc0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd6bc8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0xd6bd0 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xd6bd4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6be8 | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd6bec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6bf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6c00 | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd6c04 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6c08 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd6c14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6c18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6c1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6c20 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xd6c24 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd6c2c | store | 1 | `ret(0xd8068) +0x1` | `?` |
| `ldr` at 0xd6c30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6c34 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd6c3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd6c98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6ca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6cac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6cb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6cc0 | store | 4 | `sp -0x24` | `ret(0xd85f4)` |
| `ldmdb` at 0xd6cc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6cc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6cc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6be0 | `(unknown callee) 0xd7f54` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd6bf8 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd6cb8 | `(unknown callee) 0xd85f4` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd6c10 | `[ret(0xd8068)]:1 == 0x0` | `[ret(0xd8068)]:1 != 0x0` |
| `ldrls` at 0xd6c3c | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0xd6ca0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6cc4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd6c3c).

## `ioExpGetCfg`

- `ioExpGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7210 to 0xd7270.
  Established. A four-byte store through `r1 +0x0` retains `[[sp-28]]` (`str` at 0xd7258). The exit
  carries `[[sp-28]+4]`, following the four-byte read at `[sp-28] +0x4` (`ldmdb` at 0xd7268, `ldr`
  at 0xd7260). No calls or conditional branches appear in this span. Configuration record meaning
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7220 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd7224 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd7228 | load | 4 | `abs 0xd726c` | `-` |
| `ldr` at 0xd722c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7240 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd7248 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd724c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7250 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7254 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd7258 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd725c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7260 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd7268 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7268 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7268 | `r0` | `[[sp-28]+4]` |

## `ioExpCount`

- `ioExpCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7270 to 0xd72bc.
  Established. A four-byte load reads absolute `0x2adf98` (`ldr` at 0xd729c). Another four-byte read
  targets `[sp-24] +0x4`, with `[[sp-24]+4]` carried at the exit (`ldr` at 0xd72ac, `ldmdb` at
  0xd72b4). This span has no calls or conditional branches. The returned quantity meaning remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7280 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd7284 | load | 4 | `abs 0xd72b8` | `-` |
| `ldr` at 0xd7288 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd729c | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd72a4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd72a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd72ac | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd72b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd72b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd72b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd72b4 | `r0` | `[[sp-24]+4]` |

## `ioExpIsPresent`

- `ioExpIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd72bc to 0xd731c.
  Established. The unnamed `0xd8068` call has `r0`, `(r1 & 0xff)`, `r2`, `(r1 & 0xff)` (`bl` at
  0xd72e4); its nonzero result takes the branch to `0xd7308` (`bne` at 0xd72f8). That path reads a
  byte through `ret(0xd8068) +0x0` and assigns its value as four bytes at `sp -0x20` (`ldrb` at
  0xd730c, `str` at 0xd7310). Presence encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd72cc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd72d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd72d8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd72dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd72ec | store | 4 | `sp -0x1c` | `ret(0xd8068)` |
| `ldr` at 0xd72f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd7300 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd7308 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd730c | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `str` at 0xd7310 | store | 4 | `sp -0x20` | `[ret(0xd8068)]:1` |
| `ldr` at 0xd7314 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd7318 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7318 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7318 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd72e4 | `(unknown callee) 0xd8068` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd72f8 | `ret(0xd8068) != 0x0` | `ret(0xd8068) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7318 | `r0` | `[sp-32]` |

## `ioExpSetBit`

- `ioExpSetBit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd731c to 0xd7368.
  Established. The unnamed `0xd73b0` call has `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r2 & 0xff)`
  (`bl` at 0xd7358). Included byte assignments retain the entry-r1 and entry-r2 low bytes at
  `sp -0x15` and `sp -0x16` (`strb` at 0xd7334, `strb` at 0xd733c). The exit carries `ret(0xd73b0)`
  (`ldmdb` at 0xd7364). Bit-setting physical effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd732c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7334 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd733c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd7340 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd7344 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xd7348 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd734c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd7364 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7364 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7364 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7358 | `(unknown callee) 0xd73b0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7364 | `r0` | `ret(0xd73b0)` |

## `ioExpClearBit`

- `ioExpClearBit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7368 to 0xd73b0.
  Established. The unnamed `0xd73b0` call carries `r0`, `(r1 & 0xff)`, `(r2 & 0xff)`, `0x0` (`bl` at
  0xd73a0). Two byte stores at `sp -0x15` and `sp -0x16` retain those entry-register low bytes
  (`strb` at 0xd7380, `strb` at 0xd7388); its result reaches the exit (`ldmdb` at 0xd73ac).
  Bit-clear hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7378 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7380 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd7388 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd738c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd7390 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd7394 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd73ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd73ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd73ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd73a0 | `(unknown callee) 0xd73b0` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd73ac | `r0` | `ret(0xd73b0)` |

## `ioExpGetBit`

- `ioExpGetBit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd74b0 to 0xd7530.
  Established. The unnamed `0xd7530` call has `r0`, `(r1 & 0xff)`, `sp-29`, `(r1 & 0xff)` (`bl` at
  0xd74e4); its zero result takes the branch to `0xd7508`, followed by byte reads at `sp -0x1d` and
  `sp -0x16` (`beq` at 0xd74f8, `ldrb` at 0xd7508, `ldrb` at 0xd750c). The merged exit value is
  unknown (`ldmdb` at 0xd752c). Bit-query encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd74c0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd74c8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd74d0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xd74d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd74dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd74ec | store | 4 | `sp -0x1c` | `ret(0xd7530)` |
| `ldr` at 0xd74f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd74fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd7500 | store | 4 | `sp -0x24` | `ret(0xd7530)` |
| `ldrb` at 0xd7508 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd750c | load | 1 | `sp -0x16` | `-` |
| `str` at 0xd7524 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd7528 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd752c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd752c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd752c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd74e4 | `(unknown callee) 0xd7530` | `r0` | `(r1 & 0xff)` | `sp-29` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd74f8 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd752c | `r0` | `?` |

## `ioExpName`

- `ioExpName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7eec to 0xd7f54.
  Established. Two separate four-byte loads read `ret(0xd7f54) +0xc` (`ldr` at 0xd7f24, `ldr` at
  0xd7f30). The nonzero loaded value takes the branch to `0xd7f44` (`bne` at 0xd7f38). The other arm
  assigns four-byte `0x135518` at `sp -0x20` (`str` at 0xd7f40). Name representation and return
  interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7efc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd7f04 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd7f08 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd7f0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd7f1c | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldr` at 0xd7f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7f24 | load | 4 | `ret(0xd7f54) +0xc` | `-` |
| `str` at 0xd7f28 | store | 4 | `sp -0x20` | `[ret(0xd7f54)+12]` |
| `ldr` at 0xd7f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7f30 | load | 4 | `ret(0xd7f54) +0xc` | `-` |
| `ldr` at 0xd7f3c | load | 4 | `abs 0xd7f50` | `-` |
| `str` at 0xd7f40 | store | 4 | `sp -0x20` | `0x135518` |
| `ldr` at 0xd7f44 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd7f4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd7f4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd7f4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd7f14 | `(unknown callee) 0xd7f54` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd7f38 | `[ret(0xd7f54)+12] != 0x0` | `[ret(0xd7f54)+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd7f4c | `r0` | `?` |

## `env_pwrdwn`

- `env_pwrdwn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7c3a8 to 0x7c5cc.
  Established. Two included four-byte assignments at absolute `0x143594` write `0x0` and `0x1`
  (`str` at 0x7c57c, `str` at 0x7c590). The unnamed `0x3d890` call has `0x4`, unknown, `0x143594`,
  `0x0` (`bl` at 0x7c584). The loop branch takes `[sp-128] > 0x0`, signed 32-bit, to `0x7c400`
  (`bgt` at 0x7c538). Power-down timing and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7c3b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7c3bc | load | 4 | `abs 0x7c5a0` | `-` |
| `ldm` at 0x7c3c8 | load | 4 | `abs 0x1113bc` | `-` |
| `ldm` at 0x7c3c8 | load | 4 | `abs 0x1113c0` | `-` |
| `ldm` at 0x7c3c8 | load | 4 | `abs 0x1113c4` | `-` |
| `ldm` at 0x7c3c8 | load | 4 | `abs 0x1113c8` | `-` |
| `stm` at 0x7c3cc | store | 4 | `sp -0x2c` | `[0x1113bc]` |
| `stm` at 0x7c3cc | store | 4 | `sp -0x28` | `[0x1113c0]` |
| `stm` at 0x7c3cc | store | 4 | `sp -0x24` | `[0x1113c4]` |
| `stm` at 0x7c3cc | store | 4 | `sp -0x20` | `[0x1113c8]` |
| `ldm` at 0x7c3d0 | load | 4 | `abs 0x1113cc` | `-` |
| `ldm` at 0x7c3d0 | load | 4 | `abs 0x1113d0` | `-` |
| `stm` at 0x7c3d4 | store | 4 | `sp -0x1c` | `[0x1113cc]` |
| `stm` at 0x7c3d4 | store | 4 | `sp -0x18` | `[0x1113d0]` |
| `str` at 0x7c3dc | store | 4 | `sp -0x80` | `0x1e` |
| `ldr` at 0x7c3e0 | load | 4 | `abs 0x7c5a4` | `-` |
| `ldr` at 0x7c408 | load | 4 | `abs 0x7c5a8` | `-` |
| `ldr` at 0x7c40c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7c414 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7c418 | load | 4 | `abs 0x7c5ac` | `-` |
| `str` at 0x7c44c | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7c454 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7c45c | load | 4 | `abs 0x7c5b0` | `-` |
| `ldr` at 0x7c460 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7c46c | load | 4 | `sp -0x80` | `-` |
| `str` at 0x7c474 | store | 4 | `sp -0x80` | `[sp-128]-5` |
| `ldr` at 0x7c498 | load | 4 | `abs 0x7c5b4` | `-` |
| `str` at 0x7c4a8 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7c4b0 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7c4b8 | load | 4 | `abs 0x7c5b8` | `-` |
| `ldr` at 0x7c4bc | load | 4 | `abs 0x7c5bc` | `-` |
| `ldr` at 0x7c4c8 | load | 4 | `abs 0x7c5a4` | `-` |
| `ldr` at 0x7c4d4 | load | 4 | `abs 0x7c5a4` | `-` |
| `ldr` at 0x7c4f4 | load | 4 | `abs 0x7c5c0` | `-` |
| `str` at 0x7c504 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7c50c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7c514 | load | 4 | `abs 0x7c5b8` | `-` |
| `ldr` at 0x7c518 | load | 4 | `abs 0x7c5bc` | `-` |
| `ldr` at 0x7c524 | load | 4 | `abs 0x7c5a4` | `-` |
| `ldr` at 0x7c530 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7c544 | load | 4 | `abs 0x7c5c4` | `-` |
| `str` at 0x7c554 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7c55c | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7c564 | load | 4 | `abs 0x7c5b8` | `-` |
| `ldr` at 0x7c574 | load | 4 | `abs 0x7c5c8` | `-` |
| `str` at 0x7c57c | store | 4 | `abs 0x143594` | `0x0` |
| `ldr` at 0x7c588 | load | 4 | `abs 0x7c5c8` | `-` |
| `str` at 0x7c590 | store | 4 | `abs 0x143594` | `0x1` |
| `ldr` at 0x7c594 | load | 4 | `abs 0x7c5a4` | `-` |
| `ldmdb` at 0x7c59c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7c59c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7c59c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7c3e8 | `(unknown callee) 0x22ac8` | `0x149df8` | `0x0` | `[0x1113c4]` | `0x1e` |
| `bl` at 0x7c410 | `(unknown callee) 0x1b52c` | `sp-124` | `0x1113d4` | `[sp-128]` | `sp-124` |
| `bl` at 0x7c468 | `(unknown callee) 0x7aad8` | `0x2` | `0x4101` | `?` | `sp-124` |
| `bl` at 0x7c47c | `(unknown callee) 0x82e94` | `0x1f4` | `?` | `?` | `[sp-128]-5` |
| `bl` at 0x7c480 | `(unknown callee) 0x3dce4` | `ret(0x82e94)` | `?` | `?` | `?` |
| `bl` at 0x7c49c | `(unknown callee) 0x1b52c` | `sp-124` | `0x1113f8` | `?` | `sp-124` |
| `bl` at 0x7c4c4 | `(unknown callee) 0x7aad8` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x7c4cc | `(unknown callee) 0x22e50` | `0x149df8` | `?` | `?` | `?` |
| `bl` at 0x7c4dc | `(unknown callee) 0x22ac8` | `0x149df8` | `0x0` | `?` | `ret(0x3dce4)` |
| `bl` at 0x7c4f8 | `(unknown callee) 0x1b52c` | `sp-124` | `0x111420` | `?` | `sp-124` |
| `bl` at 0x7c520 | `(unknown callee) 0x7aad8` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x7c528 | `(unknown callee) 0x22e50` | `0x149df8` | `?` | `?` | `?` |
| `bl` at 0x7c548 | `(unknown callee) 0x1b52c` | `sp-124` | `0x111450` | `?` | `sp-124` |
| `bl` at 0x7c570 | `(unknown callee) 0x7aad8` | `0x2` | `0x4102` | `0x164` | `sp-124` |
| `bl` at 0x7c584 | `(unknown callee) 0x3d890` | `0x4` | `?` | `0x143594` | `0x0` |
| `bl` at 0x7c598 | `(unknown callee) 0x22e50` | `0x149df8` | `?` | `0x143594` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7c3f4 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `bne` at 0x7c48c | `ret(0x3dce4) != 0x0` | `ret(0x3dce4) == 0x0` |
| `bne` at 0x7c4e8 | `ret(0x22ac8) != 0x0` | `ret(0x22ac8) == 0x0` |
| `bgt` at 0x7c538 | `[sp-128] > 0x0 (signed, 32-bit)` | `[sp-128] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7c59c | `r0` | `?` |

## `APwrInit`

- `APwrInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb29e8 to 0xb2c0c.
  Established. Included equality branches take `[sp-28] == 0x43` and `[sp-28] == 0x33` to `0xb2af0`
  (`beq` at 0xb2a10, `beq` at 0xb2a1c). The unnamed `0xb4b18` call has `0x2b2940`, `0xffff014`,
  `0x2`, `0x2b2948` (`bl` at 0xb2b80). Four-byte stack assignments retain `0x4b0` at `sp -0x2c` and
  `0x9` at `sp -0x20` (`str` at 0xb2bb0, `str` at 0xb2bc8). Power task roles and selector meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb2a04 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb2a08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2a98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2aa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ac8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2ae0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2b08 | load | 4 | `abs 0xb2be4` | `-` |
| `ldr` at 0xb2b30 | load | 4 | `abs 0xb2be8` | `-` |
| `ldr` at 0xb2b38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2b40 | load | 4 | `abs 0xb2bec` | `-` |
| `ldr` at 0xb2b44 | load | 4 | `abs 0xb2bf0` | `-` |
| `str` at 0xb2b54 | store | 4 | `sp -0x18` | `ret(0xb5f4c)` |
| `ldr` at 0xb2b58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb2b64 | load | 4 | `abs 0xb2bf4` | `-` |
| `ldr` at 0xb2b70 | load | 4 | `abs 0xb2be4` | `-` |
| `ldr` at 0xb2b74 | load | 4 | `abs 0xb2bf8` | `-` |
| `ldr` at 0xb2b7c | load | 4 | `abs 0xb2bec` | `-` |
| `str` at 0xb2b88 | store | 4 | `sp -0x18` | `ret(0xb4b18)` |
| `ldr` at 0xb2b8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb2b98 | load | 4 | `abs 0xb2bfc` | `-` |
| `ldr` at 0xb2ba4 | load | 4 | `abs 0xb2be4` | `-` |
| `str` at 0xb2ba8 | store | 4 | `sp -0x30` | `0x2b2940` |
| `str` at 0xb2bb0 | store | 4 | `sp -0x2c` | `0x4b0` |
| `str` at 0xb2bb8 | store | 4 | `sp -0x28` | `0x19` |
| `str` at 0xb2bc0 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xb2bc8 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xb2bcc | load | 4 | `abs 0xb2c00` | `-` |
| `ldr` at 0xb2bd0 | load | 4 | `abs 0xb2c04` | `-` |
| `ldr` at 0xb2bd4 | load | 4 | `abs 0xb2c08` | `-` |
| `ldmdb` at 0xb2be0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb2be0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb2be0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb2be0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb29f8 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb2af4 | `(unknown callee) 0x1f26c` | `0x32` | `?` | `?` | `[sp-28]` |
| `bl` at 0xb2b14 | `(unknown callee) 0xb5e34` | `0x2b2940` | `0x14` | `0x0` | `ret(0x1f26c)` |
| `bl` at 0xb2b1c | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xb2b28 | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xb2b3c | `(unknown callee) 0x6a5c8` | `0x14a838` | `ret(0x1f26c)` | `[sp-28]` | `ret(0x1f26c)` |
| `bl` at 0xb2b4c | `(unknown callee) 0xb5f4c` | `0x2b2948` | `0x12859c` | `0x1` | `?` |
| `bl` at 0xb2b68 | `(unknown callee) 0x2079c` | `0x1285a8` | `?` | `?` | `ret(0xb5f4c)` |
| `bl` at 0xb2b80 | `(unknown callee) 0xb4b18` | `0x2b2940` | `0xffff014` | `0x2` | `0x2b2948` |
| `bl` at 0xb2b9c | `(unknown callee) 0x2079c` | `0x1285d4` | `?` | `?` | `ret(0xb4b18)` |
| `bl` at 0xb2bdc | `(unknown callee) 0x22480` | `0x2b29e0` | `0x128600` | `0xb2c0c` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb2a10 | `[sp-28] == 0x43` | `[sp-28] != 0x43` |
| `beq` at 0xb2a1c | `[sp-28] == 0x33` | `[sp-28] != 0x33` |
| `beq` at 0xb2a28 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `beq` at 0xb2a34 | `[sp-28] == 0x34` | `[sp-28] != 0x34` |
| `beq` at 0xb2a40 | `[sp-28] == 0x41` | `[sp-28] != 0x41` |
| `beq` at 0xb2a4c | `[sp-28] == 0x36` | `[sp-28] != 0x36` |
| `beq` at 0xb2a58 | `[sp-28] == 0x39` | `[sp-28] != 0x39` |
| `beq` at 0xb2a64 | `[sp-28] == 0x37` | `[sp-28] != 0x37` |
| `beq` at 0xb2a70 | `[sp-28] == 0x38` | `[sp-28] != 0x38` |
| `beq` at 0xb2a7c | `[sp-28] == 0x30` | `[sp-28] != 0x30` |
| `beq` at 0xb2a88 | `[sp-28] == 0x51` | `[sp-28] != 0x51` |
| `beq` at 0xb2a94 | `[sp-28] == 0x40` | `[sp-28] != 0x40` |
| `beq` at 0xb2aa0 | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0xb2aac | `[sp-28] == 0x42` | `[sp-28] != 0x42` |
| `beq` at 0xb2ab8 | `[sp-28] == 0x46` | `[sp-28] != 0x46` |
| `beq` at 0xb2ac4 | `[sp-28] == 0x48` | `[sp-28] != 0x48` |
| `beq` at 0xb2ad0 | `[sp-28] == 0x4c` | `[sp-28] != 0x4c` |
| `beq` at 0xb2adc | `[sp-28] == 0x4d` | `[sp-28] != 0x4d` |
| `beq` at 0xb2ae8 | `[sp-28] == 0x7c` | `[sp-28] != 0x7c` |
| `bne` at 0xb2b00 | `ret(0x1f26c) != 0x0` | `ret(0x1f26c) == 0x0` |
| `beq` at 0xb2b60 | `ret(0xb5f4c) == 0x0` | `ret(0xb5f4c) != 0x0` |
| `beq` at 0xb2b94 | `ret(0xb4b18) == 0x0` | `ret(0xb4b18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb2be0 | `r0` | `?` |

## `smpcmd_ioexp`

- `smpcmd_ioexp`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb24d4 to 0xb26a4.
  Established. A four-byte assignment through `[sp-28] +0x0` retains `ret(0x1b1b4)` (`str` at
  0xb2524). The unnamed `0xd7530` call carries `0x0`, `[sp-36]:1`, `sp-45`, `[sp-36]:1` (`bl` at
  0xb25f0); its zero result takes the branch to `0xb2628` (`beq` at 0xb2604). A later byte load
  reads `sp -0x2d` (`ldrb` at 0xb2640). Expander command formatting and status meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb24e4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb24e8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb24f8 | store | 4 | `sp -0x2c` | `ret(0xd7270)` |
| `ldr` at 0xb24fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2500 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb2524 | store | 4 | `[sp-28] +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb2528 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xb252c | store | 4 | `sp -0x28` | `[[sp-28]]` |
| `ldr` at 0xb2530 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2534 | load | 4 | `abs 0xb2690` | `-` |
| `ldr` at 0xb253c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb254c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb2554 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `str` at 0xb255c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb2560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb2564 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb2574 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0xb2588 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb258c | load | 4 | `abs 0xb2694` | `-` |
| `ldr` at 0xb2590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb2598 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb25a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb25b0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldrb` at 0xb25b4 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0xb25d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb25d4 | load | 4 | `abs 0xb2698` | `-` |
| `ldrb` at 0xb25e0 | load | 1 | `sp -0x24` | `-` |
| `str` at 0xb25f8 | store | 4 | `sp -0x20` | `ret(0xd7530)` |
| `ldr` at 0xb25fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2608 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb2614 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2618 | load | 4 | `abs 0xb269c` | `-` |
| `ldrb` at 0xb2628 | load | 1 | `sp -0x24` | `-` |
| `ldrb` at 0xb2640 | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xb2644 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2648 | load | 4 | `abs 0xb26a0` | `-` |
| `ldr` at 0xb2658 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb2668 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb2670 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0xb2674 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb267c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xb268c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb268c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb268c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb268c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb24f0 | `(unknown callee) 0xd7270` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb251c | `(unknown callee) 0x1b1b4` | `?` | `?` | `ret(0xd7270)` | `?` |
| `bl` at 0xb2538 | `(unknown callee) 0x1b52c` | `[[sp-28]]` | `0x128468` | `?` | `[[sp-28]]` |
| `bl` at 0xb2540 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb2580 | `(unknown callee) 0xd7eec` | `0x0` | `[sp-36]:1` | `?` | `[sp-36]:1` |
| `bl` at 0xb2594 | `(unknown callee) 0x1b52c` | `[sp-40]` | `0x1284b4` | `[sp-36]` | `ret(0xd7eec)` |
| `bl` at 0xb259c | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xb25c0 | `(unknown callee) 0xd72bc` | `0x0` | `[sp-36]:1` | `ret(0x1b2bc)` | `[sp-36]:1` |
| `bl` at 0xb25d8 | `(unknown callee) 0x1b52c` | `[sp-40]` | `0x1284c4` | `?` | `ret(0xd72bc)` |
| `bl` at 0xb25f0 | `(unknown callee) 0xd7530` | `0x0` | `[sp-36]:1` | `sp-45` | `[sp-36]:1` |
| `bl` at 0xb260c | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0xb2620 | `(unknown callee) 0x1b52c` | `[sp-40]` | `0x1284d4` | `ret(0x21a38)` | `ret(0x21a38)` |
| `bl` at 0xb2634 | `(unknown callee) 0xd7648` | `0x0` | `[sp-36]:1` | `?` | `[sp-36]:1` |
| `bl` at 0xb2654 | `(unknown callee) 0x1b52c` | `[sp-40]` | `0x1284f0` | `[sp-45]:1` | `?` |
| `bl` at 0xb265c | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb256c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xb25cc | `ret(0xd72bc) != 0x0` | `ret(0xd72bc) == 0x0` |
| `beq` at 0xb2604 | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb268c | `r0` | `0x0` |

## `JChip_protect_field`

- `JChip_protect_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec1ac to 0xec33c.
  Established. The unnamed `0xf2040` call has unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at
  0xec2bc). A zero `[sp-32]` takes the branch to `0xec2c0` (`beq` at 0xec298). The later byte load
  and store have unknown targets and value (`ldrb` at 0xec308, `strb` at 0xec310). Field-protection
  encoding and peripheral effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec1bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec1c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec1c4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec1c8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec1cc | load | 4 | `abs 0xec324` | `-` |
| `ldr` at 0xec1d0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec1dc | load | 4 | `abs 0xec328` | `-` |
| `ldr` at 0xec1e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec1e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec1e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec1f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec1fc | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xec200 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec20c | load | 4 | `abs 0xec32c` | `-` |
| `str` at 0xec210 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xec218 | load | 4 | `abs 0xec324` | `-` |
| `ldr` at 0xec21c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec228 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec22c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec234 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec23c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec240 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec258 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec260 | load | 4 | `abs 0xec330` | `-` |
| `ldr` at 0xec264 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec26c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec278 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec27c | load | 4 | `abs 0xec334` | `-` |
| `ldr` at 0xec280 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xec288 | load | 4 | `abs 0xec338` | `-` |
| `ldr` at 0xec290 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec29c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec2a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec2a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec2b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec2c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec2c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec2cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec2e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec2f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec2fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec300 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xec308 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xec310 | store | 1 | `(unknown)` | `?` |
| `str` at 0xec318 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xec31c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xec320 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec320 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec1ec | `(unknown callee) 0x23fbc` | `0x1372c0` | `r0` | `r1` | `r2` |
| `bl` at 0xec1f4 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xec268 | `(unknown callee) 0x23fbc` | `0x1372f8` | `?` | `?` | `?` |
| `bl` at 0xec284 | `(unknown callee) 0x23fbc` | `0x137318` | `[[sp-32]]` | `?` | `[sp-32]` |
| `bl` at 0xec28c | `(unknown callee) 0x23fbc` | `0x13732c` | `?` | `?` | `?` |
| `bl` at 0xec2bc | `(unknown callee) 0xf2040` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec1d8 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec208 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `bls` at 0xec224 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xec274 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xec298 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec320 | `r0` | `?` |
