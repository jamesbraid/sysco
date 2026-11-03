# ARM flash and reset caller paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
`cmdFlashSetDefault` and `cmdFlashBoot` each have one unresolved computed transfer.
Their tables cover known edges only. The other thirteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/flash-reset-callers-facts.txt) retain all tool sections.

## `PBaySdrFlashInfo`

- `PBaySdrFlashInfo`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa12f8 to 0xa149c.
  Established. An included halfword store saves `([r0+6]:2 & 0xffff)` at `sp -0x1ac` (`strh` at 0xa131c).
  The unnamed `0x9dc3c` call carries entry `r0`, `sp-428`, `0xfffffe64`, `sp-428` (`bl` at 0xa1354);
  its zero result takes `0xa144c`, while nonzero falls through (`beq` at 0xa1368).
  Power-bay flash information interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1308 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa130c | load | 4 | `abs 0xa1490` | `-` |
| `ldr` at 0xa1310 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa1314 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xa131c | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xa1324 | store | 4 | `sp -0x1a8` | `0xc0` |
| `str` at 0xa132c | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xa1334 | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xa133c | store | 4 | `sp -0x114` | `0x0` |
| `str` at 0xa1344 | store | 4 | `sp -0x110` | `0xff` |
| `ldr` at 0xa134c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa135c | store | 4 | `sp -0x1b0` | `ret(0x9dc3c)` |
| `ldr` at 0xa1360 | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xa136c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa1370 | store | 4 | `sp -0x214` | `[sp-36]` |
| `ldr` at 0xa1374 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa1378 | load | 1 | `[sp-36] +0x5` | `-` |
| `ldr` at 0xa1380 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa1384 | load | 1 | `[sp-36] +0x4` | `-` |
| `ldr` at 0xa1390 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa1394 | load | 1 | `[sp-36] +0x3` | `-` |
| `ldr` at 0xa13a0 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa13a4 | load | 1 | `[sp-36] +0x2` | `-` |
| `str` at 0xa13ac | store | 4 | `sp -0x1b4` | `?` |
| `str` at 0xa13c0 | store | 4 | `sp -0x210` | `ret(0x196e8)` |
| `ldr` at 0xa13c8 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa13cc | load | 1 | `[sp-532] +0x1` | `-` |
| `ldr` at 0xa13d0 | load | 4 | `sp -0x214` | `-` |
| `ldrb` at 0xa13d4 | load | 1 | `[sp-532] +0x0` | `-` |
| `ldr` at 0xa13d8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa13dc | load | 4 | `ret(0x196e8) +0x10` | `-` |
| `str` at 0xa13e4 | store | 4 | `sp -0x22c` | `[ret(0x196e8)+16]+1` |
| `ldr` at 0xa13e8 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa13ec | load | 4 | `ret(0x196e8) +0xc` | `-` |
| `str` at 0xa13f0 | store | 4 | `sp -0x228` | `[ret(0x196e8)+12]` |
| `ldr` at 0xa13f4 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa13f8 | load | 4 | `ret(0x196e8) +0x14` | `-` |
| `str` at 0xa1404 | store | 4 | `sp -0x224` | `[ret(0x196e8)+20]+1900` |
| `ldr` at 0xa1408 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa140c | load | 4 | `ret(0x196e8) +0x8` | `-` |
| `str` at 0xa1410 | store | 4 | `sp -0x220` | `[ret(0x196e8)+8]` |
| `ldr` at 0xa1414 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa1418 | load | 4 | `ret(0x196e8) +0x4` | `-` |
| `str` at 0xa141c | store | 4 | `sp -0x21c` | `[ret(0x196e8)+4]` |
| `ldr` at 0xa1420 | load | 4 | `sp -0x210` | `-` |
| `ldr` at 0xa1424 | load | 4 | `ret(0x196e8) +0x0` | `-` |
| `str` at 0xa1428 | store | 4 | `sp -0x218` | `[ret(0x196e8)]` |
| `ldr` at 0xa1430 | load | 4 | `abs 0xa1494` | `-` |
| `str` at 0xa1444 | store | 4 | `sp -0x208` | `sp-516` |
| `ldr` at 0xa144c | load | 4 | `abs 0xa1498` | `-` |
| `str` at 0xa1450 | store | 4 | `sp -0x208` | `0x1130d0` |
| `ldr` at 0xa1454 | load | 4 | `sp -0x208` | `-` |
| `str` at 0xa1470 | store | 4 | `sp -0x20c` | `ret(0x1943c)` |
| `ldr` at 0xa1474 | load | 4 | `sp -0x20c` | `-` |
| `ldr` at 0xa1478 | load | 4 | `sp -0x208` | `-` |
| `ldr` at 0xa1480 | load | 4 | `sp -0x20c` | `-` |
| `ldm` at 0xa148c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa148c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa148c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1354 | `(unknown callee) 0x9dc3c` | `r0` | `sp-428` | `0xfffffe64` | `sp-428` |
| `bl` at 0xa13b8 | `(unknown callee) 0x196e8` | `sp-436` | `?` | `?` | `sp-436` |
| `bl` at 0xa143c | `(unknown callee) 0x197a8` | `sp-516` | `0x113090` | `[[sp-532]+1]:1` | `[[sp-532]]:1` |
| `bl` at 0xa1458 | `(unknown callee) 0x19538` | `?` | `?` | `?` | `?` |
| `bl` at 0xa1468 | `(unknown callee) 0x1943c` | `ret(0x19538)+1` | `?` | `?` | `ret(0x19538)+1` |
| `bl` at 0xa147c | `(unknown callee) 0x19808` | `ret(0x1943c)` | `[sp-520]` | `?` | `ret(0x1943c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa1368 | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa148c | `r0` | `[sp-524]` |

## `PBaySdrWarmReset`

- `PBaySdrWarmReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1210 to 0xa12f8.
  Established. The unnamed `0x9dc3c` call carries entry `r0`, `sp-428`, `sp-260`, `sp-428` (`bl` at 0xa1290);
  its zero result takes `0xa12b4`, while nonzero falls through (`beq` at 0xa12a4).
  An included word assignment puts `0x20` at `sp -0x1a8` (`str` at 0xa123c).
  Power-bay warm reset command semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1220 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1224 | load | 4 | `abs 0xa12e8` | `-` |
| `ldr` at 0xa1228 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa122c | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xa1234 | store | 2 | `sp -0x1ac` | `([r0+6]:2 & 0xffff)` |
| `str` at 0xa123c | store | 4 | `sp -0x1a8` | `0x20` |
| `str` at 0xa1244 | store | 4 | `sp -0x20` | `0x3` |
| `str` at 0xa124c | store | 4 | `sp -0x1c` | `0x3` |
| `str` at 0xa1254 | store | 4 | `sp -0x110` | `0x3` |
| `ldr` at 0xa1258 | load | 4 | `abs 0xa12ec` | `-` |
| `ldrb` at 0xa125c | load | 1 | `abs 0x133b1f` | `-` |
| `strb` at 0xa1264 | store | 1 | `sp -0x10c` | `([0x133b1f]:1 & 0xff)` |
| `str` at 0xa126c | store | 4 | `sp -0x108` | `0x6` |
| `strb` at 0xa1278 | store | 1 | `sp -0x104` | `0x0` |
| `str` at 0xa1280 | store | 4 | `sp -0xc0` | `0x0` |
| `ldr` at 0xa1288 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa1298 | store | 4 | `sp -0x1b0` | `ret(0x9dc3c)` |
| `ldr` at 0xa129c | load | 4 | `sp -0x1b0` | `-` |
| `ldr` at 0xa12a8 | load | 4 | `abs 0xa12f0` | `-` |
| `str` at 0xa12ac | store | 4 | `sp -0x1b4` | `0x113080` |
| `ldr` at 0xa12b4 | load | 4 | `abs 0xa12f4` | `-` |
| `str` at 0xa12b8 | store | 4 | `sp -0x1b4` | `0x113084` |
| `str` at 0xa12c8 | store | 4 | `sp -0x1b8` | `ret(0x1943c)` |
| `ldr` at 0xa12cc | load | 4 | `sp -0x1b8` | `-` |
| `ldr` at 0xa12d0 | load | 4 | `sp -0x1b4` | `-` |
| `ldr` at 0xa12d8 | load | 4 | `sp -0x1b8` | `-` |
| `ldm` at 0xa12e4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa12e4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa12e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1290 | `(unknown callee) 0x9dc3c` | `r0` | `sp-428` | `sp-260` | `sp-428` |
| `bl` at 0xa12c0 | `(unknown callee) 0x1943c` | `0xc` | `?` | `?` | `?` |
| `bl` at 0xa12d4 | `(unknown callee) 0x19808` | `ret(0x1943c)` | `[sp-436]` | `?` | `ret(0x1943c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa12a4 | `ret(0x9dc3c) == 0x0` | `ret(0x9dc3c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa12e4 | `r0` | `[sp-440]` |

## `cmdFlashVersion`

- `cmdFlashVersion`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99750 to 0x99970.
  Established. Included word reads fetch absolute `0x29c3a8` and `0x29c3a0` (`ldr` at 0x99824, `ldr` at 0x99828).
  Their inequality takes `0x99844`, while equality falls through (`bne` at 0x99830).
  The unnamed `0xb8c60` call carries `[sp-48]`, `sp-96`, unknown, `sp-96` (`bl` at 0x99878).
  Flash version field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99760 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x99764 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x99768 | load | 4 | `abs 0x9995c` | `-` |
| `ldrb` at 0x9976c | load | 1 | `abs 0x112158` | `-` |
| `strb` at 0x99770 | store | 1 | `sp -0x2c` | `([0x112158]:1 & 0xff)` |
| `strb` at 0x9977c | store | 1 | `sp -0x2b` | `0x0` |
| `strb` at 0x99788 | store | 1 | `sp -0x2a` | `0x0` |
| `strb` at 0x99794 | store | 1 | `sp -0x29` | `0x0` |
| `strb` at 0x997a0 | store | 1 | `sp -0x28` | `0x0` |
| `strb` at 0x997ac | store | 1 | `sp -0x27` | `0x0` |
| `strb` at 0x997b8 | store | 1 | `sp -0x26` | `0x0` |
| `strb` at 0x997c4 | store | 1 | `sp -0x25` | `0x0` |
| `strb` at 0x997d0 | store | 1 | `sp -0x24` | `0x0` |
| `strb` at 0x997dc | store | 1 | `sp -0x23` | `0x0` |
| `strb` at 0x997e8 | store | 1 | `sp -0x22` | `0x0` |
| `strb` at 0x997f4 | store | 1 | `sp -0x21` | `0x0` |
| `strb` at 0x99800 | store | 1 | `sp -0x20` | `0x0` |
| `strb` at 0x9980c | store | 1 | `sp -0x1f` | `0x0` |
| `strb` at 0x99818 | store | 1 | `sp -0x1e` | `0x0` |
| `ldr` at 0x9981c | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x99820 | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x99824 | load | 4 | `abs 0x29c3a8` | `-` |
| `ldr` at 0x99828 | load | 4 | `abs 0x29c3a0` | `-` |
| `ldr` at 0x99834 | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x99838 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0x9983c | store | 4 | `sp -0x6c` | `[0x29c3a0]` |
| `ldr` at 0x99844 | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x99848 | load | 4 | `abs 0x29c3a4` | `-` |
| `str` at 0x9984c | store | 4 | `sp -0x6c` | `[0x29c3a4]` |
| `ldr` at 0x99850 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x99854 | store | 4 | `sp -0x30` | `[sp-108]` |
| `ldr` at 0x99858 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x99868 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0x99870 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99888 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9988c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x99890 | load | 4 | `abs 0x99964` | `-` |
| `ldr` at 0x99894 | load | 4 | `abs 0x99968` | `-` |
| `str` at 0x998a0 | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x998a8 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x998ac | store | 4 | `sp -0x64` | `[sp-80]` |
| `ldr` at 0x998b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x998b4 | store | 4 | `sp -0x70` | `[sp-28]` |
| `ldr` at 0x998b8 | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x998bc | load | 4 | `abs 0x99960` | `-` |
| `ldr` at 0x998c0 | load | 4 | `abs 0x29c3a8` | `-` |
| `ldr` at 0x998c4 | load | 4 | `abs 0x29c3a0` | `-` |
| `str` at 0x998d4 | store | 4 | `sp -0x74` | `0x41` |
| `str` at 0x998e0 | store | 4 | `sp -0x74` | `0x42` |
| `ldr` at 0x998e4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x998ec | store | 4 | `sp -0x90` | `[sp-80]+4` |
| `ldr` at 0x998f0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x998f8 | store | 4 | `sp -0x8c` | `[sp-80]+6` |
| `ldr` at 0x998fc | load | 4 | `sp -0x64` | `-` |
| `str` at 0x99900 | store | 4 | `sp -0x88` | `[sp-80]` |
| `ldr` at 0x99904 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9990c | store | 4 | `sp -0x84` | `[sp-80]+8` |
| `ldr` at 0x99910 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x99918 | store | 4 | `sp -0x80` | `[sp-80]+10` |
| `ldr` at 0x9991c | load | 4 | `sp -0x64` | `-` |
| `str` at 0x99924 | store | 4 | `sp -0x7c` | `[sp-80]+12` |
| `ldr` at 0x99928 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x9992c | store | 4 | `sp -0x78` | `[sp-52]` |
| `ldr` at 0x99930 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x99934 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x99938 | load | 4 | `abs 0x9996c` | `-` |
| `ldr` at 0x9993c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x99940 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x9994c | store | 4 | `sp -0x68` | `0x0` |
| `ldr` at 0x99950 | load | 4 | `sp -0x68` | `-` |
| `ldm` at 0x99958 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x99958 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99958 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99958 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99860 | `(unknown callee) 0x1943c` | `0x80` | `[0x29c3a8]` | `0x29c398` | `[sp-108]` |
| `bl` at 0x99878 | `(unknown callee) 0xb8c60` | `[sp-48]` | `sp-96` | `?` | `sp-96` |
| `bl` at 0x99898 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x1122b8` | `0x1122ec` | `[sp-28]` |
| `bl` at 0x99944 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x1122f4` | `[sp-76]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x99830 | `[0x29c3a8] != [0x29c3a0]` | `[0x29c3a8] == [0x29c3a0]` |
| `beq` at 0x99884 | `ret(0xb8c60) == 0x0` | `ret(0xb8c60) != 0x0` |
| `bne` at 0x998cc | `[0x29c3a8] != [0x29c3a0]` | `[0x29c3a8] == [0x29c3a0]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99958 | `r0` | `0x0` |

## `cmdFlashSetDefault`

- `cmdFlashSetDefault`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99970 to 0x99bb4.
  Established. The unnamed `0xbf154` call carries `[sp-40]`, unknown, `[[sp-28]]`, `[sp-36]` (`bl` at 0x99b44);
  its zero result takes `0x99b84`, while nonzero falls through (`beq` at 0x99b58).
  Included byte stores clear `[[sp-28]] +0x0` (`strb` at 0x999c0, `strb` at 0x99b90).
  Default image command selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99980 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x99984 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x99988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9998c | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0x99990 | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0x99994 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `str` at 0x9999c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x999a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x999b0 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0x999b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x999b8 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x999c0 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x999c4 | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0x999cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99b40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x99b4c | store | 4 | `sp -0x20` | `ret(0xbf154)` |
| `ldr` at 0x99b50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99b5c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99b68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99b6c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x99b70 | load | 4 | `abs 0x99bb0` | `-` |
| `str` at 0x99b7c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x99b84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99b88 | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x99b90 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `str` at 0x99b98 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x99b9c | load | 4 | `sp -0x90` | `-` |
| `ldm` at 0x99ba4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x99ba4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99ba4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99ba4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x999a8 | `(unknown callee) 0x1943c` | `0x80` | `r1` | `r2` | `0x0` |
| `bl` at 0x99b44 | `(unknown callee) 0xbf154` | `[sp-40]` | `?` | `[[sp-28]]` | `[sp-36]` |
| `bl` at 0x99b60 | `(unknown callee) 0x1f8e0` | `ret(0xbf154)` | `?` | `?` | `ret(0xbf154)` |
| `bl` at 0x99b74 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x112364` | `ret(0x1f8e0)` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x999cc | `[sp-36] <= 0x4 (unsigned, 32-bit)` | `[sp-36] > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x99b58 | `ret(0xbf154) == 0x0` | `ret(0xbf154) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99ba4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x999cc).

## `cmdFlashBoot`

- `cmdFlashBoot`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98fdc to 0x99728.
  Established. The unnamed `0x1943c` call carries `0x300`, entry `r1`, entry `r2`, `[[r0+112]]` (`bl` at 0x9900c).
  An included byte store clears `[[sp-28]] +0x0` (`strb` at 0x99024).
  The unnamed `0x99728` call carries `0x64`, unknown, `[[sp-28]]`, `[sp-36]` (`bl` at 0x99038).
  Boot command branch dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98fec | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x98ff0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x98ff4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x98ff8 | load | 4 | `r0 +0x70` | `-` |
| `ldr` at 0x98ffc | load | 4 | `[r0+112] +0x0` | `-` |
| `str` at 0x99000 | store | 4 | `sp -0x24` | `[[r0+112]]` |
| `ldr` at 0x99004 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x99014 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0x99018 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9901c | load | 4 | `[sp-28] +0x0` | `-` |
| `strb` at 0x99024 | store | 1 | `[[sp-28]] +0x0` | `0x0` |
| `ldr` at 0x99028 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9903c | load | 4 | `sp -0x24` | `-` |
| `ldrls` at 0x99048 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x996d0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x996d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x996d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x996d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9900c | `(unknown callee) 0x1943c` | `0x300` | `r1` | `r2` | `[[r0+112]]` |
| `bl` at 0x99038 | `(unknown callee) 0x99728` | `0x64` | `?` | `[[sp-28]]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x99030 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ldrls` at 0x99048 | `[sp-36]+3 <= 0xa (unsigned, 32-bit)` | `[sp-36]+3 > 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x996d0 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x99048).

## `imageSyncResetHold`

- `imageSyncResetHold`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceb04 to 0xceb80.
  Established. A nonzero `ret(0xcedbc)` takes `0xceb48`, while zero falls through (`bne` at 0xceb38).
  The unnamed `0xca538` call carries `0x0`, `[ret(0xcedbc)+2]:1`, `[ret(0xcedbc)+3]:1`, `[ret(0xcedbc)+3]:1` (`bl` at 0xceb64).
  An included word assignment puts `0xfffffc10` at `sp -0x1c` (`str` at 0xceb40).
  Image reset hold semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xceb18 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xceb1c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xceb2c | store | 4 | `sp -0x18` | `ret(0xcedbc)` |
| `ldr` at 0xceb30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xceb3c | load | 4 | `abs 0xceb7c` | `-` |
| `str` at 0xceb40 | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xceb48 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xceb4c | load | 1 | `ret(0xcedbc) +0x2` | `-` |
| `ldr` at 0xceb50 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xceb54 | load | 1 | `ret(0xcedbc) +0x3` | `-` |
| `str` at 0xceb6c | store | 4 | `sp -0x1c` | `ret(0xca538)` |
| `ldr` at 0xceb70 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xceb78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xceb78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xceb78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xceb24 | `(unknown callee) 0xcedbc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xceb64 | `(unknown callee) 0xca538` | `0x0` | `[ret(0xcedbc)+2]:1` | `[ret(0xcedbc)+3]:1` | `[ret(0xcedbc)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xceb38 | `ret(0xcedbc) != 0x0` | `ret(0xcedbc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xceb78 | `r0` | `?` |

## `imageSyncResetRelease`

- `imageSyncResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xceb80 to 0xcebfc.
  Established. The unnamed `0xca4e8` call carries `0x0`, `[ret(0xcedbc)+2]:1`, `[ret(0xcedbc)+3]:1`, `[ret(0xcedbc)+3]:1` (`bl` at 0xcebe0).
  A nonzero `ret(0xcedbc)` takes `0xcebc4`, while zero falls through (`bne` at 0xcebb4).
  A byte store saves `(r0 & 0xff)` at `sp -0x11` (`strb` at 0xceb94).
  Image reset release interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xceb94 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xceb98 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xceba8 | store | 4 | `sp -0x18` | `ret(0xcedbc)` |
| `ldr` at 0xcebac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xcebb8 | load | 4 | `abs 0xcebf8` | `-` |
| `str` at 0xcebbc | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xcebc4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcebc8 | load | 1 | `ret(0xcedbc) +0x2` | `-` |
| `ldr` at 0xcebcc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xcebd0 | load | 1 | `ret(0xcedbc) +0x3` | `-` |
| `str` at 0xcebe8 | store | 4 | `sp -0x1c` | `ret(0xca4e8)` |
| `ldr` at 0xcebec | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xcebf4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xcebf4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xcebf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xceba0 | `(unknown callee) 0xcedbc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xcebe0 | `(unknown callee) 0xca4e8` | `0x0` | `[ret(0xcedbc)+2]:1` | `[ret(0xcedbc)+3]:1` | `[ret(0xcedbc)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcebb4 | `ret(0xcedbc) != 0x0` | `ret(0xcedbc) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xcebf4 | `r0` | `?` |

## `promIceReset`

- `promIceReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3e7b0 to 0x3e8e0.
  Established. The unnamed `0xca584` call carries `0x0`, `[[sp-20]]:1`, `[[sp-20]+1]:1`, unknown (`bl` at 0x3e86c);
  its zero result takes `0x3e8b8`, while nonzero falls through (`beq` at 0x3e880).
  Included byte assignments clear `sp -0x19` and increment it with `([sp-25]:1+1 & 0xff)` (`strb` at 0x3e7c4, `strb` at 0x3e8c0).
  PROM reset routing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3e7c4 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x3e7c8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3e7cc | load | 4 | `abs 0x3e8d0` | `-` |
| `ldr` at 0x3e7d0 | load | 4 | `abs 0x29972c` | `-` |
| `ldrb` at 0x3e7dc | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x3e7f4 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3e7fc | load | 4 | `abs 0x3e8d4` | `-` |
| `ldr` at 0x3e800 | load | 4 | `abs 0x299730` | `-` |
| `str` at 0x3e808 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x3e80c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x3e828 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3e82c | load | 1 | `[sp-20] +0x1` | `-` |
| `strb` at 0x3e830 | store | 1 | `sp -0x1b` | `([[sp-20]+1]:1 & 0xff)` |
| `strb` at 0x3e83c | store | 1 | `sp -0x1b` | `0x0` |
| `ldrb` at 0x3e840 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x3e844 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x3e848 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3e84c | load | 1 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x3e850 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x3e854 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldrb` at 0x3e858 | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x3e874 | store | 4 | `sp -0x18` | `ret(0xca584)` |
| `ldr` at 0x3e878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x3e890 | load | 4 | `abs 0x3e8d8` | `-` |
| `str` at 0x3e894 | store | 4 | `sp -0x28` | `0xf64b4` |
| `ldrb` at 0x3e898 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x3e89c | store | 4 | `sp -0x24` | `[sp-25]:1` |
| `str` at 0x3e8a0 | store | 4 | `sp -0x20` | `ret(0x1fc1c)` |
| `ldr` at 0x3e8a8 | load | 4 | `abs 0x3e8dc` | `-` |
| `ldrb` at 0x3e8b8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x3e8c0 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldm` at 0x3e8cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x3e8cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x3e8cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3e7e4 | `(unknown callee) 0x3e93c` | `?` | `?` | `[0x29972c]` | `?` |
| `bl` at 0x3e818 | `(unknown callee) 0x1d444` | `[sp-25]:1+108` | `?` | `?` | `[sp-25]:1+108` |
| `bl` at 0x3e86c | `(unknown callee) 0xca584` | `0x0` | `[[sp-20]]:1` | `[[sp-20]+1]:1` | `?` |
| `bl` at 0x3e888 | `(unknown callee) 0x1fc1c` | `ret(0xca584)` | `?` | `?` | `ret(0xca584)` |
| `bl` at 0x3e8b4 | `(unknown callee) 0x200a0` | `0x1` | `0xf6480` | `0x68` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x3e7d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x3e7f0 | `ret(0x3e93c) == 0x0` | `ret(0x3e93c) != 0x0` |
| `bne` at 0x3e824 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0x3e880 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x3e8cc | `r0` | `?` |

## `tio_test_pci_reset_int`

- `tio_test_pci_reset_int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dc4c to 0x2dcbc.
  Established. Inequality `(r1 & 0xff) != 0xc` takes `0x2dcb4`, while equality falls through (`bne` at 0x2dc84).
  The unnamed `0x4bcfc` call carries `0x0`, `(r1 & 0xff)`, `0x2`, `(r3 & 0xff)` (`bl` at 0x2dcb0).
  The byte at `sp -0x16` receives `(r2 & 0xff)` (`strb` at 0x2dc70).
  PCI reset interrupt policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2dc5c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x2dc68 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x2dc70 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x2dc78 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x2dc7c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2dc88 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x2dc98 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x2dc9c | load | 1 | `sp -0x17` | `-` |
| `ldm` at 0x2dcb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2dcb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2dcb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dcb0 | `(unknown callee) 0x4bcfc` | `0x0` | `(r1 & 0xff)` | `0x2` | `(r3 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2dc84 | `(r1 & 0xff) != 0xc` | `(r1 & 0xff) == 0xc` |
| `beq` at 0x2dc94 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2dcb8 | `r0` | `?` |

## `L1FRU_2m_Reset`

- `L1FRU_2m_Reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99e30 to 0x99e94.
  Established. Absolute word stores clear `0x29a964` and `0x29a968` (`str` at 0x99e60, `str` at 0x99e88).
  The unnamed `0x1970c` call carries `[0x29a964]`, entry `r1`, entry `r2`, `0x29a944` (`bl` at 0x99e54).
  Zero `[0x29a968]` takes `0x99e80`, while nonzero falls through (`beq` at 0x99e70).
  FRU reset ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x99e3c | load | 4 | `abs 0x99e90` | `-` |
| `ldr` at 0x99e40 | load | 4 | `abs 0x29a964` | `-` |
| `ldr` at 0x99e4c | load | 4 | `abs 0x99e90` | `-` |
| `ldr` at 0x99e50 | load | 4 | `abs 0x29a964` | `-` |
| `ldr` at 0x99e58 | load | 4 | `abs 0x99e90` | `-` |
| `str` at 0x99e60 | store | 4 | `abs 0x29a964` | `0x0` |
| `ldr` at 0x99e64 | load | 4 | `abs 0x99e90` | `-` |
| `ldr` at 0x99e68 | load | 4 | `abs 0x29a968` | `-` |
| `ldr` at 0x99e74 | load | 4 | `abs 0x99e90` | `-` |
| `ldr` at 0x99e78 | load | 4 | `abs 0x29a968` | `-` |
| `ldr` at 0x99e80 | load | 4 | `abs 0x99e90` | `-` |
| `str` at 0x99e88 | store | 4 | `abs 0x29a968` | `0x0` |
| `ldm` at 0x99e8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x99e8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x99e8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99e54 | `(unknown callee) 0x1970c` | `[0x29a964]` | `r1` | `r2` | `0x29a944` |
| `bl` at 0x99e7c | `(unknown callee) 0x1970c` | `[0x29a968]` | `?` | `0x29a944` | `0x29a944` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x99e48 | `[0x29a964] == 0x0` | `[0x29a964] != 0x0` |
| `beq` at 0x99e70 | `[0x29a968] == 0x0` | `[0x29a968] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x99e8c | `r0` | `?` |

## `SMPConsoleReset`

- `SMPConsoleReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5da68 to 0x5dabc.
  Established. Included word assignments initialize `sp -0x14` to `0x0` and later write `[sp-20]+1` (`str` at 0x5da7c, `str` at 0x5daa8).
  The unnamed `0x5d778` call carries `0x0`, unknown, unknown, unknown (`bl` at 0x5dab4);
  its result is the integer return candidate (`ldm` at 0x5dab8).
  Console reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5da7c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x5da80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5da8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5daa0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x5daa8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0x5dab8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x5dab8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5dab8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5dab8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5da9c | `(unknown callee) 0x55300` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x5dab4 | `(unknown callee) 0x5d778` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x5da88 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5dab8 | `r0` | `ret(0x5d778)` |

## `exceptionIsSoftReset`

- `exceptionIsSoftReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1eeb0 to 0x1ef30.
  Established. Nonzero `[0x12ef68]` takes `0x1ef04`, while zero falls through (`bne` at 0x1eecc).
  The unnamed `0x19310` call carries `ret(0x19328)`, `0xeb9e0`, unknown, `ret(0x19328)` (`bl` at 0x1eef4).
  Included word stores assign `0x1` and `0x0` to `sp -0x18` on separate paths (`str` at 0x1ef08, `str` at 0x1ef14).
  Soft reset classification meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1eec0 | load | 4 | `abs 0x1ef24` | `-` |
| `ldr` at 0x1eec4 | load | 4 | `abs 0x12ef68` | `-` |
| `ldr` at 0x1eed0 | load | 4 | `abs 0x1ef28` | `-` |
| `str` at 0x1eedc | store | 4 | `sp -0x14` | `ret(0x19328)` |
| `ldr` at 0x1eee0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1eeec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1eef0 | load | 4 | `abs 0x1ef2c` | `-` |
| `str` at 0x1ef08 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x1ef14 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1ef18 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1ef20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1ef20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1ef20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1eed4 | `(unknown callee) 0x19328` | `0xeb9d0` | `r1` | `r2` | `[0x12ef68]` |
| `bl` at 0x1eef4 | `(unknown callee) 0x19310` | `ret(0x19328)` | `0xeb9e0` | `?` | `ret(0x19328)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x1eecc | `[0x12ef68] != 0x0` | `[0x12ef68] == 0x0` |
| `beq` at 0x1eee8 | `ret(0x19328) == 0x0` | `ret(0x19328) != 0x0` |
| `bne` at 0x1ef00 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1ef20 | `r0` | `?` |

## `exceptionResetReport`

- `exceptionResetReport`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1eff8 to 0x1f008.
  Established. The integer return candidate retains entry `r0` (`ldm` at 0x1f004).
  Reset reporting behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x1f004 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1f004 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1f004 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1f004 | `r0` | `r0` |

## `exceptionReset`

- `exceptionReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1ef30 to 0x1efe0.
  Established. Zero `[0x12ef68]` takes `0x1ef70`, while nonzero falls through (`beq` at 0x1ef54).
  The unnamed `0xbef2c` call has four unknown argument candidates (`bl` at 0x1ef98).
  A subsequent unnamed `0x20974` call carries `0x1388`, unknown, unknown, unknown (`bl` at 0x1efa8).
  Exception reset outcome remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1ef40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1ef44 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1ef48 | load | 4 | `abs 0x1efc4` | `-` |
| `ldr` at 0x1ef4c | load | 4 | `abs 0x12ef68` | `-` |
| `ldr` at 0x1ef58 | load | 4 | `abs 0x1efc8` | `-` |
| `ldr` at 0x1ef5c | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x1ef60 | load | 4 | `abs 0x1efcc` | `-` |
| `ldr` at 0x1ef70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1ef74 | load | 4 | `abs 0x1efd0` | `-` |
| `ldr` at 0x1ef88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1ef94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1ef9c | load | 4 | `abs 0x1efd4` | `-` |
| `ldr` at 0x1efa4 | load | 4 | `abs 0x1efd8` | `-` |
| `ldr` at 0x1efac | load | 4 | `abs 0x1efc8` | `-` |
| `ldr` at 0x1efb0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0x1efb4 | load | 4 | `abs 0x1efdc` | `-` |
| `ldrheq` at 0x1efc8 | load | 2 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1ef64 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xeb9e8` | `r2` | `0x12eeb8` |
| `bl` at 0x1ef6c | `(unknown callee) 0x196b8` | `?` | `?` | `?` | `?` |
| `bl` at 0x1ef84 | `(unknown callee) 0x196b8` | `?` | `?` | `?` | `0x3e7` |
| `bl` at 0x1ef98 | `(unknown callee) 0xbef2c` | `?` | `?` | `?` | `?` |
| `bl` at 0x1efa0 | `(unknown callee) 0x193f4` | `0xeba24` | `?` | `?` | `?` |
| `bl` at 0x1efa8 | `(unknown callee) 0x20974` | `0x1388` | `?` | `?` | `?` |
| `bl` at 0x1efb8 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0xeba34` | `?` | `0x12eeb8` |
| `bl` at 0x1efc0 | `(unknown callee) 0x196b8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1ef54 | `[0x12ef68] == 0x0` | `[0x12ef68] != 0x0` |
| `bne` at 0x1ef7c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x1ef90 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|

## `routerExitServiceMode`

- `routerExitServiceMode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xba970 to 0xbaaa0.
  Established. An included absolute word assignment writes `0x2` to `0x13246c` (`str` at 0xba9e4).
  The unnamed `0x3bcc4` calls carry `0x0`, `0x6`, `0x2`, unknown and `0x0`, `0x7`, `0x2`, unknown (`bl` at 0xbaa70, `bl` at 0xbaa80).
  Inequality `[sp-20] != 0x1` takes `0xba9dc`, while equality falls through (`bne` at 0xba9c4).
  Router service exit effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xba980 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xba984 | load | 4 | `abs 0xbaa94` | `-` |
| `ldm` at 0xba98c | load | 4 | `abs 0x11ec50` | `-` |
| `ldm` at 0xba98c | load | 4 | `abs 0x11ec54` | `-` |
| `ldm` at 0xba98c | load | 4 | `abs 0x11ec58` | `-` |
| `stm` at 0xba990 | store | 4 | `sp -0x20` | `[0x11ec50]` |
| `stm` at 0xba990 | store | 4 | `sp -0x1c` | `[0x11ec54]` |
| `strb` at 0xba994 | store | 1 | `sp -0x18` | `([0x11ec58] & 0xff)` |
| `ldr` at 0xba9bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xba9dc | load | 4 | `abs 0xbaa98` | `-` |
| `str` at 0xba9e4 | store | 4 | `abs 0x13246c` | `0x2` |
| `ldr` at 0xbaa84 | load | 4 | `abs 0xbaa9c` | `-` |
| `ldm` at 0xbaa90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbaa90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbaa90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xba9a0 | `(unknown callee) 0x1b5f0` | `0x50` | `0x0` | `[0x11ec58]` | `0x11ec50` |
| `bl` at 0xba9ac | `(unknown callee) 0x1b5f0` | `0x3e` | `0x0` | `?` | `?` |
| `bl` at 0xba9b8 | `(unknown callee) 0x1b5f0` | `0x3f` | `0x0` | `?` | `?` |
| `bl` at 0xba9d4 | `(unknown callee) 0x1b5f0` | `0xd` | `sp-32` | `?` | `sp-32` |
| `bl` at 0xba9f4 | `(unknown callee) 0x1b5f0` | `0xd` | `sp-32` | `0x13246c` | `sp-32` |
| `bl` at 0xbaa00 | `(unknown callee) 0x1b5f0` | `0x3d` | `0x2` | `?` | `?` |
| `bl` at 0xbaa0c | `(unknown callee) 0x1b5f0` | `0x7b` | `0x1` | `?` | `?` |
| `bl` at 0xbaa10 | `(unknown callee) 0x488cc` | `ret(0x1b5f0)` | `?` | `?` | `?` |
| `bl` at 0xbaa24 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0xbaa38 | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0xbaa4c | `(unknown callee) 0x488cc` | `ret(0x488cc)` | `?` | `?` | `?` |
| `bl` at 0xbaa70 | `(unknown callee) 0x3bcc4` | `0x0` | `0x6` | `0x2` | `?` |
| `bl` at 0xbaa80 | `(unknown callee) 0x3bcc4` | `0x0` | `0x7` | `0x2` | `?` |
| `bl` at 0xbaa88 | `(unknown callee) 0x1e4ac` | `0x11ec5c` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xba9c4 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0xbaa20 | `? == 0x52` | `? != 0x52` |
| `beq` at 0xbaa34 | `? == 0x44` | `? != 0x44` |
| `beq` at 0xbaa48 | `? == 0x36` | `? != 0x36` |
| `beq` at 0xbaa5c | `? == 0x40` | `? != 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbaa90 | `r0` | `ret(0x1e4ac)` |
