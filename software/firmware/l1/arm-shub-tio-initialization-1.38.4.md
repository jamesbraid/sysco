# ARM SHUB and TIO initialization in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/shub-tio-initialization-facts.txt) retain all tool sections.

## `SHUB2_Cfg_Init`

- `SHUB2_Cfg_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a3a4 to 0x3a698.
  Established. Word stores write `0x1` at absolute `0x147fc0` and `ret(0x1b3c4)` at `0x147fbc`
  (`str` at 0x3a3d8, `str` at 0x3a400). The initial comparison takes its bypass for `[0x147fbc] !=
  0x0` and falls through on zero (`bne` at 0x3a3cc). The call to `0x1b3c4` has `[sp-44]`, `0x1`,
  `0x147fc0`, unknown (`bl` at 0x3a3f8). The common exit reports r0 `0x0` (`ldmdb` at 0x3a674).
  Configuration fields and hardware effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a3b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3a3b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3a3bc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3a3c0 | load | 4 | `abs 0x3a678` | `-` |
| `ldr` at 0x3a3c4 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3a3d0 | load | 4 | `abs 0x3a67c` | `-` |
| `str` at 0x3a3d8 | store | 4 | `abs 0x147fc0` | `0x1` |
| `ldr` at 0x3a3dc | load | 4 | `abs 0x3a67c` | `-` |
| `ldr` at 0x3a3e0 | load | 4 | `abs 0x147fc0` | `-` |
| `str` at 0x3a3e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3a3ec | load | 4 | `abs 0x3a678` | `-` |
| `ldr` at 0x3a3f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3a400 | store | 4 | `abs 0x147fbc` | `ret(0x1b3c4)` |
| `ldr` at 0x3a404 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3a410 | load | 4 | `abs 0x3a680` | `-` |
| `str` at 0x3a414 | store | 4 | `sp -0x44` | `0x1079fc` |
| `ldr` at 0x3a418 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3a41c | store | 4 | `sp -0x40` | `[sp-44]` |
| `ldr` at 0x3a424 | load | 4 | `abs 0x3a684` | `-` |
| `ldr` at 0x3a434 | load | 4 | `abs 0x3a678` | `-` |
| `ldr` at 0x3a438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a440 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3a448 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3a44c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a450 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a454 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x3a458 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a45c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3a460 | store | 4 | `[sp-52] +0x8` | `[sp-32]` |
| `ldr` at 0x3a464 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a468 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a46c | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3a47c | store | 4 | `[sp-52] +0x4` | `ret(0x1b3c4)` |
| `ldr` at 0x3a480 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x3a48c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a494 | load | 4 | `abs 0x3a688` | `-` |
| `ldr` at 0x3a498 | load | 4 | `abs 0x3a68c` | `-` |
| `str` at 0x3a4c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3a4c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a4d0 | load | 4 | `abs 0x3a688` | `-` |
| `ldr` at 0x3a4d4 | load | 4 | `abs 0x3a68c` | `-` |
| `ldr` at 0x3a4e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3a4f0 | load | 4 | `abs 0x3a690` | `-` |
| `str` at 0x3a4f4 | store | 4 | `(unknown)` | `0x107a58` |
| `str` at 0x3a4f8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3a4fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a500 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3a504 | store | 4 | `(unknown)` | `[[sp-52]+8]` |
| `ldr` at 0x3a50c | load | 4 | `abs 0x3a684` | `-` |
| `str` at 0x3a520 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3a524 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a528 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a52c | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x3a53c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a540 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a554 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3a55c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x3a560 | store | 1 | `(unknown)` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x3a564 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a568 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a57c | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3a584 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x3a588 | store | 1 | `(unknown)` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x3a58c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a5a4 | load | 4 | `[sp-52] +0x4` | `-` |
| `strb` at 0x3a5b0 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3a5b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a5c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a5d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a5e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a5ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3a5f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3a5fc | store | 4 | `sp -0x30` | `ret(0xe92b4)` |
| `ldr` at 0x3a600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3a60c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3a618 | load | 4 | `abs 0x3a694` | `-` |
| `str` at 0x3a61c | store | 4 | `(unknown)` | `0x107a94` |
| `ldr` at 0x3a620 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a63c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3a640 | store | 4 | `(unknown)` | `?` |
| `str` at 0x3a644 | store | 4 | `(unknown)` | `ret(0x21674)` |
| `ldr` at 0x3a64c | load | 4 | `abs 0x3a684` | `-` |
| `ldr` at 0x3a65c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3a664 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x3a674 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3a674 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3a674 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3a674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a3f8 | `(unknown callee) 0x1b3c4` | `[sp-44]` | `0x1` | `0x147fc0` | `?` |
| `bl` at 0x3a430 | `(unknown callee) 0x21e28` | `0x9` | `0x107a34` | `0x6e` | `0x0` |
| `bl` at 0x3a474 | `(unknown callee) 0x1b3c4` | `[[sp-52]+8]` | `0xc` | `[sp-52]` | `[sp-52]` |
| `bl` at 0x3a49c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x107a4c` | `0x107a54` |
| `bl` at 0x3a4a8 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3a4d8 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x107a4c` | `0x107a54` |
| `bl` at 0x3a4e8 | `(unknown callee) 0x1b508` | `[sp-56]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3a518 | `(unknown callee) 0x21e28` | `0x9` | `0x107a34` | `0x7a` | `0x0` |
| `bl` at 0x3a5f4 | `(unknown callee) 0xe92b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x3a610 | `(unknown callee) 0x21674` | `ret(0xe92b4)` | `?` | `?` | `ret(0xe92b4)` |
| `bl` at 0x3a658 | `(unknown callee) 0x21e28` | `0x9` | `0x107a34` | `0x84` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3a3cc | `[0x147fbc] != 0x0` | `[0x147fbc] == 0x0` |
| `bne` at 0x3a40c | `[0x147fbc] != 0x0` | `[0x147fbc] == 0x0` |
| `bne` at 0x3a488 | `[[sp-52]+4] != 0x0` | `[[sp-52]+4] == 0x0` |
| `blt` at 0x3a534 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3a608 | `ret(0xe92b4) == 0x0` | `ret(0xe92b4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3a674 | `r0` | `0x0` |

## `SHUB2_Init`

- `SHUB2_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a698 to 0x3aab4.
  Established. Byte stores in separate arms write `0x0` and `0x1` at `[sp-84] +0x4` (`strb` at
  0x3a85c, `strb` at 0x3a86c). The entry global test takes its bypass for `[0x147fc0] == 0x0` and
  falls through for nonzero (`beq` at 0x3a6b8). The call to `0x5d83c` has all four candidates
  unknown (`bl` at 0x3a7a8). Initialization units and r0 at exit remain unresolved (`ldmdb` at
  0x3aa90).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a6a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3a6ac | load | 4 | `abs 0x3aa94` | `-` |
| `ldr` at 0x3a6b0 | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3a6bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a6c4 | load | 4 | `abs 0x3aa98` | `-` |
| `ldr` at 0x3a6c8 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3a6d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3a6dc | load | 4 | `abs 0x3aa9c` | `-` |
| `str` at 0x3a6e0 | store | 4 | `sp -0xd0` | `0xffffeffc` |
| `ldr` at 0x3a6e8 | load | 4 | `abs 0x3aa98` | `-` |
| `ldr` at 0x3a6ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3a6f4 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3a6fc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x3a704 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3a708 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a70c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a710 | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x3a724 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3a728 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a72c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a740 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x3a748 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3a74c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3a750 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a764 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0x3a76c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x3a770 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3a774 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3a788 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3a78c | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x3a790 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3a794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a7a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a7b8 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x3a7bc | load | 1 | `[sp-80] +0x10` | `-` |
| `ldr` at 0x3a7dc | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x3a7e8 | load | 4 | `abs 0x3aaa0` | `-` |
| `str` at 0x3a7ec | store | 4 | `sp -0xe4` | `0x107ab0` |
| `ldr` at 0x3a7f0 | load | 4 | `abs 0x3aaa4` | `-` |
| `ldr` at 0x3a7f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a7f8 | load | 4 | `abs 0x2ae58c` | `-` |
| `str` at 0x3a804 | store | 4 | `sp -0xe0` | `?` |
| `ldr` at 0x3a80c | load | 4 | `abs 0x3aaa8` | `-` |
| `ldr` at 0x3a82c | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x3a830 | load | 1 | `[sp-168] +0x1` | `-` |
| `str` at 0x3a834 | store | 4 | `sp -0xd4` | `[[sp-168]+1]:1` |
| `ldr` at 0x3a838 | load | 4 | `sp -0xd4` | `-` |
| `ldr` at 0x3a844 | load | 4 | `sp -0xd4` | `-` |
| `ldr` at 0x3a854 | load | 4 | `sp -0x54` | `-` |
| `strb` at 0x3a85c | store | 1 | `[sp-84] +0x4` | `0x0` |
| `ldr` at 0x3a864 | load | 4 | `sp -0x54` | `-` |
| `strb` at 0x3a86c | store | 1 | `[sp-84] +0x4` | `0x1` |
| `ldr` at 0x3a874 | load | 4 | `abs 0x3aaac` | `-` |
| `str` at 0x3a878 | store | 4 | `sp -0xe4` | `0x107af8` |
| `ldr` at 0x3a87c | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x3a880 | load | 1 | `[sp-168] +0x2` | `-` |
| `str` at 0x3a884 | store | 4 | `sp -0xe0` | `[[sp-168]+2]:1` |
| `ldr` at 0x3a88c | load | 4 | `abs 0x3aaa8` | `-` |
| `ldr` at 0x3a89c | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3a8a0 | load | 1 | `[sp-84] +0x4` | `-` |
| `ldr` at 0x3a8ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3a8b8 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x3a8bc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3a8cc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x3a8d0 | store | 4 | `sp -0x3c` | `[sp-164]` |
| `ldr` at 0x3a8d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3a8dc | load | 4 | `abs 0x3aab0` | `-` |
| `str` at 0x3a8f4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3a910 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3a914 | load | 1 | `[sp-84] +0x4` | `-` |
| `str` at 0x3a924 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3a928 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3a930 | load | 4 | `[sp-80] +0x4` | `-` |
| `str` at 0x3a954 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x3a964 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x3a968 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3a96c | load | 1 | `[sp-84] +0x4` | `-` |
| `ldr` at 0x3a978 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3a980 | store | 4 | `sp -0xdc` | `?` |
| `str` at 0x3a98c | store | 4 | `sp -0xdc` | `0x8000` |
| `ldr` at 0x3a990 | load | 4 | `sp -0xdc` | `-` |
| `ldr` at 0x3a994 | load | 4 | `sp -0xd8` | `-` |
| `str` at 0x3a998 | store | 4 | `[sp-216] +0x4` | `?` |
| `ldr` at 0x3a99c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3a9a4 | load | 4 | `[sp-80] +0x4` | `-` |
| `ldr` at 0x3a9b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a9c0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x3a9d0 | store | 4 | `sp -0x30` | `ret(0x1f1dc)` |
| `str` at 0x3a9dc | store | 4 | `sp -0x34` | `ret(0x1f1dc)` |
| `str` at 0x3a9e8 | store | 4 | `sp -0x38` | `ret(0x1f1dc)` |
| `ldr` at 0x3a9f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3aa04 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3aa14 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3aa24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3aa30 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3aa40 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3aa54 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3aa58 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3aa60 | load | 4 | `[sp-80] +0x4` | `-` |
| `ldr` at 0x3aa74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3aa7c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x3aa88 | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0x3aa8c | load | 4 | `sp -0xd0` | `-` |
| `ldmdb` at 0x3aa90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3aa90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3aa90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a7a8 | `(unknown callee) 0x5d83c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3a7cc | `(unknown callee) 0xd1ae8` | `0x0` | `[[sp-80]+16]:1` | `sp-204` | `[[sp-80]+16]:1` |
| `bl` at 0x3a818 | `(unknown callee) 0x21e28` | `0x1` | `0x107a34` | `0xae` | `0x0` |
| `bl` at 0x3a824 | `(unknown callee) 0xd1c14` | `sp-204` | `?` | `?` | `sp-204` |
| `bl` at 0x3a898 | `(unknown callee) 0x21e28` | `0x1` | `0x107a34` | `0xbe` | `0x0` |
| `bl` at 0x3a900 | `(unknown callee) 0xd1c14` | `sp-204` | `[sp-60]-800` | `?` | `sp-204` |
| `bl` at 0x3a940 | `(unknown callee) 0xebed0` | `[[sp-80]+4]` | `0x14` | `0x0` | `sp-76` |
| `bl` at 0x3a9b4 | `(unknown callee) 0xebed0` | `[[sp-80]+4]` | `0x15` | `0x1` | `sp-76` |
| `bl` at 0x3a9cc | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x3a9d8 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x3a9e4 | `(unknown callee) 0x1f1dc` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x3aa70 | `(unknown callee) 0xebed0` | `[[sp-80]+4]` | `0x17` | `0x0` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3a6b8 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3a6d8 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x3a718 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3a77c | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x3a79c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3a7b4 | `ret(0x5d83c) == 0x0` | `ret(0x5d83c) != 0x0` |
| `bne` at 0x3a7d8 | `ret(0xd1ae8) != 0x0` | `ret(0xd1ae8) == 0x0` |
| `beq` at 0x3a7e4 | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0x3a840 | `[[sp-168]+1]:1 == 0x1f` | `[[sp-168]+1]:1 != 0x1f` |
| `beq` at 0x3a84c | `[[sp-168]+1]:1 == 0x20` | `[[sp-168]+1]:1 != 0x20` |
| `bne` at 0x3a8a8 | `[[sp-84]+4]:1 != 0x0` | `[[sp-84]+4]:1 == 0x0` |
| `beq` at 0x3a8b4 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `blt` at 0x3a8c4 | `[sp-164] < [sp-60] (signed, 32-bit)` | `[sp-164] >= [sp-60] (signed, 32-bit)` |
| `bne` at 0x3a974 | `[[sp-84]+4]:1 != 0x0` | `[[sp-84]+4]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3aa90 | `r0` | `?` |

## `SHUB2_ITP_Present`

- `SHUB2_ITP_Present`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3aab4 to 0x3ab64.
  Established. A byte store writes `0x1` at an unknown effective target (`strb` at 0x3ab44). The
  initial test takes its bypass for `[0x147fc0] == 0x0` and falls through on nonzero (`beq` at
  0x3aad8). The following comparison takes its continuation for an unknown nonzero operand and falls
  through on zero (`bne` at 0x3aaf8). No calls appear in the facts. ITP presence encoding and exit
  r0 remain unestablished (`ldmdb` at 0x3ab54).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3aac4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3aac8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3aacc | load | 4 | `abs 0x3ab58` | `-` |
| `ldr` at 0x3aad0 | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3aadc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aae4 | load | 4 | `abs 0x3ab5c` | `-` |
| `ldr` at 0x3aae8 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3aaf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3aafc | load | 4 | `abs 0x3ab60` | `-` |
| `str` at 0x3ab00 | store | 4 | `sp -0x34` | `0xffffeffc` |
| `ldr` at 0x3ab08 | load | 4 | `abs 0x3ab5c` | `-` |
| `ldr` at 0x3ab0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ab14 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3ab1c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3ab20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3ab24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ab38 | load | 4 | `[sp-48] +0x4` | `-` |
| `strb` at 0x3ab44 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x3ab4c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3ab50 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3ab54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ab54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ab54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3aad8 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3aaf8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ab54 | `r0` | `?` |

## `SHUB2_SysComEnable`

- `SHUB2_SysComEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3af08 to 0x3b0c0.
  Established. The global test takes its bypass for `[0x147fc0] == 0x0` and falls through on nonzero
  (`beq` at 0x3af38). The next test has unknown operand, takes its continuation for nonzero and
  falls through on zero (`bne` at 0x3af58). The first call to `0x2070c` carries `0x107b30`, unknown,
  `[[sp-32]+8]`, unknown (`bl` at 0x3afb0). System-console interrupt enablement and the joined r0
  value remain unknown (`ldmdb` at 0x3b0a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3af18 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3af20 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x3af28 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3af2c | load | 4 | `abs 0x3b0ac` | `-` |
| `ldr` at 0x3af30 | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3af3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3af44 | load | 4 | `abs 0x3b0b0` | `-` |
| `ldr` at 0x3af48 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3af50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3af5c | load | 4 | `abs 0x3b0b4` | `-` |
| `str` at 0x3af60 | store | 4 | `sp -0x24` | `0xffffeffc` |
| `ldr` at 0x3af68 | load | 4 | `abs 0x3b0b0` | `-` |
| `ldr` at 0x3af6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3af74 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3af7c | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x3af80 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x3af90 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x3af94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3af98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3af9c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x3afac | load | 4 | `abs 0x3b0b8` | `-` |
| `ldr` at 0x3afb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3afb8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3afcc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3afd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3afd8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3afdc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3aff0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3aff8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3b00c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x3b014 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldrb` at 0x3b01c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b020 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b024 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x3b030 | load | 4 | `abs 0x3b0bc` | `-` |
| `str` at 0x3b034 | store | 4 | `sp -0x1c` | `0xffffeffa` |
| `ldr` at 0x3b03c | load | 4 | `abs 0x3b0b8` | `-` |
| `ldr` at 0x3b044 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b048 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b05c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b064 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b068 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b06c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b080 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b088 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b09c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b0a0 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x3b0a4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3b0a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b0a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b0a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3afb0 | `(unknown callee) 0x2070c` | `0x107b30` | `?` | `[[sp-32]+8]` | `?` |
| `bl` at 0x3b008 | `(unknown callee) 0xd6d40` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3b040 | `(unknown callee) 0x2070c` | `0x107b30` | `0x147fbc` | `[[sp-32]+8]` | `(r1 & 0xff)` |
| `bl` at 0x3b098 | `(unknown callee) 0xd6d40` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3af38 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3af58 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3af88 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `blt` at 0x3afa4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x3b02c | `(r1 & 0xff) < [[sp-32]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-32]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b0a8 | `r0` | `?` |

## `SHUB2_SysComInt`

- `SHUB2_SysComInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b0c0 to 0x3b0f4.
  Established. Three byte stores retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at
  entry-stack-relative sites (`strb` at 0x3b0dc, `strb` at 0x3b0e4, `strb` at 0x3b0ec). The return
  register r0 carries entry r3 (`ldmdb` at 0x3b0f0). No calls or conditional branches occur in the
  facts. Interrupt processing beyond these local stores is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b0d0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b0dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x3b0e4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x3b0ec | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldmdb` at 0x3b0f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b0f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b0f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b0f0 | `r0` | `r3` |

## `SHUB_Init`

- `SHUB_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47560 to 0x47a14.
  Established. A byte store writes `([sp-24]:1 & 0xff)` at absolute `0x148054`, followed by a word
  store of `[sp-20]` at `0x148058` (`strb` at 0x47590, `str` at 0x4759c). A separate byte store
  writes `([sp-28]:1 & 0xff)` at `0x14805c` (`strb` at 0x475a8). The call to `0x47a14` uses
  `[sp-32]`, `[sp]`, `0x148070`, `[sp+12]` (`bl` at 0x475d8). Global configuration ownership,
  per-record fields and final r0 remain unidentified (`ldmdb` at 0x479e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47570 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47574 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47578 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x4757c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47580 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0x47584 | store | 1 | `sp -0x21` | `([sp+16] & 0xff)` |
| `ldr` at 0x47588 | load | 4 | `abs 0x479ec` | `-` |
| `ldrb` at 0x4758c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x47590 | store | 1 | `abs 0x148054` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x47594 | load | 4 | `abs 0x479f0` | `-` |
| `ldr` at 0x47598 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4759c | store | 4 | `abs 0x148058` | `[sp-20]` |
| `ldr` at 0x475a0 | load | 4 | `abs 0x479f4` | `-` |
| `ldrb` at 0x475a4 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x475a8 | store | 1 | `abs 0x14805c` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x475ac | load | 4 | `abs 0x479f8` | `-` |
| `ldr` at 0x475b0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x475b4 | store | 4 | `abs 0x148068` | `[sp+4]` |
| `ldr` at 0x475b8 | load | 4 | `abs 0x479fc` | `-` |
| `ldr` at 0x475bc | load | 4 | `sp +0x8` | `-` |
| `str` at 0x475c0 | store | 4 | `abs 0x14806c` | `[sp+8]` |
| `ldr` at 0x475c4 | load | 4 | `abs 0x47a00` | `-` |
| `ldr` at 0x475c8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x475cc | store | 4 | `abs 0x148070` | `[sp+12]` |
| `ldr` at 0x475d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x475d4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x475e0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x475e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x475f4 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x475f8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47614 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47618 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x4761c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47638 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4763c | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47640 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4765c | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x47660 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47664 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x4767c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x47680 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x47684 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47688 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x476ac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x476b0 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x476b4 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x476d8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x476dc | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x476e0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47704 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47708 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x4770c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47730 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47734 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47738 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4775c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47760 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47764 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47788 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x4778c | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47790 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x477b4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x477b8 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x477bc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x477e0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x477e4 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x477e8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4780c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47810 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47814 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47838 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x4783c | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47840 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47864 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47868 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x4786c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47890 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x47894 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x47898 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x478bc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x478c0 | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x478c4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x478e8 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x478ec | load | 4 | `abs 0x47a04` | `-` |
| `ldr` at 0x478f0 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47914 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x47918 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x47920 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x47928 | load | 4 | `abs 0x47a08` | `-` |
| `strh` at 0x47930 | store | 2 | `abs 0x2b07e0` | `0x320` |
| `str` at 0x4793c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x47940 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47954 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4795c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47964 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4796c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47978 | load | 4 | `(unknown)` | `-` |
| `str` at 0x47984 | store | 4 | `sp -0x2c` | `ret(0xe92b4)` |
| `ldr` at 0x47988 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x47994 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x479a0 | load | 4 | `abs 0x47a0c` | `-` |
| `str` at 0x479a4 | store | 4 | `sp -0x38` | `0x109464` |
| `ldr` at 0x479a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x479b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x479b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x479bc | store | 4 | `sp -0x34` | `?` |
| `str` at 0x479c0 | store | 4 | `sp -0x30` | `ret(0x21674)` |
| `ldr` at 0x479c8 | load | 4 | `abs 0x47a10` | `-` |
| `ldr` at 0x479d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x479e0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x479e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x479e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x479e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x475d8 | `(unknown callee) 0x47a14` | `[sp-32]` | `[sp]` | `0x148070` | `[sp+12]` |
| `bl` at 0x47934 | `(unknown callee) 0x4e08c` | `?` | `?` | `0x2b07e0` | `0x320` |
| `bl` at 0x4797c | `(unknown callee) 0xe92b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x47998 | `(unknown callee) 0x21674` | `ret(0xe92b4)` | `?` | `?` | `ret(0xe92b4)` |
| `bl` at 0x479d4 | `(unknown callee) 0x21e28` | `0x9` | `0x109480` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x475ec | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x4794c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x47990 | `ret(0xe92b4) == 0x0` | `ret(0xe92b4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x479e8 | `r0` | `?` |

## `TIOInit`

- `TIOInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x532a0 to 0x534f4.
  Established. The absolute word store at `0x148358` carries `ret(0x1b124)` (`str` at 0x53404).
  Later word and byte stores write `[sp-24]` at `0x148350` and `([sp-28]:1 & 0xff)` at `0x2b08a0`
  (`str` at 0x53454, `strb` at 0x53460). The initial test takes its bypass for `[0x148350] == 0x0`
  and falls through for nonzero (`beq` at 0x532c8). The exit reports `ret(0x1b43c)` (`ldmdb` at
  0x534bc). TIO topology and record ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x532b0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x532b4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x532b8 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x532bc | load | 4 | `abs 0x534c0` | `-` |
| `ldr` at 0x532c0 | load | 4 | `abs 0x148350` | `-` |
| `ldr` at 0x532cc | load | 4 | `abs 0x534c4` | `-` |
| `str` at 0x532d0 | store | 4 | `sp -0x38` | `0x10b2ac` |
| `ldr` at 0x532d4 | load | 4 | `abs 0x534c8` | `-` |
| `ldrb` at 0x532d8 | load | 1 | `abs 0x2b08a0` | `-` |
| `str` at 0x532dc | store | 4 | `sp -0x34` | `[0x2b08a0]:1` |
| `ldr` at 0x532e4 | load | 4 | `abs 0x534cc` | `-` |
| `ldr` at 0x532f4 | load | 4 | `abs 0x534d0` | `-` |
| `str` at 0x532f8 | store | 4 | `sp -0x38` | `0x54018` |
| `ldr` at 0x532fc | load | 4 | `abs 0x534d4` | `-` |
| `str` at 0x53300 | store | 4 | `sp -0x34` | `0x54350` |
| `ldr` at 0x53308 | load | 4 | `abs 0x534d8` | `-` |
| `ldr` at 0x5330c | load | 4 | `abs 0x534dc` | `-` |
| `ldr` at 0x53310 | load | 4 | `abs 0x534e0` | `-` |
| `str` at 0x5331c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x53320 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53324 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53334 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53348 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53350 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5336c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x53370 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5337c | store | 4 | `sp -0x28` | `ret(0xe92b4)` |
| `ldr` at 0x53380 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5338c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x53398 | load | 4 | `abs 0x534e4` | `-` |
| `str` at 0x5339c | store | 4 | `sp -0x38` | `0x10b2dc` |
| `ldr` at 0x533a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x533b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x533bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x533c0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x533c4 | store | 4 | `sp -0x30` | `ret(0x21674)` |
| `ldr` at 0x533cc | load | 4 | `abs 0x534cc` | `-` |
| `ldr` at 0x533dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x533e4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x533ec | load | 4 | `abs 0x534e8` | `-` |
| `ldr` at 0x533f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53404 | store | 4 | `abs 0x148358` | `ret(0x1b124)` |
| `ldr` at 0x53408 | load | 4 | `abs 0x148358` | `-` |
| `ldr` at 0x53414 | load | 4 | `abs 0x534ec` | `-` |
| `str` at 0x53418 | store | 4 | `sp -0x38` | `0x10b2f8` |
| `ldr` at 0x53420 | load | 4 | `abs 0x534cc` | `-` |
| `ldr` at 0x53430 | load | 4 | `abs 0x534e8` | `-` |
| `ldr` at 0x53434 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5343c | load | 4 | `abs 0x148358` | `-` |
| `ldr` at 0x5344c | load | 4 | `abs 0x534c0` | `-` |
| `ldr` at 0x53450 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53454 | store | 4 | `abs 0x148350` | `[sp-24]` |
| `ldr` at 0x53458 | load | 4 | `abs 0x534c8` | `-` |
| `ldrb` at 0x5345c | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x53460 | store | 1 | `abs 0x2b08a0` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x53464 | load | 4 | `abs 0x534f0` | `-` |
| `ldrb` at 0x53468 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x5346c | store | 1 | `abs 0x2b0898` | `([sp-32]:1 & 0xff)` |
| `str` at 0x53474 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x53478 | load | 4 | `abs 0x534c8` | `-` |
| `ldrb` at 0x5347c | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53480 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53490 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x53498 | load | 4 | `abs 0x534e8` | `-` |
| `ldr` at 0x5349c | load | 4 | `abs 0x148358` | `-` |
| `strb` at 0x534a8 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x534ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x534b4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x534bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x534bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x534bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x534bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x532f0 | `(unknown callee) 0x21e28` | `0x9` | `0x10b2c8` | `0xd5` | `0x0` |
| `bl` at 0x53314 | `(unknown callee) 0x5796c` | `0x1` | `0x53bf0` | `0x5602c` | `0x5610c` |
| `bl` at 0x53374 | `(unknown callee) 0xe92b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x53390 | `(unknown callee) 0x21674` | `ret(0xe92b4)` | `?` | `?` | `ret(0xe92b4)` |
| `bl` at 0x533d8 | `(unknown callee) 0x21e28` | `0x9` | `0x10b2c8` | `0xe2` | `0x0` |
| `bl` at 0x533fc | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0x5342c | `(unknown callee) 0x21e28` | `0x9` | `0x10b2c8` | `0xea` | `0x0` |
| `bl` at 0x53448 | `(unknown callee) 0x1b43c` | `[0x148358]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x532c8 | `[0x148350] == 0x0` | `[0x148350] != 0x0` |
| `blt` at 0x5332c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x53388 | `ret(0xe92b4) == 0x0` | `ret(0xe92b4) != 0x0` |
| `bne` at 0x53410 | `[0x148358] != 0x0` | `[0x148358] == 0x0` |
| `blt` at 0x53488 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x534bc | `r0` | `ret(0x1b43c)` |

## `TIOTarget`

- `TIOTarget`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x536c0 to 0x53708.
  Established. The final explicit word read has unknown effective address (`ldr` at 0x536f8). R0 at
  the return is unresolved (`ldmdb` at 0x53700). No calls or deciding branches are reported.
  Physical target selection is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x536d4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x536d8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x536ec | load | 4 | `abs 0x53704` | `-` |
| `ldr` at 0x536f0 | load | 4 | `abs 0x148350` | `-` |
| `ldr` at 0x536f8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x53700 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53700 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53700 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53700 | `r0` | `?` |

## `TIOActive`

- `TIOActive`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53bf0 to 0x53c18.
  Established. The byte load reads absolute `0x148354` (`ldrb` at 0x53c08). The exit r0 expression
  is `[0x148354]:1` (`ldmdb` at 0x53c10). No calls or conditional branches appear in the facts.
  Active-state meaning remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53c00 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x53c04 | load | 4 | `abs 0x53c14` | `-` |
| `ldrb` at 0x53c08 | load | 1 | `abs 0x148354` | `-` |
| `ldmdb` at 0x53c10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53c10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53c10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53c10 | `r0` | `[0x148354]:1` |

## `TIOPresent`

- `TIOPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53fa0 to 0x53fdc.
  Established. A byte store writes `0x1` at an unknown effective target (`strb` at 0x53fd0). The
  exit preserves entry r0 (`ldmdb` at 0x53fd4). Calls and deciding branches are absent. The physical
  presence indication cannot be assigned from this store alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x53fb4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x53fb8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x53fc0 | load | 4 | `abs 0x53fd8` | `-` |
| `ldr` at 0x53fc4 | load | 4 | `abs 0x148358` | `-` |
| `strb` at 0x53fd0 | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x53fd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53fd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53fd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53fd4 | `r0` | `r0` |

## `TIONotPresent`

- `TIONotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53fdc to 0x54018.
  Established. The indexed byte store clears an unknown effective address (`strb` at 0x5400c). Entry
  r0 remains the return expression (`ldmdb` at 0x54010). The facts contain no calls or conditional
  branches. Absence-state propagation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x53ff0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x53ff4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x53ffc | load | 4 | `abs 0x54014` | `-` |
| `ldr` at 0x54000 | load | 4 | `abs 0x148358` | `-` |
| `strb` at 0x5400c | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x54010 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x54010 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x54010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x54010 | `r0` | `r0` |

## `SHUBTarget`

- `SHUBTarget`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48298 to 0x482d4.
  Established. The indexed word read uses an unknown effective address (`ldr` at 0x482c4). The
  recognized exit has unresolved r0 (`ldmdb` at 0x482cc). There are no calls in the facts. SHUB
  target-table contents and physical routing remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x482ac | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x482b0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x482b8 | load | 4 | `abs 0x482d0` | `-` |
| `ldr` at 0x482bc | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x482c4 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x482cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x482cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x482cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x482cc | `r0` | `?` |

## `SHUB_Rev`

- `SHUB_Rev`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c0f0 to 0x4c124.
  Established. A word read has unknown effective target (`ldr` at 0x4c10c); its transformed value
  remains unknown at the exit (`ldmdb` at 0x4c11c). No calls or conditional branches are reported.
  Revision encoding is unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c100 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4c104 | load | 4 | `abs 0x4c120` | `-` |
| `ldr` at 0x4c108 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c10c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x4c11c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c11c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c11c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c11c | `r0` | `?` |

## `SHUB_SetClkstop`

- `SHUB_SetClkstop`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c4e4 to 0x4c554.
  Established. The byte store at an unknown effective address carries `([sp-20]:1 & 0xff)` (`strb`
  at 0x4c538). The loop branch retains only `le flags condition (operands unknown)` and its negation
  for fall-through (`ble` at 0x4c508). No callees appear in the facts. Clock-stop polarity and the
  exit r0 value remain unknown (`ldmdb` at 0x4c54c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c4f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c4fc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c510 | load | 4 | `abs 0x4c550` | `-` |
| `ldr` at 0x4c514 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c534 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4c538 | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x4c53c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c544 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x4c54c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c54c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c54c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4c508 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c54c | `r0` | `?` |

## `SHUB_GetClkstop`

- `SHUB_GetClkstop`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c554 to 0x4c574.
  Established. The byte read addresses absolute `0x2b0775` (`ldrb` at 0x4c564). The exit carries
  `[0x2b0775]:1` in r0 (`ldmdb` at 0x4c56c). No calls or conditional branches occur in this span.
  Clock-state interpretation remains unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4c560 | load | 4 | `abs 0x4c570` | `-` |
| `ldrb` at 0x4c564 | load | 1 | `abs 0x2b0775` | `-` |
| `ldmdb` at 0x4c56c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c56c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c56c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c56c | `r0` | `[0x2b0775]:1` |
