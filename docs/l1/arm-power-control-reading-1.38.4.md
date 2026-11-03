# ARM power control and reading paths in 1.38.4

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
The shutdown and reading routines have unresolved computed transfers. Excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/power-control-reading-facts.txt) retain all tool sections.

## `powerInit`

- `powerInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c1c8 to 0x3c3f8.
  Established. The zero arm of the absolute-word test stores `0x1` at `0x147fd0` (`bne` at 0x3c22c,
  `str` at 0x3c238). The call to `0x1b3c4` carries `[sp-40]`, `0x1`, `[0x147fd0]`, unknown, and its
  result is stored as a word at `0x147fcc` (`bl` at 0x3c264, `str` at 0x3c26c). Later word stores
  include `[sp-32]` at `[sp-44] +0x8`, `[sp-36]` at `[sp-44] +0x10`, and `[sp]` at `[sp-44] +0x14`
  (`str` at 0x3c2d8, `str` at 0x3c2e4, `str` at 0x3c2f0). The r0 return is `0x0`, while physical
  initialization and field meanings remain unknown (`ldmdb` at 0x3c3c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c1d8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3c1dc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3c1e0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x3c1e4 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x3c1e8 | load | 4 | `abs 0x3c3cc` | `-` |
| `ldr` at 0x3c1ec | load | 4 | `abs 0x147fcc` | `-` |
| `ldr` at 0x3c1f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c204 | load | 4 | `abs 0x3c3d0` | `-` |
| `str` at 0x3c208 | store | 4 | `sp -0x3c` | `0x1082cc` |
| `ldr` at 0x3c210 | load | 4 | `abs 0x3c3d4` | `-` |
| `ldr` at 0x3c214 | load | 4 | `abs 0x3c3d8` | `-` |
| `ldr` at 0x3c220 | load | 4 | `abs 0x3c3dc` | `-` |
| `ldr` at 0x3c224 | load | 4 | `abs 0x147fd0` | `-` |
| `ldr` at 0x3c230 | load | 4 | `abs 0x3c3dc` | `-` |
| `str` at 0x3c238 | store | 4 | `abs 0x147fd0` | `0x1` |
| `ldr` at 0x3c23c | load | 4 | `abs 0x3c3dc` | `-` |
| `ldr` at 0x3c240 | load | 4 | `abs 0x147fd0` | `-` |
| `str` at 0x3c254 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3c258 | load | 4 | `abs 0x3c3cc` | `-` |
| `ldr` at 0x3c25c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3c26c | store | 4 | `abs 0x147fcc` | `ret(0x1b3c4)` |
| `ldr` at 0x3c270 | load | 4 | `abs 0x147fcc` | `-` |
| `ldr` at 0x3c27c | load | 4 | `abs 0x3c3e0` | `-` |
| `str` at 0x3c280 | store | 4 | `sp -0x3c` | `0x108304` |
| `ldr` at 0x3c284 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3c288 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x3c290 | load | 4 | `abs 0x3c3d4` | `-` |
| `ldr` at 0x3c294 | load | 4 | `abs 0x3c3e4` | `-` |
| `ldr` at 0x3c2a0 | load | 4 | `abs 0x3c3cc` | `-` |
| `ldr` at 0x3c2a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c2b8 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c2c0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3c2c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c2cc | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0x3c2d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c2d8 | store | 4 | `[sp-44] +0x8` | `[sp-32]` |
| `ldr` at 0x3c2dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c2e4 | store | 4 | `[sp-44] +0x10` | `[sp-36]` |
| `ldr` at 0x3c2e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2ec | load | 4 | `sp +0x0` | `-` |
| `str` at 0x3c2f0 | store | 4 | `[sp-44] +0x14` | `[sp]` |
| `ldr` at 0x3c2f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c2fc | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x3c30c | store | 4 | `[sp-44] +0x4` | `ret(0x1b3c4)` |
| `ldr` at 0x3c310 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3c31c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c324 | load | 4 | `abs 0x3c3e8` | `-` |
| `ldr` at 0x3c328 | load | 4 | `abs 0x3c3ec` | `-` |
| `str` at 0x3c354 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3c358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c360 | load | 4 | `abs 0x3c3e8` | `-` |
| `ldr` at 0x3c364 | load | 4 | `abs 0x3c3ec` | `-` |
| `ldr` at 0x3c370 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c380 | load | 4 | `abs 0x3c3f0` | `-` |
| `str` at 0x3c384 | store | 4 | `(unknown)` | `0x108350` |
| `str` at 0x3c388 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3c38c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3c390 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x3c398 | load | 4 | `abs 0x3c3d4` | `-` |
| `ldr` at 0x3c39c | load | 4 | `abs 0x3c3f4` | `-` |
| `ldr` at 0x3c3a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c3ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c3b0 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3c3b8 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldmdb` at 0x3c3c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3c3c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c3c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c3c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c21c | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x1a9` | `0x0` |
| `bl` at 0x3c264 | `(unknown callee) 0x1b3c4` | `[sp-40]` | `0x1` | `[0x147fd0]` | `?` |
| `bl` at 0x3c29c | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x1b3` | `0x0` |
| `bl` at 0x3c304 | `(unknown callee) 0x1b3c4` | `[[sp-44]+8]` | `0x1` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x3c32c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c338 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c368 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c378 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c3a4 | `(unknown callee) 0x21e28` | `0x9` | `0x1082ec` | `0x1c1` | `0x0` |
| `bl` at 0x3c3bc | `(unknown callee) 0x1b43c` | `[[sp-44]+4]` | `0x1` | `[[sp-44]+8]` | `[sp-44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c1f4 | `[0x147fcc] == 0x0` | `[0x147fcc] != 0x0` |
| `bne` at 0x3c200 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x3c22c | `[0x147fd0] != 0x0` | `[0x147fd0] == 0x0` |
| `bne` at 0x3c278 | `[0x147fcc] != 0x0` | `[0x147fcc] == 0x0` |
| `bne` at 0x3c318 | `[[sp-44]+4] != 0x0` | `[[sp-44]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c3c8 | `r0` | `0x0` |

## `powerNotPresent`

- `powerNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c3f8 to 0x3c51c.
  Established. Two signed bounds take the alternate route for entry `r1 < 0x0` and `r1 >=
  [[sp-28]+8]`, respectively, falling through for the opposite relation (`blt` at 0x3c43c, `bge` at
  0x3c450). When both fall through, a byte store clears `([[sp-28]+4] + r1) +0x0` (`strb` at
  0x3c4fc). The alternate route includes `0x21e28` with `0x6`, `0x1082ec`, `0x1ce`, `0x0` (`bl` at
  0x3c4e0). Physical absence semantics and the final r0 return are unknown (`ldmdb` at 0x3c500).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c408 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c40c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c410 | load | 4 | `abs 0x3c504` | `-` |
| `ldr` at 0x3c414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c428 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c430 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c444 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c448 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3c458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c460 | load | 4 | `abs 0x3c508` | `-` |
| `ldr` at 0x3c464 | load | 4 | `abs 0x3c50c` | `-` |
| `str` at 0x3c490 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3c494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c49c | load | 4 | `abs 0x3c508` | `-` |
| `ldr` at 0x3c4a0 | load | 4 | `abs 0x3c50c` | `-` |
| `ldr` at 0x3c4ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c4bc | load | 4 | `abs 0x3c510` | `-` |
| `str` at 0x3c4c0 | store | 4 | `(unknown)` | `0x108394` |
| `str` at 0x3c4c4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3c4c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c4cc | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x3c4d4 | load | 4 | `abs 0x3c514` | `-` |
| `ldr` at 0x3c4d8 | load | 4 | `abs 0x3c518` | `-` |
| `ldr` at 0x3c4e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c4ec | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c4f0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c4fc | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x0` |
| `ldmdb` at 0x3c500 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c500 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c500 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c468 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c474 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c4a4 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c4b4 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c4e0 | `(unknown callee) 0x21e28` | `0x6` | `0x1082ec` | `0x1ce` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c43c | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c450 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c500 | `r0` | `?` |

## `powerPresent`

- `powerPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c51c to 0x3c640.
  Established. The signed lower bound takes its alternate route for entry `r1 < 0x0` and falls
  through for nonnegative (`blt` at 0x3c560). The next bound takes the same route for `r1 >=
  [[sp-28]+8]` and falls through for less (`bge` at 0x3c574). With both falling through, byte `0x1`
  is stored at `([[sp-28]+4] + r1) +0x0` (`strb` at 0x3c620). The physical presence criterion and
  merged r0 value remain unproven (`ldmdb` at 0x3c624).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c52c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c530 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3c534 | load | 4 | `abs 0x3c628` | `-` |
| `ldr` at 0x3c538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c54c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c554 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c558 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c568 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c56c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x3c57c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c584 | load | 4 | `abs 0x3c62c` | `-` |
| `ldr` at 0x3c588 | load | 4 | `abs 0x3c630` | `-` |
| `str` at 0x3c5b4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3c5b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c5c0 | load | 4 | `abs 0x3c62c` | `-` |
| `ldr` at 0x3c5c4 | load | 4 | `abs 0x3c630` | `-` |
| `ldr` at 0x3c5d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3c5e0 | load | 4 | `abs 0x3c634` | `-` |
| `str` at 0x3c5e4 | store | 4 | `(unknown)` | `0x1083c0` |
| `str` at 0x3c5e8 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0x3c5ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3c5f0 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x3c5f8 | load | 4 | `abs 0x3c638` | `-` |
| `ldr` at 0x3c5fc | load | 4 | `abs 0x3c63c` | `-` |
| `ldr` at 0x3c60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c610 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x3c614 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c620 | store | 1 | `([[sp-28]+4] + r1) +0x0` | `0x1` |
| `ldmdb` at 0x3c624 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c624 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c58c | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c598 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c5c8 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x108344` | `0x10834c` |
| `bl` at 0x3c5d8 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0x3c604 | `(unknown callee) 0x21e28` | `0x6` | `0x1082ec` | `0x1de` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c560 | `r1 < 0x0 (signed, 32-bit)` | `r1 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x3c574 | `r1 >= [[sp-28]+8] (signed, 32-bit)` | `r1 < [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c624 | `r0` | `?` |

## `powerReset`

- `powerReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c640 to 0x3c80c.
  Established. Three byte stores address `[sp-24] +0xf`: the first has an unknown value, another
  writes `([sp-36]:1 & 0xff)`, and a third writes `0x5` (`strb` at 0x3c688, `strb` at 0x3c788,
  `strb` at 0x3c7ac). Calls include `0x40e88` with `[sp-20]`, `[sp-28]`, `[[sp-24]+4]`,
  `[[sp-28]+6]:1` and `0x4097c` with `[sp-20]`, `[sp-28]`, `[sp-29]:1`, `[sp-29]:1` (`bl` at
  0x3c704, `bl` at 0x3c7c0). The latter zero result takes the continuation and nonzero falls through
  to a second test (`beq` at 0x3c7d4). The return reports `[sp-40]`. Margin-state encoding and
  physical reset effects remain unknown (`ldmdb` at 0x3c804).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c650 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3c654 | load | 4 | `abs 0x3c808` | `-` |
| `ldr` at 0x3c658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c66c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c674 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x3c67c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3c680 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c688 | store | 1 | `[sp-24] +0xf` | `?` |
| `str` at 0x3c690 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x3c694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c698 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c69c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x3c6ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c6b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3c6c4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x3c6cc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x3c6d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c6d4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x3c6d8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3c6e0 | load | 1 | `([[sp-24]+4] + [sp-48]) +0x0` | `-` |
| `ldr` at 0x3c6ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x3c6f0 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldr` at 0x3c6fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c700 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c70c | store | 4 | `sp -0x24` | `ret(0x40e88)` |
| `ldr` at 0x3c710 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c714 | store | 4 | `sp -0x34` | `ret(0x40e88)` |
| `ldr` at 0x3c718 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c724 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c730 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c73c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c740 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3c74c | store | 1 | `sp -0x1d` | `(ret(0x40e18) & 0xff)` |
| `str` at 0x3c758 | store | 4 | `sp -0x24` | `0x2` |
| `ldr` at 0x3c75c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c760 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x3c76c | store | 1 | `sp -0x1d` | `(ret(0x40498) & 0xff)` |
| `ldr` at 0x3c770 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x3c774 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x3c780 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3c784 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x3c788 | store | 1 | `[sp-24] +0xf` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x3c790 | load | 4 | `sp -0x18` | `-` |
| `ldrsb` at 0x3c794 | load | 1 | `[sp-24] +0xf` | `-` |
| `ldr` at 0x3c798 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c7a4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x3c7ac | store | 1 | `[sp-24] +0xf` | `0x5` |
| `ldrb` at 0x3c7b0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x3c7b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c7b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c7c8 | store | 4 | `sp -0x2c` | `ret(0x4097c)` |
| `ldr` at 0x3c7cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c7d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c7e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3c7e8 | store | 4 | `sp -0x28` | `ret(0x4097c)` |
| `ldr` at 0x3c7ec | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3c7f4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x3c7fc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x3c804 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c804 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c704 | `(unknown callee) 0x40e88` | `[sp-20]` | `[sp-28]` | `[[sp-24]+4]` | `[[sp-28]+6]:1` |
| `bl` at 0x3c744 | `(unknown callee) 0x40e18` | `[sp-20]` | `[sp-28]` | `?` | `ret(0x40e88)` |
| `bl` at 0x3c764 | `(unknown callee) 0x40498` | `[sp-20]` | `[sp-28]` | `?` | `?` |
| `bl` at 0x3c7c0 | `(unknown callee) 0x4097c` | `[sp-20]` | `[sp-28]` | `[sp-29]:1` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3c6a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3c6e8 | `[([[sp-24]+4] + [sp-48])]:1 == 0x0` | `[([[sp-24]+4] + [sp-48])]:1 != 0x0` |
| `beq` at 0x3c6f8 | `[[sp-28]+6]:1 == 0xff` | `[[sp-28]+6]:1 != 0xff` |
| `beq` at 0x3c720 | `ret(0x40e88) == 0x0` | `ret(0x40e88) != 0x0` |
| `blt` at 0x3c72c | `ret(0x40e88) < 0x0 (signed, 32-bit)` | `ret(0x40e88) >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x3c738 | `ret(0x40e88) > 0x4 (signed, 32-bit)` | `ret(0x40e88) <= 0x4 (signed, 32-bit)` |
| `bne` at 0x3c77c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3c7a0 | `? == [sp-36]` | `? != [sp-36]` |
| `beq` at 0x3c7d4 | `ret(0x4097c) == 0x0` | `ret(0x4097c) != 0x0` |
| `bne` at 0x3c7e0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c804 | `r0` | `[sp-40]` |

## `powerUpSlot`

- `powerUpSlot`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c8fc to 0x3cc04.
  Established. Byte stores clear `[sp-44] +0xc`, set `[sp-44] +0xe` to `0x1`, and later set `[sp-44]
  +0xd` to `0x1` (`strb` at 0x3c98c, `strb` at 0x3c9ac, `strb` at 0x3cb6c). A call to `0x406f4` has
  `[sp-20]`, unknown, unknown, unknown (`bl` at 0x3ca40); its zero result takes the continuation and
  nonzero falls through to `0x3d284` with `[sp-20]`, `0x5`, unknown, `ret(0x406f4)` (`beq` at
  0x3ca54, `bl` at 0x3ca60). An indirect call has unknown target with `[sp-20]`, unknown, unknown,
  `[[sp-44]+16]` (`mov` at 0x3cb24). Power-up ordering, timing and the joined r0 value remain
  unknown (`ldmdb` at 0x3cbf0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c90c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3c914 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x3c91c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c920 | load | 4 | `abs 0x3cbf4` | `-` |
| `ldr` at 0x3c924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c938 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3c940 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3c944 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c94c | store | 4 | `sp -0x34` | `ret(0x4e650)` |
| `ldr` at 0x3c950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c95c | load | 4 | `abs 0x3cbf8` | `-` |
| `str` at 0x3c960 | store | 4 | `sp -0x38` | `0x1083ec` |
| `ldr` at 0x3c968 | load | 4 | `abs 0x3cbfc` | `-` |
| `str` at 0x3c96c | store | 4 | `sp -0x38` | `0x1083f0` |
| `ldr` at 0x3c970 | load | 4 | `abs 0x3cc00` | `-` |
| `ldr` at 0x3c974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c978 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3c97c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3c984 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3c98c | store | 1 | `[sp-44] +0xc` | `0x0` |
| `str` at 0x3c99c | store | 4 | `sp -0x20` | `ret(0x4e1ac)` |
| `ldr` at 0x3c9a4 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3c9ac | store | 1 | `[sp-44] +0xe` | `0x1` |
| `str` at 0x3c9b4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3c9b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c9bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3c9c0 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3c9d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c9d4 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3c9d8 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x3c9e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3c9ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3c9f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ca04 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrb` at 0x3ca0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ca18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ca1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ca30 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x3ca38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3ca48 | store | 4 | `sp -0x24` | `ret(0x406f4)` |
| `ldr` at 0x3ca4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ca58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ca68 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3ca70 | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x3ca74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ca78 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x3ca80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ca84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ca98 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x3caa0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3caac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cab0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3cac4 | load | 4 | `[sp-44] +0x0` | `-` |
| `ldrh` at 0x3cacc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x3cae4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3caec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3cafc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cb00 | load | 4 | `[sp-44] +0x10` | `-` |
| `str` at 0x3cb10 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3cb14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3cb18 | load | 4 | `[sp-44] +0x10` | `-` |
| `ldr` at 0x3cb1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cb34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cb44 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0x3cb48 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3cb4c | store | 4 | `sp -0x24` | `[sp-60]` |
| `ldr` at 0x3cb58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cb5c | store | 4 | `sp -0x20` | `[sp-60]` |
| `ldr` at 0x3cb64 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cb6c | store | 1 | `[sp-44] +0xd` | `0x1` |
| `ldr` at 0x3cb70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3cb88 | store | 4 | `sp -0x24` | `ret(0x4e10c)` |
| `ldr` at 0x3cb8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3cb98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3cba4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3cba8 | store | 4 | `sp -0x20` | `ret(0x4e10c)` |
| `ldr` at 0x3cbac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cbb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cbbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3cbc4 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x3cbcc | store | 1 | `[sp-44] +0xe` | `0x0` |
| `ldr` at 0x3cbd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3cbe4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3cbe8 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x3cbec | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x3cbf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3cbf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3cbf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c948 | `(unknown callee) 0x4e650` | `r2` | `0x147fcc` | `?` | `?` |
| `bl` at 0x3c980 | `(unknown callee) 0x2070c` | `0x1083f8` | `[sp-20]` | `[sp-56]` | `ret(0x4e650)` |
| `bl` at 0x3c994 | `(unknown callee) 0x4e1ac` | `0x5` | `?` | `[sp-44]` | `0x0` |
| `bl` at 0x3c9a0 | `(unknown callee) 0x823b0` | `ret(0x4e1ac)` | `?` | `?` | `ret(0x4e1ac)` |
| `bl` at 0x3ca40 | `(unknown callee) 0x406f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3ca60 | `(unknown callee) 0x3d284` | `[sp-20]` | `0x5` | `?` | `ret(0x406f4)` |
| `bl` at 0x3ca64 | `(unknown callee) 0x823c8` | `ret(0x3d284)` | `?` | `?` | `?` |
| `bl` at 0x3cad4 | `(unknown callee) 0x40ef8` | `?` | `[sp-44]` | `?` | `?` |
| `bl` at 0x3cae0 | `(unknown callee) 0x40ef8` | `0x19` | `[sp-44]` | `?` | `?` |
| `bl` at 0x3caf8 | `(unknown callee) 0x40ef8` | `0x64` | `?` | `?` | `[[sp-44]+8]` |
| `mov` at 0x3cb24 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-44]+16]` |
| `bl` at 0x3cb60 | `(unknown callee) 0x823c8` | `?` | `?` | `?` | `?` |
| `bl` at 0x3cb80 | `(unknown callee) 0x4e10c` | `0x5` | `?` | `[sp-44]` | `[sp-24]` |
| `bl` at 0x3cbb0 | `(unknown callee) 0x5cf84` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cbb8 | `(unknown callee) 0x5d108` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cbc0 | `(unknown callee) 0x5c7a8` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x3cbe0 | `(unknown callee) 0x824b0` | `0x0` | `?` | `[sp-44]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c958 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blt` at 0x3c9c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3c9e8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3ca14 | `? == 0xff` | `? != 0xff` |
| `beq` at 0x3ca54 | `ret(0x406f4) == 0x0` | `ret(0x406f4) != 0x0` |
| `beq` at 0x3caa8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x3cb08 | `[[sp-44]+16] == 0x0` | `[[sp-44]+16] != 0x0` |
| `beq` at 0x3cb30 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3cb3c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cb54 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0x3cb78 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x3cb94 | `ret(0x4e10c) == 0x0` | `ret(0x4e10c) != 0x0` |
| `bne` at 0x3cba0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x3cbd8 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3cbf0 | `r0` | `?` |

## `powerDownSlot`

- `powerDownSlot`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d284 to 0x3d48c.
  Established. The initial equality branch takes its bypass for entry `r1 == 0x5` and falls through
  otherwise (`beq` at 0x3d2d0). A later unknown-target call has `[sp-20]`, unknown, unknown,
  `[[sp-120]+20]` (`mov` at 0x3d330). Near the end of the loop, the call to `0x408a8` carries
  `[sp-20]`, `[sp-124]`, unknown, `0x0` (`bl` at 0x3d42c); its zero result takes the continuation
  and nonzero falls through to a second test (`beq` at 0x3d438). The returned value is
  `[sp-28]`, while physical slot power-down behavior is unproven (`ldmdb` at 0x3d480).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d294 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3d298 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3d2a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3d2a4 | load | 4 | `abs 0x3d484` | `-` |
| `ldr` at 0x3d2a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d2bc | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3d2c4 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0x3d2c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d2d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d2e0 | load | 4 | `abs 0x3d488` | `-` |
| `ldr` at 0x3d2e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d2fc | store | 4 | `sp -0x1c` | `ret(0x4e1ac)` |
| `ldr` at 0x3d300 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d308 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d30c | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x3d31c | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x3d320 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d324 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x3d328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d340 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d350 | store | 4 | `sp -0x80` | `0x1` |
| `ldr` at 0x3d354 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x3d358 | store | 4 | `sp -0x20` | `[sp-128]` |
| `ldr` at 0x3d364 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d368 | store | 4 | `sp -0x1c` | `[sp-128]` |
| `ldr` at 0x3d36c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d370 | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x3d378 | store | 4 | `sp -0x24` | `[[sp-120]+8]-1` |
| `ldr` at 0x3d37c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d38c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d390 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3d3a4 | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x3d3ac | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x3d3b0 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d3b4 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3d3c0 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d3c4 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d3c8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3d3d0 | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x3d3dc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d3e0 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d3e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x3d3ec | load | 1 | `([[sp-120]+4] + [sp-36]) +0x0` | `-` |
| `ldr` at 0x3d3f8 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d3fc | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3d400 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3d420 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x3d424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d428 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3d43c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d44c | store | 4 | `sp -0x84` | `0x1` |
| `ldr` at 0x3d450 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x3d454 | store | 4 | `sp -0x20` | `[sp-132]` |
| `ldr` at 0x3d460 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d464 | store | 4 | `sp -0x1c` | `[sp-132]` |
| `ldr` at 0x3d468 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3d470 | store | 4 | `sp -0x24` | `[sp-36]-1` |
| `ldr` at 0x3d478 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3d480 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d480 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d480 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d2d8 | `(unknown callee) 0x4e650` | `r1` | `0x147fcc` | `?` | `r1` |
| `bl` at 0x3d2ec | `(unknown callee) 0x2070c` | `0x1084fc` | `[sp-20]` | `ret(0x4e650)` | `ret(0x4e650)` |
| `bl` at 0x3d2f4 | `(unknown callee) 0x4e1ac` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x3d304 | `(unknown callee) 0x5c95c` | `[sp-20]` | `?` | `?` | `ret(0x4e1ac)` |
| `mov` at 0x3d330 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x3d408 | `(unknown callee) 0xd6928` | `[sp-20]` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x3d42c | `(unknown callee) 0x408a8` | `[sp-20]` | `[sp-124]` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d2d0 | `r1 == 0x5` | `r1 != 0x5` |
| `beq` at 0x3d314 | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x3d33c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3d348 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x3d360 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bge` at 0x3d384 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3d3bc | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x3d3d8 | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x3d3f4 | `[([[sp-120]+4] + [sp-36])]:1 != 0x0` | `[([[sp-120]+4] + [sp-36])]:1 == 0x0` |
| `bne` at 0x3d414 | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `beq` at 0x3d438 | `ret(0x408a8) == 0x0` | `ret(0x408a8) != 0x0` |
| `bne` at 0x3d444 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x3d45c | `[sp-132] == 0x0` | `[sp-132] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d480 | `r0` | `[sp-28]` |

## `powerDown`

- `powerDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d48c to 0x3d858.
  Established. Two computed transfers each take an unresolved edge for `[sp-20]-1 <= 0x4` unsigned
  and fall through above that bound (`ldrls` at 0x3d4ec, `ldrls` at 0x3d538). On known edges,
  `0x42178` receives `0x0`, `0xff`, `0x2`, unknown (`bl` at 0x3d618). Later calls to `0x51580` have
  `0x32`, `0x1085ac`, `0x0`, `[sp-24]` and `0x32`, `0x1085bc`, `0x2`, `[sp-24]`, respectively (`bl`
  at 0x3d7cc, `bl` at 0x3d7e8). The known-edge r0 return is `[sp-24]`. Excluded dispatch effects and
  physical shutdown semantics remain unknown (`ldmdb` at 0x3d834).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d49c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3d4a4 | store | 4 | `sp -0x18` | `0x0` |
| `strb` at 0x3d4b0 | store | 1 | `sp -0x71` | `(ret(0x4f090) & 0xff)` |
| `ldr` at 0x3d4b4 | load | 4 | `abs 0x3d838` | `-` |
| `ldr` at 0x3d4b8 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3d4bc | store | 4 | `sp -0x78` | `[0x147fcc]` |
| `ldrb` at 0x3d4d0 | load | 1 | `sp -0x71` | `-` |
| `ldr` at 0x3d4e0 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x3d4ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d52c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x3d538 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3d5a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d5b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d5bc | load | 4 | `abs 0x3d844` | `-` |
| `str` at 0x3d5d4 | store | 4 | `sp -0x1c` | `ret(0x51bac)` |
| `ldr` at 0x3d5dc | load | 4 | `abs 0x3d848` | `-` |
| `str` at 0x3d5ec | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `str` at 0x3d600 | store | 4 | `sp -0x18` | `ret(0x4e1ac)` |
| `str` at 0x3d620 | store | 4 | `sp -0x1c` | `ret(0x42178)` |
| `ldr` at 0x3d624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3d630 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d63c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d640 | store | 4 | `sp -0x18` | `ret(0x42178)` |
| `ldr` at 0x3d644 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d648 | load | 4 | `[sp-120] +0x14` | `-` |
| `str` at 0x3d658 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x3d65c | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d660 | load | 4 | `[sp-120] +0x14` | `-` |
| `ldr` at 0x3d67c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d68c | store | 4 | `sp -0x84` | `0x1` |
| `ldr` at 0x3d690 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x3d694 | store | 4 | `sp -0x1c` | `[sp-132]` |
| `ldr` at 0x3d6a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d6a4 | store | 4 | `sp -0x18` | `[sp-132]` |
| `ldr` at 0x3d6a8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d6ac | load | 4 | `[sp-120] +0x8` | `-` |
| `str` at 0x3d6b4 | store | 4 | `sp -0x20` | `[[sp-120]+8]-1` |
| `ldr` at 0x3d6b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d6c8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d6cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3d6e0 | load | 4 | `[sp-120] +0x0` | `-` |
| `str` at 0x3d6e8 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x3d6ec | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d6f0 | load | 1 | `[sp-124] +0x2` | `-` |
| `ldr` at 0x3d6fc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d700 | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d704 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3d70c | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x3d718 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x3d71c | load | 4 | `[sp-120] +0x4` | `-` |
| `ldr` at 0x3d720 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3d728 | load | 1 | `([[sp-120]+4] + [sp-32]) +0x0` | `-` |
| `ldr` at 0x3d734 | load | 4 | `sp -0x7c` | `-` |
| `ldrb` at 0x3d738 | load | 1 | `[sp-124] +0x2` | `-` |
| `str` at 0x3d75c | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x3d764 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x3d778 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d788 | store | 4 | `sp -0x88` | `0x1` |
| `ldr` at 0x3d78c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x3d790 | store | 4 | `sp -0x1c` | `[sp-136]` |
| `ldr` at 0x3d79c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3d7a0 | store | 4 | `sp -0x18` | `[sp-136]` |
| `ldr` at 0x3d7a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3d7ac | store | 4 | `sp -0x20` | `[sp-32]-1` |
| `ldr` at 0x3d7b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d7c4 | load | 4 | `abs 0x3d84c` | `-` |
| `str` at 0x3d7d4 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x3d7e0 | load | 4 | `abs 0x3d850` | `-` |
| `str` at 0x3d7f0 | store | 4 | `sp -0x1c` | `ret(0x51580)` |
| `ldr` at 0x3d800 | load | 4 | `abs 0x3d854` | `-` |
| `ldr` at 0x3d804 | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x3d818 | load | 4 | `abs 0x3d854` | `-` |
| `ldr` at 0x3d81c | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x3d828 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3d82c | store | 4 | `sp -0x80` | `[sp-24]` |
| `ldr` at 0x3d830 | load | 4 | `sp -0x80` | `-` |
| `ldmdb` at 0x3d834 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3d834 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3d834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3d4a8 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x3d4c0 | `(unknown callee) 0x859cc` | `ret(0x4f090)` | `?` | `?` | `[0x147fcc]` |
| `bl` at 0x3d5b4 | `(unknown callee) 0x4e650` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x3d5c4 | `(unknown callee) 0x2070c` | `0x108590` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `bl` at 0x3d5cc | `(unknown callee) 0x51bac` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d5e4 | `(unknown callee) 0x51580` | `0x32` | `0x1085a0` | `0x1` | `ret(0x51bac)` |
| `bl` at 0x3d5f0 | `(unknown callee) 0x82414` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x3d5f8 | `(unknown callee) 0x4e1ac` | `0x5` | `?` | `?` | `?` |
| `bl` at 0x3d608 | `(unknown callee) 0x5c95c` | `0x0` | `?` | `?` | `ret(0x4e1ac)` |
| `bl` at 0x3d618 | `(unknown callee) 0x42178` | `0x0` | `0xff` | `0x2` | `?` |
| `mov` at 0x3d66c | `(unknown callee) (unknown target)` | `0x0` | `?` | `?` | `[[sp-120]+20]` |
| `bl` at 0x3d744 | `(unknown callee) 0xd6928` | `0x0` | `[[sp-124]+2]:1` | `[[sp-120]+4]` | `[[sp-124]+2]:1` |
| `bl` at 0x3d768 | `(unknown callee) 0x408a8` | `0x0` | `[sp-124]` | `?` | `0x0` |
| `bl` at 0x3d7cc | `(unknown callee) 0x51580` | `0x32` | `0x1085ac` | `0x0` | `[sp-24]` |
| `bl` at 0x3d7e8 | `(unknown callee) 0x51580` | `0x32` | `0x1085bc` | `0x2` | `[sp-24]` |
| `bl` at 0x3d7f4 | `(unknown callee) 0x8242c` | `ret(0x51580)` | `?` | `?` | `ret(0x51580)` |
| `bl` at 0x3d7fc | `(unknown callee) 0x74f64` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3d814 | `(unknown callee) 0x40ef8` | `0x28` | `?` | `?` | `[0x2ae584]` |
| `mov` at 0x3d824 | `(unknown callee) (unknown target)` | `ret(0x40ef8)` | `?` | `?` | `[0x2ae584]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3d4cc | `ret(0x859cc) == 0x0` | `ret(0x859cc) != 0x0` |
| `beq` at 0x3d4d8 | `[sp-113]:1 == 0x4c` | `[sp-113]:1 != 0x4c` |
| `ldrls` at 0x3d4ec | `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` | `[sp-20]-1 > 0x4 (unsigned, 32-bit)` |
| `ldrls` at 0x3d538 | `[sp-20]-1 <= 0x4 (unsigned, 32-bit)` | `[sp-20]-1 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x3d5ac | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `beq` at 0x3d62c | `ret(0x42178) == 0x0` | `ret(0x42178) != 0x0` |
| `bne` at 0x3d638 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3d650 | `[[sp-120]+20] == 0x0` | `[[sp-120]+20] != 0x0` |
| `beq` at 0x3d678 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x3d684 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3d69c | `[sp-132] == 0x0` | `[sp-132] != 0x0` |
| `bge` at 0x3d6c0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3d6f8 | `[[sp-124]+2]:1 == 0xff` | `[[sp-124]+2]:1 != 0xff` |
| `bne` at 0x3d714 | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3d730 | `[([[sp-120]+4] + [sp-32])]:1 != 0x0` | `[([[sp-120]+4] + [sp-32])]:1 == 0x0` |
| `bne` at 0x3d750 | `ret(0xd6928) != 0x0` | `ret(0xd6928) == 0x0` |
| `beq` at 0x3d774 | `ret(0x408a8) == 0x0` | `ret(0x408a8) != 0x0` |
| `bne` at 0x3d780 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3d798 | `[sp-136] == 0x0` | `[sp-136] != 0x0` |
| `bne` at 0x3d7bc | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x3d80c | `[0x2ae584] == 0x0` | `[0x2ae584] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3d834 | `r0` | `[sp-24]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d4ec).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3d538).

## `powerStatusSlot`

- `powerStatusSlot`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3d970 to 0x3db38.
  Established. The call to `0x3db38` carries `[sp-20]`, `[[sp-48]+1]:1`, `[[sp-48]+1]:1`,
  `[[sp-48]+1]:1` (`bl` at 0x3da50); its signed positive result takes the continuation and
  less-or-equal falls through to the loop increment (`bgt` at 0x3da5c). The following call to
  `0xd6c2c` has `[sp-20]`, `[[sp-48]+2]:1`, unknown, `[[sp-48]+2]:1` (`bl` at 0x3da80). The r0
  return is `[sp-52]`, and slot-status polarity and aggregate meaning are unknown (`ldmdb` at
  0x3db30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3d980 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3d984 | load | 4 | `abs 0x3db34` | `-` |
| `ldr` at 0x3d988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3d99c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3d9a4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x3d9ac | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x3d9b4 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x3d9bc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3d9c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d9c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d9c8 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3d9d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3d9dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3d9f0 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x3d9f8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3d9fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3da00 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3da04 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3da0c | load | 1 | `([[sp-44]+4] + [sp-24]) +0x0` | `-` |
| `ldr` at 0x3da18 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3da1c | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x3da28 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3da2c | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x3da30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3da34 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x3da40 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3da44 | load | 1 | `[sp-48] +0x1` | `-` |
| `ldr` at 0x3da48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3da64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3da6c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x3da70 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3da74 | load | 1 | `[sp-48] +0x2` | `-` |
| `ldr` at 0x3da78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3da88 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x3da8c | load | 1 | `[sp-48] +0x3` | `-` |
| `str` at 0x3daa4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x3daa8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3dab4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3dac0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3dac8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3dacc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3dad4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3dadc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dae8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3daec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dafc | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x3db04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3db14 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x3db20 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x3db24 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3db28 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x3db2c | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3db30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3db30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3db30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3da50 | `(unknown callee) 0x3db38` | `[sp-20]` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` | `[[sp-48]+1]:1` |
| `bl` at 0x3da80 | `(unknown callee) 0xd6c2c` | `[sp-20]` | `[[sp-48]+2]:1` | `?` | `[[sp-48]+2]:1` |
| `bl` at 0x3daac | `(unknown callee) 0x406a4` | `[sp-48]` | `?` | `ret(0xd6c2c)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x3d9d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3da14 | `[([[sp-44]+4] + [sp-24])]:1 == 0x0` | `[([[sp-44]+4] + [sp-24])]:1 != 0x0` |
| `beq` at 0x3da24 | `[[sp-48]+2]:1 == 0xff` | `[[sp-48]+2]:1 != 0xff` |
| `bge` at 0x3da3c | `[[sp-48]+1]:1 >= [[sp-44]+8] (signed, 32-bit)` | `[[sp-48]+1]:1 < [[sp-44]+8] (signed, 32-bit)` |
| `bgt` at 0x3da5c | `ret(0x3db38) > 0x0 (signed, 32-bit)` | `ret(0x3db38) <= 0x0 (signed, 32-bit)` |
| `bne` at 0x3dabc | `[sp-36] != ret(0x406a4)` | `[sp-36] == ret(0x406a4)` |
| `beq` at 0x3dae4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x3daf4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x3db0c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3db30 | `r0` | `[sp-52]` |

## `powerState`

- `powerState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3dc58 to 0x3deb4.
  Established. The call to `0x3fcf8` has `[sp-32]`, `ret(0x404e8)`, `0x0`, `sp-52` (`bl` at
  0x3ddac). A later signed test takes its alternate arm for `ret(0xfb1b8) > 0x0` and falls through
  for less-or-equal (`bgt` at 0x3de1c). Another signed test takes that arm for `ret(0xfb1c0) < 0x0`
  and falls through for nonnegative (`blt` at 0x3de78). The two final word stores write `0x1` and
  `0x2` at `sp -0x3c`, respectively (`str` at 0x3de84, `str` at 0x3de90). Physical state thresholds
  and the joined return remain unknown (`ldmdb` at 0x3de98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dc68 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x3dc6c | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x3dc70 | load | 4 | `abs 0x3deac` | `-` |
| `ldr` at 0x3dc74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dc88 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x3dc90 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3dc94 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dc98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3dcac | load | 4 | `abs 0x3deb0` | `-` |
| `str` at 0x3dcb0 | store | 4 | `sp -0x3c` | `0xfffff9b9` |
| `ldr` at 0x3dcb8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3dcbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3dcd0 | load | 4 | `[sp-56] +0x0` | `-` |
| `str` at 0x3dcd8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3dcdc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3dce0 | load | 1 | `[sp-44] +0x2` | `-` |
| `ldr` at 0x3dcec | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3dcf0 | load | 1 | `[sp-44] +0x2` | `-` |
| `ldr` at 0x3dcf4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dd04 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3dd08 | load | 1 | `[sp-44] +0x3` | `-` |
| `str` at 0x3dd14 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x3dd18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3dd2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3dd3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3dd50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3dd64 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3dd6c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3dd70 | load | 1 | `[sp-44] +0x4` | `-` |
| `str` at 0x3dd80 | store | 4 | `sp -0x3c` | `0x2` |
| `ldr` at 0x3dd88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3dd8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3dd9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ddb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ddb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3ddc0 | store | 4 | `sp -0x30` | `ret(0x40350)` |
| `ldr` at 0x3ddc4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3ddd4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x3ddf0 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3ddf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3de20 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3de30 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x3de4c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x3de4c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3de84 | store | 4 | `sp -0x3c` | `0x1` |
| `str` at 0x3de90 | store | 4 | `sp -0x3c` | `0x2` |
| `ldr` at 0x3de94 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3de98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dc9c | `(unknown callee) 0x3c86c` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x3dcfc | `(unknown callee) 0xd6c2c` | `[sp-32]` | `[[sp-44]+2]:1` | `?` | `[[sp-44]+2]:1` |
| `bl` at 0x3dd1c | `(unknown callee) 0x406a4` | `[sp-44]` | `?` | `ret(0xd6c2c)` | `?` |
| `bl` at 0x3dd40 | `(unknown callee) 0x406a4` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x3dd90 | `(unknown callee) 0x404e8` | `[sp-32]` | `[sp-44]` | `?` | `[[sp-44]+4]:1` |
| `bl` at 0x3ddac | `(unknown callee) 0x3fcf8` | `[sp-32]` | `ret(0x404e8)` | `0x0` | `sp-52` |
| `bl` at 0x3ddb8 | `(unknown callee) 0x40350` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x3ddc8 | `(unknown callee) 0xfaca8` | `[sp-52]` | `?` | `?` | `ret(0x40350)` |
| `bl` at 0x3ddd8 | `(unknown callee) 0xfaca8` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3ddf4 | `(unknown callee) 0xfacf4` | `ret(0xfaca8)` | `?` | `?` | `?` |
| `bl` at 0x3de10 | `(unknown callee) 0xfb1b8` | `ret(0xfaca8)` | `?` | `ret(0xfacf4)` | `?` |
| `bl` at 0x3de24 | `(unknown callee) 0xfaca8` | `[sp-52]` | `?` | `?` | `ret(0xfb1b8)` |
| `bl` at 0x3de34 | `(unknown callee) 0xfaca8` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x3de50 | `(unknown callee) 0xfacf4` | `ret(0xfaca8)` | `?` | `?` | `?` |
| `bl` at 0x3de6c | `(unknown callee) 0xfb1c0` | `ret(0xfaca8)` | `?` | `ret(0xfacf4)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3dca8 | `ret(0x3c86c) != 0x0` | `ret(0x3c86c) == 0x0` |
| `beq` at 0x3dce8 | `[[sp-44]+2]:1 == 0xff` | `[[sp-44]+2]:1 != 0xff` |
| `beq` at 0x3dd28 | `ret(0x406a4) == 0x0` | `ret(0x406a4) != 0x0` |
| `bne` at 0x3dd34 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x3dd4c | `ret(0x406a4) != 0x0` | `ret(0x406a4) == 0x0` |
| `bne` at 0x3dd58 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x3dd78 | `[[sp-44]+4]:1 != 0xff` | `[[sp-44]+4]:1 == 0xff` |
| `bgt` at 0x3de1c | `ret(0xfb1b8) > 0x0 (signed, 32-bit)` | `ret(0xfb1b8) <= 0x0 (signed, 32-bit)` |
| `blt` at 0x3de78 | `ret(0xfb1c0) < 0x0 (signed, 32-bit)` | `ret(0xfb1c0) >= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3de98 | `r0` | `?` |

## `powerSetMargin`

- `powerSetMargin`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3deb4 to 0x3dfc0.
  Established. The call to `0x4097c` carries `[sp-20]`, `ret(0x405a0)`, `[sp-25]:1`, `[sp-25]:1`
  (`bl` at 0x3df3c); its zero result takes the arm that stores word `0x4` at absolute `0x142790`.
  Nonzero falls through to store word `0x5` there (`beq` at 0x3df50, `str` at 0x3df7c, `str` at
  0x3df5c). On the zero arm, the later call to `0x40ec0` has `[sp-20]`, `[sp-32]`, `0x4`, unknown
  (`bl` at 0x3df94). The margin setting and physical voltage change are unproven, and the merged r0
  value remains unknown (`ldmdb` at 0x3dfb8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3dec4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3dec8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x3ded0 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x3ded4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ded8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3def0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3def8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3defc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3df08 | store | 4 | `sp -0x20` | `ret(0x405a0)` |
| `ldr` at 0x3df0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3df18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3df24 | store | 4 | `sp -0x28` | `ret(0x4063c)` |
| `ldrb` at 0x3df2c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3df30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3df34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3df44 | store | 4 | `sp -0x24` | `ret(0x4097c)` |
| `ldr` at 0x3df48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3df54 | load | 4 | `abs 0x3dfbc` | `-` |
| `str` at 0x3df5c | store | 4 | `abs 0x142790` | `0x5` |
| `ldr` at 0x3df68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3df6c | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x3df74 | load | 4 | `abs 0x3dfbc` | `-` |
| `str` at 0x3df7c | store | 4 | `abs 0x142790` | `0x4` |
| `ldr` at 0x3df88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3df8c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3df98 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3df9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3dfa0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3dfb0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x3dfb4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x3dfb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3dfb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3dfb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3dedc | `(unknown callee) 0x3c86c` | `r0` | `r1` | `r2` | `r2` |
| `bl` at 0x3df00 | `(unknown callee) 0x405a0` | `[sp-20]` | `[sp-24]` | `?` | `ret(0x3c86c)` |
| `bl` at 0x3df1c | `(unknown callee) 0x4063c` | `[sp-20]` | `?` | `?` | `ret(0x405a0)` |
| `bl` at 0x3df3c | `(unknown callee) 0x4097c` | `[sp-20]` | `ret(0x405a0)` | `[sp-25]:1` | `[sp-25]:1` |
| `bl` at 0x3df64 | `(unknown callee) 0x74fbc` | `0x5` | `?` | `0x5` | `0x142790` |
| `bl` at 0x3df84 | `(unknown callee) 0x74fbc` | `0x4` | `?` | `0x4` | `0x142790` |
| `bl` at 0x3df94 | `(unknown callee) 0x40ec0` | `[sp-20]` | `[sp-32]` | `0x4` | `?` |
| `bl` at 0x3dfa8 | `(unknown callee) 0x40e50` | `[sp-20]` | `[sp-32]` | `[sp-25]:1` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3dee8 | `ret(0x3c86c) != 0x0` | `ret(0x3c86c) == 0x0` |
| `bne` at 0x3df14 | `ret(0x405a0) != 0x0` | `ret(0x405a0) == 0x0` |
| `beq` at 0x3df50 | `ret(0x4097c) == 0x0` | `ret(0x4097c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3dfb8 | `r0` | `?` |

## `powerGetReading`

- `powerGetReading`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3fcf8 to 0x3fff0.
  Established. The call to `0xccf4c` uses `[sp-28]`, `[ret(0x405a0)+4]:1`, `[ret(0x405a0)+5]:1`,
  `sp-46` (`bl` at 0x3fd70); its zero result takes the continuation and nonzero falls through to
  save that result as a word at `sp -0x38` (`beq` at 0x3fd84, `str` at 0x3fd8c). A halfword store
  later writes `([sp-46]:2 & 0xffff)` at `[sp-36] +0x0` (`strh` at 0x3fda8). The computed transfer
  takes an unresolved edge for `? <= 0x5` unsigned and falls through above it (`ldrls` at 0x3fe5c).
  Excluded conversions, reading units and the r0 return remain unknown (`ldmdb` at 0x3ffc0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3fd08 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x3fd0c | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x3fd10 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x3fd14 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x3fd18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fd1c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3fd28 | store | 4 | `sp -0x2c` | `ret(0x405a0)` |
| `ldr` at 0x3fd2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3fd38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3fd44 | store | 4 | `sp -0x38` | `ret(0x4063c)` |
| `ldr` at 0x3fd4c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fd50 | load | 1 | `ret(0x405a0) +0x4` | `-` |
| `ldr` at 0x3fd54 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fd58 | load | 1 | `ret(0x405a0) +0x5` | `-` |
| `ldr` at 0x3fd60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3fd78 | store | 4 | `sp -0x34` | `ret(0xccf4c)` |
| `ldr` at 0x3fd7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3fd88 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x3fd8c | store | 4 | `sp -0x38` | `ret(0xccf4c)` |
| `ldr` at 0x3fd94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3fda0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x3fda4 | load | 2 | `sp -0x2e` | `-` |
| `strh` at 0x3fda8 | store | 2 | `[sp-36] +0x0` | `([sp-46]:2 & 0xffff)` |
| `ldr` at 0x3fdac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3fdb8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3fdbc | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldrsh` at 0x3fdc0 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0x3fdcc | store | 4 | `sp -0x40` | `ret(0xfb614)` |
| `ldrsh` at 0x3fdd0 | load | 2 | `sp -0x2e` | `-` |
| `ldr` at 0x3fddc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3fde0 | load | 4 | `abs 0x3ffdc` | `-` |
| `str` at 0x3fdec | store | 4 | `sp -0x40` | `ret(0xfb44c)` |
| `ldr` at 0x3fdf0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fdf4 | load | 1 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x3fdf8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fdfc | load | 1 | `[sp-44] +0x5` | `-` |
| `ldr` at 0x3fe00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3fe14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x3fe24 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x3fe28 | store | 4 | `[sp-60] +0x0` | `ret(0xfb660)` |
| `ldr` at 0x3fe2c | load | 4 | `abs 0x3ffe0` | `-` |
| `ldr` at 0x3fe30 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x3fe34 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldrb` at 0x3fe54 | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x3fe5c | load | 4 | `(unknown)` | `-` |
| `str` at 0x3ffb8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x3ffbc | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x3ffc0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x3ffc0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3ffc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ffc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ffc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3fd20 | `(unknown callee) 0x405a0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x3fd3c | `(unknown callee) 0x4063c` | `[sp-28]` | `?` | `?` | `ret(0x405a0)` |
| `bl` at 0x3fd70 | `(unknown callee) 0xccf4c` | `[sp-28]` | `[ret(0x405a0)+4]:1` | `[ret(0x405a0)+5]:1` | `sp-46` |
| `bl` at 0x3fdc8 | `(unknown callee) 0xfb614` | `?` | `?` | `[sp-40]` | `?` |
| `bl` at 0x3fde4 | `(unknown callee) 0xfb44c` | `ret(0xfb614)` | `0x47800000` | `?` | `?` |
| `bl` at 0x3fe0c | `(unknown callee) 0xcd93c` | `[sp-28]` | `[[sp-44]+4]:1` | `[[sp-44]+5]:1` | `[[sp-44]+5]:1` |
| `bl` at 0x3fe1c | `(unknown callee) 0xfb660` | `[sp-64]` | `ret(0xcd93c)` | `?` | `ret(0xcd93c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3fd34 | `ret(0x405a0) != 0x0` | `ret(0x405a0) == 0x0` |
| `beq` at 0x3fd84 | `ret(0xccf4c) == 0x0` | `ret(0xccf4c) != 0x0` |
| `beq` at 0x3fd9c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x3fdb4 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bge` at 0x3fdd8 | `? >= 0x0 (signed, 32-bit)` | `? < 0x0 (signed, 32-bit)` |
| `ldrls` at 0x3fe5c | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ffc0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x3fe5c).

## `powerLowVoltage`

- `powerLowVoltage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x401f0 to 0x402a0.
  Established. The first call reaches `0x40350` with entry r0 through r3 (`bl` at 0x40210). A later
  call to `0x403dc` has `[sp-32]`, `[sp-36]`, unknown, unknown (`bl` at 0x40230). The final call to
  `0xfb350` carries `ret(0xfacf4)`, unknown, unknown, `ret(0xfacf4)` and supplies the r0 return
  (`bl` at 0x40288, `ldmdb` at 0x40294). The facts report no conditional branches, and a physical
  lower-voltage limit is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40200 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x40204 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x40208 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4020c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x40228 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4022c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4024c | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x4024c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40294 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40210 | `(unknown callee) 0x40350` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4021c | `(unknown callee) 0xfaca8` | `ret(0x40350)` | `?` | `?` | `ret(0x40350)` |
| `bl` at 0x40230 | `(unknown callee) 0x403dc` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x4023c | `(unknown callee) 0xfaca8` | `ret(0x403dc)` | `?` | `?` | `ret(0x403dc)` |
| `bl` at 0x40258 | `(unknown callee) 0xfa96c` | `?` | `?` | `ret(0xfaca8)` | `?` |
| `bl` at 0x40274 | `(unknown callee) 0xfacf4` | `ret(0xfaca8)` | `?` | `ret(0xfa96c)` | `?` |
| `bl` at 0x40288 | `(unknown callee) 0xfb350` | `ret(0xfacf4)` | `?` | `?` | `ret(0xfacf4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40294 | `r0` | `ret(0xfb350)` |

## `powerHighVoltage`

- `powerHighVoltage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x402a0 to 0x40350.
  Established. Entry r0 and r1 are saved as words at `sp -0x20` and `sp -0x24` (`str` at 0x402b0,
  `str` at 0x402b4). The call to `0xfa970` has `ret(0xfaca8)`, unknown, unknown, unknown (`bl` at
  0x40308). The last call to `0xfb350` receives `ret(0xfacf4)`, unknown, unknown, `ret(0xfacf4)` and
  supplies r0 at return (`bl` at 0x40338, `ldmdb` at 0x40344). No conditional branch is reported,
  and a physical upper-voltage limit remains unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x402b0 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x402b4 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x402b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x402bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x402d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x402dc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x40304 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x40304 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x402c0 | `(unknown callee) 0x40350` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x402cc | `(unknown callee) 0xfaca8` | `ret(0x40350)` | `?` | `?` | `ret(0x40350)` |
| `bl` at 0x402e0 | `(unknown callee) 0x403dc` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x402ec | `(unknown callee) 0xfaca8` | `ret(0x403dc)` | `?` | `?` | `ret(0x403dc)` |
| `bl` at 0x40308 | `(unknown callee) 0xfa970` | `ret(0xfaca8)` | `?` | `?` | `?` |
| `bl` at 0x40324 | `(unknown callee) 0xfacf4` | `ret(0xfaca8)` | `?` | `ret(0xfa970)` | `?` |
| `bl` at 0x40338 | `(unknown callee) 0xfb350` | `ret(0xfacf4)` | `?` | `?` | `ret(0xfacf4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40344 | `r0` | `ret(0xfb350)` |

## `powerNominalVoltage`

- `powerNominalVoltage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40350 to 0x403dc.
  Established. Two word stores save entry r0 and r1 at `sp -0x14` and `sp -0x18` (`str` at 0x40360,
  `str` at 0x40364). An indexed word load later supplies an unknown r0 value at the return (`ldr` at
  0x403c8, `ldmdb` at 0x403d0). No calls or conditional branches are reported. The referenced table
  and nominal-voltage units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40360 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40364 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40368 | load | 4 | `abs 0x403d4` | `-` |
| `ldr` at 0x4036c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40380 | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x40388 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4038c | load | 4 | `abs 0x403d8` | `-` |
| `ldr` at 0x40390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x403a8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x403b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x403c8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x403d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x403d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x403d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x403d0 | `r0` | `?` |

## `powerMarginPercent`

- `powerMarginPercent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x403dc to 0x40470.
  Established. The selector takes its equality arm for `? == 0xe` and falls through on inequality
  (`beq` at 0x40440). Those arms store words `0x3cdd67ba` and `0x3d4ccccd` at `sp -0x20`,
  respectively (`str` at 0x4044c, `str` at 0x40458). The r0 return is `[sp-32]` (`ldmdb` at
  0x40460). Calls are absent, and the margin percentage and numeric representation are not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x403ec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x403f0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x403f4 | load | 4 | `abs 0x40464` | `-` |
| `ldr` at 0x403f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4040c | load | 4 | `abs 0x147fcc` | `-` |
| `str` at 0x40414 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x40418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4041c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40430 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x40438 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x40448 | load | 4 | `abs 0x40468` | `-` |
| `str` at 0x4044c | store | 4 | `sp -0x20` | `0x3cdd67ba` |
| `ldr` at 0x40454 | load | 4 | `abs 0x4046c` | `-` |
| `str` at 0x40458 | store | 4 | `sp -0x20` | `0x3d4ccccd` |
| `ldr` at 0x4045c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x40460 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40460 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40460 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40440 | `? == 0xe` | `? != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40460 | `r0` | `[sp-32]` |
