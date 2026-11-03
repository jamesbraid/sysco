# ARM Linux 2.6 SHUB scan routines in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-shub-scan-facts.txt) retain all tool sections.

## `SHUB_ScanSetRackSlot`

- `SHUB_ScanSetRackSlot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x657a0 to 0x6593c.
  Established. A byte store writes 0x6c at entry sp minus 0x21 (`strb` at 0x657b8).
  The result from unnamed 0xa3c7c is stored as a word at entry sp minus 0x28 (`bl` at 0x657c0, `str` at 0x657c8).
  A later word store writes 0xfffff88e at entry sp minus 0x2c (`str` at 0x6590c).
  The return record retains an unknown value in r0 (`ldm` at 0x65918).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x657b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x657b8 | store | 1 | `sp -0x21` | `0x6c` |
| `str` at 0x657c8 | store | 4 | `sp -0x28` | `ret(0xa3c7c)` |
| `ldr` at 0x657cc | load | 4 | `abs 0x6591c` | `-` |
| `ldr` at 0x657d0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x657dc | load | 4 | `abs 0x65920` | `-` |
| `str` at 0x657e0 | store | 4 | `sp -0x38` | `0x16326c` |
| `ldr` at 0x657e8 | load | 4 | `abs 0x65924` | `-` |
| `ldr` at 0x657ec | load | 4 | `abs 0x65928` | `-` |
| `ldr` at 0x657f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x657fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x65808 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6580c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x65810 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6581c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x65820 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x65830 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65840 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x65844 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x65854 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x6585c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x65860 | load | 4 | `abs 0x6591c` | `-` |
| `ldr` at 0x65864 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x65870 | load | 4 | `abs 0x6592c` | `-` |
| `str` at 0x65874 | store | 4 | `sp -0x38` | `0x163284` |
| `ldr` at 0x65878 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x6587c | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x65880 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x65884 | store | 4 | `sp -0x30` | `[sp-32]` |
| `ldr` at 0x6588c | load | 4 | `abs 0x65924` | `-` |
| `str` at 0x658a0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x658a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x658b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x658b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x658c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x658c4 | load | 4 | `abs 0x65930` | `-` |
| `ldr` at 0x658c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x658e0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x658e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x658f0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x658f8 | load | 4 | `abs 0x65934` | `-` |
| `ldr` at 0x658fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65900 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65908 | load | 4 | `abs 0x65938` | `-` |
| `str` at 0x6590c | store | 4 | `sp -0x2c` | `0xfffff88e` |
| `ldr` at 0x65910 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x65918 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65918 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65918 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x657c0 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `0x6c` |
| `bl` at 0x657f4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x1cd` | `0x0` |
| `bl` at 0x65898 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x1d8` | `0x0` |
| `bl` at 0x658cc | `(unknown callee) 0x64ce0` | `[sp-20]` | `0x101d0100` | `[sp-28]` | `?` |
| `bl` at 0x65904 | `(unknown callee) 0x29908` | `0x1632b8` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x657d8 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6586c | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bgt` at 0x658ac | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bne` at 0x658d8 | `ret(0x64ce0) != 0x1` | `ret(0x64ce0) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65918 | `r0` | `?` |

## `SHUB_ScanResetNI`

- `SHUB_ScanResetNI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65d8c to 0x65f2c.
  Established. Word stores write 0 at entry sp minus 0x14, 0x18 and 0x1c (`str` at 0x65da0, `str` at 0x65da8, `str` at 0x65db0).
  The result from unnamed 0x6593c is stored as a word at entry sp minus 0x18 (`bl` at 0x65e34, `str` at 0x65e3c).
  When the beq falls through for a nonzero result from unnamed 0x6593c, a word store writes that result at entry sp minus 0x14 (`bl` at 0x65e34, `beq` at 0x65e48, `ldr` at 0x65e4c, `str` at 0x65e50).
  A load, addition and word store write [sp-28] plus 1 at entry sp minus 0x1c (`ldr` at 0x65f04, `add` at 0x65f08, `str` at 0x65f0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x65da0 | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0x65da8 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x65db0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x65db4 | load | 4 | `abs 0x65f24` | `-` |
| `ldrb` at 0x65db8 | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x65dbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65dc8 | load | 4 | `abs 0x65f28` | `-` |
| `ldr` at 0x65dcc | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x65dec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x65dfc | load | 4 | `abs 0x65f28` | `-` |
| `ldr` at 0x65e00 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x65e20 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x65e28 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65e3c | store | 4 | `sp -0x18` | `ret(0x6593c)` |
| `ldr` at 0x65e40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x65e4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x65e50 | store | 4 | `sp -0x14` | `ret(0x6593c)` |
| `ldr` at 0x65e54 | load | 4 | `abs 0x65f28` | `-` |
| `ldr` at 0x65e58 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x65e78 | load | 2 | `(unknown)` | `-` |
| `str` at 0x65e8c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x65e90 | load | 4 | `abs 0x65f28` | `-` |
| `ldr` at 0x65e94 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x65eb4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x65ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65ed8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x65ee8 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0x65eec | load | 4 | `sp -0x20` | `-` |
| `str` at 0x65ef0 | store | 4 | `sp -0x18` | `[sp-32]` |
| `ldr` at 0x65efc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x65f00 | store | 4 | `sp -0x14` | `[sp-32]` |
| `ldr` at 0x65f04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65f0c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x65f14 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x65f20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x65f20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x65f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65e34 | `(unknown callee) 0x6593c` | `[sp-28]` | `0x0` | `0x1` | `?` |
| `bl` at 0x65ec8 | `(unknown callee) 0x6593c` | `[sp-28]` | `0x1` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x65dc4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x65df8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x65e48 | `ret(0x6593c) == 0x0` | `ret(0x6593c) != 0x0` |
| `bne` at 0x65e84 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x65ed4 | `ret(0x6593c) == 0x0` | `ret(0x6593c) != 0x0` |
| `bne` at 0x65ee0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x65ef8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x65f20 | `r0` | `?` |

## `SHUB_ScanResetLLP`

- `SHUB_ScanResetLLP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65f2c to 0x66384.
  Established. An initial word store writes 0 at entry sp minus 0x1c (`str` at 0x65f6c).
  Word stores write 0xfffff88c at entry sp minus 0x20 after the corresponding bne falls through for a zero call result (`bl` at 0x65f94, `bne` at 0x65fa0, `str` at 0x65fa8, `bl` at 0x65fc0, `bne` at 0x65fcc, `str` at 0x65fd4).
  The call to unnamed 0x65d64 has printed r0-r3 candidates 0x1c2, unknown, [0x1a5ca8]:1 and unknown (`bl` at 0x65ff0).
  A later word store writes 0 at entry sp minus 0x20 (`str` at 0x66324).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65f3c | load | 4 | `abs 0x66334` | `-` |
| `ldr` at 0x65f40 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x65f4c | load | 4 | `abs 0x66338` | `-` |
| `str` at 0x65f50 | store | 4 | `sp -0x2c` | `0x1633ec` |
| `ldr` at 0x65f58 | load | 4 | `abs 0x6633c` | `-` |
| `ldr` at 0x65f5c | load | 4 | `abs 0x66340` | `-` |
| `str` at 0x65f6c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x65f70 | load | 4 | `abs 0x66344` | `-` |
| `ldrb` at 0x65f74 | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x65f78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65f84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65f88 | load | 4 | `abs 0x66348` | `-` |
| `ldr` at 0x65fa4 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x65fa8 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x65fb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x65fb4 | load | 4 | `abs 0x66350` | `-` |
| `ldr` at 0x65fd0 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x65fd4 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x65fdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x65fe4 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0x65fec | load | 4 | `abs 0x66354` | `-` |
| `str` at 0x65ff8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x65ffc | load | 4 | `abs 0x66344` | `-` |
| `ldrb` at 0x66000 | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x66004 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6601c | load | 4 | `abs 0x66348` | `-` |
| `ldr` at 0x66038 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x6603c | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x66044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66058 | load | 4 | `abs 0x66358` | `-` |
| `str` at 0x6605c | store | 4 | `sp -0x2c` | `0x163400` |
| `ldr` at 0x66060 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66064 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x66068 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6606c | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x66074 | load | 4 | `abs 0x6633c` | `-` |
| `ldr` at 0x66078 | load | 4 | `abs 0x6635c` | `-` |
| `ldr` at 0x66094 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66098 | load | 4 | `abs 0x66350` | `-` |
| `ldr` at 0x660b4 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x660b8 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x660c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x660d4 | load | 4 | `abs 0x66360` | `-` |
| `str` at 0x660d8 | store | 4 | `sp -0x2c` | `0x163430` |
| `ldr` at 0x660dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x660e0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x660e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x660e8 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x660f0 | load | 4 | `abs 0x6633c` | `-` |
| `ldr` at 0x660f4 | load | 4 | `abs 0x66364` | `-` |
| `ldr` at 0x66108 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66110 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x6611c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x66120 | load | 4 | `abs 0x66344` | `-` |
| `ldrb` at 0x66124 | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x66128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66134 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66154 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x66158 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x66160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66164 | load | 4 | `abs 0x66368` | `-` |
| `ldr` at 0x66180 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x66184 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x6618c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66194 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x661a8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x661ac | load | 4 | `abs 0x66344` | `-` |
| `ldrb` at 0x661b0 | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x661b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x661c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x661e8 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x661ec | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x661f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66200 | load | 4 | `abs 0x6636c` | `-` |
| `ldr` at 0x66204 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x66224 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x66234 | load | 4 | `abs 0x66370` | `-` |
| `str` at 0x66238 | store | 4 | `sp -0x2c` | `0x163460` |
| `ldr` at 0x6623c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66240 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x66244 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66248 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x66250 | load | 4 | `abs 0x6633c` | `-` |
| `ldr` at 0x66254 | load | 4 | `abs 0x66374` | `-` |
| `ldr` at 0x66270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x66274 | load | 4 | `abs 0x66378` | `-` |
| `ldr` at 0x66290 | load | 4 | `abs 0x6634c` | `-` |
| `str` at 0x66294 | store | 4 | `sp -0x20` | `0xfffff88c` |
| `ldr` at 0x6629c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x662a8 | load | 4 | `abs 0x6636c` | `-` |
| `ldr` at 0x662ac | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x662cc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x662dc | load | 4 | `abs 0x6637c` | `-` |
| `str` at 0x662e0 | store | 4 | `sp -0x2c` | `0x163490` |
| `ldr` at 0x662e4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x662e8 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0x662ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x662f0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldr` at 0x662f8 | load | 4 | `abs 0x6633c` | `-` |
| `ldr` at 0x662fc | load | 4 | `abs 0x66380` | `-` |
| `ldr` at 0x66310 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x66318 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x66324 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x66328 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x66330 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x66330 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x66330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x65f64 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x289` | `0x0` |
| `bl` at 0x65f94 | `(unknown callee) 0x64ce0` | `?` | `0x500303b0` | `0x1` | `?` |
| `bl` at 0x65fc0 | `(unknown callee) 0x64ce0` | `[sp-28]` | `0x500305b0` | `0x1` | `?` |
| `bl` at 0x65ff0 | `(unknown callee) 0x65d64` | `0x1c2` | `?` | `[0x1a5ca8]:1` | `?` |
| `bl` at 0x66028 | `(unknown callee) 0x64d58` | `?` | `0x500303b0` | `sp-20` | `sp-24` |
| `bl` at 0x66080 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x29b` | `0x0` |
| `bl` at 0x66088 | `(unknown callee) 0x65d64` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x660a4 | `(unknown callee) 0x64d58` | `[sp-28]` | `0x500305b0` | `sp-20` | `sp-24` |
| `bl` at 0x660fc | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x2a2` | `0x0` |
| `bl` at 0x66104 | `(unknown callee) 0x65d64` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x66144 | `(unknown callee) 0x64ce0` | `?` | `0x50000008` | `0x0` | `0x1` |
| `bl` at 0x66170 | `(unknown callee) 0x64ce0` | `[sp-28]` | `0x50002008` | `0x0` | `0x1` |
| `bl` at 0x661a0 | `(unknown callee) 0x65d64` | `0xa` | `?` | `[0x1a5ca8]:1` | `?` |
| `bl` at 0x661d8 | `(unknown callee) 0x64d58` | `?` | `0x50000000` | `sp-20` | `sp-24` |
| `bl` at 0x6625c | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x2bb` | `0x0` |
| `bl` at 0x66264 | `(unknown callee) 0x65d64` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x66280 | `(unknown callee) 0x64d58` | `[sp-28]` | `0x50002000` | `sp-20` | `sp-24` |
| `bl` at 0x66304 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x2c2` | `0x0` |
| `bl` at 0x6630c | `(unknown callee) 0x65d64` | `0xa` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65f48 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `ble` at 0x65f80 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x65fa0 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `bne` at 0x65fcc | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `ble` at 0x6600c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x66034 | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `beq` at 0x66054 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x660b0 | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `beq` at 0x660d0 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x66130 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x66150 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `bne` at 0x6617c | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `ble` at 0x661bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x661e4 | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `beq` at 0x661fc | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x66230 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6628c | `ret(0x64d58) != 0x0` | `ret(0x64d58) == 0x0` |
| `beq` at 0x662a4 | `[sp-24] == 0x9` | `[sp-24] != 0x9` |
| `beq` at 0x662d8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x66330 | `r0` | `?` |

## `SHUB_ScanInit_LLP_TEST_CTL`

- `SHUB_ScanInit_LLP_TEST_CTL`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x66384 to 0x6653c.
  Established. The beq has an unknown tested value: fall-through writes 0x00a00300 as a word at entry sp minus 0x18, while the taken path writes 0x00a00000 there (`beq` at 0x66410, `str` at 0x66418, `str` at 0x66424).
  Fall-through from the first bne for a zero call result writes 0xfffff88a as a word at entry sp minus 0x1c (`bl` at 0x66438, `bne` at 0x66444, `str` at 0x66458).
  Fall-through from the later bne for a zero call result writes 0xfffff889 at the same word location (`bl` at 0x664bc, `bne` at 0x664c8, `str` at 0x664dc).
  Another word store writes 0 at entry sp minus 0x1c (`str` at 0x664f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x66394 | load | 4 | `abs 0x66508` | `-` |
| `ldr` at 0x66398 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x663a4 | load | 4 | `abs 0x6650c` | `-` |
| `str` at 0x663a8 | store | 4 | `sp -0x20` | `0x1634c0` |
| `ldr` at 0x663b0 | load | 4 | `abs 0x66510` | `-` |
| `str` at 0x663c4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x663c8 | load | 4 | `abs 0x66514` | `-` |
| `ldrb` at 0x663cc | load | 1 | `abs 0x1a5ca8` | `-` |
| `ldr` at 0x663d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x663dc | load | 4 | `abs 0x66518` | `-` |
| `ldr` at 0x663e0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x66400 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x66414 | load | 4 | `abs 0x6651c` | `-` |
| `str` at 0x66418 | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x66424 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x66428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6642c | load | 4 | `abs 0x66520` | `-` |
| `ldr` at 0x66430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x66434 | load | 4 | `abs 0x66524` | `-` |
| `ldr` at 0x66448 | load | 4 | `abs 0x66528` | `-` |
| `ldr` at 0x6644c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66454 | load | 4 | `abs 0x6652c` | `-` |
| `str` at 0x66458 | store | 4 | `sp -0x1c` | `0xfffff88a` |
| `ldr` at 0x66460 | load | 4 | `abs 0x66518` | `-` |
| `ldr` at 0x66464 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x66484 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x66498 | load | 4 | `abs 0x6651c` | `-` |
| `str` at 0x6649c | store | 4 | `sp -0x18` | `0xa00300` |
| `str` at 0x664a8 | store | 4 | `sp -0x18` | `0xa00000` |
| `ldr` at 0x664ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x664b0 | load | 4 | `abs 0x66530` | `-` |
| `ldr` at 0x664b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x664b8 | load | 4 | `abs 0x66524` | `-` |
| `ldr` at 0x664cc | load | 4 | `abs 0x66534` | `-` |
| `ldr` at 0x664d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x664d8 | load | 4 | `abs 0x66538` | `-` |
| `str` at 0x664dc | store | 4 | `sp -0x1c` | `0xfffff889` |
| `ldr` at 0x664e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x664ec | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x664f8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x664fc | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x66504 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x66504 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x66504 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x663bc | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x2d8` | `0x0` |
| `bl` at 0x66438 | `(unknown callee) 0x64ce0` | `[sp-20]` | `0x50000028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x66450 | `(unknown callee) 0x29908` | `0x1634dc` | `[sp-20]` | `?` | `ret(0x64ce0)` |
| `bl` at 0x664bc | `(unknown callee) 0x64ce0` | `[sp-20]` | `0x50002028` | `[sp-24]` | `0xa5fffff` |
| `bl` at 0x664d4 | `(unknown callee) 0x29908` | `0x1634fc` | `[sp-20]` | `?` | `ret(0x64ce0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x663a0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `ble` at 0x663d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x66410 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x66444 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |
| `beq` at 0x66494 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x664c8 | `ret(0x64ce0) != 0x0` | `ret(0x64ce0) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x66504 | `r0` | `?` |

## `SHUB_ScanInitLLP`

- `SHUB_ScanInitLLP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6653c to 0x66ae0.
  Established. The result from a call through an unknown target is stored as words at entry sp minus 0x78 and 0x80 (`mov` at 0x66690, `str` at 0x66698, `ldr` at 0x6669c, `str` at 0x666a0).
  One computed halfword store writes 5 to an unknown effective target (`strh` at 0x66710).
  A separate computed halfword store writes 0 to an unknown effective target (`strh` at 0x6674c).
  A later word store writes 0xfffff888 at entry sp minus 0x7c (`str` at 0x6691c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6654c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x66554 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x66558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66560 | load | 4 | `abs 0x66a8c` | `-` |
| `ldr` at 0x66564 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x6656c | load | 4 | `(unknown)` | `-` |
| `str` at 0x66570 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x66574 | load | 4 | `abs 0x66a90` | `-` |
| `ldr` at 0x66578 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x66584 | load | 4 | `abs 0x66a94` | `-` |
| `str` at 0x66588 | store | 4 | `sp -0x90` | `0x16351c` |
| `ldr` at 0x6658c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66590 | store | 4 | `sp -0x8c` | `r0` |
| `ldr` at 0x66598 | load | 4 | `abs 0x66a98` | `-` |
| `ldr` at 0x6659c | load | 4 | `abs 0x66a9c` | `-` |
| `str` at 0x665d4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x665d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x665e4 | load | 4 | `abs 0x66aa0` | `-` |
| `ldr` at 0x665e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x665f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x665f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x66604 | load | 4 | `abs 0x66a90` | `-` |
| `ldr` at 0x66608 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x66614 | load | 4 | `abs 0x66aa4` | `-` |
| `str` at 0x66618 | store | 4 | `sp -0x90` | `0x163530` |
| `ldr` at 0x6661c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66620 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x66624 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66628 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x6662c | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x66630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66644 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x6665c | load | 2 | `(unknown)` | `-` |
| `str` at 0x66660 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x66668 | load | 4 | `abs 0x66a98` | `-` |
| `ldr` at 0x6666c | load | 4 | `abs 0x66aac` | `-` |
| `ldr` at 0x6667c | load | 4 | `abs 0x66ab0` | `-` |
| `ldr` at 0x66680 | load | 4 | `abs 0x1a5cbc` | `-` |
| `ldr` at 0x66684 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66688 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x66698 | store | 4 | `sp -0x78` | `ret(unknown)` |
| `ldr` at 0x6669c | load | 4 | `sp -0x78` | `-` |
| `str` at 0x666a0 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x666a4 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x666b0 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x666bc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x666cc | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x666dc | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x666e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x666f4 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x66710 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x66718 | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x6671c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x66730 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x6674c | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x66754 | load | 4 | `abs 0x66a90` | `-` |
| `ldr` at 0x66758 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x66764 | load | 4 | `abs 0x66ab4` | `-` |
| `str` at 0x66768 | store | 4 | `sp -0x90` | `0x16356c` |
| `ldr` at 0x6676c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66770 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x66778 | load | 4 | `abs 0x66a98` | `-` |
| `ldr` at 0x66788 | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x6678c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x667a0 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0x667bc | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x667c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x667c8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x667d0 | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x667d4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x667f4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x667f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x667fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66800 | load | 4 | `abs 0x66ab8` | `-` |
| `ldr` at 0x6680c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66814 | load | 4 | `abs 0x66ab8` | `-` |
| `str` at 0x66820 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66824 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66828 | load | 4 | `abs 0x66abc` | `-` |
| `ldr` at 0x66834 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6683c | load | 4 | `abs 0x66abc` | `-` |
| `str` at 0x66848 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6684c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66850 | load | 4 | `abs 0x66ac0` | `-` |
| `ldr` at 0x6685c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66864 | load | 4 | `abs 0x66ac0` | `-` |
| `str` at 0x66870 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66874 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x66898 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6689c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x668ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x668c0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x668cc | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x668d0 | load | 4 | `abs 0x66ac4` | `-` |
| `str` at 0x668d4 | store | 4 | `sp -0x8c` | `0x16359c` |
| `ldr` at 0x668d8 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x668f4 | load | 4 | `abs 0x66ac8` | `-` |
| `str` at 0x668f8 | store | 4 | `sp -0x90` | `0x1635a8` |
| `ldr` at 0x668fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66900 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x66908 | load | 4 | `abs 0x66a98` | `-` |
| `ldr` at 0x6690c | load | 4 | `abs 0x66acc` | `-` |
| `ldr` at 0x66918 | load | 4 | `abs 0x66ad0` | `-` |
| `str` at 0x6691c | store | 4 | `sp -0x7c` | `0xfffff888` |
| `ldr` at 0x66924 | load | 4 | `abs 0x66aa8` | `-` |
| `ldr` at 0x66928 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x66948 | load | 2 | `(unknown)` | `-` |
| `str` at 0x6694c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66950 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66954 | load | 4 | `abs 0x66ab8` | `-` |
| `ldr` at 0x66960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66968 | load | 4 | `abs 0x66ab8` | `-` |
| `str` at 0x66974 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x66978 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6697c | load | 4 | `abs 0x66abc` | `-` |
| `ldr` at 0x66988 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66990 | load | 4 | `abs 0x66abc` | `-` |
| `str` at 0x6699c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x669a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x669a4 | load | 4 | `abs 0x66ac0` | `-` |
| `ldr` at 0x669b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x669b8 | load | 4 | `abs 0x66ac0` | `-` |
| `str` at 0x669c4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x669c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x669d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x669ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x669f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x66a00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x66a14 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x66a20 | store | 4 | `sp -0x90` | `0xd` |
| `ldr` at 0x66a24 | load | 4 | `abs 0x66ad4` | `-` |
| `str` at 0x66a28 | store | 4 | `sp -0x8c` | `0x1635d4` |
| `ldr` at 0x66a2c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x66a48 | load | 4 | `abs 0x66ad8` | `-` |
| `str` at 0x66a4c | store | 4 | `sp -0x90` | `0x1635e0` |
| `ldr` at 0x66a50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x66a54 | store | 4 | `sp -0x8c` | `[sp-20]` |
| `ldr` at 0x66a5c | load | 4 | `abs 0x66a98` | `-` |
| `ldr` at 0x66a60 | load | 4 | `abs 0x66adc` | `-` |
| `ldr` at 0x66a6c | load | 4 | `abs 0x66ad0` | `-` |
| `str` at 0x66a70 | store | 4 | `sp -0x7c` | `0xfffff888` |
| `str` at 0x66a7c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x66a80 | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0x66a88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x66a88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x66a88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x665a4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x30b` | `0x0` |
| `bl` at 0x665b8 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `sp-112` | `sp-112` |
| `bl` at 0x665cc | `(unknown callee) 0x12cee4` | `0x0` | `0x0` | `0x20` | `sp-112` |
| `bl` at 0x66674 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x319` | `0x0` |
| `mov` at 0x66690 | `(unknown callee) (unknown target)` | `[sp-20]` | `?` | `?` | `[0x1a5cbc]` |
| `bl` at 0x66784 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x328` | `0x0` |
| `bl` at 0x668e4 | `(unknown callee) 0x13d9cc` | `[sp-116]` | `0xa` | `sp-32` | `0x0` |
| `bl` at 0x66914 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x336` | `0x0` |
| `bl` at 0x66a38 | `(unknown callee) 0x13d9cc` | `[sp-116]` | `0xb` | `sp-32` | `0x0` |
| `bl` at 0x66a68 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x33e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x66580 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bgt` at 0x665e0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x66600 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x66610 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x666ac | `ret(unknown) == 0x1` | `ret(unknown) != 0x1` |
| `bgt` at 0x666b8 | `ret(unknown) > 0x1 (signed, 32-bit)` | `ret(unknown) <= 0x1 (signed, 32-bit)` |
| `beq` at 0x666c4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x666d4 | `ret(unknown) == 0x2` | `ret(unknown) != 0x2` |
| `beq` at 0x66760 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x668f0 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x66a44 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x66a88 | `r0` | `?` |

## `SHUB_ScanInitConfig`

- `SHUB_ScanInitConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6814c to 0x68bc8.
  Established. Word stores write 0x7ffffd28 and 0x00602800 at entry sp minus 0xa4 and 0xa0 respectively (`str` at 0x681c8, `str` at 0x681d0).
  A halfword store writes 0x320 to absolute address 0x31a68c (`strh` at 0x682d8).
  A byte loaded from [sp-172] plus 0xc is stored as a byte at an unknown effective target (`ldrb` at 0x68534, `strb` at 0x68538).
  A halfword loaded from [sp-172] plus 8 is stored as a halfword at an unknown effective target (`ldrh` at 0x68658, `strh` at 0x6865c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6815c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x68160 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6816c | store | 4 | `sp -0xa4` | `0x0` |
| `str` at 0x68178 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x68184 | store | 4 | `sp -0x9c` | `0x0` |
| `str` at 0x68190 | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x6819c | store | 4 | `sp -0x94` | `0x0` |
| `str` at 0x681a8 | store | 4 | `sp -0x90` | `0x0` |
| `str` at 0x681b4 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x681c0 | store | 4 | `sp -0x88` | `0x0` |
| `ldr` at 0x681c4 | load | 4 | `abs 0x68b0c` | `-` |
| `str` at 0x681c8 | store | 4 | `sp -0xa4` | `0x7ffffd28` |
| `ldr` at 0x681cc | load | 4 | `abs 0x68b10` | `-` |
| `str` at 0x681d0 | store | 4 | `sp -0xa0` | `0x602800` |
| `ldr` at 0x681d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x681dc | load | 4 | `abs 0x68b14` | `-` |
| `ldr` at 0x681e0 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x681e8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x681ec | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x681f0 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x681f4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68200 | load | 4 | `abs 0x68b1c` | `-` |
| `str` at 0x68204 | store | 4 | `sp -0xc0` | `0x163988` |
| `ldr` at 0x68208 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6820c | store | 4 | `sp -0xbc` | `r0` |
| `ldr` at 0x68214 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68218 | load | 4 | `abs 0x68b24` | `-` |
| `str` at 0x68228 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x6822c | load | 4 | `abs 0x68b28` | `-` |
| `str` at 0x68230 | store | 4 | `sp -0xbc` | `0x1639a4` |
| `ldr` at 0x68234 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x68254 | load | 4 | `abs 0x68b2c` | `-` |
| `str` at 0x68258 | store | 4 | `sp -0xc0` | `0x1639b0` |
| `ldr` at 0x6825c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68260 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68268 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x6826c | load | 4 | `abs 0x68b30` | `-` |
| `ldr` at 0x68278 | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x6827c | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x68284 | load | 4 | `abs 0x68b38` | `-` |
| `ldrh` at 0x68288 | load | 2 | `abs 0x31a68c` | `-` |
| `ldr` at 0x68294 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x68298 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x682a4 | load | 4 | `abs 0x68b3c` | `-` |
| `str` at 0x682a8 | store | 4 | `sp -0xc0` | `0x1639e0` |
| `ldr` at 0x682ac | load | 4 | `abs 0x68b38` | `-` |
| `ldrh` at 0x682b0 | load | 2 | `abs 0x31a68c` | `-` |
| `str` at 0x682b4 | store | 4 | `sp -0xbc` | `[0x31a68c]:2` |
| `ldr` at 0x682bc | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x682c0 | load | 4 | `abs 0x68b40` | `-` |
| `ldr` at 0x682d0 | load | 4 | `abs 0x68b38` | `-` |
| `strh` at 0x682d8 | store | 2 | `abs 0x31a68c` | `0x320` |
| `ldr` at 0x682dc | load | 4 | `abs 0x68b44` | `-` |
| `ldrb` at 0x682e0 | load | 1 | `abs 0x1a5cb0` | `-` |
| `str` at 0x682f0 | store | 4 | `sp -0xb0` | `0x0` |
| `ldr` at 0x682f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x682fc | load | 4 | `abs 0x68b14` | `-` |
| `ldr` at 0x68300 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldrb` at 0x68308 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6830c | load | 4 | `abs 0x68b44` | `-` |
| `ldrb` at 0x68310 | load | 1 | `abs 0x1a5cb0` | `-` |
| `ldr` at 0x68318 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6832c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68330 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68350 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6835c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68360 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68380 | load | 1 | `(unknown)` | `-` |
| `str` at 0x68384 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x68388 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x6838c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68398 | load | 4 | `abs 0x68b4c` | `-` |
| `str` at 0x6839c | store | 4 | `sp -0xc0` | `0x1639f8` |
| `ldr` at 0x683a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x683a4 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x683a8 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x683ac | store | 4 | `sp -0xb8` | `[sp-176]` |
| `ldr` at 0x683b4 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x683b8 | load | 4 | `abs 0x68b50` | `-` |
| `ldr` at 0x683c8 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x683cc | load | 1 | `[sp-172] +0x6` | `-` |
| `str` at 0x683d0 | store | 4 | `sp -0xb0` | `[[sp-172]+6]:1` |
| `ldr` at 0x683d4 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x683d8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x683e4 | load | 4 | `abs 0x68b54` | `-` |
| `str` at 0x683e8 | store | 4 | `sp -0xc0` | `0x163a1c` |
| `ldr` at 0x683ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x683f0 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x683f4 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x683f8 | store | 4 | `sp -0xb8` | `[[sp-172]+6]:1` |
| `ldr` at 0x68400 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68404 | load | 4 | `abs 0x68b58` | `-` |
| `ldr` at 0x68410 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x68420 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x68438 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x6843c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6845c | load | 4 | `sp -0xb0` | `-` |
| `strb` at 0x68460 | store | 1 | `(unknown)` | `([sp-176] & 0xff)` |
| `ldr` at 0x68468 | load | 4 | `abs 0x68b5c` | `-` |
| `str` at 0x6846c | store | 4 | `sp -0xc0` | `0x163a34` |
| `ldr` at 0x68470 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68474 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x6847c | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68480 | load | 4 | `abs 0x68b60` | `-` |
| `ldr` at 0x6848c | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x68490 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x68498 | load | 4 | `abs 0x68b64` | `-` |
| `ldr` at 0x6849c | load | 4 | `abs 0x1a5cc8` | `-` |
| `ldr` at 0x684a8 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x684ac | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x684b8 | load | 4 | `abs 0x68b68` | `-` |
| `str` at 0x684bc | store | 4 | `sp -0xc0` | `0x163a88` |
| `ldr` at 0x684c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x684c4 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x684c8 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x684cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x684ec | load | 1 | `(unknown)` | `-` |
| `str` at 0x684f0 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x684f8 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x684fc | load | 4 | `abs 0x68b6c` | `-` |
| `ldr` at 0x6850c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68530 | load | 4 | `sp -0xac` | `-` |
| `ldrb` at 0x68534 | load | 1 | `[sp-172] +0xc` | `-` |
| `strb` at 0x68538 | store | 1 | `(unknown)` | `([[sp-172]+12]:1 & 0xff)` |
| `ldr` at 0x6853c | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x68540 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6854c | load | 4 | `abs 0x68b70` | `-` |
| `str` at 0x68550 | store | 4 | `sp -0xc0` | `0x163aac` |
| `ldr` at 0x68554 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68558 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x6855c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68560 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x68580 | load | 1 | `(unknown)` | `-` |
| `str` at 0x68584 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x6858c | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68590 | load | 4 | `abs 0x68b74` | `-` |
| `ldr` at 0x6859c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x685a0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x685c0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x685cc | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x685d0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x685dc | load | 4 | `abs 0x68b78` | `-` |
| `str` at 0x685e0 | store | 4 | `sp -0xc0` | `0x163ad0` |
| `ldr` at 0x685e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x685e8 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x685ec | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x685f0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x68610 | load | 2 | `(unknown)` | `-` |
| `str` at 0x68614 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x6861c | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68620 | load | 4 | `abs 0x68b7c` | `-` |
| `ldr` at 0x68630 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68634 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68654 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x68658 | load | 2 | `[sp-172] +0x8` | `-` |
| `strh` at 0x6865c | store | 2 | `(unknown)` | `([[sp-172]+8]:2 & 0xffff)` |
| `ldr` at 0x68660 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x68664 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68670 | load | 4 | `abs 0x68b80` | `-` |
| `str` at 0x68674 | store | 4 | `sp -0xc0` | `0x163afc` |
| `ldr` at 0x68678 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6867c | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68680 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68684 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x686a4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x686a8 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x686b0 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x686b4 | load | 4 | `abs 0x68b84` | `-` |
| `ldr` at 0x686c0 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x686c4 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x686e4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x686f0 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x686f4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68700 | load | 4 | `abs 0x68b88` | `-` |
| `str` at 0x68704 | store | 4 | `sp -0xc0` | `0x163b1c` |
| `ldr` at 0x68708 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6870c | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68710 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68714 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x68734 | load | 2 | `(unknown)` | `-` |
| `str` at 0x68738 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x68740 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68754 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68758 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68778 | load | 4 | `sp -0xac` | `-` |
| `ldrh` at 0x6877c | load | 2 | `[sp-172] +0xa` | `-` |
| `strh` at 0x68780 | store | 2 | `(unknown)` | `([[sp-172]+10]:2 & 0xffff)` |
| `ldr` at 0x68784 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x68788 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68794 | load | 4 | `abs 0x68b8c` | `-` |
| `str` at 0x68798 | store | 4 | `sp -0xc0` | `0x163b48` |
| `ldr` at 0x6879c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x687a0 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x687a4 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x687a8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x687c8 | load | 2 | `(unknown)` | `-` |
| `str` at 0x687cc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x687d4 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x687d8 | load | 4 | `abs 0x68b90` | `-` |
| `ldr` at 0x687e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x687fc | store | 4 | `sp -0xc0` | `0x38` |
| `ldr` at 0x68800 | load | 4 | `abs 0x68b94` | `-` |
| `str` at 0x68804 | store | 4 | `sp -0xbc` | `0x163b68` |
| `ldr` at 0x68808 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x68824 | load | 4 | `abs 0x68b98` | `-` |
| `str` at 0x68828 | store | 4 | `sp -0xc0` | `0x163b70` |
| `ldr` at 0x6882c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68830 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68838 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x6883c | load | 4 | `abs 0x68b9c` | `-` |
| `ldr` at 0x68848 | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x6884c | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x68854 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6888c | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x68894 | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68898 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x688b8 | load | 2 | `(unknown)` | `-` |
| `str` at 0x68914 | store | 4 | `sp -0xc0` | `0xf2` |
| `ldr` at 0x68918 | load | 4 | `abs 0x68ba0` | `-` |
| `str` at 0x6891c | store | 4 | `sp -0xbc` | `0x163b98` |
| `ldr` at 0x68920 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x6893c | load | 4 | `abs 0x68ba4` | `-` |
| `str` at 0x68940 | store | 4 | `sp -0xc0` | `0x163ba8` |
| `ldr` at 0x68944 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68948 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68950 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68960 | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x68964 | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x6896c | load | 4 | `abs 0x68b48` | `-` |
| `ldr` at 0x68970 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x68990 | load | 2 | `(unknown)` | `-` |
| `str` at 0x689ec | store | 4 | `sp -0xc0` | `0xf2` |
| `ldr` at 0x689f0 | load | 4 | `abs 0x68ba8` | `-` |
| `str` at 0x689f4 | store | 4 | `sp -0xbc` | `0x163bd8` |
| `ldr` at 0x689f8 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x68a14 | load | 4 | `abs 0x68bac` | `-` |
| `str` at 0x68a18 | store | 4 | `sp -0xc0` | `0x163be8` |
| `ldr` at 0x68a1c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68a20 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68a28 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68a2c | load | 4 | `abs 0x68bb0` | `-` |
| `ldr` at 0x68a38 | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x68a3c | store | 4 | `sp -0xb4` | `0xfffff887` |
| `str` at 0x68a48 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0x68a4c | load | 4 | `abs 0x68bb4` | `-` |
| `str` at 0x68a50 | store | 4 | `sp -0xbc` | `0x163c18` |
| `ldr` at 0x68a54 | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x68a74 | load | 4 | `abs 0x68bb8` | `-` |
| `str` at 0x68a78 | store | 4 | `sp -0xc0` | `0x163c24` |
| `ldr` at 0x68a7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68a80 | store | 4 | `sp -0xbc` | `[sp-20]` |
| `ldr` at 0x68a88 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68a8c | load | 4 | `abs 0x68bbc` | `-` |
| `ldr` at 0x68a98 | load | 4 | `abs 0x68b34` | `-` |
| `str` at 0x68a9c | store | 4 | `sp -0xb4` | `0xfffff887` |
| `ldr` at 0x68aa4 | load | 4 | `abs 0x68b18` | `-` |
| `ldr` at 0x68aa8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68ab4 | load | 4 | `abs 0x68bc0` | `-` |
| `str` at 0x68ab8 | store | 4 | `sp -0xc0` | `0x163c50` |
| `ldr` at 0x68ac0 | load | 4 | `abs 0x68b20` | `-` |
| `ldr` at 0x68ac4 | load | 4 | `abs 0x68bc4` | `-` |
| `str` at 0x68afc | store | 4 | `sp -0xb4` | `0x0` |
| `ldr` at 0x68b00 | load | 4 | `sp -0xb4` | `-` |
| `ldm` at 0x68b08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x68b08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x68b08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68220 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x4c2` | `0x0` |
| `bl` at 0x68244 | `(unknown callee) 0x13d9cc` | `[sp-168]` | `0x7c` | `0x0` | `0x0` |
| `bl` at 0x68274 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x4ca` | `0x0` |
| `bl` at 0x682c8 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x4d1` | `0x0` |
| `bl` at 0x68328 | `(unknown callee) 0x66ae0` | `[sp-20]` | `?` | `[0x1a5cb0]:1` | `sp-172` |
| `bl` at 0x683c0 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x4ee` | `0x0` |
| `bl` at 0x6840c | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x4f1` | `0x0` |
| `bl` at 0x68488 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x4fd` | `0x0` |
| `bl` at 0x68504 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x507` | `0x0` |
| `bl` at 0x68598 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x50a` | `0x0` |
| `bl` at 0x68628 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x513` | `0x0` |
| `bl` at 0x686bc | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x517` | `0x0` |
| `bl` at 0x6874c | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x520` | `0x0` |
| `bl` at 0x687e0 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x524` | `0x0` |
| `bl` at 0x687f0 | `(unknown callee) 0x67ab8` | `[sp-20]` | `sp-132` | `?` | `sp-132` |
| `bl` at 0x68814 | `(unknown callee) 0x13d9cc` | `[sp-168]` | `0x7e` | `sp-132` | `0x0` |
| `bl` at 0x68844 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x531` | `0x0` |
| `bl` at 0x68870 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x68884 | `(unknown callee) 0x12cee4` | `0x0` | `0x0` | `0x20` | `sp-124` |
| `bl` at 0x688d4 | `(unknown callee) 0x131ebc` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x688e4 | `(unknown callee) 0x131ebc` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x688f8 | `(unknown callee) 0x131f34` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x68908 | `(unknown callee) 0x131f34` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x6892c | `(unknown callee) 0x13d9cc` | `[sp-168]` | `0x1a` | `sp-164` | `0x0` |
| `bl` at 0x6895c | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x550` | `0x0` |
| `bl` at 0x689ac | `(unknown callee) 0x131ebc` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x689bc | `(unknown callee) 0x131ebc` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x689d0 | `(unknown callee) 0x131f34` | `0x29` | `sp-164` | `[sp-20]` | `sp-164` |
| `bl` at 0x689e0 | `(unknown callee) 0x131f34` | `0x2a` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x68a04 | `(unknown callee) 0x13d9cc` | `[sp-168]` | `0x1b` | `sp-164` | `0x0` |
| `bl` at 0x68a34 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x55d` | `0x0` |
| `bl` at 0x68a64 | `(unknown callee) 0x13d9cc` | `[sp-168]` | `0x7d` | `0x0` | `0x0` |
| `bl` at 0x68a94 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x567` | `0x0` |
| `bl` at 0x68acc | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x56a` | `0x0` |
| `bl` at 0x68ae0 | `(unknown callee) 0x12cc8c` | `0x1` | `0x0` | `sp-124` | `sp-124` |
| `bl` at 0x68af4 | `(unknown callee) 0x12cee4` | `0x0` | `0x0` | `0x20` | `sp-124` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x681fc | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x68250 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x68290 | `[0x31a68c]:2 == 0x0` | `[0x31a68c]:2 != 0x0` |
| `beq` at 0x682a0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x682e8 | `[0x1a5cb0]:1 == 0x0` | `[0x1a5cb0]:1 != 0x0` |
| `beq` at 0x68358 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x68394 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x683e0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bhi` at 0x68418 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x68430 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x684a4 | `[0x1a5cc8] == 0x0` | `[0x1a5cc8] != 0x0` |
| `beq` at 0x684b4 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x68548 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x685c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x685d8 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6866c | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x686ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0x686fc | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x68790 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x68820 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `bne` at 0x6885c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `beq` at 0x688c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x68938 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x6899c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x68a10 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x68a70 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x68ab0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x68b08 | `r0` | `?` |

## `SHUB_Scan_IDCODE`

- `SHUB_Scan_IDCODE`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68bc8 to 0x68fb0.
  Established. The call through an unknown target has printed r0-r3 candidates unknown, unknown, [0x1a0874] and unknown (`mov` at 0x68c44).
  A word store writes 0xfffff886 at entry sp minus 0x6c (`str` at 0x68c88).
  Separate computed word stores have unknown values and unknown effective targets (`str` at 0x68d50, `str` at 0x68d90).
  The result from unnamed 0x68fb0 is later stored as a word at entry sp minus 0x70 (`bl` at 0x68edc, `str` at 0x68ef4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68bd8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x68be0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x68be4 | load | 4 | `abs 0x68f58` | `-` |
| `ldr` at 0x68be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68bf0 | load | 4 | `abs 0x68f5c` | `-` |
| `ldr` at 0x68bf4 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x68bfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68c1c | load | 4 | `abs 0x68f5c` | `-` |
| `ldr` at 0x68c20 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x68c28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68c38 | load | 4 | `abs 0x1a0874` | `-` |
| `ldr` at 0x68c68 | load | 4 | `abs 0x68f60` | `-` |
| `str` at 0x68c6c | store | 4 | `sp -0x78` | `0x163c7c` |
| `ldr` at 0x68c74 | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68c78 | load | 4 | `abs 0x68f68` | `-` |
| `ldr` at 0x68c84 | load | 4 | `abs 0x68f6c` | `-` |
| `str` at 0x68c88 | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x68c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68c98 | load | 4 | `abs 0x68f5c` | `-` |
| `ldr` at 0x68c9c | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x68ca4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68cac | load | 4 | `abs 0x68f70` | `-` |
| `str` at 0x68cb8 | store | 4 | `sp -0x78` | `0x20` |
| `ldr` at 0x68cbc | load | 4 | `abs 0x68f74` | `-` |
| `str` at 0x68cc0 | store | 4 | `sp -0x74` | `0x163c94` |
| `ldr` at 0x68cc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68ce4 | load | 4 | `abs 0x68f78` | `-` |
| `str` at 0x68ce8 | store | 4 | `sp -0x78` | `0x163ca4` |
| `ldr` at 0x68cec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68cf0 | store | 4 | `sp -0x74` | `[sp-20]` |
| `ldr` at 0x68cf8 | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68cfc | load | 4 | `abs 0x68f7c` | `-` |
| `ldr` at 0x68d08 | load | 4 | `abs 0x68f6c` | `-` |
| `str` at 0x68d0c | store | 4 | `sp -0x6c` | `0xfffff886` |
| `ldr` at 0x68d14 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d1c | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68d28 | load | 4 | `abs 0x68f80` | `-` |
| `ldr` at 0x68d34 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68d44 | load | 4 | `abs 0x68f80` | `-` |
| `str` at 0x68d50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x68d54 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d5c | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68d68 | load | 4 | `abs 0x68f84` | `-` |
| `ldr` at 0x68d74 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68d84 | load | 4 | `abs 0x68f84` | `-` |
| `str` at 0x68d90 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x68d94 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68d98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68d9c | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68da4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68da8 | load | 4 | `abs 0x68f88` | `-` |
| `ldr` at 0x68db4 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68db8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68dbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68dc4 | load | 4 | `abs 0x68f88` | `-` |
| `str` at 0x68dd0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x68dd4 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68dd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68ddc | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68de0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68de4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68df4 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68dfc | load | 4 | `(unknown)` | `-` |
| `str` at 0x68e10 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x68e14 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68e18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e1c | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68e20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68e34 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e3c | load | 4 | `(unknown)` | `-` |
| `str` at 0x68e50 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x68e54 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x68e64 | load | 4 | `abs 0x68f8c` | `-` |
| `ldr` at 0x68e70 | load | 4 | `abs 0x68f90` | `-` |
| `str` at 0x68e74 | store | 4 | `sp -0x78` | `0x163cd4` |
| `ldr` at 0x68e7c | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68e80 | load | 4 | `abs 0x68f94` | `-` |
| `ldr` at 0x68e8c | load | 4 | `abs 0x68f98` | `-` |
| `str` at 0x68e90 | store | 4 | `sp -0x78` | `0x163d10` |
| `ldr` at 0x68e94 | load | 4 | `abs 0x68f70` | `-` |
| `ldr` at 0x68e98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e9c | load | 4 | `(unknown)` | `-` |
| `str` at 0x68ea4 | store | 4 | `sp -0x74` | `?` |
| `ldr` at 0x68eac | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68eb0 | load | 4 | `abs 0x68f9c` | `-` |
| `ldr` at 0x68ebc | load | 4 | `abs 0x68f6c` | `-` |
| `str` at 0x68ec0 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x68ec8 | load | 4 | `abs 0x68fa0` | `-` |
| `ldr` at 0x68ecc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x68ed8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68ee4 | load | 4 | `abs 0x68fa4` | `-` |
| `str` at 0x68ee8 | store | 4 | `sp -0x78` | `0x163d40` |
| `ldr` at 0x68eec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68ef0 | store | 4 | `sp -0x74` | `[sp-20]` |
| `str` at 0x68ef4 | store | 4 | `sp -0x70` | `ret(0x68fb0)` |
| `ldr` at 0x68efc | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68f00 | load | 4 | `abs 0x68fa8` | `-` |
| `ldr` at 0x68f0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68f20 | load | 4 | `abs 0x68fac` | `-` |
| `str` at 0x68f24 | store | 4 | `sp -0x78` | `0x163d50` |
| `ldr` at 0x68f2c | load | 4 | `abs 0x68f64` | `-` |
| `ldr` at 0x68f3c | load | 4 | `abs 0x68f6c` | `-` |
| `str` at 0x68f40 | store | 4 | `sp -0x18` | `0xfffff886` |
| `ldr` at 0x68f44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x68f48 | store | 4 | `sp -0x6c` | `[sp-24]` |
| `ldr` at 0x68f4c | load | 4 | `sp -0x6c` | `-` |
| `ldm` at 0x68f54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x68f54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x68f54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x68c44 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x1a0874]` | `?` |
| `bl` at 0x68c58 | `(unknown callee) 0x12cdbc` | `0x0` | `0x0` | `sp-104` | `sp-104` |
| `bl` at 0x68c80 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x585` | `0x0` |
| `bl` at 0x68cd4 | `(unknown callee) 0x13d9cc` | `?` | `0x3` | `0x0` | `?` |
| `bl` at 0x68d04 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x58b` | `0x0` |
| `bl` at 0x68e88 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x595` | `0x0` |
| `bl` at 0x68eb8 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x597` | `0x0` |
| `bl` at 0x68edc | `(unknown callee) 0x68fb0` | `[sp-20]` | `?` | `?` | `[0x315034]` |
| `bl` at 0x68f08 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x59b` | `0x0` |
| `bl` at 0x68f10 | `(unknown callee) 0x68fb0` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0x68f38 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x5a0` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x68c64 | `ret(0x12cdbc) != 0x0` | `ret(0x12cdbc) == 0x0` |
| `beq` at 0x68ce0 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x68e6c | `? == 0x4363049` | `? != 0x4363049` |
| `beq` at 0x68ed4 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bgt` at 0x68f1c | `ret(0x68fb0) > 0x1 (signed, 32-bit)` | `ret(0x68fb0) <= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x68f54 | `r0` | `?` |

## `SHUB_Rev`

- `SHUB_Rev`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68fb0 to 0x68fe4.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x68fc0).
  The computed word load has an unknown effective address (`ldr` at 0x68fcc).
  The return record retains an unknown value in r0 (`ldm` at 0x68fdc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68fc0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x68fc4 | load | 4 | `abs 0x68fe0` | `-` |
| `ldr` at 0x68fc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68fcc | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x68fdc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x68fdc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x68fdc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x68fdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x68fdc | `r0` | `?` |

## `SHUB_ScanSetBusratio`

- `SHUB_ScanSetBusratio`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68fe4 to 0x693b0.
  Established. A halfword store writes 0 at entry sp minus 0x22 (`strh` at 0x68ffc).
  The result from unnamed 0x120340 is stored as a word at entry sp minus 0x1c (`bl` at 0x69028, `str` at 0x69030).
  A later byte store at entry sp minus 0x23 has an unknown value after unsupported smull and rsb effects (`smull` at 0x692d8, `rsb` at 0x692e4, `rsb` at 0x692e8, `strb` at 0x692ec).
  The return record places 0 in r0 (`ldm` at 0x69368).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68ff4 | store | 4 | `sp -0x14` | `r0` |
| `strh` at 0x68ffc | store | 2 | `sp -0x22` | `0x0` |
| `ldr` at 0x69000 | load | 4 | `abs 0x6936c` | `-` |
| `ldr` at 0x69004 | load | 4 | `abs 0x1a5cd0` | `-` |
| `str` at 0x69014 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x69030 | store | 4 | `sp -0x1c` | `ret(0x120340)` |
| `ldr` at 0x69034 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x69040 | load | 4 | `abs 0x69370` | `-` |
| `str` at 0x69044 | store | 4 | `sp -0x38` | `0x163d90` |
| `ldr` at 0x6904c | load | 4 | `abs 0x69374` | `-` |
| `ldr` at 0x69050 | load | 4 | `abs 0x69378` | `-` |
| `str` at 0x69060 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x69068 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69070 | load | 4 | `abs 0x6937c` | `-` |
| `ldr` at 0x69074 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldrb` at 0x6907c | load | 1 | `(unknown)` | `-` |
| `str` at 0x6908c | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x69098 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6909c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x690a4 | load | 4 | `abs 0x6937c` | `-` |
| `ldr` at 0x690a8 | load | 4 | `abs 0x1a5cac` | `-` |
| `ldrb` at 0x690b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x690c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x690cc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x690d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x690e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x690e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x69118 | load | 4 | `abs 0x6937c` | `-` |
| `ldr` at 0x6911c | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x69124 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x69128 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x69154 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69168 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6917c | load | 4 | `abs 0x69380` | `-` |
| `ldr` at 0x69180 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6918c | load | 4 | `abs 0x69384` | `-` |
| `str` at 0x69190 | store | 4 | `sp -0x38` | `0x163dac` |
| `ldr` at 0x69194 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x69198 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x6919c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x691a0 | store | 4 | `sp -0x30` | `[sp-24]` |
| `ldr` at 0x691a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x691b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x691c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x691c4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x691cc | load | 4 | `abs 0x69374` | `-` |
| `ldr` at 0x691d0 | load | 4 | `abs 0x69388` | `-` |
| `ldrh` at 0x691dc | load | 2 | `sp -0x22` | `-` |
| `ldrh` at 0x691e8 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x691ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69200 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69208 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x69218 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6922c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x69234 | load | 4 | `(unknown)` | `-` |
| `strh` at 0x69238 | store | 2 | `sp -0x22` | `?` |
| `ldr` at 0x6923c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69244 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldrh` at 0x6924c | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x69258 | load | 4 | `abs 0x69380` | `-` |
| `ldr` at 0x6925c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x69268 | load | 4 | `abs 0x6938c` | `-` |
| `str` at 0x6926c | store | 4 | `sp -0x38` | `0x163dc8` |
| `ldr` at 0x69270 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x69274 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x6927c | load | 4 | `abs 0x69374` | `-` |
| `ldr` at 0x69280 | load | 4 | `abs 0x69390` | `-` |
| `str` at 0x69290 | store | 4 | `sp -0x28` | `0x0` |
| `ldrh` at 0x69298 | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x6929c | load | 4 | `abs 0x69394` | `-` |
| `ldr` at 0x692a8 | load | 4 | `abs 0x69398` | `-` |
| `str` at 0x692ac | store | 4 | `sp -0x38` | `0x163dfc` |
| `ldr` at 0x692b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x692b4 | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldr` at 0x692bc | load | 4 | `abs 0x69374` | `-` |
| `ldr` at 0x692c0 | load | 4 | `abs 0x6939c` | `-` |
| `ldrh` at 0x692cc | load | 2 | `sp -0x22` | `-` |
| `ldr` at 0x692d4 | load | 4 | `abs 0x693a0` | `-` |
| `strb` at 0x692ec | store | 1 | `sp -0x23` | `?` |
| `ldr` at 0x692f0 | load | 4 | `abs 0x69380` | `-` |
| `ldr` at 0x692f4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x69300 | load | 4 | `abs 0x693a4` | `-` |
| `str` at 0x69304 | store | 4 | `sp -0x38` | `0x163e3c` |
| `ldr` at 0x69308 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6930c | store | 4 | `sp -0x34` | `[sp-20]` |
| `ldrb` at 0x69310 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x69314 | store | 4 | `sp -0x30` | `[sp-35]:1` |
| `ldr` at 0x6931c | load | 4 | `abs 0x69374` | `-` |
| `ldr` at 0x69320 | load | 4 | `abs 0x693a8` | `-` |
| `ldr` at 0x6932c | load | 4 | `abs 0x693ac` | `-` |
| `ldr` at 0x69330 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x69350 | load | 1 | `sp -0x23` | `-` |
| `strb` at 0x69354 | store | 1 | `(unknown)` | `([sp-35]:1 & 0xff)` |
| `str` at 0x6935c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x69360 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x69368 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69368 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69028 | `(unknown callee) 0x120340` | `0x0` | `sp-32` | `r2` | `sp-32` |
| `bl` at 0x69058 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x5c7` | `0x0` |
| `bl` at 0x690dc | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `[sp-24]` |
| `bl` at 0x69100 | `(unknown callee) 0x7fe94` | `ret(0x6e190)` | `?` | `[sp-24]` | `?` |
| `bl` at 0x69140 | `(unknown callee) 0x120398` | `0x0` | `?` | `[sp-24]` | `?` |
| `bl` at 0x691d8 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x5d7` | `0x0` |
| `bl` at 0x69288 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x5e1` | `0x0` |
| `bl` at 0x692c8 | `(unknown callee) 0x22070` | `0x4` | `0x16320c` | `0x5e7` | `0x0` |
| `bl` at 0x69328 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x5ea` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6900c | `[0x1a5cd0] == 0x0` | `[0x1a5cd0] != 0x0` |
| `bne` at 0x6903c | `ret(0x120340) != 0x0` | `ret(0x120340) == 0x0` |
| `bne` at 0x69084 | `? != 0xff` | `? == 0xff` |
| `bgt` at 0x690d8 | `[sp-24] > 0x1 (signed, 32-bit)` | `[sp-24] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6910c | `ret(0x7fe94) == 0x0` | `ret(0x7fe94) != 0x0` |
| `bne` at 0x6914c | `ret(0x120398) != 0x0` | `ret(0x120398) == 0x0` |
| `bne` at 0x69178 | `? != 0x3` | `? == 0x3` |
| `beq` at 0x69188 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x691e4 | `[sp-34]:2 == 0x0` | `[sp-34]:2 != 0x0` |
| `bge` at 0x69210 | `[sp-34]:2 >= ? (signed, 32-bit)` | `[sp-34]:2 < ? (signed, 32-bit)` |
| `bne` at 0x69254 | `[sp-34]:2 != 0x0` | `[sp-34]:2 == 0x0` |
| `beq` at 0x69264 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bls` at 0x692a4 | `[sp-34]:2 <= 0x8fc (unsigned, 32-bit)` | `[sp-34]:2 > 0x8fc (unsigned, 32-bit)` |
| `beq` at 0x692fc | `[0x315034] == 0x0` | `[0x315034] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69368 | `r0` | `0x0` |

## `SHUB_SetClkstop`

- `SHUB_SetClkstop`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x693b0 to 0x69420.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x693c0).
  A separate word store writes 0 at entry sp minus 0x18 (`str` at 0x693c8).
  The computed byte store has an unknown value and unknown effective target (`strb` at 0x69400).
  A load, addition and word store write [sp-24] plus 1 at entry sp minus 0x18 (`ldr` at 0x69404, `add` at 0x69408, `str` at 0x6940c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x693c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x693c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x693cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x693d8 | load | 4 | `abs 0x6941c` | `-` |
| `ldr` at 0x693dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x693fc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x69400 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x69404 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6940c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x69418 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69418 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x693d4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69418 | `r0` | `?` |

## `SHUB_GetClkstop`

- `SHUB_GetClkstop`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69420 to 0x69440.
  Established. A word load from absolute address 0x6943c places 0x31a614 in r3 (`ldr` at 0x6942c).
  A byte load reads absolute address 0x31a629 (`ldrb` at 0x69430).
  The return record places that byte value in r0 (`ldrb` at 0x69430, `ldm` at 0x69438).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6942c | load | 4 | `abs 0x6943c` | `-` |
| `ldrb` at 0x69430 | load | 1 | `abs 0x31a629` | `-` |
| `ldm` at 0x69438 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x69438 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x69438 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x69438 | `r0` | `[0x31a629]:1` |

## `SHUB_SetTP`

- `SHUB_SetTP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69440 to 0x694b0.
  Established. A word store writes 0 at entry sp minus 0x18 (`str` at 0x69458).
  The computed byte store has an unknown effective target and unknown value (`strb` at 0x69490).
  A load, addition and word store write [sp-24] plus 1 at entry sp minus 0x18 (`ldr` at 0x69494, `add` at 0x69498, `str` at 0x6949c).
  The return record retains an unknown value in r0 (`ldm` at 0x694a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x69450 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x69458 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6945c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69468 | load | 4 | `abs 0x694ac` | `-` |
| `ldr` at 0x6946c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6948c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x69490 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x69494 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6949c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x694a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x694a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x694a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x69464 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x694a8 | `r0` | `?` |

## `SHUB_GetTP`

- `SHUB_GetTP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x694b0 to 0x694d0.
  Established. A word load from absolute address 0x694cc places 0x31a614 in r3 (`ldr` at 0x694bc).
  A separate byte load reads absolute address 0x31a628 (`ldrb` at 0x694c0).
  The return record places that byte value in r0 (`ldrb` at 0x694c0, `ldm` at 0x694c8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x694bc | load | 4 | `abs 0x694cc` | `-` |
| `ldrb` at 0x694c0 | load | 1 | `abs 0x31a628` | `-` |
| `ldm` at 0x694c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x694c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x694c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x694c8 | `r0` | `[0x31a628]:1` |

## `SHUB_ScanReadMCILockStatus`

- `SHUB_ScanReadMCILockStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x69d80 to 0x6a1fc.
  Established. A byte store preserves the low eight bits of entry r0 at entry sp minus 0x11 (`strb` at 0x69d94).
  Word stores write 0xfffff890 at entry sp minus 0x4c after the corresponding beq falls through for a nonzero call result (`bl` at 0x69e28, `beq` at 0x69e34, `str` at 0x69e60, `bl` at 0x69ea8, `beq` at 0x69eb4, `str` at 0x69ee0).
  Later halfword stores at entry sp minus 0x42 and 0x44 have unknown values (`strh` at 0x6a12c, `strh` at 0x6a158).
  A final word store writes 0 at entry sp minus 0x4c (`str` at 0x6a19c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x69d94 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x69d9c | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x69da0 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x69da8 | load | 4 | `abs 0x6a1ac` | `-` |
| `ldr` at 0x69dac | load | 4 | `abs 0x1a5cac` | `-` |
| `ldr` at 0x69db4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x69db8 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x69dbc | load | 4 | `abs 0x6a1b0` | `-` |
| `ldr` at 0x69dc0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x69dcc | load | 4 | `abs 0x6a1b4` | `-` |
| `str` at 0x69dd0 | store | 4 | `sp -0x5c` | `0x163eb4` |
| `ldr` at 0x69dd8 | load | 4 | `abs 0x6a1b8` | `-` |
| `ldr` at 0x69ddc | load | 4 | `abs 0x6a1bc` | `-` |
| `ldrb` at 0x69df0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x69e10 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x69e14 | load | 4 | `abs 0x6a1c0` | `-` |
| `str` at 0x69e18 | store | 4 | `sp -0x58` | `0x163ed4` |
| `ldr` at 0x69e1c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x69e38 | load | 4 | `abs 0x6a1c4` | `-` |
| `str` at 0x69e3c | store | 4 | `sp -0x5c` | `0x163edc` |
| `ldrb` at 0x69e40 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x69e44 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x69e4c | load | 4 | `abs 0x6a1b8` | `-` |
| `ldr` at 0x69e50 | load | 4 | `abs 0x6a1c8` | `-` |
| `ldr` at 0x69e5c | load | 4 | `abs 0x6a1cc` | `-` |
| `str` at 0x69e60 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x69e68 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x69e90 | store | 4 | `sp -0x5c` | `0x4c` |
| `ldr` at 0x69e94 | load | 4 | `abs 0x6a1c0` | `-` |
| `str` at 0x69e98 | store | 4 | `sp -0x58` | `0x163ed4` |
| `ldr` at 0x69e9c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x69eb8 | load | 4 | `abs 0x6a1c4` | `-` |
| `str` at 0x69ebc | store | 4 | `sp -0x5c` | `0x163edc` |
| `ldrb` at 0x69ec0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x69ec4 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x69ecc | load | 4 | `abs 0x6a1b8` | `-` |
| `ldr` at 0x69ed0 | load | 4 | `abs 0x6a1d0` | `-` |
| `ldr` at 0x69edc | load | 4 | `abs 0x6a1cc` | `-` |
| `str` at 0x69ee0 | store | 4 | `sp -0x4c` | `0xfffff890` |
| `ldrb` at 0x69ee8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x69efc | load | 4 | `abs 0x6a1d4` | `-` |
| `ldrb` at 0x69f00 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x69f18 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x69f1c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x69f20 | load | 4 | `abs 0x6a1d8` | `-` |
| `ldrb` at 0x69f24 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x69f3c | load | 2 | `(unknown)` | `-` |
| `strh` at 0x69f40 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x69f44 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x69f48 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x69f50 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x69f54 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x69f60 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x69f64 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x69f70 | load | 4 | `abs 0x6a1dc` | `-` |
| `ldrb` at 0x69f7c | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x69f80 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x69f84 | load | 4 | `abs 0x6a1e0` | `-` |
| `str` at 0x69f94 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x69f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x69fa4 | load | 4 | `abs 0x6a1b0` | `-` |
| `ldr` at 0x69fa8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x69fb4 | load | 4 | `abs 0x6a1e4` | `-` |
| `str` at 0x69fb8 | store | 4 | `sp -0x5c` | `0x163f44` |
| `ldrb` at 0x69fbc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x69fc0 | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x69fc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x69fc8 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x69fcc | load | 2 | `sp -0x42` | `-` |
| `str` at 0x69fd4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x69fdc | load | 4 | `abs 0x6a1b8` | `-` |
| `ldr` at 0x69fe0 | load | 4 | `abs 0x6a1e8` | `-` |
| `ldrh` at 0x69fec | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x69ff4 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x69ff8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a000 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6a008 | load | 4 | `abs 0x6a1d4` | `-` |
| `ldrb` at 0x6a00c | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x6a024 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x6a028 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x6a02c | load | 4 | `abs 0x6a1d8` | `-` |
| `ldrb` at 0x6a030 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x6a048 | load | 2 | `(unknown)` | `-` |
| `strh` at 0x6a04c | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x6a050 | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x6a054 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x6a05c | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x6a060 | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x6a06c | load | 4 | `abs 0x6a1dc` | `-` |
| `ldrb` at 0x6a078 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0x6a07c | load | 2 | `sp -0x42` | `-` |
| `ldr` at 0x6a080 | load | 4 | `abs 0x6a1ec` | `-` |
| `str` at 0x6a090 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x6a094 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6a0a0 | load | 4 | `abs 0x6a1b0` | `-` |
| `ldr` at 0x6a0a4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x6a0b0 | load | 4 | `abs 0x6a1f0` | `-` |
| `str` at 0x6a0b4 | store | 4 | `sp -0x5c` | `0x163f9c` |
| `ldrb` at 0x6a0b8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x6a0bc | store | 4 | `sp -0x58` | `[sp-17]:1` |
| `ldr` at 0x6a0c0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a0c4 | store | 4 | `sp -0x54` | `?` |
| `ldrh` at 0x6a0c8 | load | 2 | `sp -0x42` | `-` |
| `str` at 0x6a0d0 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x6a0d8 | load | 4 | `abs 0x6a1b8` | `-` |
| `ldr` at 0x6a0dc | load | 4 | `abs 0x6a1f4` | `-` |
| `ldrh` at 0x6a0e8 | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x6a0f0 | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x6a0f4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6a0fc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x6a104 | load | 4 | `abs 0x6a1d4` | `-` |
| `ldrb` at 0x6a108 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6a128 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x6a12c | store | 2 | `sp -0x42` | `?` |
| `ldr` at 0x6a130 | load | 4 | `abs 0x6a1d8` | `-` |
| `ldrb` at 0x6a134 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x6a154 | load | 1 | `(unknown)` | `-` |
| `strh` at 0x6a158 | store | 2 | `sp -0x44` | `?` |
| `ldrh` at 0x6a15c | load | 2 | `sp -0x42` | `-` |
| `ldrh` at 0x6a160 | load | 2 | `sp -0x44` | `-` |
| `strh` at 0x6a168 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x6a16c | load | 2 | `sp -0x42` | `-` |
| `strh` at 0x6a178 | store | 2 | `sp -0x42` | `?` |
| `ldrh` at 0x6a17c | load | 2 | `sp -0x42` | `-` |
| `ldrb` at 0x6a188 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x6a18c | load | 4 | `abs 0x6a1f8` | `-` |
| `str` at 0x6a19c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x6a1a0 | load | 4 | `sp -0x4c` | `-` |
| `ldm` at 0x6a1a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a1a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a1a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x69de4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x6be` | `0x0` |
| `bl` at 0x69dec | `(unknown callee) 0x65d64` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x69e00 | `(unknown callee) 0x694d0` | `[sp-17]:1` | `sp-48` | `sp-48` | `[sp-17]:1` |
| `bl` at 0x69e28 | `(unknown callee) 0x13d9cc` | `[sp-72]` | `0x7` | `sp-48` | `sp-64` |
| `bl` at 0x69e58 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x6cb` | `0x0` |
| `bl` at 0x69e78 | `(unknown callee) 0x698d8` | `[sp-17]:1` | `0x1` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x69e80 | `(unknown callee) 0x65d64` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x69ea8 | `(unknown callee) 0x13d9cc` | `[sp-72]` | `?` | `sp-48` | `sp-64` |
| `bl` at 0x69ed8 | `(unknown callee) 0x22070` | `0x3` | `0x16320c` | `0x6de` | `0x0` |
| `bl` at 0x69ef8 | `(unknown callee) 0x698d8` | `[sp-17]:1` | `0x2` | `sp-64` | `[sp-17]:1` |
| `bl` at 0x69f8c | `(unknown callee) 0x29908` | `0x163f08` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x69fe8 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x6f2` | `0x0` |
| `bl` at 0x6a088 | `(unknown callee) 0x29908` | `0x163f60` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |
| `bl` at 0x6a0e4 | `(unknown callee) 0x22070` | `0x6` | `0x16320c` | `0x6fc` | `0x0` |
| `bl` at 0x6a194 | `(unknown callee) 0x29908` | `0x163fb8` | `[sp-17]:1` | `[sp-66]:2` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x69dc8 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x69e34 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x69eb4 | `ret(0x13d9cc) == 0x0` | `ret(0x13d9cc) != 0x0` |
| `beq` at 0x69f78 | `? == 0x1ff` | `? != 0x1ff` |
| `bgt` at 0x69fa0 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x69fb0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a074 | `? == 0x1ff` | `? != 0x1ff` |
| `bgt` at 0x6a09c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x6a0ac | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x6a184 | `[sp-66]:2 == 0x1` | `[sp-66]:2 != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a1a8 | `r0` | `?` |

## `SHUB_Reset_Hold`

- `SHUB_Reset_Hold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6a1fc to 0x6a220.
  Established. A byte store preserves the low eight bits of entry r0 at entry sp minus 0x11 (`strb` at 0x6a210).
  The final ldm reads words at entry sp minus 0x14, 0x10, 0xc and 8 (`ldm` at 0x6a21c).
  The return record places 0 in r0 (`ldm` at 0x6a21c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6a210 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldm` at 0x6a21c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x6a21c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x6a21c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x6a21c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x6a21c | `r0` | `0x0` |
