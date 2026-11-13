# ARM scan field and data helpers in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
ELF function symbols and sizes. Span ends are exclusive. Symbol sizes can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Unknown means unresolved by these facts, not a proven indeterminate runtime value.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/scan-field-data-helpers-facts.txt) retain all tool sections.

## `JChip_protect_field`

- `JChip_protect_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb810 to 0xeb9a0 (400 bytes).
  Established. The optional call to `0xf16a4` carries unknown, `[sp-28]`, `[sp-32]`, unknown (`bl`
  at 0xeb920). The deciding branch bypasses that call for `[sp-32] == 0x0` and falls through for
  nonzero (`beq` at 0xeb8fc). A later byte store has both unknown target and unknown value (`strb`
  at 0xeb974). Protection-bit semantics and the common r0 return remain unproved (`ldmdb` at
  0xeb984).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb820 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb824 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb828 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb82c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb830 | load | 4 | `abs 0xeb988` | `-` |
| `ldr` at 0xeb834 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeb840 | load | 4 | `abs 0xeb98c` | `-` |
| `ldr` at 0xeb844 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb848 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb84c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb854 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb860 | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xeb864 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb870 | load | 4 | `abs 0xeb990` | `-` |
| `str` at 0xeb874 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xeb87c | load | 4 | `abs 0xeb988` | `-` |
| `ldr` at 0xeb880 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeb88c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb898 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xeb8a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb8a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb8a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb8bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb8c4 | load | 4 | `abs 0xeb994` | `-` |
| `ldr` at 0xeb8c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb8d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb8dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb8e0 | load | 4 | `abs 0xeb998` | `-` |
| `ldr` at 0xeb8e4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xeb8ec | load | 4 | `abs 0xeb99c` | `-` |
| `ldr` at 0xeb8f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb904 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb90c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb914 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb918 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb91c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeb924 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb930 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb938 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb93c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb940 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb948 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeb94c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb954 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb95c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb960 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeb964 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xeb96c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xeb974 | store | 1 | `(unknown)` | `?` |
| `str` at 0xeb97c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xeb980 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xeb984 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeb984 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeb984 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb850 | `(unknown callee) 0x23f2c` | `0x136738` | `r0` | `r1` | `r2` |
| `bl` at 0xeb858 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xeb8cc | `(unknown callee) 0x23f2c` | `0x136770` | `?` | `?` | `?` |
| `bl` at 0xeb8e8 | `(unknown callee) 0x23f2c` | `0x136790` | `[[sp-32]]` | `?` | `[sp-32]` |
| `bl` at 0xeb8f0 | `(unknown callee) 0x23f2c` | `0x1367a4` | `?` | `?` | `?` |
| `bl` at 0xeb920 | `(unknown callee) 0xf16a4` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xeb83c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeb86c | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `bls` at 0xeb888 | `[0x2ad25c] <= 0x1 (unsigned, 32-bit)` | `[0x2ad25c] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0xeb8d8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xeb8fc | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeb984 | `r0` | `?` |

## `JChip_get_field`

- `JChip_get_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeb9a0 to 0xebacc (300 bytes).
  Established. The call result takes its branch on `ret(0xeb27c) != 0x0` and falls through on zero
  (`bne` at 0xeb9fc). Later `0xf0448` receives unknown, `[sp-28]`, `0x0`, unknown (`bl` at 0xebaa8).
  An earlier word store retains entry r3 at `sp -0x20` (`str` at 0xeb9bc). Field interpretation and
  r0 at the merged exit are unknown (`ldmdb` at 0xebab8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeb9b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeb9b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeb9b8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeb9bc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xeb9c0 | load | 4 | `abs 0xebabc` | `-` |
| `ldr` at 0xeb9c4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeb9d0 | load | 4 | `abs 0xebac0` | `-` |
| `ldr` at 0xeb9d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeb9d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeb9dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeb9e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeb9f0 | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xeb9f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeba00 | load | 4 | `abs 0xebac4` | `-` |
| `str` at 0xeba04 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xeba0c | load | 4 | `abs 0xebabc` | `-` |
| `ldr` at 0xeba10 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xeba1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeba20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeba28 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xeba30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeba4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba54 | load | 4 | `abs 0xebac8` | `-` |
| `ldr` at 0xeba58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeba64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeba6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeba7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeba84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeba94 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0xeba98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeba9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebaa4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xebab0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xebab4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xebab8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebab8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebab8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeb9e0 | `(unknown callee) 0x23f2c` | `0x1367a8` | `r0` | `r1` | `r2` |
| `bl` at 0xeb9e8 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xeba5c | `(unknown callee) 0x23f2c` | `0x1367dc` | `?` | `?` | `?` |
| `bl` at 0xebaa8 | `(unknown callee) 0xf0448` | `?` | `[sp-28]` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xeb9cc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xeb9fc | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `bls` at 0xeba18 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebab8 | `r0` | `?` |

## `JChip_get_field_by_name`

- `JChip_get_field_by_name`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebacc to 0xebc4c (384 bytes).
  Established. The call to `0xf072c` carries `[sp-44]`, `[sp-28]`, unknown, unknown (`bl` at
  0xebb60); its unsigned bounds branch is taken for `ret(0xf072c) < [[sp-44]+2]:2` and falls through
  for greater or equal (`blo` at 0xebb7c). The subsequent call to `0xf0448` has unknown, unknown,
  `0x0`, unknown (`bl` at 0xebc28). Name-matching rules and the r0 result cannot be established from
  these candidates (`ldmdb` at 0xebc38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebadc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebae0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebae4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xebae8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xebaec | load | 4 | `abs 0xebc3c` | `-` |
| `ldr` at 0xebaf0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebafc | load | 4 | `abs 0xebc40` | `-` |
| `ldr` at 0xebb00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebb04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebb08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebb10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebb1c | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xebb20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebb2c | load | 4 | `abs 0xebc44` | `-` |
| `str` at 0xebb30 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xebb38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebb3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebb44 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xebb4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebb50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xebb54 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xebb58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xebb5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xebb68 | store | 4 | `sp -0x28` | `ret(0xf072c)` |
| `ldr` at 0xebb6c | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xebb70 | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xebb74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebb80 | load | 4 | `abs 0xebc44` | `-` |
| `str` at 0xebb84 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xebb8c | load | 4 | `abs 0xebc3c` | `-` |
| `ldr` at 0xebb90 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebb9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebba0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebba8 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xebbb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebbcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbd4 | load | 4 | `abs 0xebc48` | `-` |
| `ldr` at 0xebbd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbe0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebbe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebbec | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xebbf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbf8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebbfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebc04 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xebc0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebc10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xebc14 | store | 4 | `sp -0x34` | `[sp-32]` |
| `ldr` at 0xebc18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebc1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebc24 | load | 4 | `(unknown)` | `-` |
| `str` at 0xebc30 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xebc34 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xebc38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebc38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebc38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebb0c | `(unknown callee) 0x23f2c` | `0x1367fc` | `r0` | `r1` | `r2` |
| `bl` at 0xebb14 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xebb60 | `(unknown callee) 0xf072c` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xebbdc | `(unknown callee) 0x23f2c` | `0x1367dc` | `?` | `?` | `?` |
| `bl` at 0xebc28 | `(unknown callee) 0xf0448` | `?` | `?` | `0x0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xebaf8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xebb28 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `blo` at 0xebb7c | `ret(0xf072c) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xf072c) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `bls` at 0xebb98 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebc38 | `r0` | `?` |

## `JChip_get_field_str`

- `JChip_get_field_str`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebc4c to 0xebe24 (472 bytes).
  Established. The call to `0x1b124` has four unknown register candidates (`bl` at 0xebd78); its
  nonzero result takes the continuation and zero falls through (`bne` at 0xebd8c). A later call to
  `0xf1074` uses `[sp-40]`, `[sp-44]`, `[sp-32]`, unknown (`bl` at 0xebdf4). The final call targets
  `0x1b400` with `[sp-44]` and three unknown candidates (`bl` at 0xebdfc). String encoding,
  ownership and r0 at return remain unresolved (`ldmdb` at 0xebe0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebc5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebc60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebc64 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xebc68 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xebc6c | load | 4 | `abs 0xebe10` | `-` |
| `ldr` at 0xebc70 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebc7c | load | 4 | `abs 0xebe14` | `-` |
| `ldr` at 0xebc80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebc84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebc88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebc90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebc9c | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xebca0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebcac | load | 4 | `abs 0xebe18` | `-` |
| `str` at 0xebcb0 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xebcb8 | load | 4 | `abs 0xebe10` | `-` |
| `ldr` at 0xebcbc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebcc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebcd4 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xebcdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebce0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebcf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebd00 | load | 4 | `abs 0xebe1c` | `-` |
| `ldr` at 0xebd04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebd0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebd10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebd18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebd20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebd24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebd28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebd3c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xebd44 | load | 2 | `(unknown)` | `-` |
| `str` at 0xebd48 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xebd4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebd64 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xebd80 | store | 4 | `sp -0x2c` | `ret(0x1b124)` |
| `ldr` at 0xebd84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xebd90 | load | 4 | `abs 0xebe20` | `-` |
| `str` at 0xebd94 | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xebd9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebda0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebda8 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xebdb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebdb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebdb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebdc0 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xebdc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebdcc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xebdd0 | store | 4 | `sp -0x34` | `ret(0x1b124)` |
| `ldr` at 0xebdd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebdd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebde0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebde8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebdec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xebdf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebdf8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xebe04 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xebe08 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xebe0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebe0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebe0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebc8c | `(unknown callee) 0x23f2c` | `0x136838` | `r0` | `r1` | `r2` |
| `bl` at 0xebc94 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xebd08 | `(unknown callee) 0x23f2c` | `0x136870` | `?` | `?` | `?` |
| `bl` at 0xebd78 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xebde4 | `(unknown callee) 0xf0448` | `?` | `[sp-28]` | `0x0` | `?` |
| `bl` at 0xebdf4 | `(unknown callee) 0xf1074` | `[sp-40]` | `[sp-44]` | `[sp-32]` | `?` |
| `bl` at 0xebdfc | `(unknown callee) 0x1b400` | `[sp-44]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xebc78 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xebca8 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `bls` at 0xebcc4 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xebd8c | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebe0c | `r0` | `?` |

## `JChip_can_override_field`

- `JChip_can_override_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebe24 to 0xebed0 (172 bytes).
  Established. The nonzero call result takes the continuation and zero falls through (`bne` at
  0xebe7c). The later byte read has unknown effective address (`ldrb` at 0xebeb0). The facts report
  r0 as unknown at the common exit (`ldmdb` at 0xebec4). The routine name alone does not prove a
  field-permission rule.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebe34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebe38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebe3c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xebe40 | load | 4 | `abs 0xebec8` | `-` |
| `ldr` at 0xebe44 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebe50 | load | 4 | `abs 0xebecc` | `-` |
| `ldr` at 0xebe54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebe58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebe5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebe64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebe70 | store | 4 | `sp -0x20` | `ret(0xeb27c)` |
| `ldr` at 0xebe74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xebe84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xebe8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebe90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebe98 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xebea0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebea4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebea8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xebeb0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xebebc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xebec0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xebec4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebec4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebec4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebe60 | `(unknown callee) 0x23f2c` | `0x136894` | `r0` | `r1` | `r2` |
| `bl` at 0xebe68 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xebe4c | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xebe7c | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebec4 | `r0` | `?` |

## `JChip_override_field`

- `JChip_override_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebed0 to 0xec028 (344 bytes).
  Established. The byte read before the local guard has unknown address (`ldrb` at 0xebf6c). That
  guard takes its branch for an unknown value equal to `0x0` and falls through on nonzero (`beq` at
  0xebf78). The later call to `0xf16a4` has unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at
  0xec004). Override eligibility and the joined r0 return remain unknown (`ldmdb` at 0xec014).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebee0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebee4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebee8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xebeec | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xebef0 | load | 4 | `abs 0xec018` | `-` |
| `ldr` at 0xebef4 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebf00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebf04 | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xebf08 | store | 4 | `sp -0x2c` | `[r3]` |
| `ldr` at 0xebf0c | load | 4 | `abs 0xec01c` | `-` |
| `ldr` at 0xebf10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebf14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebf18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebf20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebf2c | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xebf30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebf3c | load | 4 | `abs 0xec020` | `-` |
| `str` at 0xebf40 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xebf48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebf4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebf54 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xebf5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf64 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xebf6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebf7c | load | 4 | `abs 0xec020` | `-` |
| `str` at 0xebf80 | store | 4 | `sp -0x28` | `0xfffff7c6` |
| `ldr` at 0xebf88 | load | 4 | `abs 0xec018` | `-` |
| `ldr` at 0xebf8c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xebf98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebfa4 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xebfac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebfb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebfb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebfc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebfd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebfd4 | load | 4 | `abs 0xec024` | `-` |
| `ldr` at 0xebfd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebfdc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xebfe4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebfe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebff0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebffc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec000 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec00c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xec010 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xec014 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec014 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec014 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebf1c | `(unknown callee) 0x23f2c` | `0x1368d0` | `r0` | `r1` | `r2` |
| `bl` at 0xebf24 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xebfe0 | `(unknown callee) 0x23f2c` | `0x136918` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xec004 | `(unknown callee) 0xf16a4` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xebefc | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xebf38 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `beq` at 0xebf78 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xebf94 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec014 | `r0` | `?` |

## `JChip_override_field_by_name`

- `JChip_override_field_by_name`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec028 to 0xec1d4 (428 bytes).
  Established. The call to `0xf072c` uses `[sp-44]`, `[sp-28]`, unknown, unknown (`bl` at 0xec0c8);
  its bounds comparison takes the continuation for unsigned `ret(0xf072c) < [[sp-44]+2]:2` and falls
  through for greater or equal (`blo` at 0xec0e4). A later call to `0xf16a4` carries unknown,
  unknown, `[sp-32]`, unknown (`bl` at 0xec1b0). Field-name interpretation and the common r0 value
  are unestablished (`ldmdb` at 0xec1c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec038 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec03c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec040 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec044 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec048 | load | 4 | `abs 0xec1c4` | `-` |
| `ldr` at 0xec04c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec058 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec05c | load | 4 | `r3 +0x0` | `-` |
| `str` at 0xec060 | store | 4 | `sp -0x34` | `[r3]` |
| `ldr` at 0xec064 | load | 4 | `abs 0xec1c8` | `-` |
| `ldr` at 0xec068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec06c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec070 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec078 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec084 | store | 4 | `sp -0x24` | `ret(0xeb27c)` |
| `ldr` at 0xec088 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec094 | load | 4 | `abs 0xec1cc` | `-` |
| `str` at 0xec098 | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec0a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec0a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec0ac | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xec0b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec0b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xec0bc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xec0c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xec0c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec0d0 | store | 4 | `sp -0x28` | `ret(0xf072c)` |
| `ldr` at 0xec0d4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xec0d8 | load | 2 | `[sp-44] +0x2` | `-` |
| `ldr` at 0xec0dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec0e8 | load | 4 | `abs 0xec1cc` | `-` |
| `str` at 0xec0ec | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec0f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec0f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec100 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec10c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec110 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xec118 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec128 | load | 4 | `abs 0xec1cc` | `-` |
| `str` at 0xec12c | store | 4 | `sp -0x30` | `0xfffff7c6` |
| `ldr` at 0xec134 | load | 4 | `abs 0xec1c4` | `-` |
| `ldr` at 0xec138 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec144 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec148 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec150 | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec158 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec15c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec160 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec174 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec17c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec180 | load | 4 | `abs 0xec1d0` | `-` |
| `ldr` at 0xec184 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec188 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xec190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec194 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec19c | load | 4 | `[sp-36] +0x28` | `-` |
| `ldr` at 0xec1a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec1a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec1ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec1b8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xec1bc | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xec1c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec1c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec1c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec074 | `(unknown callee) 0x23f2c` | `0x13694c` | `r0` | `r1` | `r2` |
| `bl` at 0xec07c | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xec0c8 | `(unknown callee) 0xf072c` | `[sp-44]` | `[sp-28]` | `?` | `?` |
| `bl` at 0xec18c | `(unknown callee) 0x23f2c` | `0x136918` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xec1b0 | `(unknown callee) 0xf16a4` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec054 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec090 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `blo` at 0xec0e4 | `ret(0xf072c) < [[sp-44]+2]:2 (unsigned, 32-bit)` | `ret(0xf072c) >= [[sp-44]+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xec124 | `? == 0x0` | `? != 0x0` |
| `bls` at 0xec140 | `[0x2ad25c] <= 0x3 (unsigned, 32-bit)` | `[0x2ad25c] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec1c0 | `r0` | `?` |

## `JChip_find_tdr`

- `JChip_find_tdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec1d4 to 0xec2bc (232 bytes).
  Established. The call to `0x1aff8` carries `[sp-24]` and three unknown candidates (`bl` at
  0xec274); its nonzero result takes the next branch and zero falls through (`bne` at 0xec280). The
  earlier loop comparison is resolved only as taken for `C == 0`, with fall-through otherwise (`blo`
  at 0xec24c). Lookup-table ordering and r0 at exit remain unknown (`ldmdb` at 0xec2ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec1e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec1e8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xec1ec | load | 4 | `abs 0xec2b0` | `-` |
| `ldr` at 0xec1f0 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec1fc | load | 4 | `abs 0xec2b4` | `-` |
| `ldr` at 0xec200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec20c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec218 | store | 4 | `sp -0x1c` | `ret(0xeb27c)` |
| `ldr` at 0xec21c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec228 | load | 4 | `abs 0xec2b8` | `-` |
| `str` at 0xec22c | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `str` at 0xec238 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xec23c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec240 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec244 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec254 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec260 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec268 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec26c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec270 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec284 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec288 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xec290 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xec298 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xec2a0 | load | 4 | `abs 0xec2b8` | `-` |
| `str` at 0xec2a4 | store | 4 | `sp -0x24` | `0xfffff7c6` |
| `ldr` at 0xec2a8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xec2ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec2ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec2ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec208 | `(unknown callee) 0x23f2c` | `0x1369a0` | `r0` | `r1` | `[0x2ad25c]` |
| `bl` at 0xec210 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xec274 | `(unknown callee) 0x1aff8` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec1f8 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec224 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `blo` at 0xec24c | `C == 0` | `not (C == 0)` |
| `bne` at 0xec280 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec2ac | `r0` | `?` |

## `JChip_find_tdr_field`

- `JChip_find_tdr_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec2bc to 0xec394 (216 bytes).
  Established. A halfword read addresses `[sp-40] +0x2` (`ldrh` at 0xec35c). The prior call to
  `0xf072c` has `[sp-40]`, `[sp-28]`, unknown, unknown (`bl` at 0xec34c). The following unsigned
  branch takes its arm for `ret(0xf072c) < [[sp-40]+2]:2` and falls through for greater or equal
  (`blo` at 0xec368). Field-index validity and the r0 result stay unresolved (`ldmdb` at 0xec384).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec2cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec2d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec2d4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xec2d8 | load | 4 | `abs 0xec388` | `-` |
| `ldr` at 0xec2dc | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xec2e8 | load | 4 | `abs 0xec38c` | `-` |
| `ldr` at 0xec2ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec2f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec2f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec2fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec308 | store | 4 | `sp -0x20` | `ret(0xeb27c)` |
| `ldr` at 0xec30c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec318 | load | 4 | `abs 0xec390` | `-` |
| `str` at 0xec31c | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xec324 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec330 | load | 4 | `ret(0xeb27c) +0x28` | `-` |
| `ldr` at 0xec338 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xec33c | load | 4 | `(unknown)` | `-` |
| `str` at 0xec340 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xec344 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xec348 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec354 | store | 4 | `sp -0x24` | `ret(0xf072c)` |
| `ldr` at 0xec358 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xec35c | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xec360 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec36c | load | 4 | `abs 0xec390` | `-` |
| `str` at 0xec370 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0xec378 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xec37c | store | 4 | `sp -0x2c` | `ret(0xf072c)` |
| `ldr` at 0xec380 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xec384 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec384 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec384 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec2f8 | `(unknown callee) 0x23f2c` | `0x1369cc` | `r0` | `r1` | `r2` |
| `bl` at 0xec300 | `(unknown callee) 0xeb27c` | `?` | `?` | `?` | `?` |
| `bl` at 0xec34c | `(unknown callee) 0xf072c` | `[sp-40]` | `[sp-28]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xec2e4 | `[0x2ad25c] <= 0x2 (unsigned, 32-bit)` | `[0x2ad25c] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xec314 | `ret(0xeb27c) != 0x0` | `ret(0xeb27c) == 0x0` |
| `blo` at 0xec368 | `ret(0xf072c) < [[sp-40]+2]:2 (unsigned, 32-bit)` | `ret(0xf072c) >= [[sp-40]+2]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec384 | `r0` | `?` |

## `tdrdata_set_data`

- `tdrdata_set_data`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1188 to 0xf125c (212 bytes).
  Established. The word store in the body has unknown target and value (`str` at 0xf1244). Both
  comparisons take their branches for `C == 0` and fall through otherwise (`blo` at 0xf11e8,
  `blo` at 0xf1214). Their compared operands remain unknown in the tool output; its exit retains entry r0
  (`ldmdb` at 0xf1258). There are no calls in the facts. Data length and physical register effects
  are unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1198 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf119c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf11a4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf11a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf11ac | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf11b0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf11c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf11cc | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf11d0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf11e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf11f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf11f4 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf11f8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf120c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf121c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1220 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1228 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1230 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1238 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1240 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1244 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1248 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1250 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0xf1258 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1258 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1258 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf11c4 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf11e8 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1214 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1258 | `r0` | `r0` |

## `tdrdata_get_data`

- `tdrdata_get_data`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf125c to 0xf13fc (416 bytes).
  Established. Entry r1 selects the taken arm on zero and falls through on nonzero (`beq` at
  0xf1280). Each arm includes a word store with unknown target and value (`str` at 0xf132c, `str` at
  0xf13e4). The fact sections contain no calls and leave r0 at exit unknown (`ldmdb` at 0xf13f8).
  Source-array meaning and copy length cannot be resolved from the effective-address facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf126c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1270 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf1274 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf1278 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1288 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf128c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1290 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1294 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf12ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf12b0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf12b4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf12c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf12d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf12d8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf12dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf12f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1300 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1314 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1318 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1328 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf132c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1330 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1338 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xf1344 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf134c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1350 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf136c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1370 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1380 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1394 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1398 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf13ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf13bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf13c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf13cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf13d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf13e0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf13e4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf13e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf13f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xf13f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf13f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf13f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf1280 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xf12a8 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf12cc | `C == 0` | `not (C == 0)` |
| `blo` at 0xf12f8 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1364 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1388 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf13b4 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf13f8 | `r0` | `?` |

## `tdrdata_reset`

- `tdrdata_reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf13fc to 0xf15d8 (476 bytes).
  Established. The first branch takes its alternate arm for `[r0+8] == 0x0` and falls through for
  nonzero (`beq` at 0xf141c). The word store in that alternate arm writes `0x0` at an unknown target
  (`str` at 0xf1574). A later byte store also writes `0x0` at an unknown target (`strb` at 0xf15c0).
  The facts contain no calls. Reset-state semantics and the final r0 remain unidentified (`ldmdb` at
  0xf15d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf140c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf1410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1414 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xf1424 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf1428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf142c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1430 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf144c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1450 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1460 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1470 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1474 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1478 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf148c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf149c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf14a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf14a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf14b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf14b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf14b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf14c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf14c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf14cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf14d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf14d8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xf14e4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf14e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf14ec | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf14f0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf150c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1510 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1520 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1530 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1534 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1538 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf154c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf155c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1568 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1574 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf1578 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1580 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xf158c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf1590 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1594 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1598 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf159c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf15ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf15b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf15b4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xf15c0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xf15c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf15cc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xf15d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf15d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf15d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf141c | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `bne` at 0xf1444 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1468 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1494 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1504 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1528 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1554 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf15a4 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf15d4 | `r0` | `?` |

## `tdrdata_reset_field`

- `tdrdata_reset_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf15d8 to 0xf16a4 (204 bytes).
  Established. The sole call reaches `0xe2174` with unknown, `[[r0+8]+8]`, unknown, `[r0+16]` (`bl`
  at 0xf1684). A following byte store writes `0x0` at `([[sp-24]+12] + [sp-28]) +0x0` (`strb` at
  0xf169c). The return propagates `ret(0xe2174)` (`ldmdb` at 0xf16a0). No conditional branch is
  reported. Field-reset meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf15e8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf15ec | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf15f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf15f4 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf15f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf160c | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf1614 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1618 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf161c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1624 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1628 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf163c | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf1644 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf164c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1650 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1654 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1668 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf1670 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf1674 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf1678 | load | 4 | `[r0+8] +0x8` | `-` |
| `ldr` at 0xf1680 | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xf1688 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf168c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf1690 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xf169c | store | 1 | `([[sp-24]+12] + [sp-28]) +0x0` | `0x0` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1684 | `(unknown callee) 0xe2174` | `?` | `[[r0+8]+8]` | `?` | `[r0+16]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf16a0 | `r0` | `ret(0xe2174)` |

## `tdrdata_override_field`

- `tdrdata_override_field`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf16a4 to 0xf1714 (112 bytes).
  Established. The byte read and store address `([r0+12] + r1) +0x0`, with the stored value unknown
  (`ldrb` at 0xf16e0, `strb` at 0xf16e8). The call to `0xf0200` receives `[[sp-20]+8]`, `[sp-24]`,
  `[sp-28]`, `0x0` (`bl` at 0xf170c); its return supplies `ret(0xf0200)` (`ldmdb` at 0xf1710). The
  absence of conditional branches does not identify the byte flags or override format.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf16b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf16b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf16bc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf16c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf16c4 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf16c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf16d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf16d4 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf16d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf16e0 | load | 1 | `([r0+12] + r1) +0x0` | `-` |
| `strb` at 0xf16e8 | store | 1 | `([r0+12] + r1) +0x0` | `?` |
| `ldr` at 0xf16ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf16f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf16f4 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xf16f8 | store | 4 | `sp -0x20` | `[[sp-20]+16]` |
| `ldr` at 0xf16fc | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf1700 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1704 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xf1710 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1710 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1710 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf170c | `(unknown callee) 0xf0200` | `[[sp-20]+8]` | `[sp-24]` | `[sp-28]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1710 | `r0` | `ret(0xf0200)` |

## `tdrdata_build_mask`

- `tdrdata_build_mask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1820 to 0xf191c (252 bytes).
  Established. The first call to `0xe1f40` uses entry r2, `[[r0+8]]:2`, entry r3, `[[r0+8]]:2` (`bl`
  at 0xf1858). A later call to `0xe2060` has unknown, unknown, `[sp-32]`, unknown (`bl` at 0xf1904).
  The local guard takes its bypass for an unknown value equal to `0x0` and falls through on nonzero
  (`beq` at 0xf18a0). Mask bit ordering and r0 at the exit remain unknown (`ldmdb` at 0xf1918).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1830 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1834 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf1838 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf183c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf1840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1844 | load | 4 | `r0 +0x8` | `-` |
| `ldrh` at 0xf1848 | load | 2 | `[r0+8] +0x0` | `-` |
| `ldr` at 0xf184c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1854 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1860 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf1864 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1868 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrh` at 0xf186c | load | 2 | `[[sp-20]+8] +0x2` | `-` |
| `ldr` at 0xf1870 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1884 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xf1888 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xf1890 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1894 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf18a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf18a8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf18ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf18c0 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xf18c8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf18cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf18d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf18d8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf18dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf18f0 | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xf18f8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1900 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1908 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1910 | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xf1918 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1918 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1918 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1858 | `(unknown callee) 0xe1f40` | `r2` | `[[r0+8]]:2` | `r3` | `[[r0+8]]:2` |
| `bl` at 0xf1904 | `(unknown callee) 0xe2060` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf1878 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf18a0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1918 | `r0` | `?` |
