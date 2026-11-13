# ARM boot, scan and character I/O helpers in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/boot-character-io-facts.txt) retain all tool sections.

## `pkr`

- `pkr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57a08 to 0x57a8c.
  Established. The indirect call target is unknown and carries entry `r0`, `r1`, `r2`, `r3` (`mov` at 0x57a44).
  Included word reads address `[sp-28] +0x0` and `[sp-32] +0x0` (`ldr` at 0x57a68, `ldr` at 0x57a6c).
  The unnamed `0x1e958` call carries `0xf9500`, `[sp-24]`, `[[sp-28]]`, `[[sp-32]]` (`bl` at 0x57a70).
  Read callback peripheral scope remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57a18 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x57a1c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x57a20 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x57a24 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x57a28 | load | 4 | `abs 0x57a84` | `-` |
| `ldr` at 0x57a2c | load | 4 | `abs 0x299854` | `-` |
| `ldr` at 0x57a30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57a3c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57a4c | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x57a50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57a54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57a58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57a5c | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x57a60 | load | 4 | `abs 0x57a88` | `-` |
| `ldr` at 0x57a64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57a68 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x57a6c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x57a74 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x57a80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x57a80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x57a80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x57a44 | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x57a70 | `(unknown callee) 0x1e958` | `0xf9500` | `[sp-24]` | `[[sp-28]]` | `[[sp-32]]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x57a80 | `r0` | `[sp-36]` |

## `pkw`

- `pkw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x57a8c to 0x57b08.
  Established. A word read addresses `0x299850` (`ldr` at 0x57ab0).
  The indirect call has unknown target with entry `r0`, `r1`, `r2`, `r3` candidates (`mov` at 0x57ac8).
  The unnamed `0x1e958` call carries `0xf9510`, `[sp-24]`, `[sp-28]`, `[sp-32]` (`bl` at 0x57aec).
  Write callback device ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x57a9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x57aa0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x57aa4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x57aa8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x57aac | load | 4 | `abs 0x57b00` | `-` |
| `ldr` at 0x57ab0 | load | 4 | `abs 0x299850` | `-` |
| `ldr` at 0x57ab4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x57ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57ac0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57ad0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x57ad4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x57ad8 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x57adc | load | 4 | `abs 0x57b04` | `-` |
| `ldr` at 0x57ae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x57ae4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x57ae8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57af0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x57afc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x57afc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x57afc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x57ac8 | `(unknown callee) (unknown target)` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x57aec | `(unknown callee) 0x1e958` | `0xf9510` | `[sp-24]` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x57afc | `r0` | `[sp-36]` |

## `sn1sc_asm_drive_tck`

- `sn1sc_asm_drive_tck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8ae4 to 0xe8b64.
  Established. Included halfword stores write unknown values to `[0x29c464] +0x0` (`strh` at 0xe8b40, `strh` at 0xe8b4c).
  The conditional return is selected when `0x0 >= [r0+4]` unsigned, with lower falling through (`pophs` at 0xe8b34).
  A loop branch takes `0xe8b3c` when `C == 0`, with the complement falling through (`blo` at 0xe8b58).
  Clock-line bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe8ae4 | store | 4 | `sp -0x4` | `lr` |
| `ldm` at 0xe8ae8 | load | 4 | `r0 +0x0` | `-` |
| `ldm` at 0xe8ae8 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0xe8b38 | load | 4 | `abs 0xe8b60` | `-` |
| `ldr` at 0xe8b3c | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xe8b40 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8b48 | load | 4 | `abs 0x29c464` | `-` |
| `strh` at 0xe8b4c | store | 2 | `[0x29c464] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `pophs` at 0xe8b34 | `0x0 >= [r0+4] (unsigned, 32-bit)` | `0x0 < [r0+4] (unsigned, 32-bit)` |
| `blo` at 0xe8b58 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `pophs` at 0xe8b34 | `r0` | `0x0` |
| `pop` at 0xe8b5c | `r0` | `?` |

## `sn1sc_dr_load`

- `sn1sc_dr_load`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8d30 to 0xd9010.
  Established. A word read addresses `r0 +0x10` (`ldr` at 0xd8d50);
  its zero value takes `0xd8df0`, while nonzero falls through (`beq` at 0xd8d60).
  Included unnamed `0xd0d10` calls carry `0x0`, unknown, `0x1`, unknown and `0x1`, `0x6`, `0x1`, unknown (`bl` at 0xd8dcc, `bl` at 0xd8fc8).
  Scan load packet semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8d40 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd8d48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8d50 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xd8d54 | store | 4 | `sp -0x20` | `[r0+16]` |
| `ldr` at 0xd8d58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8d64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8d6c | store | 4 | `sp -0x28` | `r0+24` |
| `ldr` at 0xd8d74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8d7c | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xd8d8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8d94 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xd8da0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8da4 | store | 4 | `sp -0x44` | `[r0+16]` |
| `ldr` at 0xd8da8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8dac | store | 4 | `sp -0x40` | `r0+24` |
| `ldr` at 0xd8db0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8db4 | store | 4 | `sp -0x3c` | `r0+24` |
| `ldr` at 0xd8db8 | load | 4 | `abs 0xd900c` | `-` |
| `str` at 0xd8dbc | store | 4 | `sp -0x38` | `0x29c468` |
| `str` at 0xd8dd4 | store | 4 | `sp -0x18` | `ret(0xd0d10)` |
| `ldr` at 0xd8dd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd8de8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd8df4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8dfc | load | 4 | `(unknown)` | `-` |
| `str` at 0xd8e00 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd8e04 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8e14 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd8e1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8e20 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd8e38 | store | 4 | `sp -0x1c` | `0x6` |
| `str` at 0xd8e44 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xd8e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd8e58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8e60 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd8e64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd8e6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8e74 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd8e78 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd8e80 | store | 4 | `sp -0x34` | `[sp-52]+24` |
| `ldr` at 0xd8e88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8e90 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd8e94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd8e9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8ea4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd8ea8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd8eb0 | store | 4 | `sp -0x34` | `[sp-52]+28` |
| `ldr` at 0xd8eb4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd8eb8 | store | 4 | `sp -0x28` | `[sp-52]` |
| `ldr` at 0xd8ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8ec8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd8ed4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8ed8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xd8edc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8ee0 | store | 4 | `sp -0x40` | `[sp-52]` |
| `ldr` at 0xd8ee4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8ee8 | store | 4 | `sp -0x3c` | `[sp-52]` |
| `ldr` at 0xd8eec | load | 4 | `abs 0xd900c` | `-` |
| `str` at 0xd8ef0 | store | 4 | `sp -0x38` | `0x29c468` |
| `ldr` at 0xd8efc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd8f0c | store | 4 | `sp -0x18` | `ret(0xd0d10)` |
| `ldr` at 0xd8f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8f14 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0xd8f2c | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd8f34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8f4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8f68 | store | 4 | `sp -0x2c` | `ret(0x1943c)` |
| `ldr` at 0xd8f6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd8f78 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8f7c | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xd8f84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8f8c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xd8f98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8f9c | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xd8fa0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8fa4 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0xd8fa8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd8fac | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xd8fb0 | load | 4 | `abs 0xd900c` | `-` |
| `str` at 0xd8fb4 | store | 4 | `sp -0x38` | `0x29c468` |
| `str` at 0xd8fd0 | store | 4 | `sp -0x18` | `ret(0xd0d10)` |
| `ldr` at 0xd8fd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd8fd8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8fe8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd8ff0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd8ff8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd8ffc | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0xd9000 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xd9008 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9008 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8dcc | `(unknown callee) 0xd0d10` | `0x0` | `?` | `0x1` | `?` |
| `bl` at 0xd8f04 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `[sp-28]` | `?` |
| `bl` at 0xd8f60 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xd8fc8 | `(unknown callee) 0xd0d10` | `0x1` | `0x6` | `0x1` | `?` |
| `bl` at 0xd8ff4 | `(unknown callee) 0x1970c` | `[sp-44]` | `?` | `[sp-44]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd8d60 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bne` at 0xd8de0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd8e0c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xd8e30 | `? != 0x1` | `? == 0x1` |
| `bne` at 0xd8e54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xd8f24 | `? == 0x1` | `? != 0x1` |
| `bne` at 0xd8f74 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xd8fe0 | `[sp-44] != [sp-40]` | `[sp-44] == [sp-40]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9008 | `r0` | `?` |

## `bootGetHeaderPtr`

- `bootGetHeaderPtr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8db4 to 0xb8ea0.
  Established. A nonzero `ret(0xb8ad8)` takes `0xb8de8`, while zero falls through (`bne` at 0xb8dd8).
  The unnamed `0xb8bf8` call carries `ret(0x19280)`, `ret(0xbe724)`, unknown, `ret(0xbe724)` (`bl` at 0xb8e38).
  An included word read addresses `[sp-36] +0x0` (`ldr` at 0xb8e14).
  Header pointer ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8dc4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xb8dc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8de0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb8de8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8df4 | store | 4 | `sp -0x24` | `ret(0xbe850)` |
| `ldr` at 0xb8df8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb8e08 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb8e10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8e14 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xb8e24 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8e3c | store | 4 | `sp -0x20` | `ret(0xb8bf8)` |
| `ldr` at 0xb8e40 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb8e58 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb8e60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8e74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb8e7c | store | 4 | `sp -0x30` | `ret(0xbe850)` |
| `str` at 0xb8e88 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb8e8c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb8e90 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb8e94 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xb8e9c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8e9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8e9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8e9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8dcc | `(unknown callee) 0xb8ad8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb8dec | `(unknown callee) 0xbe850` | `[sp-24]` | `?` | `?` | `ret(0xb8ad8)` |
| `bl` at 0xb8e18 | `(unknown callee) 0x19280` | `[[sp-36]]` | `?` | `?` | `[sp-36]` |
| `bl` at 0xb8e28 | `(unknown callee) 0xbe724` | `[sp-24]` | `?` | `?` | `ret(0x19280)` |
| `bl` at 0xb8e38 | `(unknown callee) 0xb8bf8` | `ret(0x19280)` | `ret(0xbe724)` | `?` | `ret(0xbe724)` |
| `bl` at 0xb8e44 | `(unknown callee) 0xb8ad8` | `ret(0xb8bf8)` | `?` | `?` | `?` |
| `bl` at 0xb8e64 | `(unknown callee) 0xbe850` | `[sp-32]` | `?` | `?` | `ret(0xb8ad8)` |
| `bl` at 0xb8e78 | `(unknown callee) 0xbe850` | `[sp-32]` | `?` | `?` | `ret(0xbe850)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8dd8 | `ret(0xb8ad8) != 0x0` | `ret(0xb8ad8) == 0x0` |
| `bne` at 0xb8e00 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb8e50 | `ret(0xb8ad8) != 0x0` | `ret(0xb8ad8) == 0x0` |
| `beq` at 0xb8e70 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8e9c | `r0` | `?` |

## `bootGetHeader`

- `bootGetHeader`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8ea0 to 0xb9050.
  Established. Included word stores write `ret(0xbe850)` to `[sp-28] +0x10` and `[sp-28] +0x2c` (`str` at 0xb8f80, `str` at 0xb9010).
  Included halfword stores write `([[sp-28]]:2 & 0xffff)` to `[sp-28] +0x2` and `([sp-42]:2 & 0xffff)` to `[sp-28] +0x0` (`strh` at 0xb902c, `strh` at 0xb9038).
  Header field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8eb0 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb8eb4 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xb8ebc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb8ec0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8ecc | store | 4 | `sp -0x28` | `ret(0xbe724)` |
| `ldr` at 0xb8ed0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8edc | store | 4 | `sp -0x20` | `ret(0xb8db4)` |
| `ldr` at 0xb8ee0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb8ef0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb8ef8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8efc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb8f0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb8f10 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8f1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8f24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8f2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8f3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8f44 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8f4c | store | 4 | `(unknown)` | `ret(0x19280)` |
| `ldr` at 0xb8f50 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb8f58 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xb8f60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8f68 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xb8f6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8f80 | store | 4 | `[sp-28] +0x10` | `ret(0xbe850)` |
| `ldr` at 0xb8f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8f88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8f8c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xb8f90 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8fa4 | store | 4 | `[sp-28] +0x14` | `ret(0xbe850)` |
| `ldr` at 0xb8fa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8fac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8fb0 | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xb8fb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8fc8 | store | 4 | `[sp-28] +0x18` | `ret(0xbe850)` |
| `ldr` at 0xb8fcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8fd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8fd4 | load | 4 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xb8fd8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8fec | store | 4 | `[sp-28] +0x24` | `ret(0xbe850)` |
| `ldr` at 0xb8ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8ff8 | load | 4 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xb8ffc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb9010 | store | 4 | `[sp-28] +0x2c` | `ret(0xbe850)` |
| `ldr` at 0xb9014 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb9018 | load | 2 | `[sp-28] +0x2` | `-` |
| `strh` at 0xb901c | store | 2 | `sp -0x2a` | `([[sp-28]+2]:2 & 0xffff)` |
| `ldr` at 0xb9020 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb9024 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb9028 | load | 2 | `[sp-28] +0x0` | `-` |
| `strh` at 0xb902c | store | 2 | `[sp-28] +0x2` | `([[sp-28]]:2 & 0xffff)` |
| `ldr` at 0xb9030 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb9034 | load | 2 | `sp -0x2a` | `-` |
| `strh` at 0xb9038 | store | 2 | `[sp-28] +0x0` | `([sp-42]:2 & 0xffff)` |
| `ldr` at 0xb903c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb9040 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0xb9044 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xb904c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb904c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb904c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb904c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8ec4 | `(unknown callee) 0xbe724` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xb8ed4 | `(unknown callee) 0xb8db4` | `[sp-24]` | `?` | `?` | `ret(0xbe724)` |
| `bl` at 0xb8f04 | `(unknown callee) 0x19610` | `[sp-28]` | `ret(0xb8db4)` | `0x30` | `ret(0xb8db4)` |
| `bl` at 0xb8f30 | `(unknown callee) 0x19280` | `?` | `?` | `?` | `?` |
| `bl` at 0xb8f70 | `(unknown callee) 0xb8bf8` | `[[sp-28]+16]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xb8f7c | `(unknown callee) 0xbe850` | `ret(0xb8bf8)` | `?` | `?` | `ret(0xb8bf8)` |
| `bl` at 0xb8f94 | `(unknown callee) 0xb8bf8` | `[[sp-28]+20]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xb8fa0 | `(unknown callee) 0xbe850` | `ret(0xb8bf8)` | `?` | `?` | `ret(0xb8bf8)` |
| `bl` at 0xb8fb8 | `(unknown callee) 0xb8bf8` | `[[sp-28]+24]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xb8fc4 | `(unknown callee) 0xbe850` | `ret(0xb8bf8)` | `?` | `?` | `ret(0xb8bf8)` |
| `bl` at 0xb8fdc | `(unknown callee) 0xb8bf8` | `[[sp-28]+36]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xb8fe8 | `(unknown callee) 0xbe850` | `ret(0xb8bf8)` | `?` | `?` | `ret(0xb8bf8)` |
| `bl` at 0xb9000 | `(unknown callee) 0xb8bf8` | `[[sp-28]+44]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xb900c | `(unknown callee) 0xbe850` | `ret(0xb8bf8)` | `?` | `?` | `ret(0xb8bf8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8ee8 | `ret(0xb8db4) != 0x0` | `ret(0xb8db4) == 0x0` |
| `bhi` at 0xb8f18 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb904c | `r0` | `?` |

## `bootAddressIsValid`

- `bootAddressIsValid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8ad8 to 0xb8bf8.
  Established. Included word reads address `0x29c398`, `0x29c3a0` and `0x29c3a4` (`ldr` at 0xb8b0c, `ldr` at 0xb8b44, `ldr` at 0xb8b94).
  The unsigned condition `[0x29c398] > r0` takes `0xb8b40`, while lower or equal falls through (`bhi` at 0xb8b18).
  Boot address region ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8ae8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb8aec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8b00 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb8b08 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b0c | load | 4 | `abs 0x29c398` | `-` |
| `ldr` at 0xb8b10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8b1c | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b20 | load | 4 | `abs 0x29c398` | `-` |
| `ldr` at 0xb8b28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8b38 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xb8b40 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b44 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0xb8b50 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b54 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0xb8b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8b64 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b68 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b6c | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0xb8b70 | load | 4 | `abs 0x29c3b0` | `-` |
| `ldr` at 0xb8b78 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8b88 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xb8b90 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8b94 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0xb8ba0 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8ba4 | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0xb8ba8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8bb4 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8bb8 | load | 4 | `abs 0xb8bf4` | `-` |
| `ldr` at 0xb8bbc | load | 4 | `abs 0x29c3a4` | `-` |
| `ldr` at 0xb8bc0 | load | 4 | `abs 0x29c3b0` | `-` |
| `ldr` at 0xb8bc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8bd8 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xb8be4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb8be8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xb8bf0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8bf0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8bf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8af8 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0xb8b18 | `[0x29c398] > r0 (unsigned, 32-bit)` | `[0x29c398] <= r0 (unsigned, 32-bit)` |
| `bls` at 0xb8b30 | `[0x29c398]+524288 <= r0 (unsigned, 32-bit)` | `[0x29c398]+524288 > r0 (unsigned, 32-bit)` |
| `beq` at 0xb8b4c | `[0x29c3a0] == 0x0` | `[0x29c3a0] != 0x0` |
| `bhi` at 0xb8b60 | `[0x29c3a0] > r0 (unsigned, 32-bit)` | `[0x29c3a0] <= r0 (unsigned, 32-bit)` |
| `bls` at 0xb8b80 | `([0x29c3a0] + [0x29c3b0]) <= r0 (unsigned, 32-bit)` | `([0x29c3a0] + [0x29c3b0]) > r0 (unsigned, 32-bit)` |
| `beq` at 0xb8b9c | `[0x29c3a4] == 0x0` | `[0x29c3a4] != 0x0` |
| `bhi` at 0xb8bb0 | `[0x29c3a4] > r0 (unsigned, 32-bit)` | `[0x29c3a4] <= r0 (unsigned, 32-bit)` |
| `bls` at 0xb8bd0 | `([0x29c3a4] + [0x29c3b0]) <= r0 (unsigned, 32-bit)` | `([0x29c3a4] + [0x29c3b0]) > r0 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8bf0 | `r0` | `?` |

## `bootAddressNormalize`

- `bootAddressNormalize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8bf8 to 0xb8c60.
  Established. A word assignment writes an unknown value to `sp -0x1c` (`str` at 0xb8c18).
  Equality `[sp-28] == 0x1` takes `0xb8c38`, while inequality falls through (`beq` at 0xb8c24).
  A later word assignment stores an unknown value at `sp -0x14` (`str` at 0xb8c4c).
  Normalized address interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8c0c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8c10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8c18 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb8c1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8c28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8c4c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xb8c50 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8c24 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `beq` at 0xb8c30 | `[sp-28] == 0x2` | `[sp-28] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8c5c | `r0` | `?` |

## `bootValidate`

- `bootValidate`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8c60 to 0xb8db4.
  Established. A halfword read addresses `[sp-40] +0x2` (`ldrh` at 0xb8ccc).
  The unsigned condition `[[sp-40]+2]:2 <= 0x1f` takes `0xb8cec`, while greater falls through (`bls` at 0xb8cd4).
  The later unsigned condition `[[sp-40]+2]:2 > 0x400` takes `0xb8cec`, while lower or equal falls through (`bhi` at 0xb8ce4).
  Boot record validation policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8c74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8c78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8c88 | store | 4 | `sp -0x18` | `sp-92` |
| `ldr` at 0xb8c8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8c9c | store | 4 | `sp -0x28` | `ret(0xb8ea0)` |
| `ldr` at 0xb8ca0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8cb0 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xb8cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8cc4 | store | 4 | `sp -0x2c` | `ret(0xb8db4)` |
| `ldr` at 0xb8cc8 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb8ccc | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xb8cd8 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb8cdc | load | 2 | `[sp-40] +0x2` | `-` |
| `str` at 0xb8cf0 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xb8cf8 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb8cfc | load | 2 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xb8d08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb8d10 | store | 4 | `sp -0x20` | `ret(0xb8db4)+4` |
| `ldr` at 0xb8d18 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb8d1c | store | 4 | `sp -0x20` | `ret(0xb8db4)` |
| `ldr` at 0xb8d20 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb8d24 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xb8d28 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb8d30 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xb8d38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb8d3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8d40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8d50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8d54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8d5c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb8d70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8d78 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x19280))` |
| `ldr` at 0xb8d80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8d90 | store | 4 | `sp -0x68` | `0x0` |
| `str` at 0xb8d9c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xb8da0 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xb8da4 | store | 4 | `sp -0x64` | `[sp-104]` |
| `ldr` at 0xb8da8 | load | 4 | `sp -0x64` | `-` |
| `ldm` at 0xb8db0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8db0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8db0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8c94 | `(unknown callee) 0xb8ea0` | `r0` | `?` | `r2` | `?` |
| `bl` at 0xb8cbc | `(unknown callee) 0xb8db4` | `[sp-20]` | `?` | `?` | `ret(0xb8ea0)` |
| `bl` at 0xb8d64 | `(unknown callee) 0x19280` | `?` | `?` | `sp-32` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb8c80 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xb8ca8 | `ret(0xb8ea0) != 0x0` | `ret(0xb8ea0) == 0x0` |
| `bls` at 0xb8cd4 | `[[sp-40]+2]:2 <= 0x1f (unsigned, 32-bit)` | `[[sp-40]+2]:2 > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xb8ce4 | `[[sp-40]+2]:2 > 0x400 (unsigned, 32-bit)` | `[[sp-40]+2]:2 <= 0x400 (unsigned, 32-bit)` |
| `bhi` at 0xb8d04 | `[[sp-40]]:2 > 0x2 (unsigned, 32-bit)` | `[[sp-40]]:2 <= 0x2 (unsigned, 32-bit)` |
| `bhs` at 0xb8d48 | `C == 1` | `not (C == 1)` |
| `bne` at 0xb8d88 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8db0 | `r0` | `[sp-100]` |

## `bootDefaultImage`

- `bootDefaultImage`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb9050 to 0xb9280.
  Established. The unnamed `0xb8c60` call carries `[sp-96]`, `sp-88`, `[0x29c3b4]`, `sp-88` (`bl` at 0xb913c).
  A zero result takes `0xb9150`, while nonzero falls through (`beq` at 0xb9148).
  The unsigned comparison `[[sp-100]]:1 >= [[sp-104]]:1` takes `0xb91e4`, while lower falls through (`bhs` at 0xb91dc).
  Default image ranking policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb9060 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb9068 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xb9070 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xb9078 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb907c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9088 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb9090 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xb9094 | load | 4 | `abs 0xb927c` | `-` |
| `ldr` at 0xb9098 | load | 4 | `abs 0x29c3b4` | `-` |
| `str` at 0xb909c | store | 4 | `sp -0x24` | `[0x29c3b4]` |
| `ldr` at 0xb90a0 | load | 4 | `abs 0xb927c` | `-` |
| `ldr` at 0xb90a4 | load | 4 | `abs 0x29c39c` | `-` |
| `ldr` at 0xb90a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb90b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb90b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb90c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb90cc | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0xb90d4 | load | 4 | `abs 0xb927c` | `-` |
| `ldr` at 0xb90d8 | load | 4 | `abs 0x29c39c` | `-` |
| `ldr` at 0xb90dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb90ec | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xb90f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb90fc | store | 4 | `sp -0x24` | `?` |
| `str` at 0xb9100 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb9104 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb9108 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb910c | load | 4 | `abs 0xb927c` | `-` |
| `ldr` at 0xb9110 | load | 4 | `abs 0x29c3b4` | `-` |
| `ldr` at 0xb9114 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9120 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9124 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb912c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xb9134 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xb9150 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9154 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xb9164 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xb9168 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xb916c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb9170 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xb9178 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb9180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb918c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb9190 | store | 4 | `sp -0x64` | `[sp-72]` |
| `ldr` at 0xb9194 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb9198 | store | 4 | `sp -0x68` | `[sp-32]` |
| `ldr` at 0xb919c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb91a8 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0xb91ac | load | 1 | `[sp-100] +0x0` | `-` |
| `ldr` at 0xb91b8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xb91bc | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xb91c8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb91cc | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xb91d0 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xb91d4 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xb91e4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb91e8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xb91ec | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xb91f0 | load | 1 | `[sp-104] +0x0` | `-` |
| `str` at 0xb9200 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb9208 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xb9210 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0xb9214 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xb921c | store | 4 | `sp -0x68` | `[sp-104]+1` |
| `ldr` at 0xb9224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb9230 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xb9234 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xb9238 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xb923c | store | 4 | `sp -0x20` | `[sp-72]` |
| `ldr` at 0xb9240 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb9248 | store | 4 | `sp -0x24` | `[sp-36]-4` |
| `ldr` at 0xb9250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb925c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb9260 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb9264 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb9268 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb926c | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xb9270 | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0xb9278 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb9278 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb9278 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb913c | `(unknown callee) 0xb8c60` | `[sp-96]` | `sp-88` | `[0x29c3b4]` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb9084 | `r0 == 0x0` | `r0 != 0x0` |
| `bls` at 0xb90b0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xb90c0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb90e4 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0xb911c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0xb9148 | `ret(0xb8c60) == 0x0` | `ret(0xb8c60) != 0x0` |
| `beq` at 0xb9160 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb9188 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb91a4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb91b4 | `[[sp-100]]:1 == 0x0` | `[[sp-100]]:1 != 0x0` |
| `beq` at 0xb91c4 | `[[sp-104]]:1 == 0x0` | `[[sp-104]]:1 != 0x0` |
| `bhs` at 0xb91dc | `[[sp-100]]:1 >= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 < [[sp-104]]:1 (unsigned, 32-bit)` |
| `bls` at 0xb91f8 | `[[sp-100]]:1 <= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 > [[sp-104]]:1 (unsigned, 32-bit)` |
| `bne` at 0xb922c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xb9258 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb9278 | `r0` | `?` |

## `LpbkInit`

- `LpbkInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x964c0 to 0x96598.
  Established. The unnamed `0xa75f8` call carries `0x29a844`, `0xffff012`, `0x1`, `0x96598` (`bl` at 0x9651c).
  A zero result takes `0x96560`, while nonzero falls through (`beq` at 0x96530).
  A word assignment writes `[0x133a28]+1` to `0x133a28` (`str` at 0x96570).
  Loopback initialization state meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x964d0 | load | 4 | `abs 0x9657c` | `-` |
| `ldr` at 0x964d4 | load | 4 | `abs 0x133a28` | `-` |
| `ldr` at 0x964e0 | load | 4 | `abs 0x96580` | `-` |
| `str` at 0x964e4 | store | 4 | `sp -0x1c` | `0x110a9c` |
| `ldr` at 0x964ec | load | 4 | `abs 0x96584` | `-` |
| `ldr` at 0x964fc | load | 4 | `abs 0x96588` | `-` |
| `ldr` at 0x9650c | load | 4 | `abs 0x96588` | `-` |
| `ldr` at 0x96510 | load | 4 | `abs 0x9658c` | `-` |
| `ldr` at 0x96518 | load | 4 | `abs 0x96590` | `-` |
| `str` at 0x96524 | store | 4 | `sp -0x14` | `ret(0xa75f8)` |
| `ldr` at 0x96528 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x96540 | load | 4 | `abs 0x96594` | `-` |
| `str` at 0x96544 | store | 4 | `sp -0x1c` | `0x110ad8` |
| `str` at 0x96548 | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0x96550 | load | 4 | `abs 0x96584` | `-` |
| `ldr` at 0x96560 | load | 4 | `abs 0x9657c` | `-` |
| `ldr` at 0x96564 | load | 4 | `abs 0x9657c` | `-` |
| `ldr` at 0x96568 | load | 4 | `abs 0x133a28` | `-` |
| `str` at 0x96570 | store | 4 | `abs 0x133a28` | `[0x133a28]+1` |
| `ldm` at 0x96578 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x96578 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x96578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x964f8 | `(unknown callee) 0x200a0` | `0x9` | `0x110ac0` | `0x2a` | `0x0` |
| `bl` at 0x96508 | `(unknown callee) 0xa89d0` | `0x29a844` | `0x12` | `0x0` | `?` |
| `bl` at 0x9651c | `(unknown callee) 0xa75f8` | `0x29a844` | `0xffff012` | `0x1` | `0x96598` |
| `bl` at 0x96538 | `(unknown callee) 0x1f8e0` | `ret(0xa75f8)` | `?` | `?` | `ret(0xa75f8)` |
| `bl` at 0x9655c | `(unknown callee) 0x200a0` | `0x9` | `0x110ac0` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x964dc | `[0x133a28] == 0x0` | `[0x133a28] != 0x0` |
| `beq` at 0x96530 | `ret(0xa75f8) == 0x0` | `ret(0xa75f8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x96578 | `r0` | `?` |

## `networkGetType`

- `networkGetType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x25fb8 to 0x25fd8.
  Established. A byte read addresses `0x132074` (`ldrb` at 0x25fc8).
  The integer return record carries `[0x132074]:1` (`ldm` at 0x25fd0).
  Network type value meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x25fc4 | load | 4 | `abs 0x25fd4` | `-` |
| `ldrb` at 0x25fc8 | load | 1 | `abs 0x132074` | `-` |
| `ldm` at 0x25fd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x25fd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x25fd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x25fd0 | `r0` | `[0x132074]:1` |

## `charioInit`

- `charioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21dfc to 0x21e78.
  Established. The unnamed `0x20bb4` call carries `0x12f620`, `0xee0c0`, `0x1`, `0x9` (`bl` at 0x21e4c).
  A word assignment writes `[sp-20]` to `0x12f674` (`str` at 0x21e58).
  A zero `[0x12f674]` takes `0x21e3c`, while nonzero falls through (`beq` at 0x21e1c).
  Character I/O initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21e0c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x21e10 | load | 4 | `abs 0x21e64` | `-` |
| `ldr` at 0x21e14 | load | 4 | `abs 0x12f674` | `-` |
| `ldr` at 0x21e20 | load | 4 | `abs 0x21e68` | `-` |
| `str` at 0x21e24 | store | 4 | `sp -0x1c` | `0xee080` |
| `ldr` at 0x21e2c | load | 4 | `abs 0x21e6c` | `-` |
| `ldr` at 0x21e3c | load | 4 | `abs 0x21e70` | `-` |
| `ldr` at 0x21e40 | load | 4 | `abs 0x21e74` | `-` |
| `ldr` at 0x21e50 | load | 4 | `abs 0x21e64` | `-` |
| `ldr` at 0x21e54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x21e58 | store | 4 | `abs 0x12f674` | `[sp-20]` |
| `ldm` at 0x21e60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x21e60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x21e60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x21e38 | `(unknown callee) 0x200a0` | `0x9` | `0xee0a4` | `0x73` | `0x0` |
| `bl` at 0x21e4c | `(unknown callee) 0x20bb4` | `0x12f620` | `0xee0c0` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x21e1c | `[0x12f674] == 0x0` | `[0x12f674] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x21e60 | `r0` | `ret(0x20bb4)` |

## `charioOpen`

- `charioOpen`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21e78 to 0x22030.
  Established. Included word assignments write `ret(0x1943c)` to `0x12f678` and `0x1` to `0x12f67c` (`str` at 0x21f38, `str` at 0x21f58).
  A nonzero `[0x12f674]` takes `0x21f60`, while zero falls through (`bne` at 0x21f24).
  The unnamed `0xbd18c` call carries `[0x12f674]`, `[sp-28]`, unknown, `0x12f674` (`bl` at 0x21f6c).
  Character stream handle ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21e88 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x21e90 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x21e98 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x21e9c | load | 4 | `abs 0x22010` | `-` |
| `ldr` at 0x21eb4 | load | 4 | `abs 0x22014` | `-` |
| `ldr` at 0x21eb8 | load | 4 | `abs 0x12f3c0` | `-` |
| `str` at 0x21ec8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x21ecc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x21ed0 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x21edc | load | 4 | `abs 0x22018` | `-` |
| `str` at 0x21ee0 | store | 4 | `sp -0x30` | `0xee0cc` |
| `ldr` at 0x21ee4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x21ee8 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x21ef0 | load | 4 | `abs 0x2201c` | `-` |
| `ldr` at 0x21f00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x21f0c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x21f14 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x21f18 | load | 4 | `abs 0x22020` | `-` |
| `ldr` at 0x21f1c | load | 4 | `abs 0x12f674` | `-` |
| `ldr` at 0x21f28 | load | 4 | `abs 0x22024` | `-` |
| `str` at 0x21f38 | store | 4 | `abs 0x12f678` | `ret(0x1943c)` |
| `ldr` at 0x21f3c | load | 4 | `abs 0x22024` | `-` |
| `ldr` at 0x21f40 | load | 4 | `abs 0x12f678` | `-` |
| `ldr` at 0x21f50 | load | 4 | `abs 0x22028` | `-` |
| `str` at 0x21f58 | store | 4 | `abs 0x12f67c` | `0x1` |
| `ldr` at 0x21f60 | load | 4 | `abs 0x22020` | `-` |
| `ldr` at 0x21f64 | load | 4 | `abs 0x12f674` | `-` |
| `ldr` at 0x21f68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x21f74 | load | 4 | `abs 0x22024` | `-` |
| `str` at 0x21f78 | store | 4 | `abs 0x12f678` | `ret(0xbd18c)` |
| `ldr` at 0x21f7c | load | 4 | `abs 0x12f678` | `-` |
| `ldr` at 0x21f88 | load | 4 | `abs 0x22028` | `-` |
| `str` at 0x21f90 | store | 4 | `abs 0x12f67c` | `0x1` |
| `str` at 0x21f98 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x21f9c | load | 4 | `abs 0x22010` | `-` |
| `ldr` at 0x21fb0 | load | 4 | `abs 0x22014` | `-` |
| `ldr` at 0x21fb4 | load | 4 | `abs 0x12f3c0` | `-` |
| `str` at 0x21fc4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x21fc8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x21fcc | store | 4 | `sp -0x20` | `[sp-40]` |
| `ldr` at 0x21fd8 | load | 4 | `abs 0x2202c` | `-` |
| `str` at 0x21fdc | store | 4 | `sp -0x30` | `0xee0f0` |
| `ldr` at 0x21fe0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x21fe4 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x21fec | load | 4 | `abs 0x2201c` | `-` |
| `ldr` at 0x21ffc | load | 4 | `abs 0x22024` | `-` |
| `ldr` at 0x22000 | load | 4 | `abs 0x12f678` | `-` |
| `ldm` at 0x2200c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x2200c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2200c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2200c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x21ea4 | `(unknown callee) 0x20cd0` | `0x12f620` | `?` | `r2` | `0x0` |
| `bl` at 0x21efc | `(unknown callee) 0x200a0` | `0x9` | `0xee0a4` | `0x93` | `0x0` |
| `bl` at 0x21f30 | `(unknown callee) 0x1943c` | `0x108` | `?` | `?` | `[0x12f674]` |
| `bl` at 0x21f4c | `(unknown callee) 0x19748` | `[0x12f678]` | `0x0` | `0x108` | `0x12f678` |
| `bl` at 0x21f6c | `(unknown callee) 0xbd18c` | `[0x12f674]` | `[sp-28]` | `?` | `0x12f674` |
| `bl` at 0x21fa0 | `(unknown callee) 0x2105c` | `0x12f620` | `?` | `?` | `0x0` |
| `bl` at 0x21ff8 | `(unknown callee) 0x200a0` | `0x9` | `0xee0a4` | `0xab` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x21eb0 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `beq` at 0x21ec0 | `[0x12f3c0] == 0x1` | `[0x12f3c0] != 0x1` |
| `beq` at 0x21ed8 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x21f08 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0x21f24 | `[0x12f674] != 0x0` | `[0x12f674] == 0x0` |
| `beq` at 0x21f84 | `[0x12f678] == 0x0` | `[0x12f678] != 0x0` |
| `beq` at 0x21fac | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |
| `beq` at 0x21fbc | `[0x12f3c0] == 0x1` | `[0x12f3c0] != 0x1` |
| `beq` at 0x21fd4 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2200c | `r0` | `[0x12f678]` |

## `charioClose`

- `charioClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22050 to 0x22184.
  Established. The unnamed `0xbd60c` call carries `[0x12f678]`, `0x1`, unknown, `0x12f678` (`bl` at 0x22100).
  Included word assignments clear `0x12f678` and `0x12f67c` to `0x0` (`str` at 0x220d0, `str` at 0x220dc).
  A zero `[0x12f67c]` takes `0x22118`, while nonzero falls through (`beq` at 0x220f0).
  Stream close result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22064 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x22068 | load | 4 | `abs 0x22168` | `-` |
| `str` at 0x22078 | store | 4 | `sp -0x18` | `ret(0x20cd0)` |
| `ldr` at 0x2207c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22088 | load | 4 | `abs 0x2216c` | `-` |
| `str` at 0x2208c | store | 4 | `sp -0x20` | `0xee0cc` |
| `ldr` at 0x22090 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22094 | store | 4 | `sp -0x1c` | `ret(0x20cd0)` |
| `ldr` at 0x2209c | load | 4 | `abs 0x22170` | `-` |
| `ldr` at 0x220ac | load | 4 | `abs 0x22174` | `-` |
| `ldr` at 0x220b0 | load | 4 | `abs 0x12f674` | `-` |
| `ldr` at 0x220bc | load | 4 | `abs 0x22178` | `-` |
| `ldr` at 0x220c0 | load | 4 | `abs 0x12f678` | `-` |
| `ldr` at 0x220c8 | load | 4 | `abs 0x22178` | `-` |
| `str` at 0x220d0 | store | 4 | `abs 0x12f678` | `0x0` |
| `ldr` at 0x220d4 | load | 4 | `abs 0x2217c` | `-` |
| `str` at 0x220dc | store | 4 | `abs 0x12f67c` | `0x0` |
| `ldr` at 0x220e4 | load | 4 | `abs 0x2217c` | `-` |
| `ldr` at 0x220e8 | load | 4 | `abs 0x12f67c` | `-` |
| `ldr` at 0x220f4 | load | 4 | `abs 0x22178` | `-` |
| `ldr` at 0x220f8 | load | 4 | `abs 0x12f678` | `-` |
| `str` at 0x22108 | store | 4 | `sp -0x14` | `ret(0xbd60c)` |
| `ldr` at 0x2210c | load | 4 | `abs 0x2217c` | `-` |
| `str` at 0x22114 | store | 4 | `abs 0x12f67c` | `0x0` |
| `ldr` at 0x22118 | load | 4 | `abs 0x22168` | `-` |
| `str` at 0x22124 | store | 4 | `sp -0x18` | `ret(0x2105c)` |
| `ldr` at 0x22128 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x22134 | load | 4 | `abs 0x22180` | `-` |
| `str` at 0x22138 | store | 4 | `sp -0x20` | `0xee0f0` |
| `ldr` at 0x2213c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x22140 | store | 4 | `sp -0x1c` | `ret(0x2105c)` |
| `ldr` at 0x22148 | load | 4 | `abs 0x22170` | `-` |
| `ldr` at 0x22158 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x22164 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x22164 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x22164 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x22070 | `(unknown callee) 0x20cd0` | `0x12f620` | `?` | `r2` | `0x0` |
| `bl` at 0x220a8 | `(unknown callee) 0x200a0` | `0x9` | `0xee0a4` | `0xce` | `0x0` |
| `bl` at 0x220c4 | `(unknown callee) 0x1970c` | `[0x12f678]` | `?` | `?` | `0x12f678` |
| `bl` at 0x22100 | `(unknown callee) 0xbd60c` | `[0x12f678]` | `0x1` | `?` | `0x12f678` |
| `bl` at 0x2211c | `(unknown callee) 0x2105c` | `0x12f620` | `?` | `?` | `?` |
| `bl` at 0x22154 | `(unknown callee) 0x200a0` | `0x9` | `0xee0a4` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x22084 | `ret(0x20cd0) == 0x0` | `ret(0x20cd0) != 0x0` |
| `bne` at 0x220b8 | `[0x12f674] != 0x0` | `[0x12f674] == 0x0` |
| `beq` at 0x220f0 | `[0x12f67c] == 0x0` | `[0x12f67c] != 0x0` |
| `beq` at 0x22130 | `ret(0x2105c) == 0x0` | `ret(0x2105c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x22164 | `r0` | `[sp-20]` |
