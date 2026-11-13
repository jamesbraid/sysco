# ARM L2 discovery and scan-control paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/l2-discovery-scan-control-facts.txt) retain all tool sections.

## `L1CfgFindL2`

- `L1CfgFindL2`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f9f8 to 0x70e7c.
  Established. The only call reaches `0x4f090` with entry r0, entry r1, entry r2, entry r3 (`bl` at
  0x6fa08). Later word reads address `0x1489b8` and `0x1489c0` (`ldr` at 0x6fa30, `ldr` at 0x6fa58).
  An unsigned comparison takes its alternate arm when an unknown value is at least `[[sp-28]+20]`
  and falls through for less (`bhs` at 0x6fad0). Two later stores write unknown words at
  `sp -0x20` (`str` at 0x70448, `str` at 0x70e70). L2 selection policy, record layout and the final
  r0 value remain unknown (`ldmdb` at 0x70e78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6fa14 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x6fa1c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x6fa24 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x6fa28 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fa2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fa30 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6fa40 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fa44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fa58 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fa60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fa6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fa78 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fa7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fa90 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x6fa98 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6faa0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6faa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fab8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fac0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fac4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fac8 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x6fad4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6faec | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x6faf4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6fafc | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fb00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fb14 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fb1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fb20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fb24 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x6fb30 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fb34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fb38 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x6fb54 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fb5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fb68 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fb6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6fb70 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x6fb8c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fb94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fba4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fba8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fbac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fbc0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fbc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fbe4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fbec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fbf8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fbfc | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fc00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fc14 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fc1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fc38 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fc40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fc4c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fc50 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fc54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fc68 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fc70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fc8c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fc94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fca0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fca4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fcbc | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fcc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fce0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fce8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fcf4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fcf8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fcfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fd10 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fd18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fd34 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fd3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fd48 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fd4c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fd50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fd64 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fd6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fd88 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fd90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fd9c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fda0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fda4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fdb8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fdc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fddc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fde4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fdf0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fdf4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fdf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fe0c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fe14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fe30 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fe38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fe44 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fe48 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fe4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6fe60 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6fe68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fe84 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fe8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fe98 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fe9c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fea0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6feb4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6febc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6fed8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6fee0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6feec | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fef0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ff08 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ff10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ff2c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ff34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ff40 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6ff44 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6ff48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ff5c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ff64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ff80 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ff88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ff94 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6ff98 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6ff9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ffb0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x6ffb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ffd4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6ffdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6ffe8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6ffec | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x6fff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70004 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7000c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70028 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70030 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7003c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70040 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70058 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70060 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7007c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70084 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70090 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70094 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70098 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x700ac | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x700b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x700d0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x700d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x700e4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x700e8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x700ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70100 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70108 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70124 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7012c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70138 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7013c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70154 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7015c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70178 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70180 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7018c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70190 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x701a8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x701b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x701cc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x701d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x701e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x701e8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x701f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x701f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70208 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70214 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70218 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70228 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70234 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70238 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70244 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70248 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70254 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70258 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70264 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70268 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70274 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70278 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70288 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70298 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702a8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702b8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702d8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702e8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x702f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x702f8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70304 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70308 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70314 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7032c | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70334 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7033c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70354 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7035c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70364 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7037c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7038c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x703a4 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x703ac | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x703b4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x703b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x703cc | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x703d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x703d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x703dc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x703e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x703f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70404 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7041c | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70424 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70428 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x70430 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x70438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70444 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x70448 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x70454 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x70458 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7045c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70460 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x70470 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70488 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70490 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7049c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x704a8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x704ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x704c0 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x704c8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x704d0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x704d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x704e8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x704f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x704f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x704f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70504 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7051c | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70524 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7052c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70530 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70544 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7054c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70554 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70560 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70584 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7058c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70598 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7059c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x705a0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x705bc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x705c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x705d8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x705dc | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x705e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x705f4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x705fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70618 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7062c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70630 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70648 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70650 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7066c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70674 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70680 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70684 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7069c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x706a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x706c0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x706c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x706d4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x706d8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x706dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x706f0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x706f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70714 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7071c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70728 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7072c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70730 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70744 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7074c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70768 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7077c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70780 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70784 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70798 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x707a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x707bc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x707c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x707d0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x707d4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x707d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x707ec | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x707f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70810 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70818 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70824 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70828 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7082c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70840 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70848 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70864 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x7086c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70878 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7087c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70880 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70894 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x7089c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x708b8 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x708c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x708cc | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x708d0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x708d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x708e8 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x708f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7090c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70914 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70920 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70924 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7093c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70944 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70960 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70968 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70974 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70978 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x7097c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70990 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x709b4 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x709bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x709c8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x709cc | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x709d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x709e4 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x709ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a08 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70a10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a1c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70a20 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70a24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70a38 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70a40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a5c | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70a64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70a70 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70a74 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70a8c | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70a94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70ab0 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70ab8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70ac4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70ac8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70acc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70ae0 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70ae8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70b04 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70b0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70b18 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70b1c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70b20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70b34 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70b3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70b58 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70b60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70b6c | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70b70 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70b74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70b88 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70b90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70bac | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70bb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70bc0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70bc4 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70bc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70bdc | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70be4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70c00 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x70c08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70c18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c1c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c2c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c3c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c4c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c5c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c6c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c7c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c8c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70c98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70c9c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ca8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cac | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70cb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cbc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70cc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70ccc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cdc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70ce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cec | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70cf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70cfc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70d08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d0c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70d18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d1c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70d28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d2c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70d38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d3c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x70d48 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70d60 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70d68 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70d70 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70d74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70d88 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70d94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70d98 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70da4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70db0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70dc0 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70dd8 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70de0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70de8 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70e00 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x70e08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x70e0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x70e10 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x70e1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x70e38 | load | 4 | `abs 0x705d4` | `-` |
| `ldr` at 0x70e3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x70e50 | load | 4 | `abs 0x1489c0` | `-` |
| `str` at 0x70e58 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x70e5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x70e64 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x70e6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x70e70 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x70e74 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x70e78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x70e78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x70e78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6fa08 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6fa38 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6fa68 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x6fa74 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0x6fad0 | `? >= [[sp-28]+20] (unsigned, 32-bit)` | `? < [[sp-28]+20] (unsigned, 32-bit)` |
| `bne` at 0x6fb2c | `? != [[sp-28]+20]` | `? == [[sp-28]+20]` |
| `beq` at 0x6fb64 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x6fb9c | `? == 0x44` | `? != 0x44` |
| `beq` at 0x6fbf4 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x6fc48 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x6fc9c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x6fcf0 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x6fd44 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x6fd98 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x6fdec | `? == 0x39` | `? != 0x39` |
| `beq` at 0x6fe40 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x6fe94 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x6fee8 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x6ff3c | `? == 0x51` | `? != 0x51` |
| `beq` at 0x6ff90 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x6ffe4 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x70038 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x7008c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x700e0 | `? == 0x48` | `? != 0x48` |
| `beq` at 0x70134 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x70188 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x701dc | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x701f0 | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x70200 | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x70210 | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x70220 | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x70230 | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x70240 | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x70250 | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x70260 | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x70270 | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x70280 | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x70290 | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x702a0 | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x702b0 | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x702c0 | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x702d0 | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x702e0 | `[[sp-28]+8] == 0x48` | `[[sp-28]+8] != 0x48` |
| `beq` at 0x702f0 | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x70300 | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x70310 | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x7036c | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x70378 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x70384 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x703e4 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x703f0 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x703fc | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x70440 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x70468 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x70498 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x704a4 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x70500 | `C == 1` | `not (C == 1)` |
| `bne` at 0x7055c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x70594 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x705cc | `? == 0x44` | `? != 0x44` |
| `beq` at 0x70628 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x7067c | `? == 0x33` | `? != 0x33` |
| `beq` at 0x706d0 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x70724 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x70778 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x707cc | `? == 0x36` | `? != 0x36` |
| `beq` at 0x70820 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x70874 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x708c8 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x7091c | `? == 0x30` | `? != 0x30` |
| `beq` at 0x70970 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x709c4 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x70a18 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x70a6c | `? == 0x42` | `? != 0x42` |
| `beq` at 0x70ac0 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x70b14 | `? == 0x48` | `? != 0x48` |
| `beq` at 0x70b68 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x70bbc | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x70c10 | `? == 0x7c` | `? != 0x7c` |
| `beq` at 0x70c24 | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x70c34 | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x70c44 | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x70c54 | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x70c64 | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x70c74 | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x70c84 | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x70c94 | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x70ca4 | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x70cb4 | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x70cc4 | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x70cd4 | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x70ce4 | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x70cf4 | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x70d04 | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x70d14 | `[[sp-28]+8] == 0x48` | `[[sp-28]+8] != 0x48` |
| `beq` at 0x70d24 | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x70d34 | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x70d44 | `[[sp-28]+8] == 0x7c` | `[[sp-28]+8] != 0x7c` |
| `bhs` at 0x70da0 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x70dac | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x70db8 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `bhs` at 0x70e18 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x70e24 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x70e30 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x70e78 | `r0` | `?` |

## `SMPCmdIRouter`

- `SMPCmdIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9a1b8 to 0x9a4cc.
  Established. Two indirect calls have unknown targets and known continuations, with candidates
  `0x1`, `[sp-32]`, `sp-2088`, `sp-2088` and `0x2`, `[sp-32]`, `sp-2088`, `sp-2088`, respectively
  (`mov` at 0x9a3cc, `mov` at 0x9a420). A later word store clears `0x1497ec` to `0x0` (`str` at
  0x9a434). The call to `0xb6bdc` has `0x2b1820`, `0x0`, `[sp-2104]`, `[sp-32]` (`bl` at 0x9a458).
  No return is recognized in the selected span. Command transport and callback meanings remain
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9a1cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9a1d0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9a1e4 | load | 4 | `abs 0x9a4a4` | `-` |
| `str` at 0x9a1f4 | store | 4 | `sp -0x1c` | `ret(0xb5698)` |
| `ldr` at 0x9a1f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a204 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a210 | load | 4 | `abs 0x9a4a8` | `-` |
| `str` at 0x9a214 | store | 4 | `sp -0x848` | `0x11da90` |
| `str` at 0x9a218 | store | 4 | `sp -0x844` | `ret(0x21674)` |
| `ldr` at 0x9a220 | load | 4 | `abs 0x9a4ac` | `-` |
| `ldr` at 0x9a224 | load | 4 | `abs 0x9a4b0` | `-` |
| `ldr` at 0x9a230 | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x9a234 | load | 1 | `[sp-2104] +0x20` | `-` |
| `str` at 0x9a244 | store | 4 | `sp -0x834` | `0x0` |
| `ldr` at 0x9a24c | load | 4 | `sp -0x838` | `-` |
| `ldrb` at 0x9a250 | load | 1 | `[sp-2104] +0x2c` | `-` |
| `ldr` at 0x9a260 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x9a264 | load | 4 | `[sp-2104] +0x30` | `-` |
| `str` at 0x9a268 | store | 4 | `sp -0x840` | `[[sp-2104]+48]` |
| `ldr` at 0x9a270 | load | 4 | `sp -0x838` | `-` |
| `str` at 0x9a27c | store | 4 | `sp -0x840` | `ret(0xb7944)` |
| `ldr` at 0x9a280 | load | 4 | `sp -0x840` | `-` |
| `str` at 0x9a284 | store | 4 | `sp -0x834` | `[sp-2112]` |
| `ldr` at 0x9a294 | load | 4 | `sp -0x834` | `-` |
| `ldr` at 0x9a29c | load | 4 | `sp -0x838` | `-` |
| `str` at 0x9a2a8 | store | 4 | `sp -0x1c` | `ret(0x98eb4)` |
| `ldr` at 0x9a2c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a2cc | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x9a2dc | store | 4 | `sp -0x830` | `0x0` |
| `ldr` at 0x9a2e0 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9a2e8 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9a2f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a300 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9a308 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9a310 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9a314 | store | 4 | `sp -0x83c` | `?` |
| `ldr` at 0x9a318 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9a320 | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9a328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a348 | load | 4 | `sp -0x830` | `-` |
| `ldr` at 0x9a350 | load | 4 | `sp -0x82c` | `-` |
| `str` at 0x9a35c | store | 4 | `sp -0x848` | `0x0` |
| `ldr` at 0x9a360 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9a36c | load | 4 | `sp -0x830` | `-` |
| `str` at 0x9a374 | store | 4 | `sp -0x830` | `[sp-2096]+1` |
| `ldr` at 0x9a37c | load | 4 | `sp -0x82c` | `-` |
| `ldr` at 0x9a384 | load | 4 | `abs 0x9a4b4` | `-` |
| `ldr` at 0x9a388 | load | 4 | `abs 0x1499b0` | `-` |
| `ldr` at 0x9a394 | load | 4 | `abs 0x9a4b8` | `-` |
| `ldr` at 0x9a398 | load | 4 | `abs 0x1497ec` | `-` |
| `ldr` at 0x9a3a8 | load | 4 | `abs 0x9a4b4` | `-` |
| `ldr` at 0x9a3b8 | load | 4 | `abs 0x1499b0` | `-` |
| `ldr` at 0x9a3c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a3d4 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x9a3d8 | load | 4 | `abs 0x9a4b4` | `-` |
| `ldr` at 0x9a3dc | load | 4 | `abs 0x1499b0` | `-` |
| `ldr` at 0x9a3e8 | load | 4 | `abs 0x9a4b8` | `-` |
| `ldr` at 0x9a3ec | load | 4 | `abs 0x1497ec` | `-` |
| `ldr` at 0x9a3fc | load | 4 | `abs 0x9a4b4` | `-` |
| `ldr` at 0x9a40c | load | 4 | `abs 0x1499b0` | `-` |
| `ldr` at 0x9a414 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a428 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x9a42c | load | 4 | `abs 0x9a4b8` | `-` |
| `str` at 0x9a434 | store | 4 | `abs 0x1497ec` | `0x0` |
| `str` at 0x9a444 | store | 4 | `sp -0x848` | `sp-2088` |
| `ldr` at 0x9a448 | load | 4 | `abs 0x9a4bc` | `-` |
| `ldr` at 0x9a450 | load | 4 | `sp -0x838` | `-` |
| `ldr` at 0x9a454 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9a460 | store | 4 | `sp -0x1c` | `ret(0xb6bdc)` |
| `ldr` at 0x9a464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a470 | load | 4 | `abs 0x9a4c0` | `-` |
| `ldr` at 0x9a474 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x9a478 | load | 4 | `abs 0x9a4c4` | `-` |
| `ldr` at 0x9a480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9a48c | load | 4 | `abs 0x9a4c8` | `-` |
| `ldr` at 0x9a498 | load | 4 | `sp -0x838` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9a1ec | `(unknown callee) 0xb5698` | `sp-2104` | `0x2b1828` | `?` | `sp-2104` |
| `bl` at 0x9a208 | `(unknown callee) 0x21674` | `ret(0xb5698)` | `?` | `?` | `ret(0xb5698)` |
| `bl` at 0x9a22c | `(unknown callee) 0x21e28` | `0x9` | `0x11d884` | `0x19b5` | `0x0` |
| `bl` at 0x9a278 | `(unknown callee) 0xb7944` | `[sp-2104]` | `0x1` | `?` | `?` |
| `bl` at 0x9a2a0 | `(unknown callee) 0x98eb4` | `?` | `sp-2092` | `[sp-2104]` | `sp-2092` |
| `bl` at 0x9a2c8 | `(unknown callee) 0x6907c` | `sp-32` | `sp-2088` | `ret(0x98eb4)` | `sp-2088` |
| `bl` at 0x9a32c | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0x9a368 | `(unknown callee) 0x692f8` | `sp-32` | `sp-2088` | `?` | `ret(0x1b22c)` |
| `bl` at 0x9a380 | `(unknown callee) 0x1b400` | `[sp-2092]` | `?` | `?` | `?` |
| `mov` at 0x9a3cc | `(unknown callee) (unknown target)` | `0x1` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `mov` at 0x9a420 | `(unknown callee) (unknown target)` | `0x2` | `[sp-32]` | `sp-2088` | `sp-2088` |
| `bl` at 0x9a458 | `(unknown callee) 0xb6bdc` | `0x2b1820` | `0x0` | `[sp-2104]` | `[sp-32]` |
| `bl` at 0x9a47c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x11dab4` | `?` | `0x144dc8` |
| `bl` at 0x9a484 | `(unknown callee) 0x21674` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x9a494 | `(unknown callee) 0x2070c` | `0x11dad4` | `ret(0x21674)` | `?` | `ret(0x21674)` |
| `bl` at 0x9a49c | `(unknown callee) 0xb56f4` | `[sp-2104]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9a200 | `ret(0xb5698) == 0x0` | `ret(0xb5698) != 0x0` |
| `beq` at 0x9a23c | `[[sp-2104]+32]:1 == 0x2` | `[[sp-2104]+32]:1 != 0x2` |
| `bne` at 0x9a25c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9a2d4 | `[sp-2092] == 0x0` | `[sp-2092] != 0x0` |
| `bne` at 0x9a2f8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9a390 | `[0x1499b0] == 0x0` | `[0x1499b0] != 0x0` |
| `beq` at 0x9a3a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9a3e4 | `[0x1499b0] == 0x0` | `[0x1499b0] != 0x0` |
| `beq` at 0x9a3f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9a46c | `ret(0xb6bdc) == 0x0` | `ret(0xb6bdc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `LpbkIRouter`

- `LpbkIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3114 to 0xa3578.
  Established. The halfword store writes `(ret(0x1b22c)+1 & 0xffff)` at `[sp-92] +0x2` (`strh` at
  0xa34f4). The call to `0xb6f64` has `0x2b19c0`, `sp-24`, `0x0`, `0x1` (`bl` at 0xa3534); its
  nonzero result takes the branch and zero falls through (`bne` at 0xa3548). The taken arm calls
  `0xb6798` with `0x2b19c0`, `ret(0xb6f64)`, unknown, `ret(0xb6f64)` (`bl` at 0xa3560). The r0 return is
  `ret(0xb56f4)` (`ldmdb` at 0xa356c). Loopback behavior and packet-field meanings remain
  unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3124 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa312c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0xa3130 | store | 4 | `sp -0x18` | `[r0+28]` |
| `str` at 0xa3138 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xa313c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa3140 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3144 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3150 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3160 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3164 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3178 | load | 1 | `(unknown)` | `-` |
| `str` at 0xa3180 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xa3184 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa3190 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa319c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa31ac | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xa31bc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa31d8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa31dc | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa31f8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa31fc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3214 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xa3218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa321c | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3230 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3244 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3258 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa325c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xa3264 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3268 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3270 | store | 4 | `sp -0x48` | `ret(0xb7944)` |
| `ldr` at 0xa3274 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xa3278 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xa327c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xa3280 | load | 4 | `sp -0x40` | `-` |
| `strh` at 0xa329c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xa32a4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa32c0 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xa32c4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa32e0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa32e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa32fc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xa3300 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3304 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3318 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa332c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3340 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3344 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa334c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3350 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3358 | store | 4 | `sp -0x50` | `ret(0xb7944)` |
| `ldr` at 0xa335c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0xa3360 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xa3364 | store | 4 | `[sp-76] +0x0` | `?` |
| `ldr` at 0xa3368 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3380 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3384 | load | 4 | `sp -0x40` | `-` |
| `ldrh` at 0xa3398 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xa339c | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xa33a4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa33c0 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xa33c4 | load | 4 | `sp -0x40` | `-` |
| `strb` at 0xa33e0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xa33e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa33fc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa3400 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3404 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa3418 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa3428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa342c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3440 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3444 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa344c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3450 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3458 | store | 4 | `sp -0x58` | `ret(0xb7944)` |
| `ldr` at 0xa345c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xa3460 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xa3464 | store | 4 | `[sp-84] +0x0` | `?` |
| `ldr` at 0xa3468 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3480 | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0xa3484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3488 | load | 4 | `sp -0x40` | `-` |
| `ldrb` at 0xa349c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa34ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa34b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa34c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa34c8 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xa34d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa34d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa34dc | store | 4 | `sp -0x60` | `ret(0xb7944)` |
| `ldr` at 0xa34e0 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xa34f0 | load | 4 | `sp -0x5c` | `-` |
| `strh` at 0xa34f4 | store | 2 | `[sp-92] +0x2` | `(ret(0x1b22c)+1 & 0xffff)` |
| `ldr` at 0xa34f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3500 | store | 4 | `sp -0x40` | `?` |
| `str` at 0xa3510 | store | 4 | `sp -0x70` | `0x2` |
| `ldr` at 0xa3514 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xa3518 | store | 4 | `sp -0x6c` | `?` |
| `str` at 0xa3520 | store | 4 | `sp -0x68` | `sp-60` |
| `ldr` at 0xa3524 | load | 4 | `abs 0xa3570` | `-` |
| `str` at 0xa353c | store | 4 | `sp -0x1c` | `ret(0xb6f64)` |
| `ldr` at 0xa3540 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa354c | load | 4 | `abs 0xa3574` | `-` |
| `ldr` at 0xa3558 | load | 4 | `abs 0xa3570` | `-` |
| `ldr` at 0xa355c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3564 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa356c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa356c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa356c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa326c | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3354 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa3454 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa34d8 | `(unknown callee) 0xb7944` | `[sp-20]` | `[sp-64]` | `0x24` | `?` |
| `bl` at 0xa34e4 | `(unknown callee) 0x1b22c` | `?` | `?` | `?` | `?` |
| `bl` at 0xa3534 | `(unknown callee) 0xb6f64` | `0x2b19c0` | `sp-24` | `0x0` | `0x1` |
| `bl` at 0xa3550 | `(unknown callee) 0x20240` | `0x124d8c` | `?` | `?` | `ret(0xb6f64)` |
| `bl` at 0xa3560 | `(unknown callee) 0xb6798` | `0x2b19c0` | `ret(0xb6f64)` | `?` | `ret(0xb6f64)` |
| `bl` at 0xa3568 | `(unknown callee) 0xb56f4` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xa314c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xa3158 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa318c | `[sp-100] == 0x1` | `[sp-100] != 0x1` |
| `bgt` at 0xa3198 | `[sp-100] > 0x1 (signed, 32-bit)` | `[sp-100] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xa31a4 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xa31b4 | `[sp-100] == 0x2` | `[sp-100] != 0x2` |
| `bne` at 0xa323c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3324 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3424 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa34a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa3548 | `ret(0xb6f64) != 0x0` | `ret(0xb6f64) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa356c | `r0` | `ret(0xb56f4)` |

## `FlashIRouter`

- `FlashIRouter`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa52d4 to 0xa5b68.
  Established. The first equality branch takes its alternate arm for `[sp-32] == 0x73` and falls
  through on inequality (`beq` at 0xa5334). Later calls include `0xcb45c` with `[sp-164]`,
  `[sp-184]`, `sp-168`, `sp-168` and `0xcb48c` with `[sp-164]`, `[sp-196]`, `[[sp-20]+46]:2`,
  `sp-172` (`bl` at 0xa5968, `bl` at 0xa5a94). Each signed-result branch takes its continuation for
  the respective callee result at least `0x0` and falls through for less (`bge` at 0xa599c, `bge` at
  0xa5ac8). Flash access semantics, reply fields and the joined r0 return remain unknown (`ldmdb` at
  0xa5b50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa52e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa52ec | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0xa52f8 | store | 4 | `sp -0x1c` | `ret(0x4f090)` |
| `ldr` at 0xa52fc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa5300 | load | 2 | `[sp-20] +0x22` | `-` |
| `str` at 0xa5304 | store | 4 | `sp -0x20` | `[[sp-20]+34]:2` |
| `str` at 0xa530c | store | 4 | `sp -0x94` | `0x0` |
| `ldr` at 0xa5310 | load | 4 | `abs 0xa5b54` | `-` |
| `ldr` at 0xa5314 | load | 4 | `abs 0x14997c` | `-` |
| `ldr` at 0xa5324 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa5328 | store | 4 | `sp -0xcc` | `[sp-32]` |
| `ldr` at 0xa532c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5338 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5344 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5354 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0xa5364 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5368 | load | 4 | `abs 0x2b36b4` | `-` |
| `ldr` at 0xa5374 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5378 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xa5384 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5388 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa539c | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0xa53ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa53cc | store | 4 | `sp -0x94` | `0x5a` |
| `ldr` at 0xa53d0 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa53d4 | load | 4 | `abs 0x2b36b4` | `-` |
| `ldr` at 0xa53e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa53e4 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xa53f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa53f4 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5404 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5408 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0xa5418 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5438 | store | 4 | `sp -0x94` | `0x5a` |
| `strb` at 0xa5440 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa5448 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa544c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa5450 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa5458 | store | 4 | `sp -0x84` | `([sp-24] + [sp-148])` |
| `strh` at 0xa5460 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa5464 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5468 | load | 4 | `abs 0x2b36c8` | `-` |
| `strb` at 0xa5478 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa5480 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa5484 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5488 | load | 4 | `abs 0x2b36c8` | `-` |
| `ldr` at 0xa548c | load | 4 | `[0x2b36c8] +0x0` | `-` |
| `str` at 0xa5490 | store | 4 | `sp -0x7c` | `[[0x2b36c8]]` |
| `strh` at 0xa5498 | store | 2 | `sp -0x7e` | `0x0` |
| `strb` at 0xa54a4 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa54ac | store | 1 | `sp -0x7f` | `0x0` |
| `str` at 0xa54b4 | store | 4 | `sp -0x7c` | `0x0` |
| `strh` at 0xa54bc | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa54c0 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa54c4 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xa54d0 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa54d4 | load | 4 | `abs 0xa5b58` | `-` |
| `str` at 0xa54dc | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xa54e4 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0xa54ec | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0xa54f4 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xa54f8 | load | 4 | `abs 0x2b36b8` | `-` |
| `str` at 0xa5508 | store | 4 | `sp -0x90` | `ret(0xa4bb8)` |
| `ldr` at 0xa550c | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa551c | store | 1 | `sp -0x78` | `0x0` |
| `strb` at 0xa5524 | store | 1 | `sp -0x77` | `0x0` |
| `str` at 0xa552c | store | 4 | `sp -0x74` | `0x0` |
| `strh` at 0xa5534 | store | 2 | `sp -0x76` | `0x0` |
| `strb` at 0xa553c | store | 1 | `sp -0x70` | `0x0` |
| `strb` at 0xa5544 | store | 1 | `sp -0x6f` | `0x0` |
| `str` at 0xa554c | store | 4 | `sp -0x6c` | `0x0` |
| `strh` at 0xa5554 | store | 2 | `sp -0x6e` | `0x0` |
| `ldr` at 0xa555c | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5560 | load | 4 | `abs 0x2b36a8` | `-` |
| `ldr` at 0xa556c | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5570 | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xa5578 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0xa557c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa5580 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0xa5590 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5598 | store | 4 | `sp -0xa4` | `(ret(0x1af68) + [sp-160])` |
| `strb` at 0xa55a0 | store | 1 | `sp -0x78` | `0x1` |
| `strb` at 0xa55a8 | store | 1 | `sp -0x77` | `0x0` |
| `ldr` at 0xa55ac | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa55b0 | store | 4 | `sp -0x74` | `(ret(0x1af68) + [sp-160])` |
| `ldr` at 0xa55b4 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0xa55c4 | store | 2 | `sp -0x76` | `(ret(0x1b22c)+1 & 0xffff)` |
| `ldr` at 0xa55c8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa55cc | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0xa55dc | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa55e4 | store | 4 | `sp -0xa4` | `(ret(0x1af68) + [sp-160])` |
| `strb` at 0xa55ec | store | 1 | `sp -0x70` | `0x1` |
| `strb` at 0xa55f4 | store | 1 | `sp -0x6f` | `0x0` |
| `ldr` at 0xa55f8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa55fc | store | 4 | `sp -0x6c` | `(ret(0x1af68) + [sp-160])` |
| `ldr` at 0xa5600 | load | 4 | `sp -0xa4` | `-` |
| `strh` at 0xa5610 | store | 2 | `sp -0x6e` | `(ret(0x1b22c)+1 & 0xffff)` |
| `ldr` at 0xa5614 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5618 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xa5624 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5628 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xa5634 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5638 | load | 4 | `abs 0xa5b58` | `-` |
| `str` at 0xa5640 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xa5648 | store | 4 | `sp -0xd4` | `sp-152` |
| `str` at 0xa5650 | store | 4 | `sp -0xd0` | `sp-156` |
| `ldr` at 0xa5658 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xa565c | load | 4 | `abs 0x2b36b8` | `-` |
| `str` at 0xa566c | store | 4 | `sp -0x90` | `ret(0xa4bb8)` |
| `str` at 0xa5678 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xa567c | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa568c | store | 1 | `sp -0x68` | `0x0` |
| `strb` at 0xa5694 | store | 1 | `sp -0x67` | `0x0` |
| `str` at 0xa569c | store | 4 | `sp -0x64` | `0x0` |
| `strh` at 0xa56a4 | store | 2 | `sp -0x66` | `0x0` |
| `strb` at 0xa56ac | store | 1 | `sp -0x60` | `0x0` |
| `strb` at 0xa56b4 | store | 1 | `sp -0x5f` | `0x0` |
| `str` at 0xa56bc | store | 4 | `sp -0x5c` | `0x0` |
| `strh` at 0xa56c4 | store | 2 | `sp -0x5e` | `0x0` |
| `ldr` at 0xa56cc | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa56d0 | load | 4 | `abs 0x2b36ac` | `-` |
| `ldr` at 0xa56dc | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa56e0 | load | 4 | `abs 0x2b36ac` | `-` |
| `str` at 0xa56e8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xa56ec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa56f0 | load | 4 | `[sp-152] +0x14` | `-` |
| `ldr` at 0xa5700 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa5708 | store | 4 | `sp -0xa0` | `(ret(0x1af68) + [sp-164])` |
| `strb` at 0xa5710 | store | 1 | `sp -0x68` | `0x1` |
| `strb` at 0xa5718 | store | 1 | `sp -0x67` | `0x0` |
| `ldr` at 0xa571c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5720 | store | 4 | `sp -0x64` | `(ret(0x1af68) + [sp-164])` |
| `ldr` at 0xa5724 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0xa5734 | store | 2 | `sp -0x66` | `(ret(0x1b22c)+1 & 0xffff)` |
| `ldr` at 0xa5738 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa573c | load | 4 | `[sp-152] +0x10` | `-` |
| `ldr` at 0xa574c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xa5754 | store | 4 | `sp -0xa0` | `(ret(0x1af68) + [sp-164])` |
| `strb` at 0xa575c | store | 1 | `sp -0x60` | `0x1` |
| `strb` at 0xa5764 | store | 1 | `sp -0x5f` | `0x0` |
| `ldr` at 0xa5768 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa576c | store | 4 | `sp -0x5c` | `(ret(0x1af68) + [sp-164])` |
| `ldr` at 0xa5770 | load | 4 | `sp -0xa0` | `-` |
| `strh` at 0xa5780 | store | 2 | `sp -0x5e` | `(ret(0x1b22c)+1 & 0xffff)` |
| `strb` at 0xa5788 | store | 1 | `sp -0x58` | `0x0` |
| `strb` at 0xa5790 | store | 1 | `sp -0x57` | `0x0` |
| `ldr` at 0xa5794 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5798 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa579c | load | 4 | `abs 0x2b36b0` | `-` |
| `ldr` at 0xa57a0 | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xa57b0 | store | 4 | `sp -0x54` | `?` |
| `strh` at 0xa57b8 | store | 2 | `sp -0x56` | `0x0` |
| `strb` at 0xa57c0 | store | 1 | `sp -0x50` | `0x0` |
| `strb` at 0xa57c8 | store | 1 | `sp -0x4f` | `0x0` |
| `ldr` at 0xa57cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa57d0 | store | 4 | `sp -0x4c` | `[sp-28]` |
| `strh` at 0xa57d8 | store | 2 | `sp -0x4e` | `0x0` |
| `strb` at 0xa57e0 | store | 1 | `sp -0x48` | `0x0` |
| `strb` at 0xa57e8 | store | 1 | `sp -0x47` | `0x0` |
| `ldr` at 0xa57ec | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa57f0 | load | 4 | `abs 0x2b36b4` | `-` |
| `str` at 0xa57f4 | store | 4 | `sp -0x44` | `[0x2b36b4]` |
| `strh` at 0xa57fc | store | 2 | `sp -0x46` | `0x0` |
| `strb` at 0xa5804 | store | 1 | `sp -0x40` | `0x0` |
| `strb` at 0xa580c | store | 1 | `sp -0x3f` | `0x0` |
| `ldr` at 0xa5810 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5814 | load | 4 | `abs 0x2b36a8` | `-` |
| `str` at 0xa5818 | store | 4 | `sp -0x3c` | `[0x2b36a8]` |
| `strh` at 0xa5820 | store | 2 | `sp -0x3e` | `0x0` |
| `strb` at 0xa5828 | store | 1 | `sp -0x38` | `0x0` |
| `strb` at 0xa5830 | store | 1 | `sp -0x37` | `0x0` |
| `ldr` at 0xa5834 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5838 | load | 4 | `abs 0x2b36ac` | `-` |
| `str` at 0xa583c | store | 4 | `sp -0x34` | `[0x2b36ac]` |
| `strh` at 0xa5844 | store | 2 | `sp -0x36` | `0x0` |
| `str` at 0xa584c | store | 4 | `sp -0x8c` | `0xb` |
| `ldr` at 0xa5850 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5854 | load | 4 | `abs 0x2b36c4` | `-` |
| `ldr` at 0xa5860 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5864 | load | 4 | `abs 0x2b36c0` | `-` |
| `strb` at 0xa5874 | store | 1 | `sp -0x30` | `0x0` |
| `strb` at 0xa587c | store | 1 | `sp -0x2f` | `0x0` |
| `ldr` at 0xa5880 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa5884 | load | 4 | `abs 0x2b36c4` | `-` |
| `str` at 0xa5888 | store | 4 | `sp -0x2c` | `[0x2b36c4]` |
| `strh` at 0xa5890 | store | 2 | `sp -0x2e` | `0x0` |
| `strb` at 0xa5898 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0xa58a0 | store | 1 | `sp -0x27` | `0x0` |
| `ldr` at 0xa58a4 | load | 4 | `abs 0xa5b58` | `-` |
| `ldr` at 0xa58a8 | load | 4 | `abs 0x2b36c0` | `-` |
| `str` at 0xa58ac | store | 4 | `sp -0x24` | `[0x2b36c0]` |
| `strh` at 0xa58b4 | store | 2 | `sp -0x26` | `0x0` |
| `str` at 0xa58bc | store | 4 | `sp -0x8c` | `0xd` |
| `str` at 0xa58c4 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa58d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa58d4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa58e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa58e4 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0xa58f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa58f8 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0xa58fc | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0xa5904 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5910 | store | 4 | `sp -0xb4` | `ret(0xb7944)` |
| `ldr` at 0xa5914 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0xa5918 | store | 4 | `sp -0xa4` | `[sp-180]` |
| `ldr` at 0xa591c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5920 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5934 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0xa5938 | store | 4 | `sp -0xb8` | `[[sp-20]+48]` |
| `ldr` at 0xa5940 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa594c | store | 4 | `sp -0xb8` | `ret(0xb7944)` |
| `ldr` at 0xa5950 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0xa5954 | store | 4 | `sp -0xa0` | `[sp-184]` |
| `ldr` at 0xa595c | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xa5960 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5970 | store | 4 | `sp -0x90` | `ret(0xcb45c)` |
| `strb` at 0xa5978 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa5980 | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa5984 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa5988 | store | 4 | `sp -0x84` | `ret(0xcb45c)` |
| `strh` at 0xa5990 | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa5994 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa59a4 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa59ac | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa59b0 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xa59b4 | store | 4 | `sp -0x7c` | `[sp-168]` |
| `strh` at 0xa59bc | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa59c0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa59d0 | store | 4 | `sp -0xbc` | `0x2` |
| `str` at 0xa59dc | store | 4 | `sp -0xbc` | `0x1` |
| `str` at 0xa59e4 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa59f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa59f4 | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0xa5a00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5a04 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0xa5a14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a18 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0xa5a1c | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0xa5a24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5a30 | store | 4 | `sp -0xc0` | `ret(0xb7944)` |
| `ldr` at 0xa5a34 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0xa5a38 | store | 4 | `sp -0xa4` | `[sp-192]` |
| `ldr` at 0xa5a3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa5a40 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0xa5a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5a54 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0xa5a58 | store | 4 | `sp -0xc4` | `[[sp-20]+48]` |
| `ldr` at 0xa5a60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa5a6c | store | 4 | `sp -0xc4` | `ret(0xb7944)` |
| `ldr` at 0xa5a70 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0xa5a74 | store | 4 | `sp -0xb0` | `[sp-196]` |
| `ldr` at 0xa5a78 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa5a7c | load | 2 | `[sp-20] +0x2e` | `-` |
| `str` at 0xa5a80 | store | 4 | `sp -0xa0` | `[[sp-20]+46]:2` |
| `ldr` at 0xa5a88 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xa5a8c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0xa5a90 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0xa5a9c | store | 4 | `sp -0x90` | `ret(0xcb48c)` |
| `strb` at 0xa5aa4 | store | 1 | `sp -0x88` | `0x0` |
| `strb` at 0xa5aac | store | 1 | `sp -0x87` | `0x0` |
| `ldr` at 0xa5ab0 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa5ab4 | store | 4 | `sp -0x84` | `ret(0xcb48c)` |
| `strh` at 0xa5abc | store | 2 | `sp -0x86` | `0x0` |
| `ldr` at 0xa5ac0 | load | 4 | `sp -0x90` | `-` |
| `strb` at 0xa5ad0 | store | 1 | `sp -0x80` | `0x0` |
| `strb` at 0xa5ad8 | store | 1 | `sp -0x7f` | `0x0` |
| `ldr` at 0xa5adc | load | 4 | `sp -0xac` | `-` |
| `str` at 0xa5ae0 | store | 4 | `sp -0x7c` | `[sp-172]` |
| `strh` at 0xa5ae8 | store | 2 | `sp -0x7e` | `0x0` |
| `ldr` at 0xa5aec | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa5afc | store | 4 | `sp -0xc8` | `0x2` |
| `str` at 0xa5b08 | store | 4 | `sp -0xc8` | `0x1` |
| `str` at 0xa5b10 | store | 4 | `sp -0xd8` | `sp-136` |
| `ldr` at 0xa5b1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa5b20 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0xa5b2c | load | 4 | `abs 0xa5b5c` | `-` |
| `str` at 0xa5b30 | store | 4 | `sp -0xd8` | `0x1262a8` |
| `ldr` at 0xa5b34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa5b38 | store | 4 | `sp -0xd4` | `[sp-32]` |
| `ldr` at 0xa5b40 | load | 4 | `abs 0xa5b60` | `-` |
| `ldr` at 0xa5b44 | load | 4 | `abs 0xa5b64` | `-` |
| `ldmdb` at 0xa5b50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa5b50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa5b50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa52f0 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `0x1` |
| `bl` at 0xa5320 | `(unknown callee) 0xa52ac` | `ret(0x4f090)` | `?` | `?` | `[0x14997c]` |
| `bl` at 0xa53b4 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa5420 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa54c8 | `(unknown callee) 0xcb21c` | `[0x2b36a8]` | `?` | `[sp-24]` | `0x2b36a0` |
| `bl` at 0xa5500 | `(unknown callee) 0xa4bb8` | `ret(0xcb21c)` | `[0x2b36a8]` | `[0x2b36b8]` | `0x0` |
| `bl` at 0xa5564 | `(unknown callee) 0xcb21c` | `[0x2b36a8]` | `?` | `?` | `0x2b36a0` |
| `bl` at 0xa5584 | `(unknown callee) 0x1af68` | `[[sp-152]+20]` | `?` | `ret(0xcb21c)` | `[sp-152]` |
| `bl` at 0xa55b8 | `(unknown callee) 0x1b22c` | `(ret(0x1af68) + [sp-160])` | `?` | `ret(0x1af68)` | `(ret(0x1af68) + [sp-160])` |
| `bl` at 0xa55d0 | `(unknown callee) 0x1af68` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0xa5604 | `(unknown callee) 0x1b22c` | `(ret(0x1af68) + [sp-160])` | `?` | `ret(0x1af68)` | `(ret(0x1af68) + [sp-160])` |
| `bl` at 0xa562c | `(unknown callee) 0xcb21c` | `[0x2b36ac]` | `?` | `?` | `0x2b36a0` |
| `bl` at 0xa5664 | `(unknown callee) 0xa4bb8` | `ret(0xcb21c)` | `[0x2b36ac]` | `[0x2b36b8]` | `0x0` |
| `bl` at 0xa56d4 | `(unknown callee) 0xcb21c` | `[0x2b36ac]` | `?` | `?` | `0x2b36a0` |
| `bl` at 0xa56f4 | `(unknown callee) 0x1af68` | `[[sp-152]+20]` | `?` | `ret(0xcb21c)` | `[sp-152]` |
| `bl` at 0xa5728 | `(unknown callee) 0x1b22c` | `(ret(0x1af68) + [sp-164])` | `?` | `ret(0x1af68)` | `(ret(0x1af68) + [sp-164])` |
| `bl` at 0xa5740 | `(unknown callee) 0x1af68` | `[[sp-152]+16]` | `?` | `?` | `[sp-152]` |
| `bl` at 0xa5774 | `(unknown callee) 0x1b22c` | `(ret(0x1af68) + [sp-164])` | `?` | `ret(0x1af68)` | `(ret(0x1af68) + [sp-164])` |
| `bl` at 0xa58d8 | `(unknown callee) 0xb6bdc` | `0x0` | `0x0` | `[sp-20]` | `?` |
| `bl` at 0xa590c | `(unknown callee) 0xb7944` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0xa5948 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa5968 | `(unknown callee) 0xcb45c` | `[sp-164]` | `[sp-184]` | `sp-168` | `sp-168` |
| `bl` at 0xa59f8 | `(unknown callee) 0xb6bdc` | `0x0` | `0x0` | `[sp-20]` | `[sp-188]` |
| `bl` at 0xa5a2c | `(unknown callee) 0xb7944` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0xa5a68 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0xa5a94 | `(unknown callee) 0xcb48c` | `[sp-164]` | `[sp-196]` | `[[sp-20]+46]:2` | `sp-172` |
| `bl` at 0xa5b24 | `(unknown callee) 0xb6bdc` | `0x0` | `0x0` | `[sp-20]` | `[sp-200]` |
| `bl` at 0xa5b4c | `(unknown callee) 0x21e28` | `0x4` | `0x126214` | `0xcdc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa531c | `[0x14997c] != 0x0` | `[0x14997c] == 0x0` |
| `beq` at 0xa5334 | `[sp-32] == 0x73` | `[sp-32] != 0x73` |
| `bgt` at 0xa5340 | `[sp-32] > 0x73 (signed, 32-bit)` | `[sp-32] <= 0x73 (signed, 32-bit)` |
| `beq` at 0xa534c | `[sp-32] == 0x72` | `[sp-32] != 0x72` |
| `beq` at 0xa535c | `[sp-32] == 0x74` | `[sp-32] != 0x74` |
| `bne` at 0xa5370 | `[0x2b36b4] != 0x2` | `[0x2b36b4] == 0x2` |
| `bls` at 0xa5380 | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xa5394 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xa53a4 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0xa53c0 | `ret(0xb7944) <= 0x3 (unsigned, 32-bit)` | `ret(0xb7944) > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xa53dc | `[0x2b36b4] != 0x3` | `[0x2b36b4] == 0x3` |
| `bls` at 0xa53ec | `[[sp-20]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-20]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xa5400 | `? != 0x0` | `? == 0x0` |
| `bls` at 0xa5410 | `[[sp-20]+48] <= 0x3 (unsigned, 32-bit)` | `[[sp-20]+48] > 0x3 (unsigned, 32-bit)` |
| `bls` at 0xa542c | `ret(0xb7944) <= 0x3 (unsigned, 32-bit)` | `ret(0xb7944) > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xa5470 | `[0x2b36c8] == 0x0` | `[0x2b36c8] != 0x0` |
| `bge` at 0xa5514 | `ret(0xa4bb8) >= 0x0 (signed, 32-bit)` | `ret(0xa4bb8) < 0x0 (signed, 32-bit)` |
| `beq` at 0xa5620 | `[0x2b36ac] == 0x0` | `[0x2b36ac] != 0x0` |
| `bge` at 0xa5684 | `[sp-144] >= 0x0 (signed, 32-bit)` | `[sp-144] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa585c | `[0x2b36c4] == 0x0` | `[0x2b36c4] != 0x0` |
| `beq` at 0xa586c | `[0x2b36c0] == 0x0` | `[0x2b36c0] != 0x0` |
| `bne` at 0xa58f0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa592c | `? != 0x0` | `? == 0x0` |
| `bge` at 0xa599c | `ret(0xcb45c) >= 0x0 (signed, 32-bit)` | `ret(0xcb45c) < 0x0 (signed, 32-bit)` |
| `bge` at 0xa59c8 | `ret(0xcb45c) >= 0x0 (signed, 32-bit)` | `ret(0xcb45c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xa5a10 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa5a4c | `? != 0x0` | `? == 0x0` |
| `bge` at 0xa5ac8 | `ret(0xcb48c) >= 0x0 (signed, 32-bit)` | `ret(0xcb48c) < 0x0 (signed, 32-bit)` |
| `bge` at 0xa5af4 | `ret(0xcb48c) >= 0x0 (signed, 32-bit)` | `ret(0xcb48c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa5b50 | `r0` | `?` |

## `PBaySdrGetCardSerial`

- `PBaySdrGetCardSerial`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaca9c to 0xacf68.
  Established. The opening byte guard takes the work arm for `[0x142950]:1 != 0xff` and falls
  through on equality (`bne` at 0xacac8). The call to `0xad614` has `[sp-20]`, `sp-696`, `[sp-288]`,
  `0x0` (`bl` at 0xacba4). The later call to `0xad730` carries `[sp-20]`, `sp-696`, `[sp-288]`,
  `([sp-706]:2+3 & 0xffff)` (`bl` at 0xacd94). Near the end, byte stores write `0xa` and `0x0` at
  unknown targets (`strb` at 0xacecc, `strb` at 0xacef4). Serial-record encoding and the merged r0
  value remain unproven (`ldmdb` at 0xacf3c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacaac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xacab0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xacab4 | load | 4 | `abs 0xacf40` | `-` |
| `str` at 0xacab8 | store | 4 | `sp -0x120` | `0x142950` |
| `ldr` at 0xacabc | load | 4 | `sp -0x120` | `-` |
| `ldrb` at 0xacac0 | load | 1 | `abs 0x142950` | `-` |
| `ldr` at 0xacacc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacad4 | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0xacadc | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xacae4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xacafc | store | 4 | `sp -0x2cc` | `ret(0x1b124)` |
| `ldr` at 0xacb00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xacb04 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xacb08 | store | 4 | `[sp-24] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xacb0c | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xacb1c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xacb24 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xacb28 | load | 4 | `abs 0xacf44` | `-` |
| `ldr` at 0xacb30 | load | 4 | `abs 0xacf48` | `-` |
| `ldr` at 0xacb34 | load | 4 | `abs 0x14299c` | `-` |
| `str` at 0xacb38 | store | 4 | `sp -0x2dc` | `[0x14299c]` |
| `ldr` at 0xacb40 | load | 4 | `abs 0xacf44` | `-` |
| `str` at 0xacb44 | store | 4 | `sp -0x2cc` | `0x142970` |
| `ldr` at 0xacb48 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xacb50 | store | 1 | `abs 0x142970` | `0x0` |
| `ldr` at 0xacb54 | load | 4 | `abs 0xacf4c` | `-` |
| `strh` at 0xacb60 | store | 2 | `sp -0x2c2` | `0x0` |
| `ldr` at 0xacb64 | load | 4 | `abs 0xacf50` | `-` |
| `strb` at 0xacb70 | store | 1 | `sp -0x2c3` | `0x8` |
| `ldr` at 0xacb78 | load | 4 | `abs 0xacf4c` | `-` |
| `ldrh` at 0xacb80 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xacb84 | load | 4 | `abs 0xacf50` | `-` |
| `ldrb` at 0xacb8c | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xacb90 | store | 4 | `sp -0x2e4` | `0x8` |
| `ldr` at 0xacb94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacb9c | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xacbac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacbbc | store | 4 | `sp -0x2bc` | `ret(0xaa7dc)` |
| `ldr` at 0xacbc0 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xacbcc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xacbd0 | load | 4 | `abs 0xacf54` | `-` |
| `ldr` at 0xacbd8 | load | 4 | `abs 0xacf48` | `-` |
| `str` at 0xacbe0 | store | 4 | `abs 0x14299c` | `?` |
| `str` at 0xacbe8 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xacbf0 | load | 4 | `sp -0x130` | `-` |
| `str` at 0xacbf8 | store | 4 | `sp -0x2d0` | `ret(0xae3dc)` |
| `ldr` at 0xacc00 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xacc18 | load | 4 | `abs 0xacf58` | `-` |
| `ldrb` at 0xacc24 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xacc30 | load | 4 | `abs 0xacf58` | `-` |
| `ldrb` at 0xacc3c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xacc48 | load | 4 | `abs 0xacf4c` | `-` |
| `ldr` at 0xacc4c | load | 4 | `abs 0xacf58` | `-` |
| `ldrb` at 0xacc58 | load | 1 | `sp -0x2d4` | `-` |
| `strh` at 0xacc64 | store | 2 | `sp -0x2c2` | `?` |
| `ldr` at 0xacc68 | load | 4 | `abs 0xacf50` | `-` |
| `strb` at 0xacc74 | store | 1 | `sp -0x2c3` | `0x3` |
| `ldr` at 0xacc7c | load | 4 | `abs 0xacf4c` | `-` |
| `ldrh` at 0xacc84 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xacc88 | load | 4 | `abs 0xacf50` | `-` |
| `ldrb` at 0xacc90 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xacc94 | store | 4 | `sp -0x2e4` | `0x3` |
| `str` at 0xacc9c | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xacca0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacca8 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xaccb8 | load | 4 | `abs 0xacf5c` | `-` |
| `strb` at 0xaccc0 | store | 1 | `sp -0x2c4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xaccc4 | load | 4 | `abs 0xacf5c` | `-` |
| `ldrb` at 0xacccc | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xaccd8 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xaccdc | load | 4 | `abs 0xacf54` | `-` |
| `ldr` at 0xacce4 | load | 4 | `abs 0xacf48` | `-` |
| `str` at 0xaccec | store | 4 | `abs 0x14299c` | `?` |
| `str` at 0xaccf4 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xaccfc | load | 4 | `abs 0xacf50` | `-` |
| `ldr` at 0xacd00 | load | 4 | `abs 0xacf60` | `-` |
| `ldrb` at 0xacd08 | load | 1 | `sp -0x117` | `-` |
| `strb` at 0xacd14 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xacd18 | load | 4 | `abs 0xacf50` | `-` |
| `ldr` at 0xacd1c | load | 4 | `abs 0xacf50` | `-` |
| `ldr` at 0xacd20 | load | 4 | `abs 0xacf5c` | `-` |
| `ldrb` at 0xacd28 | load | 1 | `sp -0x2c3` | `-` |
| `ldrb` at 0xacd30 | load | 1 | `sp -0x2c4` | `-` |
| `strb` at 0xacd3c | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xacd40 | load | 4 | `abs 0xacf4c` | `-` |
| `ldr` at 0xacd44 | load | 4 | `abs 0xacf4c` | `-` |
| `ldrh` at 0xacd4c | load | 2 | `sp -0x2c2` | `-` |
| `strh` at 0xacd58 | store | 2 | `sp -0x2c2` | `([sp-706]:2+3 & 0xffff)` |
| `ldr` at 0xacd60 | load | 4 | `abs 0xacf4c` | `-` |
| `ldrh` at 0xacd68 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xacd6c | load | 4 | `abs 0xacf50` | `-` |
| `ldrb` at 0xacd74 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xacd78 | store | 4 | `sp -0x2e4` | `[sp-707]:1` |
| `str` at 0xacd80 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xacd84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xacd8c | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xacd9c | load | 4 | `abs 0xacf5c` | `-` |
| `strb` at 0xacda4 | store | 1 | `sp -0x2c4` | `(ret(0xad730) & 0xff)` |
| `ldr` at 0xacda8 | load | 4 | `abs 0xacf5c` | `-` |
| `ldrb` at 0xacdb0 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xacdbc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xacdc0 | load | 4 | `abs 0xacf54` | `-` |
| `ldr` at 0xacdc8 | load | 4 | `abs 0xacf48` | `-` |
| `str` at 0xacdd0 | store | 4 | `abs 0x14299c` | `?` |
| `str` at 0xacdd8 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0xacde4 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xacdec | store | 4 | `sp -0x2c0` | `0x0` |
| `ldr` at 0xacdf0 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xace00 | load | 4 | `abs 0xacf64` | `-` |
| `ldr` at 0xace04 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xace14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xace28 | load | 4 | `abs 0xacf64` | `-` |
| `ldr` at 0xace2c | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xace3c | load | 1 | `(sp-16 + [sp-284]) -0x108` | `-` |
| `strb` at 0xace48 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xace4c | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xace54 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldr` at 0xace58 | load | 4 | `sp -0x2c0` | `-` |
| `ldrb` at 0xace6c | load | 1 | `(unknown)` | `-` |
| `strb` at 0xace84 | store | 1 | `(unknown)` | `0x28` |
| `ldr` at 0xace8c | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xace9c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacea0 | load | 4 | `sp -0x2cc` | `-` |
| `ldrb` at 0xaceb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacec0 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xacecc | store | 1 | `(unknown)` | `0xa` |
| `ldrb` at 0xaced8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacee4 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xacef4 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0xacf04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xacf08 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xacf10 | store | 4 | `sp -0x11c` | `?` |
| `ldr` at 0xacf14 | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xacf1c | store | 4 | `sp -0x2c0` | `[sp-704]+1` |
| `ldr` at 0xacf24 | load | 4 | `abs 0xacf48` | `-` |
| `str` at 0xacf2c | store | 4 | `abs 0x14299c` | `0x0` |
| `str` at 0xacf34 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xacf38 | load | 4 | `sp -0x2dc` | `-` |
| `ldmdb` at 0xacf3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xacf3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xacf3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacaf4 | `(unknown callee) 0x1b124` | `0x2a` | `r1` | `r2` | `r1` |
| `bl` at 0xacb2c | `(unknown callee) 0x1b508` | `[sp-716]` | `0x142970` | `[sp-24]` | `[sp-716]` |
| `bl` at 0xacba4 | `(unknown callee) 0xad614` | `[sp-20]` | `sp-696` | `[sp-288]` | `0x0` |
| `bl` at 0xacbb4 | `(unknown callee) 0xaa7dc` | `[sp-20]` | `sp-696` | `?` | `sp-696` |
| `bl` at 0xacbd4 | `(unknown callee) 0x1b2d4` | `[sp-716]` | `0x127164` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xacbf4 | `(unknown callee) 0xae3dc` | `[sp-304]` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xacc14 | `(unknown callee) 0x1b304` | `sp-728` | `ret(0xae3dc)+1` | `0x8` | `ret(0xae3dc)+1` |
| `bl` at 0xaccb0 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-696` | `[sp-288]` | `[sp-706]:2` |
| `bl` at 0xacce0 | `(unknown callee) 0x1b2d4` | `[sp-716]` | `0x127164` | `ret(0xad730)` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xacd94 | `(unknown callee) 0xad730` | `[sp-20]` | `sp-696` | `[sp-288]` | `([sp-706]:2+3 & 0xffff)` |
| `bl` at 0xacdc4 | `(unknown callee) 0x1b2d4` | `[sp-716]` | `0x127164` | `sp-16` | `(ret(0xad730) & 0xff)` |
| `bl` at 0xaceac | `(unknown callee) 0x1b304` | `[sp-716]` | `(sp-280 + [sp-284])` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xacac8 | `[0x142950]:1 != 0xff` | `[0x142950]:1 == 0xff` |
| `beq` at 0xacaec | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xacb14 | `[sp-716] != 0x0` | `[sp-716] == 0x0` |
| `bne` at 0xacbc8 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `beq` at 0xacc2c | `[sp-724]:1 == 0x0` | `[sp-724]:1 != 0x0` |
| `beq` at 0xacc44 | `[sp-724]:1 == 0xff` | `[sp-724]:1 != 0xff` |
| `bne` at 0xaccd4 | `(ret(0xad730) & 0xff) != 0x0` | `(ret(0xad730) & 0xff) == 0x0` |
| `bne` at 0xacdb8 | `(ret(0xad730) & 0xff) != 0x0` | `(ret(0xad730) & 0xff) == 0x0` |
| `ble` at 0xacdf8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xace1c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xace60 | `[sp-704] != 0x4` | `[sp-704] == 0x4` |
| `bls` at 0xace74 | `? <= 0x28 (unsigned, 32-bit)` | `? > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xacf3c | `r0` | `?` |

## `sn1sc_select_port`

- `sn1sc_select_port`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe32fc to 0xe335c.
  Established. The halfword store writes an unknown value at `[0x2b3840] +0xe` (`strh` at 0xe3348).
  Earlier byte stores save `(r0 & 0xff)` at `sp -0x11` and `(r1 & 0xff)` at `sp -0x12` (`strb` at
  0xe3314, `strb` at 0xe331c). The r0 return is `0x1` (`ldmdb` at 0xe3354). No calls or conditional
  branches are reported. Port encoding and hardware mux behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xe3314 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xe331c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xe3324 | load | 4 | `abs 0xe3358` | `-` |
| `ldr` at 0xe3328 | load | 4 | `abs 0x2b3840` | `-` |
| `ldrb` at 0xe3330 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0xe3338 | load | 1 | `sp -0x11` | `-` |
| `strh` at 0xe3348 | store | 2 | `[0x2b3840] +0xe` | `?` |
| `ldmdb` at 0xe3354 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe3354 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe3354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe3354 | `r0` | `0x1` |

## `sn1sc_dr_load`

- `sn1sc_dr_load`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5064 to 0xe5330.
  Established. The first call to `0xdcf54` has `0x0`, unknown, `0x1`, unknown (`bl` at 0xe5100); its
  nonzero result takes the continuation and zero falls through (`bne` at 0xe5114). The later call to
  the same target uses `0x1`, `0x6`, `0x1`, unknown (`bl` at 0xe52ec). After the pointer comparison,
  the inequality arm calls `0x1b400` with `[sp-44]`, unknown, `[sp-44]`, `[sp-40]` (`bne` at
  0xe5304, `bl` at 0xe5318). Data-register loading, object ownership and the joined r0 value remain
  unknown (`ldmdb` at 0xe5328).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5074 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe507c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5084 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xe5088 | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xe508c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe50a0 | store | 4 | `sp -0x28` | `r0+24` |
| `ldr` at 0xe50a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe50b0 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe50c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe50c8 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe50d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe50d8 | store | 4 | `sp -0x44` | `[r0+16]` |
| `ldr` at 0xe50dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe50e0 | store | 4 | `sp -0x40` | `r0+24` |
| `ldr` at 0xe50e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe50e8 | store | 4 | `sp -0x3c` | `r0+24` |
| `ldr` at 0xe50ec | load | 4 | `abs 0xe532c` | `-` |
| `str` at 0xe50f0 | store | 4 | `sp -0x38` | `0x2b37a8` |
| `str` at 0xe5108 | store | 4 | `sp -0x18` | `ret(0xdcf54)` |
| `ldr` at 0xe510c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe511c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe5128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5130 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5134 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe5138 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5148 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe5150 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5154 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe516c | store | 4 | `sp -0x1c` | `0x6` |
| `str` at 0xe5178 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xe517c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe518c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe5198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe51a0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe51a4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe51ac | store | 4 | `sp -0x34` | `[sp-52]+24` |
| `ldr` at 0xe51b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe51c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe51c8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe51cc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe51d4 | store | 4 | `sp -0x34` | `[sp-52]+28` |
| `ldr` at 0xe51d8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xe51dc | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0xe51e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe51ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe51f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe51fc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xe5200 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5204 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0xe5208 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe520c | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xe5210 | load | 4 | `abs 0xe532c` | `-` |
| `str` at 0xe5214 | store | 4 | `sp -0x38` | `0x2b37a8` |
| `ldr` at 0xe5220 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5230 | store | 4 | `sp -0x18` | `ret(0xdcf54)` |
| `ldr` at 0xe5234 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5238 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xe5250 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xe5258 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe5270 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe528c | store | 4 | `sp -0x2c` | `ret(0x1b124)` |
| `ldr` at 0xe5290 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe529c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe52a0 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xe52a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe52b0 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xe52bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe52c0 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xe52c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe52c8 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0xe52cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe52d0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xe52d4 | load | 4 | `abs 0xe532c` | `-` |
| `str` at 0xe52d8 | store | 4 | `sp -0x38` | `0x2b37a8` |
| `str` at 0xe52f4 | store | 4 | `sp -0x18` | `ret(0xdcf54)` |
| `ldr` at 0xe52f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe52fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe530c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe5314 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe531c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5320 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0xe5324 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xe5328 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5328 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5100 | `(unknown callee) 0xdcf54` | `0x0` | `?` | `0x1` | `?` |
| `bl` at 0xe5228 | `(unknown callee) 0xdcf54` | `0x1` | `0x1` | `[sp-28]` | `?` |
| `bl` at 0xe5284 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xe52ec | `(unknown callee) 0xdcf54` | `0x1` | `0x6` | `0x1` | `?` |
| `bl` at 0xe5318 | `(unknown callee) 0x1b400` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5094 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bne` at 0xe5114 | `ret(0xdcf54) != 0x0` | `ret(0xdcf54) == 0x0` |
| `bne` at 0xe5140 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe5164 | `? != 0x1` | `? == 0x1` |
| `bne` at 0xe5188 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe5248 | `? == 0x1` | `? != 0x1` |
| `bne` at 0xe5298 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xe5304 | `[sp-44] != [sp-40]` | `[sp-44] == [sp-40]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5328 | `r0` | `?` |

## `sn1sc_reset_brick`

- `sn1sc_reset_brick`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5330 to 0xe5418.
  Established. The absolute-word test takes its continuation for `[0x2ad258] != 0x0` and falls
  through on equality (`bne` at 0xe5378). A later indirect call has unknown target, known
  continuation, and candidates unknown, `0x1`, `0x2b37a8`, `[0x2ae5ec]` (`mov` at 0xe53c0). A
  following word store writes `ret(unknown)` at `[sp-24] +0x0` (`str` at 0xe53cc). Physical reset
  effects and the merged r0 return are unproved (`ldmdb` at 0xe53fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5340 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5344 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe534c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xe5350 | load | 4 | `abs 0xe5400` | `-` |
| `ldr` at 0xe5354 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe5360 | load | 4 | `abs 0xe5404` | `-` |
| `ldr` at 0xe5364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe536c | load | 4 | `abs 0xe5408` | `-` |
| `ldr` at 0xe5370 | load | 4 | `abs 0x2ad258` | `-` |
| `ldr` at 0xe537c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5380 | load | 4 | `abs 0xe540c` | `-` |
| `str` at 0xe5384 | store | 4 | `(unknown)` | `0xfffff7cc` |
| `str` at 0xe538c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xe5398 | load | 4 | `abs 0xe5410` | `-` |
| `ldr` at 0xe539c | load | 4 | `abs 0x2ae5ec` | `-` |
| `ldr` at 0xe53a8 | load | 4 | `abs 0xe5410` | `-` |
| `ldr` at 0xe53ac | load | 4 | `abs 0x2ae5ec` | `-` |
| `ldr` at 0xe53b8 | load | 4 | `abs 0xe5414` | `-` |
| `ldr` at 0xe53c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe53cc | store | 4 | `[sp-24] +0x0` | `ret(unknown)` |
| `ldr` at 0xe53d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe53d4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xe53e4 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xe53f4 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xe53f8 | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xe53fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe53fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe53fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5368 | `(unknown callee) 0x23f2c` | `0x1358d4` | `r0` | `r2` | `[0x2ad25c]` |
| `bl` at 0xe5394 | `(unknown callee) 0xe5418` | `?` | `?` | `?` | `[0x2ad258]` |
| `mov` at 0xe53c0 | `(unknown callee) (unknown target)` | `?` | `0x1` | `0x2b37a8` | `[0x2ae5ec]` |
| `bl` at 0xe53ec | `(unknown callee) 0xe5418` | `ret(0xe5418)` | `?` | `?` | `[0x2ae5ec]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe535c | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bne` at 0xe5378 | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |
| `beq` at 0xe53a4 | `[0x2ae5ec] == 0x0` | `[0x2ae5ec] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe53fc | `r0` | `?` |

## `sn1sc_reset_sic`

- `sn1sc_reset_sic`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5418 to 0xe58a0.
  Established. Six halfword stores write unknown values at `[0x2b37a0] +0x0` (`strh` at 0xe55e4,
  `strh` at 0xe566c, `strh` at 0xe56f0, `strh` at 0xe5774, `strh` at 0xe57f8, `strh` at 0xe587c).
  The prior absolute-word branch takes its work arm for `[0x2ad258] != 0x0` and falls through on
  equality (`bne` at 0xe544c). Near the end, a word store writes `0x1` at `sp -0x18` (`str` at
  0xe5884). Reset signaling, halfword encoding and the joined r0 value remain unknown (`ldmdb` at
  0xe588c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe5428 | load | 4 | `abs 0xe5890` | `-` |
| `ldr` at 0xe542c | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0xe5438 | load | 4 | `abs 0xe5894` | `-` |
| `ldr` at 0xe5440 | load | 4 | `abs 0xe5898` | `-` |
| `ldr` at 0xe5444 | load | 4 | `abs 0x2ad258` | `-` |
| `str` at 0xe5454 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe545c | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5460 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5464 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5480 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5484 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5488 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe54ac | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe54b0 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe54b4 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe54d8 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe54dc | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe54e0 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5504 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5508 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe550c | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5530 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5534 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5538 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `str` at 0xe555c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe5560 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5568 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe556c | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5570 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5574 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5580 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe55d0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe55e4 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xe55e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe55f0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe55f4 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe55f8 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe55fc | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe561c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5644 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5658 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe566c | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xe5674 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xe5678 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe567c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5680 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe568c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe56a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe56b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe56c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe56dc | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe56f0 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xe56f8 | store | 4 | `sp -0x14` | `0x28` |
| `ldr` at 0xe56fc | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5700 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5704 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5710 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5724 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe574c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5760 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe5774 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xe577c | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xe5780 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5784 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5788 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5794 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe57e4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe57f8 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xe5800 | store | 4 | `sp -0x14` | `0x30` |
| `ldr` at 0xe5804 | load | 4 | `abs 0xe589c` | `-` |
| `ldr` at 0xe5808 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe580c | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe582c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5840 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5868 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe587c | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `str` at 0xe5884 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xe5888 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xe588c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe588c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe588c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe543c | `(unknown callee) 0x23f2c` | `0x1358f4` | `r1` | `r2` | `[0x2ad25c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5434 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `bne` at 0xe544c | `[0x2ad258] != 0x0` | `[0x2ad258] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe588c | `r0` | `?` |

## `sn1sc_get_tap_val`

- `sn1sc_get_tap_val`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe58a0 to 0xe59c4.
  Established. Six halfword reads address `[0x2b37a0] +0x0` (`ldrh` at 0xe58b8, `ldrh` at 0xe58dc,
  `ldrh` at 0xe5908, `ldrh` at 0xe5934, `ldrh` at 0xe5960, `ldrh` at 0xe598c). A later word store
  writes an unknown value at `sp -0x14`, and the return supplies r0 `[sp-20]` (`str` at 0xe59b0,
  `ldmdb` at 0xe59bc). The facts contain no calls or conditional branches. TAP bit positions and
  physical signal meaning remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe58b0 | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe58b4 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe58b8 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe58d4 | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe58d8 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe58dc | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5900 | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe5904 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5908 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe592c | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe5930 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5934 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5958 | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe595c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5960 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5984 | load | 4 | `abs 0xe59c0` | `-` |
| `ldr` at 0xe5988 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe598c | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `str` at 0xe59b0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xe59b4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe59bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe59bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe59bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe59bc | `r0` | `[sp-20]` |

## `sn1sc_get_tap_text`

- `sn1sc_get_tap_text`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe59c4 to 0xe5bf4.
  Established. The only call targets `0x1b49c` with entry r0, `0x135908`, `[sp-28]`, `[sp-32]` (`bl`
  at 0xe5be4). Before it, separate word stores carry `[sp-36]`, `[sp-40]`, `[sp-44]` and `[sp-48]`
  at `sp -0x40`, `sp -0x3c`, `sp -0x38` and `sp -0x34`, respectively (`str` at 0xe5b4c, `str` at
  0xe5b78, `str` at 0xe5ba4, `str` at 0xe5bd0). The r0 return is `ret(0x1b49c)` (`ldmdb` at
  0xe5be8). Text-field ordering, signal interpretation and call arity are not established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe59d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe59d8 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe59dc | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe59e0 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe59fc | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a00 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5a04 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5a28 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a2c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5a30 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5a54 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a58 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5a5c | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5a80 | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5a84 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5a88 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5aac | load | 4 | `abs 0xe5bec` | `-` |
| `ldr` at 0xe5ab0 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5ab4 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `str` at 0xe5ad8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xe5adc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5af0 | store | 4 | `sp -0x1c` | `0x31` |
| `str` at 0xe5afc | store | 4 | `sp -0x1c` | `0x30` |
| `ldr` at 0xe5b00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5b14 | store | 4 | `sp -0x20` | `0x31` |
| `str` at 0xe5b20 | store | 4 | `sp -0x20` | `0x30` |
| `ldr` at 0xe5b24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5b38 | store | 4 | `sp -0x24` | `0x31` |
| `str` at 0xe5b44 | store | 4 | `sp -0x24` | `0x30` |
| `ldr` at 0xe5b48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe5b4c | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xe5b50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5b64 | store | 4 | `sp -0x28` | `0x31` |
| `str` at 0xe5b70 | store | 4 | `sp -0x28` | `0x30` |
| `ldr` at 0xe5b74 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe5b78 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0xe5b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5b90 | store | 4 | `sp -0x2c` | `0x31` |
| `str` at 0xe5b9c | store | 4 | `sp -0x2c` | `0x30` |
| `ldr` at 0xe5ba0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xe5ba4 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xe5ba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5bbc | store | 4 | `sp -0x30` | `0x31` |
| `str` at 0xe5bc8 | store | 4 | `sp -0x30` | `0x30` |
| `ldr` at 0xe5bcc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe5bd0 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xe5bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5bd8 | load | 4 | `abs 0xe5bf0` | `-` |
| `ldr` at 0xe5bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5be0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xe5be8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5be8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5be8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5be4 | `(unknown callee) 0x1b49c` | `r0` | `0x135908` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5ae8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5b0c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5b30 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5b5c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5b88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5bb4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5be8 | `r0` | `ret(0x1b49c)` |

## `sn1sc_set_tap_val`

- `sn1sc_set_tap_val`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5bf4 to 0xe5c8c.
  Established. A halfword store writes an unknown value at `[0x2b37a0] +0x0` (`strh` at 0xe5c80).
  The earlier word save places entry r0 at `sp -0x14`, and the return retains entry r0 (`str` at
  0xe5c04, `ldmdb` at 0xe5c84). There are no calls or conditional branches in the fact sections.
  Physical TAP output and bit encoding remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5c04 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe5c08 | load | 4 | `abs 0xe5c88` | `-` |
| `ldr` at 0xe5c0c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5c10 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xe5c1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5c44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5c58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5c6c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xe5c80 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldmdb` at 0xe5c84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5c84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5c84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5c84 | `r0` | `r0` |

## `sn1sc_drive_tap_fixed_tms`

- `sn1sc_drive_tap_fixed_tms`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5c8c to 0xe5ef0.
  Established. The initial branch takes the work arm for `r1 != 0x0` and falls through on equality
  (`bne` at 0xe5cb4). A later halfword store writes an unknown value at `[0x2b37a0] +0x0` (`strh` at
  0xe5e84). The two call sites target `0xf7328` and `0xf74c8`, respectively, each with `sp-52`,
  unknown, unknown, `sp-52` (`bl` at 0xe5ec8, `bl` at 0xe5ed8). The r0 return is `0x1` (`ldmdb` at
  0xe5ee8). TMS level, clock count and physical scan behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5c9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5ca0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5ca4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe5ca8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe5cac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5cbc | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe5cc4 | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5cc8 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5ccc | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5ce8 | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5cec | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5cf0 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5d14 | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5d18 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5d1c | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5d40 | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5d44 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5d48 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5d6c | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5d70 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5d74 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5d98 | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5d9c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5da0 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `str` at 0xe5dc4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe5dc8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe5dd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5df0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe5df8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe5e00 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe5e08 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe5e0c | load | 4 | `abs 0xe5eec` | `-` |
| `ldr` at 0xe5e10 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5e14 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xe5e20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe5e34 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe5e48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe5e5c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe5e70 | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xe5e84 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xe5e88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe5e8c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe5e90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe5e94 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe5e98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5ea4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5ea8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe5eac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe5eb0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe5eb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe5ee0 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xe5ee4 | load | 4 | `sp -0x3c` | `-` |
| `ldmdb` at 0xe5ee8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe5ee8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe5ee8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe5ec8 | `(unknown callee) 0xf7328` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xe5ed8 | `(unknown callee) 0xf74c8` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe5cb4 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0xe5ddc | `? == ?` | `? != ?` |
| `beq` at 0xe5dec | `? == 0x0` | `? != 0x0` |
| `beq` at 0xe5ea0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xe5ebc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe5ee8 | `r0` | `0x1` |

## `sn1sc_drive_tap`

- `sn1sc_drive_tap`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5ef0 to 0xe6118.
  Established. The comparison takes the bypass when two unknown values are equal and falls through
  when they differ (`beq` at 0xe6028). Fall-through includes a halfword store of an unknown value at
  `[0x2b37a0] +0x0` (`strh` at 0xe60d0). The later call to `0xf7548` uses `sp-52`, unknown, unknown,
  `sp-52`, and r0 is `0x1` at the return (`bl` at 0xe6104, `ldmdb` at 0xe6110). Scan input/output
  ordering and hardware signaling remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5f00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xe5f04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xe5f08 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xe5f0c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xe5f10 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5f14 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5f18 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5f34 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5f38 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5f3c | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5f60 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5f64 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5f68 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5f8c | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5f90 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5f94 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5fb8 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5fbc | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5fc0 | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `ldr` at 0xe5fe4 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe5fe8 | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe5fec | load | 2 | `[0x2b37a0] +0x0` | `-` |
| `str` at 0xe6010 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe6014 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe601c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe602c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe603c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe6044 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe604c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xe6054 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xe6058 | load | 4 | `abs 0xe6114` | `-` |
| `ldr` at 0xe605c | load | 4 | `abs 0x2b37a0` | `-` |
| `ldrh` at 0xe6060 | load | 2 | `sp -0x38` | `-` |
| `ldr` at 0xe606c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6080 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe6094 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe60a8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe60bc | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xe60d0 | store | 2 | `[0x2b37a0] +0x0` | `?` |
| `ldr` at 0xe60d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe60d8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xe60dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe60e0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe60e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe60e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe60ec | load | 4 | `sp +0x0` | `-` |
| `str` at 0xe60f0 | store | 4 | `sp -0x28` | `[sp]` |
| `ldr` at 0xe60f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe60f8 | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xe6110 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe6110 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe6110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe6104 | `(unknown callee) 0xf7548` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe6028 | `? == ?` | `? != ?` |
| `beq` at 0xe6038 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe6110 | `r0` | `0x1` |

## `lbi_tapmux_select`

- `lbi_tapmux_select`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf5790 to 0xf5830.
  Established. The two entry tests take bypasses for `r1 == 0x0` and `r2 == 0x0`, respectively, with
  nonzero falling through (`beq` at 0xf57cc, `beq` at 0xf57e4). The call to `0xd6a1c` has `[sp-32]`,
  unknown, `0x50`, unknown (`bl` at 0xf581c). An earlier byte store writes `0x10` at `sp -0x21` (`strb` at 0xf57d8).
  A later store there has unknown value (`strb` at 0xf57f0). The r0 return is
  `ret(0xd6a1c)` (`ldmdb` at 0xf5828). Mux identity and physical selection behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf57a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf57a4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf57a8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf57ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf57b8 | store | 4 | `sp -0x20` | `?` |
| `strb` at 0xf57c0 | store | 1 | `sp -0x21` | `0x0` |
| `ldr` at 0xf57c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf57d0 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xf57d8 | store | 1 | `sp -0x21` | `0x10` |
| `ldr` at 0xf57dc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xf57e8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xf57f0 | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xf57f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf57fc | load | 4 | `abs 0xf582c` | `-` |
| `ldr` at 0xf5800 | load | 4 | `abs 0x2ad44c` | `-` |
| `ldrb` at 0xf5808 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xf580c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf5810 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xf5828 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf5828 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf5828 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf581c | `(unknown callee) 0xd6a1c` | `[sp-32]` | `?` | `0x50` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf57cc | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xf57e4 | `r2 == 0x0` | `r2 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf5828 | `r0` | `ret(0xd6a1c)` |
