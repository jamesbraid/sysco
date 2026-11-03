# ARM Linux 2.6 temperature entry paths and monitor initialization in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-temperature-monitor-init-facts.txt) retain all tool sections.

## `tempInit`

- `tempInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72138 to 0x72398.
  Established. Word stores write ret(0x6e140) to absolute 0x1a6204 and ret(0x204d8) to absolute 0x1a6200 (`str` at 0x721c8, `str` at 0x721fc).
  A byte store writes ([sp-32] & 0xff) at [sp-36] +4, and a later word store writes another ret(0x204d8) at [sp-36] +8 (`strb` at 0x72268, `str` at 0x72288).
  The later call to unnamed 0x204d8 has candidate r0/r1/r2/r3 values [[sp-36]+4]:1/4/[sp-36]/[[sp-36]+4]:1 (`bl` at 0x72280).
  The final byte store writes 1 at an unknown effective address, leaving the initialized software field unidentified (`strb` at 0x72364).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72148 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7214c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x72150 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x72154 | load | 4 | `abs 0x72380` | `-` |
| `ldr` at 0x72158 | load | 4 | `abs 0x1a6200` | `-` |
| `ldr` at 0x7216c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72178 | load | 4 | `abs 0x72384` | `-` |
| `str` at 0x7217c | store | 4 | `sp -0x40` | `0x165330` |
| `ldr` at 0x72180 | load | 4 | `abs 0x72380` | `-` |
| `ldr` at 0x72184 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72188 | store | 4 | `sp -0x3c` | `[0x1a6200]` |
| `ldr` at 0x7218c | load | 4 | `abs 0x72388` | `-` |
| `ldr` at 0x72190 | load | 4 | `abs 0x1a6204` | `-` |
| `str` at 0x72194 | store | 4 | `sp -0x38` | `[0x1a6204]` |
| `ldr` at 0x7219c | load | 4 | `abs 0x7238c` | `-` |
| `ldr` at 0x721ac | load | 4 | `abs 0x72388` | `-` |
| `ldr` at 0x721b0 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x721c4 | load | 4 | `abs 0x72388` | `-` |
| `str` at 0x721c8 | store | 4 | `abs 0x1a6204` | `ret(0x6e140)` |
| `ldr` at 0x721cc | load | 4 | `abs 0x72388` | `-` |
| `ldr` at 0x721d0 | load | 4 | `abs 0x1a6204` | `-` |
| `str` at 0x721e4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x721e8 | load | 4 | `abs 0x72380` | `-` |
| `ldr` at 0x721ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x721fc | store | 4 | `abs 0x1a6200` | `ret(0x204d8)` |
| `ldr` at 0x72200 | load | 4 | `abs 0x1a6200` | `-` |
| `ldr` at 0x7220c | load | 4 | `abs 0x72390` | `-` |
| `str` at 0x72210 | store | 4 | `sp -0x40` | `0x165374` |
| `ldr` at 0x72214 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x72218 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x72220 | load | 4 | `abs 0x7238c` | `-` |
| `ldr` at 0x72230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72244 | load | 4 | `abs 0x72380` | `-` |
| `ldr` at 0x72248 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72250 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x72254 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72258 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x7225c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x72260 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72264 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x72268 | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldr` at 0x7226c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72270 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72274 | load | 1 | `[sp-36] +0x4` | `-` |
| `str` at 0x72288 | store | 4 | `[sp-36] +0x8` | `ret(0x204d8)` |
| `ldr` at 0x7228c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x72298 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x722ac | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x722b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x722c0 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x722c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x722ec | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x722f0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x722f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x722f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x72304 | load | 4 | `abs 0x72394` | `-` |
| `str` at 0x72308 | store | 4 | `(unknown)` | `0x1653ac` |
| `str` at 0x7230c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x72310 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x72318 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x72320 | load | 4 | `abs 0x7238c` | `-` |
| `str` at 0x72334 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x72338 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x7233c | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x72340 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7234c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72350 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x72358 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0x72364 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x72368 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x72370 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x7237c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7237c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7237c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7237c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72164 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a6200]` |
| `bl` at 0x721a8 | `(unknown callee) 0x22070` | `0x9` | `0x165360` | `0x4b` | `0x0` |
| `bl` at 0x721bc | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a6204]` |
| `bl` at 0x721f4 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `[0x1a6204]` | `?` |
| `bl` at 0x7222c | `(unknown callee) 0x22070` | `0x9` | `0x165360` | `0x56` | `0x0` |
| `bl` at 0x72280 | `(unknown callee) 0x204d8` | `[[sp-36]+4]:1` | `0x4` | `[sp-36]` | `[[sp-36]+4]:1` |
| `bl` at 0x722a8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x722b4 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x722fc | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x7232c | `(unknown callee) 0x22070` | `0x9` | `0x165360` | `0x62` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x72160 | `[0x1a6200] == 0x0` | `[0x1a6200] != 0x0` |
| `bne` at 0x72174 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x721b8 | `[0x1a6204] != 0x0` | `[0x1a6204] == 0x0` |
| `bne` at 0x72208 | `[0x1a6200] != 0x0` | `[0x1a6200] == 0x0` |
| `bne` at 0x72294 | `[[sp-36]+8] != 0x0` | `[[sp-36]+8] == 0x0` |
| `ble` at 0x72348 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7237c | `r0` | `?` |

## `tempRemove`

- `tempRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72398 to 0x7243c.
  Established. The signed comparison branches to 0x7242c when entry r0 is greater than or equal to [0x1a6204], falling through otherwise (`bge` at 0x723bc).
  Byte and word stores write 0 at [sp-24] +4 and [sp-24] (`strb` at 0x7241c, `str` at 0x72428).
  No calls are emitted, and the recorded r0 return remains entry r0 (`ldm` at 0x72430).
  These stores do not establish a physical sensor-removal action.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x723a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x723ac | load | 4 | `abs 0x72434` | `-` |
| `ldr` at 0x723b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x723b4 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x723c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x723d4 | load | 4 | `abs 0x72438` | `-` |
| `ldr` at 0x723d8 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x723e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x723f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72404 | load | 4 | `abs 0x72438` | `-` |
| `ldr` at 0x72408 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72410 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x72414 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x7241c | store | 1 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x72420 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x72428 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldm` at 0x72430 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72430 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72430 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x723bc | `r0 >= [0x1a6204] (signed, 32-bit)` | `r0 < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x723e8 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72430 | `r0` | `r0` |

## `tempGetCfg`

- `tempGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7243c to 0x724f4.
  Established. A word load reads [sp-28], and a word store writes [[sp-28]] at entry r1 (`ldr` at 0x724cc, `str` at 0x724d0).
  A byte load reads [sp-28] +4, followed by a word store of that loaded byte at sp-0x20 (`ldrb` at 0x724d8, `str` at 0x724dc).
  Another path stores 0 at sp-0x20, while the recorded r0 return is unknown (`str` at 0x72498, `ldm` at 0x724e8).
  The loaded word and byte have no established configuration-field meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7244c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x72450 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x72454 | load | 4 | `abs 0x724ec` | `-` |
| `ldr` at 0x72458 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7245c | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x72468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7247c | load | 4 | `abs 0x724f0` | `-` |
| `ldr` at 0x72480 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x72488 | load | 1 | `(unknown)` | `-` |
| `str` at 0x72498 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x724a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x724b4 | load | 4 | `abs 0x724f0` | `-` |
| `ldr` at 0x724b8 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x724c0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x724c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x724c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x724cc | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x724d0 | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x724d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x724d8 | load | 1 | `[sp-28] +0x4` | `-` |
| `str` at 0x724dc | store | 4 | `sp -0x20` | `[[sp-28]+4]:1` |
| `ldr` at 0x724e0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x724e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x724e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x724e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x72464 | `r0 >= [0x1a6204] (signed, 32-bit)` | `r0 < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x72490 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x724e8 | `r0` | `?` |

## `tempCount`

- `tempCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x724f4 to 0x72598.
  Established. Word loads read absolute 0x1a6204 and 0x1a6200 (`ldr` at 0x72510, `ldr` at 0x72534).
  One path stores 0 at sp-0x1c, and another reads [sp-24] +4 and stores that byte at sp-0x1c (`str` at 0x7254c, `ldrb` at 0x7257c, `str` at 0x72580).
  No calls are emitted, and the recorded r0 return is unknown, so what the returned value counts remains unestablished (`ldm` at 0x7258c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72504 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x72508 | load | 4 | `abs 0x72590` | `-` |
| `ldr` at 0x7250c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72510 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x7251c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72530 | load | 4 | `abs 0x72594` | `-` |
| `ldr` at 0x72534 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x7253c | load | 1 | `(unknown)` | `-` |
| `str` at 0x7254c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x72554 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72568 | load | 4 | `abs 0x72594` | `-` |
| `ldr` at 0x7256c | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72574 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x72578 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x7257c | load | 1 | `[sp-24] +0x4` | `-` |
| `str` at 0x72580 | store | 4 | `sp -0x1c` | `[[sp-24]+4]:1` |
| `ldr` at 0x72584 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x7258c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7258c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7258c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x72518 | `r0 >= [0x1a6204] (signed, 32-bit)` | `r0 < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x72544 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7258c | `r0` | `?` |

## `tempPresent`

- `tempPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72598 to 0x72670.
  Established. The sole call to unnamed 0x72f48 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/entry r2/(entry r1 & 0xff) (`bl` at 0x725c0).
  A nonzero call result takes the branch to 0x72644, while its other path jumps to 0x72660 (`bne` at 0x7263c, `b` at 0x72640).
  The later byte store writes 1 at an unknown effective address, leaving the software field's relationship to physical presence Unknown (`strb` at 0x7265c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x725a8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x725b0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x725b4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x725b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x725c8 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x725cc | load | 4 | `abs 0x72668` | `-` |
| `ldr` at 0x725d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x725d4 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x725e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x725f4 | load | 4 | `abs 0x7266c` | `-` |
| `ldr` at 0x725f8 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x72600 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72624 | load | 4 | `abs 0x7266c` | `-` |
| `ldr` at 0x72628 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72630 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72634 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72644 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72648 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72650 | load | 4 | `[sp-32] +0x8` | `-` |
| `strb` at 0x7265c | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x72664 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72664 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72664 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x725c0 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x725dc | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x72608 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x7263c | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72664 | `r0` | `ret(0x72f48)` |

## `tempNotPresent`

- `tempNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72670 to 0x72758.
  Established. A word load reads [sp-32] +8, and a byte store writes 0 at an unknown effective address (`ldr` at 0x72728, `strb` at 0x72734).
  The call to unnamed 0x72acc has candidate r0/r1/r2/r3 values [sp-20]/[sp-21]:1/unknown/[sp-21]:1 (`bl` at 0x72744).
  The recorded r0 return is unknown, and the body does not identify what physical action follows from the stored zero or that call (`ldm` at 0x7274c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72680 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72688 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x7268c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72690 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x726a0 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x726a4 | load | 4 | `abs 0x72750` | `-` |
| `ldr` at 0x726a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x726ac | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x726b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x726cc | load | 4 | `abs 0x72754` | `-` |
| `ldr` at 0x726d0 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x726d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x726e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x726fc | load | 4 | `abs 0x72754` | `-` |
| `ldr` at 0x72700 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72708 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x7270c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7271c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72720 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72728 | load | 4 | `[sp-32] +0x8` | `-` |
| `strb` at 0x72734 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x72738 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7273c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7274c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7274c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7274c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72698 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72744 | `(unknown callee) 0x72acc` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x726b4 | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x726e0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72714 | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7274c | `r0` | `?` |

## `tempIsPresent`

- `tempIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72758 to 0x72864.
  Established. A word store saves ret(0x72f48) at sp-0x1c, while two later paths store 0 at sp-0x24 (`str` at 0x72788, `str` at 0x727d0, `str` at 0x7282c).
  A byte load reads ret(0x72f48), and the call to unnamed 0x111978 has candidate r0/r1/r2/r3 values [sp-20]/[ret(0x72f48)]:1/unknown/[ret(0x72f48)]:1 (`ldrb` at 0x72838, `bl` at 0x72844).
  A separate byte load has unknown effective address, leaving both tested software fields and any presence interpretation unidentified (`ldrb` at 0x7281c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72768 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72770 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72774 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72778 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72788 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x7278c | load | 4 | `abs 0x7285c` | `-` |
| `ldr` at 0x72790 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72794 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x727a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x727b4 | load | 4 | `abs 0x72860` | `-` |
| `ldr` at 0x727b8 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x727c0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x727d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x727d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x727ec | load | 4 | `abs 0x72860` | `-` |
| `ldr` at 0x727f0 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x727f8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x727fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72808 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7280c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72814 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0x7281c | load | 1 | `(unknown)` | `-` |
| `str` at 0x7282c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x72834 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72838 | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x7283c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7284c | store | 4 | `sp -0x24` | `ret(0x111978)` |
| `ldr` at 0x72850 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x72858 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72858 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72780 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72844 | `(unknown callee) 0x111978` | `[sp-20]` | `[ret(0x72f48)]:1` | `?` | `[ret(0x72f48)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x7279c | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x727c8 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72804 | `ret(0x72f48) == 0x0` | `ret(0x72f48) != 0x0` |
| `bne` at 0x72824 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72858 | `r0` | `?` |

## `tempSetBias`

- `tempSetBias`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72864 to 0x72964.
  Established. Byte stores save masked entry r1 and r2 at sp-0x15 and sp-0x16 (`strb` at 0x7287c, `strb` at 0x72884).
  Two paths write 0xfffffc11 at sp-0x24, while the final path writes 0 there (`str` at 0x728e4, `str` at 0x72920, `str` at 0x72948).
  A byte store writes ([sp-22]:1 & 0xff) at an unknown effective address (`strb` at 0x72940);
  its field meaning and any numeric bias or units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72874 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x7287c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x72884 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x72888 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7288c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7289c | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x728a0 | load | 4 | `abs 0x72958` | `-` |
| `ldr` at 0x728a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x728a8 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x728b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x728c8 | load | 4 | `abs 0x7295c` | `-` |
| `ldr` at 0x728cc | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x728d4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x728e0 | load | 4 | `abs 0x72960` | `-` |
| `str` at 0x728e4 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x728ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72900 | load | 4 | `abs 0x7295c` | `-` |
| `ldr` at 0x72904 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x7290c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72910 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7291c | load | 4 | `abs 0x72960` | `-` |
| `str` at 0x72920 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x72928 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x7292c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72934 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0x7293c | load | 1 | `sp -0x16` | `-` |
| `strb` at 0x72940 | store | 1 | `(unknown)` | `([sp-22]:1 & 0xff)` |
| `str` at 0x72948 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7294c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x72954 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72954 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72954 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72894 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x728b0 | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x728dc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72918 | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72954 | `r0` | `?` |

## `tempSetLimits`

- `tempSetLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72964 to 0x72acc.
  Established. Byte loads read ret(0x72f48) and ret(0x72f48) +1 (`ldrb` at 0x72a80, `ldrb` at 0x72a88).
  Other byte loads have unknown effective addresses, and their subsequent stack stores have unknown values (`ldrb` at 0x72a4c, `strb` at 0x72a54, `ldrb` at 0x72a70, `strb` at 0x72a78).
  The call to unnamed 0x112738 has candidate r0/r1/r2/r3 values [sp-20]/[ret(0x72f48)]:1/[ret(0x72f48)+1]:1/[sp-22]:1 (`bl` at 0x72aa8).
  These byte values do not establish measurement units or limit semantics.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72974 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72980 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x72988 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x72990 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x72994 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72998 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x729a8 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x729ac | load | 4 | `abs 0x72ac0` | `-` |
| `ldr` at 0x729b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729b4 | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x729c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729d4 | load | 4 | `abs 0x72ac4` | `-` |
| `ldr` at 0x729d8 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x729e0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x729ec | load | 4 | `abs 0x72ac8` | `-` |
| `str` at 0x729f0 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x729f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72a0c | load | 4 | `abs 0x72ac4` | `-` |
| `ldr` at 0x72a10 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72a18 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72a1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72a28 | load | 4 | `abs 0x72ac8` | `-` |
| `str` at 0x72a2c | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x72a34 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72a38 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72a40 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0x72a48 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x72a4c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x72a54 | store | 1 | `sp -0x16` | `?` |
| `ldr` at 0x72a58 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72a5c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72a64 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0x72a6c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x72a70 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x72a78 | store | 1 | `sp -0x17` | `?` |
| `ldr` at 0x72a7c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72a80 | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x72a84 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72a88 | load | 1 | `ret(0x72f48) +0x1` | `-` |
| `ldrb` at 0x72a8c | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x72a90 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x72a94 | store | 4 | `sp -0x28` | `[sp-23]:1` |
| `ldr` at 0x72a98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72ab0 | store | 4 | `sp -0x24` | `ret(0x112738)` |
| `ldr` at 0x72ab4 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x72abc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72abc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72abc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x729a0 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72aa8 | `(unknown callee) 0x112738` | `[sp-20]` | `[ret(0x72f48)]:1` | `[ret(0x72f48)+1]:1` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x729bc | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x729e8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72a24 | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72abc | `r0` | `?` |

## `tempIntDisable`

- `tempIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72acc to 0x72b50.
  Established. A nonzero result from unnamed 0x72f48 takes the branch to 0x72b18, while its zero-result path stores 0xfffffc11 at sp-0x20 (`bne` at 0x72b08, `str` at 0x72b10).
  Byte loads read ret(0x72f48) and ret(0x72f48) +1 (`ldrb` at 0x72b1c, `ldrb` at 0x72b24).
  The call to unnamed 0x1140d0 has candidate r0/r1/r2/r3 values [sp-20]/[ret(0x72f48)]:1/[ret(0x72f48)+1]:1/[ret(0x72f48)+1]:1 (`bl` at 0x72b34).
  The candidates alone leave any physical disable operation unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72adc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72ae4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72ae8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72aec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72afc | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x72b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72b0c | load | 4 | `abs 0x72b4c` | `-` |
| `str` at 0x72b10 | store | 4 | `sp -0x20` | `0xfffffc11` |
| `ldr` at 0x72b18 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72b1c | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x72b20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72b24 | load | 1 | `ret(0x72f48) +0x1` | `-` |
| `ldr` at 0x72b28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72b3c | store | 4 | `sp -0x20` | `ret(0x1140d0)` |
| `ldr` at 0x72b40 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x72b48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72b48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72b48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72af4 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72b34 | `(unknown callee) 0x1140d0` | `[sp-20]` | `[ret(0x72f48)]:1` | `[ret(0x72f48)+1]:1` | `[ret(0x72f48)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72b08 | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72b48 | `r0` | `?` |

## `tempIntEnable`

- `tempIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72b50 to 0x72c6c.
  Established. Word stores write 0 and 0xfffffc11 at sp-0x24 on separate paths (`str` at 0x72bc8, `str` at 0x72c24).
  A word load reads [sp-32] +8, but the following computed byte load has unknown effective address (`ldr` at 0x72c0c, `ldrb` at 0x72c14).
  The call to unnamed 0x11411c has candidate r0/r1/r2/r3 values [sp-20]/[ret(0x72f48)]:1/[ret(0x72f48)+1]:1/[ret(0x72f48)+1]:1 (`bl` at 0x72c48).
  No physical interrupt-enable behavior is established by those loaded values or the call.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72b60 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72b68 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72b6c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72b70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72b80 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x72b84 | load | 4 | `abs 0x72c60` | `-` |
| `ldr` at 0x72b88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72b8c | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x72b98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72bac | load | 4 | `abs 0x72c64` | `-` |
| `ldr` at 0x72bb0 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x72bb8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x72bc8 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x72bd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72be4 | load | 4 | `abs 0x72c64` | `-` |
| `ldr` at 0x72be8 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72bf0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72c00 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x72c04 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72c0c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldrb` at 0x72c14 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72c20 | load | 4 | `abs 0x72c68` | `-` |
| `str` at 0x72c24 | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0x72c2c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72c30 | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x72c34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72c38 | load | 1 | `ret(0x72f48) +0x1` | `-` |
| `ldr` at 0x72c3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72c50 | store | 4 | `sp -0x24` | `ret(0x11411c)` |
| `ldr` at 0x72c54 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x72c5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72c5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72c5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72b78 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72c48 | `(unknown callee) 0x11411c` | `[sp-20]` | `[ret(0x72f48)]:1` | `[ret(0x72f48)+1]:1` | `[ret(0x72f48)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x72b94 | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x72bc0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72bfc | `ret(0x72f48) == 0x0` | `ret(0x72f48) != 0x0` |
| `bne` at 0x72c1c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72c5c | `r0` | `?` |

## `tempRead`

- `tempRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72c6c to 0x72dd8.
  Established. The call to unnamed 0x113108 has candidate r0/r1/r2/r3 values [sp-20]/[ret(0x72f48)]:1/[ret(0x72f48)+1]:1/sp-29 (`bl` at 0x72d50);
  its zero result takes the branch to 0x72d80, while the other path writes 0 at [sp-28] (`beq` at 0x72d64, `str` at 0x72d70).
  Later word stores at [sp-28] have unknown values, including the store after unsupported rsb effects (`str` at 0x72d88, `rsb` at 0x72db0, `str` at 0x72db4).
  The computed result and any measurement interpretation remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72c7c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x72c84 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x72c88 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72c8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72c90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72ca0 | store | 4 | `sp -0x28` | `ret(0x72f48)` |
| `ldr` at 0x72ca4 | load | 4 | `abs 0x72dcc` | `-` |
| `ldr` at 0x72ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72cac | load | 4 | `abs 0x1a6204` | `-` |
| `ldr` at 0x72cb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72ccc | load | 4 | `abs 0x72dd0` | `-` |
| `ldr` at 0x72cd0 | load | 4 | `abs 0x1a6200` | `-` |
| `ldrb` at 0x72cd8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72ce4 | load | 4 | `abs 0x72dd4` | `-` |
| `str` at 0x72ce8 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0x72cf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72d04 | load | 4 | `abs 0x72dd0` | `-` |
| `ldr` at 0x72d08 | load | 4 | `abs 0x1a6200` | `-` |
| `str` at 0x72d10 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x72d14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x72d20 | load | 4 | `abs 0x72dd4` | `-` |
| `str` at 0x72d24 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0x72d2c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x72d30 | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x72d34 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x72d38 | load | 1 | `ret(0x72f48) +0x1` | `-` |
| `ldr` at 0x72d40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72d58 | store | 4 | `sp -0x24` | `ret(0x113108)` |
| `ldr` at 0x72d5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x72d68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x72d70 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x72d74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x72d78 | store | 4 | `sp -0x30` | `[sp-36]` |
| `ldr` at 0x72d80 | load | 4 | `sp -0x1c` | `-` |
| `ldrsb` at 0x72d84 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x72d88 | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x72d8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72d90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72d94 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x72d98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72da0 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldrsb` at 0x72da8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x72dac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x72db4 | store | 4 | `[sp-28] +0x0` | `?` |
| `str` at 0x72dbc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x72dc0 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x72dc8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72dc8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72dc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72c98 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72d50 | `(unknown callee) 0x113108` | `[sp-20]` | `[ret(0x72f48)]:1` | `[ret(0x72f48)+1]:1` | `sp-29` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x72cb4 | `[sp-20] >= [0x1a6204] (signed, 32-bit)` | `[sp-20] < [0x1a6204] (signed, 32-bit)` |
| `bne` at 0x72ce0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x72d1c | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |
| `beq` at 0x72d64 | `ret(0x113108) == 0x0` | `ret(0x113108) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72dc8 | `r0` | `?` |

## `tempLocation`

- `tempLocation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72dd8 to 0x72e70.
  Established. A word load reads ret(0x72f48) +4 and stores the loaded value at sp-0x24 (`ldr` at 0x72e28, `str` at 0x72e2c).
  The branch targets 0x72e58 when [ret(0x72f48)+4] is nonzero, while its other path reaches unnamed 0x1144ec with candidate r0/r1 values [sp-20]/[ret(0x72f48)]:1 (`bne` at 0x72e3c, `bl` at 0x72e50).
  A separate path writes 0x1653e4 at sp-0x20, and the recorded r0 return remains unknown (`str` at 0x72e1c, `ldm` at 0x72e68).
  The returned value does not establish a physical location or device identity.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72de8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72df0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72df4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72df8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72e08 | store | 4 | `sp -0x1c` | `ret(0x72f48)` |
| `ldr` at 0x72e0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72e18 | load | 4 | `abs 0x72e6c` | `-` |
| `str` at 0x72e1c | store | 4 | `sp -0x20` | `0x1653e4` |
| `ldr` at 0x72e24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72e28 | load | 4 | `ret(0x72f48) +0x4` | `-` |
| `str` at 0x72e2c | store | 4 | `sp -0x24` | `[ret(0x72f48)+4]` |
| `ldr` at 0x72e30 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x72e34 | load | 4 | `ret(0x72f48) +0x4` | `-` |
| `ldr` at 0x72e40 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72e44 | load | 1 | `ret(0x72f48) +0x0` | `-` |
| `ldr` at 0x72e48 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72e54 | store | 4 | `sp -0x24` | `ret(0x1144ec)` |
| `ldr` at 0x72e58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x72e5c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x72e60 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x72e68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72e68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72e68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72e00 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72e50 | `(unknown callee) 0x1144ec` | `[sp-20]` | `[ret(0x72f48)]:1` | `?` | `[ret(0x72f48)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72e14 | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |
| `bne` at 0x72e3c | `[ret(0x72f48)+4] != 0x0` | `[ret(0x72f48)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72e68 | `r0` | `?` |

## `tempType`

- `tempType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72e70 to 0x72f48.
  Established. A byte load reads ret(0x72f48) +2, and its zero value takes the branch to 0x72edc (`ldrb` at 0x72ec0, `beq` at 0x72ec8).
  The call to unnamed 0x111db8 has candidate r0/r1/r2/r3 values [sp-20]/sp-28/unknown/sp-28 (`bl` at 0x72ee8).
  A signed comparison branches to 0x72f34 when [[sp-36]]:1 is at least ret(0x111db8), while the other path reaches a byte load with unknown effective address (`bge` at 0x72f04, `ldrb` at 0x72f28).
  The selected byte and any type identification remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72e80 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x72e88 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x72e8c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x72e90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72ea0 | store | 4 | `sp -0x24` | `ret(0x72f48)` |
| `ldr` at 0x72ea4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x72eb4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x72ebc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72ec0 | load | 1 | `ret(0x72f48) +0x2` | `-` |
| `ldr` at 0x72ecc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72ed0 | load | 1 | `ret(0x72f48) +0x2` | `-` |
| `str` at 0x72ed4 | store | 4 | `sp -0x28` | `[ret(0x72f48)+2]:1` |
| `ldr` at 0x72ee0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x72ef0 | store | 4 | `sp -0x20` | `ret(0x111db8)` |
| `ldr` at 0x72ef4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72ef8 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x72efc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72f08 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x72f0c | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x72f20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x72f28 | load | 1 | `(unknown)` | `-` |
| `str` at 0x72f2c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x72f38 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x72f3c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x72f44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x72f44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x72f44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72e98 | `(unknown callee) 0x72f48` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x72ee8 | `(unknown callee) 0x111db8` | `[sp-20]` | `sp-28` | `?` | `sp-28` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72eac | `ret(0x72f48) != 0x0` | `ret(0x72f48) == 0x0` |
| `beq` at 0x72ec8 | `[ret(0x72f48)+2]:1 == 0x0` | `[ret(0x72f48)+2]:1 != 0x0` |
| `bge` at 0x72f04 | `[[sp-36]]:1 >= ret(0x111db8) (signed, 32-bit)` | `[[sp-36]]:1 < ret(0x111db8) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x72f44 | `r0` | `?` |

## `monitorInit`

- `monitorInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x111324 to 0x11157c.
  Established. Word stores write ret(0x6e140) to absolute 0x314fa4 and ret(0x204d8) to absolute 0x314fa0 (`str` at 0x1113b4, `str` at 0x1113dc).
  A later call to unnamed 0x204d8 has candidate r0/r1/r2/r3 values [[sp-36]+8]/12/[sp-36]/[sp-36], and its result is stored at [sp-36] +4 (`bl` at 0x111450, `str` at 0x111458).
  The final loop writes 1 at an unknown effective address and updates sp-0x28 to [sp-40]+1 (`strb` at 0x111540, `str` at 0x11154c).
  The stored byte and initialized software objects have no established physical-monitor identity.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x111334 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x111338 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x11133c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x111340 | load | 4 | `abs 0x11155c` | `-` |
| `ldr` at 0x111344 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x111358 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111364 | load | 4 | `abs 0x111560` | `-` |
| `str` at 0x111368 | store | 4 | `sp -0x40` | `0x18dfc0` |
| `ldr` at 0x11136c | load | 4 | `abs 0x11155c` | `-` |
| `ldr` at 0x111370 | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111374 | store | 4 | `sp -0x3c` | `[0x314fa0]` |
| `ldr` at 0x111378 | load | 4 | `abs 0x111564` | `-` |
| `ldr` at 0x11137c | load | 4 | `abs 0x314fa4` | `-` |
| `str` at 0x111380 | store | 4 | `sp -0x38` | `[0x314fa4]` |
| `ldr` at 0x111388 | load | 4 | `abs 0x111568` | `-` |
| `ldr` at 0x111398 | load | 4 | `abs 0x111564` | `-` |
| `ldr` at 0x11139c | load | 4 | `abs 0x314fa4` | `-` |
| `ldr` at 0x1113b0 | load | 4 | `abs 0x111564` | `-` |
| `str` at 0x1113b4 | store | 4 | `abs 0x314fa4` | `ret(0x6e140)` |
| `ldr` at 0x1113b8 | load | 4 | `abs 0x111564` | `-` |
| `ldr` at 0x1113bc | load | 4 | `abs 0x314fa4` | `-` |
| `str` at 0x1113c4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1113c8 | load | 4 | `abs 0x11155c` | `-` |
| `ldr` at 0x1113cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1113dc | store | 4 | `abs 0x314fa0` | `ret(0x204d8)` |
| `ldr` at 0x1113e0 | load | 4 | `abs 0x314fa0` | `-` |
| `ldr` at 0x1113ec | load | 4 | `abs 0x11156c` | `-` |
| `str` at 0x1113f0 | store | 4 | `sp -0x40` | `0x18e00c` |
| `ldr` at 0x1113f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1113f8 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x111400 | load | 4 | `abs 0x111568` | `-` |
| `ldr` at 0x111404 | load | 4 | `abs 0x111570` | `-` |
| `ldr` at 0x111410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x111418 | load | 4 | `abs 0x11155c` | `-` |
| `ldr` at 0x11141c | load | 4 | `abs 0x314fa0` | `-` |
| `str` at 0x111424 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x111428 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11142c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x111430 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x111434 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x111438 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x11143c | store | 4 | `[sp-36] +0x8` | `[sp-32]` |
| `ldr` at 0x111440 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x111444 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x111448 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x111458 | store | 4 | `[sp-36] +0x4` | `ret(0x204d8)` |
| `ldr` at 0x11145c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x111468 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11147c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x111480 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x111490 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x111494 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1114bc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1114c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1114c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1114c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1114d4 | load | 4 | `abs 0x111574` | `-` |
| `str` at 0x1114d8 | store | 4 | `(unknown)` | `0x18e048` |
| `str` at 0x1114dc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x1114e0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1114e4 | load | 4 | `[sp-36] +0x8` | `-` |
| `str` at 0x1114e8 | store | 4 | `(unknown)` | `[[sp-36]+8]` |
| `ldr` at 0x1114f0 | load | 4 | `abs 0x111568` | `-` |
| `ldr` at 0x1114f4 | load | 4 | `abs 0x111578` | `-` |
| `str` at 0x111504 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x111508 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x11150c | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x111510 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x11151c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x111520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x111534 | load | 4 | `[sp-36] +0x4` | `-` |
| `strb` at 0x111540 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x111544 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x11154c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x111558 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x111558 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x111558 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x111558 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x111350 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x314fa0]` |
| `bl` at 0x111394 | `(unknown callee) 0x22070` | `0x9` | `0x18dff0` | `0x1a8` | `0x0` |
| `bl` at 0x1113a8 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x314fa4]` |
| `bl` at 0x1113d4 | `(unknown callee) 0x204d8` | `[sp-44]` | `0x1` | `ret(0x6e140)` | `?` |
| `bl` at 0x11140c | `(unknown callee) 0x22070` | `0x9` | `0x18dff0` | `0x1b3` | `0x0` |
| `bl` at 0x111450 | `(unknown callee) 0x204d8` | `[[sp-36]+8]` | `0xc` | `[sp-36]` | `[sp-36]` |
| `bl` at 0x111478 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x111484 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1114cc | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x1114fc | `(unknown callee) 0x22070` | `0x9` | `0x18dff0` | `0x1be` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x11134c | `[0x314fa0] == 0x0` | `[0x314fa0] != 0x0` |
| `bne` at 0x111360 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x1113a4 | `[0x314fa4] != 0x0` | `[0x314fa4] == 0x0` |
| `bne` at 0x1113e8 | `[0x314fa0] != 0x0` | `[0x314fa0] == 0x0` |
| `bne` at 0x111464 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |
| `ble` at 0x111518 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x111558 | `r0` | `?` |
