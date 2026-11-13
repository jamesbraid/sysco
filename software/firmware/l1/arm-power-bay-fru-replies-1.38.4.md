# ARM power-bay and FRU replies in 1.38.4

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
The computed selector in L1FRU_2m_Init remains unresolved. Tables and return values cover only known edges, with its unresolved site listed below the entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/power-bay-fru-replies-facts.txt) retain all tool sections.

## `PBayPollHwStatus`

- `PBayPollHwStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab1b4 to 0xab848.
  Established. The initial call to `0x1f1dc` has `0x55`, entry r1, entry r2, `0x0` (`bl` at
  0xab1dc); its nonzero result takes the continuation and zero falls through (`bne` at 0xab1ec). The
  next comparison takes the continuation for `ret(0xa9910) == 0x2` and falls through on inequality
  (`beq` at 0xab200). Polling units, reply-bit meanings and the common r0 remain unknown (`ldmdb` at
  0xab81c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab1c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab1cc | store | 4 | `sp -0x4c` | `0x1` |
| `str` at 0xab1d4 | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0xab1e0 | store | 4 | `sp -0x18` | `ret(0x1f1dc)` |
| `ldr` at 0xab1e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab208 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab238 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xab23c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab24c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab254 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xab264 | store | 4 | `sp -0x24` | `ret(0xae090)` |
| `ldr` at 0xab268 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xab284 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xab28c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xab294 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xab298 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab2ac | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xab2b4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab2bc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xab2c4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab2d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab2e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab2e8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xab2f0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xab2f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab2f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab308 | load | 4 | `abs 0xab820` | `-` |
| `str` at 0xab30c | store | 4 | `sp -0xac` | `0x12707c` |
| `ldr` at 0xab310 | load | 4 | `abs 0xab824` | `-` |
| `str` at 0xab314 | store | 4 | `sp -0x44` | `0xbbf` |
| `str` at 0xab31c | store | 4 | `sp -0x48` | `0x7500` |
| `ldr` at 0xab324 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab328 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab338 | load | 4 | `abs 0xab828` | `-` |
| `str` at 0xab33c | store | 4 | `sp -0xac` | `0x127088` |
| `str` at 0xab344 | store | 4 | `sp -0x44` | `0xbc0` |
| `str` at 0xab34c | store | 4 | `sp -0x48` | `0x7400` |
| `ldr` at 0xab350 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab35c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xab370 | load | 4 | `abs 0xab82c` | `-` |
| `ldr` at 0xab374 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab378 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xab384 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xab388 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xab38c | load | 4 | `sp -0x50` | `-` |
| `str` at 0xab390 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xab398 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xab39c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xab3a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab3b0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xab3b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xab3bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab3c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab3e0 | store | 4 | `sp -0x58` | `0x3` |
| `str` at 0xab3e8 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xab3ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab3fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab410 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xab420 | store | 4 | `sp -0x24` | `ret(0xae090)` |
| `ldr` at 0xab424 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xab438 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab448 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xab458 | load | 4 | `abs 0xab830` | `-` |
| `ldr` at 0xab45c | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0xab468 | store | 1 | `[sp-44] +0x1499d8` | `0x0` |
| `str` at 0xab470 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xab474 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab47c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xab488 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab498 | load | 4 | `abs 0xab830` | `-` |
| `ldr` at 0xab49c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab4a4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xab4a8 | load | 1 | `[sp-44] +0x1499d8` | `-` |
| `strb` at 0xab4b0 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xab4b4 | load | 1 | `sp -0x39` | `-` |
| `str` at 0xab4c4 | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xab4c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab4d0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xab4dc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab4ec | load | 1 | `sp -0x39` | `-` |
| `ldrb` at 0xab4f0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xab4f8 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xab4fc | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xab50c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab520 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xab52c | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xab53c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xab544 | load | 4 | `abs 0xab834` | `-` |
| `str` at 0xab548 | store | 4 | `sp -0xac` | `0x1270a4` |
| `ldr` at 0xab54c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab558 | store | 4 | `sp -0x44` | `[sp-44]+427` |
| `ldr` at 0xab55c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab564 | store | 4 | `sp -0x48` | `[sp-44]+28672` |
| `ldrb` at 0xab56c | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xab57c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab58c | load | 4 | `abs 0xab838` | `-` |
| `str` at 0xab590 | store | 4 | `sp -0xac` | `0x1270c0` |
| `ldr` at 0xab594 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab5a0 | store | 4 | `sp -0x44` | `[sp-44]+433` |
| `ldr` at 0xab5a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab5ac | store | 4 | `sp -0x48` | `[sp-44]+28928` |
| `ldrb` at 0xab5b4 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xab5c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab5d4 | load | 4 | `abs 0xab83c` | `-` |
| `str` at 0xab5d8 | store | 4 | `sp -0xac` | `0x1270d8` |
| `ldr` at 0xab5dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab5e8 | store | 4 | `sp -0x44` | `[sp-44]+439` |
| `ldr` at 0xab5ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab5f4 | store | 4 | `sp -0x48` | `[sp-44]+29184` |
| `str` at 0xab600 | store | 4 | `sp -0xac` | `0x0` |
| `ldr` at 0xab604 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xab618 | load | 4 | `abs 0xab82c` | `-` |
| `ldr` at 0xab61c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab620 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0xab62c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xab630 | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xab634 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xab638 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xab640 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xab644 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xab650 | load | 4 | `abs 0xab830` | `-` |
| `ldr` at 0xab654 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab660 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab670 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xab674 | store | 1 | `[sp-44] +0x1499d8` | `?` |
| `str` at 0xab67c | store | 4 | `sp -0x58` | `0x3` |
| `ldr` at 0xab680 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab688 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xab690 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab6a4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xab6a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab6bc | store | 4 | `sp -0x58` | `0x2` |
| `str` at 0xab6c4 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xab6c8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xab6dc | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xab6e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab6e4 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xab6e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab6f4 | store | 4 | `sp -0x40` | `ret(0xabb68)` |
| `ldr` at 0xab6f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xab704 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xab714 | load | 4 | `abs 0xab840` | `-` |
| `ldr` at 0xab718 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab720 | load | 1 | `sp -0x40` | `-` |
| `ldrb` at 0xab724 | load | 1 | `[sp-44] +0x1499df` | `-` |
| `strb` at 0xab730 | store | 1 | `sp -0x39` | `?` |
| `ldrb` at 0xab734 | load | 1 | `sp -0x39` | `-` |
| `ldr` at 0xab744 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xab758 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xab764 | store | 4 | `sp -0x58` | `[sp-88]-1` |
| `str` at 0xab774 | store | 4 | `sp -0x54` | `0x1` |
| `ldr` at 0xab78c | load | 4 | `abs 0xab844` | `-` |
| `ldr` at 0xab790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xab798 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab7a4 | store | 4 | `sp -0x44` | `[sp-44]+445` |
| `ldr` at 0xab7a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab7b0 | store | 4 | `sp -0x48` | `[sp-44]+29440` |
| `ldr` at 0xab7b8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xab7bc | store | 4 | `sp -0xb4` | `[sp-76]` |
| `ldr` at 0xab7c0 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xab7c4 | store | 4 | `sp -0xb0` | `[sp-80]` |
| `ldr` at 0xab7cc | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xab7d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xab7dc | load | 4 | `abs 0xab840` | `-` |
| `ldr` at 0xab7e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xab7e8 | load | 1 | `sp -0x40` | `-` |
| `strb` at 0xab7ec | store | 1 | `[sp-44] +0x1499df` | `([sp-64]:1 & 0xff)` |
| `ldr` at 0xab7f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xab7fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xab80c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xab81c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab81c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab81c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab1dc | `(unknown callee) 0x1f1dc` | `0x55` | `r1` | `r2` | `0x0` |
| `bl` at 0xab1f4 | `(unknown callee) 0xa9910` | `ret(0x1f1dc)` | `?` | `?` | `ret(0x1f1dc)` |
| `bl` at 0xab218 | `(unknown callee) 0x22a38` | `[sp-20]+164` | `0x0` | `?` | `[sp-20]+164` |
| `bl` at 0xab230 | `(unknown callee) 0xa9834` | `0x3` | `?` | `?` | `ret(0x22a38)` |
| `bl` at 0xab25c | `(unknown callee) 0xae090` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xab278 | `(unknown callee) 0x1f1dc` | `0x69` | `?` | `?` | `ret(0xae090)` |
| `bl` at 0xab37c | `(unknown callee) 0x1b49c` | `sp-168` | `0x127090` | `[sp-44]` | `[sp-172]` |
| `bl` at 0xab3a4 | `(unknown callee) 0x7a24c` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xab3d8 | `(unknown callee) 0x1d38c` | `0x69` | `?` | `[sp-32]` | `?` |
| `bl` at 0xab418 | `(unknown callee) 0xae090` | `[[sp-20]+8]` | `sp-56` | `sp-56` | `[sp-20]` |
| `bl` at 0xab624 | `(unknown callee) 0x1b49c` | `sp-168` | `0x127090` | `[sp-44]` | `?` |
| `bl` at 0xab64c | `(unknown callee) 0x7a24c` | `0x0` | `[sp-72]` | `[sp-68]` | `sp-168` |
| `bl` at 0xab6ec | `(unknown callee) 0xabb68` | `[[sp-20]+8]` | `?` | `?` | `[sp-20]` |
| `bl` at 0xab77c | `(unknown callee) 0xab848` | `0x64` | `[sp-88]` | `[sp-88]-1` | `0x1` |
| `bl` at 0xab794 | `(unknown callee) 0x1b49c` | `sp-168` | `0x1270e8` | `[sp-44]` | `sp-168` |
| `bl` at 0xab7d8 | `(unknown callee) 0x7a24c` | `0x0` | `[sp-44]+29440` | `[sp-44]+445` | `sp-168` |
| `bl` at 0xab808 | `(unknown callee) 0xa9834` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xab818 | `(unknown callee) 0x22dc0` | `[sp-20]+164` | `?` | `?` | `[sp-20]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xab1ec | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0xab200 | `ret(0xa9910) == 0x2` | `ret(0xa9910) != 0x2` |
| `beq` at 0xab224 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0xab248 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab270 | `ret(0xae090) == 0x0` | `ret(0xae090) != 0x0` |
| `ble` at 0xab2a0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xab2e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xab304 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xab334 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab358 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `beq` at 0xab364 | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0xab3c4 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xab3f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xab404 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xab42c | `ret(0xae090) != 0x0` | `ret(0xae090) == 0x0` |
| `beq` at 0xab454 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xab4bc | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xab508 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab518 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xab534 | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |
| `beq` at 0xab578 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab588 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab5c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab5d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xab60c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xab69c | `? == 0x0` | `? != 0x0` |
| `ble` at 0xab6b0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xab6d0 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xab700 | `ret(0xabb68) < 0x0 (signed, 32-bit)` | `ret(0xabb68) >= 0x0 (signed, 32-bit)` |
| `bne` at 0xab710 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xab73c | `[sp-57]:1 != 0x0` | `[sp-57]:1 == 0x0` |
| `beq` at 0xab750 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xab76c | `[sp-88] <= 0x0 (signed, 32-bit)` | `[sp-88] > 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab81c | `r0` | `?` |

## `PBaySdrGetDpsStateHdr`

- `PBaySdrGetDpsStateHdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab870 to 0xab8bc.
  Established. The opening call to `0x1b124` carries `0x50` and entry r1, r2, r3 (`bl` at 0xab884).
  The exit reports r0 `[sp-20]` (`ldmdb` at 0xab8b0). No conditional branches are reported. Header
  contents and ownership remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab88c | store | 4 | `sp -0x14` | `ret(0x1b124)` |
| `ldr` at 0xab890 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab894 | load | 4 | `abs 0xab8b4` | `-` |
| `ldr` at 0xab89c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab8a0 | load | 4 | `abs 0xab8b8` | `-` |
| `ldr` at 0xab8a8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xab8b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xab8b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xab8b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab884 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xab898 | `(unknown callee) 0x1b508` | `ret(0x1b124)` | `0x127118` | `?` | `ret(0x1b124)` |
| `bl` at 0xab8a4 | `(unknown callee) 0x1b2d4` | `[sp-20]` | `0x127138` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xab8b0 | `r0` | `[sp-20]` |

## `PBaySdrGetDpsState`

- `PBaySdrGetDpsState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xab8bc to 0xabb1c.
  Established. A word store clears entry `r2 +0x0` (`str` at 0xab8e0). A later word store
  writes `ret(0x1b124)` at `[sp-28] +0x0` (`str` at 0xab954). The signed guards take their bypasses
  for `[sp-24] <= 0x0` and `[sp-24] > 0x6`, respectively, with the opposite comparisons on
  fall-through (`ble` at 0xab8ec, `bgt` at 0xab8f8). Reply encoding and r0 at exit remain unresolved
  (`ldmdb` at 0xabae4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xab8cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xab8d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xab8d4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xab8d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xab8e0 | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xab8e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab8f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xab904 | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xab90c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xab914 | load | 4 | `abs 0xabae8` | `-` |
| `str` at 0xab91c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xab920 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xab924 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xab934 | store | 4 | `sp -0x1d0` | `0x0` |
| `str` at 0xab948 | store | 4 | `sp -0x28` | `ret(0x1b124)` |
| `ldr` at 0xab94c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xab950 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xab954 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xab958 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab95c | load | 4 | `abs 0xabaec` | `-` |
| `ldr` at 0xab968 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xab970 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xab978 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xab980 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xab984 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xab988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xab99c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xab9ac | store | 4 | `sp -0x1cc` | `ret(0xaa7dc)` |
| `ldr` at 0xab9b0 | load | 4 | `sp -0x1cc` | `-` |
| `ldr` at 0xab9bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xab9c0 | load | 4 | `abs 0xabaf0` | `-` |
| `str` at 0xab9cc | store | 4 | `sp -0x1d0` | `?` |
| `ldr` at 0xab9d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xab9dc | store | 4 | `sp -0x24` | `ret(0xae3dc)` |
| `ldr` at 0xab9e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xab9e4 | load | 1 | `ret(0xae3dc) +0x2` | `-` |
| `strb` at 0xab9e8 | store | 1 | `sp -0x2d` | `([ret(0xae3dc)+2]:1 & 0xff)` |
| `ldrb` at 0xab9ec | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xab9fc | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xaba0c | load | 4 | `abs 0xabaf4` | `-` |
| `str` at 0xaba10 | store | 4 | `sp -0x2c` | `0x127174` |
| `ldr` at 0xaba18 | load | 4 | `abs 0xabaf8` | `-` |
| `str` at 0xaba1c | store | 4 | `sp -0x2c` | `0x12717c` |
| `ldr` at 0xaba20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaba24 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaba2c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xaba3c | load | 4 | `abs 0xabafc` | `-` |
| `str` at 0xaba40 | store | 4 | `sp -0x2c` | `0x127184` |
| `ldr` at 0xaba48 | load | 4 | `abs 0xabb00` | `-` |
| `str` at 0xaba4c | store | 4 | `sp -0x2c` | `0x127190` |
| `ldr` at 0xaba50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaba54 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaba5c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xaba6c | load | 4 | `abs 0xabb04` | `-` |
| `str` at 0xaba70 | store | 4 | `sp -0x2c` | `0x12719c` |
| `ldr` at 0xaba78 | load | 4 | `abs 0xabb08` | `-` |
| `str` at 0xaba7c | store | 4 | `sp -0x2c` | `0x1271a8` |
| `ldr` at 0xaba80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xaba84 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaba8c | load | 1 | `sp -0x2d` | `-` |
| `ldr` at 0xaba9c | load | 4 | `abs 0xabb0c` | `-` |
| `str` at 0xabaa0 | store | 4 | `sp -0x2c` | `0x1271b4` |
| `ldr` at 0xabaa8 | load | 4 | `abs 0xabb10` | `-` |
| `str` at 0xabaac | store | 4 | `sp -0x2c` | `0x1271bc` |
| `ldr` at 0xabab0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabab4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xabac0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabac4 | load | 4 | `abs 0xabb14` | `-` |
| `ldr` at 0xabacc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabad0 | load | 4 | `abs 0xabb18` | `-` |
| `str` at 0xabadc | store | 4 | `sp -0x1d0` | `0x0` |
| `ldr` at 0xabae0 | load | 4 | `sp -0x1d0` | `-` |
| `ldmdb` at 0xabae4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabae4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabae4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xab940 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xab960 | `(unknown callee) 0x1b508` | `[sp-40]` | `0x12715c` | `[sp-28]` | `ret(0x1b124)` |
| `bl` at 0xab994 | `(unknown callee) 0xad4f4` | `[sp-20]` | `sp-456` | `[[sp-32]]:1` | `[[sp-32]]:1` |
| `bl` at 0xab9a4 | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-456` | `?` | `sp-456` |
| `bl` at 0xab9c4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xab9d8 | `(unknown callee) 0xae3dc` | `[sp-64]` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xaba28 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xaba58 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xaba88 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xabab8 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `[sp-44]` | `?` | `?` |
| `bl` at 0xabac8 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1271c4` | `?` | `?` |
| `bl` at 0xabad4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1271d0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xab8ec | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bgt` at 0xab8f8 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `bne` at 0xab92c | `[[sp-32]]:1 != 0xff` | `[[sp-32]]:1 == 0xff` |
| `bne` at 0xab9b8 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `bne` at 0xab9f8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xaba08 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaba38 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaba68 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xaba98 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabae4 | `r0` | `?` |

## `PBaySdrGetDcPortStateHdr`

- `PBaySdrGetDcPortStateHdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabb1c to 0xabb68.
  Established. The call to `0x1b124` receives `0x50`, entry r1, entry r2, entry r3 (`bl` at
  0xabb30). The recognized exit carries `[sp-20]` (`ldmdb` at 0xabb5c). Calls to the other two
  numeric targets are retained separately in the table. DC-port header fields remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabb38 | store | 4 | `sp -0x14` | `ret(0x1b124)` |
| `ldr` at 0xabb3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabb40 | load | 4 | `abs 0xabb60` | `-` |
| `ldr` at 0xabb48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabb4c | load | 4 | `abs 0xabb64` | `-` |
| `ldr` at 0xabb54 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xabb5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabb5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabb5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabb30 | `(unknown callee) 0x1b124` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xabb44 | `(unknown callee) 0x1b508` | `ret(0x1b124)` | `0x1271d4` | `?` | `ret(0x1b124)` |
| `bl` at 0xabb50 | `(unknown callee) 0x1b2d4` | `[sp-20]` | `0x1271e8` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabb5c | `r0` | `[sp-20]` |

## `PBaySdrGetDcPort`

- `PBaySdrGetDcPort`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabb68 to 0xabc48.
  Established. The signed lower-bound branch takes its bypass for `r1 <= 0x0` and falls through for
  greater (`ble` at 0xabb88). The next signed test takes that bypass for `r1 > 0x8` and falls
  through for less or equal (`bgt` at 0xabb94). The call to `0xad4f4` has entry r0, `sp-440`,
  `[[sp-28]]:1`, `[[sp-28]]:1` (`bl` at 0xabbf0). The exit reports `[sp-448]` (`ldmdb` at 0xabc40).
  DC-port numbering and reply units remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabb78 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabb7c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xabb80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabb8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabba0 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xabba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabbb0 | load | 4 | `abs 0xabc44` | `-` |
| `str` at 0xabbb8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xabbbc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xabbc0 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xabbd0 | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xabbdc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xabbe0 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xabbe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabbf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabc08 | store | 4 | `sp -0x1bc` | `ret(0xaa7dc)` |
| `ldr` at 0xabc0c | load | 4 | `sp -0x1bc` | `-` |
| `str` at 0xabc1c | store | 4 | `sp -0x1c0` | `?` |
| `ldr` at 0xabc24 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xabc2c | store | 4 | `sp -0x20` | `ret(0xae3dc)` |
| `ldr` at 0xabc30 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xabc34 | load | 1 | `ret(0xae3dc) +0x2` | `-` |
| `str` at 0xabc38 | store | 4 | `sp -0x1c0` | `[ret(0xae3dc)+2]:1` |
| `ldr` at 0xabc3c | load | 4 | `sp -0x1c0` | `-` |
| `ldmdb` at 0xabc40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabc40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabc40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabbf0 | `(unknown callee) 0xad4f4` | `r0` | `sp-440` | `[[sp-28]]:1` | `[[sp-28]]:1` |
| `bl` at 0xabc00 | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-440` | `?` | `sp-440` |
| `bl` at 0xabc28 | `(unknown callee) 0xae3dc` | `[sp-48]` | `?` | `?` | `ret(0xaa7dc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xabb88 | `r1 <= 0x0 (signed, 32-bit)` | `r1 > 0x0 (signed, 32-bit)` |
| `bgt` at 0xabb94 | `r1 > 0x8 (signed, 32-bit)` | `r1 <= 0x8 (signed, 32-bit)` |
| `bne` at 0xabbc8 | `[[sp-28]]:1 != 0xff` | `[[sp-28]]:1 == 0xff` |
| `bne` at 0xabc14 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabc40 | `r0` | `[sp-448]` |

## `PBaySdrGetDcPortState`

- `PBaySdrGetDcPortState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabc48 to 0xabd94.
  Established. A word store clears entry `r2 +0x0`, and a later word store writes `ret(0x1b124)` at
  `[sp-28] +0x0` (`str` at 0xabc6c, `str` at 0xabcb4). The call to `0xabb68` uses `[sp-20]`,
  `[sp-24]`, entry r2, `0x0` (`bl` at 0xabc78). The next branch retains only `Z == 0` and its
  negation for fall-through (`bne` at 0xabc8c). State fields and final r0 remain unknown (`ldmdb` at
  0xabd74).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabc58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabc5c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xabc60 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xabc64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xabc6c | store | 4 | `r2 +0x0` | `0x0` |
| `ldr` at 0xabc70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabc74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabc80 | store | 4 | `sp -0x24` | `ret(0xabb68)` |
| `ldr` at 0xabc84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xabc94 | store | 4 | `sp -0x1cc` | `0x0` |
| `str` at 0xabca8 | store | 4 | `sp -0x28` | `ret(0x1b124)` |
| `ldr` at 0xabcac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabcb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xabcb4 | store | 4 | `[sp-28] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xabcb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabcbc | load | 4 | `abs 0xabd78` | `-` |
| `ldr` at 0xabcc8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xabcd0 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xabcd8 | store | 1 | `[sp-40] +0x3` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xabcdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabce8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabcec | load | 4 | `abs 0xabd7c` | `-` |
| `str` at 0xabcf8 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xabd00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabd10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabd14 | load | 4 | `abs 0xabd80` | `-` |
| `ldr` at 0xabd20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabd24 | load | 4 | `abs 0xabd84` | `-` |
| `ldr` at 0xabd2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabd3c | load | 4 | `abs 0xabd88` | `-` |
| `str` at 0xabd40 | store | 4 | `sp -0x1d0` | `0x1271b4` |
| `ldr` at 0xabd48 | load | 4 | `abs 0xabd8c` | `-` |
| `str` at 0xabd4c | store | 4 | `sp -0x1d0` | `0x127220` |
| `ldr` at 0xabd50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabd54 | load | 4 | `sp -0x1d0` | `-` |
| `ldr` at 0xabd5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xabd60 | load | 4 | `abs 0xabd90` | `-` |
| `str` at 0xabd6c | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xabd70 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0xabd74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabd74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabd74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabc78 | `(unknown callee) 0xabb68` | `[sp-20]` | `[sp-24]` | `r2` | `0x0` |
| `bl` at 0xabca0 | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `ret(0xabb68)` |
| `bl` at 0xabcc0 | `(unknown callee) 0x1b508` | `[sp-40]` | `0x127204` | `[sp-28]` | `ret(0x1b124)` |
| `bl` at 0xabcf0 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `[sp-40]+3` | `[sp-36]` |
| `bl` at 0xabd18 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x12720c` | `[sp-40]+3` | `?` |
| `bl` at 0xabd28 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127214` | `[sp-40]+3` | `?` |
| `bl` at 0xabd58 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `[sp-464]` | `?` | `?` |
| `bl` at 0xabd64 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1271d0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xabc8c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xabce4 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0xabd0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xabd38 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabd74 | `r0` | `?` |

## `PBaySdrGetMain`

- `PBaySdrGetMain`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabd94 to 0xabfc8.
  Established. The entry `r1 +0x0` word store clears its target (`str` at 0xabdb4). A later word
  store writes `ret(0x1b124)` at `[sp-24] +0x0`, followed by a byte store clearing `[sp-32] +0x0`
  (`str` at 0xabe1c, `strb` at 0xabe28). The initial global test takes its continuation for
  `[0x1499e8] != 0x0` and falls through on zero (`bne` at 0xabdc4). Main-reply formatting and exit
  r0 remain unidentified (`ldmdb` at 0xabfa4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabda4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabda8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xabdac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xabdb4 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xabdb8 | load | 4 | `abs 0xabfa8` | `-` |
| `ldr` at 0xabdbc | load | 4 | `abs 0x1499e8` | `-` |
| `str` at 0xabdcc | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xabdd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabde8 | store | 4 | `sp -0x1f4` | `ret(0xae090)` |
| `ldr` at 0xabdec | load | 4 | `sp -0x1f4` | `-` |
| `str` at 0xabdfc | store | 4 | `sp -0x200` | `0x0` |
| `str` at 0xabe10 | store | 4 | `sp -0x20` | `ret(0x1b124)` |
| `ldr` at 0xabe14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabe18 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xabe1c | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xabe20 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xabe28 | store | 1 | `[sp-32] +0x0` | `0x0` |
| `str` at 0xabe30 | store | 4 | `sp -0x1f0` | `0x0` |
| `ldr` at 0xabe34 | load | 4 | `abs 0xabfa8` | `-` |
| `ldr` at 0xabe38 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xabe3c | load | 4 | `abs 0x1499e8` | `-` |
| `ldr` at 0xabe50 | load | 4 | `abs 0xabfac` | `-` |
| `ldr` at 0xabe54 | load | 4 | `sp -0x1f0` | `-` |
| `ldrb` at 0xabe6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xabe70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabe84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabe94 | store | 4 | `sp -0x1ec` | `ret(0xaa7dc)` |
| `ldr` at 0xabe98 | load | 4 | `sp -0x1ec` | `-` |
| `ldr` at 0xabea4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabea8 | load | 4 | `abs 0xabfb0` | `-` |
| `str` at 0xabeb4 | store | 4 | `sp -0x200` | `?` |
| `ldr` at 0xabebc | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xabed0 | load | 4 | `abs 0xabfb4` | `-` |
| `str` at 0xabee4 | store | 4 | `sp -0x24` | `ret(0xad9a4)` |
| `ldr` at 0xabee8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xabeec | load | 1 | `ret(0xad9a4) +0x0` | `-` |
| `ldr` at 0xabef8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabefc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xabf08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabf0c | load | 4 | `abs 0xabfb8` | `-` |
| `ldr` at 0xabf14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabf18 | load | 4 | `abs 0xabfbc` | `-` |
| `ldr` at 0xabf20 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xabf28 | store | 4 | `sp -0x1c` | `ret(0xae3dc)` |
| `ldr` at 0xabf2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabf30 | load | 4 | `abs 0xabfc0` | `-` |
| `ldrb` at 0xabf38 | load | 1 | `sp -0x1f4` | `-` |
| `strb` at 0xabf3c | store | 1 | `ret(0xae3dc) +0x3` | `([sp-500]:1 & 0xff)` |
| `ldr` at 0xabf40 | load | 4 | `sp -0x1f0` | `-` |
| `ldr` at 0xabf54 | load | 4 | `abs 0xabfb4` | `-` |
| `ldr` at 0xabf60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xabf70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabf7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xabf80 | load | 4 | `abs 0xabfc4` | `-` |
| `ldr` at 0xabf88 | load | 4 | `sp -0x1f0` | `-` |
| `str` at 0xabf90 | store | 4 | `sp -0x1f0` | `[sp-496]+1` |
| `str` at 0xabf9c | store | 4 | `sp -0x200` | `0x0` |
| `ldr` at 0xabfa0 | load | 4 | `sp -0x200` | `-` |
| `ldmdb` at 0xabfa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xabfa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xabfa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabde0 | `(unknown callee) 0xae090` | `[sp-20]` | `sp-508` | `r1` | `sp-508` |
| `bl` at 0xabe08 | `(unknown callee) 0x1b124` | `0x100` | `?` | `?` | `ret(0xae090)` |
| `bl` at 0xabe7c | `(unknown callee) 0xad4f4` | `[sp-20]` | `sp-488` | `?` | `?` |
| `bl` at 0xabe8c | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-488` | `?` | `sp-488` |
| `bl` at 0xabeac | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x127164` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xabedc | `(unknown callee) 0xad9a4` | `?` | `?` | `?` | `?` |
| `bl` at 0xabf00 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `ret(0xad9a4)` | `?` | `[ret(0xad9a4)]:1` |
| `bl` at 0xabf10 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x127224` | `?` | `[ret(0xad9a4)]:1` |
| `bl` at 0xabf1c | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x127234` | `?` | `?` |
| `bl` at 0xabf24 | `(unknown callee) 0xae3dc` | `[sp-96]` | `?` | `?` | `?` |
| `bl` at 0xabf68 | `(unknown callee) 0xad878` | `[sp-28]` | `?` | `sp-76` | `?` |
| `bl` at 0xabf78 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `sp-76` | `?` | `sp-76` |
| `bl` at 0xabf84 | `(unknown callee) 0x1b2d4` | `[sp-32]` | `0x1271d0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xabdc4 | `[0x1499e8] != 0x0` | `[0x1499e8] == 0x0` |
| `bne` at 0xabdf4 | `ret(0xae090) != 0x0` | `ret(0xae090) == 0x0` |
| `blt` at 0xabe44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xabea0 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `beq` at 0xabef4 | `[ret(0xad9a4)]:1 == 0x0` | `[ret(0xad9a4)]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xabfa4 | `r0` | `?` |

## `PBaySdrGetFruHdr`

- `PBaySdrGetFruHdr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabfc8 to 0xac014.
  Established. The opening call to `0x1b124` carries `0xa0` and entry r1 through r3 (`bl` at
  0xabfdc). R0 exits as `[sp-20]` (`ldmdb` at 0xac008). No deciding branches appear in the facts.
  FRU-header field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabfe4 | store | 4 | `sp -0x14` | `ret(0x1b124)` |
| `ldr` at 0xabfe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabfec | load | 4 | `abs 0xac00c` | `-` |
| `ldr` at 0xabff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabff8 | load | 4 | `abs 0xac010` | `-` |
| `ldr` at 0xac000 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xac008 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xac008 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xac008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabfdc | `(unknown callee) 0x1b124` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xabff0 | `(unknown callee) 0x1b508` | `ret(0x1b124)` | `0x127238` | `?` | `ret(0x1b124)` |
| `bl` at 0xabffc | `(unknown callee) 0x1b2d4` | `[sp-20]` | `0x12727c` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xac008 | `r0` | `[sp-20]` |

## `PBaySdrGetFru`

- `PBaySdrGetFru`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac014 to 0xaca9c.
  Established. A word store clears entry `r3 +0x0` (`str` at 0xac03c). A byte store at an
  unknown effective address later carries `([[sp-744]+2]:1 & 0xff)` (`strb` at 0xac160). Another
  word store writes `ret(0x1b124)` at `[sp-32] +0x0` (`str` at 0xac1a0). The first call to `0xad4f4`
  has `[sp-20]`, `sp-712`, `[[sp-740]]:1`, `[[sp-740]]:1` (`bl` at 0xac11c). FRU record semantics,
  length units and the common r0 are unproved (`ldmdb` at 0xaca50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac024 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac028 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac02c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xac030 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xac034 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac03c | store | 4 | `r3 +0x0` | `0x0` |
| `ldr` at 0xac040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac04c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac060 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xac068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac07c | load | 4 | `abs 0xaca54` | `-` |
| `str` at 0xac084 | store | 4 | `sp -0x130` | `?` |
| `ldr` at 0xac08c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac098 | load | 4 | `abs 0xaca58` | `-` |
| `str` at 0xac09c | store | 4 | `sp -0x130` | `0x14296c` |
| `ldr` at 0xac0a4 | load | 4 | `abs 0xaca54` | `-` |
| `str` at 0xac0a8 | store | 4 | `sp -0x130` | `0x142950` |
| `ldr` at 0xac0ac | load | 4 | `sp -0x130` | `-` |
| `ldrb` at 0xac0b0 | load | 1 | `[sp-304] +0x0` | `-` |
| `str` at 0xac0c0 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xac0c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac0d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac0dc | load | 4 | `abs 0xaca5c` | `-` |
| `str` at 0xac0e4 | store | 4 | `sp -0x2e4` | `?` |
| `ldr` at 0xac0e8 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xac0ec | load | 1 | `[sp-740] +0x0` | `-` |
| `str` at 0xac0fc | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xac108 | load | 4 | `sp -0x2e4` | `-` |
| `ldrb` at 0xac10c | load | 1 | `[sp-740] +0x0` | `-` |
| `ldr` at 0xac110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac124 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac134 | store | 4 | `sp -0x2cc` | `ret(0xaa7dc)` |
| `ldr` at 0xac138 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xac144 | load | 4 | `sp -0x140` | `-` |
| `str` at 0xac14c | store | 4 | `sp -0x2e8` | `ret(0xae3dc)` |
| `ldr` at 0xac154 | load | 4 | `sp -0x2e8` | `-` |
| `ldrb` at 0xac158 | load | 1 | `[sp-744] +0x2` | `-` |
| `strb` at 0xac160 | store | 1 | `(unknown)` | `([[sp-744]+2]:1 & 0xff)` |
| `ldrb` at 0xac16c | load | 1 | `(unknown)` | `-` |
| `str` at 0xac180 | store | 4 | `sp -0x300` | `0x0` |
| `str` at 0xac194 | store | 4 | `sp -0x28` | `ret(0x1b124)` |
| `ldr` at 0xac198 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac19c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac1a0 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xac1a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac1b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac1b4 | load | 4 | `abs 0xaca60` | `-` |
| `ldr` at 0xac1c0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xac1c8 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xac1d0 | store | 1 | `[sp-40] +0x1` | `([sp-24]:1+48 & 0xff)` |
| `ldr` at 0xac1d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac1e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac1e8 | load | 4 | `abs 0xaca64` | `-` |
| `ldr` at 0xac1f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac1f8 | load | 4 | `abs 0xaca68` | `-` |
| `ldr` at 0xac200 | load | 4 | `abs 0xaca6c` | `-` |
| `strh` at 0xac20c | store | 2 | `sp -0x2d2` | `0x0` |
| `ldr` at 0xac210 | load | 4 | `abs 0xaca70` | `-` |
| `strb` at 0xac21c | store | 1 | `sp -0x2d3` | `0x8` |
| `ldr` at 0xac224 | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac22c | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xac230 | load | 4 | `abs 0xaca70` | `-` |
| `ldrb` at 0xac238 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xac23c | store | 4 | `sp -0x308` | `0x8` |
| `ldr` at 0xac240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac248 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac258 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac268 | store | 4 | `sp -0x2cc` | `ret(0xaa7dc)` |
| `ldr` at 0xac26c | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xac278 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac27c | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac288 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac290 | load | 4 | `sp -0x140` | `-` |
| `str` at 0xac298 | store | 4 | `sp -0x2ec` | `ret(0xae3dc)` |
| `ldr` at 0xac2a0 | load | 4 | `sp -0x2ec` | `-` |
| `ldr` at 0xac2b8 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac2c4 | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xac2d0 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac2dc | load | 1 | `sp -0x2f0` | `-` |
| `ldr` at 0xac2e8 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac2ec | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac2f8 | load | 1 | `sp -0x2f0` | `-` |
| `strh` at 0xac304 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xac308 | load | 4 | `abs 0xaca70` | `-` |
| `strb` at 0xac314 | store | 1 | `sp -0x2d3` | `0x3` |
| `ldr` at 0xac31c | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac324 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xac328 | load | 4 | `abs 0xaca70` | `-` |
| `ldrb` at 0xac330 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xac334 | store | 4 | `sp -0x308` | `0x3` |
| `str` at 0xac33c | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xac340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac348 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac358 | load | 4 | `abs 0xaca7c` | `-` |
| `strb` at 0xac360 | store | 1 | `sp -0x2d4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xac364 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac36c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac378 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac37c | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac388 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac390 | load | 4 | `abs 0xaca70` | `-` |
| `ldr` at 0xac394 | load | 4 | `abs 0xaca80` | `-` |
| `ldrb` at 0xac39c | load | 1 | `sp -0x127` | `-` |
| `strb` at 0xac3a8 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xac3ac | load | 4 | `abs 0xaca70` | `-` |
| `ldr` at 0xac3b0 | load | 4 | `abs 0xaca70` | `-` |
| `ldr` at 0xac3b4 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac3bc | load | 1 | `sp -0x2d3` | `-` |
| `ldrb` at 0xac3c4 | load | 1 | `sp -0x2d4` | `-` |
| `strb` at 0xac3d0 | store | 1 | `sp -0x2d3` | `?` |
| `ldr` at 0xac3d4 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac3d8 | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac3e0 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xac3ec | store | 2 | `sp -0x2d2` | `([sp-722]:2+3 & 0xffff)` |
| `ldr` at 0xac3f4 | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac3fc | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xac400 | load | 4 | `abs 0xaca70` | `-` |
| `ldrb` at 0xac408 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xac40c | store | 4 | `sp -0x308` | `[sp-723]:1` |
| `str` at 0xac414 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xac418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac420 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac430 | load | 4 | `abs 0xaca7c` | `-` |
| `strb` at 0xac438 | store | 1 | `sp -0x2d4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xac43c | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac444 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac450 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac454 | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac460 | store | 4 | `sp -0x300` | `?` |
| `str` at 0xac46c | store | 4 | `sp -0x12c` | `0x0` |
| `str` at 0xac474 | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xac478 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xac488 | load | 4 | `abs 0xaca84` | `-` |
| `ldr` at 0xac48c | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xac49c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac4b0 | load | 4 | `abs 0xaca84` | `-` |
| `ldr` at 0xac4b4 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xac4c4 | load | 1 | `(sp-16 + [sp-300]) -0x118` | `-` |
| `strb` at 0xac4d0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xac4d4 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xac4dc | store | 4 | `sp -0x12c` | `[sp-300]+1` |
| `ldr` at 0xac4e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac4f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac4f8 | store | 4 | `sp -0x24` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0xac500 | load | 4 | `sp -0x12c` | `-` |
| `ldrb` at 0xac510 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac514 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xac52c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac530 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xac538 | store | 4 | `sp -0x12c` | `?` |
| `ldrb` at 0xac544 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac548 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac550 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac558 | load | 4 | `abs 0xaca88` | `-` |
| `ldr` at 0xac55c | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xac568 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xac56c | load | 1 | `[sp-720] +0x1429a0` | `-` |
| `ldr` at 0xac578 | load | 4 | `abs 0xaca88` | `-` |
| `ldr` at 0xac57c | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xac584 | load | 1 | `[sp-720] +0x1429a0` | `-` |
| `ldrb` at 0xac590 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac598 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac5a8 | load | 4 | `abs 0xaca88` | `-` |
| `ldr` at 0xac5ac | load | 4 | `sp -0x2d0` | `-` |
| `ldrb` at 0xac5b4 | load | 1 | `[sp-720] +0x1429a0` | `-` |
| `ldrb` at 0xac5c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac5c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac5d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac5d8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xac5e4 | store | 1 | `[sp-36] +0x0` | `0x20` |
| `str` at 0xac5ec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xac5f0 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xac5f8 | store | 1 | `[sp-36] +0x1` | `0x0` |
| `ldr` at 0xac5fc | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xac604 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `ldr` at 0xac60c | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac618 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xac624 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac630 | load | 1 | `sp -0x2f1` | `-` |
| `ldr` at 0xac63c | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac640 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac64c | load | 1 | `sp -0x2f1` | `-` |
| `strh` at 0xac658 | store | 2 | `sp -0x2d2` | `?` |
| `ldr` at 0xac65c | load | 4 | `abs 0xaca70` | `-` |
| `strb` at 0xac668 | store | 1 | `sp -0x2d3` | `0x6` |
| `ldr` at 0xac670 | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac678 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xac67c | load | 4 | `abs 0xaca70` | `-` |
| `ldrb` at 0xac684 | load | 1 | `sp -0x2d3` | `-` |
| `str` at 0xac688 | store | 4 | `sp -0x308` | `0x6` |
| `str` at 0xac690 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xac694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac69c | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac6ac | load | 4 | `abs 0xaca7c` | `-` |
| `strb` at 0xac6b4 | store | 1 | `sp -0x2d4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xac6b8 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac6c0 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac6cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac6d0 | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac6dc | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac6e4 | load | 4 | `abs 0xaca8c` | `-` |
| `ldrb` at 0xac6ec | load | 1 | `sp -0x123` | `-` |
| `str` at 0xac6f0 | store | 4 | `sp -0x2dc` | `[sp-291]:1` |
| `ldr` at 0xac6f4 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xac6fc | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xac700 | load | 4 | `abs 0xaca90` | `-` |
| `ldrb` at 0xac708 | load | 1 | `sp -0x124` | `-` |
| `ldr` at 0xac70c | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xac714 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xac718 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xac720 | store | 4 | `sp -0x2dc` | `?` |
| `ldrb` at 0xac72c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xac730 | load | 4 | `sp -0x2dc` | `-` |
| `str` at 0xac738 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xac73c | load | 4 | `sp -0x2dc` | `-` |
| `ldr` at 0xac748 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac760 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac76c | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xac778 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac784 | load | 1 | `sp -0x2ef` | `-` |
| `ldr` at 0xac790 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac794 | load | 4 | `abs 0xaca78` | `-` |
| `ldrb` at 0xac7a0 | load | 1 | `sp -0x2ef` | `-` |
| `strh` at 0xac7ac | store | 2 | `sp -0x2d2` | `?` |
| `str` at 0xac7b4 | store | 4 | `sp -0x2d0` | `0x0` |
| `ldr` at 0xac7bc | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac7c4 | load | 2 | `sp -0x2d2` | `-` |
| `str` at 0xac7cc | store | 4 | `sp -0x308` | `0x8` |
| `str` at 0xac7d4 | store | 4 | `sp -0x304` | `sp-764` |
| `ldr` at 0xac7d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac7e0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac7f0 | load | 4 | `abs 0xaca7c` | `-` |
| `strb` at 0xac7f8 | store | 1 | `sp -0x2d4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xac7fc | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac804 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac810 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac814 | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac820 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac848 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac84c | load | 4 | `abs 0xaca94` | `-` |
| `str` at 0xac858 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac860 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac864 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac868 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac870 | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0xac87c | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xac888 | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0xac890 | load | 4 | `abs 0xaca6c` | `-` |
| `ldrh` at 0xac898 | load | 2 | `sp -0x2d2` | `-` |
| `ldr` at 0xac89c | load | 4 | `abs 0xaca98` | `-` |
| `ldrb` at 0xac8a8 | load | 1 | `sp -0x2fa` | `-` |
| `str` at 0xac8ac | store | 4 | `sp -0x308` | `[sp-762]:1` |
| `str` at 0xac8b4 | store | 4 | `sp -0x304` | `sp-296` |
| `ldr` at 0xac8b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac8c0 | load | 4 | `sp -0x130` | `-` |
| `ldr` at 0xac8d0 | load | 4 | `abs 0xaca7c` | `-` |
| `strb` at 0xac8d8 | store | 1 | `sp -0x2d4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xac8dc | load | 4 | `abs 0xaca7c` | `-` |
| `ldr` at 0xac8e0 | load | 4 | `abs 0xaca98` | `-` |
| `ldrb` at 0xac8f0 | load | 1 | `sp -0x2d4` | `-` |
| `ldrb` at 0xac8f4 | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0xac900 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac904 | load | 4 | `abs 0xaca74` | `-` |
| `str` at 0xac910 | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac918 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac920 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac928 | load | 4 | `abs 0xaca84` | `-` |
| `ldr` at 0xac938 | load | 4 | `abs 0xaca98` | `-` |
| `ldrb` at 0xac944 | load | 1 | `sp -0x2f9` | `-` |
| `strb` at 0xac948 | store | 1 | `(unknown)` | `([sp-761]:1 & 0xff)` |
| `ldr` at 0xac950 | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac958 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xac97c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac980 | load | 4 | `abs 0xaca94` | `-` |
| `str` at 0xac98c | store | 4 | `sp -0x300` | `?` |
| `ldr` at 0xac994 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac9a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xac9ac | store | 4 | `sp -0x24` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0xac9b0 | load | 4 | `abs 0xaca98` | `-` |
| `ldrb` at 0xac9bc | load | 1 | `sp -0x2fc` | `-` |
| `ldr` at 0xac9c0 | load | 4 | `abs 0xaca98` | `-` |
| `ldrb` at 0xac9cc | load | 1 | `sp -0x2fa` | `-` |
| `ldr` at 0xac9dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac9e4 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac9e8 | load | 4 | `abs 0xaca6c` | `-` |
| `ldr` at 0xac9ec | load | 4 | `abs 0xaca7c` | `-` |
| `ldrb` at 0xac9f4 | load | 1 | `sp -0x2d4` | `-` |
| `ldrh` at 0xaca00 | load | 2 | `sp -0x2d2` | `-` |
| `strh` at 0xaca0c | store | 2 | `sp -0x2d2` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `ldr` at 0xaca10 | load | 4 | `abs 0xaca98` | `-` |
| `ldrsb` at 0xaca1c | load | 1 | `sp -0x2fb` | `-` |
| `ldr` at 0xaca2c | load | 4 | `sp -0x2d0` | `-` |
| `str` at 0xaca38 | store | 4 | `sp -0x2d0` | `[sp-720]+1` |
| `str` at 0xaca48 | store | 4 | `sp -0x300` | `0x0` |
| `ldr` at 0xaca4c | load | 4 | `sp -0x300` | `-` |
| `ldmdb` at 0xaca50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaca50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaca50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac11c | `(unknown callee) 0xad4f4` | `[sp-20]` | `sp-712` | `[[sp-740]]:1` | `[[sp-740]]:1` |
| `bl` at 0xac12c | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xac148 | `(unknown callee) 0xae3dc` | `[sp-320]` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xac18c | `(unknown callee) 0x1b124` | `0x400` | `?` | `?` | `?` |
| `bl` at 0xac1b8 | `(unknown callee) 0x1b508` | `[sp-40]` | `0x1272c0` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xac1ec | `(unknown callee) 0x1b508` | `[sp-40]` | `0x1272c8` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xac1fc | `(unknown callee) 0x1b508` | `[sp-40]` | `0x1272d0` | `[sp-32]` | `[sp-24]` |
| `bl` at 0xac250 | `(unknown callee) 0xad614` | `[sp-20]` | `sp-712` | `[sp-304]` | `0x0` |
| `bl` at 0xac260 | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-712` | `?` | `sp-712` |
| `bl` at 0xac280 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xac294 | `(unknown callee) 0xae3dc` | `[sp-320]` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xac2b4 | `(unknown callee) 0x1b304` | `sp-756` | `ret(0xae3dc)+1` | `0x8` | `ret(0xae3dc)+1` |
| `bl` at 0xac350 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xac380 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `ret(0xad730)` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xac428 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-712` | `[sp-304]` | `([sp-722]:2+3 & 0xffff)` |
| `bl` at 0xac458 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `sp-16` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xac4e4 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `sp-16` | `[sp-300]+1` |
| `bl` at 0xac520 | `(unknown callee) 0x1b304` | `[sp-36]` | `(sp-296 + [sp-300])` | `?` | `?` |
| `bl` at 0xac5a4 | `(unknown callee) 0x1b43c` | `[sp-36]` | `0x20` | `?` | `?` |
| `bl` at 0xac6a4 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xac6d4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `ret(0xad730)` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xac740 | `(unknown callee) 0xadcec` | `[sp-732]` | `?` | `?` | `?` |
| `bl` at 0xac750 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `ret(0xadcec)` | `?` | `ret(0xadcec)` |
| `bl` at 0xac7e8 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-712` | `[sp-304]` | `[sp-722]:2` |
| `bl` at 0xac818 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `ret(0xad730)` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xac834 | `(unknown callee) 0xadd44` | `sp-764` | `0x8` | `ret(0xad730)` | `sp-764` |
| `bl` at 0xac850 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1272d8` | `?` | `?` |
| `bl` at 0xac8c8 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-712` | `[sp-304]` | `(([sp-722]:2 + [sp-724]:1) & 0xffff)` |
| `bl` at 0xac908 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x127164` | `(ret(0xad730) & 0xff)` | `[sp-762]:1` |
| `bl` at 0xac968 | `(unknown callee) 0xadd44` | `sp-296` | `[sp-724]:1+1` | `sp-296` | `[sp-724]:1+1` |
| `bl` at 0xac984 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1272d8` | `?` | `?` |
| `bl` at 0xac998 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xac9e0 | `(unknown callee) 0xb3268` | `[sp-764]:1` | `[sp-762]:1` | `sp-296` | `([sp-40] + ret(0x1b22c))` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xac048 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bgt` at 0xac054 | `[sp-24] > 0x6 (signed, 32-bit)` | `[sp-24] <= 0x6 (signed, 32-bit)` |
| `ble` at 0xac070 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xac094 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xac0b8 | `[[sp-304]]:1 != 0xff` | `[[sp-304]]:1 == 0xff` |
| `ble` at 0xac0d0 | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bne` at 0xac0f4 | `[[sp-740]]:1 != 0xff` | `[[sp-740]]:1 == 0xff` |
| `beq` at 0xac140 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |
| `beq` at 0xac178 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xac1ac | `[sp-24] <= 0x0 (signed, 32-bit)` | `[sp-24] > 0x0 (signed, 32-bit)` |
| `bge` at 0xac1e0 | `[sp-24] >= 0x0 (signed, 32-bit)` | `[sp-24] < 0x0 (signed, 32-bit)` |
| `bne` at 0xac274 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `beq` at 0xac2cc | `[sp-752]:1 == 0x0` | `[sp-752]:1 != 0x0` |
| `beq` at 0xac2e4 | `[sp-752]:1 == 0xff` | `[sp-752]:1 != 0xff` |
| `bne` at 0xac374 | `(ret(0xad730) & 0xff) != 0x0` | `(ret(0xad730) & 0xff) == 0x0` |
| `bne` at 0xac44c | `(ret(0xad730) & 0xff) != 0x0` | `(ret(0xad730) & 0xff) == 0x0` |
| `ble` at 0xac480 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xac4a4 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xac574 | `? >= [[sp-720]+1321376]:1 (unsigned, 32-bit)` | `? < [[sp-720]+1321376]:1 (unsigned, 32-bit)` |
| `beq` at 0xac620 | `[sp-753]:1 == 0x0` | `[sp-753]:1 != 0x0` |
| `beq` at 0xac638 | `[sp-753]:1 == 0xff` | `[sp-753]:1 != 0xff` |
| `bne` at 0xac6c8 | `(ret(0xad730) & 0xff) != 0x0` | `(ret(0xad730) & 0xff) == 0x0` |
| `beq` at 0xac75c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0xac774 | `[sp-751]:1 == 0x0` | `[sp-751]:1 != 0x0` |
| `beq` at 0xac78c | `[sp-751]:1 == 0xff` | `[sp-751]:1 != 0xff` |
| `beq` at 0xac80c | `(ret(0xad730) & 0xff) == 0x8` | `(ret(0xad730) & 0xff) != 0x8` |
| `beq` at 0xac844 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xac8fc | `(ret(0xad730) & 0xff) == [sp-762]:1` | `(ret(0xad730) & 0xff) != [sp-762]:1` |
| `beq` at 0xac978 | `? == 0x0` | `? != 0x0` |
| `blt` at 0xaca24 | `? < 0x0 (signed, 32-bit)` | `? >= 0x0 (signed, 32-bit)` |
| `ble` at 0xaca40 | `[sp-720] <= 0x4 (signed, 32-bit)` | `[sp-720] > 0x4 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaca50 | `r0` | `?` |

## `PBaySdrGetConnector`

- `PBaySdrGetConnector`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadbc0 to 0xadcec.
  Established. Two byte stores have unknown effective targets, writing `0x0` and `([[sp-440]]:1 &
  0xff)` (`strb` at 0xadbe0, `strb` at 0xadccc). The opening call to `0xaa7dc` carries `[sp-20]`,
  `sp-428`, `0xfffffe6c`, `sp-428` (`bl` at 0xadc3c); its zero result takes the bypass and nonzero
  falls through (`beq` at 0xadc50). Connector encoding and final r0 remain unresolved (`ldmdb` at
  0xadce0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadbd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xadbe0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xadbe4 | load | 4 | `abs 0xadce4` | `-` |
| `ldr` at 0xadbe8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xadbec | load | 2 | `[sp-20] +0x6` | `-` |
| `strh` at 0xadbf4 | store | 2 | `sp -0x1ac` | `([[sp-20]+6]:2 & 0xffff)` |
| `str` at 0xadbfc | store | 4 | `sp -0x1a8` | `0xa` |
| `str` at 0xadc04 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xadc0c | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xadc14 | store | 4 | `sp -0x114` | `0x1` |
| `ldr` at 0xadc18 | load | 4 | `abs 0xadce8` | `-` |
| `strb` at 0xadc24 | store | 1 | `sp -0x1a4` | `0x0` |
| `str` at 0xadc2c | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xadc34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xadc44 | store | 4 | `sp -0x1b0` | `ret(0xaa7dc)` |
| `ldr` at 0xadc48 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xadc54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xadc58 | store | 4 | `sp -0x1b4` | `[sp-36]` |
| `ldr` at 0xadc5c | load | 4 | `sp -0x1b4` | `-` |
| `ldrb` at 0xadc60 | load | 1 | `[sp-36] +0x0` | `-` |
| `str` at 0xadc74 | store | 4 | `sp -0x1a8` | `0xb` |
| `str` at 0xadc7c | store | 4 | `sp -0x114` | `0x0` |
| `ldr` at 0xadc84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xadc94 | store | 4 | `sp -0x1b0` | `ret(0xaa7dc)` |
| `ldr` at 0xadc98 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xadca4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xadca8 | store | 4 | `sp -0x1b8` | `[sp-36]` |
| `ldr` at 0xadcac | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xadcb0 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xadcc0 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xadcc4 | load | 1 | `[sp-440] +0x0` | `-` |
| `strb` at 0xadccc | store | 1 | `(unknown)` | `([[sp-440]]:1 & 0xff)` |
| `ldrb` at 0xadcd8 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xadce0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xadce0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xadce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xadc3c | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-428` | `0xfffffe6c` | `sp-428` |
| `bl` at 0xadc8c | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-428` | `?` | `sp-428` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xadc50 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |
| `beq` at 0xadc6c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xadca0 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |
| `beq` at 0xadcb8 | `[[sp-36]]:1 == 0x0` | `[[sp-36]]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xadce0 | `r0` | `?` |

## `SdrSetChassisFru`

- `SdrSetChassisFru`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaddb8 to 0xade0c.
  Established. The first byte comparison takes its bypass for `[0x142950]:1 != 0xff` and falls
  through on equality (`bne` at 0xadddc). Fall-through byte stores write `((r0 & 0xff) & 0xff)` at
  `0x142950` and `0x0` at `0x142951` and `0x142952` (`strb` at 0xadde8, `strb` at 0xaddf4, `strb` at
  0xade00). The exit retains entry r0 (`ldmdb` at 0xade04). Chassis-FRU encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xaddcc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xaddd0 | load | 4 | `abs 0xade08` | `-` |
| `ldrb` at 0xaddd4 | load | 1 | `abs 0x142950` | `-` |
| `ldr` at 0xadde0 | load | 4 | `abs 0xade08` | `-` |
| `ldrb` at 0xadde4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xadde8 | store | 1 | `abs 0x142950` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xaddec | load | 4 | `abs 0xade08` | `-` |
| `strb` at 0xaddf4 | store | 1 | `abs 0x142951` | `0x0` |
| `ldr` at 0xaddf8 | load | 4 | `abs 0xade08` | `-` |
| `strb` at 0xade00 | store | 1 | `abs 0x142952` | `0x0` |
| `ldmdb` at 0xade04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xade04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xade04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xadddc | `[0x142950]:1 != 0xff` | `[0x142950]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xade04 | `r0` | `r0` |

## `PBaySdrGetNbrDps`

- `PBaySdrGetNbrDps`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae090 to 0xae1bc.
  Established. A byte store at an unknown target writes `0x1` (`strb` at 0xae0fc). A later byte
  store at `([sp-24] + [sp-32]) +0x0` carries `(([ret(0xae3dc)+2]:1 & 0xff) & 0xff)` (`strb` at
  0xae178). The first call to `0xad4f4` has unknown, `sp-452`, unknown, unknown (`bl` at 0xae11c).
  Counting semantics and the exit result remain unidentified (`ldmdb` at 0xae1b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae0a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae0a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae0ac | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xae0b4 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xae0b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae0c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae0d0 | load | 4 | `abs 0xae1b8` | `-` |
| `str` at 0xae0d8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xae0dc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xae0e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xae0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae0f0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xae0fc | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xae108 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xae10c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xae110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae124 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae134 | store | 4 | `sp -0x1c8` | `ret(0xaa7dc)` |
| `ldr` at 0xae138 | load | 4 | `sp -0x1c8` | `-` |
| `str` at 0xae148 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xae150 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xae158 | store | 4 | `sp -0x28` | `ret(0xae3dc)` |
| `ldr` at 0xae15c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xae160 | load | 1 | `ret(0xae3dc) +0x2` | `-` |
| `strb` at 0xae164 | store | 1 | `sp -0x29` | `([ret(0xae3dc)+2]:1 & 0xff)` |
| `ldr` at 0xae168 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae16c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xae174 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xae178 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `(([ret(0xae3dc)+2]:1 & 0xff) & 0xff)` |
| `ldrb` at 0xae17c | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xae18c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xae194 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xae198 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xae1a0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xae1a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xae1ac | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xae1b0 | load | 4 | `sp -0x1cc` | `-` |
| `ldmdb` at 0xae1b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae1b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae1b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae11c | `(unknown callee) 0xad4f4` | `?` | `sp-452` | `?` | `?` |
| `bl` at 0xae12c | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-452` | `?` | `sp-452` |
| `bl` at 0xae154 | `(unknown callee) 0xae3dc` | `[sp-60]` | `?` | `?` | `ret(0xaa7dc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xae0c0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xae0e8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xae140 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `bne` at 0xae188 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae1b4 | `r0` | `?` |

## `L1FRU_2m_Init`

- `L1FRU_2m_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6740 to 0xa69b8.
  Established. Two word stores advance absolute `0x14998c` to `[0x14998c]+1` on known edges (`str`
  at 0xa6928, `str` at 0xa695c). A separate word store writes `0x3` at absolute `0x2b1ae8` (`str` at
  0xa6944). The entry global test takes its continuation for `[0x149994] != 0x0` and falls through
  on zero (`bne` at 0xa675c). The computed selector remains unresolved (`ldrls` at 0xa6784). FRU
  initialization paths behind that selector and r0 at the recognized exit are unknown (`ldmdb` at
  0xa6960).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa6750 | load | 4 | `abs 0xa6964` | `-` |
| `ldr` at 0xa6754 | load | 4 | `abs 0x149994` | `-` |
| `ldr` at 0xa6760 | load | 4 | `abs 0xa6968` | `-` |
| `ldrls` at 0xa6784 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa68ec | load | 4 | `abs 0xa69ac` | `-` |
| `ldr` at 0xa68f0 | load | 4 | `abs 0x14998c` | `-` |
| `ldr` at 0xa68fc | load | 4 | `abs 0xa69b0` | `-` |
| `str` at 0xa6900 | store | 4 | `sp -0x1c` | `0x126614` |
| `ldr` at 0xa6908 | load | 4 | `abs 0xa69b4` | `-` |
| `ldr` at 0xa6918 | load | 4 | `abs 0xa69ac` | `-` |
| `ldr` at 0xa691c | load | 4 | `abs 0xa69ac` | `-` |
| `ldr` at 0xa6920 | load | 4 | `abs 0x14998c` | `-` |
| `str` at 0xa6928 | store | 4 | `abs 0x14998c` | `[0x14998c]+1` |
| `ldr` at 0xa692c | load | 4 | `abs 0xa6968` | `-` |
| `ldr` at 0xa6930 | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa693c | load | 4 | `abs 0xa6968` | `-` |
| `str` at 0xa6944 | store | 4 | `abs 0x2b1ae8` | `0x3` |
| `ldr` at 0xa694c | load | 4 | `abs 0xa69ac` | `-` |
| `ldr` at 0xa6950 | load | 4 | `abs 0xa69ac` | `-` |
| `ldr` at 0xa6954 | load | 4 | `abs 0x14998c` | `-` |
| `str` at 0xa695c | store | 4 | `abs 0x14998c` | `[0x14998c]+1` |
| `ldmdb` at 0xa6960 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6960 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6960 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6960 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa676c | `(unknown callee) 0x1b43c` | `0x2b1ae0` | `0x0` | `0x2c` | `[0x149994]` |
| `bl` at 0xa6770 | `(unknown callee) 0x4f090` | `ret(0x1b43c)` | `?` | `?` | `?` |
| `bl` at 0xa6914 | `(unknown callee) 0x21e28` | `0x9` | `0x126634` | `0x7e` | `0x0` |
| `bl` at 0xa6948 | `(unknown callee) 0xbd330` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa675c | `[0x149994] != 0x0` | `[0x149994] == 0x0` |
| `ldrls` at 0xa6784 | `? <= 0x1a (unsigned, 32-bit)` | `? > 0x1a (unsigned, 32-bit)` |
| `beq` at 0xa68f8 | `[0x14998c] == 0x0` | `[0x14998c] != 0x0` |
| `bne` at 0xa6938 | `[0x2b1ae8] != 0x0` | `[0x2b1ae8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6960 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa6784).

## `L1FRU_2m_Capture`

- `L1FRU_2m_Capture`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6a1c to 0xa6c2c.
  Established. A word store writes `ret(0x1b1e4)` at absolute `0x2b1aec` (`str` at 0xa6a3c). Other
  arms write `0x1` and `0x2` at absolute `0x2b1af0` (`str` at 0xa6ab0, `str` at 0xa6ac0). The
  initial call to `0x1b1e4` carries `0x0` and entry r1, r2, r3 (`bl` at 0xa6a38). The first global
  test takes its bypass for `[0x2b1b00] != 0x0` and falls through on zero (`bne` at 0xa6a4c).
  Capture-field meanings remain unestablished, with exit r0 `[sp-32]` (`ldmdb` at 0xa6c24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6a2c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa6a30 | load | 4 | `abs 0xa6c28` | `-` |
| `str` at 0xa6a3c | store | 4 | `abs 0x2b1aec` | `ret(0x1b1e4)` |
| `ldr` at 0xa6a40 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6a44 | load | 4 | `abs 0x2b1b00` | `-` |
| `ldr` at 0xa6a50 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6a54 | load | 4 | `abs 0x2b1b04` | `-` |
| `str` at 0xa6a68 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa6a70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6a7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6a90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6aa8 | load | 4 | `abs 0xa6c28` | `-` |
| `str` at 0xa6ab0 | store | 4 | `abs 0x2b1af0` | `0x1` |
| `ldr` at 0xa6ab8 | load | 4 | `abs 0xa6c28` | `-` |
| `str` at 0xa6ac0 | store | 4 | `abs 0x2b1af0` | `0x2` |
| `ldr` at 0xa6ac4 | load | 4 | `abs 0xa6c28` | `-` |
| `str` at 0xa6acc | store | 4 | `abs 0x2b1ae8` | `0x0` |
| `str` at 0xa6ad4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa6ad8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6adc | load | 4 | `abs 0x2b1ae8` | `-` |
| `ldr` at 0xa6ae8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6af0 | load | 4 | `abs 0x2b1af4` | `-` |
| `ldr` at 0xa6b00 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6b04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6b18 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6b20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6b24 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa6b28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa6b34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa6b40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa6b54 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6b6c | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6b80 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6b84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6b98 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6ba8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6bb0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa6bb8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6bbc | load | 4 | `abs 0x2b1b00` | `-` |
| `ldr` at 0xa6bc8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6bcc | load | 4 | `abs 0x2b1b00` | `-` |
| `ldr` at 0xa6bd8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6bdc | load | 4 | `abs 0x2b1b04` | `-` |
| `ldr` at 0xa6be8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6bec | load | 4 | `abs 0x2b1b04` | `-` |
| `ldr` at 0xa6bf8 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6bfc | load | 4 | `abs 0x2b1b08` | `-` |
| `ldr` at 0xa6c08 | load | 4 | `abs 0xa6c28` | `-` |
| `ldr` at 0xa6c0c | load | 4 | `abs 0x2b1b08` | `-` |
| `str` at 0xa6c1c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xa6c20 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xa6c24 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa6c24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6c24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6c24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6a38 | `(unknown callee) 0x1b1e4` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa6b78 | `(unknown callee) 0xa81a8` | `?` | `0x2b1ae0` | `?` | `?` |
| `bl` at 0xa6ba4 | `(unknown callee) 0xa92bc` | `?` | `0x2b1ae0` | `?` | `?` |
| `bl` at 0xa6bd0 | `(unknown callee) 0x1b400` | `[0x2b1b00]` | `?` | `?` | `0x2b1ae0` |
| `bl` at 0xa6bd4 | `(unknown callee) 0xa8148` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6bf0 | `(unknown callee) 0x1b400` | `[0x2b1b04]` | `?` | `?` | `0x2b1ae0` |
| `bl` at 0xa6bf4 | `(unknown callee) 0xa8168` | `?` | `?` | `?` | `?` |
| `bl` at 0xa6c10 | `(unknown callee) 0x1b400` | `[0x2b1b08]` | `?` | `?` | `0x2b1ae0` |
| `bl` at 0xa6c14 | `(unknown callee) 0xa8188` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa6a4c | `[0x2b1b00] != 0x0` | `[0x2b1b00] == 0x0` |
| `bne` at 0xa6a5c | `[0x2b1b04] != 0x0` | `[0x2b1b04] == 0x0` |
| `blt` at 0xa6a78 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0xa6a8c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa6aa4 | `? != 0x5` | `? == 0x5` |
| `bne` at 0xa6ae4 | `[0x2b1ae8] != 0x0` | `[0x2b1ae8] == 0x0` |
| `blt` at 0xa6af8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa6b30 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0xa6b3c | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0xa6b48 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0xa6bc4 | `[0x2b1b00] == 0x0` | `[0x2b1b00] != 0x0` |
| `beq` at 0xa6be4 | `[0x2b1b04] == 0x0` | `[0x2b1b04] != 0x0` |
| `beq` at 0xa6c04 | `[0x2b1b08] == 0x0` | `[0x2b1b08] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6c24 | `r0` | `[sp-32]` |

## `L1FRU_2m_Deadlock`

- `L1FRU_2m_Deadlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa6c2c to 0xa6cf8.
  Established. The call to `0xa8734` has unknown, `[sp-20]`, `[sp-24]`, unknown (`bl` at 0xa6cd4).
  The prior equality branch takes its continuation for `[sp-32] == 0x1` and falls through
  otherwise (`beq` at 0xa6c94). The loop branch retains only `lt flags condition (operands unknown)`
  and its negation (`blt` at 0xa6c5c). The exit reports r0 `0x0` (`ldmdb` at 0xa6cf0).
  Deadlock-detection semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa6c3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa6c40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa6c48 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa6c4c | load | 4 | `abs 0xa6cf4` | `-` |
| `ldr` at 0xa6c50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6c54 | load | 4 | `abs 0x2b1af4` | `-` |
| `ldr` at 0xa6c64 | load | 4 | `abs 0xa6cf4` | `-` |
| `ldr` at 0xa6c68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6c7c | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6c84 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa6c88 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa6c8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6c98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa6ca8 | load | 4 | `abs 0xa6cf4` | `-` |
| `ldr` at 0xa6cac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa6cc0 | load | 4 | `abs 0x2b1af8` | `-` |
| `ldr` at 0xa6cc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa6ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa6cd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa6cd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa6ce0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0xa6cf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa6cf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa6cf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa6cd4 | `(unknown callee) 0xa8734` | `?` | `[sp-20]` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa6c5c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xa6c94 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `blt` at 0xa6ca0 | `[sp-32] < 0x1 (signed, 32-bit)` | `[sp-32] >= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa6cf0 | `r0` | `0x0` |
