# ARM UART and power entry paths in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/uart-power-entry-paths-facts.txt) retain all tool sections.

## `uartInit`

- `uartInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc99c4 to 0xc9b90.
  Established. The absolute word at `0x2b21c0` is incremented by storing `[0x2b21c0]+1` (`str` at
  0xc9a78). The call to `0x1b43c` uses `[sp-32]`, `0x0`, `0x108`, `[0x2b21c0]+1` (`bl` at 0xc9a88).
  Word stores save `[sp-20]` at `[sp-32] +0x8`, `[sp-24]` at `[sp-32] +0xc`, and `[sp-28]` at
  `[sp-32] +0x2c` (`str` at 0xc9a94, `str` at 0xc9aa0, `str` at 0xc9aac). The r0 return is
  `[sp-32]` (`ldmdb` at 0xc9b70). Device initialization and indexed layout remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc99d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc99d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc99dc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xc99e0 | load | 4 | `abs 0xc9b74` | `-` |
| `ldr` at 0xc99e4 | load | 4 | `abs 0x2b21c0` | `-` |
| `ldr` at 0xc99f0 | load | 4 | `abs 0xc9b78` | `-` |
| `str` at 0xc99f4 | store | 4 | `sp -0x2c` | `0x133658` |
| `ldr` at 0xc99fc | load | 4 | `abs 0xc9b7c` | `-` |
| `ldr` at 0xc9a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9a20 | load | 4 | `abs 0xc9b80` | `-` |
| `str` at 0xc9a24 | store | 4 | `sp -0x2c` | `0x133684` |
| `ldr` at 0xc9a28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9a2c | store | 4 | `sp -0x28` | `[sp-20]` |
| `ldr` at 0xc9a34 | load | 4 | `abs 0xc9b7c` | `-` |
| `ldr` at 0xc9a44 | load | 4 | `abs 0xc9b74` | `-` |
| `ldr` at 0xc9a48 | load | 4 | `abs 0x2b21c0` | `-` |
| `ldr` at 0xc9a5c | load | 4 | `abs 0xc9b84` | `-` |
| `str` at 0xc9a64 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc9a68 | load | 4 | `abs 0xc9b74` | `-` |
| `ldr` at 0xc9a6c | load | 4 | `abs 0xc9b74` | `-` |
| `ldr` at 0xc9a70 | load | 4 | `abs 0x2b21c0` | `-` |
| `str` at 0xc9a78 | store | 4 | `abs 0x2b21c0` | `[0x2b21c0]+1` |
| `ldr` at 0xc9a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9a90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc9a94 | store | 4 | `[sp-32] +0x8` | `[sp-20]` |
| `ldr` at 0xc9a98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9a9c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc9aa0 | store | 4 | `[sp-32] +0xc` | `[sp-24]` |
| `ldr` at 0xc9aa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9aa8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9aac | store | 4 | `[sp-32] +0x2c` | `[sp-28]` |
| `ldr` at 0xc9ab0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9ab8 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0xc9abc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9ac4 | store | 4 | `[sp-32] +0x4` | `?` |
| `ldr` at 0xc9ac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9ad4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9ae0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9ae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9ae8 | load | 4 | `abs 0xc9b88` | `-` |
| `strb` at 0xc9af8 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xc9b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9b18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9b1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9b20 | load | 4 | `abs 0xc9b8c` | `-` |
| `strb` at 0xc9b30 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xc9b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9b44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc9b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc9b54 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xc9b64 | store | 1 | `[sp-32] +0x31` | `?` |
| `ldr` at 0xc9b68 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xc9b70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9b70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9b70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9a08 | `(unknown callee) 0x21e28` | `0x9` | `0x13367c` | `0x58` | `0x0` |
| `bl` at 0xc9a10 | `(unknown callee) 0xca738` | `?` | `?` | `?` | `?` |
| `bl` at 0xc9a40 | `(unknown callee) 0x21e28` | `0x9` | `0x13367c` | `0x5b` | `0x0` |
| `bl` at 0xc9a88 | `(unknown callee) 0x1b43c` | `[sp-32]` | `0x0` | `0x108` | `[0x2b21c0]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc99ec | `[0x2b21c0] <= 0x13 (signed, 32-bit)` | `[0x2b21c0] > 0x13 (signed, 32-bit)` |
| `beq` at 0xc9a1c | `ret(0xca738) == 0x0` | `ret(0xca738) != 0x0` |
| `bls` at 0xc9ad0 | `[sp-20] <= 0x4 (unsigned, 32-bit)` | `[sp-20] > 0x4 (unsigned, 32-bit)` |
| `bhi` at 0xc9adc | `[sp-20] > 0xc (unsigned, 32-bit)` | `[sp-20] <= 0xc (unsigned, 32-bit)` |
| `bls` at 0xc9b08 | `[sp-20] <= 0xc (unsigned, 32-bit)` | `[sp-20] > 0xc (unsigned, 32-bit)` |
| `bhi` at 0xc9b14 | `[sp-20] > 0x10 (unsigned, 32-bit)` | `[sp-20] <= 0x10 (unsigned, 32-bit)` |
| `bls` at 0xc9b40 | `[sp-20] <= 0x10 (unsigned, 32-bit)` | `[sp-20] > 0x10 (unsigned, 32-bit)` |
| `bhi` at 0xc9b4c | `[sp-20] > 0x16 (unsigned, 32-bit)` | `[sp-20] <= 0x16 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9b70 | `r0` | `[sp-32]` |

## `uartOpen`

- `uartOpen`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc9b90 to 0xca008.
  Established. The first result test takes the continuation for `ret(0xca738) != 0x0` and falls
  through on equality to store word `0x0` at `sp -0x34` (`bne` at 0xc9bc8, `str` at 0xc9bd0). Later
  calls to `0x1b40c` carry `[sp-60]`, `0x80047503`, `0x400`, `[sp-60]` and `[sp-64]`, `0x80047506`,
  `0x1`, `[sp-64]`, respectively (`bl` at 0xc9f3c, `bl` at 0xc9fbc). Each signed result test takes
  its continuation for `ret(0x1b40c) >= 0x0` and falls through for less (`bge` at 0xc9f48, `bge` at
  0xc9fc8). External opening behavior and the joined r0 value remain unknown (`ldmdb` at 0xc9fec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc9ba0 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xc9ba4 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xc9bac | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc9bb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc9bbc | store | 4 | `sp -0x24` | `ret(0xca738)` |
| `ldr` at 0xc9bc0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9bd0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xc9bd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9be0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc9be4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9bf0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9bfc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9c0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc9c1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c20 | load | 4 | `ret(0xca738) +0x34` | `-` |
| `ldr` at 0xc9c2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c38 | load | 4 | `ret(0xca738) +0x34` | `-` |
| `str` at 0xc9c48 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9c50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9c58 | store | 4 | `(unknown)` | `ret(0x22698)` |
| `str` at 0xc9c60 | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xc9c68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c6c | load | 4 | `ret(0xca738) +0x38` | `-` |
| `ldr` at 0xc9c78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9c84 | load | 4 | `ret(0xca738) +0x38` | `-` |
| `str` at 0xc9c94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9c9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9ca4 | store | 4 | `(unknown)` | `ret(0x22698)` |
| `str` at 0xc9cac | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xc9cb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9cb8 | load | 4 | `ret(0xca738) +0x34` | `-` |
| `ldr` at 0xc9cc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9cc8 | load | 4 | `ret(0xca738) +0x38` | `-` |
| `ldr` at 0xc9cd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9ce4 | load | 4 | `ret(0xca738) +0x34` | `-` |
| `ldr` at 0xc9cf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9cfc | load | 4 | `[sp-36] +0x38` | `-` |
| `str` at 0xc9d10 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9d18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9d1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9d24 | store | 4 | `(unknown)` | `ret(0x22698)` |
| `ldr` at 0xc9d28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9d2c | store | 4 | `(unknown)` | `?` |
| `str` at 0xc9d34 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xc9d38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9d44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9d50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9d54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9d60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9d64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9d70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9d74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9d84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9d8c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc9d90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9d98 | store | 4 | `[sp-36] +0x4` | `0x1` |
| `ldr` at 0xc9d9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9da0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc9db0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9dbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9dc8 | load | 4 | `abs 0xc9ff0` | `-` |
| `ldr` at 0xc9dcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc9dd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9dd4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9de0 | store | 4 | `sp -0x30` | `ret(0x1b370)` |
| `ldr` at 0xc9de4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9df8 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc9e0c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc9e24 | load | 4 | `abs 0xc9ff4` | `-` |
| `str` at 0xc9e28 | store | 4 | `sp -0x50` | `0x1336a8` |
| `ldr` at 0xc9e2c | load | 4 | `abs 0xc9ff8` | `-` |
| `str` at 0xc9e30 | store | 4 | `sp -0x4c` | `0x1336c4` |
| `str` at 0xc9e34 | store | 4 | `sp -0x48` | `ret(0x21674)` |
| `ldr` at 0xc9e38 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xc9e3c | store | 4 | `sp -0x44` | `[ret(0x1b0b8)]` |
| `ldr` at 0xc9e44 | load | 4 | `abs 0xc9ffc` | `-` |
| `str` at 0xc9e58 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9e5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9e70 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xc9e7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9e88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9e98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9e9c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc9ea0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xc9ea4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9eb0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9ec0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9ec4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc9ec8 | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xc9ecc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9ed8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9edc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9ee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc9efc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9f10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9f14 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc9f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9f20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9f24 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xc9f28 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xc9f2c | store | 4 | `sp -0x30` | `[sp-60]` |
| `ldr` at 0xc9f30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9f34 | load | 4 | `abs 0xca000` | `-` |
| `ldr` at 0xc9f4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9f50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9f5c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9f60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9f6c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc9f70 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc9f7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc9f8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9f90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9f94 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc9f9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9fa0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc9fa4 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xc9fa8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xc9fac | store | 4 | `sp -0x30` | `[sp-64]` |
| `ldr` at 0xc9fb0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc9fb4 | load | 4 | `abs 0xca004` | `-` |
| `ldr` at 0xc9fcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc9fd0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc9fdc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc9fe0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc9fe4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc9fe8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xc9fec | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc9fec | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc9fec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc9fec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc9fec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc9bb4 | `(unknown callee) 0xca738` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xc9c30 | `(unknown callee) 0x22698` | `ret(0xca738)` | `?` | `?` | `[ret(0xca738)+52]` |
| `bl` at 0xc9c54 | `(unknown callee) 0x22698` | `?` | `?` | `?` | `[ret(0xca738)+52]` |
| `bl` at 0xc9c7c | `(unknown callee) 0x22698` | `ret(0xca738)` | `?` | `?` | `[ret(0xca738)+56]` |
| `bl` at 0xc9ca0 | `(unknown callee) 0x22698` | `?` | `?` | `?` | `[ret(0xca738)+56]` |
| `bl` at 0xc9cdc | `(unknown callee) 0x22698` | `ret(0xca738)` | `?` | `?` | `?` |
| `bl` at 0xc9cf4 | `(unknown callee) 0x22698` | `ret(0x22698)` | `?` | `ret(0x22698)` | `[ret(0xca738)+52]` |
| `bl` at 0xc9d20 | `(unknown callee) 0x22698` | `?` | `?` | `?` | `?` |
| `bl` at 0xc9da8 | `(unknown callee) 0xca834` | `[[sp-36]]` | `0xf` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xc9dd8 | `(unknown callee) 0x1b370` | `?` | `[sp-32]` | `0x142a04` | `[sp-28]` |
| `bl` at 0xc9df0 | `(unknown callee) 0x1b0b8` | `ret(0x1b370)` | `?` | `?` | `ret(0x1b370)` |
| `bl` at 0xc9e04 | `(unknown callee) 0x1b0b8` | `ret(0x1b0b8)` | `?` | `?` | `[ret(0x1b0b8)]` |
| `bl` at 0xc9e10 | `(unknown callee) 0x21674` | `[ret(0x1b0b8)]` | `?` | `?` | `ret(0x1b0b8)` |
| `bl` at 0xc9e18 | `(unknown callee) 0x1b0b8` | `ret(0x21674)` | `?` | `?` | `?` |
| `bl` at 0xc9e50 | `(unknown callee) 0x21e28` | `0x3` | `0x13367c` | `0xcf` | `0x0` |
| `bl` at 0xc9e68 | `(unknown callee) 0x1b0b8` | `?` | `?` | `?` | `?` |
| `bl` at 0xc9f3c | `(unknown callee) 0x1b40c` | `[sp-60]` | `0x80047503` | `0x400` | `[sp-60]` |
| `bl` at 0xc9f54 | `(unknown callee) 0xca008` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x1b40c)` |
| `bl` at 0xc9fbc | `(unknown callee) 0x1b40c` | `[sp-64]` | `0x80047506` | `0x1` | `[sp-64]` |
| `bl` at 0xc9fd4 | `(unknown callee) 0xca008` | `[sp-36]` | `[sp-32]` | `?` | `ret(0x1b40c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc9bc8 | `ret(0xca738) != 0x0` | `ret(0xca738) == 0x0` |
| `beq` at 0xc9bec | `[sp-56] == 0x1` | `[sp-56] != 0x1` |
| `bgt` at 0xc9bf8 | `[sp-56] > 0x1 (signed, 32-bit)` | `[sp-56] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xc9c04 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0xc9c14 | `[sp-56] == 0x2` | `[sp-56] != 0x2` |
| `beq` at 0xc9c28 | `[ret(0xca738)+52] == 0x0` | `[ret(0xca738)+52] != 0x0` |
| `beq` at 0xc9c40 | `[ret(0xca738)+52] == ret(0x22698)` | `[ret(0xca738)+52] != ret(0x22698)` |
| `beq` at 0xc9c74 | `[ret(0xca738)+56] == 0x0` | `[ret(0xca738)+56] != 0x0` |
| `beq` at 0xc9c8c | `[ret(0xca738)+56] == ret(0x22698)` | `[ret(0xca738)+56] != ret(0x22698)` |
| `bne` at 0xc9cc0 | `[ret(0xca738)+52] != 0x0` | `[ret(0xca738)+52] == 0x0` |
| `bne` at 0xc9cd0 | `[ret(0xca738)+56] != 0x0` | `[ret(0xca738)+56] == 0x0` |
| `bne` at 0xc9cec | `[ret(0xca738)+52] != ret(0x22698)` | `[ret(0xca738)+52] == ret(0x22698)` |
| `bne` at 0xc9d04 | `[[sp-36]+56] != ret(0x22698)` | `[[sp-36]+56] == ret(0x22698)` |
| `beq` at 0xc9d40 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc9d4c | `[sp-28] != 0x1` | `[sp-28] == 0x1` |
| `bge` at 0xc9d5c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xc9d6c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc9d7c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xc9db8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9dc4 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bge` at 0xc9dec | `ret(0x1b370) >= 0x0 (signed, 32-bit)` | `ret(0x1b370) < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9e00 | `[ret(0x1b0b8)] == 0x4` | `[ret(0x1b0b8)] != 0x4` |
| `bge` at 0xc9e64 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xc9e78 | `[ret(0x1b0b8)] == 0x4` | `[ret(0x1b0b8)] != 0x4` |
| `beq` at 0xc9e84 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xc9e94 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9eac | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xc9ebc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc9ed4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9ee4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9ef4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9f08 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xc9f48 | `ret(0x1b40c) >= 0x0 (signed, 32-bit)` | `ret(0x1b40c) < 0x0 (signed, 32-bit)` |
| `beq` at 0xc9f68 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9f78 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc9f88 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xc9fc8 | `ret(0x1b40c) >= 0x0 (signed, 32-bit)` | `ret(0x1b40c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc9fec | `r0` | `?` |

## `uartClose`

- `uartClose`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca008 to 0xca200.
  Established. One arm stores word `0x0` at `[sp-24] +0x34` (`str` at 0xca0a0). Another clears
  `[sp-24] +0x38` with the same width and value (`str` at 0xca0dc). The equality branch takes its
  continuation for `[r0+52] == ret(0x22698)` and falls through on inequality (`beq` at 0xca088).
  Later calls to `0x1b004` have all four candidates unknown (`bl` at 0xca1bc, `bl` at 0xca1e4).
  Physical close effects and the r0 return are unproven (`ldmdb` at 0xca1fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca018 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xca01c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xca024 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xca02c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xca030 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca038 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xca03c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca048 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca054 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xca074 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca080 | load | 4 | `r0 +0x34` | `-` |
| `str` at 0xca090 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xca098 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca0a0 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `str` at 0xca0a8 | store | 4 | `sp -0x2c` | `0x4` |
| `ldr` at 0xca0b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca0bc | load | 4 | `r0 +0x38` | `-` |
| `str` at 0xca0cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xca0d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca0dc | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xca0e4 | store | 4 | `sp -0x2c` | `0x2` |
| `ldr` at 0xca0ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca0f8 | load | 4 | `r0 +0x34` | `-` |
| `ldr` at 0xca104 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca110 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0xca124 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xca12c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca134 | store | 4 | `[sp-24] +0x34` | `0x0` |
| `ldr` at 0xca138 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca140 | store | 4 | `[sp-24] +0x38` | `0x0` |
| `str` at 0xca148 | store | 4 | `sp -0x2c` | `0x6` |
| `ldr` at 0xca14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca150 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca15c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca160 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca16c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca17c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca180 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca18c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca194 | store | 4 | `[sp-24] +0x4` | `?` |
| `str` at 0xca19c | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xca1a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca1b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca1b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca1c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca1c8 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xca1cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xca1dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca1e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca1e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xca1f0 | store | 4 | `[sp-24] +0x4` | `?` |
| `ldr` at 0xca1f4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xca1fc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xca1fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca1fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca1fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca078 | `(unknown callee) 0x22698` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xca0b4 | `(unknown callee) 0x22698` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xca0f0 | `(unknown callee) 0x22698` | `r0` | `r1` | `r2` | `[sp-48]` |
| `bl` at 0xca108 | `(unknown callee) 0x22698` | `ret(0x22698)` | `?` | `ret(0x22698)` | `[r0+52]` |
| `bl` at 0xca184 | `(unknown callee) 0xca978` | `?` | `?` | `?` | `?` |
| `bl` at 0xca1bc | `(unknown callee) 0x1b004` | `?` | `?` | `?` | `?` |
| `bl` at 0xca1e4 | `(unknown callee) 0x1b004` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xca044 | `[sp-48] == 0x1` | `[sp-48] != 0x1` |
| `bgt` at 0xca050 | `[sp-48] > 0x1 (signed, 32-bit)` | `[sp-48] <= 0x1 (signed, 32-bit)` |
| `beq` at 0xca05c | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0xca06c | `[sp-48] == 0x2` | `[sp-48] != 0x2` |
| `beq` at 0xca088 | `[r0+52] == ret(0x22698)` | `[r0+52] != ret(0x22698)` |
| `beq` at 0xca0c4 | `[r0+56] == ret(0x22698)` | `[r0+56] != ret(0x22698)` |
| `bne` at 0xca100 | `[r0+52] != ret(0x22698)` | `[r0+52] == ret(0x22698)` |
| `bne` at 0xca118 | `[[sp-24]+56] != ret(0x22698)` | `[[sp-24]+56] == ret(0x22698)` |
| `bne` at 0xca158 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca168 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xca178 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xca1b0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xca1d8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca1fc | `r0` | `?` |

## `uartRead`

- `uartRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xca200 to 0xca3c0.
  Established. The entry guard takes the work arm for `r2 > 0x0` signed and falls through for less
  or equal (`bgt` at 0xca224). A call to `0x1b064` has `[[sp-20]]+1`, `sp-160`, `0x0`, `0x0` (`bl`
  at 0xca2f4). The later call to `0x1b4fc` uses `[[sp-20]]`, `[sp-24]`, `[sp-28]`, `[sp-20]` (`bl`
  at 0xca330); its signed result test takes the final arm for `[sp-32] >= 0x0` and falls through for
  less (`bge` at 0xca384). External input semantics and the joined return remain unknown (`ldmdb` at
  0xca3b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xca210 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xca214 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xca218 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xca21c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca22c | store | 4 | `sp -0xac` | `0x0` |
| `str` at 0xca238 | store | 4 | `sp -0xa8` | `sp-160` |
| `str` at 0xca240 | store | 4 | `sp -0xa4` | `0x0` |
| `ldr` at 0xca244 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0xca254 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0xca258 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xca260 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xca264 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xca26c | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xca274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca278 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xca2b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca2b4 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xca2c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xca2cc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xca2d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca2d4 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xca2e4 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0xca30c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xca314 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xca320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca324 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xca328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xca32c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xca338 | store | 4 | `sp -0x20` | `ret(0x1b4fc)` |
| `ldr` at 0xca33c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca350 | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `ldr` at 0xca360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca368 | load | 4 | `abs 0xca3bc` | `-` |
| `ldr` at 0xca36c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xca370 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xca37c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xca388 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xca39c | load | 4 | `ret(0x1b0b8) +0x0` | `-` |
| `str` at 0xca3a4 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0xca3ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xca3b0 | store | 4 | `sp -0xac` | `[sp-32]` |
| `ldr` at 0xca3b4 | load | 4 | `sp -0xac` | `-` |
| `ldmdb` at 0xca3b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xca3b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xca3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xca2f4 | `(unknown callee) 0x1b064` | `[[sp-20]]+1` | `sp-160` | `0x0` | `0x0` |
| `bl` at 0xca304 | `(unknown callee) 0x1b0b8` | `ret(0x1b064)` | `?` | `?` | `ret(0x1b064)` |
| `bl` at 0xca330 | `(unknown callee) 0x1b4fc` | `[[sp-20]]` | `[sp-24]` | `[sp-28]` | `[sp-20]` |
| `bl` at 0xca348 | `(unknown callee) 0x1b0b8` | `ret(0x1b4fc)` | `?` | `?` | `ret(0x1b4fc)` |
| `bl` at 0xca374 | `(unknown callee) 0x2070c` | `0x1336d0` | `[[sp-20]+8]` | `[[sp-20]]` | `[sp-20]` |
| `bl` at 0xca390 | `(unknown callee) 0xca51c` | `[sp-20]` | `0x1` | `?` | `[sp-32]` |
| `bl` at 0xca394 | `(unknown callee) 0x1b0b8` | `ret(0xca51c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xca224 | `r2 > 0x0 (signed, 32-bit)` | `r2 <= 0x0 (signed, 32-bit)` |
| `bls` at 0xca24c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bge` at 0xca300 | `ret(0x1b064) >= 0x0 (signed, 32-bit)` | `ret(0x1b064) < 0x0 (signed, 32-bit)` |
| `bne` at 0xca344 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xca358 | `[ret(0x1b0b8)] == 0x23` | `[ret(0x1b0b8)] != 0x23` |
| `bge` at 0xca384 | `[sp-32] >= 0x0 (signed, 32-bit)` | `[sp-32] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xca3b8 | `r0` | `?` |

## `vsc055_Int`

- `vsc055_Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd94a8 to 0xd9580.
  Established. The only reported call reaches `0xd9580` with all four candidates unknown (`bl` at
  0xd954c); its result is accumulated by a word store of `([sp-36] + ret(0xd9580))` at `sp -0x24`
  (`str` at 0xd955c). The earlier indexed byte read has unknown effective target (`ldrb` at
  0xd9524). The subsequent bypass branch retains only `Z == 1` for its taken edge and falls through
  otherwise (`beq` at 0xd952c). Interrupt-source identity and the returned r0 value remain unknown
  (`ldmdb` at 0xd9578).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd94b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd94c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd94c8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xd94cc | load | 4 | `abs 0xd957c` | `-` |
| `ldr` at 0xd94d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94e4 | load | 4 | `abs 0x142b10` | `-` |
| `str` at 0xd94ec | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xd94f4 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xd94f8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd94fc | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldrb` at 0xd9500 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd9504 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9518 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd951c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9524 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd9530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9534 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd953c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xd9544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9554 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd955c | store | 4 | `sp -0x24` | `([sp-36] + ret(0xd9580))` |
| `ldr` at 0xd9560 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9568 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd9570 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd9578 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9578 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd954c | `(unknown callee) 0xd9580` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xd950c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xd952c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9578 | `r0` | `?` |

## `powerGetMarginValue`

- `powerGetMarginValue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40e18 to 0x40e50.
  Established. A byte read addresses entry `r1 +0x10` (`ldrb` at 0x40e34). The call to `0x1f1dc` has
  `[r1+16]:1+130`, entry r1, entry r2, `[r1+16]:1+130` (`bl` at 0x40e40); its result supplies r0 at
  the return (`ldmdb` at 0x40e4c). There are no conditional branches. Margin units and the
  referenced storage remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40e28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40e2c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40e30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x40e34 | load | 1 | `r1 +0x10` | `-` |
| `ldmdb` at 0x40e4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40e4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40e4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40e40 | `(unknown callee) 0x1f1dc` | `[r1+16]:1+130` | `r1` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40e4c | `r0` | `ret(0x1f1dc)` |

## `powerSetMarginValue`

- `powerSetMarginValue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40e50 to 0x40e88.
  Established. The call to `0x1d38c` uses `[r1+16]:1+130`, entry r2, entry r2, `[r1+16]:1+130` (`bl`
  at 0x40e80). The prior byte read addresses entry `r1 +0x10` (`ldrb` at 0x40e70). The returned
  r0 value is `ret(0x1d38c)` (`ldmdb` at 0x40e84). No conditional branches are reported, and a
  physical voltage change is not proved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40e60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40e64 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x40e68 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x40e6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x40e70 | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x40e7c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40e84 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40e84 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40e84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40e80 | `(unknown callee) 0x1d38c` | `[r1+16]:1+130` | `r2` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40e84 | `r0` | `ret(0x1d38c)` |

## `powerGetMarginState`

- `powerGetMarginState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40e88 to 0x40ec0.
  Established. The byte at entry `r1 +0x10` is read before the only call (`ldrb` at 0x40ea4). That
  call reaches `0x1f1dc` with `[r1+16]:1+20`, entry r1, entry r2, `[r1+16]:1+20` (`bl` at 0x40eb0);
  its result remains in r0 at the return (`ldmdb` at 0x40ebc). The state encoding and physical
  margin interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40e98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40e9c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x40ea0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x40ea4 | load | 1 | `r1 +0x10` | `-` |
| `ldmdb` at 0x40ebc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40ebc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40ebc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40eb0 | `(unknown callee) 0x1f1dc` | `[r1+16]:1+20` | `r1` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40ebc | `r0` | `ret(0x1f1dc)` |

## `pcixPowerUp`

- `pcixPowerUp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x589c8 to 0x58ac0.
  Established. The absolute-word test takes the work arm for `[0x148360] != 0x0` and falls through
  on equality to store word `0x0` at `sp -0x28` (`bne` at 0x589f0, `str` at 0x589f8). The loop call
  targets `0x59d18` with `[sp-20]` and three unknown candidates (`bl` at 0x58a6c); its zero-result
  branch takes the bypass for `ret(0x59d18) == 0x0` and falls through otherwise (`beq` at 0x58a80).
  Power-up ordering and the joined r0 value remain unproven (`ldmdb` at 0x58ab4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x589d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x589e0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x589e4 | load | 4 | `abs 0x58ab8` | `-` |
| `ldr` at 0x589e8 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x589f8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58a00 | load | 4 | `abs 0x58abc` | `-` |
| `ldr` at 0x58a04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58a18 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x58a20 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x58a28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x58a2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58a30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58a34 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x58a44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58a48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58a5c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x58a64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58a74 | store | 4 | `sp -0x20` | `ret(0x59d18)` |
| `ldr` at 0x58a78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58a84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58a90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x58a94 | store | 4 | `sp -0x1c` | `ret(0x59d18)` |
| `ldr` at 0x58a98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58aa0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x58aa8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58aac | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x58ab0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x58ab4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58ab4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58ab4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58a6c | `(unknown callee) 0x59d18` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x589f0 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `blt` at 0x58a3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58a80 | `ret(0x59d18) == 0x0` | `ret(0x59d18) != 0x0` |
| `bne` at 0x58a8c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58ab4 | `r0` | `?` |

## `pcixPowerDown`

- `pcixPowerDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58ac0 to 0x58bb8.
  Established. A word read addresses `0x148360` before the nonzero test (`ldr` at 0x58ae0). That
  branch takes the work arm for `[0x148360] != 0x0` and falls through on equality (`bne` at
  0x58ae8). The call to `0x5a3e8` has `[sp-20]`, unknown, unknown, unknown (`bl` at 0x58b64). A
  later word store advances `sp -0x24` with `[sp-36]+1` (`str` at 0x58b98). Shutdown effects and the
  r0 result remain unknown (`ldmdb` at 0x58bac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58ad0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x58ad8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x58adc | load | 4 | `abs 0x58bb0` | `-` |
| `ldr` at 0x58ae0 | load | 4 | `abs 0x148360` | `-` |
| `str` at 0x58af0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58af8 | load | 4 | `abs 0x58bb4` | `-` |
| `ldr` at 0x58afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58b10 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x58b18 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x58b20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x58b24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58b28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58b2c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x58b3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x58b40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58b54 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x58b5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58b6c | store | 4 | `sp -0x20` | `ret(0x5a3e8)` |
| `ldr` at 0x58b70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58b7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58b88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x58b8c | store | 4 | `sp -0x1c` | `ret(0x5a3e8)` |
| `ldr` at 0x58b90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58b98 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x58ba0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x58ba4 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x58ba8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x58bac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58bac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58bac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58b64 | `(unknown callee) 0x5a3e8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58ae8 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `blt` at 0x58b34 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58b78 | `ret(0x5a3e8) == 0x0` | `ret(0x5a3e8) != 0x0` |
| `bne` at 0x58b84 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58bac | `r0` | `?` |

## `IP57_PowerUp`

- `IP57_PowerUp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x601e0 to 0x602e8.
  Established. The initial call to `0x60f28` carries `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x60210). A zero result takes the loop path and nonzero falls through (`beq` at 0x60224).
  The later call to `0xd6d40` has `0x0`, unknown, `0x80`, unknown (`bl` at 0x6029c). A byte store
  advances `sp -0x21` with `([sp-33]:1+1 & 0xff)` (`strb` at 0x602d0). Board power behavior and the
  merged r0 value remain unknown (`ldmdb` at 0x602e4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x601f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x601fc | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x60200 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60218 | store | 4 | `sp -0x20` | `ret(0x60f28)` |
| `ldr` at 0x6021c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60228 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6022c | store | 4 | `sp -0x28` | `ret(0x60f28)` |
| `strb` at 0x60238 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x6023c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x6024c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60258 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x60278 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x6027c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x60280 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x602a4 | store | 4 | `sp -0x20` | `ret(0xd6d40)` |
| `ldr` at 0x602a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x602b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x602c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x602c4 | store | 4 | `sp -0x1c` | `ret(0xd6d40)` |
| `ldrb` at 0x602c8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x602d0 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x602d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x602dc | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x602e0 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x602e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x602e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x602e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60210 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60268 | `(unknown callee) 0x5d8c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x6029c | `(unknown callee) 0xd6d40` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60224 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `bls` at 0x60244 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x60274 | `ret(0x5d8c0) == 0x0` | `ret(0x5d8c0) != 0x0` |
| `beq` at 0x602b0 | `ret(0xd6d40) == 0x0` | `ret(0xd6d40) != 0x0` |
| `bne` at 0x602bc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x602e4 | `r0` | `?` |

## `IP57_PowerDown`

- `IP57_PowerDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x602e8 to 0x603c4.
  Established. The call to `0x60f28` has `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)` (`bl` at
  0x60318); its result test takes the loop path on zero and falls through on nonzero (`beq` at
  0x6032c). The loop calls `0xd6cf8` with `0x0`, unknown, `0x80`, unknown (`bl` at 0x60378). A
  nonzero result can be saved as word `ret(0xd6cf8)` at `sp -0x1c` after the zero and saved-result
  tests (`beq` at 0x6038c, `bne` at 0x60398, `str` at 0x603a0). Physical power-down effects and the
  returned r0 value are unproven (`ldmdb` at 0x603c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x602fc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x60304 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x60308 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60320 | store | 4 | `sp -0x20` | `ret(0x60f28)` |
| `ldr` at 0x60324 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60330 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60334 | store | 4 | `sp -0x28` | `ret(0x60f28)` |
| `strb` at 0x60340 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x60344 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x60354 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60358 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x6035c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x60380 | store | 4 | `sp -0x20` | `ret(0xd6cf8)` |
| `ldr` at 0x60384 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6039c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x603a0 | store | 4 | `sp -0x1c` | `ret(0xd6cf8)` |
| `ldrb` at 0x603a4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x603ac | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x603b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x603b8 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x603bc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x603c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x603c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x603c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60318 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60378 | `(unknown callee) 0xd6cf8` | `0x0` | `?` | `0x80` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6032c | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `bls` at 0x6034c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x6038c | `ret(0xd6cf8) == 0x0` | `ret(0xd6cf8) != 0x0` |
| `bne` at 0x60398 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x603c0 | `r0` | `?` |

## `IP65_PowerUp`

- `IP65_PowerUp`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6475c to 0x647b8.
  Established. The only call targets `0x64978` with `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x64784); its zero result takes the arm that writes word `0x0` at `sp -0x20`, while
  nonzero falls through to save word `ret(0x64978)` there (`beq` at 0x64798, `str` at 0x647ac, `str`
  at 0x647a0). The joined r0 return is `[sp-32]` (`ldmdb` at 0x647b4). No physical power transition
  follows from this body alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64770 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x64774 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6478c | store | 4 | `sp -0x1c` | `ret(0x64978)` |
| `ldr` at 0x64790 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6479c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x647a0 | store | 4 | `sp -0x20` | `ret(0x64978)` |
| `str` at 0x647ac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x647b0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x647b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x647b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x647b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64784 | `(unknown callee) 0x64978` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64798 | `ret(0x64978) == 0x0` | `ret(0x64978) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x647b4 | `r0` | `[sp-32]` |

## `IP65_PowerDown`

- `IP65_PowerDown`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x647b8 to 0x64814.
  Established. Entry r0 is saved as byte `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x647cc). The call
  to `0x64978` uses `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)` (`bl` at 0x647e0). Zero takes the
  arm that stores word `0x0` at `sp -0x20` (`beq` at 0x647f4, `str` at 0x64808). Nonzero falls
  through to store word `ret(0x64978)` there (`str` at 0x647fc). The r0 return is `[sp-32]`, and external
  power-down behavior remains unknown (`ldmdb` at 0x64810).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x647cc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x647d0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x647e8 | store | 4 | `sp -0x1c` | `ret(0x64978)` |
| `ldr` at 0x647ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x647f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x647fc | store | 4 | `sp -0x20` | `ret(0x64978)` |
| `str` at 0x64808 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x6480c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x64810 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64810 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x647e0 | `(unknown callee) 0x64978` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x647f4 | `ret(0x64978) == 0x0` | `ret(0x64978) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64810 | `r0` | `[sp-32]` |

## `L1CfgEventPowerState`

- `L1CfgEventPowerState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f64 to 0x74fbc.
  Established. The comparison takes the return path for `[sp-20] == [ret(0x74908)+52]` and falls
  through on inequality (`beq` at 0x74f98). Fall-through stores word `[sp-20]` at `ret(0x74908)
  +0x34` (`str` at 0x74fa4). It then calls `0x74c78` with `0x4`, `0x0`, `0x0`, `[sp-20]` (`bl` at
  0x74fb4). Event delivery, physical state meaning and the joined r0 value remain unknown (`ldmdb`
  at 0x74fb8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74f74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74f84 | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0x74f88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74f8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74f90 | load | 4 | `ret(0x74908) +0x34` | `-` |
| `ldr` at 0x74f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74fa0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74fa4 | store | 4 | `ret(0x74908) +0x34` | `[sp-20]` |
| `ldmdb` at 0x74fb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74fb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74fb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74f7c | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x74fb4 | `(unknown callee) 0x74c78` | `0x4` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74f98 | `[sp-20] == [ret(0x74908)+52]` | `[sp-20] != [ret(0x74908)+52]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74fb8 | `r0` | `?` |
