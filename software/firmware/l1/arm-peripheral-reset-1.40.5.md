# ARM peripheral reset callers in 1.40.5

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
ioExpReset and ioExpSoftResetInt have unresolved computed transfers. Their tables and returns cover known edges only. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/peripheral-reset-facts.txt) retain all tool sections.

## `ioExpReset`

- `ioExpReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd69e0 to 0xd6bb0.
  Established. A byte read through `ret(0xd8068) +0x0` is zero when the branch takes its edge to
  `0xd6bac` (`ldrb` at 0xd6a30, `beq` at 0xd6a38). The computed transfer takes an unresolved edge
  when `[[sp-28]]:1 <= 0x4`, unsigned 32-bit (`ldrls` at 0xd6a64). Known later byte assignments
  include `0x5` at `[sp-32] +0x5` and `0x0` at `[sp-32] +0x3` (`strb` at 0xd6b68, `strb` at
  0xd6b74). The unnamed `0xd8800` call carries `[sp-20]`, `[sp-21]:1`, `[[sp-28]+7]:1`, `0x1` (`bl`
  at 0xd6ba0). Selector-specific reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd69f0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xd69f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xd69fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6a00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6a10 | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd6a14 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6a18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6a28 | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd6a2c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6a30 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd6a3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6a40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6a44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6a48 | load | 1 | `[sp-28] +0x6` | `-` |
| `ldrb` at 0xd6a4c | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd6a54 | store | 1 | `ret(0xd8068) +0x1` | `?` |
| `ldr` at 0xd6a58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6a5c | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xd6a64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd6b24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd6b30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6b38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6b3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6b4c | store | 4 | `sp -0x24` | `ret(0xd85f4)` |
| `ldr` at 0xd6b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6b54 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6b58 | load | 1 | `[sp-32] +0x3` | `-` |
| `strb` at 0xd6b5c | store | 1 | `[sp-32] +0x4` | `([[sp-32]+3]:1 & 0xff)` |
| `ldr` at 0xd6b60 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xd6b68 | store | 1 | `[sp-32] +0x5` | `0x5` |
| `ldr` at 0xd6b6c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xd6b74 | store | 1 | `[sp-32] +0x3` | `0x0` |
| `ldr` at 0xd6b78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xd6b84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xd6b88 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6b8c | load | 1 | `[sp-28] +0x7` | `-` |
| `ldr` at 0xd6b90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6ba8 | store | 4 | `sp -0x24` | `ret(0xd8800)` |
| `ldmdb` at 0xd6bac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6bac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6bac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6a08 | `(unknown callee) 0xd7f54` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xd6a20 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xd6b44 | `(unknown callee) 0xd85f4` | `[sp-20]` | `[sp-28]` | `[sp-32]+2` | `[sp-32]+2` |
| `bl` at 0xd6ba0 | `(unknown callee) 0xd8800` | `[sp-20]` | `[sp-21]:1` | `[[sp-28]+7]:1` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd6a38 | `[ret(0xd8068)]:1 == 0x0` | `[ret(0xd8068)]:1 != 0x0` |
| `ldrls` at 0xd6a64 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |
| `bne` at 0xd6b2c | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xd6b80 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6bac | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd6a64).

## `ioExpHardReset`

- `ioExpHardReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6cd4 to 0xd6d5c.
  Established. Four bytes are read from absolute `0x2adf98` (`ldr` at 0xd6d00). The unnamed
  `0xd926c` call has `r0`, `0x2adf98`, unknown, unknown (`bl` at 0xd6d10). A byte counter starts at
  zero at `sp -0x19`, then uses `([sp-25]:1+1 & 0xff)` in its update (`strb` at 0xd6d18, `strb` at
  0xd6d4c). The signed-less loop predicate has unknown operands (`blt` at 0xd6d2c). Hard-reset scope
  is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6ce4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd6ce8 | load | 4 | `abs 0xd6d58` | `-` |
| `ldr` at 0xd6cec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6d00 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd6d08 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd6d0c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0xd6d18 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xd6d1c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd6d20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6d24 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xd6d34 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xd6d38 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd6d44 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xd6d4c | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0xd6d54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6d54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6d54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6d10 | `(unknown callee) 0xd926c` | `r0` | `0x2adf98` | `?` | `?` |
| `bl` at 0xd6d40 | `(unknown callee) 0xd69e0` | `[sp-20]` | `?` | `[[sp-24]+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd6d2c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6d54 | `r0` | `?` |

## `ioExpSoftResetData`

- `ioExpSoftResetData`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6d5c to 0xd6f5c.
  Established. The unnamed `0xc80f4` call has `r0`, `0x2adf98`, unknown, unknown (`bl` at 0xd6d94);
  its zero result takes the branch to `0xd6de4` (`beq` at 0xd6da8). Later, the byte loaded through
  `ret(0xd8068) +0x0` is tested for zero by the branch to `0xd6f20` (`ldrb` at 0xd6e38, `beq` at
  0xd6e40). The unnamed `0xd85f4` call carries `[sp-24]`, `[sp-28]`, `ret(0xd8068)+2`,
  `ret(0xd8068)+2` (`bl` at 0xd6e58). Data-reset field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6d6c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xd6d70 | load | 4 | `abs 0xd6f40` | `-` |
| `ldr` at 0xd6d74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6d88 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd6d90 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd6d9c | store | 4 | `sp -0x28` | `ret(0xc80f4)` |
| `ldr` at 0xd6da0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6dac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6db8 | load | 4 | `abs 0xd6f44` | `-` |
| `str` at 0xd6dbc | store | 4 | `sp -0x3c` | `0x135388` |
| `str` at 0xd6dc0 | store | 4 | `sp -0x38` | `ret(0x21704)` |
| `ldr` at 0xd6dc8 | load | 4 | `abs 0xd6f48` | `-` |
| `ldr` at 0xd6dcc | load | 4 | `abs 0xd6f4c` | `-` |
| `str` at 0xd6ddc | store | 4 | `sp -0x34` | `?` |
| `strb` at 0xd6de8 | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0xd6dec | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6df0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd6df4 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0xd6e04 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6e08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd6e18 | store | 4 | `sp -0x1c` | `ret(0xd7f54)` |
| `ldrb` at 0xd6e1c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xd6e20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd6e30 | store | 4 | `sp -0x20` | `ret(0xd8068)` |
| `ldr` at 0xd6e34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xd6e38 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd6e44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6e4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6e50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd6e60 | store | 4 | `sp -0x28` | `ret(0xd85f4)` |
| `ldr` at 0xd6e64 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6e70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6e78 | load | 4 | `abs 0xd6f50` | `-` |
| `ldr` at 0xd6e7c | load | 4 | `abs 0xd6f54` | `-` |
| `str` at 0xd6ea8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd6eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd6eb4 | load | 4 | `abs 0xd6f50` | `-` |
| `ldr` at 0xd6eb8 | load | 4 | `abs 0xd6f54` | `-` |
| `ldr` at 0xd6ec4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd6ed4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6ee0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd6ee4 | load | 4 | `abs 0xd6f58` | `-` |
| `ldr` at 0xd6eec | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xd6efc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd6f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6f08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd6f0c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6f10 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldrb` at 0xd6f14 | load | 1 | `[sp-28] +0x4` | `-` |
| `strb` at 0xd6f1c | store | 1 | `[sp-32] +0x1` | `?` |
| `ldrb` at 0xd6f20 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xd6f28 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `str` at 0xd6f34 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd6f38 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xd6f3c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xd6f3c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd6f3c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd6f3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6d94 | `(unknown callee) 0xc80f4` | `r0` | `0x2adf98` | `?` | `?` |
| `bl` at 0xd6db0 | `(unknown callee) 0x21704` | `ret(0xc80f4)` | `?` | `?` | `ret(0xc80f4)` |
| `bl` at 0xd6dd4 | `(unknown callee) 0x21eb8` | `0x1` | `0x13525c` | `0x13d` | `0x0` |
| `bl` at 0xd6e10 | `(unknown callee) 0xd7f54` | `[sp-24]` | `?` | `[[sp-44]+4]` | `?` |
| `bl` at 0xd6e28 | `(unknown callee) 0xd8068` | `[sp-24]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0xd6e58 | `(unknown callee) 0xd85f4` | `[sp-24]` | `[sp-28]` | `ret(0xd8068)+2` | `ret(0xd8068)+2` |
| `bl` at 0xd6e80 | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6e8c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd6ebc | `(unknown callee) 0x4f9a4` | `[sp-24]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd6ecc | `(unknown callee) 0x1b598` | `[sp-48]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd6ed8 | `(unknown callee) 0x21704` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xd6ef4 | `(unknown callee) 0x2079c` | `0x1353b4` | `ret(0x1b598)` | `[[sp-28]+12]` | `ret(0x21704)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd6da8 | `ret(0xc80f4) == 0x0` | `ret(0xc80f4) != 0x0` |
| `blt` at 0xd6dfc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd6e40 | `[ret(0xd8068)]:1 == 0x0` | `[ret(0xd8068)]:1 != 0x0` |
| `beq` at 0xd6e6c | `ret(0xd85f4) == 0x0` | `ret(0xd85f4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd6f3c | `r0` | `?` |

## `ioExpSoftResetInt`

- `ioExpSoftResetInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd6f5c to 0xd7210.
  Established. The computed transfer has an unresolved target when `[[sp-24]]:1-1 <= 0x3`, unsigned
  32-bit (`ldrls` at 0xd7004). A later indirect call has unknown target and candidates `[sp-20]`,
  `[sp-24]`, `[sp-28]+3`, `[sp-28]+3` (`mov` at 0xd70d4); its zero result takes the branch to
  `0xd70f0` (`beq` at 0xd70e0). A known later unnamed `0xd768c` call carries `[sp-20]`, `[sp-29]:1`,
  `0xff`, `[sp-29]:1` (`bl` at 0xd71cc). Interrupt-reset effects and selector-specific callbacks
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd6f6c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd6f70 | load | 4 | `abs 0xd71f0` | `-` |
| `ldr` at 0xd6f74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd6f88 | load | 4 | `abs 0x2adf98` | `-` |
| `str` at 0xd6f90 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xd6f98 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0xd6f9c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd6fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd6fa4 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0xd6fb4 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd6fb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6fc8 | store | 4 | `sp -0x18` | `ret(0xd7f54)` |
| `ldrb` at 0xd6fcc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd6fd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd6fe0 | store | 4 | `sp -0x1c` | `ret(0xd8068)` |
| `ldr` at 0xd6fe4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd6fe8 | load | 1 | `ret(0xd8068) +0x0` | `-` |
| `ldr` at 0xd6ff4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd6ff8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xd7004 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd707c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd7080 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd7084 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd7088 | load | 1 | `ret(0xd8068) +0x3` | `-` |
| `strb` at 0xd708c | store | 1 | `ret(0xd8068) +0x4` | `([ret(0xd8068)+3]:1 & 0xff)` |
| `ldr` at 0xd7090 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xd7098 | store | 1 | `[sp-28] +0x5` | `0x5` |
| `ldr` at 0xd709c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd70a0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd70a4 | load | 1 | `[sp-24] +0x7` | `-` |
| `strb` at 0xd70a8 | store | 1 | `[sp-28] +0x3` | `([[sp-24]+7]:1 & 0xff)` |
| `ldr` at 0xd70ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd70b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd70c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd70c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd70c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd70e8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd70f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd70f4 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd70f8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xd70fc | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd7110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7118 | load | 4 | `abs 0xd7204` | `-` |
| `ldr` at 0xd711c | load | 4 | `abs 0xd7208` | `-` |
| `str` at 0xd7148 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd714c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd7154 | load | 4 | `abs 0xd7204` | `-` |
| `ldr` at 0xd7158 | load | 4 | `abs 0xd7208` | `-` |
| `ldr` at 0xd7164 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd7174 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7178 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd717c | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd7180 | load | 4 | `abs 0xd720c` | `-` |
| `ldr` at 0xd7188 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xd7194 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd719c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xd71a0 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldr` at 0xd71ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71b0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xd71bc | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xd71c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xd71d0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xd71d8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `str` at 0xd71e4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd71e8 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xd71ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xd71ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xd71ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd6fc0 | `(unknown callee) 0xd7f54` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0xd6fd8 | `(unknown callee) 0xd8068` | `[sp-20]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `mov` at 0xd70d4 | `(unknown callee) (unknown target)` | `[sp-20]` | `[sp-24]` | `[sp-28]+3` | `[sp-28]+3` |
| `bl` at 0xd7120 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd712c | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd715c | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x1352b0` | `0x1352b8` |
| `bl` at 0xd716c | `(unknown callee) 0x1b598` | `[sp-44]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0xd718c | `(unknown callee) 0x2079c` | `0x1353dc` | `ret(0x1b598)` | `[[sp-24]+12]` | `[[sp-28]+3]:1` |
| `bl` at 0xd71cc | `(unknown callee) 0xd768c` | `[sp-20]` | `[sp-29]:1` | `0xff` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xd6fac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xd6ff0 | `[ret(0xd8068)]:1 == 0x0` | `[ret(0xd8068)]:1 != 0x0` |
| `ldrls` at 0xd7004 | `[[sp-24]]:1-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-24]]:1-1 > 0x3 (unsigned, 32-bit)` |
| `beq` at 0xd70b4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xd70e0 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0xd710c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd71a8 | `[[sp-28]+3]:1 == 0x0` | `[[sp-28]+3]:1 != 0x0` |
| `bne` at 0xd71b8 | `[[sp-24]+8] != 0x0` | `[[sp-24]+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xd71ec | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd7004).

## `mic74_Reset`

- `mic74_Reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xda808 to 0xdaad0.
  Established. Included calls to unnamed `0xc8874` use unknown, `[sp-26]:1`, `0x4`, `0x0` (`bl` at 0xda864).
  Other calls to that target carry unknown, `[sp-26]:1`, `0x2`, `[[sp-24]+5]:1`, and unknown,
  `[sp-26]:1`, `0x5`, `[[sp-24]+6]:1` (`bl` at 0xda92c, `bl` at 0xda984). The last unnamed `0xc8998` call uses unknown,
  `[sp-26]:1`, `0x3`, `sp-27` (`bl` at 0xdaa98); its zero-result branch reaches `0xdaac4` (`beq` at
  0xdaaac). Device-register semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xda818 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xda81c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xda820 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda824 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xda828 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xda82c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda830 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xda834 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xda838 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda83c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda850 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xda86c | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xda870 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda874 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xda888 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0xda88c | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xda890 | store | 4 | `sp -0x2c` | `[sp-26]:1` |
| `str` at 0xda898 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xda89c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xda8a0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xda8ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda8b0 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xda8c0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xda8c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xda8c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xda8d0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xda8dc | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xda8e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda8ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda8f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda8fc | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xda900 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda904 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda918 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda91c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda920 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xda934 | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xda938 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda944 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda950 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda954 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xda958 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda95c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda970 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda974 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda978 | load | 1 | `[sp-24] +0x6` | `-` |
| `str` at 0xda98c | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xda990 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda99c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xda9a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xda9ac | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xda9b0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xda9b4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xda9c8 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xda9cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xda9d0 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xda9ec | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xda9f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xda9fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdaa08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdaa0c | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xdaa10 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdaa14 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdaa28 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xdaa44 | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xdaa48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdaa54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdaa60 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdaa64 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xdaa68 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdaa6c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdaa80 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0xdaaa0 | store | 4 | `sp -0x24` | `ret(0xc8998)` |
| `ldr` at 0xdaaa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdaab0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdaabc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdaac0 | store | 4 | `sp -0x20` | `ret(0xc8998)` |
| `ldr` at 0xdaac4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdaacc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaacc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaacc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xda844 | `(unknown callee) 0xc807c` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xda864 | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x4` | `0x0` |
| `bl` at 0xda87c | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda8d4 | `(unknown callee) 0xc8874` | `[sp-40]` | `[sp-26]:1` | `0x0` | `?` |
| `bl` at 0xda90c | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda92c | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xda964 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda984 | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x5` | `[[sp-24]+6]:1` |
| `bl` at 0xda9bc | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xda9e4 | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x1` | `?` |
| `bl` at 0xdaa1c | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xdaa3c | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x6` | `0x0` |
| `bl` at 0xdaa74 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xdaa98 | `(unknown callee) 0xc8998` | `?` | `[sp-26]:1` | `0x3` | `sp-27` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xda8a8 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0xda8b8 | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |
| `beq` at 0xda8e8 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xda8f4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda940 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xda94c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda998 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xda9a4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xda9f8 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xdaa04 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xdaa50 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xdaa5c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0xdaaac | `ret(0xc8998) == 0x0` | `ret(0xc8998) != 0x0` |
| `bne` at 0xdaab8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaacc | `r0` | `[sp-32]` |

## `mic74_PreReset`

- `mic74_PreReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaad0 to 0xdaba4.
  Established. Byte reads include `r1 +0x1` and `r1 +0x2` (`ldrb` at 0xdaaec, `ldrb` at 0xdaaf8).
  The two unnamed `0xc8874` calls use unknown, `[sp-26]:1`, `0x2`, `[[sp-24]+5]:1`, and unknown,
  `[sp-26]:1`, `0x1`, unknown (`bl` at 0xdab2c, `bl` at 0xdab6c). The second zero result takes the
  branch to `0xdab98` (`beq` at 0xdab80). The exit carries `[sp-32]` (`ldmdb` at 0xdaba0). Pre-reset
  signaling is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdaae0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdaae4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdaae8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaaec | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0xdaaf0 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0xdaaf4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaaf8 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0xdaafc | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0xdab00 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdab04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdab18 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xdab1c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdab20 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0xdab34 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldrb` at 0xdab38 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xdab3c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xdab50 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xdab54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdab58 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0xdab74 | store | 4 | `sp -0x24` | `ret(0xc8874)` |
| `ldr` at 0xdab78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdab84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdab90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdab94 | store | 4 | `sp -0x20` | `ret(0xc8874)` |
| `ldr` at 0xdab98 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdaba0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaba0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdab0c | `(unknown callee) 0xc807c` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0xdab2c | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0xdab44 | `(unknown callee) 0xc807c` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0xdab6c | `(unknown callee) 0xc8874` | `?` | `[sp-26]:1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdab80 | `ret(0xc8874) == 0x0` | `ret(0xc8874) != 0x0` |
| `bne` at 0xdab8c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaba0 | `r0` | `[sp-32]` |

## `vrmReset`

- `vrmReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cf94 to 0x5cfd4.
  Established. The unnamed `0x3dd74` call has entry-register candidates `r0`, `r1`, `r2`, `r3`
  (`bl` at 0x5cfac); its result unequal to `0x1` takes the branch to `0x5cfc8` (`bne` at 0x5cfb8).
  The two arms call unnamed `0x5d034` and `0x5d1e8` with `[sp-20]`, unknown, unknown, `ret(0x3dd74)`
  (`bl` at 0x5cfc0, `bl` at 0x5cfcc). Regulator reset behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cfa4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5cfa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cfc8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x5cfd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cfd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cfd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5cfac | `(unknown callee) 0x3dd74` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x5cfc0 | `(unknown callee) 0x5d034` | `[sp-20]` | `?` | `?` | `ret(0x3dd74)` |
| `bl` at 0x5cfcc | `(unknown callee) 0x5d1e8` | `[sp-20]` | `?` | `?` | `ret(0x3dd74)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5cfb8 | `ret(0x3dd74) != 0x1` | `ret(0x3dd74) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cfd0 | `r0` | `?` |

## `psicReset`

- `psicReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc258 to 0xdc358.
  Established. The first unnamed `0xdc86c` call uses `[[sp-32]]`, `[[sp-24]+3]:1`, `0xa`,
  `[[sp-24]+6]:1` (`bl` at 0xdc2e0); its zero result takes the branch to `0xdc300` (`beq` at
  0xdc2f4). The following call to that target carries `[[sp-32]]`, `[[sp-24]+3]:1`, `0xc`, `0x0`
  (`bl` at 0xdc31c). The exit retains `[sp-40]` (`ldmdb` at 0xdc350). Controller register and
  command meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc268 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc26c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdc270 | load | 4 | `abs 0xdc354` | `-` |
| `ldr` at 0xdc274 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc288 | load | 4 | `abs 0x143828` | `-` |
| `str` at 0xdc290 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdc294 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc298 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc29c | load | 1 | `r1 +0x2` | `-` |
| `ldr` at 0xdc2a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xdc2ac | store | 4 | `sp -0x20` | `?` |
| `str` at 0xdc2b4 | store | 4 | `sp -0x28` | `0x0` |
| `strb` at 0xdc2bc | store | 1 | `sp -0x29` | `?` |
| `ldr` at 0xdc2c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc2c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc2c8 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xdc2cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc2d0 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xdc2d4 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xdc2e8 | store | 4 | `sp -0x24` | `ret(0xdc86c)` |
| `ldr` at 0xdc2ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc2f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc2fc | store | 4 | `sp -0x28` | `ret(0xdc86c)` |
| `ldr` at 0xdc300 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc304 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc308 | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0xdc30c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xdc324 | store | 4 | `sp -0x24` | `ret(0xdc86c)` |
| `ldr` at 0xdc328 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc334 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc340 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc344 | store | 4 | `sp -0x28` | `ret(0xdc86c)` |
| `ldr` at 0xdc348 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xdc350 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc350 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc350 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc2e0 | `(unknown callee) 0xdc86c` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xa` | `[[sp-24]+6]:1` |
| `bl` at 0xdc31c | `(unknown callee) 0xdc86c` | `[[sp-32]]` | `[[sp-24]+3]:1` | `0xc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdc2f4 | `ret(0xdc86c) == 0x0` | `ret(0xdc86c) != 0x0` |
| `beq` at 0xdc330 | `ret(0xdc86c) == 0x0` | `ret(0xdc86c) != 0x0` |
| `bne` at 0xdc33c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc350 | `r0` | `[sp-40]` |

## `psicResetAll`

- `psicResetAll`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc358 to 0xdc424.
  Established. An unnamed `0xd7210` call carries `r0`, `sp-24`, `r2`, `sp-24` (`bl` at 0xdc380). The
  loop predicate has unknown signed-less operands (`blt` at 0xdc3a0). A later unnamed `0xdc258` call
  uses `[sp-20]` followed by three unknown candidates (`bl` at 0xdc3dc); its zero result takes the
  branch to `0xdc408` (`beq` at 0xdc3f0). A later four-byte assignment at `sp -0x24` retains
  `ret(0xdc258)` (`str` at 0xdc404). Which controllers are reset remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc368 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdc370 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdc378 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc388 | store | 4 | `sp -0x20` | `ret(0xd7210)` |
| `str` at 0xdc390 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xdc394 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc398 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc3a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc3b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdc3b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xdc3c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc3cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc3d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdc3e4 | store | 4 | `sp -0x28` | `ret(0xdc258)` |
| `ldr` at 0xdc3e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc3f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc400 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdc404 | store | 4 | `sp -0x24` | `ret(0xdc258)` |
| `ldr` at 0xdc408 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc410 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdc418 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xdc420 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdc420 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdc420 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc380 | `(unknown callee) 0xd7210` | `r0` | `sp-24` | `r2` | `sp-24` |
| `bl` at 0xdc3dc | `(unknown callee) 0xdc258` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xdc3a0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xdc3c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xdc3f0 | `ret(0xdc258) == 0x0` | `ret(0xdc258) != 0x0` |
| `bne` at 0xdc3fc | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdc420 | `r0` | `[sp-36]` |

## `imageSyncResetHold`

- `imageSyncResetHold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb76c to 0xdb7e4.
  Established. Nonzero `ret(0xdba10)` takes the branch to `0xdb7b0` (`bl` at 0xdb78c, `bne` at 0xdb7a0). The zero
  fall-through assigns four-byte `0xfffffc10` at `sp -0x1c` (`str` at 0xdb7a8). The other arm calls
  unnamed `0xd7368` with `0x0`, `[ret(0xdba10)+2]:1`, `[ret(0xdba10)+3]:1`, `[ret(0xdba10)+3]:1`
  (`bl` at 0xdb7cc). Reset-hold electrical effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdb780 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb784 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb794 | store | 4 | `sp -0x18` | `ret(0xdba10)` |
| `ldr` at 0xdb798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb7a4 | load | 4 | `abs 0xdb7e0` | `-` |
| `str` at 0xdb7a8 | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xdb7b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb7b4 | load | 1 | `ret(0xdba10) +0x2` | `-` |
| `ldr` at 0xdb7b8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb7bc | load | 1 | `ret(0xdba10) +0x3` | `-` |
| `str` at 0xdb7d4 | store | 4 | `sp -0x1c` | `ret(0xd7368)` |
| `ldr` at 0xdb7d8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdb7dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb7dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb7dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb78c | `(unknown callee) 0xdba10` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdb7cc | `(unknown callee) 0xd7368` | `0x0` | `[ret(0xdba10)+2]:1` | `[ret(0xdba10)+3]:1` | `[ret(0xdba10)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb7a0 | `ret(0xdba10) != 0x0` | `ret(0xdba10) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb7dc | `r0` | `?` |

## `imageSyncResetRelease`

- `imageSyncResetRelease`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb7e4 to 0xdb85c.
  Established. A byte read at `ret(0xdba10) +0x2` precedes another at `ret(0xdba10) +0x3` (`ldrb` at
  0xdb82c, `ldrb` at 0xdb834). The unnamed `0xd731c` call uses `0x0`, `[ret(0xdba10)+2]:1`,
  `[ret(0xdba10)+3]:1`, `[ret(0xdba10)+3]:1` (`bl` at 0xdb844); its result is assigned as four bytes
  at `sp -0x1c` (`str` at 0xdb84c). Reset-release signaling remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdb7f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb7fc | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb80c | store | 4 | `sp -0x18` | `ret(0xdba10)` |
| `ldr` at 0xdb810 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb81c | load | 4 | `abs 0xdb858` | `-` |
| `str` at 0xdb820 | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xdb828 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb82c | load | 1 | `ret(0xdba10) +0x2` | `-` |
| `ldr` at 0xdb830 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdb834 | load | 1 | `ret(0xdba10) +0x3` | `-` |
| `str` at 0xdb84c | store | 4 | `sp -0x1c` | `ret(0xd731c)` |
| `ldr` at 0xdb850 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdb854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb804 | `(unknown callee) 0xdba10` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdb844 | `(unknown callee) 0xd731c` | `0x0` | `[ret(0xdba10)+2]:1` | `[ret(0xdba10)+3]:1` | `[ret(0xdba10)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb818 | `ret(0xdba10) != 0x0` | `ret(0xdba10) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb854 | `r0` | `?` |

## `ip63_reset_hold`

- `ip63_reset_hold`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x383dc to 0x38404.
  Established. The sole unnamed call targets `0x3af68` with entry `r0`, `r1`, `r2`, `r3` (`bl` at
  0x383f4). The exit carries `ret(0x3af68)` (`ldmdb` at 0x38400). The explicit four-byte stack store
  saves entry `r0` at `sp -0x14` (`str` at 0x383ec). Hardware reset-hold effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x383ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x383f0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x38400 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38400 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38400 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x383f4 | `(unknown callee) 0x3af68` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38400 | `r0` | `ret(0x3af68)` |

## `ip63_reset_release`

- `ip63_reset_release`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38404 to 0x3842c.
  Established. Four-byte entry `r0` is saved at `sp -0x14` (`str` at 0x38414). The unnamed call to
  `0x3b128` uses `r0`, `r1`, `r2`, `r3`, and its result reaches the exit (`bl` at 0x3841c, `ldmdb`
  at 0x38428). No conditional branch occurs in this span. Reset-release behavior is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x38414 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x38418 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x38428 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38428 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3841c | `(unknown callee) 0x3b128` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38428 | `r0` | `ret(0x3b128)` |

## `opus_ip65_lbi_tap`

- `opus_ip65_lbi_tap`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2fd90 to 0x2fe04.
  Established. Zero entry `r1` takes the branch to `0x2fdcc`, and zero entry `r2` takes the branch
  to `0x2fde4` (`beq` at 0x2fdbc, `beq` at 0x2fdd4). Included byte stack stores at `sp -0x1d` retain
  `0x10` and unknown at different instructions (`strb` at 0x2fdc8, `strb` at 0x2fde0). The unnamed
  `0xd73b0` call has `0x0`, `0xa`, `0x50`, unknown (`bl` at 0x2fdf4). Tap selection and signal
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2fda0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2fda4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2fda8 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x2fdb0 | store | 1 | `sp -0x1d` | `0x0` |
| `ldr` at 0x2fdb4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x2fdc0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x2fdc8 | store | 1 | `sp -0x1d` | `0x10` |
| `ldr` at 0x2fdcc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2fdd8 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x2fde0 | store | 1 | `sp -0x1d` | `?` |
| `ldrb` at 0x2fde4 | load | 1 | `sp -0x1d` | `-` |
| `ldmdb` at 0x2fe00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2fe00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2fe00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2fdf4 | `(unknown callee) 0xd73b0` | `0x0` | `0xa` | `0x50` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2fdbc | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0x2fdd4 | `r2 == 0x0` | `r2 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2fe00 | `r0` | `ret(0xd73b0)` |

## `networkSerial`

- `networkSerial`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2474c to 0x24854.
  Established. The branch takes `[sp-28] == 0x35` to `0x247a4`, and another takes `[sp-28] == 0x4f`
  to `0x247c0` (`beq` at 0x24774, `beq` at 0x2479c). The unnamed `0xd73b0` call carries `0x0`,
  `0x5`, `0x6e`, `0x44` (`bl` at 0x247f0); its zero result takes the branch to `0x24838`, followed
  by byte `0x1` at absolute `0x148bc0` (`beq` at 0x24804, `strb` at 0x24840). Network-serial routing
  and field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24768 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x2476c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24784 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24794 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x247a8 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x247b0 | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x247b8 | store | 1 | `sp -0x17` | `0x44` |
| `strb` at 0x247c4 | store | 1 | `sp -0x15` | `0x5` |
| `strb` at 0x247cc | store | 1 | `sp -0x16` | `0x6e` |
| `strb` at 0x247d4 | store | 1 | `sp -0x17` | `0x44` |
| `ldrb` at 0x247d8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x247dc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x247e0 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x247f8 | store | 4 | `sp -0x14` | `ret(0xd73b0)` |
| `ldr` at 0x247fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24814 | load | 4 | `abs 0x24848` | `-` |
| `str` at 0x24818 | store | 4 | `sp -0x24` | `0xff49c` |
| `str` at 0x2481c | store | 4 | `sp -0x20` | `ret(0x21a38)` |
| `ldr` at 0x24824 | load | 4 | `abs 0x2484c` | `-` |
| `ldr` at 0x24838 | load | 4 | `abs 0x24850` | `-` |
| `strb` at 0x24840 | store | 1 | `abs 0x148bc0` | `0x1` |
| `ldmdb` at 0x24844 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24844 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24844 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2475c | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x247f0 | `(unknown callee) 0xd73b0` | `0x0` | `0x5` | `0x6e` | `0x44` |
| `bl` at 0x2480c | `(unknown callee) 0x21a38` | `ret(0xd73b0)` | `?` | `?` | `ret(0xd73b0)` |
| `bl` at 0x24830 | `(unknown callee) 0x21eb8` | `0x3` | `0xff4d8` | `0x90` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24774 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `bgt` at 0x24780 | `[sp-28] > 0x35 (signed, 32-bit)` | `[sp-28] <= 0x35 (signed, 32-bit)` |
| `beq` at 0x2478c | `[sp-28] == 0x21` | `[sp-28] != 0x21` |
| `beq` at 0x2479c | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0x24804 | `ret(0xd73b0) == 0x0` | `ret(0xd73b0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24844 | `r0` | `?` |
