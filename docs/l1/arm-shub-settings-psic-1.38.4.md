# ARM SHUB settings and PSIC accessors in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/shub-settings-psic-facts.txt) retain all tool sections.

## `SHUB_MMR_Write`

- `SHUB_MMR_Write`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47a98 to 0x47b0c.
  Established. The call to `0xea56c` has unknown first candidate followed by entry r1, r2 and r3
  (`bl` at 0x47adc); its nonzero result takes the branch and zero falls through (`bne` at 0x47ae8).
  The exit reports `[sp-36]` (`ldmdb` at 0x47b04). Register selection and physical write semantics
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47aa8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47aac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47ab0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x47ab4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47ac0 | load | 4 | `abs 0x47b08` | `-` |
| `ldr` at 0x47ac4 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x47acc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47ad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47ad8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47af0 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x47afc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x47b00 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x47b04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47b04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47b04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47adc | `(unknown callee) 0xea56c` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47ae8 | `ret(0xea56c) != 0x0` | `ret(0xea56c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47b04 | `r0` | `[sp-36]` |

## `SHUB_MMR_Read`

- `SHUB_MMR_Read`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47b0c to 0x47b80.
  Established. The only call reaches `0xea468` with unknown, entry r1, entry r2, entry r3 (`bl` at
  0x47b50). The deciding branch takes the nonzero-result path and falls through on zero (`bne` at
  0x47b5c). R0 leaves as `[sp-36]` (`ldmdb` at 0x47b78). The meaning of the returned value is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47b1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47b20 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47b24 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x47b28 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47b2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47b34 | load | 4 | `abs 0x47b7c` | `-` |
| `ldr` at 0x47b38 | load | 4 | `abs 0x148058` | `-` |
| `ldr` at 0x47b40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47b44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47b4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47b64 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x47b70 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x47b74 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x47b78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47b78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47b78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47b50 | `(unknown callee) 0xea468` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47b5c | `ret(0xea468) != 0x0` | `ret(0xea468) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47b78 | `r0` | `[sp-36]` |

## `SHUB_Set_MemCfg`

- `SHUB_Set_MemCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47a14 to 0x47a98.
  Established. A word store writes entry r0 at absolute `0x148060` (`str` at 0x47a4c). A byte store
  writes `([sp-24]:1 & 0xff)` at `0x148064` (`strb` at 0x47a58). The word comparison takes its
  branch for `[0x148060] != [sp-28]` and falls through on equality (`bne` at 0x47a6c). The call to
  `0x47f30` has entry r0, entry r1 and two unknown candidates (`bl` at 0x47a88).
  Memory-configuration encoding and final r0 remain unknown (`ldmdb` at 0x47a8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47a24 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47a28 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47a2c | load | 4 | `abs 0x47a90` | `-` |
| `ldr` at 0x47a30 | load | 4 | `abs 0x148060` | `-` |
| `str` at 0x47a34 | store | 4 | `sp -0x1c` | `[0x148060]` |
| `ldr` at 0x47a38 | load | 4 | `abs 0x47a94` | `-` |
| `ldrb` at 0x47a3c | load | 1 | `abs 0x148064` | `-` |
| `str` at 0x47a40 | store | 4 | `sp -0x20` | `[0x148064]:1` |
| `ldr` at 0x47a44 | load | 4 | `abs 0x47a90` | `-` |
| `ldr` at 0x47a48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x47a4c | store | 4 | `abs 0x148060` | `r0` |
| `ldr` at 0x47a50 | load | 4 | `abs 0x47a94` | `-` |
| `ldrb` at 0x47a54 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x47a58 | store | 1 | `abs 0x148064` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x47a5c | load | 4 | `abs 0x47a90` | `-` |
| `ldr` at 0x47a60 | load | 4 | `abs 0x148060` | `-` |
| `ldr` at 0x47a64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47a70 | load | 4 | `abs 0x47a94` | `-` |
| `ldrb` at 0x47a74 | load | 1 | `abs 0x148064` | `-` |
| `ldr` at 0x47a78 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x47a8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47a8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47a8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47a88 | `(unknown callee) 0x47f30` | `r0` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47a6c | `[0x148060] != [sp-28]` | `[0x148060] == [sp-28]` |
| `bne` at 0x47a80 | `[0x148064]:1 != [sp-32]` | `[0x148064]:1 == [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47a8c | `r0` | `?` |

## `SHUB_NICfg_Override`

- `SHUB_NICfg_Override`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47b80 to 0x47bcc.
  Established. The indexed word read has unknown effective target (`ldr` at 0x47bbc). No calls or
  deciding branches are reported, and r0 at exit is unresolved (`ldmdb` at 0x47bc4). Override-table
  dimensions and entries cannot be assigned from the current facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47b90 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47b94 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47b98 | load | 4 | `abs 0x47bc8` | `-` |
| `ldr` at 0x47b9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47bb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47bbc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x47bc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47bc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47bc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47bc4 | `r0` | `?` |

## `SHUB_Get_NICfg`

- `SHUB_Get_NICfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47bcc to 0x47c24.
  Established. A halfword read uses an unknown effective address (`ldrh` at 0x47c14); its result
  does not resolve r0 at exit (`ldmdb` at 0x47c1c). Calls and conditional branches are absent from
  the tool facts. NI configuration bit meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47bdc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47be0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47be4 | load | 4 | `abs 0x47c20` | `-` |
| `ldr` at 0x47be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47bfc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x47c14 | load | 2 | `(unknown)` | `-` |
| `ldmdb` at 0x47c1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47c1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47c1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47c1c | `r0` | `?` |

## `SHUB_Set_NICfg`

- `SHUB_Set_NICfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47c24 to 0x47d20.
  Established. Halfword stores at `[sp-36] +0x0` have unknown values in two arms and `([sp-26]:2 &
  0xffff)` in another (`strh` at 0x47cc4, `strh` at 0x47ce4, `strh` at 0x47cf4). The word
  comparison takes its branch for `[sp-32] == 0x2` and falls through otherwise (`beq` at 0x47c80). A
  later word store writes `0x1` at an unknown effective address (`str` at 0x47d10). Entry r0 reaches
  the exit (`ldmdb` at 0x47d14). Bit-field meanings and indexed table geometry remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47c34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47c38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47c3c | store | 4 | `sp -0x20` | `r3` |
| `strh` at 0x47c40 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldr` at 0x47c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47c58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47c64 | load | 4 | `abs 0x47d18` | `-` |
| `str` at 0x47c6c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x47c70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47c74 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x47c78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47c84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47c90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47ca0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47cb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x47cb4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x47cb8 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldrh` at 0x47cbc | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x47cc4 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x47ccc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x47cd0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x47cd4 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0x47cdc | load | 2 | `[sp-36] +0x0` | `-` |
| `strh` at 0x47ce4 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x47cec | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x47cf0 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x47cf4 | store | 2 | `[sp-36] +0x0` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0x47cf8 | load | 4 | `abs 0x47d1c` | `-` |
| `ldr` at 0x47cfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47d04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x47d10 | store | 4 | `(unknown)` | `0x1` |
| `ldmdb` at 0x47d14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47d14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47d14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47c80 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x47c8c | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x47c98 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x47ca8 | `[sp-32] == 0x3` | `[sp-32] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47d14 | `r0` | `r0` |

## `SHUB_Set_MCI`

- `SHUB_Set_MCI`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47d20 to 0x47e08.
  Established. The absolute word store at `0x148074` carries `0x1` (`str` at 0x47d4c). Subsequent
  stores to unknown targets write a byte `([sp-17]:1 & 0xff)`, a halfword `([sp-20]:2 & 0xffff)` and
  a byte `([sp-21]:1 & 0xff)` (`strb` at 0x47d90, `strh` at 0x47dbc, `strb` at 0x47de8). The loop
  branch retains a `le flags condition (operands unknown)` predicate and its negation for
  fall-through (`ble` at 0x47d60). MCI settings and the exit r0 are unresolved (`ldmdb` at 0x47dfc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x47d34 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x47d38 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x47d40 | store | 1 | `sp -0x15` | `(r2 & 0xff)` |
| `ldr` at 0x47d44 | load | 4 | `abs 0x47e00` | `-` |
| `str` at 0x47d4c | store | 4 | `abs 0x148074` | `0x1` |
| `str` at 0x47d54 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x47d58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47d68 | load | 4 | `abs 0x47e04` | `-` |
| `ldr` at 0x47d6c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x47d8c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x47d90 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x47d94 | load | 4 | `abs 0x47e04` | `-` |
| `ldr` at 0x47d98 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x47db8 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x47dbc | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x47dc0 | load | 4 | `abs 0x47e04` | `-` |
| `ldr` at 0x47dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x47de4 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x47de8 | store | 1 | `(unknown)` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x47dec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x47df4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x47dfc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47dfc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47dfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x47d60 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47dfc | `r0` | `?` |

## `SHUB_Set_WRT90`

- `SHUB_Set_WRT90`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47e28 to 0x47f10.
  Established. A word store sets absolute `0x148078` to `0x1` (`str` at 0x47e54). Two byte stores at
  unknown targets carry `([sp-17]:1 & 0xff)` and `([sp-18]:1 & 0xff)` (`strb` at 0x47e98, `strb` at
  0x47ec4). A later unknown-target halfword store writes `([sp-20]:2 & 0xffff)` (`strh` at 0x47ef0).
  No calls appear in the facts. WRT90 encoding and r0 at exit remain unknown (`ldmdb` at 0x47f04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x47e3c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x47e44 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x47e48 | store | 2 | `sp -0x14` | `(r2 & 0xffff)` |
| `ldr` at 0x47e4c | load | 4 | `abs 0x47f08` | `-` |
| `str` at 0x47e54 | store | 4 | `abs 0x148078` | `0x1` |
| `str` at 0x47e5c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x47e60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47e70 | load | 4 | `abs 0x47f0c` | `-` |
| `ldr` at 0x47e74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x47e94 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x47e98 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x47e9c | load | 4 | `abs 0x47f0c` | `-` |
| `ldr` at 0x47ea0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x47ec0 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x47ec4 | store | 1 | `(unknown)` | `([sp-18]:1 & 0xff)` |
| `ldr` at 0x47ec8 | load | 4 | `abs 0x47f0c` | `-` |
| `ldr` at 0x47ecc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x47eec | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x47ef0 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x47ef4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x47efc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x47f04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47f04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47f04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x47e68 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47f04 | `r0` | `?` |

## `SHUB_Set_FSBCfg`

- `SHUB_Set_FSBCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x480c4 to 0x48278.
  Established. The word store at absolute `0x14807c` writes `0x1` (`str` at 0x48124). Byte stores
  include unknown-target writes of `([sp-25]:1 & 0xff)` and `([sp-26]:1 & 0xff)` (`strb` at 0x48160,
  `strb` at 0x48184). The loop predicate is only `le flags condition (operands unknown)` for taken
  and its negation for fall-through (`ble` at 0x48138). FSB field meanings and final r0 are
  unestablished (`ldmdb` at 0x4826c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x480d8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x480dc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x480e0 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x480e8 | store | 1 | `sp -0x19` | `(r0 & 0xff)` |
| `strb` at 0x480f0 | store | 1 | `sp -0x1a` | `(r1 & 0xff)` |
| `strb` at 0x480f8 | store | 1 | `sp -0x1b` | `(r2 & 0xff)` |
| `strb` at 0x48100 | store | 1 | `sp -0x1c` | `(r3 & 0xff)` |
| `strb` at 0x48108 | store | 1 | `sp -0x1d` | `([sp] & 0xff)` |
| `strb` at 0x48110 | store | 1 | `sp -0x1e` | `([sp+4] & 0xff)` |
| `strb` at 0x48118 | store | 1 | `sp -0x1f` | `([sp+8] & 0xff)` |
| `ldr` at 0x4811c | load | 4 | `abs 0x48270` | `-` |
| `str` at 0x48124 | store | 4 | `abs 0x14807c` | `0x1` |
| `str` at 0x4812c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x48130 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48140 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x48144 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x4815c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x48160 | store | 1 | `(unknown)` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0x48164 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x48168 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48180 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x48184 | store | 1 | `(unknown)` | `([sp-26]:1 & 0xff)` |
| `ldr` at 0x48188 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x4818c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x481ac | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x481b0 | store | 1 | `(unknown)` | `([sp-27]:1 & 0xff)` |
| `ldr` at 0x481b4 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x481b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x481d8 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x481dc | store | 1 | `(unknown)` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x481e0 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x481e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48204 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x48208 | store | 1 | `(unknown)` | `([sp-29]:1 & 0xff)` |
| `ldr` at 0x4820c | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x48210 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48230 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x48234 | store | 1 | `(unknown)` | `([sp-30]:1 & 0xff)` |
| `ldr` at 0x48238 | load | 4 | `abs 0x48274` | `-` |
| `ldr` at 0x4823c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48254 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x48258 | store | 1 | `(unknown)` | `([sp-31]:1 & 0xff)` |
| `ldr` at 0x4825c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x48264 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x4826c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4826c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x4826c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4826c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4826c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x48138 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4826c | `r0` | `?` |

## `psicInit`

- `psicInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb6a4 to 0xdb8c4.
  Established. Word stores write `0x1` at absolute `0x142b20` and `ret(0x1b3c4)` at `0x142b1c`
  (`str` at 0xdb72c, `str` at 0xdb760). The initial comparison takes its bypass for `[0x142b1c] ==
  0x2ad23c` and falls through on inequality (`beq` at 0xdb6d0). The first call to `0x21e28` uses
  `0x9`, `0x134fa0`, `0x58`, `0x0` (`bl` at 0xdb710). The exit carries `ret(0x1b43c)` (`ldmdb` at
  0xdb89c). Port-record fields and initialization ownership remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb6b4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdb6b8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdb6bc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xdb6c0 | load | 4 | `abs 0xdb8a0` | `-` |
| `ldr` at 0xdb6c4 | load | 4 | `abs 0x142b1c` | `-` |
| `ldr` at 0xdb6c8 | load | 4 | `abs 0xdb8a4` | `-` |
| `ldr` at 0xdb6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb6e0 | load | 4 | `abs 0xdb8a8` | `-` |
| `str` at 0xdb6e4 | store | 4 | `sp -0x34` | `0x134f70` |
| `ldr` at 0xdb6e8 | load | 4 | `abs 0xdb8a0` | `-` |
| `ldr` at 0xdb6ec | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdb6f0 | store | 4 | `sp -0x30` | `[0x142b1c]` |
| `ldr` at 0xdb6f4 | load | 4 | `abs 0xdb8ac` | `-` |
| `ldr` at 0xdb6f8 | load | 4 | `abs 0x142b20` | `-` |
| `str` at 0xdb6fc | store | 4 | `sp -0x2c` | `[0x142b20]` |
| `ldr` at 0xdb704 | load | 4 | `abs 0xdb8b0` | `-` |
| `ldr` at 0xdb714 | load | 4 | `abs 0xdb8ac` | `-` |
| `ldr` at 0xdb718 | load | 4 | `abs 0x142b20` | `-` |
| `ldr` at 0xdb724 | load | 4 | `abs 0xdb8ac` | `-` |
| `str` at 0xdb72c | store | 4 | `abs 0x142b20` | `0x1` |
| `ldr` at 0xdb730 | load | 4 | `abs 0xdb8ac` | `-` |
| `ldr` at 0xdb734 | load | 4 | `abs 0x142b20` | `-` |
| `str` at 0xdb748 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdb74c | load | 4 | `abs 0xdb8a0` | `-` |
| `ldr` at 0xdb750 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdb760 | store | 4 | `abs 0x142b1c` | `ret(0x1b3c4)` |
| `ldr` at 0xdb764 | load | 4 | `abs 0x142b1c` | `-` |
| `ldr` at 0xdb770 | load | 4 | `abs 0xdb8b4` | `-` |
| `str` at 0xdb774 | store | 4 | `sp -0x34` | `0x134fa8` |
| `ldr` at 0xdb778 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdb77c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xdb784 | load | 4 | `abs 0xdb8b0` | `-` |
| `ldr` at 0xdb794 | load | 4 | `abs 0xdb8a0` | `-` |
| `ldr` at 0xdb798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb7ac | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdb7b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdb7b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb7bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdb7c0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xdb7c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb7c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb7cc | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xdb7d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb7d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb7d8 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xdb7e8 | store | 4 | `[sp-36] +0x8` | `ret(0x1b3c4)` |
| `ldr` at 0xdb7ec | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xdb7f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb800 | load | 4 | `abs 0xdb8b8` | `-` |
| `ldr` at 0xdb804 | load | 4 | `abs 0xdb8bc` | `-` |
| `str` at 0xdb830 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdb834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb83c | load | 4 | `abs 0xdb8b8` | `-` |
| `ldr` at 0xdb840 | load | 4 | `abs 0xdb8bc` | `-` |
| `ldr` at 0xdb84c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdb85c | load | 4 | `abs 0xdb8c0` | `-` |
| `str` at 0xdb860 | store | 4 | `(unknown)` | `0x134fec` |
| `str` at 0xdb864 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xdb868 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb86c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xdb874 | load | 4 | `abs 0xdb8b0` | `-` |
| `ldr` at 0xdb884 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb888 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdb88c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xdb894 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0xdb89c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdb89c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb89c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb89c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb710 | `(unknown callee) 0x21e28` | `0x9` | `0x134fa0` | `0x58` | `0x0` |
| `bl` at 0xdb758 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `[0x142b20]` | `?` |
| `bl` at 0xdb790 | `(unknown callee) 0x21e28` | `0x9` | `0x134fa0` | `0x64` | `0x0` |
| `bl` at 0xdb7e0 | `(unknown callee) 0x1b3c4` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xdb808 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134fe0` | `0x134fe8` |
| `bl` at 0xdb814 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xdb844 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134fe0` | `0x134fe8` |
| `bl` at 0xdb854 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xdb880 | `(unknown callee) 0x21e28` | `0x9` | `0x134fa0` | `0x70` | `0x0` |
| `bl` at 0xdb898 | `(unknown callee) 0x1b43c` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdb6d0 | `[0x142b1c] == 0x2ad23c` | `[0x142b1c] != 0x2ad23c` |
| `bne` at 0xdb6dc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xdb720 | `[0x142b20] != 0x0` | `[0x142b20] == 0x0` |
| `bne` at 0xdb76c | `[0x142b1c] != 0x0` | `[0x142b1c] == 0x0` |
| `bne` at 0xdb7f4 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb89c | `r0` | `ret(0x1b43c)` |

## `psicWrite`

- `psicWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdba90 to 0xdbb1c.
  Established. The call to `0xdbed8` receives unknown, `[r1+3]:1`, `0xa`, `(r2 & 0xff)` (`bl` at
  0xdbb08); its return expression reaches r0 at exit (`ldmdb` at 0xdbb14). No conditional branches
  are reported. The meaning of `0xa` and the target device are not identified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbaa0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdbaa4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdbaac | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xdbab0 | load | 4 | `abs 0xdbb18` | `-` |
| `ldr` at 0xdbab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbac8 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdbad0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbad4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbad8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbadc | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdbae4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdbaec | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbaf0 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xdbaf4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdbaf8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xdbb14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbb14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbb08 | `(unknown callee) 0xdbed8` | `?` | `[r1+3]:1` | `0xa` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbb14 | `r0` | `ret(0xdbed8)` |

## `psicRead`

- `psicRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbb1c to 0xdbba0.
  Established. The sole call to `0xdc0a0` carries unknown, `[r1+3]:1`, `0xa`, entry r2 (`bl` at
  0xdbb8c). R0 at exit is `ret(0xdc0a0)` (`ldmdb` at 0xdbb98). Read width, target-device identity
  and the pointee format remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbb2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdbb30 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdbb34 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdbb38 | load | 4 | `abs 0xdbb9c` | `-` |
| `ldr` at 0xdbb3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbb50 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdbb58 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbb5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbb60 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbb64 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdbb6c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdbb74 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbb78 | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xdbb7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdbb88 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdbb98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbb98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbb98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbb8c | `(unknown callee) 0xdc0a0` | `?` | `[r1+3]:1` | `0xa` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbb98 | `r0` | `ret(0xdc0a0)` |

## `psicIntSet`

- `psicIntSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbba0 to 0xdbc2c.
  Established. The call to `0xdbed8` uses unknown, `[r1+3]:1`, `0xc`, `(r2 & 0xff)` (`bl` at
  0xdbc18). The exit propagates `ret(0xdbed8)` (`ldmdb` at 0xdbc24). The constant `0xc` and the
  call candidates have no established physical meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbbb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdbbb4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdbbbc | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xdbbc0 | load | 4 | `abs 0xdbc28` | `-` |
| `ldr` at 0xdbbc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbbd8 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdbbe0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbbe4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbbe8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbbec | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdbbf4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdbbfc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbc00 | load | 1 | `r1 +0x3` | `-` |
| `ldrb` at 0xdbc04 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdbc08 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xdbc24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbc24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbc24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbc18 | `(unknown callee) 0xdbed8` | `?` | `[r1+3]:1` | `0xc` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbc24 | `r0` | `ret(0xdbed8)` |

## `psicIntGet`

- `psicIntGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbc2c to 0xdbcb0.
  Established. The call to `0xdc0a0` has unknown, `[r1+3]:1`, `0xc`, entry r2 (`bl` at 0xdbc9c). The
  return reports `ret(0xdc0a0)` (`ldmdb` at 0xdbca8). Interrupt status bits and the output location
  remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbc3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdbc40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdbc44 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xdbc48 | load | 4 | `abs 0xdbcac` | `-` |
| `ldr` at 0xdbc4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdbc60 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdbc68 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbc6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbc70 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbc74 | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdbc7c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdbc84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdbc88 | load | 1 | `r1 +0x3` | `-` |
| `ldr` at 0xdbc8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdbc98 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdbca8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbca8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbca8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbc9c | `(unknown callee) 0xdc0a0` | `?` | `[r1+3]:1` | `0xc` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbca8 | `r0` | `ret(0xdc0a0)` |

## `psicInt`

- `psicInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbcb0 to 0xdbed8.
  Established. The call to `0xdc0a0` carries `[sp-44]`, unknown, `0xd`, `sp-57` (`bl` at 0xdbd50);
  its zero result takes the continuation and nonzero falls through (`beq` at 0xdbd64). The earlier
  loop branch exposes only a `le flags condition (operands unknown)` predicate and its negation
  (`ble` at 0xdbd2c). The common exit reports `[sp-52]` (`ldmdb` at 0xdbebc). Interrupt-cause
  interpretation and retry units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbcc0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdbcc4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xdbcc8 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xdbccc | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xdbcd0 | load | 4 | `abs 0xdbec0` | `-` |
| `ldr` at 0xdbcd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbce8 | load | 4 | `abs 0x142b1c` | `-` |
| `str` at 0xdbcf0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdbcf4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdbcf8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdbcfc | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdbd04 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xdbd0c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdbd10 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xdbd18 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xdbd20 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xdbd24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdbd34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdbd38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdbd40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdbd58 | store | 4 | `sp -0x30` | `ret(0xdc0a0)` |
| `ldr` at 0xdbd5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdbd68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbd70 | load | 4 | `abs 0xdbec4` | `-` |
| `ldr` at 0xdbd74 | load | 4 | `abs 0xdbec8` | `-` |
| `str` at 0xdbda0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xdbda4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbdac | load | 4 | `abs 0xdbec4` | `-` |
| `ldr` at 0xdbdb0 | load | 4 | `abs 0xdbec8` | `-` |
| `ldr` at 0xdbdbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdbdcc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xdbdd8 | load | 4 | `abs 0xdbecc` | `-` |
| `str` at 0xdbddc | store | 4 | `(unknown)` | `0x135028` |
| `str` at 0xdbde0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xdbde4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbde8 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xdbdec | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0xdbdf0 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0xdbdf8 | load | 4 | `abs 0xdbed0` | `-` |
| `ldr` at 0xdbe08 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdbe0c | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdbe14 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xdbe18 | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xdbe1c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0xdbe30 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xdbe34 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xdbe38 | store | 4 | `sp -0x54` | `[sp-57]:1` |
| `ldr` at 0xdbe40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbe44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbe54 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdbe5c | store | 4 | `sp -0x34` | `([sp-52] + ret(0xd7168))` |
| `ldr` at 0xdbe64 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdbe68 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdbe70 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdbe78 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xdbe80 | load | 4 | `abs 0xdbed4` | `-` |
| `str` at 0xdbe84 | store | 4 | `sp -0x54` | `0x135050` |
| `ldr` at 0xdbe88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbe8c | load | 4 | `(unknown)` | `-` |
| `str` at 0xdbe90 | store | 4 | `sp -0x50` | `?` |
| `ldrb` at 0xdbe94 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xdbe98 | store | 4 | `sp -0x4c` | `[sp-57]:1` |
| `ldr` at 0xdbea0 | load | 4 | `abs 0xdbed0` | `-` |
| `ldr` at 0xdbeb0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdbeb4 | store | 4 | `sp -0x44` | `[sp-52]` |
| `ldr` at 0xdbeb8 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xdbebc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xdbebc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdbebc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdbebc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbd50 | `(unknown callee) 0xdc0a0` | `[sp-44]` | `?` | `0xd` | `sp-57` |
| `bl` at 0xdbd78 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134fe0` | `0x134fe8` |
| `bl` at 0xdbd84 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xdbdb4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134fe0` | `0x134fe8` |
| `bl` at 0xdbdc4 | `(unknown callee) 0x1b508` | `[sp-64]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xdbdd0 | `(unknown callee) 0x219a8` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0xdbe04 | `(unknown callee) 0x21e28` | `0x3` | `0x134fa0` | `0xe0` | `0x0` |
| `bl` at 0xdbe4c | `(unknown callee) 0xd7168` | `0x0` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0xdbeac | `(unknown callee) 0x21e28` | `0x3` | `0x134fa0` | `0xeb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xdbd2c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xdbd64 | `ret(0xdc0a0) == 0x0` | `ret(0xdc0a0) != 0x0` |
| `beq` at 0xdbe2c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdbebc | `r0` | `[sp-52]` |
