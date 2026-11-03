# ARM JChip configuration and lookup routines in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
These routines have no unresolved computed transfers. Indirect call targets and numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/jchip-config-facts.txt) retain all tool sections.

## `JChipSetup`

- `JChipSetup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9998 to 0xe99c4.
  Established. The explicit four-byte assignment writes `0xeb014` at absolute `0x143864` (`str` at
  0xe99ac). The exit carries `0x0` (`ldmdb` at 0xe99b8). No calls or conditional branches appear in
  this span. The stored pointer role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe99a4 | load | 4 | `abs 0xe99bc` | `-` |
| `ldr` at 0xe99a8 | load | 4 | `abs 0xe99c0` | `-` |
| `str` at 0xe99ac | store | 4 | `abs 0x143864` | `0xeb014` |
| `ldmdb` at 0xe99b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe99b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe99b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe99b8 | `r0` | `0x0` |

## `JChipTargetSetup`

- `JChipTargetSetup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe99c4 to 0xe9a1c.
  Established. The unnamed `0xebc18` call has `r0`, `r1`, `r2`, `r3` (`bl` at 0xe99dc); its nonzero
  result takes the branch to `0xe9a00`, followed by an unnamed `0xe9a1c` call with `ret(0xebc18)`,
  unknown, unknown, `ret(0xebc18)` (`bne` at 0xe99f0, `bl` at 0xe9a04). The zero-result arm assigns
  four-byte `0xfffff7c6` at `sp -0x1c` (`str` at 0xe99f8). Target setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe99d4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe99d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xe99e4 | store | 4 | `sp -0x18` | `ret(0xebc18)` |
| `ldr` at 0xe99e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe99f4 | load | 4 | `abs 0xe9a18` | `-` |
| `str` at 0xe99f8 | store | 4 | `sp -0x1c` | `0xfffff7c6` |
| `ldr` at 0xe9a00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9a0c | store | 4 | `sp -0x1c` | `ret(0xe9a1c)` |
| `ldr` at 0xe9a10 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe9a14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9a14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9a14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe99dc | `(unknown callee) 0xebc18` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xe9a04 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe99f0 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9a14 | `r0` | `?` |

## `JChipBrickSetup`

- `JChipBrickSetup`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9a1c to 0xe9b2c.
  Established. Included byte and halfword reads target `r0 +0x21` and `[[sp-20]] +0xe` (`ldrsb` at
  0xe9a34, `ldrh` at 0xe9a48). The zero predicate `[0x2af36c] == 0x0` takes the branch to `0xe9ad0` (`beq` at
  0xe9a64). The included unknown-target call has unknown, `0x0`, `0x2b47f8`, unknown (`mov` at
  0xe9aac). Brick setup protocol and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9a2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe9a30 | load | 4 | `sp -0x14` | `-` |
| `ldrsb` at 0xe9a34 | load | 1 | `r0 +0x21` | `-` |
| `ldr` at 0xe9a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9a44 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldrh` at 0xe9a48 | load | 2 | `[[sp-20]] +0xe` | `-` |
| `ldr` at 0xe9a58 | load | 4 | `abs 0xe9b1c` | `-` |
| `ldr` at 0xe9a5c | load | 4 | `abs 0x2af36c` | `-` |
| `ldr` at 0xe9a68 | load | 4 | `abs 0xe9b1c` | `-` |
| `ldr` at 0xe9a6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9a70 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe9a78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe9a7c | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xe9a80 | load | 4 | `[[sp-20]] +0x0` | `-` |
| `ldr` at 0xe9a8c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9a90 | load | 1 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xe9a98 | load | 4 | `abs 0x2af36c` | `-` |
| `ldr` at 0xe9aa4 | load | 4 | `abs 0xe9b20` | `-` |
| `str` at 0xe9ab4 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xe9ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ac4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9ac8 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0xe9ad0 | load | 4 | `abs 0xe9b24` | `-` |
| `str` at 0xe9ad4 | store | 4 | `sp -0x1c` | `0xfffff7c5` |
| `ldr` at 0xe9adc | load | 4 | `abs 0xe9b28` | `-` |
| `ldr` at 0xe9ae0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9ae4 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe9af0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xe9af4 | load | 1 | `[sp-20] +0x19` | `-` |
| `ldr` at 0xe9af8 | load | 4 | `abs 0x14385c` | `-` |
| `str` at 0xe9b10 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xe9b14 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xe9b18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9b18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9b18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xe9aac | `(unknown callee) (unknown target)` | `?` | `0x0` | `0x2b47f8` | `?` |
| `mov` at 0xe9b08 | `(unknown callee) (unknown target)` | `?` | `[[sp-20]+25]:1` | `[0x14385c]` | `[[sp-20]+25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9a3c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe9a54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xe9a64 | `[0x2af36c] == 0x0` | `[0x2af36c] != 0x0` |
| `beq` at 0xe9ac0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9b18 | `r0` | `?` |

## `JChipPortInit`

- `JChipPortInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9b2c to 0xe9c50.
  Established. The recursive numeric call has `0x2adffc`, `0x1`, `r2`, `r1` (`bl` at 0xe9b64).
  Included four-byte global assignments write `ret(0x1b454)` at `0x2ae108` and `[sp-28]` at
  `0x2ae10c` (`str` at 0xe9b88, `str` at 0xe9bac). A later four-byte store through `[sp-36] +0x4`
  has unknown value (`str` at 0xe9c1c). Port-record layout and initialization effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9b3c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe9b40 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe9b44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9b50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe9b5c | load | 4 | `abs 0xe9c40` | `-` |
| `str` at 0xe9b6c | store | 4 | `sp -0x28` | `ret(0xe9b2c)` |
| `ldr` at 0xe9b74 | load | 4 | `abs 0xe9c44` | `-` |
| `ldr` at 0xe9b78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9b88 | store | 4 | `abs 0x2ae108` | `ret(0x1b454)` |
| `ldr` at 0xe9b8c | load | 4 | `abs 0x2ae108` | `-` |
| `ldr` at 0xe9b98 | load | 4 | `abs 0xe9c48` | `-` |
| `str` at 0xe9b9c | store | 4 | `sp -0x28` | `0xfffff7cc` |
| `ldr` at 0xe9ba4 | load | 4 | `abs 0xe9c4c` | `-` |
| `ldr` at 0xe9ba8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9bac | store | 4 | `abs 0x2ae10c` | `[sp-28]` |
| `str` at 0xe9bb4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe9bb8 | load | 4 | `abs 0xe9c4c` | `-` |
| `ldr` at 0xe9bbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9bc0 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xe9bd0 | load | 4 | `abs 0xe9c44` | `-` |
| `ldr` at 0xe9bd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9bdc | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xe9be4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9be8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9bec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9bf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9bfc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9c00 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe9c04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9c08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9c18 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9c1c | store | 4 | `[sp-36] +0x4` | `?` |
| `ldr` at 0xe9c20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9c28 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xe9c34 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xe9c38 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xe9c3c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xe9c3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xe9c3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xe9c3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9b64 | `(unknown callee) 0xe9b2c` | `0x2adffc` | `0x1` | `r2` | `r1` |
| `bl` at 0xe9b80 | `(unknown callee) 0x1b454` | `r1` | `0x10` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9b4c | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xe9b58 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xe9b94 | `[0x2ae108] != 0x0` | `[0x2ae108] == 0x0` |
| `blt` at 0xe9bc8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xe9c3c | `r0` | `?` |

## `JChipInit`

- `JChipInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe9c50 to 0xea0e8.
  Established. A four-byte store through `[sp-36] +0x8` retains `ret(0x1b358)` (`str` at 0xe9d18). A
  later four-byte assignment at `[sp-36] +0xc` writes `[[sp-36]+12]+1` (`str` at 0xe9d74). The
  unknown-target call has `[sp-40]`, unknown, unknown, `[[[sp-40]]+16]` (`mov` at 0xe9efc); its
  nonnegative signed result takes the branch to `0xe9f38` (`bge` at 0xe9f08). Chip record fields and
  callback initialization remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe9c60 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe9c64 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe9c68 | load | 4 | `abs 0xea0b0` | `-` |
| `ldr` at 0xe9c6c | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xe9c78 | load | 4 | `abs 0xea0b4` | `-` |
| `str` at 0xe9c7c | store | 4 | `sp -0x50` | `0x136ea8` |
| `ldr` at 0xe9c84 | load | 4 | `abs 0xea0b8` | `-` |
| `ldr` at 0xe9c88 | load | 4 | `abs 0xea0bc` | `-` |
| `ldr` at 0xe9c94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9ca0 | load | 4 | `abs 0xea0b0` | `-` |
| `ldr` at 0xe9ca4 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xe9cb0 | load | 4 | `abs 0xea0c0` | `-` |
| `str` at 0xe9cb4 | store | 4 | `sp -0x30` | `0xfffff7c7` |
| `ldr` at 0xe9cbc | load | 4 | `abs 0xea0c4` | `-` |
| `ldr` at 0xe9cc0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe9cd0 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xe9cd8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xe9cdc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9ce0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9ce4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9ce8 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe9d08 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe9d18 | store | 4 | `[sp-36] +0x8` | `ret(0x1b358)` |
| `ldr` at 0xe9d1c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xe9d28 | load | 4 | `abs 0xea0c8` | `-` |
| `str` at 0xe9d2c | store | 4 | `sp -0x30` | `0xfffff7cc` |
| `ldr` at 0xe9d34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9d38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9d3c | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe9d58 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe9d60 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe9d64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9d68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9d6c | load | 4 | `[sp-36] +0xc` | `-` |
| `str` at 0xe9d74 | store | 4 | `[sp-36] +0xc` | `[[sp-36]+12]+1` |
| `ldr` at 0xe9d78 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9d84 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xe9d8c | store | 1 | `[sp-40] +0x18` | `?` |
| `ldr` at 0xe9d90 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9d94 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xe9dac | store | 1 | `[sp-40] +0x19` | `?` |
| `ldr` at 0xe9db0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9db4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xe9db8 | store | 4 | `[sp-40] +0x1c` | `[sp-28]` |
| `ldr` at 0xe9dbc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xe9dc4 | store | 1 | `[sp-40] +0x20` | `0x0` |
| `ldr` at 0xe9dc8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0xe9dd0 | store | 1 | `[sp-40] +0x21` | `?` |
| `ldr` at 0xe9dd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9dd8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9dec | store | 4 | `[sp-40] +0x0` | `ret(0xebb6c)` |
| `ldr` at 0xe9df0 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9dfc | load | 4 | `abs 0xea0cc` | `-` |
| `str` at 0xe9e00 | store | 4 | `sp -0x50` | `0x136eec` |
| `ldr` at 0xe9e04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9e10 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe9e18 | load | 4 | `abs 0xea0b8` | `-` |
| `ldr` at 0xe9e28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9e2c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xe9e38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9e3c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xe9e4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe9e54 | store | 4 | `sp -0x34` | `[sp-40]+4` |
| `ldr` at 0xe9e58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9e5c | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xe9e60 | store | 4 | `sp -0x38` | `[[sp-36]+4]` |
| `ldr` at 0xe9e64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9e68 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xe9e74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9e78 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xe9e7c | store | 4 | `sp -0x38` | `[[sp-36]]` |
| `ldr` at 0xe9e80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9e84 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9e88 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9e8c | load | 1 | `[sp-40] +0x18` | `-` |
| `str` at 0xe9e90 | store | 4 | `sp -0x50` | `[[sp-40]+24]:1` |
| `ldr` at 0xe9e94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xe9e98 | load | 4 | `abs 0xea0d0` | `-` |
| `ldr` at 0xe9e9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xe9ea0 | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xe9eac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9eb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9eb8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9ebc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9ec0 | load | 1 | `[sp-40] +0x18` | `-` |
| `ldr` at 0xe9ec8 | load | 4 | `abs 0xea0d4` | `-` |
| `ldr` at 0xe9ecc | load | 4 | `[[sp-40]] +0x8` | `-` |
| `ldr` at 0xe9ed4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9ed8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9edc | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xe9ee8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9eec | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9ef0 | load | 4 | `[[sp-40]] +0x10` | `-` |
| `ldr` at 0xe9ef4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9f0c | load | 4 | `abs 0xea0d8` | `-` |
| `str` at 0xe9f10 | store | 4 | `sp -0x50` | `0x136f28` |
| `ldr` at 0xe9f14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe9f20 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xe9f28 | load | 4 | `abs 0xea0b8` | `-` |
| `ldr` at 0xe9f2c | load | 4 | `abs 0xea0dc` | `-` |
| `str` at 0xe9f3c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xe9f40 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9f44 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xe9f4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9f5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xe9f60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xe9f7c | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xe9f84 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xe9f88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9f8c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9f90 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xe9f94 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xe9fa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9fa4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9fa8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9fac | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xe9fb8 | load | 4 | `abs 0xea0e0` | `-` |
| `str` at 0xe9fbc | store | 4 | `sp -0x50` | `0x136f60` |
| `ldr` at 0xe9fc0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xe9fc4 | load | 1 | `[sp-40] +0x19` | `-` |
| `str` at 0xe9fc8 | store | 4 | `sp -0x4c` | `[[sp-40]+25]:1` |
| `ldr` at 0xe9fcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xe9fd0 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xe9fd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9fd8 | load | 4 | `[sp-40] +0x1c` | `-` |
| `str` at 0xe9fdc | store | 4 | `sp -0x44` | `[[sp-40]+28]` |
| `ldr` at 0xe9fe0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe9fe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe9fe8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe9fec | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xe9ff0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe9ff4 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xe9ff8 | load | 4 | `[[sp-40]] +0x0` | `-` |
| `str` at 0xe9ffc | store | 4 | `sp -0x3c` | `[[[sp-40]]]` |
| `ldr` at 0xea004 | load | 4 | `abs 0xea0b8` | `-` |
| `ldr` at 0xea008 | load | 4 | `abs 0xea0e4` | `-` |
| `ldr` at 0xea014 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea018 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xea01c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xea020 | load | 1 | `[sp-40] +0x19` | `-` |
| `ldr` at 0xea02c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea030 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea034 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea038 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xea03c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea040 | load | 4 | `[[sp-40]] +0x0` | `-` |
| `ldr` at 0xea04c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea050 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea054 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea058 | load | 4 | `[sp-40] +0x1c` | `-` |
| `ldr` at 0xea064 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea068 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xea06c | load | 1 | `[sp-40] +0x20` | `-` |
| `strb` at 0xea074 | store | 1 | `[sp-40] +0x20` | `([[sp-40]+32]:1+1 & 0xff)` |
| `ldr` at 0xea07c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea080 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xea084 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xea08c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xea090 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea098 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xea0a4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xea0a8 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xea0ac | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xea0ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea0ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea0ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe9c90 | `(unknown callee) 0x21eb8` | `0x9` | `0x136ed4` | `0x12d` | `0x0` |
| `bl` at 0xe9d10 | `(unknown callee) 0x1b358` | `[[sp-36]+8]` | `?` | `[[sp-36]+12]` | `?` |
| `bl` at 0xe9de8 | `(unknown callee) 0xebb6c` | `?` | `?` | `[sp-40]` | `?` |
| `bl` at 0xe9e24 | `(unknown callee) 0x21eb8` | `0x9` | `0x136ed4` | `0x144` | `0x0` |
| `bl` at 0xe9ea4 | `(unknown callee) 0x1b52c` | `[sp-40]+4` | `0x136f18` | `?` | `[[[sp-40]]+8]` |
| `bl` at 0xe9ed0 | `(unknown callee) 0x1b52c` | `[sp-40]+4` | `0x136f20` | `[[[sp-40]]+8]` | `[[sp-40]+24]:1` |
| `mov` at 0xe9efc | `(unknown callee) (unknown target)` | `[sp-40]` | `?` | `?` | `[[[sp-40]]+16]` |
| `bl` at 0xe9f34 | `(unknown callee) 0x21eb8` | `0x9` | `0x136ed4` | `0x152` | `0x0` |
| `bl` at 0xea010 | `(unknown callee) 0x21eb8` | `0x9` | `0x136ed4` | `0x163` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xe9c74 | `[0x2ae10c] != 0x0` | `[0x2ae10c] == 0x0` |
| `blt` at 0xe9cac | `? < [0x2ae10c] (signed, 32-bit)` | `? >= [0x2ae10c] (signed, 32-bit)` |
| `bne` at 0xe9d24 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `bne` at 0xe9df8 | `[[sp-40]] != 0x0` | `[[sp-40]] == 0x0` |
| `bne` at 0xe9e34 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `bne` at 0xe9e44 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `bne` at 0xe9e70 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `beq` at 0xe9ee4 | `[[[sp-40]]+16] == 0x0` | `[[[sp-40]]+16] != 0x0` |
| `bge` at 0xe9f08 | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `blt` at 0xe9f54 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xe9f9c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xe9fb4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xea028 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xea048 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xea060 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea0ac | `r0` | `?` |

## `JChipUpdatePort`

- `JChipUpdatePort`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea0e8 to 0xea4a8.
  Established. The entry comparison takes `r0 < [0x2ae10c]`, signed 32-bit, to `0xea11c` (`blt` at
  0xea10c). An included byte read targets `[sp-56] +0x10` (`ldrb` at 0xea1b4). The unnamed `0xe77a4`
  call has `[[sp-52]+25]:1`, unknown, `[[sp-52]+32]:1`, `[[[sp-52]]]` (`bl` at 0xea3f8). A byte
  assignment through `[sp-52] +0x21` retains `(ret(0xe77a4) & 0xff)` (`strb` at 0xea400). Port
  update criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea0f8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xea0fc | load | 4 | `abs 0xea48c` | `-` |
| `ldr` at 0xea100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea104 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xea110 | load | 4 | `abs 0xea490` | `-` |
| `str` at 0xea114 | store | 4 | `sp -0x44` | `0xfffff7c5` |
| `ldr` at 0xea11c | load | 4 | `abs 0xea494` | `-` |
| `ldr` at 0xea120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea128 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xea130 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xea138 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xea13c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea144 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xea154 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea158 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xea174 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xea17c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xea180 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea184 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea198 | load | 4 | `abs 0xea498` | `-` |
| `str` at 0xea1a0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xea1a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea1a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea1b0 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xea1b4 | load | 1 | `[sp-56] +0x10` | `-` |
| `ldr` at 0xea1c0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xea1c8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xea1d0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea1d4 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xea1e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea1e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea1fc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea200 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xea210 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xea218 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xea220 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea224 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xea230 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea234 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea244 | load | 4 | `abs 0xea49c` | `-` |
| `ldr` at 0xea250 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xea258 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xea260 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea264 | load | 4 | `[sp-56] +0x8` | `-` |
| `ldr` at 0xea270 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0xea278 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xea27c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea280 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xea284 | store | 4 | `sp -0x2c` | `[[sp-52]+32]:1` |
| `str` at 0xea28c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xea290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea294 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea298 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xea2a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea2ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea2c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xea2d0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xea2d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea2d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xea2dc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xea2e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea2e4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea2e8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xea2ec | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xea2f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea2fc | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea300 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xea304 | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xea310 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea314 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xea31c | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xea320 | load | 1 | `[sp-56] +0x10` | `-` |
| `str` at 0xea330 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xea338 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea33c | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xea348 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea34c | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xea364 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea368 | load | 4 | `[sp-56] +0x14` | `-` |
| `str` at 0xea37c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xea384 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xea388 | load | 4 | `[sp-56] +0x14` | `-` |
| `ldr` at 0xea394 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea398 | load | 4 | `[sp-52] +0x1c` | `-` |
| `ldr` at 0xea3a8 | load | 4 | `abs 0xea49c` | `-` |
| `str` at 0xea3b8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xea3bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea3c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xea3cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea3d0 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea3d4 | load | 1 | `[sp-52] +0x19` | `-` |
| `ldr` at 0xea3d8 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea3dc | load | 1 | `[sp-52] +0x20` | `-` |
| `ldr` at 0xea3e0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea3e4 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xea3f4 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `strb` at 0xea400 | store | 1 | `[sp-52] +0x21` | `(ret(0xe77a4) & 0xff)` |
| `ldr` at 0xea404 | load | 4 | `abs 0xea4a0` | `-` |
| `ldr` at 0xea408 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xea414 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea418 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xea41c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea420 | load | 1 | `[sp-52] +0x18` | `-` |
| `ldr` at 0xea424 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea428 | load | 1 | `[sp-52] +0x19` | `-` |
| `str` at 0xea42c | store | 4 | `sp -0x54` | `[[sp-52]+25]:1` |
| `ldr` at 0xea430 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xea434 | load | 1 | `[sp-52] +0x20` | `-` |
| `str` at 0xea438 | store | 4 | `sp -0x50` | `[[sp-52]+32]:1` |
| `ldr` at 0xea43c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xea440 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0xea444 | load | 4 | `[[sp-52]] +0x0` | `-` |
| `str` at 0xea448 | store | 4 | `sp -0x4c` | `[[[sp-52]]]` |
| `ldr` at 0xea44c | load | 4 | `sp -0x34` | `-` |
| `ldrsb` at 0xea450 | load | 1 | `[sp-52] +0x21` | `-` |
| `str` at 0xea454 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xea458 | load | 4 | `abs 0xea4a4` | `-` |
| `ldr` at 0xea45c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xea460 | load | 4 | `[[sp-52]] +0x8` | `-` |
| `ldr` at 0xea46c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xea474 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xea480 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xea484 | load | 4 | `sp -0x44` | `-` |
| `ldmdb` at 0xea488 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xea488 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xea488 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xea488 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea3f8 | `(unknown callee) 0xe77a4` | `[[sp-52]+25]:1` | `?` | `[[sp-52]+32]:1` | `[[[sp-52]]]` |
| `bl` at 0xea468 | `(unknown callee) 0x23fbc` | `0x136fc8` | `[sp-24]` | `[[[sp-52]]+8]` | `[[sp-52]+24]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xea10c | `r0 < [0x2ae10c] (signed, 32-bit)` | `r0 >= [0x2ae10c] (signed, 32-bit)` |
| `blt` at 0xea14c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xea1bc | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xea1dc | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xea20c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xea22c | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xea24c | `? == 0xffff` | `? != 0xffff` |
| `bne` at 0xea26c | `[[sp-56]+8] != 0x0` | `[[sp-56]+8] == 0x0` |
| `blt` at 0xea2a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xea2f4 | `Z == 0` | `not (Z == 0)` |
| `blo` at 0xea30c | `C == 0` | `not (C == 0)` |
| `beq` at 0xea328 | `? == [[sp-56]+16]:1` | `? != [[sp-56]+16]:1` |
| `beq` at 0xea344 | `[[sp-56]+20] == 0x0` | `[[sp-56]+20] != 0x0` |
| `bne` at 0xea374 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xea390 | `[[sp-56]+20] != 0x0` | `[[sp-56]+20] == 0x0` |
| `beq` at 0xea3b0 | `? == 0xffff` | `? != 0xffff` |
| `bls` at 0xea410 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xea488 | `r0` | `?` |

## `JChipTargetByName`

- `JChipTargetByName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xea4a8 to 0xeaab4.
  Established. A byte store through `[sp-36] +0x0` writes `0x3a` (`strb` at 0xea584). The deciding
  branch takes `[[sp-36]]:1 != 0x2e` to `0xea588` and falls through for equality (`bne` at 0xea578).
  Included unnamed `0x1b478` calls carry `[sp-28]`, `0x137028`, `sp-92`, `sp-72`, and `[sp-28]`,
  `0x13703c`, `sp-112`, `sp-116` (`bl` at 0xea5bc, `bl` at 0xea600). Name grammar and returned
  target encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xea4b8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xea4bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea4c8 | store | 4 | `sp -0x1c` | `ret(0x1b034)` |
| `ldr` at 0xea4cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xea4d8 | store | 4 | `sp -0x20` | `ret(0x1b2bc)` |
| `ldr` at 0xea4dc | load | 4 | `abs 0xeaa90` | `-` |
| `ldrb` at 0xea4e0 | load | 1 | `abs 0x137014` | `-` |
| `strb` at 0xea4e4 | store | 1 | `sp -0x5c` | `([0x137014]:1 & 0xff)` |
| `ldr` at 0xea4fc | load | 4 | `abs 0xeaa90` | `-` |
| `ldrb` at 0xea500 | load | 1 | `abs 0x137014` | `-` |
| `strb` at 0xea504 | store | 1 | `sp -0x70` | `([0x137014]:1 & 0xff)` |
| `str` at 0xea520 | store | 4 | `sp -0x74` | `?` |
| `str` at 0xea528 | store | 4 | `sp -0x78` | `0x0` |
| `str` at 0xea530 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xea534 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xea538 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0xea53c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xea540 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xea554 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xea558 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xea568 | store | 1 | `(unknown)` | `(ret(0x1b34c) & 0xff)` |
| `ldr` at 0xea56c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xea570 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xea57c | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xea584 | store | 1 | `[sp-36] +0x0` | `0x3a` |
| `ldr` at 0xea588 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xea590 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xea5a4 | store | 4 | `sp -0xb0` | `sp-112` |
| `str` at 0xea5ac | store | 4 | `sp -0xac` | `sp-116` |
| `ldr` at 0xea5b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea5b4 | load | 4 | `abs 0xeaa94` | `-` |
| `str` at 0xea5c4 | store | 4 | `sp -0x3c` | `ret(0x1b478)` |
| `ldr` at 0xea5c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea5d4 | load | 4 | `sp -0x3c` | `-` |
| `strb` at 0xea5e4 | store | 1 | `sp -0x5c` | `0x0` |
| `ldr` at 0xea5f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea5f4 | load | 4 | `abs 0xeaa98` | `-` |
| `str` at 0xea608 | store | 4 | `sp -0x3c` | `ret(0x1b478)` |
| `ldr` at 0xea60c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xea618 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xea62c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xea630 | load | 4 | `abs 0xeaa9c` | `-` |
| `ldr` at 0xea634 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea638 | load | 4 | `abs 0x143874` | `-` |
| `ldr` at 0xea648 | load | 4 | `abs 0xeaaa0` | `-` |
| `ldr` at 0xea64c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea688 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xea69c | load | 4 | `abs 0xeaaa0` | `-` |
| `str` at 0xea6a4 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xea6ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xea6b4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xea6bc | load | 4 | `sp -0x78` | `-` |
| `ldrb` at 0xea6cc | load | 1 | `sp -0x5c` | `-` |
| `str` at 0xea6dc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xea6e0 | load | 4 | `abs 0xeaaa4` | `-` |
| `ldr` at 0xea6e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea6e8 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xea6f8 | load | 4 | `abs 0xeaaa8` | `-` |
| `ldr` at 0xea6fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea704 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xea70c | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xea710 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea720 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea728 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea744 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea74c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea764 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xea768 | store | 4 | `sp -0x7c` | `[sp-128]` |
| `ldr` at 0xea770 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xea778 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0xea784 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xea788 | load | 4 | `abs 0xeaaa4` | `-` |
| `ldr` at 0xea78c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea790 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xea7a0 | load | 4 | `abs 0xeaaa8` | `-` |
| `ldr` at 0xea7a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xea7ac | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xea7b4 | store | 4 | `sp -0x80` | `?` |
| `str` at 0xea7bc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea7c0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea7c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea7c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea7d8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea7dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea7f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea800 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea804 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xea810 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea81c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea820 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea830 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xea834 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0xea838 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xea840 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xea848 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xea850 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xea858 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xea86c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xea870 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea874 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea878 | load | 4 | `[sp-124] +0xc` | `-` |
| `ldr` at 0xea888 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea88c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xea8a8 | load | 4 | `[sp-124] +0x8` | `-` |
| `str` at 0xea8b0 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0xea8b4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea8b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea8bc | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xea8c8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xea8d4 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea8d8 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xea8e8 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea8ec | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xea8f8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xea904 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea908 | load | 1 | `[sp-128] +0x18` | `-` |
| `ldr` at 0xea90c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xea918 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea91c | load | 4 | `[sp-124] +0x0` | `-` |
| `ldr` at 0xea928 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea92c | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xea940 | store | 4 | `sp -0x9c` | `sp-148` |
| `ldr` at 0xea944 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea948 | load | 4 | `[sp-124] +0x4` | `-` |
| `str` at 0xea94c | store | 4 | `sp -0xa0` | `[[sp-124]+4]` |
| `ldr` at 0xea950 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea954 | load | 4 | `[sp-124] +0x4` | `-` |
| `ldr` at 0xea960 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xea964 | load | 4 | `[sp-124] +0x0` | `-` |
| `str` at 0xea968 | store | 4 | `sp -0xa0` | `[[sp-124]]` |
| `ldr` at 0xea96c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea970 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea974 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea978 | load | 1 | `(unknown)` | `-` |
| `str` at 0xea97c | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xea980 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xea984 | load | 4 | `abs 0xeaaac` | `-` |
| `ldr` at 0xea988 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0xea98c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea99c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea9a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea9a4 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea9a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xea9b0 | load | 4 | `abs 0xeaab0` | `-` |
| `ldr` at 0xea9b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xea9c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xea9d8 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xea9dc | load | 4 | `[sp-128] +0x0` | `-` |
| `str` at 0xea9e0 | store | 4 | `sp -0x78` | `[[sp-128]]` |
| `ldr` at 0xea9e4 | load | 4 | `sp -0x80` | `-` |
| `ldrb` at 0xea9e8 | load | 1 | `[sp-128] +0x18` | `-` |
| `str` at 0xea9ec | store | 4 | `sp -0x74` | `[[sp-128]+24]:1` |
| `ldr` at 0xea9f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xea9fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xeaa04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeaa0c | load | 4 | `abs 0xeaaa8` | `-` |
| `ldr` at 0xeaa10 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xeaa14 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xeaa1c | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xeaa20 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xeaa2c | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xeaa30 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xeaa34 | load | 4 | `[sp-120] +0x0` | `-` |
| `ldr` at 0xeaa3c | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xeaa44 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0xeaa48 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0xeaa4c | store | 4 | `sp -0xa4` | `[sp-168]` |
| `ldr` at 0xeaa50 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xeaa5c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xeaa60 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xeaa68 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0xeaa6c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0xeaa70 | store | 4 | `sp -0x98` | `[sp-164]` |
| `ldr` at 0xeaa78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeaa84 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0xeaa88 | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0xeaa8c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xeaa8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeaa8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeaa8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xea4c0 | `(unknown callee) 0x1b034` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xea4d0 | `(unknown callee) 0x1b2bc` | `[sp-24]` | `?` | `?` | `ret(0x1b034)` |
| `bl` at 0xea4f8 | `(unknown callee) 0x1b4cc` | `sp-91` | `0x0` | `0x13` | `sp-91` |
| `bl` at 0xea518 | `(unknown callee) 0x1b4cc` | `sp-111` | `0x0` | `0x13` | `sp-111` |
| `bl` at 0xea560 | `(unknown callee) 0x1b34c` | `?` | `?` | `?` | `?` |
| `bl` at 0xea5bc | `(unknown callee) 0x1b478` | `[sp-28]` | `0x137028` | `sp-92` | `sp-72` |
| `bl` at 0xea600 | `(unknown callee) 0x1b478` | `[sp-28]` | `0x13703c` | `sp-112` | `sp-116` |
| `bl` at 0xea678 | `(unknown callee) 0x1b088` | `?` | `sp-112` | `sp-112` | `?` |
| `bl` at 0xea730 | `(unknown callee) 0x1b088` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xea754 | `(unknown callee) 0x1b088` | `?` | `sp-92` | `sp-92` | `?` |
| `bl` at 0xea990 | `(unknown callee) 0x1b52c` | `sp-148` | `0x136f18` | `?` | `?` |
| `bl` at 0xea9b8 | `(unknown callee) 0x1b52c` | `sp-148` | `0x136f20` | `?` | `?` |
| `bl` at 0xea9c8 | `(unknown callee) 0x1b088` | `sp-148` | `[sp-28]` | `?` | `sp-148` |
| `bl` at 0xeaa08 | `(unknown callee) 0x1b490` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xeaa7c | `(unknown callee) 0x1b490` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xea548 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xea578 | `[[sp-36]]:1 != 0x2e` | `[[sp-36]]:1 == 0x2e` |
| `beq` at 0xea5d0 | `ret(0x1b478) == 0x3` | `ret(0x1b478) != 0x3` |
| `beq` at 0xea5dc | `ret(0x1b478) == 0x4` | `ret(0x1b478) != 0x4` |
| `beq` at 0xea614 | `ret(0x1b478) == 0x1` | `ret(0x1b478) != 0x1` |
| `beq` at 0xea620 | `ret(0x1b478) == 0x2` | `ret(0x1b478) != 0x2` |
| `blt` at 0xea640 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xea684 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xea6c4 | `[sp-120] != 0x0` | `[sp-120] == 0x0` |
| `beq` at 0xea6d4 | `[sp-92]:1 == 0x0` | `[sp-92]:1 != 0x0` |
| `blt` at 0xea6f0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xea71c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xea73c | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0xea760 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `blt` at 0xea798 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xea7d0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xea80c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xea818 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `beq` at 0xea828 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xea860 | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `blt` at 0xea880 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xea8c4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xea8d0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xea8e0 | `[[sp-128]+24]:1 == 0x0` | `[[sp-128]+24]:1 != 0x0` |
| `bne` at 0xea8f4 | `[[sp-128]+24]:1 != 0x0` | `[[sp-128]+24]:1 == 0x0` |
| `beq` at 0xea900 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xea914 | `[[sp-128]+24]:1 == [sp-116]` | `[[sp-128]+24]:1 != [sp-116]` |
| `bne` at 0xea924 | `[[sp-124]] != 0x0` | `[[sp-124]] == 0x0` |
| `bne` at 0xea934 | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xea95c | `[[sp-124]+4] != 0x0` | `[[sp-124]+4] == 0x0` |
| `bne` at 0xea9d4 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `beq` at 0xeaa58 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeaa8c | `r0` | `[sp-152]` |

## `JChipGetCfgTarget`

- `JChipGetCfgTarget`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebe18 to 0xebfe0.
  Established. Entry-byte assignments at `sp -0x11` and `sp -0x12` retain `(r0 & 0xff)` and
  `(r1 & 0xff)` (`strb` at 0xebe30, `strb` at 0xebe38). A later four-byte load reads `[sp-36] +0xc` (`ldr`
  at 0xebe64). The unnamed `0x21eb8` call has `0x9`, `0x136ed4`, `0x40d`, `0x0` (`bl` at 0xebf74).
  The exit carries `[sp-44]` (`ldmdb` at 0xebfc8). Configuration target identity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xebe30 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xebe38 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldr` at 0xebe3c | load | 4 | `abs 0xebfcc` | `-` |
| `ldrb` at 0xebe40 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xebe48 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xebe50 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xebe58 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xebe5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebe60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebe64 | load | 4 | `[sp-36] +0xc` | `-` |
| `ldr` at 0xebe74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebe78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebe94 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0xebe9c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xebea0 | load | 4 | `sp -0x28` | `-` |
| `ldrsb` at 0xebea4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xebea8 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xebeb4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebeb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebebc | load | 4 | `abs 0xebfd0` | `-` |
| `ldrb` at 0xebec0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xebee0 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xebee8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xebef0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebef4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf00 | load | 4 | `abs 0xebfd4` | `-` |
| `str` at 0xebf04 | store | 4 | `sp -0x40` | `0x137244` |
| `ldr` at 0xebf08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebf0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xebf14 | store | 4 | `sp -0x3c` | `?` |
| `ldrb` at 0xebf18 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xebf1c | store | 4 | `sp -0x38` | `(r0 & 0xff)` |
| `ldrb` at 0xebf20 | load | 1 | `sp -0x12` | `-` |
| `str` at 0xebf24 | store | 4 | `sp -0x34` | `(r1 & 0xff)` |
| `ldr` at 0xebf28 | load | 4 | `abs 0xebfd0` | `-` |
| `ldrb` at 0xebf2c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xebf4c | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0xebf54 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xebf5c | load | 1 | `(unknown)` | `-` |
| `str` at 0xebf60 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xebf68 | load | 4 | `abs 0xebfd8` | `-` |
| `ldr` at 0xebf6c | load | 4 | `abs 0xebfdc` | `-` |
| `ldr` at 0xebf78 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xebf7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xebf84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xebf88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebf98 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xebf9c | load | 1 | `(unknown)` | `-` |
| `str` at 0xebfa4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xebfac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xebfb4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xebfc0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xebfc4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xebfc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebfc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebfc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebf74 | `(unknown callee) 0x21eb8` | `0x9` | `0x136ed4` | `0x40d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xebe6c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xebeb0 | `? != (r1 & 0xff)` | `? == (r1 & 0xff)` |
| `beq` at 0xebefc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebfc8 | `r0` | `[sp-44]` |

## `JChipGetNextChip`

- `JChipGetNextChip`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeac98 to 0xeae04.
  Established. Negative entry `r0`, signed 32-bit, takes the branch to `0xeacdc` (`blt` at 0xeacc0).
  Four-byte stack assignments at `sp -0x24` include `r0+1` and `[0x2ae10c]` on separate arms (`str`
  at 0xeacd4, `str` at 0xeacec). A later four-byte read targets `[sp-48] +0x8` (`ldr` at 0xead80).
  This span has no calls. Enumeration order and returned object meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeaca8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeacac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeacb4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xeacb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeacc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeacc8 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xeaccc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeacd4 | store | 4 | `sp -0x24` | `r0+1` |
| `str` at 0xeace0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xeace4 | load | 4 | `abs 0xeadfc` | `-` |
| `ldr` at 0xeace8 | load | 4 | `abs 0x2ae10c` | `-` |
| `str` at 0xeacec | store | 4 | `sp -0x24` | `[0x2ae10c]` |
| `ldr` at 0xeacf0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeacf4 | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0xeacf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeacfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xead0c | load | 4 | `abs 0xeae00` | `-` |
| `ldr` at 0xead10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xead18 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xead20 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xead28 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xead2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xead30 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xead34 | load | 4 | `[sp-48] +0xc` | `-` |
| `ldr` at 0xead44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xead50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xead60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xead64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xead80 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0xead88 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xead90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xead94 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeadb0 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0xeadb8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeadc8 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xeadcc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeadd4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xeaddc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeade4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xeadf0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xeadf4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xeadf8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeadf8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeadf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeacc0 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `blt` at 0xead04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xead3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xead4c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xead58 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xeadc0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeadf8 | `r0` | `?` |

## `JChipGetChipList`

- `JChipGetChipList`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeab04 to 0xeac98.
  Established. The unnamed `0x1b358` call carries `[sp-36]`, `[sp-32]+50`, `[sp-32]`, `[sp-32]+50`
  (`bl` at 0xeabf8). The unsigned comparison takes `ret(0x1b2bc)+2 <= ?` to `0xeac14` (`bls` at
  0xeabe0). A four-byte assignment at `sp -0x28` retains `([sp-40] + ret(0x1b2bc))` (`str` at
  0xeac4c). List formatting and allocation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeab18 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xeab20 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xeab28 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xeab30 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xeab38 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeab3c | load | 4 | `abs 0xeac8c` | `-` |
| `ldr` at 0xeab40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeab44 | load | 4 | `abs 0x2ae10c` | `-` |
| `ldr` at 0xeab54 | load | 4 | `abs 0xeac90` | `-` |
| `ldr` at 0xeab58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeab60 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xeab68 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xeab70 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xeab74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeab78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeab7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeab8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xeab90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeabac | load | 4 | `(unknown)` | `-` |
| `str` at 0xeabb4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xeabb8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeabd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeabd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeabe4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeabec | store | 4 | `sp -0x20` | `[sp-32]+50` |
| `ldr` at 0xeabf0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeabf4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeac00 | store | 4 | `sp -0x24` | `ret(0x1b358)` |
| `ldr` at 0xeac04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeac08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeac10 | store | 4 | `sp -0x28` | `(ret(0x1b358) + [sp-28])` |
| `ldr` at 0xeac14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xeac1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeac28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeac2c | load | 4 | `abs 0xeac94` | `-` |
| `ldr` at 0xeac34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeac44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xeac4c | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1b2bc))` |
| `ldr` at 0xeac50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xeac54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeac5c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xeac60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeac68 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xeac70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeac78 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xeac80 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xeac88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeac88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeac88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeabc4 | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xeabf8 | `(unknown callee) 0x1b358` | `[sp-36]` | `[sp-32]+50` | `[sp-32]` | `[sp-32]+50` |
| `bl` at 0xeac24 | `(unknown callee) 0x1b598` | `[sp-40]` | `[sp-48]+4` | `?` | `[sp-48]+4` |
| `bl` at 0xeac30 | `(unknown callee) 0x1b364` | `[sp-40]` | `0x137048` | `?` | `?` |
| `bl` at 0xeac38 | `(unknown callee) 0x1b2bc` | `[sp-40]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xeab4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xeab84 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0xeabe0 | `ret(0x1b2bc)+2 <= ? (unsigned, 32-bit)` | `ret(0x1b2bc)+2 > ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeac88 | `r0` | `?` |

## `JChipGetChip`

- `JChipGetChip`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebc18 to 0xebd58.
  Established. A four-byte global load reads absolute `0x2ae108` (`ldr` at 0xebc40). Included
  four-byte reads target `[sp-28] +0xc` and `[sp-28] +0x8` (`ldr` at 0xebc80, `ldr` at 0xebcb0). The
  byte assignment at `sp -0x21` has unknown value (`strb` at 0xebc6c). The span contains no calls.
  Chip lookup encoding and result meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebc28 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xebc2c | load | 4 | `abs 0xebd54` | `-` |
| `ldr` at 0xebc30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebc40 | load | 4 | `abs 0x2ae108` | `-` |
| `str` at 0xebc48 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xebc4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebc58 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xebc64 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0xebc6c | store | 1 | `sp -0x21` | `?` |
| `str` at 0xebc74 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xebc78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebc7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebc80 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0xebc90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebc94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebcb0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xebcb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebcbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebcc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebccc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebcd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebcec | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xebcf4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xebcf8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xebd04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xebd08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebd24 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0xebd2c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xebd34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xebd3c | store | 4 | `sp -0x18` | `?` |
| `str` at 0xebd48 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xebd4c | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xebd50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebd50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebd50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xebc88 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xebcc8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xebd00 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebd50 | `r0` | `?` |

## `JChipGetType`

- `JChipGetType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebb6c to 0xebc18.
  Established. A four-byte load reads absolute `0x143874` (`ldr` at 0xebb90). The conditional branch
  takes `? != [sp-20]` to `0xebbf0` (`bne` at 0xebbc8). Included four-byte assignments at `sp -0x1c`
  have unknown value and `0x0` at separate sites (`str` at 0xebbe8, `str` at 0xebc04). No calls
  appear in this span. Type descriptor meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebb7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebb84 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xebb88 | load | 4 | `abs 0xebc10` | `-` |
| `ldr` at 0xebb8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebb90 | load | 4 | `abs 0x143874` | `-` |
| `ldr` at 0xebba0 | load | 4 | `abs 0xebc14` | `-` |
| `ldr` at 0xebba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebbbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xebbc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebbcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebbe0 | load | 4 | `abs 0xebc14` | `-` |
| `str` at 0xebbe8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xebbf0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xebbf8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xebc04 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xebc08 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xebc0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebc0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebc0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xebb98 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xebbc8 | `? != [sp-20]` | `? == [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebc0c | `r0` | `?` |

## `JChip_ir_load`

- `JChip_ir_load`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xecd30 to 0xed170.
  Established. The halfword read targets `[[sp-48]] +0xe` (`ldrh` at 0xece04). Included four-byte
  assignments at absolute `0x2ae004` write `0x0` and `[0x2ae004]+1` (`str` at 0xecfb0, `str` at
  0xecff8). The unsigned comparison takes `[0x2ae004] < [sp-32]` to `0xecfcc` (`blo` at 0xecfc4).
  The unnamed `0xdd8e8` call has `0x0`, `0x1`, `0x1`, `[sp-64]` (`bl` at 0xed06c).
  Instruction-register encoding and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xecd40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xecd44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xecd48 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xecd50 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xecd54 | load | 4 | `abs 0xed124` | `-` |
| `ldr` at 0xecd58 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xecd64 | load | 4 | `abs 0xed128` | `-` |
| `ldr` at 0xecd68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xecd6c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecd70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecd78 | load | 4 | `abs 0xed12c` | `-` |
| `str` at 0xecd7c | store | 4 | `sp -0x38` | `0x2ae008` |
| `ldr` at 0xecd80 | load | 4 | `abs 0xed130` | `-` |
| `str` at 0xecd84 | store | 4 | `sp -0x3c` | `0x2ae088` |
| `ldr` at 0xecd88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xecd94 | store | 4 | `sp -0x30` | `ret(0xebc18)` |
| `ldr` at 0xecd98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xecda4 | load | 4 | `abs 0xed134` | `-` |
| `ldr` at 0xecda8 | load | 4 | `abs 0xed138` | `-` |
| `ldr` at 0xecdac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xecdb4 | load | 4 | `abs 0xed134` | `-` |
| `str` at 0xecdb8 | store | 4 | `sp -0x58` | `0x2b47f8` |
| `ldr` at 0xecdc0 | load | 4 | `abs 0xed13c` | `-` |
| `ldr` at 0xecdc4 | load | 4 | `abs 0xed140` | `-` |
| `ldr` at 0xecdd0 | load | 4 | `abs 0xed144` | `-` |
| `str` at 0xecdd4 | store | 4 | `sp -0x48` | `0xfffff7c6` |
| `ldr` at 0xecddc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xecdf0 | load | 4 | `abs 0xed148` | `-` |
| `str` at 0xecdf4 | store | 4 | `sp -0x48` | `0xfffff7c5` |
| `ldr` at 0xecdfc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xece00 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrh` at 0xece04 | load | 2 | `[[sp-48]] +0xe` | `-` |
| `str` at 0xece18 | store | 4 | `sp -0x40` | `0x1` |
| `str` at 0xece20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xece24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xece28 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldrb` at 0xece2c | load | 1 | `[[sp-48]] +0xc` | `-` |
| `str` at 0xece30 | store | 4 | `sp -0x28` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xece34 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xece38 | store | 4 | `sp -0x2c` | `[[[sp-48]]+12]:1` |
| `ldr` at 0xece40 | load | 4 | `sp -0x30` | `-` |
| `ldrsb` at 0xece44 | load | 1 | `[sp-48] +0x21` | `-` |
| `str` at 0xece48 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xece4c | load | 4 | `abs 0xed14c` | `-` |
| `ldr` at 0xece50 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xece54 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xece74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xece7c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xece84 | load | 2 | `(unknown)` | `-` |
| `str` at 0xece88 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xece8c | load | 4 | `abs 0xed14c` | `-` |
| `ldr` at 0xece90 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xece94 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xeceb4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xecebc | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xecec4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xecec8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xececc | load | 4 | `abs 0xed14c` | `-` |
| `ldr` at 0xeced0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xeced4 | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xecef4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xecef8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xecefc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xecf00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xecf08 | load | 4 | `[sp-48] +0x28` | `-` |
| `ldr` at 0xecf10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xecf14 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xecf18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xecf28 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecf2c | load | 4 | `abs 0xed134` | `-` |
| `ldr` at 0xecf40 | load | 4 | `abs 0xed134` | `-` |
| `str` at 0xecf44 | store | 4 | `sp -0x58` | `0x2b47f8` |
| `ldr` at 0xecf4c | load | 4 | `abs 0xed13c` | `-` |
| `ldr` at 0xecf50 | load | 4 | `abs 0xed150` | `-` |
| `ldr` at 0xecf5c | load | 4 | `abs 0xed154` | `-` |
| `str` at 0xecf60 | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xecf68 | load | 4 | `abs 0xed124` | `-` |
| `ldr` at 0xecf6c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xecf78 | load | 4 | `abs 0xed158` | `-` |
| `ldr` at 0xecf7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xecf84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xecf98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xecfa4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xecfa8 | load | 4 | `abs 0xed15c` | `-` |
| `str` at 0xecfb0 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xecfb4 | load | 4 | `abs 0xed15c` | `-` |
| `ldr` at 0xecfb8 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xecfbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xecfcc | load | 4 | `abs 0xed15c` | `-` |
| `ldr` at 0xecfd0 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xecfd8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xecfe4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xecfe8 | load | 4 | `abs 0xed15c` | `-` |
| `ldr` at 0xecfec | load | 4 | `abs 0xed15c` | `-` |
| `ldr` at 0xecff0 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xecff8 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xed000 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed004 | load | 4 | `sp -0x34` | `-` |
| `ldrh` at 0xed008 | load | 2 | `[sp-52] +0x4` | `-` |
| `str` at 0xed00c | store | 4 | `[sp-60] +0x0` | `[[sp-52]+4]:2` |
| `ldr` at 0xed018 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed020 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xed024 | store | 4 | `sp -0x58` | `[sp-40]` |
| `ldr` at 0xed02c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xed030 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xed034 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed03c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xed040 | store | 4 | `sp -0x58` | `[sp-44]` |
| `ldr` at 0xed044 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xed048 | store | 4 | `sp -0x54` | `[sp-56]` |
| `ldr` at 0xed04c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xed050 | store | 4 | `sp -0x50` | `[sp-60]` |
| `ldr` at 0xed054 | load | 4 | `abs 0xed134` | `-` |
| `str` at 0xed058 | store | 4 | `sp -0x4c` | `0x2b47f8` |
| `ldr` at 0xed068 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed07c | load | 4 | `abs 0xed134` | `-` |
| `str` at 0xed080 | store | 4 | `sp -0x58` | `0x2b47f8` |
| `ldr` at 0xed088 | load | 4 | `abs 0xed13c` | `-` |
| `ldr` at 0xed08c | load | 4 | `abs 0xed160` | `-` |
| `ldr` at 0xed098 | load | 4 | `abs 0xed154` | `-` |
| `str` at 0xed09c | store | 4 | `sp -0x48` | `0xfffff7ca` |
| `ldr` at 0xed0a4 | load | 4 | `abs 0xed124` | `-` |
| `ldr` at 0xed0a8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0xed0b4 | load | 4 | `abs 0xed14c` | `-` |
| `ldr` at 0xed0b8 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xed0bc | load | 1 | `[sp-48] +0x19` | `-` |
| `ldr` at 0xed0dc | load | 4 | `abs 0xed164` | `-` |
| `ldr` at 0xed0e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xed0e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xed0f0 | load | 4 | `abs 0xed168` | `-` |
| `ldr` at 0xed0f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed0f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xed104 | load | 4 | `abs 0xed16c` | `-` |
| `ldr` at 0xed108 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed10c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xed118 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xed11c | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xed120 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xed120 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xed120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xecd74 | `(unknown callee) 0x23fbc` | `0x13758c` | `r0` | `r1` | `r2` |
| `bl` at 0xecd8c | `(unknown callee) 0xebc18` | `?` | `?` | `?` | `0x2ae088` |
| `bl` at 0xecdb0 | `(unknown callee) 0x1b52c` | `0x2b47f8` | `0x1375c0` | `[sp-20]` | `ret(0xebc18)` |
| `bl` at 0xecdcc | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x568` | `0x0` |
| `bl` at 0xecde0 | `(unknown callee) 0xe9a1c` | `ret(0xebc18)` | `?` | `?` | `ret(0xebc18)` |
| `bl` at 0xecf30 | `(unknown callee) 0xdd3f8` | `0x1` | `?` | `0x2b47f8` | `[sp-24]` |
| `bl` at 0xecf58 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x589` | `0x0` |
| `bl` at 0xecf80 | `(unknown callee) 0x23fbc` | `0x1375f0` | `[sp-64]` | `?` | `[0x2adfd8]` |
| `bl` at 0xed01c | `(unknown callee) 0xe2cf8` | `0x0` | `0x20` | `[sp-60]` | `[[sp-52]+4]:2` |
| `bl` at 0xed038 | `(unknown callee) 0xe2b10` | `0x0` | `[sp-60]` | `[sp-36]` | `[sp-56]` |
| `bl` at 0xed06c | `(unknown callee) 0xdd8e8` | `0x0` | `0x1` | `0x1` | `[sp-64]` |
| `bl` at 0xed094 | `(unknown callee) 0x21eb8` | `0x3` | `0x136ed4` | `0x59d` | `0x0` |
| `bl` at 0xed0e8 | `(unknown callee) 0x23fbc` | `0x137628` | `[sp-64]` | `?` | `?` |
| `bl` at 0xed0fc | `(unknown callee) 0xf81f4` | `0x0` | `0x137684` | `[sp-32]` | `[sp-56]` |
| `bl` at 0xed110 | `(unknown callee) 0xf81f4` | `0x0` | `0x137690` | `[sp-32]` | `[sp-60]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xecd60 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xecda0 | `ret(0xebc18) != 0x0` | `ret(0xebc18) == 0x0` |
| `beq` at 0xecdec | `ret(0xe9a1c) == 0x0` | `ret(0xe9a1c) != 0x0` |
| `beq` at 0xece10 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xecf20 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bne` at 0xecf3c | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `bls` at 0xecf74 | `[0x2adfd8] <= 0x3 (unsigned, 32-bit)` | `[0x2adfd8] > 0x3 (unsigned, 32-bit)` |
| `blo` at 0xecfc4 | `[0x2ae004] < [sp-32] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-32] (unsigned, 32-bit)` |
| `bne` at 0xed078 | `ret(0xdd8e8) != 0x0` | `ret(0xdd8e8) == 0x0` |
| `bls` at 0xed0b0 | `[0x2adfd8] <= 0x2 (unsigned, 32-bit)` | `[0x2adfd8] > 0x2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xed120 | `r0` | `?` |

## `JChipName`

- `JChipName`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeaab4 to 0xeab04.
  Established. Zero result from the unnamed `0xebc18` call takes the branch to `0xeaaf4` (`bl` at
  0xeaacc, `beq` at 0xeaae0). The alternative four-byte assignments at `sp -0x1c` retain
  `ret(0xebc18)+4` and `0x0` (`str` at 0xeaaec, `str` at 0xeaaf8). The merged exit value is unknown
  (`ldmdb` at 0xeab00). Name storage layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeaac4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xeaac8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeaad4 | store | 4 | `sp -0x18` | `ret(0xebc18)` |
| `ldr` at 0xeaad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeaae4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeaaec | store | 4 | `sp -0x1c` | `ret(0xebc18)+4` |
| `str` at 0xeaaf8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xeaafc | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xeab00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xeab00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xeab00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeaacc | `(unknown callee) 0xebc18` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xeaae0 | `ret(0xebc18) == 0x0` | `ret(0xebc18) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xeab00 | `r0` | `?` |

## `jchip_dump_io`

- `jchip_dump_io`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec0d0 to 0xec1ac.
  Established. The two unnamed `0x1b52c` calls have `sp-72`, `0x1372a8`, `r0`, `r1`, and `sp-72`,
  `0x1372b4`, `[sp-20]`, `[sp-24]` (`bl` at 0xec104, `bl` at 0xec15c). Included unnamed `0xf81f4`
  calls carry `0x0`, `sp-72`, unknown, `[sp-32]`, and `0x0`, `sp-72`, unknown, `[sp]` (`bl` at
  0xec144, `bl` at 0xec19c). No conditional branches appear in this span. Dump data interpretation
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec0e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec0e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xec0e8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xec0ec | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec0f8 | load | 4 | `abs 0xec1a4` | `-` |
| `ldr` at 0xec0fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec100 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec10c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec120 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec150 | load | 4 | `abs 0xec1a8` | `-` |
| `ldr` at 0xec154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec158 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec178 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xec198 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xec1a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec1a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec1a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec104 | `(unknown callee) 0x1b52c` | `sp-72` | `0x1372a8` | `r0` | `r1` |
| `bl` at 0xec144 | `(unknown callee) 0xf81f4` | `0x0` | `sp-72` | `?` | `[sp-32]` |
| `bl` at 0xec15c | `(unknown callee) 0x1b52c` | `sp-72` | `0x1372b4` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xec19c | `(unknown callee) 0xf81f4` | `0x0` | `sp-72` | `?` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec1a0 | `r0` | `ret(0xf81f4)` |
