# ARM boot, PPP and trigger helpers in 1.40.5

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
`EnvTestEvent` and `sernumBootCheck` have unresolved computed transfers. Their access, call
and return lists cover known edges only. Unknown data values and instruction effects remain
explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/boot-ppp-trigger-facts.txt) retain all tool sections.

## `bootAddressIsValid`

- `bootAddressIsValid`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5edc to 0xc5ff8.
  Established. The absolute word at `0x2b4420` is read (`ldr` at 0xc5f14). Unsigned entry `r0 < [0x2b4420]` takes `0xc5f44`, while the opposite comparison falls through (`blo` at 0xc5f1c). The
  word at `sp -0x1c` receives `0x1` on an included path (`str` at 0xc5f3c). Valid boot address
  meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5eec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5ef0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5f04 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc5f0c | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5f14 | load | 4 | `abs 0x2b4420` | `-` |
| `ldr` at 0xc5f20 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f24 | load | 4 | `abs 0x2b4420` | `-` |
| `ldr` at 0xc5f2c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5f3c | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc5f44 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f48 | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xc5f54 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5f5c | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xc5f68 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f6c | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f70 | load | 4 | `abs 0x2b4428` | `-` |
| `ldr` at 0xc5f74 | load | 4 | `abs 0x2b4438` | `-` |
| `ldr` at 0xc5f7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5f8c | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xc5f94 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5f98 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xc5fa4 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5fac | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xc5fb8 | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5fbc | load | 4 | `abs 0xc5ff4` | `-` |
| `ldr` at 0xc5fc0 | load | 4 | `abs 0x2b442c` | `-` |
| `ldr` at 0xc5fc4 | load | 4 | `abs 0x2b4438` | `-` |
| `ldr` at 0xc5fcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5fdc | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xc5fe8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc5fec | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xc5ff0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc5ff0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc5ff0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc5efc | `? == 0x0` | `? != 0x0` |
| `blo` at 0xc5f1c | `r0 < [0x2b4420] (unsigned, 32-bit)` | `r0 >= [0x2b4420] (unsigned, 32-bit)` |
| `bhs` at 0xc5f34 | `r0 >= [0x2b4420]+524288 (unsigned, 32-bit)` | `r0 < [0x2b4420]+524288 (unsigned, 32-bit)` |
| `beq` at 0xc5f50 | `[0x2b4428] == 0x0` | `[0x2b4428] != 0x0` |
| `blo` at 0xc5f64 | `r0 < [0x2b4428] (unsigned, 32-bit)` | `r0 >= [0x2b4428] (unsigned, 32-bit)` |
| `bhs` at 0xc5f84 | `r0 >= ([0x2b4428] + [0x2b4438]) (unsigned, 32-bit)` | `r0 < ([0x2b4428] + [0x2b4438]) (unsigned, 32-bit)` |
| `beq` at 0xc5fa0 | `[0x2b442c] == 0x0` | `[0x2b442c] != 0x0` |
| `blo` at 0xc5fb4 | `r0 < [0x2b442c] (unsigned, 32-bit)` | `r0 >= [0x2b442c] (unsigned, 32-bit)` |
| `bhs` at 0xc5fd4 | `r0 >= ([0x2b442c] + [0x2b4438]) (unsigned, 32-bit)` | `r0 < ([0x2b442c] + [0x2b4438]) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc5ff0 | `r0` | `?` |

## `bootAddressNormalize`

- `bootAddressNormalize`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5ff8 to 0xc605c.
  Established. The word at `sp -0x1c` has an unknown stored value (`str` at 0xc6018). Equality
  `[sp-28] == 0x1` takes `0xc6038`, while inequality falls through (`beq` at 0xc6024). The later
  word assignment at `sp -0x14` also has an unknown value (`str` at 0xc604c). Address normalization
  details remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6008 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc600c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc6010 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6018 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc601c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc604c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc6050 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc6058 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6058 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6058 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc6024 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `beq` at 0xc6030 | `[sp-28] == 0x2` | `[sp-28] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6058 | `r0` | `?` |

## `bootValidate`

- `bootValidate`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc605c to 0xc61b0.
  Established. The unnamed `0xc6298` call carries `r0`, unknown, `r2`, unknown (`bl` at 0xc6090).
  Unsigned `[[sp-40]+2]:2 <= 0x1f` takes `0xc60e8`, while larger values fall through (`bls` at 0xc60d0). The word at `sp -0x1c` later receives `([sp-28] + ret(0x1aff8))` (`str` at 0xc6178).
  Boot validation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc606c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc6070 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc6074 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6084 | store | 4 | `sp -0x18` | `sp-92` |
| `ldr` at 0xc6088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc608c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6098 | store | 4 | `sp -0x28` | `ret(0xc6298)` |
| `ldr` at 0xc609c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc60ac | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xc60b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc60c0 | store | 4 | `sp -0x2c` | `ret(0xc61b0)` |
| `ldr` at 0xc60c4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc60c8 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xc60d4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc60d8 | load | 2 | `[sp-40] +0x2` | `-` |
| `str` at 0xc60ec | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0xc60f4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc60f8 | load | 2 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xc6104 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc610c | store | 4 | `sp -0x20` | `ret(0xc61b0)+4` |
| `ldr` at 0xc6114 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xc6118 | store | 4 | `sp -0x20` | `ret(0xc61b0)` |
| `ldr` at 0xc611c | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc6120 | load | 2 | `[sp-40] +0x2` | `-` |
| `ldr` at 0xc6124 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc612c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc6134 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc6138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc613c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6150 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6154 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc615c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xc6170 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6178 | store | 4 | `sp -0x1c` | `([sp-28] + ret(0x1aff8))` |
| `ldr` at 0xc6180 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6190 | store | 4 | `sp -0x68` | `?` |
| `str` at 0xc619c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0xc61a0 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xc61a4 | store | 4 | `sp -0x64` | `[sp-104]` |
| `ldr` at 0xc61a8 | load | 4 | `sp -0x64` | `-` |
| `ldmdb` at 0xc61ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc61ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc61ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6090 | `(unknown callee) 0xc6298` | `r0` | `?` | `r2` | `?` |
| `bl` at 0xc60b8 | `(unknown callee) 0xc61b0` | `[sp-20]` | `?` | `?` | `ret(0xc6298)` |
| `bl` at 0xc6164 | `(unknown callee) 0x1aff8` | `?` | `?` | `sp-32` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc607c | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xc60a4 | `ret(0xc6298) != 0x0` | `ret(0xc6298) == 0x0` |
| `bls` at 0xc60d0 | `[[sp-40]+2]:2 <= 0x1f (unsigned, 32-bit)` | `[[sp-40]+2]:2 > 0x1f (unsigned, 32-bit)` |
| `bhi` at 0xc60e0 | `[[sp-40]+2]:2 > 0x400 (unsigned, 32-bit)` | `[[sp-40]+2]:2 <= 0x400 (unsigned, 32-bit)` |
| `bhi` at 0xc6100 | `[[sp-40]]:2 > 0x2 (unsigned, 32-bit)` | `[[sp-40]]:2 <= 0x2 (unsigned, 32-bit)` |
| `blo` at 0xc6144 | `C == 0` | `not (C == 0)` |
| `beq` at 0xc6188 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc61ac | `r0` | `[sp-100]` |

## `bootGetHeaderPtr`

- `bootGetHeaderPtr`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc61b0 to 0xc6298.
  Established. The unnamed `0xc5ff8` call carries `ret(0x1aff8)`, `ret(0xcba8c)`, unknown,
  `ret(0xcba8c)` (`bl` at 0xc6234). The subsequent unnamed `0xc5edc` call carries `ret(0xc5ff8)` and
  three unknown candidates (`bl` at 0xc6240); its nonzero result takes `0xc625c`, while zero falls
  through (`bne` at 0xc624c). Header pointer interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc61c0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc61c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc61dc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc61e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc61f0 | store | 4 | `sp -0x24` | `ret(0xcbbb0)` |
| `ldr` at 0xc61f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6204 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc620c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6210 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc6220 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6238 | store | 4 | `sp -0x20` | `ret(0xc5ff8)` |
| `ldr` at 0xc623c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6254 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc625c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6270 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6278 | store | 4 | `sp -0x30` | `ret(0xcbbb0)` |
| `str` at 0xc6284 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc6288 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc628c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6290 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xc6294 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc6294 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6294 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6294 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc61c8 | `(unknown callee) 0xc5edc` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc61e8 | `(unknown callee) 0xcbbb0` | `[sp-24]` | `?` | `?` | `ret(0xc5edc)` |
| `bl` at 0xc6214 | `(unknown callee) 0x1aff8` | `[[sp-36]]` | `?` | `?` | `[sp-36]` |
| `bl` at 0xc6224 | `(unknown callee) 0xcba8c` | `[sp-24]` | `?` | `?` | `ret(0x1aff8)` |
| `bl` at 0xc6234 | `(unknown callee) 0xc5ff8` | `ret(0x1aff8)` | `ret(0xcba8c)` | `?` | `ret(0xcba8c)` |
| `bl` at 0xc6240 | `(unknown callee) 0xc5edc` | `ret(0xc5ff8)` | `?` | `?` | `?` |
| `bl` at 0xc6260 | `(unknown callee) 0xcbbb0` | `[sp-32]` | `?` | `?` | `ret(0xc5edc)` |
| `bl` at 0xc6274 | `(unknown callee) 0xcbbb0` | `[sp-32]` | `?` | `?` | `ret(0xcbbb0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc61d4 | `ret(0xc5edc) != 0x0` | `ret(0xc5edc) == 0x0` |
| `bne` at 0xc61fc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc624c | `ret(0xc5edc) != 0x0` | `ret(0xc5edc) == 0x0` |
| `beq` at 0xc626c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6294 | `r0` | `?` |

## `bootGetHeader`

- `bootGetHeader`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6298 to 0xc6448.
  Established. The unnamed `0x1b394` call carries `[sp-28]`, `ret(0xc61b0)`, `0x30`, `ret(0xc61b0)`
  (`bl` at 0xc62fc). Included word assignments at `[sp-28] +0x10` and `[sp-28] +0x14` write
  `ret(0xcbbb0)` (`str` at 0xc637c, `str` at 0xc63a0). The halfword at `[sp-28] +0x2` receives
  `([[sp-28]]:2 & 0xffff)` (`strh` at 0xc6428). Boot header field meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc62a8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc62ac | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc62b4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc62b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc62c4 | store | 4 | `sp -0x28` | `ret(0xcba8c)` |
| `ldr` at 0xc62c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc62d4 | store | 4 | `sp -0x20` | `ret(0xc61b0)` |
| `ldr` at 0xc62d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc62e8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc62f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc62f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6304 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc6308 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6318 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6328 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6338 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6340 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6348 | store | 4 | `(unknown)` | `ret(0x1aff8)` |
| `ldr` at 0xc634c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc6354 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc635c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6364 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xc6368 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc637c | store | 4 | `[sp-28] +0x10` | `ret(0xcbbb0)` |
| `ldr` at 0xc6380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6384 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6388 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xc638c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc63a0 | store | 4 | `[sp-28] +0x14` | `ret(0xcbbb0)` |
| `ldr` at 0xc63a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63ac | load | 4 | `[sp-28] +0x18` | `-` |
| `ldr` at 0xc63b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc63c4 | store | 4 | `[sp-28] +0x18` | `ret(0xcbbb0)` |
| `ldr` at 0xc63c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63d0 | load | 4 | `[sp-28] +0x24` | `-` |
| `ldr` at 0xc63d4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc63e8 | store | 4 | `[sp-28] +0x24` | `ret(0xcbbb0)` |
| `ldr` at 0xc63ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc63f4 | load | 4 | `[sp-28] +0x2c` | `-` |
| `ldr` at 0xc63f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc640c | store | 4 | `[sp-28] +0x2c` | `ret(0xcbbb0)` |
| `ldr` at 0xc6410 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc6414 | load | 2 | `[sp-28] +0x2` | `-` |
| `strh` at 0xc6418 | store | 2 | `sp -0x2a` | `([[sp-28]+2]:2 & 0xffff)` |
| `ldr` at 0xc641c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc6420 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc6424 | load | 2 | `[sp-28] +0x0` | `-` |
| `strh` at 0xc6428 | store | 2 | `[sp-28] +0x2` | `([[sp-28]]:2 & 0xffff)` |
| `ldr` at 0xc642c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xc6430 | load | 2 | `sp -0x2a` | `-` |
| `strh` at 0xc6434 | store | 2 | `[sp-28] +0x0` | `([sp-42]:2 & 0xffff)` |
| `ldr` at 0xc6438 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc643c | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0xc6440 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xc6444 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc6444 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6444 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6444 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc62bc | `(unknown callee) 0xcba8c` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xc62cc | `(unknown callee) 0xc61b0` | `[sp-24]` | `?` | `?` | `ret(0xcba8c)` |
| `bl` at 0xc62fc | `(unknown callee) 0x1b394` | `[sp-28]` | `ret(0xc61b0)` | `0x30` | `ret(0xc61b0)` |
| `bl` at 0xc632c | `(unknown callee) 0x1aff8` | `?` | `?` | `?` | `?` |
| `bl` at 0xc636c | `(unknown callee) 0xc5ff8` | `[[sp-28]+16]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc6378 | `(unknown callee) 0xcbbb0` | `ret(0xc5ff8)` | `?` | `?` | `ret(0xc5ff8)` |
| `bl` at 0xc6390 | `(unknown callee) 0xc5ff8` | `[[sp-28]+20]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc639c | `(unknown callee) 0xcbbb0` | `ret(0xc5ff8)` | `?` | `?` | `ret(0xc5ff8)` |
| `bl` at 0xc63b4 | `(unknown callee) 0xc5ff8` | `[[sp-28]+24]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc63c0 | `(unknown callee) 0xcbbb0` | `ret(0xc5ff8)` | `?` | `?` | `ret(0xc5ff8)` |
| `bl` at 0xc63d8 | `(unknown callee) 0xc5ff8` | `[[sp-28]+36]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc63e4 | `(unknown callee) 0xcbbb0` | `ret(0xc5ff8)` | `?` | `?` | `ret(0xc5ff8)` |
| `bl` at 0xc63fc | `(unknown callee) 0xc5ff8` | `[[sp-28]+44]` | `[sp-40]` | `?` | `[sp-28]` |
| `bl` at 0xc6408 | `(unknown callee) 0xcbbb0` | `ret(0xc5ff8)` | `?` | `?` | `ret(0xc5ff8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc62e0 | `ret(0xc61b0) != 0x0` | `ret(0xc61b0) == 0x0` |
| `bls` at 0xc6310 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6444 | `r0` | `?` |

## `bootDefaultImage`

- `bootDefaultImage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6448 to 0xc6680.
  Established. A word assignment through `r0 +0x0` writes `0x0` (`str` at 0xc6488). The absolute
  word at `0x2b443c` is read (`ldr` at 0xc6490). The unnamed `0xc605c` call carries `[sp-96]`,
  `sp-88`, unknown, `sp-88` (`bl` at 0xc653c). Default image selection policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6458 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc6460 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xc6468 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xc6470 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc6474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6480 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6488 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xc648c | load | 4 | `abs 0xc667c` | `-` |
| `ldr` at 0xc6490 | load | 4 | `abs 0x2b443c` | `-` |
| `str` at 0xc6494 | store | 4 | `sp -0x24` | `[0x2b443c]` |
| `ldr` at 0xc6498 | load | 4 | `abs 0xc667c` | `-` |
| `ldr` at 0xc649c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc64a0 | load | 4 | `abs 0x2b4424` | `-` |
| `ldr` at 0xc64ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc64b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc64c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc64c8 | store | 4 | `sp -0x24` | `[sp-36]+4` |
| `ldr` at 0xc64d0 | load | 4 | `abs 0xc667c` | `-` |
| `ldr` at 0xc64d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc64d8 | load | 4 | `abs 0x2b4424` | `-` |
| `str` at 0xc64e8 | store | 4 | `sp -0x6c` | `0x0` |
| `ldr` at 0xc64f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc64f8 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc64fc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc6500 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc6504 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc6508 | load | 4 | `abs 0xc667c` | `-` |
| `ldr` at 0xc650c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6510 | load | 4 | `abs 0x2b443c` | `-` |
| `ldr` at 0xc6520 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6524 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc652c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xc6534 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xc6550 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6554 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc6564 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc6568 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xc656c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc6570 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xc6578 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc6580 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc658c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc6590 | store | 4 | `sp -0x64` | `[sp-72]` |
| `ldr` at 0xc6594 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6598 | store | 4 | `sp -0x68` | `[sp-32]` |
| `ldr` at 0xc659c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc65a8 | load | 4 | `sp -0x64` | `-` |
| `ldrb` at 0xc65ac | load | 1 | `[sp-100] +0x0` | `-` |
| `ldr` at 0xc65b8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc65bc | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xc65cc | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc65d0 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc65d4 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xc65d8 | load | 1 | `[sp-104] +0x0` | `-` |
| `ldr` at 0xc65e8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xc65ec | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0xc65f0 | load | 1 | `[sp-100] +0x0` | `-` |
| `ldrb` at 0xc65f4 | load | 1 | `[sp-104] +0x0` | `-` |
| `str` at 0xc6604 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc660c | load | 4 | `sp -0x64` | `-` |
| `str` at 0xc6614 | store | 4 | `sp -0x64` | `[sp-100]+1` |
| `ldr` at 0xc6618 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xc6620 | store | 4 | `sp -0x68` | `[sp-104]+1` |
| `ldr` at 0xc6628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6634 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xc6638 | store | 4 | `sp -0x18` | `[sp-96]` |
| `ldr` at 0xc663c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xc6640 | store | 4 | `sp -0x20` | `[sp-72]` |
| `ldr` at 0xc6644 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc664c | store | 4 | `sp -0x24` | `[sp-36]-4` |
| `ldr` at 0xc6654 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6664 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc6668 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc666c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6670 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xc6674 | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0xc6678 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6678 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6678 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc653c | `(unknown callee) 0xc605c` | `[sp-96]` | `sp-88` | `?` | `sp-88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc647c | `r0 == 0x0` | `r0 != 0x0` |
| `bhs` at 0xc64a8 | `C == 1` | `not (C == 1)` |
| `bne` at 0xc64b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc64e0 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0xc6518 | `C == 1` | `not (C == 1)` |
| `beq` at 0xc6548 | `ret(0xc605c) == 0x0` | `ret(0xc605c) != 0x0` |
| `beq` at 0xc6560 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xc6588 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0xc65a4 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xc65b4 | `[[sp-100]]:1 == 0x0` | `[[sp-100]]:1 != 0x0` |
| `bne` at 0xc65c4 | `[[sp-104]]:1 != 0x0` | `[[sp-104]]:1 == 0x0` |
| `bhs` at 0xc65e0 | `[[sp-100]]:1 >= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 < [[sp-104]]:1 (unsigned, 32-bit)` |
| `bls` at 0xc65fc | `[[sp-100]]:1 <= [[sp-104]]:1 (unsigned, 32-bit)` | `[[sp-100]]:1 > [[sp-104]]:1 (unsigned, 32-bit)` |
| `bne` at 0xc6630 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0xc665c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6678 | `r0` | `?` |

## `PPPInit`

- `PPPInit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb094 to 0xbb164.
  Established. The unnamed `0x1b4cc` call carries entry `r0`, `0x0`, `0x3c`, entry `r3` (`bl` at 0xbb0c0). Word assignments at `[sp-20] +0x10` and `[sp-20] +0x14` write `[sp-32]` and `[sp]`
  respectively (`str` at 0xbb0e4, `str` at 0xbb0f0). The later unnamed `0xbb164` call carries
  `[sp-20]`, `0x3`, `[sp-20]`, `[sp+24]` (`bl` at 0xbb15c). PPP record meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb0a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbb0a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbb0ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbb0b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbb0b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbb0cc | store | 4 | `[sp-20] +0x0` | `[sp-24]` |
| `ldr` at 0xbb0d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbb0d8 | store | 4 | `[sp-20] +0x4` | `[sp-28]` |
| `ldr` at 0xbb0dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbb0e4 | store | 4 | `[sp-20] +0x10` | `[sp-32]` |
| `ldr` at 0xbb0e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0ec | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb0f0 | store | 4 | `[sp-20] +0x14` | `[sp]` |
| `ldr` at 0xbb0f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb0f8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xbb108 | store | 4 | `[sp-20] +0x20` | `?` |
| `ldr` at 0xbb10c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb110 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xbb120 | store | 4 | `[sp-20] +0x24` | `?` |
| `ldr` at 0xbb124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb128 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xbb12c | store | 4 | `[sp-20] +0x28` | `[sp+12]` |
| `ldr` at 0xbb130 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb134 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xbb138 | store | 4 | `[sp-20] +0x2c` | `[sp+16]` |
| `ldr` at 0xbb13c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb140 | load | 4 | `sp +0x14` | `-` |
| `str` at 0xbb144 | store | 4 | `[sp-20] +0x34` | `[sp+20]` |
| `ldr` at 0xbb148 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb14c | load | 4 | `sp +0x18` | `-` |
| `str` at 0xbb150 | store | 4 | `[sp-20] +0x38` | `[sp+24]` |
| `ldr` at 0xbb154 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbb160 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb160 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb160 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb0c0 | `(unknown callee) 0x1b4cc` | `r0` | `0x0` | `0x3c` | `r3` |
| `bl` at 0xbb15c | `(unknown callee) 0xbb164` | `[sp-20]` | `0x3` | `[sp-20]` | `[sp+24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb160 | `r0` | `ret(0xbb164)` |

## `PPPReset`

- `PPPReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb164 to 0xbb1c4.
  Established. The word assignments at `r0 +0xc` and `[sp-20] +0x30` each write `0x0` (`str` at 0xbb194, `str` at 0xbb1a0). Another word assignment writes `0x0` to an unknown effective target
  (`str` at 0xbb1bc). The return reports entry `r0` (`ldmdb` at 0xbb1c0). Protocol reset scope
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb174 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbb178 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbb17c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb18c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb194 | store | 4 | `r0 +0xc` | `0x0` |
| `ldr` at 0xbb198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb1a0 | store | 4 | `[sp-20] +0x30` | `0x0` |
| `ldr` at 0xbb1a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb1b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb1bc | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0xbb1c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb1c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb1c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb188 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xbb1b0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb1c0 | `r0` | `r0` |

## `PPPWrite`

- `PPPWrite`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb1c4 to 0xbb6d8.
  Established. The byte assignment through `[[sp-20]+24] +0x0` writes `0x7e` (`strb` at 0xbb210). A
  later byte assignment through that printed target writes `0x7d` (`strb` at 0xbb2fc). The indirect
  call at `0xbb3f0` has an unknown target and candidates `[sp-28]`, `[[sp-20]+16]`, `[[sp-20]+28]`,
  `[sp-20]` (`mov` at 0xbb3f0). PPP framing behavior beyond the listed effects remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb1d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbb1d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbb1dc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbb1e0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbb1e4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb1e8 | store | 4 | `sp -0x30` | `[sp]` |
| `ldr` at 0xbb1ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb1f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb1f4 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xbb1f8 | store | 4 | `r0 +0x18` | `[r0+16]` |
| `ldr` at 0xbb1fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb204 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbb210 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `str` at 0xbb218 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb21c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb224 | store | 4 | `[sp-20] +0x1c` | `0x1` |
| `strh` at 0xbb22c | store | 2 | `sp -0x26` | `?` |
| `ldr` at 0xbb230 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb244 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xbb248 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbb24c | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xbb250 | store | 4 | `sp -0x3c` | `[[sp-32]]:1` |
| `ldr` at 0xbb254 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb260 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb26c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb278 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb288 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb29c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb2a0 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xbb2b0 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbb2b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb2bc | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xbb2cc | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xbb2d8 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbb2dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbb2e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb2f0 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbb2fc | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xbb304 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb310 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xbb314 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbb318 | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xbb324 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbb32c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb334 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb338 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb340 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xbb348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb350 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0xbb354 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbb35c | load | 1 | `[sp-32] +0x0` | `-` |
| `strb` at 0xbb360 | store | 1 | `[[sp-20]+24] +0x0` | `([[sp-32]]:1 & 0xff)` |
| `str` at 0xbb368 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb36c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb374 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb37c | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xbb380 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xbb384 | load | 1 | `[sp-32] +0x0` | `-` |
| `ldrh` at 0xbb388 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbb39c | store | 2 | `sp -0x26` | `(ret(0xbafbc) & 0xffff)` |
| `ldr` at 0xbb3a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbb3a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xbb3ac | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb3b4 | store | 4 | `sp +0x0` | `[sp]-1` |
| `ldr` at 0xbb3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb3bc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbb3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb3c8 | load | 4 | `[sp-20] +0x14` | `-` |
| `ldr` at 0xbb3d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb3d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb3dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb3e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbb3e4 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbb3e8 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb3f8 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xbb3fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb400 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb404 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbb410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb414 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb41c | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb424 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbb42c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbb430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb438 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbb43c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbb444 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbb448 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xbb450 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb458 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xbb45c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+16]` |
| `ldr` at 0xbb460 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb468 | store | 4 | `[sp-20] +0x1c` | `0x0` |
| `ldrh` at 0xbb470 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbb478 | store | 2 | `sp -0x26` | `?` |
| `ldrb` at 0xbb47c | load | 1 | `sp -0x26` | `-` |
| `strb` at 0xbb480 | store | 1 | `sp -0x2c` | `([sp-38]:1 & 0xff)` |
| `ldrh` at 0xbb484 | load | 2 | `sp -0x26` | `-` |
| `strb` at 0xbb48c | store | 1 | `sp -0x2b` | `?` |
| `strh` at 0xbb494 | store | 2 | `sp -0x26` | `0x0` |
| `ldrh` at 0xbb498 | load | 2 | `sp -0x26` | `-` |
| `str` at 0xbb4ac | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0xbb4b0 | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbb4c4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xbb4c8 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0xbb4cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb4d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb4e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb4f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb500 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb514 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb518 | load | 4 | `[sp-20] +0x24` | `-` |
| `str` at 0xbb528 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbb530 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb534 | load | 4 | `[sp-20] +0x20` | `-` |
| `str` at 0xbb544 | store | 4 | `sp -0x34` | `0x1` |
| `str` at 0xbb550 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xbb554 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbb560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb568 | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbb574 | store | 1 | `[[sp-20]+24] +0x0` | `0x7d` |
| `str` at 0xbb57c | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb580 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb588 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xbb58c | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbb5a0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbb5ac | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbb5b4 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb5b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb5bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb5c0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb5c8 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+2` |
| `ldr` at 0xbb5d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb5d8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrh` at 0xbb5dc | load | 2 | `sp -0x26` | `-` |
| `ldrb` at 0xbb5f4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbb5f8 | store | 1 | `[[sp-20]+24] +0x0` | `?` |
| `str` at 0xbb600 | store | 4 | `[sp-20] +0x18` | `[[sp-20]+24]+1` |
| `ldr` at 0xbb604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb60c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb614 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldrh` at 0xbb618 | load | 2 | `sp -0x26` | `-` |
| `strh` at 0xbb620 | store | 2 | `sp -0x26` | `([sp-38]:2+1 & 0xffff)` |
| `ldr` at 0xbb628 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb62c | load | 4 | `[sp-20] +0x18` | `-` |
| `strb` at 0xbb634 | store | 1 | `[[sp-20]+24] +0x0` | `0x7e` |
| `ldr` at 0xbb638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb63c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb640 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb648 | store | 4 | `[sp-20] +0x1c` | `[[sp-20]+28]+1` |
| `ldr` at 0xbb64c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbb65c | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbb660 | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0xbb670 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xbb674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb678 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb67c | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbb688 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb68c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb694 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb69c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbb6a4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbb6a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb6ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb6b0 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0xbb6b4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0xbb6bc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbb6c0 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0xbb6c8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbb6cc | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0xbb6d0 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xbb6d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb6d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb6d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb394 | `(unknown callee) 0xbafbc` | `[[sp-32]]:1` | `[sp-38]:2` | `[sp-38]:2` | `[[sp-32]]:1` |
| `mov` at 0xbb3f0 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xbb440 | `(unknown callee) 0xbb014` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |
| `mov` at 0xbb668 | `(unknown callee) (unknown target)` | `[sp-28]` | `[[sp-20]+16]` | `[[sp-20]+28]` | `[sp-20]` |
| `bl` at 0xbb6b8 | `(unknown callee) 0xbb014` | `[sp-20]` | `ret(unknown)` | `[[sp-20]+16]` | `[[sp-20]+28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbb238 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xbb25c | `[[sp-32]]:1 == 0x13` | `[[sp-32]]:1 != 0x13` |
| `bgt` at 0xbb268 | `[[sp-32]]:1 > 0x13 (signed, 32-bit)` | `[[sp-32]]:1 <= 0x13 (signed, 32-bit)` |
| `beq` at 0xbb274 | `[[sp-32]]:1 == 0xd` | `[[sp-32]]:1 != 0xd` |
| `beq` at 0xbb280 | `[[sp-32]]:1 == 0x11` | `[[sp-32]]:1 != 0x11` |
| `bhi` at 0xbb294 | `[[sp-32]]:1-125 > 0x1 (unsigned, 32-bit)` | `[[sp-32]]:1-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xbb2a8 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xbb2c4 | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xbb2e4 | `Z == 1` | `not (Z == 1)` |
| `blo` at 0xbb3d0 | `[[sp-20]+28]+5 < [[sp-20]+20] (unsigned, 32-bit)` | `[[sp-20]+28]+5 >= [[sp-20]+20] (unsigned, 32-bit)` |
| `beq` at 0xbb40c | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |
| `bls` at 0xbb4a0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xbb4d4 | `[sp-64] == 0x13` | `[sp-64] != 0x13` |
| `bgt` at 0xbb4e0 | `[sp-64] > 0x13 (signed, 32-bit)` | `[sp-64] <= 0x13 (signed, 32-bit)` |
| `beq` at 0xbb4ec | `[sp-64] == 0xd` | `[sp-64] != 0xd` |
| `beq` at 0xbb4f8 | `[sp-64] == 0x11` | `[sp-64] != 0x11` |
| `bhi` at 0xbb50c | `[sp-64]-125 > 0x1 (unsigned, 32-bit)` | `[sp-64]-125 <= 0x1 (unsigned, 32-bit)` |
| `beq` at 0xbb520 | `[[sp-20]+36] == 0x0` | `[[sp-20]+36] != 0x0` |
| `beq` at 0xbb53c | `[[sp-20]+32] == 0x0` | `[[sp-20]+32] != 0x0` |
| `beq` at 0xbb55c | `[sp-52] == 0x0` | `[sp-52] != 0x0` |
| `beq` at 0xbb684 | `ret(unknown) == [[sp-20]+28]` | `ret(unknown) != [[sp-20]+28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb6d4 | `r0` | `?` |

## `PPPRead`

- `PPPRead`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb6d8 to 0xbc05c.
  Established. The byte assignment through `([sp-64] + [sp-56]) +0x0` writes `0x55` (`strb` at 0xbbf88). The following byte assignment through `([sp-64]+1 + [sp-56]) +0x0` has an unknown value
  (`strb` at 0xbbfac). The indirect call at `0xbbeb0` has an unknown target and candidates
  `[sp-32]`, unknown, `[[sp-24]+4]`, unknown (`mov` at 0xbbeb0). Receive protocol policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb6e8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbb6ec | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbb6f0 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xbb6f4 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xbb6fc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xbb704 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xbb70c | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0xbb714 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xbb71c | store | 4 | `sp -0x40` | `0x0` |
| `str` at 0xbb724 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0xbb72c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbb734 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb73c | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbb744 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbb74c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbb754 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xbb758 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb75c | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xbb760 | store | 4 | `sp -0x60` | `[r0+40]` |
| `ldr` at 0xbb764 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb768 | load | 4 | `r0 +0x2c` | `-` |
| `str` at 0xbb76c | store | 4 | `sp -0x64` | `[r0+44]` |
| `ldr` at 0xbb770 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb780 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xbb784 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbb78c | store | 4 | `sp +0x0` | `?` |
| `ldr` at 0xbb790 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbb7a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb7a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbb7a8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb7b0 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xbb7b8 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbb7c0 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbb7c8 | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xbb7d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbb7dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb7e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbb7f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb7f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb7f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbb804 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbb810 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbb818 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xbb81c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb820 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbb82c | store | 4 | `sp -0x38` | `ret(0x1b358)` |
| `ldr` at 0xbb830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb838 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xbb840 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xbb844 | store | 1 | `sp -0x2d` | `?` |
| `str` at 0xbb84c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbb850 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb854 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb858 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbb860 | store | 4 | `[sp-24] +0xc` | `[[sp-24]+12]-1` |
| `ldr` at 0xbb868 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb870 | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0xbb878 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbb87c | store | 1 | `([sp-64] + [sp-56]) +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbb884 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xbb888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb88c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbb898 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbb89c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb8a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb8ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb8b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xbb8b8 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbb8c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb8c4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbb8c8 | store | 4 | `sp -0x44` | `[[sp-24]+12]` |
| `str` at 0xbb8d4 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbb8d8 | load | 4 | `sp -0x5c` | `-` |
| `ldrb` at 0xbb8e4 | load | 1 | `sp -0x2d` | `-` |
| `ldrb` at 0xbb8f4 | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xbb8f8 | store | 4 | `sp -0x70` | `[sp-45]:1` |
| `ldr` at 0xbb8fc | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb908 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb914 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xbb924 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xbb938 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xbb93c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb940 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xbb94c | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0xbb958 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb95c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb960 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb96c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb970 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb974 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb978 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb97c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb984 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb98c | load | 4 | `abs 0xbc03c` | `-` |
| `ldr` at 0xbb994 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb9a0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb9ac | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbb9b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbb9bc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbb9c0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbb9cc | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbb9d0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbb9d4 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbb9d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbb9dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbb9e4 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbb9e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb9ec | load | 4 | `abs 0xbc040` | `-` |
| `ldr` at 0xbb9f4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbba00 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbba0c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xbba18 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbba20 | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xbba24 | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xbba28 | store | 1 | `sp -0x33` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xbba2c | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbba34 | store | 4 | `sp -0x48` | `[sp-72]-2` |
| `ldrb` at 0xbba38 | load | 1 | `[sp-72] -0x2` | `-` |
| `strb` at 0xbba3c | store | 1 | `sp -0x34` | `([[sp-72]-2]:1 & 0xff)` |
| `ldr` at 0xbba40 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbba48 | store | 4 | `sp -0x4c` | `[sp-76]-2` |
| `ldr` at 0xbba50 | load | 4 | `sp -0x54` | `-` |
| `ldrb` at 0xbba5c | load | 1 | `sp -0x34` | `-` |
| `strb` at 0xbba60 | store | 1 | `sp -0x33` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0xbba64 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xbba6c | store | 4 | `sp -0x48` | `[sp-72]-1` |
| `ldrb` at 0xbba70 | load | 1 | `[sp-72] -0x1` | `-` |
| `strb` at 0xbba74 | store | 1 | `sp -0x34` | `([[sp-72]-1]:1 & 0xff)` |
| `ldr` at 0xbba78 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbba80 | store | 4 | `sp -0x4c` | `[sp-76]-1` |
| `ldr` at 0xbba84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbba88 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbba8c | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xbba98 | load | 4 | `abs 0xbc044` | `-` |
| `ldr` at 0xbbaa0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbab8 | load | 4 | `abs 0xbc048` | `-` |
| `str` at 0xbbac8 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0xbbad0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbad4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbad8 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbae4 | store | 4 | `sp -0x7c` | `sp-52` |
| `ldr` at 0xbbae8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbaec | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbaf0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbaf4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbafc | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbbb00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbb04 | load | 4 | `abs 0xbc04c` | `-` |
| `ldr` at 0xbbb0c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbb14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbbb20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbb28 | store | 4 | `[sp-24] +0x30` | `0x1` |
| `ldr` at 0xbbb2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbb30 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbb34 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbb3c | store | 4 | `sp -0x48` | `([[sp-36]] + [sp-96])` |
| `str` at 0xbbb44 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbbb4c | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbbb54 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbbb5c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xbbb64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbb68 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xbbb78 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0xbbb80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbb84 | load | 4 | `[sp-24] +0x24` | `-` |
| `ldr` at 0xbbb90 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbb98 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xbbb9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbba0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbba4 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbbb0 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbbbb4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbbb8 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbbbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbbc0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbbc8 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbbbcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbbd0 | load | 4 | `abs 0xbc050` | `-` |
| `ldr` at 0xbbbd8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbbe0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbbec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbbf4 | load | 4 | `abs 0xbc050` | `-` |
| `str` at 0xbbbf8 | store | 4 | `sp -0x6c` | `0xfffffd40` |
| `ldr` at 0xbbc00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbc04 | load | 4 | `[sp-24] +0x30` | `-` |
| `ldr` at 0xbbc10 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0xbbc24 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbbc28 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbc2c | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbc30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbc34 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbc3c | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbbc40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbc58 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbbc60 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0xbbc6c | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbbc74 | store | 1 | `sp -0x2d` | `?` |
| `str` at 0xbbc7c | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xbbc80 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbc84 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbbc90 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xbbc9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbca0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbca4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbca8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xbbcb4 | store | 4 | `[sp-36] +0x0` | `ret(0x1b358)` |
| `ldr` at 0xbbcb8 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbcc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbccc | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xbbcd4 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbbcd8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbcdc | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbce0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbce4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbcec | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbbcf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbcf4 | load | 4 | `abs 0xbc054` | `-` |
| `ldr` at 0xbbd04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbd10 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbd18 | load | 4 | `abs 0xbc054` | `-` |
| `str` at 0xbbd1c | store | 4 | `sp -0x6c` | `0xfffffd3e` |
| `ldr` at 0xbbd24 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbd28 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbd2c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xbbd34 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbd3c | store | 4 | `sp -0x48` | `(([[sp-36]] + [sp]) + [sp-96])` |
| `ldr` at 0xbbd40 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xbbd44 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbd4c | store | 4 | `sp +0x0` | `?` |
| `str` at 0xbbd54 | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xbbd5c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xbbd68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbd6c | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbd70 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbd7c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbbd80 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbd84 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbd88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbd8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbd94 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0xbbd98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbda4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbdac | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbdb4 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `str` at 0xbbdbc | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbbdc4 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbbdcc | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbbdd4 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xbbddc | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbbde4 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbbdf0 | load | 4 | `sp -0x50` | `-` |
| `ldrb` at 0xbbdf8 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbbdfc | store | 1 | `[sp-80] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbbe04 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0xbbe08 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xbbe10 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0xbbe1c | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xbbe24 | load | 1 | `sp -0x2d` | `-` |
| `strb` at 0xbbe28 | store | 1 | `[sp-72] +0x0` | `([sp-45]:1 & 0xff)` |
| `str` at 0xbbe30 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xbbe34 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbbe3c | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldr` at 0xbbe44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbbe50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xbbe5c | load | 4 | `abs 0xbc058` | `-` |
| `ldr` at 0xbbe60 | load | 4 | `abs 0x14ab40` | `-` |
| `ldr` at 0xbbe6c | load | 4 | `abs 0xbc058` | `-` |
| `ldr` at 0xbbe70 | load | 4 | `abs 0x14ab40` | `-` |
| `ldr` at 0xbbe78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xbbe80 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xbbe84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe90 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbbe94 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xbbe98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbe9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbbea0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbea8 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0xbbeb4 | store | 4 | `(unknown)` | `ret(unknown)` |
| `ldr` at 0xbbeb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbbec4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbec8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbbecc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xbbed0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0xbbedc | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xbbee0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbbee4 | store | 4 | `sp -0x78` | `[sp-56]` |
| `ldr` at 0xbbee8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbeec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xbbef4 | store | 4 | `sp -0x74` | `([sp-64] + [sp-68])` |
| `ldr` at 0xbbef8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbefc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xbbf04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xbbf0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbf14 | store | 4 | `[sp-24] +0x30` | `0x0` |
| `ldr` at 0xbbf18 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbf24 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbf2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbf30 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xbbf34 | store | 4 | `sp -0x6c` | `[[sp-24]+12]` |
| `ldr` at 0xbbf3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbf44 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbbf50 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbbf58 | store | 4 | `sp -0x3c` | `[sp-60]+256` |
| `ldr` at 0xbbf5c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbbf60 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xbbf6c | store | 4 | `sp -0x38` | `ret(0x1b358)` |
| `ldr` at 0xbbf74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbf7c | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xbbf88 | store | 1 | `([sp-64] + [sp-56]) +0x0` | `0x55` |
| `str` at 0xbbf90 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xbbf98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xbbfa0 | load | 4 | `sp -0x38` | `-` |
| `strb` at 0xbbfac | store | 1 | `([sp-64]+1 + [sp-56]) +0x0` | `?` |
| `str` at 0xbbfb4 | store | 4 | `sp -0x40` | `[sp-64]+2` |
| `str` at 0xbbfbc | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xbbfc0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xbbfcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbfd0 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xbbfe0 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xbbfe8 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbbff0 | store | 4 | `sp -0x50` | `sp-52` |
| `str` at 0xbbff8 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0xbc000 | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xbc008 | store | 4 | `sp -0x5c` | `0x1` |
| `ldr` at 0xbc00c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbc018 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbc024 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbc02c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xbc030 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0xbc034 | load | 4 | `sp -0x6c` | `-` |
| `ldmdb` at 0xbc038 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbc038 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc038 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc038 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb824 | `(unknown callee) 0x1b358` | `?` | `?` | `?` | `?` |
| `bl` at 0xbb8bc | `(unknown callee) 0x1b394` | `([sp-56] + [sp-64]+1)+1` | `[[sp-24]+8]` | `[[sp-24]+12]` | `([sp-56] + [sp-64]+1)+1` |
| `bl` at 0xbb998 | `(unknown callee) 0xbb014` | `[sp-24]` | `0xfffffd41` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbb9f8 | `(unknown callee) 0xbb014` | `[sp-24]` | `0xfffffd42` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbbaa8 | `(unknown callee) 0xbae9c` | `0xffff` | `([[sp-36]] + [sp-96])` | `[sp-76]` | `sp-52` |
| `bl` at 0xbbb10 | `(unknown callee) 0xbb014` | `[sp-24]` | `0xfffffd44` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbbbdc | `(unknown callee) 0xbb014` | `[sp-24]` | `0xfffffd40` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbbbf0 | `(unknown callee) 0x1b490` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xbbc50 | `(unknown callee) 0xbb014` | `[sp-24]` | `?` | `sp-45` | `0x1` |
| `bl` at 0xbbcac | `(unknown callee) 0x1b358` | `[[sp-36]]` | `[sp-100]` | `[sp-76]` | `[sp-36]` |
| `bl` at 0xbbd00 | `(unknown callee) 0xbb014` | `[sp-24]` | `0xfffffd3e` | `0x0` | `0x0` |
| `bl` at 0xbbd14 | `(unknown callee) 0x1b490` | `[sp-56]` | `?` | `?` | `[sp-56]` |
| `bl` at 0xbbda8 | `(unknown callee) 0xbb014` | `[sp-24]` | `?` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbbe74 | `(unknown callee) 0x22780` | `[0x14ab40]` | `?` | `?` | `0x14ab40` |
| `mov` at 0xbbeb0 | `(unknown callee) (unknown target)` | `[sp-32]` | `?` | `[[sp-24]+4]` | `?` |
| `bl` at 0xbbf08 | `(unknown callee) 0xbb014` | `[sp-24]` | `[[sp-24]+12]` | `([[sp-36]] + [sp-96])` | `[sp-76]` |
| `bl` at 0xbbf28 | `(unknown callee) 0x1b490` | `[sp-56]` | `?` | `[sp-24]` | `[sp-56]` |
| `bl` at 0xbbf64 | `(unknown callee) 0x1b358` | `[sp-56]` | `[sp-60]+256` | `[sp-64]+3` | `[sp-60]+256` |
| `bl` at 0xbc028 | `(unknown callee) 0x1b490` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbb778 | `[sp] >= 0x0 (signed, 32-bit)` | `[sp] < 0x0 (signed, 32-bit)` |
| `beq` at 0xbb79c | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xbb7d8 | `Z == 0` | `not (Z == 0)` |
| `bgt` at 0xbb7e8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `blo` at 0xbb80c | `C == 0` | `not (C == 0)` |
| `beq` at 0xbb894 | `[[sp-24]+12] == 0x0` | `[[sp-24]+12] != 0x0` |
| `beq` at 0xbb8e0 | `[sp-92] == 0x0` | `[sp-92] != 0x0` |
| `beq` at 0xbb8ec | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xbb904 | `[sp-45]:1 == 0x7d` | `[sp-45]:1 != 0x7d` |
| `bgt` at 0xbb910 | `[sp-45]:1 > 0x7d (signed, 32-bit)` | `[sp-45]:1 <= 0x7d (signed, 32-bit)` |
| `beq` at 0xbb91c | `[sp-45]:1 == 0xd` | `[sp-45]:1 != 0xd` |
| `beq` at 0xbb92c | `[sp-45]:1 == 0x7e` | `[sp-45]:1 != 0x7e` |
| `beq` at 0xbb948 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xbb954 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `beq` at 0xbb9a8 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bhi` at 0xbb9b4 | `[sp-76] > 0x2 (unsigned, 32-bit)` | `[sp-76] <= 0x2 (unsigned, 32-bit)` |
| `beq` at 0xbba08 | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bne` at 0xbba14 | `[sp-84] != 0x0` | `[sp-84] == 0x0` |
| `bne` at 0xbba58 | `[sp-84] != 0x1` | `[sp-84] == 0x1` |
| `bne` at 0xbbac0 | `? != 0xf0b8` | `? == 0xf0b8` |
| `bne` at 0xbbb1c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xbbb70 | `[[sp-24]+48] == 0x0` | `[[sp-24]+48] != 0x0` |
| `beq` at 0xbbb8c | `[[sp-24]+36] == 0x0` | `[[sp-24]+36] != 0x0` |
| `beq` at 0xbbbe8 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bne` at 0xbbc0c | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `bne` at 0xbbc18 | `[sp-92] != 0x0` | `[sp-92] == 0x0` |
| `beq` at 0xbbc68 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `blo` at 0xbbc8c | `[sp-76] < [sp] (unsigned, 32-bit)` | `[sp-76] >= [sp] (unsigned, 32-bit)` |
| `beq` at 0xbbc98 | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `bne` at 0xbbcc0 | `[[sp-36]] != 0x0` | `[[sp-36]] == 0x0` |
| `beq` at 0xbbd0c | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bls` at 0xbbd64 | `[sp-84] <= 0x1 (unsigned, 32-bit)` | `[sp-84] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xbbe4c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xbbe58 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbbe68 | `[0x14ab40] == 0x0` | `[0x14ab40] != 0x0` |
| `bge` at 0xbbec0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xbbf20 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xbbf4c | `[sp-64]+3 < [sp-60] (unsigned, 32-bit)` | `[sp-64]+3 >= [sp-60] (unsigned, 32-bit)` |
| `beq` at 0xbbfc8 | `[sp-104] == 0x0` | `[sp-104] != 0x0` |
| `bne` at 0xbbfd8 | `[[sp-24]+48] != 0x0` | `[[sp-24]+48] == 0x0` |
| `beq` at 0xbc014 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xbc020 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc038 | `r0` | `?` |

## `EnvTestEvent`

- `EnvTestEvent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82ab4 to 0x82bd8.
  Established. Unsigned entry `r0 <= 0x4` takes an unresolved computed destination, while larger
  values fall through (`ldrls` at 0x82ad0). The known return reports entry `r0` (`ldmdb` at 0x82bb8). Environmental event effects on unresolved edges remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82ac4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x82ac8 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x82ad0 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x82bb8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82bb8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82bb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x82ad0 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82bb8 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x82ad0).

## `sernumBootCheck`

- `sernumBootCheck`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x50820 to 0x50da0.
  Established. The unnamed `0x1f26c` call at `0x50854` carries `0x50` and three unknown candidates
  (`bl` at 0x50854); its zero result takes `0x50870`, while nonzero falls through (`beq` at 0x50860). The absolute byte at `0x2af28c` is read (`ldrb` at 0x50948). Serial-number boot policy
  beyond known edges remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x50834 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x50840 | store | 4 | `sp -0x18` | `ret(0x1f26c)` |
| `str` at 0x5084c | store | 4 | `sp -0x20` | `ret(0x1f26c)` |
| `str` at 0x50868 | store | 4 | `sp -0xc4` | `0x0` |
| `str` at 0x50874 | store | 4 | `sp -0x1c` | `ret(0x4f730)` |
| `ldr` at 0x50878 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x50884 | load | 4 | `abs 0x50d24` | `-` |
| `str` at 0x50888 | store | 4 | `sp -0xd0` | `0x10b790` |
| `ldr` at 0x50890 | load | 4 | `abs 0x50d28` | `-` |
| `ldr` at 0x50894 | load | 4 | `abs 0x50d2c` | `-` |
| `str` at 0x508a4 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x508ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x508b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x508c8 | load | 4 | `abs 0x50d30` | `-` |
| `ldr` at 0x508d0 | load | 4 | `abs 0x50d34` | `-` |
| `ldr` at 0x508d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x508d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x508e0 | load | 4 | `abs 0x50d38` | `-` |
| `str` at 0x508e4 | store | 4 | `sp -0xc4` | `0xfffff95c` |
| `ldr` at 0x508ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x508f8 | load | 4 | `abs 0x50d3c` | `-` |
| `ldr` at 0x5090c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50918 | load | 4 | `abs 0x50d40` | `-` |
| `ldr` at 0x5092c | load | 4 | `abs 0x50d44` | `-` |
| `ldr` at 0x50930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50938 | load | 4 | `abs 0x50d48` | `-` |
| `str` at 0x5093c | store | 4 | `sp -0xc4` | `0xfffff95a` |
| `ldr` at 0x50944 | load | 4 | `abs 0x50d4c` | `-` |
| `ldrb` at 0x50948 | load | 1 | `abs 0x2af28c` | `-` |
| `ldr` at 0x50954 | load | 4 | `abs 0x50d4c` | `-` |
| `ldr` at 0x50958 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5095c | load | 1 | `abs 0x2af28c` | `-` |
| `ldrb` at 0x50960 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x50970 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0x50978 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5097c | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0x50988 | load | 4 | `(unknown)` | `-` |
| `str` at 0x509b8 | store | 4 | `sp -0x14` | `0xa` |
| `ldr` at 0x509c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x509c8 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x509d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x509e0 | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x509fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50a04 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x50a0c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50a14 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50a34 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50a3c | load | 1 | `[sp-24] +0x3` | `-` |
| `ldr` at 0x50a44 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50a4c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50a6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50a74 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x50a7c | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50a84 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50aa4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50aac | load | 1 | `[sp-24] +0x5` | `-` |
| `ldr` at 0x50ab4 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50abc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50adc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50ae4 | load | 1 | `[sp-24] +0x6` | `-` |
| `ldr` at 0x50aec | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50af4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x50b14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x50b1c | load | 1 | `[sp-24] +0x7` | `-` |
| `ldr` at 0x50b24 | load | 4 | `ret(0x1b55c) +0x0` | `-` |
| `ldrh` at 0x50b2c | load | 2 | `(unknown)` | `-` |
| `str` at 0x50b40 | store | 4 | `sp -0x14` | `0xb` |
| `ldr` at 0x50b44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x50b50 | load | 4 | `abs 0x50d50` | `-` |
| `ldr` at 0x50b58 | load | 4 | `abs 0x50d54` | `-` |
| `ldr` at 0x50b5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50b64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x50b68 | store | 4 | `sp -0xc8` | `[sp-20]` |
| `ldr` at 0x50b6c | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50b78 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50b84 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50b94 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x50ba4 | load | 4 | `abs 0x50d4c` | `-` |
| `ldrb` at 0x50ba8 | load | 1 | `abs 0x2af28c` | `-` |
| `ldr` at 0x50bac | load | 4 | `abs 0x50d58` | `-` |
| `ldr` at 0x50bbc | load | 4 | `abs 0x50d5c` | `-` |
| `ldr` at 0x50bc8 | load | 4 | `abs 0x50d60` | `-` |
| `ldr` at 0x50bd4 | load | 4 | `abs 0x50d64` | `-` |
| `ldr` at 0x50bdc | load | 4 | `abs 0x50d68` | `-` |
| `str` at 0x50be0 | store | 4 | `sp -0xc4` | `0xfffff959` |
| `ldr` at 0x50be8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50bec | load | 4 | `abs 0x50d6c` | `-` |
| `ldr` at 0x50c00 | load | 4 | `abs 0x50d70` | `-` |
| `ldr` at 0x50c10 | load | 4 | `abs 0x50d74` | `-` |
| `ldr` at 0x50c14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50c18 | load | 4 | `abs 0x50d6c` | `-` |
| `ldr` at 0x50c20 | load | 4 | `abs 0x50d78` | `-` |
| `ldr` at 0x50c24 | load | 4 | `abs 0x14a7c0` | `-` |
| `str` at 0x50c2c | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x50c30 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50c3c | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50c48 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50c58 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x50c70 | load | 4 | `abs 0x50d7c` | `-` |
| `ldr` at 0x50c84 | load | 4 | `abs 0x50d80` | `-` |
| `ldr` at 0x50c98 | load | 4 | `abs 0x50d84` | `-` |
| `ldr` at 0x50cac | load | 4 | `abs 0x50d88` | `-` |
| `str` at 0x50cb0 | store | 4 | `sp -0xc4` | `0xfffff95b` |
| `ldr` at 0x50cb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50cc4 | load | 4 | `abs 0x50d8c` | `-` |
| `ldr` at 0x50cd8 | load | 4 | `abs 0x50d90` | `-` |
| `ldr` at 0x50cdc | load | 4 | `abs 0x50d8c` | `-` |
| `ldr` at 0x50cf0 | load | 4 | `abs 0x50d94` | `-` |
| `ldr` at 0x50cf8 | load | 4 | `abs 0x50d98` | `-` |
| `ldr` at 0x50cfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x50d00 | load | 4 | `abs 0x50d6c` | `-` |
| `ldr` at 0x50d08 | load | 4 | `abs 0x50d9c` | `-` |
| `str` at 0x50d0c | store | 4 | `sp -0xc4` | `0xfffff958` |
| `str` at 0x50d18 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x50d1c | load | 4 | `sp -0xc4` | `-` |
| `ldmdb` at 0x50d20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x50d20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x50d20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5083c | `(unknown callee) 0x1f26c` | `0xd` | `r1` | `r2` | `0x0` |
| `bl` at 0x50848 | `(unknown callee) 0x1f26c` | `0xf` | `?` | `?` | `?` |
| `bl` at 0x50854 | `(unknown callee) 0x1f26c` | `0x50` | `?` | `?` | `?` |
| `bl` at 0x50870 | `(unknown callee) 0x4f730` | `ret(0x1f26c)` | `?` | `?` | `ret(0x1f26c)` |
| `bl` at 0x5089c | `(unknown callee) 0x21eb8` | `0x1` | `0x10b7b8` | `0x22e` | `0x0` |
| `bl` at 0x508b8 | `(unknown callee) 0x1b2e0` | `[sp-32]` | `ret(0x4f730)` | `0x6` | `ret(0x4f730)` |
| `bl` at 0x508cc | `(unknown callee) 0x202d0` | `0x10b7d0` | `?` | `?` | `ret(0x1b2e0)` |
| `bl` at 0x508dc | `(unknown callee) 0x2079c` | `0x10b7f0` | `[sp-32]` | `[sp-28]` | `?` |
| `bl` at 0x508fc | `(unknown callee) 0x1b088` | `[sp-24]+1` | `0x10b814` | `?` | `[sp-24]+1` |
| `bl` at 0x5091c | `(unknown callee) 0x1b088` | `[sp-24]+1` | `0x10b81c` | `?` | `[sp-24]+1` |
| `bl` at 0x50934 | `(unknown callee) 0x2079c` | `0x10b824` | `[sp-24]` | `?` | `ret(0x1b088)` |
| `bl` at 0x509ec | `(unknown callee) 0x1b55c` | `ret(0x1b088)` | `?` | `0x1` | `[[sp-24]+1]:1` |
| `bl` at 0x50a24 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x50a5c | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x50a94 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x50acc | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x50b04 | `(unknown callee) 0x1b55c` | `ret(0x1b55c)` | `ret(0x1b55c)` | `?` | `?` |
| `bl` at 0x50b54 | `(unknown callee) 0x202d0` | `0x10b844` | `?` | `?` | `?` |
| `bl` at 0x50b60 | `(unknown callee) 0x2079c` | `0x10b85c` | `[sp-24]` | `?` | `?` |
| `bl` at 0x50bb4 | `(unknown callee) 0x2079c` | `0x10b868` | `[0x2af28c]:1` | `?` | `[0x2af28c]:1` |
| `bl` at 0x50bc0 | `(unknown callee) 0x2079c` | `0x10b888` | `?` | `?` | `[sp-20]` |
| `bl` at 0x50bcc | `(unknown callee) 0x2079c` | `0x10b8c4` | `?` | `?` | `[sp-20]` |
| `bl` at 0x50bd8 | `(unknown callee) 0x2079c` | `0x10b904` | `?` | `?` | `[sp-20]` |
| `bl` at 0x50bf0 | `(unknown callee) 0x1b088` | `[sp-24]` | `0x2b28c8` | `?` | `?` |
| `bl` at 0x50c04 | `(unknown callee) 0x202d0` | `0x10b92c` | `?` | `?` | `ret(0x1b088)` |
| `bl` at 0x50c1c | `(unknown callee) 0x1b52c` | `sp-112` | `0x10b94c` | `[sp-24]` | `0x2b28c8` |
| `bl` at 0x50c74 | `(unknown callee) 0x1b364` | `sp-112` | `0x10b974` | `?` | `sp-112` |
| `bl` at 0x50c88 | `(unknown callee) 0x1b364` | `sp-112` | `0x10b988` | `?` | `sp-112` |
| `bl` at 0x50c9c | `(unknown callee) 0x1b364` | `sp-112` | `0x10b998` | `?` | `sp-112` |
| `bl` at 0x50ca8 | `(unknown callee) 0x202d0` | `sp-112` | `?` | `?` | `sp-112` |
| `bl` at 0x50cc8 | `(unknown callee) 0x1b088` | `[sp-24]+1` | `0x10b9a8` | `?` | `[sp-24]+1` |
| `bl` at 0x50ce0 | `(unknown callee) 0x1b088` | `0x2b28c9` | `0x10b9a8` | `?` | `ret(0x1b088)` |
| `bl` at 0x50cf4 | `(unknown callee) 0x202d0` | `0x10b9b0` | `?` | `?` | `ret(0x1b088)` |
| `bl` at 0x50d04 | `(unknown callee) 0x2079c` | `0x10b9c4` | `[sp-24]` | `0x2b28c8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x50860 | `ret(0x1f26c) == 0x0` | `ret(0x1f26c) != 0x0` |
| `bne` at 0x50880 | `ret(0x4f730) != 0x0` | `ret(0x4f730) == 0x0` |
| `beq` at 0x508c4 | `ret(0x1b2e0) == 0x0` | `ret(0x1b2e0) != 0x0` |
| `beq` at 0x50908 | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `bne` at 0x50928 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `beq` at 0x50950 | `[0x2af28c]:1 == 0x0` | `[0x2af28c]:1 != 0x0` |
| `beq` at 0x50968 | `[0x2af28c]:1 == [[sp-24]]:1` | `[0x2af28c]:1 != [[sp-24]]:1` |
| `ldrls` at 0x50988 | `[[sp-24]]:1-76 <= 0x8 (unsigned, 32-bit)` | `[[sp-24]]:1-76 > 0x8 (unsigned, 32-bit)` |
| `bls` at 0x509d0 | `[[sp-24]+1]:1 <= 0x2f (unsigned, 32-bit)` | `[[sp-24]+1]:1 > 0x2f (unsigned, 32-bit)` |
| `bhi` at 0x509e8 | `[[sp-24]+1]:1 > 0x33 (unsigned, 32-bit)` | `[[sp-24]+1]:1 <= 0x33 (unsigned, 32-bit)` |
| `beq` at 0x50a20 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50a58 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50a90 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50ac8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x50b00 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x50b38 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x50b4c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x50b74 | `[sp-20] == 0xa` | `[sp-20] != 0xa` |
| `bgt` at 0x50b80 | `[sp-20] > 0xa (signed, 32-bit)` | `[sp-20] <= 0xa (signed, 32-bit)` |
| `beq` at 0x50b8c | `[sp-20] == 0x1` | `[sp-20] != 0x1` |
| `beq` at 0x50b9c | `[sp-20] == 0xb` | `[sp-20] != 0xb` |
| `beq` at 0x50bfc | `ret(0x1b088) == 0x0` | `ret(0x1b088) != 0x0` |
| `beq` at 0x50c38 | `[sp-204] == 0x2000` | `[sp-204] != 0x2000` |
| `bgt` at 0x50c44 | `[sp-204] > 0x2000 (signed, 32-bit)` | `[sp-204] <= 0x2000 (signed, 32-bit)` |
| `beq` at 0x50c50 | `[sp-204] == 0x1000` | `[sp-204] != 0x1000` |
| `beq` at 0x50c60 | `[sp-204] == 0x4000` | `[sp-204] != 0x4000` |
| `bne` at 0x50cd4 | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |
| `bne` at 0x50cec | `ret(0x1b088) != 0x0` | `ret(0x1b088) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x50d20 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x50988).

## `TRIGSyndToBit`

- `TRIGSyndToBit`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0dfc to 0xc0e74.
  Established. The word at `sp -0x18` receives `0x7f` (`str` at 0xc0e14). A later word read has an
  unknown effective address (`ldr` at 0xc0e3c). The branch takes `0xc0e54` when `Z == 0`, while `not (Z == 0)` falls through (`bne` at 0xc0e44). Syndrome mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0e0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0e14 | store | 4 | `sp -0x18` | `0x7f` |
| `str` at 0xc0e1c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc0e20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0e30 | load | 4 | `abs 0xc0e70` | `-` |
| `ldr` at 0xc0e34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0e3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0e48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0e4c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc0e54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0e5c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc0e64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc0e6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0e6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0e6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc0e28 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc0e44 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0e6c | `r0` | `?` |

## `TRIGBitToSide`

- `TRIGBitToSide`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0e74 to 0xc0f2c.
  Established. Unsigned entry `r0 <= 0xb` takes `0xc0ea8`, while larger values fall through (`bls` at 0xc0e98). A later unsigned `r0 <= 0xf` test takes `0xc0f18`, while larger values fall through
  (`bls` at 0xc0ea4). The word at `sp -0x18` receives `0x1` on an included path (`str` at 0xc0f1c).
  Bit-side meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0e84 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0e8c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc0e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ea8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0eb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ec0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0ef0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0f08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0f1c | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xc0f20 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc0f28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0f28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0f28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc0e98 | `r0 <= 0xb (unsigned, 32-bit)` | `r0 > 0xb (unsigned, 32-bit)` |
| `bls` at 0xc0ea4 | `r0 <= 0xf (unsigned, 32-bit)` | `r0 > 0xf (unsigned, 32-bit)` |
| `bls` at 0xc0eb0 | `r0 <= 0x17 (unsigned, 32-bit)` | `r0 > 0x17 (unsigned, 32-bit)` |
| `bls` at 0xc0ebc | `r0 <= 0x19 (unsigned, 32-bit)` | `r0 > 0x19 (unsigned, 32-bit)` |
| `bls` at 0xc0ec8 | `r0 <= 0x1d (unsigned, 32-bit)` | `r0 > 0x1d (unsigned, 32-bit)` |
| `bls` at 0xc0ed4 | `r0 <= 0x2f (unsigned, 32-bit)` | `r0 > 0x2f (unsigned, 32-bit)` |
| `bls` at 0xc0ee0 | `r0 <= 0x37 (unsigned, 32-bit)` | `r0 > 0x37 (unsigned, 32-bit)` |
| `bls` at 0xc0eec | `r0 <= 0x39 (unsigned, 32-bit)` | `r0 > 0x39 (unsigned, 32-bit)` |
| `bls` at 0xc0ef8 | `r0 <= 0x3d (unsigned, 32-bit)` | `r0 > 0x3d (unsigned, 32-bit)` |
| `bls` at 0xc0f04 | `r0 <= 0x3f (unsigned, 32-bit)` | `r0 > 0x3f (unsigned, 32-bit)` |
| `bhi` at 0xc0f10 | `r0 > 0x3f (unsigned, 32-bit)` | `r0 <= 0x3f (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0f28 | `r0` | `?` |

## `TRIGFindAddrIndex`

- `TRIGFindAddrIndex`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0f2c to 0xc1008.
  Established. The word at `sp -0x28` receives `r2+1` (`str` at 0xc0f54). Included halfword and word
  reads have unknown effective addresses (`ldrh` at 0xc0f90, `ldr` at 0xc0fcc). The condition `? != r1` takes `0xc0fec`, while `? == r1` falls through (`bne` at 0xc0fe0). Address-index matching
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0f3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0f40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc0f44 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc0f48 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xc0f4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc0f54 | store | 4 | `sp -0x28` | `r2+1` |
| `str` at 0xc0f5c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0f60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0f74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0f88 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xc0f90 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xc0fa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0fb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0fc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0fcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0fd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc0fe4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0fe8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc0fec | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0ff4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0ffc | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xc1004 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc1004 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc1004 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc0f6c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc0fac | `? != r0` | `? == r0` |
| `bne` at 0xc0fe0 | `? != r1` | `? == r1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc1004 | `r0` | `?` |
