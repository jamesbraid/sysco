# ARM board-reset wrappers in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/board-reset-wrappers-facts.txt) retain all tool sections.

## `NL4R_ResetHold`

- `NL4R_ResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2797c to 0x27aac.
  Established. The byte store clears `0x147eb9` to `0x0` (`strb` at 0x279bc). The selector branch
  takes its arm for `[sp-20] != 0xff` and falls through on equality (`bne` at 0x279c8). Calls to
  `0x27f9c` have unknown, `0x147ebc`, unknown, unknown at both sites (`bl` at 0x27a04, `bl` at
  0x27a84). Reset assertion and the merged r0 result remain unknown (`ldmdb` at 0x27a94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2798c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27994 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27998 | load | 4 | `abs 0x27a98` | `-` |
| `ldrb` at 0x2799c | load | 1 | `abs 0x147eb8` | `-` |
| `ldr` at 0x279a8 | load | 4 | `abs 0x27a9c` | `-` |
| `str` at 0x279ac | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x279b4 | load | 4 | `abs 0x27aa0` | `-` |
| `strb` at 0x279bc | store | 1 | `abs 0x147eb9` | `0x0` |
| `ldr` at 0x279c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x279d0 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x279d4 | load | 4 | `abs 0x27aa4` | `-` |
| `ldr` at 0x279d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x279dc | load | 4 | `abs 0x147ec0` | `-` |
| `ldr` at 0x279ec | load | 4 | `abs 0x27aa8` | `-` |
| `ldr` at 0x279f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x279f8 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27a0c | store | 4 | `sp -0x1c` | `ret(0x27f9c)` |
| `ldr` at 0x27a10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27a1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27a28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27a2c | store | 4 | `sp -0x18` | `ret(0x27f9c)` |
| `ldr` at 0x27a30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27a38 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27a40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27a44 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x27a4c | load | 4 | `abs 0x27aa4` | `-` |
| `ldr` at 0x27a50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27a54 | load | 4 | `abs 0x147ec0` | `-` |
| `str` at 0x27a64 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27a6c | load | 4 | `abs 0x27aa8` | `-` |
| `ldr` at 0x27a70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27a78 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27a8c | store | 4 | `sp -0x20` | `ret(0x27f9c)` |
| `ldr` at 0x27a90 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x27a94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27a94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27a94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27a04 | `(unknown callee) 0x27f9c` | `?` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27a84 | `(unknown callee) 0x27f9c` | `?` | `0x147ebc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x279a4 | `[0x147eb8]:1 != 0x0` | `[0x147eb8]:1 == 0x0` |
| `bne` at 0x279c8 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x279e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27a18 | `ret(0x27f9c) == 0x0` | `ret(0x27f9c) != 0x0` |
| `bne` at 0x27a24 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x27a5c | `[sp-20] < [0x147ec0] (signed, 32-bit)` | `[sp-20] >= [0x147ec0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27a94 | `r0` | `?` |

## `NL4R_ResetRelease`

- `NL4R_ResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27aac to 0x27c08.
  Established. Byte stores write `0x0` and later `0x1` at `0x147eb9` (`strb` at 0x27aec, `strb` at
  0x27bc4). The final call to `0x27d28` uses `[sp-20]` and three unknown candidates (`bl` at
  0x27be0). The prior branch bypasses it when an unknown value equals `0x40` and falls through
  on inequality (`beq` at 0x27bd8). Reset-release effects and r0 at exit are unestablished (`ldmdb`
  at 0x27bf0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27abc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27ac4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27ac8 | load | 4 | `abs 0x27bf4` | `-` |
| `ldrb` at 0x27acc | load | 1 | `abs 0x147eb8` | `-` |
| `ldr` at 0x27ad8 | load | 4 | `abs 0x27bf8` | `-` |
| `str` at 0x27adc | store | 4 | `sp -0x24` | `0xfffffed4` |
| `ldr` at 0x27ae4 | load | 4 | `abs 0x27bfc` | `-` |
| `strb` at 0x27aec | store | 1 | `abs 0x147eb9` | `0x0` |
| `ldr` at 0x27af4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27b04 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27b08 | load | 4 | `abs 0x27c00` | `-` |
| `ldr` at 0x27b0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27b10 | load | 4 | `abs 0x147ec0` | `-` |
| `ldr` at 0x27b20 | load | 4 | `abs 0x27c04` | `-` |
| `ldr` at 0x27b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27b2c | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27b40 | store | 4 | `sp -0x20` | `ret(0x2812c)` |
| `ldr` at 0x27b44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27b5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x27b60 | store | 4 | `sp -0x1c` | `ret(0x2812c)` |
| `ldr` at 0x27b64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27b6c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x27b74 | load | 4 | `abs 0x27c00` | `-` |
| `ldr` at 0x27b78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27b7c | load | 4 | `abs 0x147ec0` | `-` |
| `str` at 0x27b8c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x27b94 | load | 4 | `abs 0x27c04` | `-` |
| `ldr` at 0x27b98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27ba0 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27bb4 | store | 4 | `sp -0x1c` | `ret(0x2812c)` |
| `ldr` at 0x27bbc | load | 4 | `abs 0x27bfc` | `-` |
| `strb` at 0x27bc4 | store | 1 | `abs 0x147eb9` | `0x1` |
| `ldr` at 0x27bdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27be4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27be8 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x27bec | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x27bf0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27bf0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27bf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27af0 | `(unknown callee) 0xe335c` | `r0` | `r1` | `0x147eb9` | `0x0` |
| `bl` at 0x27b38 | `(unknown callee) 0x2812c` | `?` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27bac | `(unknown callee) 0x2812c` | `?` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27bb8 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `?` |
| `bl` at 0x27bc8 | `(unknown callee) 0x4f090` | `ret(0xe33c8)` | `?` | `0x147eb9` | `0x1` |
| `bl` at 0x27be0 | `(unknown callee) 0x27d28` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27ad4 | `[0x147eb8]:1 != 0x0` | `[0x147eb8]:1 == 0x0` |
| `bne` at 0x27afc | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x27b18 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27b4c | `ret(0x2812c) == 0x0` | `ret(0x2812c) != 0x0` |
| `bne` at 0x27b58 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x27b84 | `[sp-20] < [0x147ec0] (signed, 32-bit)` | `[sp-20] >= [0x147ec0] (signed, 32-bit)` |
| `beq` at 0x27bd8 | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27bf0 | `r0` | `?` |

## `NL4R_SoftReset`

- `NL4R_SoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27c08 to 0x27d28.
  Established. The selector branch is taken for `r0 != 0xff` and falls through for
  `r0 == 0xff` (`bne` at 0x27c48). Both calls to `0x28468` carry unknown, `0x147ebc`, unknown, unknown
  (`bl` at 0x27c84, `bl` at 0x27d04). The second selection test takes its arm for signed `r0 <
  [0x147ec0]` and falls through for greater or equal (`blt` at 0x27cdc). Soft-reset signaling and
  the common r0 return remain unidentified (`ldmdb` at 0x27d14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27c18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x27c20 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x27c24 | load | 4 | `abs 0x27d18` | `-` |
| `ldrb` at 0x27c28 | load | 1 | `abs 0x147eb8` | `-` |
| `ldr` at 0x27c34 | load | 4 | `abs 0x27d1c` | `-` |
| `str` at 0x27c38 | store | 4 | `sp -0x20` | `0xfffffed4` |
| `ldr` at 0x27c40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27c50 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27c54 | load | 4 | `abs 0x27d20` | `-` |
| `ldr` at 0x27c58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27c5c | load | 4 | `abs 0x147ec0` | `-` |
| `ldr` at 0x27c6c | load | 4 | `abs 0x27d24` | `-` |
| `ldr` at 0x27c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27c78 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27c8c | store | 4 | `sp -0x1c` | `ret(0x28468)` |
| `ldr` at 0x27c90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x27c9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x27ca8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x27cac | store | 4 | `sp -0x18` | `ret(0x28468)` |
| `ldr` at 0x27cb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27cb8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27cc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x27cc4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27ccc | load | 4 | `abs 0x27d20` | `-` |
| `ldr` at 0x27cd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27cd4 | load | 4 | `abs 0x147ec0` | `-` |
| `str` at 0x27ce4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27cec | load | 4 | `abs 0x27d24` | `-` |
| `ldr` at 0x27cf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27cf8 | load | 4 | `abs 0x147ebc` | `-` |
| `str` at 0x27d0c | store | 4 | `sp -0x20` | `ret(0x28468)` |
| `ldr` at 0x27d10 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x27d14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27d14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27d14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27c84 | `(unknown callee) 0x28468` | `?` | `0x147ebc` | `?` | `?` |
| `bl` at 0x27d04 | `(unknown callee) 0x28468` | `?` | `0x147ebc` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27c30 | `[0x147eb8]:1 != 0x0` | `[0x147eb8]:1 == 0x0` |
| `bne` at 0x27c48 | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x27c64 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x27c98 | `ret(0x28468) == 0x0` | `ret(0x28468) != 0x0` |
| `bne` at 0x27ca4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0x27cdc | `r0 < [0x147ec0] (signed, 32-bit)` | `r0 >= [0x147ec0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27d14 | `r0` | `?` |

## `PICResetHold`

- `PICResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34294 to 0x34454.
  Established. The call to `0xd69d4` includes `0x0`, `[[sp-24]+2]:1`, `[[sp-24]+3]:1`,
  `[[sp-24]+3]:1` (`bl` at 0x34374); its zero result takes the continuation and nonzero falls
  through (`beq` at 0x34388). A later byte store writes `0x0` at `0x147f51` (`strb` at 0x34400). The
  final call to `0x58bb8` has `0x0`, `0xff`, `0xff`, `0x0` (`bl` at 0x34410). PIC reset-line meaning
  and r0 at exit remain unknown (`ldmdb` at 0x34440).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x342a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x342a8 | load | 4 | `abs 0x34444` | `-` |
| `ldr` at 0x342ac | load | 4 | `abs 0x147f4c` | `-` |
| `ldr` at 0x342b8 | load | 4 | `abs 0x34448` | `-` |
| `str` at 0x342bc | store | 4 | `sp -0x24` | `0xfffff128` |
| `str` at 0x342c8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x342cc | load | 4 | `abs 0x3444c` | `-` |
| `ldrb` at 0x342d0 | load | 1 | `abs 0x147f50` | `-` |
| `ldr` at 0x342d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x342e4 | load | 4 | `abs 0x34444` | `-` |
| `ldr` at 0x342e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x342fc | load | 4 | `abs 0x147f4c` | `-` |
| `str` at 0x34304 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34308 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3430c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34310 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34314 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3432c | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x34330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3433c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34340 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldr` at 0x34348 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3434c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x34358 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3435c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x34360 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34364 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0x3437c | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x34380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3438c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34390 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldr` at 0x34398 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3439c | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x343a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x343ac | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x343b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x343b4 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0x343cc | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x343d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x343dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x343e0 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldr` at 0x343e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x343f0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x343f8 | load | 4 | `abs 0x34450` | `-` |
| `strb` at 0x34400 | store | 1 | `abs 0x147f51` | `0x0` |
| `str` at 0x34418 | store | 4 | `sp -0x1c` | `ret(0x58bb8)` |
| `ldr` at 0x3441c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x34428 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3442c | store | 4 | `sp -0x24` | `ret(0x58bb8)` |
| `str` at 0x34438 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3443c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x34440 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34440 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34440 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x34324 | `(unknown callee) 0xd69d4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x34374 | `(unknown callee) 0xd69d4` | `0x0` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[[sp-24]+3]:1` |
| `bl` at 0x343c4 | `(unknown callee) 0xd69d4` | `0x0` | `[[sp-24]+4]:1` | `[[sp-24]+5]:1` | `[[sp-24]+5]:1` |
| `bl` at 0x34410 | `(unknown callee) 0x58bb8` | `0x0` | `0xff` | `0xff` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x342b4 | `[0x147f4c] != 0x0` | `[0x147f4c] == 0x0` |
| `blt` at 0x342dc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x34338 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x34354 | `[[sp-24]+2]:1 == 0xff` | `[[sp-24]+2]:1 != 0xff` |
| `beq` at 0x34388 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x343a4 | `[[sp-24]+4]:1 == 0xff` | `[[sp-24]+4]:1 != 0xff` |
| `beq` at 0x343d8 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x34424 | `ret(0x58bb8) == 0x0` | `ret(0x58bb8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34440 | `r0` | `?` |

## `PICResetRelease`

- `PICResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34454 to 0x34740.
  Established. The call to `0x5964c` uses `0x0`, `0x0`, unknown, `ret(0x3d8e0)` (`bl` at 0x344b0). A
  later call to the same target has `0x0`, `0x1`, `[0x147f50]:1`, unknown (`bl` at 0x34698). Near
  the end a byte store writes `0x1` at `0x147f51` (`strb` at 0x3471c). The final call to `0x58e8c`
  has `0x0`, `0xff`, `0xff`, `ret(0x59534)` (`bl` at 0x346f0). Physical release order and the joined
  r0 value are unproved (`ldmdb` at 0x3472c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34464 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3446c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x34470 | load | 4 | `abs 0x34730` | `-` |
| `ldr` at 0x34474 | load | 4 | `abs 0x147f4c` | `-` |
| `ldr` at 0x34480 | load | 4 | `abs 0x34734` | `-` |
| `str` at 0x34484 | store | 4 | `sp -0x2c` | `0xfffff128` |
| `str` at 0x344a0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x344b8 | store | 4 | `sp -0x20` | `ret(0x5964c)` |
| `ldr` at 0x344bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x344c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x344cc | store | 4 | `sp -0x2c` | `ret(0x5964c)` |
| `str` at 0x344d8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x344dc | load | 4 | `abs 0x34738` | `-` |
| `ldrb` at 0x344e0 | load | 1 | `abs 0x147f50` | `-` |
| `ldr` at 0x344e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x344f8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x344fc | load | 4 | `abs 0x34730` | `-` |
| `ldr` at 0x34500 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34514 | load | 4 | `abs 0x147f4c` | `-` |
| `str` at 0x3451c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34520 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34524 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34530 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34538 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x3453c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34540 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34544 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34548 | load | 1 | `(unknown)` | `-` |
| `str` at 0x34560 | store | 4 | `sp -0x20` | `ret(0xd6988)` |
| `ldr` at 0x34564 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34570 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34574 | store | 4 | `sp -0x2c` | `ret(0xd6988)` |
| `ldr` at 0x3457c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34580 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3458c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34594 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x34598 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3459c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x345a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x345a4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x345bc | store | 4 | `sp -0x20` | `ret(0xd6988)` |
| `ldr` at 0x345c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x345cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x345d0 | store | 4 | `sp -0x2c` | `ret(0xd6988)` |
| `ldr` at 0x345d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x345e0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x345e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x34600 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x34604 | load | 4 | `abs 0x34738` | `-` |
| `ldrb` at 0x34608 | load | 1 | `abs 0x147f50` | `-` |
| `ldr` at 0x3460c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3461c | load | 4 | `abs 0x34730` | `-` |
| `ldr` at 0x34620 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x34634 | load | 4 | `abs 0x147f4c` | `-` |
| `str` at 0x3463c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x34640 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x34644 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x34648 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3464c | load | 1 | `(unknown)` | `-` |
| `str` at 0x34664 | store | 4 | `sp -0x20` | `ret(0xd6988)` |
| `ldr` at 0x34668 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34674 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x34678 | store | 4 | `sp -0x2c` | `ret(0xd6988)` |
| `ldr` at 0x34680 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x34688 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x346a0 | store | 4 | `sp -0x20` | `ret(0x5964c)` |
| `ldr` at 0x346a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x346b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x346b4 | store | 4 | `sp -0x2c` | `ret(0x5964c)` |
| `str` at 0x346c8 | store | 4 | `sp -0x20` | `ret(0x59534)` |
| `ldr` at 0x346cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x346d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x346dc | store | 4 | `sp -0x2c` | `ret(0x59534)` |
| `str` at 0x346f8 | store | 4 | `sp -0x20` | `ret(0x58e8c)` |
| `ldr` at 0x346fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x34708 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3470c | store | 4 | `sp -0x2c` | `ret(0x58e8c)` |
| `ldr` at 0x34714 | load | 4 | `abs 0x3473c` | `-` |
| `strb` at 0x3471c | store | 1 | `abs 0x147f51` | `0x1` |
| `str` at 0x34724 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x34728 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3472c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3472c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3472c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3448c | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `[0x147f4c]` |
| `bl` at 0x344b0 | `(unknown callee) 0x5964c` | `0x0` | `0x0` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x34558 | `(unknown callee) 0xd6988` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x345b4 | `(unknown callee) 0xd6988` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x345f8 | `(unknown callee) 0x34920` | `0xa` | `?` | `[0x147f50]:1` | `[sp-28]` |
| `bl` at 0x3465c | `(unknown callee) 0xd6988` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x34698 | `(unknown callee) 0x5964c` | `0x0` | `0x1` | `[0x147f50]:1` | `?` |
| `bl` at 0x346c0 | `(unknown callee) 0x59534` | `0x0` | `?` | `?` | `ret(0x5964c)` |
| `bl` at 0x346f0 | `(unknown callee) 0x58e8c` | `0x0` | `0xff` | `0xff` | `ret(0x59534)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3447c | `[0x147f4c] != 0x0` | `[0x147f4c] == 0x0` |
| `bne` at 0x34498 | `ret(0x3d8e0) != 0x0` | `ret(0x3d8e0) == 0x0` |
| `beq` at 0x344c4 | `ret(0x5964c) == 0x0` | `ret(0x5964c) != 0x0` |
| `blt` at 0x344ec | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3452c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x3456c | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x34588 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x345c8 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x345f0 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `blt` at 0x34614 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x34670 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x346ac | `ret(0x5964c) == 0x0` | `ret(0x5964c) != 0x0` |
| `beq` at 0x346d4 | `ret(0x59534) == 0x0` | `ret(0x59534) != 0x0` |
| `beq` at 0x34704 | `ret(0x58e8c) == 0x0` | `ret(0x58e8c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3472c | `r0` | `?` |

## `SHUB2_ResetHold`

- `SHUB2_ResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ab64 to 0x3ad24.
  Established. A word store writes `0x0` at `[0x147fbc] +0xc` (`str` at 0x3abe4). Both calls to
  `0x3b1dc` have four unknown candidates (`bl` at 0x3ac58, `bl` at 0x3acf8). The later signed branch
  takes its arm for `[sp-20] < [[sp-40]+8]` and falls through for greater or equal (`blt` at
  0x3aca4). SHUB2 reset assertion and the final r0 remain unknown (`ldmdb` at 0x3ad10).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ab74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3ab7c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x3ab84 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3ab88 | load | 4 | `abs 0x3ad14` | `-` |
| `ldr` at 0x3ab8c | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3ab98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3aba0 | load | 4 | `abs 0x3ad18` | `-` |
| `ldr` at 0x3aba4 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3abac | load | 4 | `[0x147fbc] +0x8` | `-` |
| `ldr` at 0x3abb8 | load | 4 | `abs 0x3ad1c` | `-` |
| `str` at 0x3abbc | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3abc4 | load | 4 | `abs 0x3ad18` | `-` |
| `ldr` at 0x3abc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3abd0 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3abd8 | store | 4 | `sp -0x28` | `[0x147fbc]` |
| `ldr` at 0x3abdc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3abe4 | store | 4 | `[0x147fbc] +0xc` | `0x0` |
| `ldr` at 0x3abe8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3abf8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3abfc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ac04 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3ac14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ac2c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3ac34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ac4c | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3ac60 | store | 4 | `sp -0x1c` | `ret(0x3b1dc)` |
| `ldr` at 0x3ac64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ac70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ac7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ac80 | store | 4 | `sp -0x20` | `ret(0x3b1dc)` |
| `ldr` at 0x3ac84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ac8c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3ac94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ac98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ac9c | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3aca8 | load | 4 | `abs 0x3ad20` | `-` |
| `str` at 0x3acac | store | 4 | `sp -0x20` | `0xffffeffa` |
| `ldr` at 0x3acb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3acb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3accc | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3acd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3acd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3acec | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3ad00 | store | 4 | `sp -0x20` | `ret(0x3b1dc)` |
| `ldr` at 0x3ad04 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3ad08 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x3ad0c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3ad10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ad10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ad10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ac58 | `(unknown callee) 0x3b1dc` | `?` | `?` | `?` | `?` |
| `bl` at 0x3acf8 | `(unknown callee) 0x3b1dc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3ab94 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3abb4 | `[[0x147fbc]+8] != 0x0` | `[[0x147fbc]+8] == 0x0` |
| `bne` at 0x3abf0 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x3ac0c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3ac6c | `ret(0x3b1dc) == 0x0` | `ret(0x3b1dc) != 0x0` |
| `bne` at 0x3ac78 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x3aca4 | `[sp-20] < [[sp-40]+8] (signed, 32-bit)` | `[sp-20] >= [[sp-40]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ad10 | `r0` | `?` |

## `SHUB2_ResetRelease`

- `SHUB2_ResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3ad24 to 0x3af08.
  Established. The early word store writes `0x0` at `[0x147fbc] +0xc`, and a later store writes
  `0x1` at `[sp-40] +0xc` (`str` at 0x3ada4, `str` at 0x3aed4). The call to `0x3af08` carries
  `[sp-36]`, `[sp-20]:1`, `[sp-40]`, `[sp-20]:1` (`bl` at 0x3aee4). The earlier selector branch
  takes its arm for `[sp-20] != 0xff` and falls through on equality (`bne` at 0x3adb4). Release
  signaling and r0 at the merged exit remain unresolved (`ldmdb` at 0x3aef4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3ad34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3ad3c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x3ad44 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3ad48 | load | 4 | `abs 0x3aef8` | `-` |
| `ldr` at 0x3ad4c | load | 4 | `abs 0x147fc0` | `-` |
| `ldr` at 0x3ad58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ad60 | load | 4 | `abs 0x3aefc` | `-` |
| `ldr` at 0x3ad64 | load | 4 | `abs 0x147fbc` | `-` |
| `ldr` at 0x3ad6c | load | 4 | `[0x147fbc] +0x8` | `-` |
| `ldr` at 0x3ad78 | load | 4 | `abs 0x3af00` | `-` |
| `str` at 0x3ad7c | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3ad84 | load | 4 | `abs 0x3aefc` | `-` |
| `ldr` at 0x3ad88 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3ad90 | load | 4 | `abs 0x147fbc` | `-` |
| `str` at 0x3ad98 | store | 4 | `sp -0x28` | `[0x147fbc]` |
| `ldr` at 0x3ad9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3ada4 | store | 4 | `[0x147fbc] +0xc` | `0x0` |
| `ldr` at 0x3adac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3adbc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3adc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3adc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3adc8 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3add8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3addc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3adf0 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3adf8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3adfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ae10 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3ae24 | store | 4 | `sp -0x1c` | `ret(0x3b42c)` |
| `ldr` at 0x3ae28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3ae34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3ae40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3ae44 | store | 4 | `sp -0x20` | `ret(0x3b42c)` |
| `ldr` at 0x3ae48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ae50 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x3ae58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ae5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ae60 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x3ae6c | load | 4 | `abs 0x3af04` | `-` |
| `str` at 0x3ae70 | store | 4 | `sp -0x20` | `0xffffeffa` |
| `ldr` at 0x3ae78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ae7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3ae90 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x3ae98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3ae9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aeb0 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x3aec4 | store | 4 | `sp -0x20` | `ret(0x3b42c)` |
| `ldr` at 0x3aecc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3aed4 | store | 4 | `[sp-40] +0xc` | `0x1` |
| `ldrb` at 0x3aed8 | load | 1 | `sp -0x14` | `-` |
| `ldr` at 0x3aedc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3aee8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3aeec | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldr` at 0x3aef0 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3aef4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3aef4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3aef4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3ada8 | `(unknown callee) 0xe335c` | `r0` | `0x147fbc` | `[0x147fbc]` | `0x0` |
| `bl` at 0x3ae1c | `(unknown callee) 0x3b42c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3aebc | `(unknown callee) 0x3b42c` | `?` | `?` | `?` | `?` |
| `bl` at 0x3aec8 | `(unknown callee) 0xe33c8` | `?` | `?` | `?` | `?` |
| `bl` at 0x3aee4 | `(unknown callee) 0x3af08` | `[sp-36]` | `[sp-20]:1` | `[sp-40]` | `[sp-20]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3ad54 | `[0x147fc0] == 0x0` | `[0x147fc0] != 0x0` |
| `bne` at 0x3ad74 | `[[0x147fbc]+8] != 0x0` | `[[0x147fbc]+8] == 0x0` |
| `bne` at 0x3adb4 | `[sp-20] != 0xff` | `[sp-20] == 0xff` |
| `blt` at 0x3add0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3ae30 | `ret(0x3b42c) == 0x0` | `ret(0x3b42c) != 0x0` |
| `bne` at 0x3ae3c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x3ae68 | `[sp-20] < [[sp-40]+8] (signed, 32-bit)` | `[sp-20] >= [[sp-40]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3aef4 | `r0` | `?` |

## `ip63_reset_hold`

- `ip63_reset_hold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x37fd8 to 0x38000.
  Established. The single call reaches `0x3ab64` with entry r0, entry r1, entry r2, entry r3 (`bl`
  at 0x37ff0). The return propagates `ret(0x3ab64)` (`ldmdb` at 0x37ffc). There are no conditional
  branches in the facts. Physical reset-hold behavior lies beyond the wrapper.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x37fe8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x37fec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x37ffc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x37ffc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x37ffc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x37ff0 | `(unknown callee) 0x3ab64` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x37ffc | `r0` | `ret(0x3ab64)` |

## `ip63_reset_release`

- `ip63_reset_release`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38000 to 0x38028.
  Established. The wrapper calls `0x3ad24` with entry r0, entry r1, entry r2, entry r3 (`bl` at
  0x38018); its result reaches the return as `ret(0x3ad24)` (`ldmdb` at 0x38024). No conditional
  branch is reported. The wrapper does not establish the callee physical release effects.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38010 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x38014 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x38024 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38024 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38024 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38018 | `(unknown callee) 0x3ad24` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38024 | `r0` | `ret(0x3ad24)` |

## `IP63_SoftReset`

- `IP63_SoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3826c to 0x38294.
  Established. The only call reaches `0x20240` with `0x107650`, entry r1, entry r2, entry r3 (`bl`
  at 0x38284). The exit reports an unknown r0 (`ldmdb` at 0x3828c). Conditional-branch facts are
  absent. A physical soft-reset operation is not proved by that numeric call.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3827c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x38280 | load | 4 | `abs 0x38290` | `-` |
| `ldmdb` at 0x3828c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3828c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3828c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38284 | `(unknown callee) 0x20240` | `0x107650` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3828c | `r0` | `?` |

## `IP65_ResetHold`

- `IP65_ResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64814 to 0x64838.
  Established. The explicit byte save writes `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x64828). The
  return supplies r0 `0x0` (`ldmdb` at 0x64834). No calls or conditional branches appear in the
  facts. Reset-hold signaling is unestablished in this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64828 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x64834 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64834 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64834 | `r0` | `0x0` |

## `IP65_ResetRelease`

- `IP65_ResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64838 to 0x6485c.
  Established. Entry r0 is saved as `(r0 & 0xff)` at stack location `sp -0x11` (`strb` at 0x6484c);
  its unbranched exit carries r0 `0x0` (`ldmdb` at 0x64858). Calls and conditional branches are
  absent from the facts. The routine name does not establish a reset release.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6484c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x64858 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64858 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64858 | `r0` | `0x0` |

## `IP65_SoftReset`

- `IP65_SoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6485c to 0x64880.
  Established. The byte store at `sp -0x11` retains `(r0 & 0xff)` (`strb` at 0x64870). The exit
  reports r0 `0x0` (`ldmdb` at 0x6487c). The fact sections have neither calls nor conditional
  branches. Hardware soft-reset effects remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64870 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x6487c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6487c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6487c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6487c | `r0` | `0x0` |

## `SHUB_Reset_MCI`

- `SHUB_Reset_MCI`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47e08 to 0x47e28.
  Established. The word store clears `0x148074` to `0x0` (`str` at 0x47e1c); its return retains
  entry r0 (`ldmdb` at 0x47e20). There are no calls or conditional branches in the fact sections.
  MCI reset meaning is not established by the cleared location.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x47e14 | load | 4 | `abs 0x47e24` | `-` |
| `str` at 0x47e1c | store | 4 | `abs 0x148074` | `0x0` |
| `ldmdb` at 0x47e20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47e20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47e20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47e20 | `r0` | `r0` |

## `SHUB_Reset_WRT90`

- `SHUB_Reset_WRT90`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47f10 to 0x47f30.
  Established. The word write places `0x0` at `0x148078` (`str` at 0x47f24). Entry r0 survives at
  the return (`ldmdb` at 0x47f28). No call or conditional branch is reported. The cleared location
  has no established physical reset role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x47f1c | load | 4 | `abs 0x47f2c` | `-` |
| `str` at 0x47f24 | store | 4 | `abs 0x148078` | `0x0` |
| `ldmdb` at 0x47f28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47f28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47f28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47f28 | `r0` | `r0` |
