# ARM hub configuration and access helpers in 1.40.5

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
These routines have no unresolved computed jumps. Numeric callee names, arity and effects
remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/hub-config-access-facts.txt) retain all tool sections.

## `SHUB_Set_MemCfg`

- `SHUB_Set_MemCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47e18 to 0x47e9c.
  Established. Included absolute assignments write four-byte `r0` at `0x148d94` and byte `([sp-24]:1 & 0xff)` at `0x148d98` (`str` at 0x47e50, `strb` at 0x47e5c). Two unequal-value tests take
  branches to `0x47e8c`: `[0x148d94] != [sp-28]` and `[0x148d98]:1 != [sp-32]` (`bne` at 0x47e70,
  `bne` at 0x47e84). The unnamed call there uses `r0`, `r1`, unknown, unknown (`bl` at 0x47e8c).
  Memory-configuration semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47e28 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47e2c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47e30 | load | 4 | `abs 0x47e94` | `-` |
| `ldr` at 0x47e34 | load | 4 | `abs 0x148d94` | `-` |
| `str` at 0x47e38 | store | 4 | `sp -0x1c` | `[0x148d94]` |
| `ldr` at 0x47e3c | load | 4 | `abs 0x47e98` | `-` |
| `ldrb` at 0x47e40 | load | 1 | `abs 0x148d98` | `-` |
| `str` at 0x47e44 | store | 4 | `sp -0x20` | `[0x148d98]:1` |
| `ldr` at 0x47e48 | load | 4 | `abs 0x47e94` | `-` |
| `ldr` at 0x47e4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x47e50 | store | 4 | `abs 0x148d94` | `r0` |
| `ldr` at 0x47e54 | load | 4 | `abs 0x47e98` | `-` |
| `ldrb` at 0x47e58 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x47e5c | store | 1 | `abs 0x148d98` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x47e60 | load | 4 | `abs 0x47e94` | `-` |
| `ldr` at 0x47e64 | load | 4 | `abs 0x148d94` | `-` |
| `ldr` at 0x47e68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47e74 | load | 4 | `abs 0x47e98` | `-` |
| `ldrb` at 0x47e78 | load | 1 | `abs 0x148d98` | `-` |
| `ldr` at 0x47e7c | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x47e90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47e90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47e90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47e8c | `(unknown callee) 0x48334` | `r0` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47e70 | `[0x148d94] != [sp-28]` | `[0x148d94] == [sp-28]` |
| `bne` at 0x47e84 | `[0x148d98]:1 != [sp-32]` | `[0x148d98]:1 == [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47e90 | `r0` | `?` |

## `SHUB_MMR_Write`

- `SHUB_MMR_Write`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47e9c to 0x47f10.
  Established. The unnamed `0xeaf08` call has unknown, `r1`, `r2`, `r3` (`bl` at 0x47ee0); its
  nonzero result takes the branch to `0x47efc` (`bne` at 0x47eec). Four-byte stack assignments at
  `sp -0x24` retain `0x1` on the fall-through and `0x0` on the taken path (`str` at 0x47ef4, `str`
  at 0x47f00). The exit carries `[sp-36]` (`ldmdb` at 0x47f08). Register-write effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47eac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47eb0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47eb4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x47eb8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47ebc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47ec4 | load | 4 | `abs 0x47f0c` | `-` |
| `ldr` at 0x47ec8 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x47ed0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47ed8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47edc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47ef4 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x47f00 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x47f04 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x47f08 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47f08 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47f08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47ee0 | `(unknown callee) 0xeaf08` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47eec | `ret(0xeaf08) != 0x0` | `ret(0xeaf08) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47f08 | `r0` | `[sp-36]` |

## `SHUB_MMR_Read`

- `SHUB_MMR_Read`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47f10 to 0x47f84.
  Established. A four-byte absolute read targets `0x148d8c` (`ldr` at 0x47f3c). The unnamed
  `0xeae04` call carries unknown, `r1`, `r2`, `r3` (`bl` at 0x47f54). Nonzero result takes the
  branch to `0x47f70` (`bne` at 0x47f60). Separate four-byte assignments at `sp -0x24` retain `0x1`
  and `0x0` (`str` at 0x47f68, `str` at 0x47f74). Hardware-read semantics and success convention
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47f20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47f24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47f28 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x47f2c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x47f30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47f38 | load | 4 | `abs 0x47f80` | `-` |
| `ldr` at 0x47f3c | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x47f44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x47f48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47f4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47f50 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x47f68 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x47f74 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x47f78 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x47f7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47f7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47f7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47f54 | `(unknown callee) 0xeae04` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47f60 | `ret(0xeae04) != 0x0` | `ret(0xeae04) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47f7c | `r0` | `[sp-36]` |

## `SHUB_NICfg_Override`

- `SHUB_NICfg_Override`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47f84 to 0x47fd0.
  Established. The explicit stack stores retain four-byte `r0` at `sp -0x14` and `r1` at `sp -0x18`
  (`str` at 0x47f94, `str` at 0x47f98). A later four-byte load has an unknown effective address, and
  the return value remains unknown (`ldr` at 0x47fc0, `ldmdb` at 0x47fc8). There are no calls or
  conditional branches in the span. Override encoding is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47f94 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47f98 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47f9c | load | 4 | `abs 0x47fcc` | `-` |
| `ldr` at 0x47fa0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47fb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47fc0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x47fc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x47fc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x47fc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x47fc8 | `r0` | `?` |

## `SHUB_Get_NICfg`

- `SHUB_Get_NICfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47fd0 to 0x48028.
  Established. Four-byte entry `r0` and `r1` are saved at `sp -0x14` and `sp -0x18` (`str` at
  0x47fe0, `str` at 0x47fe4). The data load is a halfword read with unknown effective address
  (`ldrh` at 0x48018). The exit value is unknown (`ldmdb` at 0x48020). No call or conditional branch
  occurs. Network configuration field meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47fe0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47fe4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47fe8 | load | 4 | `abs 0x48024` | `-` |
| `ldr` at 0x47fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48000 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x48018 | load | 2 | `(unknown)` | `-` |
| `ldmdb` at 0x48020 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48020 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48020 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48020 | `r0` | `?` |

## `SHUB_Set_NICfg`

- `SHUB_Set_NICfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48028 to 0x48124.
  Established. The branch takes `[sp-32] == 0x2` to `0x480d0`, while a later equality with `0x3`
  takes its branch to `0x480f0` (`beq` at 0x48084, `beq` at 0x480ac). Included halfword stores at
  `[sp-36] +0x0` have unknown values at two sites and `([sp-26]:2 & 0xffff)` at another (`strh` at
  0x480c8, `strh` at 0x480e8, `strh` at 0x480f8). A final four-byte assignment writes `0x1` at an
  unknown effective address (`str` at 0x48114). Configuration operation meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x48038 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4803c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x48040 | store | 4 | `sp -0x20` | `r3` |
| `strh` at 0x48044 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldr` at 0x48048 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4805c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48068 | load | 4 | `abs 0x4811c` | `-` |
| `str` at 0x48070 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x48074 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x48078 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x4807c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48088 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48094 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x480a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x480b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x480b8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x480bc | load | 2 | `[sp-36] +0x0` | `-` |
| `ldrh` at 0x480c0 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x480c8 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x480d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x480d4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x480d8 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0x480e0 | load | 2 | `[sp-36] +0x0` | `-` |
| `strh` at 0x480e8 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x480f0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x480f4 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x480f8 | store | 2 | `[sp-36] +0x0` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0x480fc | load | 4 | `abs 0x48120` | `-` |
| `ldr` at 0x48100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48108 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48114 | store | 4 | `(unknown)` | `0x1` |
| `ldmdb` at 0x48118 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48118 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x48084 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x48090 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x4809c | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x480ac | `[sp-32] == 0x3` | `[sp-32] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48118 | `r0` | `r0` |

## `SHUB_Set_MCI`

- `SHUB_Set_MCI`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48124 to 0x4820c.
  Established. Four-byte `0x1` is written at absolute `0x148da8` (`str` at 0x48150). Subsequent
  stores with unresolved effective addresses include byte `([sp-17]:1 & 0xff)`, halfword `([sp-20]:2 & 0xffff)` and byte `([sp-21]:1 & 0xff)` (`strb` at 0x48194, `strh` at 0x481c0, `strb` at
  0x481ec). The loop branch uses a signed-less-or-equal flags condition whose operands are unknown
  (`ble` at 0x48164). MCI record meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x48138 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x4813c | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x48144 | store | 1 | `sp -0x15` | `(r2 & 0xff)` |
| `ldr` at 0x48148 | load | 4 | `abs 0x48204` | `-` |
| `str` at 0x48150 | store | 4 | `abs 0x148da8` | `0x1` |
| `str` at 0x48158 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4815c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4816c | load | 4 | `abs 0x48208` | `-` |
| `ldr` at 0x48170 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x48190 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x48194 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x48198 | load | 4 | `abs 0x48208` | `-` |
| `ldr` at 0x4819c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x481bc | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x481c0 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x481c4 | load | 4 | `abs 0x48208` | `-` |
| `ldr` at 0x481c8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x481e8 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x481ec | store | 1 | `(unknown)` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x481f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x481f8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x48200 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48200 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48200 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x48164 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48200 | `r0` | `?` |

## `SHUB_Set_WRT90`

- `SHUB_Set_WRT90`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4822c to 0x48314.
  Established. The absolute four-byte assignment targets `0x148dac` with `0x1` (`str` at 0x48258).
  Later byte stores have unresolved addresses and values `([sp-17]:1 & 0xff)` and `([sp-18]:1 & 0xff)` (`strb` at 0x4829c, `strb` at 0x482c8). A halfword store at another unresolved address
  retains `([sp-20]:2 & 0xffff)` (`strh` at 0x482f4). Device timing and field semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x48240 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x48248 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x4824c | store | 2 | `sp -0x14` | `(r2 & 0xffff)` |
| `ldr` at 0x48250 | load | 4 | `abs 0x4830c` | `-` |
| `str` at 0x48258 | store | 4 | `abs 0x148dac` | `0x1` |
| `str` at 0x48260 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x48264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48274 | load | 4 | `abs 0x48310` | `-` |
| `ldr` at 0x48278 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x48298 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x4829c | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x482a0 | load | 4 | `abs 0x48310` | `-` |
| `ldr` at 0x482a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x482c4 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x482c8 | store | 1 | `(unknown)` | `([sp-18]:1 & 0xff)` |
| `ldr` at 0x482cc | load | 4 | `abs 0x48310` | `-` |
| `ldr` at 0x482d0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x482f0 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x482f4 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x482f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48300 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x48308 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48308 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48308 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4826c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48308 | `r0` | `?` |

## `SHUB_Set_FSBCfg`

- `SHUB_Set_FSBCfg`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x484c8 to 0x4867c.
  Established. Three four-byte reads fetch entry `sp +0x0`, `sp +0x4` and `sp +0x8` (`ldr` at
  0x484dc, `ldr` at 0x484e0, `ldr` at 0x484e4). An absolute four-byte assignment writes `0x1` at
  `0x148db0` (`str` at 0x48528). Included byte stores at unresolved addresses retain `([sp-29]:1 & 0xff)`, `([sp-30]:1 & 0xff)` and `([sp-31]:1 & 0xff)` (`strb` at 0x4860c, `strb` at 0x48638,
  `strb` at 0x4865c). Bus-configuration fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x484dc | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x484e0 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x484e4 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x484ec | store | 1 | `sp -0x19` | `(r0 & 0xff)` |
| `strb` at 0x484f4 | store | 1 | `sp -0x1a` | `(r1 & 0xff)` |
| `strb` at 0x484fc | store | 1 | `sp -0x1b` | `(r2 & 0xff)` |
| `strb` at 0x48504 | store | 1 | `sp -0x1c` | `(r3 & 0xff)` |
| `strb` at 0x4850c | store | 1 | `sp -0x1d` | `([sp] & 0xff)` |
| `strb` at 0x48514 | store | 1 | `sp -0x1e` | `([sp+4] & 0xff)` |
| `strb` at 0x4851c | store | 1 | `sp -0x1f` | `([sp+8] & 0xff)` |
| `ldr` at 0x48520 | load | 4 | `abs 0x48674` | `-` |
| `str` at 0x48528 | store | 4 | `abs 0x148db0` | `0x1` |
| `str` at 0x48530 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x48534 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48544 | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x48548 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48560 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x48564 | store | 1 | `(unknown)` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0x48568 | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x4856c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48584 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x48588 | store | 1 | `(unknown)` | `([sp-26]:1 & 0xff)` |
| `ldr` at 0x4858c | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x48590 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x485b0 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x485b4 | store | 1 | `(unknown)` | `([sp-27]:1 & 0xff)` |
| `ldr` at 0x485b8 | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x485bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x485dc | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x485e0 | store | 1 | `(unknown)` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x485e4 | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x485e8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48608 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x4860c | store | 1 | `(unknown)` | `([sp-29]:1 & 0xff)` |
| `ldr` at 0x48610 | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x48614 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48634 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x48638 | store | 1 | `(unknown)` | `([sp-30]:1 & 0xff)` |
| `ldr` at 0x4863c | load | 4 | `abs 0x48678` | `-` |
| `ldr` at 0x48640 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x48658 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x4865c | store | 1 | `(unknown)` | `([sp-31]:1 & 0xff)` |
| `ldr` at 0x48660 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x48668 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x48670 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x48670 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x48670 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48670 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48670 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4853c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48670 | `r0` | `?` |

## `SHUBTarget`

- `SHUBTarget`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4869c to 0x486d8.
  Established. The initial explicit store saves byte `(r0 & 0xff)` at `sp -0x11` (`strb` at
  0x486b0). A four-byte load reads absolute `0x148d8c`, then another four-byte load has an
  unresolved effective address (`ldr` at 0x486c0, `ldr` at 0x486c8). The exit value is unknown
  (`ldmdb` at 0x486d0). Target-record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x486b0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x486b4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x486bc | load | 4 | `abs 0x486d4` | `-` |
| `ldr` at 0x486c0 | load | 4 | `abs 0x148d8c` | `-` |
| `ldr` at 0x486c8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x486d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x486d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x486d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x486d0 | `r0` | `?` |

## `SHUB_SetClkstop`

- `SHUB_SetClkstop`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c920 to 0x4c990.
  Established. A byte store has unknown effective address and retains `([sp-20]:1 & 0xff)` (`strb`
  at 0x4c974). The four-byte counter at `sp -0x18` starts at `0x0` and later takes `[sp-24]+1`
  (`str` at 0x4c938, `str` at 0x4c980). The signed-less-or-equal loop predicate has unknown operands
  (`ble` at 0x4c944). Clock-stop signaling and record placement are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c930 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c938 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c93c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c94c | load | 4 | `abs 0x4c98c` | `-` |
| `ldr` at 0x4c950 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4c970 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4c974 | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x4c978 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4c980 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x4c988 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c988 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c988 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4c944 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c988 | `r0` | `?` |

## `SHUB_GetClkstop`

- `SHUB_GetClkstop`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c990 to 0x4c9b0.
  Established. A byte read targets absolute `0x2b14f5` (`ldrb` at 0x4c9a0). The return carries
  `[0x2b14f5]:1` (`ldmdb` at 0x4c9a8). This span makes no calls and has no conditional branches.
  Clock-stop encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4c99c | load | 4 | `abs 0x4c9ac` | `-` |
| `ldrb` at 0x4c9a0 | load | 1 | `abs 0x2b14f5` | `-` |
| `ldmdb` at 0x4c9a8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c9a8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c9a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c9a8 | `r0` | `[0x2b14f5]:1` |

## `SHUB_SetTP`

- `SHUB_SetTP`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c9b0 to 0x4ca20.
  Established. The explicit four-byte stack save retains entry `r0` at `sp -0x14` (`str` at
  0x4c9c0). A later byte store writes `([sp-20]:1 & 0xff)` at an unknown effective address (`strb`
  at 0x4ca04). The signed-less-or-equal branch has unknown operands (`ble` at 0x4c9d4). Test-point
  behavior and record fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c9c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4c9c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4c9cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4c9dc | load | 4 | `abs 0x4ca1c` | `-` |
| `ldr` at 0x4c9e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4ca00 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x4ca04 | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x4ca08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4ca10 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x4ca18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ca18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ca18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x4c9d4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ca18 | `r0` | `?` |

## `SHUB_GetTP`

- `SHUB_GetTP`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ca20 to 0x4ca40.
  Established. A byte read targets absolute `0x2b14f4` (`ldrb` at 0x4ca30); its
  value reaches the return as `[0x2b14f4]:1` (`ldmdb` at 0x4ca38). No call or conditional branch
  occurs. The meaning of that stored byte remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4ca2c | load | 4 | `abs 0x4ca3c` | `-` |
| `ldrb` at 0x4ca30 | load | 1 | `abs 0x2b14f4` | `-` |
| `ldmdb` at 0x4ca38 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4ca38 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4ca38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4ca38 | `r0` | `[0x2b14f4]:1` |

## `SHUB_Rev`

- `SHUB_Rev`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4c52c to 0x4c560.
  Established. Four-byte entry `r0` is saved at `sp -0x14` (`str` at 0x4c53c). The later four-byte
  data load has unresolved effective address (`ldr` at 0x4c548). The exit value remains unknown in
  the fact sheet (`ldmdb` at 0x4c558). Revision encoding and table placement are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4c53c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4c540 | load | 4 | `abs 0x4c55c` | `-` |
| `ldr` at 0x4c544 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4c548 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x4c558 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4c558 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4c558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4c558 | `r0` | `?` |
