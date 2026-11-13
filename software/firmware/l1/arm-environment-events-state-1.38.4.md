# ARM environmental events and state in 1.38.4

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
Computed transfers in EnvQuery, EnvTestEvent and monitorReset remain unresolved. Tables and return values cover only known edges. The unresolved transfer site is listed below each affected entry. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/environment-events-state-facts.txt) retain all tool sections.

## `EnvEvent`

- `EnvEvent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a24c to 0x7a91c.
  Established. Two word stores at absolute `0x148df4` have unknown values (`str` at 0x7a44c, `str`
  at 0x7a524). The opening call to `0x22a38` carries `0x149024`, unknown, entry r2, entry r3 (`bl`
  at 0x7a274); its zero result takes the bypass and nonzero falls through (`beq` at 0x7a288). The
  final r0 expression is `ret(0x75270)` (`ldmdb` at 0x7a8dc). Environmental event fields and
  hardware consequences remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a25c | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x7a260 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x7a264 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x7a268 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x7a26c | load | 4 | `abs 0x7a8e0` | `-` |
| `str` at 0x7a27c | store | 4 | `sp -0xf0` | `ret(0x22a38)` |
| `ldr` at 0x7a280 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x7a28c | load | 4 | `abs 0x7a8e4` | `-` |
| `str` at 0x7a290 | store | 4 | `sp -0x100` | `0x1106d4` |
| `ldr` at 0x7a294 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x7a298 | store | 4 | `sp -0xfc` | `ret(0x22a38)` |
| `ldr` at 0x7a2a0 | load | 4 | `abs 0x7a8e8` | `-` |
| `ldr` at 0x7a2b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x7a2bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a2cc | load | 4 | `abs 0x7a8ec` | `-` |
| `ldr` at 0x7a304 | load | 4 | `abs 0x7a8f0` | `-` |
| `ldr` at 0x7a310 | load | 4 | `abs 0x1495a4` | `-` |
| `ldr` at 0x7a324 | load | 4 | `abs 0x7a8f4` | `-` |
| `ldr` at 0x7a334 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a344 | load | 4 | `abs 0x7a8ec` | `-` |
| `ldr` at 0x7a34c | load | 4 | `abs 0x7a8f8` | `-` |
| `ldr` at 0x7a350 | load | 4 | `abs 0x148df8` | `-` |
| `ldr` at 0x7a35c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7a37c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7a394 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7a3c0 | store | 4 | `sp -0x100` | `0x1` |
| `str` at 0x7a3dc | store | 4 | `sp -0x100` | `0x0` |
| `ldr` at 0x7a3e0 | load | 4 | `abs 0x7a8fc` | `-` |
| `ldr` at 0x7a3e8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x7a3f8 | store | 4 | `sp -0xf0` | `ret(0xb6d54)` |
| `ldr` at 0x7a3fc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a418 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a428 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a438 | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a43c | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a440 | load | 4 | `abs 0x148df4` | `-` |
| `ldr` at 0x7a444 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x7a44c | store | 4 | `abs 0x148df4` | `?` |
| `ldr` at 0x7a450 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a46c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a478 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a488 | load | 4 | `abs 0x7a904` | `-` |
| `ldr` at 0x7a48c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a498 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a4a8 | load | 4 | `abs 0x7a908` | `-` |
| `ldr` at 0x7a4ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a4b8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a4c8 | load | 4 | `abs 0x7a90c` | `-` |
| `ldr` at 0x7a4cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a4d8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a4e8 | load | 4 | `abs 0x7a910` | `-` |
| `ldr` at 0x7a4ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a4f8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a508 | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a50c | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a510 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7a51c | load | 4 | `abs 0x148df4` | `-` |
| `str` at 0x7a524 | store | 4 | `abs 0x148df4` | `?` |
| `ldr` at 0x7a52c | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a530 | load | 4 | `abs 0x148df4` | `-` |
| `ldr` at 0x7a544 | load | 4 | `abs 0x7a904` | `-` |
| `ldr` at 0x7a554 | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a558 | load | 4 | `abs 0x148df4` | `-` |
| `ldr` at 0x7a56c | load | 4 | `abs 0x7a908` | `-` |
| `ldr` at 0x7a57c | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a580 | load | 4 | `abs 0x148df4` | `-` |
| `ldr` at 0x7a594 | load | 4 | `abs 0x7a90c` | `-` |
| `ldr` at 0x7a5a4 | load | 4 | `abs 0x7a900` | `-` |
| `ldr` at 0x7a5a8 | load | 4 | `abs 0x148df4` | `-` |
| `ldr` at 0x7a5bc | load | 4 | `abs 0x7a910` | `-` |
| `ldr` at 0x7a5d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7a5e4 | load | 4 | `abs 0x7a8e0` | `-` |
| `str` at 0x7a5f0 | store | 4 | `sp -0xf0` | `ret(0x22dc0)` |
| `ldr` at 0x7a5f4 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0x7a600 | load | 4 | `abs 0x7a914` | `-` |
| `str` at 0x7a604 | store | 4 | `sp -0x100` | `0x110714` |
| `ldr` at 0x7a608 | load | 4 | `sp -0xf0` | `-` |
| `str` at 0x7a60c | store | 4 | `sp -0xfc` | `ret(0x22dc0)` |
| `ldr` at 0x7a614 | load | 4 | `abs 0x7a8e8` | `-` |
| `ldr` at 0x7a618 | load | 4 | `abs 0x7a918` | `-` |
| `ldr` at 0x7a630 | load | 4 | `ret(0x74908) +0x5c` | `-` |
| `str` at 0x7a634 | store | 4 | `sp -0xf4` | `[ret(0x74908)+92]` |
| `ldr` at 0x7a638 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7a640 | store | 4 | `sp -0xf8` | `?` |
| `ldr` at 0x7a644 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a650 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a65c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a668 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a674 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a680 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a68c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a698 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6a8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6b4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6c4 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6d0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6dc | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6e8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a6f8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a704 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a714 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a720 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a72c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a738 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a744 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a750 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a760 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a76c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a77c | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a788 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a794 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7a0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7b0 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7bc | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7c8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7d8 | load | 4 | `sp -0xf8` | `-` |
| `ldr` at 0x7a7e8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a7f0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a7f8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a800 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a808 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a810 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a818 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a820 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a828 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a830 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a838 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a840 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a848 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a850 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a858 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a860 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a868 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a870 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a878 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a880 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a888 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a890 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a898 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a8a0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a8a8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a8b0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a8b8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a8c0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a8c8 | load | 4 | `sp -0xf4` | `-` |
| `str` at 0x7a8d0 | store | 4 | `sp -0xf4` | `?` |
| `ldr` at 0x7a8d4 | load | 4 | `sp -0xf4` | `-` |
| `ldmdb` at 0x7a8dc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7a8dc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7a8dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7a8dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7a8dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a274 | `(unknown callee) 0x22a38` | `0x149024` | `?` | `r2` | `r3` |
| `bl` at 0x7a2ac | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0xb2` | `0x0` |
| `bl` at 0x7a2c0 | `(unknown callee) 0x20240` | `[sp-40]` | `?` | `?` | `[sp]` |
| `bl` at 0x7a2d0 | `(unknown callee) 0x1b508` | `sp-236` | `0x110708` | `?` | `sp-236` |
| `bl` at 0x7a2e0 | `(unknown callee) 0x1b22c` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x7a2f0 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `ret(0x1b22c)` |
| `bl` at 0x7a2fc | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x7a318 | `(unknown callee) 0x69d3c` | `(sp-236 + ret(0x1b22c))` | `ret(0x1f1dc)` | `[0x1495a4]` | `ret(0x1f1dc)` |
| `bl` at 0x7a328 | `(unknown callee) 0x1b2d4` | `sp-236` | `0x11070c` | `?` | `sp-236` |
| `bl` at 0x7a338 | `(unknown callee) 0x1b2d4` | `sp-236` | `[sp-40]` | `?` | `sp-236` |
| `bl` at 0x7a348 | `(unknown callee) 0x1b2d4` | `sp-236` | `0x110708` | `?` | `sp-236` |
| `bl` at 0x7a380 | `(unknown callee) 0x6907c` | `sp-84` | `sp-80` | `[sp-32]` | `sp-84` |
| `bl` at 0x7a398 | `(unknown callee) 0x690d8` | `sp-84` | `sp-80` | `[sp-36]` | `sp-84` |
| `bl` at 0x7a3a4 | `(unknown callee) 0x1b22c` | `sp-236` | `?` | `?` | `sp-236` |
| `bl` at 0x7a3d0 | `(unknown callee) 0x692f8` | `sp-84` | `sp-80` | `sp-236` | `ret(0x1b22c)` |
| `bl` at 0x7a3f0 | `(unknown callee) 0xb6d54` | `0x148e24` | `0x0` | `[sp-84]` | `sp-80` |
| `bl` at 0x7a410 | `(unknown callee) 0x51aa0` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x7a420 | `(unknown callee) 0x51580` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x7a464 | `(unknown callee) 0x51aa0` | `0x1` | `0x148df4` | `[0x148df4]` | `?` |
| `bl` at 0x7a474 | `(unknown callee) 0x51580` | `0x32` | `[sp-40]` | `0x2` | `?` |
| `bl` at 0x7a490 | `(unknown callee) 0x1b508` | `0x1495a8` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7a4b0 | `(unknown callee) 0x1b508` | `0x1495f8` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7a4d0 | `(unknown callee) 0x1b508` | `0x149648` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7a4f0 | `(unknown callee) 0x1b508` | `0x149698` | `[sp-40]` | `?` | `?` |
| `bl` at 0x7a528 | `(unknown callee) 0x51a6c` | `0x148df4` | `0x148df4` | `?` | `?` |
| `bl` at 0x7a54c | `(unknown callee) 0x51580` | `0x32` | `0x1495a8` | `0x2` | `?` |
| `bl` at 0x7a574 | `(unknown callee) 0x51580` | `0x32` | `0x1495f8` | `0x2` | `?` |
| `bl` at 0x7a59c | `(unknown callee) 0x51580` | `0x32` | `0x149648` | `0x2` | `?` |
| `bl` at 0x7a5c4 | `(unknown callee) 0x51580` | `0x32` | `0x149698` | `0x2` | `?` |
| `bl` at 0x7a5d0 | `(unknown callee) 0x51bac` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x7a5e0 | `(unknown callee) 0x51580` | `0x32` | `[sp-40]` | `0x1` | `?` |
| `bl` at 0x7a5e8 | `(unknown callee) 0x22dc0` | `0x149024` | `?` | `?` | `?` |
| `bl` at 0x7a620 | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0x11a` | `0x0` |
| `bl` at 0x7a628 | `(unknown callee) 0x74908` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7a8d8 | `(unknown callee) 0x75270` | `[sp-244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7a288 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `beq` at 0x7a2b8 | `[sp] == 0x0` | `[sp] != 0x0` |
| `beq` at 0x7a358 | `[0x148df8] == 0x0` | `[0x148df8] != 0x0` |
| `bgt` at 0x7a364 | `[sp-28] > 0x1 (signed, 32-bit)` | `[sp-28] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x7a408 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a434 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a45c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a484 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a4a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a4c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a4e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a504 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a53c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a564 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a58c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a5b4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7a5fc | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `beq` at 0x7a64c | `[sp-248] == 0x3300` | `[sp-248] != 0x3300` |
| `bhi` at 0x7a658 | `[sp-248] > 0x3300 (unsigned, 32-bit)` | `[sp-248] <= 0x3300 (unsigned, 32-bit)` |
| `beq` at 0x7a664 | `[sp-248] == 0x2000` | `[sp-248] != 0x2000` |
| `bhi` at 0x7a670 | `[sp-248] > 0x2000 (unsigned, 32-bit)` | `[sp-248] <= 0x2000 (unsigned, 32-bit)` |
| `beq` at 0x7a67c | `[sp-248] == 0x1200` | `[sp-248] != 0x1200` |
| `bhi` at 0x7a688 | `[sp-248] > 0x1200 (unsigned, 32-bit)` | `[sp-248] <= 0x1200 (unsigned, 32-bit)` |
| `beq` at 0x7a694 | `[sp-248] == 0x1000` | `[sp-248] != 0x1000` |
| `beq` at 0x7a6a0 | `[sp-248] == 0x1100` | `[sp-248] != 0x1100` |
| `beq` at 0x7a6b0 | `[sp-248] == 0x1300` | `[sp-248] != 0x1300` |
| `beq` at 0x7a6bc | `[sp-248] == 0x1400` | `[sp-248] != 0x1400` |
| `beq` at 0x7a6cc | `[sp-248] == 0x3000` | `[sp-248] != 0x3000` |
| `bhi` at 0x7a6d8 | `[sp-248] > 0x3000 (unsigned, 32-bit)` | `[sp-248] <= 0x3000 (unsigned, 32-bit)` |
| `beq` at 0x7a6e4 | `[sp-248] == 0x2100` | `[sp-248] != 0x2100` |
| `beq` at 0x7a6f0 | `[sp-248] == 0x2200` | `[sp-248] != 0x2200` |
| `beq` at 0x7a700 | `[sp-248] == 0x3100` | `[sp-248] != 0x3100` |
| `beq` at 0x7a70c | `[sp-248] == 0x3200` | `[sp-248] != 0x3200` |
| `beq` at 0x7a71c | `[sp-248] == 0x6000` | `[sp-248] != 0x6000` |
| `bhi` at 0x7a728 | `[sp-248] > 0x6000 (unsigned, 32-bit)` | `[sp-248] <= 0x6000 (unsigned, 32-bit)` |
| `beq` at 0x7a734 | `[sp-248] == 0x4000` | `[sp-248] != 0x4000` |
| `bhi` at 0x7a740 | `[sp-248] > 0x4000 (unsigned, 32-bit)` | `[sp-248] <= 0x4000 (unsigned, 32-bit)` |
| `beq` at 0x7a74c | `[sp-248] == 0x3400` | `[sp-248] != 0x3400` |
| `beq` at 0x7a758 | `[sp-248] == 0x3500` | `[sp-248] != 0x3500` |
| `beq` at 0x7a768 | `[sp-248] == 0x4100` | `[sp-248] != 0x4100` |
| `beq` at 0x7a774 | `[sp-248] == 0x5000` | `[sp-248] != 0x5000` |
| `beq` at 0x7a784 | `[sp-248] == 0x7200` | `[sp-248] != 0x7200` |
| `bhi` at 0x7a790 | `[sp-248] > 0x7200 (unsigned, 32-bit)` | `[sp-248] <= 0x7200 (unsigned, 32-bit)` |
| `beq` at 0x7a79c | `[sp-248] == 0x7000` | `[sp-248] != 0x7000` |
| `beq` at 0x7a7a8 | `[sp-248] == 0x7100` | `[sp-248] != 0x7100` |
| `beq` at 0x7a7b8 | `[sp-248] == 0x7400` | `[sp-248] != 0x7400` |
| `bhi` at 0x7a7c4 | `[sp-248] > 0x7400 (unsigned, 32-bit)` | `[sp-248] <= 0x7400 (unsigned, 32-bit)` |
| `beq` at 0x7a7d0 | `[sp-248] == 0x7300` | `[sp-248] != 0x7300` |
| `beq` at 0x7a7e0 | `[sp-248] == 0x7500` | `[sp-248] != 0x7500` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7a8dc | `r0` | `ret(0x75270)` |

## `EnvInit`

- `EnvInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7a91c to 0x7ae5c.
  Established. A word store advances absolute `0x148dec` to `[0x148dec]+1`, followed by a word store
  clearing `0x148e04` (`str` at 0x7a9d0, `str` at 0x7a9dc). The unsigned comparison takes its branch
  for `ret(0x825cc) <= 0x7ffff` and falls through for greater (`bls` at 0x7a944). The call to
  `0xb553c` uses `0x148e24`, `0x4`, `0x0`, `0x0` (`bl` at 0x7a9ec). The common exit carries
  `ret(0x82140)` (`ldmdb` at 0x7ad74). Task ownership and numeric units are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7a930 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x7a93c | load | 4 | `abs 0x7ad78` | `-` |
| `str` at 0x7a94c | store | 4 | `sp -0x2c` | `0x960` |
| `ldr` at 0x7a954 | load | 4 | `abs 0x7ad7c` | `-` |
| `str` at 0x7a958 | store | 4 | `sp -0x2c` | `0x5dc` |
| `ldr` at 0x7a95c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7a960 | store | 4 | `sp -0x24` | `[sp-44]` |
| `ldr` at 0x7a96c | load | 4 | `abs 0x7ad78` | `-` |
| `str` at 0x7a97c | store | 4 | `sp -0x30` | `0x960` |
| `ldr` at 0x7a984 | load | 4 | `abs 0x7ad7c` | `-` |
| `str` at 0x7a988 | store | 4 | `sp -0x30` | `0x5dc` |
| `ldr` at 0x7a98c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7a990 | store | 4 | `sp -0x28` | `[sp-48]` |
| `ldr` at 0x7a994 | load | 4 | `abs 0x7ad80` | `-` |
| `ldr` at 0x7a998 | load | 4 | `abs 0x148dec` | `-` |
| `ldr` at 0x7a9a4 | load | 4 | `abs 0x7ad84` | `-` |
| `str` at 0x7a9a8 | store | 4 | `sp -0x44` | `0x110738` |
| `ldr` at 0x7a9b0 | load | 4 | `abs 0x7ad88` | `-` |
| `ldr` at 0x7a9b4 | load | 4 | `abs 0x7ad8c` | `-` |
| `ldr` at 0x7a9c0 | load | 4 | `abs 0x7ad80` | `-` |
| `ldr` at 0x7a9c4 | load | 4 | `abs 0x7ad80` | `-` |
| `ldr` at 0x7a9c8 | load | 4 | `abs 0x148dec` | `-` |
| `str` at 0x7a9d0 | store | 4 | `abs 0x148dec` | `[0x148dec]+1` |
| `ldr` at 0x7a9d4 | load | 4 | `abs 0x7ad90` | `-` |
| `str` at 0x7a9dc | store | 4 | `abs 0x148e04` | `0x0` |
| `ldr` at 0x7a9e0 | load | 4 | `abs 0x7ad94` | `-` |
| `ldr` at 0x7a9f0 | load | 4 | `abs 0x7ad98` | `-` |
| `ldr` at 0x7a9f4 | load | 4 | `abs 0x7ad9c` | `-` |
| `ldr` at 0x7aa04 | load | 4 | `abs 0x7ada0` | `-` |
| `ldr` at 0x7aa08 | load | 4 | `abs 0x7ada4` | `-` |
| `ldr` at 0x7aa18 | load | 4 | `abs 0x7ada8` | `-` |
| `ldr` at 0x7aa1c | load | 4 | `abs 0x7adac` | `-` |
| `ldr` at 0x7aa2c | load | 4 | `abs 0x7adb0` | `-` |
| `ldr` at 0x7aa30 | load | 4 | `abs 0x7adb4` | `-` |
| `ldr` at 0x7aa40 | load | 4 | `abs 0x7adb8` | `-` |
| `ldr` at 0x7aa44 | load | 4 | `abs 0x7adbc` | `-` |
| `ldr` at 0x7aa54 | load | 4 | `abs 0x7adc0` | `-` |
| `ldr` at 0x7aa58 | load | 4 | `abs 0x7adc4` | `-` |
| `ldr` at 0x7aa68 | load | 4 | `abs 0x7adc8` | `-` |
| `ldr` at 0x7aa6c | load | 4 | `abs 0x7adcc` | `-` |
| `ldr` at 0x7aa7c | load | 4 | `abs 0x7add0` | `-` |
| `ldr` at 0x7aa80 | load | 4 | `abs 0x7add4` | `-` |
| `str` at 0x7aa94 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7aa98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7aa9c | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7aaa4 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7aaac | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7aab4 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7aab8 | load | 4 | `abs 0x7add8` | `-` |
| `ldr` at 0x7aabc | load | 4 | `abs 0x7addc` | `-` |
| `ldr` at 0x7aac0 | load | 4 | `abs 0x7ade0` | `-` |
| `ldr` at 0x7aac4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7aad0 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7aad4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7aad8 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7aae0 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7aae8 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7aaf0 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7aaf4 | load | 4 | `abs 0x7ade4` | `-` |
| `ldr` at 0x7aaf8 | load | 4 | `abs 0x7ade8` | `-` |
| `ldr` at 0x7aafc | load | 4 | `abs 0x7adec` | `-` |
| `ldr` at 0x7ab00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7ab0c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7ab10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7ab14 | store | 4 | `sp -0x40` | `[sp-36]` |
| `str` at 0x7ab1c | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7ab24 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7ab2c | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7ab30 | load | 4 | `abs 0x7adf0` | `-` |
| `ldr` at 0x7ab34 | load | 4 | `abs 0x7adf4` | `-` |
| `ldr` at 0x7ab38 | load | 4 | `abs 0x7adf8` | `-` |
| `ldr` at 0x7ab3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7ab48 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7ab4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7ab50 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7ab58 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7ab60 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7ab68 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7ab6c | load | 4 | `abs 0x7adfc` | `-` |
| `ldr` at 0x7ab70 | load | 4 | `abs 0x7ae00` | `-` |
| `ldr` at 0x7ab74 | load | 4 | `abs 0x7ae04` | `-` |
| `ldr` at 0x7ab78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7ab84 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7ab88 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7ab8c | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7ab94 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7ab9c | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7aba4 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7aba8 | load | 4 | `abs 0x7ae08` | `-` |
| `ldr` at 0x7abac | load | 4 | `abs 0x7ae0c` | `-` |
| `ldr` at 0x7abb0 | load | 4 | `abs 0x7ae10` | `-` |
| `ldr` at 0x7abb4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7abc0 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x7abc4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7abc8 | store | 4 | `sp -0x40` | `[sp-40]` |
| `str` at 0x7abd0 | store | 4 | `sp -0x3c` | `0xa` |
| `str` at 0x7abd8 | store | 4 | `sp -0x38` | `0xa` |
| `str` at 0x7abe0 | store | 4 | `sp -0x34` | `0x9` |
| `ldr` at 0x7abe4 | load | 4 | `abs 0x7ae14` | `-` |
| `ldr` at 0x7abe8 | load | 4 | `abs 0x7ae18` | `-` |
| `ldr` at 0x7abec | load | 4 | `abs 0x7ae1c` | `-` |
| `ldr` at 0x7abf0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7abf8 | load | 4 | `abs 0x7ae20` | `-` |
| `ldr` at 0x7abfc | load | 4 | `abs 0x2ae5fc` | `-` |
| `ldr` at 0x7ac08 | load | 4 | `abs 0x7ae20` | `-` |
| `ldr` at 0x7ac0c | load | 4 | `abs 0x2ae5fc` | `-` |
| `ldr` at 0x7ac10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7ac14 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac20 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac30 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac38 | load | 4 | `abs 0x7ae28` | `-` |
| `str` at 0x7ac40 | store | 4 | `abs 0x1490f4` | `ret(0x3c80c)` |
| `ldr` at 0x7ac44 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac48 | load | 4 | `abs 0x1490f4` | `-` |
| `ldr` at 0x7ac54 | load | 4 | `abs 0x7ae2c` | `-` |
| `str` at 0x7ac58 | store | 4 | `sp -0x44` | `0x1107fc` |
| `ldr` at 0x7ac60 | load | 4 | `abs 0x7ad88` | `-` |
| `ldr` at 0x7ac64 | load | 4 | `abs 0x7ae30` | `-` |
| `ldr` at 0x7ac74 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac78 | load | 4 | `abs 0x1490f4` | `-` |
| `ldr` at 0x7ac80 | load | 4 | `abs 0x7ae34` | `-` |
| `ldr` at 0x7ac90 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ac98 | load | 4 | `abs 0x7ae38` | `-` |
| `str` at 0x7aca0 | store | 4 | `abs 0x1490f8` | `ret(0x45d70)` |
| `ldr` at 0x7aca4 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7aca8 | load | 4 | `abs 0x1490f8` | `-` |
| `ldr` at 0x7acb4 | load | 4 | `abs 0x7ae3c` | `-` |
| `str` at 0x7acb8 | store | 4 | `sp -0x44` | `0x110820` |
| `ldr` at 0x7acc0 | load | 4 | `abs 0x7ad88` | `-` |
| `ldr` at 0x7acc4 | load | 4 | `abs 0x7ae40` | `-` |
| `ldr` at 0x7acd4 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7acd8 | load | 4 | `abs 0x1490f8` | `-` |
| `ldr` at 0x7acec | load | 4 | `abs 0x7ae44` | `-` |
| `ldr` at 0x7acfc | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ad04 | load | 4 | `abs 0x7ae48` | `-` |
| `str` at 0x7ad0c | store | 4 | `abs 0x1490fc` | `ret(0x52d9c)` |
| `ldr` at 0x7ad10 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ad14 | load | 4 | `abs 0x1490fc` | `-` |
| `ldr` at 0x7ad20 | load | 4 | `abs 0x7ae4c` | `-` |
| `str` at 0x7ad24 | store | 4 | `sp -0x44` | `0x11083c` |
| `ldr` at 0x7ad2c | load | 4 | `abs 0x7ad88` | `-` |
| `ldr` at 0x7ad30 | load | 4 | `abs 0x7ae50` | `-` |
| `ldr` at 0x7ad40 | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ad44 | load | 4 | `abs 0x1490fc` | `-` |
| `ldr` at 0x7ad4c | load | 4 | `abs 0x7ae54` | `-` |
| `ldr` at 0x7ad5c | load | 4 | `abs 0x7ae24` | `-` |
| `ldr` at 0x7ad64 | load | 4 | `abs 0x7ae58` | `-` |
| `str` at 0x7ad6c | store | 4 | `abs 0x14910c` | `ret(0xcc7c8)` |
| `ldmdb` at 0x7ad74 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x7ad74 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7ad74 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7ad74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7a934 | `(unknown callee) 0x825cc` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0x7a964 | `(unknown callee) 0x825cc` | `ret(0x825cc)` | `?` | `ret(0x825cc)` | `[sp-44]` |
| `bl` at 0x7a9bc | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0x165` | `0x0` |
| `bl` at 0x7a9ec | `(unknown callee) 0xb553c` | `0x148e24` | `0x4` | `0x0` | `0x0` |
| `bl` at 0x7aa00 | `(unknown callee) 0x22920` | `0x148e2c` | `0x110754` | `0x0` | `0x9` |
| `bl` at 0x7aa14 | `(unknown callee) 0x22920` | `0x148e80` | `0x110760` | `0x0` | `0x9` |
| `bl` at 0x7aa28 | `(unknown callee) 0x22920` | `0x148ed4` | `0x11076c` | `0x0` | `0x9` |
| `bl` at 0x7aa3c | `(unknown callee) 0x22920` | `0x148f28` | `0x110778` | `0x0` | `0x9` |
| `bl` at 0x7aa50 | `(unknown callee) 0x22920` | `0x148f7c` | `0x110784` | `0x0` | `0x9` |
| `bl` at 0x7aa64 | `(unknown callee) 0x22920` | `0x148fd0` | `0x110790` | `0x0` | `0x9` |
| `bl` at 0x7aa78 | `(unknown callee) 0x22920` | `0x149024` | `0x11079c` | `0x1` | `0x9` |
| `bl` at 0x7aa8c | `(unknown callee) 0x22920` | `0x149078` | `0x1107a8` | `0x1` | `0x9` |
| `bl` at 0x7aac8 | `(unknown callee) 0x223f0` | `0x1490cc` | `0x1107b4` | `0x7e030` | `[sp-32]` |
| `bl` at 0x7ab04 | `(unknown callee) 0x223f0` | `0x1490d0` | `0x1107c0` | `0x7e9d8` | `[sp-32]` |
| `bl` at 0x7ab40 | `(unknown callee) 0x223f0` | `0x1490d4` | `0x1107cc` | `0x7ef0c` | `[sp-32]` |
| `bl` at 0x7ab7c | `(unknown callee) 0x223f0` | `0x1490d8` | `0x1107d8` | `0x7f7d0` | `[sp-32]` |
| `bl` at 0x7abb8 | `(unknown callee) 0x223f0` | `0x1490dc` | `0x1107e4` | `0x7fadc` | `[sp-32]` |
| `bl` at 0x7abf4 | `(unknown callee) 0x223f0` | `0x1490e0` | `0x1107f0` | `0x7fe20` | `[sp-32]` |
| `mov` at 0x7ac1c | `(unknown callee) (unknown target)` | `[sp-32]` | `0x1490e4` | `?` | `[0x2ae5fc]` |
| `bl` at 0x7ac2c | `(unknown callee) 0x1b43c` | `0x1490e4` | `0x0` | `0x4c0` | `?` |
| `bl` at 0x7ac3c | `(unknown callee) 0x3c80c` | `0x0` | `0x149100` | `?` | `?` |
| `bl` at 0x7ac6c | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0x1a3` | `0x0` |
| `bl` at 0x7ac8c | `(unknown callee) 0x1b43c` | `0x149114` | `0x0` | `?` | `?` |
| `bl` at 0x7ac9c | `(unknown callee) 0x45d70` | `0x0` | `0x149104` | `?` | `?` |
| `bl` at 0x7accc | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0x1ae` | `0x0` |
| `bl` at 0x7acf8 | `(unknown callee) 0x1b43c` | `0x149280` | `0x0` | `?` | `?` |
| `bl` at 0x7ad08 | `(unknown callee) 0x52d9c` | `0x0` | `0x149108` | `?` | `?` |
| `bl` at 0x7ad38 | `(unknown callee) 0x21e28` | `0x9` | `0x1106f4` | `0x1b9` | `0x0` |
| `bl` at 0x7ad58 | `(unknown callee) 0x1b43c` | `0x149554` | `0x0` | `?` | `?` |
| `bl` at 0x7ad68 | `(unknown callee) 0xcc7c8` | `0x0` | `0x149110` | `?` | `?` |
| `bl` at 0x7ad70 | `(unknown callee) 0x82140` | `ret(0xcc7c8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x7a944 | `ret(0x825cc) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x825cc) > 0x7ffff (unsigned, 32-bit)` |
| `bls` at 0x7a974 | `ret(0x825cc) <= 0x7ffff (unsigned, 32-bit)` | `ret(0x825cc) > 0x7ffff (unsigned, 32-bit)` |
| `beq` at 0x7a9a0 | `[0x148dec] == 0x0` | `[0x148dec] != 0x0` |
| `beq` at 0x7ac04 | `[0x2ae5fc] == 0x0` | `[0x2ae5fc] != 0x0` |
| `ble` at 0x7ac50 | `[0x1490f4] <= 0x2a (signed, 32-bit)` | `[0x1490f4] > 0x2a (signed, 32-bit)` |
| `ble` at 0x7acb0 | `[0x1490f8] <= 0x14 (signed, 32-bit)` | `[0x1490f8] > 0x14 (signed, 32-bit)` |
| `ble` at 0x7ad1c | `[0x1490fc] <= 0x14 (signed, 32-bit)` | `[0x1490fc] > 0x14 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7ad74 | `r0` | `ret(0x82140)` |

## `env_pwrdwn`

- `env_pwrdwn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7bb1c to 0x7bd40.
  Established. Word stores write `0x0` and later `0x1` at absolute `0x142888` (`str` at 0x7bcf0,
  `str` at 0x7bd04). The opening call to `0x22a38` has `0x149078`, `0x0`, `[0x110874]`, `0x1e` (`bl`
  at 0x7bb5c); its zero result takes the continuation and nonzero falls through (`beq` at 0x7bb68).
  The call to `0x7a24c` includes candidates `0x2`, `0x4101`, unknown, `sp-124` (`bl` at 0x7bbdc).
  Power-down timing, physical sequencing and the joined return remain unknown (`ldmdb` at 0x7bd10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7bb2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7bb30 | load | 4 | `abs 0x7bd14` | `-` |
| `ldm` at 0x7bb3c | load | 4 | `abs 0x11086c` | `-` |
| `ldm` at 0x7bb3c | load | 4 | `abs 0x110870` | `-` |
| `ldm` at 0x7bb3c | load | 4 | `abs 0x110874` | `-` |
| `ldm` at 0x7bb3c | load | 4 | `abs 0x110878` | `-` |
| `stm` at 0x7bb40 | store | 4 | `sp -0x2c` | `[0x11086c]` |
| `stm` at 0x7bb40 | store | 4 | `sp -0x28` | `[0x110870]` |
| `stm` at 0x7bb40 | store | 4 | `sp -0x24` | `[0x110874]` |
| `stm` at 0x7bb40 | store | 4 | `sp -0x20` | `[0x110878]` |
| `ldm` at 0x7bb44 | load | 4 | `abs 0x11087c` | `-` |
| `ldm` at 0x7bb44 | load | 4 | `abs 0x110880` | `-` |
| `stm` at 0x7bb48 | store | 4 | `sp -0x1c` | `[0x11087c]` |
| `stm` at 0x7bb48 | store | 4 | `sp -0x18` | `[0x110880]` |
| `str` at 0x7bb50 | store | 4 | `sp -0x80` | `0x1e` |
| `ldr` at 0x7bb54 | load | 4 | `abs 0x7bd18` | `-` |
| `ldr` at 0x7bb7c | load | 4 | `abs 0x7bd1c` | `-` |
| `ldr` at 0x7bb80 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7bb88 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7bb8c | load | 4 | `abs 0x7bd20` | `-` |
| `str` at 0x7bbc0 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7bbc8 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7bbd0 | load | 4 | `abs 0x7bd24` | `-` |
| `ldr` at 0x7bbd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7bbe0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x7bbe8 | store | 4 | `sp -0x80` | `[sp-128]-5` |
| `ldr` at 0x7bc0c | load | 4 | `abs 0x7bd28` | `-` |
| `str` at 0x7bc1c | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7bc24 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7bc2c | load | 4 | `abs 0x7bd2c` | `-` |
| `ldr` at 0x7bc30 | load | 4 | `abs 0x7bd30` | `-` |
| `ldr` at 0x7bc3c | load | 4 | `abs 0x7bd18` | `-` |
| `ldr` at 0x7bc48 | load | 4 | `abs 0x7bd18` | `-` |
| `ldr` at 0x7bc68 | load | 4 | `abs 0x7bd34` | `-` |
| `str` at 0x7bc78 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7bc80 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7bc88 | load | 4 | `abs 0x7bd2c` | `-` |
| `ldr` at 0x7bc8c | load | 4 | `abs 0x7bd30` | `-` |
| `ldr` at 0x7bc98 | load | 4 | `abs 0x7bd18` | `-` |
| `ldr` at 0x7bca4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x7bcb8 | load | 4 | `abs 0x7bd38` | `-` |
| `str` at 0x7bcc8 | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0x7bcd0 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7bcd8 | load | 4 | `abs 0x7bd2c` | `-` |
| `ldr` at 0x7bce8 | load | 4 | `abs 0x7bd3c` | `-` |
| `str` at 0x7bcf0 | store | 4 | `abs 0x142888` | `0x0` |
| `ldr` at 0x7bcfc | load | 4 | `abs 0x7bd3c` | `-` |
| `str` at 0x7bd04 | store | 4 | `abs 0x142888` | `0x1` |
| `ldr` at 0x7bd08 | load | 4 | `abs 0x7bd18` | `-` |
| `ldmdb` at 0x7bd10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7bd10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7bd10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7bb5c | `(unknown callee) 0x22a38` | `0x149078` | `0x0` | `[0x110874]` | `0x1e` |
| `bl` at 0x7bb84 | `(unknown callee) 0x1b49c` | `sp-124` | `0x110884` | `[sp-128]` | `sp-124` |
| `bl` at 0x7bbdc | `(unknown callee) 0x7a24c` | `0x2` | `0x4101` | `?` | `sp-124` |
| `bl` at 0x7bbf0 | `(unknown callee) 0x825e4` | `0x1f4` | `?` | `?` | `[sp-128]-5` |
| `bl` at 0x7bbf4 | `(unknown callee) 0x3d8e0` | `ret(0x825e4)` | `?` | `?` | `?` |
| `bl` at 0x7bc10 | `(unknown callee) 0x1b49c` | `sp-124` | `0x1108a8` | `?` | `sp-124` |
| `bl` at 0x7bc38 | `(unknown callee) 0x7a24c` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x7bc40 | `(unknown callee) 0x22dc0` | `0x149078` | `?` | `?` | `?` |
| `bl` at 0x7bc50 | `(unknown callee) 0x22a38` | `0x149078` | `0x0` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x7bc6c | `(unknown callee) 0x1b49c` | `sp-124` | `0x1108d0` | `?` | `sp-124` |
| `bl` at 0x7bc94 | `(unknown callee) 0x7a24c` | `0x2` | `0x4102` | `0x163` | `sp-124` |
| `bl` at 0x7bc9c | `(unknown callee) 0x22dc0` | `0x149078` | `?` | `?` | `?` |
| `bl` at 0x7bcbc | `(unknown callee) 0x1b49c` | `sp-124` | `0x110900` | `?` | `sp-124` |
| `bl` at 0x7bce4 | `(unknown callee) 0x7a24c` | `0x2` | `0x4102` | `0x164` | `sp-124` |
| `bl` at 0x7bcf8 | `(unknown callee) 0x3d48c` | `0x4` | `?` | `0x142888` | `0x0` |
| `bl` at 0x7bd0c | `(unknown callee) 0x22dc0` | `0x149078` | `?` | `0x142888` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7bb68 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x7bc00 | `ret(0x3d8e0) != 0x0` | `ret(0x3d8e0) == 0x0` |
| `bne` at 0x7bc5c | `ret(0x22a38) != 0x0` | `ret(0x22a38) == 0x0` |
| `bgt` at 0x7bcac | `[sp-128] > 0x0 (signed, 32-bit)` | `[sp-128] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7bd10 | `r0` | `?` |

## `EnvState`

- `EnvState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x807cc to 0x808b4.
  Established. Separate word stores write `0x1` at `0x1490e8` and `0x0` at `0x1490ec` (`str` at
  0x80848, `str` at 0x80854). Other arms write `0x0` at `0x1490e8` and `0x1` at `0x1490ec` (`str` at
  0x80868, `str` at 0x80874). The call at `0x80858` reaches `0x82140` with unknown, unknown,
  `0x1490e4`, `0x0` (`bl` at 0x80858). All recognized exits converge on r0 `0x0` (`ldmdb` at
  0x808a4). The entry selector and physical state meanings remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x807dc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x807e0 | load | 4 | `abs 0x808a8` | `-` |
| `ldr` at 0x807e4 | load | 4 | `abs 0x148df0` | `-` |
| `ldr` at 0x807f4 | load | 4 | `abs 0x808ac` | `-` |
| `ldr` at 0x807f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x80800 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x80804 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x80808 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x80840 | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80848 | store | 4 | `abs 0x1490e8` | `0x1` |
| `ldr` at 0x8084c | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80854 | store | 4 | `abs 0x1490ec` | `0x0` |
| `ldr` at 0x80860 | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80868 | store | 4 | `abs 0x1490e8` | `0x0` |
| `ldr` at 0x8086c | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80874 | store | 4 | `abs 0x1490ec` | `0x1` |
| `ldr` at 0x80880 | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80888 | store | 4 | `abs 0x1490e8` | `0x0` |
| `ldr` at 0x80890 | load | 4 | `abs 0x808b0` | `-` |
| `str` at 0x80898 | store | 4 | `abs 0x1490ec` | `0x0` |
| `ldmdb` at 0x808a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x808a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x808a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x807fc | `(unknown callee) 0x23f2c` | `0x110fac` | `r0` | `r2` | `?` |
| `bl` at 0x80858 | `(unknown callee) 0x82140` | `?` | `?` | `0x1490e4` | `0x0` |
| `bl` at 0x80878 | `(unknown callee) 0x82140` | `?` | `?` | `0x1490e4` | `0x1` |
| `bl` at 0x8088c | `(unknown callee) 0x82140` | `?` | `?` | `0x1490e4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x807f0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x80810 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x8081c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x80828 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x80838 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x808a4 | `r0` | `0x0` |

## `EnvQuery`

- `EnvQuery`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x81c98 to 0x8204c.
  Established. A word store writes `ret(0x1b124)` at `[sp-24] +0x0` (`str` at 0x81d2c). A byte store
  later clears `[[sp-24]] +0x0` (`strb` at 0x81d6c). The opening call to `0x1f1dc` uses `0x6` and
  entry r1, r2, r3 (`bl` at 0x81cb4). The first test of that result takes its branch for
  `[[sp-24]] != 0x0` and falls through on zero (`bne` at 0x81d38). Query format and r0 at the
  known-edge exit remain unidentified (`ldmdb` at 0x82004).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x81ca8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x81cac | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x81cc8 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x81ce4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x81ce8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x81cec | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81cf0 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81cf4 | load | 4 | `abs 0x1490f4` | `-` |
| `ldr` at 0x81cf8 | load | 4 | `abs 0x1490f8` | `-` |
| `ldr` at 0x81d00 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81d04 | load | 4 | `abs 0x1490fc` | `-` |
| `str` at 0x81d2c | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0x81d30 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x81d3c | load | 4 | `abs 0x8200c` | `-` |
| `ldr` at 0x81d4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x81d50 | store | 4 | `[sp-24] +0x0` | `ret(0x1afa4)` |
| `ldr` at 0x81d58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x81d5c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x81d60 | store | 4 | `sp -0x28` | `[[sp-24]]` |
| `ldr` at 0x81d64 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x81d6c | store | 1 | `[[sp-24]] +0x0` | `0x0` |
| `ldr` at 0x81d70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x81d7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81d80 | load | 4 | `abs 0x82010` | `-` |
| `ldr` at 0x81d88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81d8c | load | 4 | `abs 0x82014` | `-` |
| `ldr` at 0x81d94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81d98 | load | 4 | `abs 0x82018` | `-` |
| `ldr` at 0x81da0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81dac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81db0 | load | 4 | `abs 0x8201c` | `-` |
| `ldr` at 0x81db8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81dbc | load | 4 | `abs 0x82020` | `-` |
| `ldr` at 0x81dc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81dc8 | load | 4 | `abs 0x82024` | `-` |
| `ldr` at 0x81dd0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81dd4 | load | 4 | `abs 0x82028` | `-` |
| `ldr` at 0x81de0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81de4 | load | 4 | `abs 0x8202c` | `-` |
| `ldr` at 0x81dec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81dfc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81e04 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81e08 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81e0c | load | 4 | `abs 0x1490e4` | `-` |
| `str` at 0x81e10 | store | 4 | `sp -0x2c` | `[0x1490e4]` |
| `ldr` at 0x81e14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81e20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x81e30 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81e34 | load | 4 | `abs 0x1490f0` | `-` |
| `ldrls` at 0x81e3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x81e94 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81e98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81e9c | load | 4 | `abs 0x82040` | `-` |
| `ldr` at 0x81ea0 | load | 4 | `abs 0x1490f0` | `-` |
| `ldr` at 0x81eac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81eb0 | load | 4 | `abs 0x82044` | `-` |
| `ldr` at 0x81ebc | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81ec0 | load | 4 | `abs 0x82008` | `-` |
| `ldr` at 0x81ec4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81ec8 | load | 4 | `abs 0x82048` | `-` |
| `ldr` at 0x81ecc | load | 4 | `abs 0x1490e4` | `-` |
| `ldr` at 0x81ed0 | load | 4 | `abs 0x1490f0` | `-` |
| `ldr` at 0x81ed8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81ee8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81ef0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81ef4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x81f04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f08 | load | 4 | `abs 0x82030` | `-` |
| `ldr` at 0x81f10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81f28 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81f2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81f4c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81f50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f68 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81f70 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81f74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81f8c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x81f94 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b22c))` |
| `ldr` at 0x81f98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x81fa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81fa8 | load | 4 | `abs 0x82010` | `-` |
| `ldr` at 0x81fb0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81fb4 | load | 4 | `abs 0x82014` | `-` |
| `ldr` at 0x81fbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81fc0 | load | 4 | `abs 0x82018` | `-` |
| `ldr` at 0x81fc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x81fd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81fd8 | load | 4 | `abs 0x8201c` | `-` |
| `ldr` at 0x81fe0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81fe4 | load | 4 | `abs 0x82020` | `-` |
| `ldr` at 0x81fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81ff0 | load | 4 | `abs 0x82024` | `-` |
| `ldr` at 0x81ff8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x81ffc | load | 4 | `abs 0x82028` | `-` |
| `ldmdb` at 0x82004 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82004 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82004 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82004 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x81cb4 | `(unknown callee) 0x1f1dc` | `0x6` | `r1` | `r2` | `r3` |
| `bl` at 0x81cd0 | `(unknown callee) 0x1f1dc` | `0x7` | `?` | `?` | `?` |
| `bl` at 0x81d24 | `(unknown callee) 0x1b124` | `?` | `[0x1490f4]` | `(([0x1490f4] + [0x1490f8]) + [0x1490fc])` | `?` |
| `bl` at 0x81d40 | `(unknown callee) 0x1afa4` | `0x111404` | `?` | `?` | `[[sp-24]]` |
| `bl` at 0x81d84 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111434` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x81d90 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111468` | `?` | `?` |
| `bl` at 0x81d9c | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x11149c` | `?` | `?` |
| `bl` at 0x81db4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1114d0` | `?` | `[sp-36]` |
| `bl` at 0x81dc0 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111504` | `?` | `?` |
| `bl` at 0x81dcc | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111538` | `?` | `?` |
| `bl` at 0x81dd8 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111568` | `?` | `?` |
| `bl` at 0x81de8 | `(unknown callee) 0x1b508` | `[sp-40]` | `0x11159c` | `[[sp-24]]` | `[sp-32]` |
| `bl` at 0x81df0 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81ea4 | `(unknown callee) 0x1b49c` | `([sp-40] + ret(0x1b22c))` | `0x111640` | `[0x1490f0]` | `0x1490e4` |
| `bl` at 0x81eb4 | `(unknown callee) 0x1b508` | `([sp-40] + ret(0x1b22c))` | `0x111670` | `ret(0x1b22c)` | `[0x1490e4]` |
| `bl` at 0x81ed4 | `(unknown callee) 0x1b49c` | `([sp-40] + ret(0x1b22c))` | `0x111680` | `[0x1490e4]` | `[0x1490f0]` |
| `bl` at 0x81edc | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81f0c | `(unknown callee) 0x1b2d4` | `([sp-40] + ret(0x1b22c))` | `0x11110c` | `ret(0x1b22c)` | `[sp-28]` |
| `bl` at 0x81f14 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81f30 | `(unknown callee) 0x808b4` | `([sp-40] + ret(0x1b22c))` | `?` | `ret(0x1b22c)` | `([sp-40] + ret(0x1b22c))` |
| `bl` at 0x81f38 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81f54 | `(unknown callee) 0x80db8` | `([sp-40] + ret(0x1b22c))` | `?` | `ret(0x1b22c)` | `([sp-40] + ret(0x1b22c))` |
| `bl` at 0x81f5c | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81f78 | `(unknown callee) 0x81070` | `([sp-40] + ret(0x1b22c))` | `?` | `ret(0x1b22c)` | `([sp-40] + ret(0x1b22c))` |
| `bl` at 0x81f80 | `(unknown callee) 0x1b22c` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x81fac | `(unknown callee) 0x1b2d4` | `([sp-40] + ret(0x1b22c))` | `0x111434` | `ret(0x1b22c)` | `[sp-32]` |
| `bl` at 0x81fb8 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111468` | `?` | `?` |
| `bl` at 0x81fc4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x11149c` | `?` | `?` |
| `bl` at 0x81fdc | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x1114d0` | `?` | `[sp-36]` |
| `bl` at 0x81fe8 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111504` | `?` | `?` |
| `bl` at 0x81ff4 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111538` | `?` | `?` |
| `bl` at 0x82000 | `(unknown callee) 0x1b2d4` | `[sp-40]` | `0x111568` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x81d38 | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0x81d78 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x81da8 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `beq` at 0x81e1c | `[0x1490e4] == 0x0` | `[0x1490e4] != 0x0` |
| `beq` at 0x81e28 | `[0x1490e4] == 0x1` | `[0x1490e4] != 0x1` |
| `ldrls` at 0x81e3c | `[0x1490f0] <= 0x3 (unsigned, 32-bit)` | `[0x1490f0] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x81efc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x81fa0 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x81fd0 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82004 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x81e3c).

## `EnvAuto`

- `EnvAuto`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x820ac to 0x82118.
  Established. The entry guard takes the bypass for `r0 == 0x0` and falls through for nonzero (`beq`
  at 0x820c8). The call to `0x1d38c` carries `0x7`, unknown, entry r2, unknown (`bl` at 0x820e4). A
  later call to that target has `0x6` and three unknown candidates (`bl` at 0x8210c). The return
  propagates `ret(0x82140)` (`ldmdb` at 0x82114). Automatic-control polarity remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x820bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x820c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x820cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x820e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82108 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x82114 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82114 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82114 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x820e4 | `(unknown callee) 0x1d38c` | `0x7` | `?` | `r2` | `?` |
| `bl` at 0x82100 | `(unknown callee) 0x1d38c` | `0x7d` | `?` | `?` | `?` |
| `bl` at 0x8210c | `(unknown callee) 0x1d38c` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x82110 | `(unknown callee) 0x82140` | `ret(0x1d38c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x820c8 | `r0 == 0x0` | `r0 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82114 | `r0` | `ret(0x82140)` |

## `EnvAltitude`

- `EnvAltitude`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82118 to 0x82140.
  Established. The call to `0x1d38c` receives `0xc`, entry r0, entry r2, entry r3 (`bl` at 0x82134);
  its result becomes the first candidate of the call to `0x82140`, with the rest unknown (`bl` at
  0x82138). The latter return expression reaches r0 at exit (`ldmdb` at 0x8213c). Altitude units and
  range checks are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82128 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82130 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8213c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8213c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8213c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82134 | `(unknown callee) 0x1d38c` | `0xc` | `r0` | `r2` | `r3` |
| `bl` at 0x82138 | `(unknown callee) 0x82140` | `ret(0x1d38c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8213c | `r0` | `ret(0x82140)` |

## `EnvReset`

- `EnvReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82140 to 0x82204.
  Established. The initial test takes its continuation for `[0x148dec] != 0x0` and falls through on
  zero (`bne` at 0x8215c). A word store at absolute `0x1490e4` has unknown value (`str` at 0x82184).
  The call to `0x7bd40` uses `ret(0x1f1dc)` and three unknown candidates (`bl` at 0x82188). The
  indirect call appears in the call table (`mov` at 0x821a8). Reset scope and r0 at the merged
  exit remain unknown (`ldmdb` at 0x821f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x82150 | load | 4 | `abs 0x821f8` | `-` |
| `ldr` at 0x82154 | load | 4 | `abs 0x148dec` | `-` |
| `ldr` at 0x82168 | load | 4 | `abs 0x821fc` | `-` |
| `str` at 0x82184 | store | 4 | `abs 0x1490e4` | `?` |
| `ldr` at 0x8218c | load | 4 | `abs 0x82200` | `-` |
| `ldr` at 0x82190 | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x8219c | load | 4 | `abs 0x82200` | `-` |
| `ldr` at 0x821a0 | load | 4 | `abs 0x2ae584` | `-` |
| `ldr` at 0x821ac | load | 4 | `abs 0x821fc` | `-` |
| `ldr` at 0x821b0 | load | 4 | `abs 0x1490e4` | `-` |
| `ldr` at 0x821bc | load | 4 | `abs 0x821fc` | `-` |
| `ldr` at 0x821c0 | load | 4 | `abs 0x1490f0` | `-` |
| `str` at 0x821d0 | store | 4 | `sp -0x18` | `0x6` |
| `str` at 0x821dc | store | 4 | `sp -0x18` | `0x2` |
| `str` at 0x821e8 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x821ec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x821f4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x821f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x821f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x821f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x82164 | `(unknown callee) 0x7ae5c` | `r0` | `r1` | `r2` | `[0x148dec]` |
| `bl` at 0x82170 | `(unknown callee) 0x1f1dc` | `0x6` | `?` | `?` | `?` |
| `bl` at 0x82188 | `(unknown callee) 0x7bd40` | `ret(0x1f1dc)` | `?` | `?` | `?` |
| `mov` at 0x821a8 | `(unknown callee) (unknown target)` | `ret(0x7bd40)` | `?` | `?` | `[0x2ae584]` |
| `bl` at 0x821f0 | `(unknown callee) 0x75270` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8215c | `[0x148dec] != 0x0` | `[0x148dec] == 0x0` |
| `beq` at 0x82198 | `[0x2ae584] == 0x0` | `[0x2ae584] != 0x0` |
| `bne` at 0x821b8 | `[0x1490e4] != 0x0` | `[0x1490e4] == 0x0` |
| `beq` at 0x821c8 | `[0x1490f0] == 0x0` | `[0x1490f0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x821f4 | `r0` | `?` |

## `EnvTestEvent`

- `EnvTestEvent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82204 to 0x82328.
  Established. The computed branch takes its unresolved edge for unsigned `r0 <= 0x4` and falls
  through for greater (`ldrls` at 0x82220). The sole recognized exit on known edges retains entry r0
  (`ldmdb` at 0x82308). No calls are reported on those edges. The decoded selector arms do not
  establish event semantics without resolved destinations.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82214 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82218 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x82220 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x82308 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82308 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x82220 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82308 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x82220).

## `EnvGetStatus`

- `EnvGetStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82448 to 0x82484.
  Established. The word store writes `[0x142884]` at entry `r0 +0x0` (`str` at 0x82468).
  The exit r0 value is `[0x1490fc]` (`ldmdb` at 0x82478). No calls or conditional branches are
  reported. Status encoding and count units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82458 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8245c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82460 | load | 4 | `abs 0x8247c` | `-` |
| `ldr` at 0x82464 | load | 4 | `abs 0x142884` | `-` |
| `str` at 0x82468 | store | 4 | `r0 +0x0` | `[0x142884]` |
| `ldr` at 0x8246c | load | 4 | `abs 0x82480` | `-` |
| `ldr` at 0x82470 | load | 4 | `abs 0x1490fc` | `-` |
| `ldmdb` at 0x82478 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82478 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82478 | `r0` | `[0x1490fc]` |

## `EnvMessages`

- `EnvMessages`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82504 to 0x825cc.
  Established. Word stores set absolute `0x148df8` to `0x1` and `0x0` in separate arms (`str` at
  0x82588, `str` at 0x825a0). The selector comparison takes its branch for `r0 == 0x1` and falls
  through on inequality (`beq` at 0x82528). The following signed comparison takes the branch for `r0
  > 0x1` and falls through for less or equal (`bgt` at 0x82534). The exit reads r0 as `[sp-24]`
  (`ldmdb` at 0x825b4). Message-control semantics remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82514 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82518 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8251c | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x82520 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8252c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82538 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x82558 | load | 4 | `abs 0x825b8` | `-` |
| `ldr` at 0x8255c | load | 4 | `abs 0x148df8` | `-` |
| `ldr` at 0x82568 | load | 4 | `abs 0x825bc` | `-` |
| `str` at 0x8256c | store | 4 | `sp -0x18` | `0x11175c` |
| `ldr` at 0x82574 | load | 4 | `abs 0x825c0` | `-` |
| `str` at 0x82578 | store | 4 | `sp -0x18` | `0x111780` |
| `ldr` at 0x82580 | load | 4 | `abs 0x825b8` | `-` |
| `str` at 0x82588 | store | 4 | `abs 0x148df8` | `0x1` |
| `ldr` at 0x8258c | load | 4 | `abs 0x825c4` | `-` |
| `str` at 0x82590 | store | 4 | `sp -0x18` | `0x1117a4` |
| `ldr` at 0x82598 | load | 4 | `abs 0x825b8` | `-` |
| `str` at 0x825a0 | store | 4 | `abs 0x148df8` | `0x0` |
| `ldr` at 0x825a4 | load | 4 | `abs 0x825c8` | `-` |
| `str` at 0x825a8 | store | 4 | `sp -0x18` | `0x1117d0` |
| `ldr` at 0x825b0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x825b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x825b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x825b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x82528 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0x82534 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x82540 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x82550 | `r0 == 0x2` | `r0 != 0x2` |
| `beq` at 0x82564 | `[0x148df8] == 0x0` | `[0x148df8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x825b4 | `r0` | `[sp-24]` |

## `L1CfgEventEnvironmental`

- `L1CfgEventEnvironmental`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75270 to 0x752c8.
  Established. The word store at `ret(0x74908) +0x5c` carries `[sp-20]` (`str` at 0x752b0); its
  deciding test takes the bypass for `[sp-20] == [ret(0x74908)+92]` and falls through on inequality
  (`beq` at 0x752a4). The fall-through call to `0x74c78` has `0x8`, `0x0`, `0x0`, `[sp-20]` (`bl` at
  0x752c0). Event publication behavior and the merged r0 value remain unestablished (`ldmdb` at
  0x752c4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75280 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75290 | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0x75294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7529c | load | 4 | `ret(0x74908) +0x5c` | `-` |
| `ldr` at 0x752a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x752ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x752b0 | store | 4 | `ret(0x74908) +0x5c` | `[sp-20]` |
| `ldmdb` at 0x752c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x752c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x752c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75288 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x752c0 | `(unknown callee) 0x74c78` | `0x8` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x752a4 | `[sp-20] == [ret(0x74908)+92]` | `[sp-20] != [ret(0x74908)+92]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x752c4 | `r0` | `?` |

## `monitorReset`

- `monitorReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc4d8 to 0xcc7c8.
  Established. Two byte stores clear an unknown target and `[sp-36] +0x0` (`strb` at 0xcc57c, `strb`
  at 0xcc588). The earlier byte comparison takes its bypass for `[[sp-32]]:1 != 0x63` and falls
  through on equality (`bne` at 0xcc560). Calls on known edges include `0xce238` with `[sp-24]`,
  `[sp-37]:1`, unknown, `[sp-37]:1` (`bl` at 0xcc548). Monitor type encoding, excluded selector
  paths and r0 at the recognized exit remain unresolved (`ldmdb` at 0xcc7a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc4e8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xcc4ec | load | 4 | `abs 0xcc7a8` | `-` |
| `ldr` at 0xcc4f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc4f8 | load | 4 | `abs 0x2ad200` | `-` |
| `str` at 0xcc500 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xcc508 | store | 1 | `sp -0x25` | `0x0` |
| `ldrb` at 0xcc50c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcc510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcc514 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xcc524 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcc528 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcc538 | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xcc53c | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcc540 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xcc550 | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xcc554 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc558 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xcc564 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc568 | load | 1 | `sp -0x25` | `-` |
| `ldr` at 0xcc570 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0xcc57c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xcc580 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xcc588 | store | 1 | `[sp-36] +0x0` | `0x0` |
| `ldr` at 0xcc58c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcc590 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xcc59c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc5a0 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrls` at 0xcc5ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcc658 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc660 | load | 4 | `abs 0xcc7ac` | `-` |
| `ldr` at 0xcc664 | load | 4 | `abs 0xcc7b0` | `-` |
| `str` at 0xcc690 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc69c | load | 4 | `abs 0xcc7ac` | `-` |
| `ldr` at 0xcc6a0 | load | 4 | `abs 0xcc7b0` | `-` |
| `ldr` at 0xcc6ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc6bc | load | 4 | `abs 0xcc7b4` | `-` |
| `str` at 0xcc6c0 | store | 4 | `(unknown)` | `0x133e68` |
| `str` at 0xcc6c4 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xcc6c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc6cc | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xcc6d0 | store | 4 | `(unknown)` | `[[sp-32]]:1` |
| `ldr` at 0xcc6d8 | load | 4 | `abs 0xcc7b8` | `-` |
| `ldr` at 0xcc6dc | load | 4 | `abs 0xcc7bc` | `-` |
| `ldr` at 0xcc6e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc6f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc6fc | load | 4 | `abs 0xcc7ac` | `-` |
| `ldr` at 0xcc700 | load | 4 | `abs 0xcc7b0` | `-` |
| `str` at 0xcc72c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xcc730 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcc738 | load | 4 | `abs 0xcc7ac` | `-` |
| `ldr` at 0xcc73c | load | 4 | `abs 0xcc7b0` | `-` |
| `ldr` at 0xcc748 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc758 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xcc764 | load | 4 | `abs 0xcc7c0` | `-` |
| `str` at 0xcc768 | store | 4 | `(unknown)` | `0x133e80` |
| `str` at 0xcc76c | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldr` at 0xcc770 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcc774 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0xcc778 | store | 4 | `(unknown)` | `[[sp-32]+8]` |
| `str` at 0xcc77c | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0xcc784 | load | 4 | `abs 0xcc7b8` | `-` |
| `ldr` at 0xcc788 | load | 4 | `abs 0xcc7c4` | `-` |
| `ldrb` at 0xcc794 | load | 1 | `sp -0x25` | `-` |
| `strb` at 0xcc79c | store | 1 | `sp -0x25` | `([sp-37]:1+1 & 0xff)` |
| `ldmdb` at 0xcc7a4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xcc7a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcc7a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcc7a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc530 | `(unknown callee) 0xce118` | `?` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0xcc548 | `(unknown callee) 0xce238` | `[sp-24]` | `[sp-37]:1` | `?` | `[sp-37]:1` |
| `bl` at 0xcc668 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc674 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc6a4 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc6b4 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc6e4 | `(unknown callee) 0x21e28` | `0x9` | `0x133d7c` | `0x195` | `0x0` |
| `bl` at 0xcc704 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc710 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc740 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x133dd4` | `0x133ddc` |
| `bl` at 0xcc750 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xcc75c | `(unknown callee) 0x219a8` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xcc790 | `(unknown callee) 0x21e28` | `0x1` | `0x133d7c` | `0x19b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xcc51c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xcc560 | `[[sp-32]]:1 != 0x63` | `[[sp-32]]:1 == 0x63` |
| `beq` at 0xcc598 | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0xcc5ac | `[[sp-32]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-32]]:1-1 > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xcc6f0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcc7a4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcc5ac).

## `MInfoReset`

- `MInfoReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6a8c to 0xc6b2c.
  Established. Two word stores clear absolute `0x149e24` and `0x149e28` (`str` at 0xc6ab8, `str` at
  0xc6ac4). The entry test takes the continuation for `[0x149e3c] != 0x0` and falls through on zero
  (`bne` at 0xc6aa8). The first call to `0x1b43c` carries `0x179e40`, `0x0`, `0x66664`, `0x0` (`bl`
  at 0xc6ad4). Memory-region ownership and the common return value are unknown (`ldmdb` at 0xc6b08).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc6a9c | load | 4 | `abs 0xc6b0c` | `-` |
| `ldr` at 0xc6aa0 | load | 4 | `abs 0x149e3c` | `-` |
| `ldr` at 0xc6ab0 | load | 4 | `abs 0xc6b10` | `-` |
| `str` at 0xc6ab8 | store | 4 | `abs 0x149e24` | `0x0` |
| `ldr` at 0xc6abc | load | 4 | `abs 0xc6b14` | `-` |
| `str` at 0xc6ac4 | store | 4 | `abs 0x149e28` | `0x0` |
| `ldr` at 0xc6ac8 | load | 4 | `abs 0xc6b18` | `-` |
| `ldr` at 0xc6ad0 | load | 4 | `abs 0xc6b1c` | `-` |
| `ldr` at 0xc6ad8 | load | 4 | `abs 0xc6b20` | `-` |
| `ldr` at 0xc6ae0 | load | 4 | `abs 0xc6b1c` | `-` |
| `ldr` at 0xc6ae8 | load | 4 | `abs 0xc6b24` | `-` |
| `ldr` at 0xc6af0 | load | 4 | `abs 0xc6b1c` | `-` |
| `ldr` at 0xc6af8 | load | 4 | `abs 0xc6b28` | `-` |
| `ldmdb` at 0xc6b08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6b08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6b08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6ad4 | `(unknown callee) 0x1b43c` | `0x179e40` | `0x0` | `0x66664` | `0x0` |
| `bl` at 0xc6ae4 | `(unknown callee) 0x1b43c` | `0x1e04a4` | `0x0` | `0x66664` | `?` |
| `bl` at 0xc6af4 | `(unknown callee) 0x1b43c` | `0x246b08` | `0x0` | `0x66664` | `?` |
| `bl` at 0xc6b04 | `(unknown callee) 0x1b43c` | `0x149e40` | `0x0` | `0x30000` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc6aa8 | `[0x149e3c] != 0x0` | `[0x149e3c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6b08 | `r0` | `?` |

## `ramLogReset`

- `ramLogReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb988c to 0xb98d4.
  Established. The entry-r0 test takes its continuation on nonzero and falls through on zero (`bne`
  at 0xb98a8). Word stores carry `[r0+8]` at `r0 +0x10` and `[[sp-20]+8]` at `[sp-20] +0x14` (`str`
  at 0xb98bc, `str` at 0xb98cc). Entry r0 is the reported exit value (`ldmdb` at 0xb98d0).
  Log-record field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb989c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb98a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb98b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb98b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb98b8 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xb98bc | store | 4 | `r0 +0x10` | `[r0+8]` |
| `ldr` at 0xb98c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb98c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb98c8 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xb98cc | store | 4 | `[sp-20] +0x14` | `[[sp-20]+8]` |
| `ldmdb` at 0xb98d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb98d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb98d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb98a8 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb98d0 | `r0` | `r0` |
