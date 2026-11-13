# ARM Linux 2.6 regulator and MIC74 controls in 1.44.0

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
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-regulator-mic74-controls-facts.txt) retain all tool sections.

## `monitorPSMIHiwaterReset`

- `monitorPSMIHiwaterReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11ea34 to 0x11ec7c.
  Established. A byte store initializes sp-0x21 to 0x53, and a later store increments that field modulo 256 (`strb` at 0x11eb44, `strb` at 0x11ec44).
  The call to unnamed 0x11f424 has candidate values the words at sp-0x18 and sp-0x1c, unknown and zero (`bl` at 0x11eb68).
  Both recorded result paths store zero at sp-0x3c, and the recorded exit returns zero (`str` at 0x11eb38, `str` at 0x11ec5c, `ldm` at 0x11ec68).
  The modified physical state behind the routine's reset name remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11ea44 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11ea48 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11ea4c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x11ea50 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11ea64 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x11ea68 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x11ea78 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x11ea7c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11eaa4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x11eaa8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x11eaac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11eab0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11eabc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11eac0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11eac4 | load | 4 | `abs 0x11ec6c` | `-` |
| `ldr` at 0x11eacc | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x11ead4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11eae4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11eaec | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0x11eaf4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11eaf8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11eb08 | store | 4 | `sp -0x28` | `ret(0x11f61c)` |
| `ldr` at 0x11eb0c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11eb18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11eb24 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11eb28 | load | 4 | `abs 0x11ec70` | `-` |
| `str` at 0x11eb38 | store | 4 | `sp -0x3c` | `0x0` |
| `strb` at 0x11eb44 | store | 1 | `sp -0x21` | `0x53` |
| `ldrb` at 0x11eb48 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x11eb54 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11eb58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11eb5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11eb70 | store | 4 | `sp -0x28` | `ret(0x11f424)` |
| `ldr` at 0x11eb74 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11eb80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11eb94 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x11eb98 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x11eba8 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x11ebac | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11ebd4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x11ebd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11ebdc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x11ebe0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11ebec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x11ebf8 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x11ebfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ec00 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x11ec04 | store | 4 | `(unknown)` | `[[sp-28]+8]` |
| `str` at 0x11ec08 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11ec0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ec10 | load | 4 | `abs 0x11ec74` | `-` |
| `ldr` at 0x11ec20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ec30 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11ec38 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldrb` at 0x11ec3c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x11ec44 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x11ec4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ec50 | load | 4 | `abs 0x11ec78` | `-` |
| `str` at 0x11ec5c | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x11ec60 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0x11ec68 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11ec68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11ec68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11ec68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11ea60 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11ea6c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11eab4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x11ead0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18f0c4` | `ret(0x203f4)` | `[[sp-28]+8]` |
| `bl` at 0x11ead8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11eb00 | `(unknown callee) 0x11f61c` | `[sp-24]` | `[sp-28]` | `0x0` | `sp-42` |
| `bl` at 0x11eb1c | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `ret(0x11f61c)` |
| `bl` at 0x11eb30 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18f0d0` | `ret(0x219a4)` | `ret(0x219a4)` |
| `bl` at 0x11eb68 | `(unknown callee) 0x11f424` | `[sp-24]` | `[sp-28]` | `?` | `0x0` |
| `bl` at 0x11eb90 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11eb9c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11ebe4 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x11ebf0 | `(unknown callee) 0x219a4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x11ec1c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x18f100` | `[sp-33]:1` | `ret(0x203f4)` |
| `bl` at 0x11ec24 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x11ec54 | `(unknown callee) 0x20634` | `[sp-32]` | `0x18f130` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x11eb14 | `Z == 0` | `not (Z == 0)` |
| `bhi` at 0x11eb50 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x11eb7c | `ret(0x11f424) == 0x0` | `ret(0x11f424) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11ec68 | `r0` | `0x0` |

## `monitorPSMIUpdateEEPROM`

- `monitorPSMIUpdateEEPROM`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11ec7c to 0x11f2dc.
  Established. The byte at entry r1 plus two is loaded and reduced by 16 into the word at sp-0x20 (`ldrb` at 0x11ec9c, `str` at 0x11eca4).
  A halfword at sp-0xc6 is loaded, while its subsequent transformed stack value is unknown (`ldrh` at 0x11ee24, `str` at 0x11ee34).
  Two byte stores have unknown effective addresses, with recorded values zero and the low byte of a result of unnamed 0x52800 (`strb` at 0x11f1ac, `strb` at 0x11f1d0).
  The call to unnamed 0x120688 has candidate values the word at sp-0x14, unknown, unknown and the word at sp-0x74, so an EEPROM destination or write contract is not established (`bl` at 0x11f204).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11ec8c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x11ec90 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x11ec94 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x11ec98 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x11ec9c | load | 1 | `r1 +0x2` | `-` |
| `str` at 0x11eca4 | store | 4 | `sp -0x20` | `[r1+2]:1-16` |
| `str` at 0x11ecac | store | 4 | `sp -0x28` | `?` |
| `str` at 0x11ecb4 | store | 4 | `sp -0xbc` | `0x0` |
| `str` at 0x11ecbc | store | 4 | `sp -0xc0` | `0x0` |
| `str` at 0x11ecc4 | store | 4 | `sp -0xc4` | `0x0` |
| `ldr` at 0x11eccc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11ecdc | store | 4 | `sp -0x24` | `ret(0x1202e8)` |
| `str` at 0x11ece4 | store | 4 | `sp -0xcc` | `0x0` |
| `ldr` at 0x11ece8 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x11ecec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11ecf8 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x11ed0c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11ed14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x11ed18 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x11ed1c | load | 1 | `[sp-24] +0x1` | `-` |
| `ldr` at 0x11ed28 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x11ed3c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x11ed44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x11ed48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11ed54 | load | 4 | `sp -0xcc` | `-` |
| `str` at 0x11ed58 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x11ed5c | load | 4 | `sp -0xcc` | `-` |
| `str` at 0x11ed64 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x11ed6c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11ed78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ed7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ed80 | load | 4 | `abs 0x11f2a0` | `-` |
| `ldr` at 0x11ed84 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0x11ed90 | store | 4 | `sp -0xd4` | `0x0` |
| `ldr` at 0x11ed98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11eda4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11edb0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x11edbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11edc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11edc4 | load | 4 | `abs 0x11f2a4` | `-` |
| `ldr` at 0x11edc8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11edd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11eddc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11edec | store | 4 | `sp -0xd0` | `ret(0x11f61c)` |
| `ldr` at 0x11edf0 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x11edfc | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x11ee08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ee0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ee10 | load | 4 | `abs 0x11f2a8` | `-` |
| `ldr` at 0x11ee14 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0x11ee24 | load | 2 | `sp -0xc6` | `-` |
| `str` at 0x11ee34 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x11ee38 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11ee3c | load | 4 | `abs 0x11f2ac` | `-` |
| `ldr` at 0x11ee54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ee58 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ee5c | load | 4 | `abs 0x11f2b0` | `-` |
| `ldr` at 0x11ee60 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11ee64 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11ee70 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x11ee88 | store | 4 | `sp -0xb0` | `ret(0x20544)` |
| `ldr` at 0x11ee8c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x11ee9c | store | 4 | `sp -0xb4` | `ret(0x20544)` |
| `ldr` at 0x11eea0 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11eeac | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11eeb8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11eec8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11eed4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11eee0 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11eeec | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11eefc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ef00 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x11ef04 | store | 4 | `sp -0xdc` | `[sp-76]` |
| `ldr` at 0x11ef08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ef0c | load | 4 | `abs 0x11f2b4` | `-` |
| `ldr` at 0x11ef10 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11ef14 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11ef20 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11ef2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ef30 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x11ef34 | store | 4 | `sp -0xdc` | `[sp-76]` |
| `ldr` at 0x11ef38 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ef3c | load | 4 | `abs 0x11f2b8` | `-` |
| `ldr` at 0x11ef40 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11ef44 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11ef50 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11ef5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11ef60 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x11ef64 | store | 4 | `sp -0xdc` | `[sp-76]` |
| `ldr` at 0x11ef68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ef6c | load | 4 | `abs 0x11f2bc` | `-` |
| `ldr` at 0x11ef70 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11ef74 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11ef7c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11ef88 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x11ef94 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x11efa0 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x11efb4 | store | 4 | `sp -0xc4` | `0x1` |
| `ldr` at 0x11efb8 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11efc4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11efd8 | store | 4 | `sp -0xc0` | `0x1` |
| `ldr` at 0x11efe0 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x11eff0 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x11eff8 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11f004 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f008 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f00c | load | 4 | `abs 0x11f2c0` | `-` |
| `ldr` at 0x11f010 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f014 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11f020 | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11f02c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f030 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f034 | load | 4 | `abs 0x11f2c4` | `-` |
| `ldr` at 0x11f038 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f03c | load | 4 | `sp -0xb8` | `-` |
| `ldr` at 0x11f048 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x11f054 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x11f060 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f064 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x11f068 | store | 4 | `sp -0xdc` | `[sp-76]` |
| `ldrh` at 0x11f06c | load | 2 | `sp -0xc6` | `-` |
| `str` at 0x11f070 | store | 4 | `sp -0xd8` | `[sp-198]:2` |
| `ldr` at 0x11f074 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f078 | load | 4 | `abs 0x11f2c8` | `-` |
| `ldr` at 0x11f07c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f080 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x11f08c | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x11f098 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x11f0a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f0a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f0ac | load | 4 | `abs 0x11f2cc` | `-` |
| `ldr` at 0x11f0b0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f0bc | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x11f0c8 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x11f0d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f0d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f0dc | load | 4 | `abs 0x11f2d0` | `-` |
| `ldr` at 0x11f0e0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f0f0 | load | 4 | `sp -0x74` | `-` |
| `ldrb` at 0x11f0f8 | load | 1 | `[sp-116] +0x3` | `-` |
| `str` at 0x11f104 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x11f108 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f10c | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x11f114 | load | 1 | `([sp-116] + [sp-168]) +0x0` | `-` |
| `str` at 0x11f11c | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x11f120 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x11f124 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x11f130 | store | 4 | `sp -0xa8` | `([sp-172] + [sp-168])+1` |
| `ldr` at 0x11f134 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f138 | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x11f140 | load | 1 | `([sp-116] + ([sp-172] + [sp-168])+1) +0x0` | `-` |
| `str` at 0x11f148 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x11f14c | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x11f150 | load | 4 | `sp -0xa8` | `-` |
| `str` at 0x11f15c | store | 4 | `sp -0xa8` | `([sp-172] + ([sp-172] + [sp-168])+1)+1` |
| `ldr` at 0x11f160 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f164 | load | 4 | `sp -0xa8` | `-` |
| `ldrb` at 0x11f16c | load | 1 | `([sp-116] + ([sp-172] + ([sp-172] + [sp-168])+1)+1) +0x0` | `-` |
| `str` at 0x11f174 | store | 4 | `sp -0xac` | `?` |
| `ldr` at 0x11f17c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f180 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x11f190 | store | 1 | `([sp-116] + ([sp-172] + ([sp-172] + [sp-168])+1)+1) +0x1` | `0x41` |
| `ldr` at 0x11f198 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f19c | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x11f1ac | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x11f1b0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f1b4 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x11f1c0 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f1c4 | load | 4 | `sp -0x78` | `-` |
| `strb` at 0x11f1d0 | store | 1 | `(unknown)` | `(ret(0x52800) & 0xff)` |
| `ldr` at 0x11f1d4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x11f1dc | load | 1 | `sp -0x98` | `-` |
| `ldr` at 0x11f1ec | load | 4 | `sp -0x78` | `-` |
| `str` at 0x11f1f8 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x11f1fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11f200 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x11f20c | store | 4 | `sp -0xd0` | `ret(0x120688)` |
| `ldr` at 0x11f210 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x11f21c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x11f228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f22c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f230 | load | 4 | `abs 0x11f2d4` | `-` |
| `ldr` at 0x11f234 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f244 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f248 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x11f24c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x11f258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11f25c | load | 4 | `abs 0x11f2d8` | `-` |
| `ldr` at 0x11f260 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x11f268 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11f270 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11f290 | store | 4 | `sp -0xd4` | `0x0` |
| `ldr` at 0x11f294 | load | 4 | `sp -0xd4` | `-` |
| `ldm` at 0x11f29c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11f29c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11f29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11ecd4 | `(unknown callee) 0x1202e8` | `[sp-20]` | `sp-44` | `r2` | `sp-44` |
| `bl` at 0x11ed88 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f14c` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x11edac | `(unknown callee) 0x11ff70` | `[sp-20]` | `?` | `sp-164` | `?` |
| `bl` at 0x11edcc | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f180` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x11ede4 | `(unknown callee) 0x11f61c` | `[sp-20]` | `[sp-24]` | `0x41` | `sp-198` |
| `bl` at 0x11ee00 | `(unknown callee) 0x219a4` | `ret(0x11f61c)` | `?` | `?` | `ret(0x11f61c)` |
| `bl` at 0x11ee1c | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f1b4` | `[[sp-24]+8]` | `ret(0x219a4)` |
| `bl` at 0x11ee44 | `(unknown callee) 0x20340` | `[sp-80]` | `0x18f1ec` | `0x9` | `?` |
| `bl` at 0x11ee68 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f1f8` | `[[sp-24]+8]` | `[sp-80]` |
| `bl` at 0x11ee84 | `(unknown callee) 0x20544` | `[sp-80]+9` | `0x0` | `0xa` | `[sp-80]+9` |
| `bl` at 0x11ee98 | `(unknown callee) 0x20544` | `[sp-76]` | `0x0` | `0x10` | `?` |
| `bl` at 0x11ef18 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f22c` | `[[sp-24]+8]` | `[sp-80]` |
| `bl` at 0x11ef48 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f27c` | `[[sp-24]+8]` | `[sp-80]` |
| `bl` at 0x11ef78 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f2b8` | `[[sp-24]+8]` | `[sp-80]` |
| `bl` at 0x11f018 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f2f8` | `[[sp-24]+8]` | `[sp-184]` |
| `bl` at 0x11f040 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f334` | `[[sp-24]+8]` | `[sp-184]` |
| `bl` at 0x11f084 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f380` | `[[sp-24]+8]` | `[sp-80]` |
| `bl` at 0x11f0b4 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f3dc` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x11f0e4 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f428` | `[[sp-24]+8]` | `[sp-24]` |
| `bl` at 0x11f1b8 | `(unknown callee) 0x52800` | `[sp-116]` | `[sp-120]` | `?` | `0x0` |
| `bl` at 0x11f204 | `(unknown callee) 0x120688` | `[sp-20]` | `?` | `?` | `[sp-116]` |
| `bl` at 0x11f220 | `(unknown callee) 0x219a4` | `ret(0x120688)` | `?` | `?` | `ret(0x120688)` |
| `bl` at 0x11f23c | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f458` | `[[sp-24]+8]` | `ret(0x219a4)` |
| `bl` at 0x11f264 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x18f48c` | `[[sp-24]+8]` | `([sp-116] + [sp-168])+1` |
| `bl` at 0x11f27c | `(unknown callee) 0x11ff70` | `[sp-20]` | `?` | `0x0` | `?` |
| `bl` at 0x11f288 | `(unknown callee) 0x120154` | `sp-164` | `?` | `?` | `sp-164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x11ecf4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x11ed24 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x11ed50 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x11ed74 | `[sp-40] >= 0x0 (signed, 32-bit)` | `[sp-40] < 0x0 (signed, 32-bit)` |
| `beq` at 0x11edb8 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `beq` at 0x11edf8 | `ret(0x11f61c) == 0x0` | `ret(0x11f61c) != 0x0` |
| `beq` at 0x11ee50 | `ret(0x20340) == 0x0` | `ret(0x20340) != 0x0` |
| `bne` at 0x11eea8 | `[sp-176] != 0x3` | `[sp-176] == 0x3` |
| `beq` at 0x11eeb4 | `ret(0x20544) == 0x2` | `ret(0x20544) != 0x2` |
| `bne` at 0x11eec0 | `ret(0x20544) != 0xa2` | `ret(0x20544) == 0xa2` |
| `bne` at 0x11eed0 | `[sp-176] != 0x4` | `[sp-176] == 0x4` |
| `beq` at 0x11eedc | `ret(0x20544) == 0x3` | `ret(0x20544) != 0x3` |
| `beq` at 0x11eee8 | `ret(0x20544) == 0xa3` | `ret(0x20544) != 0xa3` |
| `bne` at 0x11eef4 | `ret(0x20544) != 0x4` | `ret(0x20544) == 0x4` |
| `bgt` at 0x11ef28 | `[sp-176] > 0x2 (signed, 32-bit)` | `[sp-176] <= 0x2 (signed, 32-bit)` |
| `ble` at 0x11ef58 | `[sp-176] <= 0x4 (signed, 32-bit)` | `[sp-176] > 0x4 (signed, 32-bit)` |
| `bne` at 0x11ef84 | `[sp-176] != 0x3` | `[sp-176] == 0x3` |
| `beq` at 0x11ef90 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x11ef9c | `[sp-176] != 0x4` | `[sp-176] == 0x4` |
| `beq` at 0x11efa8 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x11efc0 | `[sp-184] == 0xb` | `[sp-184] != 0xb` |
| `beq` at 0x11efcc | `[sp-184] == 0xc` | `[sp-184] != 0xc` |
| `bne` at 0x11efe8 | `[sp-184] != 0xd` | `[sp-184] == 0xd` |
| `bgt` at 0x11f000 | `[sp-184] > 0xa (signed, 32-bit)` | `[sp-184] <= 0xa (signed, 32-bit)` |
| `ble` at 0x11f028 | `[sp-184] <= 0xd (signed, 32-bit)` | `[sp-184] > 0xd (signed, 32-bit)` |
| `beq` at 0x11f050 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x11f05c | `[sp-196] != 0x0` | `[sp-196] == 0x0` |
| `beq` at 0x11f094 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x11f0a0 | `[sp-196] == 0x0` | `[sp-196] != 0x0` |
| `bne` at 0x11f0c4 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x11f0d0 | `[sp-196] != 0x0` | `[sp-196] == 0x0` |
| `beq` at 0x11f218 | `ret(0x120688) == 0x0` | `ret(0x120688) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11f29c | `r0` | `0x0` |

## `mic74_Reset`

- `mic74_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129378 to 0x129644.
  Established. Bytes at entry r1 plus one and two are copied into sp-0x19 and sp-0x1a (`ldrb` at 0x129394, `strb` at 0x129398, `ldrb` at 0x1293a0, `strb` at 0x1293a4).
  Three calls to unnamed 0x10adc4 have third candidate values four, zero and two (`bl` at 0x1293d4, `bl` at 0x129444, `bl` at 0x12949c).
  Three later calls have third candidates five, one and six (`bl` at 0x1294f4, `bl` at 0x129554, `bl` at 0x1295ac).
  The final call to unnamed 0x10aef0 has candidate values unknown, the byte at sp-0x1a, three and sp-0x1b (`bl` at 0x129608).
  The recorded return is the word at sp-0x20, without establishing which physical reset actions the callees perform (`ldm` at 0x129640).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x129388 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12938c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x129390 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129394 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0x129398 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0x12939c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1293a0 | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0x1293a4 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0x1293a8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1293ac | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1293c0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x1293dc | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x1293e0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1293e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1293f8 | store | 4 | `sp -0x28` | `?` |
| `ldrb` at 0x1293fc | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x129400 | store | 4 | `sp -0x2c` | `[sp-26]:1` |
| `str` at 0x129408 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x12940c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x129410 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x12941c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129420 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x129430 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x129434 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x129438 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x129440 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12944c | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x129450 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12945c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x129468 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12946c | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x129470 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x129474 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x129488 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x12948c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129490 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0x1294a4 | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x1294a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1294b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1294c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1294c4 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x1294c8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1294cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1294e0 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x1294e4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1294e8 | load | 1 | `[sp-24] +0x6` | `-` |
| `str` at 0x1294fc | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x129500 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12950c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x129518 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12951c | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x129520 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x129524 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x129538 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x12953c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129540 | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x12955c | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x129560 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12956c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x129578 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12957c | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x129580 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x129584 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x129598 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x1295b4 | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x1295b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1295c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1295d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1295d4 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x1295d8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1295dc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1295f0 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x129610 | store | 4 | `sp -0x24` | `ret(0x10aef0)` |
| `ldr` at 0x129614 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x129620 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12962c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x129630 | store | 4 | `sp -0x20` | `ret(0x10aef0)` |
| `ldr` at 0x129634 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x129640 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129640 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129640 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1293b4 | `(unknown callee) 0x10a4ec` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0x1293d4 | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x4` | `0x0` |
| `bl` at 0x1293ec | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x129444 | `(unknown callee) 0x10adc4` | `[sp-40]` | `[sp-26]:1` | `0x0` | `?` |
| `bl` at 0x12947c | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x12949c | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0x1294d4 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x1294f4 | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x5` | `[[sp-24]+6]:1` |
| `bl` at 0x12952c | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x129554 | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x1` | `?` |
| `bl` at 0x12958c | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x1295ac | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x6` | `0x0` |
| `bl` at 0x1295e4 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x129608 | `(unknown callee) 0x10aef0` | `?` | `[sp-26]:1` | `0x3` | `sp-27` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x129418 | `[[sp-24]+8] == 0x0` | `[[sp-24]+8] != 0x0` |
| `beq` at 0x129428 | `[[sp-24]+4]:1 == 0x0` | `[[sp-24]+4]:1 != 0x0` |
| `beq` at 0x129458 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x129464 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x1294b0 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x1294bc | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x129508 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x129514 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x129568 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x129574 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x1295c0 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x1295cc | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x12961c | `ret(0x10aef0) == 0x0` | `ret(0x10aef0) != 0x0` |
| `bne` at 0x129628 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129640 | `r0` | `[sp-32]` |

## `mic74_PreReset`

- `mic74_PreReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129644 to 0x12971c.
  Established. The byte loaded at offset five of the pointer from sp-0x18 becomes the fourth candidate at the first call to unnamed 0x10adc4 (`ldrb` at 0x129694, `bl` at 0x1296a0).
  The second such call has candidate values unknown, the byte at sp-0x1a, one and unknown (`bl` at 0x1296e0);
  its result replaces sp-0x20 only when that result is nonzero and the existing word is zero, and the exit returns that stack field (`beq` at 0x1296f4, `bne` at 0x129700, `str` at 0x129708, `ldm` at 0x129718).
  The pre-reset hardware sequence remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x129654 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x129658 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x12965c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129660 | load | 1 | `r1 +0x1` | `-` |
| `strb` at 0x129664 | store | 1 | `sp -0x19` | `([r1+1]:1 & 0xff)` |
| `ldr` at 0x129668 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12966c | load | 1 | `r1 +0x2` | `-` |
| `strb` at 0x129670 | store | 1 | `sp -0x1a` | `([r1+2]:1 & 0xff)` |
| `ldrb` at 0x129674 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x129678 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x12968c | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x129690 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129694 | load | 1 | `[sp-24] +0x5` | `-` |
| `str` at 0x1296a8 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldrb` at 0x1296ac | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x1296b0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x1296c4 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0x1296c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1296cc | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x1296e8 | store | 4 | `sp -0x24` | `ret(0x10adc4)` |
| `ldr` at 0x1296ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1296f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x129704 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x129708 | store | 4 | `sp -0x20` | `ret(0x10adc4)` |
| `ldr` at 0x12970c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x129718 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129718 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129718 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x129680 | `(unknown callee) 0x10a4ec` | `r0` | `([r1+1]:1 & 0xff)` | `r2` | `([r1+1]:1 & 0xff)` |
| `bl` at 0x1296a0 | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x2` | `[[sp-24]+5]:1` |
| `bl` at 0x1296b8 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[sp-25]:1` | `?` | `[sp-25]:1` |
| `bl` at 0x1296e0 | `(unknown callee) 0x10adc4` | `?` | `[sp-26]:1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1296f4 | `ret(0x10adc4) == 0x0` | `ret(0x10adc4) != 0x0` |
| `bne` at 0x129700 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129718 | `r0` | `[sp-32]` |

## `mic74_IntSet`

- `mic74_IntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12971c to 0x129788.
  Established. The low byte of entry r2 is stored at sp-0x19 (`strb` at 0x129738).
  The second call, to unnamed 0x10adc4, has candidate values unknown, the byte at offset two of the pointer from sp-0x18, four and the byte at sp-0x19 (`bl` at 0x129774);
  its result is the recorded return, but those candidates do not identify an interrupt register or its bit meanings (`ldm` at 0x129784).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12972c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x129730 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x129738 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x12973c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x129740 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0x129744 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x129758 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12975c | load | 1 | `[sp-24] +0x2` | `-` |
| `ldrb` at 0x129760 | load | 1 | `sp -0x19` | `-` |
| `ldm` at 0x129784 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129784 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129784 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12974c | `(unknown callee) 0x10a4ec` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0x129774 | `(unknown callee) 0x10adc4` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129784 | `r0` | `ret(0x10adc4)` |

## `mic74_IntGet`

- `mic74_IntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x129788 to 0x1297ec.
  Established. Entry r2 is saved as a word at sp-0x1c (`str` at 0x1297a0).
  The call to unnamed 0x10aef0 has candidate values unknown, the byte at offset two of the pointer from sp-0x18, four and the word at sp-0x1c (`bl` at 0x1297d8).
  The exit carries the call result, leaving any output-buffer contents and physical interrupt source unknown (`ldm` at 0x1297e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x129798 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12979c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1297a0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1297a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1297a8 | load | 1 | `r1 +0x1` | `-` |
| `ldr` at 0x1297ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1297c0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1297c4 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x1297d4 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x1297e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1297e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1297e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1297b4 | `(unknown callee) 0x10a4ec` | `r0` | `[r1+1]:1` | `r2` | `[r1+1]:1` |
| `bl` at 0x1297d8 | `(unknown callee) 0x10aef0` | `?` | `[[sp-24]+2]:1` | `0x4` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1297e8 | `r0` | `ret(0x10aef0)` |

## `mic74_Int`

- `mic74_Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1297ec to 0x129a64.
  Established. The call to unnamed 0x10aef0 has candidate values unknown, a byte at offset two of the pointer from sp-0x1c, three and sp-0x22 (`bl` at 0x129860);
  its zero-result branch targets 0x12992c, while the nonzero path continues into the following calls (`beq` at 0x129874).
  A word store at sp-0x28 adds its existing loaded value to a result of unnamed 0x1261c4 (`str` at 0x129974).
  The recorded return is that stack field, and the byte accesses alone do not establish a physical interrupt protocol (`ldm` at 0x129a54).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1297fc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x129800 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x129804 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0x129808 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `str` at 0x129810 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x129818 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x12981c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x129828 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12982c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x129830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x129844 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x129848 | load | 1 | `[sp-28] +0x2` | `-` |
| `str` at 0x129868 | store | 4 | `sp -0x2c` | `ret(0x10aef0)` |
| `ldr` at 0x12986c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x129878 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12988c | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x129890 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1298a0 | store | 4 | `sp -0x38` | `ret(0x2031c)+1` |
| `ldr` at 0x1298a4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1298cc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x1298d0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1298d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1298d8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x1298e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1298f0 | load | 4 | `abs 0x129a58` | `-` |
| `str` at 0x1298f4 | store | 4 | `(unknown)` | `0x1901f4` |
| `str` at 0x1298f8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x1298fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x129900 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x129904 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0x129908 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x129910 | load | 4 | `abs 0x129a5c` | `-` |
| `ldr` at 0x129920 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x129924 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x12992c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x129930 | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0x129934 | load | 1 | `sp -0x22` | `-` |
| `ldrb` at 0x129948 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x12994c | load | 1 | `sp -0x22` | `-` |
| `str` at 0x129950 | store | 4 | `sp -0x54` | `[sp-34]:1` |
| `ldr` at 0x129954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x129958 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12995c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12996c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x129974 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1261c4))` |
| `ldr` at 0x12997c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x129980 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x129988 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x129990 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x129998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1299ac | store | 4 | `sp -0x3c` | `ret(0x6e1b0)` |
| `ldr` at 0x1299b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1299c0 | store | 4 | `sp -0x38` | `ret(0x2031c)+1` |
| `ldr` at 0x1299c4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x1299ec | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1299f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1299f4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1299f8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x129a04 | load | 4 | `abs 0x129a60` | `-` |
| `str` at 0x129a08 | store | 4 | `(unknown)` | `0x190238` |
| `str` at 0x129a0c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x129a10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x129a14 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x129a18 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0x129a1c | load | 1 | `sp -0x22` | `-` |
| `str` at 0x129a20 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0x129a24 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x129a28 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0x129a2c | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0x129a34 | load | 4 | `abs 0x129a5c` | `-` |
| `ldr` at 0x129a44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x129a48 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x129a4c | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x129a54 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x129a54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x129a54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x129a54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x129838 | `(unknown callee) 0x10a4ec` | `?` | `?` | `?` | `?` |
| `bl` at 0x129860 | `(unknown callee) 0x10aef0` | `?` | `[[sp-28]+2]:1` | `0x3` | `sp-34` |
| `bl` at 0x129888 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x129894 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1298dc | `(unknown callee) 0x203f4` | `[sp-60]` | `[sp-52]` | `[sp-56]` | `?` |
| `bl` at 0x1298e8 | `(unknown callee) 0x21ce0` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x12991c | `(unknown callee) 0x22070` | `0x3` | `0x19021c` | `0x87` | `0x0` |
| `bl` at 0x129964 | `(unknown callee) 0x1261c4` | `[sp-24]` | `[sp-28]` | `[sp-32]` | `[sp-33]:1` |
| `bl` at 0x1299a8 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1299b4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1299fc | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-60]` | `[sp-56]` | `?` |
| `bl` at 0x129a40 | `(unknown callee) 0x22070` | `0x3` | `0x19021c` | `0x92` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x129824 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x129874 | `ret(0x10aef0) == 0x0` | `ret(0x10aef0) != 0x0` |
| `beq` at 0x129944 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x129a54 | `r0` | `[sp-40]` |

## `vrmInit`

- `vrmInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e08c to 0x7e2b8.
  Established. Word stores put results of unnamed 0x6e140 and 0x204d8 at absolute addresses 0x1a6250 and 0x1a624c (`str` at 0x7e11c, `str` at 0x7e150).
  The object referenced by sp-0x24 receives a byte at offset eight from the low byte of the word at sp-0x20, then a result of unnamed 0x204d8 at offset four (`strb` at 0x7e1bc, `str` at 0x7e1dc).
  The final call to unnamed 0x20550 has candidate values the object's word at offset four, one, its byte at offset eight and the object pointer (`bl` at 0x7e294).
  These are software fields whose regulator-hardware relationship remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e09c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7e0a0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x7e0a4 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x7e0a8 | load | 4 | `abs 0x7e2a0` | `-` |
| `ldr` at 0x7e0ac | load | 4 | `abs 0x1a624c` | `-` |
| `ldr` at 0x7e0c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7e0cc | load | 4 | `abs 0x7e2a4` | `-` |
| `str` at 0x7e0d0 | store | 4 | `sp -0x3c` | `0x16648c` |
| `ldr` at 0x7e0d4 | load | 4 | `abs 0x7e2a0` | `-` |
| `ldr` at 0x7e0d8 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e0dc | store | 4 | `sp -0x38` | `[0x1a624c]` |
| `ldr` at 0x7e0e0 | load | 4 | `abs 0x7e2a8` | `-` |
| `ldr` at 0x7e0e4 | load | 4 | `abs 0x1a6250` | `-` |
| `str` at 0x7e0e8 | store | 4 | `sp -0x34` | `[0x1a6250]` |
| `ldr` at 0x7e0f0 | load | 4 | `abs 0x7e2ac` | `-` |
| `ldr` at 0x7e100 | load | 4 | `abs 0x7e2a8` | `-` |
| `ldr` at 0x7e104 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e118 | load | 4 | `abs 0x7e2a8` | `-` |
| `str` at 0x7e11c | store | 4 | `abs 0x1a6250` | `ret(0x6e140)` |
| `ldr` at 0x7e120 | load | 4 | `abs 0x7e2a8` | `-` |
| `ldr` at 0x7e124 | load | 4 | `abs 0x1a6250` | `-` |
| `str` at 0x7e138 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e13c | load | 4 | `abs 0x7e2a0` | `-` |
| `ldr` at 0x7e140 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7e150 | store | 4 | `abs 0x1a624c` | `ret(0x204d8)` |
| `ldr` at 0x7e154 | load | 4 | `abs 0x1a624c` | `-` |
| `ldr` at 0x7e160 | load | 4 | `abs 0x7e2b0` | `-` |
| `str` at 0x7e164 | store | 4 | `sp -0x3c` | `0x1664d0` |
| `ldr` at 0x7e168 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7e16c | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0x7e174 | load | 4 | `abs 0x7e2ac` | `-` |
| `ldr` at 0x7e184 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7e198 | load | 4 | `abs 0x7e2a0` | `-` |
| `ldr` at 0x7e19c | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e1a4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7e1a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e1ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7e1b0 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x7e1b4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e1b8 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x7e1bc | store | 1 | `[sp-36] +0x8` | `([sp-32] & 0xff)` |
| `ldr` at 0x7e1c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e1c4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e1c8 | load | 1 | `[sp-36] +0x8` | `-` |
| `str` at 0x7e1dc | store | 4 | `[sp-36] +0x4` | `ret(0x204d8)` |
| `ldr` at 0x7e1e0 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x7e1ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7e200 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x7e204 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x7e214 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x7e218 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7e240 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x7e244 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7e248 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7e24c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7e258 | load | 4 | `abs 0x7e2b4` | `-` |
| `str` at 0x7e25c | store | 4 | `(unknown)` | `0x166508` |
| `str` at 0x7e260 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x7e264 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7e268 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x7e270 | load | 4 | `abs 0x7e2ac` | `-` |
| `ldr` at 0x7e280 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e284 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e288 | load | 1 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x7e28c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldm` at 0x7e29c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7e29c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e29c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e29c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e0b8 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a624c]` |
| `bl` at 0x7e0fc | `(unknown callee) 0x22070` | `0x9` | `0x1664bc` | `0x48` | `0x0` |
| `bl` at 0x7e110 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a6250]` |
| `bl` at 0x7e148 | `(unknown callee) 0x204d8` | `[sp-40]` | `0x1` | `[0x1a6250]` | `?` |
| `bl` at 0x7e180 | `(unknown callee) 0x22070` | `0x9` | `0x1664bc` | `0x53` | `0x0` |
| `bl` at 0x7e1d4 | `(unknown callee) 0x204d8` | `[[sp-36]+8]:1` | `0x1` | `[sp-36]` | `[[sp-36]+8]:1` |
| `bl` at 0x7e1fc | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7e208 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7e250 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x7e27c | `(unknown callee) 0x22070` | `0x9` | `0x1664bc` | `0x5e` | `0x0` |
| `bl` at 0x7e294 | `(unknown callee) 0x20550` | `[[sp-36]+4]` | `0x1` | `[[sp-36]+8]:1` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e0b4 | `[0x1a624c] == 0x0` | `[0x1a624c] != 0x0` |
| `bne` at 0x7e0c8 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x7e10c | `[0x1a6250] != 0x0` | `[0x1a6250] == 0x0` |
| `bne` at 0x7e15c | `[0x1a624c] != 0x0` | `[0x1a624c] == 0x0` |
| `bne` at 0x7e1e8 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e29c | `r0` | `ret(0x20550)` |

## `vrmRemove`

- `vrmRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e2b8 to 0x7e380.
  Established. A byte at offset eight and words at offsets zero and four of the pointer from sp-0x18 are cleared (`strb` at 0x7e348, `str` at 0x7e354, `str` at 0x7e36c).
  Between the word clears, the call to unnamed 0x20514 has candidate values the object's word at offset four, entry r1 and the object pointer twice (`bl` at 0x7e360).
  The exit value is unknown, and these clears do not establish a hardware removal transaction (`ldm` at 0x7e374).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e2c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7e2cc | load | 4 | `abs 0x7e378` | `-` |
| `ldr` at 0x7e2d0 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e2dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e2f0 | load | 4 | `abs 0x7e37c` | `-` |
| `ldr` at 0x7e2f4 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e2fc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e30c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e320 | load | 4 | `abs 0x7e37c` | `-` |
| `ldr` at 0x7e324 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e32c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x7e330 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7e334 | load | 1 | `[sp-24] +0x8` | `-` |
| `ldr` at 0x7e340 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x7e348 | store | 1 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x7e34c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7e354 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x7e358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7e35c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x7e364 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7e36c | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldm` at 0x7e374 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e374 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e374 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e360 | `(unknown callee) 0x20514` | `[[sp-24]+4]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e2d8 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e304 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7e33c | `[[sp-24]+8]:1 == 0x0` | `[[sp-24]+8]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e374 | `r0` | `?` |

## `vrmGetCfg`

- `vrmGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e678 to 0x7e778.
  Established. One path stores zero at sp-0x20, while another stores 0xfffff9b9 there (`str` at 0x7e6d0, `str` at 0x7e718).
  A word from the pointer at sp-0x1c is copied to the address in entry r1 (`ldr` at 0x7e74c, `str` at 0x7e750).
  That object's byte at offset eight is saved as a word at sp-0x20 (`ldrb` at 0x7e758, `str` at 0x7e75c).
  The recorded return is unknown despite these path-specific stores (`ldm` at 0x7e768).
  The copied word's configuration meaning is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e688 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7e68c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x7e690 | load | 4 | `abs 0x7e76c` | `-` |
| `ldr` at 0x7e694 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e6a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e6b4 | load | 4 | `abs 0x7e770` | `-` |
| `ldr` at 0x7e6b8 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e6c0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x7e6d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7e6d8 | load | 4 | `abs 0x7e76c` | `-` |
| `ldr` at 0x7e6dc | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e6e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e6fc | load | 4 | `abs 0x7e770` | `-` |
| `ldr` at 0x7e700 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e708 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e714 | load | 4 | `abs 0x7e774` | `-` |
| `str` at 0x7e718 | store | 4 | `sp -0x20` | `0xfffff9b9` |
| `ldr` at 0x7e720 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e734 | load | 4 | `abs 0x7e770` | `-` |
| `ldr` at 0x7e738 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e740 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7e744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7e748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e74c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x7e750 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x7e754 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e758 | load | 1 | `[sp-28] +0x8` | `-` |
| `str` at 0x7e75c | store | 4 | `sp -0x20` | `[[sp-28]+8]:1` |
| `ldr` at 0x7e760 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7e768 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e768 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e768 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e69c | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e6c8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x7e6e4 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e710 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e768 | `r0` | `?` |

## `vrmEnable`

- `vrmEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e4dc to 0x7e638.
  Established. Entry r1 is reduced to a byte at sp-0x15 (`strb` at 0x7e4f4).
  An unsigned comparison branches to 0x7e58c when the object's byte at offset eight is at most that input byte, falling through when it is greater (`bls` at 0x7e56c).
  The fall-through path stores one at the input byte plus the object's loaded word at offset four (`strb` at 0x7e584).
  This software byte update does not establish a physical enable operation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e4ec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7e4f4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7e4f8 | load | 4 | `abs 0x7e628` | `-` |
| `ldr` at 0x7e4fc | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e508 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e51c | load | 4 | `abs 0x7e62c` | `-` |
| `ldr` at 0x7e520 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e528 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e54c | load | 4 | `abs 0x7e62c` | `-` |
| `ldr` at 0x7e550 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e558 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7e55c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e560 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x7e564 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e570 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e574 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e578 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x7e584 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x1` |
| `ldr` at 0x7e58c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7e5a0 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x7e5a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7e5b4 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x7e5b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7e5e0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e5e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7e5e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e5ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e5f8 | load | 4 | `abs 0x7e630` | `-` |
| `str` at 0x7e5fc | store | 4 | `(unknown)` | `0x166560` |
| `str` at 0x7e600 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x7e604 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e608 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x7e610 | load | 4 | `abs 0x7e634` | `-` |
| `ldm` at 0x7e624 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e624 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e59c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7e5a8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7e5f0 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x7e61c | `(unknown callee) 0x22070` | `0x6` | `0x1664bc` | `0x8f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e504 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e530 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7e56c | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e624 | `r0` | `?` |

## `vrmDisable`

- `vrmDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e380 to 0x7e4dc.
  Established. The byte stored at sp-0x15 is entry r1 masked to eight bits (`strb` at 0x7e398).
  When the object's byte at offset eight exceeds that input byte, the known fall-through path stores zero at the input byte plus the object's loaded word at offset four (`bls` at 0x7e410, `strb` at 0x7e428).
  The other comparison direction targets 0x7e430, where the call to unnamed 0x6e1b0 has candidate values entry r0, one, zero and zero (`bls` at 0x7e410, `bl` at 0x7e440).
  The cleared software field does not identify a regulator disable register.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e390 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7e398 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7e39c | load | 4 | `abs 0x7e4cc` | `-` |
| `ldr` at 0x7e3a0 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e3ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e3c0 | load | 4 | `abs 0x7e4d0` | `-` |
| `ldr` at 0x7e3c4 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e3cc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e3dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e3f0 | load | 4 | `abs 0x7e4d0` | `-` |
| `ldr` at 0x7e3f4 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e3fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7e400 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e404 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x7e408 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e414 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7e418 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7e41c | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x7e428 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x0` |
| `ldr` at 0x7e430 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7e444 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x7e448 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7e458 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x7e45c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7e484 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e488 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7e48c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e490 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e49c | load | 4 | `abs 0x7e4d4` | `-` |
| `str` at 0x7e4a0 | store | 4 | `(unknown)` | `0x166544` |
| `str` at 0x7e4a4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x7e4a8 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x7e4ac | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x7e4b4 | load | 4 | `abs 0x7e4d8` | `-` |
| `ldm` at 0x7e4c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e4c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e4c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e440 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7e44c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7e494 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x7e4c0 | `(unknown callee) 0x22070` | `0x6` | `0x1664bc` | `0x7f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e3a8 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e3d4 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x7e410 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e4c8 | `r0` | `?` |

## `vrmReset`

- `vrmReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e638 to 0x7e678.
  Established. The first call targets unnamed 0x558d8 with entry r0 through r3 as candidate values (`bl` at 0x7e650).
  A result equal to one falls through to the call to unnamed 0x7e778, while any unequal result branches to 0x7e66c before the call to unnamed 0x7e970 (`bne` at 0x7e65c, `bl` at 0x7e664, `bl` at 0x7e670).
  Both later calls have candidate values the saved entry r0, unknown, unknown and the first result (`bl` at 0x7e664, `bl` at 0x7e670).
  The exit value and physical reset semantics remain unknown (`ldm` at 0x7e674).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e648 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7e64c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e660 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7e66c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7e674 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7e674 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e674 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e650 | `(unknown callee) 0x558d8` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x7e664 | `(unknown callee) 0x7e778` | `[sp-20]` | `?` | `?` | `ret(0x558d8)` |
| `bl` at 0x7e670 | `(unknown callee) 0x7e970` | `[sp-20]` | `?` | `?` | `ret(0x558d8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x7e65c | `ret(0x558d8) != 0x1` | `ret(0x558d8) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e674 | `r0` | `?` |

## `vrmIntEnable`

- `vrmIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e778 to 0x7e970.
  Established. A byte store puts one at offset nine of the pointer from sp-0x28 (`strb` at 0x7e7f8).
  Several following byte loads have unknown effective addresses (`ldrb` at 0x7e828, `ldrb` at 0x7e850, `ldrb` at 0x7e85c, `ldrb` at 0x7e860).
  The call to unnamed 0x125d44 has candidate values the word at sp-0x1c and three unknowns (`bl` at 0x7e878);
  its zero-result branch targets 0x7e948, while a nonzero result continues into the following calls (`beq` at 0x7e88c).
  The object flag and numeric callee do not establish a physical interrupt-enable contract.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e788 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x7e78c | load | 4 | `abs 0x7e960` | `-` |
| `ldr` at 0x7e790 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e79c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e7b0 | load | 4 | `abs 0x7e964` | `-` |
| `ldr` at 0x7e7b4 | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e7bc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e7cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e7e0 | load | 4 | `abs 0x7e964` | `-` |
| `ldr` at 0x7e7e4 | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e7ec | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e7f0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7e7f8 | store | 1 | `[sp-40] +0x9` | `0x1` |
| `str` at 0x7e800 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7e804 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7e808 | load | 1 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x7e80c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e818 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7e81c | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x7e820 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7e828 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e834 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7e838 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7e840 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x7e848 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x7e84c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7e850 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e854 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7e858 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7e85c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7e860 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e86c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7e880 | store | 4 | `sp -0x20` | `ret(0x125d44)` |
| `ldr` at 0x7e884 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e890 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7e8a4 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x7e8a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7e8b8 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x7e8bc | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7e8e4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7e8e8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7e8ec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7e8f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7e8fc | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7e900 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x7e904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e914 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7e920 | load | 4 | `abs 0x7e968` | `-` |
| `str` at 0x7e924 | store | 4 | `(unknown)` | `0x166578` |
| `str` at 0x7e928 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7e92c | store | 4 | `(unknown)` | `ret(0x1265c4)` |
| `str` at 0x7e930 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x7e938 | load | 4 | `abs 0x7e96c` | `-` |
| `ldr` at 0x7e948 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7e950 | store | 4 | `sp -0x24` | `?` |
| `ldm` at 0x7e95c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x7e95c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7e95c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7e95c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7e95c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7e878 | `(unknown callee) 0x125d44` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x7e8a0 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7e8ac | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7e8f4 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x7e90c | `(unknown callee) 0x1265c4` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x7e918 | `(unknown callee) 0x21ce0` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7e944 | `(unknown callee) 0x22070` | `0x3` | `0x1664bc` | `0xce` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e798 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e7c4 | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7e814 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7e830 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7e88c | `ret(0x125d44) == 0x0` | `ret(0x125d44) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7e95c | `r0` | `?` |

## `vrmIntDisable`

- `vrmIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7e970 to 0x7ebc0.
  Established. The byte at offset nine of the pointer from sp-0x28 is cleared (`strb` at 0x7e9f0).
  The call to unnamed 0x125958 has candidate values the word at sp-0x1c, unknown, the object's word at offset four and unknown (`bl` at 0x7ea88).
  A nonzero result branches to 0x7ea9c, while a zero result falls through to a jump to 0x7eb98 (`bne` at 0x7ea94, `b` at 0x7ea98).
  The later call to unnamed 0x125cf8 has candidate values the word at sp-0x1c and three unknowns, so its physical interrupt-disable effects remain unknown (`bl` at 0x7eac8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7e980 | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0x7e984 | load | 4 | `abs 0x7ebb0` | `-` |
| `ldr` at 0x7e988 | load | 4 | `abs 0x1a6250` | `-` |
| `ldr` at 0x7e994 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e9a8 | load | 4 | `abs 0x7ebb4` | `-` |
| `ldr` at 0x7e9ac | load | 4 | `abs 0x1a624c` | `-` |
| `ldrb` at 0x7e9b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7e9c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7e9d8 | load | 4 | `abs 0x7ebb4` | `-` |
| `ldr` at 0x7e9dc | load | 4 | `abs 0x1a624c` | `-` |
| `str` at 0x7e9e4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x7e9e8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x7e9f0 | store | 1 | `[sp-40] +0x9` | `0x0` |
| `str` at 0x7e9f8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7e9fc | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x7ea00 | load | 1 | `[sp-40] +0x8` | `-` |
| `ldr` at 0x7ea04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7ea10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ea14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7ea1c | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x7ea24 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x7ea28 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7ea2c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ea40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ea44 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x7ea48 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7ea50 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ea5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7ea60 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldr` at 0x7ea64 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7ea6c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ea78 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7ea7c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7ea80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7ea9c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7eaa0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7eaa4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7eaa8 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7eaac | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7eab0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7eabc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7ead0 | store | 4 | `sp -0x20` | `ret(0x125cf8)` |
| `ldr` at 0x7ead4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7eae0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7eaf4 | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x7eaf8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x7eb08 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x7eb0c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7eb34 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x7eb38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x7eb3c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x7eb40 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7eb4c | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x7eb50 | load | 1 | `[sp-44] +0x0` | `-` |
| `ldr` at 0x7eb54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7eb64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7eb70 | load | 4 | `abs 0x7ebb8` | `-` |
| `str` at 0x7eb74 | store | 4 | `(unknown)` | `0x1665b0` |
| `str` at 0x7eb78 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x7eb7c | store | 4 | `(unknown)` | `ret(0x1265c4)` |
| `str` at 0x7eb80 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x7eb88 | load | 4 | `abs 0x7ebbc` | `-` |
| `ldr` at 0x7eb98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x7eba0 | store | 4 | `sp -0x24` | `?` |
| `ldm` at 0x7ebac | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x7ebac | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7ebac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7ebac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7ebac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7ea88 | `(unknown callee) 0x125958` | `[sp-28]` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0x7eac8 | `(unknown callee) 0x125cf8` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x7eaf0 | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x7eafc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x7eb44 | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x7eb5c | `(unknown callee) 0x1265c4` | `[sp-28]` | `[[sp-44]]:1` | `?` | `[[sp-44]]:1` |
| `bl` at 0x7eb68 | `(unknown callee) 0x21ce0` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x7eb94 | `(unknown callee) 0x22070` | `0x3` | `0x1664bc` | `0xee` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x7e990 | `[0x1a6250] == 0x0` | `[0x1a6250] != 0x0` |
| `bne` at 0x7e9bc | `? != 0x0` | `? == 0x0` |
| `ble` at 0x7ea0c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x7ea38 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x7ea58 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7ea74 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x7ea94 | `ret(0x125958) != 0x0` | `ret(0x125958) == 0x0` |
| `beq` at 0x7eadc | `ret(0x125cf8) == 0x0` | `ret(0x125cf8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7ebac | `r0` | `?` |
