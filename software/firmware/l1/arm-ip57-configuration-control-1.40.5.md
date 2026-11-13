# ARM IP57 configuration and control paths in 1.40.5

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
No unresolved computed transfers are reported. Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/ip57-configuration-control-facts.txt) retain all tool sections.

## `IP57_Cfg_Init`

- `IP57_Cfg_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f764 to 0x5f970.
  Established. The byte assignment at `0x1491a0` writes `0x1` (`strb` at 0x5f7b8). The word
  assignment at `0x1491ac` writes `0xc` (`str` at 0x5f7dc). The unnamed `0x47964` call carries
  `[0x2b1680]`, `[0x1491a4]`, `[0x1491ac]`, `0x0` (`bl` at 0x5f934). Configuration effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f774 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5f778 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5f77c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5f780 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5f784 | load | 4 | `abs 0x5f948` | `-` |
| `ldrb` at 0x5f788 | load | 1 | `abs 0x1491a0` | `-` |
| `ldr` at 0x5f794 | load | 4 | `abs 0x5f94c` | `-` |
| `str` at 0x5f798 | store | 4 | `sp -0x44` | `0x10d070` |
| `ldr` at 0x5f7a0 | load | 4 | `abs 0x5f950` | `-` |
| `ldr` at 0x5f7b0 | load | 4 | `abs 0x5f948` | `-` |
| `strb` at 0x5f7b8 | store | 1 | `abs 0x1491a0` | `0x1` |
| `ldr` at 0x5f7bc | load | 4 | `abs 0x5f954` | `-` |
| `ldr` at 0x5f7c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5f7c4 | store | 4 | `abs 0x1491a4` | `[sp-28]` |
| `ldr` at 0x5f7c8 | load | 4 | `abs 0x5f958` | `-` |
| `ldr` at 0x5f7cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5f7d0 | store | 4 | `abs 0x1491a8` | `[sp-24]` |
| `ldr` at 0x5f7d4 | load | 4 | `abs 0x5f95c` | `-` |
| `str` at 0x5f7dc | store | 4 | `abs 0x1491ac` | `0xc` |
| `ldr` at 0x5f7e0 | load | 4 | `abs 0x5f960` | `-` |
| `ldr` at 0x5f7e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5f7f8 | store | 4 | `abs 0x2b1680` | `ret(0x1b1b4)` |
| `ldr` at 0x5f7fc | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5f808 | load | 4 | `abs 0x5f964` | `-` |
| `str` at 0x5f80c | store | 4 | `sp -0x44` | `0x10d0a0` |
| `ldr` at 0x5f814 | load | 4 | `abs 0x5f950` | `-` |
| `str` at 0x5f828 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5f82c | load | 4 | `abs 0x5f954` | `-` |
| `ldr` at 0x5f830 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f834 | load | 4 | `abs 0x1491a4` | `-` |
| `ldr` at 0x5f844 | load | 4 | `abs 0x5f958` | `-` |
| `ldr` at 0x5f848 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f85c | load | 4 | `abs 0x1491a8` | `-` |
| `str` at 0x5f864 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x5f868 | load | 4 | `abs 0x5f960` | `-` |
| `ldr` at 0x5f86c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f874 | load | 4 | `abs 0x2b1680` | `-` |
| `str` at 0x5f87c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x5f880 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f884 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5f888 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x5f890 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x5f894 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f898 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x5f89c | load | 1 | `[sp-44] +0x3` | `-` |
| `strb` at 0x5f8a4 | store | 1 | `[sp-48] +0x1` | `([[sp-44]+3]:1+5 & 0xff)` |
| `ldr` at 0x5f8a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f8ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x5f8b0 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x5f8b4 | store | 4 | `[sp-48] +0x4` | `[[sp-44]+12]` |
| `ldr` at 0x5f8b8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x5f8c0 | store | 4 | `[sp-48] +0x8` | `0x200` |
| `ldr` at 0x5f8c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5f8c8 | load | 4 | `abs 0x5f968` | `-` |
| `ldr` at 0x5f8cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x5f8d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x5f8d4 | store | 4 | `[sp-48] +0xc` | `?` |
| `ldr` at 0x5f8d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5f8e0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x5f8e8 | load | 4 | `abs 0x5f96c` | `-` |
| `ldr` at 0x5f8f0 | load | 4 | `abs 0x5f960` | `-` |
| `ldr` at 0x5f8f4 | load | 4 | `abs 0x5f954` | `-` |
| `ldr` at 0x5f8f8 | load | 4 | `abs 0x5f95c` | `-` |
| `str` at 0x5f900 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x5f904 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5f908 | store | 4 | `sp -0x40` | `[sp-32]` |
| `ldr` at 0x5f90c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x5f910 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x5f914 | load | 4 | `abs 0x5f96c` | `-` |
| `str` at 0x5f918 | store | 4 | `sp -0x38` | `0x1491b8` |
| `str` at 0x5f920 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x5f924 | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5f928 | load | 4 | `abs 0x1491a4` | `-` |
| `ldr` at 0x5f92c | load | 4 | `abs 0x1491ac` | `-` |
| `ldr` at 0x5f938 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x5f93c | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0x5f944 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5f944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f7ac | `(unknown callee) 0x21eb8` | `0x9` | `0x10d08c` | `0x91` | `0x0` |
| `bl` at 0x5f7f0 | `(unknown callee) 0x1b1b4` | `?` | `?` | `0x1491ac` | `?` |
| `bl` at 0x5f820 | `(unknown callee) 0x21eb8` | `0x9` | `0x10d08c` | `0x9d` | `0x0` |
| `bl` at 0x5f8ec | `(unknown callee) 0x5fbe4` | `0x1491b8` | `?` | `?` | `[0x1491a4]` |
| `bl` at 0x5f934 | `(unknown callee) 0x47964` | `[0x2b1680]` | `[0x1491a4]` | `[0x1491ac]` | `0x0` |
| `bl` at 0x5f940 | `(unknown callee) 0x5f970` | `[sp]` | `[sp+4]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f790 | `[0x1491a0]:1 == 0x0` | `[0x1491a0]:1 != 0x0` |
| `bne` at 0x5f804 | `[0x2b1680] != 0x0` | `[0x2b1680] == 0x0` |
| `blt` at 0x5f83c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f944 | `r0` | `ret(0x5f970)` |

## `IP67_Cfg_Init`

- `IP67_Cfg_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fa30 to 0x5fbe4.
  Established. The absolute word assignments at `0x1491a4` and `0x1491ac` write `0x1` and `0x8`
  (`str` at 0x5fa90, `str` at 0x5faa8). The unnamed `0x1b1b4` call carries `0x10`, unknown,
  `0x1491ac`, `0x8` (`bl` at 0x5fab4). The word store at `[0x2b1680] +0x8` writes `0xff000000`
  (`str` at 0x5fb44). Configuration field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5fa40 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x5fa44 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x5fa48 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x5fa4c | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x5fa50 | load | 4 | `abs 0x5fbbc` | `-` |
| `ldrb` at 0x5fa54 | load | 1 | `abs 0x1491a0` | `-` |
| `ldr` at 0x5fa60 | load | 4 | `abs 0x5fbc0` | `-` |
| `str` at 0x5fa64 | store | 4 | `sp -0x38` | `0x10d118` |
| `ldr` at 0x5fa6c | load | 4 | `abs 0x5fbc4` | `-` |
| `ldr` at 0x5fa7c | load | 4 | `abs 0x5fbbc` | `-` |
| `strb` at 0x5fa84 | store | 1 | `abs 0x1491a0` | `0x1` |
| `ldr` at 0x5fa88 | load | 4 | `abs 0x5fbc8` | `-` |
| `str` at 0x5fa90 | store | 4 | `abs 0x1491a4` | `0x1` |
| `ldr` at 0x5fa94 | load | 4 | `abs 0x5fbcc` | `-` |
| `ldr` at 0x5fa98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5fa9c | store | 4 | `abs 0x1491a8` | `[sp-24]` |
| `ldr` at 0x5faa0 | load | 4 | `abs 0x5fbd0` | `-` |
| `str` at 0x5faa8 | store | 4 | `abs 0x1491ac` | `0x8` |
| `ldr` at 0x5faac | load | 4 | `abs 0x5fbd4` | `-` |
| `str` at 0x5fabc | store | 4 | `abs 0x2b1680` | `ret(0x1b1b4)` |
| `ldr` at 0x5fac0 | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5facc | load | 4 | `abs 0x5fbd8` | `-` |
| `str` at 0x5fad0 | store | 4 | `sp -0x38` | `0x10d0a0` |
| `ldr` at 0x5fad8 | load | 4 | `abs 0x5fbc4` | `-` |
| `ldr` at 0x5fae8 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5faec | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5faf0 | load | 4 | `abs 0x5fbcc` | `-` |
| `ldr` at 0x5faf4 | load | 4 | `abs 0x1491a8` | `-` |
| `ldrb` at 0x5faf8 | load | 1 | `[0x1491a8] +0x3` | `-` |
| `strb` at 0x5fb00 | store | 1 | `[0x2b1680] +0x0` | `([[0x1491a8]+3]:1+1 & 0xff)` |
| `ldr` at 0x5fb04 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5fb08 | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5fb0c | load | 4 | `abs 0x5fbcc` | `-` |
| `ldr` at 0x5fb10 | load | 4 | `abs 0x1491a8` | `-` |
| `ldrb` at 0x5fb14 | load | 1 | `[0x1491a8] +0x3` | `-` |
| `strb` at 0x5fb1c | store | 1 | `[0x2b1680] +0x1` | `([[0x1491a8]+3]:1+5 & 0xff)` |
| `ldr` at 0x5fb20 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5fb24 | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5fb28 | load | 4 | `abs 0x5fbcc` | `-` |
| `ldr` at 0x5fb2c | load | 4 | `abs 0x1491a8` | `-` |
| `ldr` at 0x5fb30 | load | 4 | `[0x1491a8] +0xc` | `-` |
| `str` at 0x5fb34 | store | 4 | `[0x2b1680] +0x4` | `[[0x1491a8]+12]` |
| `ldr` at 0x5fb38 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5fb3c | load | 4 | `abs 0x2b1680` | `-` |
| `str` at 0x5fb44 | store | 4 | `[0x2b1680] +0x8` | `0xff000000` |
| `ldr` at 0x5fb48 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5fb4c | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5fb50 | load | 4 | `abs 0x5fbdc` | `-` |
| `ldr` at 0x5fb54 | load | 4 | `abs 0x10d060` | `-` |
| `str` at 0x5fb58 | store | 4 | `[0x2b1680] +0xc` | `[0x10d060]` |
| `ldr` at 0x5fb5c | load | 4 | `abs 0x5fbe0` | `-` |
| `ldr` at 0x5fb64 | load | 4 | `abs 0x5fbd4` | `-` |
| `ldr` at 0x5fb68 | load | 4 | `abs 0x5fbc8` | `-` |
| `ldr` at 0x5fb6c | load | 4 | `abs 0x5fbd0` | `-` |
| `str` at 0x5fb74 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x5fb78 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5fb7c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x5fb80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5fb84 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x5fb88 | load | 4 | `abs 0x5fbe0` | `-` |
| `str` at 0x5fb8c | store | 4 | `sp -0x2c` | `0x1491b8` |
| `str` at 0x5fb94 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5fb98 | load | 4 | `abs 0x2b1680` | `-` |
| `ldr` at 0x5fb9c | load | 4 | `abs 0x1491a4` | `-` |
| `ldr` at 0x5fba0 | load | 4 | `abs 0x1491ac` | `-` |
| `ldr` at 0x5fbac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5fbb0 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0x5fbb8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5fbb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5fbb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5fbb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5fa78 | `(unknown callee) 0x21eb8` | `0x9` | `0x10d08c` | `0xf1` | `0x0` |
| `bl` at 0x5fab4 | `(unknown callee) 0x1b1b4` | `0x10` | `?` | `0x1491ac` | `0x8` |
| `bl` at 0x5fae4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10d08c` | `0xfc` | `0x0` |
| `bl` at 0x5fb60 | `(unknown callee) 0x5fbe4` | `0x1491b8` | `?` | `[0x2b1680]` | `[0x10d060]` |
| `bl` at 0x5fba8 | `(unknown callee) 0x47964` | `[0x2b1680]` | `[0x1491a4]` | `[0x1491ac]` | `0x0` |
| `bl` at 0x5fbb4 | `(unknown callee) 0x47e18` | `[sp-36]` | `[sp]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5fa5c | `[0x1491a0]:1 == 0x0` | `[0x1491a0]:1 != 0x0` |
| `bne` at 0x5fac8 | `[0x2b1680] != 0x0` | `[0x2b1680] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5fbb8 | `r0` | `ret(0x47e18)` |

## `IP57_Set_MemCfg`

- `IP57_Set_MemCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f970 to 0x5fa30.
  Established. Entry `r0 == 0x64` takes `0x5f9b0`, while inequality falls through (`beq` at
  0x5f9a8). The word assignments at `sp -0x14` and `sp -0x18` write `0x10cf70` and `0x6` (`str` at
  0x5f9b4, `str` at 0x5f9bc). All four candidates for the unnamed `0x47e18` call are unknown (`bl` at 0x5fa18). Memory configuration meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5f980 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x5f984 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x5f988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5f9a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f9b0 | load | 4 | `abs 0x5fa20` | `-` |
| `str` at 0x5f9b4 | store | 4 | `sp -0x14` | `0x10cf70` |
| `str` at 0x5f9bc | store | 4 | `sp -0x18` | `0x6` |
| `ldr` at 0x5f9c4 | load | 4 | `abs 0x5fa24` | `-` |
| `str` at 0x5f9c8 | store | 4 | `sp -0x20` | `0x10d0dc` |
| `ldr` at 0x5f9cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5f9d0 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x5f9d8 | load | 4 | `abs 0x5fa28` | `-` |
| `ldr` at 0x5f9e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5f9f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5fa00 | load | 4 | `abs 0x5fa2c` | `-` |
| `str` at 0x5fa04 | store | 4 | `sp -0x14` | `0x10cfd0` |
| `str` at 0x5fa0c | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x5fa10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5fa14 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5fa1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5fa1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5fa1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f9e4 | `(unknown callee) 0x21eb8` | `0x9` | `0x10d08c` | `0xd4` | `0x0` |
| `bl` at 0x5fa18 | `(unknown callee) 0x47e18` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f990 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x5f99c | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0x5f9a8 | `r0 == 0x64` | `r0 != 0x64` |
| `beq` at 0x5f9f0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x5f9fc | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5fa1c | `r0` | `ret(0x47e18)` |

## `IP57_Probe`

- `IP57_Probe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fca0 to 0x5ff1c.
  Established. The unnamed `0x617b4` call carries `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x5fcd0); its zero result takes `0x5fcf4`, while nonzero falls through (`beq` at
  0x5fce4). The byte assignment at `[sp-17]:1 +0x2b1660` writes `0x2` (`strb` at 0x5fd10). Probe
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5fcb4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5fcbc | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x5fcc0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5fcd8 | store | 4 | `sp -0x20` | `ret(0x617b4)` |
| `ldr` at 0x5fcdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5fcec | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5fcf4 | load | 4 | `abs 0x5ff04` | `-` |
| `ldrb` at 0x5fcf8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5fd00 | store | 1 | `[sp-17]:1 +0x1491b4` | `0x0` |
| `ldr` at 0x5fd04 | load | 4 | `abs 0x5ff08` | `-` |
| `ldrb` at 0x5fd08 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5fd10 | store | 1 | `[sp-17]:1 +0x2b1660` | `0x2` |
| `ldr` at 0x5fd14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5fd18 | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x5fd20 | store | 1 | `sp -0x21` | `([[sp-24]+3]:1+5 & 0xff)` |
| `ldrb` at 0x5fd24 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5fd28 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5fd2c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x5fd30 | load | 4 | `abs 0x5ff0c` | `-` |
| `ldr` at 0x5fd34 | load | 4 | `abs 0x1491ac` | `-` |
| `ldrb` at 0x5fd4c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5fd5c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x5fd64 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `strb` at 0x5fd70 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x5fd74 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5fd84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5fd88 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x5fd8c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x5fdb0 | store | 4 | `sp -0x20` | `ret(0xd7530)` |
| `ldr` at 0x5fdb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5fdc0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x5fdd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5fde0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x5fdec | load | 4 | `abs 0x5ff10` | `-` |
| `str` at 0x5fdf0 | store | 4 | `sp -0x34` | `0x10d138` |
| `ldrb` at 0x5fdf4 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x5fdf8 | store | 4 | `sp -0x30` | `[sp-33]:1` |
| `str` at 0x5fdfc | store | 4 | `sp -0x2c` | `ret(0x21a38)` |
| `ldr` at 0x5fe04 | load | 4 | `abs 0x5ff14` | `-` |
| `ldr` at 0x5fe08 | load | 4 | `abs 0x5ff18` | `-` |
| `ldrb` at 0x5fe14 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fe18 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5fe2c | load | 4 | `abs 0x5ff04` | `-` |
| `ldrb` at 0x5fe30 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5fe34 | load | 4 | `abs 0x5ff04` | `-` |
| `ldrb` at 0x5fe38 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fe3c | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5fe48 | load | 1 | `[sp-17]:1 +0x1491b4` | `-` |
| `strb` at 0x5fe50 | store | 1 | `[sp-17]:1 +0x1491b4` | `?` |
| `ldrb` at 0x5fe54 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x5fe64 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fe70 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x5fe90 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fe94 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x5fea8 | load | 4 | `abs 0x5ff08` | `-` |
| `ldrb` at 0x5feac | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5feb4 | store | 1 | `[sp-17]:1 +0x2b1660` | `0x1` |
| `ldrb` at 0x5feb8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x5fec0 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x5fec8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fee0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5fef8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x5fefc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x5ff00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ff00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ff00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5fcd0 | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5fd58 | `(unknown callee) 0xd224c` | `0x0` | `?` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0x5fda8 | `(unknown callee) 0xd7530` | `0x0` | `?` | `sp-25` | `?` |
| `bl` at 0x5fde4 | `(unknown callee) 0x21a38` | `ret(0xd7530)` | `?` | `?` | `ret(0xd7530)` |
| `bl` at 0x5fe10 | `(unknown callee) 0x21eb8` | `0x1` | `0x10d08c` | `0x165` | `0x0` |
| `bl` at 0x5fe24 | `(unknown callee) 0x608d8` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5fe80 | `(unknown callee) 0x5e14c` | `?` | `[sp-17]:1` | `?` | `?` |
| `bl` at 0x5fea0 | `(unknown callee) 0x607e0` | `[sp-17]:1` | `[sp-33]:1` | `[sp-33]:1` | `[sp-17]:1` |
| `bl` at 0x5fed0 | `(unknown callee) 0x5e7d8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x5fef0 | `(unknown callee) 0x60204` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5fce4 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `blt` at 0x5fd44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bls` at 0x5fd7c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x5fdbc | `ret(0xd7530) != 0x0` | `ret(0xd7530) == 0x0` |
| `bne` at 0x5fdcc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x5fddc | `ret(0xd7530) == 0x0` | `ret(0xd7530) != 0x0` |
| `beq` at 0x5fe60 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x5fe8c | `ret(0x5e14c) != 0x0` | `ret(0x5e14c) == 0x0` |
| `bne` at 0x5fedc | `ret(0x5e7d8) != 0x0` | `ret(0x5e7d8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ff00 | `r0` | `?` |

## `IP57_Init`

- `IP57_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ff1c to 0x5ff94.
  Established. The unnamed `0x60204` call carries `[sp-17]:1`, `0x0`, `0x1`, `[sp-17]:1` (`bl` at
  0x5ff74). The word assignments at `sp -0x18` write `0x0` and `0x1` (`str` at 0x5ff7c, `str` at
  0x5ff88). Initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5ff30 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5ff34 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5ff4c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5ff64 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5ff7c | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x5ff88 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x5ff8c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5ff90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ff90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ff90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ff3c | `(unknown callee) 0x5e754` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x5ff54 | `(unknown callee) 0x5e7d8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x5ff74 | `(unknown callee) 0x60204` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ff48 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `bne` at 0x5ff60 | `ret(0x5e7d8) != 0x0` | `ret(0x5e7d8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ff90 | `r0` | `?` |

## `IP57_NodeRemove`

- `IP57_NodeRemove`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ff94 to 0x600a8.
  Established. The unnamed `0x60204` call carries `[sp-17]:1`, `0x1`, `0x1`, `[sp-17]:1` (`bl` at
  0x5fff0). The byte assignment at `[sp-17]:1 +0x2b1660` writes `0xf` (`strb` at 0x60090).
  Node-removal effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5ffa8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5ffac | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5ffc4 | store | 4 | `sp -0x1c` | `ret(0x617b4)` |
| `ldr` at 0x5ffc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5ffd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5ffd8 | store | 4 | `sp -0x24` | `ret(0x617b4)` |
| `ldrb` at 0x5ffe0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5fff4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5fff8 | load | 1 | `[sp-24] +0x3` | `-` |
| `strb` at 0x5fffc | store | 1 | `sp -0x1d` | `([[sp-24]+3]:1 & 0xff)` |
| `ldrb` at 0x60000 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x60004 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60008 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0x6001c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x6002c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x60034 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x6003c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60040 | load | 1 | `[sp-24] +0x0` | `-` |
| `strb` at 0x60044 | store | 1 | `sp -0x1d` | `([[sp-24]]:1 & 0xff)` |
| `ldrb` at 0x60048 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x6004c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60050 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrb` at 0x60064 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x60074 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x6007c | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x60084 | load | 4 | `abs 0x600a4` | `-` |
| `ldrb` at 0x60088 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x60090 | store | 1 | `[sp-17]:1 +0x2b1660` | `0xf` |
| `str` at 0x60098 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6009c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x600a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x600a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x600a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ffbc | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5fff0 | `(unknown callee) 0x60204` | `[sp-17]:1` | `0x1` | `0x1` | `[sp-17]:1` |
| `bl` at 0x60028 | `(unknown callee) 0xd21b4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x60070 | `(unknown callee) 0xd513c` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ffd0 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `blt` at 0x60014 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x6005c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x600a0 | `r0` | `?` |

## `IP57_NodeEnable`

- `IP57_NodeEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x600e0 to 0x60114.
  Established. The unnamed `0x603c8` call carries `(r0 & 0xff)`, `0x0`, `r2`, `(r0 & 0xff)` (`bl` at
  0x60104); its result is the reported return value (`ldmdb` at 0x60110). Node enabling remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x600f4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x600f8 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x60110 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60110 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60110 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60104 | `(unknown callee) 0x603c8` | `(r0 & 0xff)` | `0x0` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60110 | `r0` | `ret(0x603c8)` |

## `IP57_NodeDisable`

- `IP57_NodeDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x600a8 to 0x600e0.
  Established. The unnamed `0x60204` call uses `(r0 & 0xff)`, `0x0`, `0x1`, `(r0 & 0xff)` (`bl` at
  0x600d0). The return reports `ret(0x60204)` (`ldmdb` at 0x600dc). Node disabling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x600bc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x600c0 | load | 1 | `sp -0x11` | `-` |
| `ldmdb` at 0x600dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x600dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x600dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x600d0 | `(unknown callee) 0x60204` | `(r0 & 0xff)` | `0x0` | `0x1` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x600dc | `r0` | `ret(0x60204)` |

## `IP57_NodePresent`

- `IP57_NodePresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6064c to 0x60688.
  Established. The byte read uses `(r0 & 0xff) +0x2b1660` (`ldrb` at 0x6066c). The reported return
  value is unknown (`ldmdb` at 0x60680). Presence encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60660 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x60664 | load | 4 | `abs 0x60684` | `-` |
| `ldrb` at 0x60668 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6066c | load | 1 | `(r0 & 0xff) +0x2b1660` | `-` |
| `ldmdb` at 0x60680 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60680 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60680 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60680 | `r0` | `?` |

## `IP57_CpuPresent`

- `IP57_CpuPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60688 to 0x60700.
  Established. The unnamed `0x1f26c` call carries `0x7b`, `r1`, `r2`, unknown (`bl` at 0x606bc). A
  result unequal to `0x1` takes `0x606d8`, while equality falls through (`bne` at 0x606c8). The byte
  load at `[sp-18]:1 +0x1491b4` has an unknown value (`ldrb` at 0x606e0). CPU presence bits remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6069c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x606a0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x606a8 | store | 1 | `sp -0x12` | `?` |
| `ldrb` at 0x606ac | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x606b4 | store | 1 | `sp -0x13` | `?` |
| `str` at 0x606d0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x606d8 | load | 4 | `abs 0x606fc` | `-` |
| `ldrb` at 0x606dc | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x606e0 | load | 1 | `[sp-18]:1 +0x1491b4` | `-` |
| `ldrb` at 0x606e4 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x606f0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x606f4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x606f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x606f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x606f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x606bc | `(unknown callee) 0x1f26c` | `0x7b` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x606c8 | `ret(0x1f26c) != 0x1` | `ret(0x1f26c) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x606f8 | `r0` | `?` |

## `IP57_CpuDisable`

- `IP57_CpuDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60700 to 0x60770.
  Established. The unnamed `0x60688` call carries `(r0 & 0xff)`, `r1`, `r2`, `(r0 & 0xff)` (`bl` at
  0x60720); its zero result takes `0x6075c`, while nonzero falls through (`beq` at 0x6072c). The
  word assignment at `sp -0x18` writes `0xfffff50e` (`str` at 0x60760). CPU disable effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60714 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60718 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60730 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6073c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60754 | store | 4 | `sp -0x18` | `ret(0x607e0)` |
| `ldr` at 0x6075c | load | 4 | `abs 0x6076c` | `-` |
| `str` at 0x60760 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x60764 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x60768 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60768 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60768 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60720 | `(unknown callee) 0x60688` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x6074c | `(unknown callee) 0x607e0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6072c | `ret(0x60688) == 0x0` | `ret(0x60688) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60768 | `r0` | `?` |

## `IP57_CpuEnable`

- `IP57_CpuEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60770 to 0x607e0.
  Established. The unnamed `0x6099c` call has four unknown candidates (`bl` at 0x607bc). The word
  assignment at `sp -0x18` writes `0xfffff50e` (`str` at 0x607d0). CPU enable effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60784 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60788 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x607a0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x607ac | load | 1 | `sp -0x11` | `-` |
| `str` at 0x607c4 | store | 4 | `sp -0x18` | `ret(0x6099c)` |
| `ldr` at 0x607cc | load | 4 | `abs 0x607dc` | `-` |
| `str` at 0x607d0 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x607d4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x607d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x607d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x607d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60790 | `(unknown callee) 0x60688` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x607bc | `(unknown callee) 0x6099c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6079c | `ret(0x60688) == 0x0` | `ret(0x60688) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x607d8 | `r0` | `?` |

## `IP57_Nmi`

- `IP57_Nmi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x613d4 to 0x614b0.
  Established. The unnamed `0xd7368` call carries `0x0`, `([[sp-24]+2]:1+2 & 0xff)`, `0x10`,
  `([[sp-24]+2]:1+2 & 0xff)` (`bl` at 0x61440); its zero result takes `0x61464`, while nonzero falls
  through (`beq` at 0x61454). The unnamed `0xd731c` call carries `0x0`, `[sp-25]:1`, `0x10`,
  `[sp-25]:1` (`bl` at 0x6147c). NMI signaling effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x613e8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x613ec | load | 1 | `sp -0x11` | `-` |
| `str` at 0x61404 | store | 4 | `sp -0x20` | `ret(0x617b4)` |
| `ldr` at 0x61408 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61414 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61418 | store | 4 | `sp -0x24` | `ret(0x617b4)` |
| `ldr` at 0x61420 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x61424 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x6142c | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x61430 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x61448 | store | 4 | `sp -0x20` | `ret(0xd7368)` |
| `ldr` at 0x6144c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61458 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6145c | store | 4 | `sp -0x24` | `ret(0xd7368)` |
| `ldrb` at 0x6146c | load | 1 | `sp -0x19` | `-` |
| `str` at 0x61484 | store | 4 | `sp -0x20` | `ret(0xd731c)` |
| `ldr` at 0x61488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x61494 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x61498 | store | 4 | `sp -0x24` | `ret(0xd731c)` |
| `str` at 0x614a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x614a8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x614ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x614ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x614ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x613fc | `(unknown callee) 0x617b4` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x61440 | `(unknown callee) 0xd7368` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x10` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x61468 | `(unknown callee) 0x61854` | `0xa` | `?` | `?` | `ret(0xd7368)` |
| `bl` at 0x6147c | `(unknown callee) 0xd731c` | `0x0` | `[sp-25]:1` | `0x10` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x61410 | `ret(0x617b4) == 0x0` | `ret(0x617b4) != 0x0` |
| `beq` at 0x61454 | `ret(0xd7368) == 0x0` | `ret(0xd7368) != 0x0` |
| `beq` at 0x61490 | `ret(0xd731c) == 0x0` | `ret(0xd731c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x614ac | `r0` | `?` |

## `IP57_Get_Cfg`

- `IP57_Get_Cfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61724 to 0x61784.
  Established. The word at `0x1491a8` is read (`ldr` at 0x6173c). Nonzero takes `0x61754`, while
  zero falls through (`bne` at 0x61744). The word assignment through `r0 +0x0` writes `[0x1491a8]`
  (`str` at 0x61760). Configuration record contents remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x61734 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x61738 | load | 4 | `abs 0x61778` | `-` |
| `ldr` at 0x6173c | load | 4 | `abs 0x1491a8` | `-` |
| `ldr` at 0x61748 | load | 4 | `abs 0x6177c` | `-` |
| `str` at 0x6174c | store | 4 | `sp -0x18` | `0xfffff510` |
| `ldr` at 0x61754 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x61758 | load | 4 | `abs 0x61778` | `-` |
| `ldr` at 0x6175c | load | 4 | `abs 0x1491a8` | `-` |
| `str` at 0x61760 | store | 4 | `r0 +0x0` | `[0x1491a8]` |
| `ldr` at 0x61764 | load | 4 | `abs 0x61780` | `-` |
| `ldr` at 0x61768 | load | 4 | `abs 0x1491a4` | `-` |
| `str` at 0x6176c | store | 4 | `sp -0x18` | `[0x1491a4]` |
| `ldr` at 0x61770 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x61774 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x61774 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x61774 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x61744 | `[0x1491a8] != 0x0` | `[0x1491a8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x61774 | `r0` | `?` |

## `IP57_GetCpuMask`

- `IP57_GetCpuMask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x61784 to 0x617b4.
  Established. The byte at `(r0 & 0xff) +0x1491b4` is loaded (`ldrb` at 0x617a4). The reported
  return value is `[(r0 & 0xff)+1348020]:1` (`ldmdb` at 0x617ac). CPU mask interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x61798 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x6179c | load | 4 | `abs 0x617b0` | `-` |
| `ldrb` at 0x617a0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x617a4 | load | 1 | `(r0 & 0xff) +0x1491b4` | `-` |
| `ldmdb` at 0x617ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x617ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x617ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x617ac | `r0` | `[(r0 & 0xff)+1348020]:1` |
