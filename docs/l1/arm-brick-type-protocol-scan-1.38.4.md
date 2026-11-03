# ARM brick types, protocol reset and scan helpers in 1.38.4

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
exact tool notation. A `:1` suffix marks a byte load. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed branches in the brick-type helpers remain unresolved. Tables and return values cover only known edges, with excluded sites listed under the affected entries. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/brick-type-protocol-scan-facts.txt) retain all tool sections.

## `semReset`

- `semReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22ea4 to 0x22f84.
  Established. Byte stores write `([sp-24]:1 & 0xff)` at `[sp-20] +0x50` and `0x1`
  at `[sp-20] +0x51` (`strb` at 0x22ef8, `strb` at 0x22f04). The opening call to `0x1b1f0` has `r0+8`, entry r1,
  entry r2, `r0+8` (`bl` at 0x22ec8); its zero result takes the continuation and nonzero falls
  through (`beq` at 0x22edc). Synchronization semantics and the common return value remain unknown
  (`ldmdb` at 0x22f74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22eb4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x22eb8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x22ebc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22ed0 | store | 4 | `sp -0x1c` | `ret(0x1b1f0)` |
| `ldr` at 0x22ed4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22ee0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22ee4 | store | 4 | `sp -0x20` | `ret(0x1b1f0)` |
| `ldr` at 0x22ef0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x22ef4 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x22ef8 | store | 1 | `[sp-20] +0x50` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x22efc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x22f04 | store | 1 | `[sp-20] +0x51` | `0x1` |
| `ldr` at 0x22f08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22f1c | store | 4 | `sp -0x1c` | `ret(0x1b298)` |
| `ldr` at 0x22f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x22f2c | load | 4 | `abs 0x22f78` | `-` |
| `str` at 0x22f30 | store | 4 | `sp -0x28` | `0xfe8b8` |
| `ldr` at 0x22f34 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22f38 | store | 4 | `sp -0x24` | `ret(0x1b298)` |
| `ldr` at 0x22f40 | load | 4 | `abs 0x22f7c` | `-` |
| `ldr` at 0x22f44 | load | 4 | `abs 0x22f80` | `-` |
| `ldr` at 0x22f50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x22f64 | store | 4 | `sp -0x1c` | `ret(0x1b46c)` |
| `ldr` at 0x22f68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x22f6c | store | 4 | `sp -0x20` | `ret(0x1b46c)` |
| `ldr` at 0x22f70 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x22f74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x22f74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x22f74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22ec8 | `(unknown callee) 0x1b1f0` | `r0+8` | `r1` | `r2` | `r0+8` |
| `bl` at 0x22eec | `(unknown callee) 0x1b280` | `ret(0x1b1f0)` | `?` | `?` | `ret(0x1b1f0)` |
| `bl` at 0x22f14 | `(unknown callee) 0x1b298` | `[sp-20]+32` | `?` | `[sp-20]` | `[sp-20]+32` |
| `bl` at 0x22f4c | `(unknown callee) 0x21e28` | `0x9` | `0xfe7f8` | `0x1d5` | `0x0` |
| `bl` at 0x22f5c | `(unknown callee) 0x1b46c` | `[sp-20]+8` | `?` | `?` | `[sp-20]+8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x22edc | `ret(0x1b1f0) == 0x0` | `ret(0x1b1f0) != 0x0` |
| `beq` at 0x22f28 | `ret(0x1b298) == 0x0` | `ret(0x1b298) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x22f74 | `r0` | `?` |

## `UtilDspBrick`

- `UtilDspBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69af8 to 0x69d3c.
  Established. The call to `0x69d3c` carries `sp-36`, `[ret(0x74908)+8]`, `[ret(0x74908)+16]`,
  `[ret(0x74908)+12]` (`bl` at 0x69b38). The computed branch takes its unresolved edge for unsigned
  `[[sp-40]+16]-35 <= 0x59` and falls through for greater (`ldrls` at 0x69b4c). The known-edge exit
  carries `ret(0x51580)` (`ldmdb` at 0x69d30). Physical display and brick-label interpretation
  remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69b14 | store | 4 | `sp -0x28` | `ret(0x74908)` |
| `ldr` at 0x69b1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69b20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69b24 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69b2c | load | 4 | `ret(0x74908) +0x8` | `-` |
| `ldr` at 0x69b30 | load | 4 | `ret(0x74908) +0x10` | `-` |
| `ldr` at 0x69b34 | load | 4 | `ret(0x74908) +0xc` | `-` |
| `ldr` at 0x69b3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x69b40 | load | 4 | `[sp-40] +0x10` | `-` |
| `ldrls` at 0x69b4c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x69d30 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x69d30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69d30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69d30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69b0c | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x69b38 | `(unknown callee) 0x69d3c` | `sp-36` | `[ret(0x74908)+8]` | `[ret(0x74908)+16]` | `[ret(0x74908)+12]` |
| `bl` at 0x69d2c | `(unknown callee) 0x51580` | `0x31` | `sp-36` | `0x0` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x69b4c | `[[sp-40]+16]-35 <= 0x59 (unsigned, 32-bit)` | `[[sp-40]+16]-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69d30 | `r0` | `ret(0x51580)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x69b4c).

## `UtilFmtBrick`

- `UtilFmtBrick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69d3c to 0x69dd0.
  Established. The call to `0xc4f78` has entry r2, entry r1, entry r2, entry r3 (`bl` at 0x69d60).
  The next call reaches `0x1b49c` with `[sp-20]`, `0x10efec`, `[sp-24]`, unknown (`bl` at 0x69d84).
  The first guard takes its bypass for `[sp-24] != 0x0` and falls through on zero (`bne` at
  0x69d90). Format fields and final r0 are unknown (`ldmdb` at 0x69dc8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69d4c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69d50 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x69d54 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x69d58 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x69d5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69d6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x69d70 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x69d74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69d78 | load | 4 | `abs 0x69dcc` | `-` |
| `ldr` at 0x69d7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69d88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69d94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69da4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69db0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x69dc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x69dc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x69dc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69d60 | `(unknown callee) 0xc4f78` | `r2` | `r1` | `r2` | `r3` |
| `bl` at 0x69d84 | `(unknown callee) 0x1b49c` | `[sp-20]` | `0x10efec` | `[sp-24]` | `?` |
| `bl` at 0x69da0 | `(unknown callee) 0x1b43c` | `[sp-20]` | `0x2d` | `0x3` | `[sp-24]` |
| `bl` at 0x69dc4 | `(unknown callee) 0x1b43c` | `[sp-20]+4` | `0x2d` | `0x2` | `[sp-20]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x69d90 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x69dac | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x69dc8 | `r0` | `?` |

## `SMPCmdCheckBrickType`

- `SMPCmdCheckBrickType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d2c8 to 0x8d5dc.
  Established. Word stores clear `[sp-28] +0x0` and later write `ret(0x1b124)` at the same
  expression (`str` at 0x8d594, `str` at 0x8d5a8). The initial call to `0x4f090` carries entry r0
  through r3 (`bl` at 0x8d2e0). The entry-r0 value saved at `[sp-24]` takes the continuation for
  nonzero and falls through on zero (`bne` at 0x8d310). Type-selection arms behind the unresolved
  branch and r0 at exit remain unknown (`ldmdb` at 0x8d5d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d2d8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8d2dc | store | 4 | `sp -0x1c` | `r1` |
| `strb` at 0x8d2e4 | store | 1 | `sp -0x1d` | `(ret(0x4f090) & 0xff)` |
| `strb` at 0x8d2ec | store | 1 | `sp -0x1e` | `0x0` |
| `str` at 0x8d2f4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x8d2fc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x8d304 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x8d308 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d318 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8d320 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x8d324 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x8d334 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8d338 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d340 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrb` at 0x8d344 | load | 1 | `sp -0x1d` | `-` |
| `ldrls` at 0x8d350 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x8d4e8 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x8d4ec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x8d4f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d50c | store | 4 | `sp -0x24` | `?` |
| `str` at 0x8d514 | store | 4 | `sp -0x34` | `0x0` |
| `ldrb` at 0x8d518 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x8d524 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x8d528 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d544 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x8d548 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8d54c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x8d550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8d554 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d560 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8d564 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8d570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d574 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8d580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d584 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8d58c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8d594 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x8d598 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8d5a8 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x8d5ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d5b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8d5b4 | load | 4 | `abs 0x8d5d8` | `-` |
| `str` at 0x8d5c0 | store | 4 | `sp -0x30` | `?` |
| `str` at 0x8d5cc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x8d5d0 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x8d5d4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8d5d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8d5d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8d5d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d2e0 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x8d4f8 | `(unknown callee) 0x1af8c` | `[sp-24]` | `(ret(0x4f090) & 0xff)` | `?` | `(ret(0x4f090) & 0xff)` |
| `bl` at 0x8d530 | `(unknown callee) 0x1af8c` | `[sp-24]` | `[sp-30]:1` | `?` | `[sp-30]:1` |
| `bl` at 0x8d588 | `(unknown callee) 0x1b400` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |
| `bl` at 0x8d5a0 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x8d5b8 | `(unknown callee) 0x1b508` | `[[sp-28]]` | `0x11a874` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8d310 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x8d32c | `[[sp-24]]:1 != 0x7e` | `[[sp-24]]:1 == 0x7e` |
| `ldrls` at 0x8d350 | `(ret(0x4f090) & 0xff)-35 <= 0x59 (unsigned, 32-bit)` | `(ret(0x4f090) & 0xff)-35 > 0x59 (unsigned, 32-bit)` |
| `beq` at 0x8d520 | `[sp-30]:1 == 0x0` | `[sp-30]:1 != 0x0` |
| `beq` at 0x8d53c | `ret(0x1af8c) == 0x0` | `ret(0x1af8c) != 0x0` |
| `beq` at 0x8d55c | `[sp-36] == [sp-44]` | `[sp-36] != [sp-44]` |
| `beq` at 0x8d56c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8d57c | `[[sp-28]] == 0x0` | `[[sp-28]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8d5d4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8d350).

## `L1FRU_2m_EnvStatus`

- `L1FRU_2m_EnvStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa8168 to 0xa8188.
  Established. The sole call to `0x81c98` carries `0x2b1b04`, `0x1`, entry r2, entry r3 (`bl` at
  0xa817c). The exit reports `ret(0x81c98)` (`ldmdb` at 0xa8180). Environmental-status format and
  destination ownership remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa8174 | load | 4 | `abs 0xa8184` | `-` |
| `ldmdb` at 0xa8180 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa8180 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa8180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa817c | `(unknown callee) 0x81c98` | `0x2b1b04` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa8180 | `r0` | `ret(0x81c98)` |

## `PPPReset`

- `PPPReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba86c to 0xba8cc.
  Established. Word stores clear `r0 +0xc`, `[sp-20] +0x30` and an unknown effective target (`str`
  at 0xba89c, `str` at 0xba8a8, `str` at 0xba8c4). Both deciding branches retain unknown operands
  with zero for taken and nonzero for fall-through (`beq` at 0xba890, `beq` at 0xba8b8). Entry r0
  survives as the exit expression (`ldmdb` at 0xba8c8). Protocol field meanings cannot be assigned
  from these facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba87c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xba880 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xba884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xba894 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xba89c | store | 4 | `r0 +0xc` | `0x0` |
| `ldr` at 0xba8a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xba8a8 | store | 4 | `[sp-20] +0x30` | `0x0` |
| `ldr` at 0xba8ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xba8bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xba8c4 | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0xba8c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xba8c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xba8c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xba890 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xba8b8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xba8c8 | `r0` | `r0` |

## `brickConvertString`

- `brickConvertString`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4760 to 0xc4b30.
  Established. The opening call to `0x1b22c` uses entry r0, r1, r2 and `0x3f` (`bl` at 0xc4780); its
  result takes the branch for inequality with `0x1` and falls through on equality (`bne` at
  0xc478c). The subsequent call to `0x1b2bc` has `[[sp-20]]:1`, unknown, unknown, `[[sp-20]]:1`
  (`bl` at 0xc479c). The known-edge exit reports `[sp-24]` (`ldmdb` at 0xc4aec). String/type
  mappings beyond the computed selector remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4770 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc4778 | store | 4 | `sp -0x18` | `0x3f` |
| `ldr` at 0xc477c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4790 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc4794 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldrls` at 0xc47ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc48a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc48ac | load | 4 | `abs 0xc4af0` | `-` |
| `str` at 0xc48c4 | store | 4 | `sp -0x18` | `0x43` |
| `ldr` at 0xc48cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc48d0 | load | 4 | `abs 0xc4af4` | `-` |
| `str` at 0xc48e8 | store | 4 | `sp -0x18` | `0x33` |
| `ldr` at 0xc48f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc48f4 | load | 4 | `abs 0xc4af8` | `-` |
| `str` at 0xc490c | store | 4 | `sp -0x18` | `0x34` |
| `ldr` at 0xc4914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4918 | load | 4 | `abs 0xc4afc` | `-` |
| `str` at 0xc4930 | store | 4 | `sp -0x18` | `0x35` |
| `ldr` at 0xc4938 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc493c | load | 4 | `abs 0xc4b00` | `-` |
| `str` at 0xc4954 | store | 4 | `sp -0x18` | `0x36` |
| `ldr` at 0xc495c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4960 | load | 4 | `abs 0xc4b04` | `-` |
| `str` at 0xc4978 | store | 4 | `sp -0x18` | `0x40` |
| `ldr` at 0xc4980 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4984 | load | 4 | `abs 0xc4b08` | `-` |
| `str` at 0xc499c | store | 4 | `sp -0x18` | `0x4f` |
| `ldr` at 0xc49a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc49a8 | load | 4 | `abs 0xc4b0c` | `-` |
| `str` at 0xc49c0 | store | 4 | `sp -0x18` | `0x42` |
| `ldr` at 0xc49c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc49cc | load | 4 | `abs 0xc4b10` | `-` |
| `str` at 0xc49e4 | store | 4 | `sp -0x18` | `0x4c` |
| `ldr` at 0xc49ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc49f0 | load | 4 | `abs 0xc4b14` | `-` |
| `str` at 0xc4a08 | store | 4 | `sp -0x18` | `0x5a` |
| `ldr` at 0xc4a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4a14 | load | 4 | `abs 0xc4b18` | `-` |
| `str` at 0xc4a2c | store | 4 | `sp -0x18` | `0x7c` |
| `ldr` at 0xc4a34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4a38 | load | 4 | `abs 0xc4b1c` | `-` |
| `str` at 0xc4a50 | store | 4 | `sp -0x18` | `0x44` |
| `ldr` at 0xc4a58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4a5c | load | 4 | `abs 0xc4b20` | `-` |
| `str` at 0xc4a74 | store | 4 | `sp -0x18` | `0x53` |
| `ldr` at 0xc4a7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4a80 | load | 4 | `abs 0xc4b24` | `-` |
| `str` at 0xc4a98 | store | 4 | `sp -0x18` | `0x31` |
| `ldr` at 0xc4aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4aa4 | load | 4 | `abs 0xc4b28` | `-` |
| `str` at 0xc4abc | store | 4 | `sp -0x18` | `0x32` |
| `ldr` at 0xc4ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ac8 | load | 4 | `abs 0xc4b2c` | `-` |
| `str` at 0xc4ae0 | store | 4 | `sp -0x18` | `0x54` |
| `ldr` at 0xc4ae4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc4aec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4aec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4aec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4780 | `(unknown callee) 0x1b22c` | `r0` | `r1` | `r2` | `0x3f` |
| `bl` at 0xc479c | `(unknown callee) 0x1b2bc` | `[[sp-20]]:1` | `?` | `?` | `[[sp-20]]:1` |
| `bl` at 0xc48b0 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b08` | `?` | `ret(0x1b22c)` |
| `bl` at 0xc48d4 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b10` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc48f8 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b18` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc491c | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b20` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4940 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b28` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4964 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b30` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4988 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b38` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc49ac | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b40` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc49d0 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b48` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc49f4 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b50` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4a18 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b58` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4a3c | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b60` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4a60 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b68` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4a84 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b70` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4aa8 | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b74` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc4acc | `(unknown callee) 0x1aff8` | `[sp-20]` | `0x132b78` | `?` | `ret(0x1aff8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc478c | `ret(0x1b22c) != 0x1` | `ret(0x1b22c) == 0x1` |
| `ldrls` at 0xc47ac | `ret(0x1b2bc)-35 <= 0x36 (unsigned, 32-bit)` | `ret(0x1b2bc)-35 > 0x36 (unsigned, 32-bit)` |
| `bne` at 0xc48bc | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc48e0 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4904 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4928 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc494c | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4970 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4994 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc49b8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc49dc | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4a00 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4a24 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4a48 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4a6c | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4a90 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4ab4 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |
| `bne` at 0xc4ad8 | `ret(0x1aff8) != 0x0` | `ret(0x1aff8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4aec | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc47ac).

## `brickConvertType`

- `brickConvertType`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4b30 to 0xc4f78.
  Established. The computed branch takes its unresolved edge for unsigned `r0-35 <= 0x59` and falls
  through for greater (`ldrls` at 0xc4b50). The known-edge call to `0x1b49c` has `0x149e14`,
  `0x132ce0`, entry r0, `r0-35` (`bl` at 0xc4edc). The recognized exit returns `0x149e14` on those
  edges (`ldmdb` at 0xc4ee8). Type descriptions in excluded arms are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4b40 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc4b44 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc4b50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4ed0 | load | 4 | `abs 0xc4eec` | `-` |
| `ldr` at 0xc4ed4 | load | 4 | `abs 0xc4f74` | `-` |
| `ldr` at 0xc4ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ee0 | load | 4 | `abs 0xc4eec` | `-` |
| `ldmdb` at 0xc4ee8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4ee8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4ee8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4edc | `(unknown callee) 0x1b49c` | `0x149e14` | `0x132ce0` | `r0` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc4b50 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4ee8 | `r0` | `0x149e14` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc4b50).

## `brickTypeToChar`

- `brickTypeToChar`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4f78 to 0xc52c4.
  Established. The initial selector takes an unresolved edge for unsigned `r0-35 <= 0x59` and falls
  through for greater (`ldrls` at 0xc4f98). The known-edge call to `0x1b0e8` carries entry r0, r1,
  r2, `r0-35` (`bl` at 0xc52b0); its final r0 expression is unresolved (`ldmdb` at 0xc52c0).
  Character mappings inside disconnected arms remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4f88 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc4f8c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc4f98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc52ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc52c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc52c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc52c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc52b0 | `(unknown callee) 0x1b0e8` | `r0` | `r1` | `r2` | `r0-35` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc4f98 | `r0-35 <= 0x59 (unsigned, 32-bit)` | `r0-35 > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc52c0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc4f98).

## `brickIsCompute`

- `brickIsCompute`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc52c4 to 0xc53dc.
  Established. The first equality branch takes its continuation for `r0 == 0x43` and falls through
  otherwise (`beq` at 0xc52e8). The next tests `r0 == 0x33` with the same directions (`beq` at
  0xc52f4). No callees are reported. The common exit r0 is unknown (`ldmdb` at 0xc53d8). Physical
  compute-brick membership is unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc52d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc52dc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc52e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc52f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc531c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc534c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc537c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc53a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc53ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc53b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc53cc | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xc53d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc53d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc53d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc53d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc52e8 | `r0 == 0x43` | `r0 != 0x43` |
| `beq` at 0xc52f4 | `r0 == 0x33` | `r0 != 0x33` |
| `beq` at 0xc5300 | `r0 == 0x35` | `r0 != 0x35` |
| `beq` at 0xc530c | `r0 == 0x34` | `r0 != 0x34` |
| `beq` at 0xc5318 | `r0 == 0x41` | `r0 != 0x41` |
| `beq` at 0xc5324 | `r0 == 0x36` | `r0 != 0x36` |
| `beq` at 0xc5330 | `r0 == 0x39` | `r0 != 0x39` |
| `beq` at 0xc533c | `r0 == 0x37` | `r0 != 0x37` |
| `beq` at 0xc5348 | `r0 == 0x38` | `r0 != 0x38` |
| `beq` at 0xc5354 | `r0 == 0x30` | `r0 != 0x30` |
| `beq` at 0xc5360 | `r0 == 0x51` | `r0 != 0x51` |
| `beq` at 0xc536c | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0xc5378 | `r0 == 0x4f` | `r0 != 0x4f` |
| `beq` at 0xc5384 | `r0 == 0x42` | `r0 != 0x42` |
| `beq` at 0xc5390 | `r0 == 0x46` | `r0 != 0x46` |
| `beq` at 0xc539c | `r0 == 0x48` | `r0 != 0x48` |
| `beq` at 0xc53a8 | `r0 == 0x4c` | `r0 != 0x4c` |
| `beq` at 0xc53b4 | `r0 == 0x4d` | `r0 != 0x4d` |
| `beq` at 0xc53c0 | `r0 == 0x7c` | `r0 != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc53d8 | `r0` | `?` |

## `scancmd_dump`

- `scancmd_dump`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc3bc to 0xdc96c.
  Established. The entry-r3 guard takes its continuation on nonzero and falls through on zero (`bne`
  at 0xdc3e4). The first call to `0x24028` has unknown, `0x1350c8`, entry r1, entry r1 (`bl` at
  0xdc42c). A separate arm calls the same target with unknown, `0x1350cc`, entry r2, entry r1 (`bl`
  at 0xdc43c). Command-field labels and the joined r0 value remain unresolved (`ldmdb` at 0xdc904).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc3cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc3d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xdc3d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xdc3d8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xdc3dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc3ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc3f0 | store | 4 | `sp -0x40` | `r0` |
| `ldr` at 0xdc3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc400 | load | 4 | `abs 0xdc908` | `-` |
| `ldr` at 0xdc404 | load | 4 | `abs 0x144dc4` | `-` |
| `str` at 0xdc408 | store | 4 | `sp -0x40` | `[0x144dc4]` |
| `ldr` at 0xdc40c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdc410 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xdc414 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc420 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc424 | load | 4 | `abs 0xdc90c` | `-` |
| `ldr` at 0xdc428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc438 | load | 4 | `abs 0xdc910` | `-` |
| `ldr` at 0xdc440 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc444 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc450 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc454 | load | 4 | `abs 0xdc914` | `-` |
| `ldr` at 0xdc464 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc46c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc47c | load | 4 | `abs 0xdc918` | `-` |
| `ldr` at 0xdc488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc48c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc49c | load | 4 | `abs 0xdc91c` | `-` |
| `ldr` at 0xdc4ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc4b4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc4c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc4c4 | load | 4 | `abs 0xdc920` | `-` |
| `ldr` at 0xdc4d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc4d4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc4e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc4e4 | load | 4 | `abs 0xdc924` | `-` |
| `ldr` at 0xdc4f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc4fc | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc50c | load | 4 | `abs 0xdc928` | `-` |
| `ldr` at 0xdc518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc51c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xdc524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc528 | load | 4 | `abs 0xdc92c` | `-` |
| `ldr` at 0xdc538 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc540 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc54c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc550 | load | 4 | `abs 0xdc930` | `-` |
| `ldr` at 0xdc560 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc568 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdc574 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc578 | load | 4 | `abs 0xdc934` | `-` |
| `ldr` at 0xdc588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc590 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc59c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5a0 | load | 4 | `abs 0xdc938` | `-` |
| `ldr` at 0xdc5b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc5b8 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdc5c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5c8 | load | 4 | `abs 0xdc93c` | `-` |
| `ldr` at 0xdc5d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc5e0 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc5ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc5f0 | load | 4 | `abs 0xdc940` | `-` |
| `ldr` at 0xdc600 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc608 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdc614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc618 | load | 4 | `abs 0xdc944` | `-` |
| `ldr` at 0xdc628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc630 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc63c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc640 | load | 4 | `abs 0xdc948` | `-` |
| `ldr` at 0xdc650 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc658 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xdc660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc664 | load | 4 | `abs 0xdc94c` | `-` |
| `ldr` at 0xdc674 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc67c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc68c | load | 4 | `abs 0xdc950` | `-` |
| `ldr` at 0xdc69c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc6a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc6a8 | load | 4 | `abs 0xdc954` | `-` |
| `ldr` at 0xdc6ac | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdc6b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc6c0 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xdc6c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc6cc | load | 4 | `abs 0xdc958` | `-` |
| `ldr` at 0xdc6dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc6e4 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdc6f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc700 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc708 | store | 4 | `sp -0x3c` | `[sp-32]+16` |
| `ldr` at 0xdc710 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc718 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xdc730 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc738 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0xdc744 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xdc748 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdc750 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc754 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdc75c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xdc760 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc764 | load | 4 | `abs 0xdc95c` | `-` |
| `str` at 0xdc770 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdc774 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc778 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc788 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc790 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc798 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc7a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc7b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc7bc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc7c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc7cc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xdc7d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc7dc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc7e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc7ec | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xdc7f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc7fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc808 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc80c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xdc810 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc81c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc828 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc82c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xdc830 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc83c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc848 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc84c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xdc850 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc85c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc868 | load | 4 | `(unknown)` | `-` |
| `str` at 0xdc86c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xdc870 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc874 | load | 4 | `abs 0xdc960` | `-` |
| `ldr` at 0xdc878 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdc87c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdc884 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc88c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xdc894 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdc8a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdc8a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc8ac | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdc8bc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc8c4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc8d0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc8d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc8dc | load | 4 | `abs 0xdc964` | `-` |
| `ldr` at 0xdc8e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xdc8e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc8f0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xdc8f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc8fc | load | 4 | `abs 0xdc968` | `-` |
| `ldmdb` at 0xdc904 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc904 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc904 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc42c | `(unknown callee) 0x24028` | `?` | `0x1350c8` | `r1` | `r1` |
| `bl` at 0xdc43c | `(unknown callee) 0x24028` | `?` | `0x1350cc` | `r2` | `r1` |
| `bl` at 0xdc45c | `(unknown callee) 0x24028` | `[sp-20]` | `0x1350e8` | `?` | `?` |
| `bl` at 0xdc484 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135104` | `?` | `?` |
| `bl` at 0xdc4a4 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135114` | `?` | `?` |
| `bl` at 0xdc4cc | `(unknown callee) 0x24028` | `[sp-20]` | `0x135130` | `?` | `?` |
| `bl` at 0xdc4ec | `(unknown callee) 0x24028` | `[sp-20]` | `0x135140` | `?` | `?` |
| `bl` at 0xdc514 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13515c` | `?` | `?` |
| `bl` at 0xdc530 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13516c` | `?` | `?` |
| `bl` at 0xdc558 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135188` | `?` | `?` |
| `bl` at 0xdc580 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135198` | `?` | `?` |
| `bl` at 0xdc5a8 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1351b4` | `?` | `?` |
| `bl` at 0xdc5d0 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1351c4` | `?` | `?` |
| `bl` at 0xdc5f8 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1351e0` | `?` | `?` |
| `bl` at 0xdc620 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1351f0` | `?` | `?` |
| `bl` at 0xdc648 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13520c` | `?` | `?` |
| `bl` at 0xdc66c | `(unknown callee) 0x24028` | `[sp-20]` | `0x13521c` | `?` | `?` |
| `bl` at 0xdc694 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135238` | `?` | `?` |
| `bl` at 0xdc6b0 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13524c` | `[[sp-32]+12]` | `[sp-32]+12` |
| `bl` at 0xdc6d4 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135264` | `?` | `?` |
| `bl` at 0xdc768 | `(unknown callee) 0x24028` | `[sp-20]` | `0x135278` | `?` | `?` |
| `bl` at 0xdc880 | `(unknown callee) 0x24028` | `[sp-20]` | `0x13529c` | `?` | `?` |
| `bl` at 0xdc8e4 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1352d0` | `?` | `?` |
| `bl` at 0xdc900 | `(unknown callee) 0x24028` | `[sp-20]` | `0x1352d8` | `?` | `[sp-56]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc3e4 | `r3 != 0x0` | `r3 == 0x0` |
| `bne` at 0xdc3fc | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xdc41c | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xdc6ec | `[[sp-32]+12] == 0x0` | `[[sp-32]+12] != 0x0` |
| `bne` at 0xdc6f8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blo` at 0xdc780 | `C == 0` | `not (C == 0)` |
| `beq` at 0xdc89c | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xdc8b4 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc904 | `r0` | `?` |

## `scan_multi_sic_ir`

- `scan_multi_sic_ir`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe12e8 to 0xe16c4.
  Established. Three word stores have unknown effective targets, carrying unknown, `0x0` and `0x0`,
  respectively (`str` at 0xe1340, `str` at 0xe1360, `str` at 0xe1380). The call to `0xe0e38` carries
  `sp-328`, unknown, unknown, `sp-136` (`bl` at 0xe13ac). The next call reaches `0xdcf54` with
  `0x0`, `[sp-32]`, `[sp]`, `[sp+4]` (`bl` at 0xe13e8); its nonzero result takes the continuation
  and zero falls through (`bne` at 0xe13f4). Scan lengths, expected-data fields and the merged
  return are unknown (`ldmdb` at 0xe16a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe12f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe12fc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe1300 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe1304 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xe130c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1310 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe1324 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1328 | load | 4 | `abs 0xe16ac` | `-` |
| `ldr` at 0xe133c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe1340 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe1344 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1360 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1364 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1380 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe1384 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe138c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xe13a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe13a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe13b4 | store | 4 | `sp -0x28` | `ret(0xe0e38)` |
| `ldr` at 0xe13b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe13bc | store | 4 | `sp -0x168` | `ret(0xe0e38)` |
| `str` at 0xe13c4 | store | 4 | `sp -0x164` | `sp-136` |
| `str` at 0xe13cc | store | 4 | `sp -0x160` | `sp-232` |
| `ldr` at 0xe13d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xe13d4 | store | 4 | `sp -0x15c` | `[sp+12]` |
| `ldr` at 0xe13dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe13e0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xe13e4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xe13fc | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xe1404 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1410 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1418 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0xe141c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe1424 | store | 4 | `sp -0x150` | `?` |
| `str` at 0xe142c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe1430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1434 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1444 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe145c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1464 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe146c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1470 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe147c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe1484 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe148c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe14a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe14a8 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xe14ac | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe14b0 | load | 4 | `abs 0xe16b0` | `-` |
| `ldr` at 0xe14b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe14b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe14c4 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xe14cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe14d4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xe14e0 | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0xe14e8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe14ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe14f0 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0xe1504 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe150c | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe1514 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1518 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe1520 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe1528 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe152c | load | 4 | `abs 0xe16b4` | `-` |
| `ldr` at 0xe1538 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1540 | load | 4 | `abs 0xe16b4` | `-` |
| `str` at 0xe154c | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1550 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1554 | load | 4 | `abs 0xe16b8` | `-` |
| `ldr` at 0xe1560 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1568 | load | 4 | `abs 0xe16b8` | `-` |
| `str` at 0xe1574 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe1578 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe157c | load | 4 | `abs 0xe16bc` | `-` |
| `ldr` at 0xe1588 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1590 | load | 4 | `abs 0xe16bc` | `-` |
| `str` at 0xe159c | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe15a0 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe15b0 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe15c4 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe15c8 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe15d8 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe15ec | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xe15f0 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1608 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe160c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1614 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe161c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1624 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1628 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1638 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe1640 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xe1648 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe164c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xe1654 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xe166c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe1670 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xe1678 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xe167c | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xe1680 | load | 4 | `abs 0xe16c0` | `-` |
| `ldr` at 0xe1684 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0xe1694 | store | 4 | `sp -0x158` | `0x0` |
| `str` at 0xe16a0 | store | 4 | `sp -0x158` | `0x1` |
| `ldr` at 0xe16a4 | load | 4 | `sp -0x158` | `-` |
| `ldmdb` at 0xe16a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe16a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe16a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe13ac | `(unknown callee) 0xe0e38` | `sp-328` | `?` | `?` | `sp-136` |
| `bl` at 0xe13e8 | `(unknown callee) 0xdcf54` | `0x0` | `[sp-32]` | `[sp]` | `[sp+4]` |
| `bl` at 0xe14bc | `(unknown callee) 0x1b49c` | `[sp+12]` | `0x1354e4` | `[sp-36]` | `?` |
| `bl` at 0xe168c | `(unknown callee) 0x1b49c` | `[sp+12]` | `0x135534` | `[sp-332]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xe131c | `C == 0` | `not (C == 0)` |
| `bne` at 0xe13f4 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `beq` at 0xe140c | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `blo` at 0xe143c | `C == 0` | `not (C == 0)` |
| `beq` at 0xe1478 | `? == ?` | `? != ?` |
| `blo` at 0xe14f8 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe1634 | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe16a8 | `r0` | `?` |
