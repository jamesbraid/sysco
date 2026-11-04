# ARM Linux 2.6 power-bay and ICMB records in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes. They can include trailing literal words after a return.

`sp` denotes the entry stack pointer. Register expressions such as `r0` denote entry
values, and brackets denote loaded values. Decimal numbers inside expressions retain the
exact tool notation. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-pbay-icmb-records-facts.txt) retain all tool sections.

## `PBaySdrGetCardSerial`

- `PBaySdrGetCardSerial`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec2cc to 0xec798.
  Established. The result from unnamed 0x201fc is stored as a word at sp-0x2cc and at the address held in [sp-24] (`bl` at 0xec324, `str` at 0xec32c, `str` at 0xec338).
  A halfword load, add of three and halfword store write ([sp-706]:2+3 & 0xffff) at sp-0x2c2 (`ldrh` at 0xec57c, `add` at 0xec580, `strh` at 0xec588).
  Byte stores with unknown effective addresses write 0x28, 0xa and zero (`strb` at 0xec6b0, `strb` at 0xec6f8, `strb` at 0xec720).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec2dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xec2e0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xec2e4 | load | 4 | `abs 0xec770` | `-` |
| `str` at 0xec2e8 | store | 4 | `sp -0x120` | `0x1a0448` |
| `ldr` at 0xec2ec | load | 4 | `sp -0x120` | `-` |
| `ldrb` at 0xec2f0 | load | 1 | `abs 0x1a0448` | `-` |
| `ldr` at 0xec2fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xec304 | store | 4 | `r1 +0x0` | `0x0` |
| `str` at 0xec30c | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xec314 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xec32c | store | 4 | `sp -0x2cc` | `ret(0x201fc)` |
| `ldr` at 0xec330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec334 | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xec338 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xec33c | load | 4 | `sp -0x2cc` | `-` |
| `str` at 0xec34c | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xec354 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xec358 | load | 4 | `abs 0xec774` | `-` |
| `ldr` at 0xec360 | load | 4 | `abs 0xec778` | `-` |
| `ldr` at 0xec364 | load | 4 | `abs 0x1a0494` | `-` |
| `str` at 0xec368 | store | 4 | `sp -0x2dc` | `[0x1a0494]` |
| `ldr` at 0xec370 | load | 4 | `abs 0xec774` | `-` |
| `str` at 0xec374 | store | 4 | `sp -0x2cc` | `0x1a0468` |
| `ldr` at 0xec378 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xec380 | store | 1 | `abs 0x1a0468` | `0x0` |
| `ldr` at 0xec384 | load | 4 | `abs 0xec77c` | `-` |
| `strh` at 0xec390 | store | 2 | `sp -0x2c2` | `0x0` |
| `ldr` at 0xec394 | load | 4 | `abs 0xec780` | `-` |
| `strb` at 0xec3a0 | store | 1 | `sp -0x2c3` | `0x8` |
| `ldr` at 0xec3a8 | load | 4 | `abs 0xec77c` | `-` |
| `ldrh` at 0xec3b0 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xec3b4 | load | 4 | `abs 0xec780` | `-` |
| `ldrb` at 0xec3bc | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xec3c0 | store | 4 | `sp -0x2e4` | `0x8` |
| `ldr` at 0xec3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec3cc | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xec3dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec3ec | store | 4 | `sp -0x2bc` | `ret(0xea010)` |
| `ldr` at 0xec3f0 | load | 4 | `sp -0x2bc` | `-` |
| `ldr` at 0xec3fc | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xec400 | load | 4 | `abs 0xec784` | `-` |
| `ldr` at 0xec408 | load | 4 | `abs 0xec778` | `-` |
| `str` at 0xec410 | store | 4 | `abs 0x1a0494` | `?` |
| `str` at 0xec418 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xec420 | load | 4 | `sp -0x130` | `-` |
| `str` at 0xec428 | store | 4 | `sp -0x2d0` | `ret(0xedbdc)` |
| `ldr` at 0xec430 | load | 4 | `sp -0x2d0` | `-` |
| `ldr` at 0xec448 | load | 4 | `abs 0xec788` | `-` |
| `ldrb` at 0xec454 | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec460 | load | 4 | `abs 0xec788` | `-` |
| `ldrb` at 0xec46c | load | 1 | `sp -0x2d4` | `-` |
| `ldr` at 0xec478 | load | 4 | `abs 0xec77c` | `-` |
| `ldr` at 0xec47c | load | 4 | `abs 0xec788` | `-` |
| `ldrb` at 0xec488 | load | 1 | `sp -0x2d4` | `-` |
| `strh` at 0xec494 | store | 2 | `sp -0x2c2` | `?` |
| `ldr` at 0xec498 | load | 4 | `abs 0xec780` | `-` |
| `strb` at 0xec4a4 | store | 1 | `sp -0x2c3` | `0x3` |
| `ldr` at 0xec4ac | load | 4 | `abs 0xec77c` | `-` |
| `ldrh` at 0xec4b4 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xec4b8 | load | 4 | `abs 0xec780` | `-` |
| `ldrb` at 0xec4c0 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xec4c4 | store | 4 | `sp -0x2e4` | `0x3` |
| `str` at 0xec4cc | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xec4d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec4d8 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xec4e8 | load | 4 | `abs 0xec78c` | `-` |
| `strb` at 0xec4f0 | store | 1 | `sp -0x2c4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xec4f4 | load | 4 | `abs 0xec78c` | `-` |
| `ldrb` at 0xec4fc | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xec508 | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xec50c | load | 4 | `abs 0xec784` | `-` |
| `ldr` at 0xec514 | load | 4 | `abs 0xec778` | `-` |
| `str` at 0xec51c | store | 4 | `abs 0x1a0494` | `?` |
| `str` at 0xec524 | store | 4 | `sp -0x2dc` | `?` |
| `ldr` at 0xec52c | load | 4 | `abs 0xec780` | `-` |
| `ldr` at 0xec530 | load | 4 | `abs 0xec790` | `-` |
| `ldrb` at 0xec538 | load | 1 | `sp -0x117` | `-` |
| `strb` at 0xec544 | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xec548 | load | 4 | `abs 0xec780` | `-` |
| `ldr` at 0xec54c | load | 4 | `abs 0xec780` | `-` |
| `ldr` at 0xec550 | load | 4 | `abs 0xec78c` | `-` |
| `ldrb` at 0xec558 | load | 1 | `sp -0x2c3` | `-` |
| `ldrb` at 0xec560 | load | 1 | `sp -0x2c4` | `-` |
| `strb` at 0xec56c | store | 1 | `sp -0x2c3` | `?` |
| `ldr` at 0xec570 | load | 4 | `abs 0xec77c` | `-` |
| `ldr` at 0xec574 | load | 4 | `abs 0xec77c` | `-` |
| `ldrh` at 0xec57c | load | 2 | `sp -0x2c2` | `-` |
| `strh` at 0xec588 | store | 2 | `sp -0x2c2` | `([sp-706]:2+3 & 0xffff)` |
| `ldr` at 0xec590 | load | 4 | `abs 0xec77c` | `-` |
| `ldrh` at 0xec598 | load | 2 | `sp -0x2c2` | `-` |
| `ldr` at 0xec59c | load | 4 | `abs 0xec780` | `-` |
| `ldrb` at 0xec5a4 | load | 1 | `sp -0x2c3` | `-` |
| `str` at 0xec5a8 | store | 4 | `sp -0x2e4` | `[sp-707]:1` |
| `str` at 0xec5b0 | store | 4 | `sp -0x2e0` | `sp-280` |
| `ldr` at 0xec5b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec5bc | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xec5cc | load | 4 | `abs 0xec78c` | `-` |
| `strb` at 0xec5d4 | store | 1 | `sp -0x2c4` | `(ret(0xecf80) & 0xff)` |
| `ldr` at 0xec5d8 | load | 4 | `abs 0xec78c` | `-` |
| `ldrb` at 0xec5e0 | load | 1 | `sp -0x2c4` | `-` |
| `ldr` at 0xec5ec | load | 4 | `sp -0x2cc` | `-` |
| `ldr` at 0xec5f0 | load | 4 | `abs 0xec784` | `-` |
| `ldr` at 0xec5f8 | load | 4 | `abs 0xec778` | `-` |
| `str` at 0xec600 | store | 4 | `abs 0x1a0494` | `?` |
| `str` at 0xec608 | store | 4 | `sp -0x2dc` | `?` |
| `str` at 0xec614 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xec61c | store | 4 | `sp -0x2c0` | `0x0` |
| `ldr` at 0xec620 | load | 4 | `sp -0x2c0` | `-` |
| `ldr` at 0xec62c | load | 4 | `abs 0xec794` | `-` |
| `ldr` at 0xec630 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xec640 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec654 | load | 4 | `abs 0xec794` | `-` |
| `ldr` at 0xec658 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xec668 | load | 1 | `(sp-16 + [sp-284]) -0x108` | `-` |
| `strb` at 0xec674 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xec678 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xec680 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldr` at 0xec684 | load | 4 | `sp -0x2c0` | `-` |
| `ldrb` at 0xec698 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xec6b0 | store | 1 | `(unknown)` | `0x28` |
| `ldr` at 0xec6b8 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xec6c8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec6cc | load | 4 | `sp -0x2cc` | `-` |
| `ldrb` at 0xec6e4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec6ec | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xec6f8 | store | 1 | `(unknown)` | `0xa` |
| `ldrb` at 0xec704 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec710 | load | 4 | `sp -0x2cc` | `-` |
| `strb` at 0xec720 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0xec730 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xec734 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xec73c | store | 4 | `sp -0x11c` | `?` |
| `ldr` at 0xec740 | load | 4 | `sp -0x2c0` | `-` |
| `str` at 0xec748 | store | 4 | `sp -0x2c0` | `[sp-704]+1` |
| `ldr` at 0xec750 | load | 4 | `abs 0xec778` | `-` |
| `str` at 0xec758 | store | 4 | `abs 0x1a0494` | `0x0` |
| `str` at 0xec760 | store | 4 | `sp -0x2dc` | `0x0` |
| `ldr` at 0xec764 | load | 4 | `sp -0x2dc` | `-` |
| `ldm` at 0xec76c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xec76c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xec76c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec324 | `(unknown callee) 0x201fc` | `0x2a` | `r1` | `r2` | `r1` |
| `bl` at 0xec35c | `(unknown callee) 0x20634` | `[sp-716]` | `0x1a0468` | `[sp-24]` | `[sp-716]` |
| `bl` at 0xec3d4 | `(unknown callee) 0xece5c` | `[sp-20]` | `sp-696` | `[sp-288]` | `0x0` |
| `bl` at 0xec3e4 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-696` | `?` | `sp-696` |
| `bl` at 0xec404 | `(unknown callee) 0x203c4` | `[sp-716]` | `0x184b38` | `?` | `ret(0xea010)` |
| `bl` at 0xec424 | `(unknown callee) 0xedbdc` | `[sp-304]` | `?` | `?` | `ret(0xea010)` |
| `bl` at 0xec444 | `(unknown callee) 0x203f4` | `sp-728` | `ret(0xedbdc)+1` | `0x8` | `ret(0xedbdc)+1` |
| `bl` at 0xec4e0 | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-696` | `[sp-288]` | `[sp-706]:2` |
| `bl` at 0xec510 | `(unknown callee) 0x203c4` | `[sp-716]` | `0x184b38` | `ret(0xecf80)` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xec5c4 | `(unknown callee) 0xecf80` | `[sp-20]` | `sp-696` | `[sp-288]` | `([sp-706]:2+3 & 0xffff)` |
| `bl` at 0xec5f4 | `(unknown callee) 0x203c4` | `[sp-716]` | `0x184b38` | `sp-16` | `(ret(0xecf80) & 0xff)` |
| `bl` at 0xec6d8 | `(unknown callee) 0x203f4` | `[sp-716]` | `(sp-280 + [sp-284])` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xec2f8 | `[0x1a0448]:1 != 0xff` | `[0x1a0448]:1 == 0xff` |
| `beq` at 0xec31c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xec344 | `[sp-716] != 0x0` | `[sp-716] == 0x0` |
| `bne` at 0xec3f8 | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `beq` at 0xec45c | `[sp-724]:1 == 0x0` | `[sp-724]:1 != 0x0` |
| `beq` at 0xec474 | `[sp-724]:1 == 0xff` | `[sp-724]:1 != 0xff` |
| `bne` at 0xec504 | `(ret(0xecf80) & 0xff) != 0x0` | `(ret(0xecf80) & 0xff) == 0x0` |
| `bne` at 0xec5e8 | `(ret(0xecf80) & 0xff) != 0x0` | `(ret(0xecf80) & 0xff) == 0x0` |
| `bgt` at 0xec628 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xec648 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xec68c | `[sp-704] != 0x4` | `[sp-704] == 0x4` |
| `bls` at 0xec6a0 | `? <= 0x28 (unsigned, 32-bit)` | `? > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xec76c | `r0` | `?` |

## `SdrDecode`

- `SdrDecode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xec798 to 0xec850.
  Established. Two word stores save entry r0 at sp-0x14 and sp-0x18 (`str` at 0xec7a8, `str` at 0xec7b0).
  A byte loaded from entry r0 plus 3 is stored as a word at sp-0x1c (`ldrb` at 0xec7b8, `str` at 0xec7bc).
  The predicated load into pc has an unresolved target for [r0+3]:1-1 <= 0x11 (unsigned, 32-bit) (`ldrb` at 0xec7b8, `sub` at 0xec7c4, `ldrls` at 0xec7cc).
  It falls through for [r0+3]:1-1 > 0x11 (unsigned, 32-bit) (`ldrb` at 0xec7b8, `sub` at 0xec7c4, `ldrls` at 0xec7cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xec7a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xec7ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xec7b0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xec7b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xec7b8 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xec7bc | store | 4 | `sp -0x1c` | `[r0+3]:1` |
| `ldr` at 0xec7c0 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xec7cc | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xec84c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xec84c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xec84c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xec7cc | `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)` | `[r0+3]:1-1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xec84c | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xec7cc).

## `PBaySdrGetConnector`

- `PBaySdrGetConnector`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed390 to 0xed4c0.
  Established. A byte store writes zero at an unknown effective address (`strb` at 0xed3b0).
  Two calls to unnamed 0xea010 have their results stored at sp-0x1b0 (`bl` at 0xed40c, `str` at 0xed414, `bl` at 0xed45c, `str` at 0xed464).
  A later byte store with an unknown effective address writes ([[sp-440]]:1 & 0xff) (`ldrb` at 0xed494, `strb` at 0xed49c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed3a0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xed3b0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xed3b4 | load | 4 | `abs 0xed4b8` | `-` |
| `ldr` at 0xed3b8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xed3bc | load | 2 | `[sp-20] +0x6` | `-` |
| `strh` at 0xed3c4 | store | 2 | `sp -0x1ac` | `([[sp-20]+6]:2 & 0xffff)` |
| `str` at 0xed3cc | store | 4 | `sp -0x1a8` | `0xa` |
| `str` at 0xed3d4 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xed3dc | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xed3e4 | store | 4 | `sp -0x114` | `0x1` |
| `ldr` at 0xed3e8 | load | 4 | `abs 0xed4bc` | `-` |
| `strb` at 0xed3f4 | store | 1 | `sp -0x1a4` | `0x0` |
| `str` at 0xed3fc | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xed404 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed414 | store | 4 | `sp -0x1b0` | `ret(0xea010)` |
| `ldr` at 0xed418 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xed424 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed428 | store | 4 | `sp -0x1b4` | `[sp-36]` |
| `ldr` at 0xed42c | load | 4 | `sp -0x1b4` | `-` |
| `ldrb` at 0xed430 | load | 1 | `[sp-36] +0x0` | `-` |
| `str` at 0xed444 | store | 4 | `sp -0x1a8` | `0xb` |
| `str` at 0xed44c | store | 4 | `sp -0x114` | `0x0` |
| `ldr` at 0xed454 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed464 | store | 4 | `sp -0x1b0` | `ret(0xea010)` |
| `ldr` at 0xed468 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xed474 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed478 | store | 4 | `sp -0x1b8` | `[sp-36]` |
| `ldr` at 0xed47c | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xed480 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xed490 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xed494 | load | 1 | `[sp-440] +0x0` | `-` |
| `strb` at 0xed49c | store | 1 | `(unknown)` | `([[sp-440]]:1 & 0xff)` |
| `ldrb` at 0xed4a8 | load | 1 | `(unknown)` | `-` |
| `ldm` at 0xed4b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xed4b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xed4b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed40c | `(unknown callee) 0xea010` | `[sp-20]` | `sp-428` | `0xfffffe6c` | `sp-428` |
| `bl` at 0xed45c | `(unknown callee) 0xea010` | `[sp-20]` | `sp-428` | `?` | `sp-428` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xed420 | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |
| `beq` at 0xed43c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xed470 | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |
| `beq` at 0xed488 | `[[sp-36]]:1 == 0x0` | `[[sp-36]]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xed4b4 | `r0` | `?` |

## `PBaySdrWarmReset`

- `PBaySdrWarmReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed5e4 to 0xed6cc.
  Established. A halfword loaded from entry r0 plus 6 is stored as a halfword at sp-0x1ac (`ldrh` at 0xed600, `strh` at 0xed608).
  The result from unnamed 0xea010 is stored at sp-0x1b0 (`bl` at 0xed664, `str` at 0xed66c).
  Printed candidates for the call to unnamed 0x20634 include r0=ret(0x201fc) and r1=[sp-436] (`bl` at 0xed694, `bl` at 0xed6a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed5f4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xed5f8 | load | 4 | `abs 0xed6bc` | `-` |
| `ldr` at 0xed5fc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xed600 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xed608 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xed610 | store | 4 | `sp -0x1a8` | `0x20` |
| `str` at 0xed618 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xed620 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xed628 | store | 4 | `sp -0x110` | `0x3` |
| `ldr` at 0xed62c | load | 4 | `abs 0xed6c0` | `-` |
| `ldrb` at 0xed630 | load | 1 | `abs 0x1b0f0f` | `-` |
| `strb` at 0xed638 | store | 1 | `sp -0x10c` | `([0x1b0f0f]:1 & 0xff)` |
| `str` at 0xed640 | store | 4 | `sp -0x108` | `0x6` |
| `strb` at 0xed64c | store | 1 | `sp -0x104` | `0x0` |
| `str` at 0xed654 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0xed65c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed66c | store | 4 | `sp -0x1b0` | `ret(0xea010)` |
| `ldr` at 0xed670 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xed67c | load | 4 | `abs 0xed6c4` | `-` |
| `str` at 0xed680 | store | 4 | `sp -0x1b4` | `0x184cd8` |
| `ldr` at 0xed688 | load | 4 | `abs 0xed6c8` | `-` |
| `str` at 0xed68c | store | 4 | `sp -0x1b4` | `0x184cdc` |
| `str` at 0xed69c | store | 4 | `sp -0x1b8` | `ret(0x201fc)` |
| `ldr` at 0xed6a0 | load | 4 | `sp -0x1b8` | `-` |
| `ldr` at 0xed6a4 | load | 4 | `sp -0x1b4` | `-` |
| `ldr` at 0xed6ac | load | 4 | `sp -0x1b8` | `-` |
| `ldm` at 0xed6b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xed6b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xed6b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed664 | `(unknown callee) 0xea010` | `r0` | `sp-428` | `sp-260` | `sp-428` |
| `bl` at 0xed694 | `(unknown callee) 0x201fc` | `0xc` | `?` | `?` | `?` |
| `bl` at 0xed6a8 | `(unknown callee) 0x20634` | `ret(0x201fc)` | `[sp-436]` | `?` | `ret(0x201fc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xed678 | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xed6b8 | `r0` | `[sp-440]` |

## `PBaySdrFlashInfo`

- `PBaySdrFlashInfo`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed6cc to 0xed870.
  Established. The result of the call to unnamed 0x204f0 is stored at sp-0x210 (`bl` at 0xed78c, `str` at 0xed794).
  A word load from that result plus 0x10, an add of one and a word store produce [ret(0x204f0)+16]+1 at sp-0x22c (`bl` at 0xed78c, `ldr` at 0xed7b0, `add` at 0xed7b4, `str` at 0xed7b8).
  The call to unnamed 0x201fc has its result stored at sp-0x20c (`bl` at 0xed83c, `str` at 0xed844).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed6dc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xed6e0 | load | 4 | `abs 0xed864` | `-` |
| `ldr` at 0xed6e4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xed6e8 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xed6f0 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xed6f8 | store | 4 | `sp -0x1a8` | `0xc0` |
| `str` at 0xed700 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xed708 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xed710 | store | 4 | `sp -0x114` | `0x0` |
| `str` at 0xed718 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xed720 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed730 | store | 4 | `sp -0x1b0` | `ret(0xea010)` |
| `ldr` at 0xed734 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xed740 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xed744 | store | 4 | `sp -0x214` | `[sp-36]` |
| `ldr` at 0xed748 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed74c | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0xed754 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed758 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xed764 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed768 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0xed774 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed778 | load | 1 | `[sp-36] +0x2` | `-` |
| `str` at 0xed780 | store | 4 | `sp -0x1b4` | `?` |
| `str` at 0xed794 | store | 4 | `sp -0x210` | `ret(0x204f0)` |
| `ldr` at 0xed79c | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed7a0 | load | 1 | `[sp-532] +0x1` | `-` |
| `ldr` at 0xed7a4 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xed7a8 | load | 1 | `[sp-532] +0x0` | `-` |
| `ldr` at 0xed7ac | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7b0 | load | 4 | `ret(0x204f0) +0x10` | `-` |
| `str` at 0xed7b8 | store | 4 | `sp -0x22c` | `[ret(0x204f0)+16]+1` |
| `ldr` at 0xed7bc | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7c0 | load | 4 | `ret(0x204f0) +0xc` | `-` |
| `str` at 0xed7c4 | store | 4 | `sp -0x228` | `[ret(0x204f0)+12]` |
| `ldr` at 0xed7c8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7cc | load | 4 | `ret(0x204f0) +0x14` | `-` |
| `str` at 0xed7d8 | store | 4 | `sp -0x224` | `[ret(0x204f0)+20]+1900` |
| `ldr` at 0xed7dc | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7e0 | load | 4 | `ret(0x204f0) +0x8` | `-` |
| `str` at 0xed7e4 | store | 4 | `sp -0x220` | `[ret(0x204f0)+8]` |
| `ldr` at 0xed7e8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7ec | load | 4 | `ret(0x204f0) +0x4` | `-` |
| `str` at 0xed7f0 | store | 4 | `sp -0x21c` | `[ret(0x204f0)+4]` |
| `ldr` at 0xed7f4 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xed7f8 | load | 4 | `ret(0x204f0) +0x0` | `-` |
| `str` at 0xed7fc | store | 4 | `sp -0x218` | `[ret(0x204f0)]` |
| `ldr` at 0xed804 | load | 4 | `abs 0xed868` | `-` |
| `str` at 0xed818 | store | 4 | `sp -0x208` | `sp-516` |
| `ldr` at 0xed820 | load | 4 | `abs 0xed86c` | `-` |
| `str` at 0xed824 | store | 4 | `sp -0x208` | `0x184d28` |
| `ldr` at 0xed828 | load | 4 | `sp -0x208` | `-` |
| `str` at 0xed844 | store | 4 | `sp -0x20c` | `ret(0x201fc)` |
| `ldr` at 0xed848 | load | 4 | `sp -0x20c` | `-` |
| `ldr` at 0xed84c | load | 4 | `sp -0x208` | `-` |
| `ldr` at 0xed854 | load | 4 | `sp -0x20c` | `-` |
| `ldm` at 0xed860 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xed860 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xed860 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed728 | `(unknown callee) 0xea010` | `r0` | `sp-428` | `0xfffffe64` | `sp-428` |
| `bl` at 0xed78c | `(unknown callee) 0x204f0` | `sp-436` | `?` | `?` | `sp-436` |
| `bl` at 0xed810 | `(unknown callee) 0x205bc` | `sp-516` | `0x184ce8` | `[[sp-532]+1]:1` | `[[sp-532]]:1` |
| `bl` at 0xed82c | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0xed83c | `(unknown callee) 0x201fc` | `ret(0x2031c)+1` | `?` | `?` | `ret(0x2031c)+1` |
| `bl` at 0xed850 | `(unknown callee) 0x20634` | `ret(0x201fc)` | `[sp-520]` | `?` | `ret(0x201fc)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xed73c | `ret(0xea010) == 0x0` | `ret(0xea010) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xed860 | `r0` | `[sp-524]` |

## `PBaySdrGetNbrDps`

- `PBaySdrGetNbrDps`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed870 to 0xed99c.
  Established. A word store initializes sp-0x1c to zero and another initializes sp-0x20 to one (`str` at 0xed88c, `str` at 0xed894).
  A byte loaded from the result of unnamed 0xedbdc plus two is stored as a byte at sp-0x29 (`bl` at 0xed930, `ldrb` at 0xed93c, `strb` at 0xed940).
  Load, add and store sequences write [sp-28]+1 at sp-0x1c and [sp-32]+1 at sp-0x20 (`ldr` at 0xed968, `add` at 0xed96c, `str` at 0xed970, `ldr` at 0xed974, `add` at 0xed978, `str` at 0xed97c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed880 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xed884 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xed88c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xed894 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xed898 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed8a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xed8ac | load | 4 | `abs 0xed998` | `-` |
| `str` at 0xed8b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xed8b8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xed8bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xed8c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed8cc | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xed8d8 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xed8e4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xed8e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xed8ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xed900 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xed910 | store | 4 | `sp -0x1c8` | `ret(0xea010)` |
| `ldr` at 0xed914 | load | 4 | `sp -0x1c8` | `-` |
| `str` at 0xed924 | store | 4 | `sp -0x1cc` | `0x0` |
| `ldr` at 0xed92c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xed934 | store | 4 | `sp -0x28` | `ret(0xedbdc)` |
| `ldr` at 0xed938 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xed93c | load | 1 | `ret(0xedbdc) +0x2` | `-` |
| `strb` at 0xed940 | store | 1 | `sp -0x29` | `([ret(0xedbdc)+2]:1 & 0xff)` |
| `ldr` at 0xed944 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xed948 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xed950 | load | 1 | `sp -0x29` | `-` |
| `strb` at 0xed954 | store | 1 | `([sp-24] + [sp-32]) +0x0` | `(([ret(0xedbdc)+2]:1 & 0xff) & 0xff)` |
| `ldrb` at 0xed958 | load | 1 | `sp -0x29` | `-` |
| `ldr` at 0xed968 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xed970 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xed974 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xed97c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xed984 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xed988 | store | 4 | `sp -0x1cc` | `?` |
| `ldr` at 0xed98c | load | 4 | `sp -0x1cc` | `-` |
| `ldm` at 0xed994 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xed994 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xed994 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xed8f8 | `(unknown callee) 0xecd34` | `?` | `sp-452` | `?` | `?` |
| `bl` at 0xed908 | `(unknown callee) 0xea010` | `[sp-20]` | `sp-452` | `?` | `sp-452` |
| `bl` at 0xed930 | `(unknown callee) 0xedbdc` | `[sp-60]` | `?` | `?` | `ret(0xea010)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xed8a0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0xed8c4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xed91c | `ret(0xea010) != 0x0` | `ret(0xea010) == 0x0` |
| `bne` at 0xed964 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xed994 | `r0` | `?` |

## `IcmbIpmbRqBuild`

- `IcmbIpmbRqBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xed99c to 0xeda34.
  Established. Byte stores write ((r0 & 0xff) & 0xff) at the address held in [sp+8] and ([sp-32] & 0xff) at that printed expression plus 2 (`strb` at 0xed9c8, `strb` at 0xeda04).
  Printed candidates for the call to unnamed 0x203f4 include r0=[sp+8]+3, r1=[sp] and r2=[sp+4] (`bl` at 0xeda1c).
  The return record is [sp+4]+3, formed by a load and add (`ldr` at 0xeda20, `add` at 0xeda24, `ldm` at 0xeda30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xed9ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xed9b0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xed9b4 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xed9bc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xed9c0 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xed9c4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xed9c8 | store | 1 | `[sp+8] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xed9d0 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xed9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xed9e0 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xed9f0 | store | 1 | `[sp+8] +0x1` | `?` |
| `ldr` at 0xed9f8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeda00 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xeda04 | store | 1 | `[sp+8] +0x2` | `([sp-32] & 0xff)` |
| `ldr` at 0xeda08 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xeda14 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeda18 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xeda20 | load | 4 | `sp +0x4` | `-` |
| `ldm` at 0xeda30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeda30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeda30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeda1c | `(unknown callee) 0x203f4` | `[sp+8]+3` | `[sp]` | `[sp+4]` | `[sp+8]+3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeda30 | `r0` | `[sp+4]+3` |

## `IcmbIpmbRsBuild`

- `IcmbIpmbRsBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeda34 to 0xedae4.
  Established. The low byte of entry r0 is saved at sp-0x11 (`strb` at 0xeda54).
  Byte stores write ([sp-32] & 0xff) at the address formed by [sp+12] plus 2 and ([sp] & 0xff) at the expression [sp+12] plus 3 (`strb` at 0xedaa0, `strb` at 0xedab4).
  A load and add form the recorded return [sp+8]+4 (`ldr` at 0xedad0, `add` at 0xedad4, `ldm` at 0xedae0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeda44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeda48 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeda4c | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xeda54 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xeda58 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xeda5c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xeda60 | store | 1 | `[sp+12] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xeda68 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xeda70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeda78 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xeda8c | store | 1 | `[sp+12] +0x1` | `?` |
| `ldr` at 0xeda94 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xeda9c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xedaa0 | store | 1 | `[sp+12] +0x2` | `([sp-32] & 0xff)` |
| `ldr` at 0xedaa8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xedab0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xedab4 | store | 1 | `[sp+12] +0x3` | `([sp] & 0xff)` |
| `ldr` at 0xedab8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xedac4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xedac8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xedad0 | load | 4 | `sp +0x8` | `-` |
| `ldm` at 0xedae0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedae0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedae0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xedacc | `(unknown callee) 0x203f4` | `[sp+12]+4` | `[sp+4]` | `[sp+8]` | `[sp+12]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedae0 | `r0` | `[sp+8]+4` |

## `IcmbIpmbGetNetFn`

- `IcmbIpmbGetNetFn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedae4 to 0xedb28.
  Established. A byte load reads entry r0+1 (`ldrb` at 0xedb04);
  its subsequent operations include a right shift by two and AND masks 0xff and 0x3e (`lsr` at 0xedb08, `and` at 0xedb0c, `and` at 0xedb10).
  The word store at sp-0x18 has an unknown value, and the recorded return is [sp-24] (`str` at 0xedb14, `ldm` at 0xedb24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedaf4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedafc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedb04 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xedb14 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xedb18 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xedb24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedb24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedb24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedb24 | `r0` | `[sp-24]` |

## `IcmbIpmbGetRqRsId`

- `IcmbIpmbGetRqRsId`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedb28 to 0xedb6c.
  Established. Entry r0 is saved as a word at sp-0x14 (`str` at 0xedb38).
  A byte load reads entry r0+1, and subsequent instructions shift right by two and apply AND masks 0xff and one (`ldrb` at 0xedb48, `lsr` at 0xedb4c, `and` at 0xedb50, `and` at 0xedb54).
  The stored word at sp-0x18 remains unknown in the tool facts (`str` at 0xedb58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedb38 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedb40 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedb48 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xedb58 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xedb5c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xedb68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedb68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedb68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedb68 | `r0` | `[sp-24]` |

## `IcmbIpmbGetCmd`

- `IcmbIpmbGetCmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedb6c to 0xedba4.
  Established. A byte load reads entry r0+2 (`ldrb` at 0xedb8c).
  A word store at sp-0x18 records [r0+2]:1 (`str` at 0xedb90).
  The return record retains that loaded byte value (`ldrb` at 0xedb8c, `ldm` at 0xedba0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedb7c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedb84 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedb8c | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xedb90 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xedb94 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xedba0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedba0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedba0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedba0 | `r0` | `[r0+2]:1` |

## `IcmbIpmbGetCmpCode`

- `IcmbIpmbGetCmpCode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedba4 to 0xedbdc.
  Established. A word store saves entry r0 at sp-0x14 (`str` at 0xedbb4).
  The byte loaded from entry r0+3 is stored as a word at sp-0x18 (`ldrb` at 0xedbc4, `str` at 0xedbc8).
  The recorded return r0 is [r0+3]:1 (`ldrb` at 0xedbc4, `ldm` at 0xedbd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedbb4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedbbc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xedbc4 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xedbc8 | store | 4 | `sp -0x18` | `[r0+3]:1` |
| `ldr` at 0xedbcc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xedbd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedbd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedbd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedbd8 | `r0` | `[r0+3]:1` |

## `IcmbIpmbGetData`

- `IcmbIpmbGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedbdc to 0xedc38.
  Established. The call to unnamed 0xedb28 has its result stored at sp-0x18 (`bl` at 0xedbf4, `str` at 0xedbfc).
  The branch takes 0xedc1c when that result is nonzero and falls through when it is zero (`bl` at 0xedbf4, `bne` at 0xedc08).
  The two load, add and store sequences write [sp-20]+3 and [sp-20]+4 at sp-0x1c (`ldr` at 0xedc0c, `add` at 0xedc10, `str` at 0xedc14, `ldr` at 0xedc1c, `add` at 0xedc20, `str` at 0xedc24).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedbec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xedbf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xedbfc | store | 4 | `sp -0x18` | `ret(0xedb28)` |
| `ldr` at 0xedc00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xedc0c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xedc14 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xedc1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xedc24 | store | 4 | `sp -0x1c` | `[sp-20]+4` |
| `ldr` at 0xedc28 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xedc34 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedc34 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedc34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xedbf4 | `(unknown callee) 0xedb28` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xedc08 | `ret(0xedb28) != 0x0` | `ret(0xedb28) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedc34 | `r0` | `[sp-28]` |

## `IcmbPktBuild`

- `IcmbPktBuild`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xedcd0 to 0xeddac.
  Established. Halfword stores save the low 16 bits of entry r0 and r1 at sp-0x12 and sp-0x14 (`strh` at 0xedce4, `strh` at 0xedce8).
  A byte load, add of seven and byte store produce ((r3 & 0xff)+7 & 0xff) at sp-0x1a (`ldrb` at 0xedcf0, `add` at 0xedcf4, `strb` at 0xedcf8).
  A halfword store writes the low 16 bits of the result from unnamed 0xedc38 at sp-0x1c (`bl` at 0xedd5c, `strh` at 0xedd64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xedce0 | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0xedce4 | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strh` at 0xedce8 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0xedcec | store | 1 | `sp -0x19` | `(r3 & 0xff)` |
| `ldrb` at 0xedcf0 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xedcf8 | store | 1 | `sp -0x1a` | `((r3 & 0xff)+7 & 0xff)` |
| `ldrh` at 0xedcfc | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xedd00 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xedd0c | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xedd14 | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0xedd28 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xedd34 | store | 1 | `[sp] +0x4` | `0x0` |
| `ldr` at 0xedd38 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xedd40 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xedd48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xedd50 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xedd54 | load | 4 | `sp +0x0` | `-` |
| `strh` at 0xedd64 | store | 2 | `sp -0x1c` | `(ret(0xedc38) & 0xffff)` |
| `ldrb` at 0xedd68 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xedd6c | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xedd80 | load | 2 | `sp -0x1c` | `-` |
| `ldrb` at 0xedd9c | load | 1 | `sp -0x1a` | `-` |
| `ldm` at 0xedda8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xedda8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xedda8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xedd08 | `(unknown callee) 0xeded8` | `[sp]` | `(r1 & 0xffff)` | `r2` | `(r1 & 0xffff)` |
| `bl` at 0xedd20 | `(unknown callee) 0xeded8` | `[sp]+2` | `[sp-18]:2` | `[sp-18]:2` | `[sp]+2` |
| `bl` at 0xedd4c | `(unknown callee) 0x203f4` | `[sp]+5` | `[sp-24]` | `[sp-25]:1` | `[sp]+5` |
| `bl` at 0xedd5c | `(unknown callee) 0xedc38` | `[sp]` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xedd98 | `(unknown callee) 0xeded8` | `([sp-26]:1 + [sp])-2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xedda8 | `r0` | `[sp-26]:1` |

## `IcmbPktDecode`

- `IcmbPktDecode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeddac to 0xede7c.
  Established. A halfword store writes the low 16 bits of the result from unnamed 0xedc38 at sp-0x24 (`bl` at 0xedddc, `strh` at 0xedde4).
  Another halfword store writes the low 16 bits of the result from unnamed 0xede7c at the address held in [sp-28] (`bl` at 0xede38, `strh` at 0xede44).
  A byte load, subtraction of seven and byte store write ([sp-21]:1-7 & 0xff) at sp-0x21 (`ldrb` at 0xede48, `sub` at 0xede4c, `strb` at 0xede50).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeddbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeddc0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xeddc4 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xeddcc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xeddd0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xeddd4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xedde4 | store | 2 | `sp -0x24` | `(ret(0xedc38) & 0xffff)` |
| `ldrb` at 0xedde8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xeddec | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xede04 | store | 2 | `sp -0x26` | `(ret(0xede7c) & 0xffff)` |
| `ldrh` at 0xede08 | load | 2 | `sp -0x24` | `-` |
| `ldrh` at 0xede0c | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0xede14 | load | 4 | `abs 0xede78` | `-` |
| `str` at 0xede24 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xede2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xede40 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xede44 | store | 2 | `[sp-28] +0x0` | `(ret(0xede7c) & 0xffff)` |
| `ldrb` at 0xede48 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xede50 | store | 1 | `sp -0x21` | `([sp-21]:1-7 & 0xff)` |
| `ldr` at 0xede54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xede58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xede60 | store | 4 | `[sp-32] +0x0` | `[sp-20]+5` |
| `ldrb` at 0xede64 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xede68 | store | 4 | `sp -0x2c` | `[sp-33]:1` |
| `ldr` at 0xede6c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0xede74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xede74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xede74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xedddc | `(unknown callee) 0xedc38` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xeddfc | `(unknown callee) 0xede7c` | `([sp-21]:1 + [sp-20])-2` | `?` | `[sp-21]:1` | `([sp-21]:1 + [sp-20])-2` |
| `bl` at 0xede38 | `(unknown callee) 0xede7c` | `[sp-20]+2` | `?` | `([sp-36]:2 + (ret(0xede7c) & 0xffff))` | `[sp-20]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xede1c | `([sp-36]:2 + (ret(0xede7c) & 0xffff)) == 0xffff` | `([sp-36]:2 + (ret(0xede7c) & 0xffff)) != 0xffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xede74 | `r0` | `?` |
