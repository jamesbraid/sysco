# ARM TIO, PCI-X and system reset paths in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
One routine has unresolved computed-transfer destinations; its tables and return cover only known edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/tio-pcix-system-reset-facts.txt) retain all tool sections.

## `TIOJTAGInit`

- `TIOJTAGInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x534f4 to 0x536c0.
  Established. Byte stores write `0x1` at `0x148354` and, at two later sites, `0x0` at that same
  address (`strb` at 0x53524, `strb` at 0x535d4, `strb` at 0x535f4). The call to `0xd6b9c` uses
  `0x0`, unknown, `sp-17`, unknown (`bl` at 0x53580); its zero result takes the continuation and
  nonzero falls through (`beq` at 0x53594). TIO JTAG wiring and r0 at exit remain unestablished
  (`ldmdb` at 0x5369c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53518 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5351c | load | 4 | `abs 0x536a0` | `-` |
| `strb` at 0x53524 | store | 1 | `abs 0x148354` | `0x1` |
| `ldr` at 0x53528 | load | 4 | `abs 0x536a4` | `-` |
| `ldrb` at 0x5352c | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5353c | load | 4 | `abs 0x536a0` | `-` |
| `ldrb` at 0x53540 | load | 1 | `abs 0x148354` | `-` |
| `ldr` at 0x53550 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53564 | load | 4 | `abs 0x536a8` | `-` |
| `ldr` at 0x53568 | load | 4 | `abs 0x148350` | `-` |
| `ldrb` at 0x53570 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53588 | store | 4 | `sp -0x1c` | `ret(0xd6b9c)` |
| `ldr` at 0x5358c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x535a4 | load | 4 | `abs 0x536ac` | `-` |
| `str` at 0x535a8 | store | 4 | `sp -0x28` | `0x10b314` |
| `ldr` at 0x535ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x535b0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x535b4 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x535bc | load | 4 | `abs 0x536b0` | `-` |
| `ldr` at 0x535c0 | load | 4 | `abs 0x536b4` | `-` |
| `ldr` at 0x535cc | load | 4 | `abs 0x536a0` | `-` |
| `strb` at 0x535d4 | store | 1 | `abs 0x148354` | `0x0` |
| `ldrb` at 0x535dc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x535ec | load | 4 | `abs 0x536a0` | `-` |
| `strb` at 0x535f4 | store | 1 | `abs 0x148354` | `0x0` |
| `ldr` at 0x535f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53600 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x53608 | load | 4 | `abs 0x536a0` | `-` |
| `ldrb` at 0x5360c | load | 1 | `abs 0x148354` | `-` |
| `str` at 0x5361c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x53620 | load | 4 | `abs 0x536a4` | `-` |
| `ldrb` at 0x53624 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53628 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x53638 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x53648 | store | 4 | `sp -0x1c` | `ret(0x54664)` |
| `ldr` at 0x5364c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53664 | load | 4 | `abs 0x536b8` | `-` |
| `str` at 0x53668 | store | 4 | `sp -0x28` | `0x10b334` |
| `ldr` at 0x5366c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53670 | store | 4 | `sp -0x24` | `[sp-24]` |
| `str` at 0x53674 | store | 4 | `sp -0x20` | `ret(0x219a8)` |
| `ldr` at 0x5367c | load | 4 | `abs 0x536b0` | `-` |
| `ldr` at 0x53680 | load | 4 | `abs 0x536bc` | `-` |
| `ldr` at 0x5368c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53694 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x5369c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5369c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5369c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53504 | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x53580 | `(unknown callee) 0xd6b9c` | `0x0` | `?` | `sp-17` | `?` |
| `bl` at 0x5359c | `(unknown callee) 0x219a8` | `ret(0xd6b9c)` | `?` | `?` | `ret(0xd6b9c)` |
| `bl` at 0x535c8 | `(unknown callee) 0x21e28` | `0x1` | `0x10b2c8` | `0x107` | `0x0` |
| `bl` at 0x53640 | `(unknown callee) 0x54664` | `[sp-24]:1` | `?` | `[0x2b08a0]:1` | `[sp-24]:1` |
| `bl` at 0x5365c | `(unknown callee) 0x219a8` | `ret(0x54664)` | `?` | `?` | `ret(0x54664)` |
| `bl` at 0x53688 | `(unknown callee) 0x21e28` | `0x1` | `0x10b2c8` | `0x113` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x53510 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bge` at 0x53538 | `[sp-24] >= [0x2b08a0]:1 (signed, 32-bit)` | `[sp-24] < [0x2b08a0]:1 (signed, 32-bit)` |
| `bne` at 0x53548 | `[0x148354]:1 != 0x0` | `[0x148354]:1 == 0x0` |
| `beq` at 0x53594 | `ret(0xd6b9c) == 0x0` | `ret(0xd6b9c) != 0x0` |
| `bne` at 0x535e8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x53614 | `[0x148354]:1 == 0x0` | `[0x148354]:1 != 0x0` |
| `blt` at 0x53630 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x53654 | `ret(0x54664) == 0x0` | `ret(0x54664) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5369c | `r0` | `?` |

## `TIOResetHold`

- `TIOResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53708 to 0x53914.
  Established. The byte store clears `0x148354` to `0x0` (`strb` at 0x53748). Calls include
  `0xd6a1c` with `0x0`, `[[sp-24]]:1`, `0x60`, `0x0` (`bl` at 0x5383c); its zero result takes the
  continuation and nonzero falls through (`beq` at 0x53850). The last call reaches `0x58bb8` with
  `0x0`, `0xff`, `0xff`, unknown (`bl` at 0x538c4). Reset assertion and the common r0 result remain
  unknown (`ldmdb` at 0x538fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53718 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x53720 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x53724 | load | 4 | `abs 0x53900` | `-` |
| `ldr` at 0x53728 | load | 4 | `abs 0x148350` | `-` |
| `ldr` at 0x53734 | load | 4 | `abs 0x53904` | `-` |
| `str` at 0x53738 | store | 4 | `sp -0x2c` | `0xfffff768` |
| `ldr` at 0x53740 | load | 4 | `abs 0x53908` | `-` |
| `strb` at 0x53748 | store | 1 | `abs 0x148354` | `0x0` |
| `str` at 0x53750 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x53754 | load | 4 | `abs 0x5390c` | `-` |
| `ldrb` at 0x53758 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x5375c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x5376c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x53774 | load | 4 | `abs 0x53910` | `-` |
| `ldr` at 0x53778 | load | 4 | `abs 0x148358` | `-` |
| `ldrb` at 0x53780 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x5378c | load | 4 | `abs 0x53900` | `-` |
| `ldr` at 0x53790 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x537a4 | load | 4 | `abs 0x148350` | `-` |
| `str` at 0x537ac | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x537b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x537b4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x537cc | store | 4 | `sp -0x20` | `ret(0xd6cf8)` |
| `ldr` at 0x537d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x537dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x537e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x537ec | store | 4 | `sp -0x1c` | `ret(0xd6cf8)` |
| `ldr` at 0x537f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53800 | store | 4 | `sp -0x20` | `ret(0x561cc)` |
| `ldr` at 0x53804 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53810 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5381c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53820 | store | 4 | `sp -0x1c` | `ret(0x561cc)` |
| `ldr` at 0x53824 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x53828 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x53844 | store | 4 | `sp -0x20` | `ret(0xd6a1c)` |
| `ldr` at 0x53848 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53860 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x53864 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x53868 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5386c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x53884 | store | 4 | `sp -0x20` | `ret(0xd69d4)` |
| `ldr` at 0x53888 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53894 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x538a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x538a4 | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x538a8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x538b0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x538cc | store | 4 | `sp -0x20` | `ret(0x58bb8)` |
| `ldr` at 0x538d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x538dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x538e8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x538ec | store | 4 | `sp -0x1c` | `ret(0x58bb8)` |
| `ldr` at 0x538f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x538f4 | store | 4 | `sp -0x2c` | `[sp-28]` |
| `ldr` at 0x538f8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x538fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x538fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x538fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x537c4 | `(unknown callee) 0xd6cf8` | `0x0` | `?` | `0x3` | `?` |
| `bl` at 0x537f8 | `(unknown callee) 0x561cc` | `[sp-24]` | `0x0` | `?` | `?` |
| `bl` at 0x5383c | `(unknown callee) 0xd6a1c` | `0x0` | `[[sp-24]]:1` | `0x60` | `0x0` |
| `bl` at 0x5387c | `(unknown callee) 0xd69d4` | `0x0` | `[[sp-24]]:1` | `0x4` | `[[sp-24]]:1` |
| `bl` at 0x538c4 | `(unknown callee) 0x58bb8` | `0x0` | `0xff` | `0xff` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x53730 | `[0x148350] != 0x0` | `[0x148350] == 0x0` |
| `blt` at 0x53764 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53788 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x537d8 | `ret(0xd6cf8) == 0x0` | `ret(0xd6cf8) != 0x0` |
| `bne` at 0x537e4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x5380c | `ret(0x561cc) == 0x0` | `ret(0x561cc) != 0x0` |
| `bne` at 0x53818 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x53850 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `bne` at 0x5385c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x53890 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `bne` at 0x5389c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x538d8 | `ret(0x58bb8) == 0x0` | `ret(0x58bb8) != 0x0` |
| `bne` at 0x538e4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x538fc | `r0` | `?` |

## `TIOResetRelease`

- `TIOResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53914 to 0x53bf0.
  Established. The call to `0x5964c` has `0x0`, `0x0`, unknown, `ret(0x59534)` (`bl` at 0x53990).
  Later `0x58e8c` receives `0x0`, `0xff`, `0xff`, `ret(0x5964c)` (`bl` at 0x53af4). A following byte
  store writes `0x1` at `0x148354` (`strb` at 0x53b20). Physical release ordering and r0 at the
  joined exit remain unresolved (`ldmdb` at 0x53bd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53924 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x53928 | load | 4 | `abs 0x53bdc` | `-` |
| `ldr` at 0x5392c | load | 4 | `abs 0x148350` | `-` |
| `ldr` at 0x53938 | load | 4 | `abs 0x53be0` | `-` |
| `str` at 0x5393c | store | 4 | `sp -0x20` | `0xfffff768` |
| `str` at 0x53958 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x5396c | store | 4 | `sp -0x18` | `ret(0x59534)` |
| `ldr` at 0x53970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5397c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53980 | store | 4 | `sp -0x20` | `ret(0x59534)` |
| `str` at 0x53998 | store | 4 | `sp -0x18` | `ret(0x5964c)` |
| `ldr` at 0x5399c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x539a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x539ac | store | 4 | `sp -0x20` | `ret(0x5964c)` |
| `str` at 0x539b8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x539bc | load | 4 | `abs 0x53be4` | `-` |
| `ldrb` at 0x539c0 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x539c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x539d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x539dc | load | 4 | `abs 0x53be8` | `-` |
| `ldr` at 0x539e0 | load | 4 | `abs 0x148358` | `-` |
| `ldrb` at 0x539e8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x539f8 | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x53a08 | store | 4 | `sp -0x18` | `ret(0x548d0)` |
| `ldr` at 0x53a10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53a1c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53a20 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x53a28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53a30 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x53a3c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x53a40 | load | 4 | `abs 0x53be4` | `-` |
| `ldrb` at 0x53a44 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53a48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53a58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53a60 | load | 4 | `abs 0x53be8` | `-` |
| `ldr` at 0x53a64 | load | 4 | `abs 0x148358` | `-` |
| `ldrb` at 0x53a6c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x53a7c | load | 1 | `sp -0x1c` | `-` |
| `str` at 0x53a8c | store | 4 | `sp -0x18` | `ret(0x55568)` |
| `ldr` at 0x53a94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53aa0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53aa4 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x53aac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53ab4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x53acc | store | 4 | `sp -0x18` | `ret(0x5964c)` |
| `ldr` at 0x53ad0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53adc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53ae0 | store | 4 | `sp -0x20` | `ret(0x5964c)` |
| `str` at 0x53afc | store | 4 | `sp -0x18` | `ret(0x58e8c)` |
| `ldr` at 0x53b00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53b0c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53b10 | store | 4 | `sp -0x20` | `ret(0x58e8c)` |
| `ldr` at 0x53b18 | load | 4 | `abs 0x53bec` | `-` |
| `strb` at 0x53b20 | store | 1 | `abs 0x148354` | `0x1` |
| `str` at 0x53b28 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x53b2c | load | 4 | `abs 0x53be4` | `-` |
| `ldrb` at 0x53b30 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53b34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53b44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53b4c | load | 4 | `abs 0x53be8` | `-` |
| `ldr` at 0x53b50 | load | 4 | `abs 0x148358` | `-` |
| `ldrb` at 0x53b58 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x53b64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x53b78 | load | 4 | `abs 0x53bdc` | `-` |
| `ldr` at 0x53b7c | load | 4 | `abs 0x148350` | `-` |
| `ldrb` at 0x53b84 | load | 1 | `(unknown)` | `-` |
| `str` at 0x53ba0 | store | 4 | `sp -0x18` | `ret(0xd6d88)` |
| `ldr` at 0x53ba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x53bb0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x53bb4 | store | 4 | `sp -0x20` | `ret(0xd6d88)` |
| `ldr` at 0x53bbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x53bc4 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x53bd0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x53bd4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x53bd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53bd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53bd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53944 | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `[0x148350]` |
| `bl` at 0x53964 | `(unknown callee) 0x59534` | `0x0` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x53990 | `(unknown callee) 0x5964c` | `0x0` | `0x0` | `?` | `ret(0x59534)` |
| `bl` at 0x539f4 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x53a00 | `(unknown callee) 0x548d0` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x53a0c | `(unknown callee) 0xe33c8` | `ret(0x548d0)` | `?` | `?` | `ret(0x548d0)` |
| `bl` at 0x53a78 | `(unknown callee) 0xe335c` | `?` | `?` | `?` | `?` |
| `bl` at 0x53a84 | `(unknown callee) 0x55568` | `[sp-28]:1` | `?` | `?` | `[sp-28]:1` |
| `bl` at 0x53a90 | `(unknown callee) 0xe33c8` | `ret(0x55568)` | `?` | `?` | `ret(0x55568)` |
| `bl` at 0x53ac4 | `(unknown callee) 0x5964c` | `0x0` | `0x1` | `[0x2b08a0]:1` | `?` |
| `bl` at 0x53af4 | `(unknown callee) 0x58e8c` | `0x0` | `0xff` | `0xff` | `ret(0x5964c)` |
| `bl` at 0x53b98 | `(unknown callee) 0xd6d88` | `0x0` | `?` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x53934 | `[0x148350] != 0x0` | `[0x148350] == 0x0` |
| `bne` at 0x53950 | `ret(0x3d8e0) != 0x0` | `ret(0x3d8e0) == 0x0` |
| `beq` at 0x53978 | `ret(0x59534) == 0x0` | `ret(0x59534) != 0x0` |
| `beq` at 0x539a4 | `ret(0x5964c) == 0x0` | `ret(0x5964c) != 0x0` |
| `blt` at 0x539cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x539f0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53a18 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `blt` at 0x53a50 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53a74 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53a9c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x53ad8 | `ret(0x5964c) == 0x0` | `ret(0x5964c) != 0x0` |
| `beq` at 0x53b08 | `ret(0x58e8c) == 0x0` | `ret(0x58e8c) != 0x0` |
| `blt` at 0x53b3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53b60 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53bac | `ret(0xd6d88) == 0x0` | `ret(0xd6d88) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53bd8 | `r0` | `?` |

## `TIOPciResetInt`

- `TIOPciResetInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x53e88 to 0x53fa0.
  Established. The three entry-byte saves retain `(r1 & 0xff)`, `(r2 & 0xff)` and `(r3 & 0xff)` at
  `sp -0x15`, `sp -0x16` and `sp -0x17`, respectively (`strb` at 0x53ea4, `strb` at 0x53eac, `strb`
  at 0x53eb4). Both calls to `0x5625c` have four unknown candidates (`bl` at 0x53f64, `bl` at
  0x53f84). The byte read before the first call has unknown target (`ldrb` at 0x53f5c). PCI reset-line
  selection and the exit r0 are unidentified (`ldmdb` at 0x53f88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x53e98 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x53ea4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x53eac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x53eb4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x53ebc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x53ec0 | load | 4 | `abs 0x53f8c` | `-` |
| `ldrb` at 0x53ec4 | load | 1 | `abs 0x2b08a0` | `-` |
| `ldr` at 0x53ec8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53ed4 | load | 4 | `abs 0x53f90` | `-` |
| `str` at 0x53ed8 | store | 4 | `sp -0x28` | `0x10b3b4` |
| `ldrb` at 0x53edc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x53ee0 | store | 4 | `sp -0x24` | `(r1 & 0xff)` |
| `ldr` at 0x53ee8 | load | 4 | `abs 0x53f94` | `-` |
| `ldr` at 0x53eec | load | 4 | `abs 0x53f98` | `-` |
| `ldr` at 0x53efc | load | 4 | `abs 0x53f9c` | `-` |
| `ldr` at 0x53f04 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x53f20 | load | 4 | `abs 0x148350` | `-` |
| `str` at 0x53f28 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x53f30 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x53f34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53f38 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x53f3c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x53f48 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x53f58 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53f5c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x53f68 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x53f78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x53f7c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x53f88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x53f88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x53f88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x53ef4 | `(unknown callee) 0x21e28` | `0x1` | `0x10b2c8` | `0x265` | `0x0` |
| `bl` at 0x53f64 | `(unknown callee) 0x5625c` | `?` | `?` | `?` | `?` |
| `bl` at 0x53f84 | `(unknown callee) 0x5625c` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x53ed0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x53f44 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x53f54 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x53f74 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x53f88 | `r0` | `?` |

## `pcixResetHold`

- `pcixResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58bb8 to 0x58e8c.
  Established. The byte store in the indexed body has unknown target and value (`strb` at 0x58d24).
  The call to `0x5b20c` uses `[sp-20]`, `[sp-21]:1`, `[sp-22]:1`, `[sp-21]:1` (`bl` at 0x58e28); its
  nonzero result takes the continuation and zero falls through (`bne` at 0x58e3c). The final call to
  `0x5a5d0` has `[sp-20]`, `ret(0x5b20c)`, `0x0`, `ret(0x5b20c)` (`bl` at 0x58e6c). Physical PCI-X
  reset coverage and r0 at exit remain unknown (`ldmdb` at 0x58e7c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58bc8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58bd0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58bd8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x58be0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58be4 | load | 4 | `abs 0x58e80` | `-` |
| `ldr` at 0x58be8 | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x58bf4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58c00 | load | 4 | `abs 0x58e84` | `-` |
| `str` at 0x58c04 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x58c10 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58c14 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x58c18 | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x58c20 | load | 4 | `abs 0x58e88` | `-` |
| `ldr` at 0x58c24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58c38 | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x58c40 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58c44 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58c54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58c58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58c5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58c60 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x58c70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58c74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58c7c | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58c84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58c98 | store | 4 | `sp -0x2c` | `ret(0x5a5d0)` |
| `ldr` at 0x58c9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58ca8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58cb4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58cb8 | store | 4 | `sp -0x28` | `ret(0x5a5d0)` |
| `ldr` at 0x58cbc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58cc4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `str` at 0x58cd0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58cd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58cd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58cdc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x58cec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58cf0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58cf8 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x58d00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58d04 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58d18 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x58d20 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x58d24 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x58d28 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58d30 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58d38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58d3c | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58d44 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58d50 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58d54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58d64 | store | 4 | `sp -0x20` | `ret(0x5b130)` |
| `ldr` at 0x58d68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x58d74 | load | 4 | `abs 0x58e84` | `-` |
| `str` at 0x58d78 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58d80 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58d84 | load | 1 | `ret(0x5b130) +0x1` | `-` |
| `str` at 0x58d88 | store | 4 | `sp -0x30` | `[ret(0x5b130)+1]:1` |
| `ldr` at 0x58d8c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58d90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58d94 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x58d98 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x58da0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58db0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58db4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58dbc | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x58dc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58dd8 | store | 4 | `sp -0x2c` | `ret(0x5a5d0)` |
| `ldr` at 0x58ddc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58de8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58df4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58df8 | store | 4 | `sp -0x28` | `ret(0x5a5d0)` |
| `ldr` at 0x58dfc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58e04 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58e0c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58e10 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58e18 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58e1c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58e20 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58e30 | store | 4 | `sp -0x24` | `ret(0x5b20c)` |
| `ldr` at 0x58e34 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x58e40 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x58e44 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x58e48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58e58 | store | 4 | `sp -0x34` | `ret(0x5b3ec)` |
| `ldr` at 0x58e60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58e64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x58e74 | store | 4 | `sp -0x34` | `ret(0x5a5d0)` |
| `ldr` at 0x58e78 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x58e7c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x58e7c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x58e7c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58c90 | `(unknown callee) 0x5a5d0` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58d5c | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58dd0 | `(unknown callee) 0x5a5d0` | `[sp-20]` | `?` | `0x1` | `?` |
| `bl` at 0x58e28 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58e50 | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x58e6c | `(unknown callee) 0x5a5d0` | `[sp-20]` | `ret(0x5b20c)` | `0x0` | `ret(0x5b20c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58bf0 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x58bfc | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58c4c | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58c68 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58ca4 | `ret(0x5a5d0) == 0x0` | `ret(0x5a5d0) != 0x0` |
| `bne` at 0x58cb0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `blt` at 0x58ce4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x58d4c | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58d70 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `blt` at 0x58da8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58de4 | `ret(0x5a5d0) == 0x0` | `ret(0x5a5d0) != 0x0` |
| `bne` at 0x58df0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58e3c | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x58e7c | `r0` | `?` |

## `pcixResetRelease`

- `pcixResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x58e8c to 0x59094.
  Established. The call to `0x5b130` carries `[sp-20]`, `[sp-21]:1`, unknown, `[sp-21]:1` (`bl` at
  0x58fcc); its nonzero result takes the continuation and zero falls through (`bne` at 0x58fe0). The
  final call to `0x5aba8` uses `[sp-20]`, `ret(0x5b20c)`, unknown, `ret(0x5b20c)` (`bl` at 0x59074).
  Release mechanics and the common r0 value are not established (`ldmdb` at 0x59084).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x58e9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x58ea4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x58eac | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x58eb4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x58eb8 | load | 4 | `abs 0x59088` | `-` |
| `ldr` at 0x58ebc | load | 4 | `abs 0x148360` | `-` |
| `ldrb` at 0x58ec8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58ed4 | load | 4 | `abs 0x5908c` | `-` |
| `str` at 0x58ed8 | store | 4 | `sp -0x38` | `0xfffff703` |
| `str` at 0x58ee4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x58ee8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x58eec | store | 4 | `sp -0x34` | `[sp-56]` |
| `ldr` at 0x58ef4 | load | 4 | `abs 0x59090` | `-` |
| `ldr` at 0x58ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58f0c | load | 4 | `abs 0x14835c` | `-` |
| `str` at 0x58f14 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x58f18 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x58f28 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x58f2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f30 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58f34 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x58f44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x58f48 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x58f5c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x58f64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58f74 | store | 4 | `sp -0x2c` | `ret(0x5a9f4)` |
| `ldr` at 0x58f78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x58f84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x58f90 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x58f94 | store | 4 | `sp -0x28` | `ret(0x5a9f4)` |
| `ldr` at 0x58f98 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x58fa0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x58fa8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x58fac | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x58fb4 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x58fc0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x58fc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x58fd4 | store | 4 | `sp -0x24` | `ret(0x5b130)` |
| `ldr` at 0x58fd8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x58fe4 | load | 4 | `abs 0x5908c` | `-` |
| `str` at 0x58fe8 | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x58ff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x58ff4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x59000 | store | 4 | `sp -0x2c` | `ret(0x5a9f4)` |
| `ldr` at 0x59004 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x59010 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x59014 | store | 4 | `sp -0x28` | `ret(0x5a9f4)` |
| `ldr` at 0x59018 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x5901c | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldrb` at 0x59024 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59028 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x5902c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5903c | store | 4 | `sp -0x20` | `ret(0x5b20c)` |
| `ldr` at 0x59040 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x5904c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x59050 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x59054 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x59064 | store | 4 | `sp -0x34` | `ret(0x5b3ec)` |
| `ldr` at 0x5906c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59070 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x5907c | store | 4 | `sp -0x34` | `ret(0x5aba8)` |
| `ldr` at 0x59080 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x59084 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x59084 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x59084 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x58f6c | `(unknown callee) 0x5a9f4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x58fcc | `(unknown callee) 0x5b130` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x58ff8 | `(unknown callee) 0x5a9f4` | `[sp-20]` | `ret(0x5b130)` | `?` | `ret(0x5b130)` |
| `bl` at 0x59034 | `(unknown callee) 0x5b20c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x5905c | `(unknown callee) 0x5b3ec` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x59074 | `(unknown callee) 0x5aba8` | `[sp-20]` | `ret(0x5b20c)` | `?` | `ret(0x5b20c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x58ec4 | `[0x148360] != 0x0` | `[0x148360] == 0x0` |
| `beq` at 0x58ed0 | `(r1 & 0xff) == 0xff` | `(r1 & 0xff) != 0xff` |
| `bne` at 0x58f20 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `blt` at 0x58f3c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x58f80 | `ret(0x5a9f4) == 0x0` | `ret(0x5a9f4) != 0x0` |
| `bne` at 0x58f8c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x58fbc | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x58fe0 | `ret(0x5b130) != 0x0` | `ret(0x5b130) == 0x0` |
| `beq` at 0x5900c | `ret(0x5a9f4) == 0x0` | `ret(0x5a9f4) != 0x0` |
| `bne` at 0x59048 | `ret(0x5b20c) != 0x0` | `ret(0x5b20c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x59084 | `r0` | `?` |

## `IP57_ResetHold`

- `IP57_ResetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x603c4 to 0x60668.
  Established. The indirect call has unknown target and candidates unknown, unknown, `[0x142b50]`,
  unknown (`mov` at 0x6045c). A later call to `0xd6a1c` uses `0x0`, `[sp-29]:1`, `0xc`, `0x8` (`bl`
  at 0x604f4); its result selects the taken continuation on zero and falls through on nonzero (`beq`
  at 0x60508). Reset pin identity and r0 at exit remain unresolved (`ldmdb` at 0x60660).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x603d8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x603dc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x603f4 | store | 4 | `sp -0x1c` | `ret(0x60f28)` |
| `ldr` at 0x603f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60404 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60408 | store | 4 | `sp -0x24` | `ret(0x60f28)` |
| `ldr` at 0x60410 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60414 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x6041c | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x60420 | load | 4 | `abs 0x60664` | `-` |
| `ldr` at 0x60424 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60428 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x6043c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x60440 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x6044c | load | 4 | `abs 0x142b50` | `-` |
| `ldr` at 0x604a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x604ac | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldrb` at 0x604b4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x604c4 | store | 4 | `sp -0x1c` | `ret(0x4d334)` |
| `ldr` at 0x604c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x604d4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x604d8 | store | 4 | `sp -0x24` | `ret(0x4d334)` |
| `ldrb` at 0x604e0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x604fc | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x60500 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6050c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60510 | store | 4 | `sp -0x24` | `ret(0xd6a1c)` |
| `ldrb` at 0x60518 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60530 | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x60534 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60540 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60544 | store | 4 | `sp -0x24` | `ret(0xd6988)` |
| `strb` at 0x60550 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x60554 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x60564 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60570 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x60590 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60594 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x60598 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x605bc | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x605c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x605cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x605d0 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldrb` at 0x605d8 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x605e0 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x605e8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60604 | store | 4 | `sp -0x1c` | `ret(0xd6a1c)` |
| `ldr` at 0x60608 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60614 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60618 | store | 4 | `sp -0x24` | `ret(0xd6a1c)` |
| `ldrb` at 0x60620 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60638 | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x6063c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60648 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6064c | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `str` at 0x60658 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6065c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x60660 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60660 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60660 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x603ec | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x6045c | `(unknown callee) (unknown target)` | `?` | `?` | `[0x142b50]` | `?` |
| `bl` at 0x60464 | `(unknown callee) 0x4dc1c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6046c | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x60470 | `(unknown callee) 0x3d8e0` | `ret(0x60fc8)` | `?` | `?` | `?` |
| `bl` at 0x60480 | `(unknown callee) 0x4f090` | `ret(0x3d8e0)` | `?` | `?` | `ret(0x3d8e0)` |
| `bl` at 0x60498 | `(unknown callee) 0xe5330` | `?` | `sp-28` | `sp-28` | `?` |
| `bl` at 0x604bc | `(unknown callee) 0x4d334` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x604f4 | `(unknown callee) 0xd6a1c` | `0x0` | `[sp-29]:1` | `0xc` | `0x8` |
| `bl` at 0x60528 | `(unknown callee) 0xd6988` | `0x0` | `[sp-29]:1` | `0x10` | `[sp-29]:1` |
| `bl` at 0x60580 | `(unknown callee) 0x5d8c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x605b4 | `(unknown callee) 0xd69d4` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x605fc | `(unknown callee) 0xd6a1c` | `0x0` | `[sp-29]:1` | `0xc0` | `0x0` |
| `bl` at 0x60630 | `(unknown callee) 0xd69d4` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60400 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `beq` at 0x6047c | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bne` at 0x604a4 | `ret(0xe5330) != 0x0` | `ret(0xe5330) == 0x0` |
| `beq` at 0x604d0 | `ret(0x4d334) == 0x0` | `ret(0x4d334) != 0x0` |
| `beq` at 0x60508 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `beq` at 0x6053c | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `bls` at 0x6055c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x6058c | `ret(0x5d8c0) == 0x0` | `ret(0x5d8c0) != 0x0` |
| `beq` at 0x605c8 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x60610 | `ret(0xd6a1c) == 0x0` | `ret(0xd6a1c) != 0x0` |
| `beq` at 0x60644 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60660 | `r0` | `?` |

## `IP57_ResetRelease`

- `IP57_ResetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60668 to 0x60a6c.
  Established. The unknown-target callback carries unknown, unknown, `[0x142b50]`, unknown (`mov` at
  0x60700). Later calls to `0xdc9a8` have `0x1`, `0x0`, `sp-112`, `sp-112` at both sites (`bl` at
  0x60820, `bl` at 0x60a30). The call to `0x4dc1c` has `0x1` and three unknown candidates (`bl` at
  0x60a38). Physical reset timing and the merged r0 result remain unknown (`ldmdb` at 0x60a48).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6067c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60680 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60698 | store | 4 | `sp -0x1c` | `ret(0x60f28)` |
| `ldr` at 0x6069c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x606a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x606ac | store | 4 | `sp -0x78` | `ret(0x60f28)` |
| `ldr` at 0x606b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x606b8 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x606c0 | store | 1 | `sp -0x1d` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldr` at 0x606c4 | load | 4 | `abs 0x60a4c` | `-` |
| `ldr` at 0x606c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x606cc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x606e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x606e4 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x606f0 | load | 4 | `abs 0x142b50` | `-` |
| `ldrb` at 0x60704 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60714 | store | 4 | `sp -0x1c` | `ret(0x4d358)` |
| `ldr` at 0x60718 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60724 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60728 | store | 4 | `sp -0x78` | `ret(0x4d358)` |
| `ldrb` at 0x60730 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60748 | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x6074c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60758 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6075c | store | 4 | `sp -0x78` | `ret(0xd6988)` |
| `ldrb` at 0x6076c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x60784 | store | 4 | `sp -0x1c` | `ret(0xd69d4)` |
| `ldr` at 0x60788 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60794 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60798 | store | 4 | `sp -0x78` | `ret(0xd69d4)` |
| `ldrb` at 0x607a8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x607c0 | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x607c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x607d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x607d4 | store | 4 | `sp -0x78` | `ret(0xd6988)` |
| `ldrb` at 0x607e4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x607f4 | store | 4 | `sp -0x1c` | `ret(0x4ceb4)` |
| `ldr` at 0x607f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60804 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60808 | store | 4 | `sp -0x78` | `ret(0x4ceb4)` |
| `ldrb` at 0x60824 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x6083c | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x60840 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6084c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60850 | store | 4 | `sp -0x78` | `ret(0xd6988)` |
| `strb` at 0x60864 | store | 1 | `sp -0x1e` | `0x0` |
| `ldrb` at 0x60868 | load | 1 | `sp -0x1e` | `-` |
| `ldrb` at 0x60878 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60884 | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x608a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x608a8 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x608ac | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x608d0 | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x608d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x608e0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x608e4 | store | 4 | `sp -0x78` | `ret(0xd6988)` |
| `ldrb` at 0x608ec | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x608f4 | store | 1 | `sp -0x1e` | `([sp-30]:1+1 & 0xff)` |
| `ldrb` at 0x60904 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x6091c | store | 4 | `sp -0x1c` | `ret(0xd6988)` |
| `ldr` at 0x60920 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6092c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60930 | store | 4 | `sp -0x78` | `ret(0xd6988)` |
| `ldr` at 0x60938 | load | 4 | `abs 0x60a50` | `-` |
| `ldr` at 0x6093c | load | 4 | `abs 0x142794` | `-` |
| `str` at 0x60948 | store | 4 | `sp -0x74` | `0x0` |
| `ldr` at 0x6094c | load | 4 | `sp -0x74` | `-` |
| `ldrb` at 0x6095c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6096c | store | 4 | `sp -0x1c` | `ret(0x4d8a0)` |
| `ldr` at 0x60970 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60988 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x60990 | store | 4 | `sp -0x74` | `[sp-116]+1` |
| `ldr` at 0x60998 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x609a4 | load | 4 | `abs 0x60a54` | `-` |
| `ldr` at 0x609a8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x609ac | load | 4 | `abs 0x14804c` | `-` |
| `ldr` at 0x609bc | load | 4 | `abs 0x60a58` | `-` |
| `str` at 0x609c0 | store | 4 | `sp -0x88` | `0x10c604` |
| `ldr` at 0x609c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x609d0 | load | 4 | `abs 0x60a5c` | `-` |
| `str` at 0x609d4 | store | 4 | `sp -0x7c` | `0x10c634` |
| `ldr` at 0x609dc | load | 4 | `abs 0x60a60` | `-` |
| `str` at 0x609e0 | store | 4 | `sp -0x7c` | `0x10c63c` |
| `ldr` at 0x609e4 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x609e8 | store | 4 | `sp -0x84` | `[sp-124]` |
| `ldr` at 0x609ec | load | 4 | `sp -0x74` | `-` |
| `str` at 0x609f0 | store | 4 | `sp -0x80` | `?` |
| `ldr` at 0x609f8 | load | 4 | `abs 0x60a64` | `-` |
| `ldr` at 0x609fc | load | 4 | `abs 0x60a68` | `-` |
| `ldr` at 0x60a08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x60a14 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x60a18 | store | 4 | `sp -0x78` | `[sp-28]` |
| `str` at 0x60a40 | store | 4 | `sp -0x78` | `0x0` |
| `ldr` at 0x60a44 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x60a48 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60a48 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60a48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60690 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `mov` at 0x60700 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x142b50]` | `?` |
| `bl` at 0x6070c | `(unknown callee) 0x4d358` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x60740 | `(unknown callee) 0xd6988` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x60768 | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `ret(0xd6988)` |
| `bl` at 0x6077c | `(unknown callee) 0xd69d4` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x607a4 | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `ret(0xd69d4)` |
| `bl` at 0x607b8 | `(unknown callee) 0xd6988` | `0x0` | `[sp-29]:1` | `0x2` | `[sp-29]:1` |
| `bl` at 0x607e0 | `(unknown callee) 0x60fc8` | `0x15` | `?` | `?` | `ret(0xd6988)` |
| `bl` at 0x607ec | `(unknown callee) 0x4ceb4` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x60820 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x60834 | `(unknown callee) 0xd6988` | `0x0` | `[sp-29]:1` | `0xc0` | `[sp-29]:1` |
| `bl` at 0x6085c | `(unknown callee) 0x60fc8` | `0x3` | `?` | `?` | `ret(0xd6988)` |
| `bl` at 0x60894 | `(unknown callee) 0x5d8c0` | `?` | `?` | `?` | `?` |
| `bl` at 0x608c8 | `(unknown callee) 0xd6988` | `0x0` | `?` | `0x4` | `?` |
| `bl` at 0x60900 | `(unknown callee) 0x60fc8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x60914 | `(unknown callee) 0xd6988` | `0x0` | `[sp-29]:1` | `0x4` | `[sp-29]:1` |
| `bl` at 0x60940 | `(unknown callee) 0x60fc8` | `[0x142794]` | `?` | `?` | `0x142794` |
| `bl` at 0x60964 | `(unknown callee) 0x4d8a0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x60984 | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `ret(0x4d8a0)` |
| `bl` at 0x60a04 | `(unknown callee) 0x21e28` | `0x4` | `0x10c53c` | `0x3fb` | `0x0` |
| `bl` at 0x60a30 | `(unknown callee) 0xdc9a8` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x60a38 | `(unknown callee) 0x4dc1c` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x606a4 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `beq` at 0x60720 | `ret(0x4d358) == 0x0` | `ret(0x4d358) != 0x0` |
| `beq` at 0x60754 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x60790 | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x607cc | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x60800 | `ret(0x4ceb4) == 0x0` | `ret(0x4ceb4) != 0x0` |
| `beq` at 0x60848 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `bls` at 0x60870 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x608a0 | `ret(0x5d8c0) == 0x0` | `ret(0x5d8c0) != 0x0` |
| `beq` at 0x608dc | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `beq` at 0x60928 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |
| `ble` at 0x60954 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x60978 | `ret(0x4d8a0) != 0x0` | `ret(0x4d8a0) == 0x0` |
| `bne` at 0x609a0 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bgt` at 0x609b4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x609cc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x60a10 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60a48 | `r0` | `?` |

## `IP57_SoftReset`

- `IP57_SoftReset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60a6c to 0x60b48.
  Established. The call to `0xd69d4` receives `0x0`, `([[sp-24]+2]:1+2 & 0xff)`, `0x8`,
  `([[sp-24]+2]:1+2 & 0xff)` (`bl` at 0x60ad8); its zero result takes the continuation and nonzero
  falls through (`beq` at 0x60aec). The later call to `0xd6988` has `0x0`, `[sp-25]:1`, `0x8`,
  `[sp-25]:1` (`bl` at 0x60b14). Soft-reset signaling and the common r0 remain unproved (`ldmdb` at
  0x60b44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60a80 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x60a84 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x60a9c | store | 4 | `sp -0x20` | `ret(0x60f28)` |
| `ldr` at 0x60aa0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60aac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60ab0 | store | 4 | `sp -0x24` | `ret(0x60f28)` |
| `ldr` at 0x60ab8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x60abc | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x60ac4 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x60ac8 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x60ae0 | store | 4 | `sp -0x20` | `ret(0xd69d4)` |
| `ldr` at 0x60ae4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60af0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60af4 | store | 4 | `sp -0x24` | `ret(0xd69d4)` |
| `ldrb` at 0x60b04 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x60b1c | store | 4 | `sp -0x20` | `ret(0xd6988)` |
| `ldr` at 0x60b20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x60b2c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x60b30 | store | 4 | `sp -0x24` | `ret(0xd6988)` |
| `str` at 0x60b3c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x60b40 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x60b44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60b44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60b44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x60a94 | `(unknown callee) 0x60f28` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x60ad8 | `(unknown callee) 0xd69d4` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x8` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x60b00 | `(unknown callee) 0x60fc8` | `0xa` | `?` | `?` | `ret(0xd69d4)` |
| `bl` at 0x60b14 | `(unknown callee) 0xd6988` | `0x0` | `[sp-25]:1` | `0x8` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x60aa8 | `ret(0x60f28) == 0x0` | `ret(0x60f28) != 0x0` |
| `beq` at 0x60aec | `ret(0xd69d4) == 0x0` | `ret(0xd69d4) != 0x0` |
| `beq` at 0x60b28 | `ret(0xd6988) == 0x0` | `ret(0xd6988) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60b44 | `r0` | `?` |

## `SHUB_Reset_FSBCfg`

- `SHUB_Reset_FSBCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x48278 to 0x48298.
  Established. The word write clears `0x14807c` to `0x0` (`str` at 0x4828c). The return retains
  entry r0 (`ldmdb` at 0x48290). Neither calls nor conditional branches appear in the fact sections.
  FSB configuration reset semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x48284 | load | 4 | `abs 0x48294` | `-` |
| `str` at 0x4828c | store | 4 | `abs 0x14807c` | `0x0` |
| `ldmdb` at 0x48290 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48290 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48290 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48290 | `r0` | `r0` |

## `SHUB_Allow_Resets`

- `SHUB_Allow_Resets`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x482d4 to 0x48548.
  Established. The two calls to `0x47a98` use unknown, `0x50000010`, `0x0`, `[sp-40]` and
  `[sp-17]:1`, `0x50002010`, `0x0`, `[sp-40]`, respectively (`bl` at 0x48340, `bl` at 0x48370).
  Later calls to `0x47b0c` include `[sp-17]:1`, `0x50000010`, `sp-44`, `sp-48` (`bl` at 0x483b0).
  The latter result takes its continuation on nonzero and falls through on zero (`bne` at 0x483bc).
  Hardware reset permissions and r0 at exit are unestablished (`ldmdb` at 0x48524).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x482e8 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x482ec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x482f4 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x482fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x48300 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x48310 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x48318 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4831c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x4832c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x48334 | load | 4 | `abs 0x48528` | `-` |
| `ldr` at 0x4833c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48350 | load | 4 | `abs 0x4852c` | `-` |
| `str` at 0x48354 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldrb` at 0x4835c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x48364 | load | 4 | `abs 0x48530` | `-` |
| `ldr` at 0x4836c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x48380 | load | 4 | `abs 0x4852c` | `-` |
| `str` at 0x48384 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x4838c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x48398 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x483a8 | load | 4 | `abs 0x48528` | `-` |
| `ldr` at 0x483c0 | load | 4 | `abs 0x4852c` | `-` |
| `str` at 0x483c4 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x483cc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x483dc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x483ec | load | 4 | `abs 0x48530` | `-` |
| `ldr` at 0x48404 | load | 4 | `abs 0x4852c` | `-` |
| `str` at 0x48408 | store | 4 | `sp -0x20` | `0xfffff893` |
| `ldr` at 0x48410 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x48424 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x4842c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x48434 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x4843c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48448 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48454 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48460 | load | 4 | `abs 0x48534` | `-` |
| `str` at 0x48464 | store | 4 | `sp -0x38` | `0x109494` |
| `ldr` at 0x4846c | load | 4 | `abs 0x48538` | `-` |
| `str` at 0x48470 | store | 4 | `sp -0x38` | `0x1094a0` |
| `ldr` at 0x48474 | load | 4 | `abs 0x4853c` | `-` |
| `ldr` at 0x48478 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x4847c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x48484 | load | 4 | `abs 0x48540` | `-` |
| `ldr` at 0x48488 | load | 4 | `abs 0x2ad25c` | `-` |
| `ldr` at 0x48494 | load | 4 | `abs 0x4853c` | `-` |
| `str` at 0x48498 | store | 4 | `sp -0x4c` | `0x1094a8` |
| `ldr` at 0x4849c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x484a8 | load | 4 | `abs 0x48534` | `-` |
| `str` at 0x484ac | store | 4 | `sp -0x3c` | `0x109494` |
| `ldr` at 0x484b4 | load | 4 | `abs 0x48538` | `-` |
| `str` at 0x484b8 | store | 4 | `sp -0x3c` | `0x1094a0` |
| `ldr` at 0x484bc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x484c0 | store | 4 | `sp -0x48` | `[sp-60]` |
| `ldr` at 0x484c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x484c8 | store | 4 | `sp -0x44` | `[sp-28]` |
| `ldr` at 0x484d0 | load | 4 | `abs 0x48544` | `-` |
| `ldr` at 0x484e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x484f0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x484f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x48504 | load | 4 | `abs 0x4852c` | `-` |
| `str` at 0x48508 | store | 4 | `sp -0x40` | `0xfffff893` |
| `str` at 0x48514 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x48518 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x4851c | store | 4 | `sp -0x34` | `[sp-64]` |
| `ldr` at 0x48520 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x48524 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x48524 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x48524 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x48340 | `(unknown callee) 0x47a98` | `?` | `0x50000010` | `0x0` | `[sp-40]` |
| `bl` at 0x48370 | `(unknown callee) 0x47a98` | `[sp-17]:1` | `0x50002010` | `0x0` | `[sp-40]` |
| `bl` at 0x483b0 | `(unknown callee) 0x47b0c` | `[sp-17]:1` | `0x50000010` | `sp-44` | `sp-48` |
| `bl` at 0x483f4 | `(unknown callee) 0x47b0c` | `[sp-17]:1` | `0x50002010` | `sp-44` | `sp-48` |
| `bl` at 0x48480 | `(unknown callee) 0x2070c` | `0x1094a8` | `[sp-56]` | `?` | `?` |
| `bl` at 0x484dc | `(unknown callee) 0x21e28` | `0x6` | `0x109480` | `0x1b0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x48324 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x4834c | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `bne` at 0x4837c | `ret(0x47a98) != 0x0` | `ret(0x47a98) == 0x0` |
| `bne` at 0x48394 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x483bc | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `bne` at 0x483d4 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `bne` at 0x48400 | `ret(0x47b0c) != 0x0` | `ret(0x47b0c) == 0x0` |
| `bne` at 0x48418 | `[sp-48] != 0x1` | `[sp-48] == 0x1` |
| `ble` at 0x48444 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x48450 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x4845c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x48490 | `[0x2ad25c] == 0x0` | `[0x2ad25c] != 0x0` |
| `beq` at 0x484a4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `beq` at 0x484e8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x48500 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x48524 | `r0` | `?` |

## `reset`

- `reset`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e10c to 0x4e1ac.
  Established. Entry r0 takes its bypass on equality with `0x5` and falls through on inequality
  (`beq` at 0x4e128). The call to `0x4e788` has `0xa`, unknown, unknown, `ret(0x4e6f0)` (`bl` at
  0x4e16c). A following call to `0x4e1fc` uses `0x5` and three unknown candidates (`bl` at 0x4e174).
  Board-wide reset effects and the common r0 result remain unknown (`ldmdb` at 0x4e1a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e11c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e120 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e12c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e138 | load | 4 | `abs 0x4e1a8` | `-` |
| `str` at 0x4e14c | store | 4 | `sp -0x1c` | `ret(0x4e6f0)` |
| `ldr` at 0x4e150 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e15c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e160 | store | 4 | `sp -0x20` | `ret(0x4e6f0)` |
| `str` at 0x4e17c | store | 4 | `sp -0x1c` | `ret(0x4e1fc)` |
| `ldr` at 0x4e180 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e18c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e190 | store | 4 | `sp -0x20` | `ret(0x4e1fc)` |
| `str` at 0x4e19c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4e1a0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x4e1a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e1a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e1a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e130 | `(unknown callee) 0x4e650` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e140 | `(unknown callee) 0x2070c` | `0x10a5a0` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `bl` at 0x4e144 | `(unknown callee) 0x4e6f0` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e16c | `(unknown callee) 0x4e788` | `0xa` | `?` | `?` | `ret(0x4e6f0)` |
| `bl` at 0x4e174 | `(unknown callee) 0x4e1fc` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e128 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x4e158 | `ret(0x4e6f0) == 0x0` | `ret(0x4e6f0) != 0x0` |
| `beq` at 0x4e188 | `ret(0x4e1fc) == 0x0` | `ret(0x4e1fc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e1a4 | `r0` | `?` |

## `resetHold`

- `resetHold`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e1ac to 0x4e1fc.
  Established. The entry test takes its bypass for `r0 == 0x5` and falls through otherwise (`beq` at
  0x4e1c8). The final call reaches `0x4e6f0` with `ret(0xe5418)` and three unknown candidates (`bl`
  at 0x4e1e8). The return propagates `ret(0x4e6f0)` (`ldmdb` at 0x4e1f4). Physical reset-hold scope
  remains unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e1bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e1c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e1cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e1d8 | load | 4 | `abs 0x4e1f8` | `-` |
| `ldmdb` at 0x4e1f4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e1f4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e1f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e1d0 | `(unknown callee) 0x4e650` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x4e1e0 | `(unknown callee) 0x2070c` | `0x10a5ac` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `bl` at 0x4e1e4 | `(unknown callee) 0xe5418` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e1e8 | `(unknown callee) 0x4e6f0` | `ret(0xe5418)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e1c8 | `r0 == 0x5` | `r0 != 0x5` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e1f4 | `r0` | `ret(0x4e6f0)` |

## `resetRelease`

- `resetRelease`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e1fc to 0x4e44c.
  Established. Two unknown-target callbacks carry `0xff`, unknown, unknown, `[0x2ae5c8]` and `0xff`,
  unknown, unknown, `[0x2ae5cc]`, respectively (`mov` at 0x4e39c, `mov` at 0x4e408). The intervening
  call to `0x41e80` has `0x0`, `0xff`, `0x5`, unknown (`bl` at 0x4e3bc); its zero result takes the
  continuation and nonzero falls through (`beq` at 0x4e3d0). Release policy and r0 at the common
  return remain unproved (`ldmdb` at 0x4e440).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e20c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4e21c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x4e224 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e228 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e234 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e240 | load | 4 | `abs 0x4e444` | `-` |
| `ldr` at 0x4e250 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e268 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e26c | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x4e27c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e288 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e2f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e300 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e30c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e324 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e33c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e348 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e354 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e37c | load | 4 | `abs 0x4e448` | `-` |
| `ldr` at 0x4e380 | load | 4 | `abs 0x2ae5c8` | `-` |
| `ldr` at 0x4e38c | load | 4 | `abs 0x4e448` | `-` |
| `ldr` at 0x4e390 | load | 4 | `abs 0x2ae5c8` | `-` |
| `str` at 0x4e3a4 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e3c4 | store | 4 | `sp -0x20` | `ret(0x41e80)` |
| `ldr` at 0x4e3c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e3d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e3e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e3e4 | store | 4 | `sp -0x1c` | `ret(0x41e80)` |
| `ldr` at 0x4e3e8 | load | 4 | `abs 0x4e448` | `-` |
| `ldr` at 0x4e3ec | load | 4 | `abs 0x2ae5cc` | `-` |
| `ldr` at 0x4e3f8 | load | 4 | `abs 0x4e448` | `-` |
| `ldr` at 0x4e3fc | load | 4 | `abs 0x2ae5cc` | `-` |
| `str` at 0x4e410 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x4e414 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4e420 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4e42c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4e430 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldr` at 0x4e434 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4e438 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x4e43c | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x4e440 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e440 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e440 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e210 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4e238 | `(unknown callee) 0x4e650` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4e248 | `(unknown callee) 0x2070c` | `0x10a5bc` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `bl` at 0x4e258 | `(unknown callee) 0xe5330` | `[sp-24]` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x4e278 | `(unknown callee) 0x4e788` | `0xa` | `?` | `?` | `ret(0xe5330)` |
| `bl` at 0x4e364 | `(unknown callee) 0x826fc` | `ret(0x4e788)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x4e368 | `(unknown callee) 0x85a0c` | `ret(0x826fc)` | `?` | `?` | `?` |
| `bl` at 0x4e36c | `(unknown callee) 0x68018` | `?` | `?` | `?` | `?` |
| `bl` at 0x4e378 | `(unknown callee) 0x5dcac` | `0xff` | `0x0` | `?` | `?` |
| `mov` at 0x4e39c | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x2ae5c8]` |
| `bl` at 0x4e3ac | `(unknown callee) 0x407dc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x4e3bc | `(unknown callee) 0x41e80` | `0x0` | `0xff` | `0x5` | `?` |
| `mov` at 0x4e408 | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x2ae5cc]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e230 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `bne` at 0x4e264 | `ret(0xe5330) != 0x0` | `ret(0xe5330) == 0x0` |
| `beq` at 0x4e284 | `[sp-24] == 0x43` | `[sp-24] != 0x43` |
| `beq` at 0x4e290 | `[sp-24] == 0x33` | `[sp-24] != 0x33` |
| `beq` at 0x4e29c | `[sp-24] == 0x35` | `[sp-24] != 0x35` |
| `beq` at 0x4e2a8 | `[sp-24] == 0x34` | `[sp-24] != 0x34` |
| `beq` at 0x4e2b4 | `[sp-24] == 0x41` | `[sp-24] != 0x41` |
| `beq` at 0x4e2c0 | `[sp-24] == 0x36` | `[sp-24] != 0x36` |
| `beq` at 0x4e2cc | `[sp-24] == 0x39` | `[sp-24] != 0x39` |
| `beq` at 0x4e2d8 | `[sp-24] == 0x37` | `[sp-24] != 0x37` |
| `beq` at 0x4e2e4 | `[sp-24] == 0x38` | `[sp-24] != 0x38` |
| `beq` at 0x4e2f0 | `[sp-24] == 0x30` | `[sp-24] != 0x30` |
| `beq` at 0x4e2fc | `[sp-24] == 0x51` | `[sp-24] != 0x51` |
| `beq` at 0x4e308 | `[sp-24] == 0x40` | `[sp-24] != 0x40` |
| `beq` at 0x4e314 | `[sp-24] == 0x4f` | `[sp-24] != 0x4f` |
| `beq` at 0x4e320 | `[sp-24] == 0x42` | `[sp-24] != 0x42` |
| `beq` at 0x4e32c | `[sp-24] == 0x46` | `[sp-24] != 0x46` |
| `beq` at 0x4e338 | `[sp-24] == 0x48` | `[sp-24] != 0x48` |
| `beq` at 0x4e344 | `[sp-24] == 0x4c` | `[sp-24] != 0x4c` |
| `beq` at 0x4e350 | `[sp-24] == 0x4d` | `[sp-24] != 0x4d` |
| `beq` at 0x4e35c | `[sp-24] == 0x7c` | `[sp-24] != 0x7c` |
| `beq` at 0x4e388 | `[0x2ae5c8] == 0x0` | `[0x2ae5c8] != 0x0` |
| `beq` at 0x4e3d0 | `ret(0x41e80) == 0x0` | `ret(0x41e80) != 0x0` |
| `bne` at 0x4e3dc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x4e3f4 | `[0x2ae5cc] == 0x0` | `[0x2ae5cc] != 0x0` |
| `beq` at 0x4e41c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x4e428 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e440 | `r0` | `?` |

## `resetSource`

- `resetSource`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e650 to 0x4e6f0.
  Established. The computed transfer takes its arm for `r0-1 <= 0x4 (unsigned, 32-bit)` and falls through
  for `r0-1 > 0x4 (unsigned, 32-bit)` (`ldrls` at 0x4e670); its taken destinations remain unresolved and excluded from the fact
  sections (`ldrls` at 0x4e670). The reported known-edge return supplies r0 `0x10a61c` (`ldmdb` at
  0x4e6d4). No calls are reported on known edges. Source-label mapping for the unresolved paths is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e660 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4e664 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x4e670 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x4e6c8 | load | 4 | `abs 0x4e6ec` | `-` |
| `str` at 0x4e6cc | store | 4 | `sp -0x18` | `0x10a61c` |
| `ldr` at 0x4e6d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x4e6d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e6d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e6d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x4e670 | `r0-1 <= 0x4 (unsigned, 32-bit)` | `r0-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e6d4 | `r0` | `0x10a61c` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x4e670).
