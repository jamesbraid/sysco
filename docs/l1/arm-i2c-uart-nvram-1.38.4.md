# ARM raw multiplexer, UART and NVRAM paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/i2c-uart-nvram-facts.txt) retain all tool sections.

## `i2cMuxNotPresent`

- `i2cMuxNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd47a8 to 0xd48bc.
  Established. The signed bound takes its alternate path for `(r1 & 0xff) >= [[sp-28]+4]` and falls
  through for less (`bge` at 0xd47f8). The fall-through byte store clears `((r1 & 0xff) +
  [[sp-28]+8]) +0x0` (`strb` at 0xd4810). The other path includes a call to `0x21e28` with `0x6`,
  `0x1344fc`, `0x7a`, `0x0` (`bl` at 0xd48a0). The physical presence flag and joined r0 value remain
  unknown (`ldmdb` at 0xd48a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd47b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd47c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd47c4 | load | 4 | `abs 0xd48a8` | `-` |
| `ldr` at 0xd47c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd47dc | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd47e4 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd47e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd47ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd47f0 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd47fc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd4800 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4804 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd4810 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd4818 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4820 | load | 4 | `abs 0xd48ac` | `-` |
| `ldr` at 0xd4824 | load | 4 | `abs 0xd48b0` | `-` |
| `str` at 0xd4850 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd4854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd485c | load | 4 | `abs 0xd48ac` | `-` |
| `ldr` at 0xd4860 | load | 4 | `abs 0xd48b0` | `-` |
| `ldr` at 0xd486c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd487c | load | 4 | `abs 0xd48b4` | `-` |
| `str` at 0xd4880 | store | 4 | `(unknown)` | `0x1345a0` |
| `str` at 0xd4884 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `ldrb` at 0xd4888 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd488c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd4894 | load | 4 | `abs 0xd48b8` | `-` |
| `ldmdb` at 0xd48a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd48a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd48a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4828 | `(unknown callee) 0x4f564` | `r0` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd4834 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd4864 | `(unknown callee) 0x4f564` | `[sp-20]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd4874 | `(unknown callee) 0x1b508` | `[sp-32]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd48a0 | `(unknown callee) 0x21e28` | `0x6` | `0x1344fc` | `0x7a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd47f8 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd48a4 | `r0` | `?` |

## `i2cMuxProbe`

- `i2cMuxProbe`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd48bc to 0xd4a3c.
  Established. The call to `0xd4bdc` carries entry r0, `(r1 & 0xff)`, `0x0`, `(r1 & 0xff)` (`bl` at
  0xd4920); its zero result takes the arm that stores word `0x1` at `sp -0x34`, while nonzero falls
  through to a second test (`beq` at 0xd4934, `str` at 0xd4a10). The latter route includes a
  byte clear at `([sp-29]:1 + [[sp-36]+8]) +0x0` (`strb` at 0xd4a04). The probing protocol and
  merged r0 return are unproven (`ldmdb` at 0xd4a24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd48cc | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd48d4 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd48d8 | load | 4 | `abs 0xd4a28` | `-` |
| `ldr` at 0xd48dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd48f0 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd48f8 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xd48fc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4900 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4904 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0xd4910 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4914 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd4928 | store | 4 | `sp -0x28` | `ret(0xd4bdc)` |
| `ldr` at 0xd492c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4938 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd494c | load | 4 | `abs 0xd4a2c` | `-` |
| `ldr` at 0xd4950 | load | 4 | `abs 0xd4a30` | `-` |
| `str` at 0xd497c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd4980 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4988 | load | 4 | `abs 0xd4a2c` | `-` |
| `ldr` at 0xd498c | load | 4 | `abs 0xd4a30` | `-` |
| `ldr` at 0xd4998 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xd49a8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd49ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd49bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd49c8 | load | 4 | `abs 0xd4a34` | `-` |
| `str` at 0xd49cc | store | 4 | `(unknown)` | `0x1345c8` |
| `str` at 0xd49d0 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0xd49d4 | store | 4 | `(unknown)` | `ret(0xd4dbc)` |
| `str` at 0xd49d8 | store | 4 | `(unknown)` | `ret(0x219a8)` |
| `ldr` at 0xd49e0 | load | 4 | `abs 0xd4a38` | `-` |
| `ldr` at 0xd49f0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd49f4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd49f8 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xd4a04 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `str` at 0xd4a10 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xd4a1c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd4a20 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd4a24 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd4a24 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd4a24 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4a24 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4a24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4920 | `(unknown callee) 0xd4bdc` | `r0` | `(r1 & 0xff)` | `0x0` | `(r1 & 0xff)` |
| `bl` at 0xd4954 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd4960 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd4990 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd49a0 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd49b4 | `(unknown callee) 0xd4dbc` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd49c0 | `(unknown callee) 0x219a8` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd49ec | `(unknown callee) 0x21e28` | `0x1` | `0x1344fc` | `0x8c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd490c | `(r1 & 0xff) >= [[sp-36]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-36]+4] (signed, 32-bit)` |
| `beq` at 0xd4934 | `ret(0xd4bdc) == 0x0` | `ret(0xd4bdc) != 0x0` |
| `beq` at 0xd4940 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4a24 | `r0` | `?` |

## `i2cMuxSelect`

- `i2cMuxSelect`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4bdc to 0xd4d10.
  Established. The branch takes its continuing edge for `ret(0xc7a50) == 0x0` and falls through on
  nonzero to an early saved result (`beq` at 0xd4c98, `str` at 0xd4ca0). The next selector takes
  its alternate arm for `[[sp-32]+1]:1 != 0x0` and falls through on zero (`bne` at 0xd4cb4). Those
  arms call `0xd535c` and `0xd5108`, respectively, with `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`,
  `[sp-21]:1` (`bl` at 0xd4ce8, `bl` at 0xd4cc8). Device selection and the joined return remain
  unknown (`ldmdb` at 0xd4d04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd4bec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd4bf4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd4bfc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xd4c00 | load | 4 | `abs 0xd4d08` | `-` |
| `ldr` at 0xd4c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4c18 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4c20 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd4c24 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4c28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4c38 | store | 4 | `sp -0x20` | `ret(0xd583c)` |
| `ldr` at 0xd4c3c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd4c40 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd4c44 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd4c4c | load | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `-` |
| `ldr` at 0xd4c58 | load | 4 | `abs 0xd4d0c` | `-` |
| `str` at 0xd4c5c | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0xd4c64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd4c68 | load | 1 | `ret(0xd583c) +0x1` | `-` |
| `ldr` at 0xd4c6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4c8c | store | 4 | `sp -0x24` | `ret(0xc7a50)` |
| `ldr` at 0xd4c90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd4c9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4ca0 | store | 4 | `sp -0x28` | `ret(0xc7a50)` |
| `ldr` at 0xd4ca8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd4cac | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd4cb8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd4cbc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd4cc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4cd0 | store | 4 | `sp -0x24` | `ret(0xd5108)` |
| `ldrb` at 0xd4cd8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xd4cdc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd4ce0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd4cf0 | store | 4 | `sp -0x24` | `ret(0xd535c)` |
| `ldr` at 0xd4cf8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4cfc | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xd4d00 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd4d04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd4d04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd4d04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4c30 | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd4c74 | `(unknown callee) 0xc7784` | `[sp-20]` | `[ret(0xd583c)+1]:1` | `[sp-21]:1` | `[ret(0xd583c)+1]:1` |
| `bl` at 0xd4c84 | `(unknown callee) 0xc7a50` | `?` | `?` | `?` | `?` |
| `bl` at 0xd4cc8 | `(unknown callee) 0xd5108` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xd4ce8 | `(unknown callee) 0xd535c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0xd4cf4 | `(unknown callee) 0xc7b90` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd4c54 | `[([sp-21]:1 + [[sp-28]+8])]:1 != 0x0` | `[([sp-21]:1 + [[sp-28]+8])]:1 == 0x0` |
| `beq` at 0xd4c98 | `ret(0xc7a50) == 0x0` | `ret(0xc7a50) != 0x0` |
| `bne` at 0xd4cb4 | `[[sp-32]+1]:1 != 0x0` | `[[sp-32]+1]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd4d04 | `r0` | `?` |

## `i2cMuxResetRaw`

- `i2cMuxResetRaw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd4e24 to 0xd5108.
  Established. The first test takes the continuation for `[0x2ad218] != 0x0` and falls through on
  zero to an early zero stack result (`bne` at 0xd4e54, `str` at 0xd4e5c). Later byte stores
  clear `0x2ad210` and `0x2ad211`, with unknown bytes written at `0x142b0c`, `0x142b0d`, `0x142b0e`,
  `0x142b0f` (`strb` at 0xd4f10, `strb` at 0xd5068, `strb` at 0xd4f1c, `strb` at 0xd4f28, `strb` at
  0xd5074, `strb` at 0xd5080). The final call to `0xd5544` carries `[0x2ad210]:1`, `[0x142b0c]:1`,
  `[0x2ad210]:1`, `[0x142b0c]:1` (`bl` at 0xd50ac). Physical reset actions and the merged r0 value
  remain unknown (`ldmdb` at 0xd50e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd4e34 | load | 4 | `abs 0xd50e8` | `-` |
| `ldr` at 0xd4e38 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd4e3c | store | 4 | `sp -0x14` | `[0x2ad214]` |
| `str` at 0xd4e44 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd4e48 | load | 4 | `abs 0xd50ec` | `-` |
| `ldr` at 0xd4e4c | load | 4 | `abs 0x2ad218` | `-` |
| `str` at 0xd4e5c | store | 4 | `sp -0x24` | `0x0` |
| `strb` at 0xd4e68 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd4e6c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4e70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd4e84 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd4e88 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4e8c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd4e94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd4ea0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd4ea4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4eac | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xd4eb4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd4ec0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd4ed4 | store | 4 | `sp -0x1c` | `ret(0xd5544)` |
| `ldr` at 0xd4ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4ee4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4ef0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd4ef4 | store | 4 | `sp -0x18` | `ret(0xd5544)` |
| `ldrb` at 0xd4ef8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd4f00 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xd4f08 | load | 4 | `abs 0xd50f0` | `-` |
| `strb` at 0xd4f10 | store | 1 | `abs 0x2ad210` | `0x0` |
| `ldr` at 0xd4f14 | load | 4 | `abs 0xd50f4` | `-` |
| `strb` at 0xd4f1c | store | 1 | `abs 0x142b0c` | `?` |
| `ldr` at 0xd4f20 | load | 4 | `abs 0xd50f8` | `-` |
| `strb` at 0xd4f28 | store | 1 | `abs 0x142b0d` | `?` |
| `strb` at 0xd4f30 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0xd4f34 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd4f38 | load | 4 | `abs 0xd50ec` | `-` |
| `ldr` at 0xd4f3c | load | 4 | `abs 0x2ad218` | `-` |
| `strb` at 0xd4f50 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd4f54 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd4f5c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xd4f6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd4f70 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4f74 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrb` at 0xd4f7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd4f88 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd4f8c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4f94 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xd4f9c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd4fa8 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0xd4fac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd4fb0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd4fb8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrb` at 0xd4fc0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xd4fe0 | store | 4 | `sp -0x1c` | `ret(0xc7c84)` |
| `ldr` at 0xd4fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4ff0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4ffc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5000 | store | 4 | `sp -0x18` | `ret(0xc7c84)` |
| `ldrb` at 0xd5004 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0xd5008 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd501c | store | 4 | `sp -0x1c` | `ret(0xd5544)` |
| `ldr` at 0xd5020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd502c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5038 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd503c | store | 4 | `sp -0x18` | `ret(0xd5544)` |
| `ldrb` at 0xd5040 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd5048 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0xd5050 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0xd5058 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd5060 | load | 4 | `abs 0xd50fc` | `-` |
| `strb` at 0xd5068 | store | 1 | `abs 0x2ad211` | `0x0` |
| `ldr` at 0xd506c | load | 4 | `abs 0xd5100` | `-` |
| `strb` at 0xd5074 | store | 1 | `abs 0x142b0e` | `?` |
| `ldr` at 0xd5078 | load | 4 | `abs 0xd5104` | `-` |
| `strb` at 0xd5080 | store | 1 | `abs 0x142b0f` | `?` |
| `ldr` at 0xd5084 | load | 4 | `abs 0xd50f4` | `-` |
| `ldrb` at 0xd5088 | load | 1 | `abs 0x142b0c` | `-` |
| `ldr` at 0xd5094 | load | 4 | `abs 0xd50f0` | `-` |
| `ldrb` at 0xd5098 | load | 1 | `abs 0x2ad210` | `-` |
| `ldr` at 0xd509c | load | 4 | `abs 0xd50f4` | `-` |
| `ldrb` at 0xd50a0 | load | 1 | `abs 0x142b0c` | `-` |
| `str` at 0xd50b4 | store | 4 | `sp -0x1c` | `ret(0xd5544)` |
| `ldr` at 0xd50b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd50c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd50d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd50d4 | store | 4 | `sp -0x18` | `ret(0xd5544)` |
| `ldr` at 0xd50d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd50dc | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0xd50e0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd50e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd50e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd50e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd4ecc | `(unknown callee) 0xd5544` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xd4fc8 | `(unknown callee) 0xc7784` | `?` | `?` | `?` | `?` |
| `bl` at 0xd4fd8 | `(unknown callee) 0xc7c84` | `?` | `?` | `?` | `?` |
| `bl` at 0xd5014 | `(unknown callee) 0xd5544` | `[sp-30]:1` | `[sp-29]:1` | `[sp-29]:1` | `[sp-30]:1` |
| `bl` at 0xd50ac | `(unknown callee) 0xd5544` | `[0x2ad210]:1` | `[0x142b0c]:1` | `[0x2ad210]:1` | `[0x142b0c]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd4e54 | `[0x2ad218] != 0x0` | `[0x2ad218] == 0x0` |
| `blt` at 0xd4e7c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd4e9c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xd4ebc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xd4ee0 | `ret(0xd5544) == 0x0` | `ret(0xd5544) != 0x0` |
| `bne` at 0xd4eec | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `blt` at 0xd4f44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xd4f64 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd4f84 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xd4fa4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xd4fec | `ret(0xc7c84) == 0x0` | `ret(0xc7c84) != 0x0` |
| `bne` at 0xd4ff8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xd5028 | `ret(0xd5544) == 0x0` | `ret(0xd5544) != 0x0` |
| `bne` at 0xd5034 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xd5090 | `[0x142b0c]:1 == 0xfe` | `[0x142b0c]:1 != 0xfe` |
| `beq` at 0xd50c0 | `ret(0xd5544) == 0x0` | `ret(0xd5544) != 0x0` |
| `bne` at 0xd50cc | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd50e4 | `r0` | `?` |

## `i2cMuxSelectRaw`

- `i2cMuxSelectRaw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5108 to 0xd535c.
  Established. The call to `0xd594c` has `[sp-36]`, `[sp-26]:1`, `0x142b0f`, `[sp-26]:1` (`bl` at
  0xd5244); its zero result takes the arm that stores bytes `([sp-24]:1 & 0xff)` at `0x2ad210`,
  `([sp-25]:1 & 0xff)` at `0x142b0c`, and `([sp-26]:1 & 0xff)` at `0x142b0d` (`beq` at 0xd5258,
  `strb` at 0xd5304, `strb` at 0xd5310, `strb` at 0xd531c). Nonzero falls through to `0x4f564`
  (`beq` at 0xd5258, `bl` at 0xd526c). The physical selector encoding and returned r0 value are
  unknown (`ldmdb` at 0xd532c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5118 | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xd5120 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xd5128 | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xd512c | load | 4 | `abs 0xd5330` | `-` |
| `ldr` at 0xd5130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5144 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd514c | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd5150 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5154 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd5164 | store | 4 | `sp -0x24` | `ret(0xd583c)` |
| `ldr` at 0xd5168 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd516c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd5170 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd5178 | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd5184 | load | 4 | `abs 0xd5334` | `-` |
| `str` at 0xd5188 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xd5190 | load | 4 | `abs 0xd5338` | `-` |
| `ldrb` at 0xd5194 | load | 1 | `abs 0x2ad210` | `-` |
| `ldr` at 0xd5198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd51a4 | load | 4 | `abs 0xd533c` | `-` |
| `ldrb` at 0xd51a8 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd51ac | load | 1 | `abs 0x142b0c` | `-` |
| `ldr` at 0xd51b8 | load | 4 | `abs 0xd5340` | `-` |
| `ldrb` at 0xd51bc | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd51c0 | load | 1 | `abs 0x142b0d` | `-` |
| `str` at 0xd51d0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd51d8 | load | 4 | `abs 0xd533c` | `-` |
| `ldrb` at 0xd51dc | load | 1 | `abs 0x142b0c` | `-` |
| `ldr` at 0xd51e8 | load | 4 | `abs 0xd5340` | `-` |
| `ldrb` at 0xd51ec | load | 1 | `abs 0x142b0d` | `-` |
| `ldr` at 0xd51f8 | load | 4 | `abs 0xd5338` | `-` |
| `ldrb` at 0xd51fc | load | 1 | `abs 0x2ad210` | `-` |
| `ldr` at 0xd5200 | load | 4 | `abs 0xd533c` | `-` |
| `ldrb` at 0xd5204 | load | 1 | `abs 0x142b0c` | `-` |
| `ldr` at 0xd5214 | load | 4 | `abs 0xd5344` | `-` |
| `strb` at 0xd521c | store | 1 | `abs 0x2ad211` | `0x0` |
| `ldr` at 0xd5220 | load | 4 | `abs 0xd5348` | `-` |
| `strb` at 0xd5228 | store | 1 | `abs 0x142b0e` | `?` |
| `ldr` at 0xd522c | load | 4 | `abs 0xd534c` | `-` |
| `strb` at 0xd5234 | store | 1 | `abs 0x142b0f` | `?` |
| `ldrb` at 0xd5238 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd523c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd524c | store | 4 | `sp -0x28` | `ret(0xd594c)` |
| `ldr` at 0xd5250 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd525c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5264 | load | 4 | `abs 0xd5350` | `-` |
| `ldr` at 0xd5268 | load | 4 | `abs 0xd5354` | `-` |
| `str` at 0xd5294 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd5298 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd52a0 | load | 4 | `abs 0xd5350` | `-` |
| `ldr` at 0xd52a4 | load | 4 | `abs 0xd5354` | `-` |
| `ldr` at 0xd52b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd52c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd52cc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd52d0 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd52d4 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd52d8 | store | 4 | `(unknown)` | `ret(0x21674)` |
| `ldr` at 0xd52dc | load | 4 | `abs 0xd5358` | `-` |
| `ldr` at 0xd52f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd52f4 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd52fc | load | 4 | `abs 0xd5338` | `-` |
| `ldrb` at 0xd5300 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd5304 | store | 1 | `abs 0x2ad210` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xd5308 | load | 4 | `abs 0xd533c` | `-` |
| `ldrb` at 0xd530c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd5310 | store | 1 | `abs 0x142b0c` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xd5314 | load | 4 | `abs 0xd5340` | `-` |
| `ldrb` at 0xd5318 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd531c | store | 1 | `abs 0x142b0d` | `([sp-26]:1 & 0xff)` |
| `str` at 0xd5324 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd5328 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd532c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd532c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd532c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd532c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd515c | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5210 | `(unknown callee) 0xd5544` | `[0x2ad210]:1` | `[0x142b0c]:1` | `[0x2ad210]:1` | `[0x142b0c]:1` |
| `bl` at 0xd5244 | `(unknown callee) 0xd594c` | `[sp-36]` | `[sp-26]:1` | `0x142b0f` | `[sp-26]:1` |
| `bl` at 0xd526c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd5278 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd52a8 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd52b8 | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd52c4 | `(unknown callee) 0x21674` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd52ec | `(unknown callee) 0x2070c` | `0x13461c` | `ret(0x1b508)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd5180 | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xd51a0 | `[sp-24] != [0x2ad210]:1` | `[sp-24] == [0x2ad210]:1` |
| `bne` at 0xd51b4 | `[sp-25]:1 != [0x142b0c]:1` | `[sp-25]:1 == [0x142b0c]:1` |
| `bne` at 0xd51c8 | `[sp-26]:1 != [0x142b0d]:1` | `[sp-26]:1 == [0x142b0d]:1` |
| `beq` at 0xd51e4 | `[0x142b0c]:1 == 0xfe` | `[0x142b0c]:1 != 0xfe` |
| `beq` at 0xd51f4 | `[0x142b0d]:1 == 0x0` | `[0x142b0d]:1 != 0x0` |
| `beq` at 0xd5258 | `ret(0xd594c) == 0x0` | `ret(0xd594c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd532c | `r0` | `?` |

## `i2cMuxSelectSubRaw`

- `i2cMuxSelectSubRaw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd535c to 0xd5544.
  Established. The call to `0xd594c` carries `ret(0xd583c)`, `[sp-26]:1`, unknown, `[sp-26]:1` (`bl`
  at 0xd5438); its zero result takes the route that writes bytes `([sp-24]:1 & 0xff)` at `0x2ad211`,
  `([sp-25]:1 & 0xff)` at `0x142b0e`, and `([sp-26]:1 & 0xff)` at `0x142b0f` (`beq` at 0xd544c,
  `strb` at 0xd54f8, `strb` at 0xd5504, `strb` at 0xd5510). Nonzero falls through to the other arm,
  which includes `0x2070c` with `0x134650`, `ret(0x1b508)`, `[[sp-36]+2]:1`, `[sp-26]:1` (`beq` at
  0xd544c, `bl` at 0xd54e0). Subselector meaning and the merged return remain unproven (`ldmdb` at
  0xd5520).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd536c | store | 4 | `sp -0x18` | `r0` |
| `strb` at 0xd5374 | store | 1 | `sp -0x19` | `(r1 & 0xff)` |
| `strb` at 0xd537c | store | 1 | `sp -0x1a` | `(r2 & 0xff)` |
| `ldr` at 0xd5380 | load | 4 | `abs 0xd5524` | `-` |
| `ldr` at 0xd5384 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5398 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd53a0 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xd53a4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd53a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd53b8 | store | 4 | `sp -0x24` | `ret(0xd583c)` |
| `ldr` at 0xd53bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd53c0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd53c4 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0xd53cc | load | 1 | `([sp-25]:1 + [[sp-32]+8]) +0x0` | `-` |
| `ldr` at 0xd53d8 | load | 4 | `abs 0xd5528` | `-` |
| `str` at 0xd53dc | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xd53e4 | load | 4 | `abs 0xd552c` | `-` |
| `ldrb` at 0xd53e8 | load | 1 | `abs 0x2ad211` | `-` |
| `ldr` at 0xd53ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd53f8 | load | 4 | `abs 0xd5530` | `-` |
| `ldrb` at 0xd53fc | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd5400 | load | 1 | `abs 0x142b0e` | `-` |
| `ldr` at 0xd540c | load | 4 | `abs 0xd5534` | `-` |
| `ldrb` at 0xd5410 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd5414 | load | 1 | `abs 0x142b0f` | `-` |
| `str` at 0xd5424 | store | 4 | `sp -0x30` | `0x0` |
| `ldrb` at 0xd542c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd5430 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd5440 | store | 4 | `sp -0x28` | `ret(0xd594c)` |
| `ldr` at 0xd5444 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd5450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5458 | load | 4 | `abs 0xd5538` | `-` |
| `ldr` at 0xd545c | load | 4 | `abs 0xd553c` | `-` |
| `str` at 0xd5488 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd548c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd5494 | load | 4 | `abs 0xd5538` | `-` |
| `ldr` at 0xd5498 | load | 4 | `abs 0xd553c` | `-` |
| `ldr` at 0xd54a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd54b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd54c0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd54c4 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd54c8 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xd54cc | store | 4 | `(unknown)` | `ret(0x21674)` |
| `ldr` at 0xd54d0 | load | 4 | `abs 0xd5540` | `-` |
| `ldr` at 0xd54e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd54e8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd54f0 | load | 4 | `abs 0xd552c` | `-` |
| `ldrb` at 0xd54f4 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xd54f8 | store | 1 | `abs 0x2ad211` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0xd54fc | load | 4 | `abs 0xd5530` | `-` |
| `ldrb` at 0xd5500 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd5504 | store | 1 | `abs 0x142b0e` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0xd5508 | load | 4 | `abs 0xd5534` | `-` |
| `ldrb` at 0xd550c | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd5510 | store | 1 | `abs 0x142b0f` | `([sp-26]:1 & 0xff)` |
| `str` at 0xd5518 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd551c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd5520 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5520 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5520 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd53b0 | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd5438 | `(unknown callee) 0xd594c` | `ret(0xd583c)` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xd5460 | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd546c | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd549c | `(unknown callee) 0x4f564` | `[sp-24]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd54ac | `(unknown callee) 0x1b508` | `[sp-44]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd54b8 | `(unknown callee) 0x21674` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd54e0 | `(unknown callee) 0x2070c` | `0x134650` | `ret(0x1b508)` | `[[sp-36]+2]:1` | `[sp-26]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd53d4 | `[([sp-25]:1 + [[sp-32]+8])]:1 != 0x0` | `[([sp-25]:1 + [[sp-32]+8])]:1 == 0x0` |
| `bne` at 0xd53f4 | `[sp-24] != [0x2ad211]:1` | `[sp-24] == [0x2ad211]:1` |
| `bne` at 0xd5408 | `[sp-25]:1 != [0x142b0e]:1` | `[sp-25]:1 == [0x142b0e]:1` |
| `bne` at 0xd541c | `[sp-26]:1 != [0x142b0f]:1` | `[sp-26]:1 == [0x142b0f]:1` |
| `beq` at 0xd544c | `ret(0xd594c) == 0x0` | `ret(0xd594c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5520 | `r0` | `?` |

## `i2cMuxDeselectRaw`

- `i2cMuxDeselectRaw`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd5544 to 0xd5760.
  Established. The call to `0xd594c` uses `ret(0xd583c)`, `0x0`, `[sp-29]:1`, `[([sp-29]:1 +
  [[sp-36]+8])]:1` (`bl` at 0xd55cc); its zero result takes the continuation and nonzero falls
  through to the error arm (`beq` at 0xd55e0). Within that continuation, clearing byte
  `0x2ad210` and writing unknown bytes at `0x142b0c` and `0x142b0d` requires all three tests to
  fall through: `[[sp-40]+1]:1 == 0x0`, `[sp-28] == [0x2ad210]:1`, and
  `[sp-29]:1 == [0x142b0c]:1` (`bne` at 0xd56a8, 0xd56bc, 0xd56d0,
  `strb` at 0xd56dc, 0xd56e8, 0xd56f4). Taking any of those branches skips this first group
  (`bne` at 0xd56a8, 0xd56bc, 0xd56d0).
  Both routes then clear byte `0x2ad211` and write unknown bytes at `0x142b0e` and `0x142b0f`
  (`strb` at 0xd5700, 0xd570c, 0xd5718). Physical deselection and the final r0 value are unknown
  (`ldmdb` at 0xd5728).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd5554 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xd555c | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xd5560 | load | 4 | `abs 0xd572c` | `-` |
| `ldr` at 0xd5564 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5578 | load | 4 | `abs 0x2ad214` | `-` |
| `str` at 0xd5580 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0xd5584 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd5588 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd5598 | store | 4 | `sp -0x28` | `ret(0xd583c)` |
| `ldr` at 0xd559c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd55a0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd55a4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldrb` at 0xd55ac | load | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `-` |
| `ldr` at 0xd55b8 | load | 4 | `abs 0xd5730` | `-` |
| `str` at 0xd55bc | store | 4 | `sp -0x34` | `0xfffffc11` |
| `ldr` at 0xd55c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd55d4 | store | 4 | `sp -0x2c` | `ret(0xd594c)` |
| `ldr` at 0xd55d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd55e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd55ec | load | 4 | `abs 0xd5734` | `-` |
| `ldr` at 0xd55f0 | load | 4 | `abs 0xd5738` | `-` |
| `str` at 0xd561c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd5620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd5628 | load | 4 | `abs 0xd5734` | `-` |
| `ldr` at 0xd562c | load | 4 | `abs 0xd5738` | `-` |
| `ldr` at 0xd5638 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xd5648 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd564c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd565c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd5668 | load | 4 | `abs 0xd573c` | `-` |
| `str` at 0xd566c | store | 4 | `(unknown)` | `0x134688` |
| `str` at 0xd5670 | store | 4 | `(unknown)` | `ret(0x1b508)` |
| `str` at 0xd5674 | store | 4 | `(unknown)` | `ret(0xd4dbc)` |
| `str` at 0xd5678 | store | 4 | `(unknown)` | `ret(0x21674)` |
| `ldr` at 0xd5680 | load | 4 | `abs 0xd5740` | `-` |
| `ldr` at 0xd5684 | load | 4 | `abs 0xd5744` | `-` |
| `ldr` at 0xd5690 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd5694 | store | 4 | `sp -0x34` | `[sp-44]` |
| `ldr` at 0xd569c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xd56a0 | load | 1 | `[sp-40] +0x1` | `-` |
| `ldr` at 0xd56ac | load | 4 | `abs 0xd5748` | `-` |
| `ldrb` at 0xd56b0 | load | 1 | `abs 0x2ad210` | `-` |
| `ldr` at 0xd56b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd56c0 | load | 4 | `abs 0xd574c` | `-` |
| `ldrb` at 0xd56c4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd56c8 | load | 1 | `abs 0x142b0c` | `-` |
| `ldr` at 0xd56d4 | load | 4 | `abs 0xd5748` | `-` |
| `strb` at 0xd56dc | store | 1 | `abs 0x2ad210` | `0x0` |
| `ldr` at 0xd56e0 | load | 4 | `abs 0xd574c` | `-` |
| `strb` at 0xd56e8 | store | 1 | `abs 0x142b0c` | `?` |
| `ldr` at 0xd56ec | load | 4 | `abs 0xd5750` | `-` |
| `strb` at 0xd56f4 | store | 1 | `abs 0x142b0d` | `?` |
| `ldr` at 0xd56f8 | load | 4 | `abs 0xd5754` | `-` |
| `strb` at 0xd5700 | store | 1 | `abs 0x2ad211` | `0x0` |
| `ldr` at 0xd5704 | load | 4 | `abs 0xd5758` | `-` |
| `strb` at 0xd570c | store | 1 | `abs 0x142b0e` | `?` |
| `ldr` at 0xd5710 | load | 4 | `abs 0xd575c` | `-` |
| `strb` at 0xd5718 | store | 1 | `abs 0x142b0f` | `?` |
| `str` at 0xd5720 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd5724 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd5728 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xd5728 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd5728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd5728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd5728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd5590 | `(unknown callee) 0xd583c` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xd55cc | `(unknown callee) 0xd594c` | `ret(0xd583c)` | `0x0` | `[sp-29]:1` | `[([sp-29]:1 + [[sp-36]+8])]:1` |
| `bl` at 0xd55f4 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd5600 | `(unknown callee) 0x1b22c` | `ret(0x4f564)` | `?` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5630 | `(unknown callee) 0x4f564` | `[sp-28]` | `0x0` | `0x134554` | `0x13455c` |
| `bl` at 0xd5640 | `(unknown callee) 0x1b508` | `[sp-48]` | `ret(0x4f564)` | `?` | `ret(0x4f564)` |
| `bl` at 0xd5654 | `(unknown callee) 0xd4dbc` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xd5660 | `(unknown callee) 0x21674` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xd568c | `(unknown callee) 0x21e28` | `0x1` | `0x1344fc` | `0x1f7` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd55b4 | `[([sp-29]:1 + [[sp-36]+8])]:1 != 0x0` | `[([sp-29]:1 + [[sp-36]+8])]:1 == 0x0` |
| `beq` at 0xd55e0 | `ret(0xd594c) == 0x0` | `ret(0xd594c) != 0x0` |
| `bne` at 0xd56a8 | `[[sp-40]+1]:1 != 0x0` | `[[sp-40]+1]:1 == 0x0` |
| `bne` at 0xd56bc | `[sp-28] != [0x2ad210]:1` | `[sp-28] == [0x2ad210]:1` |
| `bne` at 0xd56d0 | `[sp-29]:1 != [0x142b0c]:1` | `[sp-29]:1 == [0x142b0c]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd5728 | `r0` | `?` |

## `uartWrite`

- `uartWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca3c0 to 0xca450.
  Established. The entry signed test takes the continuing edge for `r2 > 0x0` and falls through for
  less-or-equal (`bgt` at 0xca3e4). The reached call to `0x1afc8` carries `[r0+4]`, entry r1, entry
  r2, entry r0 (`bl` at 0xca404); its signed nonnegative result takes the continuation and a
  negative result falls through to `0xca51c` with `[sp-20]`, `0x2`, unknown, `ret(0x1afc8)` (`bge`
  at 0xca418, `bl` at 0xca424). Transmitted-data meaning and the joined r0 return remain unknown
  (`ldmdb` at 0xca44c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca3d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca3d4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca3d8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xca3dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca3ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xca3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca3f8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xca3fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca400 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca40c | store | 4 | `sp -0x20` | `ret(0x1afc8)` |
| `ldr` at 0xca410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca41c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca430 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xca438 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xca440 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca444 | store | 4 | `sp -0x24` | `ret(0x1afc8)` |
| `ldr` at 0xca448 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xca44c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca44c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca44c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca404 | `(unknown callee) 0x1afc8` | `[r0+4]` | `r1` | `r2` | `r0` |
| `bl` at 0xca424 | `(unknown callee) 0xca51c` | `[sp-20]` | `0x2` | `?` | `ret(0x1afc8)` |
| `bl` at 0xca428 | `(unknown callee) 0x1b0b8` | `ret(0xca51c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xca3e4 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bge` at 0xca418 | `ret(0x1afc8) >= 0x0 (signed, 32-bit)` | `ret(0x1afc8) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca44c | `r0` | `?` |

## `uartSetConfig`

- `uartSetConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca48c to 0xca51c.
  Established. The first test takes its bypass for `[r1+22]:1 != 0x1` and falls through on equality
  to store byte `0x2` at entry `r1 +0x16` (`bne` at 0xca4b0, `strb` at 0xca4bc). Two later byte
  stores also write `0x2`, but their effective targets are unknown (`strb` at 0xca4d8, `strb` at
  0xca4f4). The two multiple-store instructions contain four and three word stores, respectively,
  all with unknown targets and values (`stm` at 0xca50c, `stm` at 0xca514). The configuration
  encoding and returned r0 value are not established (`ldmdb` at 0xca518).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca49c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca4a0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xca4a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca4a8 | load | 1 | `r1 +0x16` | `-` |
| `ldr` at 0xca4b4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xca4bc | store | 1 | `r1 +0x16` | `0x2` |
| `ldr` at 0xca4c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca4c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xca4d0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xca4d8 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xca4dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xca4e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xca4ec | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xca4f4 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0xca4f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca4fc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xca508 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xca508 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xca508 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xca508 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xca50c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xca50c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xca50c | store | 4 | `(unknown)` | `?` |
| `stm` at 0xca50c | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xca510 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xca510 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xca510 | load | 4 | `(unknown)` | `-` |
| `stm` at 0xca514 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xca514 | store | 4 | `(unknown)` | `?` |
| `stm` at 0xca514 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xca518 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca518 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca518 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca4b0 | `[r1+22]:1 != 0x1` | `[r1+22]:1 == 0x1` |
| `bne` at 0xca4cc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca4e8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca518 | `r0` | `?` |

## `uartGetConfig`

- `uartGetConfig`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca450 to 0xca48c.
  Established. The first multiple-store writes words `[r0+16]`, `[r0+20]`, `[r0+24]`, `[r0+28]` at
  entry `r1 +0x0`, `+0x4`, `+0x8`, `+0xc`, respectively (`stm` at 0xca47c). The next writes
  `[r0+32]`, `[r0+36]`, `[r0+40]` at entry `r1 +0x10`, `+0x14`, `+0x18` (`stm` at 0xca484). The r0
  return is `[r0+32]` (`ldmdb` at 0xca488). Calls and conditional branches are absent, and the
  copied fields have unknown meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca460 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca464 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xca468 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca46c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xca478 | load | 4 | `r0 +0x10` | `-` |
| `ldm` at 0xca478 | load | 4 | `r0 +0x14` | `-` |
| `ldm` at 0xca478 | load | 4 | `r0 +0x18` | `-` |
| `ldm` at 0xca478 | load | 4 | `r0 +0x1c` | `-` |
| `stm` at 0xca47c | store | 4 | `r1 +0x0` | `[r0+16]` |
| `stm` at 0xca47c | store | 4 | `r1 +0x4` | `[r0+20]` |
| `stm` at 0xca47c | store | 4 | `r1 +0x8` | `[r0+24]` |
| `stm` at 0xca47c | store | 4 | `r1 +0xc` | `[r0+28]` |
| `ldm` at 0xca480 | load | 4 | `r0 +0x20` | `-` |
| `ldm` at 0xca480 | load | 4 | `r0 +0x24` | `-` |
| `ldm` at 0xca480 | load | 4 | `r0 +0x28` | `-` |
| `stm` at 0xca484 | store | 4 | `r1 +0x10` | `[r0+32]` |
| `stm` at 0xca484 | store | 4 | `r1 +0x14` | `[r0+36]` |
| `stm` at 0xca484 | store | 4 | `r1 +0x18` | `[r0+40]` |
| `ldmdb` at 0xca488 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca488 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca488 | `r0` | `[r0+32]` |

## `uartFlush`

- `uartFlush`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca51c to 0xca5c4.
  Established. The call to `0x1b40c` carries `[sp-32]`, `0x80047502`, entry r1, `[sp-32]` (`bl` at
  0xca574); its signed nonnegative result takes the exit route and a negative result falls through
  to `0x1b0b8` (`bge` at 0xca580, `bl` at 0xca584). The later call to `0x24028` has
  `[0x144dc8]`, `0x1336f4`, `[[sp-20]+8]`, `ret(0x21674)` (`bl` at 0xca5b0). Physical flush behavior
  and the merged r0 value remain unknown (`ldmdb` at 0xca5b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca52c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca530 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xca534 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca548 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xca54c | store | 4 | `sp -0x20` | `[r0+4]` |
| `ldr` at 0xca554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca558 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xca55c | store | 4 | `sp -0x20` | `[r0]` |
| `ldr` at 0xca560 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca564 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xca568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xca56c | load | 4 | `abs 0xca5b8` | `-` |
| `ldr` at 0xca570 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca58c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xca598 | load | 4 | `abs 0xca5bc` | `-` |
| `ldr` at 0xca59c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca5a0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xca5a4 | load | 4 | `abs 0xca5c0` | `-` |
| `ldr` at 0xca5a8 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xca5b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca5b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca5b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca574 | `(unknown callee) 0x1b40c` | `[sp-32]` | `0x80047502` | `r1` | `[sp-32]` |
| `bl` at 0xca584 | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xca590 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xca5b0 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1336f4` | `[[sp-20]+8]` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xca540 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xca580 | `ret(0x1b40c) >= 0x0 (signed, 32-bit)` | `ret(0x1b40c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca5b4 | `r0` | `?` |

## `uartSendBreak`

- `uartSendBreak`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca5c4 to 0xca63c.
  Established. A word read addresses entry `r0 +0x4` before the call to `0x1b40c` (`ldr` at
  0xca5e0). That call has `[r0+4]`, `0x80047504`, `0x0`, entry r0 (`bl` at 0xca5ec). The signed
  result takes its exit route when nonnegative and falls through on negative to `0x1b0b8` (`bge`
  at 0xca5f8, `bl` at 0xca5fc). Break timing, serial-line effects and the joined return are unknown
  (`ldmdb` at 0xca62c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca5d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca5d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xca5dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca5e0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xca5e4 | load | 4 | `abs 0xca630` | `-` |
| `ldr` at 0xca604 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xca610 | load | 4 | `abs 0xca634` | `-` |
| `ldr` at 0xca614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca618 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xca61c | load | 4 | `abs 0xca638` | `-` |
| `ldr` at 0xca620 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldmdb` at 0xca62c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca62c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca62c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca5ec | `(unknown callee) 0x1b40c` | `[r0+4]` | `0x80047504` | `0x0` | `r0` |
| `bl` at 0xca5fc | `(unknown callee) 0x1b0b8` | `ret(0x1b40c)` | `?` | `?` | `ret(0x1b40c)` |
| `bl` at 0xca608 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xca628 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x13371c` | `[[sp-20]+8]` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xca5f8 | `ret(0x1b40c) >= 0x0 (signed, 32-bit)` | `ret(0x1b40c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca62c | `r0` | `?` |

## `uartShowStatus`

- `uartShowStatus`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca63c to 0xca738.
  Established. The call to `0xca738` carries entry r0 through r3, and its nonzero result takes the
  continuation while zero falls through (`bl` at 0xca65c, `bne` at 0xca670). Later word stores save
  `[[sp-44]+156]:2` at `sp -0x48`, `[[sp-44]+158]:2` at `sp -0x44`, `[[sp-44]+160]:2` at `sp -0x40`,
  and `[[sp-44]+260]:2` at `sp -0x34` (`str` at 0xca6dc, `str` at 0xca6e8, `str` at 0xca6f4, `str`
  at 0xca70c). The final call to `0x1b49c` has `[sp-40]`, `0x133748`, `[sp-36]`, `[[sp-44]+44]`
  (`bl` at 0xca720). Status-field units remain unknown, and the return reports `[sp-48]` (`ldmdb` at
  0xca730).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca64c | store | 4 | `sp -0x20` | `r0` |
| `str` at 0xca650 | store | 4 | `sp -0x24` | `r1` |
| `str` at 0xca654 | store | 4 | `sp -0x28` | `r2` |
| `ldr` at 0xca658 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca664 | store | 4 | `sp -0x2c` | `ret(0xca738)` |
| `ldr` at 0xca668 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xca678 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca680 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca694 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca698 | load | 4 | `[sp-44] +0x98` | `-` |
| `ldr` at 0xca6a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca6b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca6bc | load | 4 | `[sp-44] +0x100` | `-` |
| `ldr` at 0xca6c8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xca6cc | store | 4 | `sp -0x50` | `ret(0xca7ec)` |
| `str` at 0xca6d0 | store | 4 | `sp -0x4c` | `ret(0xca810)` |
| `ldr` at 0xca6d4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xca6d8 | load | 2 | `[sp-44] +0x9c` | `-` |
| `str` at 0xca6dc | store | 4 | `sp -0x48` | `[[sp-44]+156]:2` |
| `ldr` at 0xca6e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xca6e4 | load | 2 | `[sp-44] +0x9e` | `-` |
| `str` at 0xca6e8 | store | 4 | `sp -0x44` | `[[sp-44]+158]:2` |
| `ldr` at 0xca6ec | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xca6f0 | load | 2 | `[sp-44] +0xa0` | `-` |
| `str` at 0xca6f4 | store | 4 | `sp -0x40` | `[[sp-44]+160]:2` |
| `str` at 0xca6f8 | store | 4 | `sp -0x3c` | `ret(0xca7ec)` |
| `str` at 0xca6fc | store | 4 | `sp -0x38` | `ret(0xca810)` |
| `ldr` at 0xca700 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xca708 | load | 2 | `[sp-44] +0x104` | `-` |
| `str` at 0xca70c | store | 4 | `sp -0x34` | `[[sp-44]+260]:2` |
| `ldr` at 0xca710 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xca714 | load | 4 | `abs 0xca734` | `-` |
| `ldr` at 0xca718 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xca71c | load | 4 | `[sp-44] +0x2c` | `-` |
| `str` at 0xca728 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xca72c | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca65c | `(unknown callee) 0xca738` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xca68c | `(unknown callee) 0xca7ec` | `ret(0xca738)+60` | `?` | `?` | `ret(0xca738)+60` |
| `bl` at 0xca69c | `(unknown callee) 0xca810` | `[[sp-44]+152]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xca6b0 | `(unknown callee) 0xca7ec` | `[sp-44]+164` | `?` | `?` | `[sp-44]+164` |
| `bl` at 0xca6c0 | `(unknown callee) 0xca810` | `[[sp-44]+256]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xca720 | `(unknown callee) 0x1b49c` | `[sp-40]` | `0x133748` | `[sp-36]` | `[[sp-44]+44]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xca670 | `ret(0xca738) != 0x0` | `ret(0xca738) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca730 | `r0` | `[sp-48]` |

## `nvramRead`

- `nvramRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20b38 to 0x20ba8.
  Established. The signed range branch takes its continuation for `(r0 + r2) <= 0x1fff` and falls
  through for greater values (`ble` at 0x20b68). The fall-through arm writes an unknown word at `sp
  -0x1c` (`str` at 0x20b74). The call to `0x1b304` has four unknown candidates (`bl` at 0x20b90).
  The r0 return is `[sp-28]`, while the physical NVRAM mapping and copy extent remain unknown
  (`ldmdb` at 0x20b9c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20b48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20b4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x20b50 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x20b54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20b58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20b60 | load | 4 | `abs 0x20ba0` | `-` |
| `ldr` at 0x20b6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20b74 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x20b78 | load | 4 | `abs 0x20ba4` | `-` |
| `ldr` at 0x20b7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20b84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20b8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20b94 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x20b9c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20b9c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20b9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20b90 | `(unknown callee) 0x1b304` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x20b68 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20b9c | `r0` | `[sp-28]` |

## `nvramWrite`

- `nvramWrite`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20ba8 to 0x20c18.
  Established. The bound takes the continuation for `(r0 + r2) <= 0x1fff` signed and falls through
  above it (`ble` at 0x20bd8). The fall-through word store at `sp -0x1c` has an unknown value (`str`
  at 0x20be4). The only call reaches `0x1b304` with four unknown candidates (`bl` at 0x20c00). The
  return retains `[sp-28]` (`ldmdb` at 0x20c0c). The write destination and persistence guarantees
  remain unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20bb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x20bbc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x20bc0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x20bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20bc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20bd0 | load | 4 | `abs 0x20c10` | `-` |
| `ldr` at 0x20bdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x20be4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x20be8 | load | 4 | `abs 0x20c14` | `-` |
| `ldr` at 0x20bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x20bf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x20c04 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x20c0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20c0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20c0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20c00 | `(unknown callee) 0x1b304` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x20bd8 | `(r0 + r2) <= 0x1fff (signed, 32-bit)` | `(r0 + r2) > 0x1fff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20c0c | `r0` | `[sp-28]` |
