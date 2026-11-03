# ARM router packet paths in 1.40.5

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
The selected spans have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/router-packet-paths-facts.txt) retain all tool sections.

## `RouteInit`

- `RouteInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1998 to 0xb1a6c.
  Established. A four-byte absolute assignment at `0x14a828` writes `[0x14a828]+1` (`str` at
  0xb1a48). The unnamed `0xb4a70` call carries `0x2b28e0`, `0xffff017`, `0x1`, `0xb1a6c` (`bl` at
  0xb19f4); its zero result takes the branch to `0xb1a38` (`beq` at 0xb1a08). Routing registration
  effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb19a8 | load | 4 | `abs 0xb1a50` | `-` |
| `ldr` at 0xb19ac | load | 4 | `abs 0x14a828` | `-` |
| `ldr` at 0xb19b8 | load | 4 | `abs 0xb1a54` | `-` |
| `str` at 0xb19bc | store | 4 | `sp -0x1c` | `0x128234` |
| `ldr` at 0xb19c4 | load | 4 | `abs 0xb1a58` | `-` |
| `ldr` at 0xb19d4 | load | 4 | `abs 0xb1a5c` | `-` |
| `ldr` at 0xb19e4 | load | 4 | `abs 0xb1a5c` | `-` |
| `ldr` at 0xb19e8 | load | 4 | `abs 0xb1a60` | `-` |
| `ldr` at 0xb19f0 | load | 4 | `abs 0xb1a64` | `-` |
| `str` at 0xb19fc | store | 4 | `sp -0x14` | `ret(0xb4a70)` |
| `ldr` at 0xb1a00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1a0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1a18 | load | 4 | `abs 0xb1a68` | `-` |
| `str` at 0xb1a1c | store | 4 | `sp -0x1c` | `0x128270` |
| `str` at 0xb1a20 | store | 4 | `sp -0x18` | `ret(0x21704)` |
| `ldr` at 0xb1a28 | load | 4 | `abs 0xb1a58` | `-` |
| `ldr` at 0xb1a38 | load | 4 | `abs 0xb1a50` | `-` |
| `ldr` at 0xb1a3c | load | 4 | `abs 0xb1a50` | `-` |
| `ldr` at 0xb1a40 | load | 4 | `abs 0x14a828` | `-` |
| `str` at 0xb1a48 | store | 4 | `abs 0x14a828` | `[0x14a828]+1` |
| `ldmdb` at 0xb1a4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1a4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1a4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb19d0 | `(unknown callee) 0x21eb8` | `0x9` | `0x128258` | `0x2a` | `0x0` |
| `bl` at 0xb19e0 | `(unknown callee) 0xb5e34` | `0x2b28e0` | `0x17` | `0x0` | `?` |
| `bl` at 0xb19f4 | `(unknown callee) 0xb4a70` | `0x2b28e0` | `0xffff017` | `0x1` | `0xb1a6c` |
| `bl` at 0xb1a10 | `(unknown callee) 0x21704` | `ret(0xb4a70)` | `?` | `?` | `ret(0xb4a70)` |
| `bl` at 0xb1a34 | `(unknown callee) 0x21eb8` | `0x9` | `0x128258` | `0x38` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb19b4 | `[0x14a828] == 0x0` | `[0x14a828] != 0x0` |
| `beq` at 0xb1a08 | `ret(0xb4a70) == 0x0` | `ret(0xb4a70) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1a4c | `r0` | `?` |

## `RouteProcessIn`

- `RouteProcessIn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1c34 to 0xb1de4.
  Established. The branch takes `[0x14a82c] != 0x0` to `0xb1c88` and falls through for equality
  (`bne` at 0xb1c70). The fall-through route includes a four-byte assignment at absolute `0x14a82c`
  retaining `ret(0x75194)` (`str` at 0xb1c84). A later four-byte store with unknown effective target
  writes `ret(0x1b274)` (`str` at 0xb1dc0). Route fields and indexed targets remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1c44 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb1c48 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb1c4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb1c50 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xb1c60 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1c64 | load | 4 | `abs 0xb1de0` | `-` |
| `ldr` at 0xb1c68 | load | 4 | `abs 0x14a82c` | `-` |
| `ldr` at 0xb1c80 | load | 4 | `abs 0xb1de0` | `-` |
| `str` at 0xb1c84 | store | 4 | `abs 0x14a82c` | `ret(0x75194)` |
| `str` at 0xb1c8c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb1c94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb1c98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1ca4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1cb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1cb8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb1ccc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1cdc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1ce0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1cf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1d04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1d08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1d24 | load | 4 | `abs 0xb1de0` | `-` |
| `ldr` at 0xb1d28 | load | 4 | `abs 0x14a82c` | `-` |
| `ldr` at 0xb1d2c | load | 4 | `[0x14a82c] +0xc` | `-` |
| `ldr` at 0xb1d3c | load | 4 | `abs 0xb1de0` | `-` |
| `ldr` at 0xb1d40 | load | 4 | `abs 0x14a82c` | `-` |
| `ldr` at 0xb1d44 | load | 4 | `[0x14a82c] +0x8` | `-` |
| `ldr` at 0xb1d54 | load | 4 | `abs 0xb1de0` | `-` |
| `ldr` at 0xb1d58 | load | 4 | `abs 0x14a82c` | `-` |
| `ldr` at 0xb1d5c | load | 4 | `[0x14a82c] +0x10` | `-` |
| `ldr` at 0xb1d6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1d7c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb1d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1d98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb1d9c | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xb1da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb1da4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1dc0 | store | 4 | `(unknown)` | `ret(0x1b274)` |
| `str` at 0xb1dc8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xb1dcc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1dd4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xb1ddc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1ddc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1ddc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1c78 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `[0x14a82c]` |
| `bl` at 0xb1d0c | `(unknown callee) 0xb823c` | `?` | `?` | `0x24` | `?` |
| `bl` at 0xb1dbc | `(unknown callee) 0x1b274` | `0x0` | `[sp-24]` | `0x30` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1c70 | `[0x14a82c] != 0x0` | `[0x14a82c] == 0x0` |
| `bne` at 0xb1ca0 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xb1cac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb1cd8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb1cfc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb1d18 | `ret(0xb823c) != 0x0` | `ret(0xb823c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1ddc | `r0` | `?` |

## `RouteProcessOut`

- `RouteProcessOut`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb1de4 to 0xb1f48.
  Established. A four-byte absolute assignment at `0x14a830` retains `ret(0x75194)` (`str` at
  0xb1e34). The earlier branch takes `[0x14a830] != 0x0` to `0xb1e38` (`bne` at 0xb1e20). A later
  four-byte store with unknown effective target writes `[sp-40]` (`str` at 0xb1f24). Output-route
  encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb1df4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb1df8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb1dfc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb1e00 | load | 1 | `r0 +0x16` | `-` |
| `str` at 0xb1e10 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb1e14 | load | 4 | `abs 0xb1f44` | `-` |
| `ldr` at 0xb1e18 | load | 4 | `abs 0x14a830` | `-` |
| `ldr` at 0xb1e30 | load | 4 | `abs 0xb1f44` | `-` |
| `str` at 0xb1e34 | store | 4 | `abs 0x14a830` | `ret(0x75194)` |
| `str` at 0xb1e3c | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xb1e44 | store | 4 | `sp -0x1c` | `0x3f` |
| `ldr` at 0xb1e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1e54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1e68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb1e7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xb1e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1e90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1ea4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb1eb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1ed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1ed4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1ee8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1eec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1ef0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1ef8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb1f04 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1f08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb1f0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1f20 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb1f24 | store | 4 | `(unknown)` | `[sp-40]` |
| `str` at 0xb1f2c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xb1f30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1f38 | store | 4 | `sp -0x1c` | `[sp-28]-1` |
| `ldmdb` at 0xb1f40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1f40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1f40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb1e28 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `[0x14a830]` |
| `bl` at 0xb1ebc | `(unknown callee) 0xb823c` | `?` | `?` | `0x24` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb1e20 | `[0x14a830] != 0x0` | `[0x14a830] == 0x0` |
| `bne` at 0xb1e50 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xb1e5c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xb1e88 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb1eac | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb1ec8 | `ret(0xb823c) != 0x0` | `ret(0xb823c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1f40 | `r0` | `?` |

## `IRouterPacketUseCount`

- `IRouterPacketUseCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb432c to 0xb4494.
  Established. A halfword assignment at `[sp-20] +0x4` retains `([sp-32]:2 & 0xffff)` (`strh` at
  0xb43d8). The unnamed `0x22ac8` call carries `0x14a8d4`, unknown, `r2`, `r3` (`bl` at 0xb434c);
  its zero result takes the branch to `0xb4390` (`beq` at 0xb4360). Use-count encoding remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb433c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4340 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb4344 | load | 4 | `abs 0xb4474` | `-` |
| `str` at 0xb4354 | store | 4 | `sp -0x1c` | `ret(0x22ac8)` |
| `ldr` at 0xb4358 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4364 | load | 4 | `abs 0xb4478` | `-` |
| `str` at 0xb4368 | store | 4 | `sp -0x2c` | `0x128c4c` |
| `ldr` at 0xb436c | load | 4 | `abs 0xb4474` | `-` |
| `str` at 0xb4370 | store | 4 | `sp -0x28` | `0x14a8d4` |
| `ldr` at 0xb4374 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4378 | store | 4 | `sp -0x24` | `ret(0x22ac8)` |
| `ldr` at 0xb4380 | load | 4 | `abs 0xb447c` | `-` |
| `ldr` at 0xb4390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4394 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb4398 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xb43a4 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xb43ac | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xb43b0 | store | 2 | `sp -0x20` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xb43b4 | load | 4 | `abs 0xb4480` | `-` |
| `ldrh` at 0xb43c0 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xb43d0 | store | 2 | `sp -0x20` | `?` |
| `ldrh` at 0xb43d4 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0xb43d8 | store | 2 | `[sp-20] +0x4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0xb43dc | load | 4 | `abs 0xb4484` | `-` |
| `ldr` at 0xb43e0 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb43ec | load | 4 | `abs 0xb4484` | `-` |
| `ldr` at 0xb43f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb43f4 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb4400 | load | 4 | `abs 0xb4488` | `-` |
| `ldr` at 0xb4404 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb4408 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb4418 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb441c | load | 4 | `abs 0xb448c` | `-` |
| `ldr` at 0xb4420 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb4428 | load | 4 | `abs 0xb4474` | `-` |
| `str` at 0xb4434 | store | 4 | `sp -0x1c` | `ret(0x22e50)` |
| `ldr` at 0xb4438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb4444 | load | 4 | `abs 0xb4490` | `-` |
| `str` at 0xb4448 | store | 4 | `sp -0x2c` | `0x128ca4` |
| `ldr` at 0xb444c | load | 4 | `abs 0xb4474` | `-` |
| `str` at 0xb4450 | store | 4 | `sp -0x28` | `0x14a8d4` |
| `ldr` at 0xb4454 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb4458 | store | 4 | `sp -0x24` | `ret(0x22e50)` |
| `ldr` at 0xb4460 | load | 4 | `abs 0xb447c` | `-` |
| `ldmdb` at 0xb4470 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb4470 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb4470 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb434c | `(unknown callee) 0x22ac8` | `0x14a8d4` | `?` | `r2` | `r3` |
| `bl` at 0xb438c | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x52` | `0x0` |
| `bl` at 0xb4424 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128c98` | `[sp-20]` | `?` |
| `bl` at 0xb442c | `(unknown callee) 0x22e50` | `0x14a8d4` | `?` | `?` | `?` |
| `bl` at 0xb446c | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4360 | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0xb43e8 | `[0x14a878] == 0x0` | `[0x14a878] != 0x0` |
| `bne` at 0xb43fc | `[sp-20] != [0x14a878]` | `[sp-20] == [0x14a878]` |
| `beq` at 0xb4440 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb4470 | `r0` | `?` |

## `IRouterUnregister`

- `IRouterUnregister`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb4f8c to 0xb52fc.
  Established. Included four-byte absolute assignments at `0x2b2e00` and `0x2b2a40` retain
  `[[sp-36]+40]` and `[[sp-48]+24]` respectively (`str` at 0xb5150, `str` at 0xb522c). The unnamed
  `0x1b490` call has `[sp-36]`, `[[sp-36]]`, `[[sp-36]+40]`, `[[sp-36]+40]` (`bl` at 0xb5180).
  Unregistration matching and record ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb4f9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb4fa0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb4fa4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb4fa8 | load | 4 | `abs 0xb52d0` | `-` |
| `ldr` at 0xb4fac | load | 4 | `abs 0x2b2e00` | `-` |
| `str` at 0xb4fb0 | store | 4 | `sp -0x24` | `[0x2b2e00]` |
| `str` at 0xb4fb8 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb4fc0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb4fc4 | load | 4 | `abs 0xb52d4` | `-` |
| `ldr` at 0xb4fc8 | load | 4 | `abs 0x2b2a40` | `-` |
| `str` at 0xb4fcc | store | 4 | `sp -0x30` | `[0x2b2a40]` |
| `str` at 0xb4fd4 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xb4fdc | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb4fe0 | load | 4 | `abs 0xb52d8` | `-` |
| `str` at 0xb4ff0 | store | 4 | `sp -0x3c` | `ret(0x22ac8)` |
| `ldr` at 0xb4ff4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb5000 | load | 4 | `abs 0xb52dc` | `-` |
| `str` at 0xb5004 | store | 4 | `sp -0x48` | `0x128c4c` |
| `ldr` at 0xb5008 | load | 4 | `abs 0xb52d8` | `-` |
| `str` at 0xb500c | store | 4 | `sp -0x44` | `0x14a880` |
| `ldr` at 0xb5010 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb5014 | store | 4 | `sp -0x40` | `ret(0x22ac8)` |
| `ldr` at 0xb501c | load | 4 | `abs 0xb52e0` | `-` |
| `ldr` at 0xb5020 | load | 4 | `abs 0xb52e4` | `-` |
| `ldr` at 0xb502c | load | 4 | `abs 0xb52e8` | `-` |
| `ldr` at 0xb5030 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5040 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb5054 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb5058 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5068 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5074 | load | 4 | `abs 0xb52ec` | `-` |
| `ldr` at 0xb5078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb5090 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5094 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5098 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb50a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb50b8 | load | 4 | `abs 0xb52ec` | `-` |
| `ldr` at 0xb50d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb50d8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb50e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb50f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb50f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb50fc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xb5100 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb510c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5118 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb511c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xb5120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5130 | load | 4 | `abs 0xb52d0` | `-` |
| `ldr` at 0xb5134 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5138 | load | 4 | `abs 0x2b2e00` | `-` |
| `ldr` at 0xb5144 | load | 4 | `abs 0xb52d0` | `-` |
| `ldr` at 0xb5148 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb514c | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb5150 | store | 4 | `abs 0x2b2e00` | `[[sp-36]+40]` |
| `ldr` at 0xb5158 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb515c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5160 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb5164 | store | 4 | `[sp-40] +0x28` | `[[sp-36]+40]` |
| `ldr` at 0xb5168 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb516c | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb5170 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5174 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb5178 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xb517c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb5188 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb518c | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xb5190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb5194 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0xb5198 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0xb51a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb51ac | load | 4 | `abs 0xb52f0` | `-` |
| `ldr` at 0xb51b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb51b4 | load | 4 | `abs 0x2b2a00` | `-` |
| `ldr` at 0xb51b8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb51c4 | load | 4 | `abs 0xb52f0` | `-` |
| `ldr` at 0xb51d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb51e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb51f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb51f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb51fc | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0xb5200 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb520c | load | 4 | `abs 0xb52d4` | `-` |
| `ldr` at 0xb5210 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5214 | load | 4 | `abs 0x2b2a40` | `-` |
| `ldr` at 0xb5220 | load | 4 | `abs 0xb52d4` | `-` |
| `ldr` at 0xb5224 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5228 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb522c | store | 4 | `abs 0x2b2a40` | `[[sp-48]+24]` |
| `ldr` at 0xb5234 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb5238 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb523c | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb5240 | store | 4 | `[sp-52] +0x18` | `[[sp-48]+24]` |
| `ldr` at 0xb5244 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb5248 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xb524c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5250 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb5254 | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xb5258 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb5264 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xb5268 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xb526c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb5270 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0xb5274 | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0xb527c | load | 4 | `abs 0xb52d8` | `-` |
| `str` at 0xb5288 | store | 4 | `sp -0x3c` | `ret(0x22e50)` |
| `ldr` at 0xb528c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xb5298 | load | 4 | `abs 0xb52f4` | `-` |
| `str` at 0xb529c | store | 4 | `sp -0x48` | `0x128ca4` |
| `ldr` at 0xb52a0 | load | 4 | `abs 0xb52d8` | `-` |
| `str` at 0xb52a4 | store | 4 | `sp -0x44` | `0x14a880` |
| `ldr` at 0xb52a8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb52ac | store | 4 | `sp -0x40` | `ret(0x22e50)` |
| `ldr` at 0xb52b4 | load | 4 | `abs 0xb52e0` | `-` |
| `ldr` at 0xb52b8 | load | 4 | `abs 0xb52f8` | `-` |
| `ldmdb` at 0xb52cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb52cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb52cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb4fe8 | `(unknown callee) 0x22ac8` | `0x14a880` | `?` | `r2` | `0x0` |
| `bl` at 0xb5028 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x167` | `0x0` |
| `bl` at 0xb503c | `(unknown callee) 0x2079c` | `0x128cd4` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xb50cc | `(unknown callee) 0x1b4cc` | `?` | `0x0` | `0x24` | `?` |
| `bl` at 0xb5180 | `(unknown callee) 0x1b490` | `[sp-36]` | `[[sp-36]]` | `[[sp-36]+40]` | `[[sp-36]+40]` |
| `bl` at 0xb51d0 | `(unknown callee) 0x1b4cc` | `0x2b2a00` | `0x0` | `0x24` | `[[sp-20]]` |
| `bl` at 0xb525c | `(unknown callee) 0x1b490` | `[sp-48]` | `[[sp-48]]` | `[[sp-48]+24]` | `[[sp-48]+24]` |
| `bl` at 0xb5280 | `(unknown callee) 0x22e50` | `0x14a880` | `?` | `?` | `?` |
| `bl` at 0xb52c0 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x1a6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb4ffc | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0xb504c | `? == 0x0` | `? != 0x0` |
| `ble` at 0xb5060 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb5070 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xb50a0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb50ec | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0xb5108 | `[[sp-36]] != [[sp-20]]` | `[[sp-36]] == [[sp-20]]` |
| `beq` at 0xb5114 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xb5128 | `[[sp-36]+4] == [sp-24]` | `[[sp-36]+4] != [sp-24]` |
| `bne` at 0xb5140 | `[sp-36] != [0x2b2e00]` | `[sp-36] == [0x2b2e00]` |
| `bne` at 0xb51a8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0xb51c0 | `[0x2b2a00] != [[sp-20]]` | `[0x2b2a00] == [[sp-20]]` |
| `beq` at 0xb51e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb51ec | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0xb5208 | `[[sp-48]] != [[sp-20]]` | `[[sp-48]] == [[sp-20]]` |
| `bne` at 0xb521c | `[sp-48] != [0x2b2a40]` | `[sp-48] == [0x2b2a40]` |
| `beq` at 0xb5294 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb52cc | `r0` | `0x0` |

## `IRouterPacketCount`

- `IRouterPacketCount`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb52fc to 0xb5424.
  Established. Four-byte absolute assignments at `0x14a86c` and `0x14a870` write `([0x14a86c] + [sp-20])` and `([0x14a870] + [sp-20])` respectively (`str` at 0xb5370, `str` at 0xb5388). The
  branch takes signed `[0x14a870] <= [0x14a874]` to `0xb53b4` and falls through for greater values
  (`ble` at 0xb53a0). Count units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb530c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5310 | load | 4 | `abs 0xb5400` | `-` |
| `str` at 0xb5320 | store | 4 | `sp -0x18` | `ret(0x22ac8)` |
| `ldr` at 0xb5324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5330 | load | 4 | `abs 0xb5404` | `-` |
| `str` at 0xb5334 | store | 4 | `sp -0x24` | `0x128c4c` |
| `ldr` at 0xb5338 | load | 4 | `abs 0xb5400` | `-` |
| `str` at 0xb533c | store | 4 | `sp -0x20` | `0x14a8d4` |
| `ldr` at 0xb5340 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb5344 | store | 4 | `sp -0x1c` | `ret(0x22ac8)` |
| `ldr` at 0xb534c | load | 4 | `abs 0xb5408` | `-` |
| `ldr` at 0xb5350 | load | 4 | `abs 0xb540c` | `-` |
| `ldr` at 0xb535c | load | 4 | `abs 0xb5410` | `-` |
| `ldr` at 0xb5360 | load | 4 | `abs 0xb5410` | `-` |
| `ldr` at 0xb5364 | load | 4 | `abs 0x14a86c` | `-` |
| `ldr` at 0xb5368 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5370 | store | 4 | `abs 0x14a86c` | `([0x14a86c] + [sp-20])` |
| `ldr` at 0xb5374 | load | 4 | `abs 0xb5414` | `-` |
| `ldr` at 0xb5378 | load | 4 | `abs 0xb5414` | `-` |
| `ldr` at 0xb537c | load | 4 | `abs 0x14a870` | `-` |
| `ldr` at 0xb5380 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5388 | store | 4 | `abs 0x14a870` | `([0x14a870] + [sp-20])` |
| `ldr` at 0xb538c | load | 4 | `abs 0xb5414` | `-` |
| `ldr` at 0xb5390 | load | 4 | `abs 0xb5418` | `-` |
| `ldr` at 0xb5394 | load | 4 | `abs 0x14a870` | `-` |
| `ldr` at 0xb5398 | load | 4 | `abs 0x14a874` | `-` |
| `ldr` at 0xb53a4 | load | 4 | `abs 0xb5418` | `-` |
| `ldr` at 0xb53a8 | load | 4 | `abs 0xb5414` | `-` |
| `ldr` at 0xb53ac | load | 4 | `abs 0x14a870` | `-` |
| `str` at 0xb53b0 | store | 4 | `abs 0x14a874` | `[0x14a870]` |
| `ldr` at 0xb53b4 | load | 4 | `abs 0xb5400` | `-` |
| `str` at 0xb53c0 | store | 4 | `sp -0x18` | `ret(0x22e50)` |
| `ldr` at 0xb53c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb53d0 | load | 4 | `abs 0xb541c` | `-` |
| `str` at 0xb53d4 | store | 4 | `sp -0x24` | `0x128ca4` |
| `ldr` at 0xb53d8 | load | 4 | `abs 0xb5400` | `-` |
| `str` at 0xb53dc | store | 4 | `sp -0x20` | `0x14a8d4` |
| `ldr` at 0xb53e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb53e4 | store | 4 | `sp -0x1c` | `ret(0x22e50)` |
| `ldr` at 0xb53ec | load | 4 | `abs 0xb5408` | `-` |
| `ldr` at 0xb53f0 | load | 4 | `abs 0xb5420` | `-` |
| `ldmdb` at 0xb53fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb53fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb53fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5318 | `(unknown callee) 0x22ac8` | `0x14a8d4` | `?` | `r2` | `r3` |
| `bl` at 0xb5358 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x1b5` | `0x0` |
| `bl` at 0xb53b8 | `(unknown callee) 0x22e50` | `0x14a8d4` | `[0x14a870]` | `?` | `?` |
| `bl` at 0xb53f8 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x1bd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb532c | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `ble` at 0xb53a0 | `[0x14a870] <= [0x14a874] (signed, 32-bit)` | `[0x14a870] > [0x14a874] (signed, 32-bit)` |
| `beq` at 0xb53cc | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb53fc | `r0` | `?` |

## `IRouterFreePacket`

- `IRouterFreePacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5fec to 0xb63b8.
  Established. A halfword assignment at `[sp-20] +0x4` writes `([sp-40]:2 & 0xffff)` (`strh` at
  0xb61bc). Included four-byte absolute assignments at `0x14a878` and `0x14a870` write `0x0` and
  `[0x14a870]-1` respectively (`str` at 0xb62c8, `str` at 0xb62f0). The unnamed `0x1b490` call has
  `[[sp-20]+8]`, unknown, `[[sp-20]+32]:1`, `[sp-20]` (`bl` at 0xb6260). Packet lifetime rules
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5ffc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb6000 | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb6010 | store | 4 | `sp -0x20` | `ret(0x22ac8)` |
| `ldr` at 0xb6014 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6020 | load | 4 | `abs 0xb6378` | `-` |
| `str` at 0xb6024 | store | 4 | `sp -0x34` | `0x128c4c` |
| `ldr` at 0xb6028 | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb602c | store | 4 | `sp -0x30` | `0x14a8d4` |
| `ldr` at 0xb6030 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6034 | store | 4 | `sp -0x2c` | `ret(0x22ac8)` |
| `ldr` at 0xb603c | load | 4 | `abs 0xb637c` | `-` |
| `ldr` at 0xb6040 | load | 4 | `abs 0xb6380` | `-` |
| `ldr` at 0xb604c | load | 4 | `abs 0xb6384` | `-` |
| `ldr` at 0xb6050 | load | 4 | `abs 0x14a860` | `-` |
| `ldr` at 0xb605c | load | 4 | `abs 0xb6388` | `-` |
| `ldr` at 0xb6060 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6064 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb6078 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb607c | load | 4 | `abs 0xb638c` | `-` |
| `ldr` at 0xb6080 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6088 | load | 4 | `abs 0xb6388` | `-` |
| `ldr` at 0xb608c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb6094 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6098 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb60b8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb60bc | load | 1 | `[sp-20] +0x16` | `-` |
| `ldr` at 0xb60c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb60c4 | load | 4 | `abs 0xb6390` | `-` |
| `ldr` at 0xb60cc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xb60d8 | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb60e4 | store | 4 | `sp -0x20` | `ret(0x22e50)` |
| `ldr` at 0xb60e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb60f4 | load | 4 | `abs 0xb6394` | `-` |
| `str` at 0xb60f8 | store | 4 | `sp -0x34` | `0x128ca4` |
| `ldr` at 0xb60fc | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb6100 | store | 4 | `sp -0x30` | `0x14a8d4` |
| `ldr` at 0xb6104 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6108 | store | 4 | `sp -0x2c` | `ret(0x22e50)` |
| `ldr` at 0xb6110 | load | 4 | `abs 0xb637c` | `-` |
| `ldr` at 0xb6114 | load | 4 | `abs 0xb6398` | `-` |
| `str` at 0xb6124 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb612c | load | 4 | `abs 0xb639c` | `-` |
| `ldr` at 0xb6130 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb613c | load | 4 | `abs 0xb639c` | `-` |
| `ldr` at 0xb6140 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6144 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb6150 | load | 4 | `abs 0xb6388` | `-` |
| `ldr` at 0xb6154 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6158 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0xb616c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb6170 | load | 4 | `abs 0xb63a0` | `-` |
| `ldr` at 0xb6174 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb617c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6180 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6184 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0xb6190 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0xb6194 | store | 2 | `sp -0x28` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0xb6198 | load | 4 | `abs 0xb63a4` | `-` |
| `ldrh` at 0xb61a4 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb61b4 | store | 2 | `sp -0x28` | `?` |
| `ldrh` at 0xb61b8 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb61bc | store | 2 | `[sp-20] +0x4` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xb61dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb61e0 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0xb61f0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb61f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb61f8 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0xb61fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb620c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6218 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb6220 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb622c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6230 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6238 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb6240 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6248 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6250 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb6258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb625c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xb6264 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6268 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0xb626c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb627c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6280 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xb6284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6288 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb628c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb6290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb629c | load | 4 | `abs 0xb639c` | `-` |
| `ldr` at 0xb62a0 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb62ac | load | 4 | `abs 0xb639c` | `-` |
| `ldr` at 0xb62b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb62b4 | load | 4 | `abs 0x14a878` | `-` |
| `ldr` at 0xb62c0 | load | 4 | `abs 0xb639c` | `-` |
| `str` at 0xb62c8 | store | 4 | `abs 0x14a878` | `0x0` |
| `ldr` at 0xb62cc | load | 4 | `abs 0xb6388` | `-` |
| `ldr` at 0xb62d0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb62d4 | load | 4 | `abs 0xb63a8` | `-` |
| `ldr` at 0xb62d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb62e0 | load | 4 | `abs 0xb63ac` | `-` |
| `ldr` at 0xb62e4 | load | 4 | `abs 0xb63ac` | `-` |
| `ldr` at 0xb62e8 | load | 4 | `abs 0x14a870` | `-` |
| `str` at 0xb62f0 | store | 4 | `abs 0x14a870` | `[0x14a870]-1` |
| `ldr` at 0xb62f4 | load | 4 | `abs 0xb6384` | `-` |
| `ldr` at 0xb62f8 | load | 4 | `abs 0x14a860` | `-` |
| `ldr` at 0xb6304 | load | 4 | `abs 0xb6388` | `-` |
| `ldr` at 0xb6308 | load | 4 | `abs 0xb63ac` | `-` |
| `ldr` at 0xb630c | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb6310 | load | 4 | `abs 0xb63b0` | `-` |
| `ldr` at 0xb6314 | load | 4 | `abs 0x14a870` | `-` |
| `ldr` at 0xb631c | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb6328 | store | 4 | `sp -0x20` | `ret(0x22e50)` |
| `ldr` at 0xb632c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6338 | load | 4 | `abs 0xb6394` | `-` |
| `str` at 0xb633c | store | 4 | `sp -0x34` | `0x128ca4` |
| `ldr` at 0xb6340 | load | 4 | `abs 0xb6374` | `-` |
| `str` at 0xb6344 | store | 4 | `sp -0x30` | `0x14a8d4` |
| `ldr` at 0xb6348 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb634c | store | 4 | `sp -0x2c` | `ret(0x22e50)` |
| `ldr` at 0xb6354 | load | 4 | `abs 0xb637c` | `-` |
| `ldr` at 0xb6358 | load | 4 | `abs 0xb63b4` | `-` |
| `str` at 0xb6368 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb636c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb6370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6008 | `(unknown callee) 0x22ac8` | `0x14a8d4` | `?` | `r2` | `r3` |
| `bl` at 0xb6048 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x3d9` | `0x0` |
| `bl` at 0xb6084 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128d08` | `[sp-20]` | `?` |
| `bl` at 0xb6090 | `(unknown callee) 0x1b0ac` | `[0x145ae8]` | `?` | `?` | `0x145ae8` |
| `bl` at 0xb60b0 | `(unknown callee) 0x22728` | `?` | `?` | `?` | `?` |
| `bl` at 0xb60d4 | `(unknown callee) 0x2079c` | `0x128d18` | `[[sp-20]+22]:1` | `[[sp-20]+28]` | `ret(0x22728)` |
| `bl` at 0xb60dc | `(unknown callee) 0x22e50` | `0x14a8d4` | `?` | `?` | `?` |
| `bl` at 0xb611c | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x3e6` | `0x0` |
| `bl` at 0xb6178 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128d44` | `[sp-20]` | `?` |
| `bl` at 0xb6244 | `(unknown callee) 0x1b490` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0xb6260 | `(unknown callee) 0x1b490` | `[[sp-20]+8]` | `?` | `[[sp-20]+32]:1` | `[sp-20]` |
| `bl` at 0xb6294 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xb62dc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128d50` | `[sp-20]` | `0x145ae8` |
| `bl` at 0xb6318 | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128d5c` | `[0x14a870]` | `0x145ae8` |
| `bl` at 0xb6320 | `(unknown callee) 0x22e50` | `0x14a8d4` | `?` | `?` | `?` |
| `bl` at 0xb6360 | `(unknown callee) 0x21eb8` | `0x9` | `0x128c78` | `0x416` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb601c | `ret(0x22ac8) == 0x0` | `ret(0x22ac8) != 0x0` |
| `beq` at 0xb6058 | `[0x14a860] == 0x0` | `[0x14a860] != 0x0` |
| `bne` at 0xb60ac | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb60f0 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |
| `beq` at 0xb6138 | `[0x14a878] == 0x0` | `[0x14a878] != 0x0` |
| `bne` at 0xb614c | `[sp-20] != [0x14a878]` | `[sp-20] == [0x14a878]` |
| `bne` at 0xb61d8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb61e8 | `[[sp-20]+8] == 0x0` | `[[sp-20]+8] != 0x0` |
| `blt` at 0xb6204 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb6228 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb6274 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb62a8 | `[0x14a878] == 0x0` | `[0x14a878] != 0x0` |
| `bne` at 0xb62bc | `[sp-20] != [0x14a878]` | `[sp-20] == [0x14a878]` |
| `beq` at 0xb6300 | `[0x14a860] == 0x0` | `[0x14a860] != 0x0` |
| `beq` at 0xb6334 | `ret(0x22e50) == 0x0` | `ret(0x22e50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6370 | `r0` | `0x0` |

## `IRouterSendPacket`

- `IRouterSendPacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7090 to 0xb7164.
  Established. A byte read targets `r1 +0x16` (`ldrb` at 0xb70ac). The unnamed `0xb6690` call has
  `r0`, `r1`, `0x1`, `[r1+22]:1` (`bl` at 0xb70e8); its zero result takes the branch to `0xb714c`
  (`beq` at 0xb70fc). Another unnamed call to `0xb5fec` carries `[sp-24]`, unknown, unknown, unknown
  (`bl` at 0xb7150). Send modes and ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb70a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb70a4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb70a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb70ac | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb70b0 | store | 4 | `sp -0x20` | `[r1+22]:1` |
| `ldr` at 0xb70b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb70c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb70cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb70dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb70e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb70f0 | store | 4 | `sp -0x1c` | `ret(0xb6690)` |
| `ldr` at 0xb70f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7100 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7104 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xb7114 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb7118 | load | 4 | `abs 0xb7160` | `-` |
| `ldr` at 0xb711c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7134 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7140 | store | 4 | `sp -0x1c` | `ret(0xb6d00)` |
| `str` at 0xb7148 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb714c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7154 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb715c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb715c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb715c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb70e8 | `(unknown callee) 0xb6690` | `r0` | `r1` | `0x1` | `[r1+22]:1` |
| `bl` at 0xb7128 | `(unknown callee) 0xb74d4` | `0x2b2a60` | `ret(0xb6690)` | `[sp-24]` | `0x0` |
| `bl` at 0xb7138 | `(unknown callee) 0xb6d00` | `r0` | `r1` | `r2` | `[r1+22]:1` |
| `bl` at 0xb7150 | `(unknown callee) 0xb5fec` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb70bc | `[r1+22]:1 < 0x0 (signed, 32-bit)` | `[r1+22]:1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0xb70c8 | `[r1+22]:1 <= 0x1 (signed, 32-bit)` | `[r1+22]:1 > 0x1 (signed, 32-bit)` |
| `beq` at 0xb70d4 | `[r1+22]:1 == 0x2` | `[r1+22]:1 != 0x2` |
| `beq` at 0xb70fc | `ret(0xb6690) == 0x0` | `ret(0xb6690) != 0x0` |
| `bne` at 0xb710c | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb715c | `r0` | `[sp-28]` |

## `IRouterQPacket`

- `IRouterQPacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7164 to 0xb7394.
  Established. Included four-byte stores with unknown effective targets write `0x0` (`str` at
  0xb7194, `str` at 0xb71a8). The branch takes `[[sp-24]+22]:1 != 0x2` to `0xb7224` and falls
  through for equality (`bne` at 0xb71b8). The unnamed `0x23744` call carries `[sp-20]`, `sp-24`,
  `[0x1436b8]`, `0xb6438` (`bl` at 0xb7254). Queue suspension and callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7174 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7178 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7180 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb7194 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xb71a8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb71ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb71b0 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xb71c0 | store | 4 | `sp -0x54` | `sp-24` |
| `ldr` at 0xb71c4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb71c8 | load | 2 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb71e0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb71e4 | load | 2 | `[sp-24] +0x6` | `-` |
| `str` at 0xb71f4 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xb71fc | load | 4 | `abs 0xb736c` | `-` |
| `ldr` at 0xb7200 | load | 4 | `abs 0x1436bc` | `-` |
| `str` at 0xb7204 | store | 4 | `sp -0x58` | `[0x1436bc]` |
| `ldr` at 0xb7208 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb720c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xb7210 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xb721c | store | 4 | `sp -0x28` | `ret(0x23b30)` |
| `ldr` at 0xb7228 | load | 4 | `abs 0xb7370` | `-` |
| `ldr` at 0xb722c | load | 4 | `abs 0xb7374` | `-` |
| `str` at 0xb7230 | store | 4 | `sp -0x74` | `0xb657c` |
| `str` at 0xb7238 | store | 4 | `sp -0x70` | `sp-32` |
| `str` at 0xb7240 | store | 4 | `sp -0x6c` | `0x1` |
| `ldr` at 0xb7244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb724c | load | 4 | `abs 0x1436b8` | `-` |
| `ldr` at 0xb7250 | load | 4 | `abs 0xb7378` | `-` |
| `str` at 0xb725c | store | 4 | `sp -0x28` | `ret(0x23744)` |
| `ldr` at 0xb7260 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb726c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7270 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xb7274 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb7280 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb728c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb729c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb72ac | load | 4 | `abs 0xb737c` | `-` |
| `str` at 0xb72b0 | store | 4 | `sp -0x28` | `0xfffffd9d` |
| `str` at 0xb72b8 | store | 4 | `sp -0x5c` | `sp-80` |
| `ldr` at 0xb72bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb72c4 | store | 4 | `sp -0x60` | `[sp-20]+120` |
| `ldr` at 0xb72c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb72cc | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb72d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb72dc | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb72e8 | load | 4 | `abs 0xb7380` | `-` |
| `str` at 0xb72ec | store | 4 | `sp -0x64` | `0x128d80` |
| `ldr` at 0xb72f4 | load | 4 | `abs 0xb7384` | `-` |
| `str` at 0xb72f8 | store | 4 | `sp -0x64` | `0x128e28` |
| `ldr` at 0xb7300 | load | 4 | `abs 0xb7388` | `-` |
| `str` at 0xb7304 | store | 4 | `sp -0x64` | `0x128d7c` |
| `ldr` at 0xb7308 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xb730c | load | 4 | `abs 0xb738c` | `-` |
| `ldr` at 0xb7310 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xb7314 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb732c | load | 4 | `abs 0xb7390` | `-` |
| `str` at 0xb7330 | store | 4 | `sp -0x28` | `0xfffffd9e` |
| `ldr` at 0xb7340 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7358 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7360 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb7368 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7368 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7214 | `(unknown callee) 0x23b30` | `[sp-20]` | `sp-24` | `?` | `?` |
| `bl` at 0xb7254 | `(unknown callee) 0x23744` | `[sp-20]` | `sp-24` | `[0x1436b8]` | `0xb6438` |
| `bl` at 0xb7318 | `(unknown callee) 0x1b52c` | `sp-80` | `0x128e2c` | `[sp-20]+120` | `?` |
| `bl` at 0xb7324 | `(unknown callee) 0x202d0` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0xb735c | `(unknown callee) 0xb5fec` | `?` | `sp-16` | `sp-16` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb71b8 | `[[sp-24]+22]:1 != 0x2` | `[[sp-24]+22]:1 == 0x2` |
| `beq` at 0xb71dc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb7268 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb727c | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xb7288 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xb7294 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb72a4 | `[sp-104] == 0x6e` | `[sp-104] != 0x6e` |
| `beq` at 0xb72d4 | `[[sp-24]+22]:1 == 0x0` | `[[sp-24]+22]:1 != 0x0` |
| `bne` at 0xb72e4 | `[[sp-24]+22]:1 != 0x1` | `[[sp-24]+22]:1 == 0x1` |
| `beq` at 0xb7348 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7368 | `r0` | `[sp-40]` |

## `IRouterRequest`

- `IRouterRequest`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7394 to 0xb74d4.
  Established. The branch takes `[sp+8] == 0x0` to `0xb73d0` and falls through for nonzero (`beq` at
  0xb73bc). A four-byte assignment at `[sp+8] +0x0` retains `[r0+6]:2` (`str` at 0xb73cc). The
  unnamed `0xb5550` call has `[sp-48]`, `0x0`, `0x0`, `[sp-52]` (`bl` at 0xb7480). Request field
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb73a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb73a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb73ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb73b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb73b4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb73c0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb73c4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb73c8 | load | 2 | `r0 +0x6` | `-` |
| `str` at 0xb73cc | store | 4 | `[sp+8] +0x0` | `[r0+6]:2` |
| `str` at 0xb73d4 | store | 4 | `sp -0x30` | `sp-40` |
| `ldr` at 0xb73d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb73dc | load | 1 | `(unknown)` | `-` |
| `str` at 0xb73e0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb73e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb73e8 | store | 4 | `sp -0x5c` | `?` |
| `str` at 0xb73f0 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xb73f4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb73f8 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb7400 | store | 2 | `(unknown)` | `?` |
| `str` at 0xb740c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb7410 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7414 | store | 4 | `sp -0x50` | `[sp-24]` |
| `ldr` at 0xb7418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7424 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7428 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xb7434 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7438 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb7440 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7444 | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0xb7448 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb744c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb7450 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb7454 | store | 4 | `sp -0x48` | `[sp]` |
| `ldr` at 0xb7458 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb745c | store | 4 | `sp -0x44` | `[sp+4]` |
| `ldr` at 0xb7460 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb7464 | store | 4 | `sp -0x40` | `[sp+12]` |
| `str` at 0xb746c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xb7470 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb747c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb7488 | store | 4 | `sp -0x24` | `ret(0xb5550)` |
| `ldr` at 0xb748c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7498 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb749c | store | 4 | `sp -0x2c` | `ret(0xb5550)` |
| `ldr` at 0xb74a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb74a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb74b8 | store | 4 | `sp -0x24` | `ret(0xb6690)` |
| `ldr` at 0xb74bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb74c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb74c8 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb74cc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb74d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb74d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb74d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7480 | `(unknown callee) 0xb5550` | `[sp-48]` | `0x0` | `0x0` | `[sp-52]` |
| `bl` at 0xb74b0 | `(unknown callee) 0xb6690` | `[sp-20]` | `[sp-40]` | `0x1` | `ret(0xb5550)` |
| `bl` at 0xb74c0 | `(unknown callee) 0xb5fec` | `[sp-40]` | `?` | `?` | `ret(0xb6690)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb73bc | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xb7420 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xb7430 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0xb7494 | `ret(0xb5550) == 0x0` | `ret(0xb5550) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb74d0 | `r0` | `?` |

## `IRouterResponse`

- `IRouterResponse`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb74d4 to 0xb764c.
  Established. Byte reads target `r0 +0x4` and `r2 +0x17` on separate routes (`ldrb` at 0xb750c,
  `ldrb` at 0xb751c). The deciding branch takes `r0 == 0x0` to `0xb7518` and falls through for
  nonzero (`beq` at 0xb7504). The unnamed `0xb5550` call carries `[sp-48]`, `0x1`, `0x0`, `[sp-52]`
  (`bl` at 0xb75f4). Response encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb74e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb74e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb74ec | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb74f0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb74f8 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xb74fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7508 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb750c | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xb7510 | store | 4 | `sp -0x34` | `[r0+4]:1` |
| `ldr` at 0xb7518 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb751c | load | 1 | `r2 +0x17` | `-` |
| `str` at 0xb7520 | store | 4 | `sp -0x34` | `[r2+23]:1` |
| `ldr` at 0xb7524 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb7528 | load | 2 | `r2 +0x22` | `-` |
| `str` at 0xb752c | store | 4 | `sp -0x5c` | `[r2+34]:2` |
| `ldr` at 0xb7530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb753c | load | 4 | `abs 0xb7648` | `-` |
| `str` at 0xb756c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xb7570 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb7574 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xb7578 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb757c | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xb7588 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb758c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7594 | store | 4 | `sp -0x50` | `[sp-28]+28` |
| `ldr` at 0xb7598 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb759c | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb75a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb75ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb75b4 | store | 4 | `sp -0x38` | `[sp-28]+24` |
| `ldr` at 0xb75b8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb75bc | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb75c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb75c4 | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xb75c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb75cc | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0xb75d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb75d4 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xb75d8 | store | 4 | `sp -0x40` | `[[sp-28]+12]` |
| `str` at 0xb75e0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xb75e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb75f0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb75fc | store | 4 | `sp -0x28` | `ret(0xb5550)` |
| `ldr` at 0xb7600 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb760c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7610 | store | 4 | `sp -0x2c` | `ret(0xb5550)` |
| `ldr` at 0xb7618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb761c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb762c | store | 4 | `sp -0x28` | `ret(0xb6690)` |
| `ldr` at 0xb7630 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7638 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb763c | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xb7640 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb7644 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7644 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7644 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb75f4 | `(unknown callee) 0xb5550` | `[sp-48]` | `0x1` | `0x0` | `[sp-52]` |
| `bl` at 0xb7624 | `(unknown callee) 0xb6690` | `[sp-20]` | `[sp-36]` | `0x1` | `ret(0xb5550)` |
| `bl` at 0xb7634 | `(unknown callee) 0xb5fec` | `[sp-36]` | `?` | `?` | `ret(0xb6690)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7504 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xb75a8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xb7608 | `ret(0xb5550) == 0x0` | `ret(0xb5550) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7644 | `r0` | `?` |

## `IRouterEvent`

- `IRouterEvent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb764c to 0xb7730.
  Established. A four-byte absolute assignment at `0x14a87c` writes `[0x14a87c]+1` (`str` at
  0xb769c). The unnamed `0xb5550` call has `sp-36`, `0x2`, `0x0`, `[r0+4]:1` (`bl` at 0xb76dc); its
  zero result takes the branch to `0xb7700` (`beq` at 0xb76f0). Event counter and payload meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb765c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7660 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7664 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7668 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb7670 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb7674 | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xb7678 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb767c | store | 4 | `sp -0x50` | `r1` |
| `str` at 0xb7684 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xb7688 | load | 4 | `abs 0xb772c` | `-` |
| `ldr` at 0xb768c | load | 4 | `abs 0x14a87c` | `-` |
| `str` at 0xb7694 | store | 4 | `sp -0x48` | `[0x14a87c]` |
| `str` at 0xb769c | store | 4 | `abs 0x14a87c` | `[0x14a87c]+1` |
| `str` at 0xb76a4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb76a8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb76ac | store | 4 | `sp -0x40` | `[sp-20]` |
| `ldr` at 0xb76b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb76b4 | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xb76b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb76bc | store | 4 | `sp -0x38` | `[sp-32]` |
| `str` at 0xb76c4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb76c8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb76cc | store | 4 | `sp -0x30` | `[sp]` |
| `str` at 0xb76e4 | store | 4 | `sp -0x28` | `ret(0xb5550)` |
| `ldr` at 0xb76e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb76f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb76f8 | store | 4 | `sp -0x2c` | `ret(0xb5550)` |
| `ldr` at 0xb7700 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7704 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7710 | store | 4 | `sp -0x28` | `ret(0xb6d00)` |
| `ldr` at 0xb7714 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb771c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7720 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xb7724 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb7728 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7728 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7728 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb76dc | `(unknown callee) 0xb5550` | `sp-36` | `0x2` | `0x0` | `[r0+4]:1` |
| `bl` at 0xb7708 | `(unknown callee) 0xb6d00` | `[sp-20]` | `[sp-36]` | `?` | `ret(0xb5550)` |
| `bl` at 0xb7718 | `(unknown callee) 0xb5fec` | `[sp-36]` | `?` | `?` | `ret(0xb6d00)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb76f0 | `ret(0xb5550) == 0x0` | `ret(0xb5550) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7728 | `r0` | `?` |

## `IRouterCommand`

- `IRouterCommand`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7730 to 0xb785c.
  Established. The unnamed `0xb7394` call carries `r0`, `r1`, `r2`, `r3` (`bl` at 0xb7780); its zero
  result takes the branch to `0xb77a4` (`beq` at 0xb7794). A later unnamed `0xb5f90` call has
  `[sp+8]`, `[sp+12]`, `[sp+16]`, `ret(0xb7394)` (`bl` at 0xb77b0). Command reply matching and error
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7740 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7744 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7748 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb774c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb7750 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb7754 | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0xb7758 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb775c | store | 4 | `sp -0x3c` | `[sp+4]` |
| `str` at 0xb7764 | store | 4 | `sp -0x38` | `sp-40` |
| `str` at 0xb776c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb7770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb777c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7788 | store | 4 | `sp -0x24` | `ret(0xb7394)` |
| `ldr` at 0xb778c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7798 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb779c | store | 4 | `sp -0x30` | `ret(0xb7394)` |
| `ldr` at 0xb77a4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb77a8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb77ac | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb77b8 | store | 4 | `sp -0x24` | `ret(0xb5f90)` |
| `ldr` at 0xb77bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb77c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb77cc | store | 4 | `sp -0x30` | `ret(0xb5f90)` |
| `ldr` at 0xb77d4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb77d8 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb77dc | load | 1 | `[[sp+8]] +0x14` | `-` |
| `ldr` at 0xb77e0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb77e4 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb77e8 | load | 1 | `[[sp+8]] +0x21` | `-` |
| `str` at 0xb77f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb77f8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb77fc | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb7800 | load | 1 | `[[sp+8]] +0x12` | `-` |
| `ldr` at 0xb780c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7814 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7820 | load | 4 | `abs 0xb7858` | `-` |
| `str` at 0xb7824 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `ldr` at 0xb782c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb783c | load | 4 | `abs 0xb7858` | `-` |
| `str` at 0xb7840 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `str` at 0xb784c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb7850 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xb7854 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7854 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7780 | `(unknown callee) 0xb7394` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb77b0 | `(unknown callee) 0xb5f90` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ret(0xb7394)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7794 | `ret(0xb7394) == 0x0` | `ret(0xb7394) != 0x0` |
| `beq` at 0xb77c4 | `ret(0xb5f90) == 0x0` | `ret(0xb5f90) != 0x0` |
| `bne` at 0xb7808 | `[[[sp+8]]+18]:1 != 0x0` | `[[[sp+8]]+18]:1 == 0x0` |
| `beq` at 0xb781c | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xb7838 | `[sp-40] == [sp-44]` | `[sp-40] != [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7854 | `r0` | `?` |

## `IRouterMakePacket`

- `IRouterMakePacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb785c to 0xb7924.
  Established. A halfword assignment at `r0 +0x6` writes `([r0+6]:2+1 & 0xffff)` (`strh` at
  0xb78ac). The unnamed `0xb5550` call has `sp-36`, `[sp-32]`, `[sp-40]`, `[r0+4]:1` (`bl` at
  0xb78f8); its zero result takes the branch to `0xb7910` (`beq` at 0xb7904). Packet layout and
  sequence meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb786c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7870 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7874 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7878 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb7880 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb7888 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb788c | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xb7890 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb7894 | store | 4 | `sp -0x4c` | `r2` |
| `str` at 0xb789c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xb78a0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb78a4 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xb78ac | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0xb78b8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb78bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb78c0 | store | 4 | `sp -0x40` | `[sp-24]` |
| `ldr` at 0xb78c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb78c8 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0xb78cc | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb78d0 | store | 4 | `sp -0x38` | `[sp+4]` |
| `ldr` at 0xb78d4 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb78d8 | store | 4 | `sp -0x34` | `[sp+8]` |
| `str` at 0xb78e0 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xb78e8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb78ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb78f0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb790c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xb7914 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb7918 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7920 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7920 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7920 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb78f8 | `(unknown callee) 0xb5550` | `sp-36` | `[sp-32]` | `[sp-40]` | `[r0+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7904 | `ret(0xb5550) == 0x0` | `ret(0xb5550) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7920 | `r0` | `[sp-36]` |

## `IRouterReadPacket`

- `IRouterReadPacket`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7924 to 0xb7da0.
  Established. The indirect call has unknown target and candidates `[sp-28]`, `sp-44`, `[sp-64]`,
  `sp-44` (`mov` at 0xb79f0); its signed positive result takes the branch to `0xb7b2c` and
  nonpositive falls through (`bgt` at 0xb7a04). A later four-byte assignment at `[sp-32] +0x0`
  retains `[sp-40]` (`str` at 0xb7d54). Input callback behavior and packet assembly rules remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7934 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7938 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb793c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7940 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb7948 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb7950 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xb7958 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xb7960 | store | 4 | `sp -0x3c` | `0x410` |
| `ldr` at 0xb7964 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb796c | store | 4 | `sp -0x40` | `0x400` |
| `ldr` at 0xb7970 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb797c | store | 4 | `sp -0x2c` | `ret(0x1b1b4)` |
| `ldr` at 0xb7980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb798c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb799c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb79a0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb79a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb79a8 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb79ac | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb79b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb79bc | load | 4 | `abs 0xb7d70` | `-` |
| `str` at 0xb79c0 | store | 4 | `sp -0x60` | `0xfffffda4` |
| `ldr` at 0xb79c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb79dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb79e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb79e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb79f8 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xb79fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7a08 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7a0c | store | 4 | `sp -0x64` | `ret(unknown)` |
| `ldr` at 0xb7a10 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb7a1c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xb7a30 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb7a38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7a3c | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xb7a44 | load | 4 | `abs 0xb7d74` | `-` |
| `str` at 0xb7a48 | store | 4 | `sp -0x44` | `0xfffffd9c` |
| `ldr` at 0xb7a4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7a58 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7a5c | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7a60 | store | 4 | `sp -0x48` | `[[sp-40]]` |
| `ldr` at 0xb7a64 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7a74 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7a78 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7a7c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb7a80 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7a88 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb7a8c | store | 4 | `sp -0x48` | `[sp-76]` |
| `ldr` at 0xb7a94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7a9c | store | 4 | `[sp-40] +0x0` | `0x0` |
| `ldr` at 0xb7aa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7aac | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb7ab0 | load | 1 | `[sp-40] +0x16` | `-` |
| `str` at 0xb7ac0 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xb7ac4 | load | 4 | `abs 0xb7d78` | `-` |
| `ldr` at 0xb7ac8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb7acc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7ad8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7ae4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7af0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7b00 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7b04 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb7b08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7b0c | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7b10 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb7b14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7b20 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xb7b24 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xb7b2c | load | 4 | `abs 0xb7d7c` | `-` |
| `ldr` at 0xb7b30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7b34 | load | 4 | `abs 0x14a85c` | `-` |
| `ldr` at 0xb7b3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7b48 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb7b4c | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xb7b60 | load | 4 | `abs 0xb7d80` | `-` |
| `ldr` at 0xb7b64 | load | 4 | `abs 0x14a864` | `-` |
| `ldr` at 0xb7b7c | load | 4 | `abs 0xb7d84` | `-` |
| `ldr` at 0xb7b80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7b88 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7b94 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xb7b9c | load | 4 | `abs 0xb7d88` | `-` |
| `ldr` at 0xb7ba0 | load | 4 | `abs 0x14a860` | `-` |
| `ldr` at 0xb7bac | load | 4 | `abs 0xb7d8c` | `-` |
| `ldr` at 0xb7bb0 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb7bb4 | load | 4 | `abs 0xb7d90` | `-` |
| `ldr` at 0xb7bb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7bc0 | load | 4 | `abs 0xb7d8c` | `-` |
| `ldr` at 0xb7bc4 | load | 4 | `abs 0x145ae8` | `-` |
| `ldr` at 0xb7bcc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7bd0 | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0xb7bd8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb7bdc | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xb7be4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb7bf4 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xb7bf8 | load | 4 | `abs 0xb7d80` | `-` |
| `ldr` at 0xb7bfc | load | 4 | `abs 0x14a864` | `-` |
| `ldr` at 0xb7c18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7c28 | load | 4 | `abs 0xb7d80` | `-` |
| `ldr` at 0xb7c2c | load | 4 | `abs 0x14a864` | `-` |
| `ldr` at 0xb7c40 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb7c4c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb7c54 | load | 4 | `abs 0xb7d94` | `-` |
| `ldr` at 0xb7c68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7c70 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7c74 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb7c78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7c7c | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7c80 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb7c84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7c90 | load | 4 | `abs 0xb7d80` | `-` |
| `ldr` at 0xb7c94 | load | 4 | `abs 0x14a864` | `-` |
| `ldr` at 0xb7ca0 | load | 4 | `abs 0xb7d98` | `-` |
| `ldr` at 0xb7ca4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7cac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7cb4 | load | 4 | `abs 0xb7d9c` | `-` |
| `str` at 0xb7cb8 | store | 4 | `sp -0x60` | `0xfffffd9a` |
| `ldr` at 0xb7cc0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7cc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7cc8 | store | 4 | `[sp-48] +0x0` | `[sp-44]` |
| `ldr` at 0xb7ccc | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xb7cd0 | load | 2 | `[sp-44] +0x4` | `-` |
| `strh` at 0xb7cd4 | store | 2 | `sp -0x68` | `([[sp-44]+4]:2 & 0xffff)` |
| `ldrh` at 0xb7cd8 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xb7ce4 | store | 2 | `sp -0x68` | `?` |
| `ldrh` at 0xb7ce8 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xb7cec | store | 2 | `[sp-44] +0x4` | `([sp-104]:2 & 0xffff)` |
| `ldr` at 0xb7cf0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7cf8 | store | 4 | `[sp-44] +0x0` | `0x0` |
| `ldr` at 0xb7cfc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb7d00 | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0xb7d0c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7d10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7d14 | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xb7d18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb7d24 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb7d28 | load | 2 | `[sp-40] +0x4` | `-` |
| `strh` at 0xb7d2c | store | 2 | `sp -0x6c` | `([[sp-40]+4]:2 & 0xffff)` |
| `ldrh` at 0xb7d30 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xb7d40 | store | 2 | `sp -0x6c` | `?` |
| `ldrh` at 0xb7d44 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xb7d48 | store | 2 | `[sp-40] +0x4` | `([sp-108]:2 & 0xffff)` |
| `ldr` at 0xb7d4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7d50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7d54 | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `str` at 0xb7d64 | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xb7d68 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xb7d6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7d6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7d6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7974 | `(unknown callee) 0x1b1b4` | `0x410` | `?` | `?` | `0x400` |
| `bl` at 0xb79b4 | `(unknown callee) 0x1b490` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb79d4 | `(unknown callee) 0x1b4cc` | `ret(0x1b1b4)` | `0x0` | `0x2c` | `ret(0x1b1b4)` |
| `mov` at 0xb79f0 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-44` | `[sp-64]` | `sp-44` |
| `bl` at 0xb7a84 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7ad4 | `(unknown callee) 0xb74d4` | `0x2b2a60` | `?` | `[sp-40]` | `0x0` |
| `bl` at 0xb7ae8 | `(unknown callee) 0x1b490` | `[sp-44]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xb7b18 | `(unknown callee) 0x1b490` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb7b38 | `(unknown callee) 0xb7f6c` | `[sp-44]` | `[0x14a85c]` | `?` | `0x14a85c` |
| `bl` at 0xb7b78 | `(unknown callee) 0xb89f4` | `0x0` | `0x0` | `?` | `[0x14a864]` |
| `bl` at 0xb7b84 | `(unknown callee) 0xb89f4` | `0x128e44` | `[sp-44]` | `?` | `?` |
| `bl` at 0xb7b8c | `(unknown callee) 0x1b490` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb7bbc | `(unknown callee) 0x240b8` | `[0x145ae8]` | `0x128e4c` | `[sp-44]` | `0x145ae8` |
| `bl` at 0xb7bc8 | `(unknown callee) 0x1b0ac` | `[0x145ae8]` | `?` | `?` | `0x145ae8` |
| `bl` at 0xb7c10 | `(unknown callee) 0xb89f4` | `0x0` | `0x0` | `?` | `[0x14a864]` |
| `bl` at 0xb7c5c | `(unknown callee) 0x1b52c` | `sp-92` | `0x128e58` | `?` | `?` |
| `bl` at 0xb7c6c | `(unknown callee) 0xb89f4` | `sp-92` | `[sp-40]` | `?` | `sp-92` |
| `bl` at 0xb7c88 | `(unknown callee) 0x1b490` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb7ca8 | `(unknown callee) 0xb89f4` | `0x128e60` | `[sp-44]` | `?` | `[0x14a864]` |
| `bl` at 0xb7cb0 | `(unknown callee) 0x1b490` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb7d5c | `(unknown callee) 0xb52fc` | `0x1` | `([[sp-40]+4]:2 & 0xffff)` | `[sp-32]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7988 | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xb7994 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bgt` at 0xb7a04 | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xb7a18 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb7a24 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xb7a54 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xb7a6c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb7aa8 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xb7ab8 | `[[sp-40]+22]:1 != 0x0` | `[[sp-40]+22]:1 == 0x0` |
| `beq` at 0xb7ae0 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0xb7af8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb7b44 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb7b5c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb7b6c | `[0x14a864] == 0x0` | `[0x14a864] != 0x0` |
| `beq` at 0xb7ba8 | `[0x14a860] == 0x0` | `[0x14a860] != 0x0` |
| `beq` at 0xb7bec | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xb7c04 | `[0x14a864] == 0x0` | `[0x14a864] != 0x0` |
| `bne` at 0xb7c20 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xb7c34 | `[0x14a864] == 0x0` | `[0x14a864] != 0x0` |
| `beq` at 0xb7c9c | `[0x14a864] == 0x0` | `[0x14a864] != 0x0` |
| `bne` at 0xb7d20 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7d6c | `r0` | `?` |
