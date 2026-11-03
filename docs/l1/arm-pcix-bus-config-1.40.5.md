# ARM PCI-X bus configuration and probe callers in 1.40.5

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

[Fact sheets and full listings](disasm/arm-1.40.5-package4/pcix-bus-config-facts.txt) retain all tool sections.

## `pcixInt`

- `pcixInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x598a8 to 0x59b94.
  Established. The entry reads four bytes at absolute `0x1490e0` (`ldr` at 0x598dc). A nonzero value
  takes the branch to `0x598ec` (`bne` at 0x598e4). The unnamed `0x5bce4` call carries `[sp-20]`,
  `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at 0x5991c); its nonzero result takes the branch to
  `0x599d8`, where a byte read targets `ret(0x5bce4) +0x6` (`bne` at 0x59930, `ldrb` at 0x599dc).
  Later calls include unnamed `0x5acd8` and `0x5c894`, with candidates retained below (`bl` at
  0x59a94, `bl` at 0x59afc). Interrupt-source and acknowledgement meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x598b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x598c4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x598cc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x598d4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x598d8 | load | 4 | `abs 0x59b5c` | `-` |
| `ldr` at 0x598dc | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x598ec | load | 4 | `abs 0x59b60` | `-` |
| `ldr` at 0x598f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59904 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5990c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x59910 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x59914 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59924 | store | 4 | `sp -0x20` | `ret(0x5bce4)` |
| `ldr` at 0x59928 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59934 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5993c | load | 4 | `abs 0x59b64` | `-` |
| `ldr` at 0x59940 | load | 4 | `abs 0x59b68` | `-` |
| `str` at 0x5996c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x59970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59978 | load | 4 | `abs 0x59b64` | `-` |
| `ldr` at 0x5997c | load | 4 | `abs 0x59b68` | `-` |
| `ldr` at 0x59988 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59998 | load | 4 | `abs 0x59b6c` | `-` |
| `str` at 0x5999c | store | 4 | `(unknown)` | `0x10c628` |
| `str` at 0x599a0 | store | 4 | `(unknown)` | `ret(0x1b598)` |
| `ldrb` at 0x599a4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x599a8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x599b0 | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x599b4 | load | 4 | `abs 0x59b74` | `-` |
| `ldrb` at 0x599c0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x599c4 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x599c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x599d8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x599dc | load | 1 | `ret(0x5bce4) +0x6` | `-` |
| `ldr` at 0x599ec | load | 4 | `abs 0x59b78` | `-` |
| `str` at 0x599f0 | store | 4 | `sp -0x38` | `0x10c658` |
| `ldr` at 0x599f4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x599f8 | load | 1 | `ret(0x5bce4) +0x0` | `-` |
| `str` at 0x599fc | store | 4 | `sp -0x34` | `[ret(0x5bce4)]:1` |
| `ldr` at 0x59a00 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59a04 | load | 1 | `ret(0x5bce4) +0x1` | `-` |
| `str` at 0x59a08 | store | 4 | `sp -0x30` | `[ret(0x5bce4)+1]:1` |
| `ldr` at 0x59a10 | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x59a14 | load | 4 | `abs 0x59b7c` | `-` |
| `ldrb` at 0x59a20 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59a24 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59a28 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x59a38 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x59a48 | load | 1 | `sp -0x17` | `-` |
| `ldr` at 0x59a58 | load | 4 | `abs 0x59b80` | `-` |
| `str` at 0x59a5c | store | 4 | `sp -0x38` | `0x10c684` |
| `ldr` at 0x59a60 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59a64 | load | 1 | `ret(0x5bce4) +0x0` | `-` |
| `str` at 0x59a68 | store | 4 | `sp -0x34` | `[ret(0x5bce4)]:1` |
| `ldr` at 0x59a6c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59a70 | load | 1 | `ret(0x5bce4) +0x1` | `-` |
| `str` at 0x59a74 | store | 4 | `sp -0x30` | `[ret(0x5bce4)+1]:1` |
| `ldr` at 0x59a7c | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x59a80 | load | 4 | `abs 0x59b84` | `-` |
| `ldr` at 0x59a8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59a90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59a9c | store | 4 | `sp -0x24` | `ret(0x5acd8)` |
| `ldr` at 0x59aa0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59aac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59ab8 | load | 4 | `abs 0x59b88` | `-` |
| `str` at 0x59abc | store | 4 | `sp -0x38` | `0x10c6a4` |
| `ldr` at 0x59ac0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59ac4 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x59ac8 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x59acc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59ad0 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x59ad4 | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x59ad8 | store | 4 | `sp -0x2c` | `ret(0x21a38)` |
| `ldr` at 0x59ae0 | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x59af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59af4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59b04 | store | 4 | `sp -0x24` | `ret(0x5c894)` |
| `ldr` at 0x59b08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59b14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59b20 | load | 4 | `abs 0x59b8c` | `-` |
| `str` at 0x59b24 | store | 4 | `sp -0x38` | `0x10c6cc` |
| `ldr` at 0x59b28 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59b2c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0x59b30 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0x59b34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59b38 | load | 1 | `[sp-32] +0x1` | `-` |
| `str` at 0x59b3c | store | 4 | `sp -0x30` | `[[sp-32]+1]:1` |
| `str` at 0x59b40 | store | 4 | `sp -0x2c` | `ret(0x21a38)` |
| `ldr` at 0x59b48 | load | 4 | `abs 0x59b70` | `-` |
| `ldr` at 0x59b4c | load | 4 | `abs 0x59b90` | `-` |
| `ldmdb` at 0x59b58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59b58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59b58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5991c | `(unknown callee) 0x5bce4` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x59944 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59950 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x59980 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59990 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x599bc | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0x31b` | `0x0` |
| `bl` at 0x599d0 | `(unknown callee) 0xd768c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59a1c | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0x326` | `0x0` |
| `bl` at 0x59a30 | `(unknown callee) 0xd768c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59a88 | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0x331` | `0x0` |
| `bl` at 0x59a94 | `(unknown callee) 0x5acd8` | `[sp-20]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x59ab0 | `(unknown callee) 0x21a38` | `ret(0x5acd8)` | `?` | `?` | `ret(0x5acd8)` |
| `bl` at 0x59aec | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0x338` | `0x0` |
| `bl` at 0x59afc | `(unknown callee) 0x5c894` | `[sp-20]` | `[sp-32]` | `0x1` | `?` |
| `bl` at 0x59b18 | `(unknown callee) 0x21a38` | `ret(0x5c894)` | `?` | `?` | `ret(0x5c894)` |
| `bl` at 0x59b54 | `(unknown callee) 0x21eb8` | `0x1` | `0x10c56c` | `0x33f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x598e4 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `bne` at 0x59930 | `ret(0x5bce4) != 0x0` | `ret(0x5bce4) == 0x0` |
| `beq` at 0x599e8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59a44 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x59a54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x59aa8 | `ret(0x5acd8) == 0x0` | `ret(0x5acd8) != 0x0` |
| `beq` at 0x59b10 | `ret(0x5c894) == 0x0` | `ret(0x5c894) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59b58 | `r0` | `?` |

## `pcixPresentInt`

- `pcixPresentInt`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59b94 to 0x59d48.
  Established. An unnamed call to `0x5be80` carries `r0`, `(r1 & 0xff)`, unknown, `(r1 & 0xff)`
  (`bl` at 0x59c04). A byte read then targets `ret(0x5be80) +0x6` (`ldrb` at 0x59c14). The branch to
  `0x59cac` tests an unknown expression for zero (`beq` at 0x59c34). Separate calls to unnamed
  `0x2079c` use first candidates `0x10c6f4` and `0x10c72c`, each followed by `ret(0x1b598)` and the
  remaining candidates shown below (`bl` at 0x59ca4, `bl` at 0x59d30). Presence-event semantics are
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59ba4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x59bb0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x59bb8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x59bc0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x59bc8 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0x59bcc | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x59bd0 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x59bd8 | store | 1 | `sp -0x21` | `?` |
| `ldrb` at 0x59bdc | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x59bf0 | store | 1 | `sp -0x22` | `0x1` |
| `ldrb` at 0x59bf4 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59bf8 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x59bfc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59c0c | store | 4 | `sp -0x1c` | `ret(0x5be80)` |
| `ldr` at 0x59c10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x59c14 | load | 1 | `ret(0x5be80) +0x6` | `-` |
| `ldrb` at 0x59c28 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c40 | load | 4 | `abs 0x59d38` | `-` |
| `ldr` at 0x59c44 | load | 4 | `abs 0x59d3c` | `-` |
| `str` at 0x59c70 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x59c74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59c7c | load | 4 | `abs 0x59d38` | `-` |
| `ldr` at 0x59c80 | load | 4 | `abs 0x59d3c` | `-` |
| `ldr` at 0x59c8c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59c9c | load | 4 | `abs 0x59d40` | `-` |
| `ldrb` at 0x59cac | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59cc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59ccc | load | 4 | `abs 0x59d38` | `-` |
| `ldr` at 0x59cd0 | load | 4 | `abs 0x59d3c` | `-` |
| `str` at 0x59cfc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x59d00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59d08 | load | 4 | `abs 0x59d38` | `-` |
| `ldr` at 0x59d0c | load | 4 | `abs 0x59d3c` | `-` |
| `ldr` at 0x59d18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59d28 | load | 4 | `abs 0x59d44` | `-` |
| `ldmdb` at 0x59d34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59d34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59d34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59c04 | `(unknown callee) 0x5be80` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x59c48 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59c54 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x59c84 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59c94 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x59ca4 | `(unknown callee) 0x2079c` | `0x10c6f4` | `ret(0x1b598)` | `?` | `ret(0x1b598)` |
| `bl` at 0x59cd4 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59ce0 | `(unknown callee) 0x1b2bc` | `ret(0x4f9a4)` | `?` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x59d10 | `(unknown callee) 0x4f9a4` | `[sp-20]` | `0x0` | `0x10c5b8` | `0x10c5c0` |
| `bl` at 0x59d20 | `(unknown callee) 0x1b598` | `[sp-40]` | `ret(0x4f9a4)` | `?` | `ret(0x4f9a4)` |
| `bl` at 0x59d30 | `(unknown callee) 0x2079c` | `0x10c72c` | `ret(0x1b598)` | `?` | `ret(0x1b598)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x59be8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59c20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59c34 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59cc0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59d34 | `r0` | `?` |

## `pcixSetBusSpeed`

- `pcixSetBusSpeed`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59d48 to 0x59e60.
  Established. A nonzero `[0x1490e0]` takes the branch to `0x59d80` (`bne` at 0x59d70). The loop
  includes a four-byte read at `[sp-24] +0x8`, while its signed-less predicate has unknown operands
  (`ldr` at 0x59db4, `blt` at 0x59dbc). The unnamed `0x5c170` call has `[sp-20]` followed by three
  unknown candidates (`bl` at 0x59e0c); its zero result takes the branch to `0x59e38` (`beq` at
  0x59e20). Bus-speed units and callee effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59d58 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59d60 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x59d64 | load | 4 | `abs 0x59e58` | `-` |
| `ldr` at 0x59d68 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x59d78 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x59d80 | load | 4 | `abs 0x59e5c` | `-` |
| `ldr` at 0x59d84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59d98 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59da0 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x59da8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x59dac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59db0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59db4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x59dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59dc8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59dd0 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldrb` at 0x59dd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59de4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59de8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59dfc | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x59e04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59e14 | store | 4 | `sp -0x20` | `ret(0x5c170)` |
| `ldr` at 0x59e18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59e24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59e30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x59e34 | store | 4 | `sp -0x1c` | `ret(0x5c170)` |
| `ldr` at 0x59e38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59e40 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x59e48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x59e4c | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x59e50 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x59e54 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59e54 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59e54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59e0c | `(unknown callee) 0x5c170` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59d70 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `blt` at 0x59dbc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x59de0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x59e20 | `ret(0x5c170) == 0x0` | `ret(0x5c170) != 0x0` |
| `bne` at 0x59e2c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59e54 | `r0` | `?` |

## `pcixSetM66EN`

- `pcixSetM66EN`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59e60 to 0x59fe8.
  Established. The call to unnamed `0x5cb28` carries `0xa`, `[sp-28]`, unknown, `[sp-48]` (`bl` at
  0x59f78). Two prior branches skip it when `[sp-24] != 0x1` or `[sp-48] != 0x0`, respectively
  (`bne` at 0x59f64, `bne` at 0x59f70). The next unnamed `0x5c920` call uses `[sp-20]`, unknown,
  `[sp-24]`, `[sp-48]+1` (`bl` at 0x59f94); its result is retained in a four-byte stack assignment
  (`str` at 0x59f9c). Electrical M66EN behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59e70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x59e74 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x59e7c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x59e84 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x59e88 | load | 4 | `abs 0x59fe0` | `-` |
| `ldr` at 0x59e8c | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x59e9c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x59ea4 | load | 4 | `abs 0x59fe4` | `-` |
| `ldr` at 0x59ea8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59ebc | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x59ec4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x59ecc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x59ed0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59ed4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59ed8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x59ee8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59eec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59ef4 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x59efc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x59f00 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59f04 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59f18 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59f1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59f2c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x59f30 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59f3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x59f40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59f48 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x59f50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x59f5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x59f68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x59f7c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x59f84 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x59f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59f8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x59f90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x59f9c | store | 4 | `sp -0x28` | `ret(0x5c920)` |
| `ldr` at 0x59fa0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x59fac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x59fb8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x59fbc | store | 4 | `sp -0x24` | `ret(0x5c920)` |
| `ldr` at 0x59fc0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x59fc8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x59fd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59fd4 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0x59fd8 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x59fdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59fdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x59f78 | `(unknown callee) 0x5cb28` | `0xa` | `[sp-28]` | `?` | `[sp-48]` |
| `bl` at 0x59f94 | `(unknown callee) 0x5c920` | `[sp-20]` | `?` | `[sp-24]` | `[sp-48]+1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x59e94 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `blt` at 0x59ee0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x59f10 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59f28 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x59f38 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x59f58 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x59f64 | `[sp-24] != 0x1` | `[sp-24] == 0x1` |
| `bne` at 0x59f70 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `beq` at 0x59fa8 | `ret(0x5c920) == 0x0` | `ret(0x5c920) != 0x0` |
| `bne` at 0x59fb4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59fdc | `r0` | `?` |

## `pcixBusPopulation`

- `pcixBusPopulation`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x59fe8 to 0x5a144.
  Established. The unnamed `0x5b944` call carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl`
  at 0x5a058); its nonzero result takes the branch to `0x5a07c`, followed by a byte read at
  `ret(0x5b944) +0x1` (`bne` at 0x5a06c, `ldrb` at 0x5a080). A second unnamed call, `0x5bf68`, has
  `[sp-20]`, unknown, `sp-40`, unknown (`bl` at 0x5a0cc); its zero result takes the branch to
  `0x5a0f0` (`beq` at 0x5a0e0). Population-count meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x59ff8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a000 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x5a008 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x5a00c | load | 4 | `abs 0x5a13c` | `-` |
| `ldr` at 0x5a010 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x5a020 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x5a028 | load | 4 | `abs 0x5a140` | `-` |
| `ldr` at 0x5a02c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a040 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a048 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x5a04c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5a050 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5a060 | store | 4 | `sp -0x20` | `ret(0x5b944)` |
| `ldr` at 0x5a064 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5a074 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x5a07c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5a080 | load | 1 | `ret(0x5b944) +0x1` | `-` |
| `str` at 0x5a084 | store | 4 | `sp -0x34` | `[ret(0x5b944)+1]:1` |
| `ldr` at 0x5a088 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5a08c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5a090 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5a094 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5a09c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5a0ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a0b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x5a0b8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x5a0c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5a0d4 | store | 4 | `sp -0x30` | `ret(0x5bf68)` |
| `ldr` at 0x5a0d8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x5a0e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5a0e8 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldrb` at 0x5a0fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5a110 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5a118 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x5a11c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x5a124 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x5a12c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x5a130 | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0x5a134 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x5a138 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a138 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a138 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5a058 | `(unknown callee) 0x5b944` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x5a0cc | `(unknown callee) 0x5bf68` | `[sp-20]` | `?` | `sp-40` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a018 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `bne` at 0x5a06c | `ret(0x5b944) != 0x0` | `ret(0x5b944) == 0x0` |
| `blt` at 0x5a0a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5a0e0 | `ret(0x5bf68) == 0x0` | `ret(0x5bf68) != 0x0` |
| `beq` at 0x5a10c | `? == 0x3` | `? != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a138 | `r0` | `?` |

## `pcixOverrideSpeed`

- `pcixOverrideSpeed`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a144 to 0x5a2b0.
  Established. A byte assignment at `sp -0x16` retains `([sp-22]:1-1 & 0xff)` (`strb` at 0x5a258).
  The branch takes `([sp-22]:1-1 & 0xff) <= [[sp-28]+8]` (signed, 32-bit) to `0x5a27c`
  (`ble` at 0x5a26c). The unnamed call there targets `0x1d41c` with `([sp-22]:1-1 & 0xff)+170`,
  `[sp-30]:1`, `[sp-30]:1`, and the same first expression (`bl` at 0x5a290). Separate four-byte
  stack assignments include `0xfffff703` (`str` at 0x5a1a4, `str` at 0x5a248, `str` at 0x5a274).
  Override storage and speed encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a154 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a160 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x5a168 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x5a170 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x5a174 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x5a178 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0x5a180 | store | 1 | `sp -0x1e` | `?` |
| `ldr` at 0x5a184 | load | 4 | `abs 0x5a2a4` | `-` |
| `ldr` at 0x5a188 | load | 4 | `abs 0x1490e0` | `-` |
| `ldrb` at 0x5a194 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5a1a0 | load | 4 | `abs 0x5a2a8` | `-` |
| `str` at 0x5a1a4 | store | 4 | `sp -0x28` | `0xfffff703` |
| `str` at 0x5a1b0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x5a1b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5a1b8 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x5a1c0 | load | 4 | `abs 0x5a2ac` | `-` |
| `ldr` at 0x5a1c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a1d8 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a1e0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x5a1e4 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x5a1f4 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5a1f8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5a1fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a200 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x5a210 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5a218 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x5a228 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5a230 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldrb` at 0x5a238 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5a244 | load | 4 | `abs 0x5a2a8` | `-` |
| `str` at 0x5a248 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x5a250 | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x5a258 | store | 1 | `sp -0x16` | `([sp-22]:1-1 & 0xff)` |
| `ldrb` at 0x5a25c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5a260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a264 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x5a270 | load | 4 | `abs 0x5a2a8` | `-` |
| `str` at 0x5a274 | store | 4 | `sp -0x24` | `0xfffff703` |
| `ldrb` at 0x5a27c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x5a284 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x5a298 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x5a29c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5a2a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a2a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a2a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5a224 | `(unknown callee) 0x1d41c` | `?` | `[sp-30]:1` | `[sp-30]:1` | `?` |
| `bl` at 0x5a290 | `(unknown callee) 0x1d41c` | `([sp-22]:1-1 & 0xff)+170` | `[sp-30]:1` | `[sp-30]:1` | `([sp-22]:1-1 & 0xff)+170` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a190 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |
| `beq` at 0x5a19c | `(r2 & 0xff) == 0xff` | `(r2 & 0xff) != 0xff` |
| `bne` at 0x5a1ec | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `blt` at 0x5a208 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5a240 | `[sp-22]:1 != 0x0` | `[sp-22]:1 == 0x0` |
| `ble` at 0x5a26c | `([sp-22]:1-1 & 0xff) <= [[sp-28]+8] (signed, 32-bit)` | `([sp-22]:1-1 & 0xff) > [[sp-28]+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a2a0 | `r0` | `?` |

## `pcixBusPresent`

- `pcixBusPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a2b0 to 0x5a32c.
  Established. The branch takes `[0x1490e0] != 0x0` to `0x5a2e0` (`bne` at 0x5a2d8). That path
  includes a four-byte read at `[sp-28] +0x14` and a byte store of `0x1` at an unresolved effective
  address (`ldr` at 0x5a310, `strb` at 0x5a31c). The exit preserves entry `r0` (`ldmdb` at 0x5a320).
  Bus-presence field meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a2c0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a2c8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5a2cc | load | 4 | `abs 0x5a324` | `-` |
| `ldr` at 0x5a2d0 | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x5a2e0 | load | 4 | `abs 0x5a328` | `-` |
| `ldr` at 0x5a2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a2f8 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a300 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a304 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5a308 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5a310 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5a31c | store | 1 | `(unknown)` | `0x1` |
| `ldmdb` at 0x5a320 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a320 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a320 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a2d8 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a320 | `r0` | `r0` |

## `pcixBusNotPresent`

- `pcixBusNotPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a32c to 0x5a3a8.
  Established. Four bytes are read from absolute `0x1490e0` (`ldr` at 0x5a34c). The branch takes nonzero `[0x1490e0]` to `0x5a35c` and falls through for zero (`bne` at
  0x5a354). The subsequent byte assignment stores
  `0x0` at an unknown effective address (`strb` at 0x5a398). No calls appear in this span. Physical
  absence and pointee semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a33c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a344 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5a348 | load | 4 | `abs 0x5a3a0` | `-` |
| `ldr` at 0x5a34c | load | 4 | `abs 0x1490e0` | `-` |
| `ldr` at 0x5a35c | load | 4 | `abs 0x5a3a4` | `-` |
| `ldr` at 0x5a360 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a374 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a37c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a380 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5a384 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5a38c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x5a398 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x5a39c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a39c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a39c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a354 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a39c | `r0` | `r0` |

## `pcixBusIsPresent`

- `pcixBusIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a3a8 to 0x5a460.
  Established. The first absolute data read targets `0x1490dc` (`ldr` at 0x5a3dc). A later branch
  takes nonzero `[0x1490e0]` to `0x5a404` (`bne` at 0x5a3f4). That arm includes a byte read at an
  unresolved address and a four-byte stack store whose value is unknown (`ldrb` at 0x5a43c, `str` at
  0x5a44c). The return value also remains unknown (`ldmdb` at 0x5a454). Presence interpretation is
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a3b8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a3c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5a3c4 | load | 4 | `abs 0x5a458` | `-` |
| `ldr` at 0x5a3c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a3dc | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a3e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a3e8 | load | 4 | `abs 0x5a45c` | `-` |
| `ldr` at 0x5a3ec | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x5a3fc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5a404 | load | 4 | `abs 0x5a458` | `-` |
| `ldr` at 0x5a408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a41c | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a424 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a428 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5a42c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5a434 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x5a43c | load | 1 | `(unknown)` | `-` |
| `str` at 0x5a44c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5a450 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5a454 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a454 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a454 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a3f4 | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a454 | `r0` | `?` |

## `pcixSlotIsPresent`

- `pcixSlotIsPresent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5a460 to 0x5a52c.
  Established. This span reads four bytes at absolute `0x1490dc` and `0x1490e0` (`ldr` at
  0x5a494, `ldr` at 0x5a4a4). Nonzero `[0x1490e0]` takes the branch to `0x5a4bc` (`bne` at 0x5a4ac). The
  following path includes four-byte reads at `[sp-28] +0xc` and `+0x14`, with intervening and
  subsequent byte reads at unknown effective addresses (`ldr` at 0x5a4f0, `ldr` at 0x5a500, `ldrb`
  at 0x5a4f8, `ldrb` at 0x5a508). Slot-to-bus mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5a470 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x5a478 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x5a47c | load | 4 | `abs 0x5a524` | `-` |
| `ldr` at 0x5a480 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a494 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a49c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a4a0 | load | 4 | `abs 0x5a528` | `-` |
| `ldr` at 0x5a4a4 | load | 4 | `abs 0x1490e0` | `-` |
| `str` at 0x5a4b4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x5a4bc | load | 4 | `abs 0x5a524` | `-` |
| `ldr` at 0x5a4c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5a4d4 | load | 4 | `abs 0x1490dc` | `-` |
| `str` at 0x5a4dc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5a4e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5a4e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x5a4e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x5a4f0 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0x5a4f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5a500 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldrb` at 0x5a508 | load | 1 | `(unknown)` | `-` |
| `str` at 0x5a518 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5a51c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5a520 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5a520 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5a520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5a4ac | `[0x1490e0] != 0x0` | `[0x1490e0] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5a520 | `r0` | `?` |

## `pcixcmdInit`

- `pcixcmdInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa5430 to 0xa54b0.
  Established. A nonzero `[0x14a688]` takes the branch past the call and stores to `0xa5488` (`bne`
  at 0xa5448). The fall-through calls unnamed `0xbc05c` with `0x125d3c`, `0x19`, `r2`, `[0x14a688]`
  (`bl` at 0xa5454). Four-byte absolute assignments then write `0x5839c` to `0x14a550`, `0x586a4` to
  `0x14a554`, `0x58964` to `0x14a558`, and `0x1` to `0x14a688` (`str` at 0xa5460, `str` at 0xa546c,
  `str` at 0xa5478, `str` at 0xa5484). Registration effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa543c | load | 4 | `abs 0xa5490` | `-` |
| `ldr` at 0xa5440 | load | 4 | `abs 0x14a688` | `-` |
| `ldr` at 0xa544c | load | 4 | `abs 0xa5494` | `-` |
| `ldr` at 0xa5458 | load | 4 | `abs 0xa5498` | `-` |
| `ldr` at 0xa545c | load | 4 | `abs 0xa549c` | `-` |
| `str` at 0xa5460 | store | 4 | `abs 0x14a550` | `0x5839c` |
| `ldr` at 0xa5464 | load | 4 | `abs 0xa54a0` | `-` |
| `ldr` at 0xa5468 | load | 4 | `abs 0xa54a4` | `-` |
| `str` at 0xa546c | store | 4 | `abs 0x14a554` | `0x586a4` |
| `ldr` at 0xa5470 | load | 4 | `abs 0xa54a8` | `-` |
| `ldr` at 0xa5474 | load | 4 | `abs 0xa54ac` | `-` |
| `str` at 0xa5478 | store | 4 | `abs 0x14a558` | `0x58964` |
| `ldr` at 0xa547c | load | 4 | `abs 0xa5490` | `-` |
| `str` at 0xa5484 | store | 4 | `abs 0x14a688` | `0x1` |
| `ldmdb` at 0xa548c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa548c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa548c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa5454 | `(unknown callee) 0xbc05c` | `0x125d3c` | `0x19` | `r2` | `[0x14a688]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa5448 | `[0x14a688] != 0x0` | `[0x14a688] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa548c | `r0` | `?` |

## `IP65_Cfg_Init`

- `IP65_Cfg_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64474 to 0x644fc.
  Established. A byte read targets absolute `0x14926c` (`ldrb` at 0x64498). Zero takes the branch to
  `0x644c0`, bypassing the unnamed `0x21eb8` call with candidates `0x9`, `0x10d970`, `0x90`, `0x0`
  (`beq` at 0x644a0, `bl` at 0x644bc). The later assignments write byte `0x1` at `0x14926c`,
  four-byte `[sp-24]` at `0x149270`, and four-byte `[sp-20]` at `0x149274` (`strb` at 0x644c8, `str`
  at 0x644d4, `str` at 0x644e0). Configuration meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x64484 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64488 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6448c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x64490 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x64494 | load | 4 | `abs 0x644e8` | `-` |
| `ldrb` at 0x64498 | load | 1 | `abs 0x14926c` | `-` |
| `ldr` at 0x644a4 | load | 4 | `abs 0x644ec` | `-` |
| `str` at 0x644a8 | store | 4 | `sp -0x28` | `0x10d954` |
| `ldr` at 0x644b0 | load | 4 | `abs 0x644f0` | `-` |
| `ldr` at 0x644c0 | load | 4 | `abs 0x644e8` | `-` |
| `strb` at 0x644c8 | store | 1 | `abs 0x14926c` | `0x1` |
| `ldr` at 0x644cc | load | 4 | `abs 0x644f4` | `-` |
| `ldr` at 0x644d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x644d4 | store | 4 | `abs 0x149270` | `[sp-24]` |
| `ldr` at 0x644d8 | load | 4 | `abs 0x644f8` | `-` |
| `ldr` at 0x644dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x644e0 | store | 4 | `abs 0x149274` | `[sp-20]` |
| `ldmdb` at 0x644e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x644e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x644e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x644bc | `(unknown callee) 0x21eb8` | `0x9` | `0x10d970` | `0x90` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x644a0 | `[0x14926c]:1 == 0x0` | `[0x14926c]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x644e4 | `r0` | `?` |

## `IP65_Set_MemCfg`

- `IP65_Set_MemCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x644fc to 0x64518.
  Established. The explicit stores save entry `r0` and `r1` as four bytes at `sp -0x14` and
  `sp -0x18` (`str` at 0x6450c, `str` at 0x64510). There are no calls or conditional branches in the
  span. The return carries entry `r0` (`ldmdb` at 0x64514). Memory-configuration behavior beyond
  these stack accesses is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6450c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x64510 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0x64514 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64514 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64514 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64514 | `r0` | `r0` |

## `IP65_Probe`

- `IP65_Probe`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64538 to 0x64644.
  Established. The unnamed `0x65204` call carries `(r0 & 0xff)`, `sp-24`, `sp-24`, `(r0 & 0xff)`
  (`bl` at 0x64568); its zero result takes the branch to `0x6458c` (`beq` at 0x6457c). Byte stores
  on that arm write `0x0` at `[sp-17]:1 +0x14927c` and `0x2` at `[sp-17]:1 +0x2b16c8` (`strb` at
  0x64598, `strb` at 0x645a8). A later nonzero `ret(0x5e7d8)` takes the branch past the unnamed
  `0x648e0` call (`bne` at 0x64614, `bl` at 0x64628). Probe and device semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6454c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x64554 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x64558 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64570 | store | 4 | `sp -0x20` | `ret(0x65204)` |
| `ldr` at 0x64574 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x64584 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6458c | load | 4 | `abs 0x6463c` | `-` |
| `ldrb` at 0x64590 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x64598 | store | 1 | `[sp-17]:1 +0x14927c` | `0x0` |
| `ldr` at 0x6459c | load | 4 | `abs 0x64640` | `-` |
| `ldrb` at 0x645a0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x645a8 | store | 1 | `[sp-17]:1 +0x2b16c8` | `0x2` |
| `ldr` at 0x645ac | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x645b0 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x645b4 | store | 1 | `sp -0x21` | `([[sp-24]+2]:1 & 0xff)` |
| `ldrb` at 0x645b8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x645bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x645c0 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x645d4 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x645e4 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x645ec | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldrb` at 0x645f4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64600 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64618 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64630 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x64634 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x64638 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64638 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64638 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64568 | `(unknown callee) 0x65204` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x645e0 | `(unknown callee) 0xd224c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x645fc | `(unknown callee) 0x64e9c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x64608 | `(unknown callee) 0x5e7d8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x64628 | `(unknown callee) 0x648e0` | `[sp-17]:1` | `0x1` | `0x0` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6457c | `ret(0x65204) == 0x0` | `ret(0x65204) != 0x0` |
| `ble` at 0x645cc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x64614 | `ret(0x5e7d8) != 0x0` | `ret(0x5e7d8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64638 | `r0` | `?` |

## `IP65_Init`

- `IP65_Init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64644 to 0x646c4.
  Established. Zero `ret(0x5e754)` takes the first branch to `0x646b4`, while nonzero
  `ret(0x5e7d8)` takes the second branch there (`beq` at 0x64678, `bne` at 0x64690). Both taken
  edges bypass the call (`beq` at 0x64678, `bne` at 0x64690). The unnamed `0x648e0` call carries
  `[sp-17]:1`, `0x0`, `0x1`, `[sp-17]:1` (`bl` at 0x646a4). Separate four-byte stack assignments retain `0x0` and
  `0x1` (`str` at 0x646ac, `str` at 0x646b8). Initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64658 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x64660 | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x64664 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6467c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64694 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x646ac | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x646b8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x646bc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x646c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x646c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x646c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6466c | `(unknown callee) 0x5e754` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x64684 | `(unknown callee) 0x5e7d8` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x646a4 | `(unknown callee) 0x648e0` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64678 | `ret(0x5e754) == 0x0` | `ret(0x5e754) != 0x0` |
| `bne` at 0x64690 | `ret(0x5e7d8) != 0x0` | `ret(0x5e7d8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x646c0 | `r0` | `?` |
