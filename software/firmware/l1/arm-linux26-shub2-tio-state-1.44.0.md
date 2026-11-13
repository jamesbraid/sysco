# ARM Linux 2.6 SHUB2 and TIO state routines in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-shub2-tio-state-facts.txt) retain all tool sections.

## `SHUB2_ResetReleaseMask`

- `SHUB2_ResetReleaseMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e120 to 0x8e220.
  Established. An initial word store writes 0 at entry sp minus 0x20 (`str` at 0x8e138).
  The result from unnamed 0x3e274 is reduced by 1 and stored as a word at entry sp minus 0x18 (`bl` at 0x8e13c, `sub` at 0x8e144, `str` at 0x8e148).
  The result from unnamed 0x8dcdc is later stored as a word at entry sp minus 0x1c (`bl` at 0x8e1a8, `str` at 0x8e1b0).
  The return record places [sp-32] in r0 (`ldr` at 0x8e210, `ldm` at 0x8e21c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e130 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e138 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x8e148 | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8e14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e158 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e160 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x8e174 | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8e178 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e188 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e190 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e1a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e1b0 | store | 4 | `sp -0x1c` | `ret(0x8dcdc)` |
| `ldr` at 0x8e1b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e1c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e1cc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8e1d0 | store | 4 | `sp -0x20` | `ret(0x8dcdc)` |
| `ldr` at 0x8e1d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e1dc | store | 4 | `sp -0x18` | `?` |
| `str` at 0x8e1f0 | store | 4 | `sp -0x18` | `ret(0x3e274)-1` |
| `ldr` at 0x8e1f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e200 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e208 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x8e210 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8e21c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8e21c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8e21c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e13c | `(unknown callee) 0x3e274` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8e168 | `(unknown callee) 0x3e274` | `ret(0x3e274)` | `?` | `?` | `?` |
| `bl` at 0x8e1a8 | `(unknown callee) 0x8dcdc` | `?` | `0xff` | `?` | `?` |
| `bl` at 0x8e1e4 | `(unknown callee) 0x3e274` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x8e154 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x8e180 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8e19c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8e1bc | `ret(0x8dcdc) == 0x0` | `ret(0x8dcdc) != 0x0` |
| `bne` at 0x8e1c8 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x8e1fc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8e21c | `r0` | `[sp-32]` |

## `SHUB2_NMI`

- `SHUB2_NMI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e220 to 0x8e44c.
  Established. An initial word store writes 0 at entry sp minus 0x20 (`str` at 0x8e23c).
  A later word store there writes the result from unnamed 0x125a0c (`bl` at 0x8e388, `str` at 0x8e390).
  Another word store at that destination writes the result from a separate call to unnamed 0x1259bc (`bl` at 0x8e3ec, `str` at 0x8e3f4).
  The two word-store sites at entry sp minus 0x30 write 0xfffff0c4 and 0xfffff0c2 respectively (`str` at 0x8e280, `str` at 0x8e2e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e230 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e234 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8e23c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8e240 | load | 4 | `abs 0x8e43c` | `-` |
| `ldr` at 0x8e244 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8e250 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e264 | load | 4 | `abs 0x8e440` | `-` |
| `ldr` at 0x8e268 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8e270 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e27c | load | 4 | `abs 0x8e444` | `-` |
| `str` at 0x8e280 | store | 4 | `sp -0x30` | `0xfffff0c4` |
| `ldr` at 0x8e288 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e29c | load | 4 | `abs 0x8e440` | `-` |
| `ldr` at 0x8e2a0 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e2a8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8e2ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e2bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8e2c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e2c4 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x8e2c8 | store | 4 | `sp -0x2c` | `[[sp-36]+8]` |
| `ldr` at 0x8e2d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e2d4 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x8e2d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e2e4 | load | 4 | `abs 0x8e448` | `-` |
| `str` at 0x8e2e8 | store | 4 | `sp -0x30` | `0xfffff0c2` |
| `ldr` at 0x8e2f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e2f4 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0x8e2f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e300 | store | 4 | `sp -0x2c` | `r1+1` |
| `ldr` at 0x8e304 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8e308 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x8e30c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e310 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8e31c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e334 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e33c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e340 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e358 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e35c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e370 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e378 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e37c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e390 | store | 4 | `sp -0x20` | `ret(0x125a0c)` |
| `ldr` at 0x8e394 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e3ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e3b0 | store | 4 | `sp -0x1c` | `ret(0x125a0c)` |
| `ldr` at 0x8e3bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e3c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e3d4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e3dc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e3e0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e3f4 | store | 4 | `sp -0x20` | `ret(0x1259bc)` |
| `ldr` at 0x8e3f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e404 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e410 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e414 | store | 4 | `sp -0x1c` | `ret(0x1259bc)` |
| `ldr` at 0x8e418 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e420 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x8e428 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e42c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x8e430 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x8e438 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8e438 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8e438 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e348 | `(unknown callee) 0x125958` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e388 | `(unknown callee) 0x125a0c` | `[sp-20]` | `?` | `0x10` | `?` |
| `bl` at 0x8e3b8 | `(unknown callee) 0x8e44c` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x8e3ec | `(unknown callee) 0x1259bc` | `[sp-20]` | `?` | `0x10` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8e24c | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8e278 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8e2b4 | `r1 != 0xff` | `r1 == 0xff` |
| `bgt` at 0x8e2e0 | `[[sp-36]+8] > r1 (signed, 32-bit)` | `[[sp-36]+8] <= r1 (signed, 32-bit)` |
| `bge` at 0x8e318 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8e354 | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `beq` at 0x8e39c | `ret(0x125a0c) == 0x0` | `ret(0x125a0c) != 0x0` |
| `bne` at 0x8e3a8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x8e400 | `ret(0x1259bc) == 0x0` | `ret(0x1259bc) != 0x0` |
| `bne` at 0x8e40c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8e438 | `r0` | `?` |

## `SHUB2_SoftReset`

- `SHUB2_SoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e474 to 0x8e6a0.
  Established. The call to unnamed 0x125a0c has printed r0-r3 candidates [sp-20], unknown, 8 and unknown (`bl` at 0x8e5dc);
  its result is stored as a word at entry sp minus 0x20 (`bl` at 0x8e5dc, `str` at 0x8e5e4).
  A later call to unnamed 0x1259bc has its result stored as a word at that same destination (`bl` at 0x8e640, `str` at 0x8e648).
  The return record retains an unknown value in r0 (`ldm` at 0x8e68c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e484 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e488 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8e490 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8e494 | load | 4 | `abs 0x8e690` | `-` |
| `ldr` at 0x8e498 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8e4a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e4b8 | load | 4 | `abs 0x8e694` | `-` |
| `ldr` at 0x8e4bc | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8e4c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8e4d0 | load | 4 | `abs 0x8e698` | `-` |
| `str` at 0x8e4d4 | store | 4 | `sp -0x30` | `0xfffff0c4` |
| `ldr` at 0x8e4dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e4f0 | load | 4 | `abs 0x8e694` | `-` |
| `ldr` at 0x8e4f4 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e4fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8e500 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e510 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8e514 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e518 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x8e51c | store | 4 | `sp -0x2c` | `[[sp-36]+8]` |
| `ldr` at 0x8e524 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e528 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x8e52c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e538 | load | 4 | `abs 0x8e69c` | `-` |
| `str` at 0x8e53c | store | 4 | `sp -0x30` | `0xfffff0c2` |
| `ldr` at 0x8e544 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e548 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0x8e54c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e554 | store | 4 | `sp -0x2c` | `r1+1` |
| `ldr` at 0x8e558 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8e55c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x8e560 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e564 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8e570 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e574 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e588 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e590 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e5ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e5b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e5c4 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e5cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e5d0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e5e4 | store | 4 | `sp -0x20` | `ret(0x125a0c)` |
| `ldr` at 0x8e5e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e5f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e600 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e604 | store | 4 | `sp -0x1c` | `ret(0x125a0c)` |
| `ldr` at 0x8e610 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e614 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e628 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x8e630 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8e634 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e648 | store | 4 | `sp -0x20` | `ret(0x1259bc)` |
| `ldr` at 0x8e64c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e658 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e664 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e668 | store | 4 | `sp -0x1c` | `ret(0x1259bc)` |
| `ldr` at 0x8e66c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e674 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x8e67c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8e680 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x8e684 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x8e68c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8e68c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8e68c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e59c | `(unknown callee) 0x125958` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e5dc | `(unknown callee) 0x125a0c` | `[sp-20]` | `?` | `0x8` | `?` |
| `bl` at 0x8e60c | `(unknown callee) 0x8e44c` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x8e640 | `(unknown callee) 0x1259bc` | `[sp-20]` | `?` | `0x8` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8e4a0 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8e4cc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8e508 | `r1 != 0xff` | `r1 == 0xff` |
| `bgt` at 0x8e534 | `[[sp-36]+8] > r1 (signed, 32-bit)` | `[[sp-36]+8] <= r1 (signed, 32-bit)` |
| `bge` at 0x8e56c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x8e5a8 | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `beq` at 0x8e5f0 | `ret(0x125a0c) == 0x0` | `ret(0x125a0c) != 0x0` |
| `bne` at 0x8e5fc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x8e654 | `ret(0x1259bc) == 0x0` | `ret(0x1259bc) != 0x0` |
| `bne` at 0x8e660 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8e68c | `r0` | `?` |

## `SHUB2_SetLinkEnable`

- `SHUB2_SetLinkEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e6a0 to 0x8eaa4.
  Established. The result from one call to unnamed 0x6e190 is stored as a word at entry sp minus 0x40 (`bl` at 0x8e71c, `str` at 0x8e720).
  A separate call to that unnamed address has its result stored as a word at entry sp minus 0x44 (`bl` at 0x8e7c4, `str` at 0x8e7c8).
  The two word-store sites at entry sp minus 0x3c write 0xfffff0c4 and 0xfffff0c2 respectively (`str` at 0x8e798, `str` at 0x8e84c).
  A later word store writes the result from unnamed 0x13fd34 at entry sp minus 0x24 (`bl` at 0x8e928, `str` at 0x8e930).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e6b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e6b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8e6b8 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x8e6bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e6c4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x8e6c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e6e0 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x8e6e8 | load | 4 | `abs 0x8ea84` | `-` |
| `ldr` at 0x8e6ec | load | 4 | `abs 0x1a641c` | `-` |
| `str` at 0x8e720 | store | 4 | `sp -0x40` | `ret(0x6e190)` |
| `ldr` at 0x8e724 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8e72c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8e738 | load | 4 | `abs 0x8ea88` | `-` |
| `ldr` at 0x8e73c | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e744 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x8e74c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e754 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x8e758 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8e774 | load | 4 | `abs 0x8ea88` | `-` |
| `ldr` at 0x8e778 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e780 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x8e784 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x8e788 | load | 4 | `[sp-64] +0x8` | `-` |
| `ldr` at 0x8e794 | load | 4 | `abs 0x8ea8c` | `-` |
| `str` at 0x8e798 | store | 4 | `sp -0x3c` | `0xfffff0c4` |
| `str` at 0x8e7c8 | store | 4 | `sp -0x44` | `ret(0x6e190)` |
| `ldr` at 0x8e7cc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x8e7d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x8e7e0 | load | 4 | `abs 0x8ea88` | `-` |
| `ldr` at 0x8e7e4 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e7ec | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x8e7f4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e7fc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x8e800 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x8e81c | load | 4 | `abs 0x8ea88` | `-` |
| `ldr` at 0x8e820 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8e828 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x8e82c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8e830 | store | 4 | `sp -0x34` | `[sp-68]` |
| `ldr` at 0x8e834 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8e838 | load | 4 | `[sp-68] +0x8` | `-` |
| `ldr` at 0x8e83c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8e848 | load | 4 | `abs 0x8ea90` | `-` |
| `str` at 0x8e84c | store | 4 | `sp -0x3c` | `0xfffff0c2` |
| `ldr` at 0x8e854 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e860 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x8e864 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x8e880 | load | 4 | `[sp-68] +0x4` | `-` |
| `ldrb` at 0x8e888 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8e898 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x8e8a0 | load | 4 | `abs 0x8ea94` | `-` |
| `ldr` at 0x8e8a4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x8e8b0 | load | 4 | `abs 0x8ea98` | `-` |
| `ldr` at 0x8e8b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e8b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e8bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e8c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8e8c8 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0x8e8cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e8d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e8ec | load | 4 | `abs 0x8ea94` | `-` |
| `ldr` at 0x8e8f0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x8e8fc | load | 4 | `abs 0x8ea9c` | `-` |
| `ldr` at 0x8e900 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e910 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e918 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e930 | store | 4 | `sp -0x24` | `ret(0x13fd34)` |
| `ldr` at 0x8e938 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e944 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8e948 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x8e950 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8e960 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0x8e96c | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x8e970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e97c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e994 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8e998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e9a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e9b8 | load | 4 | `abs 0x8ea94` | `-` |
| `ldr` at 0x8e9bc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x8e9c8 | load | 4 | `abs 0x8eaa0` | `-` |
| `ldr` at 0x8e9cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e9d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e9dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e9e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8e9fc | store | 4 | `sp -0x24` | `ret(0x13fd34)` |
| `ldr` at 0x8ea04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ea10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8ea14 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `str` at 0x8ea20 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8ea24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ea30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ea48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8ea50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8ea68 | store | 4 | `sp -0x24` | `ret(0x13fd34)` |
| `ldr` at 0x8ea70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8ea74 | store | 4 | `sp -0x3c` | `[sp-36]` |
| `ldr` at 0x8ea78 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x8ea80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8ea80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8ea80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e6f8 | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `[0x1a641c]` |
| `bl` at 0x8e708 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x8e71c | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e7a0 | `(unknown callee) 0x6e140` | `?` | `?` | `[sp-64]` | `[[sp-64]+8]` |
| `bl` at 0x8e7b0 | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x8e7c4 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e8c0 | `(unknown callee) 0x243d4` | `0x167b7c` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x8e8e8 | `(unknown callee) 0x13f048` | `[sp-20]` | `[sp-24]+9` | `0x19` | `sp-32` |
| `bl` at 0x8e908 | `(unknown callee) 0x243d4` | `0x167bb4` | `[sp-24]` | `[sp-28]` | `[0x315034]` |
| `bl` at 0x8e90c | `(unknown callee) 0x134634` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e928 | `(unknown callee) 0x13fd34` | `[sp-20]` | `0x3` | `[sp-24]+9` | `0x0` |
| `bl` at 0x8e934 | `(unknown callee) 0x1346fc` | `ret(0x13fd34)` | `?` | `?` | `ret(0x13fd34)` |
| `bl` at 0x8e98c | `(unknown callee) 0x13f048` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x8e9b4 | `(unknown callee) 0x13f048` | `[sp-20]` | `[sp-24]+11` | `0x2` | `sp-32` |
| `bl` at 0x8e9d4 | `(unknown callee) 0x243d4` | `0x167bec` | `[sp-24]` | `[sp-28]` | `[0x315034]` |
| `bl` at 0x8e9d8 | `(unknown callee) 0x134634` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e9f4 | `(unknown callee) 0x13fd34` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x8ea00 | `(unknown callee) 0x1346fc` | `ret(0x13fd34)` | `?` | `?` | `ret(0x13fd34)` |
| `bl` at 0x8ea40 | `(unknown callee) 0x13f048` | `[sp-20]` | `[sp-24]+11` | `0x0` | `sp-32` |
| `bl` at 0x8ea44 | `(unknown callee) 0x134634` | `ret(0x13f048)` | `?` | `?` | `?` |
| `bl` at 0x8ea60 | `(unknown callee) 0x13fd34` | `[sp-20]` | `0x3` | `[sp-24]+11` | `0x0` |
| `bl` at 0x8ea6c | `(unknown callee) 0x1346fc` | `ret(0x13fd34)` | `?` | `?` | `ret(0x13fd34)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8e6d8 | `? == 0x7` | `? != 0x7` |
| `beq` at 0x8e6f4 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `beq` at 0x8e704 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0x8e714 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `bne` at 0x8e790 | `[[sp-64]+8] != 0x0` | `[[sp-64]+8] == 0x0` |
| `beq` at 0x8e7ac | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0x8e7bc | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `bgt` at 0x8e844 | `[[sp-68]+8] > [sp-56] (signed, 32-bit)` | `[[sp-68]+8] <= [sp-56] (signed, 32-bit)` |
| `beq` at 0x8e85c | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x8e890 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8e8ac | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bls` at 0x8e8f8 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x8e940 | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `bne` at 0x8e958 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bls` at 0x8e9c4 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x8ea0c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8ea80 | `r0` | `?` |

## `SHUB2_HandleJunkbusStatusInterrupt`

- `SHUB2_HandleJunkbusStatusInterrupt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8eaa4 to 0x8ef38.
  Established. A byte loaded from absolute address 0x167c20 is stored as a byte at entry sp minus 0x90 (`ldrb` at 0x8eac4, `strb` at 0x8eac8).
  The result from unnamed 0x202c8 is stored as a word at entry sp minus 0x98 (`bl` at 0x8eaec, `str` at 0x8eaf0).
  A separate call to unnamed 0x13ce4c has its result stored as a word at entry sp minus 0x28 (`bl` at 0x8eb24, `str` at 0x8eb2c).
  The return record places 0 in r0 (`ldm` at 0x8eef0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8eab4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x8eabc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x8eac0 | load | 4 | `abs 0x8eef4` | `-` |
| `ldrb` at 0x8eac4 | load | 1 | `abs 0x167c20` | `-` |
| `strb` at 0x8eac8 | store | 1 | `sp -0x90` | `([0x167c20]:1 & 0xff)` |
| `str` at 0x8eae4 | store | 4 | `sp -0x94` | `sp-144` |
| `str` at 0x8eaf0 | store | 4 | `sp -0x98` | `ret(0x202c8)` |
| `ldr` at 0x8eaf4 | load | 4 | `abs 0x8eef8` | `-` |
| `ldr` at 0x8eaf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8eb08 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8eb0c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8eb2c | store | 4 | `sp -0x28` | `ret(0x13ce4c)` |
| `ldr` at 0x8eb30 | load | 4 | `abs 0x8eefc` | `-` |
| `str` at 0x8eb3c | store | 4 | `sp -0x20` | `ret(0x13e280)` |
| `str` at 0x8eb44 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8eb48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8eb4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8eb58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8eb64 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x8eb68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8eb74 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x8eb7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8eb84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8eb9c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x8ebac | store | 4 | `sp -0x1c` | `ret(0x8bbac)` |
| `ldr` at 0x8ebb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ebbc | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x8ebc8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ebd4 | load | 4 | `abs 0x8ef00` | `-` |
| `str` at 0x8ebd8 | store | 4 | `(unknown)` | `0x167c84` |
| `str` at 0x8ebdc | store | 4 | `(unknown)` | `ret(0x87564)` |
| `str` at 0x8ebe0 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x8ebe8 | load | 4 | `abs 0x8ef04` | `-` |
| `ldr` at 0x8ebec | load | 4 | `abs 0x8ef08` | `-` |
| `ldr` at 0x8ebfc | load | 4 | `abs 0x8ef0c` | `-` |
| `ldr` at 0x8ec00 | load | 4 | `abs 0x1a647c` | `-` |
| `ldrb` at 0x8ec10 | load | 1 | `sp -0x9c` | `-` |
| `ldrb` at 0x8ec28 | load | 1 | `sp -0x9c` | `-` |
| `ldrb` at 0x8ec40 | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ec54 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x8ec60 | load | 4 | `abs 0x8ef10` | `-` |
| `str` at 0x8ec64 | store | 4 | `(unknown)` | `0x167cb4` |
| `str` at 0x8ec68 | store | 4 | `(unknown)` | `ret(0x87564)` |
| `ldrb` at 0x8ec6c | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ec7c | load | 4 | `abs 0x8ef14` | `-` |
| `str` at 0x8ec80 | store | 4 | `sp -0xb4` | `0x167cd0` |
| `ldr` at 0x8ec88 | load | 4 | `abs 0x8ef18` | `-` |
| `str` at 0x8ec8c | store | 4 | `sp -0xb4` | `0x167cdc` |
| `ldr` at 0x8ec90 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x8ec94 | store | 4 | `(unknown)` | `[sp-180]` |
| `ldrb` at 0x8ec98 | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ecb0 | load | 4 | `abs 0x8ef1c` | `-` |
| `str` at 0x8ecb4 | store | 4 | `sp -0xb8` | `0x167ce0` |
| `ldr` at 0x8ecbc | load | 4 | `abs 0x8ef18` | `-` |
| `str` at 0x8ecc0 | store | 4 | `sp -0xb8` | `0x167cdc` |
| `ldr` at 0x8ecc4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x8ecc8 | store | 4 | `(unknown)` | `[sp-184]` |
| `ldrb` at 0x8eccc | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ece4 | load | 4 | `abs 0x8ef20` | `-` |
| `str` at 0x8ece8 | store | 4 | `sp -0xbc` | `0x167cec` |
| `ldr` at 0x8ecf0 | load | 4 | `abs 0x8ef18` | `-` |
| `str` at 0x8ecf4 | store | 4 | `sp -0xbc` | `0x167cdc` |
| `ldr` at 0x8ecf8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x8ecfc | store | 4 | `(unknown)` | `[sp-188]` |
| `ldrb` at 0x8ed00 | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ed18 | load | 4 | `abs 0x8ef24` | `-` |
| `str` at 0x8ed1c | store | 4 | `sp -0xc0` | `0x167cf8` |
| `ldr` at 0x8ed24 | load | 4 | `abs 0x8ef18` | `-` |
| `str` at 0x8ed28 | store | 4 | `sp -0xc0` | `0x167cdc` |
| `ldr` at 0x8ed2c | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x8ed30 | store | 4 | `(unknown)` | `[sp-192]` |
| `ldr` at 0x8ed38 | load | 4 | `abs 0x8ef04` | `-` |
| `ldr` at 0x8ed3c | load | 4 | `abs 0x8ef28` | `-` |
| `ldrb` at 0x8ed48 | load | 1 | `sp -0x9c` | `-` |
| `ldr` at 0x8ed60 | load | 4 | `abs 0x8eef8` | `-` |
| `ldr` at 0x8ed64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8ed68 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8ed6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ed7c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8ed84 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldrb` at 0x8ed88 | load | 1 | `sp -0x90` | `-` |
| `ldr` at 0x8ed9c | load | 4 | `abs 0x8ef2c` | `-` |
| `str` at 0x8edbc | store | 4 | `sp -0x94` | `(sp-144 + ret(0x2031c))` |
| `ldr` at 0x8edd0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8ede4 | store | 4 | `sp -0xa8` | `ret(0x6e1b0)` |
| `ldr` at 0x8ede8 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x8edf8 | store | 4 | `sp -0xac` | `ret(0x2031c)+1` |
| `ldr` at 0x8edfc | load | 4 | `sp -0xac` | `-` |
| `str` at 0x8ee24 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x8ee28 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8ee2c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x8ee30 | load | 4 | `sp -0xac` | `-` |
| `str` at 0x8ee3c | store | 4 | `sp -0xc4` | `ret(0x203f4)` |
| `ldr` at 0x8ee44 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x8ee4c | store | 4 | `sp -0xc4` | `ret(0x13ccc0)` |
| `ldr` at 0x8ee50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8ee5c | load | 4 | `abs 0x8ef30` | `-` |
| `str` at 0x8ee60 | store | 4 | `sp -0xc8` | `0x167d18` |
| `ldr` at 0x8ee68 | load | 4 | `abs 0x8ef18` | `-` |
| `str` at 0x8ee6c | store | 4 | `sp -0xc8` | `0x167cdc` |
| `ldr` at 0x8ee70 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x8ee74 | load | 4 | `abs 0x8ef34` | `-` |
| `ldr` at 0x8ee78 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x8ee7c | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x8ee84 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x8ee94 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8ee9c | store | 4 | `sp -0x94` | `([sp-148] + ret(0x2031c))` |
| `ldr` at 0x8eea0 | load | 4 | `abs 0x8eef8` | `-` |
| `ldr` at 0x8eea4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8eea8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x8eeac | store | 4 | `(unknown)` | `[sp-152]` |
| `ldr` at 0x8eeb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8eebc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8eec4 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x8eecc | load | 1 | `sp -0x90` | `-` |
| `ldm` at 0x8eef0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x8eef0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8eef0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8eef0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8eadc | `(unknown callee) 0x20550` | `sp-143` | `0x0` | `0x63` | `sp-143` |
| `bl` at 0x8eaec | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `sp-144` |
| `bl` at 0x8eb20 | `(unknown callee) 0x8e44c` | `0x14` | `0x1a6420` | `ret(0x202c8)` | `?` |
| `bl` at 0x8eb24 | `(unknown callee) 0x13ce4c` | `?` | `?` | `?` | `?` |
| `bl` at 0x8eb34 | `(unknown callee) 0x13e280` | `0xff0700` | `?` | `?` | `ret(0x13ce4c)` |
| `bl` at 0x8eba4 | `(unknown callee) 0x8bbac` | `?` | `sp-156` | `?` | `sp-156` |
| `bl` at 0x8ebc0 | `(unknown callee) 0x87564` | `[sp-160]` | `?` | `?` | `ret(0x8bbac)` |
| `bl` at 0x8ebcc | `(unknown callee) 0x219a4` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x8ebf4 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0x3fa` | `0x0` |
| `bl` at 0x8ec58 | `(unknown callee) 0x87564` | `[sp-160]` | `?` | `?` | `?` |
| `bl` at 0x8ed44 | `(unknown callee) 0x22070` | `0x1` | `0x1679cc` | `0x404` | `0x0` |
| `bl` at 0x8eda0 | `(unknown callee) 0x20634` | `sp-144` | `0x167d08` | `[sp-152]` | `sp-144` |
| `bl` at 0x8edb0 | `(unknown callee) 0x2031c` | `sp-144` | `?` | `?` | `sp-144` |
| `bl` at 0x8edc0 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `?` |
| `bl` at 0x8ede0 | `(unknown callee) 0x6e1b0` | `[sp-36]` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x8edec | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x8ee34 | `(unknown callee) 0x203f4` | `[sp-176]` | `[sp-168]` | `[sp-172]` | `?` |
| `bl` at 0x8ee48 | `(unknown callee) 0x13ccc0` | `[sp-164]` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x8ee80 | `(unknown callee) 0x205bc` | `[sp-148]` | `0x167d1c` | `[sp-200]` | `?` |
| `bl` at 0x8ee88 | `(unknown callee) 0x2031c` | `[sp-148]` | `?` | `?` | `?` |
| `bl` at 0x8eeb8 | `(unknown callee) 0xb5e80` | `[sp-36]` | `0x0` | `?` | `?` |
| `bl` at 0x8eee0 | `(unknown callee) 0x293f8` | `sp-144` | `?` | `?` | `sp-144` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x8eb18 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `bge` at 0x8eb54 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x8eb90 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ebb8 | `ret(0x8bbac) == 0x0` | `ret(0x8bbac) != 0x0` |
| `beq` at 0x8ec0c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x8ec24 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ec3c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8ec4c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8ec78 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8ecac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8ece0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8ed14 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8ed5c | `? == 0x0` | `? != 0x0` |
| `ble` at 0x8ed78 | `? <= 0x3 (signed, 32-bit)` | `? > 0x3 (signed, 32-bit)` |
| `bne` at 0x8ed90 | `[sp-144]:1 != 0x0` | `[sp-144]:1 == 0x0` |
| `ble` at 0x8edcc | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `ble` at 0x8ee58 | `[sp-44] <= 0x1 (signed, 32-bit)` | `[sp-44] > 0x1 (signed, 32-bit)` |
| `beq` at 0x8eed4 | `[sp-144]:1 == 0x0` | `[sp-144]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8eef0 | `r0` | `0x0` |

## `SHUB2_Target`

- `SHUB2_Target`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8ef38 to 0x8f0a4.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x8ef48, `str` at 0x8ef4c).
  The two word-store sites at entry sp minus 0x20 write 0xfffff0c4 and 0xfffff0c2 respectively (`str` at 0x8ef90, `str` at 0x8efd4).
  A later computed word load has an unknown effective address (`ldr` at 0x8f078).
  The return record retains an unknown value in r0 (`ldm` at 0x8f090).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8ef48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8ef4c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8ef50 | load | 4 | `abs 0x8f094` | `-` |
| `ldr` at 0x8ef54 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8ef60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8ef74 | load | 4 | `abs 0x8f098` | `-` |
| `ldr` at 0x8ef78 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8ef80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8ef8c | load | 4 | `abs 0x8f09c` | `-` |
| `str` at 0x8ef90 | store | 4 | `sp -0x20` | `0xfffff0c4` |
| `ldr` at 0x8ef98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8efac | load | 4 | `abs 0x8f098` | `-` |
| `ldr` at 0x8efb0 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8efb8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8efbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8efc0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x8efc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8efd0 | load | 4 | `abs 0x8f0a0` | `-` |
| `str` at 0x8efd4 | store | 4 | `sp -0x20` | `0xfffff0c2` |
| `ldr` at 0x8eff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f000 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f018 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8f020 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8f028 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f02c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f034 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f048 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f050 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f058 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f05c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f070 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x8f078 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8f07c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f080 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f084 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x8f088 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8f090 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f090 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f090 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8efdc | `(unknown callee) 0x6e140` | `r0` | `r1` | `[[sp-28]+8]` | `r1` |
| `bl` at 0x8efec | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8ef5c | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8ef88 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8efcc | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |
| `ble` at 0x8efe8 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0x8effc | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f090 | `r0` | `?` |

## `SHUB2_IsRunning`

- `SHUB2_IsRunning`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f0a4 to 0x8f194.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x8f0b4, `str` at 0x8f0b8).
  The two word-store sites at entry sp minus 0x20 write 0xfffff0c4 and 0xfffff0c2 respectively (`str` at 0x8f0fc, `str` at 0x8f140).
  The later computed byte load has an unknown effective address (`ldrb` at 0x8f170).
  The return record retains an unknown value in r0 (`ldm` at 0x8f180).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f0b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8f0b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8f0bc | load | 4 | `abs 0x8f184` | `-` |
| `ldr` at 0x8f0c0 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8f0cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f0e0 | load | 4 | `abs 0x8f188` | `-` |
| `ldr` at 0x8f0e4 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8f0ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f0f8 | load | 4 | `abs 0x8f18c` | `-` |
| `str` at 0x8f0fc | store | 4 | `sp -0x20` | `0xfffff0c4` |
| `ldr` at 0x8f104 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f118 | load | 4 | `abs 0x8f188` | `-` |
| `ldr` at 0x8f11c | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8f124 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f12c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x8f130 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f13c | load | 4 | `abs 0x8f190` | `-` |
| `str` at 0x8f140 | store | 4 | `sp -0x20` | `0xfffff0c2` |
| `ldr` at 0x8f148 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f14c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f168 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x8f170 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8f174 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8f178 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8f180 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f180 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f180 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8f0c8 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8f0f4 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8f138 | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f180 | `r0` | `?` |

## `SHUB2_HT_IsAllowed`

- `SHUB2_HT_IsAllowed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f194 to 0x8f284.
  Established. A word load reads the address formed by [sp-28] plus 4 (`ldr` at 0x8f258).
  A subsequent computed byte load has an unknown effective address (`ldrb` at 0x8f260).
  The following word store at entry sp minus 0x20 has an unknown value (`str` at 0x8f264).
  The return record retains an unknown value in r0 (`ldm` at 0x8f270).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f1a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8f1a8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8f1ac | load | 4 | `abs 0x8f274` | `-` |
| `ldr` at 0x8f1b0 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8f1bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f1d0 | load | 4 | `abs 0x8f278` | `-` |
| `ldr` at 0x8f1d4 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8f1dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f1e8 | load | 4 | `abs 0x8f27c` | `-` |
| `str` at 0x8f1ec | store | 4 | `sp -0x20` | `0xfffff0c4` |
| `ldr` at 0x8f1f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f208 | load | 4 | `abs 0x8f278` | `-` |
| `ldr` at 0x8f20c | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8f214 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f218 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f21c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x8f220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f22c | load | 4 | `abs 0x8f280` | `-` |
| `str` at 0x8f230 | store | 4 | `sp -0x20` | `0xfffff0c2` |
| `ldr` at 0x8f238 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f23c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f258 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x8f260 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8f264 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8f268 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8f270 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f270 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8f1b8 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8f1e4 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8f228 | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f270 | `r0` | `?` |

## `SHUB2_FastFSB_IsAllowed`

- `SHUB2_FastFSB_IsAllowed`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f284 to 0x8f380.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x8f294, `str` at 0x8f298).
  The two word-store sites at entry sp minus 0x20 write 0xfffff0c4 and 0xfffff0c2 respectively (`str` at 0x8f2dc, `str` at 0x8f320).
  A later computed word load has an unknown effective address (`ldr` at 0x8f350).
  The word store at entry sp minus 0x20 following the merged predicated effects has an unknown value (`movne` at 0x8f358, `moveq` at 0x8f35c, `str` at 0x8f360).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f294 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8f298 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8f29c | load | 4 | `abs 0x8f370` | `-` |
| `ldr` at 0x8f2a0 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8f2ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f2c0 | load | 4 | `abs 0x8f374` | `-` |
| `ldr` at 0x8f2c4 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8f2cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f2d8 | load | 4 | `abs 0x8f378` | `-` |
| `str` at 0x8f2dc | store | 4 | `sp -0x20` | `0xfffff0c4` |
| `ldr` at 0x8f2e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f2f8 | load | 4 | `abs 0x8f374` | `-` |
| `ldr` at 0x8f2fc | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8f304 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f308 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f30c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x8f310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f31c | load | 4 | `abs 0x8f37c` | `-` |
| `str` at 0x8f320 | store | 4 | `sp -0x20` | `0xfffff0c2` |
| `ldr` at 0x8f328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f32c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f348 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x8f350 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8f360 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8f364 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x8f36c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f36c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f36c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8f2a8 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8f2d4 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8f318 | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f36c | `r0` | `?` |

## `SHUB2_CPU_IsPresent`

- `SHUB2_CPU_IsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f380 to 0x8f4fc.
  Established. The call to unnamed 0x28230 has printed r0-r3 candidates 0x9c, 8, [sp-36] and unknown (`bl` at 0x8f4ac).
  One word store writes 0 at entry sp minus 0x28 (`str` at 0x8f47c).
  A later byte load reads the address formed by [sp-36] plus 3 (`ldrb` at 0x8f4c0).
  Another word store writes 1 at entry sp minus 0x28 (`str` at 0x8f4dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f390 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8f394 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8f398 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x8f39c | load | 4 | `abs 0x8f4ec` | `-` |
| `ldr` at 0x8f3a0 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8f3ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f3c0 | load | 4 | `abs 0x8f4f0` | `-` |
| `ldr` at 0x8f3c4 | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8f3cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f3d8 | load | 4 | `abs 0x8f4f4` | `-` |
| `str` at 0x8f3dc | store | 4 | `sp -0x28` | `0xfffff0c4` |
| `ldr` at 0x8f3e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f3f8 | load | 4 | `abs 0x8f4f0` | `-` |
| `ldr` at 0x8f3fc | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8f404 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8f408 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8f40c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0x8f410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f41c | load | 4 | `abs 0x8f4f8` | `-` |
| `str` at 0x8f420 | store | 4 | `sp -0x28` | `0xfffff0c2` |
| `ldr` at 0x8f428 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8f42c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f448 | load | 4 | `[sp-32] +0x4` | `-` |
| `str` at 0x8f450 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f454 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f458 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x8f46c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8f47c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x8f484 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f488 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x8f49c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8f4bc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x8f4c0 | load | 1 | `[sp-36] +0x3` | `-` |
| `str` at 0x8f4d0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x8f4dc | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x8f4e0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x8f4e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f4e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f4e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8f4ac | `(unknown callee) 0x28230` | `0x9c` | `0x8` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8f3a8 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8f3d4 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8f418 | `[[sp-32]+8] > r1 (signed, 32-bit)` | `[[sp-32]+8] <= r1 (signed, 32-bit)` |
| `bne` at 0x8f474 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8f4a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x8f4b8 | `ret(0x28230) == 0x1` | `ret(0x28230) != 0x1` |
| `bne` at 0x8f4c8 | `[[sp-36]+3]:1 != 0x0` | `[[sp-36]+3]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f4e8 | `r0` | `?` |

## `SHUB2_GetState`

- `SHUB2_GetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f4fc to 0x8f5e0.
  Established. One word store writes 0 at entry sp minus 0x24 (`str` at 0x8f554).
  A separate later word store writes 0 at that same destination (`str` at 0x8f598).
  Another word store there has an unknown value (`str` at 0x8f5c8).
  The return record retains an unknown value in r0 (`ldm` at 0x8f5d4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f50c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8f510 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8f514 | load | 4 | `abs 0x8f5d8` | `-` |
| `ldr` at 0x8f518 | load | 4 | `abs 0x1a641c` | `-` |
| `ldr` at 0x8f524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f538 | load | 4 | `abs 0x8f5dc` | `-` |
| `ldr` at 0x8f53c | load | 4 | `abs 0x1a6418` | `-` |
| `ldr` at 0x8f544 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8f554 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8f55c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f570 | load | 4 | `abs 0x8f5dc` | `-` |
| `ldr` at 0x8f574 | load | 4 | `abs 0x1a6418` | `-` |
| `str` at 0x8f57c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f580 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f584 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x8f588 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8f598 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x8f5a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f5a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f5c0 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x8f5c8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f5cc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x8f5d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x8f5d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x8f5d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8f520 | `[0x1a641c] == 0x0` | `[0x1a641c] != 0x0` |
| `bne` at 0x8f54c | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x8f590 | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x8f5d4 | `r0` | `?` |

## `TIOCfgInit`

- `TIOCfgInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x730cc to 0x73524.
  Established. The result from unnamed 0x6e140 is stored as a word at absolute address 0x1a6214 (`bl` at 0x730fc, `str` at 0x73108).
  The result from a call to unnamed 0x204d8 is stored as a word at absolute address 0x1a6210 (`bl` at 0x73134, `str` at 0x7313c).
  A separate call to unnamed 0x204d8 has its result stored as a word at the address formed by [sp-52] plus 4 (`bl` at 0x731bc, `str` at 0x731c4).
  The return record preserves the result from the final call to unnamed 0x790b4 (`bl` at 0x734e4, `ldm` at 0x734ec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x730dc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x730e0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x730e4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x730e8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x730ec | load | 4 | `abs 0x734f0` | `-` |
| `ldr` at 0x730f0 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x73104 | load | 4 | `abs 0x734f4` | `-` |
| `str` at 0x73108 | store | 4 | `abs 0x1a6214` | `ret(0x6e140)` |
| `ldr` at 0x7310c | load | 4 | `abs 0x734f4` | `-` |
| `ldr` at 0x73110 | load | 4 | `abs 0x1a6214` | `-` |
| `str` at 0x73124 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x73128 | load | 4 | `abs 0x734f0` | `-` |
| `ldr` at 0x7312c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x7313c | store | 4 | `abs 0x1a6210` | `ret(0x204d8)` |
| `ldr` at 0x73140 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x7314c | load | 4 | `abs 0x734f8` | `-` |
| `str` at 0x73150 | store | 4 | `sp -0x50` | `0x165418` |
| `ldr` at 0x73154 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x73158 | store | 4 | `sp -0x4c` | `[sp-56]` |
| `ldr` at 0x73160 | load | 4 | `abs 0x734fc` | `-` |
| `ldr` at 0x73170 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73184 | load | 4 | `abs 0x734f0` | `-` |
| `ldr` at 0x73188 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x73190 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x73194 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73198 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7319c | store | 4 | `[sp-52] +0x0` | `?` |
| `ldr` at 0x731a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x731a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x731a8 | store | 4 | `[sp-52] +0x8` | `[sp-32]` |
| `ldr` at 0x731ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x731b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x731b4 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x731c4 | store | 4 | `[sp-52] +0x4` | `ret(0x204d8)` |
| `ldr` at 0x731c8 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x731d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x731e8 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x731ec | load | 4 | `sp -0x38` | `-` |
| `str` at 0x731fc | store | 4 | `sp -0x3c` | `ret(0x2031c)+1` |
| `ldr` at 0x73200 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x73228 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x7322c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x73230 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x73234 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x73240 | load | 4 | `abs 0x73500` | `-` |
| `str` at 0x73244 | store | 4 | `(unknown)` | `0x165464` |
| `str` at 0x73248 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x7324c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73250 | load | 4 | `[sp-52] +0x8` | `-` |
| `str` at 0x73254 | store | 4 | `(unknown)` | `[[sp-52]+8]` |
| `ldr` at 0x7325c | load | 4 | `abs 0x734fc` | `-` |
| `ldr` at 0x73260 | load | 4 | `abs 0x73504` | `-` |
| `str` at 0x73270 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x73274 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73278 | load | 4 | `[sp-52] +0x8` | `-` |
| `ldr` at 0x7327c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73288 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7328c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73294 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x7329c | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x732a0 | store | 1 | `(unknown)` | `([sp-24] & 0xff)` |
| `ldr` at 0x732a4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x732a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x732b0 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x732b8 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x732bc | store | 1 | `(unknown)` | `([sp-48] & 0xff)` |
| `ldr` at 0x732c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x732c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x732cc | load | 4 | `[sp-52] +0x4` | `-` |
| `strb` at 0x732d8 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x732dc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x732e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x732e8 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldr` at 0x732f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x732f4 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x73304 | store | 1 | `(unknown)` | `(([sp-36] + [sp-48]) & 0xff)` |
| `ldr` at 0x73308 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7331c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x73324 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73328 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73340 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73344 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7334c | load | 4 | `[sp-52] +0x4` | `-` |
| `strb` at 0x73358 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x7335c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73360 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73368 | load | 4 | `[sp-52] +0x4` | `-` |
| `ldrb` at 0x73370 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73394 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x733a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x733a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x733b8 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x733c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x733c8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x733cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x733d4 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x733d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x733e8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x733f0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x733f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x733fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73410 | load | 4 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x73418 | load | 4 | `(unknown)` | `-` |
| `str` at 0x7341c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x73420 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x73424 | store | 4 | `sp -0x40` | `[sp-68]` |
| `ldr` at 0x73428 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7343c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73444 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x73448 | load | 4 | `(unknown)` | `-` |
| `str` at 0x73454 | store | 4 | `sp -0x2c` | `ret(0x13ba30)` |
| `ldr` at 0x73458 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73464 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73470 | load | 4 | `abs 0x73508` | `-` |
| `str` at 0x73474 | store | 4 | `(unknown)` | `0x16549c` |
| `ldr` at 0x73478 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7348c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73494 | load | 4 | `(unknown)` | `-` |
| `str` at 0x73498 | store | 4 | `(unknown)` | `?` |
| `str` at 0x7349c | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x734a4 | load | 4 | `abs 0x734fc` | `-` |
| `ldr` at 0x734a8 | load | 4 | `abs 0x7350c` | `-` |
| `ldr` at 0x734b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x734bc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x734c4 | load | 4 | `abs 0x73510` | `-` |
| `str` at 0x734c8 | store | 4 | `(unknown)` | `0x7490c` |
| `ldr` at 0x734cc | load | 4 | `abs 0x73514` | `-` |
| `str` at 0x734d0 | store | 4 | `(unknown)` | `0x74d6c` |
| `ldr` at 0x734d8 | load | 4 | `abs 0x73518` | `-` |
| `ldr` at 0x734dc | load | 4 | `abs 0x7351c` | `-` |
| `ldr` at 0x734e0 | load | 4 | `abs 0x73520` | `-` |
| `ldm` at 0x734ec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x734ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x734ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x734ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x730fc | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `[0x1a6210]` |
| `bl` at 0x73134 | `(unknown callee) 0x204d8` | `[sp-56]` | `0x1` | `[0x1a6214]` | `?` |
| `bl` at 0x7316c | `(unknown callee) 0x22070` | `0x9` | `0x165450` | `0xf8` | `0x0` |
| `bl` at 0x731bc | `(unknown callee) 0x204d8` | `[[sp-52]+8]` | `0x8` | `[sp-52]` | `[sp-52]` |
| `bl` at 0x731e4 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x731f0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x73238 | `(unknown callee) 0x203f4` | `[sp-64]` | `[sp-56]` | `[sp-60]` | `?` |
| `bl` at 0x73268 | `(unknown callee) 0x22070` | `0x9` | `0x165450` | `0x106` | `0x0` |
| `bl` at 0x73330 | `(unknown callee) 0x125958` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x7337c | `(unknown callee) 0x6e140` | `ret(0x125958)` | `[sp-52]` | `?` | `?` |
| `bl` at 0x7338c | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x7344c | `(unknown callee) 0x13ba30` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x73468 | `(unknown callee) 0x219a4` | `ret(0x13ba30)` | `?` | `?` | `ret(0x13ba30)` |
| `bl` at 0x734b0 | `(unknown callee) 0x22070` | `0x9` | `0x165450` | `0x119` | `0x0` |
| `bl` at 0x734e4 | `(unknown callee) 0x790b4` | `0x1` | `0x74020` | `0x77384` | `0x774a4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x730f8 | `[0x1a6210] != 0x0` | `[0x1a6210] == 0x0` |
| `bne` at 0x73148 | `[0x1a6210] != 0x0` | `[0x1a6210] == 0x0` |
| `bne` at 0x731d0 | `[[sp-52]+4] != 0x0` | `[[sp-52]+4] == 0x0` |
| `ble` at 0x73284 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7333c | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `beq` at 0x73378 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x73388 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0x7339c | `ret(0x6e190) == [sp-24]` | `ret(0x6e190) != [sp-24]` |
| `beq` at 0x73460 | `ret(0x13ba30) == 0x0` | `ret(0x13ba30) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x734ec | `r0` | `ret(0x790b4)` |

## `TIORemove`

- `TIORemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73524 to 0x735ec.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x73534).
  Later word-store sites write 0 to [sp-24] plus 8, to the address held in [sp-24], and to [sp-24] plus 4 (`str` at 0x735b4, `str` at 0x735c0, `str` at 0x735d8).
  A word loaded from [sp-24] plus 4 supplies the printed r0 candidate for a call to unnamed 0x20514 (`ldr` at 0x735c8, `bl` at 0x735cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73534 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x73538 | load | 4 | `abs 0x735e4` | `-` |
| `ldr` at 0x7353c | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x73548 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7355c | load | 4 | `abs 0x735e8` | `-` |
| `ldr` at 0x73560 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x73568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73578 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7358c | load | 4 | `abs 0x735e8` | `-` |
| `ldr` at 0x73590 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x73598 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x7359c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x735a0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x735ac | load | 4 | `sp -0x18` | `-` |
| `str` at 0x735b4 | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x735b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x735c0 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x735c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x735c8 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x735d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x735d8 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldm` at 0x735e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x735e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x735e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x735cc | `(unknown callee) 0x20514` | `[[sp-24]+4]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x73544 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x73570 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x735a8 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x735e0 | `r0` | `?` |

## `TIOInit`

- `TIOInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x735ec to 0x73960.
  Established. An initial word store writes 0 at entry sp minus 0x20 (`str` at 0x7367c).
  A word store writes 1 to the address formed by [sp-40] plus 0xc (`str` at 0x73688).
  The result from unnamed 0x125b8c is stored as a word at entry sp minus 0x24 (`bl` at 0x736e0, `str` at 0x736e8).
  A later word store writes 0 to the address formed by [sp-40] plus 0xc (`str` at 0x737a4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x735fc | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x73600 | load | 4 | `abs 0x73944` | `-` |
| `ldr` at 0x73604 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x73610 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73624 | load | 4 | `abs 0x73948` | `-` |
| `ldr` at 0x73628 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x73630 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x73654 | load | 4 | `abs 0x73948` | `-` |
| `ldr` at 0x73658 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x73660 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x73664 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7367c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x73680 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x73688 | store | 4 | `[sp-40] +0xc` | `0x1` |
| `ldr` at 0x7368c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73690 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x73694 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x736a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x736a4 | load | 4 | `[sp-40] +0xc` | `-` |
| `ldr` at 0x736b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x736b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x736c8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x736d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x736d8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x736e8 | store | 4 | `sp -0x24` | `ret(0x125b8c)` |
| `ldr` at 0x736ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x736f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7370c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x73710 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x73720 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x73724 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7374c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x73750 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x73754 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73758 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73764 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73770 | load | 4 | `abs 0x7394c` | `-` |
| `str` at 0x73774 | store | 4 | `(unknown)` | `0x1654b8` |
| `str` at 0x73778 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x7377c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73780 | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x73784 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x7378c | load | 4 | `abs 0x73950` | `-` |
| `ldr` at 0x73790 | load | 4 | `abs 0x73954` | `-` |
| `ldr` at 0x7379c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x737a4 | store | 4 | `[sp-40] +0xc` | `0x0` |
| `ldrb` at 0x737ac | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x737bc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x737c4 | store | 4 | `[sp-40] +0xc` | `0x0` |
| `ldr` at 0x737c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x737d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x737d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x737dc | load | 4 | `[sp-40] +0xc` | `-` |
| `str` at 0x737ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x737f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x737f4 | load | 4 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x737f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73804 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7381c | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0x73824 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73830 | load | 4 | `[sp-40] +0x4` | `-` |
| `str` at 0x73844 | store | 4 | `sp -0x24` | `ret(0x751a8)` |
| `ldr` at 0x73848 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73854 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x73868 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x7386c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7387c | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x73880 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x738a8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x738ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x738b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x738b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x738c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x738cc | load | 4 | `abs 0x73958` | `-` |
| `str` at 0x738d0 | store | 4 | `(unknown)` | `0x1654dc` |
| `str` at 0x738d4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x738d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x738dc | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x738e0 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x738e8 | load | 4 | `abs 0x73950` | `-` |
| `ldr` at 0x738ec | load | 4 | `abs 0x7395c` | `-` |
| `ldr` at 0x738fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x73900 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73914 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldrb` at 0x7391c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7392c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73934 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldm` at 0x73940 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x73940 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x73940 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x73940 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x73668 | `(unknown callee) 0x558d8` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x736e0 | `(unknown callee) 0x125b8c` | `[sp-24]` | `?` | `sp-25` | `?` |
| `bl` at 0x73708 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x73714 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7375c | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x73768 | `(unknown callee) 0x21ce0` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x73798 | `(unknown callee) 0x22070` | `0x1` | `0x165450` | `0x147` | `0x0` |
| `bl` at 0x7383c | `(unknown callee) 0x751a8` | `?` | `?` | `?` | `?` |
| `bl` at 0x73864 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x73870 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x738b8 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0x738c4 | `(unknown callee) 0x21ce0` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x738f4 | `(unknown callee) 0x22070` | `0x1` | `0x165450` | `0x153` | `0x0` |
| `bl` at 0x73928 | `(unknown callee) 0xb8924` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7360c | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x73638 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x73674 | `ret(0x558d8) == 0x0` | `ret(0x558d8) != 0x0` |
| `ble` at 0x7369c | `[[sp-40]+8] <= [sp-32] (signed, 32-bit)` | `[[sp-40]+8] > [sp-32] (signed, 32-bit)` |
| `beq` at 0x736ac | `[[sp-40]+12] == 0x0` | `[[sp-40]+12] != 0x0` |
| `beq` at 0x736f4 | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |
| `bne` at 0x737b8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x737e4 | `[[sp-40]+12] == 0x0` | `[[sp-40]+12] != 0x0` |
| `ble` at 0x73800 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x73850 | `ret(0x751a8) == 0x0` | `ret(0x751a8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x73940 | `r0` | `?` |

## `TIOResetHold`

- `TIOResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73960 to 0x73c5c.
  Established. A word store writes 0 to the address formed by [sp-44] plus 0xc (`str` at 0x739f4).
  A later word store writes 0 to the address formed by [sp-44] plus 0x10 (`str` at 0x73adc).
  Another word store writes 0 to absolute address 0x1a6218 (`str` at 0x73ae8).
  The result from unnamed 0x7a50c is later stored as a word at entry sp minus 0x20 (`bl` at 0x73c0c, `str` at 0x73c14).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73970 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x73974 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x7397c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x73980 | load | 4 | `abs 0x73c4c` | `-` |
| `ldr` at 0x73984 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x73990 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x739a4 | load | 4 | `abs 0x73c50` | `-` |
| `ldr` at 0x739a8 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x739b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x739bc | load | 4 | `abs 0x73c54` | `-` |
| `str` at 0x739c0 | store | 4 | `sp -0x38` | `0xfffff768` |
| `ldr` at 0x739c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x739dc | load | 4 | `abs 0x73c50` | `-` |
| `ldr` at 0x739e0 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x739e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x739ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x739f4 | store | 4 | `[sp-44] +0xc` | `0x0` |
| `str` at 0x739fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x73a00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73a04 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x73a08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73a14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73a18 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73a20 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldrb` at 0x73a28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73a34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73a38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73a4c | load | 4 | `[sp-44] +0x0` | `-` |
| `str` at 0x73a54 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x73a58 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x73a5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73a64 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0x73a6c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x73a70 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x73a74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73a78 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73a7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73a8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x73a90 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x73aa0 | store | 4 | `sp -0x20` | `ret(0x775d0)` |
| `ldr` at 0x73aa4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73abc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73ac0 | store | 4 | `sp -0x1c` | `ret(0x775d0)` |
| `ldr` at 0x73ac4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73ac8 | load | 1 | `[sp-48] +0x15` | `-` |
| `ldr` at 0x73ad4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x73adc | store | 4 | `[sp-44] +0x10` | `0x0` |
| `ldr` at 0x73ae0 | load | 4 | `abs 0x73c58` | `-` |
| `str` at 0x73ae8 | store | 4 | `abs 0x1a6218` | `0x0` |
| `ldr` at 0x73aec | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x73af0 | load | 1 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x73af4 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73af8 | load | 1 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x73afc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73b00 | load | 1 | `[sp-48] +0x15` | `-` |
| `str` at 0x73b14 | store | 4 | `sp -0x20` | `ret(0x1259bc)` |
| `ldr` at 0x73b18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b24 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73b28 | store | 4 | `sp -0x1c` | `ret(0x1259bc)` |
| `ldr` at 0x73b2c | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x73b30 | load | 1 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x73b34 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73b38 | load | 1 | `[sp-48] +0x16` | `-` |
| `ldr` at 0x73b3c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73b40 | load | 1 | `[sp-48] +0x17` | `-` |
| `str` at 0x73b54 | store | 4 | `sp -0x20` | `ret(0x125a0c)` |
| `ldr` at 0x73b58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b64 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73b68 | store | 4 | `sp -0x1c` | `ret(0x125a0c)` |
| `ldr` at 0x73b6c | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73b70 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x73b74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73b8c | store | 4 | `sp -0x20` | `ret(0x125a58)` |
| `ldr` at 0x73b90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73b9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73ba8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73bac | store | 4 | `sp -0x1c` | `ret(0x125a58)` |
| `ldr` at 0x73bb0 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x73bb4 | load | 1 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x73bb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73bcc | store | 4 | `sp -0x20` | `ret(0x125a0c)` |
| `ldr` at 0x73bd0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73be8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73bec | store | 4 | `sp -0x1c` | `ret(0x125a0c)` |
| `ldr` at 0x73bf0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x73bf8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x73c00 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73c14 | store | 4 | `sp -0x20` | `ret(0x7a50c)` |
| `ldr` at 0x73c18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73c24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73c30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73c34 | store | 4 | `sp -0x1c` | `ret(0x7a50c)` |
| `ldr` at 0x73c38 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73c3c | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x73c40 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x73c48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x73c48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x73c48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x73a88 | `(unknown callee) 0xb8afc` | `?` | `?` | `?` | `?` |
| `bl` at 0x73a98 | `(unknown callee) 0x775d0` | `[sp-48]` | `[sp-52]` | `0x0` | `?` |
| `bl` at 0x73b0c | `(unknown callee) 0x1259bc` | `[[sp-52]]:1` | `[[sp-48]+20]:1` | `[[sp-48]+21]:1` | `[[sp-48]+21]:1` |
| `bl` at 0x73b4c | `(unknown callee) 0x125a0c` | `[[sp-52]]:1` | `[[sp-48]+22]:1` | `[[sp-48]+23]:1` | `[[sp-48]+23]:1` |
| `bl` at 0x73b84 | `(unknown callee) 0x125a58` | `[sp-20]` | `[[sp-48]]:1` | `0x60` | `0x0` |
| `bl` at 0x73bc4 | `(unknown callee) 0x125a0c` | `[sp-20]` | `[[sp-48]]:1` | `0x4` | `[[sp-48]]:1` |
| `bl` at 0x73c0c | `(unknown callee) 0x7a50c` | `[sp-20]` | `0xff` | `0xff` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7398c | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x739b8 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x73a10 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x73a30 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x73aac | `ret(0x775d0) == 0x0` | `ret(0x775d0) != 0x0` |
| `bne` at 0x73ab8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x73ad0 | `[[sp-48]+21]:1 == 0x0` | `[[sp-48]+21]:1 != 0x0` |
| `beq` at 0x73b20 | `ret(0x1259bc) == 0x0` | `ret(0x1259bc) != 0x0` |
| `beq` at 0x73b60 | `ret(0x125a0c) == 0x0` | `ret(0x125a0c) != 0x0` |
| `beq` at 0x73b98 | `ret(0x125a58) == 0x0` | `ret(0x125a58) != 0x0` |
| `bne` at 0x73ba4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x73bd8 | `ret(0x125a0c) == 0x0` | `ret(0x125a0c) != 0x0` |
| `bne` at 0x73be4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x73c20 | `ret(0x7a50c) == 0x0` | `ret(0x7a50c) != 0x0` |
| `bne` at 0x73c2c | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x73c48 | `r0` | `?` |
