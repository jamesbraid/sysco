# ARM configuration records and protocol callers in 1.40.5

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
L1CfgGetCfg has an unresolved computed transfer (`ldrls` at 0x751b0); its tables and return cover known edges only. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/configuration-access-facts.txt) retain all tool sections.

## `L1CfgReset`

- `L1CfgReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ba6c to 0x6bb3c.
  Established. An absolute four-byte store at `0x149738` writes `0x0` (`str` at 0x6ba84). Included
  four-byte stack stores at `sp -0x30` and `sp -0x2c` retain `[0x1496b8]` and `[0x1496bc]` (`str` at
  0x6bac4, `str` at 0x6bad0). The unnamed `0x71708` call has `0x149740`, `0x149738`, `0x14973c`,
  `[[sp-24]+8]` (`bl` at 0x6bb24). Reset and record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6ba7c | load | 4 | `abs 0x6bb2c` | `-` |
| `str` at 0x6ba84 | store | 4 | `abs 0x149738` | `0x0` |
| `ldr` at 0x6ba88 | load | 4 | `abs 0x6bb2c` | `-` |
| `ldr` at 0x6ba8c | load | 4 | `abs 0x6bb2c` | `-` |
| `ldr` at 0x6ba90 | load | 4 | `abs 0x14973c` | `-` |
| `ldr` at 0x6baa4 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x6bab4 | load | 4 | `abs 0x6bb2c` | `-` |
| `str` at 0x6bab8 | store | 4 | `sp -0x18` | `0x1496ac` |
| `ldr` at 0x6babc | load | 4 | `abs 0x6bb2c` | `-` |
| `ldr` at 0x6bac0 | load | 4 | `abs 0x1496b8` | `-` |
| `str` at 0x6bac4 | store | 4 | `sp -0x30` | `[0x1496b8]` |
| `ldr` at 0x6bac8 | load | 4 | `abs 0x6bb2c` | `-` |
| `ldr` at 0x6bacc | load | 4 | `abs 0x1496bc` | `-` |
| `str` at 0x6bad0 | store | 4 | `sp -0x2c` | `[0x1496bc]` |
| `ldr` at 0x6bad4 | load | 4 | `abs 0x6bb2c` | `-` |
| `ldr` at 0x6bad8 | load | 4 | `abs 0x149704` | `-` |
| `str` at 0x6bae8 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x6baf4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x6baf8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6bafc | store | 4 | `sp -0x28` | `[sp-28]` |
| `str` at 0x6bb04 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x6bb0c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6bb10 | load | 4 | `abs 0x6bb30` | `-` |
| `ldr` at 0x6bb14 | load | 4 | `abs 0x6bb34` | `-` |
| `ldr` at 0x6bb18 | load | 4 | `abs 0x6bb38` | `-` |
| `ldr` at 0x6bb1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6bb20 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0x6bb28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bb28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bb28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6bab0 | `(unknown callee) 0x1b4cc` | `[0x149740]` | `0x0` | `?` | `?` |
| `bl` at 0x6bb24 | `(unknown callee) 0x71708` | `0x149740` | `0x149738` | `0x14973c` | `[[sp-24]+8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6bae0 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bb28 | `r0` | `ret(0x71708)` |

## `L1CfgRefresh`

- `L1CfgRefresh`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bb3c to 0x6bbac.
  Established. Included unnamed `0x72734` calls have first candidates `0x2`, `0xd`, `0x14`, `0x15`
  (`bl` at 0x6bb4c, `bl` at 0x6bb54, `bl` at 0x6bb5c, `bl` at 0x6bb64). The final call has `0xf`,
  unknown, unknown, unknown (`bl` at 0x6bba4); its result is the printed `r0` return (`ldmdb` at
  0x6bba8). Request-code meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x6bba8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6bba8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6bba8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6bb4c | `(unknown callee) 0x72734` | `0x2` | `r1` | `r2` | `r3` |
| `bl` at 0x6bb54 | `(unknown callee) 0x72734` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x6bb5c | `(unknown callee) 0x72734` | `0x14` | `?` | `?` | `?` |
| `bl` at 0x6bb64 | `(unknown callee) 0x72734` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x6bb6c | `(unknown callee) 0x72734` | `0x16` | `?` | `?` | `?` |
| `bl` at 0x6bb74 | `(unknown callee) 0x72734` | `0x17` | `?` | `?` | `?` |
| `bl` at 0x6bb7c | `(unknown callee) 0x72734` | `0x18` | `?` | `?` | `?` |
| `bl` at 0x6bb84 | `(unknown callee) 0x72734` | `0x19` | `?` | `?` | `?` |
| `bl` at 0x6bb8c | `(unknown callee) 0x72734` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6bb94 | `(unknown callee) 0x72734` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x6bb9c | `(unknown callee) 0x72734` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x6bba4 | `(unknown callee) 0x72734` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6bba8 | `r0` | `ret(0x72734)` |

## `L1CfgLock`

- `L1CfgLock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c95c to 0x6c9f4.
  Established. The unnamed `0x22ac8` call carries `0x14964c`, `0xc8`, `r2`, `r3` (`bl` at 0x6c978); its zero result
  takes the branch to `0x6c9cc` (`beq` at 0x6c98c). Included four-byte stack stores
  at `sp -0x20` and `sp -0x1c` write `0x2b23a8` and `0x2b23d0` (`str` at 0x6c9ac, `str` at 0x6c9b4).
  The later unnamed `0x1b598` call has `0x2b23a8`, `[sp-20]`, unknown, unknown (`bl` at 0x6c9d4).
  Lock ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6c96c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6c970 | load | 4 | `abs 0x6c9dc` | `-` |
| `str` at 0x6c980 | store | 4 | `sp -0x18` | `ret(0x22ac8)` |
| `ldr` at 0x6c984 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6c990 | load | 4 | `abs 0x6c9e0` | `-` |
| `str` at 0x6c994 | store | 4 | `sp -0x2c` | `0x10fd20` |
| `ldr` at 0x6c998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6c99c | store | 4 | `sp -0x28` | `ret(0x22ac8)` |
| `ldr` at 0x6c9a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6c9a4 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x6c9a8 | load | 4 | `abs 0x6c9e4` | `-` |
| `str` at 0x6c9ac | store | 4 | `sp -0x20` | `0x2b23a8` |
| `ldr` at 0x6c9b0 | load | 4 | `abs 0x6c9e8` | `-` |
| `str` at 0x6c9b4 | store | 4 | `sp -0x1c` | `0x2b23d0` |
| `ldr` at 0x6c9bc | load | 4 | `abs 0x6c9ec` | `-` |
| `ldr` at 0x6c9c0 | load | 4 | `abs 0x6c9f0` | `-` |
| `ldr` at 0x6c9cc | load | 4 | `abs 0x6c9e4` | `-` |
| `ldr` at 0x6c9d0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6c9d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c9d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c9d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c978 | `(unknown callee) 0x22ac8` | `0x14964c` | `0xc8` | `r2` | `r3` |
| `bl` at 0x6c9c8 | `(unknown callee) 0x21eb8` | `0x9` | `0x10fc24` | `0x27b` | `0x0` |
| `bl` at 0x6c9d4 | `(unknown callee) 0x1b598` | `0x2b23a8` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c98c | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c9d8 | `r0` | `ret(0x1b598)` |

## `L1CfgUnlock`

- `L1CfgUnlock`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c9f4 to 0x6ca7c.
  Established. Two unnamed `0x1b598` calls carry `0x2b23d0`, `0x2b23a8`, `r2`, `r3`, and `0x2b23a8`,
  `0x10fd58`, unknown, unknown (`bl` at 0x6ca0c, `bl` at 0x6ca18). The unnamed `0x22e50` call has
  `0x14964c`, unknown, unknown, unknown (`bl` at 0x6ca20); its zero result takes the branch
  to `0x6ca5c` (`beq` at 0x6ca34). Release semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6ca04 | load | 4 | `abs 0x6ca60` | `-` |
| `ldr` at 0x6ca08 | load | 4 | `abs 0x6ca64` | `-` |
| `ldr` at 0x6ca10 | load | 4 | `abs 0x6ca64` | `-` |
| `ldr` at 0x6ca14 | load | 4 | `abs 0x6ca68` | `-` |
| `ldr` at 0x6ca1c | load | 4 | `abs 0x6ca6c` | `-` |
| `str` at 0x6ca28 | store | 4 | `sp -0x14` | `ret(0x22e50)` |
| `ldr` at 0x6ca2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ca38 | load | 4 | `abs 0x6ca70` | `-` |
| `str` at 0x6ca3c | store | 4 | `sp -0x1c` | `0x10fd64` |
| `ldr` at 0x6ca40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6ca44 | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0x6ca4c | load | 4 | `abs 0x6ca74` | `-` |
| `ldr` at 0x6ca50 | load | 4 | `abs 0x6ca78` | `-` |
| `ldmdb` at 0x6ca5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ca5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ca5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ca0c | `(unknown callee) 0x1b598` | `0x2b23d0` | `0x2b23a8` | `r2` | `r3` |
| `bl` at 0x6ca18 | `(unknown callee) 0x1b598` | `0x2b23a8` | `0x10fd58` | `?` | `?` |
| `bl` at 0x6ca20 | `(unknown callee) 0x22e50` | `0x14964c` | `?` | `?` | `?` |
| `bl` at 0x6ca58 | `(unknown callee) 0x21eb8` | `0x9` | `0x10fc24` | `0x28b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6ca34 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ca5c | `r0` | `?` |

## `L1CfgProcessResponse`

- `L1CfgProcessResponse`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73034 to 0x73104.
  Established. A byte read at `r0 +0x13` precedes the branch routing `[r0+19]:1 != 0x0` to `0x730f0`
  (`ldrb` at 0x7304c, `bne` at 0x73054). A later four-byte absolute assignment at `0x1493f0` writes
  `ret(0x1b274)` (`str` at 0x730dc). The unnamed `0x6f354` call has `[sp-24]`, `0x0`, unknown,
  `[sp-36]` (`bl` at 0x730ec). Response interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73044 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x73048 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7304c | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x73058 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7305c | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x7306c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73070 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x73074 | store | 4 | `sp -0x24` | `[r0+40]` |
| `ldr` at 0x7307c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73088 | store | 4 | `sp -0x24` | `ret(0xb823c)` |
| `ldr` at 0x7308c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x73090 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x73094 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x730a4 | load | 4 | `abs 0x730f4` | `-` |
| `ldr` at 0x730a8 | load | 4 | `abs 0x1493d4` | `-` |
| `ldr` at 0x730b8 | load | 4 | `abs 0x730f8` | `-` |
| `ldr` at 0x730bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x730c0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0x730c4 | load | 4 | `abs 0x730fc` | `-` |
| `ldr` at 0x730c8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x730d0 | load | 4 | `abs 0x73100` | `-` |
| `str` at 0x730dc | store | 4 | `abs 0x1493f0` | `ret(0x1b274)` |
| `ldr` at 0x730e4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x730f0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x730f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x730f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x730f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x73084 | `(unknown callee) 0xb823c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x730cc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x110468` | `[[sp-24]+12]` | `0x145ae8` |
| `bl` at 0x730d8 | `(unknown callee) 0x1b274` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x730ec | `(unknown callee) 0x6f354` | `[sp-24]` | `0x0` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x73054 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `bne` at 0x73068 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7309c | `[sp-36] == 0x3e8` | `[sp-36] != 0x3e8` |
| `beq` at 0x730b4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x730f0 | `r0` | `?` |

## `L1CfgProcessEvent`

- `L1CfgProcessEvent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73104 to 0x73278.
  Established. The halfword read at `r0 +0x22` is saved as a four-byte value at `sp -0x8c` (`ldrh`
  at 0x7312c, `str` at 0x73130). The branch takes `[r0+34]:2 == 0x6` to `0x7322c`, followed by the
  unnamed `0x763a4` call with `r0`, `r1`, `[r0+34]:2`, `[r0+34]:2` (`beq` at 0x7313c, `bl` at
  0x73230). Event-code meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73114 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x73118 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x7311c | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x73128 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x7312c | load | 2 | `r0 +0x22` | `-` |
| `str` at 0x73130 | store | 4 | `sp -0x8c` | `[r0+34]:2` |
| `ldr` at 0x73134 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x73140 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x7314c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x73158 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x73164 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x73170 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x73180 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x7318c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x7319c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731a8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731b4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731c0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731d0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731dc | load | 4 | `abs 0x73274` | `-` |
| `ldr` at 0x731e0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x731f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x731fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7322c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7325c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73268 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x73270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x73270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x73270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x731f4 | `(unknown callee) 0x75e20` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73200 | `(unknown callee) 0x75b80` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x7320c | `(unknown callee) 0x76024` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73218 | `(unknown callee) 0x76194` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73224 | `(unknown callee) 0x7629c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73230 | `(unknown callee) 0x763a4` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x7323c | `(unknown callee) 0x764ac` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73248 | `(unknown callee) 0x765bc` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73254 | `(unknown callee) 0x76db4` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x73260 | `(unknown callee) 0x7749c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x7326c | `(unknown callee) 0x7210c` | `r0` | `r1` | `[r0+34]:2` | `0x3e9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x73124 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `beq` at 0x7313c | `[r0+34]:2 == 0x6` | `[r0+34]:2 != 0x6` |
| `bgt` at 0x73148 | `[r0+34]:2 > 0x6 (signed, 32-bit)` | `[r0+34]:2 <= 0x6 (signed, 32-bit)` |
| `beq` at 0x73154 | `[r0+34]:2 == 0x3` | `[r0+34]:2 != 0x3` |
| `bgt` at 0x73160 | `[r0+34]:2 > 0x3 (signed, 32-bit)` | `[r0+34]:2 <= 0x3 (signed, 32-bit)` |
| `beq` at 0x7316c | `[r0+34]:2 == 0x1` | `[r0+34]:2 != 0x1` |
| `beq` at 0x73178 | `[r0+34]:2 == 0x2` | `[r0+34]:2 != 0x2` |
| `beq` at 0x73188 | `[r0+34]:2 == 0x4` | `[r0+34]:2 != 0x4` |
| `beq` at 0x73194 | `[r0+34]:2 == 0x5` | `[r0+34]:2 != 0x5` |
| `beq` at 0x731a4 | `[r0+34]:2 == 0x64` | `[r0+34]:2 != 0x64` |
| `bgt` at 0x731b0 | `[r0+34]:2 > 0x64 (signed, 32-bit)` | `[r0+34]:2 <= 0x64 (signed, 32-bit)` |
| `beq` at 0x731bc | `[r0+34]:2 == 0x7` | `[r0+34]:2 != 0x7` |
| `beq` at 0x731c8 | `[r0+34]:2 == 0x8` | `[r0+34]:2 != 0x8` |
| `beq` at 0x731d8 | `[r0+34]:2 == 0x66` | `[r0+34]:2 != 0x66` |
| `beq` at 0x731e8 | `[r0+34]:2 == 0x3e9` | `[r0+34]:2 != 0x3e9` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x73270 | `r0` | `?` |

## `L1CfgIRouter`

- `L1CfgIRouter`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73278 to 0x73334.
  Established. The branch takes `[r0+22]:1 != 0x0` to `0x73310` (`bne` at 0x73300); its fall-through
  path calls unnamed `0x72978` with `r0`, `r1`, `r2`, `[r0+22]:1` (`bl` at 0x73308). Another arm
  includes unnamed `0x73034` with `r0`, `r1`, `r2`, `[r0+22]:1` (`bl` at 0x73324). Packet
  classification remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73288 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x73290 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x73294 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x73298 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x732a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x732a8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x732b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x732b8 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x732d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x732d4 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0x732e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x732f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x732f8 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x73304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73310 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x73314 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0x73320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73328 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x73330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x73330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x73330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x732ec | `(unknown callee) 0x73104` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0x73308 | `(unknown callee) 0x72978` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x73324 | `(unknown callee) 0x73034` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x7332c | `(unknown callee) 0xb5fec` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x732a0 | `[r0+22]:1 != 0x2` | `[r0+22]:1 == 0x2` |
| `bge` at 0x732b0 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x732c8 | `? == 0xe` | `? != 0xe` |
| `beq` at 0x732e0 | `? == 0xe` | `? != 0xe` |
| `bne` at 0x73300 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `bne` at 0x7331c | `[r0+22]:1 != 0x1` | `[r0+22]:1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x73330 | `r0` | `ret(0xb5fec)` |

## `L1CfgGetBrickType`

- `L1CfgGetBrickType`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f0c to 0x74f94.
  Established. The unnamed `0x70010` call carries `[0x149740]`, `[0x149738]`, `[sp-20]`, `[sp-24]`
  (`bl` at 0x74f44); its zero result takes the branch to `0x74f6c` (`beq` at 0x74f58). Separate
  four-byte assignments at `sp -0x24` retain `[ret(0x70010)+8]` and `0x3f` (`str` at 0x74f64, `str`
  at 0x74f70). Brick-type encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74f1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74f20 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x74f24 | load | 4 | `abs 0x74f8c` | `-` |
| `ldr` at 0x74f2c | load | 4 | `abs 0x74f90` | `-` |
| `ldr` at 0x74f30 | load | 4 | `abs 0x74f90` | `-` |
| `ldr` at 0x74f34 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x74f38 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x74f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74f40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74f4c | store | 4 | `sp -0x1c` | `ret(0x70010)` |
| `ldr` at 0x74f50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74f5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74f60 | load | 4 | `ret(0x70010) +0x8` | `-` |
| `str` at 0x74f64 | store | 4 | `sp -0x24` | `[ret(0x70010)+8]` |
| `str` at 0x74f70 | store | 4 | `sp -0x24` | `0x3f` |
| `ldr` at 0x74f74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x74f78 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x74f80 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x74f88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74f88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74f88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74f28 | `(unknown callee) 0x6c95c` | `0x1107d8` | `r1` | `r2` | `r3` |
| `bl` at 0x74f44 | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `[sp-20]` | `[sp-24]` |
| `bl` at 0x74f7c | `(unknown callee) 0x6c9f4` | `ret(0x70010)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74f58 | `ret(0x70010) == 0x0` | `ret(0x70010) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74f88 | `r0` | `[sp-32]` |

## `L1CfgGetBrickCfg`

- `L1CfgGetBrickCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74f94 to 0x74fe4.
  Established. Two absolute four-byte reads target `0x14974c` and `0x149744` (`ldr` at 0x74fbc,
  `ldr` at 0x74fc0). The unnamed `0x7012c` call has `[0x14974c]`, `[0x149744]`, `r0`, `r1`, and the
  printed return retains its result (`bl` at 0x74fcc, `ldmdb` at 0x74fdc). Configuration-record
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74fa4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74fa8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74fb0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x74fb4 | load | 4 | `abs 0x74fe0` | `-` |
| `ldr` at 0x74fb8 | load | 4 | `abs 0x74fe0` | `-` |
| `ldr` at 0x74fbc | load | 4 | `abs 0x14974c` | `-` |
| `ldr` at 0x74fc0 | load | 4 | `abs 0x149744` | `-` |
| `ldr` at 0x74fc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74fc8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x74fd0 | store | 4 | `sp -0x1c` | `ret(0x7012c)` |
| `ldr` at 0x74fd4 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x74fdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74fdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74fcc | `(unknown callee) 0x7012c` | `[0x14974c]` | `[0x149744]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74fdc | `r0` | `ret(0x7012c)` |

## `L1CfgGetBrickIndex`

- `L1CfgGetBrickIndex`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74fe4 to 0x750dc.
  Established. Included four-byte reads target absolute `0x149738` and `0x149740` (`ldr` at 0x75020,
  `ldr` at 0x75048, `ldr` at 0x75078). Two other four-byte reads have unknown effective targets
  (`ldr` at 0x75050, `ldr` at 0x75080). The later four-byte stack assignment at `sp -0x1c` writes
  `0x1` (`str` at 0x75094). Search predicates and index meaning remain Unknown where the facts
  expose flags only.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74ff4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74ff8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x75000 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x75008 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x7500c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x75018 | load | 4 | `abs 0x750d8` | `-` |
| `ldr` at 0x7501c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x75020 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x75030 | load | 4 | `abs 0x750d8` | `-` |
| `ldr` at 0x75034 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x75048 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x75050 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x75054 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75060 | load | 4 | `abs 0x750d8` | `-` |
| `ldr` at 0x75064 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x75078 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x75080 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x75084 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x75094 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x75098 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x750a0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x750a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x750b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x750bc | store | 4 | `sp -0x24` | `?` |
| `str` at 0x750c8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x750cc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x750d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x750d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x750d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x75014 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x75028 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x7505c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7508c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x750b0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x750d4 | `r0` | `[sp-36]` |

## `L1CfgGetIndexBrick`

- `L1CfgGetIndexBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x750dc to 0x75150.
  Established. A signed negative `r0` takes the branch to `0x75114`, followed by a four-byte stack
  assignment of `0x0` at `sp -0x18` (`blt` at 0x750f8, `str` at 0x75118). Another branch takes
  `r0 >= [0x149738]`, signed 32-bit, to the same target (`bge` at 0x7510c). The four-byte store at
  `sp -0x18` on the other arm has unknown value (`str` at 0x75140). Indexed record layout remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x750ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x750f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x750fc | load | 4 | `abs 0x7514c` | `-` |
| `ldr` at 0x75100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75104 | load | 4 | `abs 0x149738` | `-` |
| `str` at 0x75118 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x75120 | load | 4 | `abs 0x7514c` | `-` |
| `ldr` at 0x75124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75138 | load | 4 | `abs 0x149740` | `-` |
| `str` at 0x75140 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x75144 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x75148 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75148 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75148 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x750f8 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `bge` at 0x7510c | `r0 >= [0x149738] (signed, 32-bit)` | `r0 < [0x149738] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75148 | `r0` | `?` |

## `L1CfgGetBrick`

- `L1CfgGetBrick`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75150 to 0x75194.
  Established. The unnamed `0x70010` call uses `[0x149740]`, `[0x149738]`, `r0`, `r1` (`bl` at 0x75180); its result is
  the printed return (`ldmdb` at 0x7518c). The first two candidates come from
  separate four-byte absolute reads at `0x149740` and `0x149738` (`ldr` at 0x75170, `ldr` at
  0x75174). Lookup semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75160 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75164 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x75168 | load | 4 | `abs 0x75190` | `-` |
| `ldr` at 0x7516c | load | 4 | `abs 0x75190` | `-` |
| `ldr` at 0x75170 | load | 4 | `abs 0x149740` | `-` |
| `ldr` at 0x75174 | load | 4 | `abs 0x149738` | `-` |
| `ldr` at 0x75178 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7517c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x7518c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7518c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7518c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75180 | `(unknown callee) 0x70010` | `[0x149740]` | `[0x149738]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7518c | `r0` | `ret(0x70010)` |

## `L1CfgGetCfg`

- `L1CfgGetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75194 to 0x75344.
  Established. The computed transfer takes unsigned `r0 <= 0x28` to an unresolved destination and
  falls through when `r0 > 0x28` (`ldrls` at 0x751b0). On the known fall-through route, a four-byte
  stack assignment at `sp -0x28` retains `0x149950`, which is also the printed return on known edges
  (`str` at 0x752fc, `ldmdb` at 0x75338). Selector-specific record mappings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x751a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x751a8 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x751b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x752f8 | load | 4 | `abs 0x75340` | `-` |
| `str` at 0x752fc | store | 4 | `sp -0x28` | `0x149950` |
| `ldr` at 0x75334 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x75338 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75338 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75338 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x751b0 | `r0 <= 0x28 (unsigned, 32-bit)` | `r0 > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75338 | `r0` | `0x149950` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x751b0).

## `L1CfgGetL2`

- `L1CfgGetL2`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75344 to 0x753bc.
  Established. Zero entry `r0` takes the branch to `0x7537c` (`beq` at 0x75368). The fall-through
  path includes a four-byte assignment through `r0 +0x0` with `[0x149720]` (`str` at 0x75378). Two
  later four-byte stores have unknown effective targets and values `[0x149724]` and `[0x149728]`
  respectively (`str` at 0x75394, `str` at 0x753b0). L2 output meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75354 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x75358 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x7535c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x75360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7536c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75370 | load | 4 | `abs 0x753b8` | `-` |
| `ldr` at 0x75374 | load | 4 | `abs 0x149720` | `-` |
| `str` at 0x75378 | store | 4 | `r0 +0x0` | `[0x149720]` |
| `ldr` at 0x7537c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75388 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7538c | load | 4 | `abs 0x753b8` | `-` |
| `ldr` at 0x75390 | load | 4 | `abs 0x149724` | `-` |
| `str` at 0x75394 | store | 4 | `(unknown)` | `[0x149724]` |
| `ldr` at 0x75398 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x753a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x753a8 | load | 4 | `abs 0x753b8` | `-` |
| `ldr` at 0x753ac | load | 4 | `abs 0x149728` | `-` |
| `str` at 0x753b0 | store | 4 | `(unknown)` | `[0x149728]` |
| `ldmdb` at 0x753b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x753b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x753b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75368 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x75384 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x753a0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x753b4 | `r0` | `r0` |

## `L1CfgGetPort`

- `L1CfgGetPort`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x753bc to 0x75404.
  Established. An entry `r0` save writes four bytes at `sp -0x14` (`str` at 0x753cc). A later
  four-byte absolute read targets `0x149734` (`ldr` at 0x753f0). The printed `r0` return remains
  unknown (`ldmdb` at 0x753fc). Port-record stride and result interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x753cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x753d0 | load | 4 | `abs 0x75400` | `-` |
| `ldr` at 0x753d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x753f0 | load | 4 | `abs 0x149734` | `-` |
| `ldmdb` at 0x753fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x753fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x753fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x753fc | `r0` | `?` |
