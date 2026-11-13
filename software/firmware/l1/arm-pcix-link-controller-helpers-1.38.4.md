# ARM PCI-X and link controller helpers in 1.38.4

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
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/pcix-link-controller-helpers-facts.txt) retain all tool sections.

## `pcixInit`

- `pcixInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x574f4 to 0x5796c.
  Established. The global word read accesses absolute `0x14835c` (`ldr` at 0x57518). Zero takes the
  branch and nonzero falls through (`beq` at 0x57520). The opening call to `0x21e28` has `0x9`,
  `0x10ba60`, `0xa5` and `0x0` as candidates (`bl` at 0x57560). The PCI-X initialisation contract
  remains unknown beyond the recorded effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57504 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x57508 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5750c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x57510 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x57514 | load | 4 | `abs 0x57944` | `-` |
| `ldr` at 0x57518 | load | 4 | `abs 0x14835c` | `-` |
| `ldr` at 0x57524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57530 | load | 4 | `abs 0x57948` | `-` |
| `str` at 0x57534 | store | 4 | `sp -0x48` | `0x10ba30` |
| `ldr` at 0x57538 | load | 4 | `abs 0x57944` | `-` |
| `ldr` at 0x5753c | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57540 | store | 4 | `sp -0x44` | `[0x14835c]` |
| `ldr` at 0x57544 | load | 4 | `abs 0x5794c` | `-` |
| `ldr` at 0x57548 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x5754c | store | 4 | `sp -0x40` | `[0x148360]` |
| `ldr` at 0x57554 | load | 4 | `abs 0x57950` | `-` |
| `ldr` at 0x57564 | load | 4 | `abs 0x5794c` | `-` |
| `ldr` at 0x57568 | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x57574 | load | 4 | `abs 0x5794c` | `-` |
| `str` at 0x5757c | store | 4 | `abs 0x148360` | `0x1` |
| `ldr` at 0x57580 | load | 4 | `abs 0x5794c` | `-` |
| `ldr` at 0x57584 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x57598 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x5759c | load | 4 | `abs 0x57944` | `-` |
| `ldr` at 0x575a0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x575b0 | store | 4 | `abs 0x14835c` | `ret(0x1b3c4)` |
| `ldr` at 0x575b4 | load | 4 | `abs 0x14835c` | `-` |
| `ldr` at 0x575c0 | load | 4 | `abs 0x57954` | `-` |
| `str` at 0x575c4 | store | 4 | `sp -0x48` | `0x10ba74` |
| `ldr` at 0x575c8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x575cc | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0x575d4 | load | 4 | `abs 0x57950` | `-` |
| `ldr` at 0x575e4 | load | 4 | `abs 0x57944` | `-` |
| `ldr` at 0x575e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x575fc | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57604 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x57608 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5760c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x57610 | store | 4 | `[sp-48] +0x0` | `[sp-28]` |
| `ldr` at 0x57614 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57618 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5761c | store | 4 | `[sp-48] +0x4` | `[sp-32]` |
| `ldr` at 0x57620 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57624 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57628 | store | 4 | `[sp-48] +0x8` | `[sp-36]` |
| `ldr` at 0x5762c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57630 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x57634 | store | 4 | `[sp-48] +0xc` | `[sp]` |
| `ldr` at 0x57638 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5763c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x57640 | store | 4 | `[sp-48] +0x10` | `[sp+4]` |
| `ldr` at 0x57644 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57648 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x5764c | store | 4 | `[sp-48] +0x18` | `[sp+8]` |
| `ldr` at 0x57650 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57654 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57658 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x57668 | store | 4 | `[sp-48] +0x14` | `ret(0x1b3c4)` |
| `ldr` at 0x5766c | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57680 | load | 4 | `abs 0x57958` | `-` |
| `ldr` at 0x57684 | load | 4 | `abs 0x5795c` | `-` |
| `str` at 0x576b0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x576b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x576bc | load | 4 | `abs 0x57958` | `-` |
| `ldr` at 0x576c0 | load | 4 | `abs 0x5795c` | `-` |
| `ldr` at 0x576cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x576dc | load | 4 | `abs 0x57960` | `-` |
| `str` at 0x576e0 | store | 4 | `(unknown)` | `0x10bab8` |
| `str` at 0x576e4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x576e8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x576ec | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x576f0 | store | 4 | `(unknown)` | `[[sp-48]+8]` |
| `ldr` at 0x576f8 | load | 4 | `abs 0x57950` | `-` |
| `ldr` at 0x57708 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5770c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57710 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57718 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x57724 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57728 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5772c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57730 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x57740 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57744 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x57758 | load | 4 | `abs 0x57964` | `-` |
| `str` at 0x57760 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x57764 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57768 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57770 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57778 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5777c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57790 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x57798 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x5779c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x577a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x577a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x577ac | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x577b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x577b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x577cc | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x577d4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x577d8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x577dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x577e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x577e8 | load | 4 | `[sp-48] +0x14` | `-` |
| `strb` at 0x577f4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x577f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x577fc | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x57800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57818 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5781c | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x57828 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5782c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57844 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldr` at 0x5784c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x57850 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57858 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x57864 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x57868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x57880 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x57884 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57898 | load | 4 | `abs 0x57958` | `-` |
| `ldr` at 0x5789c | load | 4 | `abs 0x5795c` | `-` |
| `str` at 0x578c8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x578cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x578d4 | load | 4 | `abs 0x57958` | `-` |
| `ldr` at 0x578d8 | load | 4 | `abs 0x5795c` | `-` |
| `ldr` at 0x578e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x578f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57900 | load | 4 | `abs 0x57968` | `-` |
| `str` at 0x57904 | store | 4 | `(unknown)` | `0x10baf4` |
| `str` at 0x57908 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x5790c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57910 | store | 4 | `(unknown)` | `[sp-44]` |
| `str` at 0x57914 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x5791c | load | 4 | `abs 0x57950` | `-` |
| `ldr` at 0x5792c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57934 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldmdb` at 0x57940 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x57940 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57940 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57940 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57560 | `(unknown callee) 0x21e28` | `0x9` | `0x10ba60` | `0xa5` | `0x0` |
| `bl` at 0x575a8 | `(unknown callee) 0x1b3c4` | `[sp-52]` | `0x1` | `[0x148360]` | `?` |
| `bl` at 0x575e0 | `(unknown callee) 0x21e28` | `0x9` | `0x10ba60` | `0xb1` | `0x0` |
| `bl` at 0x57660 | `(unknown callee) 0x1b3c4` | `[[sp-48]+8]` | `0x8` | `[sp-48]` | `[sp-48]` |
| `bl` at 0x57688 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x57694 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x576c4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x576d4 | `(unknown callee) 0x1b508` | `[sp-52]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x57704 | `(unknown callee) 0x21e28` | `0x9` | `0x10ba60` | `0xc3` | `0x0` |
| `bl` at 0x5771c | `(unknown callee) 0x1b43c` | `[[sp-48]+20]` | `0x1` | `[[sp-48]+8]` | `[sp-48]` |
| `mov` at 0x57808 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-48]` | `?` | `[[sp-52]]` |
| `mov` at 0x57878 | `(unknown callee) (unknown target)` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x578a0 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x578ac | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x578dc | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x578ec | `(unknown callee) 0x1b508` | `[sp-56]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x578f8 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x57928 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0xd0` | `0x0` |
| `bl` at 0x5793c | `(unknown callee) 0xa4b38` | `?` | `?` | `?` | `[[sp-48]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x57520 | `[0x14835c] == 0x0` | `[0x14835c] != 0x0` |
| `bne` at 0x5752c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x57570 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `bne` at 0x575bc | `[0x14835c] != 0x0` | `[0x14835c] == 0x0` |
| `bne` at 0x57674 | `[[sp-48]+20] != 0x0` | `[[sp-48]+20] == 0x0` |
| `blt` at 0x57738 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x57814 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x57824 | `[[sp-52]+8] == 0x0` | `[[sp-52]+8] != 0x0` |
| `beq` at 0x5788c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57940 | `r0` | `ret(0xa4b38)` |

## `pcixGetSlotCfg`

- `pcixGetSlotCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57a68 to 0x57af8.
  Established. A word load reads absolute `0x148360` (`ldr` at 0x57a84). Nonzero takes the branch and zero
  falls through (`bne` at 0x57a8c). The common return expression is unknown (`ldmdb` at 0x57aec).
  Slot configuration geometry is not established by the supported facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57a78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x57a7c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x57a80 | load | 4 | `abs 0x57af0` | `-` |
| `ldr` at 0x57a84 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x57a94 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x57a9c | load | 4 | `abs 0x57af4` | `-` |
| `ldr` at 0x57aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57ab4 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57abc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x57ac0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57acc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57ad4 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x57ad8 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+12]` |
| `ldr` at 0x57adc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57ae0 | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x57ae4 | store | 4 | `sp -0x20` | `[[sp-28]+16]` |
| `ldr` at 0x57ae8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x57aec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57aec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x57a8c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x57ac8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57aec | `r0` | `?` |

## `pcixUp`

- `pcixUp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57b88 to 0x57e90.
  Established. The initial decision takes `[0x148360] != 0x0` and falls through on zero (`bne` at
  0x57bb8). The first call to `0x589c8` has candidates `[sp-20]`, unknown, unknown and `[[sp-28]+8]` (`bl`
  at 0x57c68). The final value is unknown (`ldmdb` at 0x57e80), as is the external slot-up
  operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57b98 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x57ba0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x57ba8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x57bac | load | 4 | `abs 0x57e84` | `-` |
| `ldr` at 0x57bb0 | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x57bbc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57bc8 | load | 4 | `abs 0x57e88` | `-` |
| `str` at 0x57bcc | store | 4 | `sp -0x34` | `0xfffff703` |
| `str` at 0x57bd8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x57bdc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x57be0 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x57be8 | load | 4 | `abs 0x57e8c` | `-` |
| `ldr` at 0x57bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57c00 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57c08 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x57c0c | load | 1 | `sp -0x15` | `-` |
| `str` at 0x57c1c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57c20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57c24 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57c28 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x57c38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57c3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57c44 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57c50 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x57c54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57c5c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x57c64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x57c70 | store | 4 | `sp -0x28` | `ret(0x589c8)` |
| `str` at 0x57c78 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57c7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57c80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57c84 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x57c94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57c98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57ca0 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57cac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57cb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57cb8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x57cc0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57cc4 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x57ccc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x57cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57cdc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57ce4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57cf0 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x57cf4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57cf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x57d08 | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x57d0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57d18 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57d1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57d24 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57d30 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57d34 | load | 4 | `abs 0x57e88` | `-` |
| `str` at 0x57d38 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x57d40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57d44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57d50 | store | 4 | `sp -0x28` | `ret(0x59d18)` |
| `ldr` at 0x57d54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57d58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57d60 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57d6c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57d70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57d74 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x57d7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57d80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57d88 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57d94 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x57d98 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x57d9c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x57da0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x57db0 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x57db4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x57dc0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57dc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57dcc | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57dd8 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x57ddc | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x57de0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x57de4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x57df4 | store | 4 | `sp -0x30` | `ret(0x5b3ec)` |
| `ldr` at 0x57dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57e00 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57e0c | store | 4 | `sp -0x28` | `ret(0x59e34)` |
| `ldr` at 0x57e10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x57e1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57e20 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57e28 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57e34 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57e38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57e3c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x57e44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57e48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57e54 | store | 4 | `sp -0x28` | `ret(0x5aba8)` |
| `ldr` at 0x57e58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57e5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57e64 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57e70 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57e74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57e78 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x57e7c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x57e80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x57e80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x57e80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57c68 | `(unknown callee) 0x589c8` | `[sp-20]` | `?` | `?` | `[[sp-28]+8]` |
| `bl` at 0x57d00 | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x57d48 | `(unknown callee) 0x59d18` | `[sp-20]` | `ret(0x5b130)` | `?` | `ret(0x5b130)` |
| `bl` at 0x57da8 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x57dec | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x57e04 | `(unknown callee) 0x59e34` | `[sp-20]` | `ret(0x5b20c)` | `?` | `ret(0x5b20c)` |
| `bl` at 0x57e4c | `(unknown callee) 0x5aba8` | `[sp-20]` | `[sp-36]` | `?` | `ret(0x59e34)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x57bb8 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x57bc4 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x57c14 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x57c30 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x57c8c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x57cd4 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x57d14 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `bne` at 0x57dbc | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |
| `bge` at 0x57e18 | `ret(0x59e34) >= 0x0 (signed, 32-bit)` | `ret(0x59e34) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x57e80 | `r0` | `?` |

## `pcixDown`

- `pcixDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57e90 to 0x58150.
  Established. The global word load reads absolute `0x148360` (`ldr` at 0x57eb8); its nonzero value takes the
  first branch and zero falls through (`bne` at 0x57ec0). The call to `0x58ac0` has `[sp-20]`,
  unknown, unknown and `[[sp-28]+8]` as candidates (`bl` at 0x57f70). The slot-down meaning remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57ea0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x57ea8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x57eb0 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x57eb4 | load | 4 | `abs 0x58144` | `-` |
| `ldr` at 0x57eb8 | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x57ec4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57ed0 | load | 4 | `abs 0x58148` | `-` |
| `str` at 0x57ed4 | store | 4 | `sp -0x34` | `0xfffff703` |
| `str` at 0x57ee0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x57ee4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x57ee8 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x57ef0 | load | 4 | `abs 0x5814c` | `-` |
| `ldr` at 0x57ef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57f08 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x57f10 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x57f14 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x57f24 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57f28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57f2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f30 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x57f40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57f44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f4c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57f58 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x57f5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57f64 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x57f6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x57f78 | store | 4 | `sp -0x28` | `ret(0x58ac0)` |
| `str` at 0x57f80 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x57f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57f88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57f8c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x57f9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57fa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x57fa8 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57fb4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x57fb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x57fc0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x57fc8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x57fcc | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x57fd4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x57fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x57fe4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x57fec | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x57ff8 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x57ffc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58000 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58010 | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x58014 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58020 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58024 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5802c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58038 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x5803c | load | 4 | `abs 0x58148` | `-` |
| `str` at 0x58040 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x58048 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5804c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x58058 | store | 4 | `sp -0x28` | `ret(0x5a3e8)` |
| `ldr` at 0x5805c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58060 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58068 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58074 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58078 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5807c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x58084 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x58088 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58090 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5809c | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x580a0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x580a4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x580a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x580b8 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x580bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x580c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x580cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x580d4 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x580e0 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x580e4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x580e8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x580ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x580fc | store | 4 | `sp -0x30` | `ret(0x5b3ec)` |
| `ldr` at 0x58104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58108 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58114 | store | 4 | `sp -0x28` | `ret(0x5a4c4)` |
| `ldr` at 0x58118 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5811c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58124 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x58130 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x58134 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58138 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x5813c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x58140 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58140 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58140 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x57f70 | `(unknown callee) 0x58ac0` | `[sp-20]` | `?` | `?` | `[[sp-28]+8]` |
| `bl` at 0x58008 | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58050 | `(unknown callee) 0x5a3e8` | `[sp-20]` | `ret(0x5b130)` | `?` | `ret(0x5b130)` |
| `bl` at 0x580b0 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x580f4 | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x5810c | `(unknown callee) 0x5a4c4` | `[sp-20]` | `ret(0x5b20c)` | `?` | `ret(0x5b20c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x57ec0 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x57ecc | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x57f1c | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x57f38 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x57f94 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x57fdc | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x5801c | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `bne` at 0x580c4 | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58140 | `r0` | `?` |

## `pcixStatus`

- `pcixStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x586a4 to 0x58814.
  Established. The branch takes `[0x148360] != 0x0` and falls through on zero (`bne` at 0x586d8).
  The opening call to `0x5b20c` has candidates `[sp-20]`, `[sp-21]:1`, `[sp-22]:1` and `[sp-21]:1` (`bl` at
  0x5871c). The status encoding and merged r0 result remain unknown (`ldmdb` at 0x58800).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x586b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x586b8 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x586c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x586c8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x586cc | load | 4 | `abs 0x58804` | `-` |
| `ldr` at 0x586d0 | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x586dc | load | 4 | `abs 0x58808` | `-` |
| `str` at 0x586e0 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x586e8 | load | 4 | `abs 0x5880c` | `-` |
| `ldr` at 0x586ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58700 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x58708 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x5870c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58710 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58714 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58724 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x58728 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58734 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58738 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5873c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5874c | store | 4 | `sp -0x30` | `ret(0x5b3ec)` |
| `ldr` at 0x58754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58758 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5875c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58768 | store | 4 | `sp -0x2c` | `ret(0x5b754)` |
| `ldr` at 0x5876c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58778 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5877c | store | 4 | `sp -0x30` | `ret(0x5b754)` |
| `ldrb` at 0x58784 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58788 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58798 | store | 4 | `sp -0x28` | `ret(0x5b130)` |
| `ldr` at 0x5879c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x587a8 | load | 4 | `abs 0x58808` | `-` |
| `str` at 0x587ac | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x587b4 | load | 4 | `abs 0x58810` | `-` |
| `ldr` at 0x587b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x587bc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x587dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x587e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x587e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x587e8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x587f8 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0x587fc | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x58800 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58800 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58800 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5871c | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58744 | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58760 | `(unknown callee) 0x5b754` | `[sp-20]` | `ret(0x5b20c)` | `[sp-28]` | `ret(0x5b20c)` |
| `bl` at 0x58790 | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `mov` at 0x587f0 | `(unknown callee) (unknown target)` | `[sp-20]` | `ret(0x5b130)` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x586d8 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `bne` at 0x58730 | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |
| `beq` at 0x58774 | `ret(0x5b754) == 0x0` | `ret(0x5b754) != 0x0` |
| `bne` at 0x587a4 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58800 | `r0` | `?` |

## `pcixInt`

- `pcixInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59094 to 0x59380.
  Established. The initial branch takes `[0x148360] != 0x0` and falls through on zero (`bne` at
  0x590d0). The first call to `0x5b4d0` has candidates `[sp-20]`, `[sp-21]:1`, unknown and `[sp-21]:1` (`bl`
  at 0x59108). The exit value and interrupt-routing effect remain unknown (`ldmdb` at 0x59344).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x590a4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x590b0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x590b8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x590c0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x590c4 | load | 4 | `abs 0x59348` | `-` |
| `ldr` at 0x590c8 | load | 4 | `abs 0x148360` | `-` |
| `ldr` at 0x590d8 | load | 4 | `abs 0x5934c` | `-` |
| `ldr` at 0x590dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x590f0 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x590f8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x590fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59100 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59110 | store | 4 | `sp -0x20` | `ret(0x5b4d0)` |
| `ldr` at 0x59114 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59128 | load | 4 | `abs 0x59350` | `-` |
| `ldr` at 0x5912c | load | 4 | `abs 0x59354` | `-` |
| `str` at 0x59158 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5915c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59164 | load | 4 | `abs 0x59350` | `-` |
| `ldr` at 0x59168 | load | 4 | `abs 0x59354` | `-` |
| `ldr` at 0x59174 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59184 | load | 4 | `abs 0x59358` | `-` |
| `str` at 0x59188 | store | 4 | `(unknown)` | `0x10bb1c` |
| `str` at 0x5918c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x59190 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x59194 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x5919c | load | 4 | `abs 0x5935c` | `-` |
| `ldr` at 0x591a0 | load | 4 | `abs 0x59360` | `-` |
| `ldrb` at 0x591ac | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x591b0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x591b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x591c4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x591c8 | load | 1 | `ret(0x5b4d0) +0x6` | `-` |
| `ldr` at 0x591d8 | load | 4 | `abs 0x59364` | `-` |
| `str` at 0x591dc | store | 4 | `sp -0x38` | `0x10bb4c` |
| `ldr` at 0x591e0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x591e4 | load | 1 | `ret(0x5b4d0) +0x0` | `-` |
| `str` at 0x591e8 | store | 4 | `sp -0x34` | `[ret(0x5b4d0)]:1` |
| `ldr` at 0x591ec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x591f0 | load | 1 | `ret(0x5b4d0) +0x1` | `-` |
| `str` at 0x591f4 | store | 4 | `sp -0x30` | `[ret(0x5b4d0)+1]:1` |
| `ldr` at 0x591fc | load | 4 | `abs 0x5935c` | `-` |
| `ldr` at 0x59200 | load | 4 | `abs 0x59368` | `-` |
| `ldrb` at 0x5920c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59210 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59214 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x59224 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x59234 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x59244 | load | 4 | `abs 0x5936c` | `-` |
| `str` at 0x59248 | store | 4 | `sp -0x38` | `0x10bb78` |
| `ldr` at 0x5924c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59250 | load | 1 | `ret(0x5b4d0) +0x0` | `-` |
| `str` at 0x59254 | store | 4 | `sp -0x34` | `[ret(0x5b4d0)]:1` |
| `ldr` at 0x59258 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5925c | load | 1 | `ret(0x5b4d0) +0x1` | `-` |
| `str` at 0x59260 | store | 4 | `sp -0x30` | `[ret(0x5b4d0)+1]:1` |
| `ldr` at 0x59268 | load | 4 | `abs 0x5935c` | `-` |
| `ldr` at 0x5926c | load | 4 | `abs 0x59370` | `-` |
| `ldr` at 0x59278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5927c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59288 | store | 4 | `sp -0x24` | `ret(0x5a4c4)` |
| `ldr` at 0x5928c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59298 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x592a4 | load | 4 | `abs 0x59374` | `-` |
| `str` at 0x592a8 | store | 4 | `sp -0x38` | `0x10bb98` |
| `ldr` at 0x592ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x592b0 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x592b4 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x592b8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x592bc | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x592c0 | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x592c4 | store | 4 | `sp -0x2c` | `ret(0x219a8)` |
| `ldr` at 0x592cc | load | 4 | `abs 0x5935c` | `-` |
| `ldr` at 0x592dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x592e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x592f0 | store | 4 | `sp -0x24` | `ret(0x5c008)` |
| `ldr` at 0x592f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59300 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5930c | load | 4 | `abs 0x59378` | `-` |
| `str` at 0x59310 | store | 4 | `sp -0x38` | `0x10bbc0` |
| `ldr` at 0x59314 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59318 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x5931c | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x59320 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59324 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x59328 | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x5932c | store | 4 | `sp -0x2c` | `ret(0x219a8)` |
| `ldr` at 0x59334 | load | 4 | `abs 0x5935c` | `-` |
| `ldr` at 0x59338 | load | 4 | `abs 0x5937c` | `-` |
| `ldmdb` at 0x59344 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59344 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59108 | `(unknown callee) 0x5b4d0` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x59130 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x5913c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5916c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x5917c | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x591a8 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0x31b` | `0x0` |
| `bl` at 0x591bc | `(unknown callee) 0xd6cf8` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59208 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0x326` | `0x0` |
| `bl` at 0x5921c | `(unknown callee) 0xd6cf8` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59274 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0x331` | `0x0` |
| `bl` at 0x59280 | `(unknown callee) 0x5a4c4` | `[sp-20]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x5929c | `(unknown callee) 0x219a8` | `ret(0x5a4c4)` | `?` | `?` | `ret(0x5a4c4)` |
| `bl` at 0x592d8 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0x338` | `0x0` |
| `bl` at 0x592e8 | `(unknown callee) 0x5c008` | `[sp-20]` | `[sp-32]` | `0x1` | `?` |
| `bl` at 0x59304 | `(unknown callee) 0x219a8` | `ret(0x5c008)` | `?` | `?` | `ret(0x5c008)` |
| `bl` at 0x59340 | `(unknown callee) 0x21e28` | `0x1` | `0x10ba60` | `0x33f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x590d0 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `bne` at 0x5911c | `ret(0x5b4d0) != 0x0` | `ret(0x5b4d0) == 0x0` |
| `beq` at 0x591d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59230 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x59240 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x59294 | `ret(0x5a4c4) == 0x0` | `ret(0x5a4c4) != 0x0` |
| `beq` at 0x592fc | `ret(0x5c008) == 0x0` | `ret(0x5c008) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59344 | `r0` | `?` |

## `pcixPresentInt`

- `pcixPresentInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59380 to 0x59534.
  Established. The opening branch has an unknown operand (`beq` at 0x593d4). The first call targets
  `0x5b66c` with candidates entry r0, `(r1 & 0xff)`, unknown and `(r1 & 0xff)` (`bl` at 0x593f0). The common
  exit retains an unknown r0 value (`ldmdb` at 0x59520). Presence-interrupt semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59390 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5939c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x593a4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x593ac | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x593b4 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0x593b8 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x593bc | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x593c4 | store | 1 | `sp -0x21` | `?` |
| `ldrb` at 0x593c8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x593dc | store | 1 | `sp -0x22` | `0x1` |
| `ldrb` at 0x593e0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x593e4 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x593e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x593f8 | store | 4 | `sp -0x1c` | `ret(0x5b66c)` |
| `ldr` at 0x593fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59400 | load | 1 | `ret(0x5b66c) +0x6` | `-` |
| `ldrb` at 0x59414 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5942c | load | 4 | `abs 0x59524` | `-` |
| `ldr` at 0x59430 | load | 4 | `abs 0x59528` | `-` |
| `str` at 0x5945c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x59460 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59468 | load | 4 | `abs 0x59524` | `-` |
| `ldr` at 0x5946c | load | 4 | `abs 0x59528` | `-` |
| `ldr` at 0x59478 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59488 | load | 4 | `abs 0x5952c` | `-` |
| `ldrb` at 0x59498 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x594b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x594b8 | load | 4 | `abs 0x59524` | `-` |
| `ldr` at 0x594bc | load | 4 | `abs 0x59528` | `-` |
| `str` at 0x594e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x594ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x594f4 | load | 4 | `abs 0x59524` | `-` |
| `ldr` at 0x594f8 | load | 4 | `abs 0x59528` | `-` |
| `ldr` at 0x59504 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59514 | load | 4 | `abs 0x59530` | `-` |
| `ldmdb` at 0x59520 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59520 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x593f0 | `(unknown callee) 0x5b66c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x59434 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x59440 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x59470 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x59480 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x59490 | `(unknown callee) 0x2070c` | `0x10bbe8` | `ret(0x1b508)` | `?` | `ret(0x1b508)` |
| `bl` at 0x594c0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x594cc | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x594fc | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10baac` | `0x10bab4` |
| `bl` at 0x5950c | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5951c | `(unknown callee) 0x2070c` | `0x10bc20` | `ret(0x1b508)` | `?` | `ret(0x1b508)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x593d4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5940c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59420 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x594ac | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59520 | `r0` | `?` |

## `pcixSetM66EN`

- `pcixSetM66EN`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5964c to 0x597d4.
  Established. The first branch takes `[0x148360] != 0x0` and falls through on zero (`bne` at
  0x59680). The call to `0x5c29c` has candidates `0xa`, `[sp-28]`, unknown and `[sp-48]` (`bl` at 0x59764).
  The signal timing and M66EN polarity remain unknown, as does the merged return (`ldmdb` at
  0x597c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5965c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59660 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x59668 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x59670 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x59674 | load | 4 | `abs 0x597cc` | `-` |
| `ldr` at 0x59678 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x59688 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x59690 | load | 4 | `abs 0x597d0` | `-` |
| `ldr` at 0x59694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x596a8 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x596b0 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x596b8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x596bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x596c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x596c4 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x596d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x596d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x596e0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x596e8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x596ec | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x596f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59704 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59708 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59718 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5971c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5972c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59734 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x5973c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59748 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59754 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59768 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59770 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x59774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59778 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5977c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59788 | store | 4 | `sp -0x28` | `ret(0x5c094)` |
| `ldr` at 0x5978c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59798 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x597a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x597a8 | store | 4 | `sp -0x24` | `ret(0x5c094)` |
| `ldr` at 0x597ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x597b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x597bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x597c0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x597c4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x597c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x597c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x597c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59764 | `(unknown callee) 0x5c29c` | `0xa` | `[sp-28]` | `?` | `[sp-48]` |
| `bl` at 0x59780 | `(unknown callee) 0x5c094` | `[sp-20]` | `?` | `[sp-24]` | `[sp-48]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59680 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `blt` at 0x596cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x596fc | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59714 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59724 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x59744 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x59750 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `bne` at 0x5975c | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `beq` at 0x59794 | `ret(0x5c094) == 0x0` | `ret(0x5c094) != 0x0` |
| `bne` at 0x597a0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x597c8 | `r0` | `?` |

## `pcixOverrideSpeed`

- `pcixOverrideSpeed`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59930 to 0x59a9c.
  Established. A global word read accesses absolute `0x148360` (`ldr` at 0x59974). The nonzero test
  takes the branch and zero falls through (`bne` at 0x5997c). The call to `0x1d38c` has candidates
  unknown, `[sp-30]:1`, `[sp-30]:1` and unknown (`bl` at 0x59a10). Speed encoding and the final
  value are unknown (`ldmdb` at 0x59a8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59940 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5994c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x59954 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x5995c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x59960 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59964 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x5996c | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x59970 | load | 4 | `abs 0x59a90` | `-` |
| `ldr` at 0x59974 | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x59980 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5998c | load | 4 | `abs 0x59a94` | `-` |
| `str` at 0x59990 | store | 4 | `sp -0x28` | `0xfffff703` |
| `str` at 0x5999c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x599a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x599a4 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x599ac | load | 4 | `abs 0x59a98` | `-` |
| `ldr` at 0x599b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599c4 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x599cc | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x599d0 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x599e0 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x599e4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x599e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x599ec | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x599fc | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x59a04 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x59a14 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x59a1c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldrb` at 0x59a24 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59a30 | load | 4 | `abs 0x59a94` | `-` |
| `str` at 0x59a34 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x59a3c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x59a44 | store | 1 | `sp -0x16` | `([sp-22]:1-1 & 0xff)` |
| `ldrb` at 0x59a48 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59a50 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x59a5c | load | 4 | `abs 0x59a94` | `-` |
| `str` at 0x59a60 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x59a68 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x59a70 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x59a84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x59a88 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x59a8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59a8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59a8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59a10 | `(unknown callee) 0x1d38c` | `?` | `[sp-30]:1` | `[sp-30]:1` | `?` |
| `bl` at 0x59a7c | `(unknown callee) 0x1d38c` | `([sp-22]:1-1 & 0xff)+170` | `[sp-30]:1` | `[sp-30]:1` | `([sp-22]:1-1 & 0xff)+170` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5997c | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x59988 | `(r2 & 0xff) == 0xff` | `(r2 & 0xff) != 0xff` |
| `bne` at 0x599d8 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `blt` at 0x599f4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59a2c | `[sp-22]:1 != 0x0` | `[sp-22]:1 == 0x0` |
| `ble` at 0x59a58 | `([sp-22]:1-1 & 0xff) <= [[sp-28]+8] (signed, 32-bit)` | `([sp-22]:1-1 & 0xff) > [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59a8c | `r0` | `?` |

## `pcixSlotIsPresent`

- `pcixSlotIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59c4c to 0x59d18.
  Established. A word load reads absolute `0x14835c` (`ldr` at 0x59c80). The later global guard
  takes `[0x148360] != 0x0` and falls through on zero (`bne` at 0x59c98). The recorded r0 exit is
  unknown (`ldmdb` at 0x59d0c). The slot-presence definition remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59c5c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59c64 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x59c68 | load | 4 | `abs 0x59d10` | `-` |
| `ldr` at 0x59c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c80 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59c88 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59c8c | load | 4 | `abs 0x59d14` | `-` |
| `ldr` at 0x59c90 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x59ca0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x59ca8 | load | 4 | `abs 0x59d10` | `-` |
| `ldr` at 0x59cac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59cc0 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x59cc8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x59ccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59cd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59cd4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59cdc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0x59ce4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59cec | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x59cf4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x59d04 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x59d08 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x59d0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59d0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59d0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59c98 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59d0c | `r0` | `?` |

## `NL4R_Init`

- `NL4R_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27808 to 0x27940.
  Established. The byte read uses absolute `0x147eb8` (`ldrb` at 0x27824). Zero takes the branch and
  nonzero falls through (`beq` at 0x2782c). The first call to `0x21e28` has `0x9`, `0x1012e4`,
  `0x71` and `0x0` as candidates (`bl` at 0x27848). The return carries `ret(0x37344)` (`ldmdb` at
  0x27924), whose hardware meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2781c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x27820 | load | 4 | `abs 0x27928` | `-` |
| `ldrb` at 0x27824 | load | 1 | `abs 0x147eb8` | `-` |
| `ldr` at 0x27830 | load | 4 | `abs 0x2792c` | `-` |
| `str` at 0x27834 | store | 4 | `sp -0x2c` | `0x1012bc` |
| `ldr` at 0x2783c | load | 4 | `abs 0x27930` | `-` |
| `ldr` at 0x2784c | load | 4 | `abs 0x27928` | `-` |
| `strb` at 0x27854 | store | 1 | `abs 0x147eb8` | `0x1` |
| `ldr` at 0x27858 | load | 4 | `abs 0x27934` | `-` |
| `ldr` at 0x2785c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27860 | store | 4 | `abs 0x147ebc` | `[sp-20]` |
| `ldr` at 0x27864 | load | 4 | `abs 0x27938` | `-` |
| `ldr` at 0x27868 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2786c | store | 4 | `abs 0x147ec0` | `[sp-24]` |
| `str` at 0x27874 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27878 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2787c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2788c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2789c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x278a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x278ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x278b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x278bc | store | 4 | `sp -0x20` | `ret(0xe92b4)` |
| `ldr` at 0x278c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x278cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x278d8 | load | 4 | `abs 0x2793c` | `-` |
| `str` at 0x278dc | store | 4 | `sp -0x2c` | `0x1012f8` |
| `ldr` at 0x278e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x278e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x278f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x278f4 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x278f8 | store | 4 | `sp -0x24` | `ret(0x21674)` |
| `ldr` at 0x27900 | load | 4 | `abs 0x27930` | `-` |
| `ldr` at 0x27910 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27918 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x27924 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27924 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27924 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27848 | `(unknown callee) 0x21e28` | `0x9` | `0x1012e4` | `0x71` | `0x0` |
| `bl` at 0x278b4 | `(unknown callee) 0xe92b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x278d0 | `(unknown callee) 0x21674` | `ret(0xe92b4)` | `?` | `?` | `ret(0xe92b4)` |
| `bl` at 0x2790c | `(unknown callee) 0x21e28` | `0x9` | `0x1012e4` | `0x81` | `0x0` |
| `bl` at 0x27920 | `(unknown callee) 0x37344` | `?` | `?` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2782c | `[0x147eb8]:1 == 0x0` | `[0x147eb8]:1 != 0x0` |
| `blt` at 0x27884 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x278c8 | `ret(0xe92b4) == 0x0` | `ret(0xe92b4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27924 | `r0` | `ret(0x37344)` |

## `NL4RTarget`

- `NL4RTarget`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27940 to 0x2797c.
  Established. The word load reads absolute `0x147ebc` (`ldr` at 0x27964). The later record access
  leaves the final r0 expression unknown (`ldmdb` at 0x27974). The target record geometry and
  physical mapping remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x27954 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x27958 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x27960 | load | 4 | `abs 0x27978` | `-` |
| `ldr` at 0x27964 | load | 4 | `abs 0x147ebc` | `-` |
| `ldr` at 0x2796c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x27974 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27974 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27974 | `r0` | `?` |

## `NL4R_SysComEnable`

- `NL4R_SysComEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27d28 to 0x27e24.
  Established. The selector takes `r0 != 0xff` and falls through on equality (`bne` at 0x27d44). One
  call to `0x2070c` has candidates `0x101314`, `0x147ebc`, unknown and unknown (`bl` at 0x27d84).
  The exit records `[sp-28]` (`ldmdb` at 0x27e14). The link-enable effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27d38 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x27d3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27d4c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27d50 | load | 4 | `abs 0x27e18` | `-` |
| `ldr` at 0x27d54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27d58 | load | 4 | `abs 0x147ec0` | `-` |
| `ldr` at 0x27d68 | load | 4 | `abs 0x27e1c` | `-` |
| `ldr` at 0x27d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27d74 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27d7c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x27d80 | load | 4 | `abs 0x27e20` | `-` |
| `ldr` at 0x27d88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x27d8c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x27da0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27da8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27db0 | load | 4 | `abs 0x27e18` | `-` |
| `ldr` at 0x27db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27db8 | load | 4 | `abs 0x147ec0` | `-` |
| `str` at 0x27dc8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x27dd0 | load | 4 | `abs 0x27e1c` | `-` |
| `ldr` at 0x27dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27ddc | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27de4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x27de8 | load | 4 | `abs 0x27e20` | `-` |
| `ldr` at 0x27df0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x27df4 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0x27e0c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27e10 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x27e14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27e14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27e14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27d84 | `(unknown callee) 0x2070c` | `0x101314` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27d9c | `(unknown callee) 0xd6d40` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |
| `bl` at 0x27dec | `(unknown callee) 0x2070c` | `0x101314` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27e04 | `(unknown callee) 0xd6d40` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27d44 | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x27d60 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x27dc0 | `r0 < [0x147ec0] (signed, 32-bit)` | `r0 >= [0x147ec0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27e14 | `r0` | `[sp-28]` |

## `NL4R_SysComInt`

- `NL4R_SysComInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27e24 to 0x27f18.
  Established. The loop comparison has unknown operands (`blt` at 0x27e6c). The call to `0x21e28`
  has candidates `0x1`, `0x1012e4`, `0x127` and `0x0` (`bl` at 0x27e90). The final r0 value is unknown
  (`ldmdb` at 0x27f00). A specific system-communication interrupt action is unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27e34 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x27e40 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x27e48 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x27e50 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x27e58 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x27e5c | load | 4 | `abs 0x27f04` | `-` |
| `ldr` at 0x27e60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27e64 | load | 4 | `abs 0x147ec0` | `-` |
| `ldr` at 0x27e70 | load | 4 | `abs 0x27f08` | `-` |
| `str` at 0x27e74 | store | 4 | `sp -0x28` | `0x101324` |
| `ldrb` at 0x27e78 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x27e7c | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x27e84 | load | 4 | `abs 0x27f0c` | `-` |
| `ldr` at 0x27e88 | load | 4 | `abs 0x27f10` | `-` |
| `ldr` at 0x27e98 | load | 4 | `abs 0x27f14` | `-` |
| `ldr` at 0x27ea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27eac | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27eb4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x27ebc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27ec0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x27ec4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x27ec8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x27ed4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x27ed8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x27ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x27ef4 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x27f00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27f00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27f00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27e90 | `(unknown callee) 0x21e28` | `0x1` | `0x1012e4` | `0x127` | `0x0` |
| `bl` at 0x27efc | `(unknown callee) 0x837e4` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x27e6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x27ed0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x27eec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27f00 | `r0` | `?` |

## `PICInit`

- `PICInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34008 to 0x34294.
  Established. The word load reads absolute `0x147f4c` (`ldr` at 0x34024); its zero value takes the
  branch and nonzero falls through (`beq` at 0x3402c). The opening call to `0x21e28` has candidates `0x9`,
  `0x105b30`, `0x3b` and `0x0` (`bl` at 0x34054). The controller-initialisation role remains unknown
  beyond the explicit accesses and calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34018 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3401c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x34020 | load | 4 | `abs 0x3426c` | `-` |
| `ldr` at 0x34024 | load | 4 | `abs 0x147f4c` | `-` |
| `ldr` at 0x34030 | load | 4 | `abs 0x34270` | `-` |
| `str` at 0x34034 | store | 4 | `sp -0x34` | `0x105b14` |
| `ldr` at 0x34038 | load | 4 | `abs 0x34274` | `-` |
| `ldrb` at 0x3403c | load | 1 | `abs 0x147f50` | `-` |
| `str` at 0x34040 | store | 4 | `sp -0x30` | `[0x147f50]:1` |
| `ldr` at 0x34048 | load | 4 | `abs 0x34278` | `-` |
| `str` at 0x3405c | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x34064 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3406c | load | 4 | `abs 0x3427c` | `-` |
| `ldr` at 0x34070 | load | 4 | `abs 0x34280` | `-` |
| `ldr` at 0x34074 | load | 4 | `abs 0x34284` | `-` |
| `str` at 0x34080 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x34084 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34088 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x34098 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x340ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x340b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x340c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x340d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x340d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x340e0 | store | 4 | `sp -0x24` | `ret(0xe92b4)` |
| `ldr` at 0x340e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x340f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x340fc | load | 4 | `abs 0x34288` | `-` |
| `str` at 0x34100 | store | 4 | `sp -0x34` | `0x105b44` |
| `ldr` at 0x34104 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34118 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x34120 | load | 4 | `(unknown)` | `-` |
| `str` at 0x34124 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x34128 | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0x34130 | load | 4 | `abs 0x34278` | `-` |
| `ldr` at 0x34140 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x34148 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x34150 | load | 4 | `abs 0x3426c` | `-` |
| `ldr` at 0x34154 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x34158 | store | 4 | `abs 0x147f4c` | `[sp-20]` |
| `ldr` at 0x3415c | load | 4 | `abs 0x34274` | `-` |
| `ldrb` at 0x34160 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x34164 | store | 1 | `abs 0x147f50` | `([sp-24]:1 & 0xff)` |
| `str` at 0x3416c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x34170 | load | 4 | `abs 0x3428c` | `-` |
| `strb` at 0x34178 | store | 1 | `abs 0x147f51` | `0x1` |
| `ldr` at 0x3417c | load | 4 | `abs 0x34274` | `-` |
| `ldrb` at 0x34180 | load | 1 | `abs 0x147f50` | `-` |
| `ldr` at 0x34184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x34190 | load | 4 | `abs 0x3428c` | `-` |
| `ldrb` at 0x34194 | load | 1 | `abs 0x147f51` | `-` |
| `ldr` at 0x341a4 | load | 4 | `abs 0x3426c` | `-` |
| `ldr` at 0x341a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x341bc | load | 4 | `abs 0x147f4c` | `-` |
| `str` at 0x341c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x341c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x341cc | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0x341e4 | store | 4 | `sp -0x24` | `ret(0xd6b9c)` |
| `ldr` at 0x341e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x341f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34200 | load | 4 | `abs 0x34290` | `-` |
| `str` at 0x34204 | store | 4 | `sp -0x34` | `0x105b60` |
| `str` at 0x34208 | store | 4 | `sp -0x30` | `ret(0x219a8)` |
| `ldr` at 0x34210 | load | 4 | `abs 0x34278` | `-` |
| `ldr` at 0x34220 | load | 4 | `abs 0x3428c` | `-` |
| `strb` at 0x34228 | store | 1 | `abs 0x147f51` | `0x0` |
| `ldr` at 0x34230 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x34234 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x34238 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldr` at 0x3424c | load | 4 | `abs 0x3428c` | `-` |
| `strb` at 0x34254 | store | 1 | `abs 0x147f51` | `0x0` |
| `ldr` at 0x34258 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x34260 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x34268 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34268 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34054 | `(unknown callee) 0x21e28` | `0x9` | `0x105b30` | `0x3b` | `0x0` |
| `bl` at 0x34078 | `(unknown callee) 0x5796c` | `0x0` | `0x34740` | `0x34768` | `0x347b8` |
| `bl` at 0x340d8 | `(unknown callee) 0xe92b4` | `?` | `?` | `?` | `?` |
| `bl` at 0x340f4 | `(unknown callee) 0x21674` | `ret(0xe92b4)` | `?` | `?` | `ret(0xe92b4)` |
| `bl` at 0x3413c | `(unknown callee) 0x21e28` | `0x1` | `0x105b30` | `0x49` | `0x0` |
| `bl` at 0x341dc | `(unknown callee) 0xd6b9c` | `0x0` | `[[sp-28]]:1` | `sp-29` | `[[sp-28]]:1` |
| `bl` at 0x341f8 | `(unknown callee) 0x219a8` | `ret(0xd6b9c)` | `?` | `?` | `ret(0xd6b9c)` |
| `bl` at 0x3421c | `(unknown callee) 0x21e28` | `0x1` | `0x105b30` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3402c | `[0x147f4c] == 0x0` | `[0x147f4c] != 0x0` |
| `blt` at 0x34090 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x340ec | `ret(0xe92b4) == 0x0` | `ret(0xe92b4) != 0x0` |
| `bge` at 0x3418c | `[sp-40] >= [0x147f50]:1 (signed, 32-bit)` | `[sp-40] < [0x147f50]:1 (signed, 32-bit)` |
| `bne` at 0x3419c | `[0x147f51]:1 != 0x0` | `[0x147f51]:1 == 0x0` |
| `beq` at 0x341f0 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `bne` at 0x34248 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34268 | `r0` | `?` |
