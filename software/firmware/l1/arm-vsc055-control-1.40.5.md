# ARM VSC055 control callers in 1.40.5

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
These routines have no unresolved computed transfers in the fact sheets. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/vsc055-control-facts.txt) retain all tool sections.

## `vsc055_Init`

- `vsc055_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8cd8 to 0xd8ef8.
  Established. The branch takes `[0x143820] != 0x0` past a four-byte `0x1` assignment at absolute
  `0x143820` (`bne` at 0xd8d54, `str` at 0xd8d60). The unnamed call to `0x1b454` carries `[sp-40]`,
  `0x1`, `[0x143820]`, unknown (`bl` at 0xd8d8c); its result is stored as four bytes at absolute
  `0x14381c` (`str` at 0xd8d94). Another call result is assigned at `[sp-36] +0x8` (`bl` at 0xd8e14,
  `str` at 0xd8e1c). Allocation effects and device initialization semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8ce8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xd8cec | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xd8cf0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xd8cf4 | load | 4 | `abs 0xd8ed4` | `-` |
| `ldr` at 0xd8cf8 | load | 4 | `abs 0x14381c` | `-` |
| `ldr` at 0xd8cfc | load | 4 | `abs 0xd8ed8` | `-` |
| `ldr` at 0xd8d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8d14 | load | 4 | `abs 0xd8edc` | `-` |
| `str` at 0xd8d18 | store | 4 | `sp -0x34` | `0x135698` |
| `ldr` at 0xd8d1c | load | 4 | `abs 0xd8ed4` | `-` |
| `ldr` at 0xd8d20 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd8d24 | store | 4 | `sp -0x30` | `[0x14381c]` |
| `ldr` at 0xd8d28 | load | 4 | `abs 0xd8ee0` | `-` |
| `ldr` at 0xd8d2c | load | 4 | `abs 0x143820` | `-` |
| `str` at 0xd8d30 | store | 4 | `sp -0x2c` | `[0x143820]` |
| `ldr` at 0xd8d38 | load | 4 | `abs 0xd8ee4` | `-` |
| `ldr` at 0xd8d48 | load | 4 | `abs 0xd8ee0` | `-` |
| `ldr` at 0xd8d4c | load | 4 | `abs 0x143820` | `-` |
| `ldr` at 0xd8d58 | load | 4 | `abs 0xd8ee0` | `-` |
| `str` at 0xd8d60 | store | 4 | `abs 0x143820` | `0x1` |
| `ldr` at 0xd8d64 | load | 4 | `abs 0xd8ee0` | `-` |
| `ldr` at 0xd8d68 | load | 4 | `abs 0x143820` | `-` |
| `str` at 0xd8d7c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd8d80 | load | 4 | `abs 0xd8ed4` | `-` |
| `ldr` at 0xd8d84 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8d94 | store | 4 | `abs 0x14381c` | `ret(0x1b454)` |
| `ldr` at 0xd8d98 | load | 4 | `abs 0x14381c` | `-` |
| `ldr` at 0xd8da4 | load | 4 | `abs 0xd8ee8` | `-` |
| `str` at 0xd8da8 | store | 4 | `sp -0x34` | `0x1356e4` |
| `ldr` at 0xd8dac | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd8db0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xd8db8 | load | 4 | `abs 0xd8ee4` | `-` |
| `ldr` at 0xd8dc8 | load | 4 | `abs 0xd8ed4` | `-` |
| `ldr` at 0xd8dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8de0 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd8de8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd8dec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8df0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd8df4 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xd8df8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8dfc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8e00 | store | 4 | `[sp-36] +0x4` | `[sp-32]` |
| `ldr` at 0xd8e04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8e0c | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0xd8e1c | store | 4 | `[sp-36] +0x8` | `ret(0x1b454)` |
| `ldr` at 0xd8e20 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd8e2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8e34 | load | 4 | `abs 0xd8eec` | `-` |
| `ldr` at 0xd8e38 | load | 4 | `abs 0xd8ef0` | `-` |
| `str` at 0xd8e64 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd8e68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8e70 | load | 4 | `abs 0xd8eec` | `-` |
| `ldr` at 0xd8e74 | load | 4 | `abs 0xd8ef0` | `-` |
| `ldr` at 0xd8e80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8e90 | load | 4 | `abs 0xd8ef4` | `-` |
| `str` at 0xd8e94 | store | 4 | `(unknown)` | `0x135728` |
| `str` at 0xd8e98 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldr` at 0xd8e9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd8ea0 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xd8ea8 | load | 4 | `abs 0xd8ee4` | `-` |
| `ldr` at 0xd8eb8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8ebc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8ec0 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0xd8ec8 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldmdb` at 0xd8ed0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd8ed0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd8ed0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd8ed0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8d44 | `(unknown callee) 0x21eb8` | `0x9` | `0x1356c8` | `0x47` | `0x0` |
| `bl` at 0xd8d8c | `(unknown callee) 0x1b454` | `[sp-40]` | `0x1` | `[0x143820]` | `?` |
| `bl` at 0xd8dc4 | `(unknown callee) 0x21eb8` | `0x9` | `0x1356c8` | `0x53` | `0x0` |
| `bl` at 0xd8e14 | `(unknown callee) 0x1b454` | `[[sp-36]+4]` | `0x1` | `[sp-36]` | `[sp-36]` |
| `bl` at 0xd8e3c | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd8e48 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8e78 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd8e88 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd8eb4 | `(unknown callee) 0x21eb8` | `0x9` | `0x1356c8` | `0x5f` | `0x0` |
| `bl` at 0xd8ecc | `(unknown callee) 0x1b4cc` | `[[sp-36]+8]` | `0x1` | `[[sp-36]+4]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd8d04 | `[0x14381c] == 0x2adfa0` | `[0x14381c] != 0x2adfa0` |
| `bne` at 0xd8d10 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xd8d54 | `[0x143820] != 0x0` | `[0x143820] == 0x0` |
| `bne` at 0xd8da0 | `[0x14381c] != 0x0` | `[0x14381c] == 0x0` |
| `bne` at 0xd8e28 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd8ed0 | `r0` | `ret(0x1b4cc)` |

## `vsc055_Enable`

- `vsc055_Enable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd8ef8 to 0xd90b4.
  Established. The branch is taken when `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)`,
  reaching `0xd9010` (`bge` at 0xd8f48). The other path includes an unnamed call to `0xd7210` with `r0`, `sp-36`, unknown,
  `sp-36` (`bl` at 0xd8f70). The later byte store writes `0x1` at `([sp-21]:1 + [[sp-28]+8]) +0x0`
  (`strb` at 0xd9008). Hardware enable semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd8f08 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd8f10 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd8f14 | load | 4 | `abs 0xd90a0` | `-` |
| `ldr` at 0xd8f18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8f2c | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd8f34 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd8f38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd8f40 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd8f4c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8f50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8f58 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd8f60 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd8f68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd8f78 | store | 4 | `sp -0x28` | `ret(0xd7210)` |
| `strb` at 0xd8f80 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xd8f84 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd8f88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd8f98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8f9c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8fa0 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd8fa4 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd8fb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd8fb4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd8fb8 | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd8fbc | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd8fc8 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd8fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd8fd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd8fe0 | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xd8fe4 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xd8fec | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xd8ff4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd8ff8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd8ffc | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd9008 | store | 1 | `([sp-21]:1 + [[sp-28]+8]) +0x0` | `0x1` |
| `ldr` at 0xd9010 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9018 | load | 4 | `abs 0xd90a4` | `-` |
| `ldr` at 0xd901c | load | 4 | `abs 0xd90a8` | `-` |
| `str` at 0xd9048 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd904c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9054 | load | 4 | `abs 0xd90a4` | `-` |
| `ldr` at 0xd9058 | load | 4 | `abs 0xd90a8` | `-` |
| `ldr` at 0xd9064 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd9074 | load | 4 | `abs 0xd90ac` | `-` |
| `str` at 0xd9078 | store | 4 | `(unknown)` | `0x135764` |
| `str` at 0xd907c | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd9080 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd9084 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd908c | load | 4 | `abs 0xd90b0` | `-` |
| `ldmdb` at 0xd909c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd909c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd909c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd8f70 | `(unknown callee) 0xd7210` | `r0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xd8fd4 | `(unknown callee) 0xd6798` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd9020 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd902c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd905c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd906c | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9098 | `(unknown callee) 0x21eb8` | `0x6` | `0x1356c8` | `0x7e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd8f48 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `blt` at 0xd8f90 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd8fac | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xd8fc4 | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd909c | `r0` | `?` |

## `vsc055_Disable`

- `vsc055_Disable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd90b4 to 0xd926c.
  Established. The branch takes `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` to `0xd91c8` (`bge` at
  0xd9104). The fall-through writes byte `0x0` at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`strb` at
  0xd911c). A later unnamed call to `0xd68bc` has candidates `[sp-20]`, unknown, `[sp-32]`, unknown
  (`bl` at 0xd91a8). Disabling a physical device is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd90c4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd90cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd90d0 | load | 4 | `abs 0xd9258` | `-` |
| `ldr` at 0xd90d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd90e8 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd90f0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd90f4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd90f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd90fc | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd9108 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd910c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9110 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xd911c | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xd9120 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9124 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd912c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd9134 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd913c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd914c | store | 4 | `sp -0x28` | `ret(0xd7210)` |
| `strb` at 0xd9154 | store | 1 | `sp -0x29` | `0x0` |
| `ldrb` at 0xd9158 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd915c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd916c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9170 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9174 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xd9178 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd9184 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9188 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd918c | load | 1 | `[sp-36] +0x2` | `-` |
| `ldrb` at 0xd9190 | load | 1 | `[sp-32] +0x1` | `-` |
| `ldrb` at 0xd919c | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xd91a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd91ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd91b4 | store | 4 | `sp -0x24` | `[sp-36]+16` |
| `ldrb` at 0xd91b8 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xd91c0 | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xd91c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd91d0 | load | 4 | `abs 0xd925c` | `-` |
| `ldr` at 0xd91d4 | load | 4 | `abs 0xd9260` | `-` |
| `str` at 0xd9200 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd9204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd920c | load | 4 | `abs 0xd925c` | `-` |
| `ldr` at 0xd9210 | load | 4 | `abs 0xd9260` | `-` |
| `ldr` at 0xd921c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd922c | load | 4 | `abs 0xd9264` | `-` |
| `str` at 0xd9230 | store | 4 | `(unknown)` | `0x135784` |
| `str` at 0xd9234 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd9238 | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd923c | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd9244 | load | 4 | `abs 0xd9268` | `-` |
| `ldmdb` at 0xd9254 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9254 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9254 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9144 | `(unknown callee) 0xd7210` | `[sp-20]` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xd91a8 | `(unknown callee) 0xd68bc` | `[sp-20]` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd91d8 | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd91e4 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9214 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd9224 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9250 | `(unknown callee) 0x21eb8` | `0x6` | `0x1356c8` | `0x9e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xd9104 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |
| `blt` at 0xd9164 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xd9180 | `[[sp-36]+1]:1 != [[sp-32]]:1` | `[[sp-36]+1]:1 == [[sp-32]]:1` |
| `bne` at 0xd9198 | `[[sp-36]+2]:1 != [[sp-32]+1]:1` | `[[sp-36]+2]:1 == [[sp-32]+1]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9254 | `r0` | `?` |

## `vsc055_ResetAll`

- `vsc055_ResetAll`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd926c to 0xd9308.
  Established. The four-byte load reads absolute `0x14381c` (`ldr` at 0xd9298). A later byte read
  has unresolved effective address (`ldrb` at 0xd92d4). The zero-flag branch is taken when `Z == 1`,
  reaching `0xd92f0` (`beq` at 0xd92dc); its fall-through includes an unnamed call to `0xd9308`
  whose four candidates are unknown (`bl` at 0xd92ec). Reset coverage and physical effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd927c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd9280 | load | 4 | `abs 0xd9304` | `-` |
| `ldr` at 0xd9284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9298 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd92a0 | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xd92a8 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd92ac | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd92b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd92b4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd92c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd92c8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd92cc | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xd92d4 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd92e0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd92e4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd92f0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd92f8 | store | 1 | `sp -0x19` | `?` |
| `ldmdb` at 0xd9300 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9300 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9300 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd92ec | `(unknown callee) 0xd9308` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd92bc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd92dc | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9300 | `r0` | `?` |

## `vsc055_Reset`

- `vsc055_Reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9308 to 0xd95e4.
  Established. The unnamed call to `0xc8874` carries unknown, `[[sp-32]+1]:1`, `0xfc`, `0x80` (`bl`
  at 0xd946c). A zero result takes the branch to `0xd9488` (`beq` at 0xd9480). A later call to
  `0xc8998` has unknown, `[[sp-32]+1]:1`, `0xff`, `sp-33` (`bl` at 0xd94c8). The branch takes
  `[sp-33]:1 != 0x21` to `0xd94f4` (`bne` at 0xd94ec). Device-reset register meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9318 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd9320 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd9324 | load | 4 | `abs 0xd95c8` | `-` |
| `ldr` at 0xd9328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd933c | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd9344 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd9348 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd934c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9350 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xd935c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9364 | load | 4 | `abs 0xd95cc` | `-` |
| `ldr` at 0xd9368 | load | 4 | `abs 0xd95d0` | `-` |
| `str` at 0xd9394 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd9398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd93a0 | load | 4 | `abs 0xd95cc` | `-` |
| `ldr` at 0xd93a4 | load | 4 | `abs 0xd95d0` | `-` |
| `ldr` at 0xd93b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd93c0 | load | 4 | `abs 0xd95d4` | `-` |
| `str` at 0xd93c4 | store | 4 | `(unknown)` | `0x1357a8` |
| `str` at 0xd93c8 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0xd93cc | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd93d0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0xd93d8 | load | 4 | `abs 0xd95d8` | `-` |
| `ldr` at 0xd93e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd93ec | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd93f0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd93f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd9408 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd940c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9414 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd941c | store | 4 | `sp -0x20` | `?` |
| `str` at 0xd9424 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd9428 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9438 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd943c | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd9440 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9454 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9458 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xd9474 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xd9478 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd9490 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9494 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd9498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94ac | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd94b0 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0xd94d0 | store | 4 | `sp -0x28` | `ret(0xc8998)` |
| `ldr` at 0xd94d4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xd94e4 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd94f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd94fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd9504 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd950c | load | 4 | `abs 0xd95cc` | `-` |
| `ldr` at 0xd9510 | load | 4 | `abs 0xd95d0` | `-` |
| `str` at 0xd953c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd9540 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9548 | load | 4 | `abs 0xd95cc` | `-` |
| `ldr` at 0xd954c | load | 4 | `abs 0xd95d0` | `-` |
| `ldr` at 0xd9558 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd9564 | store | 4 | `sp -0x34` | `ret(0x1b598)` |
| `ldr` at 0xd9568 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd9574 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd957c | store | 4 | `sp -0x38` | `ret(0x21a38)` |
| `ldr` at 0xd9584 | load | 4 | `abs 0xd95dc` | `-` |
| `str` at 0xd9588 | store | 4 | `sp -0x38` | `0x1357c0` |
| `ldr` at 0xd958c | load | 4 | `abs 0xd95e0` | `-` |
| `str` at 0xd9590 | store | 4 | `(unknown)` | `0x1357d8` |
| `ldr` at 0xd9594 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd9598 | store | 4 | `(unknown)` | `[sp-52]` |
| `ldr` at 0xd959c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd95a0 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0xd95a4 | store | 4 | `(unknown)` | `[[sp-32]+4]` |
| `ldr` at 0xd95a8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd95ac | store | 4 | `(unknown)` | `[sp-56]` |
| `ldr` at 0xd95b4 | load | 4 | `abs 0xd95d8` | `-` |
| `ldmdb` at 0xd95c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd95c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd95c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd936c | `(unknown callee) 0x4f9a4` | `r0` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd9378 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd93a8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd93b8 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd93e4 | `(unknown callee) 0x21eb8` | `0x9` | `0x1356c8` | `0xc1` | `0x0` |
| `bl` at 0xd9448 | `(unknown callee) 0xc807c` | `?` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xd946c | `(unknown callee) 0xc8874` | `?` | `[[sp-32]+1]:1` | `0xfc` | `0x80` |
| `bl` at 0xd948c | `(unknown callee) 0xda7e0` | `0x2` | `?` | `?` | `ret(0xc8874)` |
| `bl` at 0xd94a0 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-32]]:1` | `?` | `[[sp-32]]:1` |
| `bl` at 0xd94c8 | `(unknown callee) 0xc8998` | `?` | `[[sp-32]+1]:1` | `0xff` | `sp-33` |
| `bl` at 0xd9514 | `(unknown callee) 0x4f9a4` | `?` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd9520 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9550 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd9560 | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9578 | `(unknown callee) 0x21a38` | `[sp-40]` | `?` | `?` | `[sp-40]` |
| `bl` at 0xd95c0 | `(unknown callee) 0x21eb8` | `0x3` | `0x1356c8` | `0xe1` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xd9358 | `(r1 & 0xff) <= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) > [[sp-28]+4] (signed, 32-bit)` |
| `bne` at 0xd9400 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xd9430 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xd9480 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `beq` at 0xd94dc | `ret(0xc8998) == 0x0` | `ret(0xc8998) != 0x0` |
| `bne` at 0xd94ec | `[sp-33]:1 != 0x21` | `[sp-33]:1 == 0x21` |
| `beq` at 0xd9570 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd95c4 | `r0` | `?` |

## `vsc055_PortReset`

- `vsc055_PortReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd95e4 to 0xd9730.
  Established. Two byte stores at `sp -0x1e` and `sp -0x1d` have unknown values (`strb` at 0xd9634,
  `strb` at 0xd9660). The unnamed call to `0xc8874` carries unknown, `[[sp-24]+2]:1`,
  `[[sp-24]+3]:1`, `[sp-30]:1` (`bl` at 0xd96a4). A second call to that target uses unknown,
  `[[sp-24]+2]:1`, unknown, `[sp-29]:1` (`bl` at 0xd96f4); its zero-result branch reaches `0xd9720`
  (`beq` at 0xd9708). Port-reset signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd95f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd95f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd95fc | load | 4 | `abs 0xd972c` | `-` |
| `ldr` at 0xd9600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9614 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd961c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd9620 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9624 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9628 | load | 1 | `r1 +0x5` | `-` |
| `ldrb` at 0xd962c | load | 1 | `r1 +0x6` | `-` |
| `strb` at 0xd9634 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd9638 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd963c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9640 | load | 1 | `r1 +0x5` | `-` |
| `ldr` at 0xd9648 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9650 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldrb` at 0xd9658 | load | 1 | `r1 +0x4` | `-` |
| `strb` at 0xd9660 | store | 1 | `sp -0x1d` | `?` |
| `ldr` at 0xd9664 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9668 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd966c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9680 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9684 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9688 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd968c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd9690 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd96ac | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xd96b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd96b4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd96b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd96cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd96d0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd96d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd96d8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd96e4 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xd96fc | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xd9700 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd970c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9718 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd971c | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xd9720 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd9728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9674 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `[r1+4]:1` | `[[sp-24]+1]:1` |
| `bl` at 0xd96a4 | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `[sp-30]:1` |
| `bl` at 0xd96c0 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd96f4 | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9708 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xd9714 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9728 | `r0` | `[sp-36]` |

## `vsc055_PortSoftReset`

- `vsc055_PortSoftReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9730 to 0xd9a9c.
  Established. The unnamed call to `0xc8998` has candidates unknown, `[[sp-24]+2]:1`, unknown,
  `sp-27` (`bl` at 0xd9794); its zero result takes the branch to `0xd97b8` (`beq` at 0xd97a8). Later
  calls to `0xc8874` include candidates unknown, `[[sp-24]+2]:1`, unknown, `[sp-28]:1`, and unknown,
  `[[sp-24]+2]:1`, `[[sp-24]+3]:1`, unknown (`bl` at 0xd99e4, `bl` at 0xd9a5c). The byte store at
  `sp -0x1c` has unknown value (`strb` at 0xd9888). Soft-reset effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9740 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9744 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xd974c | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xd9750 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9754 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0xd9758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd976c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9770 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9774 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9778 | load | 1 | `[sp-24] +0x3` | `-` |
| `str` at 0xd979c | store | 4 | `sp -0x20` | `ret(0xc8998)` |
| `ldr` at 0xd97a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd97ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd97b0 | store | 4 | `sp -0x28` | `ret(0xc8998)` |
| `ldr` at 0xd97b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd97bc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd97c0 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0xd97d8 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xd97dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd97e0 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd97e8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd97ec | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd97f4 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd9804 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd980c | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0xd9810 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9814 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd9818 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd981c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd9824 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd9830 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd9840 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0xd9848 | store | 1 | `sp -0x1a` | `?` |
| `ldrb` at 0xd984c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xd9858 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd985c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9860 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9864 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd9868 | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd9878 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd9880 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0xd9888 | store | 1 | `sp -0x1c` | `?` |
| `ldr` at 0xd988c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9894 | load | 4 | `abs 0xd9a90` | `-` |
| `ldr` at 0xd9898 | load | 4 | `abs 0xd9a94` | `-` |
| `str` at 0xd98c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd98c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd98d0 | load | 4 | `abs 0xd9a90` | `-` |
| `ldr` at 0xd98d4 | load | 4 | `abs 0xd9a94` | `-` |
| `ldr` at 0xd98e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd98f0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd98f4 | load | 1 | `sp -0x1a` | `-` |
| `ldrb` at 0xd98f8 | load | 1 | `sp -0x1b` | `-` |
| `str` at 0xd98fc | store | 4 | `(unknown)` | `[sp-27]:1` |
| `ldrb` at 0xd9900 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xd9904 | store | 4 | `(unknown)` | `[sp-28]:1` |
| `ldrb` at 0xd9908 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xd990c | store | 4 | `(unknown)` | `[sp-25]:1` |
| `ldr` at 0xd9910 | load | 4 | `abs 0xd9a98` | `-` |
| `ldr` at 0xd9918 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xd9924 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9928 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd992c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9940 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9944 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9948 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd994c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xd9950 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9954 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd9958 | load | 1 | `sp -0x1b` | `-` |
| `ldrb` at 0xd9964 | load | 1 | `sp -0x19` | `-` |
| `str` at 0xd9984 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldr` at 0xd9988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9994 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9998 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xd99a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd99a4 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd99a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd99bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd99c0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd99c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd99c8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd99d4 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0xd99ec | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldr` at 0xd99f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd99fc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9a00 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xd9a08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9a0c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd9a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9a24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9a28 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0xd9a2c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9a30 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xd9a34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9a38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9a3c | load | 1 | `[sp-24] +0x5` | `-` |
| `ldrb` at 0xd9a40 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0xd9a48 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xd9a64 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldr` at 0xd9a68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9a74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9a78 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `str` at 0xd9a84 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd9a88 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd9a8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9a8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9a8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9760 | `(unknown callee) 0xc807c` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0xd9794 | `(unknown callee) 0xc8998` | `?` | `[[sp-24]+2]:1` | `?` | `sp-27` |
| `bl` at 0xd989c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd98a8 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd98d8 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x13571c` | `0x135724` |
| `bl` at 0xd98e8 | `(unknown callee) 0x1b598` | `[sp-36]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd9920 | `(unknown callee) 0x2079c` | `0x1357fc` | `ret(0x1b598)` | `[[sp-24]+12]` | `[sp-26]:1` |
| `bl` at 0xd9934 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd997c | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |
| `bl` at 0xd99b0 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd99e4 | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `?` | `[sp-28]:1` |
| `bl` at 0xd9a18 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9a5c | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `[[sp-24]+3]:1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd97a8 | `ret(0xc8998) == 0x0` | `ret(0xc8998) != 0x0` |
| `beq` at 0xd9800 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd983c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9854 | `[sp-26]:1 == 0x0` | `[sp-26]:1 != 0x0` |
| `beq` at 0xd9990 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `beq` at 0xd99f8 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `beq` at 0xd9a70 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9a8c | `r0` | `?` |

## `vsc055_GetCfg`

- `vsc055_GetCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9a9c to 0xd9b08.
  Established. A zero entry `r1` takes the branch past an optional four-byte store (`beq` at
  0xd9ae0). The fall-through writes `[[sp-28]]` at `r1 +0x0` (`str` at 0xd9af0). A later four-byte
  load at `[sp-28] +0x4` supplies return expression `[[sp-28]+4]` (`ldr` at 0xd9af8, `ldmdb` at
  0xd9b00). Configuration and count units are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9aac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9ab0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xd9ab4 | load | 4 | `abs 0xd9b04` | `-` |
| `ldr` at 0xd9ab8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9acc | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd9ad4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xd9ad8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9ae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9ae8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9aec | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xd9af0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xd9af4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9af8 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldmdb` at 0xd9b00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9b00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9b00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9ae0 | `r1 == 0x0` | `r1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9b00 | `r0` | `[[sp-28]+4]` |

## `vsc055_Count`

- `vsc055_Count`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9b08 to 0xd9b54.
  Established. The absolute four-byte load reads `0x14381c` (`ldr` at 0xd9b34). A second four-byte
  read targets `[sp-24] +0x4`, and the return retains `[[sp-24]+4]` (`ldr` at 0xd9b44, `ldmdb` at
  0xd9b4c). The counted object is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9b18 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd9b1c | load | 4 | `abs 0xd9b50` | `-` |
| `ldr` at 0xd9b20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9b34 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd9b3c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd9b40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd9b44 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldmdb` at 0xd9b4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9b4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9b4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9b4c | `r0` | `[[sp-24]+4]` |

## `vsc055_Present`

- `vsc055_Present`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9b54 to 0xd9bd8.
  Established. The branch takes `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` to `0xd9bb4` (`blt` at
  0xd9ba4); its fall-through assigns four-byte zero at `sp -0x20` (`str` at 0xd9bac). The taken arm
  reads byte `((r1 & 0xff) + [[sp-28]+8]) +0x0` and stores the loaded expression as four bytes there
  (`ldrb` at 0xd9bc4, `str` at 0xd9bc8). The merged return is unknown (`ldmdb` at 0xd9bd0). Presence
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9b64 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd9b6c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xd9b70 | load | 4 | `abs 0xd9bd4` | `-` |
| `ldr` at 0xd9b74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9b88 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd9b90 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xd9b94 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9b98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9b9c | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xd9bac | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xd9bb4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9bb8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd9bbc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xd9bc4 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xd9bc8 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xd9bcc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xd9bd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9bd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9bd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd9ba4 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9bd0 | `r0` | `?` |

## `vsc055_IntSet`

- `vsc055_IntSet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9bd8 to 0xd9cf0.
  Established. The byte load targets `r3 +0x143824` (`ldrb` at 0xd9c08). A later byte store
  at `sp -0x22` has unknown value (`strb` at 0xd9c60). The unnamed call to `0xc8874` has candidates
  unknown, `[[sp-24]+2]:1`, `[sp-34]:1`, `[sp-35]:1` (`bl` at 0xd9c9c); its zero result takes the
  branch to `0xd9cc0` (`beq` at 0xd9cb0). Interrupt register identities are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9be8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9bec | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd9bf0 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xd9bf8 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0xd9bfc | load | 4 | `abs 0xd9cec` | `-` |
| `ldr` at 0xd9c00 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9c08 | load | 1 | `r3 +0x143824` | `-` |
| `strb` at 0xd9c0c | store | 1 | `sp -0x23` | `([r3+1325092]:1 & 0xff)` |
| `strb` at 0xd9c14 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd9c18 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xd9c28 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd9c38 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9c3c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xd9c48 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd9c60 | store | 1 | `sp -0x22` | `?` |
| `ldr` at 0xd9c64 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9c68 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd9c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9c80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9c84 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xd9c88 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0xd9c8c | load | 1 | `sp -0x23` | `-` |
| `str` at 0xd9ca4 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xd9ca8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd9cb4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd9cb8 | store | 4 | `sp -0x2c` | `ret(0xc8874)` |
| `ldrb` at 0xd9cc0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd9cc8 | store | 1 | `sp -0x19` | `?` |
| `ldrb` at 0xd9ccc | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd9cd4 | store | 1 | `sp -0x21` | `?` |
| `str` at 0xd9ce0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd9ce4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xd9ce8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9ce8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9ce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9c74 | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9c9c | `(unknown callee) 0xc8874` | `?` | `[[sp-24]+2]:1` | `[sp-34]:1` | `[sp-35]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd9c20 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd9c34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9cb0 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9ce8 | `r0` | `?` |

## `vsc055_IntGet`

- `vsc055_IntGet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9cf0 to 0xd9e3c.
  Established. A byte-zero assignment targets `r2 +0x0` (`strb` at 0xd9d14). The unnamed call to
  `0xc8998` uses unknown, `[[sp-24]+2]:1`, `[sp-30]:1`, `sp-31` (`bl` at 0xd9d94); its zero result
  takes the branch to `0xd9db8` (`beq` at 0xd9da8). Three equality branches take `[sp-44] == 0xa`,
  `== 0x6` or `== 0xe` to `0xd9dfc` (`beq` at 0xd9dcc, `beq` at 0xd9de4, `beq` at 0xd9df4). The
  later byte store at `[sp-28] +0x0` has unknown value (`strb` at 0xd9e18). Interrupt-status
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9d00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9d04 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd9d08 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xd9d0c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd9d14 | store | 1 | `r2 +0x0` | `0x0` |
| `strb` at 0xd9d1c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd9d20 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd9d30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9d34 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldrb` at 0xd9d40 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd9d58 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0xd9d5c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9d60 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0xd9d64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9d78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd9d7c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0xd9d80 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0xd9d9c | store | 4 | `sp -0x24` | `ret(0xc8998)` |
| `ldr` at 0xd9da0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd9dac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd9db0 | store | 4 | `sp -0x28` | `ret(0xc8998)` |
| `ldrb` at 0xd9db8 | load | 1 | `sp -0x1f` | `-` |
| `str` at 0xd9dc0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd9dc4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9dd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9ddc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9dec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd9dfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9e00 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd9e04 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xd9e10 | load | 1 | `[sp-28] +0x0` | `-` |
| `strb` at 0xd9e18 | store | 1 | `[sp-28] +0x0` | `?` |
| `ldrb` at 0xd9e1c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd9e24 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xd9e30 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xd9e34 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xd9e38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9e38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9e38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9d6c | `(unknown callee) 0xc807c` | `[sp-20]` | `[[sp-24]+1]:1` | `?` | `[[sp-24]+1]:1` |
| `bl` at 0xd9d94 | `(unknown callee) 0xc8998` | `?` | `[[sp-24]+2]:1` | `[sp-30]:1` | `sp-31` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd9d28 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xd9da8 | `ret(0xc8998) == 0x0` | `ret(0xc8998) != 0x0` |
| `beq` at 0xd9dcc | `[sp-44] == 0xa` | `[sp-44] != 0xa` |
| `bgt` at 0xd9dd8 | `[sp-44] > 0xa (signed, 32-bit)` | `[sp-44] <= 0xa (signed, 32-bit)` |
| `beq` at 0xd9de4 | `[sp-44] == 0x6` | `[sp-44] != 0x6` |
| `beq` at 0xd9df4 | `[sp-44] == 0xe` | `[sp-44] != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9e38 | `r0` | `?` |

## `vsc055_Int`

- `vsc055_Int`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9e3c to 0xd9f14.
  Established. The unnamed call to `0xd9f14` has four unknown candidates (`bl` at 0xd9ee0); its
  return participates in the four-byte store `([sp-36] + ret(0xd9f14))` at `sp -0x24` (`str` at
  0xd9ef0). The byte load at an unresolved effective address precedes a branch taken when `Z == 1`
  to `0xd9ef4` (`ldrb` at 0xd9eb8, `beq` at 0xd9ec0). The aggregate interrupt meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9e4c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd9e54 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xd9e5c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xd9e60 | load | 4 | `abs 0xd9f10` | `-` |
| `ldr` at 0xd9e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9e78 | load | 4 | `abs 0x14381c` | `-` |
| `str` at 0xd9e80 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xd9e88 | store | 4 | `sp -0x24` | `0x0` |
| `ldrb` at 0xd9e8c | load | 1 | `sp -0x15` | `-` |
| `str` at 0xd9e90 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldrb` at 0xd9e94 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xd9e98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9ea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9eac | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xd9eb0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd9eb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xd9ec4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9ec8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd9ed0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xd9ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9ee8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd9ef0 | store | 4 | `sp -0x24` | `([sp-36] + ret(0xd9f14))` |
| `ldr` at 0xd9ef4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9efc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xd9f04 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xd9f0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd9f0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd9f0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9ee0 | `(unknown callee) 0xd9f14` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xd9ea0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xd9ec0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd9f0c | `r0` | `?` |

## `vsc055_FanPWMInit`

- `vsc055_FanPWMInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda618 to 0xda6d0.
  Established. The unnamed call to `0xd7210` has candidates `r0`, `sp-32`, `r2`, `sp-32` (`bl` at
  0xda654). A later call to `0xc8874` carries unknown, `[sp-22]:1`, unknown, `[sp-41]:1` (`bl` at
  0xda6b8); its return is retained as `ret(0xc8874)` (`ldmdb` at 0xda6cc). A byte assignment
  at `sp -0x29` has unknown value (`strb` at 0xda670). PWM register and duty-cycle semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda628 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda62c | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xda634 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xda63c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xda644 | store | 1 | `sp -0x29` | `0x0` |
| `ldr` at 0xda64c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda65c | store | 4 | `sp -0x24` | `ret(0xd7210)` |
| `ldr` at 0xda660 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xda668 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xda670 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xda674 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xda678 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda68c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xda698 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xda6a4 | load | 1 | `sp -0x29` | `-` |
| `str` at 0xda6c0 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xda6c4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xda6cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda6cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda6cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda654 | `(unknown callee) 0xd7210` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xda680 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-21]:1` | `[sp-41]:1` | `[sp-21]:1` |
| `bl` at 0xda6b8 | `(unknown callee) 0xc8874` | `?` | `[sp-22]:1` | `?` | `[sp-41]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda6cc | `r0` | `ret(0xc8874)` |

## `vsc055_FanMonitorInit`

- `vsc055_FanMonitorInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda6d0 to 0xda7e0.
  Established. The first call to unnamed `0xc8874` has candidates unknown, `[sp-22]:1`, `[sp-41]:1`,
  `0xff` (`bl` at 0xda758). The second call to that target carries unknown, `[sp-22]:1`,
  `[sp-41]:1`, `[sp-42]:1` (`bl` at 0xda7c8). Byte assignments at `sp -0x29` have unknown values
  (`strb` at 0xda724, `strb` at 0xda790). The exit retains `ret(0xc8874)` (`ldmdb` at 0xda7dc).
  Fan-monitor register meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda6e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda6e4 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xda6ec | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xda6f4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xda6fc | store | 1 | `sp -0x2a` | `0x0` |
| `ldr` at 0xda704 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda714 | store | 4 | `sp -0x24` | `ret(0xd7210)` |
| `ldr` at 0xda718 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xda724 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xda728 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xda72c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda740 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xda744 | load | 1 | `sp -0x29` | `-` |
| `str` at 0xda760 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldrb` at 0xda764 | load | 1 | `sp -0x2a` | `-` |
| `strb` at 0xda770 | store | 1 | `sp -0x2a` | `?` |
| `ldrb` at 0xda774 | load | 1 | `sp -0x2a` | `-` |
| `ldrb` at 0xda778 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xda780 | store | 1 | `sp -0x2a` | `?` |
| `ldr` at 0xda784 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xda790 | store | 1 | `sp -0x29` | `?` |
| `ldrb` at 0xda794 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xda798 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda7ac | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0xda7b0 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xda7b4 | load | 1 | `sp -0x2a` | `-` |
| `str` at 0xda7d0 | store | 4 | `sp -0x28` | `ret(0xc8874)` |
| `ldr` at 0xda7d4 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xda7dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xda7dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xda7dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda70c | `(unknown callee) 0xd7210` | `r0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0xda734 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xda758 | `(unknown callee) 0xc8874` | `?` | `[sp-22]:1` | `[sp-41]:1` | `0xff` |
| `bl` at 0xda7a0 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-21]:1` | `[sp-42]:1` | `[sp-21]:1` |
| `bl` at 0xda7c8 | `(unknown callee) 0xc8874` | `?` | `[sp-22]:1` | `[sp-41]:1` | `[sp-42]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xda7dc | `r0` | `ret(0xc8874)` |
