# ARM outbound communication and interrupt paths in 1.40.5

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
These routines have no unresolved computed transfers. Numeric callee names, arity and effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/syscom-outbound-facts.txt) retain all tool sections.

## `NL4R_SysComEnable`

- `NL4R_SysComEnable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27db8 to 0x27eb4.
  Established. The branch takes signed `r0 < [0x148be0]` to `0x27e60` (`blt` at 0x27e50). That route
  includes the unnamed `0xd76d4` call with `0x0`, `[[sp-24]+2]:1`, `0x2`, `[[sp-24]+2]:1` (`bl` at
  0x27e94). The prior byte read targets `[sp-24] +0x2` (`ldrb` at 0x27e84). Interrupt-enable
  effects and the loaded value meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27dc8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x27dcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27ddc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27de0 | load | 4 | `abs 0x27ea8` | `-` |
| `ldr` at 0x27de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27de8 | load | 4 | `abs 0x148be0` | `-` |
| `ldr` at 0x27df8 | load | 4 | `abs 0x27eac` | `-` |
| `ldr` at 0x27dfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27e04 | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27e0c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x27e10 | load | 4 | `abs 0x27eb0` | `-` |
| `ldr` at 0x27e18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x27e1c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x27e30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x27e38 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x27e40 | load | 4 | `abs 0x27ea8` | `-` |
| `ldr` at 0x27e44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27e48 | load | 4 | `abs 0x148be0` | `-` |
| `str` at 0x27e58 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x27e60 | load | 4 | `abs 0x27eac` | `-` |
| `ldr` at 0x27e64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x27e6c | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27e74 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x27e78 | load | 4 | `abs 0x27eb0` | `-` |
| `ldr` at 0x27e80 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x27e84 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0x27e9c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x27ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x27ea4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27ea4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27ea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27e14 | `(unknown callee) 0x2079c` | `0x101cb0` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27e2c | `(unknown callee) 0xd76d4` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |
| `bl` at 0x27e7c | `(unknown callee) 0x2079c` | `0x101cb0` | `0x148bdc` | `?` | `?` |
| `bl` at 0x27e94 | `(unknown callee) 0xd76d4` | `0x0` | `[[sp-24]+2]:1` | `0x2` | `[[sp-24]+2]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x27dd4 | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x27df0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x27e50 | `r0 < [0x148be0] (signed, 32-bit)` | `r0 >= [0x148be0] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27ea4 | `r0` | `[sp-28]` |

## `NL4R_SysComInt`

- `NL4R_SysComInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27eb4 to 0x27fa8.
  Established. Three byte stack stores at `sp -0x15`, `sp -0x16` and `sp -0x17` retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` (`strb` at 0x27ed0, `strb` at 0x27ed8, `strb` at 0x27ee0).
  A later unnamed `0x84094` call has unknown, unknown, `(r3 & 0xff)`, unknown (`bl` at 0x27f8c).
  Interrupt source and mask interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x27ec4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x27ed0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x27ed8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x27ee0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x27ee8 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x27eec | load | 4 | `abs 0x27f94` | `-` |
| `ldr` at 0x27ef0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27ef4 | load | 4 | `abs 0x148be0` | `-` |
| `ldr` at 0x27f00 | load | 4 | `abs 0x27f98` | `-` |
| `str` at 0x27f04 | store | 4 | `sp -0x28` | `0x101cc0` |
| `ldrb` at 0x27f08 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x27f0c | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x27f14 | load | 4 | `abs 0x27f9c` | `-` |
| `ldr` at 0x27f18 | load | 4 | `abs 0x27fa0` | `-` |
| `ldr` at 0x27f28 | load | 4 | `abs 0x27fa4` | `-` |
| `ldr` at 0x27f30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x27f3c | load | 4 | `abs 0x148bdc` | `-` |
| `str` at 0x27f44 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x27f4c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x27f50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x27f54 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x27f58 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x27f64 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x27f68 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x27f80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x27f84 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x27f90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x27f90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x27f90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x27f20 | `(unknown callee) 0x21eb8` | `0x1` | `0x101c80` | `0x127` | `0x0` |
| `bl` at 0x27f8c | `(unknown callee) 0x84094` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x27efc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x27f60 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x27f7c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x27f90 | `r0` | `?` |

## `TIOSysComInt`

- `TIOSysComInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54598 to 0x5469c.
  Established. An included byte absolute read targets `0x2b1620` (`ldrb` at 0x545d4). Later byte
  reads have unknown effective targets (`ldrb` at 0x54648, `ldrb` at 0x54678). The unnamed `0x84094`
  call carries unknown, unknown, `(r3 & 0xff)`, unknown (`bl` at 0x54680). Interrupt mapping and
  callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x545a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x545b4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x545bc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x545c4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x545cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x545d0 | load | 4 | `abs 0x54688` | `-` |
| `ldrb` at 0x545d4 | load | 1 | `abs 0x2b1620` | `-` |
| `ldr` at 0x545d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x545e4 | load | 4 | `abs 0x5468c` | `-` |
| `str` at 0x545e8 | store | 4 | `sp -0x28` | `0x10be9c` |
| `ldrb` at 0x545ec | load | 1 | `sp -0x15` | `-` |
| `str` at 0x545f0 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x545f8 | load | 4 | `abs 0x54690` | `-` |
| `ldr` at 0x545fc | load | 4 | `abs 0x54694` | `-` |
| `ldr` at 0x5460c | load | 4 | `abs 0x54698` | `-` |
| `ldr` at 0x54614 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54630 | load | 4 | `abs 0x1490d0` | `-` |
| `str` at 0x54638 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x54640 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x54644 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x54648 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x5464c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x54658 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x5465c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x54674 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x54678 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x54684 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x54684 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x54684 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54604 | `(unknown callee) 0x21eb8` | `0x1` | `0x10bdd4` | `0x241` | `0x0` |
| `bl` at 0x54680 | `(unknown callee) 0x84094` | `?` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x545e0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x54654 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x54670 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x54684 | `r0` | `?` |

## `syscom_process_outbound_request`

- `syscom_process_outbound_request`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x84dc8 to 0x85244.
  Established. The branch takes `ret(0x3dce4) == 0x1` to `0x84e7c` (`beq` at 0x84de8); its
  fall-through includes the unnamed `0xb74d4` call with `0x2b24a0`, `0x0`, `[sp-20]`, `[sp-120]`
  (`bl` at 0x84e74). A separate indirect call has unknown target and candidates `[sp-136]`,
  `sp-124`, `0x1`, unknown (`mov` at 0x84fd8). Request framing and callback identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x84dd8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x84e04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e08 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x84e1c | load | 4 | `abs 0x85220` | `-` |
| `str` at 0x84e48 | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x84e60 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x84e64 | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x84e6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e70 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x84e7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e80 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x84e8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x84e90 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x84e98 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x84e9c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x84ea4 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0x84eb0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x84eb4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x84eb8 | store | 4 | `sp -0x84` | `[sp-140]` |
| `ldr` at 0x84ebc | load | 4 | `sp -0x84` | `-` |
| `str` at 0x84ec8 | store | 4 | `sp -0x88` | `ret(0x83fe4)` |
| `ldr` at 0x84ecc | load | 4 | `sp -0x84` | `-` |
| `str` at 0x84ed8 | store | 4 | `sp -0x84` | `ret(0x8401c)` |
| `ldr` at 0x84edc | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x84ee0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84f04 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x84f0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84f18 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84f1c | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x84f2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x84f30 | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x84f34 | store | 1 | `sp -0x8d` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x84f3c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x84f4c | store | 1 | `sp -0x8d` | `(ret(0xb823c) & 0xff)` |
| `ldrb` at 0x84f50 | load | 1 | `sp -0x8d` | `-` |
| `strb` at 0x84f54 | store | 1 | `sp -0x7c` | `?` |
| `strb` at 0x84f5c | store | 1 | `sp -0x7b` | `0x0` |
| `ldr` at 0x84f60 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84f7c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84f80 | load | 4 | `abs 0x85228` | `-` |
| `ldr` at 0x84f90 | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x84f94 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x84fb8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x84fc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x84fc8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x84fe0 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x84fe4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x84ff4 | store | 4 | `sp -0x80` | `0x0` |
| `ldrb` at 0x84ffc | load | 1 | `sp -0x84` | `-` |
| `strb` at 0x85000 | store | 1 | `sp -0x7c` | `([sp-132]:1 & 0xff)` |
| `ldr` at 0x85004 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85008 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x85018 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x8501c | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x85020 | store | 1 | `sp -0x8e` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x85028 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x85038 | store | 1 | `sp -0x8e` | `(ret(0xb823c) & 0xff)` |
| `ldrb` at 0x8503c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x85040 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x85044 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x85060 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x85064 | load | 4 | `abs 0x8522c` | `-` |
| `ldr` at 0x85074 | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x85078 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x8509c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x850a8 | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x850ac | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x850d0 | load | 4 | `abs 0x2b24b4` | `-` |
| `str` at 0x850e0 | store | 4 | `sp -0x94` | `0x2` |
| `ldr` at 0x850e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x850ec | load | 4 | `sp -0x88` | `-` |
| `str` at 0x850fc | store | 4 | `sp -0x80` | `ret(0xbb1c4)` |
| `ldr` at 0x85100 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x85110 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x85124 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x8512c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x85138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8513c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x85150 | load | 4 | `abs 0x85230` | `-` |
| `ldr` at 0x85158 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x8515c | load | 4 | `abs 0x85234` | `-` |
| `ldr` at 0x85170 | load | 4 | `abs 0x85238` | `-` |
| `ldr` at 0x85184 | load | 4 | `abs 0x8523c` | `-` |
| `ldr` at 0x8518c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x85190 | load | 4 | `abs 0x85234` | `-` |
| `ldr` at 0x8519c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x851c0 | load | 4 | `abs 0x85240` | `-` |
| `str` at 0x851ec | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x85204 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x85208 | load | 4 | `abs 0x85224` | `-` |
| `ldr` at 0x85210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85214 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x8521c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8521c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8521c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x84ddc | `(unknown callee) 0x3dce4` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x84e00 | `(unknown callee) 0x69908` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x84e0c | `(unknown callee) 0x69d50` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x84e20 | `(unknown callee) 0x1b52c` | `sp-100` | `0x11290c` | `ret(0x69d50)` | `sp-100` |
| `bl` at 0x84e2c | `(unknown callee) 0x1b2bc` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x84e58 | `(unknown callee) 0x69b84` | `sp-120` | `sp-116` | `sp-100` | `ret(0x1b2bc)` |
| `bl` at 0x84e74 | `(unknown callee) 0xb74d4` | `0x2b24a0` | `0x0` | `[sp-20]` | `[sp-120]` |
| `bl` at 0x84ec0 | `(unknown callee) 0x83fe4` | `[sp-140]` | `?` | `?` | `[sp-140]` |
| `bl` at 0x84ed0 | `(unknown callee) 0x8401c` | `[sp-132]` | `?` | `?` | `ret(0x83fe4)` |
| `bl` at 0x84f44 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x84f68 | `(unknown callee) 0x82ebc` | `[sp-136]` | `0x8` | `?` | `0x0` |
| `bl` at 0x84f8c | `(unknown callee) 0x82f54` | `[sp-136]` | `0x112930` | `sp-124` | `0x1` |
| `mov` at 0x84fd8 | `(unknown callee) (unknown target)` | `[sp-136]` | `sp-124` | `0x1` | `?` |
| `bl` at 0x85030 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x8504c | `(unknown callee) 0x82ebc` | `[sp-136]` | `0x10` | `?` | `?` |
| `bl` at 0x85070 | `(unknown callee) 0x82f54` | `[sp-136]` | `0x112940` | `sp-124` | `0x2` |
| `bl` at 0x850f4 | `(unknown callee) 0xbb1c4` | `?` | `?` | `[sp-136]` | `sp-124` |
| `bl` at 0x85128 | `(unknown callee) 0x69908` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x85140 | `(unknown callee) 0x6a080` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x85154 | `(unknown callee) 0x1b52c` | `sp-100` | `0x112954` | `ret(0x6a080)` | `sp-100` |
| `bl` at 0x85174 | `(unknown callee) 0x1b364` | `sp-100` | `0x112968` | `[sp-128]` | `sp-100` |
| `bl` at 0x85188 | `(unknown callee) 0x1b364` | `sp-100` | `0x112980` | `[sp-128]` | `sp-100` |
| `bl` at 0x851a0 | `(unknown callee) 0x21704` | `[sp-128]` | `?` | `[sp-128]` | `0xfffff69e` |
| `bl` at 0x851b4 | `(unknown callee) 0x1b364` | `sp-100` | `ret(0x21704)` | `ret(0x21704)` | `sp-100` |
| `bl` at 0x851c4 | `(unknown callee) 0x1b364` | `sp-100` | `0x11298c` | `?` | `sp-100` |
| `bl` at 0x851d0 | `(unknown callee) 0x1b2bc` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x851fc | `(unknown callee) 0x69b84` | `sp-120` | `sp-116` | `sp-100` | `ret(0x1b2bc)` |
| `bl` at 0x85218 | `(unknown callee) 0xb74d4` | `0x2b24a0` | `0x0` | `[sp-20]` | `[sp-120]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x84de8 | `ret(0x3dce4) == 0x1` | `ret(0x3dce4) != 0x1` |
| `blt` at 0x84e88 | `[[sp-20]+24] < 0x0 (signed, 32-bit)` | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x84f14 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x84f28 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x84f74 | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `bne` at 0x84fec | `ret(unknown) != 0x1` | `ret(unknown) == 0x1` |
| `bne` at 0x85014 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x85058 | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `bne` at 0x85108 | `ret(0xbb1c4) != 0x2` | `ret(0xbb1c4) == 0x2` |
| `beq` at 0x85134 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bne` at 0x85164 | `[sp-128] != 0xfffff69e` | `[sp-128] == 0xfffff69e` |
| `beq` at 0x85198 | `[sp-128] == 0xfffff69e` | `[sp-128] != 0xfffff69e` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8521c | `r0` | `ret(0xb74d4)` |

## `syscom_process_outbound_event`

- `syscom_process_outbound_event`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x85244 to 0x85924.
  Established. Included byte stores with unknown effective targets write `0x0` and `0x1` (`strb` at
  0x8549c, `strb` at 0x8572c). The unnamed `0x1b394` call has `(sp-148 + [sp-152])`, unknown,
  `[sp-164]`, `(sp-148 + [sp-152])` (`bl` at 0x856b4). A later indirect call carries `[sp-176]`,
  `sp-148`, `[sp-152]`, unknown to an unknown target (`mov` at 0x8588c). Event record encoding and
  delivery effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x85254 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8525c | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x85260 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x85264 | load | 4 | `abs 0x2b24ac` | `-` |
| `ldr` at 0x85274 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x85278 | load | 4 | `abs 0x2b24ac` | `-` |
| `str` at 0x85284 | store | 4 | `sp -0xb0` | `ret(0x83fe4)` |
| `ldr` at 0x85288 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x8528c | load | 4 | `abs 0x2b24ac` | `-` |
| `str` at 0x85298 | store | 4 | `sp -0xac` | `ret(0x8401c)` |
| `ldr` at 0x8529c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x852a0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x852ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x852b0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x852c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x852cc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x852dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x852e0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x852ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x852f0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x85308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8530c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldrb` at 0x8531c | load | 1 | `sp -0xac` | `-` |
| `strb` at 0x85324 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x85328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8532c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x85330 | store | 4 | `sp -0xa8` | `[[sp-20]+28]` |
| `ldr` at 0x85334 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x8533c | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x85340 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85348 | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x8534c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85354 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x85358 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x8535c | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `ldr` at 0x85360 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85364 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x85374 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85378 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x8537c | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x85384 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x85390 | store | 4 | `sp -0xb4` | `ret(0xb823c)` |
| `ldr` at 0x85394 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x85398 | store | 4 | `sp -0xa8` | `[sp-180]` |
| `ldr` at 0x8539c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x853a4 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x853a8 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x853b0 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x853b4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x853bc | store | 1 | `sp -0x8d` | `?` |
| `ldrb` at 0x853c0 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x853c4 | store | 1 | `sp -0x8c` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x853cc | store | 1 | `sp -0x8b` | `0x0` |
| `str` at 0x853d4 | store | 4 | `sp -0x98` | `0xa` |
| `str` at 0x853dc | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x853e0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x853ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x853f0 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x853f4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85404 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85408 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x8541c | load | 1 | `(unknown)` | `-` |
| `str` at 0x85424 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x85428 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x85434 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x85440 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x85450 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x85460 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x85474 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85480 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x8549c | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x854a4 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x854a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x854ac | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x854c0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x854d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x854d4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x854e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x854ec | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x854f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x854f8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85500 | store | 4 | `sp -0xb8` | `ret(0xb823c)` |
| `ldr` at 0x85504 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x85508 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x85510 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85528 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85530 | store | 1 | `(unknown)` | `?` |
| `str` at 0x85538 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85540 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85558 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85560 | store | 1 | `(unknown)` | `?` |
| `str` at 0x85568 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x85570 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85588 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85590 | store | 1 | `(unknown)` | `?` |
| `str` at 0x85598 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x855a0 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x855b8 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x855bc | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x855c4 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x855cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x855d0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x855e4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x855e8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x855ec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x855f0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85604 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85610 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x85628 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x85634 | store | 1 | `(unknown)` | `?` |
| `str` at 0x8563c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x85640 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85644 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85658 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x85668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8566c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85680 | load | 4 | `(unknown)` | `-` |
| `str` at 0x85684 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x8568c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85690 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85698 | store | 4 | `sp -0xbc` | `ret(0xb823c)` |
| `ldr` at 0x856a0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x856ac | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x856b0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x856b8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x856bc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x856c4 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x856cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x856d0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x856e4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x856e8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x856ec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x856f0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85704 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85710 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x8572c | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x85734 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x85738 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8573c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85750 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x85760 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85764 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85778 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8577c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x85784 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85788 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85790 | store | 4 | `sp -0xc0` | `ret(0xb823c)` |
| `ldr` at 0x85798 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x857a4 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x857ac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x857b0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x857b8 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x857bc | load | 1 | `sp -0x8b` | `-` |
| `strb` at 0x857c4 | store | 1 | `sp -0x8b` | `?` |
| `ldr` at 0x857c8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x857d0 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x857d8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x857f4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x857f8 | load | 4 | `abs 0x85920` | `-` |
| `ldr` at 0x85800 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85808 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x8580c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x85830 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x85838 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x85844 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x85848 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8586c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x85878 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8587c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x85884 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85894 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x85898 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x858bc | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x858c8 | load | 4 | `abs 0x8591c` | `-` |
| `ldr` at 0x858cc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x858f0 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x858fc | load | 4 | `sp -0x98` | `-` |
| `str` at 0x85900 | store | 4 | `sp -0xc8` | `[sp-152]` |
| `ldr` at 0x85908 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8590c | load | 4 | `sp -0xb0` | `-` |
| `ldmdb` at 0x85918 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x85918 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x85918 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8527c | `(unknown callee) 0x83fe4` | `[0x2b24ac]` | `r1` | `r2` | `0x2b24a0` |
| `bl` at 0x85290 | `(unknown callee) 0x8401c` | `[0x2b24ac]` | `?` | `?` | `0x2b24a0` |
| `bl` at 0x8538c | `(unknown callee) 0xb823c` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x854fc | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85694 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x856b4 | `(unknown callee) 0x1b394` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x8578c | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x857a8 | `(unknown callee) 0x1b598` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x857e0 | `(unknown callee) 0x82ebc` | `[sp-176]` | `0x20` | `?` | `?` |
| `bl` at 0x85804 | `(unknown callee) 0x82f54` | `[sp-176]` | `0x112990` | `sp-148` | `[sp-152]` |
| `mov` at 0x8588c | `(unknown callee) (unknown target)` | `[sp-176]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x85914 | `(unknown callee) 0xbb1c4` | `?` | `?` | `[sp-176]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8526c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x852a8 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x852c0 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x852d8 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x852e8 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x85300 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x85318 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x85370 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x853e8 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x853fc | `C == 0` | `not (C == 0)` |
| `beq` at 0x85430 | `[sp-196] == 0x1` | `[sp-196] != 0x1` |
| `bgt` at 0x8543c | `[sp-196] > 0x1 (signed, 32-bit)` | `[sp-196] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x85448 | `[sp-196] == 0x0` | `[sp-196] != 0x0` |
| `beq` at 0x85458 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `bls` at 0x8546c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x854cc | `? != 0x0` | `? == 0x0` |
| `bls` at 0x855fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x85664 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x856fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x8575c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x857ec | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `bne` at 0x85840 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x85918 | `r0` | `?` |

## `syscom_process_outbound_response`

- `syscom_process_outbound_response`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x85924 to 0x86094.
  Established. The branch takes signed `[r0+24] < 0x0` to `0x85978` (`blt` at 0x85954). Included
  byte assignments at `sp -0x8f` and `sp -0x8c` retain `0x0` and `([[sp-20]+19]:1 & 0xff)`
  respectively (`strb` at 0x85a98, `strb` at 0x85b00). A later indirect call has unknown target and candidates `[sp-180]`,
  `[sp-176]`, `sp-148`, `[sp-152]` (`mov` at 0x85f70). Response codes and callback semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x85934 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8593c | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x85944 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x85948 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8594c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x85958 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8595c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x85964 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x85968 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x85970 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x8597c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x85980 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x85984 | store | 4 | `sp -0xb0` | `[sp-184]` |
| `ldr` at 0x85988 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x85994 | store | 4 | `sp -0xb4` | `ret(0x83fe4)` |
| `ldr` at 0x85998 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x859a4 | store | 4 | `sp -0xb0` | `ret(0x8401c)` |
| `ldr` at 0x859a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x859ac | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x859b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x859bc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x859c4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x859c8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x859d0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x859d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x859dc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x859e4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x859e8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x859ec | store | 4 | `sp -0xac` | `[sp-188]` |
| `ldr` at 0x859f0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x859fc | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85a00 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85a0c | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x85a14 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x85a18 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85a1c | load | 1 | `[sp-20] +0x13` | `-` |
| `ldr` at 0x85a2c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85a30 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x85a40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85a44 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x85a48 | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x85a50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x85a5c | store | 4 | `sp -0xc0` | `ret(0xb823c)` |
| `ldr` at 0x85a60 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x85a64 | store | 4 | `sp -0xa8` | `[sp-192]` |
| `ldr` at 0x85a68 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85a70 | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x85a74 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85a7c | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x85a80 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85a88 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x85a8c | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x85a90 | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x85a98 | store | 1 | `sp -0x8f` | `0x0` |
| `str` at 0x85aa0 | store | 4 | `sp -0x98` | `0x6` |
| `strb` at 0x85aac | store | 1 | `sp -0x93` | `?` |
| `strb` at 0x85ab4 | store | 1 | `sp -0x92` | `?` |
| `strb` at 0x85abc | store | 1 | `sp -0x91` | `?` |
| `strb` at 0x85ac4 | store | 1 | `sp -0x90` | `?` |
| `ldr` at 0x85ac8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85acc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85ad4 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x85ad8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85adc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85ae4 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x85ae8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85aec | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85af4 | store | 1 | `sp -0x8d` | `?` |
| `ldr` at 0x85af8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85afc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x85b00 | store | 1 | `sp -0x8c` | `([[sp-20]+19]:1 & 0xff)` |
| `str` at 0x85b08 | store | 4 | `sp -0x98` | `0x9` |
| `str` at 0x85b10 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x85b18 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x85b1c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x85b28 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x85b2c | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x85b30 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85b40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85b44 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85b58 | load | 1 | `(unknown)` | `-` |
| `str` at 0x85b60 | store | 4 | `sp -0xd0` | `?` |
| `ldr` at 0x85b64 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x85b70 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x85b7c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x85b8c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x85b9c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x85bb0 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85bbc | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x85bd8 | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x85be0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85be4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85be8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85bfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x85c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85c10 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85c24 | load | 4 | `(unknown)` | `-` |
| `str` at 0x85c28 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x85c30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85c34 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85c3c | store | 4 | `sp -0xc4` | `ret(0xb823c)` |
| `ldr` at 0x85c40 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x85c44 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x85c4c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85c64 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85c6c | store | 1 | `(unknown)` | `?` |
| `str` at 0x85c74 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85c7c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85c94 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85c9c | store | 1 | `(unknown)` | `?` |
| `str` at 0x85ca4 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x85cac | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85cc4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x85ccc | store | 1 | `(unknown)` | `?` |
| `str` at 0x85cd4 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x85cdc | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x85cf4 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x85cf8 | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x85d00 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x85d08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85d0c | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x85d20 | load | 2 | `(unknown)` | `-` |
| `str` at 0x85d24 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x85d28 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85d2c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85d40 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85d4c | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x85d64 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x85d70 | store | 1 | `(unknown)` | `?` |
| `str` at 0x85d78 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85d80 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85d94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x85da4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85da8 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85dbc | load | 4 | `(unknown)` | `-` |
| `str` at 0x85dc0 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x85dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85dcc | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85dd4 | store | 4 | `sp -0xc8` | `ret(0xb823c)` |
| `ldr` at 0x85ddc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85de8 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x85dec | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x85df4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85df8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85e00 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x85e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85e0c | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x85e20 | load | 2 | `(unknown)` | `-` |
| `str` at 0x85e24 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x85e28 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85e2c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85e40 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x85e4c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x85e68 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x85e70 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x85e74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85e78 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x85e8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x85e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85ea0 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x85eb4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x85eb8 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x85ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x85ec4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85ecc | store | 4 | `sp -0xcc` | `ret(0xb823c)` |
| `ldr` at 0x85ed4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85ee0 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x85ee8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85eec | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x85ef4 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x85ef8 | load | 1 | `sp -0x8f` | `-` |
| `strb` at 0x85f00 | store | 1 | `sp -0x8f` | `([sp-143]:1+1 & 0xff)` |
| `ldr` at 0x85f04 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x85f0c | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x85f14 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85f30 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85f34 | load | 4 | `abs 0x86088` | `-` |
| `ldr` at 0x85f3c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85f44 | load | 4 | `abs 0x8608c` | `-` |
| `ldr` at 0x85f48 | load | 4 | `abs 0x14a490` | `-` |
| `ldr` at 0x85f54 | load | 4 | `abs 0x8608c` | `-` |
| `ldr` at 0x85f5c | load | 4 | `abs 0x14a490` | `-` |
| `ldr` at 0x85f60 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85f64 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x85f68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x85f74 | load | 4 | `abs 0x86090` | `-` |
| `ldr` at 0x85f78 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85f9c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x85fa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x85fb0 | load | 4 | `abs 0x86090` | `-` |
| `ldr` at 0x85fb4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85fd8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x85fe4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x85fe8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x85ff0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x86000 | load | 4 | `abs 0x86090` | `-` |
| `ldr` at 0x86004 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x86028 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x86034 | load | 4 | `abs 0x86090` | `-` |
| `ldr` at 0x86038 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x8605c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0x86068 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x8606c | store | 4 | `sp -0xd4` | `[sp-152]` |
| `ldr` at 0x86074 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x86078 | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x86084 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x86084 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x86084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8598c | `(unknown callee) 0x83fe4` | `[sp-184]` | `r1` | `r2` | `?` |
| `bl` at 0x8599c | `(unknown callee) 0x8401c` | `[sp-176]` | `?` | `?` | `ret(0x83fe4)` |
| `bl` at 0x85a04 | `(unknown callee) 0x87058` | `[sp-180]` | `[sp-20]` | `?` | `[sp-188]` |
| `bl` at 0x85a58 | `(unknown callee) 0xb823c` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x85c38 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85dd0 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85df0 | `(unknown callee) 0x1b394` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x85ec8 | `(unknown callee) 0xb823c` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x85ee4 | `(unknown callee) 0x1b598` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x85f1c | `(unknown callee) 0x82ebc` | `[sp-180]` | `0x40` | `?` | `?` |
| `bl` at 0x85f40 | `(unknown callee) 0x82f54` | `[sp-180]` | `0x1129a0` | `sp-148` | `[sp-152]` |
| `mov` at 0x85f70 | `(unknown callee) (unknown target)` | `[sp-180]` | `[sp-176]` | `sp-148` | `[sp-152]` |
| `mov` at 0x85ff8 | `(unknown callee) (unknown target)` | `[sp-180]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x86080 | `(unknown callee) 0xbb1c4` | `?` | `?` | `[sp-180]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x85954 | `[r0+24] < 0x0 (signed, 32-bit)` | `[r0+24] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x859b4 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x859f8 | `[sp-188] != 0x15` | `[sp-188] == 0x15` |
| `beq` at 0x85a24 | `[[sp-20]+19]:1 == 0x0` | `[[sp-20]+19]:1 != 0x0` |
| `bne` at 0x85a3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x85b24 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x85b38 | `C == 0` | `not (C == 0)` |
| `beq` at 0x85b6c | `[sp-208] == 0x1` | `[sp-208] != 0x1` |
| `bgt` at 0x85b78 | `[sp-208] > 0x1 (signed, 32-bit)` | `[sp-208] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x85b84 | `[sp-208] == 0x0` | `[sp-208] != 0x0` |
| `beq` at 0x85b94 | `[sp-208] == 0x2` | `[sp-208] != 0x2` |
| `bls` at 0x85ba8 | `[sp-152]+5 <= 0x7f (unsigned, 32-bit)` | `[sp-152]+5 > 0x7f (unsigned, 32-bit)` |
| `bne` at 0x85c08 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x85d38 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x85da0 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x85e38 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x85e98 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x85f28 | `ret(0x82ebc) == 0x0` | `ret(0x82ebc) != 0x0` |
| `beq` at 0x85f50 | `[0x14a490] == 0x0` | `[0x14a490] != 0x0` |
| `bne` at 0x85fac | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x86084 | `r0` | `?` |

## `syscom_process_outbound_packet`

- `syscom_process_outbound_packet`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x86094 to 0x86110.
  Established. A byte read targets `r0 +0x16` (`ldrb` at 0x860ac). The branch takes `[r0+22]:1 == 0x1` to `0x86104`, whose unnamed `0x85924` call carries `r0`, `r1`, `r2`, `[r0+22]:1` (`beq` at
  0x860bc, `bl` at 0x86108). Other included calls target unnamed `0x84dc8` and `0x85244` (`bl` at
  0x860f0, `bl` at 0x860fc). Packet selector meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x860a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x860a8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x860ac | load | 1 | `r0 +0x16` | `-` |
| `str` at 0x860b0 | store | 4 | `sp -0x18` | `[r0+22]:1` |
| `ldr` at 0x860b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x860c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x860cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x860dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x860ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x860f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x86104 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8610c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8610c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8610c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x860f0 | `(unknown callee) 0x84dc8` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x860fc | `(unknown callee) 0x85244` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0x86108 | `(unknown callee) 0x85924` | `r0` | `r1` | `r2` | `[r0+22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x860bc | `[r0+22]:1 == 0x1` | `[r0+22]:1 != 0x1` |
| `bgt` at 0x860c8 | `[r0+22]:1 > 0x1 (signed, 32-bit)` | `[r0+22]:1 <= 0x1 (signed, 32-bit)` |
| `beq` at 0x860d4 | `[r0+22]:1 == 0x0` | `[r0+22]:1 != 0x0` |
| `beq` at 0x860e4 | `[r0+22]:1 == 0x2` | `[r0+22]:1 != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8610c | `r0` | `?` |

## `SysComEventDisable`

- `SysComEventDisable`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x862bc to 0x862e8.
  Established. A four-byte absolute assignment at `0x2b24ac` has an unknown stored value (`str` at
  0x862d0). The unnamed `0x1d41c` call uses `0x13`, `0x0`, `0x2b24a0`, unknown (`bl` at 0x862dc).
  The return reports `ret(0x1d41c)` (`ldmdb` at 0x862e0). Event-disable effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x862c8 | load | 4 | `abs 0x862e4` | `-` |
| `str` at 0x862d0 | store | 4 | `abs 0x2b24ac` | `?` |
| `ldmdb` at 0x862e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x862e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x862e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x862dc | `(unknown callee) 0x1d41c` | `0x13` | `0x0` | `0x2b24a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x862e0 | `r0` | `ret(0x1d41c)` |

## `SysComLogInit_2m`

- `SysComLogInit_2m`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafa2c to 0xafb60.
  Established. Included four-byte absolute stores at `0x14a498`, `0x14a49c` and `0x14a4a0` write
  `0x0` (`str` at 0xafa54, `str` at 0xafa60, `str` at 0xafa6c). The unnamed `0xba0a4` call carries
  `0x1000`, `0x0`, unknown, `[0x2b24b0]` (`bl` at 0xafaf8). A later four-byte store at an unknown
  effective target writes `ret(0xba0a4)` (`str` at 0xafb14). Log allocation and record format remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xafa3c | load | 4 | `abs 0xafb44` | `-` |
| `ldr` at 0xafa40 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafa4c | load | 4 | `abs 0xafb48` | `-` |
| `str` at 0xafa54 | store | 4 | `abs 0x14a498` | `0x0` |
| `ldr` at 0xafa58 | load | 4 | `abs 0xafb4c` | `-` |
| `str` at 0xafa60 | store | 4 | `abs 0x14a49c` | `0x0` |
| `ldr` at 0xafa64 | load | 4 | `abs 0xafb50` | `-` |
| `str` at 0xafa6c | store | 4 | `abs 0x14a4a0` | `0x0` |
| `ldr` at 0xafa74 | load | 4 | `abs 0xafb54` | `-` |
| `ldr` at 0xafa78 | load | 4 | `abs 0xafb44` | `-` |
| `ldr` at 0xafa7c | load | 4 | `abs 0x2b24b0` | `-` |
| `str` at 0xafa90 | store | 4 | `abs 0x14a7c4` | `ret(0x1b1b4)` |
| `ldr` at 0xafa94 | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafaa0 | load | 4 | `abs 0xafb58` | `-` |
| `ldr` at 0xafaa8 | load | 4 | `abs 0xafb48` | `-` |
| `str` at 0xafab0 | store | 4 | `abs 0x14a498` | `0x0` |
| `ldr` at 0xafab4 | load | 4 | `abs 0xafb4c` | `-` |
| `str` at 0xafabc | store | 4 | `abs 0x14a49c` | `0x0` |
| `ldr` at 0xafac0 | load | 4 | `abs 0xafb50` | `-` |
| `str` at 0xafac8 | store | 4 | `abs 0x14a4a0` | `0x0` |
| `str` at 0xafad4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xafad8 | load | 4 | `abs 0xafb44` | `-` |
| `ldr` at 0xafadc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafae0 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafb00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafb08 | load | 4 | `abs 0xafb54` | `-` |
| `ldr` at 0xafb0c | load | 4 | `abs 0x14a7c4` | `-` |
| `str` at 0xafb14 | store | 4 | `(unknown)` | `ret(0xba0a4)` |
| `ldr` at 0xafb18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafb24 | load | 4 | `abs 0xafb5c` | `-` |
| `ldr` at 0xafb28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafb30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xafb38 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xafb40 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xafb40 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafb40 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafb40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafa88 | `(unknown callee) 0x1b1b4` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xafaa4 | `(unknown callee) 0x2079c` | `0x12800c` | `?` | `?` | `[0x14a7c4]` |
| `bl` at 0xafaf8 | `(unknown callee) 0xba0a4` | `0x1000` | `0x0` | `?` | `[0x2b24b0]` |
| `bl` at 0xafb2c | `(unknown callee) 0x2079c` | `0x128034` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xafa48 | `[0x2b24b0] != 0x0` | `[0x2b24b0] == 0x0` |
| `bne` at 0xafa9c | `[0x14a7c4] != 0x0` | `[0x14a7c4] == 0x0` |
| `blt` at 0xafae8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xafb20 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafb40 | `r0` | `?` |

## `SysComLogInsert_2m`

- `SysComLogInsert_2m`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafb60 to 0xafbbc.
  Established. The branch takes `[0x14a7c4] != 0x0` to `0xafb90` (`bne` at 0xafb88). That route
  includes a four-byte read with unknown effective target and the unnamed `0xba1cc` call with
  unknown, `r1`, `r2`, unknown (`ldr` at 0xafba4, `bl` at 0xafbb0). Log insertion effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xafb70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xafb74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xafb78 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xafb7c | load | 4 | `abs 0xafbb8` | `-` |
| `ldr` at 0xafb80 | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafb90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafb98 | load | 4 | `abs 0xafbb8` | `-` |
| `ldr` at 0xafb9c | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafba4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafbac | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xafbb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafbb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafbb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafbb0 | `(unknown callee) 0xba1cc` | `?` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xafb88 | `[0x14a7c4] != 0x0` | `[0x14a7c4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafbb4 | `r0` | `?` |

## `SysComLogFetch_2m`

- `SysComLogFetch_2m`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafbbc to 0xafe10.
  Established. An included byte assignment at `ret(0x1b1b4) +0x0` writes `0x0` (`strb` at 0xafcf0).
  The unnamed `0x6a5c8` call carries `(ret(0x1b2bc) + [sp-36])`, `[[sp-24]+8]`, `[[sp-24]+16]`,
  `[[sp-24]+12]` (`bl` at 0xafd50). A later four-byte stack assignment at `sp -0x2c` retains
  `[sp-36]` (`str` at 0xafdcc). Fetch result layout and log ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xafbcc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xafbd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xafbe8 | store | 4 | `sp -0x18` | `ret(0x75194)` |
| `ldr` at 0xafbec | load | 4 | `abs 0xafe04` | `-` |
| `ldr` at 0xafbf0 | load | 4 | `abs 0x2b24b0` | `-` |
| `str` at 0xafc04 | store | 4 | `sp -0x28` | `ret(0x1b1b4)` |
| `str` at 0xafc0c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xafc10 | load | 4 | `abs 0xafe04` | `-` |
| `ldr` at 0xafc14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafc18 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafc28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafc30 | load | 4 | `abs 0xafe08` | `-` |
| `ldr` at 0xafc34 | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafc3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafc4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafc54 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xafc5c | store | 4 | `(unknown)` | `ret(0xba658)` |
| `ldr` at 0xafc60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xafc68 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xafc74 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xafc7c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xafc80 | load | 4 | `abs 0xafe04` | `-` |
| `ldr` at 0xafc84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafc88 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafc98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafca0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xafca8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafcb8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xafcc4 | store | 4 | `sp -0x20` | `(ret(0x1b2bc) + [sp-32])+80` |
| `ldr` at 0xafcc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xafcd0 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xafcd8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xafce4 | store | 4 | `sp -0x24` | `ret(0x1b1b4)` |
| `ldr` at 0xafce8 | load | 4 | `sp -0x24` | `-` |
| `strb` at 0xafcf0 | store | 1 | `ret(0x1b1b4) +0x0` | `0x0` |
| `str` at 0xafcf8 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xafd00 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xafd04 | load | 4 | `abs 0xafe04` | `-` |
| `ldr` at 0xafd08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafd0c | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafd1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafd38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafd3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafd44 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xafd48 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xafd4c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xafd54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd70 | load | 4 | `abs 0xafe0c` | `-` |
| `ldr` at 0xafd74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafd7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafd84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xafd8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xafd90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafd98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xafda0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xafda8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafdb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xafdb8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xafdc0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xafdc8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xafdcc | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xafdd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafddc | load | 4 | `abs 0xafe08` | `-` |
| `ldr` at 0xafde0 | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafde8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xafdf8 | store | 4 | `sp -0x2c` | `ret(0xba658)` |
| `ldr` at 0xafdfc | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xafe00 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafe00 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafe00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafbe0 | `(unknown callee) 0x75194` | `0x0` | `r1` | `r2` | `r0` |
| `bl` at 0xafbfc | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xafc44 | `(unknown callee) 0xba658` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xafcac | `(unknown callee) 0x1b2bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xafcdc | `(unknown callee) 0x1b1b4` | `?` | `?` | `?` | `[0x2b24b0]` |
| `bl` at 0xafd20 | `(unknown callee) 0x1b2bc` | `[sp-36]` | `?` | `?` | `[0x2b24b0]` |
| `bl` at 0xafd50 | `(unknown callee) 0x6a5c8` | `(ret(0x1b2bc) + [sp-36])` | `[[sp-24]+8]` | `[[sp-24]+16]` | `[[sp-24]+12]` |
| `bl` at 0xafd58 | `(unknown callee) 0x1b2bc` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xafd78 | `(unknown callee) 0x1b52c` | `(ret(0x1b2bc) + [sp-36])` | `0x12805c` | `[sp-28]` | `(ret(0x1b2bc) + [sp-36])` |
| `bl` at 0xafd94 | `(unknown callee) 0x1b364` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xafdac | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |
| `bl` at 0xafdc4 | `(unknown callee) 0x1b490` | `[sp-40]` | `?` | `?` | `[0x2b24b0]` |
| `bl` at 0xafdf0 | `(unknown callee) 0xba658` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xafbd8 | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xafc20 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xafc90 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xafd14 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafe00 | `r0` | `?` |

## `SysComLogReset_2m`

- `SysComLogReset_2m`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafe10 to 0xafea4.
  Established. The branch takes `Z == 0` to `0xafe7c`, falling through for `not (Z == 0)` (`bne` at
  0xafe2c). The unnamed `0xba184` calls have four unknown candidates and unknown, `r1`, unknown,
  unknown respectively (`bl` at 0xafe68, `bl` at 0xafe94). Four-byte reads at both sites have
  unknown effective targets (`ldr` at 0xafe64, `ldr` at 0xafe90). Reset selector and callee effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xafe20 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xafe24 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xafe34 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xafe38 | load | 4 | `abs 0xafe9c` | `-` |
| `ldr` at 0xafe3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafe40 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xafe50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xafe58 | load | 4 | `abs 0xafea0` | `-` |
| `ldr` at 0xafe5c | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafe64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xafe6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xafe74 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xafe7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafe84 | load | 4 | `abs 0xafea0` | `-` |
| `ldr` at 0xafe88 | load | 4 | `abs 0x14a7c4` | `-` |
| `ldr` at 0xafe90 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xafe98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xafe98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xafe98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafe68 | `(unknown callee) 0xba184` | `?` | `?` | `?` | `?` |
| `bl` at 0xafe94 | `(unknown callee) 0xba184` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xafe2c | `Z == 0` | `not (Z == 0)` |
| `blt` at 0xafe48 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xafe98 | `r0` | `?` |

## `SysCom_2m_Init`

- `SysCom_2m_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xafea4 to 0xb0000.
  Established. Included four-byte absolute assignments write `0xafa2c` at `0x14a494`, `0xafb60` at
  `0x14a498`, `0xafbbc` at `0x14a49c` and `0xafe10` at `0x14a4a0` (`str` at 0xaffb4, `str` at
  0xaffc0, `str` at 0xaffcc, `str` at 0xaffd8). The earlier branch takes `[sp-20] == 0x43` to
  `0xaffac` (`beq` at 0xafecc). Registration and selector meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xafec0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xafec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafedc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xafef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaff9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaffac | load | 4 | `abs 0xaffe0` | `-` |
| `ldr` at 0xaffb0 | load | 4 | `abs 0xaffe4` | `-` |
| `str` at 0xaffb4 | store | 4 | `abs 0x14a494` | `0xafa2c` |
| `ldr` at 0xaffb8 | load | 4 | `abs 0xaffe8` | `-` |
| `ldr` at 0xaffbc | load | 4 | `abs 0xaffec` | `-` |
| `str` at 0xaffc0 | store | 4 | `abs 0x14a498` | `0xafb60` |
| `ldr` at 0xaffc4 | load | 4 | `abs 0xafff0` | `-` |
| `ldr` at 0xaffc8 | load | 4 | `abs 0xafff4` | `-` |
| `str` at 0xaffcc | store | 4 | `abs 0x14a49c` | `0xafbbc` |
| `ldr` at 0xaffd0 | load | 4 | `abs 0xafff8` | `-` |
| `ldr` at 0xaffd4 | load | 4 | `abs 0xafffc` | `-` |
| `str` at 0xaffd8 | store | 4 | `abs 0x14a4a0` | `0xafe10` |
| `ldmdb` at 0xaffdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaffdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaffdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xafeb4 | `(unknown callee) 0x4f4d0` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xafecc | `[sp-20] == 0x43` | `[sp-20] != 0x43` |
| `beq` at 0xafed8 | `[sp-20] == 0x33` | `[sp-20] != 0x33` |
| `beq` at 0xafee4 | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0xafef0 | `[sp-20] == 0x34` | `[sp-20] != 0x34` |
| `beq` at 0xafefc | `[sp-20] == 0x41` | `[sp-20] != 0x41` |
| `beq` at 0xaff08 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0xaff14 | `[sp-20] == 0x39` | `[sp-20] != 0x39` |
| `beq` at 0xaff20 | `[sp-20] == 0x37` | `[sp-20] != 0x37` |
| `beq` at 0xaff2c | `[sp-20] == 0x38` | `[sp-20] != 0x38` |
| `beq` at 0xaff38 | `[sp-20] == 0x30` | `[sp-20] != 0x30` |
| `beq` at 0xaff44 | `[sp-20] == 0x51` | `[sp-20] != 0x51` |
| `beq` at 0xaff50 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0xaff5c | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0xaff68 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0xaff74 | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0xaff80 | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0xaff8c | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0xaff98 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0xaffa4 | `[sp-20] == 0x7c` | `[sp-20] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaffdc | `r0` | `ret(0x4f4d0)` |

## `SysComCaptureIn`

- `SysComCaptureIn`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb02d0 to 0xb093c.
  Established. Included stores with unknown effective targets write byte `([sp-32]:1 & 0xff)` and
  halfword `([sp-40]:2 & 0xffff)` (`strb` at 0xb06e4, `strh` at 0xb0764). The unnamed `0x1b394` call
  has unknown, `[sp-28]`, `[sp-32]`, unknown (`bl` at 0xb0668). The branch takes signed `[sp-24] < ?` to `0xb0410` (`blt` at 0xb0408). Capture layout and the comparison right operand remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb02e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb02e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb02e8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb02ec | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb02f0 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb02f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0318 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb034c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb0354 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb0358 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb035c | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0384 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb038c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb03b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb03c0 | load | 4 | `abs 0xb092c` | `-` |
| `ldrh` at 0xb03c4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb03c8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb03cc | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb03d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb03f4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb03fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0410 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb041c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb0420 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0448 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb047c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0484 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0488 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb04ac | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb04b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb04e0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xb04e8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xb0508 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xb050c | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0534 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb053c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0570 | load | 4 | `abs 0xb092c` | `-` |
| `ldr` at 0xb0574 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb059c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb05a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb05d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb05d8 | load | 4 | `abs 0xb092c` | `-` |
| `ldrh` at 0xb05dc | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb05e4 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xb05e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0614 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0620 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0624 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0648 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0650 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0660 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb0664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb066c | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0694 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb069c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb06c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb06d4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb06e0 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xb06e4 | store | 1 | `(unknown)` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xb06e8 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb06ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0710 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0718 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0744 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb074c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb0750 | load | 4 | `abs 0xb0930` | `-` |
| `ldrh` at 0xb0760 | load | 2 | `sp -0x28` | `-` |
| `strh` at 0xb0764 | store | 2 | `(unknown)` | `([sp-40]:2 & 0xffff)` |
| `ldr` at 0xb0768 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb076c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb0790 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb07c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb07cc | load | 4 | `abs 0xb0934` | `-` |
| `ldr` at 0xb07d0 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb07d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb07f8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb082c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0834 | load | 4 | `abs 0xb0934` | `-` |
| `ldr` at 0xb0838 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb0840 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xb0844 | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb0848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb086c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb0874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb08a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb08a8 | load | 4 | `abs 0xb0938` | `-` |
| `ldr` at 0xb08ac | load | 4 | `abs 0xb0928` | `-` |
| `ldr` at 0xb08b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb08d4 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb08dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb0908 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0910 | load | 4 | `abs 0xb0938` | `-` |
| `ldr` at 0xb0914 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb0918 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb0920 | store | 4 | `(unknown)` | `?` |
| `ldmdb` at 0xb0924 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb0924 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb0924 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb0668 | `(unknown callee) 0x1b394` | `?` | `[sp-28]` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb0408 | `[sp-24] < ? (signed, 32-bit)` | `[sp-24] >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb0924 | `r0` | `0x2b24a0` |

## `SysComCfg`

- `SysComCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb17b0 to 0xb1998.
  Established. A four-byte assignment at `r0 +0x0` retains `ret(0x1b1b4)` (`str` at 0xb17ec).
  Included unnamed `0x1b598` calls carry `[sp-28]`, `0x1281f4`, unknown, `[sp-36]` and `[sp-28]`,
  `0x128204`, unknown, `[sp-36]` (`bl` at 0xb18a0, `bl` at 0xb18b0). A later four-byte stack
  assignment at `sp -0x20` writes `[sp-32]+1` (`str` at 0xb1970). Configuration text and record
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb17c0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xb17c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb17c8 | load | 4 | `abs 0xb197c` | `-` |
| `ldr` at 0xb17cc | load | 4 | `abs 0x2b24b0` | `-` |
| `str` at 0xb17ec | store | 4 | `r0 +0x0` | `ret(0x1b1b4)` |
| `ldr` at 0xb17f0 | load | 4 | `r0 +0x0` | `-` |
| `str` at 0xb17f4 | store | 4 | `sp -0x1c` | `[r0]` |
| `str` at 0xb17fc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb1800 | load | 4 | `abs 0xb197c` | `-` |
| `ldr` at 0xb1804 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1808 | load | 4 | `abs 0x2b24b0` | `-` |
| `ldr` at 0xb1818 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb181c | load | 4 | `abs 0xb1980` | `-` |
| `ldr` at 0xb1820 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb1828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1838 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1840 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `ldr` at 0xb1844 | load | 4 | `abs 0xb197c` | `-` |
| `ldr` at 0xb1848 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb186c | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1874 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1878 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb187c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1888 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb1898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb189c | load | 4 | `abs 0xb1984` | `-` |
| `ldr` at 0xb18a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb18ac | load | 4 | `abs 0xb1988` | `-` |
| `ldr` at 0xb18b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb18c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb18cc | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `ldr` at 0xb18d0 | load | 4 | `abs 0xb197c` | `-` |
| `ldr` at 0xb18d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb18f8 | load | 4 | `abs 0x2b24b4` | `-` |
| `ldr` at 0xb1900 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb1904 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb1908 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1914 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb1924 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1928 | load | 4 | `abs 0xb198c` | `-` |
| `ldr` at 0xb1934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1938 | load | 4 | `abs 0xb1990` | `-` |
| `ldr` at 0xb1940 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb1944 | load | 4 | `abs 0xb1994` | `-` |
| `ldr` at 0xb194c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb195c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb1964 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1b2bc))` |
| `ldr` at 0xb1968 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb1970 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xb1978 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb1978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb1978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb1978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb17e4 | `(unknown callee) 0x1b1b4` | `?` | `r1` | `[0x2b24b0]` | `?` |
| `bl` at 0xb1824 | `(unknown callee) 0x1b52c` | `?` | `0x1281e8` | `?` | `[0x2b24b0]` |
| `bl` at 0xb182c | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb18a0 | `(unknown callee) 0x1b598` | `[sp-28]` | `0x1281f4` | `?` | `[sp-36]` |
| `bl` at 0xb18b0 | `(unknown callee) 0x1b598` | `[sp-28]` | `0x128204` | `?` | `[sp-36]` |
| `bl` at 0xb18b8 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0xb192c | `(unknown callee) 0x1b598` | `[sp-28]` | `0x128214` | `?` | `[sp-40]` |
| `bl` at 0xb193c | `(unknown callee) 0x1b598` | `[sp-28]` | `0x128224` | `?` | `[sp-40]` |
| `bl` at 0xb1948 | `(unknown callee) 0x1b364` | `[sp-28]` | `0x1281d0` | `?` | `?` |
| `bl` at 0xb1950 | `(unknown callee) 0x1b2bc` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb1810 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0xb1884 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0xb1890 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |
| `beq` at 0xb1910 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `beq` at 0xb191c | `[sp-40] == 0x1` | `[sp-40] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb1978 | `r0` | `?` |
