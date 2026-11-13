# ARM Linux 2.6 EEPROM access paths in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Computed transfers in eepromReadData, eepromFreeData and eepromStatus leave unresolved
edges. Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-eeprom-access-facts.txt) retain all tool sections.

## `eepromInit`

- `eepromInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11f7c8 to 0x11faa0.
  Established. Word assignments store `ret(0x6e140)` at `0x314fac` and `ret(0x204d8)` at `0x314fa8` (`str` at 0x11f840, `str` at 0x11f868).
  Other word assignments write `ret(0x201fc)` to `[sp-44] +0x8` and `ret(0x204d8)` to `[sp-44] +0xc` (`str` at 0x11f8f8, `str` at 0x11f9cc).
  The unnamed `0x20550` call carries `[[sp-44]+8]`, `0x1`, `[[sp-44]+4]` and `[sp-44]` (`bl` at 0x11f9b0).
  EEPROM initialization and physical storage ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11f7d8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x11f7dc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11f7e0 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x11f7e4 | load | 4 | `abs 0x11fa78` | `-` |
| `ldr` at 0x11f7e8 | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11f7fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f808 | load | 4 | `abs 0x11fa7c` | `-` |
| `str` at 0x11f80c | store | 4 | `sp -0x44` | `0x18f63c` |
| `ldr` at 0x11f814 | load | 4 | `abs 0x11fa80` | `-` |
| `ldr` at 0x11f818 | load | 4 | `abs 0x11fa84` | `-` |
| `ldr` at 0x11f824 | load | 4 | `abs 0x11fa88` | `-` |
| `ldr` at 0x11f828 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11f83c | load | 4 | `abs 0x11fa88` | `-` |
| `str` at 0x11f840 | store | 4 | `abs 0x314fac` | `ret(0x6e140)` |
| `ldr` at 0x11f844 | load | 4 | `abs 0x11fa88` | `-` |
| `ldr` at 0x11f848 | load | 4 | `abs 0x314fac` | `-` |
| `str` at 0x11f850 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x11f854 | load | 4 | `abs 0x11fa78` | `-` |
| `ldr` at 0x11f858 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x11f868 | store | 4 | `abs 0x314fa8` | `ret(0x204d8)` |
| `ldr` at 0x11f86c | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11f878 | load | 4 | `abs 0x11fa8c` | `-` |
| `str` at 0x11f87c | store | 4 | `sp -0x44` | `0x18f67c` |
| `ldr` at 0x11f880 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x11f884 | store | 4 | `sp -0x40` | `[sp-40]` |
| `ldr` at 0x11f88c | load | 4 | `abs 0x11fa80` | `-` |
| `ldr` at 0x11f890 | load | 4 | `abs 0x11fa90` | `-` |
| `ldr` at 0x11f89c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f8a4 | load | 4 | `abs 0x11fa78` | `-` |
| `ldr` at 0x11f8a8 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x11f8b0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x11f8b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f8b8 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x11f8c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11f8cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f8d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11f8d4 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0x11f8d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f8dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11f8e0 | store | 4 | `[sp-44] +0x4` | `[sp-32]` |
| `ldr` at 0x11f8e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f8e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f8ec | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0x11f8f8 | store | 4 | `[sp-44] +0x8` | `ret(0x201fc)` |
| `ldr` at 0x11f8fc | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x11f908 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11f91c | store | 4 | `sp -0x30` | `ret(0x6e1b0)` |
| `ldr` at 0x11f920 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x11f930 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x11f934 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11f95c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x11f960 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x11f964 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11f968 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11f974 | load | 4 | `abs 0x11fa94` | `-` |
| `str` at 0x11f978 | store | 4 | `(unknown)` | `0x18f6b4` |
| `str` at 0x11f97c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11f980 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11f984 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x11f98c | load | 4 | `abs 0x11fa80` | `-` |
| `ldr` at 0x11f990 | load | 4 | `abs 0x11fa98` | `-` |
| `ldr` at 0x11f99c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f9a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f9a4 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0x11f9ac | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0x11f9b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f9b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x11f9bc | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0x11f9cc | store | 4 | `[sp-44] +0xc` | `ret(0x204d8)` |
| `ldr` at 0x11f9d0 | load | 4 | `[sp-44] +0xc` | `-` |
| `ldr` at 0x11f9dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11f9f0 | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x11f9f4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x11fa04 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x11fa08 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11fa30 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x11fa34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x11fa38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x11fa3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11fa48 | load | 4 | `abs 0x11fa9c` | `-` |
| `str` at 0x11fa4c | store | 4 | `(unknown)` | `0x18f6f0` |
| `str` at 0x11fa50 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x11fa54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11fa58 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x11fa60 | load | 4 | `abs 0x11fa80` | `-` |
| `ldm` at 0x11fa74 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11fa74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11fa74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11fa74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11f7f4 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x314fa8]` |
| `bl` at 0x11f820 | `(unknown callee) 0x22070` | `0x9` | `0x18f660` | `0x129` | `0x0` |
| `bl` at 0x11f834 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x314fac]` |
| `bl` at 0x11f860 | `(unknown callee) 0x204d8` | `[sp-40]` | `0x1` | `ret(0x6e140)` | `?` |
| `bl` at 0x11f898 | `(unknown callee) 0x22070` | `0x9` | `0x18f660` | `0x132` | `0x0` |
| `bl` at 0x11f8c8 | `(unknown callee) 0x11faa0` | `?` | `?` | `?` | `[[sp-44]+4]` |
| `bl` at 0x11f8f0 | `(unknown callee) 0x201fc` | `[[sp-44]+4]` | `?` | `[sp-44]` | `[sp-44]` |
| `bl` at 0x11f918 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11f924 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11f96c | `(unknown callee) 0x203f4` | `[sp-56]` | `[sp-48]` | `[sp-52]` | `?` |
| `bl` at 0x11f998 | `(unknown callee) 0x22070` | `0x9` | `0x18f660` | `0x147` | `0x0` |
| `bl` at 0x11f9b0 | `(unknown callee) 0x20550` | `[[sp-44]+8]` | `0x1` | `[[sp-44]+4]` | `[sp-44]` |
| `bl` at 0x11f9c4 | `(unknown callee) 0x204d8` | `[[sp-44]+4]` | `0x78` | `?` | `[sp-44]` |
| `bl` at 0x11f9ec | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11f9f8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11fa40 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x11fa6c | `(unknown callee) 0x22070` | `0x9` | `0x18f660` | `0x14c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11f7f0 | `[0x314fa8] == 0x0` | `[0x314fa8] != 0x0` |
| `bne` at 0x11f804 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x11f830 | `[0x314fac] != 0x0` | `[0x314fac] == 0x0` |
| `bne` at 0x11f874 | `[0x314fa8] != 0x0` | `[0x314fa8] == 0x0` |
| `beq` at 0x11f8c0 | `[[sp-44]+4] == 0x0` | `[[sp-44]+4] != 0x0` |
| `bne` at 0x11f904 | `[[sp-44]+8] != 0x0` | `[[sp-44]+8] == 0x0` |
| `bne` at 0x11f9d8 | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11fa74 | `r0` | `?` |

## `eepromRemove`

- `eepromRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11faa0 to 0x11fbc0.
  Established. Separate word assignments clear `[sp-24] +0x4`, `[sp-24] +0x0` and `[sp-24] +0xc` (`str` at 0x11fb38, `str` at 0x11fb44, `str` at 0x11fbac).
  The unnamed `0x120154` call carries unknown, `[sp-24]`, unknown and unknown (`bl` at 0x11fb84).
  The unnamed `0x20514` call carries `[[sp-24]+12]`, unknown, unknown and `[sp-24]` (`bl` at 0x11fba0).
  EEPROM removal and resource ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11fab0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x11fab4 | load | 4 | `abs 0x11fbb8` | `-` |
| `ldr` at 0x11fab8 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fac4 | load | 4 | `abs 0x11fbb8` | `-` |
| `ldr` at 0x11fac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11facc | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fad8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fae0 | load | 4 | `abs 0x11fbbc` | `-` |
| `ldr` at 0x11fae4 | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11faec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11fafc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fb04 | load | 4 | `abs 0x11fbbc` | `-` |
| `ldr` at 0x11fb08 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x11fb10 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x11fb14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11fb18 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0x11fb24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11fb28 | load | 4 | `[sp-24] +0x4` | `-` |
| `str` at 0x11fb2c | store | 4 | `sp -0x20` | `[[sp-24]+4]` |
| `ldr` at 0x11fb30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11fb38 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x11fb3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11fb44 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `str` at 0x11fb4c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x11fb50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fb54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x11fb60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11fb64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fb78 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x11fb88 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11fb90 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x11fb98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x11fb9c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x11fba4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11fbac | store | 4 | `[sp-24] +0xc` | `0x0` |
| `ldm` at 0x11fbb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11fbb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11fbb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11fb84 | `(unknown callee) 0x120154` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0x11fba0 | `(unknown callee) 0x20514` | `[[sp-24]+12]` | `?` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11fac0 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x11fad4 | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x11faf4 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x11fb20 | `[[sp-24]+4] == 0x0` | `[[sp-24]+4] != 0x0` |
| `bge` at 0x11fb5c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11fbb4 | `r0` | `?` |

## `eepromNotPresent`

- `eepromNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11fbc0 to 0x11fca8.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+4]` takes `0x11fc6c`, while lower falls through (`bge` at 0x11fc4c).
  The fall-through path clears the byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`strb` at 0x11fc64).
  The unnamed `0x22070` call carries `0x6`, `0x18f660`, `0x174` and `0x0` (`bl` at 0x11fc8c).
  Presence-state encoding and physical detection remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11fbd0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x11fbd8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x11fbdc | load | 4 | `abs 0x11fc98` | `-` |
| `ldr` at 0x11fbe0 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fbec | load | 4 | `abs 0x11fc98` | `-` |
| `ldr` at 0x11fbf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fbf4 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fc00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fc08 | load | 4 | `abs 0x11fc9c` | `-` |
| `ldr` at 0x11fc0c | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11fc14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11fc24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fc2c | load | 4 | `abs 0x11fc9c` | `-` |
| `ldr` at 0x11fc30 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x11fc38 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x11fc3c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11fc40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fc44 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x11fc50 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11fc54 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11fc58 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0x11fc64 | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0x11fc6c | load | 4 | `abs 0x11fca0` | `-` |
| `str` at 0x11fc70 | store | 4 | `sp -0x24` | `0x18f72c` |
| `ldrb` at 0x11fc74 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x11fc78 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0x11fc80 | load | 4 | `abs 0x11fca4` | `-` |
| `ldm` at 0x11fc94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11fc94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11fc94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11fc8c | `(unknown callee) 0x22070` | `0x6` | `0x18f660` | `0x174` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11fbe8 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x11fbfc | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x11fc1c | `? != 0x0` | `? == 0x0` |
| `bge` at 0x11fc4c | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11fc94 | `r0` | `?` |

## `eepromProbe`

- `eepromProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11fca8 to 0x11fe90.
  Established. One byte assignment writes `0x1` to `((r1 & 0xff) + [[sp-48]+8]) +0x0` (`strb` at 0x11fd48).
  Another clears the byte at `([sp-29]:1 + [[sp-48]+8]) +0x0` (`strb` at 0x11fe64).
  The unnamed `0x120414` call carries `[sp-28]`, `[sp-29]:1`, `0x0` and `sp-44` (`bl` at 0x11fd6c);
  its zero result takes `0x11fe68`, while nonzero falls through (`beq` at 0x11fd80).
  Probe acceptance and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11fcb8 | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0x11fcc0 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `str` at 0x11fcc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x11fccc | load | 4 | `abs 0x11fe7c` | `-` |
| `ldr` at 0x11fcd0 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fcdc | load | 4 | `abs 0x11fe7c` | `-` |
| `ldr` at 0x11fce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fce4 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fcf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fcf8 | load | 4 | `abs 0x11fe80` | `-` |
| `ldr` at 0x11fcfc | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11fd04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11fd10 | load | 4 | `abs 0x11fe84` | `-` |
| `str` at 0x11fd14 | store | 4 | `sp -0x40` | `0xfffffbb4` |
| `ldr` at 0x11fd1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fd24 | load | 4 | `abs 0x11fe80` | `-` |
| `ldr` at 0x11fd28 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x11fd30 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x11fd34 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x11fd38 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11fd3c | load | 4 | `[sp-48] +0x8` | `-` |
| `strb` at 0x11fd48 | store | 1 | `((r1 & 0xff) + [[sp-48]+8]) +0x0` | `0x1` |
| `ldrb` at 0x11fd4c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x11fd58 | store | 4 | `sp -0x50` | `0x8` |
| `ldr` at 0x11fd5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11fd74 | store | 4 | `sp -0x24` | `ret(0x120414)` |
| `ldr` at 0x11fd78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11fd84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x11fd94 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x11fd9c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x11fdb0 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x11fdb4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x11fdc4 | store | 4 | `sp -0x38` | `ret(0x2031c)+1` |
| `ldr` at 0x11fdc8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x11fdf0 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x11fdf4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x11fdf8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x11fdfc | load | 4 | `sp -0x38` | `-` |
| `ldrb` at 0x11fe08 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11fe0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11fe1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11fe28 | load | 4 | `abs 0x11fe88` | `-` |
| `str` at 0x11fe2c | store | 4 | `(unknown)` | `0x18f748` |
| `str` at 0x11fe30 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `str` at 0x11fe34 | store | 4 | `(unknown)` | `ret(0x1208b8)` |
| `str` at 0x11fe38 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x11fe40 | load | 4 | `abs 0x11fe8c` | `-` |
| `ldr` at 0x11fe50 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0x11fe54 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x11fe58 | load | 4 | `[sp-48] +0x8` | `-` |
| `strb` at 0x11fe64 | store | 1 | `([sp-29]:1 + [[sp-48]+8]) +0x0` | `0x0` |
| `ldr` at 0x11fe68 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x11fe6c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x11fe70 | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x11fe78 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x11fe78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x11fe78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11fe78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11fe78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11fd6c | `(unknown callee) 0x120414` | `[sp-28]` | `[sp-29]:1` | `0x0` | `sp-44` |
| `bl` at 0x11fdac | `(unknown callee) 0x6e1b0` | `[sp-28]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x11fdb8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x11fe00 | `(unknown callee) 0x203f4` | `[sp-60]` | `[sp-52]` | `[sp-56]` | `?` |
| `bl` at 0x11fe14 | `(unknown callee) 0x1208b8` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0x11fe20 | `(unknown callee) 0x219a4` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0x11fe4c | `(unknown callee) 0x22070` | `0x1` | `0x18f660` | `0x18c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11fcd8 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x11fcec | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x11fd0c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x11fd80 | `ret(0x120414) == 0x0` | `ret(0x120414) != 0x0` |
| `bne` at 0x11fd8c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11fe78 | `r0` | `?` |

## `eepromReadAllData`

- `eepromReadAllData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11fe90 to 0x11ff70.
  Established. A byte assignment clears `sp -0x15` (`strb` at 0x11ff08).
  A later byte assignment writes `([sp-21]:1+1 & 0xff)` to `sp -0x15` (`strb` at 0x11ff58).
  The unnamed `0x11ff70` call carries `[sp-20]`, unknown, unknown and unknown (`bl` at 0x11ff4c).
  Complete EEPROM data coverage and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11fea0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x11fea4 | load | 4 | `abs 0x11ff68` | `-` |
| `ldr` at 0x11fea8 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11feb4 | load | 4 | `abs 0x11ff68` | `-` |
| `ldr` at 0x11feb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11febc | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11fec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fed0 | load | 4 | `abs 0x11ff6c` | `-` |
| `ldr` at 0x11fed4 | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11fedc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11feec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fef4 | load | 4 | `abs 0x11ff6c` | `-` |
| `ldr` at 0x11fef8 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x11ff00 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0x11ff08 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x11ff0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11ff10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x11ff14 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldrb` at 0x11ff20 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11ff24 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x11ff28 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11ff3c | load | 4 | `[sp-28] +0xc` | `-` |
| `ldr` at 0x11ff44 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x11ff50 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x11ff58 | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldm` at 0x11ff64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11ff64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11ff64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x11ff4c | `(unknown callee) 0x11ff70` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11feb0 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x11fec4 | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x11fee4 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x11ff1c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11ff64 | `r0` | `?` |

## `eepromReadData`

- `eepromReadData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11ff70 to 0x120154.
  Established. A word assignment writes `[[sp-36]+4]:1` to `[sp-28] +0x0` (`str` at 0x120070).
  The unnamed `0x20550` call carries `[sp-28]`, `0x0`, `0x78` and unknown (`bl` at 0x120060).
  A conditional word read has unknown effective address when `[[sp-28]]-1 <= 0x3 (unsigned, 32-bit)` (`ldrls` at 0x1200b8).
  The computed transfer remains unresolved, and omitted data selector effects and physical EEPROM roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11ff80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x11ff88 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x11ff8c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x11ff90 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x11ff9c | load | 4 | `abs 0x120144` | `-` |
| `ldr` at 0x11ffa0 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11ffac | load | 4 | `abs 0x120144` | `-` |
| `ldr` at 0x11ffb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11ffb4 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x11ffc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11ffc8 | load | 4 | `abs 0x120148` | `-` |
| `ldr` at 0x11ffcc | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x11ffd4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x11ffe0 | load | 4 | `abs 0x12014c` | `-` |
| `str` at 0x11ffe4 | store | 4 | `sp -0x28` | `0xfffffbb4` |
| `ldr` at 0x11ffec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11fff4 | load | 4 | `abs 0x120148` | `-` |
| `ldr` at 0x11fff8 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x120000 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0x120004 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x120008 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x120018 | store | 4 | `sp -0x24` | `ret(0x1209e4)` |
| `ldr` at 0x12001c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120028 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x12002c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x120040 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0x120048 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x12004c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120054 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120064 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120068 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x12006c | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x120070 | store | 4 | `[sp-28] +0x0` | `[[sp-36]+4]:1` |
| `ldrb` at 0x120074 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x120080 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x120084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12009c | load | 4 | `abs 0x12014c` | `-` |
| `str` at 0x1200a0 | store | 4 | `sp -0x28` | `0xfffffbb4` |
| `ldr` at 0x1200a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1200ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x1200b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x120130 | load | 4 | `abs 0x120150` | `-` |
| `str` at 0x120134 | store | 4 | `sp -0x28` | `0xfffffbb1` |
| `ldr` at 0x120138 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x120140 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x120140 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x120140 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x120010 | `(unknown callee) 0x1209e4` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x120050 | `(unknown callee) 0x120154` | `[sp-28]` | `[sp-32]` | `?` | `?` |
| `bl` at 0x120060 | `(unknown callee) 0x20550` | `[sp-28]` | `0x0` | `0x78` | `?` |
| `bl` at 0x12008c | `(unknown callee) 0x120398` | `[sp-20]` | `[sp-21]:1` | `[sp-28]` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x11ff98 | `(r1 & 0xff) > 0xec (unsigned, 32-bit)` | `(r1 & 0xff) <= 0xec (unsigned, 32-bit)` |
| `beq` at 0x11ffa8 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x11ffbc | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x11ffdc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x120024 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhi` at 0x12007c | `[sp-21]:1 > 0xec (unsigned, 32-bit)` | `[sp-21]:1 <= 0xec (unsigned, 32-bit)` |
| `bne` at 0x120098 | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `ldrls` at 0x1200b8 | `[[sp-28]]-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x120140 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x1200b8).

## `eepromFreeData`

- `eepromFreeData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120154 to 0x1202e8.
  Established. An included word read addresses entry `r0 +0x0` (`ldr` at 0x12016c).
  A conditional word read has unknown effective address when `[r0]-1 <= 0x3 (unsigned, 32-bit)` (`ldrls` at 0x120178).
  The unnamed `0x20550` call carries entry `r0`, `0x0`, `0x78` and `[r0]-1` (`bl` at 0x1202e0).
  The computed transfer remains unresolved, and omitted data release effects and physical ownership remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x120164 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x120168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12016c | load | 4 | `r0 +0x0` | `-` |
| `ldrls` at 0x120178 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1202d4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1202e4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1202e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1202e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1202e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1202e0 | `(unknown callee) 0x20550` | `r0` | `0x0` | `0x78` | `[r0]-1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x120178 | `[r0]-1 <= 0x3 (unsigned, 32-bit)` | `[r0]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1202e4 | `r0` | `ret(0x20550)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x120178).

## `eepromGetCfg`

- `eepromGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1202e8 to 0x120340.
  Established. A word read addresses `[sp-28] +0x0` (`ldr` at 0x120320).
  A word assignment writes `[[sp-28]]` to entry `r1 +0x0` (`str` at 0x120324).
  The integer return candidate is `[[sp-28]+4]` (`ldm` at 0x120338).
  Configuration record meaning and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1202f8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1202fc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x120300 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x120308 | load | 4 | `abs 0x12033c` | `-` |
| `ldr` at 0x12030c | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x120314 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x120318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12031c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120320 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x120324 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x120328 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12032c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0x120338 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x120338 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x120338 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x120338 | `r0` | `[[sp-28]+4]` |

## `eepromGetData`

- `eepromGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120340 to 0x120398.
  Established. A word read addresses `[sp-28] +0xc` (`ldr` at 0x120378).
  A word assignment writes `[[sp-28]+12]` to entry `r1 +0x0` (`str` at 0x12037c).
  The integer return candidate is `[[sp-28]+4]` (`ldm` at 0x120390).
  Cached data ownership and physical EEPROM identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x120350 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x120354 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x120358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x120360 | load | 4 | `abs 0x120394` | `-` |
| `ldr` at 0x120364 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x12036c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x120370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x120374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120378 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x12037c | store | 4 | `r1 +0x0` | `[[sp-28]+12]` |
| `ldr` at 0x120380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120384 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0x120390 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x120390 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x120390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x120390 | `r0` | `[[sp-28]+4]` |

## `eepromIsPresent`

- `eepromIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120398 to 0x120414.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0x1203ec`, while greater than or equal falls through (`blt` at 0x1203dc).
  The taken path reads a byte at `((r1 & 0xff) + [[sp-28]+8]) +0x0` (`ldrb` at 0x1203fc).
  The fall-through path clears the word at `sp -0x20` (`str` at 0x1203e4).
  Presence-byte encoding and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1203a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1203b0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x1203b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1203bc | load | 4 | `abs 0x120410` | `-` |
| `ldr` at 0x1203c0 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x1203c8 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x1203cc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1203d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1203d4 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x1203e4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x1203ec | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1203f0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1203f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x1203fc | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0x120400 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0x120404 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x12040c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12040c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12040c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x1203dc | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12040c | `r0` | `?` |

## `eepromRead`

- `eepromRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120414 to 0x120688.
  Established. An included halfword read addresses `ret(0x1209e4) +0x2` (`ldrh` at 0x1204e4).
  One halfword assignment writes `0x20` to `sp -0x2c` (`strh` at 0x1205a8).
  The unnamed `0x10acd4` call carries unknown, `[sp-42]:1`, `[sp-28]` and `[sp-44]:2` (`bl` at 0x12061c).
  EEPROM read transaction semantics and hardware responsibility remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x120424 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x120428 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0x12042c | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x120434 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0x120438 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0x12043c | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0x120444 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0x120448 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x120454 | load | 4 | `abs 0x120674` | `-` |
| `ldr` at 0x120458 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x120464 | load | 4 | `abs 0x120674` | `-` |
| `ldr` at 0x120468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12046c | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x120478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x120480 | load | 4 | `abs 0x120678` | `-` |
| `ldr` at 0x120484 | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x12048c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x120498 | load | 4 | `abs 0x12067c` | `-` |
| `str` at 0x12049c | store | 4 | `sp -0x34` | `0xfffffbb4` |
| `ldr` at 0x1204a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1204ac | load | 4 | `abs 0x120678` | `-` |
| `ldr` at 0x1204b0 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x1204b8 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x1204bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1204c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1204d0 | store | 4 | `sp -0x24` | `ret(0x1209e4)` |
| `ldrh` at 0x1204d4 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0x1204d8 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0x1204e0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x1204e4 | load | 2 | `ret(0x1209e4) +0x2` | `-` |
| `ldr` at 0x1204f0 | load | 4 | `abs 0x120680` | `-` |
| `str` at 0x1204f4 | store | 4 | `sp -0x48` | `0x18f768` |
| `ldr` at 0x1204f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1204fc | load | 4 | `ret(0x1209e4) +0x8` | `-` |
| `str` at 0x120500 | store | 4 | `sp -0x44` | `[ret(0x1209e4)+8]` |
| `ldrh` at 0x120504 | load | 2 | `sp -0x18` | `-` |
| `str` at 0x120508 | store | 4 | `sp -0x40` | `[sp-24]:2` |
| `ldrh` at 0x12050c | load | 2 | `sp -0x1e` | `-` |
| `str` at 0x120510 | store | 4 | `sp -0x3c` | `[sp-30]:2` |
| `ldr` at 0x120514 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x120518 | load | 2 | `ret(0x1209e4) +0x2` | `-` |
| `str` at 0x12051c | store | 4 | `sp -0x38` | `[ret(0x1209e4)+2]:2` |
| `ldr` at 0x120524 | load | 4 | `abs 0x120684` | `-` |
| `str` at 0x120538 | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0x120540 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0x12054c | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x120558 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0x120564 | store | 1 | `sp -0x29` | `?` |
| `ldrh` at 0x120568 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0x12056c | store | 1 | `sp -0x2a` | `([sp-24]:2 & 0xff)` |
| `ldrb` at 0x120570 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0x120578 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0x120584 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0x120590 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0x120594 | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldrh` at 0x120598 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0x1205a8 | store | 2 | `sp -0x2c` | `0x20` |
| `ldr` at 0x1205ac | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x1205b0 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x1205b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1205d4 | store | 4 | `sp -0x28` | `ret(0x10a87c)` |
| `ldr` at 0x1205d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1205e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1205e8 | store | 4 | `sp -0x34` | `ret(0x10a87c)` |
| `ldr` at 0x1205f0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x1205f4 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0x1205f8 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0x120604 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0x120608 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0x120614 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x120624 | store | 4 | `sp -0x28` | `ret(0x10acd4)` |
| `ldrh` at 0x12062c | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0x120630 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0x120638 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0x12063c | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0x120640 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x120648 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0x12064c | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0x120650 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0x120658 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0x120660 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x120664 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0x120668 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x120670 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x120670 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x120670 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1204c8 | `(unknown callee) 0x1209e4` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x120530 | `(unknown callee) 0x22070` | `0x6` | `0x18f660` | `0x264` | `0x0` |
| `bl` at 0x1205bc | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0x1205cc | `(unknown callee) 0x10a87c` | `?` | `?` | `?` | `?` |
| `bl` at 0x12061c | `(unknown callee) 0x10acd4` | `?` | `[sp-42]:1` | `[sp-28]` | `[sp-44]:2` |
| `bl` at 0x120628 | `(unknown callee) 0x10a9c0` | `ret(0x10acd4)` | `?` | `?` | `ret(0x10acd4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x120450 | `(r1 & 0xff) > 0xec (unsigned, 32-bit)` | `(r1 & 0xff) <= 0xec (unsigned, 32-bit)` |
| `beq` at 0x120460 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x120474 | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x120494 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x1204ec | `([sp-24]:2 + [sp-30]:2) <= [ret(0x1209e4)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0x1209e4)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0x120548 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0x120554 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0x12058c | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |
| `bls` at 0x1205a0 | `[sp-44]:2 <= 0x20 (unsigned, 32-bit)` | `[sp-44]:2 > 0x20 (unsigned, 32-bit)` |
| `beq` at 0x1205e0 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x120670 | `r0` | `?` |

## `eepromWrite`

- `eepromWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120688 to 0x1208b8.
  Established. An included halfword read addresses `ret(0x1209e4) +0x2` (`ldrh` at 0x12074c).
  The unnamed `0x122ca4` call carries unknown, unknown, `[sp-42]:1` and `[sp-28]` (`bl` at 0x12084c).
  A word assignment writes `([sp-28] + [sp-44]:2)` to `sp -0x1c` (`str` at 0x120874).
  EEPROM write transaction semantics and physical persistence remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x120698 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12069c | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0x1206a0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x1206a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0x1206ac | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0x1206b0 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0x1206b8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1206bc | load | 4 | `abs 0x1208a0` | `-` |
| `ldr` at 0x1206c0 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x1206cc | load | 4 | `abs 0x1208a0` | `-` |
| `ldr` at 0x1206d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1206d4 | load | 4 | `abs 0x314fac` | `-` |
| `ldr` at 0x1206e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1206e8 | load | 4 | `abs 0x1208a4` | `-` |
| `ldr` at 0x1206ec | load | 4 | `abs 0x314fa8` | `-` |
| `ldr` at 0x1206f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x120700 | load | 4 | `abs 0x1208a8` | `-` |
| `str` at 0x120704 | store | 4 | `sp -0x34` | `0xfffffbb4` |
| `ldr` at 0x12070c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x120714 | load | 4 | `abs 0x1208a4` | `-` |
| `ldr` at 0x120718 | load | 4 | `abs 0x314fa8` | `-` |
| `str` at 0x120720 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x120724 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x120728 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x120738 | store | 4 | `sp -0x24` | `ret(0x1209e4)` |
| `ldrh` at 0x12073c | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0x120740 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0x120748 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x12074c | load | 2 | `ret(0x1209e4) +0x2` | `-` |
| `ldr` at 0x120758 | load | 4 | `abs 0x1208ac` | `-` |
| `str` at 0x12075c | store | 4 | `sp -0x48` | `0x18f768` |
| `ldr` at 0x120760 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x120764 | load | 4 | `ret(0x1209e4) +0x8` | `-` |
| `str` at 0x120768 | store | 4 | `sp -0x44` | `[ret(0x1209e4)+8]` |
| `ldrh` at 0x12076c | load | 2 | `sp -0x18` | `-` |
| `str` at 0x120770 | store | 4 | `sp -0x40` | `[sp-24]:2` |
| `ldrh` at 0x120774 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0x120778 | store | 4 | `sp -0x3c` | `[sp-30]:2` |
| `ldr` at 0x12077c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x120780 | load | 2 | `ret(0x1209e4) +0x2` | `-` |
| `str` at 0x120784 | store | 4 | `sp -0x38` | `[ret(0x1209e4)+2]:2` |
| `ldr` at 0x12078c | load | 4 | `abs 0x1208b0` | `-` |
| `ldr` at 0x120790 | load | 4 | `abs 0x1208b4` | `-` |
| `str` at 0x1207a0 | store | 4 | `sp -0x34` | `0x0` |
| `ldrh` at 0x1207a8 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0x1207b4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x1207c0 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0x1207cc | store | 1 | `sp -0x29` | `?` |
| `ldrh` at 0x1207d0 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0x1207d4 | store | 1 | `sp -0x2a` | `([sp-24]:2 & 0xff)` |
| `ldrb` at 0x1207d8 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0x1207e0 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0x1207ec | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0x1207f8 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0x1207fc | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0x120800 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x120804 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x120808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12081c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x120820 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0x120824 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0x120830 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0x120834 | load | 2 | `sp -0x2c` | `-` |
| `str` at 0x120838 | store | 4 | `sp -0x48` | `[sp-44]:2` |
| `ldr` at 0x120848 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x120854 | store | 4 | `sp -0x28` | `ret(0x122ca4)` |
| `ldrh` at 0x120858 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0x12085c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0x120864 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0x120868 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0x12086c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x120874 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0x120878 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0x12087c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0x120884 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0x12088c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x120890 | store | 4 | `sp -0x34` | `[sp-40]` |
| `ldr` at 0x120894 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x12089c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12089c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12089c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x120730 | `(unknown callee) 0x1209e4` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0x120798 | `(unknown callee) 0x22070` | `0x6` | `0x18f660` | `0x2ae` | `0x0` |
| `bl` at 0x120810 | `(unknown callee) 0x10a4ec` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0x12084c | `(unknown callee) 0x122ca4` | `?` | `?` | `[sp-42]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1206c8 | `[0x314fac] == 0x0` | `[0x314fac] != 0x0` |
| `bge` at 0x1206dc | `r0 >= [0x314fac] (signed, 32-bit)` | `r0 < [0x314fac] (signed, 32-bit)` |
| `bne` at 0x1206fc | `? != 0x0` | `? == 0x0` |
| `bls` at 0x120754 | `([sp-24]:2 + [sp-30]:2) <= [ret(0x1209e4)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0x1209e4)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0x1207b0 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0x1207bc | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0x1207f4 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12089c | `r0` | `?` |

## `eepromName`

- `eepromName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1208b8 to 0x120944.
  Established. The unnamed `0x1209e4` call carries entry `r0`, `(r1 & 0xff)`, entry `r2` and `(r1 & 0xff)` (`bl` at 0x1208e0);
  its nonzero result takes `0x120904`, while zero falls through (`bne` at 0x1208f4).
  Included word reads address `ret(0x1209e4) +0x8` (`ldr` at 0x120908, `ldr` at 0x120914).
  A separate word assignment places `0x18f7a8` at `sp -0x20` (`str` at 0x1208fc).
  Name record ownership and hardware identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1208c8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1208d0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x1208d4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1208d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1208e8 | store | 4 | `sp -0x1c` | `ret(0x1209e4)` |
| `ldr` at 0x1208ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1208f8 | load | 4 | `abs 0x12093c` | `-` |
| `str` at 0x1208fc | store | 4 | `sp -0x20` | `0x18f7a8` |
| `ldr` at 0x120904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120908 | load | 4 | `ret(0x1209e4) +0x8` | `-` |
| `str` at 0x12090c | store | 4 | `sp -0x24` | `[ret(0x1209e4)+8]` |
| `ldr` at 0x120910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x120914 | load | 4 | `ret(0x1209e4) +0x8` | `-` |
| `ldr` at 0x120920 | load | 4 | `abs 0x120940` | `-` |
| `str` at 0x120924 | store | 4 | `sp -0x24` | `0x18f7b0` |
| `ldr` at 0x120928 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12092c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x120930 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x120938 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x120938 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x120938 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1208e0 | `(unknown callee) 0x1209e4` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1208f4 | `ret(0x1209e4) != 0x0` | `ret(0x1209e4) == 0x0` |
| `bne` at 0x12091c | `[ret(0x1209e4)+8] != 0x0` | `[ret(0x1209e4)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x120938 | `r0` | `?` |

## `eepromStatus`

- `eepromStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x120944 to 0x1209e4.
  Established. A conditional word read has unknown effective address when entry `r0 <= 0x4 (unsigned, 32-bit)` (`ldrls` at 0x120960).
  A known path assigns `0x18f804` to the word at `sp -0x18` (`str` at 0x1209bc);
  its integer return candidate is `0x18f804` (`ldm` at 0x1209c8).
  The computed transfer remains unresolved, and omitted status choices and hardware encoding remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x120954 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x120958 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x120960 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1209b8 | load | 4 | `abs 0x1209e0` | `-` |
| `str` at 0x1209bc | store | 4 | `sp -0x18` | `0x18f804` |
| `ldr` at 0x1209c0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1209c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1209c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1209c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x120960 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1209c8 | `r0` | `0x18f804` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x120960).

## `i2cInitializeSlot`

- `i2cInitializeSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x598a8 to 0x59920.
  Established. The unnamed `0x8767c` call carries `0x0`, `0x0`, entry `r2` and entry `r3` (`bl` at 0x598c8).
  Zero `[sp-24]` takes `0x598e0`, while nonzero falls through (`beq` at 0x598d4).
  The unnamed `0x125150` call carries `[sp-20]`, unknown, unknown and `[sp-24]` (`bl` at 0x598dc).
  The unnamed `0x11fe90` call carries `[sp-20]`, unknown, unknown and unknown (`bl` at 0x598e4).
  Slot initialization responsibilities at the physical bus remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x598b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x598bc | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x598cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x598d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x598f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59900 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59908 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x59910 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5991c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5991c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5991c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x598c8 | `(unknown callee) 0x8767c` | `0x0` | `0x0` | `r2` | `r3` |
| `bl` at 0x598dc | `(unknown callee) 0x125150` | `[sp-20]` | `?` | `?` | `[sp-24]` |
| `bl` at 0x598e4 | `(unknown callee) 0x11fe90` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x598ec | `(unknown callee) 0x111a1c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x598f4 | `(unknown callee) 0x52ed4` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x598fc | `(unknown callee) 0x7e638` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x59904 | `(unknown callee) 0x6235c` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x5990c | `(unknown callee) 0x5b3f0` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x59914 | `(unknown callee) 0xb286c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x598d4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5991c | `r0` | `ret(0xb286c)` |
