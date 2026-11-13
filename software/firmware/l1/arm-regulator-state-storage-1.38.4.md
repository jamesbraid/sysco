# ARM regulator state and storage helpers in 1.38.4

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
vramSet and vramGet have unresolved computed selectors. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/regulator-state-storage-facts.txt) retain all tool sections.

## `vrmInit`

- `vrmInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c2c4 to 0x5c4e0.
  Established. A word load reads absolute `0x14838c` (`ldr` at 0x5c2e4); its zero value takes the
  first branch and nonzero falls through (`beq` at 0x5c2ec). The opening call to `0x21e28` carries
  `0x9`, `0x10bee4`, `0x36` and `0x0` (`bl` at 0x5c32c). The regulator record geometry and
  initialisation effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c2d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5c2d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5c2dc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x5c2e0 | load | 4 | `abs 0x5c4c0` | `-` |
| `ldr` at 0x5c2e4 | load | 4 | `abs 0x14838c` | `-` |
| `ldr` at 0x5c2f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c2fc | load | 4 | `abs 0x5c4c4` | `-` |
| `str` at 0x5c300 | store | 4 | `sp -0x34` | `0x10beb4` |
| `ldr` at 0x5c304 | load | 4 | `abs 0x5c4c0` | `-` |
| `ldr` at 0x5c308 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c30c | store | 4 | `sp -0x30` | `[0x14838c]` |
| `ldr` at 0x5c310 | load | 4 | `abs 0x5c4c8` | `-` |
| `ldr` at 0x5c314 | load | 4 | `abs 0x148390` | `-` |
| `str` at 0x5c318 | store | 4 | `sp -0x2c` | `[0x148390]` |
| `ldr` at 0x5c320 | load | 4 | `abs 0x5c4cc` | `-` |
| `ldr` at 0x5c330 | load | 4 | `abs 0x5c4c8` | `-` |
| `ldr` at 0x5c334 | load | 4 | `abs 0x148390` | `-` |
| `ldr` at 0x5c340 | load | 4 | `abs 0x5c4c8` | `-` |
| `str` at 0x5c348 | store | 4 | `abs 0x148390` | `0x1` |
| `ldr` at 0x5c34c | load | 4 | `abs 0x5c4c8` | `-` |
| `ldr` at 0x5c350 | load | 4 | `abs 0x148390` | `-` |
| `str` at 0x5c364 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5c368 | load | 4 | `abs 0x5c4c0` | `-` |
| `ldr` at 0x5c36c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5c37c | store | 4 | `abs 0x14838c` | `ret(0x1b3c4)` |
| `ldr` at 0x5c380 | load | 4 | `abs 0x14838c` | `-` |
| `ldr` at 0x5c38c | load | 4 | `abs 0x5c4d0` | `-` |
| `str` at 0x5c390 | store | 4 | `sp -0x34` | `0x10bef8` |
| `ldr` at 0x5c394 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5c398 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x5c3a0 | load | 4 | `abs 0x5c4cc` | `-` |
| `ldr` at 0x5c3b0 | load | 4 | `abs 0x5c4c0` | `-` |
| `ldr` at 0x5c3b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c3c8 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c3d0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5c3d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5c3d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5c3dc | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x5c3e0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5c3e4 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0x5c3e8 | store | 1 | `[sp-36] +0x8` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0x5c3ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5c3f0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5c3f4 | load | 1 | `[sp-36] +0x8` | `-` |
| `str` at 0x5c408 | store | 4 | `[sp-36] +0x4` | `ret(0x1b3c4)` |
| `ldr` at 0x5c40c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x5c418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c420 | load | 4 | `abs 0x5c4d4` | `-` |
| `ldr` at 0x5c424 | load | 4 | `abs 0x5c4d8` | `-` |
| `str` at 0x5c450 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5c454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c45c | load | 4 | `abs 0x5c4d4` | `-` |
| `ldr` at 0x5c460 | load | 4 | `abs 0x5c4d8` | `-` |
| `ldr` at 0x5c46c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5c47c | load | 4 | `abs 0x5c4dc` | `-` |
| `str` at 0x5c480 | store | 4 | `(unknown)` | `0x10bf3c` |
| `str` at 0x5c484 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x5c488 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5c48c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x5c494 | load | 4 | `abs 0x5c4cc` | `-` |
| `ldr` at 0x5c4a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5c4a8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5c4ac | load | 1 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x5c4b0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0x5c4bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c4bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c4bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c4bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c32c | `(unknown callee) 0x21e28` | `0x9` | `0x10bee4` | `0x36` | `0x0` |
| `bl` at 0x5c374 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `[0x148390]` | `?` |
| `bl` at 0x5c3ac | `(unknown callee) 0x21e28` | `0x9` | `0x10bee4` | `0x42` | `0x0` |
| `bl` at 0x5c400 | `(unknown callee) 0x1b3c4` | `[[sp-36]+8]:1` | `0x1` | `[sp-36]` | `[[sp-36]+8]:1` |
| `bl` at 0x5c428 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c434 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c464 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c474 | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c4a0 | `(unknown callee) 0x21e28` | `0x9` | `0x10bee4` | `0x4d` | `0x0` |
| `bl` at 0x5c4b8 | `(unknown callee) 0x1b43c` | `[[sp-36]+4]` | `0x1` | `[[sp-36]+8]:1` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5c2ec | `[0x14838c] == 0x0` | `[0x14838c] != 0x0` |
| `bne` at 0x5c2f8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x5c33c | `[0x148390] != 0x0` | `[0x148390] == 0x0` |
| `bne` at 0x5c388 | `[0x14838c] != 0x0` | `[0x14838c] == 0x0` |
| `bne` at 0x5c414 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c4bc | `r0` | `ret(0x1b43c)` |

## `vrmDisable`

- `vrmDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c4e0 to 0x5c5f4.
  Established. The unsigned branch takes `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` and falls
  through below that bound (`bhs` at 0x5c530). A fall-through byte store writes `0x0` to `((r1 &
  0xff) + [[sp-28]+4]) +0x0` (`strb` at 0x5c548). The external disable effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c4f0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5c4f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5c4fc | load | 4 | `abs 0x5c5e0` | `-` |
| `ldr` at 0x5c500 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c514 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c51c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5c520 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5c524 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5c528 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5c534 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5c538 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5c53c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x5c548 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x0` |
| `ldr` at 0x5c550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c558 | load | 4 | `abs 0x5c5e4` | `-` |
| `ldr` at 0x5c55c | load | 4 | `abs 0x5c5e8` | `-` |
| `str` at 0x5c588 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5c58c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c594 | load | 4 | `abs 0x5c5e4` | `-` |
| `ldr` at 0x5c598 | load | 4 | `abs 0x5c5e8` | `-` |
| `ldr` at 0x5c5a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c5b4 | load | 4 | `abs 0x5c5ec` | `-` |
| `str` at 0x5c5b8 | store | 4 | `(unknown)` | `0x10bf78` |
| `str` at 0x5c5bc | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x5c5c0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5c5c4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x5c5cc | load | 4 | `abs 0x5c5f0` | `-` |
| `ldmdb` at 0x5c5dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c5dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c5dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c560 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c56c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c59c | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c5ac | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c5d8 | `(unknown callee) 0x21e28` | `0x6` | `0x10bee4` | `0x5c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5c530 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c5dc | `r0` | `?` |

## `vrmEnable`

- `vrmEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c5f4 to 0x5c708.
  Established. The range decision takes `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` and falls
  through below the bound (`bhs` at 0x5c644). The fall-through byte store writes `0x1` at `((r1 &
  0xff) + [[sp-28]+4]) +0x0` (`strb` at 0x5c65c). The value's hardware enable meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c604 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5c60c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5c610 | load | 4 | `abs 0x5c6f4` | `-` |
| `ldr` at 0x5c614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c628 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c630 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5c634 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5c638 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5c63c | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5c648 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5c64c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5c650 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x5c65c | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x1` |
| `ldr` at 0x5c664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c66c | load | 4 | `abs 0x5c6f8` | `-` |
| `ldr` at 0x5c670 | load | 4 | `abs 0x5c6fc` | `-` |
| `str` at 0x5c69c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5c6a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c6a8 | load | 4 | `abs 0x5c6f8` | `-` |
| `ldr` at 0x5c6ac | load | 4 | `abs 0x5c6fc` | `-` |
| `ldr` at 0x5c6b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c6c8 | load | 4 | `abs 0x5c700` | `-` |
| `str` at 0x5c6cc | store | 4 | `(unknown)` | `0x10bf94` |
| `str` at 0x5c6d0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0x5c6d4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x5c6d8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x5c6e0 | load | 4 | `abs 0x5c704` | `-` |
| `ldmdb` at 0x5c6f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c6f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c6f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c674 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c680 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c6b0 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c6c0 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c6ec | `(unknown callee) 0x21e28` | `0x6` | `0x10bee4` | `0x6a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5c644 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c6f0 | `r0` | `?` |

## `vrmGetCfg`

- `vrmGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c748 to 0x5c7a8.
  Established. A word read uses absolute `0x14838c` (`ldr` at 0x5c778). The word store writes
  `[[sp-28]]` at `r1 +0x0` (`str` at 0x5c790). The recorded return is the byte expression
  `[[sp-28]+8]:1` (`ldmdb` at 0x5c7a0). The configuration layout remains unknown beyond these
  accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c758 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5c75c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5c760 | load | 4 | `abs 0x5c7a4` | `-` |
| `ldr` at 0x5c764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5c778 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c780 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5c784 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5c788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c78c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5c790 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x5c794 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5c798 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldmdb` at 0x5c7a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c7a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c7a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c7a0 | `r0` | `[[sp-28]+8]:1` |

## `vrmIntEnable`

- `vrmIntEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c7a8 to 0x5c95c.
  Established. A byte store sets `[sp-32] +0x9` to `0x1` (`strb` at 0x5c7e8). The opening loop
  comparison has unknown operands (`blt` at 0x5c804). The call to `0xd6d40` carries `[sp-28]`
  followed by three unknown candidates (`bl` at 0x5c86c). Interrupt routing and polarity remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c7b8 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5c7bc | load | 4 | `abs 0x5c948` | `-` |
| `ldr` at 0x5c7c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c7d4 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c7dc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5c7e0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x5c7e8 | store | 1 | `[sp-32] +0x9` | `0x1` |
| `str` at 0x5c7f0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5c7f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5c7f8 | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x5c7fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5c80c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c810 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5c814 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5c81c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5c828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c82c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5c834 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5c83c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5c840 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5c844 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5c848 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5c84c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5c850 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5c854 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5c860 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5c874 | store | 4 | `sp -0x24` | `ret(0xd6d40)` |
| `ldr` at 0x5c878 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5c884 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c88c | load | 4 | `abs 0x5c94c` | `-` |
| `ldr` at 0x5c890 | load | 4 | `abs 0x5c950` | `-` |
| `str` at 0x5c8bc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5c8c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c8c8 | load | 4 | `abs 0x5c94c` | `-` |
| `ldr` at 0x5c8cc | load | 4 | `abs 0x5c950` | `-` |
| `ldr` at 0x5c8d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5c8e8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5c8ec | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x5c8f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5c90c | load | 4 | `abs 0x5c954` | `-` |
| `str` at 0x5c910 | store | 4 | `(unknown)` | `0x10bfac` |
| `str` at 0x5c914 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x5c918 | store | 4 | `(unknown)` | `ret(0xd7558)` |
| `str` at 0x5c91c | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x5c924 | load | 4 | `abs 0x5c958` | `-` |
| `ldr` at 0x5c934 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5c93c | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x5c944 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5c944 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5c944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5c944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5c944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5c86c | `(unknown callee) 0xd6d40` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5c894 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c8a0 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c8d0 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5c8e0 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5c8f8 | `(unknown callee) 0xd7558` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x5c904 | `(unknown callee) 0x219a8` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x5c930 | `(unknown callee) 0x21e28` | `0x3` | `0x10bee4` | `0xa3` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5c804 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5c824 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x5c880 | `ret(0xd6d40) == 0x0` | `ret(0xd6d40) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5c944 | `r0` | `?` |

## `vrmIntDisable`

- `vrmIntDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5c95c to 0x5cb50.
  Established. The byte at `[sp-32] +0x9` is cleared (`strb` at 0x5c99c). The loop comparison
  retains unknown operands (`blt` at 0x5c9b8). The first call to `0xd6928` receives `[sp-28]`,
  unknown, `[[sp-32]+4]` and unknown (`bl` at 0x5ca20). The hardware disable policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5c96c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5c970 | load | 4 | `abs 0x5cb3c` | `-` |
| `ldr` at 0x5c974 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5c988 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5c990 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5c994 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x5c99c | store | 1 | `[sp-32] +0x9` | `0x0` |
| `str` at 0x5c9a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5c9a8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5c9ac | load | 1 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x5c9b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5c9c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c9c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5c9cc | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x5c9d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5c9d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c9dc | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5c9e0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5c9e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5c9f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5c9f8 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0x5c9fc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x5ca04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ca10 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5ca14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ca18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ca34 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5ca38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ca3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5ca40 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5ca44 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5ca48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5ca54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ca68 | store | 4 | `sp -0x24` | `ret(0xd6cf8)` |
| `ldr` at 0x5ca6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5ca78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ca80 | load | 4 | `abs 0x5cb40` | `-` |
| `ldr` at 0x5ca84 | load | 4 | `abs 0x5cb44` | `-` |
| `str` at 0x5cab0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5cab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5cabc | load | 4 | `abs 0x5cb40` | `-` |
| `ldr` at 0x5cac0 | load | 4 | `abs 0x5cb44` | `-` |
| `ldr` at 0x5cacc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5cadc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5cae0 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x5cae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5caf4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cb00 | load | 4 | `abs 0x5cb48` | `-` |
| `str` at 0x5cb04 | store | 4 | `(unknown)` | `0x10bfe4` |
| `str` at 0x5cb08 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x5cb0c | store | 4 | `(unknown)` | `ret(0xd7558)` |
| `str` at 0x5cb10 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0x5cb18 | load | 4 | `abs 0x5cb4c` | `-` |
| `ldr` at 0x5cb28 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5cb30 | store | 4 | `sp -0x28` | `?` |
| `ldmdb` at 0x5cb38 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5cb38 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5cb38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cb38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cb38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ca20 | `(unknown callee) 0xd6928` | `[sp-28]` | `?` | `[[sp-32]+4]` | `?` |
| `bl` at 0x5ca60 | `(unknown callee) 0xd6cf8` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x5ca88 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5ca94 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5cac4 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5cad4 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5caec | `(unknown callee) 0xd7558` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x5caf8 | `(unknown callee) 0x219a8` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x5cb24 | `(unknown callee) 0x21e28` | `0x3` | `0x10bee4` | `0xbc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5c9b8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5c9f0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5ca0c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5ca2c | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `beq` at 0x5ca74 | `ret(0xd6cf8) == 0x0` | `ret(0xd6cf8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cb38 | `r0` | `?` |

## `vrmPresent`

- `vrmPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cca4 to 0x5cda8.
  Established. The initial bound check takes `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` and
  falls through below it (`bhs` at 0x5ccf4). The call to `0xd6b9c` carries entry r0, `[[sp-36]]:1`,
  `sp-29` and `[[sp-36]]:1` (`bl` at 0x5cd5c). The return expression is `[sp-40]` (`ldmdb` at
  0x5cda0). Presence polarity is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ccb4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5ccbc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5ccc0 | load | 4 | `abs 0x5cda4` | `-` |
| `ldr` at 0x5ccc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5ccd8 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5cce0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5cce4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cce8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5ccec | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5ccf8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5ccfc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5cd00 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x5cd08 | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldr` at 0x5cd14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cd18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5cd20 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5cd28 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5cd2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cd30 | load | 1 | `[sp-36] +0x3` | `-` |
| `str` at 0x5cd40 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x5cd48 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cd4c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x5cd54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5cd70 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x5cd78 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5cd7c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5cd8c | store | 4 | `sp -0x28` | `ret(0x5cc20)` |
| `str` at 0x5cd98 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5cd9c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5cda0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cda0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cda0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cd5c | `(unknown callee) 0xd6b9c` | `r0` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x5cd84 | `(unknown callee) 0x5cc20` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5ccf4 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |
| `beq` at 0x5cd10 | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x5cd38 | `[[sp-36]+3]:1 != 0x0` | `[[sp-36]+3]:1 == 0x0` |
| `beq` at 0x5cd68 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cda0 | `r0` | `[sp-40]` |

## `vrmOK`

- `vrmOK`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cda8 to 0x5cf84.
  Established. The first call to `0x5cca4` uses entry r0 and three copies of `(r1 & 0xff)` (`bl` at
  0x5ce24). The initial unsigned bound branch takes `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned,
  32-bit)` and falls through below it (`bhs` at 0x5cdf8). The reported health semantics and return
  interpretation remain unknown (`ldmdb` at 0x5cf78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cdb8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5cdc0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5cdc4 | load | 4 | `abs 0x5cf7c` | `-` |
| `ldr` at 0x5cdc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cddc | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5cde4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5cde8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cdec | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5cdf0 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5cdfc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5ce00 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5ce04 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x5ce0c | load | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `-` |
| `ldrb` at 0x5ce18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5ce1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ce38 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5ce40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5ce44 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5ce4c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5ce54 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x5ce58 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5ce5c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x5ce64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5ce80 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5ce88 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5ce8c | load | 1 | `[sp-36] +0x1` | `-` |
| `ldr` at 0x5ce90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cea4 | load | 4 | `abs 0x5cf80` | `-` |
| `ldrb` at 0x5cebc | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5cec4 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0x5cec8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cecc | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0x5cedc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cee0 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x5ceec | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5cef0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5cf0c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cf10 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldrb` at 0x5cf1c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5cf20 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5cf3c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x5cf48 | store | 4 | `sp -0x28` | `0x1` |
| `ldrb` at 0x5cf50 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5cf54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5cf64 | store | 4 | `sp -0x28` | `ret(0x5cb9c)` |
| `str` at 0x5cf70 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5cf74 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5cf78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cf78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cf78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ce24 | `(unknown callee) 0x5cca4` | `r0` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` |
| `bl` at 0x5ce6c | `(unknown callee) 0xd6b9c` | `[sp-20]` | `[[sp-36]]:1` | `sp-29` | `[[sp-36]]:1` |
| `bl` at 0x5ce98 | `(unknown callee) 0x40350` | `[sp-20]` | `[[sp-36]+1]:1` | `?` | `[[sp-36]+1]:1` |
| `bl` at 0x5cea8 | `(unknown callee) 0xfb9a0` | `ret(0x40350)` | `0x42400000` | `?` | `ret(0x40350)` |
| `bl` at 0x5cef8 | `(unknown callee) 0x5cb9c` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x5cf28 | `(unknown callee) 0x5cc20` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x5cf5c | `(unknown callee) 0x5cb9c` | `[sp-36]` | `[sp-29]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0x5cdf8 | `(r1 & 0xff) >= [[sp-28]+8]:1 (unsigned, 32-bit)` | `(r1 & 0xff) < [[sp-28]+8]:1 (unsigned, 32-bit)` |
| `beq` at 0x5ce14 | `[((r1 & 0xff) + [[sp-28]+4])]:1 == 0x0` | `[((r1 & 0xff) + [[sp-28]+4])]:1 != 0x0` |
| `bne` at 0x5ce30 | `ret(0x5cca4) != 0x0` | `ret(0x5cca4) == 0x0` |
| `beq` at 0x5ce78 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `beq` at 0x5ceb4 | `ret(0xfb9a0) == 0x0` | `ret(0xfb9a0) != 0x0` |
| `beq` at 0x5ced8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5cee8 | `[[sp-36]+4]:1 == 0x0` | `[[sp-36]+4]:1 != 0x0` |
| `bne` at 0x5cf04 | `ret(0x5cb9c) != 0x0` | `ret(0x5cb9c) == 0x0` |
| `beq` at 0x5cf18 | `[[sp-36]+3]:1 == 0x0` | `[[sp-36]+3]:1 != 0x0` |
| `bne` at 0x5cf34 | `ret(0x5cc20) != 0x0` | `ret(0x5cc20) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cf78 | `r0` | `[sp-40]` |

## `vrmAllPresent`

- `vrmAllPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cf84 to 0x5d108.
  Established. Two word stores initialise `sp -0x20` to `0x1` and `sp -0x24` to `0x0` (`str` at
  0x5cfc0, `str` at 0x5cfc8). A nonzero `ret(0x5cca4)` takes the result branch and zero falls
  through (`bne` at 0x5cffc). The merged return is unknown (`ldmdb` at 0x5d0ec). These local values
  do not establish which physical regulators are present.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cf94 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5cf98 | load | 4 | `abs 0x5d0f0` | `-` |
| `ldr` at 0x5cf9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5cfb0 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5cfb8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5cfc0 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x5cfc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5cfcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5cfd0 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5cfd4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5cfe4 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x5cfe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d000 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d008 | load | 4 | `abs 0x5d0f4` | `-` |
| `ldr` at 0x5d00c | load | 4 | `abs 0x5d0f8` | `-` |
| `str` at 0x5d038 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x5d03c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d044 | load | 4 | `abs 0x5d0f4` | `-` |
| `ldr` at 0x5d048 | load | 4 | `abs 0x5d0f8` | `-` |
| `ldr` at 0x5d054 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5d064 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d070 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d078 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d07c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d08c | load | 4 | `abs 0x5d0fc` | `-` |
| `str` at 0x5d090 | store | 4 | `(unknown)` | `0x10c01c` |
| `str` at 0x5d094 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x5d098 | store | 4 | `(unknown)` | `ret(0x3fff0)` |
| `ldr` at 0x5d09c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d0a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d0a8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d0b0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5d0b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x5d0bc | load | 4 | `abs 0x5d100` | `-` |
| `ldr` at 0x5d0c0 | load | 4 | `abs 0x5d104` | `-` |
| `str` at 0x5d0d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5d0d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d0dc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x5d0e4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5d0ec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5d0ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d0ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d0ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cff0 | `(unknown callee) 0x5cca4` | `?` | `[sp-36]:1` | `[[sp-28]+8]:1` | `[sp-36]:1` |
| `bl` at 0x5d010 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5d01c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5d04c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5d05c | `(unknown callee) 0x1b508` | `[sp-40]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5d084 | `(unknown callee) 0x3fff0` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5d0c8 | `(unknown callee) 0x21e28` | `0x1` | `0x10bee4` | `0x12b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5cfdc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5cffc | `ret(0x5cca4) != 0x0` | `ret(0x5cca4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d0ec | `r0` | `?` |

## `vrmAllOK`

- `vrmAllOK`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d108 to 0x5d2d0.
  Established. Word stores set `sp -0x20` to `0x1` and clear `sp -0x24` (`str` at 0x5d144, `str` at
  0x5d14c). The call to `0x5cda8` carries unknown, `[sp-36]:1`, `[[sp-28]+8]:1` and `[sp-36]:1`
  (`bl` at 0x5d174). Nonzero takes the following branch and zero falls through (`bne` at 0x5d180).
  The aggregate health meaning remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d118 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x5d11c | load | 4 | `abs 0x5d2b8` | `-` |
| `ldr` at 0x5d120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d134 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5d13c | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5d144 | store | 4 | `sp -0x20` | `0x1` |
| `str` at 0x5d14c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5d150 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d154 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d158 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x5d168 | load | 1 | `sp -0x24` | `-` |
| `ldr` at 0x5d16c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d184 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d188 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d190 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d198 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d19c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d1b0 | load | 4 | `abs 0x5d2bc` | `-` |
| `ldr` at 0x5d1c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d1d0 | load | 4 | `abs 0x5d2c0` | `-` |
| `ldr` at 0x5d1d4 | load | 4 | `abs 0x5d2c4` | `-` |
| `str` at 0x5d200 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5d204 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d20c | load | 4 | `abs 0x5d2c0` | `-` |
| `ldr` at 0x5d210 | load | 4 | `abs 0x5d2c4` | `-` |
| `ldr` at 0x5d21c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5d22c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d230 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d238 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d240 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d244 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d254 | load | 4 | `abs 0x5d2c8` | `-` |
| `str` at 0x5d258 | store | 4 | `(unknown)` | `0x10c044` |
| `str` at 0x5d25c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0x5d260 | store | 4 | `(unknown)` | `ret(0x3fff0)` |
| `ldr` at 0x5d264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d268 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5d270 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x5d278 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5d27c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x5d284 | load | 4 | `abs 0x5d2cc` | `-` |
| `str` at 0x5d298 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5d29c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5d2a4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x5d2ac | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5d2b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5d2b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d2b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d2b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d174 | `(unknown callee) 0x5cda8` | `?` | `[sp-36]:1` | `[[sp-28]+8]:1` | `[sp-36]:1` |
| `bl` at 0x5d1a4 | `(unknown callee) 0x40350` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5d1b4 | `(unknown callee) 0xfb9a0` | `ret(0x40350)` | `0x42400000` | `?` | `ret(0x40350)` |
| `bl` at 0x5d1d8 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5d1e4 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x5d214 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x10bf30` | `0x10bf38` |
| `bl` at 0x5d224 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x5d24c | `(unknown callee) 0x3fff0` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5d290 | `(unknown callee) 0x21e28` | `0x1` | `0x10bee4` | `0x148` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d160 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5d180 | `ret(0x5cda8) != 0x0` | `ret(0x5cda8) == 0x0` |
| `bne` at 0x5d1c0 | `ret(0xfb9a0) != 0x0` | `ret(0xfb9a0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d2b4 | `r0` | `?` |

## `vrmInt`

- `vrmInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d2d0 to 0x5d81c.
  Established. Three byte saves retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at `sp -0x15`,
  `sp -0x16` and `sp -0x17` (`strb` at 0x5d2ec, `strb` at 0x5d2f4, `strb` at 0x5d2fc). The first
  loop decision has unknown operands (`blt` at 0x5d33c). The call to `0xd6b9c` uses `0x0`, unknown,
  `sp-97` and unknown (`bl` at 0x5d394). The interrupt policy and merged r0 value remain unknown
  (`ldmdb` at 0x5d7c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5d2e0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5d2ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5d2f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x5d2fc | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x5d300 | load | 4 | `abs 0x5d7cc` | `-` |
| `ldr` at 0x5d304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d318 | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5d320 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x5d328 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x5d32c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5d330 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5d334 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x5d344 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d348 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0x5d350 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x5d358 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x5d35c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d360 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5d364 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5d370 | load | 4 | `abs 0x5d7d0` | `-` |
| `ldr` at 0x5d374 | load | 4 | `abs 0x148df0` | `-` |
| `ldrb` at 0x5d384 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x5d39c | store | 1 | `sp -0x62` | `(ret(0xd6b9c) & 0xff)` |
| `ldrb` at 0x5d3a0 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d3a4 | load | 1 | `sp -0x61` | `-` |
| `ldrb` at 0x5d3b0 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5d3b4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d3cc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d3d0 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5d3d4 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x5d3d8 | load | 4 | `abs 0x5d7d4` | `-` |
| `ldrb` at 0x5d3ec | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d3f0 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5d3f4 | load | 1 | `sp -0x61` | `-` |
| `ldr` at 0x5d3f8 | load | 4 | `abs 0x5d7d8` | `-` |
| `ldr` at 0x5d408 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d40c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d410 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5d424 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5d428 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x5d440 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d444 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x5d454 | load | 4 | `abs 0x5d7dc` | `-` |
| `ldr` at 0x5d458 | load | 4 | `abs 0x148390` | `-` |
| `ldr` at 0x5d464 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d468 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d46c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d480 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d484 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5d488 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5d490 | load | 4 | `abs 0x5d7e0` | `-` |
| `ldr` at 0x5d494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d4a4 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d4a8 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d4ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d4c0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d4c4 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5d4cc | load | 4 | `abs 0x5d7e4` | `-` |
| `str` at 0x5d4e0 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5d4e8 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5d4f0 | load | 4 | `abs 0x5d7e8` | `-` |
| `ldr` at 0x5d4f4 | load | 4 | `abs 0x5d7ec` | `-` |
| `ldr` at 0x5d500 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d504 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d51c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d520 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5d528 | load | 4 | `abs 0x5d7f0` | `-` |
| `str` at 0x5d53c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5d544 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5d54c | load | 4 | `abs 0x5d7f4` | `-` |
| `ldr` at 0x5d550 | load | 4 | `abs 0x5d7f8` | `-` |
| `ldr` at 0x5d560 | load | 4 | `abs 0x5d7d0` | `-` |
| `ldr` at 0x5d564 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x5d574 | load | 4 | `abs 0x5d7fc` | `-` |
| `ldr` at 0x5d580 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d584 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5d588 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d59c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d5a0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5d5a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d5b8 | load | 4 | `abs 0x5d800` | `-` |
| `ldrb` at 0x5d5d0 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5d5d8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d5dc | load | 1 | `[sp-104] +0x4` | `-` |
| `str` at 0x5d5f0 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5d5f8 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5d608 | load | 4 | `abs 0x5d804` | `-` |
| `ldrb` at 0x5d614 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x5d618 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x5d630 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d634 | load | 1 | `[sp-104] +0x5` | `-` |
| `ldr` at 0x5d644 | load | 4 | `abs 0x5d7dc` | `-` |
| `ldr` at 0x5d648 | load | 4 | `abs 0x148390` | `-` |
| `ldr` at 0x5d654 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d658 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d670 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d674 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5d678 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5d680 | load | 4 | `abs 0x5d808` | `-` |
| `ldr` at 0x5d684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d694 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d698 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d69c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d6b0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d6b4 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5d6bc | load | 4 | `abs 0x5d80c` | `-` |
| `str` at 0x5d6d0 | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5d6d8 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5d6e0 | load | 4 | `abs 0x5d7e8` | `-` |
| `ldr` at 0x5d6e4 | load | 4 | `abs 0x5d7ec` | `-` |
| `ldr` at 0x5d6f0 | load | 4 | `abs 0x5d7dc` | `-` |
| `ldr` at 0x5d6f4 | load | 4 | `abs 0x148390` | `-` |
| `ldr` at 0x5d700 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d704 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d71c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d720 | load | 1 | `[sp-104] +0x2` | `-` |
| `str` at 0x5d724 | store | 4 | `sp -0x70` | `[[sp-104]+2]:1` |
| `ldr` at 0x5d72c | load | 4 | `abs 0x5d810` | `-` |
| `ldr` at 0x5d730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d740 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d744 | load | 1 | `[sp-104] +0x1` | `-` |
| `ldr` at 0x5d748 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5d75c | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x5d760 | load | 1 | `[sp-104] +0x2` | `-` |
| `ldr` at 0x5d768 | load | 4 | `abs 0x5d814` | `-` |
| `str` at 0x5d77c | store | 4 | `sp -0x70` | `0x1` |
| `str` at 0x5d784 | store | 4 | `sp -0x6c` | `0x10` |
| `ldr` at 0x5d78c | load | 4 | `abs 0x5d7e8` | `-` |
| `ldr` at 0x5d790 | load | 4 | `abs 0x5d7ec` | `-` |
| `ldr` at 0x5d79c | load | 4 | `abs 0x5d7d0` | `-` |
| `ldr` at 0x5d7a0 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x5d7b0 | load | 4 | `abs 0x5d818` | `-` |
| `ldr` at 0x5d7b8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x5d7c0 | store | 4 | `sp -0x60` | `?` |
| `ldmdb` at 0x5d7c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d7c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d7c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d394 | `(unknown callee) 0xd6b9c` | `0x0` | `?` | `sp-97` | `?` |
| `bl` at 0x5d3e4 | `(unknown callee) 0x2070c` | `0x10c068` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x5d404 | `(unknown callee) 0x2070c` | `0x10c0a4` | `[sp-22]:1` | `[sp-23]:1` | `[sp-97]:1` |
| `bl` at 0x5d430 | `(unknown callee) 0x5cc20` | `?` | `?` | `?` | `?` |
| `bl` at 0x5d474 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d49c | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c0e0` | `[sp-20]` | `ret(0x3fff0)` |
| `bl` at 0x5d4b4 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d4d4 | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c110` | `ret(0x3fff0)` | `[[sp-104]+2]:1` |
| `bl` at 0x5d4f8 | `(unknown callee) 0x7a24c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5d510 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d530 | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c138` | `ret(0x3fff0)` | `[[sp-104]+2]:1` |
| `bl` at 0x5d558 | `(unknown callee) 0x7a24c` | `0x0` | `0x3401` | `0x1a2` | `sp-92` |
| `bl` at 0x5d578 | `(unknown callee) 0x2070c` | `0x10c15c` | `?` | `?` | `?` |
| `bl` at 0x5d5ac | `(unknown callee) 0x40350` | `?` | `?` | `?` | `?` |
| `bl` at 0x5d5bc | `(unknown callee) 0xfb9a0` | `ret(0x40350)` | `0x42400000` | `?` | `ret(0x40350)` |
| `bl` at 0x5d60c | `(unknown callee) 0x7a24c` | `0x0` | `0x3500` | `0x1a4` | `0x10c18c` |
| `bl` at 0x5d620 | `(unknown callee) 0x5cb9c` | `[sp-104]` | `[sp-23]:1` | `?` | `[sp-23]:1` |
| `bl` at 0x5d664 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d68c | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c1a4` | `[sp-20]` | `ret(0x3fff0)` |
| `bl` at 0x5d6a4 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d6c4 | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c1d4` | `ret(0x3fff0)` | `[[sp-104]+2]:1` |
| `bl` at 0x5d6e8 | `(unknown callee) 0x7a24c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5d710 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d738 | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c1fc` | `[sp-20]` | `ret(0x3fff0)` |
| `bl` at 0x5d750 | `(unknown callee) 0x3fff0` | `[sp-20]` | `[[sp-104]+1]:1` | `?` | `[[sp-104]+1]:1` |
| `bl` at 0x5d770 | `(unknown callee) 0x1b49c` | `sp-92` | `0x10c224` | `ret(0x3fff0)` | `[[sp-104]+2]:1` |
| `bl` at 0x5d794 | `(unknown callee) 0x7a24c` | `0x0` | `0x3402` | `0x1a3` | `sp-92` |
| `bl` at 0x5d7b4 | `(unknown callee) 0x2070c` | `0x10c244` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5d33c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5d36c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5d380 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d3ac | `[sp-22]:1 == [sp-97]:1` | `[sp-22]:1 != [sp-97]:1` |
| `bne` at 0x5d3c4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5d420 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5d43c | `ret(0x5cc20) != 0x0` | `ret(0x5cc20) == 0x0` |
| `beq` at 0x5d450 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x5d460 | `[0x148390] <= 0x1 (signed, 32-bit)` | `[0x148390] > 0x1 (signed, 32-bit)` |
| `beq` at 0x5d570 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d598 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x5d5c8 | `ret(0xfb9a0) == 0x0` | `ret(0xfb9a0) != 0x0` |
| `beq` at 0x5d5e8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5d62c | `ret(0x5cb9c) != 0x0` | `ret(0x5cb9c) == 0x0` |
| `beq` at 0x5d640 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x5d650 | `[0x148390] <= 0x1 (signed, 32-bit)` | `[0x148390] > 0x1 (signed, 32-bit)` |
| `ble` at 0x5d6fc | `[0x148390] <= 0x1 (signed, 32-bit)` | `[0x148390] > 0x1 (signed, 32-bit)` |
| `beq` at 0x5d7ac | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d7c8 | `r0` | `?` |

## `vramInit`

- `vramInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e4d8 to 0x5e75c.
  Established. A word store clears `sp -0x24` (`str` at 0x5e4ec). The global word read accesses
  absolute `0x1483f0`, and zero takes the branch while nonzero falls through (`ldr` at 0x5e4f4,
  `beq` at 0x5e4fc). The first call to `0x21e28` has `0x9`, `0x10c300`, `0x71` and `0x0` as
  candidates (`bl` at 0x5e518). The mapped storage role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e4ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5e4f0 | load | 4 | `abs 0x5e72c` | `-` |
| `ldr` at 0x5e4f4 | load | 4 | `abs 0x1483f0` | `-` |
| `ldr` at 0x5e500 | load | 4 | `abs 0x5e730` | `-` |
| `str` at 0x5e504 | store | 4 | `sp -0x34` | `0x10c2e0` |
| `ldr` at 0x5e50c | load | 4 | `abs 0x5e734` | `-` |
| `ldr` at 0x5e51c | load | 4 | `abs 0x5e72c` | `-` |
| `ldr` at 0x5e520 | load | 4 | `abs 0x5e72c` | `-` |
| `ldr` at 0x5e524 | load | 4 | `abs 0x1483f0` | `-` |
| `str` at 0x5e52c | store | 4 | `abs 0x1483f0` | `[0x1483f0]+1` |
| `ldr` at 0x5e530 | load | 4 | `abs 0x5e738` | `-` |
| `ldr` at 0x5e540 | load | 4 | `abs 0x5e73c` | `-` |
| `ldr` at 0x5e544 | load | 4 | `abs 0x5e740` | `-` |
| `str` at 0x5e550 | store | 4 | `sp -0x18` | `ret(0x1b370)` |
| `ldr` at 0x5e554 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e568 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x5e580 | load | 4 | `abs 0x5e744` | `-` |
| `str` at 0x5e584 | store | 4 | `sp -0x34` | `0x10c320` |
| `ldr` at 0x5e588 | load | 4 | `abs 0x5e748` | `-` |
| `str` at 0x5e58c | store | 4 | `sp -0x30` | `0x10c344` |
| `str` at 0x5e590 | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0x5e594 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0x5e598 | store | 4 | `sp -0x28` | `[ret(0x1b0b8)]` |
| `ldr` at 0x5e5a0 | load | 4 | `abs 0x5e734` | `-` |
| `ldr` at 0x5e5b4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5e5b8 | store | 4 | `sp -0x34` | `ret(0x1b370)` |
| `str` at 0x5e5c0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x5e5d8 | store | 4 | `sp -0x1c` | `ret(0x1b0a0)` |
| `ldr` at 0x5e5dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5e5f0 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0x5e608 | load | 4 | `abs 0x5e74c` | `-` |
| `str` at 0x5e60c | store | 4 | `sp -0x34` | `0x10c350` |
| `ldr` at 0x5e610 | load | 4 | `abs 0x5e748` | `-` |
| `str` at 0x5e614 | store | 4 | `sp -0x30` | `0x10c344` |
| `str` at 0x5e618 | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0x5e61c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0x5e620 | store | 4 | `sp -0x28` | `[ret(0x1b0b8)]` |
| `ldr` at 0x5e628 | load | 4 | `abs 0x5e734` | `-` |
| `ldr` at 0x5e638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5e644 | load | 4 | `abs 0x5e750` | `-` |
| `ldr` at 0x5e648 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5e64c | store | 4 | `abs 0x2b08c0` | `[sp-28]` |
| `ldr` at 0x5e650 | load | 4 | `abs 0x5e754` | `-` |
| `ldr` at 0x5e654 | load | 4 | `abs 0x148394` | `-` |
| `ldr` at 0x5e660 | load | 4 | `abs 0x5e754` | `-` |
| `ldr` at 0x5e664 | load | 4 | `abs 0x5e750` | `-` |
| `ldr` at 0x5e668 | load | 4 | `abs 0x2b08c0` | `-` |
| `str` at 0x5e66c | store | 4 | `abs 0x148394` | `[0x2b08c0]` |
| `ldr` at 0x5e68c | load | 4 | `abs 0x5e754` | `-` |
| `ldr` at 0x5e690 | load | 4 | `abs 0x148394` | `-` |
| `ldr` at 0x5e698 | load | 4 | `abs 0x5e754` | `-` |
| `ldr` at 0x5e6a0 | load | 4 | `abs 0x148394` | `-` |
| `ldr` at 0x5e6bc | load | 4 | `abs 0x5e758` | `-` |
| `ldrb` at 0x5e6c0 | load | 1 | `abs 0x148398` | `-` |
| `ldr` at 0x5e6cc | load | 4 | `abs 0x5e758` | `-` |
| `strb` at 0x5e6d4 | store | 1 | `abs 0x148398` | `0x0` |
| `ldr` at 0x5e6d8 | load | 4 | `abs 0x5e738` | `-` |
| `ldr` at 0x5e6dc | load | 4 | `abs 0x5e758` | `-` |
| `ldrb` at 0x5e6e0 | load | 1 | `abs 0x148398` | `-` |
| `ldr` at 0x5e6e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e6f8 | load | 4 | `abs 0x5e738` | `-` |
| `ldr` at 0x5e6fc | load | 4 | `abs 0x5e758` | `-` |
| `ldrb` at 0x5e700 | load | 1 | `abs 0x148398` | `-` |
| `ldr` at 0x5e708 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5e710 | load | 4 | `abs 0x5e758` | `-` |
| `ldr` at 0x5e714 | load | 4 | `abs 0x5e758` | `-` |
| `ldrb` at 0x5e718 | load | 1 | `abs 0x148398` | `-` |
| `strb` at 0x5e720 | store | 1 | `abs 0x148398` | `([0x148398]:1+1 & 0xff)` |
| `ldmdb` at 0x5e728 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5e728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5e728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5e728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5e518 | `(unknown callee) 0x21e28` | `0x9` | `0x10c300` | `0x71` | `0x0` |
| `bl` at 0x5e53c | `(unknown callee) 0x1b43c` | `0x1483f4` | `0x0` | `0x20` | `[0x1483f0]+1` |
| `bl` at 0x5e548 | `(unknown callee) 0x1b370` | `0x10c314` | `0x1002` | `?` | `?` |
| `bl` at 0x5e560 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0x5e56c | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0x5e574 | `(unknown callee) 0x1b0b8` | `ret(0x21674)` | `?` | `?` | `?` |
| `bl` at 0x5e5ac | `(unknown callee) 0x21e28` | `0x3` | `0x10c300` | `0x84` | `0x0` |
| `bl` at 0x5e5d4 | `(unknown callee) 0x1b0a0` | `0x0` | `0x8000` | `0x3` | `0x1` |
| `bl` at 0x5e5e8 | `(unknown callee) 0x1b0b8` | `ret(0x1b0a0)` | `?` | `?` | `ret(0x1b0a0)` |
| `bl` at 0x5e5f4 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0x5e5fc | `(unknown callee) 0x1b0b8` | `ret(0x21674)` | `?` | `?` | `?` |
| `bl` at 0x5e634 | `(unknown callee) 0x21e28` | `0x3` | `0x10c300` | `0x8a` | `0x0` |
| `bl` at 0x5e63c | `(unknown callee) 0x1b004` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x5e670 | `(unknown callee) 0x20c60` | `ret(0x1b0a0)` | `?` | `?` | `?` |
| `bl` at 0x5e688 | `(unknown callee) 0x5e75c` | `?` | `?` | `?` | `?` |
| `bl` at 0x5e6a8 | `(unknown callee) 0x1b304` | `[0x148394]+128` | `[0x148394]` | `0x80` | `[0x148394]+128` |
| `bl` at 0x5e6ac | `(unknown callee) 0x20c60` | `ret(0x1b304)` | `?` | `?` | `?` |
| `bl` at 0x5e70c | `(unknown callee) 0x20240` | `?` | `?` | `0x1483f4` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5e4fc | `[0x1483f0] == 0x0` | `[0x1483f0] != 0x0` |
| `bge` at 0x5e55c | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `bne` at 0x5e5e4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x5e65c | `[0x148394] != 0x0` | `[0x148394] == 0x0` |
| `beq` at 0x5e6b8 | `ret(0x20c60) == 0x0` | `ret(0x20c60) != 0x0` |
| `beq` at 0x5e6c8 | `[0x148398]:1 == 0x0` | `[0x148398]:1 != 0x0` |
| `bne` at 0x5e6f0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5e728 | `r0` | `?` |

## `vramSet`

- `vramSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5e8d4 to 0x5ea88.
  Established. Two word saves preserve entry r0 and entry r1 at `sp -0x18` and `sp -0x1c` (`str` at
  0x5e8e4, `str` at 0x5e8e8). The selector takes `r0-1 <= 0x30 (unsigned, 32-bit)` and falls through
  above that bound (`ldrls` at 0x5e8f8); its taken edges remain unresolved. The known-edge exit
  carries entry r0 (`ldmdb` at 0x5ea80), while the selected write effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5e8e4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5e8e8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x5e8ec | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x5e8f8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x5ea80 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5ea80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ea80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ea80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5e8f8 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ea80 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5e8f8).

## `vramGet`

- `vramGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ea88 to 0x5ec0c.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0x5ea98). The selector takes
  `r0-1 <= 0x30 (unsigned, 32-bit)` and falls through above the bound (`ldrls` at 0x5eaa8). The
  known fall-through path clears `sp -0x18`, with `0x0` recorded at its exit (`str` at 0x5ebfc,
  `ldmdb` at 0x5ec04). Reads on unresolved selected edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5ea98 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5ea9c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x5eaa8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5ebfc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5ec00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5ec04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ec04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ec04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x5eaa8 | `r0-1 <= 0x30 (unsigned, 32-bit)` | `r0-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ec04 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x5eaa8).

## `IP65_Set_MemCfg`

- `IP65_Set_MemCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63c70 to 0x63c8c.
  Established. Two word stores save entry r0 and entry r1 at `sp -0x14` and `sp -0x18` (`str` at
  0x63c80, `str` at 0x63c84). Entry r0 survives to the recorded return (`ldmdb` at 0x63c88). A
  memory-configuration effect is not established by these stack saves.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63c80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x63c84 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0x63c88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63c88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63c88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63c88 | `r0` | `r0` |
