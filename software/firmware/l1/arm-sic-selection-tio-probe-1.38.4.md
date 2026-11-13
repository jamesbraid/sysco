# ARM SIC selection and TIO probe paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/sic-selection-tio-probe-facts.txt) retain all tool sections.

## `jmstr_sel_sic`

- `jmstr_sel_sic`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdd4e4 to 0xdf3ac.
  Established. Two word stores in separate arms write `0x4` at `[sp-64] +0x0` (`str` at 0xdd730,
  `str` at 0xdd748). A later word store at that expression has unknown value (`str` at 0xdd77c). The
  opening call to `0x1b124` carries `0x60` and entry r1, r2, r3 (`bl` at 0xdd508); its nonzero
  result takes the continuation and zero falls through (`bne` at 0xdd51c). Calls include `0xdcf54`
  with `0x0`, `0x0`, `0x1`, `[sp-52]` (`bl` at 0xdd88c). The later call to `0xe0c68` has `[sp-56]`,
  `[sp-60]`, `[sp-68]`, `[sp+8]` (`bl` at 0xdf19c). SIC-selection encoding, temporary-record
  ownership and r0 at the common exit remain unknown (`ldmdb` at 0xdf3a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdd4f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdd4f8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdd4fc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdd500 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xdd510 | store | 4 | `sp -0x40` | `ret(0x1b124)` |
| `ldr` at 0xdd514 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd520 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdd524 | load | 4 | `abs 0xde4bc` | `-` |
| `str` at 0xdd530 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdd544 | store | 4 | `sp -0x44` | `ret(0x1b124)` |
| `ldr` at 0xdd548 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdd554 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdd558 | load | 4 | `abs 0xde4bc` | `-` |
| `ldr` at 0xdd560 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdd56c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdd574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd584 | load | 4 | `abs 0xde4c0` | `-` |
| `str` at 0xdd588 | store | 4 | `sp -0x28` | `0xff800000` |
| `str` at 0xdd590 | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0xdd598 | load | 4 | `abs 0xde4c4` | `-` |
| `str` at 0xdd59c | store | 4 | `sp -0x28` | `0xfff00000` |
| `ldr` at 0xdd5a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd5a4 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xdd5a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd5ac | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdd5b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd5c0 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdd5cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd5d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd5d4 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdd5e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd5e8 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdd5f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd5f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd5fc | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdd608 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd610 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdd61c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd620 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd630 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdd644 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd648 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd658 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdd66c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xdd670 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdd67c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdd684 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdd68c | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xdd690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdd6a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd6a8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdd6bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd6c4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd6d0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdd6dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd6e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdd6f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd6fc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd708 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdd710 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdd724 | store | 4 | `sp -0x24` | `0x9` |
| `ldr` at 0xdd728 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdd730 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `str` at 0xdd73c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xdd740 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdd748 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `ldr` at 0xdd74c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd750 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd754 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd758 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdd764 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd768 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd770 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdd77c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd780 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd784 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd788 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd78c | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdd798 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd79c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd7a4 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdd7b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd7b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd7b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd7bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd7c0 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdd7cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd7d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd7d8 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdd7e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd7e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd7ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd7f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd800 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd804 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdd818 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd81c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd820 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd824 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd834 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd838 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdd84c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd850 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd858 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdd85c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd860 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdd864 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdd868 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdd86c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd870 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdd874 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdd878 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdd888 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdd89c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd8a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd8b0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdd8b8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdd8bc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdd8c0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdd8cc | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xdd8d4 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xdd8d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd8dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdd8e0 | store | 4 | `[sp-64] +0x0` | `[sp-24]` |
| `ldr` at 0xdd8e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdd8ec | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdd8f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd8f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd8f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd8fc | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdd908 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd90c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd914 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdd920 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd924 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd928 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd92c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd930 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdd93c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd940 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd948 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdd954 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd958 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd95c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd960 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd964 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdd970 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd974 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd97c | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdd988 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd98c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd990 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd994 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd9a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd9a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdd9bc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd9c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd9c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd9c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdd9d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdd9dc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdd9f0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdd9f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdd9f8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdd9fc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdda00 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdda04 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdda08 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdda0c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdda10 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdda20 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdda34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdda3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdda48 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdda50 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdda54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdda58 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdda60 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdda64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdda74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdda88 | store | 4 | `sp -0x4c` | `0x9` |
| `str` at 0xdda94 | store | 4 | `sp -0x4c` | `0xc` |
| `ldr` at 0xdda98 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xdda9c | store | 4 | `sp -0x24` | `[sp-76]` |
| `ldr` at 0xddaa0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddaa8 | store | 4 | `[sp-64] +0x0` | `0xe` |
| `ldr` at 0xddaac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddab4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xddab8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddabc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddac0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddac4 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xddad0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddad4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddadc | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xddae8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddaec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddaf0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddaf4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddaf8 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xddb04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb08 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddb10 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xddb1c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddb20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb28 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddb2c | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xddb38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb3c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddb44 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xddb50 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddb54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb5c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddb6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb70 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddb84 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddb88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddb90 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddba0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddba4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddbb8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddbbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xddbc0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xddbc4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddbc8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xddbcc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddbd0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xddbd4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xddbd8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xddbe8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xddbfc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc04 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddc10 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xddc18 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddc1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddc20 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xddc2c | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xddc34 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xddc38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc3c | load | 4 | `sp +0x0` | `-` |
| `str` at 0xddc40 | store | 4 | `[sp-64] +0x0` | `[sp]` |
| `ldr` at 0xddc44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc4c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddc50 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xddc5c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc60 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddc68 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xddc74 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddc78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc80 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddc84 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xddc90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddc94 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddc9c | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xddca8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddcac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddcb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddcb4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddcb8 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xddcc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddcc8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddcd0 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xddcdc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddce0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddce4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddce8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddcf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddcfc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddd10 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddd14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddd18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddd1c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddd2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddd30 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddd44 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddd4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddd58 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xddd5c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddd64 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xddd6c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddd78 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xddd7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xddd80 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xddd84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddd88 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xddd8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddd90 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xddd94 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xddd98 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xddda8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdddbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdddc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdddd0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdddd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xddde4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddde8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdddec | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdddf4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdddfc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdde08 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xdde14 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdde20 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdde28 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xdde34 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xdde38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdde4c | store | 4 | `sp -0x50` | `0x9` |
| `str` at 0xdde58 | store | 4 | `sp -0x50` | `0xc` |
| `ldr` at 0xdde5c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xdde60 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0xdde64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdde68 | load | 4 | `abs 0xde4c8` | `-` |
| `str` at 0xdde6c | store | 4 | `[sp-64] +0x0` | `0xfff` |
| `ldr` at 0xdde70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdde78 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdde7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdde80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdde84 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdde88 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdde94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdde98 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddea0 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xddeac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddeb0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddeb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddeb8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddebc | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xddec8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddecc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdded4 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xddee0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddee4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddee8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddeec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddef0 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xddefc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf00 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddf08 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xddf14 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddf18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf20 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddf30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf34 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddf48 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddf4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xddf64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddf68 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xddf7c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xddf80 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xddf84 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xddf88 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xddf8c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xddf90 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddf94 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xddf98 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xddf9c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xddfac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xddfc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xddfc8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xddfd4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xddfdc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddfe0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddfe4 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xddff0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xddff4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddff8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xddffc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde000 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde008 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde00c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde010 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde020 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde024 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde028 | load | 4 | `abs 0xde4d8` | `-` |
| `ldr` at 0xde02c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde034 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde03c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde048 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xde054 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xde058 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde05c | load | 4 | `abs 0xde4cc` | `-` |
| `str` at 0xde060 | store | 4 | `[sp-64] +0x0` | `0xa800ffea` |
| `ldr` at 0xde064 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde06c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde070 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde074 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde078 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde07c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde080 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde084 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde088 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde08c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde09c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde0b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde0c4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde0cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde0d0 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde0d4 | load | 4 | `abs 0xde4d0` | `-` |
| `ldr` at 0xde0e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde0e4 | load | 4 | `abs 0xde4d4` | `-` |
| `ldr` at 0xde0ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde0f4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde100 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde108 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde11c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xde120 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde128 | store | 4 | `[sp-64] +0x0` | `0x9` |
| `ldr` at 0xde12c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde134 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde138 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde13c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde140 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde144 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xde150 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde154 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde15c | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xde168 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde16c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde170 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde174 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde178 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xde184 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde188 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde190 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xde19c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde1a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde1ac | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xde1b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde1c4 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xde1d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde1d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde1ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde1f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde204 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde208 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde20c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde210 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde220 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde224 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde238 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde23c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde240 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde244 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde248 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde24c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde250 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde254 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde258 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde268 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde27c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde284 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde290 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde298 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde29c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde2a0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde2ac | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde2b0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde2b4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde2b8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde2bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde2c4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde2c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde2cc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde2d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde2dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde2e0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde2e4 | load | 4 | `abs 0xde4d8` | `-` |
| `ldr` at 0xde2e8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde2f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde2f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde304 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xde310 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xde314 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde318 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xde31c | store | 4 | `[sp-64] +0x0` | `[sp+4]` |
| `ldr` at 0xde320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde324 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde328 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde32c | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xde338 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde33c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde344 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xde350 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde354 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde358 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde35c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde360 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xde36c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde370 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde378 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xde384 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde388 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde38c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde390 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde394 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xde3a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3a4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde3ac | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xde3b8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde3bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3c4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde3d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3d8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde3ec | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde3f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde3f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde408 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde40c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde420 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde424 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde42c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde430 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde434 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde438 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde43c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde440 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde444 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde448 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde44c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde45c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde470 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde478 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde484 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde48c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde490 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde494 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde49c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde4a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde4b4 | store | 4 | `sp -0x54` | `0x9` |
| `str` at 0xde4f4 | store | 4 | `sp -0x54` | `0xc` |
| `ldr` at 0xde4f8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xde4fc | store | 4 | `sp -0x24` | `[sp-84]` |
| `ldr` at 0xde500 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde508 | store | 4 | `[sp-64] +0x0` | `0x8` |
| `ldr` at 0xde50c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde514 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde518 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde51c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde520 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde524 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xde530 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde534 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde53c | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xde548 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde54c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde550 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde554 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde558 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xde564 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde568 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde570 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xde57c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde580 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde584 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde588 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde58c | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xde598 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde59c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde5a4 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xde5b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde5b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde5cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde5e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde5e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde5f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde600 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde604 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde618 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde61c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde620 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde624 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde628 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde62c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde630 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde634 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde638 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xde648 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xde65c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde664 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde670 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde678 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde67c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde680 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xde68c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde690 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde694 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde698 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde69c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xde6a4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xde6a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde6ac | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde6b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xde6bc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xde6c0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xde6c4 | load | 4 | `abs 0xde4d8` | `-` |
| `ldr` at 0xde6c8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xde6d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde6d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde6e4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xde6ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xde700 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xde704 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde708 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde710 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde714 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde718 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde71c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde720 | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xde72c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde730 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde738 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xde744 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde748 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde74c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde750 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde754 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xde760 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde764 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde76c | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xde778 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde77c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde780 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde784 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde788 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xde794 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde798 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde7a0 | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xde7ac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde7b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7b8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde7c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7cc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde7e0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde7e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde7ec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde7fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde800 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde814 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde81c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde828 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xde82c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde834 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde83c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde848 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `str` at 0xde854 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xde858 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde868 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde86c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde874 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde87c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde880 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xde88c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xde898 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde8a0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde8ac | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xde8b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde8b8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xde8c0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde8cc | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xde8d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8d8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde8dc | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xde8e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde8ec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde8f4 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xde900 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde904 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde908 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde90c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde910 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xde91c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde920 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde928 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xde934 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde938 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde93c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde940 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde944 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xde950 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde954 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde95c | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xde968 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde96c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde970 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde974 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde984 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde988 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde99c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde9a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde9a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde9a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xde9b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xde9bc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xde9d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xde9d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xde9d8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xde9dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xde9e0 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xde9e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xde9e8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xde9ec | load | 4 | `sp +0x10` | `-` |
| `str` at 0xde9f0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdea00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdea14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdea1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdea28 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdea30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdea3c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea44 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdea4c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdea58 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea64 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdea6c | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xdea78 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xdea7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdea8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdeaa0 | store | 4 | `sp -0x58` | `0x9` |
| `str` at 0xdeaac | store | 4 | `sp -0x58` | `0xc` |
| `ldr` at 0xdeab0 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xdeab4 | store | 4 | `sp -0x24` | `[sp-88]` |
| `ldr` at 0xdeab8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdeac0 | store | 4 | `[sp-64] +0x0` | `0x1` |
| `ldr` at 0xdeac4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdeacc | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdead0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdead4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdead8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeadc | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdeae8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeaec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeaf4 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdeb00 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb0c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb10 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdeb1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb20 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb28 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdeb34 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb40 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb44 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdeb50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb5c | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdeb68 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeb6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb74 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeb84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeb88 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdeb9c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdeba0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeba4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeba8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdebb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdebbc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xdebd0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdebd4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdebd8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xdebdc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdebe0 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdebe4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdebe8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdebec | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdebf0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdec00 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdec14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdec28 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdec30 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdec38 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdec3c | store | 4 | `[sp-68] +0x0` | `[[sp-68]]` |
| `str` at 0xdec44 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xdec48 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdec50 | store | 4 | `[sp-64] +0x0` | `0x0` |
| `ldr` at 0xdec54 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdec5c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xdec60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec68 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdec6c | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdec78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec7c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdec84 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdec90 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdec94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdec9c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdeca0 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdecac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecb0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdecb8 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdecc4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdecc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdeccc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdecd0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdecd4 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdece0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdece4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xdecec | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdecf8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xdecfc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded04 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded18 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xded2c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xded30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded38 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xded48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xded4c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xded60 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xded64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xded68 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xded6c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xded70 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xded74 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xded78 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xded7c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xded80 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xded90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdeda4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdedac | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdedb8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdedc0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdedc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdedc8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdedcc | load | 4 | `abs 0xde4dc` | `-` |
| `ldr` at 0xdedd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdeddc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdede4 | load | 4 | `abs 0xde4dc` | `-` |
| `str` at 0xdedf0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdedf4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdedf8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdedfc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdee00 | load | 4 | `abs 0xde4e0` | `-` |
| `ldr` at 0xdee0c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee10 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdee18 | load | 4 | `abs 0xde4e0` | `-` |
| `str` at 0xdee24 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdee28 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee2c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee30 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdee34 | load | 4 | `abs 0xde4e4` | `-` |
| `ldr` at 0xdee40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee44 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdee4c | load | 4 | `abs 0xde4e4` | `-` |
| `str` at 0xdee58 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdee5c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee60 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee64 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdee74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee78 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdee8c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdee90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee94 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdee98 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xdeea8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdeeac | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdeec0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xdeec4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xdeec8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdeecc | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xdeed0 | store | 4 | `[sp+8] +0x0` | `[[sp-68]]` |
| `str` at 0xdeed8 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xdeedc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdeef0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdeef4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdef08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef10 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdef1c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdef24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdef34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdef44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdef5c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdef60 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdef74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdef7c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdef88 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdef90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdefa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdefb8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdefc0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdefd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdefdc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdefe8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdeff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf000 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf008 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf014 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xdf020 | store | 4 | `sp -0x38` | `0x100` |
| `ldr` at 0xdf024 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf034 | store | 4 | `sp -0x3c` | `0x300` |
| `ldr` at 0xdf038 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdf04c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf058 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdf060 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdf064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf068 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf078 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf080 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf08c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf090 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf098 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf0a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf0ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf0bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf0c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf0d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf0d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf0dc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf0e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf0f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf108 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf110 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf11c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf120 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf128 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf130 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf134 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xdf138 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf13c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdf140 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf144 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdf148 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdf14c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdf15c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf170 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf178 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf184 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf18c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdf190 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf194 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf198 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xdf1ac | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf1b0 | load | 4 | `abs 0xde4e8` | `-` |
| `ldr` at 0xdf1b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf1c0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf1cc | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdf1d8 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0xdf1dc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf1ec | store | 4 | `sp -0x3c` | `0xc0` |
| `ldr` at 0xdf1f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdf204 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdf210 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdf218 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xdf21c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf220 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf230 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf238 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf244 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf248 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf250 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf25c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf264 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf274 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf27c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf288 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf28c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf294 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf2a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf2b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf2c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf2c8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf2d4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf2d8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdf2e0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xdf2e8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf2ec | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xdf2f0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf2f4 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xdf2f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf2fc | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xdf300 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xdf304 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xdf314 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdf328 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf330 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf33c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf344 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdf348 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf34c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf350 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xdf364 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xdf368 | load | 4 | `abs 0xde4ec` | `-` |
| `ldr` at 0xdf370 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf378 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf384 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdf38c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf394 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf3a0 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xdf3a4 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xdf3a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdf3a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdf3a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdd508 | `(unknown callee) 0x1b124` | `0x60` | `r1` | `r2` | `r3` |
| `bl` at 0xdd528 | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x1353e8` | `?` | `ret(0x1b124)` |
| `bl` at 0xdd53c | `(unknown callee) 0x1b124` | `0x60` | `?` | `?` | `ret(0x1b124)` |
| `bl` at 0xdd55c | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x1353e8` | `?` | `ret(0x1b124)` |
| `bl` at 0xdd564 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdd6ac | `(unknown callee) 0xdc9a8` | `0x0` | `0x1` | `[sp+16]` | `?` |
| `bl` at 0xdd6c0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdd6c8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdd6e4 | `(unknown callee) 0xdc9a8` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xdd6f8 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdd700 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdd88c | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xdd8a0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdd8a8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdda24 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdda38 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdda40 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xddbec | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xddc00 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xddc08 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdddac | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdddc0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdddc8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xddfb0 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xddfc4 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xddfcc | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde030 | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135444` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xde038 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde040 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde0a0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde0b4 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xde0bc | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde0e8 | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135414` | `[[sp-68]]` | `0x54007ff5` |
| `bl` at 0xde0f0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde0f8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde26c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde280 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xde288 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde2ec | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135444` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xde2f4 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde2fc | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde460 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde474 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xde47c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde64c | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xde660 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xde668 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xde6cc | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135444` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xde6d4 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xde6dc | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdea04 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdea18 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdea20 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdec04 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xdec18 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdec20 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xded94 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdeda8 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdedb0 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdeef8 | `(unknown callee) 0xdc9a8` | `0x0` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xdef0c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdef14 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdef64 | `(unknown callee) 0xdc9a8` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xdef78 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdef80 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdefc4 | `(unknown callee) 0xdcb88` | `0x0` | `[sp-52]` | `0x0` | `[sp+16]` |
| `bl` at 0xdefd8 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcb88)` |
| `bl` at 0xdefe0 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf004 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf00c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf160 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdf174 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdf17c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf19c | `(unknown callee) 0xe0c68` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xdf1b4 | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135470` | `?` | `ret(0xe0c68)` |
| `bl` at 0xdf1bc | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf1c4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf318 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xdf32c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdf334 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf354 | `(unknown callee) 0xe0c68` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+12]` |
| `bl` at 0xdf36c | `(unknown callee) 0x1b49c` | `[sp+16]` | `0x135498` | `?` | `ret(0xe0c68)` |
| `bl` at 0xdf374 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf37c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf390 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `ret(0xe0c68)` |
| `bl` at 0xdf398 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdd51c | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xdd550 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `beq` at 0xdd580 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdd69c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdd6b8 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xdd6f0 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `beq` at 0xdd71c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdd898 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdda30 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xdda70 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdda80 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xddbf8 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdddb8 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xddde0 | `[sp-36] != 0x18` | `[sp-36] == 0x18` |
| `beq` at 0xdde44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xddfbc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xde01c | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xde0ac | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xde0dc | `[[sp-68]] == 0x54007ff5` | `[[sp-68]] != 0x54007ff5` |
| `beq` at 0xde114 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde278 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xde2d8 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xde46c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xde4ac | `? == 0x0` | `? != 0x0` |
| `bne` at 0xde658 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xde6b8 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `beq` at 0xde6f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xde864 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdea10 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdea38 | `[sp-36] != 0x8` | `[sp-36] == 0x8` |
| `beq` at 0xdea88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdea98 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdec10 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdeda0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xdeee8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdef04 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xdef30 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdef40 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdef50 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdef70 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xdef9c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdefac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xdefd0 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xdeffc | `? != 0x0` | `? == 0x0` |
| `blo` at 0xdf070 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf0b4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf100 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdf16c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdf1a8 | `ret(0xe0c68) != 0x0` | `ret(0xe0c68) == 0x0` |
| `blo` at 0xdf228 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf26c | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf2b8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdf324 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdf360 | `ret(0xe0c68) != 0x0` | `ret(0xe0c68) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdf3a8 | `r0` | `?` |

## `jmstr_sel_sic_multi`

- `jmstr_sel_sic_multi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdf3ac to 0xe0c68.
  Established. The first test takes its bypass for `[r0+16] == 0x0` and falls through on nonzero
  (`beq` at 0xdf3e8). The next unsigned test takes the same bypass for `[r0+16] > 0x8` and falls
  through for less or equal (`bhi` at 0xdf3f4). The call to `0x1b124` carries `0x60`, entry r1,
  `0x10`, `[r0+16]` (`bl` at 0xdf41c). Three early word stores have unknown targets, with unknown,
  unknown and `0x0` values (`str` at 0xdf534, `str` at 0xdf5c0, `str` at 0xdf6d0). Calls include
  `0xe0e38` with `sp-136`, `0xc`, `[sp-104]`, unknown and `0xe12e8` with `[sp-104]`, `0x8`, `0xc`,
  `0x1` (`bl` at 0xdf5e8, `bl` at 0xe02f0). Multi-SIC ordering and final r0 remain unestablished
  (`ldmdb` at 0xe0c64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdf3bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdf3c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdf3c4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdf3c8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdf3d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf3d8 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xdf3dc | store | 4 | `sp -0x68` | `[r0+16]` |
| `ldr` at 0xdf3e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf3ec | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf3fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf400 | load | 4 | `abs 0xe0340` | `-` |
| `ldr` at 0xdf404 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xdf410 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdf424 | store | 4 | `sp -0x3c` | `ret(0x1b124)` |
| `ldr` at 0xdf428 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf438 | load | 4 | `abs 0xe0344` | `-` |
| `str` at 0xdf444 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdf458 | store | 4 | `sp -0x40` | `ret(0x1b124)` |
| `ldr` at 0xdf45c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf468 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf46c | load | 4 | `abs 0xe0344` | `-` |
| `ldr` at 0xdf474 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf480 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdf494 | store | 4 | `sp -0x44` | `ret(0x1b124)` |
| `ldr` at 0xdf498 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdf4a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf4a8 | load | 4 | `abs 0xe0344` | `-` |
| `ldr` at 0xdf4b0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf4b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf4c4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdf4d0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdf4d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf4d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf4e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf4fc | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdf500 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf518 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdf524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf530 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf534 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf538 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf540 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdf54c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdf550 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf554 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf564 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf578 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdf57c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf594 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdf5a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf5ac | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf5c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xdf5c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf5cc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdf5e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf5e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf5f0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xdf5f4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xdf608 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf60c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf620 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf628 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf630 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf63c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdf648 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf64c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdf660 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf668 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf670 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf67c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdf684 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xdf68c | store | 4 | `sp -0x94` | `[sp-104]-1` |
| `ldr` at 0xdf690 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdf6a4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdf6a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf6ac | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf6bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf6c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf6d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf6d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf6dc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdf6e8 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdf6f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf700 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdf708 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdf70c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf710 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf724 | load | 4 | `abs 0xe0348` | `-` |
| `ldr` at 0xdf72c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf734 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf73c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdf744 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdf748 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdf74c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf754 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xdf760 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf764 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdf768 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf76c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdf770 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf774 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdf778 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdf77c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdf78c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf7a0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf7a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf7b0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf7bc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdf7c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf7c8 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdf7cc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf7d0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdf7d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf7d8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdf7dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdf7e0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdf7f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf804 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf80c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf814 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf820 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdf82c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdf830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf834 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf844 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf84c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf858 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf85c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf864 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdf870 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdf874 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdf888 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdf88c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdf898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdf8a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf8a8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xdf8b0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xdf8b8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf8c0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdf8c8 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdf8cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf8d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf8ec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf8f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf8fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdf904 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdf908 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdf90c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf910 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdf91c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdf920 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdf924 | store | 4 | `sp -0xb0` | `[sp-36]` |
| `ldr` at 0xdf928 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf92c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdf930 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdf934 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdf938 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdf93c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdf94c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdf960 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdf968 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdf970 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdf97c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdf984 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xdf99c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdf9b0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdf9b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf9b8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdf9c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdf9d0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdf9dc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdf9e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdf9e8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdf9f4 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdfa04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfa0c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfa14 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdfa18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfa1c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfa30 | load | 4 | `abs 0xe0348` | `-` |
| `ldr` at 0xdfa38 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfa40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfa48 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdfa50 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdfa54 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdfa58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdfa5c | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdfa60 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfa64 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdfa68 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfa6c | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdfa70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfa74 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdfa84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfa98 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfaa0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfaa8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfab4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfac0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfac4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfac8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfad8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfae0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfaec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfaf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfaf8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdfb04 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdfb08 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfb1c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdfb20 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdfb2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfb38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdfb3c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xdfb44 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xdfb4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfb54 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfb5c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdfb60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfb64 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfb80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfb88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfb90 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdfb98 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdfb9c | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdfba0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdfba4 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdfba8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfbac | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdfbb0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfbb4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdfbb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfbbc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdfbcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfbe0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfbe8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfbf0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfbfc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdfc04 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xdfc1c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdfc30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfc34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfc38 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfc48 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfc50 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfc5c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfc60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfc68 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdfc74 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdfc84 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfc8c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfc94 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdfc98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfc9c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfcb0 | load | 4 | `abs 0xe0348` | `-` |
| `ldr` at 0xdfcb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfcc0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfcc8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdfcd0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdfcd4 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdfcd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdfcdc | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdfce0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfce4 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdfce8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfcec | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdfcf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfcf4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdfd04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfd18 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfd20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfd28 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfd34 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfd40 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfd44 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfd48 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfd58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfd60 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfd6c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfd70 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfd78 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdfd84 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdfd88 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfd9c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xdfda0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xdfdac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdfdb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdfdbc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xdfdc4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xdfdcc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfdd4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfddc | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xdfde0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfde4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfe00 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfe08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfe10 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdfe18 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdfe1c | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xdfe20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdfe24 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xdfe28 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfe2c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdfe30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfe34 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdfe38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdfe3c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdfe4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdfe60 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdfe68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdfe70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdfe7c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xdfe88 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdfe8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfe90 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfea0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfea8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdfeb4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfeb8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdfec0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdfecc | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xdfed0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdfed4 | store | 4 | `sp -0x2c` | `[sp-148]` |
| `ldr` at 0xdfed8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdfedc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdfef0 | load | 4 | `abs 0xe0348` | `-` |
| `ldr` at 0xdfef8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdff00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdff08 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdff10 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xdff14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdff18 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdff1c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xdff20 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdff24 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xdff28 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdff2c | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xdff30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdff34 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xdff44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdff58 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdff60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdff68 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdff74 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xdff7c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xdff84 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `str` at 0xdff90 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdff94 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdff98 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xdffa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdffc0 | load | 4 | `abs 0xe034c` | `-` |
| `str` at 0xdffc4 | store | 4 | `(unknown)` | `0x800ffe0` |
| `ldr` at 0xdffc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdffd0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdffdc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdffe0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdffe8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdfff4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdfff8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0000 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0014 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0018 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0020 | store | 4 | `sp -0x24` | `ret(0xe0e38)` |
| `ldr` at 0xe0024 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0028 | store | 4 | `sp -0xb0` | `ret(0xe0e38)` |
| `ldr` at 0xe002c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0030 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0034 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0038 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe003c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0040 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0050 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0064 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe006c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0074 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0080 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0088 | load | 4 | `abs 0xe034c` | `-` |
| `str` at 0xe008c | store | 4 | `sp -0x88` | `0x800ffe0` |
| `ldr` at 0xe0090 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0094 | load | 4 | `abs 0xe0354` | `-` |
| `ldr` at 0xe00a0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe00a8 | load | 4 | `abs 0xe0354` | `-` |
| `str` at 0xe00b4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe00b8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe00bc | load | 4 | `abs 0xe0358` | `-` |
| `ldr` at 0xe00c8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe00d0 | load | 4 | `abs 0xe0358` | `-` |
| `str` at 0xe00dc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe00e0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe00e4 | load | 4 | `abs 0xe035c` | `-` |
| `ldr` at 0xe00f0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe00f8 | load | 4 | `abs 0xe035c` | `-` |
| `str` at 0xe0104 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0108 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0118 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xe012c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe0130 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0140 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xe0154 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xe015c | load | 4 | `sp -0x68` | `-` |
| `str` at 0xe0164 | store | 4 | `sp -0x9c` | `?` |
| `str` at 0xe016c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xe0170 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0174 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0184 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe019c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe01a0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe01a8 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0xe01b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe01b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe01cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe01e4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe01ec | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe01f4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xe0200 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0208 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe020c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0214 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe021c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xe0220 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xe0228 | store | 4 | `sp -0x88` | `?` |
| `str` at 0xe0230 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0234 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0238 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0248 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0250 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0258 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0270 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0274 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0284 | load | 4 | `abs 0xe0350` | `-` |
| `ldr` at 0xe028c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0294 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe029c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe02a8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe02b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe02b8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xe02c4 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xe02c8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe02cc | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xe02d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe02d4 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xe02d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe02dc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe02e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0300 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0308 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0310 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe031c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe0328 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe032c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0330 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0368 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe037c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xe0380 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xe038c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0398 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe039c | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xe03a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe03b8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xe03c4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xe03d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe03d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe03dc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe03e8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe03ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe03f4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0400 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0404 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe040c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0420 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0424 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe042c | store | 4 | `sp -0x24` | `ret(0xe0e38)` |
| `ldr` at 0xe0430 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe0434 | store | 4 | `sp -0xb0` | `ret(0xe0e38)` |
| `ldr` at 0xe0438 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe043c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0440 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0444 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0448 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe044c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe045c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0470 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0478 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0480 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe048c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0494 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xe04a8 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xe04ac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe04b0 | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xe04b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe04b8 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xe04bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe04c0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe04c4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe04e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe04ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe04f4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0500 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xe050c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0510 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0514 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0524 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0540 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0544 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe054c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0558 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe055c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0564 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0570 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0574 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe057c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0590 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xe0594 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe059c | store | 4 | `sp -0x24` | `ret(0xe0e38)` |
| `ldr` at 0xe05a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe05a4 | store | 4 | `sp -0xb0` | `ret(0xe0e38)` |
| `ldr` at 0xe05a8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe05ac | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe05b0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe05b4 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe05b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe05bc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe05cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe05e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe05e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe05f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe05fc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0604 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0608 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe060c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0610 | load | 4 | `abs 0xe0354` | `-` |
| `ldr` at 0xe061c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0620 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0628 | load | 4 | `abs 0xe0354` | `-` |
| `str` at 0xe0634 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe0638 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe063c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0640 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0644 | load | 4 | `abs 0xe0358` | `-` |
| `ldr` at 0xe0650 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0654 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe065c | load | 4 | `abs 0xe0358` | `-` |
| `str` at 0xe0668 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe066c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0670 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0674 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0678 | load | 4 | `abs 0xe035c` | `-` |
| `ldr` at 0xe0684 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0688 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe0690 | load | 4 | `abs 0xe035c` | `-` |
| `str` at 0xe069c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe06a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe06b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06bc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe06d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe06d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xe06ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe06f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe0704 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xe0708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe070c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0710 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xe0714 | store | 4 | `[sp-24] +0x0` | `[[sp-64]]` |
| `str` at 0xe071c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe0720 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0734 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0738 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe074c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0754 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe075c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0768 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0770 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0780 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe0790 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe07a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe07ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe07c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe07c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe07d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe07dc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe07e4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe07f4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe080c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0814 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0828 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0830 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0838 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0844 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe084c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xe085c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0864 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe086c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0878 | store | 4 | `sp -0xa0` | `0x1` |
| `str` at 0xe0884 | store | 4 | `sp -0x34` | `0x100` |
| `ldr` at 0xe0888 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe0898 | store | 4 | `sp -0x38` | `0x300` |
| `ldr` at 0xe089c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe08b0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe08bc | store | 4 | `sp -0x28` | `?` |
| `str` at 0xe08c4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe08c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe08cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe08dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe08e4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe08f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe08f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe08fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0908 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0910 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0928 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0934 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0938 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0940 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe094c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe095c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe096c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0974 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0980 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0984 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe098c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0994 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe0998 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe099c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe09a0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe09a4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe09a8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe09ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe09b0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe09c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe09d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe09dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe09e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe09f0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe09fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0a00 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe0a04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0a18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0a1c | load | 4 | `abs 0xe0360` | `-` |
| `ldr` at 0xe0a24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0a2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0a34 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0a40 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0a48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0a54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0a58 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe0a5c | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `str` at 0xe0a64 | store | 4 | `sp -0x34` | `0x40` |
| `ldr` at 0xe0a68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe0a78 | store | 4 | `sp -0x38` | `0xc0` |
| `ldr` at 0xe0a7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0a90 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe0a9c | store | 4 | `sp -0x28` | `?` |
| `str` at 0xe0aa4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xe0aa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0aac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0abc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0ac4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0ad0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0ad4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0adc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0ae8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0af0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b08 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0b14 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe0b18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0b20 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0b2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0b3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe0b54 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0b60 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0b64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe0b6c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xe0b74 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe0b78 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xe0b7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xe0b80 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xe0b84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xe0b88 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xe0b8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0b90 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xe0ba0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0bb4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0bbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0bc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0bd0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0bdc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0be0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe0be4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0bf8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0bfc | load | 4 | `abs 0xe0364` | `-` |
| `ldr` at 0xe0c04 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0c0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0c14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0c20 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xe0c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0c34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0c38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe0c3c | store | 4 | `[sp-28] +0x0` | `[sp-40]` |
| `ldr` at 0xe0c40 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xe0c48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xe0c50 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xe0c5c | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0xe0c60 | load | 4 | `sp -0xa0` | `-` |
| `ldmdb` at 0xe0c64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe0c64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe0c64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdf408 | `(unknown callee) 0x1b49c` | `r3` | `0x1354c8` | `[r0+16]` | `[r0+16]` |
| `bl` at 0xdf41c | `(unknown callee) 0x1b124` | `0x60` | `r1` | `0x10` | `[r0+16]` |
| `bl` at 0xdf43c | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x1353e8` | `?` | `ret(0x1b124)` |
| `bl` at 0xdf450 | `(unknown callee) 0x1b124` | `0x60` | `?` | `?` | `ret(0x1b124)` |
| `bl` at 0xdf470 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x1353e8` | `?` | `ret(0x1b124)` |
| `bl` at 0xdf478 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xdf48c | `(unknown callee) 0x1b124` | `0x60` | `?` | `?` | `ret(0x1b124)` |
| `bl` at 0xdf4ac | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x1353e8` | `?` | `ret(0x1b124)` |
| `bl` at 0xdf4b4 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xdf4bc | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf5e8 | `(unknown callee) 0xe0e38` | `sp-136` | `0xc` | `[sp-104]` | `?` |
| `bl` at 0xdf610 | `(unknown callee) 0xdc9a8` | `0x0` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0xdf624 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdf62c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf634 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf650 | `(unknown callee) 0xdc9a8` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xdf664 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xdf66c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf674 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf6fc | `(unknown callee) 0xe1094` | `0x4` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdf730 | `(unknown callee) 0xe1094` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdf790 | `(unknown callee) 0xdcf54` | `0x0` | `0x0` | `0x1` | `[sp-48]` |
| `bl` at 0xdf7a4 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdf7ac | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf7b4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf7f4 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdf808 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdf810 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf818 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdf8bc | `(unknown callee) 0xe1094` | `[sp-136]` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdf8f0 | `(unknown callee) 0xe1094` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xdf950 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdf964 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdf96c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdf974 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfa08 | `(unknown callee) 0xe1094` | `0xe` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfa3c | `(unknown callee) 0xe1094` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfa88 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdfa9c | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdfaa4 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfaac | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfb50 | `(unknown callee) 0xe1094` | `[sp-136]` | `0x18` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfb84 | `(unknown callee) 0xe1094` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfbd0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdfbe4 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdfbec | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfbf4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfc88 | `(unknown callee) 0xe1094` | `0x9` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfcbc | `(unknown callee) 0xe1094` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfd08 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdfd1c | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdfd24 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfd2c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfdd0 | `(unknown callee) 0xe1094` | `[sp-136]` | `0x8` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfe04 | `(unknown callee) 0xe1094` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xdfe50 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdfe64 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdfe6c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdfe74 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdfefc | `(unknown callee) 0xe1094` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xdff48 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xdff5c | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xdff64 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdff6c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe001c | `(unknown callee) 0xe0e38` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe0054 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0068 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe0070 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0078 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0288 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x135414` | `?` | `?` |
| `bl` at 0xe0290 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0298 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe02a0 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe02f0 | `(unknown callee) 0xe12e8` | `[sp-104]` | `0x8` | `0xc` | `0x1` |
| `bl` at 0xe0304 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xe12e8)` |
| `bl` at 0xe030c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0314 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0428 | `(unknown callee) 0xe0e38` | `sp-136` | `0x8` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe0460 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0474 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe047c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0484 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe04d4 | `(unknown callee) 0xe12e8` | `[sp-104]` | `0x1` | `0xc` | `0x1` |
| `bl` at 0xe04e8 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xe12e8)` |
| `bl` at 0xe04f0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe04f8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0598 | `(unknown callee) 0xe0e38` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xe05d0 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe05e4 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe05ec | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe05f4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe073c | `(unknown callee) 0xdc9a8` | `0x0` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xe0750 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xe0758 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0760 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe07b0 | `(unknown callee) 0xdc9a8` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xe07c4 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdc9a8)` |
| `bl` at 0xe07cc | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe07d4 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0818 | `(unknown callee) 0xdcb88` | `0x0` | `[sp-48]` | `0x0` | `[sp-32]` |
| `bl` at 0xe082c | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcb88)` |
| `bl` at 0xe0834 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe083c | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0860 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0868 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0870 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe09c4 | `(unknown callee) 0xdcf54` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe09d8 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe09e0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe09e8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0a08 | `(unknown callee) 0xe0c68` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xe0a20 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x135470` | `?` | `ret(0xe0c68)` |
| `bl` at 0xe0a28 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0a30 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0a38 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0ba4 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xe0bb8 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `ret(0xdcf54)` |
| `bl` at 0xe0bc0 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0bc8 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0be8 | `(unknown callee) 0xe0c68` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xe0c00 | `(unknown callee) 0x1b49c` | `[sp-32]` | `0x135498` | `?` | `ret(0xe0c68)` |
| `bl` at 0xe0c08 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0c10 | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0c18 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xe0c44 | `(unknown callee) 0x1b400` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xe0c4c | `(unknown callee) 0x1b400` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xe0c54 | `(unknown callee) 0x1b400` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdf3e8 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bhi` at 0xdf3f4 | `[r0+16] > 0x8 (unsigned, 32-bit)` | `[r0+16] <= 0x8 (unsigned, 32-bit)` |
| `bne` at 0xdf430 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xdf464 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xdf4a0 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `blo` at 0xdf4e0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf55c | `C == 0` | `not (C == 0)` |
| `beq` at 0xdf600 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdf61c | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xdf65c | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bge` at 0xdf698 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xdf6b4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf718 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdf75c | `[sp-148] != [sp-104]-1` | `[sp-148] == [sp-104]-1` |
| `bne` at 0xdf79c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xdf800 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xdf83c | `C == 0` | `not (C == 0)` |
| `blo` at 0xdf8d8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdf95c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xdf9a8 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xdf9c0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfa24 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfa94 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xdfad0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfb6c | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfbdc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xdfc28 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xdfc40 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfca4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfd14 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xdfd50 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfdec | `C == 0` | `not (C == 0)` |
| `bne` at 0xdfe5c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xdfe98 | `C == 0` | `not (C == 0)` |
| `blo` at 0xdfee4 | `C == 0` | `not (C == 0)` |
| `bne` at 0xdff54 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xdffa0 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0060 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `blo` at 0xe017c | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0240 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe027c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xe02fc | `ret(0xe12e8) != 0x0` | `ret(0xe12e8) == 0x0` |
| `blo` at 0xe0338 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe046c | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xe04a0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe04e0 | `ret(0xe12e8) != 0x0` | `ret(0xe12e8) == 0x0` |
| `blo` at 0xe051c | `C == 0` | `not (C == 0)` |
| `bne` at 0xe05dc | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xe072c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xe0748 | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xe077c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe078c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe079c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe07bc | `ret(0xdc9a8) != 0x0` | `ret(0xdc9a8) == 0x0` |
| `bne` at 0xe07f0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe0800 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xe0824 | `ret(0xdcb88) != 0x0` | `ret(0xdcb88) == 0x0` |
| `bne` at 0xe0858 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xe08d4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0918 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0964 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe09d0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xe0a14 | `ret(0xe0c68) != 0x0` | `ret(0xe0c68) == 0x0` |
| `beq` at 0xe0a50 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blo` at 0xe0ab4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0af8 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0b44 | `C == 0` | `not (C == 0)` |
| `bne` at 0xe0bb0 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xe0bf4 | `ret(0xe0c68) != 0x0` | `ret(0xe0c68) == 0x0` |
| `beq` at 0xe0c30 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe0c64 | `r0` | `?` |

## `TIO_Issue_CX_Probe`

- `TIO_Issue_CX_Probe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54e58 to 0x55568.
  Established. The entry byte comparison takes its bypass for `(r1 & 0xff) != 0x1` and falls through
  on equality (`bne` at 0x54e84). The call to `0xec394` has unknown, `0x0`, `0x1`, unknown at its
  first site (`bl` at 0x54eac). Later calls to `0xea56c` carry `[sp-20]`, `0xb0000000`, `0x0`,
  `0x0`, then `[sp-20]`, `0xb0008000`, `0x0`, `0x0`, then `[sp-20]`, `0xb0008060`, `0x0`, `0x0`
  (`bl` at 0x553f4, `bl` at 0x55428, `bl` at 0x5545c). The common exit reports `[sp-108]` (`ldmdb`
  at 0x55538). Probe completion semantics, external register meanings and timing units remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54e68 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x54e70 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x54e78 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x54e7c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54e88 | load | 4 | `abs 0x5553c` | `-` |
| `ldr` at 0x54e8c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x54e98 | load | 4 | `abs 0x55540` | `-` |
| `ldr` at 0x54ea0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54eb4 | store | 4 | `sp -0x20` | `ret(0xec394)` |
| `ldr` at 0x54eb8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54ec4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54ec8 | store | 4 | `sp -0x6c` | `ret(0xec394)` |
| `ldr` at 0x54ed0 | load | 4 | `abs 0x5553c` | `-` |
| `ldr` at 0x54ed4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x54ee0 | load | 4 | `abs 0x55544` | `-` |
| `ldr` at 0x54ee8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54efc | store | 4 | `sp -0x20` | `ret(0xec394)` |
| `ldr` at 0x54f00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54f0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54f10 | store | 4 | `sp -0x6c` | `ret(0xec394)` |
| `ldr` at 0x54f20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54f24 | load | 4 | `abs 0x55548` | `-` |
| `str` at 0x54f38 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x54f3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54f48 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54f4c | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `ldr` at 0x54f54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x54f60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x54f64 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x54f70 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x54f7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54f80 | load | 4 | `abs 0x5554c` | `-` |
| `str` at 0x54f94 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x54f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54fa4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54fa8 | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `ldr` at 0x54fb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54fbc | load | 4 | `abs 0x55550` | `-` |
| `str` at 0x54fd0 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x54fd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54fe0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54fe4 | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `ldr` at 0x54ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x54ff8 | load | 4 | `abs 0x55554` | `-` |
| `str` at 0x5500c | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x55010 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5501c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55020 | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `ldr` at 0x55028 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x55030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55034 | load | 4 | `abs 0x5554c` | `-` |
| `ldr` at 0x5503c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x55048 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x5504c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55058 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5505c | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55064 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55068 | load | 4 | `abs 0x55554` | `-` |
| `str` at 0x5507c | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55080 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5508c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55090 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5509c | load | 4 | `abs 0x55550` | `-` |
| `str` at 0x550b0 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x550b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x550c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x550c4 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `str` at 0x550d0 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0x550d4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x550e8 | store | 4 | `sp -0x7c` | `0x0` |
| `str` at 0x550f0 | store | 4 | `sp -0x78` | `0x30000000` |
| `str` at 0x550f8 | store | 4 | `sp -0x74` | `sp-44` |
| `str` at 0x55100 | store | 4 | `sp -0x70` | `sp-48` |
| `ldr` at 0x55104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55108 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5510c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5511c | store | 4 | `sp -0x1c` | `ret(0x63a3c)` |
| `ldr` at 0x55120 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5512c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55130 | load | 4 | `abs 0x5554c` | `-` |
| `ldr` at 0x55134 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x55138 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x55144 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55154 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55158 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55164 | load | 4 | `abs 0x55550` | `-` |
| `ldr` at 0x55168 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x5516c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x55178 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x5517c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55188 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5518c | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55198 | load | 4 | `abs 0x55554` | `-` |
| `ldr` at 0x5519c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x551a0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x551ac | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x551b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x551bc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x551c0 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `str` at 0x551cc | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0x551d4 | load | 4 | `abs 0x5553c` | `-` |
| `ldr` at 0x551d8 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x551e4 | load | 4 | `abs 0x55540` | `-` |
| `ldr` at 0x551ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55200 | store | 4 | `sp -0x20` | `ret(0xec394)` |
| `ldr` at 0x55204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55210 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55214 | store | 4 | `sp -0x6c` | `ret(0xec394)` |
| `ldr` at 0x5521c | load | 4 | `abs 0x5553c` | `-` |
| `ldr` at 0x55220 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x5522c | load | 4 | `abs 0x55544` | `-` |
| `ldr` at 0x55234 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55248 | store | 4 | `sp -0x20` | `ret(0xec394)` |
| `ldr` at 0x5524c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55258 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5525c | store | 4 | `sp -0x6c` | `ret(0xec394)` |
| `ldr` at 0x55264 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55268 | load | 4 | `abs 0x55558` | `-` |
| `str` at 0x5527c | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55280 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5528c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55290 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5529c | load | 4 | `abs 0x55558` | `-` |
| `str` at 0x552b0 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x552b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x552c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x552c4 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x552d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x552d8 | load | 4 | `abs 0x5555c` | `-` |
| `str` at 0x552ec | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x552f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x552fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55300 | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `str` at 0x5530c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x55310 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55324 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x5532c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x5533c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x55354 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x55368 | store | 4 | `sp -0x20` | `ret(0xea468)` |
| `ldr` at 0x5536c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55378 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5537c | store | 4 | `sp -0x6c` | `ret(0xea468)` |
| `ldr` at 0x55384 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x5538c | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x55390 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x553a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x553a8 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0x553ac | load | 4 | `sp -0x60` | `-` |
| `str` at 0x553b8 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x553bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x553c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x553cc | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x553d4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x553dc | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x553e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x553fc | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55400 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5540c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55410 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5541c | load | 4 | `abs 0x55560` | `-` |
| `str` at 0x55430 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55434 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55440 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55444 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x5544c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55450 | load | 4 | `abs 0x55564` | `-` |
| `str` at 0x55464 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55468 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55474 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55478 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x55480 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x55488 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0x55490 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x55494 | load | 4 | `abs 0x5554c` | `-` |
| `ldr` at 0x55498 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5549c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x554a8 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x554ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x554b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x554bc | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x554c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x554c8 | load | 4 | `abs 0x55550` | `-` |
| `ldr` at 0x554cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x554d0 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x554dc | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x554e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x554ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x554f0 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x554f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x554fc | load | 4 | `abs 0x55554` | `-` |
| `ldr` at 0x55500 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x55504 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x55510 | store | 4 | `sp -0x20` | `ret(0xea56c)` |
| `ldr` at 0x55514 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x55520 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x55524 | store | 4 | `sp -0x6c` | `ret(0xea56c)` |
| `ldr` at 0x5552c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x55530 | store | 4 | `sp -0x6c` | `[sp-28]` |
| `ldr` at 0x55534 | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0x55538 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x55538 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x55538 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54e9c | `(unknown callee) 0x23f2c` | `0x10b4f4` | `r1` | `r2` | `[0x2ad25c]` |
| `bl` at 0x54eac | `(unknown callee) 0xec394` | `?` | `0x0` | `0x1` | `?` |
| `bl` at 0x54ee4 | `(unknown callee) 0x23f2c` | `0x10b514` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x54ef4 | `(unknown callee) 0xec394` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x54f30 | `(unknown callee) 0xea468` | `?` | `0x90060e80` | `sp-44` | `sp-48` |
| `bl` at 0x54f8c | `(unknown callee) 0xea468` | `[sp-20]` | `0xb0000028` | `sp-52` | `sp-56` |
| `bl` at 0x54fc8 | `(unknown callee) 0xea468` | `[sp-20]` | `0xb0000100` | `sp-68` | `sp-72` |
| `bl` at 0x55004 | `(unknown callee) 0xea468` | `[sp-20]` | `0xb0000108` | `sp-60` | `sp-64` |
| `bl` at 0x55040 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000028` | `?` | `[sp-56]` |
| `bl` at 0x55074 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000108` | `0x0` | `0x0` |
| `bl` at 0x550a8 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000100` | `0x0` | `0xc8` |
| `bl` at 0x55114 | `(unknown callee) 0x63a3c` | `[sp-20]` | `[sp-36]` | `[sp-40]` | `0xff` |
| `bl` at 0x5513c | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x55170 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x551a4 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |
| `bl` at 0x551e8 | `(unknown callee) 0x23f2c` | `0x10b4f4` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x551f8 | `(unknown callee) 0xec394` | `[sp-20]` | `0x0` | `0x1` | `?` |
| `bl` at 0x55230 | `(unknown callee) 0x23f2c` | `0x10b514` | `?` | `?` | `[0x2ad25c]` |
| `bl` at 0x55240 | `(unknown callee) 0xec394` | `[sp-20]` | `0x0` | `0x2` | `?` |
| `bl` at 0x55274 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb000b100` | `0x0` | `0x8` |
| `bl` at 0x552a8 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb000b100` | `0x0` | `0x0` |
| `bl` at 0x552e4 | `(unknown callee) 0xea468` | `[sp-20]` | `0xb0008118` | `sp-84` | `sp-88` |
| `bl` at 0x55360 | `(unknown callee) 0xea468` | `[sp-20]` | `?` | `sp-92` | `sp-96` |
| `bl` at 0x553b0 | `(unknown callee) 0xea56c` | `[sp-20]` | `?` | `[sp-92]` | `[sp-96]` |
| `bl` at 0x553f4 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000000` | `0x0` | `0x0` |
| `bl` at 0x55428 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0008000` | `0x0` | `0x0` |
| `bl` at 0x5545c | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0008060` | `0x0` | `0x0` |
| `bl` at 0x554a0 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000028` | `[sp-52]` | `[sp-56]` |
| `bl` at 0x554d4 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000100` | `[sp-68]` | `[sp-72]` |
| `bl` at 0x55508 | `(unknown callee) 0xea56c` | `[sp-20]` | `0xb0000108` | `[sp-60]` | `[sp-64]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x54e84 | `(r1 & 0xff) != 0x1` | `(r1 & 0xff) == 0x1` |
| `bls` at 0x54e94 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x54ec0 | `ret(0xec394) >= 0x0 (signed, 32-bit)` | `ret(0xec394) < 0x0 (signed, 32-bit)` |
| `bls` at 0x54edc | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x54f08 | `ret(0xec394) == 0x0` | `ret(0xec394) != 0x0` |
| `beq` at 0x54f44 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x54fa0 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x54fdc | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x55018 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x55054 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x55088 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x550bc | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `bls` at 0x550dc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x55128 | `ret(0x63a3c) != 0x0` | `ret(0x63a3c) == 0x0` |
| `beq` at 0x55150 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x55184 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x551b8 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `bls` at 0x551e0 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x5520c | `ret(0xec394) >= 0x0 (signed, 32-bit)` | `ret(0xec394) < 0x0 (signed, 32-bit)` |
| `bls` at 0x55228 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x55254 | `ret(0xec394) == 0x0` | `ret(0xec394) != 0x0` |
| `beq` at 0x55288 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x552bc | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x552f8 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `bls` at 0x55318 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x55338 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x55374 | `ret(0xea468) == 0x0` | `ret(0xea468) != 0x0` |
| `beq` at 0x553c4 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x55408 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x5543c | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x55470 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x554b4 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x554e8 | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |
| `beq` at 0x5551c | `ret(0xea56c) == 0x0` | `ret(0xea56c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x55538 | `r0` | `[sp-108]` |

## `build_sic_tdi`

- `build_sic_tdi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe0e38 to 0xe1094.
  Established. A word store clears entry `r3 +0x0` (`str` at 0xe0ebc). Two later word stores have
  unknown effective targets and unknown values (`str` at 0xe0ff8, `str` at 0xe105c). The first loop
  branch retains only `C == 0` for taken and its negation for fall-through (`blo` at 0xe0e74). No
  calls appear in the facts. Packed scan-data format and r0 at exit remain unresolved (`ldmdb` at
  0xe1084).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe0e48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe0e4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe0e50 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe0e54 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe0e5c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe0e64 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe0e68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0e6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe0e80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0e88 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe0e90 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe0e94 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe0e9c | store | 4 | `sp -0x34` | `?` |
| `str` at 0xe0ea8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xe0eb0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe0eb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe0ebc | store | 4 | `r3 +0x0` | `0x0` |
| `str` at 0xe0ec4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe0ec8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0ecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe0edc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe0ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe0eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe0ef0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe0ef8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0efc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f00 | load | 4 | `abs 0xe1088` | `-` |
| `ldr` at 0xe0f0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f14 | load | 4 | `abs 0xe1088` | `-` |
| `str` at 0xe0f20 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0f24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f28 | load | 4 | `abs 0xe108c` | `-` |
| `ldr` at 0xe0f34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f3c | load | 4 | `abs 0xe108c` | `-` |
| `str` at 0xe0f48 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0f4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f50 | load | 4 | `abs 0xe1090` | `-` |
| `ldr` at 0xe0f5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f64 | load | 4 | `abs 0xe1090` | `-` |
| `str` at 0xe0f70 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0f74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0f84 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe0f98 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0f9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0fac | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe0fc0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe0fc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0fcc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0fd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe0fdc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe0fe4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe0fe8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe0ff0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe0ff8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe0ffc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1000 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe1008 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])` |
| `ldr` at 0xe100c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1018 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1020 | store | 4 | `sp -0x24` | `([sp-36] + [sp-24])-32` |
| `ldr` at 0xe1024 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe1028 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1030 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe1034 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe103c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xe1040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1048 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe1050 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1054 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe105c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1060 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe1068 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0xe1070 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1078 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xe1084 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe1084 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe1084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe0e74 | `C == 0` | `not (C == 0)` |
| `blo` at 0xe0ed4 | `C == 0` | `not (C == 0)` |
| `bls` at 0xe1014 | `([sp-36] + [sp-24]) <= 0x1f (unsigned, 32-bit)` | `([sp-36] + [sp-24]) > 0x1f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe1084 | `r0` | `?` |

## `add_sic_tdi`

- `add_sic_tdi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe1094 to 0xe12e8.
  Established. Two word stores have unknown effective targets and unknown values (`str` at 0xe124c,
  `str` at 0xe12b0). A later word store writes `([[sp-32]] + [sp-24])` at `[sp-32] +0x0` (`str` at
  0xe12c8). The opening loop branch takes `C == 0` and falls through on its negation (`blo` at
  0xe10d0). The exit carries `[[sp-32]]` (`ldmdb` at 0xe12d8). Bit-order and length units are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe10a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe10a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe10ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe10b0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe10b8 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xe10c0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe10c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe10c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe10dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe10e4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe10ec | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe10f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe10f8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe1100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1104 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe110c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1110 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1114 | load | 4 | `abs 0xe12dc` | `-` |
| `ldr` at 0xe1120 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1128 | load | 4 | `abs 0xe12dc` | `-` |
| `str` at 0xe1134 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1138 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe113c | load | 4 | `abs 0xe12e0` | `-` |
| `ldr` at 0xe1148 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1150 | load | 4 | `abs 0xe12e0` | `-` |
| `str` at 0xe115c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1160 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1164 | load | 4 | `abs 0xe12e4` | `-` |
| `ldr` at 0xe1170 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1178 | load | 4 | `abs 0xe12e4` | `-` |
| `str` at 0xe1184 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe1188 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe1198 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe11ac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe11b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe11c0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe11d4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe11d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe11dc | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xe11e0 | store | 4 | `sp -0x28` | `[r3]` |
| `str` at 0xe11e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe11ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe11fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1204 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe1208 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1210 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe1218 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1228 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe1238 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe123c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1244 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe124c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe1254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe125c | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])` |
| `ldr` at 0xe1260 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe126c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1274 | store | 4 | `sp -0x28` | `([sp-40] + [sp-24])-32` |
| `ldr` at 0xe1278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe127c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1284 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe1288 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1290 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe1294 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe129c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe12a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xe12a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe12b0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe12b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe12b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe12bc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xe12c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe12c8 | store | 4 | `[sp-32] +0x0` | `([[sp-32]] + [sp-24])` |
| `ldr` at 0xe12cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe12d0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldmdb` at 0xe12d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe12d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe12d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe10d0 | `C == 0` | `not (C == 0)` |
| `bhi` at 0xe11f4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0xe1268 | `([sp-40] + [sp-24]) <= 0x20 (unsigned, 32-bit)` | `([sp-40] + [sp-24]) > 0x20 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe12d8 | `r0` | `[[sp-32]]` |

## `memory_access_function`

- `memory_access_function`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe4d9c to 0xe5064.
  Established. A word store clears `[sp-20] +0xc` (`str` at 0xe502c). The entry test has unknown
  operand, takes its branch for nonzero and falls through on zero (`bne` at 0xe4dc8). The unsigned
  global comparison takes its bypass for `[0x2ad25c] <= 0x1` and falls through for greater (`bls` at
  0xe4e40). The call to `0xeb47c` has unknown, unknown, `0x4`, unknown (`bl` at 0xe4eb8).
  Memory-operation layout, physical target and final r0 remain unidentified (`ldmdb` at 0xe503c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe4dac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe4db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4dbc | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe4dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4dd8 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xe4de0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe4de4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe4dec | store | 4 | `sp -0x38` | `[sp-56]-2` |
| `ldr` at 0xe4df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e00 | load | 4 | `r0 +0xc` | `-` |
| `str` at 0xe4e08 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe4e0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe4e14 | store | 4 | `sp -0x38` | `[sp-56]-1` |
| `ldr` at 0xe4e18 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe4e1c | store | 4 | `sp -0x20` | `[sp-56]` |
| `ldr` at 0xe4e20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e24 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xe4e30 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe4e34 | load | 4 | `abs 0xe5040` | `-` |
| `ldr` at 0xe4e38 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe4e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e50 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe4e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e68 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe4e74 | load | 4 | `abs 0xe5044` | `-` |
| `ldr` at 0xe4e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4e8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe4e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4ea4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe4ec0 | store | 4 | `sp -0x18` | `ret(0xeb47c)` |
| `ldr` at 0xe4ec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4ed0 | load | 4 | `abs 0xe5048` | `-` |
| `str` at 0xe4edc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe4ee4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe4ef0 | load | 4 | `abs 0xe504c` | `-` |
| `str` at 0xe4efc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe4f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4f10 | load | 4 | `abs 0xe5050` | `-` |
| `str` at 0xe4f1c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe4f24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe4f34 | load | 4 | `abs 0xe5054` | `-` |
| `str` at 0xe4f40 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe4f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4f54 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xe4f58 | store | 4 | `sp -0x28` | `[[sp-20]+16]` |
| `ldr` at 0xe4f60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe4f68 | load | 4 | `[sp-20] +0x14` | `-` |
| `str` at 0xe4f6c | store | 4 | `sp -0x2c` | `[[sp-20]+20]` |
| `ldr` at 0xe4f70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe4f78 | store | 4 | `sp -0x30` | `[sp-20]+24` |
| `ldr` at 0xe4f7c | load | 4 | `abs 0xe5040` | `-` |
| `ldr` at 0xe4f80 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe4f8c | load | 4 | `abs 0xe5058` | `-` |
| `ldr` at 0xe4f90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe4f94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe4f98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4fa4 | store | 4 | `sp -0x4c` | `0xff` |
| `ldr` at 0xe4fa8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe4fac | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe4fb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe4fb4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe4fb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe4fcc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe4fd0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe4fd4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe4fdc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe4ff0 | store | 4 | `sp -0x1c` | `ret(0x610e8)` |
| `ldr` at 0xe4ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5000 | load | 4 | `abs 0xe505c` | `-` |
| `ldr` at 0xe5004 | load | 4 | `abs 0xe5060` | `-` |
| `ldr` at 0xe5008 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5014 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xe5020 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe502c | store | 4 | `[sp-20] +0xc` | `0x0` |
| `str` at 0xe5034 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xe5038 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xe503c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe503c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe503c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe4e7c | `(unknown callee) 0x23f2c` | `0x1357e8` | `?` | `?` | `?` |
| `bl` at 0xe4eb8 | `(unknown callee) 0xeb47c` | `?` | `?` | `0x4` | `?` |
| `bl` at 0xe4ed4 | `(unknown callee) 0x2070c` | `0x135810` | `?` | `?` | `ret(0xeb47c)` |
| `bl` at 0xe4ef4 | `(unknown callee) 0x2070c` | `0x13583c` | `?` | `?` | `[sp-36]` |
| `bl` at 0xe4f14 | `(unknown callee) 0x2070c` | `0x135854` | `?` | `?` | `[sp-32]` |
| `bl` at 0xe4f38 | `(unknown callee) 0x2070c` | `0x135870` | `?` | `?` | `?` |
| `bl` at 0xe4f9c | `(unknown callee) 0x23f2c` | `0x135890` | `[sp-24]` | `[[sp-20]+16]` | `[[sp-20]+20]` |
| `bl` at 0xe4fe8 | `(unknown callee) 0x610e8` | `0x1` | `[sp-24]` | `?` | `?` |
| `bl` at 0xe500c | `(unknown callee) 0x1b49c` | `0x2b37a8` | `0x1358c0` | `ret(0x610e8)` | `ret(0x610e8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe4dc8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xe4e40 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xe4ecc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xe4eec | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0xe4f0c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xe4f30 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe4f88 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0xe4ffc | `ret(0x610e8) == 0x0` | `ret(0x610e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe503c | `r0` | `?` |
