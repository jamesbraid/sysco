# ARM scan field and definition helpers in 1.40.5

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
These routines have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-field-facts.txt) retain all tool sections.

## `JChip_get_field`

- `JChip_get_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec33c to 0xec468.
  Established. The unnamed `0xf0de4` call has unknown, `[sp-28]`, `0x0`, unknown (`bl` at 0xec444).
  An included four-byte stack assignment at `sp -0x2c` retains `[sp-32]` (`str` at 0xec430). The
  unsigned branch takes `[0x2adfd8] <= 0x3` to `0xec3fc` (`bls` at 0xec3b4). Field extraction layout
  and output meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec34c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec350 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec354 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec358 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec35c | load | 4 | `abs 0xec458` | `-` |
| `ldr` at 0xec360 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec36c | load | 4 | `abs 0xec45c` | `-` |
| `ldr` at 0xec370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec378 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec38c | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xec390 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec39c | load | 4 | `abs 0xec460` | `-` |
| `str` at 0xec3a0 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xec3a8 | load | 4 | `abs 0xec458` | `-` |
| `ldr` at 0xec3ac | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec3b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec3bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec3c4 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec3cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec3d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec3d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec3e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec3f0 | load | 4 | `abs 0xec464` | `-` |
| `ldr` at 0xec3f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec3fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec408 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec410 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec414 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec420 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec428 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec42c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec430 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xec434 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec440 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec44c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xec450 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xec454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec37c | `(unknown callee) 0x23fbc` | `0x137330` | `r0` | `r1` | `r2` |
| `bl` at 0xec384 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xec3f8 | `(unknown callee) 0x23fbc` | `0x137364` | `?` | `?` | `?` |
| `bl` at 0xec444 | `(unknown callee) 0xf0de4` | `?` | `[sp-28]` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec368 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec398 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `bls` at 0xec3b4 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec454 | `r0` | `?` |

## `JChip_get_field_str`

- `JChip_get_field_str`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec5e8 to 0xec7c0.
  Established. A halfword load has unknown effective target, with its value assigned as four bytes
  at `sp -0x28` (`ldrh` at 0xec6e0, `str` at 0xec6e4). The unnamed `0x1b1b4` call has four unknown
  candidates (`bl` at 0xec714); its nonzero result takes the branch to `0xec738` (`bne` at 0xec728).
  The later unnamed `0xf1a10` call carries `[sp-40]`, `[sp-44]`, `[sp-32]`, unknown (`bl` at
  0xec790). Field string representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec5f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec5fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec600 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec604 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec608 | load | 4 | `abs 0xec7ac` | `-` |
| `ldr` at 0xec60c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec618 | load | 4 | `abs 0xec7b0` | `-` |
| `ldr` at 0xec61c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec62c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec638 | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xec63c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec648 | load | 4 | `abs 0xec7b4` | `-` |
| `str` at 0xec64c | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec654 | load | 4 | `abs 0xec7ac` | `-` |
| `ldr` at 0xec658 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec664 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec668 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec670 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec678 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec67c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec680 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec694 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec69c | load | 4 | `abs 0xec7b8` | `-` |
| `ldr` at 0xec6a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec6a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec6ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec6b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec6bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec6c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec6c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec6d8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xec6e0 | load | 2 | `(unknown)` | `-` |
| `str` at 0xec6e4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xec6e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec700 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xec71c | store | 4 | `sp -0x2c` | `ret(0x1b1b4)` |
| `ldr` at 0xec720 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xec72c | load | 4 | `abs 0xec7bc` | `-` |
| `str` at 0xec730 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xec738 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec73c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec744 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec74c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec750 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec754 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec75c | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec768 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xec76c | store | 4 | `sp -0x34` | `ret(0x1b1b4)` |
| `ldr` at 0xec770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec774 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec77c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec784 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec788 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xec78c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec794 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xec7a0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xec7a4 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xec7a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec7a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec7a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec628 | `(unknown callee) 0x23fbc` | `0x1373c0` | `r0` | `r1` | `r2` |
| `bl` at 0xec630 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xec6a4 | `(unknown callee) 0x23fbc` | `0x1373f8` | `?` | `?` | `?` |
| `bl` at 0xec714 | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xec780 | `(unknown callee) 0xf0de4` | `?` | `[sp-28]` | `0x0` | `?` |
| `bl` at 0xec790 | `(unknown callee) 0xf1a10` | `[sp-40]` | `[sp-44]` | `[sp-32]` | `?` |
| `bl` at 0xec798 | `(unknown callee) 0x1b490` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec614 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec644 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `bls` at 0xec660 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xec728 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec7a8 | `r0` | `?` |

## `JChip_get_field_by_name`

- `JChip_get_field_by_name`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec468 to 0xec5e8.
  Established. A halfword load targets `[sp-44] +0x2` (`ldrh` at 0xec50c). The unsigned branch takes
  `ret(0xf10c8) < [[sp-44]+2]:2` to `0xec528` (`blo` at 0xec518). The later unnamed `0xf0de4` call
  has unknown, unknown, `0x0`, unknown (`bl` at 0xec5c4). Name lookup and output-field semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec478 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec47c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec480 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec484 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec488 | load | 4 | `abs 0xec5d8` | `-` |
| `ldr` at 0xec48c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec498 | load | 4 | `abs 0xec5dc` | `-` |
| `ldr` at 0xec49c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec4a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec4ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec4b8 | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xec4bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec4c8 | load | 4 | `abs 0xec5e0` | `-` |
| `str` at 0xec4cc | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec4d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec4d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec4e0 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec4e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec4ec | load | 4 | `(unknown)` | `-` |
| `str` at 0xec4f0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xec4f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xec4f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec504 | store | 4 | `sp -0x28` | `ret(0xf10c8)` |
| `ldr` at 0xec508 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xec50c | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xec510 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec51c | load | 4 | `abs 0xec5e0` | `-` |
| `str` at 0xec520 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec528 | load | 4 | `abs 0xec5d8` | `-` |
| `ldr` at 0xec52c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec538 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec53c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec544 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec54c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec554 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec570 | load | 4 | `abs 0xec5e4` | `-` |
| `ldr` at 0xec574 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec57c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec580 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec588 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec590 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec594 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec5a0 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec5a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec5ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec5b0 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0xec5b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec5b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec5c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec5cc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xec5d0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xec5d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec5d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec5d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec4a8 | `(unknown callee) 0x23fbc` | `0x137384` | `r0` | `r1` | `r2` |
| `bl` at 0xec4b0 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xec4fc | `(unknown callee) 0xf10c8` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xec578 | `(unknown callee) 0x23fbc` | `0x137364` | `?` | `?` | `?` |
| `bl` at 0xec5c4 | `(unknown callee) 0xf0de4` | `?` | `?` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec494 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec4c4 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `blo` at 0xec518 | `ret(0xf10c8) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xf10c8) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `bls` at 0xec534 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec5d4 | `r0` | `?` |

## `JChip_can_override_field`

- `JChip_can_override_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec7c0 to 0xec86c.
  Established. The nonzero result from unnamed `0xebc18` takes the branch to `0xec828` (`bl` at
  0xec804, `bne` at 0xec818). That path includes a byte read at an unknown target and a four-byte
  stack assignment at `sp -0x24` with unknown value (`ldrb` at 0xec84c, `str` at 0xec858). Override
  permission meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec7d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec7d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec7d8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xec7dc | load | 4 | `abs 0xec864` | `-` |
| `ldr` at 0xec7e0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec7ec | load | 4 | `abs 0xec868` | `-` |
| `ldr` at 0xec7f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec7f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec800 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec80c | store | 4 | `sp -0x20` | `ret(0xebc18)` |
| `ldr` at 0xec810 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec820 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xec828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec82c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec834 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec83c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec840 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec844 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xec84c | load | 1 | `(unknown)` | `-` |
| `str` at 0xec858 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xec85c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xec860 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec860 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec860 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec7fc | `(unknown callee) 0x23fbc` | `0x13741c` | `r0` | `r1` | `r2` |
| `bl` at 0xec804 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec7e8 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec818 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec860 | `r0` | `?` |

## `JChip_override_field`

- `JChip_override_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec86c to 0xec9c4.
  Established. The unnamed `0xf2040` call has unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at
  0xec9a0). A prior byte read has unknown effective target, and the following branch takes `? == 0x0` to `0xec924` (`ldrb` at 0xec908, `beq` at 0xec914). The included four-byte stack assignment
  at `sp -0x28` retains `0xfffff7c6` (`str` at 0xec91c). Override policy and field encoding remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec87c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec880 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec884 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec888 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec88c | load | 4 | `abs 0xec9b4` | `-` |
| `ldr` at 0xec890 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec89c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec8a0 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xec8a4 | store | 4 | `sp -0x2c` | `[r3]` |
| `ldr` at 0xec8a8 | load | 4 | `abs 0xec9b8` | `-` |
| `ldr` at 0xec8ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec8b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec8b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec8bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec8c8 | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xec8cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec8d8 | load | 4 | `abs 0xec9bc` | `-` |
| `str` at 0xec8dc | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xec8e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec8e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec8f0 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec8f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec8fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec900 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xec908 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec918 | load | 4 | `abs 0xec9bc` | `-` |
| `str` at 0xec91c | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xec924 | load | 4 | `abs 0xec9b4` | `-` |
| `ldr` at 0xec928 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec934 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec940 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xec948 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec96c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec970 | load | 4 | `abs 0xec9c0` | `-` |
| `ldr` at 0xec974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec978 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xec980 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec98c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec994 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec99c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec9a8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xec9ac | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xec9b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec9b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec9b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec8b8 | `(unknown callee) 0x23fbc` | `0x137458` | `r0` | `r1` | `r2` |
| `bl` at 0xec8c0 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xec97c | `(unknown callee) 0x23fbc` | `0x1374a0` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xec9a0 | `(unknown callee) 0xf2040` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec898 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec8d4 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xec914 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xec930 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec9b0 | `r0` | `?` |

## `JChip_override_field_by_name`

- `JChip_override_field_by_name`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec9c4 to 0xecb70.
  Established. The unnamed `0xf10c8` call carries `[sp-44]`, `[sp-28]`, unknown, unknown (`bl` at
  0xeca64); its result below `[[sp-44]+2]:2`, unsigned 32-bit, takes the branch to `0xeca90` (`blo`
  at 0xeca80). The later unnamed `0xf2040` call has unknown, unknown, `[sp-32]`, unknown (`bl` at
  0xecb4c). Name-based field selection semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec9d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec9d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec9dc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec9e0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec9e4 | load | 4 | `abs 0xecb60` | `-` |
| `ldr` at 0xec9e8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xec9f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec9f8 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xec9fc | store | 4 | `sp -0x34` | `[r3]` |
| `ldr` at 0xeca00 | load | 4 | `abs 0xecb64` | `-` |
| `ldr` at 0xeca04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeca08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeca0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeca14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeca20 | store | 4 | `sp -0x24` | `ret(0xebc18)` |
| `ldr` at 0xeca24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeca30 | load | 4 | `abs 0xecb68` | `-` |
| `str` at 0xeca34 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xeca3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeca40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeca48 | load | 4 | `ret(0xebc18) +0x28` | `-` |
| `ldr` at 0xeca50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeca54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeca58 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xeca5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeca60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeca6c | store | 4 | `sp -0x28` | `ret(0xf10c8)` |
| `ldr` at 0xeca70 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xeca74 | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xeca78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeca84 | load | 4 | `abs 0xecb68` | `-` |
| `str` at 0xeca88 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xeca90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeca94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeca9c | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xecaa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecaa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecaac | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xecab4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xecac4 | load | 4 | `abs 0xecb68` | `-` |
| `str` at 0xecac8 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xecad0 | load | 4 | `abs 0xecb60` | `-` |
| `ldr` at 0xecad4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xecae0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xecae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecaec | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xecaf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecaf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecafc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xecb10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecb18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xecb1c | load | 4 | `abs 0xecb6c` | `-` |
| `ldr` at 0xecb20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecb24 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xecb2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xecb30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecb38 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xecb40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xecb44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xecb48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xecb54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xecb58 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xecb5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xecb5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xecb5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeca10 | `(unknown callee) 0x23fbc` | `0x1374d4` | `r0` | `r1` | `r2` |
| `bl` at 0xeca18 | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `?` |
| `bl` at 0xeca64 | `(unknown callee) 0xf10c8` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xecb28 | `(unknown callee) 0x23fbc` | `0x1374a0` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xecb4c | `(unknown callee) 0xf2040` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec9f0 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeca2c | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `blo` at 0xeca80 | `ret(0xf10c8) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xf10c8) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xecac0 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xecadc | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xecb5c | `r0` | `?` |

## `tdrfld_create`

- `tdrfld_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0640 to 0xf06b8.
  Established. The unnamed `0x1b1b4` call carries `0xc`, `r1`, `r2`, `r3` (`bl` at 0xf0658); its
  nonzero result takes the branch to `0xf067c`, followed by the unnamed `0xf06b8` call with
  `ret(0x1b1b4)`, `[sp-20]`, unknown, `ret(0x1b1b4)` (`bne` at 0xf066c, `bl` at 0xf0684). A later
  four-byte stack assignment at `sp -0x1c` retains `[sp-24]` (`str` at 0xf06ac). Field allocation
  and ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0650 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0660 | store | 4 | `sp -0x18` | `ret(0x1b1b4)` |
| `ldr` at 0xf0664 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf0674 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf067c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0694 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf06a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf06a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf06ac | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xf06b0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xf06b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf06b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf06b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0658 | `(unknown callee) 0x1b1b4` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xf0684 | `(unknown callee) 0xf06b8` | `ret(0x1b1b4)` | `[sp-20]` | `?` | `ret(0x1b1b4)` |
| `bl` at 0xf0698 | `(unknown callee) 0x1b490` | `[sp-24]` | `?` | `?` | `ret(0xf06b8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf066c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xf0690 | `ret(0xf06b8) != 0x0` | `ret(0xf06b8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf06b4 | `r0` | `?` |

## `tdrfld_init`

- `tdrfld_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf06b8 to 0xf0748.
  Established. Zero entry `r1` takes the branch to `0xf0708`, followed by four-byte `0x0` through
  `r0 +0x0` (`beq` at 0xf06d8, `str` at 0xf0710). Included halfword stores at `[sp-24] +0x4` and
  `[sp-24] +0x6` and a byte store at `[sp-24] +0x8` each write `0x0` (`strh` at 0xf071c, `strh` at
  0xf0728, `strb` at 0xf0734). Field record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf06c8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf06cc | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf06d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf06dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf06e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf06e8 | store | 4 | `r0 +0x0` | `ret(0x1b034)` |
| `ldr` at 0xf06ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf06f0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf0700 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf0708 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf0710 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xf0714 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xf071c | store | 2 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0xf0720 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xf0728 | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xf072c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xf0734 | store | 1 | `[sp-24] +0x8` | `0x0` |
| `str` at 0xf073c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xf0740 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xf0744 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf0744 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0744 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0744 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf06e4 | `(unknown callee) 0x1b034` | `r1` | `r1` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf06d8 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xf06f8 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0744 | `r0` | `?` |

## `tdrfld_dump`

- `tdrfld_dump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0748 to 0xf085c.
  Established. Two halfword loads read `[sp-28] +0x4` and `[sp-28] +0x6` (`ldrh` at 0xf07f0, `ldrh`
  at 0xf0808). The following unnamed `0x240b8` calls have `[sp-20]`, `0x137bc8`, `[[sp-28]+4]:2`,
  `[[sp-28]+4]:2`, and `[sp-20]`, `0x137bd4`, `[[sp-28]+6]:2`, `[[sp-28]+6]:2` (`bl` at 0xf0800,
  `bl` at 0xf0818). Dump formatting and field units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0758 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf075c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0760 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf0764 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0780 | load | 4 | `abs 0xf083c` | `-` |
| `ldr` at 0xf0784 | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xf0788 | store | 4 | `sp -0x14` | `[0x145ae4]` |
| `ldr` at 0xf078c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0798 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf079c | load | 4 | `abs 0xf0840` | `-` |
| `ldr` at 0xf07a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf07ac | load | 4 | `abs 0xf0844` | `-` |
| `ldr` at 0xf07b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf07b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf07bc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xf07c0 | store | 4 | `sp -0x20` | `[[sp-28]]` |
| `ldr` at 0xf07c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf07c8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf07d4 | load | 4 | `abs 0xf0848` | `-` |
| `str` at 0xf07d8 | store | 4 | `sp -0x20` | `0x137bb4` |
| `ldr` at 0xf07dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf07e0 | load | 4 | `abs 0xf084c` | `-` |
| `ldr` at 0xf07e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf07ec | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf07f0 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xf07f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf07f8 | load | 4 | `abs 0xf0850` | `-` |
| `ldr` at 0xf0804 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0808 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xf080c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0810 | load | 4 | `abs 0xf0854` | `-` |
| `ldr` at 0xf081c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf0820 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xf0824 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf0828 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xf082c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0830 | load | 4 | `abs 0xf0858` | `-` |
| `ldmdb` at 0xf0838 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0838 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf07a0 | `(unknown callee) 0x240b8` | `?` | `0x137b90` | `r2` | `r1` |
| `bl` at 0xf07b4 | `(unknown callee) 0x240b8` | `?` | `0x137bb0` | `r1` | `r1` |
| `bl` at 0xf07e8 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137bbc` | `?` | `?` |
| `bl` at 0xf0800 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137bc8` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xf0818 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137bd4` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xf0834 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137be0` | `[[sp-28]+8]:1` | `[[sp-28]+8]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf076c | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xf077c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf0794 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xf07d0 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0838 | `r0` | `?` |

## `tdrdef_create`

- `tdrdef_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf085c to 0xf08dc.
  Established. The unnamed `0x1b1b4` call uses `0xc`, `r1`, `r2`, `r3` (`bl` at 0xf0878); its
  nonzero result takes the branch to `0xf089c` (`bne` at 0xf088c). The unnamed `0xf08dc` call
  carries `ret(0x1b1b4)`, `[sp-20]`, `[sp-24]`, `ret(0x1b1b4)` (`bl` at 0xf08a8). Definition
  allocation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf086c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0870 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0880 | store | 4 | `sp -0x1c` | `ret(0x1b1b4)` |
| `ldr` at 0xf0884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf0894 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf089c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf08a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf08a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf08b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf08c4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf08cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf08d0 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xf08d4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xf08d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf08d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf08d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0878 | `(unknown callee) 0x1b1b4` | `0xc` | `r1` | `r2` | `r3` |
| `bl` at 0xf08a8 | `(unknown callee) 0xf08dc` | `ret(0x1b1b4)` | `[sp-20]` | `[sp-24]` | `ret(0x1b1b4)` |
| `bl` at 0xf08bc | `(unknown callee) 0x1b490` | `[sp-28]` | `?` | `?` | `ret(0xf08dc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf088c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xf08b4 | `ret(0xf08dc) != 0x0` | `ret(0xf08dc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf08d8 | `r0` | `?` |

## `tdrdef_init`

- `tdrdef_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf08dc to 0xf09f0.
  Established. Included four-byte pointee assignments retain `ret(0x1b1b4)` at `r0 +0x4` and
  `[sp-24] +0x8` (`str` at 0xf0920, `str` at 0xf0998). A halfword assignment through `[sp-24] +0x2`
  writes `([sp-32]:2 & 0xffff)` (`strh` at 0xf0958). The later unnamed `0x1b4cc` call has
  `[[sp-24]+8]`, `0x0`, `[sp-36]`, `[sp-24]` (`bl` at 0xf09d0). Definition fields and allocation
  sizes remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf08ec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf08f0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xf08f4 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xf08f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf090c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf0910 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0914 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0920 | store | 4 | `r0 +0x4` | `ret(0x1b1b4)` |
| `ldr` at 0xf0924 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xf0934 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf093c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0940 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xf0948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0950 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf0954 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xf0958 | store | 2 | `[sp-24] +0x2` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xf095c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0974 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0984 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf0988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf098c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0998 | store | 4 | `[sp-24] +0x8` | `ret(0x1b1b4)` |
| `ldr` at 0xf099c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf09a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf09ac | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xf09b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf09c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf09c4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf09cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf09d4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf09d8 | load | 2 | `sp -0x1c` | `-` |
| `strh` at 0xf09dc | store | 2 | `[sp-24] +0x0` | `([sp-28]:2 & 0xffff)` |
| `str` at 0xf09e4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xf09e8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xf09ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf09ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf09ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf09ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0918 | `(unknown callee) 0x1b1b4` | `[sp-36]` | `r1` | `r2` | `?` |
| `bl` at 0xf094c | `(unknown callee) 0x1b4cc` | `[[sp-24]+4]` | `0x0` | `[sp-36]` | `[sp-24]` |
| `bl` at 0xf0990 | `(unknown callee) 0x1b1b4` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xf09b0 | `(unknown callee) 0x1b490` | `[[sp-24]+4]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf09d0 | `(unknown callee) 0x1b4cc` | `[[sp-24]+8]` | `0x0` | `[sp-36]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf092c | `[r0+4] != 0x0` | `[r0+4] == 0x0` |
| `bne` at 0xf09a4 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf09ec | `r0` | `?` |

## `tdrdef_dump`

- `tdrdef_dump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf09f0 to 0xf0b9c.
  Established. Two halfword reads target `[sp-28] +0x0` and `[sp-28] +0x2` (`ldrh` at 0xf0a64,
  `ldrh` at 0xf0a7c). The subsequent unnamed `0x240b8` calls carry `[sp-20]`, `0x137bd4`,
  `[[sp-28]]:2`, `[[sp-28]]:2`, and `[sp-20]`, `0x137c20`, `[[sp-28]+2]:2`, `[[sp-28]+2]:2` (`bl` at
  0xf0a74, `bl` at 0xf0a8c). A later unnamed `0xf81f4` call has `[sp-20]`, `0x137c3c`, `[sp-76]`,
  `[[sp-28]+8]` (`bl` at 0xf0b78). Definition formatting and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0a00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0a04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0a08 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf0a0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0a1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0a28 | load | 4 | `abs 0xf0b80` | `-` |
| `ldr` at 0xf0a2c | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xf0a30 | store | 4 | `sp -0x14` | `[0x145ae4]` |
| `ldr` at 0xf0a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0a44 | load | 4 | `abs 0xf0b84` | `-` |
| `ldr` at 0xf0a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0a54 | load | 4 | `abs 0xf0b88` | `-` |
| `ldr` at 0xf0a58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0a60 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0a64 | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf0a68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0a6c | load | 4 | `abs 0xf0b8c` | `-` |
| `ldr` at 0xf0a78 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0a7c | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xf0a80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0a84 | load | 4 | `abs 0xf0b90` | `-` |
| `str` at 0xf0a94 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf0a98 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0a9c | load | 2 | `[sp-28] +0x2` | `-` |
| `ldr` at 0xf0aa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0ab8 | load | 4 | `abs 0xf0b94` | `-` |
| `ldr` at 0xf0abc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0ac8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0acc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0ae0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xf0ae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0af8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0b00 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf0b08 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0b0c | load | 2 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf0b24 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0b28 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xf0b38 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf0b40 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf0b44 | load | 2 | `[sp-28] +0x0` | `-` |
| `str` at 0xf0b54 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf0b58 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xf0b60 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xf0b64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0b68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0b6c | load | 4 | `abs 0xf0b98` | `-` |
| `ldr` at 0xf0b70 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf0b74 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0xf0b7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0b7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0b7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0a48 | `(unknown callee) 0x240b8` | `?` | `0x137bf8` | `r2` | `r1` |
| `bl` at 0xf0a5c | `(unknown callee) 0x240b8` | `?` | `0x137bb0` | `r1` | `r1` |
| `bl` at 0xf0a74 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137bd4` | `[[sp-28]]:2` | `[[sp-28]]:2` |
| `bl` at 0xf0a8c | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137c20` | `[[sp-28]+2]:2` | `[[sp-28]+2]:2` |
| `bl` at 0xf0ac0 | `(unknown callee) 0x1b52c` | `sp-72` | `0x137c2c` | `?` | `sp-72` |
| `bl` at 0xf0af4 | `(unknown callee) 0xf0748` | `[sp-20]` | `sp-72` | `?` | `?` |
| `bl` at 0xf0b78 | `(unknown callee) 0xf81f4` | `[sp-20]` | `0x137c3c` | `[sp-76]` | `[[sp-28]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf0a14 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xf0a24 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf0a3c | `r1 != 0x0` | `r1 == 0x0` |
| `blo` at 0xf0aa8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf0b20 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0b7c | `r0` | `?` |

## `tdrdef_set_field`

- `tdrdef_set_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0b9c to 0xf0de4.
  Established. Included four-byte stores write `0x1` at `sp -0x30` and `0x0` at `sp -0x24` (`str` at
  0xf0c34, `str` at 0xf0cac). The branch takes `[sp-48] != 0x80000000` to `0xf0d94` (`bne` at 0xf0d78); its
  fall-through path includes four-byte `[sp-40]-1` at `sp -0x28` (`str` at 0xf0d84). Two four-byte stores have unknown effective targets and values (`str` at 0xf0d34, `str`
  at 0xf0d6c). Field destination and bit mapping remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0bac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0bb0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0bb4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf0bb8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf0bbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0bc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0bd4 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0bdc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0bfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0c10 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0c18 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf0c2c | store | 4 | `sp -0x28` | `?` |
| `str` at 0xf0c34 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xf0c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0c3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0c50 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf0c58 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0c5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0c68 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf0c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0c70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0c84 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf0c8c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0c90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0ca4 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xf0cac | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf0cb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0cc8 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xf0cd0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0cd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0ce4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0cec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0cf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0cf8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0d08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0d10 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0d18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0d20 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0d28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0d2c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0d34 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf0d3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0d44 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0d4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0d54 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0d5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0d64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf0d6c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf0d70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0d7c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf0d84 | store | 4 | `sp -0x28` | `[sp-40]-1` |
| `str` at 0xf0d8c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xf0d94 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf0d9c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf0da0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0dac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf0db4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xf0dbc | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf0dc4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0dcc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xf0dd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0dd8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xf0de0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf0de0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf0de0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf0cdc | `C == 0` | `not (C == 0)` |
| `beq` at 0xf0d04 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xf0d78 | `[sp-48] != 0x80000000` | `[sp-48] == 0x80000000` |
| `bne` at 0xf0da8 | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf0de0 | `r0` | `?` |

## `tdrdef_get_field`

- `tdrdef_get_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf0de4 to 0xf10c8.
  Established. A four-byte store with unknown effective target writes `0x0` (`str` at 0xf0f78). The
  later branch takes `[sp-56] != 0x80000000` to `0xf1078` (`bne` at 0xf105c); its fall-through path includes four-byte
  `[sp-48]-1` at `sp -0x30` (`str` at 0xf1068). Two other four-byte stores have
  unknown effective targets and values (`str` at 0xf1018, `str` at 0xf1050). Extracted field
  representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf0df4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf0df8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf0dfc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf0e00 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf0e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0e08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e1c | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0e24 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e54 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0e5c | load | 2 | `(unknown)` | `-` |
| `str` at 0xf0e6c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf0e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0e78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0e8c | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0e94 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf0ea4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf0ea8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf0eb0 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xf0eb4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf0eb8 | store | 4 | `sp -0x28` | `[sp-60]` |
| `ldr` at 0xf0ebc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf0ec4 | store | 4 | `sp -0x30` | `[sp-60]-1` |
| `str` at 0xf0ecc | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf0ed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0ee8 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0ef0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0ef4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf0f00 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xf0f08 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf0f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0f10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0f24 | load | 4 | `r0 +0x4` | `-` |
| `ldrh` at 0xf0f2c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0f30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0f3c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0f44 | store | 4 | `sp -0x34` | `?` |
| `str` at 0xf0f4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf0f50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0f54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0f64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0f6c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xf0f78 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf0f7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0f84 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xf0f90 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf0f94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf0f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0fac | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf0fb4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf0fb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0fc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0fd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0fd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0fdc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0fec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0ff4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf0ffc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf1004 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf100c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1010 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf1018 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1020 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf1028 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf1030 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf1038 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xf1040 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf1048 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1050 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1054 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf1060 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf1068 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `str` at 0xf1070 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0xf1078 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf1080 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf1084 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf1090 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1098 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xf10a0 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xf10a8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf10b0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xf10b4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf10bc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0xf10c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf10c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf10c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf0e38 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf0f5c | `C == 0` | `not (C == 0)` |
| `blo` at 0xf0fc0 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf0fe8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xf105c | `[sp-56] != 0x80000000` | `[sp-56] == 0x80000000` |
| `bne` at 0xf108c | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf10c4 | `r0` | `?` |

## `tdrdef_find_field`

- `tdrdef_find_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf10c8 to 0xf1164.
  Established. The unnamed `0x1b088` call has four unknown candidates (`bl` at 0xf1128); its nonzero
  result takes the branch to `0xf1144`, while fall-through includes four-byte `[sp-28]` at `sp -0x20` (`bne` at 0xf1134, `str` at 0xf113c). A later four-byte stack assignment at `sp -0x1c`
  retains `[sp-28]+1` (`str` at 0xf114c). Field matching and the return meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf10d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf10dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf10e4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf10e8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xf10ec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf10f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1104 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1118 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1124 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1138 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf113c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xf1144 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf114c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf1154 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1158 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf115c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xf1160 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1160 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1160 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1128 | `(unknown callee) 0x1b088` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf10f8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1134 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1160 | `r0` | `?` |
