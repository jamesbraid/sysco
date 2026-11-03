# ARM boot and board configuration in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/boot-board-configuration-facts.txt) retain all tool sections.

## `bootAddressIsValid`

- `bootAddressIsValid`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc55e4 to 0xc5700.
  Established. The first branch has an unknown operand (`beq` at 0xc5604). A later word read uses
  absolute `0x2b36a0` (`ldr` at 0xc561c). The recorded return expression is unknown (`ldmdb` at
  0xc56f8). Accepted boot-address ranges cannot be established from these supported facts alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc55f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc55f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc560c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc5614 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc5618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc561c | load | 4 | `abs 0x2b36a0` | `-` |
| `ldr` at 0xc5628 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc562c | load | 4 | `abs 0x2b36a0` | `-` |
| `ldr` at 0xc5634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5644 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc564c | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc5650 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xc565c | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc5660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5664 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xc5670 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc5674 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc5678 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xc567c | load | 4 | `abs 0x2b36b8` | `-` |
| `ldr` at 0xc5684 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5694 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc569c | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc56a0 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xc56ac | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc56b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc56b4 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xc56c0 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc56c4 | load | 4 | `abs 0xc56fc` | `-` |
| `ldr` at 0xc56c8 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xc56cc | load | 4 | `abs 0x2b36b8` | `-` |
| `ldr` at 0xc56d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc56e4 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xc56f0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc56f4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc56f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc56f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc56f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5604 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xc5624 | `r0 < [0x2b36a0] (unsigned, 32-bit)` | `r0 >= [0x2b36a0] (unsigned, 32-bit)` |
| `bhs` at 0xc563c | `r0 >= [0x2b36a0]+524288 (unsigned, 32-bit)` | `r0 < [0x2b36a0]+524288 (unsigned, 32-bit)` |
| `beq` at 0xc5658 | `[0x2b36a8] == 0x0` | `[0x2b36a8] != 0x0` |
| `blo` at 0xc566c | `r0 < [0x2b36a8] (unsigned, 32-bit)` | `r0 >= [0x2b36a8] (unsigned, 32-bit)` |
| `bhs` at 0xc568c | `r0 >= ([0x2b36a8] + [0x2b36b8]) (unsigned, 32-bit)` | `r0 < ([0x2b36a8] + [0x2b36b8]) (unsigned, 32-bit)` |
| `beq` at 0xc56a8 | `[0x2b36ac] == 0x0` | `[0x2b36ac] != 0x0` |
| `blo` at 0xc56bc | `r0 < [0x2b36ac] (unsigned, 32-bit)` | `r0 >= [0x2b36ac] (unsigned, 32-bit)` |
| `bhs` at 0xc56dc | `r0 >= ([0x2b36ac] + [0x2b36b8]) (unsigned, 32-bit)` | `r0 < ([0x2b36ac] + [0x2b36b8]) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc56f8 | `r0` | `?` |

## `bootAddressNormalize`

- `bootAddressNormalize`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5700 to 0xc5764.
  Established. The first branch takes `[sp-28] == 0x1` and falls through on inequality (`beq` at
  0xc572c). The recorded r0 exit is unknown (`ldmdb` at 0xc5760). Address-normalisation semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5710 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5714 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc5718 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5720 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc5724 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5730 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5748 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5754 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc5758 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5760 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5760 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc572c | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `beq` at 0xc5738 | `[sp-28] == 0x2` | `[sp-28] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5760 | `r0` | `?` |

## `bootValidate`

- `bootValidate`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5764 to 0xc58b8.
  Established. The entry-r1 guard takes nonzero and falls through on zero (`bne` at 0xc5784). The
  call to `0xc59a0` has entry r0, unknown, entry r2 and unknown as candidates (`bl` at 0xc5798). The return
  carries `[sp-100]` (`ldmdb` at 0xc58b4). The image-validation result encoding is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5774 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5778 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc577c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc578c | store | 4 | `sp -0x18` | `sp-92` |
| `ldr` at 0xc5790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5794 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc57a0 | store | 4 | `sp -0x28` | `ret(0xc59a0)` |
| `ldr` at 0xc57a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc57b4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xc57bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc57c8 | store | 4 | `sp -0x2c` | `ret(0xc58b8)` |
| `ldr` at 0xc57cc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc57d0 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xc57dc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc57e0 | load | 2 | `[sp-40] +0x2` | `-` |
| `str` at 0xc57f4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xc57fc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc5800 | load | 2 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xc580c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc5814 | store | 4 | `sp -0x20` | `ret(0xc58b8)+4` |
| `ldr` at 0xc581c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc5820 | store | 4 | `sp -0x20` | `ret(0xc58b8)` |
| `ldr` at 0xc5824 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc5828 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xc582c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5834 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc583c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc5840 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5844 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5858 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc585c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc5864 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc5878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5880 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1af68))` |
| `ldr` at 0xc5888 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5898 | store | 4 | `sp -0x68` | `?` |
| `str` at 0xc58a4 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xc58a8 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xc58ac | store | 4 | `sp -0x64` | `[sp-104]` |
| `ldr` at 0xc58b0 | load | 4 | `sp -0x64` | `-` |
| `ldmdb` at 0xc58b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc58b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc58b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5798 | `(unknown callee) 0xc59a0` | `r0` | `?` | `r2` | `?` |
| `bl` at 0xc57c0 | `(unknown callee) 0xc58b8` | `[sp-20]` | `?` | `?` | `ret(0xc59a0)` |
| `bl` at 0xc586c | `(unknown callee) 0x1af68` | `?` | `?` | `sp-32` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc5784 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xc57ac | `ret(0xc59a0) != 0x0` | `ret(0xc59a0) == 0x0` |
| `bls` at 0xc57d8 | `[[sp-40]+2]:2 <= 0x1f (unsigned, 32-bit)` | `[[sp-40]+2]:2 > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xc57e8 | `[[sp-40]+2]:2 > 0x400 (unsigned, 32-bit)` | `[[sp-40]+2]:2 <= 0x400 (unsigned, 32-bit)` |
| `bhi` at 0xc5808 | `[[sp-40]]:2 > 0x2 (unsigned, 32-bit)` | `[[sp-40]]:2 <= 0x2 (unsigned, 32-bit)` |
| `blo` at 0xc584c | `C == 0` | `not (C == 0)` |
| `beq` at 0xc5890 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc58b4 | `r0` | `[sp-100]` |

## `bootGetHeaderPtr`

- `bootGetHeaderPtr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc58b8 to 0xc59a0.
  Established. The opening call to `0xc55e4` carries entry r0 through r3 as candidates (`bl` at
  0xc58d0); its nonzero result takes the branch and zero falls through (`bne` at 0xc58dc). The
  common r0 expression is unknown (`ldmdb` at 0xc599c). The returned header location remains
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc58c8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc58cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc58e4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc58ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc58f8 | store | 4 | `sp -0x24` | `ret(0xcb21c)` |
| `ldr` at 0xc58fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc590c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc5914 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5918 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5928 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5940 | store | 4 | `sp -0x20` | `ret(0xc5700)` |
| `ldr` at 0xc5944 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc595c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc5964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc5978 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5980 | store | 4 | `sp -0x30` | `ret(0xcb21c)` |
| `str` at 0xc598c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc5990 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc5994 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc5998 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xc599c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc599c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc599c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc599c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc58d0 | `(unknown callee) 0xc55e4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc58f0 | `(unknown callee) 0xcb21c` | `[sp-24]` | `?` | `?` | `ret(0xc55e4)` |
| `bl` at 0xc591c | `(unknown callee) 0x1af68` | `[[sp-36]]` | `?` | `?` | `[sp-36]` |
| `bl` at 0xc592c | `(unknown callee) 0xcb0f8` | `[sp-24]` | `?` | `?` | `ret(0x1af68)` |
| `bl` at 0xc593c | `(unknown callee) 0xc5700` | `ret(0x1af68)` | `ret(0xcb0f8)` | `?` | `ret(0xcb0f8)` |
| `bl` at 0xc5948 | `(unknown callee) 0xc55e4` | `ret(0xc5700)` | `?` | `?` | `?` |
| `bl` at 0xc5968 | `(unknown callee) 0xcb21c` | `[sp-32]` | `?` | `?` | `ret(0xc55e4)` |
| `bl` at 0xc597c | `(unknown callee) 0xcb21c` | `[sp-32]` | `?` | `?` | `ret(0xcb21c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc58dc | `ret(0xc55e4) != 0x0` | `ret(0xc55e4) == 0x0` |
| `bne` at 0xc5904 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc5954 | `ret(0xc55e4) != 0x0` | `ret(0xc55e4) == 0x0` |
| `beq` at 0xc5974 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc599c | `r0` | `?` |

## `bootGetHeader`

- `bootGetHeader`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc59a0 to 0xc5b50.
  Established. The first call targets `0xcb0f8` with candidates entry r0, entry r1, entry r2 and `0x0` (`bl` at
  0xc59c4). The guard takes `ret(0xc58b8) != 0x0` and falls through on zero (`bne` at 0xc59e8). The
  final r0 expression and header-format interpretation remain unknown (`ldmdb` at 0xc5b4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc59b0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc59b4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc59bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc59c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc59cc | store | 4 | `sp -0x28` | `ret(0xcb0f8)` |
| `ldr` at 0xc59d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc59dc | store | 4 | `sp -0x20` | `ret(0xc58b8)` |
| `ldr` at 0xc59e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc59f0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc59f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc59fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5a0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc5a10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5a20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc5a40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5a48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5a50 | store | 4 | `(unknown)` | `ret(0x1af68)` |
| `ldr` at 0xc5a54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5a5c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc5a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a6c | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xc5a70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5a84 | store | 4 | `[sp-28] +0x10` | `ret(0xcb21c)` |
| `ldr` at 0xc5a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5a90 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xc5a94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5aa8 | store | 4 | `[sp-28] +0x14` | `ret(0xcb21c)` |
| `ldr` at 0xc5aac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5ab4 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xc5ab8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5acc | store | 4 | `[sp-28] +0x18` | `ret(0xcb21c)` |
| `ldr` at 0xc5ad0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5ad8 | load | 4 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc5adc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5af0 | store | 4 | `[sp-28] +0x24` | `ret(0xcb21c)` |
| `ldr` at 0xc5af4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5af8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5afc | load | 4 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc5b00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5b14 | store | 4 | `[sp-28] +0x2c` | `ret(0xcb21c)` |
| `ldr` at 0xc5b18 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc5b1c | load | 2 | `[sp-28] +0x2` | `-` |
| `strh` at 0xc5b20 | store | 2 | `sp -0x2a` | `([[sp-28]+2]:2 & 0xffff)` |
| `ldr` at 0xc5b24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5b28 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc5b2c | load | 2 | `[sp-28] +0x0` | `-` |
| `strh` at 0xc5b30 | store | 2 | `[sp-28] +0x2` | `([[sp-28]]:2 & 0xffff)` |
| `ldr` at 0xc5b34 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc5b38 | load | 2 | `sp -0x2a` | `-` |
| `strh` at 0xc5b3c | store | 2 | `[sp-28] +0x0` | `([sp-42]:2 & 0xffff)` |
| `ldr` at 0xc5b40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5b44 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0xc5b48 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xc5b4c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc5b4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5b4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5b4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc59c4 | `(unknown callee) 0xcb0f8` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xc59d4 | `(unknown callee) 0xc58b8` | `[sp-24]` | `?` | `?` | `ret(0xcb0f8)` |
| `bl` at 0xc5a04 | `(unknown callee) 0x1b304` | `[sp-28]` | `ret(0xc58b8)` | `0x30` | `ret(0xc58b8)` |
| `bl` at 0xc5a34 | `(unknown callee) 0x1af68` | `?` | `?` | `?` | `?` |
| `bl` at 0xc5a74 | `(unknown callee) 0xc5700` | `[[sp-28]+16]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc5a80 | `(unknown callee) 0xcb21c` | `ret(0xc5700)` | `?` | `?` | `ret(0xc5700)` |
| `bl` at 0xc5a98 | `(unknown callee) 0xc5700` | `[[sp-28]+20]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc5aa4 | `(unknown callee) 0xcb21c` | `ret(0xc5700)` | `?` | `?` | `ret(0xc5700)` |
| `bl` at 0xc5abc | `(unknown callee) 0xc5700` | `[[sp-28]+24]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc5ac8 | `(unknown callee) 0xcb21c` | `ret(0xc5700)` | `?` | `?` | `ret(0xc5700)` |
| `bl` at 0xc5ae0 | `(unknown callee) 0xc5700` | `[[sp-28]+36]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc5aec | `(unknown callee) 0xcb21c` | `ret(0xc5700)` | `?` | `?` | `ret(0xc5700)` |
| `bl` at 0xc5b04 | `(unknown callee) 0xc5700` | `[[sp-28]+44]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc5b10 | `(unknown callee) 0xcb21c` | `ret(0xc5700)` | `?` | `?` | `ret(0xc5700)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc59e8 | `ret(0xc58b8) != 0x0` | `ret(0xc58b8) == 0x0` |
| `bls` at 0xc5a18 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5b4c | `r0` | `?` |

## `bootDefaultImage`

- `bootDefaultImage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5b50 to 0xc5d88.
  Established. Zero entry r0 takes the first branch and nonzero falls through (`beq` at 0xc5b84). A
  word read accesses absolute `0x2b36bc` (`ldr` at 0xc5b98). The call to `0xc5764` has candidates
  `[sp-96]`, `sp-88`, unknown and `sp-88` (`bl` at 0xc5c44). Default-image selection and the final
  value remain unknown (`ldmdb` at 0xc5d80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5b60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5b68 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xc5b70 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xc5b78 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc5b7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5b88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5b90 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xc5b94 | load | 4 | `abs 0xc5d84` | `-` |
| `ldr` at 0xc5b98 | load | 4 | `abs 0x2b36bc` | `-` |
| `str` at 0xc5b9c | store | 4 | `sp -0x24` | `[0x2b36bc]` |
| `ldr` at 0xc5ba0 | load | 4 | `abs 0xc5d84` | `-` |
| `ldr` at 0xc5ba4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5ba8 | load | 4 | `abs 0x2b36a4` | `-` |
| `ldr` at 0xc5bb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5bb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc5bc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5bd0 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0xc5bd8 | load | 4 | `abs 0xc5d84` | `-` |
| `ldr` at 0xc5bdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5be0 | load | 4 | `abs 0x2b36a4` | `-` |
| `str` at 0xc5bf0 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xc5bf8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5c00 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc5c04 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc5c08 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5c0c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc5c10 | load | 4 | `abs 0xc5d84` | `-` |
| `ldr` at 0xc5c14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5c18 | load | 4 | `abs 0x2b36bc` | `-` |
| `ldr` at 0xc5c28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5c2c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc5c34 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xc5c3c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xc5c58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5c5c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5c6c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc5c70 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xc5c74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5c78 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xc5c80 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc5c88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5c94 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc5c98 | store | 4 | `sp -0x64` | `[sp-72]` |
| `ldr` at 0xc5c9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5ca0 | store | 4 | `sp -0x68` | `[sp-32]` |
| `ldr` at 0xc5ca4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc5cb0 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0xc5cb4 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldr` at 0xc5cc0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc5cc4 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xc5cd4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc5cd8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc5cdc | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xc5ce0 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xc5cf0 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc5cf4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc5cf8 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xc5cfc | load | 1 | `[sp-104] +0x0` | `-` |
| `str` at 0xc5d0c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc5d14 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc5d1c | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0xc5d20 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xc5d28 | store | 4 | `sp -0x68` | `[sp-104]+1` |
| `ldr` at 0xc5d30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5d3c | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc5d40 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xc5d44 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc5d48 | store | 4 | `sp -0x20` | `[sp-72]` |
| `ldr` at 0xc5d4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc5d54 | store | 4 | `sp -0x24` | `[sp-36]-4` |
| `ldr` at 0xc5d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5d6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5d70 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc5d74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc5d78 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xc5d7c | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0xc5d80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5d80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5d80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5c44 | `(unknown callee) 0xc5764` | `[sp-96]` | `sp-88` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5b84 | `r0 == 0x0` | `r0 != 0x0` |
| `bhs` at 0xc5bb0 | `C == 1` | `not (C == 1)` |
| `bne` at 0xc5bc0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc5be8 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xc5c20 | `C == 1` | `not (C == 1)` |
| `beq` at 0xc5c50 | `ret(0xc5764) == 0x0` | `ret(0xc5764) != 0x0` |
| `beq` at 0xc5c68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc5c90 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xc5cac | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc5cbc | `[[sp-100]]:1 == 0x0` | `[[sp-100]]:1 != 0x0` |
| `bne` at 0xc5ccc | `[[sp-104]]:1 != 0x0` | `[[sp-104]]:1 == 0x0` |
| `bhs` at 0xc5ce8 | `[[sp-100]]:1 >= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 < [[sp-104]]:1 (unsigned, 32-bit)` |
| `bls` at 0xc5d04 | `[[sp-100]]:1 <= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 > [[sp-104]]:1 (unsigned, 32-bit)` |
| `bne` at 0xc5d38 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xc5d64 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5d80 | `r0` | `?` |

## `IP57_Cfg_Init`

- `IP57_Cfg_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5eed8 to 0x5f0e4.
  Established. The global byte load reads `0x148420` (`ldrb` at 0x5eefc). Zero takes the branch and
  nonzero falls through (`beq` at 0x5ef04). The opening call to `0x21e28` has `0x9`, `0x10c53c`,
  `0x91` and `0x0` as candidates (`bl` at 0x5ef20). The recorded exit carries `ret(0x5f0e4)`
  (`ldmdb` at 0x5f0b8). The board-configuration contract remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5eee8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5eeec | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5eef0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5eef4 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5eef8 | load | 4 | `abs 0x5f0bc` | `-` |
| `ldrb` at 0x5eefc | load | 1 | `abs 0x148420` | `-` |
| `ldr` at 0x5ef08 | load | 4 | `abs 0x5f0c0` | `-` |
| `str` at 0x5ef0c | store | 4 | `sp -0x44` | `0x10c520` |
| `ldr` at 0x5ef14 | load | 4 | `abs 0x5f0c4` | `-` |
| `ldr` at 0x5ef24 | load | 4 | `abs 0x5f0bc` | `-` |
| `strb` at 0x5ef2c | store | 1 | `abs 0x148420` | `0x1` |
| `ldr` at 0x5ef30 | load | 4 | `abs 0x5f0c8` | `-` |
| `ldr` at 0x5ef34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ef38 | store | 4 | `abs 0x148424` | `[sp-28]` |
| `ldr` at 0x5ef3c | load | 4 | `abs 0x5f0cc` | `-` |
| `ldr` at 0x5ef40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5ef44 | store | 4 | `abs 0x148428` | `[sp-24]` |
| `ldr` at 0x5ef48 | load | 4 | `abs 0x5f0d0` | `-` |
| `str` at 0x5ef50 | store | 4 | `abs 0x14842c` | `0xc` |
| `ldr` at 0x5ef54 | load | 4 | `abs 0x5f0d4` | `-` |
| `ldr` at 0x5ef58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ef6c | store | 4 | `abs 0x2b0900` | `ret(0x1b124)` |
| `ldr` at 0x5ef70 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5ef7c | load | 4 | `abs 0x5f0d8` | `-` |
| `str` at 0x5ef80 | store | 4 | `sp -0x44` | `0x10c550` |
| `ldr` at 0x5ef88 | load | 4 | `abs 0x5f0c4` | `-` |
| `str` at 0x5ef9c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5efa0 | load | 4 | `abs 0x5f0c8` | `-` |
| `ldr` at 0x5efa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5efa8 | load | 4 | `abs 0x148424` | `-` |
| `ldr` at 0x5efb8 | load | 4 | `abs 0x5f0cc` | `-` |
| `ldr` at 0x5efbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5efd0 | load | 4 | `abs 0x148428` | `-` |
| `str` at 0x5efd8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5efdc | load | 4 | `abs 0x5f0d4` | `-` |
| `ldr` at 0x5efe0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5efe8 | load | 4 | `abs 0x2b0900` | `-` |
| `str` at 0x5eff0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5eff4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5eff8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5effc | load | 1 | `(unknown)` | `-` |
| `strb` at 0x5f004 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x5f008 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f00c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5f010 | load | 1 | `[sp-44] +0x3` | `-` |
| `strb` at 0x5f018 | store | 1 | `[sp-48] +0x1` | `([[sp-44]+3]:1+5 & 0xff)` |
| `ldr` at 0x5f01c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f020 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5f024 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x5f028 | store | 4 | `[sp-48] +0x4` | `[[sp-44]+12]` |
| `ldr` at 0x5f02c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5f034 | store | 4 | `[sp-48] +0x8` | `0x200` |
| `ldr` at 0x5f038 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f03c | load | 4 | `abs 0x5f0dc` | `-` |
| `ldr` at 0x5f040 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f044 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5f048 | store | 4 | `[sp-48] +0xc` | `?` |
| `ldr` at 0x5f04c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5f054 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x5f05c | load | 4 | `abs 0x5f0e0` | `-` |
| `ldr` at 0x5f064 | load | 4 | `abs 0x5f0d4` | `-` |
| `ldr` at 0x5f068 | load | 4 | `abs 0x5f0c8` | `-` |
| `ldr` at 0x5f06c | load | 4 | `abs 0x5f0d0` | `-` |
| `str` at 0x5f074 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x5f078 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5f07c | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0x5f080 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5f084 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x5f088 | load | 4 | `abs 0x5f0e0` | `-` |
| `str` at 0x5f08c | store | 4 | `sp -0x38` | `0x148438` |
| `str` at 0x5f094 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x5f098 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f09c | load | 4 | `abs 0x148424` | `-` |
| `ldr` at 0x5f0a0 | load | 4 | `abs 0x14842c` | `-` |
| `ldr` at 0x5f0ac | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5f0b0 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0x5f0b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5f0b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f0b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f0b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ef20 | `(unknown callee) 0x21e28` | `0x9` | `0x10c53c` | `0x91` | `0x0` |
| `bl` at 0x5ef64 | `(unknown callee) 0x1b124` | `?` | `?` | `0x14842c` | `?` |
| `bl` at 0x5ef94 | `(unknown callee) 0x21e28` | `0x9` | `0x10c53c` | `0x9d` | `0x0` |
| `bl` at 0x5f060 | `(unknown callee) 0x5f358` | `0x148438` | `?` | `?` | `[0x148424]` |
| `bl` at 0x5f0a8 | `(unknown callee) 0x47560` | `[0x2b0900]` | `[0x148424]` | `[0x14842c]` | `0x0` |
| `bl` at 0x5f0b4 | `(unknown callee) 0x5f0e4` | `[sp]` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ef04 | `[0x148420]:1 == 0x0` | `[0x148420]:1 != 0x0` |
| `bne` at 0x5ef78 | `[0x2b0900] != 0x0` | `[0x2b0900] == 0x0` |
| `blt` at 0x5efb0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f0b8 | `r0` | `ret(0x5f0e4)` |

## `IP57_Set_MemCfg`

- `IP57_Set_MemCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f0e4 to 0x5f1a4.
  Established. The initial guard takes `r0 == 0x0` and falls through on nonzero (`beq` at 0x5f104).
  The call to `0x21e28` has candidates `0x9`, `0x10c53c`, `0xd4` and `0x0` (`bl` at 0x5f158). The return
  records `ret(0x47a14)` (`ldmdb` at 0x5f190). The chosen memory configuration remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f0f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f0f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5f0fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f108 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f124 | load | 4 | `abs 0x5f194` | `-` |
| `str` at 0x5f128 | store | 4 | `sp -0x14` | `0x10c420` |
| `str` at 0x5f130 | store | 4 | `sp -0x18` | `0x6` |
| `ldr` at 0x5f138 | load | 4 | `abs 0x5f198` | `-` |
| `str` at 0x5f13c | store | 4 | `sp -0x20` | `0x10c58c` |
| `ldr` at 0x5f140 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5f144 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5f14c | load | 4 | `abs 0x5f19c` | `-` |
| `ldr` at 0x5f15c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f174 | load | 4 | `abs 0x5f1a0` | `-` |
| `str` at 0x5f178 | store | 4 | `sp -0x14` | `0x10c480` |
| `str` at 0x5f180 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x5f184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f188 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5f190 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f190 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f190 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f158 | `(unknown callee) 0x21e28` | `0x9` | `0x10c53c` | `0xd4` | `0x0` |
| `bl` at 0x5f18c | `(unknown callee) 0x47a14` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f104 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x5f110 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x5f11c | `r0 == 0x64` | `r0 != 0x64` |
| `beq` at 0x5f164 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5f170 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f190 | `r0` | `ret(0x47a14)` |

## `IP67_Cfg_Init`

- `IP67_Cfg_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f1a4 to 0x5f358.
  Established. A byte load reads absolute `0x148420` (`ldrb` at 0x5f1c8). Zero takes the first
  branch and nonzero falls through (`beq` at 0x5f1d0). The opening call to `0x21e28` uses `0x9`,
  `0x10c53c`, `0xf1` and `0x0` as candidates (`bl` at 0x5f1ec). The hardware configuration effect is
  unknown beyond the listed operations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f1b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5f1b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5f1bc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5f1c0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5f1c4 | load | 4 | `abs 0x5f330` | `-` |
| `ldrb` at 0x5f1c8 | load | 1 | `abs 0x148420` | `-` |
| `ldr` at 0x5f1d4 | load | 4 | `abs 0x5f334` | `-` |
| `str` at 0x5f1d8 | store | 4 | `sp -0x38` | `0x10c5c8` |
| `ldr` at 0x5f1e0 | load | 4 | `abs 0x5f338` | `-` |
| `ldr` at 0x5f1f0 | load | 4 | `abs 0x5f330` | `-` |
| `strb` at 0x5f1f8 | store | 1 | `abs 0x148420` | `0x1` |
| `ldr` at 0x5f1fc | load | 4 | `abs 0x5f33c` | `-` |
| `str` at 0x5f204 | store | 4 | `abs 0x148424` | `0x1` |
| `ldr` at 0x5f208 | load | 4 | `abs 0x5f340` | `-` |
| `ldr` at 0x5f20c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5f210 | store | 4 | `abs 0x148428` | `[sp-24]` |
| `ldr` at 0x5f214 | load | 4 | `abs 0x5f344` | `-` |
| `str` at 0x5f21c | store | 4 | `abs 0x14842c` | `0x8` |
| `ldr` at 0x5f220 | load | 4 | `abs 0x5f348` | `-` |
| `str` at 0x5f230 | store | 4 | `abs 0x2b0900` | `ret(0x1b124)` |
| `ldr` at 0x5f234 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f240 | load | 4 | `abs 0x5f34c` | `-` |
| `str` at 0x5f244 | store | 4 | `sp -0x38` | `0x10c550` |
| `ldr` at 0x5f24c | load | 4 | `abs 0x5f338` | `-` |
| `ldr` at 0x5f25c | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f260 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f264 | load | 4 | `abs 0x5f340` | `-` |
| `ldr` at 0x5f268 | load | 4 | `abs 0x148428` | `-` |
| `ldrb` at 0x5f26c | load | 1 | `[0x148428] +0x3` | `-` |
| `strb` at 0x5f274 | store | 1 | `[0x2b0900] +0x0` | `([[0x148428]+3]:1+1 & 0xff)` |
| `ldr` at 0x5f278 | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f27c | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f280 | load | 4 | `abs 0x5f340` | `-` |
| `ldr` at 0x5f284 | load | 4 | `abs 0x148428` | `-` |
| `ldrb` at 0x5f288 | load | 1 | `[0x148428] +0x3` | `-` |
| `strb` at 0x5f290 | store | 1 | `[0x2b0900] +0x1` | `([[0x148428]+3]:1+5 & 0xff)` |
| `ldr` at 0x5f294 | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f298 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f29c | load | 4 | `abs 0x5f340` | `-` |
| `ldr` at 0x5f2a0 | load | 4 | `abs 0x148428` | `-` |
| `ldr` at 0x5f2a4 | load | 4 | `[0x148428] +0xc` | `-` |
| `str` at 0x5f2a8 | store | 4 | `[0x2b0900] +0x4` | `[[0x148428]+12]` |
| `ldr` at 0x5f2ac | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f2b0 | load | 4 | `abs 0x2b0900` | `-` |
| `str` at 0x5f2b8 | store | 4 | `[0x2b0900] +0x8` | `0xff000000` |
| `ldr` at 0x5f2bc | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f2c0 | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f2c4 | load | 4 | `abs 0x5f350` | `-` |
| `ldr` at 0x5f2c8 | load | 4 | `abs 0x10c510` | `-` |
| `str` at 0x5f2cc | store | 4 | `[0x2b0900] +0xc` | `[0x10c510]` |
| `ldr` at 0x5f2d0 | load | 4 | `abs 0x5f354` | `-` |
| `ldr` at 0x5f2d8 | load | 4 | `abs 0x5f348` | `-` |
| `ldr` at 0x5f2dc | load | 4 | `abs 0x5f33c` | `-` |
| `ldr` at 0x5f2e0 | load | 4 | `abs 0x5f344` | `-` |
| `str` at 0x5f2e8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x5f2ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5f2f0 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x5f2f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5f2f8 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x5f2fc | load | 4 | `abs 0x5f354` | `-` |
| `str` at 0x5f300 | store | 4 | `sp -0x2c` | `0x148438` |
| `str` at 0x5f308 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5f30c | load | 4 | `abs 0x2b0900` | `-` |
| `ldr` at 0x5f310 | load | 4 | `abs 0x148424` | `-` |
| `ldr` at 0x5f314 | load | 4 | `abs 0x14842c` | `-` |
| `ldr` at 0x5f320 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5f324 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0x5f32c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5f32c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f32c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f32c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f1ec | `(unknown callee) 0x21e28` | `0x9` | `0x10c53c` | `0xf1` | `0x0` |
| `bl` at 0x5f228 | `(unknown callee) 0x1b124` | `0x10` | `?` | `0x14842c` | `0x8` |
| `bl` at 0x5f258 | `(unknown callee) 0x21e28` | `0x9` | `0x10c53c` | `0xfc` | `0x0` |
| `bl` at 0x5f2d4 | `(unknown callee) 0x5f358` | `0x148438` | `?` | `[0x2b0900]` | `[0x10c510]` |
| `bl` at 0x5f31c | `(unknown callee) 0x47560` | `[0x2b0900]` | `[0x148424]` | `[0x14842c]` | `0x0` |
| `bl` at 0x5f328 | `(unknown callee) 0x47a14` | `[sp-36]` | `[sp]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f1d0 | `[0x148420]:1 == 0x0` | `[0x148420]:1 != 0x0` |
| `bne` at 0x5f23c | `[0x2b0900] != 0x0` | `[0x2b0900] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f32c | `r0` | `ret(0x47a14)` |

## `IP57_Probe`

- `IP57_Probe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f414 to 0x5f690.
  Established. The call to `0x60f28` has candidates `(r0 & 0xff)`, `sp-24`, `sp-24` and `(r0 & 0xff)` (`bl`
  at 0x5f444); its zero result takes the branch and nonzero falls through (`beq` at 0x5f458). The
  final r0 expression is unknown (`ldmdb` at 0x5f674). Probe success and processor-presence meanings
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5f428 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5f430 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x5f434 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5f44c | store | 4 | `sp -0x20` | `ret(0x60f28)` |
| `ldr` at 0x5f450 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5f460 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5f468 | load | 4 | `abs 0x5f678` | `-` |
| `ldrb` at 0x5f46c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5f474 | store | 1 | `[sp-17]:1 +0x148434` | `0x0` |
| `ldr` at 0x5f478 | load | 4 | `abs 0x5f67c` | `-` |
| `ldrb` at 0x5f47c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5f484 | store | 1 | `[sp-17]:1 +0x2b08e0` | `0x2` |
| `ldr` at 0x5f488 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f48c | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x5f494 | store | 1 | `sp -0x21` | `([[sp-24]+3]:1+5 & 0xff)` |
| `ldrb` at 0x5f498 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5f49c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f4a0 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x5f4a4 | load | 4 | `abs 0x5f680` | `-` |
| `ldr` at 0x5f4a8 | load | 4 | `abs 0x14842c` | `-` |
| `ldrb` at 0x5f4c0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5f4d0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x5f4d8 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `strb` at 0x5f4e4 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x5f4e8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5f4f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5f4fc | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x5f500 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x5f524 | store | 4 | `sp -0x20` | `ret(0xd6b9c)` |
| `ldr` at 0x5f528 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5f534 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x5f548 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5f554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5f560 | load | 4 | `abs 0x5f684` | `-` |
| `str` at 0x5f564 | store | 4 | `sp -0x34` | `0x10c5e8` |
| `ldrb` at 0x5f568 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x5f56c | store | 4 | `sp -0x30` | `[sp-33]:1` |
| `str` at 0x5f570 | store | 4 | `sp -0x2c` | `ret(0x219a8)` |
| `ldr` at 0x5f578 | load | 4 | `abs 0x5f688` | `-` |
| `ldr` at 0x5f57c | load | 4 | `abs 0x5f68c` | `-` |
| `ldrb` at 0x5f588 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f58c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5f5a0 | load | 4 | `abs 0x5f678` | `-` |
| `ldrb` at 0x5f5a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5f5a8 | load | 4 | `abs 0x5f678` | `-` |
| `ldrb` at 0x5f5ac | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f5b0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5f5bc | load | 1 | `[sp-17]:1 +0x148434` | `-` |
| `strb` at 0x5f5c4 | store | 1 | `[sp-17]:1 +0x148434` | `?` |
| `ldrb` at 0x5f5c8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x5f5d8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f5e4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5f604 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f608 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5f61c | load | 4 | `abs 0x5f67c` | `-` |
| `ldrb` at 0x5f620 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5f628 | store | 1 | `[sp-17]:1 +0x2b08e0` | `0x1` |
| `ldrb` at 0x5f62c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x5f634 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x5f63c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f654 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5f66c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x5f670 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5f674 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f674 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f444 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5f4cc | `(unknown callee) 0xd18b8` | `0x0` | `?` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0x5f51c | `(unknown callee) 0xd6b9c` | `0x0` | `?` | `sp-25` | `?` |
| `bl` at 0x5f558 | `(unknown callee) 0x219a8` | `ret(0xd6b9c)` | `?` | `?` | `ret(0xd6b9c)` |
| `bl` at 0x5f584 | `(unknown callee) 0x21e28` | `0x1` | `0x10c53c` | `0x165` | `0x0` |
| `bl` at 0x5f598 | `(unknown callee) 0x6004c` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5f5f4 | `(unknown callee) 0x5d8c0` | `?` | `[sp-17]:1` | `?` | `?` |
| `bl` at 0x5f614 | `(unknown callee) 0x5ff54` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5f644 | `(unknown callee) 0x5df4c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x5f664 | `(unknown callee) 0x5f978` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f458 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `blt` at 0x5f4b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x5f4f0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x5f530 | `ret(0xd6b9c) != 0x0` | `ret(0xd6b9c) == 0x0` |
| `bne` at 0x5f540 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5f550 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `beq` at 0x5f5d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5f600 | `ret(0x5d8c0) != 0x0` | `ret(0x5d8c0) == 0x0` |
| `bne` at 0x5f650 | `ret(0x5df4c) != 0x0` | `ret(0x5df4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f674 | `r0` | `?` |

## `IP65_Probe`

- `IP65_Probe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63cac to 0x63db8.
  Established. The opening call to `0x64978` has candidates `(r0 & 0xff)`, `sp-24`, `sp-24` and `(r0 &
  0xff)` (`bl` at 0x63cdc). A zero result takes the branch, while nonzero falls through (`beq` at
  0x63cf0). The recorded return is unknown (`ldmdb` at 0x63dac). The board probe's physical
  interpretation remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x63cc0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x63cc8 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x63ccc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x63ce4 | store | 4 | `sp -0x20` | `ret(0x64978)` |
| `ldr` at 0x63ce8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x63cf8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x63d00 | load | 4 | `abs 0x63db0` | `-` |
| `ldrb` at 0x63d04 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x63d0c | store | 1 | `[sp-17]:1 +0x1484fc` | `0x0` |
| `ldr` at 0x63d10 | load | 4 | `abs 0x63db4` | `-` |
| `ldrb` at 0x63d14 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x63d1c | store | 1 | `[sp-17]:1 +0x2b0948` | `0x2` |
| `ldr` at 0x63d20 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63d24 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x63d28 | store | 1 | `sp -0x21` | `([[sp-24]+2]:1 & 0xff)` |
| `ldrb` at 0x63d2c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x63d30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x63d34 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x63d48 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x63d58 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x63d60 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x63d68 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x63d74 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x63d8c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x63da4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x63da8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x63dac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63dac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63dac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63cdc | `(unknown callee) 0x64978` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x63d54 | `(unknown callee) 0xd18b8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x63d70 | `(unknown callee) 0x64610` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x63d7c | `(unknown callee) 0x5df4c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x63d9c | `(unknown callee) 0x64054` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x63cf0 | `ret(0x64978) == 0x0` | `ret(0x64978) != 0x0` |
| `ble` at 0x63d40 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x63d88 | `ret(0x5df4c) != 0x0` | `ret(0x5df4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63dac | `r0` | `?` |

## `networkInit`

- `networkInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24664 to 0x246bc.
  Established. The first call to `0x1f1dc` has `0x34`, entry r1, entry r2 and entry r3 as candidates
  (`bl` at 0x24674). Zero takes the following branch and nonzero falls through (`beq` at 0x24680).
  The return carries `ret(0x246bc)` (`ldmdb` at 0x246b0). Network initialisation behavior remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24684 | load | 4 | `abs 0x246b4` | `-` |
| `ldr` at 0x246a4 | load | 4 | `abs 0x246b8` | `-` |
| `str` at 0x246ac | store | 4 | `abs 0x1423ec` | `0x0` |
| `ldmdb` at 0x246b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x246b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x246b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24674 | `(unknown callee) 0x1f1dc` | `0x34` | `r1` | `r2` | `r3` |
| `bl` at 0x24688 | `(unknown callee) 0x2070c` | `0xfeacc` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x24694 | `(unknown callee) 0x1d38c` | `0x34` | `0x0` | `?` | `?` |
| `bl` at 0x24698 | `(unknown callee) 0x246bc` | `ret(0x1d38c)` | `?` | `?` | `?` |
| `bl` at 0x246a0 | `(unknown callee) 0x246bc` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24680 | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x246b0 | `r0` | `ret(0x246bc)` |

## `networkGetType`

- `networkGetType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x247c4 to 0x247e4.
  Established. A byte read accesses absolute `0x147ea0` (`ldrb` at 0x247d4). The return preserves
  `[0x147ea0]:1` (`ldmdb` at 0x247dc). Network-type encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x247d0 | load | 4 | `abs 0x247e0` | `-` |
| `ldrb` at 0x247d4 | load | 1 | `abs 0x147ea0` | `-` |
| `ldmdb` at 0x247dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x247dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x247dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x247dc | `r0` | `[0x147ea0]:1` |

## `L1NetActive`

- `L1NetActive`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x656f8 to 0x65718.
  Established. The byte load reads absolute `0x2b0968` (`ldrb` at 0x65708); its value reaches the
  exit as `[0x2b0968]:1` (`ldmdb` at 0x65710). The active-state meaning of this byte remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65704 | load | 4 | `abs 0x65714` | `-` |
| `ldrb` at 0x65708 | load | 1 | `abs 0x2b0968` | `-` |
| `ldmdb` at 0x65710 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65710 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65710 | `r0` | `[0x2b0968]:1` |

## `B2BConfigured`

- `B2BConfigured`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79e74 to 0x79ebc.
  Established. A literal word is read at absolute `0x79eb8` (`ldr` at 0x79e88). The later word
  load has an unknown effective address (`ldr` at 0x79ea0). The return expression is unknown
  (`ldmdb` at 0x79eb4). The pointee layout and configured-state encoding remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79e84 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x79e88 | load | 4 | `abs 0x79eb8` | `-` |
| `ldr` at 0x79e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79ea0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x79eb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x79eb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x79eb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x79eb4 | `r0` | `?` |
