# ARM hub initialization and link callers in 1.40.5

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
These routines have no unresolved computed jumps. Indirect call targets and numeric
callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/hub-init-link-facts.txt) retain all tool sections.

## `SHUB2_Cfg_Init`

- `SHUB2_Cfg_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3a7a8 to 0x3aa9c.
  Established. The four-byte assignment writes `0x1` at absolute `0x148cf4` (`str` at 0x3a7dc). The
  unnamed `0x1b454` call carries `[sp-44]`, `0x1`, `0x148cf4`, unknown, and its result is stored as
  four bytes at absolute `0x148cf0` (`bl` at 0x3a7fc, `str` at 0x3a804). Included later byte stores
  at unknown effective addresses retain `([sp-24]:1 & 0xff)`, `([sp-36]:1 & 0xff)` and `0x1` (`strb`
  at 0x3a964, `strb` at 0x3a98c, `strb` at 0x3a9b4). Hub-configuration record meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3a7b8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x3a7bc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x3a7c0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x3a7c4 | load | 4 | `abs 0x3aa7c` | `-` |
| `ldr` at 0x3a7c8 | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3a7d4 | load | 4 | `abs 0x3aa80` | `-` |
| `str` at 0x3a7dc | store | 4 | `abs 0x148cf4` | `0x1` |
| `ldr` at 0x3a7e0 | load | 4 | `abs 0x3aa80` | `-` |
| `ldr` at 0x3a7e4 | load | 4 | `abs 0x148cf4` | `-` |
| `str` at 0x3a7ec | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x3a7f0 | load | 4 | `abs 0x3aa7c` | `-` |
| `ldr` at 0x3a7f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3a804 | store | 4 | `abs 0x148cf0` | `ret(0x1b454)` |
| `ldr` at 0x3a808 | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3a814 | load | 4 | `abs 0x3aa84` | `-` |
| `str` at 0x3a818 | store | 4 | `sp -0x44` | `0x108508` |
| `ldr` at 0x3a81c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x3a820 | store | 4 | `sp -0x40` | `[sp-44]` |
| `ldr` at 0x3a828 | load | 4 | `abs 0x3aa88` | `-` |
| `ldr` at 0x3a838 | load | 4 | `abs 0x3aa7c` | `-` |
| `ldr` at 0x3a83c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a844 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3a84c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x3a850 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a854 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3a858 | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x3a85c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a860 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3a864 | store | 4 | `[sp-52] +0x8` | `[sp-32]` |
| `ldr` at 0x3a868 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a86c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a870 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3a880 | store | 4 | `[sp-52] +0x4` | `ret(0x1b454)` |
| `ldr` at 0x3a884 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x3a890 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a898 | load | 4 | `abs 0x3aa8c` | `-` |
| `ldr` at 0x3a89c | load | 4 | `abs 0x3aa90` | `-` |
| `str` at 0x3a8c8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x3a8cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3a8d4 | load | 4 | `abs 0x3aa8c` | `-` |
| `ldr` at 0x3a8d8 | load | 4 | `abs 0x3aa90` | `-` |
| `ldr` at 0x3a8e4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x3a8f4 | load | 4 | `abs 0x3aa94` | `-` |
| `str` at 0x3a8f8 | store | 4 | `(unknown)` | `0x108564` |
| `str` at 0x3a8fc | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0x3a900 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a904 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x3a908 | store | 4 | `(unknown)` | `[[sp-52]+8]` |
| `ldr` at 0x3a910 | load | 4 | `abs 0x3aa88` | `-` |
| `str` at 0x3a924 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3a928 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a92c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a930 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x3a940 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a944 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a958 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3a960 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x3a964 | store | 1 | `(unknown)` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x3a968 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a96c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a980 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x3a988 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0x3a98c | store | 1 | `(unknown)` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0x3a990 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3a994 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a9a8 | load | 4 | `[sp-52] +0x4` | `-` |
| `strb` at 0x3a9b4 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3a9b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a9cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a9d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3a9e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3a9f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3a9f4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3aa00 | store | 4 | `sp -0x30` | `ret(0xe9c50)` |
| `ldr` at 0x3aa04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3aa10 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3aa1c | load | 4 | `abs 0x3aa98` | `-` |
| `str` at 0x3aa20 | store | 4 | `(unknown)` | `0x1085a0` |
| `ldr` at 0x3aa24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3aa38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3aa40 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3aa44 | store | 4 | `(unknown)` | `?` |
| `str` at 0x3aa48 | store | 4 | `(unknown)` | `ret(0x21704)` |
| `ldr` at 0x3aa50 | load | 4 | `abs 0x3aa88` | `-` |
| `ldr` at 0x3aa60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3aa68 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x3aa78 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x3aa78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3aa78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3aa78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3a7fc | `(unknown callee) 0x1b454` | `[sp-44]` | `0x1` | `0x148cf4` | `?` |
| `bl` at 0x3a834 | `(unknown callee) 0x21eb8` | `0x9` | `0x108540` | `0x6e` | `0x0` |
| `bl` at 0x3a878 | `(unknown callee) 0x1b454` | `[[sp-52]+8]` | `0xc` | `[sp-52]` | `[sp-52]` |
| `bl` at 0x3a8a0 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x108558` | `0x108560` |
| `bl` at 0x3a8ac | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3a8dc | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x108558` | `0x108560` |
| `bl` at 0x3a8ec | `(unknown callee) 0x1b598` | `[sp-56]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x3a91c | `(unknown callee) 0x21eb8` | `0x9` | `0x108540` | `0x7a` | `0x0` |
| `bl` at 0x3a9f8 | `(unknown callee) 0xe9c50` | `?` | `?` | `?` | `?` |
| `bl` at 0x3aa14 | `(unknown callee) 0x21704` | `ret(0xe9c50)` | `?` | `?` | `ret(0xe9c50)` |
| `bl` at 0x3aa5c | `(unknown callee) 0x21eb8` | `0x9` | `0x108540` | `0x84` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x3a7d0 | `[0x148cf0] != 0x0` | `[0x148cf0] == 0x0` |
| `bne` at 0x3a810 | `[0x148cf0] != 0x0` | `[0x148cf0] == 0x0` |
| `bne` at 0x3a88c | `[[sp-52]+4] != 0x0` | `[[sp-52]+4] == 0x0` |
| `blt` at 0x3a938 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3aa0c | `ret(0xe9c50) == 0x0` | `ret(0xe9c50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3aa78 | `r0` | `0x0` |

## `SHUB2_Init`

- `SHUB2_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3aa9c to 0x3aeb8.
  Established. A zero `[0x148cf4]` takes the branch to `0x3aae0`, where a four-byte stack assignment
  retains `0xffffeffc` at `sp -0xd0` (`beq` at 0x3aabc, `str` at 0x3aae4). Included byte assignments
  at `[sp-84] +0x4` retain `0x0` and `0x1` in distinct paths (`strb` at 0x3ac60, `strb` at 0x3ac70).
  Three unnamed `0xec86c` calls carry `[[sp-80]+4]` with second candidates `0x14`, `0x15` and
  `0x17`, respectively, and their full candidates appear below (`bl` at 0x3ad44, `bl` at 0x3adb8,
  `bl` at 0x3ae74). Initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3aaac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x3aab0 | load | 4 | `abs 0x3ae98` | `-` |
| `ldr` at 0x3aab4 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3aac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aac8 | load | 4 | `abs 0x3ae9c` | `-` |
| `ldr` at 0x3aacc | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3aad4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3aae0 | load | 4 | `abs 0x3aea0` | `-` |
| `str` at 0x3aae4 | store | 4 | `sp -0xd0` | `0xffffeffc` |
| `ldr` at 0x3aaec | load | 4 | `abs 0x3ae9c` | `-` |
| `ldr` at 0x3aaf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aaf8 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3ab00 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x3ab08 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3ab0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ab10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ab14 | load | 4 | `[sp-44] +0x8` | `-` |
| `str` at 0x3ab28 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x3ab2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ab30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ab44 | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x3ab4c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x3ab50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3ab54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ab68 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0x3ab70 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x3ab74 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3ab78 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3ab8c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3ab90 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x3ab94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3ab98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3aba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3abbc | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0x3abc0 | load | 1 | `[sp-80] +0x10` | `-` |
| `ldr` at 0x3abe0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x3abec | load | 4 | `abs 0x3aea4` | `-` |
| `str` at 0x3abf0 | store | 4 | `sp -0xe4` | `0x1085bc` |
| `ldr` at 0x3abf4 | load | 4 | `abs 0x3aea8` | `-` |
| `ldr` at 0x3abf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3abfc | load | 4 | `abs 0x2af30c` | `-` |
| `str` at 0x3ac08 | store | 4 | `sp -0xe0` | `?` |
| `ldr` at 0x3ac10 | load | 4 | `abs 0x3aeac` | `-` |
| `ldr` at 0x3ac30 | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x3ac34 | load | 1 | `[sp-168] +0x1` | `-` |
| `str` at 0x3ac38 | store | 4 | `sp -0xd4` | `[[sp-168]+1]:1` |
| `ldr` at 0x3ac3c | load | 4 | `sp -0xd4` | `-` |
| `ldr` at 0x3ac48 | load | 4 | `sp -0xd4` | `-` |
| `ldr` at 0x3ac58 | load | 4 | `sp -0x54` | `-` |
| `strb` at 0x3ac60 | store | 1 | `[sp-84] +0x4` | `0x0` |
| `ldr` at 0x3ac68 | load | 4 | `sp -0x54` | `-` |
| `strb` at 0x3ac70 | store | 1 | `[sp-84] +0x4` | `0x1` |
| `ldr` at 0x3ac78 | load | 4 | `abs 0x3aeb0` | `-` |
| `str` at 0x3ac7c | store | 4 | `sp -0xe4` | `0x108604` |
| `ldr` at 0x3ac80 | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x3ac84 | load | 1 | `[sp-168] +0x2` | `-` |
| `str` at 0x3ac88 | store | 4 | `sp -0xe0` | `[[sp-168]+2]:1` |
| `ldr` at 0x3ac90 | load | 4 | `abs 0x3aeac` | `-` |
| `ldr` at 0x3aca0 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3aca4 | load | 1 | `[sp-84] +0x4` | `-` |
| `ldr` at 0x3acb0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3acbc | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x3acc0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3acd0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x3acd4 | store | 4 | `sp -0x3c` | `[sp-164]` |
| `ldr` at 0x3acd8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x3ace0 | load | 4 | `abs 0x3aeb4` | `-` |
| `str` at 0x3acf8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x3ad14 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3ad18 | load | 1 | `[sp-84] +0x4` | `-` |
| `str` at 0x3ad28 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3ad2c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3ad34 | load | 4 | `[sp-80] +0x4` | `-` |
| `str` at 0x3ad58 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0x3ad68 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x3ad6c | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0x3ad70 | load | 1 | `[sp-84] +0x4` | `-` |
| `ldr` at 0x3ad7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x3ad84 | store | 4 | `sp -0xdc` | `?` |
| `str` at 0x3ad90 | store | 4 | `sp -0xdc` | `0x8000` |
| `ldr` at 0x3ad94 | load | 4 | `sp -0xdc` | `-` |
| `ldr` at 0x3ad98 | load | 4 | `sp -0xd8` | `-` |
| `str` at 0x3ad9c | store | 4 | `[sp-216] +0x4` | `?` |
| `ldr` at 0x3ada0 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3ada8 | load | 4 | `[sp-80] +0x4` | `-` |
| `ldr` at 0x3adbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3adc4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x3add4 | store | 4 | `sp -0x30` | `ret(0x1f26c)` |
| `str` at 0x3ade0 | store | 4 | `sp -0x34` | `ret(0x1f26c)` |
| `str` at 0x3adec | store | 4 | `sp -0x38` | `ret(0x1f26c)` |
| `ldr` at 0x3adfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3ae08 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x3ae18 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3ae28 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3ae34 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x3ae44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x3ae58 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3ae5c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x3ae64 | load | 4 | `[sp-80] +0x4` | `-` |
| `ldr` at 0x3ae78 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x3ae80 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x3ae8c | store | 4 | `sp -0xd0` | `0x0` |
| `ldr` at 0x3ae90 | load | 4 | `sp -0xd0` | `-` |
| `ldmdb` at 0x3ae94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3ae94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3ae94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3abac | `(unknown callee) 0x5e0c8` | `?` | `?` | `?` | `?` |
| `bl` at 0x3abd0 | `(unknown callee) 0xd247c` | `0x0` | `[[sp-80]+16]:1` | `sp-204` | `[[sp-80]+16]:1` |
| `bl` at 0x3ac1c | `(unknown callee) 0x21eb8` | `0x1` | `0x108540` | `0xae` | `0x0` |
| `bl` at 0x3ac28 | `(unknown callee) 0xd25a8` | `sp-204` | `?` | `?` | `sp-204` |
| `bl` at 0x3ac9c | `(unknown callee) 0x21eb8` | `0x1` | `0x108540` | `0xbe` | `0x0` |
| `bl` at 0x3ad04 | `(unknown callee) 0xd25a8` | `sp-204` | `[sp-60]-800` | `?` | `sp-204` |
| `bl` at 0x3ad44 | `(unknown callee) 0xec86c` | `[[sp-80]+4]` | `0x14` | `0x0` | `sp-76` |
| `bl` at 0x3adb8 | `(unknown callee) 0xec86c` | `[[sp-80]+4]` | `0x15` | `0x1` | `sp-76` |
| `bl` at 0x3add0 | `(unknown callee) 0x1f26c` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x3addc | `(unknown callee) 0x1f26c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x3ade8 | `(unknown callee) 0x1f26c` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x3ae74 | `(unknown callee) 0xec86c` | `[[sp-80]+4]` | `0x17` | `0x0` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3aabc | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3aadc | `? != 0x0` | `? == 0x0` |
| `blt` at 0x3ab1c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x3ab80 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0x3aba0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x3abb8 | `ret(0x5e0c8) == 0x0` | `ret(0x5e0c8) != 0x0` |
| `bne` at 0x3abdc | `ret(0xd247c) != 0x0` | `ret(0xd247c) == 0x0` |
| `beq` at 0x3abe8 | `[sp-172] == 0x0` | `[sp-172] != 0x0` |
| `beq` at 0x3ac44 | `[[sp-168]+1]:1 == 0x1f` | `[[sp-168]+1]:1 != 0x1f` |
| `beq` at 0x3ac50 | `[[sp-168]+1]:1 == 0x20` | `[[sp-168]+1]:1 != 0x20` |
| `bne` at 0x3acac | `[[sp-84]+4]:1 != 0x0` | `[[sp-84]+4]:1 == 0x0` |
| `beq` at 0x3acb8 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `blt` at 0x3acc8 | `[sp-164] < [sp-60] (signed, 32-bit)` | `[sp-164] >= [sp-60] (signed, 32-bit)` |
| `bne` at 0x3ad78 | `[[sp-84]+4]:1 != 0x0` | `[[sp-84]+4]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3ae94 | `r0` | `?` |

## `SHUB2_ITP_Present`

- `SHUB2_ITP_Present`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3aeb8 to 0x3af68.
  Established. The branch takes `[0x148cf4] == 0x0` to `0x3af00` (`beq` at 0x3aedc). A later byte
  store with unresolved effective address assigns `0x1` (`strb` at 0x3af48). Separate four-byte
  assignments at `sp -0x34` retain `0xffffeffc` and `0x0` (`str` at 0x3af04, `str` at 0x3af50).
  Presence-state meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3aec8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3aecc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x3aed0 | load | 4 | `abs 0x3af5c` | `-` |
| `ldr` at 0x3aed4 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3aee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3aee8 | load | 4 | `abs 0x3af60` | `-` |
| `ldr` at 0x3aeec | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3aef4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3af00 | load | 4 | `abs 0x3af64` | `-` |
| `str` at 0x3af04 | store | 4 | `sp -0x34` | `0xffffeffc` |
| `ldr` at 0x3af0c | load | 4 | `abs 0x3af60` | `-` |
| `ldr` at 0x3af10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3af18 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3af20 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x3af24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x3af28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3af3c | load | 4 | `[sp-48] +0x4` | `-` |
| `strb` at 0x3af48 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x3af50 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3af54 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3af58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3af58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3af58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3aedc | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3aefc | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3af58 | `r0` | `?` |

## `SHUB2_SysComEnable`

- `SHUB2_SysComEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b30c to 0x3b4c4.
  Established. Two unnamed `0xd76d4` calls have `0x0` followed by three unknown candidates (`bl` at
  0x3b40c, `bl` at 0x3b49c). A signed-less branch takes `(r1 & 0xff) < [[sp-32]+8]` to `0x3b440`
  (`blt` at 0x3b430). A four-byte assignment in the other arm retains `0xffffeffa` at `sp -0x1c`
  (`str` at 0x3b438). Communication-enable signals and index meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b31c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b324 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x3b32c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x3b330 | load | 4 | `abs 0x3b4b0` | `-` |
| `ldr` at 0x3b334 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3b340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b348 | load | 4 | `abs 0x3b4b4` | `-` |
| `ldr` at 0x3b34c | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3b354 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3b360 | load | 4 | `abs 0x3b4b8` | `-` |
| `str` at 0x3b364 | store | 4 | `sp -0x24` | `0xffffeffc` |
| `ldr` at 0x3b36c | load | 4 | `abs 0x3b4b4` | `-` |
| `ldr` at 0x3b370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b378 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3b380 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x3b384 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x3b394 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x3b398 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b39c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b3a0 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x3b3b0 | load | 4 | `abs 0x3b4bc` | `-` |
| `ldr` at 0x3b3b8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b3bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b3d0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b3d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b3dc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b3e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b3f4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b3fc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3b410 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x3b418 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldrb` at 0x3b420 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b424 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x3b428 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x3b434 | load | 4 | `abs 0x3b4c0` | `-` |
| `str` at 0x3b438 | store | 4 | `sp -0x1c` | `0xffffeffa` |
| `ldr` at 0x3b440 | load | 4 | `abs 0x3b4bc` | `-` |
| `ldr` at 0x3b448 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b44c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b460 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b468 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b46c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x3b470 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x3b484 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldrb` at 0x3b48c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3b4a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b4a4 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x3b4a8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x3b4ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b4ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b4ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b3b4 | `(unknown callee) 0x2079c` | `0x10863c` | `?` | `[[sp-32]+8]` | `?` |
| `bl` at 0x3b40c | `(unknown callee) 0xd76d4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x3b444 | `(unknown callee) 0x2079c` | `0x10863c` | `0x148cf0` | `[[sp-32]+8]` | `(r1 & 0xff)` |
| `bl` at 0x3b49c | `(unknown callee) 0xd76d4` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3b33c | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3b35c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x3b38c | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `blt` at 0x3b3a8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x3b430 | `(r1 & 0xff) < [[sp-32]+8] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-32]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b4ac | `r0` | `?` |

## `SHUB2_SysComInt`

- `SHUB2_SysComInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b4c4 to 0x3b4f8.
  Established. The explicit four-byte stack store retains entry `r0` at `sp -0x14` (`str` at
  0x3b4d4). Three byte stores retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at `sp -0x15`,
  `sp -0x16`, `sp -0x17` (`strb` at 0x3b4e0, `strb` at 0x3b4e8, `strb` at 0x3b4f0). The exit carries
  entry `r3` (`ldmdb` at 0x3b4f4). Interrupt handling beyond those accesses is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b4d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x3b4e0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x3b4e8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x3b4f0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldmdb` at 0x3b4f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b4f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b4f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b4f4 | `r0` | `r3` |

## `SHUB2_SetLinkEnable`

- `SHUB2_SetLinkEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3b4f8 to 0x3b5e0.
  Established. A branch takes `[[0x148cf0]+12] != 0x0` to `0x3b5ac` (`bne` at 0x3b59c). That arm
  calls unnamed `0x3c3a8` with `r0`, `r1`, `r2`, `[[0x148cf0]+12]` (`bl` at 0x3b5b8). Earlier
  included four-byte stack assignments retain `0x0` and `0xffffeffc` at `sp -0x2c` in different
  paths (`str` at 0x3b52c, `str` at 0x3b570). Link-enable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3b508 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3b50c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3b510 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3b514 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3b52c | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x3b538 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x3b53c | load | 4 | `abs 0x3b5d4` | `-` |
| `ldr` at 0x3b540 | load | 4 | `abs 0x148cf4` | `-` |
| `ldr` at 0x3b54c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b554 | load | 4 | `abs 0x3b5d8` | `-` |
| `ldr` at 0x3b558 | load | 4 | `abs 0x148cf0` | `-` |
| `ldr` at 0x3b560 | load | 4 | `[0x148cf0] +0x8` | `-` |
| `ldr` at 0x3b56c | load | 4 | `abs 0x3b5dc` | `-` |
| `str` at 0x3b570 | store | 4 | `sp -0x2c` | `0xffffeffc` |
| `ldr` at 0x3b578 | load | 4 | `abs 0x3b5d8` | `-` |
| `ldr` at 0x3b57c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3b584 | load | 4 | `abs 0x148cf0` | `-` |
| `str` at 0x3b58c | store | 4 | `sp -0x28` | `[0x148cf0]` |
| `ldr` at 0x3b590 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x3b594 | load | 4 | `[0x148cf0] +0xc` | `-` |
| `str` at 0x3b5a4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x3b5ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3b5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3b5b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3b5c0 | store | 4 | `sp -0x20` | `ret(0x3c3a8)` |
| `ldr` at 0x3b5c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x3b5c8 | store | 4 | `sp -0x2c` | `ret(0x3c3a8)` |
| `ldr` at 0x3b5cc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x3b5d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3b5d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3b5d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3b5b8 | `(unknown callee) 0x3c3a8` | `r0` | `r1` | `r2` | `[[0x148cf0]+12]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3b524 | `? == 0x7` | `? != 0x7` |
| `beq` at 0x3b548 | `[0x148cf4] == 0x0` | `[0x148cf4] != 0x0` |
| `bne` at 0x3b568 | `[[0x148cf0]+8] != 0x0` | `[[0x148cf0]+8] == 0x0` |
| `bne` at 0x3b59c | `[[0x148cf0]+12] != 0x0` | `[[0x148cf0]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3b5d0 | `r0` | `?` |

## `shub2_set_link_enable`

- `shub2_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3c3a8 to 0x3c5cc.
  Established. The unnamed `0xed170` calls include `[sp-20]`, `0x3`, `[sp-24]+9`, `0x0`, and
  `[sp-20]`, `0x3`, `[sp-24]+11`, `0x0` (`bl` at 0x3c44c, `bl` at 0x3c518, `bl` at 0x3c584).
  Signed-nonnegative `[sp-36]` takes each result test to `0x3c474`, `0x3c540` or `0x3c5ac`,
  respectively (`bge` at 0x3c464, `bge` at 0x3c530, `bge` at 0x3c59c). The unnamed `0xec86c` call at
  the first site has unknown, unknown, `0x19`, `sp-32` (`bl` at 0x3c40c). Link-state and
  scan-operation meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3c3b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x3c3bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x3c3c0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x3c3c4 | load | 4 | `abs 0x3c5bc` | `-` |
| `ldr` at 0x3c3c8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x3c3d4 | load | 4 | `abs 0x3c5c0` | `-` |
| `ldr` at 0x3c3d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c3dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c3e8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c3ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x3c3f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c3fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c410 | load | 4 | `abs 0x3c5bc` | `-` |
| `ldr` at 0x3c414 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x3c420 | load | 4 | `abs 0x3c5c4` | `-` |
| `ldr` at 0x3c424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c428 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c43c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c454 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x3c45c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c468 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c46c | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x3c474 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3c484 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x3c490 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x3c494 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c4a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c4b8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c4bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c4c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c4dc | load | 4 | `abs 0x3c5bc` | `-` |
| `ldr` at 0x3c4e0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x3c4ec | load | 4 | `abs 0x3c5c8` | `-` |
| `ldr` at 0x3c4f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c4f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x3c500 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c508 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c520 | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x3c528 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c534 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c538 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x3c544 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x3c548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3c56c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3c574 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3c58c | store | 4 | `sp -0x24` | `ret(0xed170)` |
| `ldr` at 0x3c594 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3c5a0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x3c5a4 | store | 4 | `sp -0x34` | `[sp-36]` |
| `str` at 0x3c5b0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x3c5b4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x3c5b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3c5b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3c5b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3c3e4 | `(unknown callee) 0x23fbc` | `0x108ac0` | `r0` | `r1` | `r2` |
| `bl` at 0x3c40c | `(unknown callee) 0xec86c` | `?` | `?` | `0x19` | `sp-32` |
| `bl` at 0x3c42c | `(unknown callee) 0x23fbc` | `0x108af8` | `[sp-24]` | `[sp-28]` | `[0x2adfd8]` |
| `bl` at 0x3c430 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x3c44c | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+9` | `0x0` |
| `bl` at 0x3c458 | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |
| `bl` at 0x3c4b0 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x3c4d8 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+11` | `0x2` | `sp-32` |
| `bl` at 0x3c4f8 | `(unknown callee) 0x23fbc` | `0x108b30` | `[sp-24]` | `[sp-28]` | `[0x2adfd8]` |
| `bl` at 0x3c4fc | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x3c518 | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x3c524 | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |
| `bl` at 0x3c564 | `(unknown callee) 0xec86c` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x3c568 | `(unknown callee) 0xe3cf8` | `ret(0xec86c)` | `?` | `?` | `?` |
| `bl` at 0x3c584 | `(unknown callee) 0xed170` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x3c590 | `(unknown callee) 0xe3d64` | `ret(0xed170)` | `?` | `?` | `ret(0xed170)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x3c3d0 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bls` at 0x3c41c | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3c464 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x3c47c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x3c4e8 | `[0x2adfd8] <= 0x1 (unsigned, 32-bit)` | `[0x2adfd8] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x3c530 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bge` at 0x3c59c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3c5b8 | `r0` | `?` |

## `SHUB_Init`

- `SHUB_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47964 to 0x47e18.
  Established. Absolute assignments include byte `([sp-24]:1 & 0xff)` at `0x148d88`, four-byte
  `[sp-20]` at `0x148d8c`, and byte `([sp-28]:1 & 0xff)` at `0x148d90` (`strb` at 0x47994, `str` at
  0x479a0, `strb` at 0x479ac). Later halfword stores at unresolved addresses retain `0x0`, `0x5` and
  `0x0` at separate instructions (`strh` at 0x47c3c, `strh` at 0x47cec, `strh` at 0x47d18). The
  absolute halfword assignment at `0x2b1560` uses `0x320` (`strh` at 0x47d34). Initialization
  records and units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47974 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47978 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4797c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x47980 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47984 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0x47988 | store | 1 | `sp -0x21` | `([sp+16] & 0xff)` |
| `ldr` at 0x4798c | load | 4 | `abs 0x47df0` | `-` |
| `ldrb` at 0x47990 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x47994 | store | 1 | `abs 0x148d88` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x47998 | load | 4 | `abs 0x47df4` | `-` |
| `ldr` at 0x4799c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x479a0 | store | 4 | `abs 0x148d8c` | `[sp-20]` |
| `ldr` at 0x479a4 | load | 4 | `abs 0x47df8` | `-` |
| `ldrb` at 0x479a8 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x479ac | store | 1 | `abs 0x148d90` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x479b0 | load | 4 | `abs 0x47dfc` | `-` |
| `ldr` at 0x479b4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x479b8 | store | 4 | `abs 0x148d9c` | `[sp+4]` |
| `ldr` at 0x479bc | load | 4 | `abs 0x47e00` | `-` |
| `ldr` at 0x479c0 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x479c4 | store | 4 | `abs 0x148da0` | `[sp+8]` |
| `ldr` at 0x479c8 | load | 4 | `abs 0x47e04` | `-` |
| `ldr` at 0x479cc | load | 4 | `sp +0xc` | `-` |
| `str` at 0x479d0 | store | 4 | `abs 0x148da4` | `[sp+12]` |
| `ldr` at 0x479d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x479d8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x479e4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x479e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x479f8 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x479fc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47a18 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47a1c | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47a20 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47a3c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47a40 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47a44 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47a60 | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x47a64 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47a68 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x47a80 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x47a84 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x47a88 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47a8c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47ab0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47ab4 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47ab8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47adc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47ae0 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47ae4 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47b08 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47b0c | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47b10 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47b34 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47b38 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47b3c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47b60 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47b64 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47b68 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47b8c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x47b90 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47b94 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47bb8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47bbc | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47bc0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47be4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47be8 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47bec | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47c10 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47c14 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47c18 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47c3c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x47c40 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47c44 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47c68 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47c6c | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47c70 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47c94 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x47c98 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47c9c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x47cc0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x47cc4 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47cc8 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47cec | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x47cf0 | load | 4 | `abs 0x47e08` | `-` |
| `ldr` at 0x47cf4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x47d18 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x47d1c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x47d24 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x47d2c | load | 4 | `abs 0x47e0c` | `-` |
| `strh` at 0x47d34 | store | 2 | `abs 0x2b1560` | `0x320` |
| `str` at 0x47d40 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x47d44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47d48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47d58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47d68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47d70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47d7c | load | 4 | `(unknown)` | `-` |
| `str` at 0x47d88 | store | 4 | `sp -0x2c` | `ret(0xe9c50)` |
| `ldr` at 0x47d8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x47d98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x47da4 | load | 4 | `abs 0x47e10` | `-` |
| `str` at 0x47da8 | store | 4 | `sp -0x38` | `0x109f70` |
| `ldr` at 0x47dac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x47db4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47dbc | load | 4 | `(unknown)` | `-` |
| `str` at 0x47dc0 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x47dc4 | store | 4 | `sp -0x30` | `ret(0x21704)` |
| `ldr` at 0x47dcc | load | 4 | `abs 0x47e14` | `-` |
| `ldr` at 0x47ddc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x47de4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x47dec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47dec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47dec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x479dc | `(unknown callee) 0x47e18` | `[sp-32]` | `[sp]` | `0x148da4` | `[sp+12]` |
| `bl` at 0x47d38 | `(unknown callee) 0x4e4cc` | `?` | `?` | `0x2b1560` | `0x320` |
| `bl` at 0x47d80 | `(unknown callee) 0xe9c50` | `?` | `?` | `?` | `?` |
| `bl` at 0x47d9c | `(unknown callee) 0x21704` | `ret(0xe9c50)` | `?` | `?` | `ret(0xe9c50)` |
| `bl` at 0x47dd8 | `(unknown callee) 0x21eb8` | `0x9` | `0x109f8c` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x479f0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x47d50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x47d94 | `ret(0xe9c50) == 0x0` | `ret(0xe9c50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47dec | `r0` | `?` |

## `SHUB_ScanInit_LLP_TEST_CTL`

- `SHUB_ScanInit_LLP_TEST_CTL`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49510 to 0x496c0.
  Established. Included four-byte stack assignments at `sp -0x18` retain `0xa00300` and `0xa00000`
  on different paths (`str` at 0x495a4, `str` at 0x495b0). Two unnamed `0x47e9c` calls have
  `[sp-20]`, `0x50000028`, `[sp-24]`, `0xa5fffff`, and `[sp-20]`, `0x50002028`, `[sp-24]`,
  `0xa5fffff` (`bl` at 0x495c4, `bl` at 0x49644). Their nonzero-result branches reach `0x495ec` and
  `0x4966c`, respectively (`bne` at 0x495d0, `bne` at 0x49650). Link-test register fields remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x49520 | load | 4 | `abs 0x4968c` | `-` |
| `ldr` at 0x49524 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x49530 | load | 4 | `abs 0x49690` | `-` |
| `str` at 0x49534 | store | 4 | `sp -0x20` | `0x10a240` |
| `ldr` at 0x4953c | load | 4 | `abs 0x49694` | `-` |
| `str` at 0x49550 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x49554 | load | 4 | `abs 0x49698` | `-` |
| `ldrb` at 0x49558 | load | 1 | `abs 0x148d88` | `-` |
| `ldr` at 0x4955c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4956c | load | 4 | `abs 0x4969c` | `-` |
| `ldr` at 0x49570 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x49590 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x495a0 | load | 4 | `abs 0x496a0` | `-` |
| `str` at 0x495a4 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x495b0 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x495b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x495b8 | load | 4 | `abs 0x496a4` | `-` |
| `ldr` at 0x495bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x495c0 | load | 4 | `abs 0x496a8` | `-` |
| `ldr` at 0x495d4 | load | 4 | `abs 0x496ac` | `-` |
| `ldr` at 0x495d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x495e0 | load | 4 | `abs 0x496b0` | `-` |
| `str` at 0x495e4 | store | 4 | `sp -0x1c` | `0xfffff88a` |
| `ldr` at 0x495ec | load | 4 | `abs 0x4969c` | `-` |
| `ldr` at 0x495f0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x49610 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x49620 | load | 4 | `abs 0x496a0` | `-` |
| `str` at 0x49624 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x49630 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x49634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49638 | load | 4 | `abs 0x496b4` | `-` |
| `ldr` at 0x4963c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49640 | load | 4 | `abs 0x496a8` | `-` |
| `ldr` at 0x49654 | load | 4 | `abs 0x496b8` | `-` |
| `ldr` at 0x49658 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49660 | load | 4 | `abs 0x496bc` | `-` |
| `str` at 0x49664 | store | 4 | `sp -0x1c` | `0xfffff889` |
| `ldr` at 0x4966c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49674 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x49680 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x49684 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x49688 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49688 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49688 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49548 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x2d8` | `0x0` |
| `bl` at 0x495c4 | `(unknown callee) 0x47e9c` | `[sp-20]` | `0x50000028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x495dc | `(unknown callee) 0x2079c` | `0x10a25c` | `[sp-20]` | `?` | `ret(0x47e9c)` |
| `bl` at 0x49644 | `(unknown callee) 0x47e9c` | `[sp-20]` | `0x50002028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x4965c | `(unknown callee) 0x2079c` | `0x10a27c` | `[sp-20]` | `?` | `ret(0x47e9c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4952c | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `blt` at 0x49564 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x4959c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x495d0 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |
| `bne` at 0x4961c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x49650 | `ret(0x47e9c) != 0x0` | `ret(0x47e9c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49688 | `r0` | `?` |

## `SHUB_ScanInitLLP`

- `SHUB_ScanInitLLP`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x496c0 to 0x49c64.
  Established. An unknown-target call carries `[sp-20]`, unknown, unknown, `[0x148d9c]` (`mov` at
  0x49818); its result equal to `0x1` takes the branch to `0x498a0`, whose halfword store at an
  unresolved address retains `0x0` (`beq` at 0x49834, `strh` at 0x498d4). Two unnamed `0xeb5ec`
  calls use `[sp-116]`, `0xa`, `sp-32`, `0x0`, and `[sp-116]`, `0xb`, `sp-32`, `0x0` (`bl` at
  0x49a6c, `bl` at 0x49bc0). Link initialization and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x496d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x496d8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x496dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x496e4 | load | 4 | `abs 0x49c10` | `-` |
| `ldr` at 0x496e8 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x496f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x496f4 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x496f8 | load | 4 | `abs 0x49c14` | `-` |
| `ldr` at 0x496fc | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x49708 | load | 4 | `abs 0x49c18` | `-` |
| `str` at 0x4970c | store | 4 | `sp -0x90` | `0x10a29c` |
| `ldr` at 0x49710 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49714 | store | 4 | `sp -0x8c` | `r0` |
| `ldr` at 0x4971c | load | 4 | `abs 0x49c1c` | `-` |
| `ldr` at 0x49720 | load | 4 | `abs 0x49c20` | `-` |
| `str` at 0x49758 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4975c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4976c | load | 4 | `abs 0x49c24` | `-` |
| `ldr` at 0x49770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49780 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4978c | load | 4 | `abs 0x49c14` | `-` |
| `ldr` at 0x49790 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4979c | load | 4 | `abs 0x49c28` | `-` |
| `str` at 0x497a0 | store | 4 | `sp -0x90` | `0x10a2b0` |
| `ldr` at 0x497a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x497a8 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x497ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x497b0 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x497b4 | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x497b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x497cc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x497e4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x497e8 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x497f0 | load | 4 | `abs 0x49c1c` | `-` |
| `ldr` at 0x497f4 | load | 4 | `abs 0x49c30` | `-` |
| `ldr` at 0x49804 | load | 4 | `abs 0x49c34` | `-` |
| `ldr` at 0x49808 | load | 4 | `abs 0x148d9c` | `-` |
| `ldr` at 0x4980c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49810 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49820 | store | 4 | `sp -0x78` | `ret(unknown)` |
| `ldr` at 0x49824 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x49828 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x4982c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49838 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49844 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49854 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x49864 | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x49868 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4987c | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x49898 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x498a0 | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x498a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x498b8 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x498d4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x498dc | load | 4 | `abs 0x49c14` | `-` |
| `ldr` at 0x498e0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x498ec | load | 4 | `abs 0x49c38` | `-` |
| `str` at 0x498f0 | store | 4 | `sp -0x90` | `0x10a2ec` |
| `ldr` at 0x498f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x498f8 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x49900 | load | 4 | `abs 0x49c1c` | `-` |
| `ldr` at 0x49910 | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x49914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x49928 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x49944 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x49948 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x49950 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x49958 | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x4995c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4997c | load | 2 | `(unknown)` | `-` |
| `str` at 0x49980 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49984 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49988 | load | 4 | `abs 0x49c3c` | `-` |
| `ldr` at 0x49994 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4999c | load | 4 | `abs 0x49c3c` | `-` |
| `str` at 0x499a8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x499ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x499b0 | load | 4 | `abs 0x49c40` | `-` |
| `ldr` at 0x499bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x499c4 | load | 4 | `abs 0x49c40` | `-` |
| `str` at 0x499d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x499d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x499d8 | load | 4 | `abs 0x49c44` | `-` |
| `ldr` at 0x499e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x499ec | load | 4 | `abs 0x49c44` | `-` |
| `str` at 0x499f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x499fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49a0c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49a20 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49a24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49a34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49a48 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x49a54 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x49a58 | load | 4 | `abs 0x49c48` | `-` |
| `str` at 0x49a5c | store | 4 | `sp -0x8c` | `0x10a31c` |
| `ldr` at 0x49a60 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x49a7c | load | 4 | `abs 0x49c4c` | `-` |
| `str` at 0x49a80 | store | 4 | `sp -0x90` | `0x10a328` |
| `ldr` at 0x49a84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49a88 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x49a90 | load | 4 | `abs 0x49c1c` | `-` |
| `ldr` at 0x49a94 | load | 4 | `abs 0x49c50` | `-` |
| `ldr` at 0x49aa0 | load | 4 | `abs 0x49c54` | `-` |
| `str` at 0x49aa4 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `ldr` at 0x49aac | load | 4 | `abs 0x49c2c` | `-` |
| `ldr` at 0x49ab0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x49ad0 | load | 2 | `(unknown)` | `-` |
| `str` at 0x49ad4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49ad8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49adc | load | 4 | `abs 0x49c3c` | `-` |
| `ldr` at 0x49ae8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49af0 | load | 4 | `abs 0x49c3c` | `-` |
| `str` at 0x49afc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b04 | load | 4 | `abs 0x49c40` | `-` |
| `ldr` at 0x49b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b18 | load | 4 | `abs 0x49c40` | `-` |
| `str` at 0x49b24 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49b28 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b2c | load | 4 | `abs 0x49c44` | `-` |
| `ldr` at 0x49b38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b40 | load | 4 | `abs 0x49c44` | `-` |
| `str` at 0x49b4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49b74 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x49b78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49b88 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49b9c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x49ba8 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x49bac | load | 4 | `abs 0x49c58` | `-` |
| `str` at 0x49bb0 | store | 4 | `sp -0x8c` | `0x10a354` |
| `ldr` at 0x49bb4 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x49bd0 | load | 4 | `abs 0x49c5c` | `-` |
| `str` at 0x49bd4 | store | 4 | `sp -0x90` | `0x10a360` |
| `ldr` at 0x49bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x49bdc | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x49be4 | load | 4 | `abs 0x49c1c` | `-` |
| `ldr` at 0x49be8 | load | 4 | `abs 0x49c60` | `-` |
| `ldr` at 0x49bf4 | load | 4 | `abs 0x49c54` | `-` |
| `str` at 0x49bf8 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `str` at 0x49c04 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x49c08 | load | 4 | `sp -0x7c` | `-` |
| `ldmdb` at 0x49c0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49c0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49c0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49728 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x30b` | `0x0` |
| `bl` at 0x4973c | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x49750 | `(unknown callee) 0xdd51c` | `0x0` | `0x0` | `0x20` | `sp-112` |
| `bl` at 0x497fc | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x319` | `0x0` |
| `mov` at 0x49818 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[0x148d9c]` |
| `bl` at 0x4990c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x328` | `0x0` |
| `bl` at 0x49a6c | `(unknown callee) 0xeb5ec` | `[sp-116]` | `0xa` | `sp-32` | `0x0` |
| `bl` at 0x49a9c | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x336` | `0x0` |
| `bl` at 0x49bc0 | `(unknown callee) 0xeb5ec` | `[sp-116]` | `0xb` | `sp-32` | `0x0` |
| `bl` at 0x49bf0 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x33e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x49704 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `ble` at 0x49764 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x49788 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x49798 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x49834 | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |
| `bgt` at 0x49840 | `ret(unknown) > 0x1 (signed, 32-bit)` | `ret(unknown) <= 0x1 (signed, 32-bit)` |
| `beq` at 0x4984c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x4985c | `ret(unknown) == 0x2` | `ret(unknown) != 0x2` |
| `beq` at 0x498e8 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x49a78 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x49bcc | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49c0c | `r0` | `?` |

## `SHUB_ScanInitConfig`

- `SHUB_ScanInitConfig`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b6cc to 0x4c14c.
  Established. Initial four-byte stack assignments retain `0x7ffffd28` at `sp -0xa4` and `0x602800`
  at `sp -0xa0` (`str` at 0x4b748, `str` at 0x4b750). A zero halfword `[0x2b1560]:2` takes the
  branch to `0x4b850`, where a halfword assignment writes `0x320` at absolute `0x2b1560` (`beq` at
  0x4b810, `strh` at 0x4b858). Included unnamed `0xeb5ec` calls use `[sp-168]`, `0x7c`, `0x0`,
  `0x0`, and `[sp-168]`, `0x7e`, `sp-132`, `0x0` (`bl` at 0x4b7c4, `bl` at 0x4bd9c).
  Scan-configuration records remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b6dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4b6e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4b6ec | store | 4 | `sp -0xa4` | `0x0` |
| `str` at 0x4b6f8 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x4b704 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x4b710 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x4b71c | store | 4 | `sp -0x94` | `0x0` |
| `str` at 0x4b728 | store | 4 | `sp -0x90` | `0x0` |
| `str` at 0x4b734 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x4b740 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x4b744 | load | 4 | `abs 0x4c090` | `-` |
| `str` at 0x4b748 | store | 4 | `sp -0xa4` | `0x7ffffd28` |
| `ldr` at 0x4b74c | load | 4 | `abs 0x4c094` | `-` |
| `str` at 0x4b750 | store | 4 | `sp -0xa0` | `0x602800` |
| `ldr` at 0x4b754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b75c | load | 4 | `abs 0x4c098` | `-` |
| `ldr` at 0x4b760 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4b768 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4b76c | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x4b770 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4b774 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4b780 | load | 4 | `abs 0x4c0a0` | `-` |
| `str` at 0x4b784 | store | 4 | `sp -0xc4` | `0x10a708` |
| `ldr` at 0x4b788 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b78c | store | 4 | `sp -0xc0` | `r0` |
| `ldr` at 0x4b794 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4b798 | load | 4 | `abs 0x4c0a8` | `-` |
| `str` at 0x4b7a8 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x4b7ac | load | 4 | `abs 0x4c0ac` | `-` |
| `str` at 0x4b7b0 | store | 4 | `sp -0xc0` | `0x10a724` |
| `ldr` at 0x4b7b4 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4b7d4 | load | 4 | `abs 0x4c0b0` | `-` |
| `str` at 0x4b7d8 | store | 4 | `sp -0xc4` | `0x10a730` |
| `ldr` at 0x4b7dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b7e0 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b7e8 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4b7ec | load | 4 | `abs 0x4c0b4` | `-` |
| `ldr` at 0x4b7f8 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4b7fc | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4b804 | load | 4 | `abs 0x4c0bc` | `-` |
| `ldrh` at 0x4b808 | load | 2 | `abs 0x2b1560` | `-` |
| `ldr` at 0x4b814 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4b818 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4b824 | load | 4 | `abs 0x4c0c0` | `-` |
| `str` at 0x4b828 | store | 4 | `sp -0xc4` | `0x10a760` |
| `ldr` at 0x4b82c | load | 4 | `abs 0x4c0bc` | `-` |
| `ldrh` at 0x4b830 | load | 2 | `abs 0x2b1560` | `-` |
| `str` at 0x4b834 | store | 4 | `sp -0xc0` | `[0x2b1560]:2` |
| `ldr` at 0x4b83c | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4b840 | load | 4 | `abs 0x4c0c4` | `-` |
| `ldr` at 0x4b850 | load | 4 | `abs 0x4c0bc` | `-` |
| `strh` at 0x4b858 | store | 2 | `abs 0x2b1560` | `0x320` |
| `ldr` at 0x4b85c | load | 4 | `abs 0x4c0c8` | `-` |
| `ldrb` at 0x4b860 | load | 1 | `abs 0x148d90` | `-` |
| `str` at 0x4b870 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0x4b874 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b87c | load | 4 | `abs 0x4c098` | `-` |
| `ldr` at 0x4b880 | load | 4 | `abs 0x148d8c` | `-` |
| `ldrb` at 0x4b888 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b88c | load | 4 | `abs 0x4c0c8` | `-` |
| `ldrb` at 0x4b890 | load | 1 | `abs 0x148d90` | `-` |
| `ldr` at 0x4b898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b8ac | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4b8b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b8d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b8dc | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4b8e0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b900 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b904 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x4b908 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4b90c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4b918 | load | 4 | `abs 0x4c0d0` | `-` |
| `str` at 0x4b91c | store | 4 | `sp -0xc4` | `0x10a778` |
| `ldr` at 0x4b920 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b924 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b928 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b92c | store | 4 | `sp -0xbc` | `[sp-176]` |
| `ldr` at 0x4b934 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4b938 | load | 4 | `abs 0x4c0d4` | `-` |
| `ldr` at 0x4b948 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x4b94c | load | 1 | `[sp-172] +0x6` | `-` |
| `str` at 0x4b950 | store | 4 | `sp -0xb0` | `[[sp-172]+6]:1` |
| `ldr` at 0x4b954 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4b958 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4b964 | load | 4 | `abs 0x4c0d8` | `-` |
| `str` at 0x4b968 | store | 4 | `sp -0xc4` | `0x10a79c` |
| `ldr` at 0x4b96c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b970 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4b974 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b978 | store | 4 | `sp -0xbc` | `[[sp-172]+6]:1` |
| `ldr` at 0x4b980 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4b984 | load | 4 | `abs 0x4c0dc` | `-` |
| `ldr` at 0x4b990 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x4b994 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x4b998 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b9a4 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b9b0 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x4b9c0 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4b9c4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4b9e4 | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x4b9e8 | store | 1 | `(unknown)` | `([sp-176]:1 & 0xff)` |
| `ldr` at 0x4b9f0 | load | 4 | `abs 0x4c0e0` | `-` |
| `str` at 0x4b9f4 | store | 4 | `sp -0xc4` | `0x10a7b4` |
| `ldr` at 0x4b9f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4b9fc | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4ba04 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4ba08 | load | 4 | `abs 0x4c0e4` | `-` |
| `ldr` at 0x4ba14 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4ba18 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4ba20 | load | 4 | `abs 0x4c0e8` | `-` |
| `ldr` at 0x4ba24 | load | 4 | `abs 0x148da8` | `-` |
| `ldr` at 0x4ba30 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4ba34 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4ba40 | load | 4 | `abs 0x4c0ec` | `-` |
| `str` at 0x4ba44 | store | 4 | `sp -0xc4` | `0x10a808` |
| `ldr` at 0x4ba48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ba4c | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4ba50 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4ba54 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4ba74 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4ba78 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4ba80 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4ba84 | load | 4 | `abs 0x4c0f0` | `-` |
| `ldr` at 0x4ba94 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4ba98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bab8 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x4babc | load | 1 | `[sp-172] +0xc` | `-` |
| `strb` at 0x4bac0 | store | 1 | `(unknown)` | `([[sp-172]+12]:1 & 0xff)` |
| `ldr` at 0x4bac4 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4bac8 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4bad4 | load | 4 | `abs 0x4c0f4` | `-` |
| `str` at 0x4bad8 | store | 4 | `sp -0xc4` | `0x10a82c` |
| `ldr` at 0x4badc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bae0 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bae4 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bae8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4bb08 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4bb0c | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4bb14 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bb18 | load | 4 | `abs 0x4c0f8` | `-` |
| `ldr` at 0x4bb24 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bb28 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bb48 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4bb54 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4bb58 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4bb64 | load | 4 | `abs 0x4c0fc` | `-` |
| `str` at 0x4bb68 | store | 4 | `sp -0xc4` | `0x10a850` |
| `ldr` at 0x4bb6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bb70 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bb74 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bb78 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bb98 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bb9c | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4bba4 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bba8 | load | 4 | `abs 0x4c100` | `-` |
| `ldr` at 0x4bbb8 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bbbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bbdc | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x4bbe0 | load | 2 | `[sp-172] +0x8` | `-` |
| `strh` at 0x4bbe4 | store | 2 | `(unknown)` | `([[sp-172]+8]:2 & 0xffff)` |
| `ldr` at 0x4bbe8 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4bbec | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4bbf8 | load | 4 | `abs 0x4c104` | `-` |
| `str` at 0x4bbfc | store | 4 | `sp -0xc4` | `0x10a87c` |
| `ldr` at 0x4bc00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bc04 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bc08 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bc0c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bc2c | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bc30 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4bc38 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bc3c | load | 4 | `abs 0x4c108` | `-` |
| `ldr` at 0x4bc48 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bc4c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bc6c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x4bc78 | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4bc7c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4bc88 | load | 4 | `abs 0x4c10c` | `-` |
| `str` at 0x4bc8c | store | 4 | `sp -0xc4` | `0x10a89c` |
| `ldr` at 0x4bc90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bc94 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bc98 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bc9c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bcbc | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bcc0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4bcc8 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bcdc | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bce0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4bd00 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x4bd04 | load | 2 | `[sp-172] +0xa` | `-` |
| `strh` at 0x4bd08 | store | 2 | `(unknown)` | `([[sp-172]+10]:2 & 0xffff)` |
| `ldr` at 0x4bd0c | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4bd10 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4bd1c | load | 4 | `abs 0x4c110` | `-` |
| `str` at 0x4bd20 | store | 4 | `sp -0xc4` | `0x10a8c8` |
| `ldr` at 0x4bd24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bd28 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bd2c | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bd30 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bd50 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bd54 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x4bd5c | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bd60 | load | 4 | `abs 0x4c114` | `-` |
| `ldr` at 0x4bd70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bd84 | store | 4 | `sp -0xc4` | `0x38` |
| `ldr` at 0x4bd88 | load | 4 | `abs 0x4c118` | `-` |
| `str` at 0x4bd8c | store | 4 | `sp -0xc0` | `0x10a8e8` |
| `ldr` at 0x4bd90 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bdac | load | 4 | `abs 0x4c11c` | `-` |
| `str` at 0x4bdb0 | store | 4 | `sp -0xc4` | `0x10a8f0` |
| `ldr` at 0x4bdb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bdb8 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bdc0 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bdc4 | load | 4 | `abs 0x4c120` | `-` |
| `ldr` at 0x4bdd0 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4bdd4 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4bddc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4be14 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x4be1c | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4be20 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4be40 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4be9c | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x4bea0 | load | 4 | `abs 0x4c124` | `-` |
| `str` at 0x4bea4 | store | 4 | `sp -0xc0` | `0x10a918` |
| `ldr` at 0x4bea8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bec4 | load | 4 | `abs 0x4c128` | `-` |
| `str` at 0x4bec8 | store | 4 | `sp -0xc4` | `0x10a928` |
| `ldr` at 0x4becc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bed0 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bed8 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bee8 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4beec | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4bef4 | load | 4 | `abs 0x4c0cc` | `-` |
| `ldr` at 0x4bef8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x4bf18 | load | 2 | `(unknown)` | `-` |
| `str` at 0x4bf74 | store | 4 | `sp -0xc4` | `0xf2` |
| `ldr` at 0x4bf78 | load | 4 | `abs 0x4c12c` | `-` |
| `str` at 0x4bf7c | store | 4 | `sp -0xc0` | `0x10a958` |
| `ldr` at 0x4bf80 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bf9c | load | 4 | `abs 0x4c130` | `-` |
| `str` at 0x4bfa0 | store | 4 | `sp -0xc4` | `0x10a968` |
| `ldr` at 0x4bfa4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4bfa8 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4bfb0 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4bfb4 | load | 4 | `abs 0x4c134` | `-` |
| `ldr` at 0x4bfc0 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4bfc4 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `str` at 0x4bfd0 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x4bfd4 | load | 4 | `abs 0x4c138` | `-` |
| `str` at 0x4bfd8 | store | 4 | `sp -0xc0` | `0x10a998` |
| `ldr` at 0x4bfdc | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x4bffc | load | 4 | `abs 0x4c13c` | `-` |
| `str` at 0x4c000 | store | 4 | `sp -0xc4` | `0x10a9a4` |
| `ldr` at 0x4c004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c008 | store | 4 | `sp -0xc0` | `[sp-20]` |
| `ldr` at 0x4c010 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4c014 | load | 4 | `abs 0x4c140` | `-` |
| `ldr` at 0x4c020 | load | 4 | `abs 0x4c0b8` | `-` |
| `str` at 0x4c024 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x4c02c | load | 4 | `abs 0x4c09c` | `-` |
| `ldr` at 0x4c030 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4c03c | load | 4 | `abs 0x4c144` | `-` |
| `str` at 0x4c040 | store | 4 | `sp -0xc4` | `0x10a9d0` |
| `ldr` at 0x4c048 | load | 4 | `abs 0x4c0a4` | `-` |
| `ldr` at 0x4c04c | load | 4 | `abs 0x4c148` | `-` |
| `str` at 0x4c084 | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x4c088 | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x4c08c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c08c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c08c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b7a0 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x4c2` | `0x0` |
| `bl` at 0x4b7c4 | `(unknown callee) 0xeb5ec` | `[sp-168]` | `0x7c` | `0x0` | `0x0` |
| `bl` at 0x4b7f4 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x4ca` | `0x0` |
| `bl` at 0x4b848 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x4d1` | `0x0` |
| `bl` at 0x4b8a8 | `(unknown callee) 0x49c64` | `[sp-20]` | `?` | `[0x148d90]:1` | `sp-172` |
| `bl` at 0x4b940 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x4ee` | `0x0` |
| `bl` at 0x4b98c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x4f1` | `0x0` |
| `bl` at 0x4ba10 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x4fd` | `0x0` |
| `bl` at 0x4ba8c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x507` | `0x0` |
| `bl` at 0x4bb20 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x50a` | `0x0` |
| `bl` at 0x4bbb0 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x513` | `0x0` |
| `bl` at 0x4bc44 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x517` | `0x0` |
| `bl` at 0x4bcd4 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x520` | `0x0` |
| `bl` at 0x4bd68 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x524` | `0x0` |
| `bl` at 0x4bd78 | `(unknown callee) 0x4b038` | `[sp-20]` | `sp-132` | `?` | `sp-132` |
| `bl` at 0x4bd9c | `(unknown callee) 0xeb5ec` | `[sp-168]` | `0x7e` | `sp-132` | `0x0` |
| `bl` at 0x4bdcc | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x531` | `0x0` |
| `bl` at 0x4bdf8 | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x4be0c | `(unknown callee) 0xdd51c` | `0x0` | `0x0` | `0x20` | `sp-124` |
| `bl` at 0x4be5c | `(unknown callee) 0xe259c` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4be6c | `(unknown callee) 0xe259c` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4be80 | `(unknown callee) 0xe2610` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4be90 | `(unknown callee) 0xe2610` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4beb4 | `(unknown callee) 0xeb5ec` | `[sp-168]` | `0x1a` | `sp-164` | `0x0` |
| `bl` at 0x4bee4 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x550` | `0x0` |
| `bl` at 0x4bf34 | `(unknown callee) 0xe259c` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4bf44 | `(unknown callee) 0xe259c` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4bf58 | `(unknown callee) 0xe2610` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x4bf68 | `(unknown callee) 0xe2610` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x4bf8c | `(unknown callee) 0xeb5ec` | `[sp-168]` | `0x1b` | `sp-164` | `0x0` |
| `bl` at 0x4bfbc | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x55d` | `0x0` |
| `bl` at 0x4bfec | `(unknown callee) 0xeb5ec` | `[sp-168]` | `0x7d` | `0x0` | `0x0` |
| `bl` at 0x4c01c | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x567` | `0x0` |
| `bl` at 0x4c054 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x56a` | `0x0` |
| `bl` at 0x4c068 | `(unknown callee) 0xdd33c` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x4c07c | `(unknown callee) 0xdd51c` | `0x0` | `0x0` | `0x20` | `sp-124` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4b77c | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4b7d0 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4b810 | `[0x2b1560]:2 == 0x0` | `[0x2b1560]:2 != 0x0` |
| `beq` at 0x4b820 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4b868 | `[0x148d90]:1 == 0x0` | `[0x148d90]:1 != 0x0` |
| `beq` at 0x4b8d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4b914 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4b960 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `blt` at 0x4b9a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x4b9ac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4b9b8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x4ba2c | `[0x148da8] == 0x0` | `[0x148da8] != 0x0` |
| `beq` at 0x4ba3c | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bad0 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bb50 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bb60 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bbf4 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bc74 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bc84 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bd18 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4bda8 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `bne` at 0x4bde4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x4be4c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bec0 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4bf24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4bf98 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4bff8 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4c038 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c08c | `r0` | `?` |

## `SHUB_Scan_IDCODE`

- `SHUB_Scan_IDCODE`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c14c to 0x4c52c.
  Established. The unknown-target call has unknown, unknown, `[0x14385c]`, unknown (`mov` at
  0x4c1c4). The unnamed `0xeb5ec` call carries unknown, `0x3`, `0x0`, unknown (`bl` at 0x4c254); its
  zero result takes the branch to `0x4c294` (`beq` at 0x4c260). Included later four-byte stores have
  unresolved effective addresses and unknown values (`str` at 0x4c2d0, `str` at 0x4c310, `str` at
  0x4c350, `str` at 0x4c390, `str` at 0x4c3d0). ID-code interpretation and scan operations remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c15c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c164 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c168 | load | 4 | `abs 0x4c4d4` | `-` |
| `ldr` at 0x4c16c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c174 | load | 4 | `abs 0x4c4d8` | `-` |
| `ldr` at 0x4c178 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4c180 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c194 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c19c | load | 4 | `abs 0x4c4d8` | `-` |
| `ldr` at 0x4c1a0 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4c1a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c1b4 | load | 4 | `abs 0x14385c` | `-` |
| `ldr` at 0x4c1e8 | load | 4 | `abs 0x4c4dc` | `-` |
| `str` at 0x4c1ec | store | 4 | `sp -0x78` | `0x10a9fc` |
| `ldr` at 0x4c1f4 | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c1f8 | load | 4 | `abs 0x4c4e4` | `-` |
| `ldr` at 0x4c204 | load | 4 | `abs 0x4c4e8` | `-` |
| `str` at 0x4c208 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x4c210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c218 | load | 4 | `abs 0x4c4d8` | `-` |
| `ldr` at 0x4c21c | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4c224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c22c | load | 4 | `abs 0x4c4ec` | `-` |
| `str` at 0x4c238 | store | 4 | `sp -0x78` | `0x20` |
| `ldr` at 0x4c23c | load | 4 | `abs 0x4c4f0` | `-` |
| `str` at 0x4c240 | store | 4 | `sp -0x74` | `0x10aa14` |
| `ldr` at 0x4c244 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c264 | load | 4 | `abs 0x4c4f4` | `-` |
| `str` at 0x4c268 | store | 4 | `sp -0x78` | `0x10aa24` |
| `ldr` at 0x4c26c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c270 | store | 4 | `sp -0x74` | `[sp-20]` |
| `ldr` at 0x4c278 | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c27c | load | 4 | `abs 0x4c4f8` | `-` |
| `ldr` at 0x4c288 | load | 4 | `abs 0x4c4e8` | `-` |
| `str` at 0x4c28c | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x4c294 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c29c | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c2a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c2a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c2a8 | load | 4 | `abs 0x4c4fc` | `-` |
| `ldr` at 0x4c2b4 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c2b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c2bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c2c4 | load | 4 | `abs 0x4c4fc` | `-` |
| `str` at 0x4c2d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4c2d4 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c2d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c2dc | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c2e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c2e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c2e8 | load | 4 | `abs 0x4c500` | `-` |
| `ldr` at 0x4c2f4 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c2fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c304 | load | 4 | `abs 0x4c500` | `-` |
| `str` at 0x4c310 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4c314 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c31c | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c324 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c328 | load | 4 | `abs 0x4c504` | `-` |
| `ldr` at 0x4c334 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c33c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c344 | load | 4 | `abs 0x4c504` | `-` |
| `str` at 0x4c350 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4c354 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c35c | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c364 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c374 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c378 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c37c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c390 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4c394 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c39c | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c3a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c3a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c3b4 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c3bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c3d0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x4c3d4 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c3d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c3dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c3e4 | load | 4 | `abs 0x4c508` | `-` |
| `ldr` at 0x4c3f0 | load | 4 | `abs 0x4c50c` | `-` |
| `str` at 0x4c3f4 | store | 4 | `sp -0x78` | `0x10aa54` |
| `ldr` at 0x4c3fc | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c400 | load | 4 | `abs 0x4c510` | `-` |
| `ldr` at 0x4c40c | load | 4 | `abs 0x4c514` | `-` |
| `str` at 0x4c410 | store | 4 | `sp -0x78` | `0x10aa90` |
| `ldr` at 0x4c414 | load | 4 | `abs 0x4c4ec` | `-` |
| `ldr` at 0x4c418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c41c | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c424 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x4c42c | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c430 | load | 4 | `abs 0x4c518` | `-` |
| `ldr` at 0x4c43c | load | 4 | `abs 0x4c4e8` | `-` |
| `str` at 0x4c440 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x4c448 | load | 4 | `abs 0x4c51c` | `-` |
| `ldr` at 0x4c44c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4c458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c464 | load | 4 | `abs 0x4c520` | `-` |
| `str` at 0x4c468 | store | 4 | `sp -0x78` | `0x10aac0` |
| `ldr` at 0x4c46c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c470 | store | 4 | `sp -0x74` | `[sp-20]` |
| `str` at 0x4c474 | store | 4 | `sp -0x70` | `ret(0x4c52c)` |
| `ldr` at 0x4c47c | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c480 | load | 4 | `abs 0x4c524` | `-` |
| `ldr` at 0x4c48c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c4a0 | load | 4 | `abs 0x4c528` | `-` |
| `str` at 0x4c4a4 | store | 4 | `sp -0x78` | `0x10aad0` |
| `ldr` at 0x4c4ac | load | 4 | `abs 0x4c4e0` | `-` |
| `ldr` at 0x4c4bc | load | 4 | `abs 0x4c4e8` | `-` |
| `str` at 0x4c4c0 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x4c4c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c4c8 | store | 4 | `sp -0x6c` | `[sp-24]` |
| `ldr` at 0x4c4cc | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0x4c4d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c4d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c4d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x4c1c4 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x14385c]` | `?` |
| `bl` at 0x4c1d8 | `(unknown callee) 0xdd3f8` | `0x0` | `0x0` | `sp-104` | `sp-104` |
| `bl` at 0x4c200 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x585` | `0x0` |
| `bl` at 0x4c254 | `(unknown callee) 0xeb5ec` | `?` | `0x3` | `0x0` | `?` |
| `bl` at 0x4c284 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x58b` | `0x0` |
| `bl` at 0x4c408 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x595` | `0x0` |
| `bl` at 0x4c438 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x597` | `0x0` |
| `bl` at 0x4c45c | `(unknown callee) 0x4c52c` | `[sp-20]` | `?` | `?` | `[0x2adfd8]` |
| `bl` at 0x4c488 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x59b` | `0x0` |
| `bl` at 0x4c490 | `(unknown callee) 0x4c52c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x4c4b8 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x5a0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4c1e4 | `ret(0xdd3f8) != 0x0` | `ret(0xdd3f8) == 0x0` |
| `beq` at 0x4c260 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4c3ec | `? == 0x4363049` | `? != 0x4363049` |
| `beq` at 0x4c454 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bgt` at 0x4c49c | `ret(0x4c52c) > 0x1 (signed, 32-bit)` | `ret(0x4c52c) <= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c4d0 | `r0` | `?` |

## `SHUB_ScanSetBusratio`

- `SHUB_ScanSetBusratio`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c560 to 0x4c920.
  Established. Zero `[0x148db0]` takes the branch to `0x4c598` (`beq` at 0x4c588). The unnamed
  `0xd2790` call carries `0x0`, `sp-32`, `r2`, `sp-32`, and its nonzero result takes the branch to
  `0x4c5e4` (`bl` at 0x4c5a4, `bne` at 0x4c5b8). A later branch takes `[sp-34]:2 <= 0x8fc`, unsigned
  32-bit, to `0x4c840` (`bls` at 0x4c818). The final included byte store has unknown effective
  address and retains `([sp-35]:1 & 0xff)` (`strb` at 0x4c8c8). Bus-ratio units and encoding remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c570 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x4c578 | store | 2 | `sp -0x22` | `0x0` |
| `ldr` at 0x4c57c | load | 4 | `abs 0x4c8dc` | `-` |
| `ldr` at 0x4c580 | load | 4 | `abs 0x148db0` | `-` |
| `str` at 0x4c590 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4c5ac | store | 4 | `sp -0x1c` | `ret(0xd2790)` |
| `ldr` at 0x4c5b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4c5bc | load | 4 | `abs 0x4c8e0` | `-` |
| `str` at 0x4c5c0 | store | 4 | `sp -0x38` | `0x10ab10` |
| `ldr` at 0x4c5c8 | load | 4 | `abs 0x4c8e4` | `-` |
| `ldr` at 0x4c5cc | load | 4 | `abs 0x4c8e8` | `-` |
| `str` at 0x4c5dc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c5e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c5ec | load | 4 | `abs 0x4c8ec` | `-` |
| `ldr` at 0x4c5f0 | load | 4 | `abs 0x148d8c` | `-` |
| `ldrb` at 0x4c5f8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x4c608 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x4c614 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c620 | load | 4 | `abs 0x4c8ec` | `-` |
| `ldr` at 0x4c624 | load | 4 | `abs 0x148d8c` | `-` |
| `ldrb` at 0x4c62c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4c640 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4c648 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4c64c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c65c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4c668 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x4c688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c690 | load | 4 | `abs 0x4c8ec` | `-` |
| `ldr` at 0x4c694 | load | 4 | `abs 0x148d8c` | `-` |
| `ldrb` at 0x4c69c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x4c6a0 | load | 1 | `sp -0x18` | `-` |
| `ldr` at 0x4c6c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c6dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c6e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c6f0 | load | 4 | `abs 0x4c8f0` | `-` |
| `ldr` at 0x4c6f4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4c700 | load | 4 | `abs 0x4c8f4` | `-` |
| `str` at 0x4c704 | store | 4 | `sp -0x38` | `0x10ab2c` |
| `ldr` at 0x4c708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c70c | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c710 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c714 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x4c718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c72c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c734 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4c738 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4c740 | load | 4 | `abs 0x4c8e4` | `-` |
| `ldr` at 0x4c744 | load | 4 | `abs 0x4c8f8` | `-` |
| `ldrh` at 0x4c750 | load | 2 | `sp -0x22` | `-` |
| `ldrh` at 0x4c75c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4c77c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4c78c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c7a0 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0x4c7a8 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4c7ac | store | 2 | `sp -0x22` | `?` |
| `ldr` at 0x4c7b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c7b8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrh` at 0x4c7c0 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c7cc | load | 4 | `abs 0x4c8f0` | `-` |
| `ldr` at 0x4c7d0 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4c7dc | load | 4 | `abs 0x4c8fc` | `-` |
| `str` at 0x4c7e0 | store | 4 | `sp -0x38` | `0x10ab48` |
| `ldr` at 0x4c7e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c7e8 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c7f0 | load | 4 | `abs 0x4c8e4` | `-` |
| `ldr` at 0x4c7f4 | load | 4 | `abs 0x4c900` | `-` |
| `str` at 0x4c804 | store | 4 | `sp -0x28` | `0x0` |
| `ldrh` at 0x4c80c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c810 | load | 4 | `abs 0x4c904` | `-` |
| `ldr` at 0x4c81c | load | 4 | `abs 0x4c908` | `-` |
| `str` at 0x4c820 | store | 4 | `sp -0x38` | `0x10ab7c` |
| `ldr` at 0x4c824 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c828 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x4c830 | load | 4 | `abs 0x4c8e4` | `-` |
| `ldr` at 0x4c834 | load | 4 | `abs 0x4c90c` | `-` |
| `ldrh` at 0x4c840 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x4c848 | load | 4 | `abs 0x4c910` | `-` |
| `strb` at 0x4c860 | store | 1 | `sp -0x23` | `?` |
| `ldr` at 0x4c864 | load | 4 | `abs 0x4c8f0` | `-` |
| `ldr` at 0x4c868 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4c874 | load | 4 | `abs 0x4c914` | `-` |
| `str` at 0x4c878 | store | 4 | `sp -0x38` | `0x10abbc` |
| `ldr` at 0x4c87c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4c880 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldrb` at 0x4c884 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x4c888 | store | 4 | `sp -0x30` | `[sp-35]:1` |
| `ldr` at 0x4c890 | load | 4 | `abs 0x4c8e4` | `-` |
| `ldr` at 0x4c894 | load | 4 | `abs 0x4c918` | `-` |
| `ldr` at 0x4c8a0 | load | 4 | `abs 0x4c91c` | `-` |
| `ldr` at 0x4c8a4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x4c8c4 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0x4c8c8 | store | 1 | `(unknown)` | `([sp-35]:1 & 0xff)` |
| `str` at 0x4c8d0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x4c8d4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x4c8d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c8d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c8d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4c5a4 | `(unknown callee) 0xd2790` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x4c5d4 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x5c7` | `0x0` |
| `bl` at 0x4c678 | `(unknown callee) 0x5e14c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4c6b4 | `(unknown callee) 0xd27e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4c74c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x5d7` | `0x0` |
| `bl` at 0x4c7fc | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x5e1` | `0x0` |
| `bl` at 0x4c83c | `(unknown callee) 0x21eb8` | `0x4` | `0x109f8c` | `0x5e7` | `0x0` |
| `bl` at 0x4c89c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x5ea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4c588 | `[0x148db0] == 0x0` | `[0x148db0] != 0x0` |
| `bne` at 0x4c5b8 | `ret(0xd2790) != 0x0` | `ret(0xd2790) == 0x0` |
| `bne` at 0x4c600 | `? != 0xff` | `? == 0xff` |
| `ble` at 0x4c654 | `[sp-24] <= 0x1 (signed, 32-bit)` | `[sp-24] > 0x1 (signed, 32-bit)` |
| `beq` at 0x4c684 | `ret(0x5e14c) == 0x0` | `ret(0x5e14c) != 0x0` |
| `bne` at 0x4c6c0 | `ret(0xd27e4) != 0x0` | `ret(0xd27e4) == 0x0` |
| `bne` at 0x4c6ec | `? != 0x3` | `? == 0x3` |
| `beq` at 0x4c6fc | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4c758 | `[sp-34]:2 == 0x0` | `[sp-34]:2 != 0x0` |
| `bge` at 0x4c784 | `[sp-34]:2 >= ? (signed, 32-bit)` | `[sp-34]:2 < ? (signed, 32-bit)` |
| `bne` at 0x4c7c8 | `[sp-34]:2 != 0x0` | `[sp-34]:2 == 0x0` |
| `beq` at 0x4c7d8 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `bls` at 0x4c818 | `[sp-34]:2 <= 0x8fc (unsigned, 32-bit)` | `[sp-34]:2 > 0x8fc (unsigned, 32-bit)` |
| `beq` at 0x4c870 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c8d8 | `r0` | `0x0` |

## `SHUB_ScanReadMCILockStatus`

- `SHUB_ScanReadMCILockStatus`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4d2f0 to 0x4d770.
  Established. The first unnamed `0xeb5ec` call uses `[sp-72]`, `0x7`, `sp-48`, `sp-64` (`bl` at
  0x4d398); its zero result takes the branch to `0x4d3d8` (`beq` at 0x4d3a4). Another call to that
  target carries `[sp-72]`, unknown, `sp-48`, `sp-64` (`bl` at 0x4d418). Two equality branches test
  unknown expressions against `0x1ff`, reaching `0x4d57c` and `0x4d67c` (`beq` at 0x4d4e8, `beq` at
  0x4d5e8). Lock-status field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x4d304 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x4d30c | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x4d310 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d318 | load | 4 | `abs 0x4d720` | `-` |
| `ldr` at 0x4d31c | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x4d324 | load | 4 | `(unknown)` | `-` |
| `str` at 0x4d328 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x4d32c | load | 4 | `abs 0x4d724` | `-` |
| `ldr` at 0x4d330 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4d33c | load | 4 | `abs 0x4d728` | `-` |
| `str` at 0x4d340 | store | 4 | `sp -0x5c` | `0x10ac34` |
| `ldr` at 0x4d348 | load | 4 | `abs 0x4d72c` | `-` |
| `ldr` at 0x4d34c | load | 4 | `abs 0x4d730` | `-` |
| `ldrb` at 0x4d360 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d380 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x4d384 | load | 4 | `abs 0x4d734` | `-` |
| `str` at 0x4d388 | store | 4 | `sp -0x58` | `0x10ac54` |
| `ldr` at 0x4d38c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x4d3a8 | load | 4 | `abs 0x4d738` | `-` |
| `str` at 0x4d3ac | store | 4 | `sp -0x5c` | `0x10ac5c` |
| `ldrb` at 0x4d3b0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d3b4 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d3bc | load | 4 | `abs 0x4d72c` | `-` |
| `ldr` at 0x4d3c0 | load | 4 | `abs 0x4d73c` | `-` |
| `ldr` at 0x4d3cc | load | 4 | `abs 0x4d740` | `-` |
| `str` at 0x4d3d0 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x4d3d8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d400 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x4d404 | load | 4 | `abs 0x4d734` | `-` |
| `str` at 0x4d408 | store | 4 | `sp -0x58` | `0x10ac54` |
| `ldr` at 0x4d40c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x4d428 | load | 4 | `abs 0x4d738` | `-` |
| `str` at 0x4d42c | store | 4 | `sp -0x5c` | `0x10ac5c` |
| `ldrb` at 0x4d430 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d434 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d43c | load | 4 | `abs 0x4d72c` | `-` |
| `ldr` at 0x4d440 | load | 4 | `abs 0x4d744` | `-` |
| `ldr` at 0x4d44c | load | 4 | `abs 0x4d740` | `-` |
| `str` at 0x4d450 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x4d458 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d46c | load | 4 | `abs 0x4d748` | `-` |
| `ldrb` at 0x4d470 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d488 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d48c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d490 | load | 4 | `abs 0x4d74c` | `-` |
| `ldrb` at 0x4d494 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d4ac | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d4b0 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d4b4 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d4b8 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d4c0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d4c4 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d4d0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d4d4 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d4e0 | load | 4 | `abs 0x4d750` | `-` |
| `ldrb` at 0x4d4ec | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d4f0 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d4f4 | load | 4 | `abs 0x4d754` | `-` |
| `str` at 0x4d504 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d508 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d518 | load | 4 | `abs 0x4d724` | `-` |
| `ldr` at 0x4d51c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4d528 | load | 4 | `abs 0x4d758` | `-` |
| `str` at 0x4d52c | store | 4 | `sp -0x5c` | `0x10acc4` |
| `ldrb` at 0x4d530 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d534 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d538 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d53c | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x4d540 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x4d548 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x4d550 | load | 4 | `abs 0x4d72c` | `-` |
| `ldr` at 0x4d554 | load | 4 | `abs 0x4d75c` | `-` |
| `ldrh` at 0x4d560 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d568 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d56c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d574 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d57c | load | 4 | `abs 0x4d748` | `-` |
| `ldrb` at 0x4d580 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d598 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d59c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d5a0 | load | 4 | `abs 0x4d74c` | `-` |
| `ldrb` at 0x4d5a4 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d5bc | load | 2 | `(unknown)` | `-` |
| `strh` at 0x4d5c0 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d5c4 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d5c8 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d5d0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d5d4 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d5e0 | load | 4 | `abs 0x4d750` | `-` |
| `ldrb` at 0x4d5ec | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x4d5f0 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x4d5f4 | load | 4 | `abs 0x4d760` | `-` |
| `str` at 0x4d604 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4d608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4d618 | load | 4 | `abs 0x4d724` | `-` |
| `ldr` at 0x4d61c | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4d628 | load | 4 | `abs 0x4d764` | `-` |
| `str` at 0x4d62c | store | 4 | `sp -0x5c` | `0x10ad1c` |
| `ldrb` at 0x4d630 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x4d634 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x4d638 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d63c | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x4d640 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x4d648 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x4d650 | load | 4 | `abs 0x4d72c` | `-` |
| `ldr` at 0x4d654 | load | 4 | `abs 0x4d768` | `-` |
| `ldrh` at 0x4d660 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d668 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d66c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4d674 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x4d67c | load | 4 | `abs 0x4d748` | `-` |
| `ldrb` at 0x4d680 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d6a0 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x4d6a4 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x4d6a8 | load | 4 | `abs 0x4d74c` | `-` |
| `ldrb` at 0x4d6ac | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4d6cc | load | 1 | `(unknown)` | `-` |
| `strh` at 0x4d6d0 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x4d6d4 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x4d6d8 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x4d6e0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d6e4 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x4d6f0 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x4d6f4 | load | 2 | `sp -0x42` | `-` |
| `ldrb` at 0x4d700 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4d704 | load | 4 | `abs 0x4d76c` | `-` |
| `str` at 0x4d714 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x4d718 | load | 4 | `sp -0x4c` | `-` |
| `ldmdb` at 0x4d71c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4d71c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4d71c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4d354 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x6be` | `0x0` |
| `bl` at 0x4d35c | `(unknown callee) 0x4e524` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x4d370 | `(unknown callee) 0x4ca40` | `[sp-17]:1` | `sp-48` | `sp-48` | `[sp-17]:1` |
| `bl` at 0x4d398 | `(unknown callee) 0xeb5ec` | `[sp-72]` | `0x7` | `sp-48` | `sp-64` |
| `bl` at 0x4d3c8 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x6cb` | `0x0` |
| `bl` at 0x4d3e8 | `(unknown callee) 0x4ce48` | `[sp-17]:1` | `0x1` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x4d3f0 | `(unknown callee) 0x4e524` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x4d418 | `(unknown callee) 0xeb5ec` | `[sp-72]` | `?` | `sp-48` | `sp-64` |
| `bl` at 0x4d448 | `(unknown callee) 0x21eb8` | `0x3` | `0x109f8c` | `0x6de` | `0x0` |
| `bl` at 0x4d468 | `(unknown callee) 0x4ce48` | `[sp-17]:1` | `0x2` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x4d4fc | `(unknown callee) 0x2079c` | `0x10ac88` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x4d55c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x6f2` | `0x0` |
| `bl` at 0x4d5fc | `(unknown callee) 0x2079c` | `0x10ace0` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x4d65c | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x6fc` | `0x0` |
| `bl` at 0x4d70c | `(unknown callee) 0x2079c` | `0x10ad38` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4d338 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4d3a4 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4d424 | `ret(0xeb5ec) == 0x0` | `ret(0xeb5ec) != 0x0` |
| `beq` at 0x4d4e8 | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x4d510 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4d524 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4d5e8 | `? == 0x1ff` | `? != 0x1ff` |
| `ble` at 0x4d610 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x4d624 | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4d6fc | `[sp-66]:2 == 0x1` | `[sp-66]:2 != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4d71c | `r0` | `?` |

## `shub_set_link_enable`

- `shub_set_link_enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4de84 to 0x4df64.
  Established. A zero `[0x14928c]` takes the branch to `0x4df08` (`beq` at 0x4ded0). A later unnamed
  `0xeaf08` call has `[sp-20]`, `0x10060300`, `0x0`, `0x1` (`bl` at 0x4df34). Four-byte stack
  assignments at `sp -0x20` retain `0x0` at two instructions, and the exit carries zero (`str` at
  0x4df18, `str` at 0x4df40, `ldmdb` at 0x4df48). Link-enable hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4de94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4de98 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4de9c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x4dea0 | load | 4 | `abs 0x4df4c` | `-` |
| `ldr` at 0x4dea4 | load | 4 | `abs 0x2adfd8` | `-` |
| `ldr` at 0x4deb0 | load | 4 | `abs 0x4df50` | `-` |
| `ldr` at 0x4deb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4deb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4debc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4dec4 | load | 4 | `abs 0x4df54` | `-` |
| `ldr` at 0x4dec8 | load | 4 | `abs 0x14928c` | `-` |
| `ldr` at 0x4ded4 | load | 4 | `abs 0x4df50` | `-` |
| `str` at 0x4ded8 | store | 4 | `sp -0x30` | `0x10af10` |
| `ldr` at 0x4dedc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4dee0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x4dee4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4dee8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x4deec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4def0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x4def8 | load | 4 | `abs 0x4df58` | `-` |
| `ldr` at 0x4defc | load | 4 | `abs 0x4df5c` | `-` |
| `ldr` at 0x4df08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4df18 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4df24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4df28 | load | 4 | `abs 0x4df60` | `-` |
| `str` at 0x4df40 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4df44 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x4df48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4df48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4df48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4dec0 | `(unknown callee) 0x23fbc` | `0x10af10` | `r0` | `r1` | `r2` |
| `bl` at 0x4df04 | `(unknown callee) 0x21eb8` | `0x6` | `0x109f8c` | `0x7ef` | `0x0` |
| `bl` at 0x4df20 | `(unknown callee) 0xe3cf8` | `?` | `?` | `?` | `?` |
| `bl` at 0x4df34 | `(unknown callee) 0xeaf08` | `[sp-20]` | `0x10060300` | `0x0` | `0x1` |
| `bl` at 0x4df38 | `(unknown callee) 0xe3d64` | `ret(0xeaf08)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4deac | `[0x2adfd8] == 0x0` | `[0x2adfd8] != 0x0` |
| `beq` at 0x4ded0 | `[0x14928c] == 0x0` | `[0x14928c] != 0x0` |
| `bne` at 0x4df10 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4df48 | `r0` | `0x0` |
