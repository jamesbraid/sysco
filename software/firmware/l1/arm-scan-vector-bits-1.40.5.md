# ARM scan vector bit helpers in 1.40.5

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/scan-vector-bits-facts.txt) retain all tool sections.

## `vector_set_bit`

- `vector_set_bit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe259c to 0xe2610.
  Established. The word at `sp -0x20` has an unknown stored value (`str` at 0xe25c8). A word read
  and word assignment have unknown effective targets (`ldr` at 0xe25fc, `str` at 0xe2608). The
  return reports entry `r0` (`ldmdb` at 0xe260c). Single-bit update semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe25ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe25b0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe25b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe25bc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe25c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe25c8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe25d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe25d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe25dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe25e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe25ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe25f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe25fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2600 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2608 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xe260c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe260c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe260c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe260c | `r0` | `r0` |

## `vector_clr_bit`

- `vector_clr_bit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2610 to 0xe2688.
  Established. The word assignment at `sp -0x24` writes an unknown value (`str` at 0xe2650). The
  word read and assignment at unknown targets also have unknown values (`ldr` at 0xe2674, `str` at 0xe2680). The return reports entry `r0` (`ldmdb` at 0xe2684). Bit-clearing effect remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2620 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2624 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe2628 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2630 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe2634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe263c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe2644 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2650 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe265c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2664 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe266c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2674 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2678 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2680 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xe2684 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2684 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2684 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2684 | `r0` | `r0` |

## `vector_get_bit`

- `vector_get_bit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2688 to 0xe2700.
  Established. A word read has an unknown effective target (`ldr` at 0xe26d8). The word at `sp -0x24` receives an unknown value (`str` at 0xe26f0). The return reports `[sp-36]` (`ldmdb` at 0xe26fc). Bit extraction meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2698 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe269c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe26a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe26a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe26ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe26b4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe26bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe26c4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe26c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe26d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe26d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe26dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe26f0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe26f4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe26fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe26fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe26fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe26fc | `r0` | `[sp-36]` |

## `vector_clr_pad`

- `vector_clr_pad`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2700 to 0xe27f0.
  Established. The word at `sp -0x20` receives `0x1` (`str` at 0xe276c). The branch takes `0xe2784`
  when `C == 0`, while `not (C == 0)` falls through (`blo` at 0xe277c). The later word assignment at
  an unknown effective target has an unknown value (`str` at 0xe27e8). Padding clear scope remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2710 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2714 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe2718 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2728 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2734 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe2740 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe2744 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe2748 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe274c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe275c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe2764 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe276c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xe2770 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2774 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2784 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2788 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2790 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2794 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe279c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe27a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe27a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe27b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe27c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe27d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe27d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe27e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe27e8 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xe27ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe27ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe27ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe2724 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe2754 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0xe277c | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe27ec | `r0` | `?` |

## `vector_set_pad`

- `vector_set_pad`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe27f0 to 0xe28dc.
  Established. The word at `sp -0x2c` receives `0x0` on an included path (`str` at 0xe2830). The
  branch takes `0xe28d8` when `Z == 1`, while `not (Z == 1)` falls through (`beq` at 0xe2844). A
  later word assignment at an unknown effective target has an unknown value (`str` at 0xe28d4).
  Padding set policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2800 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2804 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xe2808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2818 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2824 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe2830 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe2834 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe2838 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe283c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe284c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe2854 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xe285c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xe2860 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2864 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2874 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2878 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2880 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2884 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe288c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xe2890 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2898 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe28a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe28b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe28c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe28c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe28cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe28d4 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xe28d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe28d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe28d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe2814 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe2844 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0xe286c | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe28d8 | `r0` | `?` |

## `vector_clr_bits`

- `vector_clr_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe28dc to 0xe29fc.
  Established. Unsigned `[sp-24] >= [sp-44]` takes `0xe2934`, while a smaller value falls through
  (`bhs` at 0xe2928). The word assignment at an unknown target has an unknown value (`str` at 0xe29c4). The word at `sp -0x24` later receives `0x0` (`str` at 0xe29e8). Multi-bit clear effect
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe28ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe28f0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe28f4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe28f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2900 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe2904 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe290c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2910 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2918 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe291c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe292c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe2930 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xe2938 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2940 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xe2948 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xe2950 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe2954 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2958 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2968 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe2970 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe2978 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe297c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe2984 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe298c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe2994 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe2998 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe29a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe29a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe29b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe29b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe29bc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe29c4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe29c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe29cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe29d4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe29d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe29e0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe29e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe29ec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xe29f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe29f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe29f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe2928 | `[sp-24] >= [sp-44] (unsigned, 32-bit)` | `[sp-24] < [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xe2960 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe29f4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe29f8 | `r0` | `r0` |

## `vector_set_bits`

- `vector_set_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe29fc to 0xe2b10.
  Established. The word at `sp -0x34` receives `0x0` (`str` at 0xe2a70). The word read and
  assignment at unknown targets have unknown values (`ldr` at 0xe2acc, `str` at 0xe2ad8). Nonzero
  `[sp-24]` takes `0xe2a30`, while zero falls through (`bne` at 0xe2b08). Bit-set range meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2a0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2a10 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2a14 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe2a18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2a20 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe2a24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2a2c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2a30 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2a38 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe2a3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2a40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2a4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe2a50 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xe2a58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2a60 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xe2a68 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xe2a70 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe2a74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2a78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2a88 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe2a90 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe2a98 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe2a9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe2aa4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe2aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2ab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2abc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2ac4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2acc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2ad0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe2ad8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2adc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2ae0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe2ae8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe2aec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2af4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe2afc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe2b00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xe2b0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2b0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe2a48 | `[sp-24] >= [sp-44] (unsigned, 32-bit)` | `[sp-24] < [sp-44] (unsigned, 32-bit)` |
| `blo` at 0xe2a80 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe2b08 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2b0c | `r0` | `r0` |

## `vector_cpy_bits`

- `vector_cpy_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2b10 to 0xe2cf8.
  Established. The entry-stack word at `sp +0x0` is read (`ldr` at 0xe2b60). Two included word
  assignments have unknown effective targets and values (`str` at 0xe2c2c, `str` at 0xe2c64).
  Nonzero `[sp-64]` takes `0xe2c88`, while zero falls through (`bne` at 0xe2c80). Bit-copy
  boundaries remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2b20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2b24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2b28 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe2b2c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe2b30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2b38 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe2b3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2b44 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe2b48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2b50 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe2b54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2b5c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2b60 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe2b64 | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0xe2b68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2b78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe2b80 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2b84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2b8c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe2b90 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2b94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe2ba0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe2ba4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe2ba8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe2bb4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe2bb8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe2bc8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe2bcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2bdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2be0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2be8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe2bf0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe2bf4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2bfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2c04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2c0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2c14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2c18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2c24 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2c2c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2c30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2c38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2c48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2c50 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xe2c54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2c5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2c64 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2c68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2c6c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe2c74 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe2c78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2c88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2c8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe2c94 | store | 4 | `sp -0x28` | `([sp-40] + [sp-60])` |
| `ldr` at 0xe2c98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2ca8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe2cb0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xe2cb8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe2cbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2cc0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe2cc8 | store | 4 | `sp -0x24` | `([sp-36] + [sp-60])` |
| `ldr` at 0xe2ccc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe2cdc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe2ce4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe2cec | store | 4 | `sp -0x24` | `0x0` |
| `ldmdb` at 0xe2cf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2cf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2b70 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe2c80 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0xe2ca4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe2cd8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2cf4 | `r0` | `?` |

## `vector_flip_bits`

- `vector_flip_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2cf8 to 0xe2ed8.
  Established. The word at `sp -0x24` receives `(r0 + r1)-1` (`str` at 0xe2d40). Included word
  assignments at unknown targets have unknown values (`str` at 0xe2e08, `str` at 0xe2e40, `str` at 0xe2e7c, `str` at 0xe2eb4). The final word assignment at `sp -0x24` writes `[sp-36]-1` (`str` at 0xe2ecc). Bit reversal semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2d08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2d0c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2d10 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xe2d14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2d1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2d24 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe2d28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2d2c | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xe2d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2d34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe2d40 | store | 4 | `sp -0x24` | `(r0 + r1)-1` |
| `ldr` at 0xe2d44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe2d48 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe2d58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2d60 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe2d64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2d74 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe2d78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2d80 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe2d84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2d94 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe2d98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2da0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2da8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2dac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe2db4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2db8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2dc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2dcc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe2ddc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2de4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2df4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2dfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2e00 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe2e08 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2e10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2e18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2e20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2e28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2e30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2e38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2e40 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2e44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2e50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2e58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2e60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2e70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2e74 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe2e7c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2e84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2e8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2e94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2e9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe2ea4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe2eac | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2eb4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2eb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2ec0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xe2ec4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2ecc | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldmdb` at 0xe2ed4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe2ed4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe2ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe2d50 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe2dd8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe2e4c | `[sp-56] == 0x0` | `[sp-56] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe2ed4 | `r0` | `?` |

## `vector_bitcpy`

- `vector_bitcpy`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe2ed8 to 0xe30a4.
  Established. The word through `r0 +0x0` receives `0x0` (`str` at 0xe2f1c). A later word assignment
  writes `0x0` to an unknown effective target (`str` at 0xe305c). The word at `sp -0x28` receives
  `0x0` (`str` at 0xe3064). Copy initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe2ee8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe2eec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe2ef0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe2ef4 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe2efc | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xe2f04 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe2f08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2f10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe2f1c | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xe2f20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2f28 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe2f2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe2f34 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe2f38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe2f3c | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0xe2f40 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe2f50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe2f58 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe2f5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2f64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe2f68 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe2f6c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe2f78 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe2f7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe2f80 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe2f8c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe2f90 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe2fa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe2fa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe2fb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe2fb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe2fc0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe2fc4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2fd4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe2fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe2fe4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xe2fe8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe2ff0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe2ff8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe2ffc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe3000 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe3008 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe300c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe301c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3020 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe3028 | store | 4 | `sp -0x28` | `([sp-40] + [sp-60])` |
| `ldr` at 0xe302c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe303c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3044 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xe3048 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3050 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe305c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xe3064 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe3068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe306c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe3074 | store | 4 | `sp -0x24` | `([sp-36] + [sp-60])` |
| `ldr` at 0xe3078 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3088 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe3090 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe3098 | store | 4 | `sp -0x24` | `0x0` |
| `ldmdb` at 0xe30a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe30a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe30a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe2f48 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe3014 | `[sp-64] != 0x0` | `[sp-64] == 0x0` |
| `bne` at 0xe3038 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe3084 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe30a0 | `r0` | `?` |

## `vector_shift_left`

- `vector_shift_left`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe30a4 to 0xe32b4.
  Established. Unsigned entry `r1 < r2` takes `0xe30e8`, while at least `r2` falls through (`blo` at 0xe30e0). Included word assignments at unknown targets have unknown values (`str` at 0xe31d0,
  `str` at 0xe3208, `str` at 0xe323c). The word at `sp -0x24` receives `0x80000000` on an included
  path (`str` at 0xe3250). Left-shift boundaries remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe30b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe30b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe30bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe30c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe30c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe30d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe30d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe30e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe30ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe30fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3100 | store | 4 | `sp -0x34` | `r1` |
| `ldr` at 0xe3104 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe310c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe3110 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3120 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3124 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3128 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe3130 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe3134 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe313c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe3140 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3150 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe3154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3158 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3160 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xe3168 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe316c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3170 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe3180 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3188 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3190 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3194 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe31a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe31ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe31b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe31bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe31c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe31c8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe31d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe31d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe31e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe31e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe31f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe31f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3200 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe3208 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe320c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3214 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe321c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3224 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe322c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3234 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe323c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe3240 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3250 | store | 4 | `sp -0x24` | `0x80000000` |
| `ldr` at 0xe3254 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe325c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe3264 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe326c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3270 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3280 | store | 4 | `sp -0x28` | `0x80000000` |
| `ldr` at 0xe3284 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe328c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xe3294 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe329c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe32a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe32a8 | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldmdb` at 0xe32b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe32b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe32b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe30cc | `r0 != 0x0` | `r0 == 0x0` |
| `blo` at 0xe30e0 | `r1 < r2 (unsigned, 32-bit)` | `r1 >= r2 (unsigned, 32-bit)` |
| `bls` at 0xe30f4 | `r0 <= r1 (unsigned, 32-bit)` | `r0 > r1 (unsigned, 32-bit)` |
| `blo` at 0xe3178 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe31a0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe3248 | `[sp-36] != 0x1` | `[sp-36] == 0x1` |
| `bne` at 0xe3278 | `[sp-40] != 0x1` | `[sp-40] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe32b0 | `r0` | `?` |

## `vector_shift_right`

- `vector_shift_right`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe32b4 to 0xe34b8.
  Established. The word at `sp -0x38` receives `(r0 + r2)-1` (`str` at 0xe3334). The word at `sp -0x24` receives `0x1` on an included path (`str` at 0xe3454). Three included word assignments have
  unknown effective targets and values (`str` at 0xe33d4, `str` at 0xe340c, `str` at 0xe3440).
  Right-shift effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe32c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe32c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe32cc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe32d0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe32d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe32e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe32e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe32f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3300 | store | 4 | `sp -0x34` | `r2-1` |
| `ldr` at 0xe3304 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe330c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe3310 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3320 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3324 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3328 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3334 | store | 4 | `sp -0x38` | `(r0 + r2)-1` |
| `ldr` at 0xe3338 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3340 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe3344 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3354 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe3358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe335c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe3364 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xe336c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xe3370 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe3374 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe3384 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe338c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3394 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3398 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe33a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe33b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe33b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe33c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe33c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe33cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe33d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe33dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe33e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe33ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe33f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe33fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3404 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe340c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe3410 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3418 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3420 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3428 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3438 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe3440 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe3444 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3454 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xe3458 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe3460 | store | 4 | `sp -0x2c` | `[sp-44]-1` |
| `ldr` at 0xe3468 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3470 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3474 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3484 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe3488 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3490 | store | 4 | `sp -0x30` | `[sp-48]-1` |
| `ldr` at 0xe3498 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe34a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe34a4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe34ac | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldmdb` at 0xe34b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe34b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe34b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe32dc | `r0 != 0x0` | `r0 == 0x0` |
| `blo` at 0xe32f0 | `r1 < r2 (unsigned, 32-bit)` | `r1 >= r2 (unsigned, 32-bit)` |
| `blo` at 0xe337c | `C == 0` | `not (C == 0)` |
| `beq` at 0xe33a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe344c | `[sp-36] != 0x80000000` | `[sp-36] == 0x80000000` |
| `bne` at 0xe347c | `[sp-40] != 0x80000000` | `[sp-40] == 0x80000000` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe34b4 | `r0` | `?` |

## `vector_append_data`

- `vector_append_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe34b8 to 0xe3780.
  Established. Entry `r0 != 0x0` takes `0xe3634`, while zero falls through (`bne` at 0xe3508). The
  word at `sp -0x28` receives `0x80000000` on an included path (`str` at 0xe3518). The final word
  assignment has an unknown effective target and value (`str` at 0xe3778). Data append ordering
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe34c8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe34cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe34d0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe34d4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe34d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34dc | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe34e4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe34e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe34ec | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe34fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe3500 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe3510 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe3518 | store | 4 | `sp -0x28` | `0x80000000` |
| `str` at 0xe3520 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe3524 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3528 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3538 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3540 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3548 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe354c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe355c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3564 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe356c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3574 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe357c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3580 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3588 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe3590 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3598 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe35a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe35a8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe35b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe35b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe35c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe35c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe35d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe35d8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe35e0 | store | 4 | `sp -0x28` | `0x80000000` |
| `ldr` at 0xe35e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe35f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe35f4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe3600 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3608 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xe3610 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xe3618 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3620 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe3624 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe362c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe3634 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3640 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xe3648 | store | 4 | `sp -0x28` | `0x1` |
| `str` at 0xe3650 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe3654 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe3668 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3670 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3678 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe367c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe368c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3694 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe369c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe36a4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe36ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe36b0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe36b8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe36c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe36c8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe36d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe36d8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe36e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe36e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe36f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe36f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe3700 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3708 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `str` at 0xe3710 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xe3718 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3720 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe3724 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe3730 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3738 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xe3740 | store | 4 | `sp -0x34` | `0x80000000` |
| `ldr` at 0xe3748 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3750 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe3754 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe375c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe3764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe376c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3770 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3778 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xe377c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe377c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe377c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe3508 | `r0 != 0x0` | `r0 == 0x0` |
| `blo` at 0xe3530 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe3558 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe35cc | `[sp-40] != 0x1` | `[sp-40] == 0x1` |
| `bne` at 0xe35fc | `[sp-52] != 0x1` | `[sp-52] == 0x1` |
| `blo` at 0xe3660 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe3688 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe36fc | `[sp-40] != 0x80000000` | `[sp-40] == 0x80000000` |
| `bne` at 0xe372c | `[sp-52] != 0x1` | `[sp-52] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe377c | `r0` | `?` |

## `vector_extract_data`

- `vector_extract_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe3870 to 0xe398c.
  Established. The word through `[sp] +0x0` receives `0x0` (`str` at 0xe38c8). The word at `sp -0x2c` receives `0x80000000` on an included path (`str` at 0xe394c). The final word assignment
  through `[sp-24] +0x0` writes `([[sp-24]] + [sp-28])` (`str` at 0xe3984). Extracted bit order
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe3880 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe3884 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe3888 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe388c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe3890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3894 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe389c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe38a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe38a4 | load | 4 | `r1 +0x0` | `-` |
| `str` at 0xe38b4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xe38bc | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe38c0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe38c8 | store | 4 | `[sp] +0x0` | `0x0` |
| `str` at 0xe38d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe38d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe38d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe38e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe38f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe38f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe38fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe390c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3910 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe3914 | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xe3918 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3920 | store | 4 | `[sp] +0x0` | `?` |
| `ldr` at 0xe3924 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe392c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe3930 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe393c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe3944 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0xe394c | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0xe3954 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe395c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe3960 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3968 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe3970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3978 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xe397c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe3984 | store | 4 | `[sp-24] +0x0` | `([[sp-24]] + [sp-28])` |
| `ldmdb` at 0xe3988 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3988 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3988 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe38e0 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe3908 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe3938 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3988 | `r0` | `r0` |

## `vector_mask_bits`

- `vector_mask_bits`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe398c to 0xe3ac8.
  Established. Unsigned `[sp-32] >= [sp-48]` takes `0xe39e8`, while a smaller value falls through
  (`bhs` at 0xe39dc). The word read and assignment at unknown targets have unknown values (`ldr` at 0xe3a84, `str` at 0xe3a90). The word at `sp -0x28` later receives `0x0` (`str` at 0xe3ab4). Mask
  application scope remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe399c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe39a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe39a4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe39a8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe39ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe39b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe39b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe39c0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe39c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe39cc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe39d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe39d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe39e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe39e4 | store | 4 | `sp -0x30` | `[sp-32]` |
| `str` at 0xe39ec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe39f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe39fc | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe3a04 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe3a08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe3a0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe3a1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3a24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe3a2c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe3a34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3a3c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3a44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe3a48 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe3a50 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe3a54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe3a5c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe3a64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3a6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3a74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe3a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe3a84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe3a88 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe3a90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe3a94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe3a98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe3aa0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe3aa4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe3aac | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe3ab4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe3ab8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xe3ac4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3ac4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3ac4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe39dc | `[sp-32] >= [sp-48] (unsigned, 32-bit)` | `[sp-32] < [sp-48] (unsigned, 32-bit)` |
| `blo` at 0xe3a14 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe3ac0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3ac4 | `r0` | `r0` |
