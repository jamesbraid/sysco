# ARM TDR fields and chip access builders in 1.40.5

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/tdr-field-access-facts.txt) retain all tool sections.

## `tdrdata_create`

- `tdrdata_create`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1164 to 0xf11f4.
  Established. The unnamed `0x1b1b4` call carries `0x14`, `r1`, `r2`, `r1` (`bl` at 0xf1188); its
  nonzero result takes `0xf11ac`, while zero falls through (`bne` at 0xf119c). The later unnamed
  `0xf11f4` call carries `ret(0x1b1b4)`, `[sp-20]`, `[sp-22]:2`, `[sp-28]` (`bl` at 0xf11c0).
  Allocation and initialization roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1174 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf117c | store | 4 | `sp -0x1c` | `r2` |
| `strh` at 0xf1180 | store | 2 | `sp -0x16` | `(r1 & 0xffff)` |
| `str` at 0xf1190 | store | 4 | `sp -0x20` | `ret(0x1b1b4)` |
| `ldr` at 0xf1194 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf11a4 | store | 4 | `sp -0x24` | `0x0` |
| `ldrh` at 0xf11ac | load | 2 | `sp -0x16` | `-` |
| `ldr` at 0xf11b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf11b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf11bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf11d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf11dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf11e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf11e8 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0xf11ec | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf11f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf11f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf11f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1188 | `(unknown callee) 0x1b1b4` | `0x14` | `r1` | `r2` | `r1` |
| `bl` at 0xf11c0 | `(unknown callee) 0xf11f4` | `ret(0x1b1b4)` | `[sp-20]` | `[sp-22]:2` | `[sp-28]` |
| `bl` at 0xf11d4 | `(unknown callee) 0x1b490` | `[sp-32]` | `?` | `?` | `ret(0xf11f4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf119c | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `bne` at 0xf11cc | `ret(0xf11f4) != 0x0` | `ret(0xf11f4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf11f0 | `r0` | `?` |

## `tdrdata_init`

- `tdrdata_init`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf11f4 to 0xf1394.
  Established. The halfword at `[sp-24] +0x6` receives `0x0` (`strh` at 0xf126c). The word at
  `[sp-24] +0x8` receives `[sp-36]` (`str` at 0xf1278). The unnamed `0x1b4cc` call carries
  `[[sp-24]+12]`, `0x0`, `[[sp-36]+2]:2`, `[sp-24]` (`bl` at 0xf1378). Record field meanings remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1204 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf1208 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xf120c | store | 4 | `sp -0x24` | `r3` |
| `strh` at 0xf1210 | store | 2 | `sp -0x1e` | `(r2 & 0xffff)` |
| `ldr` at 0xf1214 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1220 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1224 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf122c | store | 4 | `r0 +0x0` | `ret(0x1b034)` |
| `ldr` at 0xf1230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1234 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf1244 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf124c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1254 | store | 4 | `r0 +0x0` | `0x0` |
| `ldr` at 0xf1258 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xf125c | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xf1260 | store | 2 | `[sp-24] +0x4` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xf1264 | load | 4 | `sp -0x18` | `-` |
| `strh` at 0xf126c | store | 2 | `[sp-24] +0x6` | `0x0` |
| `ldr` at 0xf1270 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1274 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1278 | store | 4 | `[sp-24] +0x8` | `[sp-36]` |
| `ldr` at 0xf127c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf1280 | load | 2 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf12a0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf12a4 | load | 2 | `[sp-36] +0x0` | `-` |
| `str` at 0xf12bc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf12c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf12c4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf12d0 | store | 4 | `[sp-24] +0x10` | `ret(0x1b1b4)` |
| `ldr` at 0xf12d4 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xf12e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf12e4 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf12f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf12f4 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf1300 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf1308 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf130c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf1310 | load | 2 | `[sp-36] +0x2` | `-` |
| `str` at 0xf1320 | store | 4 | `[sp-24] +0xc` | `ret(0x1b1b4)` |
| `ldr` at 0xf1324 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf1330 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1334 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf1340 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1344 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xf134c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1350 | load | 4 | `[sp-24] +0x10` | `-` |
| `str` at 0xf135c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf1364 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1368 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xf136c | load | 2 | `[sp-36] +0x2` | `-` |
| `ldr` at 0xf1370 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf137c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1388 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xf138c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xf1390 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf1390 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1390 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1228 | `(unknown callee) 0x1b034` | `r1` | `r1` | `r2` | `r1` |
| `bl` at 0xf12c8 | `(unknown callee) 0x1b1b4` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xf12f8 | `(unknown callee) 0x1b490` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf1318 | `(unknown callee) 0x1b1b4` | `[[sp-36]+2]:2` | `?` | `?` | `[[sp-36]+2]:2` |
| `bl` at 0xf1348 | `(unknown callee) 0x1b490` | `[[sp-24]]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf1354 | `(unknown callee) 0x1b490` | `[[sp-24]+16]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xf1378 | `(unknown callee) 0x1b4cc` | `[[sp-24]+12]` | `0x0` | `[[sp-36]+2]:2` | `[sp-24]` |
| `bl` at 0xf1380 | `(unknown callee) 0xf1d98` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf121c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xf123c | `[[sp-24]] != 0x0` | `[[sp-24]] == 0x0` |
| `bne` at 0xf12dc | `[[sp-24]+16] != 0x0` | `[[sp-24]+16] == 0x0` |
| `beq` at 0xf12ec | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |
| `bne` at 0xf132c | `[[sp-24]+12] != 0x0` | `[[sp-24]+12] == 0x0` |
| `beq` at 0xf133c | `[[sp-24]] == 0x0` | `[[sp-24]] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1390 | `r0` | `?` |

## `tdrdata_dump`

- `tdrdata_dump`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1394 to 0xf16d4.
  Established. Entry `r2 != 0x0` takes `0xf13c4`, while zero falls through (`bne` at 0xf13bc). The
  absolute word at `0x145ae4` is read (`ldr` at 0xf13d4). The unnamed `0xf16d4` call carries
  `[sp-20]`, `[sp-28]`, `0x1`, unknown (`bl` at 0xf169c). Dump output meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf13a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf13a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf13ac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf13b0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf13b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf13c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13d0 | load | 4 | `abs 0xf16a4` | `-` |
| `ldr` at 0xf13d4 | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xf13d8 | store | 4 | `sp -0x14` | `[0x145ae4]` |
| `ldr` at 0xf13dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf13e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13ec | load | 4 | `abs 0xf16a8` | `-` |
| `ldr` at 0xf13f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf13fc | load | 4 | `abs 0xf16ac` | `-` |
| `ldr` at 0xf1400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf140c | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xf1410 | store | 4 | `sp -0x34` | `[[sp-28]]` |
| `ldr` at 0xf1414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1418 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xf1424 | load | 4 | `abs 0xf16b0` | `-` |
| `str` at 0xf1428 | store | 4 | `sp -0x34` | `0x137c7c` |
| `ldr` at 0xf142c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1430 | load | 4 | `abs 0xf16b4` | `-` |
| `ldr` at 0xf1434 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf143c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf1440 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xf1444 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf1448 | load | 2 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xf144c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1450 | load | 4 | `abs 0xf16b8` | `-` |
| `ldr` at 0xf1458 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf145c | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xf1460 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xf1464 | load | 2 | `[sp-28] +0x6` | `-` |
| `ldr` at 0xf1468 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf146c | load | 4 | `abs 0xf16bc` | `-` |
| `ldr` at 0xf1474 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1488 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xf1494 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1498 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf149c | load | 4 | `abs 0xf16c0` | `-` |
| `ldr` at 0xf14a0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xf14a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf14b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf14bc | load | 4 | `abs 0xf16c4` | `-` |
| `ldr` at 0xf14c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf14c8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xf14cc | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xf14dc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf14e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf14e4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xf14e8 | load | 2 | `[[sp-28]+8] +0x2` | `-` |
| `str` at 0xf14f8 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xf1500 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf1504 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1508 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf151c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xf1520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf1530 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1534 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf153c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf1544 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0xf154c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1550 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1554 | load | 4 | `abs 0xf16c8` | `-` |
| `ldr` at 0xf1560 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf1568 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xf1570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1574 | load | 4 | `abs 0xf16cc` | `-` |
| `ldr` at 0xf157c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1584 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf158c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf159c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf15a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf15a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf15b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf15b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf15c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf15c8 | load | 4 | `[sp-28] +0xc` | `-` |
| `ldrb` at 0xf15d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf15d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf15d8 | load | 4 | `abs 0xf16c8` | `-` |
| `ldr` at 0xf15e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf15ec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf15f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf15f8 | load | 4 | `abs 0xf16cc` | `-` |
| `ldr` at 0xf1600 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1604 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xf1608 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `ldr` at 0xf1620 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1624 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xf1628 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xf1638 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf1640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1644 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrh` at 0xf1648 | load | 2 | `[[sp-28]+8] +0x0` | `-` |
| `str` at 0xf1658 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf165c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf1664 | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0xf1668 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf166c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1670 | load | 4 | `abs 0xf16d0` | `-` |
| `ldr` at 0xf1674 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf1678 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xf1680 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1690 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1694 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf16a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf13f0 | `(unknown callee) 0x240b8` | `?` | `0x137c58` | `r2` | `r1` |
| `bl` at 0xf1404 | `(unknown callee) 0x240b8` | `?` | `0x137c78` | `r1` | `r1` |
| `bl` at 0xf1438 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137c84` | `?` | `?` |
| `bl` at 0xf1454 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137c90` | `[[sp-28]+4]:2` | `[[sp-28]+4]:2` |
| `bl` at 0xf1470 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137ca8` | `[[sp-28]+6]:2` | `[[sp-28]+6]:2` |
| `bl` at 0xf14a4 | `(unknown callee) 0xf09f0` | `[sp-20]` | `0x137cc0` | `[[sp-28]+8]` | `[sp-28]` |
| `bl` at 0xf14c0 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137cdc` | `?` | `?` |
| `bl` at 0xf155c | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137cf8` | `?` | `?` |
| `bl` at 0xf1578 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137d00` | `?` | `?` |
| `bl` at 0xf15e0 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137cf8` | `?` | `?` |
| `bl` at 0xf15fc | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137d00` | `?` | `[sp-48]` |
| `bl` at 0xf167c | `(unknown callee) 0xf81f4` | `[sp-20]` | `0x137d04` | `[sp-56]` | `[[sp-28]+16]` |
| `bl` at 0xf169c | `(unknown callee) 0xf16d4` | `[sp-20]` | `[sp-28]` | `0x1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf13bc | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xf13cc | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf13e4 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xf1420 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0xf1480 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xf1490 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0xf14b4 | `? == 0x0` | `? != 0x0` |
| `blo` at 0xf1510 | `C == 0` | `not (C == 0)` |
| `bls` at 0xf1528 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0xf1594 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `blo` at 0xf15ac | `C == 0` | `not (C == 0)` |
| `bne` at 0xf161c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xf168c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf16a0 | `r0` | `?` |

## `tdrdata_reset_field`

- `tdrdata_reset_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1f74 to 0xf2040.
  Established. The word at `r0 +0x10` is read (`ldr` at 0xf201c). The unnamed `0xe2b10` call carries
  unknown, `[[r0+8]+8]`, unknown, `[r0+16]` (`bl` at 0xf2020). A byte assignment through
  `([[sp-24]+12] + [sp-28]) +0x0` writes `0x0` (`strb` at 0xf2038). Reset field semantics remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1f84 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf1f88 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xf1f8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1f90 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1f94 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1fa8 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf1fb0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1fb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1fb8 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1fbc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1fc0 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1fd8 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf1fe0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1fec | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xf1ff0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2004 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xf200c | load | 2 | `(unknown)` | `-` |
| `str` at 0xf2010 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf2014 | load | 4 | `[r0+8] +0x8` | `-` |
| `ldr` at 0xf201c | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xf2024 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2028 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf202c | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xf2038 | store | 1 | `([[sp-24]+12] + [sp-28]) +0x0` | `0x0` |
| `ldmdb` at 0xf203c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf203c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf203c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf203c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf2020 | `(unknown callee) 0xe2b10` | `?` | `[[r0+8]+8]` | `?` | `[r0+16]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf203c | `r0` | `ret(0xe2b10)` |

## `tdrdata_dump_fields`

- `tdrdata_dump_fields`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf16d4 to 0xf1a10.
  Established. The absolute word at `0x2ae118` receives `0x0` (`str` at 0xf1830). A later word
  assignment there writes `[0x2ae118]+1` (`str` at 0xf1930). The unnamed `0xf0de4` call carries
  `[[sp-24]+8]`, `[sp-32]`, `0x0`, `[[sp-24]+16]` (`bl` at 0xf1958). Field dump policy remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf16e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf16e8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf16ec | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf16f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf16fc | load | 4 | `abs 0xf1a00` | `-` |
| `ldr` at 0xf1700 | load | 4 | `abs 0x145ae4` | `-` |
| `str` at 0xf1704 | store | 4 | `sp -0x14` | `[0x145ae4]` |
| `ldr` at 0xf1708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf170c | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0xf171c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1720 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf1724 | load | 2 | `[r1+8] +0x0` | `-` |
| `ldr` at 0xf173c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1740 | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf1744 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xf1754 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf1758 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf1760 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf1768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf176c | load | 4 | `r1 +0x8` | `-` |
| `ldrh` at 0xf1770 | load | 2 | `[r1+8] +0x0` | `-` |
| `str` at 0xf1780 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf1784 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf1790 | store | 4 | `sp -0x78` | `?` |
| `ldr` at 0xf1794 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xf17a0 | store | 4 | `sp -0x24` | `ret(0x1b1b4)` |
| `ldr` at 0xf17a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf17b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf17b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf17bc | load | 4 | `abs 0xf1a04` | `-` |
| `ldr` at 0xf17c0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xf17cc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf17d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf17d4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xf17d8 | load | 2 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xf17dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf17ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf17f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1804 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1808 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf180c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf1814 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf1828 | load | 4 | `abs 0xf1a08` | `-` |
| `str` at 0xf1830 | store | 4 | `abs 0x2ae118` | `0x0` |
| `ldr` at 0xf1834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1838 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf183c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1850 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf1858 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1870 | load | 4 | `abs 0xf1a08` | `-` |
| `ldr` at 0xf1874 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1878 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf187c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1890 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf1898 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf18a8 | load | 4 | `abs 0x2ae118` | `-` |
| `ldr` at 0xf18b8 | load | 4 | `abs 0xf1a08` | `-` |
| `ldr` at 0xf18bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf18c0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf18c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf18d8 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf18e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf18f4 | load | 4 | `abs 0x2ae118` | `-` |
| `ldr` at 0xf1904 | load | 4 | `abs 0xf1a08` | `-` |
| `ldr` at 0xf1908 | load | 4 | `abs 0x2ae118` | `-` |
| `ldr` at 0xf1910 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf191c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf1920 | load | 4 | `abs 0xf1a08` | `-` |
| `ldr` at 0xf1924 | load | 4 | `abs 0xf1a08` | `-` |
| `ldr` at 0xf1928 | load | 4 | `abs 0x2ae118` | `-` |
| `str` at 0xf1930 | store | 4 | `abs 0x2ae118` | `[0x2ae118]+1` |
| `ldr` at 0xf1938 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf193c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1940 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1944 | store | 4 | `sp -0x7c` | `[sp-36]` |
| `ldr` at 0xf1948 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf194c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1954 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xf195c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1960 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf1964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1978 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldrh` at 0xf1980 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf198c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1994 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1998 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xf199c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf19b0 | load | 4 | `[[sp-24]+8] +0x4` | `-` |
| `ldr` at 0xf19b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf19bc | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xf19c0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf19c8 | load | 1 | `([[sp-24]+12] + [sp-32]) +0x0` | `-` |
| `ldr` at 0xf19d0 | load | 4 | `abs 0xf1a0c` | `-` |
| `ldr` at 0xf19d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf19e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf19ec | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xf19f4 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xf19fc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf19fc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf19fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf1798 | `(unknown callee) 0x1b1b4` | `[sp-120]` | `r1` | `r2` | `?` |
| `bl` at 0xf17c4 | `(unknown callee) 0x240b8` | `[sp-20]` | `0x137d18` | `[[sp-24]]` | `[sp-24]` |
| `bl` at 0xf1958 | `(unknown callee) 0xf0de4` | `[[sp-24]+8]` | `[sp-32]` | `0x0` | `[[sp-24]+16]` |
| `bl` at 0xf1990 | `(unknown callee) 0xf1a10` | `?` | `[sp-36]` | `sp-116` | `?` |
| `bl` at 0xf19e0 | `(unknown callee) 0x23fbc` | `0x137d38` | `?` | `[([[sp-24]+12] + [sp-32])]:1` | `sp-116` |
| `bl` at 0xf19f8 | `(unknown callee) 0x1b490` | `[sp-36]` | `?` | `[[[sp-24]+8]+2]:2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf16f8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xf1714 | `[r1+8] != 0x0` | `[r1+8] == 0x0` |
| `bne` at 0xf1738 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf17ac | `ret(0x1b1b4) != 0x0` | `ret(0x1b1b4) == 0x0` |
| `blo` at 0xf17e4 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf17f4 | `[sp-28] == 0x1` | `[sp-28] != 0x1` |
| `bne` at 0xf1800 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0xf1820 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xf186c | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf18b0 | `[0x2ae118] < ? (unsigned, 32-bit)` | `[0x2ae118] >= ? (unsigned, 32-bit)` |
| `blo` at 0xf18fc | `[0x2ae118] < ? (unsigned, 32-bit)` | `[0x2ae118] >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf19fc | `r0` | `?` |

## `tdrdata_create_field_str`

- `tdrdata_create_field_str`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1a10 to 0xf1b24.
  Established. The word at `sp -0x34` receives `0x137d58` (`str` at 0xf1a30). One byte assignment
  has unknown target and value (`strb` at 0xf1ac0). Another byte assignment writes `0x0` to an
  unknown target (`strb` at 0xf1b18). String construction details remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1a20 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1a24 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf1a28 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf1a2c | load | 4 | `abs 0xf1b20` | `-` |
| `str` at 0xf1a30 | store | 4 | `sp -0x34` | `0x137d58` |
| `str` at 0xf1a38 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf1a3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1a50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf1a5c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf1a60 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xf1a70 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xf1a78 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1a7c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1a80 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf1a90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1a94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf1a9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1aa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1aa8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf1ab4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xf1abc | load | 1 | `(unknown)` | `-` |
| `strb` at 0xf1ac0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xf1ac4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1acc | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf1ad0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1ae0 | store | 4 | `sp -0x2c` | `0x1c` |
| `ldr` at 0xf1ae4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf1aec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xf1af4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf1afc | store | 4 | `sp -0x2c` | `[sp-44]-4` |
| `ldr` at 0xf1b00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1b08 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xf1b10 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xf1b18 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0xf1b1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1b1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1b1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf1a88 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1ad8 | `[sp-44] != 0x0` | `[sp-44] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1b1c | `r0` | `r0` |

## `tdrdata_set_data`

- `tdrdata_set_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1b24 to 0xf1bf8.
  Established. The word assignment at `sp -0x1c` initializes it to `0x0` (`str` at 0xf1b40). The
  later word assignment at an unknown effective target has an unknown value (`str` at 0xf1be0). The
  return reports entry `r0` (`ldmdb` at 0xf1bf4). Data transfer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1b34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1b38 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf1b40 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf1b44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1b48 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1b4c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1b64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1b68 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1b6c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1b7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1b8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1b90 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1b94 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1ba8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1bbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1bc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1bcc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1bd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1bdc | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1be0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1be4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf1bec | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0xf1bf4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1bf4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1bf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf1b60 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1b84 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1bb0 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1bf4 | `r0` | `r0` |

## `tdrdata_get_data`

- `tdrdata_get_data`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1bf8 to 0xf1d98.
  Established. Entry `r1 == 0x0` takes `0xf1cdc`, while nonzero falls through (`beq` at 0xf1c1c).
  Both included word assignments have unknown effective targets and values (`str` at 0xf1cc8, `str` at 0xf1d80). The final return value is unknown (`ldmdb` at 0xf1d94). Data selection meanings
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1c08 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf1c0c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf1c10 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf1c14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1c24 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1c28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1c2c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1c30 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1c48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1c4c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1c50 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1c60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1c70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1c74 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1c78 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1c8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1c9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1ca4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1cac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1cb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1cb4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1cbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1cc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1cc8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1ccc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1cd4 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `str` at 0xf1ce0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf1ce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1ce8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1cec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1d04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1d08 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1d0c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1d1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1d30 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1d34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1d48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d58 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf1d68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1d6c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf1d74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1d7c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1d80 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1d84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf1d8c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xf1d94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1d94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf1c1c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xf1c44 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1c68 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1c94 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1d00 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1d24 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1d50 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1d94 | `r0` | `?` |

## `tdrdata_reset`

- `tdrdata_reset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf1d98 to 0xf1f74.
  Established. The word through `r0 +0x8` is read (`ldr` at 0xf1db0); its zero value takes
  `0xf1e7c`, while nonzero falls through (`beq` at 0xf1db8). A word assignment and a byte assignment
  each write `0x0` at unknown effective targets (`str` at 0xf1f10, `strb` at 0xf1f5c). Reset scope
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf1da8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf1dac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1db0 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xf1dc0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf1dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1dc8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1dcc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1de8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1dec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1dfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1e10 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1e14 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1e28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1e3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1e50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1e54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1e5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1e64 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1e68 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf1e6c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1e74 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xf1e80 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf1e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1e88 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1e8c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1ea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1ea8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1eac | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1ebc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1ed0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1ed4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1ee8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1f04 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf1f10 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xf1f14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1f1c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xf1f28 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf1f2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1f30 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf1f34 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf1f38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf1f48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf1f4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf1f50 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xf1f5c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xf1f60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf1f68 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0xf1f70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf1f70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf1f70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xf1db8 | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `bne` at 0xf1de0 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1e04 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1e30 | `C == 0` | `not (C == 0)` |
| `bne` at 0xf1ea0 | `? != 0x0` | `? == 0x0` |
| `blo` at 0xf1ec4 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1ef0 | `C == 0` | `not (C == 0)` |
| `blo` at 0xf1f40 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf1f70 | `r0` | `?` |

## `tdrdata_override_field`

- `tdrdata_override_field`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf2040 to 0xf20b0.
  Established. The byte through `([r0+12] + r1) +0x0` is read and assigned an unknown value (`ldrb` at 0xf207c, `strb` at 0xf2084). The unnamed `0xf0b9c` call carries `[[sp-20]+8]`, `[sp-24]`,
  `[sp-28]`, `0x0` (`bl` at 0xf20a8); its result is reported by the return (`ldmdb` at 0xf20ac).
  Override policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf2050 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf2054 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf2058 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xf205c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2060 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf2064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf206c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2070 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xf2074 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xf207c | load | 1 | `([r0+12] + r1) +0x0` | `-` |
| `strb` at 0xf2084 | store | 1 | `([r0+12] + r1) +0x0` | `?` |
| `ldr` at 0xf2088 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf208c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2090 | load | 4 | `[sp-20] +0x10` | `-` |
| `str` at 0xf2094 | store | 4 | `sp -0x20` | `[[sp-20]+16]` |
| `ldr` at 0xf2098 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf209c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf20a0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xf20ac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf20ac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf20ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf20a8 | `(unknown callee) 0xf0b9c` | `[[sp-20]+8]` | `[sp-24]` | `[sp-28]` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf20ac | `r0` | `ret(0xf0b9c)` |

## `tdrdata_add_overrides`

- `tdrdata_add_overrides`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf20b0 to 0xf21bc.
  Established. A byte read has an unknown effective target (`ldrb` at 0xf2100). A zero tested value
  takes `0xf21a8`, while nonzero falls through (`beq` at 0xf210c). The unnamed `0xe2b10` call has
  four unknown register candidates (`bl` at 0xf21a4). Override aggregation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf20c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf20c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf20c8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf20d0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xf20d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf20d8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf20dc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf20e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf20f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf20f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf20f8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xf2100 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2114 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf2118 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf212c | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf2134 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf2138 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf213c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2140 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf2144 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2158 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf2160 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf2164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf216c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf2174 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf2188 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xf2190 | load | 2 | `(unknown)` | `-` |
| `str` at 0xf2194 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf2198 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf21a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf21a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf21b0 | store | 4 | `sp -0x20` | `?` |
| `ldmdb` at 0xf21b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf21b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf21b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf21a4 | `(unknown callee) 0xe2b10` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf20e8 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf210c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf21b8 | `r0` | `?` |

## `tdrdata_build_mask`

- `tdrdata_build_mask`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf21bc to 0xf22b8.
  Established. The halfword through `[r0+8] +0x0` is read (`ldrh` at 0xf21e4). The unnamed `0xe28dc`
  call carries `r2`, `[[r0+8]]:2`, `r3`, `[[r0+8]]:2` (`bl` at 0xf21f4). A later unnamed `0xe29fc`
  call carries unknown, unknown, `[sp-32]`, unknown (`bl` at 0xf22a0). Mask meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf21cc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf21d0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xf21d4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xf21d8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xf21dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf21e0 | load | 4 | `r0 +0x8` | `-` |
| `ldrh` at 0xf21e4 | load | 2 | `[r0+8] +0x0` | `-` |
| `ldr` at 0xf21e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf21f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf21fc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xf2200 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2204 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldrh` at 0xf2208 | load | 2 | `[[sp-20]+8] +0x2` | `-` |
| `ldr` at 0xf220c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf221c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2220 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0xf2224 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xf222c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf2230 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf2240 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2244 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf2248 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf225c | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xf2264 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf2268 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf2270 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xf2274 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0xf2278 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf228c | load | 4 | `[[sp-20]+8] +0x4` | `-` |
| `ldrh` at 0xf2294 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf229c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf22a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf22ac | store | 4 | `sp -0x24` | `?` |
| `ldmdb` at 0xf22b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf22b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf22b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf21f4 | `(unknown callee) 0xe28dc` | `r2` | `[[r0+8]]:2` | `r3` | `[[r0+8]]:2` |
| `bl` at 0xf22a0 | `(unknown callee) 0xe29fc` | `?` | `?` | `[sp-32]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xf2214 | `C == 0` | `not (C == 0)` |
| `beq` at 0xf223c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf22b4 | `r0` | `?` |

## `setup_tdrdata_lbi`

- `setup_tdrdata_lbi`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf7c9c to 0xf7cc4.
  Established. The unnamed `0xf7680` call carries entry `r0`, `r1`, `r2`, `r3` (`bl` at 0xf7cb4);
  its result is returned (`ldmdb` at 0xf7cc0). LBI setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf7cac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xf7cb0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xf7cc0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xf7cc0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xf7cc0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf7cb4 | `(unknown callee) 0xf7680` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xf7cc0 | `r0` | `ret(0xf7680)` |

## `build_dr_access_chip`

- `build_dr_access_chip`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebfe0 to 0xec0d0.
  Established. The word through `[sp] +0x0` receives `(r0 + r2)-1` (`str` at 0xec014). Equality
  `[sp-32] == [sp+4]` takes `0xec0c8`, while inequality falls through (`beq` at 0xec024). The
  unnamed `0xe2b10` call carries `0x0`, `[sp-32]`, `[sp-24]`, `[sp+4]` (`bl` at 0xec0c4). Chip
  access construction meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebff0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebff4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebff8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xebffc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xec000 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xec004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xec008 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec014 | store | 4 | `[sp] +0x0` | `(r0 + r2)-1` |
| `ldr` at 0xec018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec01c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xec028 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xec02c | load | 4 | `[sp] +0x0` | `-` |
| `ldr` at 0xec040 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xec044 | load | 4 | `[sp] +0x0` | `-` |
| `str` at 0xec050 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xec054 | load | 4 | `abs 0xec0cc` | `-` |
| `str` at 0xec05c | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xec060 | load | 4 | `abs 0xec0cc` | `-` |
| `ldr` at 0xec064 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xec068 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xec078 | load | 4 | `abs 0xec0cc` | `-` |
| `ldr` at 0xec07c | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xec084 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xec090 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xec094 | load | 4 | `abs 0xec0cc` | `-` |
| `ldr` at 0xec098 | load | 4 | `abs 0xec0cc` | `-` |
| `ldr` at 0xec09c | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xec0a4 | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xec0ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xec0b0 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xec0b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xec0bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xec0c0 | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0xec0c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xec0c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xec0c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xec0c4 | `(unknown callee) 0xe2b10` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp+4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xec024 | `[sp-32] == [sp+4]` | `[sp-32] != [sp+4]` |
| `blo` at 0xec070 | `[0x2ae004] < [sp-36] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xec0c8 | `r0` | `?` |

## `build_ir_access_chip`

- `build_ir_access_chip`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xebd58 to 0xebe18.
  Established. The absolute word at `0x2ae004` receives `0x0` (`str` at 0xebda4); its later word
  assignment writes `[0x2ae004]+1` (`str` at 0xebdec). The unnamed `0xe2b10` call carries `0x0`,
  `[sp-32]`, `[sp-24]`, `[sp]` (`bl` at 0xebe0c). Instruction-register access effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xebd68 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xebd6c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xebd70 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xebd74 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xebd78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xebd8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xebd98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xebd9c | load | 4 | `abs 0xebe14` | `-` |
| `str` at 0xebda4 | store | 4 | `abs 0x2ae004` | `0x0` |
| `ldr` at 0xebda8 | load | 4 | `abs 0xebe14` | `-` |
| `ldr` at 0xebdac | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xebdb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xebdc0 | load | 4 | `abs 0xebe14` | `-` |
| `ldr` at 0xebdc4 | load | 4 | `abs 0x2ae004` | `-` |
| `ldr` at 0xebdcc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xebdd8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xebddc | load | 4 | `abs 0xebe14` | `-` |
| `ldr` at 0xebde0 | load | 4 | `abs 0xebe14` | `-` |
| `ldr` at 0xebde4 | load | 4 | `abs 0x2ae004` | `-` |
| `str` at 0xebdec | store | 4 | `abs 0x2ae004` | `[0x2ae004]+1` |
| `ldr` at 0xebdf4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xebdf8 | store | 4 | `sp -0x28` | `[sp-28]` |
| `ldr` at 0xebe00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xebe04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xebe08 | load | 4 | `sp +0x0` | `-` |
| `ldmdb` at 0xebe10 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xebe10 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xebe10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xebe0c | `(unknown callee) 0xe2b10` | `0x0` | `[sp-32]` | `[sp-24]` | `[sp]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xebdb8 | `[0x2ae004] < [sp-36] (unsigned, 32-bit)` | `[0x2ae004] >= [sp-36] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xebe10 | `r0` | `ret(0xe2b10)` |
