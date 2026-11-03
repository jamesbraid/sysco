# ARM SHUB configuration and field paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/shub-configuration-fields-facts.txt) retain all tool sections.

## `SHUB_Init`

- `SHUB_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24250 to 0x2470c.
  Established. Absolute stores include byte `([sp-24]:1 & 0xff)` at `0x119a24`, word `[sp-20]` at
  `0x119a28`, and byte `([sp-28]:1 & 0xff)` at `0x119a2c` (`strb` at 0x24280, `str` at 0x2428c,
  `strb` at 0x24298). A later halfword store writes `0x320` at absolute `0x280f60` (`strh` at
  0x24620). The call to `0xc8e70` has four unknown candidates (`bl` at 0x24674); its zero result
  takes the continuation, while nonzero falls through to `0x1e71c` and `0x1eed0` (`beq` at
  0x24688, `bl` at 0x24690, `bl` at 0x246cc). The initialization fields’ device meanings and
  physical effects are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24260 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24264 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x24268 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x2426c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x24270 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0x24274 | store | 1 | `sp -0x21` | `([sp+16] & 0xff)` |
| `ldr` at 0x24278 | load | 4 | `abs 0x246e4` | `-` |
| `ldrb` at 0x2427c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x24280 | store | 1 | `abs 0x119a24` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x24284 | load | 4 | `abs 0x246e8` | `-` |
| `ldr` at 0x24288 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2428c | store | 4 | `abs 0x119a28` | `[sp-20]` |
| `ldr` at 0x24290 | load | 4 | `abs 0x246ec` | `-` |
| `ldrb` at 0x24294 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x24298 | store | 1 | `abs 0x119a2c` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x2429c | load | 4 | `abs 0x246f0` | `-` |
| `ldr` at 0x242a0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x242a4 | store | 4 | `abs 0x119a38` | `[sp+4]` |
| `ldr` at 0x242a8 | load | 4 | `abs 0x246f4` | `-` |
| `ldr` at 0x242ac | load | 4 | `sp +0x8` | `-` |
| `str` at 0x242b0 | store | 4 | `abs 0x119a3c` | `[sp+8]` |
| `ldr` at 0x242b4 | load | 4 | `abs 0x246f8` | `-` |
| `ldr` at 0x242b8 | load | 4 | `sp +0xc` | `-` |
| `str` at 0x242bc | store | 4 | `abs 0x119a40` | `[sp+12]` |
| `ldr` at 0x242c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x242c4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x242d0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x242d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x242e4 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x242e8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x24304 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x24308 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x2430c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x24328 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x2432c | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24330 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x2434c | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x24350 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24354 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x2436c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x24370 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x24374 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24378 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x2439c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x243a0 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x243a4 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x243c8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x243cc | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x243d0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x243f4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x243f8 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x243fc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x24420 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x24424 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24428 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x2444c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x24450 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24454 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x24478 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x2447c | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24480 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x244a4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x244a8 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x244ac | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x244d0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x244d4 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x244d8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x244fc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x24500 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24504 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x24528 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x2452c | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24530 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x24554 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x24558 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x2455c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x24580 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x24584 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x24588 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x245ac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x245b0 | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x245b4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x245d8 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x245dc | load | 4 | `abs 0x246fc` | `-` |
| `ldr` at 0x245e0 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x24604 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x24608 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x24610 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x24618 | load | 4 | `abs 0x24700` | `-` |
| `strh` at 0x24620 | store | 2 | `abs 0x280f60` | `0x320` |
| `str` at 0x2462c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x24630 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24644 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2464c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24654 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x2465c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x24660 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24670 | load | 4 | `(unknown)` | `-` |
| `str` at 0x2467c | store | 4 | `sp -0x2c` | `ret(0xc8e70)` |
| `ldr` at 0x24680 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2468c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x24698 | load | 4 | `abs 0x24704` | `-` |
| `str` at 0x2469c | store | 4 | `sp -0x38` | `0xda5d8` |
| `ldr` at 0x246a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x246a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x246b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x246b4 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x246b8 | store | 4 | `sp -0x30` | `ret(0x1e71c)` |
| `ldr` at 0x246c0 | load | 4 | `abs 0x24708` | `-` |
| `ldr` at 0x246d0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x246d8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldmdb` at 0x246e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x246e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x246e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x242c8 | `(unknown callee) 0x2470c` | `[sp-32]` | `[sp]` | `0x119a40` | `[sp+12]` |
| `bl` at 0x24624 | `(unknown callee) 0x2acf0` | `?` | `?` | `0x280f60` | `0x320` |
| `bl` at 0x24674 | `(unknown callee) 0xc8e70` | `?` | `?` | `?` | `?` |
| `bl` at 0x24690 | `(unknown callee) 0x1e71c` | `ret(0xc8e70)` | `?` | `?` | `ret(0xc8e70)` |
| `bl` at 0x246cc | `(unknown callee) 0x1eed0` | `0x9` | `0xda5f4` | `0xa5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x242dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0x2463c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x24688 | `ret(0xc8e70) == 0x0` | `ret(0xc8e70) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x246e0 | `r0` | `?` |

## `SHUB_Set_MemCfg`

- `SHUB_Set_MemCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2470c to 0x24790.
  Established. A word store writes entry r0 at absolute `0x119a30`, followed by byte `([sp-24]:1 &
  0xff)` at `0x119a34` (`str` at 0x24744, `strb` at 0x24750). The two later branches take their
  edges for `[0x119a30] != [sp-28]` and `[0x119a34]:1 != [sp-32]`, respectively, and fall through on
  equality (`bne` at 0x24764, `bne` at 0x24778). The reached call to `0x24c08` carries entry r0,
  entry r1, unknown, unknown (`bl` at 0x24780). Memory-configuration semantics and the joined return
  remain unknown (`ldmdb` at 0x24784).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2471c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24720 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x24724 | load | 4 | `abs 0x24788` | `-` |
| `ldr` at 0x24728 | load | 4 | `abs 0x119a30` | `-` |
| `str` at 0x2472c | store | 4 | `sp -0x1c` | `[0x119a30]` |
| `ldr` at 0x24730 | load | 4 | `abs 0x2478c` | `-` |
| `ldrb` at 0x24734 | load | 1 | `abs 0x119a34` | `-` |
| `str` at 0x24738 | store | 4 | `sp -0x20` | `[0x119a34]:1` |
| `ldr` at 0x2473c | load | 4 | `abs 0x24788` | `-` |
| `ldr` at 0x24740 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24744 | store | 4 | `abs 0x119a30` | `r0` |
| `ldr` at 0x24748 | load | 4 | `abs 0x2478c` | `-` |
| `ldrb` at 0x2474c | load | 1 | `sp -0x18` | `-` |
| `strb` at 0x24750 | store | 1 | `abs 0x119a34` | `([sp-24]:1 & 0xff)` |
| `ldr` at 0x24754 | load | 4 | `abs 0x24788` | `-` |
| `ldr` at 0x24758 | load | 4 | `abs 0x119a30` | `-` |
| `ldr` at 0x2475c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24768 | load | 4 | `abs 0x2478c` | `-` |
| `ldrb` at 0x2476c | load | 1 | `abs 0x119a34` | `-` |
| `ldr` at 0x24770 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x24784 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24784 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24780 | `(unknown callee) 0x24c08` | `r0` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x24764 | `[0x119a30] != [sp-28]` | `[0x119a30] == [sp-28]` |
| `bne` at 0x24778 | `[0x119a34]:1 != [sp-32]` | `[0x119a34]:1 == [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24784 | `r0` | `?` |

## `SHUB_NICfg_Override`

- `SHUB_NICfg_Override`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24858 to 0x248a4.
  Established. The entry r0 and r1 words are saved at `sp -0x14` and `sp -0x18` (`str` at 0x24868,
  `str` at 0x2486c). The later word load has unknown effective target (`ldr` at 0x24894); its
  resulting r0 return is unknown (`ldmdb` at 0x2489c). No calls are reported. Override selection and
  the loaded value’s device role are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24868 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2486c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x24870 | load | 4 | `abs 0x248a0` | `-` |
| `ldr` at 0x24874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24888 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24894 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x2489c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2489c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2489c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2489c | `r0` | `?` |

## `SHUB_Get_NICfg`

- `SHUB_Get_NICfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x248a4 to 0x248fc.
  Established. Two word stores retain entry r0 and r1 at `sp -0x14` and `sp -0x18` (`str` at
  0x248b4, `str` at 0x248b8). The only explicit halfword load has unknown target (`ldrh` at
  0x248ec). The reported r0 return is unknown (`ldmdb` at 0x248f4). Calls and conditional branches
  are absent from the facts, and the returned configuration interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x248b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x248b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x248bc | load | 4 | `abs 0x248f8` | `-` |
| `ldr` at 0x248c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x248d4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x248ec | load | 2 | `(unknown)` | `-` |
| `ldmdb` at 0x248f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x248f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x248f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x248f4 | `r0` | `?` |

## `SHUB_Set_NICfg`

- `SHUB_Set_NICfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x248fc to 0x249f8.
  Established. Three alternative halfword stores address `[sp-36] +0x0`: the first two have unknown
  values and the third writes `([sp-26]:2 & 0xffff)` (`strh` at 0x2499c, `strh` at 0x249bc, `strh`
  at 0x249cc). Their selector tests take equality edges for `[sp-32] == 0x1`, `0x2`, `0x3`,
  respectively, and fall through on inequality (`beq` at 0x24970, `beq` at 0x24958, `beq` at
  0x24980). The final word store writes `0x1` at an unknown target (`str` at 0x249e8). Entry r0
  remains the reported return value (`ldmdb` at 0x249ec). The selector operations’ physical meaning
  is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2490c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24910 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x24914 | store | 4 | `sp -0x20` | `r3` |
| `strh` at 0x24918 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldr` at 0x2491c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2493c | load | 4 | `abs 0x249f0` | `-` |
| `str` at 0x24944 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x24948 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x2494c | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x24950 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x2495c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24968 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24978 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24988 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2498c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x24990 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldrh` at 0x24994 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x2499c | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x249a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x249a8 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x249ac | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0x249b4 | load | 2 | `[sp-36] +0x0` | `-` |
| `strh` at 0x249bc | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x249c4 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x249c8 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x249cc | store | 2 | `[sp-36] +0x0` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0x249d0 | load | 4 | `abs 0x249f4` | `-` |
| `ldr` at 0x249d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x249dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x249e8 | store | 4 | `(unknown)` | `0x1` |
| `ldmdb` at 0x249ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x249ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x249ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x24958 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x24964 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x24970 | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x24980 | `[sp-32] == 0x3` | `[sp-32] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x249ec | `r0` | `r0` |

## `SHUB_Set_MCI`

- `SHUB_Set_MCI`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x249f8 to 0x24ae0.
  Established. The absolute word `0x119a44` receives `0x1` (`str` at 0x24a24). Later stores at
  unknown targets write byte `([sp-17]:1 & 0xff)`, halfword `([sp-20]:2 & 0xffff)`, and byte
  `([sp-21]:1 & 0xff)` (`strb` at 0x24a68, `strh` at 0x24a94, `strb` at 0x24ac0). The loop test
  takes its edge for a less-or-equal flags predicate with unknown operands and falls through
  otherwise (`ble` at 0x24a38). The r0 return and MCI-field meaning are unknown (`ldmdb` at
  0x24ad4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x24a0c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x24a10 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x24a18 | store | 1 | `sp -0x15` | `(r2 & 0xff)` |
| `ldr` at 0x24a1c | load | 4 | `abs 0x24ad8` | `-` |
| `str` at 0x24a24 | store | 4 | `abs 0x119a44` | `0x1` |
| `str` at 0x24a2c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x24a30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24a40 | load | 4 | `abs 0x24adc` | `-` |
| `ldr` at 0x24a44 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24a64 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x24a68 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x24a6c | load | 4 | `abs 0x24adc` | `-` |
| `ldr` at 0x24a70 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x24a90 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x24a94 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x24a98 | load | 4 | `abs 0x24adc` | `-` |
| `ldr` at 0x24a9c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24abc | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x24ac0 | store | 1 | `(unknown)` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x24ac4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x24acc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x24ad4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24ad4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24ad4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24a38 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24ad4 | `r0` | `?` |

## `SHUB_Set_WRT90`

- `SHUB_Set_WRT90`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24b00 to 0x24be8.
  Established. A word store sets absolute `0x119a48` to `0x1` (`str` at 0x24b2c). The later byte
  stores write `([sp-17]:1 & 0xff)` and `([sp-18]:1 & 0xff)` at unknown targets (`strb` at 0x24b70,
  `strb` at 0x24b9c). A halfword store writes `([sp-20]:2 & 0xffff)` at another unknown target
  (`strh` at 0x24bc8). The deciding loop branch retains a less-or-equal flags predicate with unknown
  operands, following its edge when that predicate holds and falling through otherwise (`ble` at
  0x24b40). The physical timing-field interpretation is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x24b14 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x24b1c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x24b20 | store | 2 | `sp -0x14` | `(r2 & 0xffff)` |
| `ldr` at 0x24b24 | load | 4 | `abs 0x24be0` | `-` |
| `str` at 0x24b2c | store | 4 | `abs 0x119a48` | `0x1` |
| `str` at 0x24b34 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x24b38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24b48 | load | 4 | `abs 0x24be4` | `-` |
| `ldr` at 0x24b4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x24b6c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x24b70 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x24b74 | load | 4 | `abs 0x24be4` | `-` |
| `ldr` at 0x24b78 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x24b98 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x24b9c | store | 1 | `(unknown)` | `([sp-18]:1 & 0xff)` |
| `ldr` at 0x24ba0 | load | 4 | `abs 0x24be4` | `-` |
| `ldr` at 0x24ba4 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x24bc4 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x24bc8 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x24bcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x24bd4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x24bdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24bdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24bdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24b40 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24bdc | `r0` | `?` |

## `SHUB_Set_FSBCfg`

- `SHUB_Set_FSBCfg`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24d9c to 0x24f50.
  Established. The absolute word store writes `0x1` at `0x119a4c` (`str` at 0x24dfc). The seven
  later byte stores have unknown targets and values `([sp-25]:1 & 0xff)`, `([sp-26]:1 & 0xff)`,
  `([sp-27]:1 & 0xff)`, `([sp-28]:1 & 0xff)`, `([sp-29]:1 & 0xff)`, `([sp-30]:1 & 0xff)`,
  `([sp-31]:1 & 0xff)`, respectively (`strb` at 0x24e38, `strb` at 0x24e5c, `strb` at 0x24e88,
  `strb` at 0x24eb4, `strb` at 0x24ee0, `strb` at 0x24f0c, `strb` at 0x24f30). The loop branch takes
  its edge for a less-or-equal flags predicate with unknown operands and falls through otherwise
  (`ble` at 0x24e10). The clock-field meanings and r0 return remain unknown (`ldmdb` at 0x24f44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x24db0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x24db4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x24db8 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x24dc0 | store | 1 | `sp -0x19` | `(r0 & 0xff)` |
| `strb` at 0x24dc8 | store | 1 | `sp -0x1a` | `(r1 & 0xff)` |
| `strb` at 0x24dd0 | store | 1 | `sp -0x1b` | `(r2 & 0xff)` |
| `strb` at 0x24dd8 | store | 1 | `sp -0x1c` | `(r3 & 0xff)` |
| `strb` at 0x24de0 | store | 1 | `sp -0x1d` | `([sp] & 0xff)` |
| `strb` at 0x24de8 | store | 1 | `sp -0x1e` | `([sp+4] & 0xff)` |
| `strb` at 0x24df0 | store | 1 | `sp -0x1f` | `([sp+8] & 0xff)` |
| `ldr` at 0x24df4 | load | 4 | `abs 0x24f48` | `-` |
| `str` at 0x24dfc | store | 4 | `abs 0x119a4c` | `0x1` |
| `str` at 0x24e04 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x24e08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24e18 | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24e1c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24e34 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x24e38 | store | 1 | `(unknown)` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0x24e3c | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24e40 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24e58 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x24e5c | store | 1 | `(unknown)` | `([sp-26]:1 & 0xff)` |
| `ldr` at 0x24e60 | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24e64 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24e84 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x24e88 | store | 1 | `(unknown)` | `([sp-27]:1 & 0xff)` |
| `ldr` at 0x24e8c | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24e90 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24eb0 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x24eb4 | store | 1 | `(unknown)` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x24eb8 | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24ebc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24edc | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x24ee0 | store | 1 | `(unknown)` | `([sp-29]:1 & 0xff)` |
| `ldr` at 0x24ee4 | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24ee8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24f08 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x24f0c | store | 1 | `(unknown)` | `([sp-30]:1 & 0xff)` |
| `ldr` at 0x24f10 | load | 4 | `abs 0x24f4c` | `-` |
| `ldr` at 0x24f14 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24f2c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x24f30 | store | 1 | `(unknown)` | `([sp-31]:1 & 0xff)` |
| `ldr` at 0x24f34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x24f3c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x24f44 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x24f44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x24f44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x24f44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x24f44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x24e10 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x24f44 | `r0` | `?` |

## `SHUB_SetClkstop`

- `SHUB_SetClkstop`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29098 to 0x29108.
  Established. An explicit byte store at an unknown target writes `([sp-20]:1 & 0xff)` (`strb` at
  0x290ec). Stack word `sp -0x18` starts at `0x0` and later receives `[sp-24]+1` (`str` at 0x290b0,
  `str` at 0x290f8). The loop test takes its edge for a less-or-equal flags predicate with unknown
  operands and falls through otherwise (`ble` at 0x290bc). The reported r0 return and clock-stop
  behavior are unknown (`ldmdb` at 0x29100).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x290a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x290b0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x290b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x290c4 | load | 4 | `abs 0x29104` | `-` |
| `ldr` at 0x290c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x290e8 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x290ec | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x290f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x290f8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x29100 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29100 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x290bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29100 | `r0` | `?` |

## `SHUB_GetClkstop`

- `SHUB_GetClkstop`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29108 to 0x29128.
  Established. A byte load reads absolute `0x280ef5` (`ldrb` at 0x29118). The value moves to r0 and
  reaches the return as `[0x280ef5]:1` (`mov` at 0x2911c, `ldmdb` at 0x29120). The facts report no
  calls or explicit stores. The byte’s clock-stop status interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x29114 | load | 4 | `abs 0x29124` | `-` |
| `ldrb` at 0x29118 | load | 1 | `abs 0x280ef5` | `-` |
| `ldmdb` at 0x29120 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29120 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29120 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29120 | `r0` | `[0x280ef5]:1` |

## `SHUB_SetTP`

- `SHUB_SetTP`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29128 to 0x29198.
  Established. The byte store writes `([sp-20]:1 & 0xff)` at an unknown target (`strb` at 0x2917c).
  Two stack word stores initialize `sp -0x18` to `0x0` and later put `[sp-24]+1` there (`str` at
  0x29140, `str` at 0x29188). The loop branch retains a less-or-equal flags predicate with unknown
  operands, following its edge for that predicate and falling through otherwise (`ble` at 0x2914c). No
  calls are reported, and the physical TP operation is unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x29138 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x29140 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x29144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x29154 | load | 4 | `abs 0x29194` | `-` |
| `ldr` at 0x29158 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x29178 | load | 1 | `sp -0x14` | `-` |
| `strb` at 0x2917c | store | 1 | `(unknown)` | `([sp-20]:1 & 0xff)` |
| `ldr` at 0x29180 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x29188 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x29190 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x29190 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x29190 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x2914c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x29190 | `r0` | `?` |

## `SHUB_GetTP`

- `SHUB_GetTP`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x29198 to 0x291b8.
  Established. The byte at absolute `0x280ef4` is loaded, moved to r0 and returned as `[0x280ef4]:1`
  (`ldrb` at 0x291a8, `mov` at 0x291ac, `ldmdb` at 0x291b0). No calls or conditional branches are
  reported. The value’s hardware-field interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x291a4 | load | 4 | `abs 0x291b4` | `-` |
| `ldrb` at 0x291a8 | load | 1 | `abs 0x280ef4` | `-` |
| `ldmdb` at 0x291b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x291b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x291b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x291b0 | `r0` | `[0x280ef4]:1` |

## `SHUB_Rev`

- `SHUB_Rev`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28d34 to 0x28d68.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0x28d44). The following indexed
  word read has unknown effective target (`ldr` at 0x28d50). The return reports unknown r0 (`ldmdb`
  at 0x28d60). No calls are reported, and a chip-revision encoding is not established by these
  facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x28d44 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x28d48 | load | 4 | `abs 0x28d64` | `-` |
| `ldr` at 0x28d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x28d50 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x28d60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x28d60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x28d60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x28d60 | `r0` | `?` |

## `shub_set_link_enable`

- `shub_set_link_enable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a5d4 to 0x2a790.
  Established. Two calls to `0xcb8f4` carry `[sp-20]`, `0x3`, `[sp-24]+9`, `0x0` (`bl` at 0x2a6d4,
  `bl` at 0x2a740). Each is followed by a call to `0xc2d08` with `ret(0xcb8f4)`, unknown, unknown,
  `ret(0xcb8f4)` (`bl` at 0x2a6e0, `bl` at 0x2a74c). The corresponding signed tests take their
  continuation for `[sp-44] >= 0x0` and fall through below zero (`bge` at 0x2a6ec, `bge` at
  0x2a758). Stack word `sp -0x20` is set to `0x1` before the earlier sequence and `0x0` before the
  later one (`str` at 0x2a674, `str` at 0x2a700). Link-enable actions and timing are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a5e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2a5e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x2a5ec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x2a5f0 | load | 4 | `abs 0x2a778` | `-` |
| `ldr` at 0x2a5f4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a600 | load | 4 | `abs 0x2a77c` | `-` |
| `ldr` at 0x2a604 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a614 | load | 4 | `abs 0x2a780` | `-` |
| `ldr` at 0x2a618 | load | 4 | `abs 0x119f84` | `-` |
| `ldr` at 0x2a624 | load | 4 | `abs 0x2a77c` | `-` |
| `str` at 0x2a628 | store | 4 | `sp -0x48` | `0xdb578` |
| `ldr` at 0x2a62c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a630 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x2a634 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x2a638 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x2a63c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a640 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x2a648 | load | 4 | `abs 0x2a784` | `-` |
| `ldr` at 0x2a64c | load | 4 | `abs 0x2a788` | `-` |
| `ldr` at 0x2a658 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2a668 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x2a674 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x2a678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a698 | load | 4 | `abs 0x2a778` | `-` |
| `ldr` at 0x2a69c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0x2a6a8 | load | 4 | `abs 0x2a78c` | `-` |
| `ldr` at 0x2a6ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a6b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2a6bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a6c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a6dc | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x2a6e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2a6f0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x2a6f4 | store | 4 | `sp -0x38` | `[sp-44]` |
| `str` at 0x2a700 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x2a704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a710 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a730 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2a748 | store | 4 | `sp -0x2c` | `ret(0xcb8f4)` |
| `ldr` at 0x2a750 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x2a75c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x2a760 | store | 4 | `sp -0x38` | `[sp-44]` |
| `str` at 0x2a76c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x2a770 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x2a774 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a774 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a774 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a610 | `(unknown callee) 0x20f14` | `0xdb578` | `r0` | `r1` | `r2` |
| `bl` at 0x2a654 | `(unknown callee) 0x1eed0` | `0x6` | `0xda5f4` | `0x7df` | `0x0` |
| `bl` at 0x2a694 | `(unknown callee) 0xcb020` | `?` | `?` | `0x0` | `sp-32` |
| `bl` at 0x2a6b4 | `(unknown callee) 0x20f14` | `0xdb5b0` | `[sp-24]` | `[sp-28]` | `[0x27eb98]` |
| `bl` at 0x2a6b8 | `(unknown callee) 0xc2c9c` | `?` | `?` | `?` | `?` |
| `bl` at 0x2a6d4 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x3` | `[sp-24]+9` | `0x0` |
| `bl` at 0x2a6e0 | `(unknown callee) 0xc2d08` | `ret(0xcb8f4)` | `?` | `?` | `ret(0xcb8f4)` |
| `bl` at 0x2a720 | `(unknown callee) 0xcb020` | `[sp-20]` | `[sp-24]+9` | `0x0` | `sp-32` |
| `bl` at 0x2a724 | `(unknown callee) 0xc2c9c` | `ret(0xcb020)` | `?` | `?` | `?` |
| `bl` at 0x2a740 | `(unknown callee) 0xcb8f4` | `[sp-20]` | `0x3` | `[sp-24]+9` | `0x0` |
| `bl` at 0x2a74c | `(unknown callee) 0xc2d08` | `ret(0xcb8f4)` | `?` | `?` | `ret(0xcb8f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a5fc | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `beq` at 0x2a620 | `[0x119f84] == 0x0` | `[0x119f84] != 0x0` |
| `bne` at 0x2a660 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x2a6a4 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x2a6ec | `[sp-44] >= 0x0 (signed, 32-bit)` | `[sp-44] < 0x0 (signed, 32-bit)` |
| `bge` at 0x2a758 | `[sp-44] >= 0x0 (signed, 32-bit)` | `[sp-44] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a774 | `r0` | `?` |

## `SHUB_WarCheck`

- `SHUB_WarCheck`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2a790 to 0x2a830.
  Established. Stack words retain `[0x119c00]:1` at `sp -0x20` and `[0x119c01]:1` at `sp -0x1c`
  (`str` at 0x2a7f4, `str` at 0x2a800). The branch takes its bypass for `? == 0x0` and falls through
  on inequality (`beq` at 0x2a7e0). The only call targets `0x1eed0` with `0x1`, `0xda5f4`, `0x839`,
  `0x0` (`bl` at 0x2a814). The joined r0 return and workaround-check interpretation remain unknown
  (`ldmdb` at 0x2a818).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2a7a0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x2a7a4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x2a7a8 | load | 4 | `abs 0x2a81c` | `-` |
| `ldrb` at 0x2a7ac | load | 1 | `abs 0x119c00` | `-` |
| `ldr` at 0x2a7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x2a7c0 | load | 4 | `abs 0x2a820` | `-` |
| `ldrb` at 0x2a7c4 | load | 1 | `abs 0x119c01` | `-` |
| `ldr` at 0x2a7c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2a7e4 | load | 4 | `abs 0x2a824` | `-` |
| `str` at 0x2a7e8 | store | 4 | `sp -0x24` | `0xdb5e4` |
| `ldr` at 0x2a7ec | load | 4 | `abs 0x2a81c` | `-` |
| `ldrb` at 0x2a7f0 | load | 1 | `abs 0x119c00` | `-` |
| `str` at 0x2a7f4 | store | 4 | `sp -0x20` | `[0x119c00]:1` |
| `ldr` at 0x2a7f8 | load | 4 | `abs 0x2a820` | `-` |
| `ldrb` at 0x2a7fc | load | 1 | `abs 0x119c01` | `-` |
| `str` at 0x2a800 | store | 4 | `sp -0x1c` | `[0x119c01]:1` |
| `ldr` at 0x2a808 | load | 4 | `abs 0x2a828` | `-` |
| `ldr` at 0x2a80c | load | 4 | `abs 0x2a82c` | `-` |
| `ldmdb` at 0x2a818 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2a818 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2a818 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2a814 | `(unknown callee) 0x1eed0` | `0x1` | `0xda5f4` | `0x839` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2a7e0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2a818 | `r0` | `?` |
