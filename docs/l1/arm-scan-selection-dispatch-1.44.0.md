# ARM scan selection and dispatch paths in 1.44.0

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
`sn1sc_process_scancmd` has one unresolved computed transfer;
its tables cover known edges only. The other fourteen routines report none.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/scan-selection-dispatch-facts.txt) retain all tool sections.

## `sn1sc_process_scancmd`

- `sn1sc_process_scancmd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd7168 to 0xd8a64.
  Established. An included byte assignment clears absolute `0x29c468` (`strb` at 0xd71d0).
  Unsigned `[0x29737c] <= 0x2` takes `0xd71a4`, while greater values fall through (`bls` at 0xd7188).
  The indirect call carries unknown, unknown, `[0x12da68]`, unknown (`mov` at 0xd7264).
  Scan command dispatch semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd7178 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xd717c | load | 4 | `abs 0xd80f0` | `-` |
| `ldr` at 0xd7180 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd7190 | load | 4 | `abs 0xd80bc` | `-` |
| `ldr` at 0xd7198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71a4 | load | 4 | `abs 0xd80f0` | `-` |
| `ldr` at 0xd71a8 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd71b8 | load | 4 | `abs 0xd80c0` | `-` |
| `ldr` at 0xd71c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71c8 | load | 4 | `abs 0xd80e8` | `-` |
| `strb` at 0xd71d0 | store | 1 | `abs 0x29c468` | `0x0` |
| `ldr` at 0xd71d4 | load | 4 | `abs 0xd80c4` | `-` |
| `ldr` at 0xd71d8 | load | 4 | `abs 0x297378` | `-` |
| `str` at 0xd71e8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd71ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd71f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd71fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7200 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd720c | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xd7214 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xd7220 | load | 4 | `abs 0xd80c8` | `-` |
| `ldr` at 0xd7228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7230 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd7240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7248 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd7254 | load | 4 | `abs 0x12da68` | `-` |
| `ldr` at 0xd7278 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7280 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd7290 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd7298 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xd72a4 | load | 4 | `abs 0xd80e8` | `-` |
| `ldr` at 0xd72a8 | load | 4 | `abs 0xd80cc` | `-` |
| `ldr` at 0xd72b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd72bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd72c0 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd72c8 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xd72d0 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xd72d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd72dc | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xd72f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd8990 | load | 4 | `abs 0xd80e8` | `-` |
| `ldr` at 0xd8994 | load | 4 | `abs 0xd80e0` | `-` |
| `str` at 0xd89a0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xd89a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd89ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd89b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd89bc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xd89c8 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xd89cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd89d8 | load | 4 | `abs 0xd80f0` | `-` |
| `ldr` at 0xd89dc | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd89e8 | load | 4 | `abs 0xd80e8` | `-` |
| `ldrb` at 0xd89ec | load | 1 | `abs 0x29c468` | `-` |
| `ldr` at 0xd89f8 | load | 4 | `abs 0xd80e4` | `-` |
| `ldr` at 0xd89fc | load | 4 | `abs 0xd80e8` | `-` |
| `ldr` at 0xd8a04 | load | 4 | `abs 0xd80f0` | `-` |
| `ldr` at 0xd8a08 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd8a18 | load | 4 | `abs 0xd80ec` | `-` |
| `ldr` at 0xd8a20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8a2c | load | 4 | `abs 0xd80f0` | `-` |
| `ldr` at 0xd8a30 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd8a40 | load | 4 | `abs 0xd80f4` | `-` |
| `ldr` at 0xd8a48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd8a50 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd8a54 | store | 4 | `sp -0x5c` | `[sp-28]` |
| `ldr` at 0xd8a58 | load | 4 | `sp -0x5c` | `-` |
| `ldm` at 0xd8a60 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd8a60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd8a60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd8a60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd719c | `(unknown callee) 0xd0168` | `0x0` | `0x12147c` | `0x1` | `r0` |
| `bl` at 0xd71c4 | `(unknown callee) 0xd0168` | `0x0` | `0x1214a0` | `0x0` | `r0` |
| `bl` at 0xd721c | `(unknown callee) 0xd708c` | `?` | `?` | `0x29c468` | `[0x297378]` |
| `mov` at 0xd7264 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x12da68]` | `?` |
| `bl` at 0xd72b0 | `(unknown callee) 0x197a8` | `0x29c468` | `0x1214c4` | `?` | `?` |
| `bl` at 0xd72b4 | `(unknown callee) 0xd70fc` | `ret(0x197a8)` | `?` | `?` | `?` |
| `bl` at 0xd8998 | `(unknown callee) 0x19808` | `0x29c468` | `0x121508` | `?` | `?` |
| `bl` at 0xd89a4 | `(unknown callee) 0xd70fc` | `ret(0x19808)` | `?` | `?` | `0x0` |
| `bl` at 0xd8a00 | `(unknown callee) 0x221f0` | `0x121534` | `0x29c468` | `?` | `[0x29c468]:1` |
| `bl` at 0xd8a24 | `(unknown callee) 0xd0168` | `0x0` | `0x121544` | `0x1` | `[sp-24]` |
| `bl` at 0xd8a4c | `(unknown callee) 0xd0168` | `0x0` | `0x12156c` | `0x0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xd7188 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xd71b0 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xd71e0 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `beq` at 0xd7270 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `ldrls` at 0xd72f0 | `? <= 0x14 (unsigned, 32-bit)` | `? > 0x14 (unsigned, 32-bit)` |
| `bne` at 0xd89d4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xd89e4 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0xd89f4 | `[0x29c468]:1 == 0x0` | `[0x29c468]:1 != 0x0` |
| `bls` at 0xd8a10 | `[0x29737c] <= 0x2 (unsigned, 32-bit)` | `[0x29737c] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xd8a38 | `[0x29737c] <= 0x1 (unsigned, 32-bit)` | `[0x29737c] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd8a60 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xd72f0).

## `jmstr_sel_sic`

- `jmstr_sel_sic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd12a4 to 0xd3148.
  Established. A nonzero `ret(0x1943c)` takes `0xd12f4`, while zero falls through (`bne` at 0xd12d8).
  An included word assignment places `0x80000000` at `sp -0x2c` (`str` at 0xd1348).
  The unnamed `0xd0754` call carries `0x0`, `0x1`, `[sp+16]`, unknown (`bl` at 0xd1464).
  SIC selection protocol semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd12b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd12b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd12bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd12c0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xd12cc | store | 4 | `sp -0x40` | `ret(0x1943c)` |
| `ldr` at 0xd12d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd12dc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd12e0 | load | 4 | `abs 0xd227c` | `-` |
| `str` at 0xd12ec | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xd12fc | store | 4 | `sp -0x44` | `ret(0x1943c)` |
| `ldr` at 0xd1300 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd130c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd1310 | load | 4 | `abs 0xd227c` | `-` |
| `ldr` at 0xd1318 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1324 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd132c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd133c | load | 4 | `abs 0xd2280` | `-` |
| `str` at 0xd1340 | store | 4 | `sp -0x28` | `0xff800000` |
| `str` at 0xd1348 | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0xd1350 | load | 4 | `abs 0xd2284` | `-` |
| `str` at 0xd1354 | store | 4 | `sp -0x28` | `0xfff00000` |
| `ldr` at 0xd1358 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd135c | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0xd1360 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1364 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd1370 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1378 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd1384 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd1388 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd138c | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd1398 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd13a0 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd13ac | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd13b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd13b4 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd13c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd13c8 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd13d4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd13d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd13e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd13fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd1400 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1410 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd1424 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd1428 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1434 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd143c | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd1444 | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0xd1448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd145c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1460 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd1474 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd147c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1488 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1494 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1498 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd14ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd14b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd14c0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd14c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd14dc | store | 4 | `sp -0x24` | `0x9` |
| `ldr` at 0xd14e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd14e8 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `str` at 0xd14f4 | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xd14f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1500 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `ldr` at 0xd1504 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1508 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd150c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1510 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd151c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1520 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1528 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd1534 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1538 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd153c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1540 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1544 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd1550 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1554 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd155c | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd1568 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd156c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1570 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1574 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1578 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd1584 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1588 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1590 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd159c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd15a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd15b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15bc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd15d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd15d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd15ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd15f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1604 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1608 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1610 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1614 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1618 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd161c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1620 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd1624 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1628 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd162c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd1630 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd1640 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1654 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd165c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1668 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1670 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1674 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1678 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd1684 | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xd168c | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xd1690 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1694 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd1698 | store | 4 | `[sp-64] +0x0` | `[sp-24]` |
| `ldr` at 0xd169c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd16a4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd16a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16b0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd16b4 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd16c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16c4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd16cc | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd16d8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd16dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16e4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd16e8 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd16f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd16f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1700 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd170c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1710 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1714 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1718 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd171c | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd1728 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd172c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1734 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd1740 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1744 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1748 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd174c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd175c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1760 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1774 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1778 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd177c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1780 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1790 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1794 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd17a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd17ac | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd17b0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd17b4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd17b8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd17bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd17c0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd17c4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd17c8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd17d8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd17ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd17f4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1800 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1808 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd180c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1810 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd1818 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd181c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd182c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd1840 | store | 4 | `sp -0x4c` | `0x9` |
| `str` at 0xd184c | store | 4 | `sp -0x4c` | `0xc` |
| `ldr` at 0xd1850 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xd1854 | store | 4 | `sp -0x24` | `[sp-76]` |
| `ldr` at 0xd1858 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1860 | store | 4 | `[sp-64] +0x0` | `0xe` |
| `ldr` at 0xd1864 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd186c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1870 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1874 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1878 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd187c | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd1888 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd188c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1894 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd18a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd18a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd18b0 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd18bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd18c8 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd18d4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd18d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd18e4 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd18f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd18f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd18fc | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd1908 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd190c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1910 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1914 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1924 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1928 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd193c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1940 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1944 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1948 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1958 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd195c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1970 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1974 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1978 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd197c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1980 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd1984 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1988 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd198c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd1990 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd19a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd19b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd19bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd19c8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd19d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd19d4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd19d8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd19e4 | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0xd19ec | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xd19f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd19f4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xd19f8 | store | 4 | `[sp-64] +0x0` | `[sp]` |
| `ldr` at 0xd19fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a04 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a08 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd1a14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a18 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a20 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd1a2c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1a30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a38 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a3c | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd1a48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a4c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a54 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd1a60 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1a64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a6c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a70 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd1a7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a80 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1a88 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd1a94 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1a98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1a9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1aa0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1ab0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ab4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1ac8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1acc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ad0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ad4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1ae4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ae8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1afc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1b04 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1b10 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xd1b14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1b1c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1b24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1b30 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xd1b34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1b38 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd1b3c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1b40 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd1b44 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1b48 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd1b4c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd1b50 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd1b60 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1b74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1b7c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1b88 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1b90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd1b9c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1ba0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1ba4 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd1bac | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd1bb4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1bc0 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xd1bcc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1bd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1be0 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xd1bec | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xd1bf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd1c04 | store | 4 | `sp -0x50` | `0x9` |
| `str` at 0xd1c10 | store | 4 | `sp -0x50` | `0xc` |
| `ldr` at 0xd1c14 | load | 4 | `sp -0x50` | `-` |
| `str` at 0xd1c18 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0xd1c1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c20 | load | 4 | `abs 0xd2288` | `-` |
| `str` at 0xd1c24 | store | 4 | `[sp-64] +0x0` | `0xfff` |
| `ldr` at 0xd1c28 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1c30 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1c34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c3c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1c40 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd1c4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c50 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1c58 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd1c64 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1c68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c70 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1c74 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd1c80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1c84 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1c8c | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd1c98 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1c9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ca0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1ca4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1ca8 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd1cb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1cb8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1cc0 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd1ccc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1cd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1cd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1cd8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1ce8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1cec | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1d00 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1d04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1d08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1d0c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1d1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1d20 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1d34 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1d38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1d3c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd1d40 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1d44 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd1d48 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1d4c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd1d50 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd1d54 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd1d64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1d78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1d80 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1d8c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1d94 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1d98 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1d9c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd1da8 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd1dac | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1db0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1db4 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd1db8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd1dc0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd1dc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1dc8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd1dcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd1dd8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1ddc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd1de0 | load | 4 | `abs 0xd2298` | `-` |
| `ldr` at 0xd1de4 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd1dec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1df4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1e00 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xd1e0c | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xd1e10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1e14 | load | 4 | `abs 0xd228c` | `-` |
| `str` at 0xd1e18 | store | 4 | `[sp-64] +0x0` | `0xa800ffea` |
| `ldr` at 0xd1e1c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1e24 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1e28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd1e2c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd1e30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1e34 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd1e38 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1e3c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd1e40 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd1e44 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd1e54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd1e68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1e70 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1e7c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1e84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1e88 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd1e8c | load | 4 | `abs 0xd2290` | `-` |
| `ldr` at 0xd1e98 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd1e9c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd1ea0 | load | 4 | `abs 0xd2294` | `-` |
| `ldr` at 0xd1ea4 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd1eac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1eb4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1ec0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd1ec8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd1edc | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0xd1ee0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd1ee8 | store | 4 | `[sp-64] +0x0` | `0x9` |
| `ldr` at 0xd1eec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd1ef4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd1ef8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1efc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f00 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f04 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd1f10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f14 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f1c | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd1f28 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1f2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f34 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f38 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd1f44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f48 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f50 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd1f5c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1f60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f68 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f6c | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd1f78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f7c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1f84 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd1f90 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1f94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1f9c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1fac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1fb0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1fc4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1fc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1fcc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1fd0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd1fe0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd1fe4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd1ff8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd1ffc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd2000 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd2004 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2008 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd200c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2010 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd2014 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd2018 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd2028 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd203c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2044 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2050 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2058 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd205c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2060 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd206c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2070 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2074 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2078 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd207c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2084 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2088 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd208c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2090 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd209c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd20a0 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd20a4 | load | 4 | `abs 0xd2298` | `-` |
| `ldr` at 0xd20a8 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd20b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd20b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd20c4 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xd20d0 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xd20d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd20d8 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xd20dc | store | 4 | `[sp-64] +0x0` | `[sp+4]` |
| `ldr` at 0xd20e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd20e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd20e8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd20ec | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd20f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd20fc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2104 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd2110 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2114 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2118 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd211c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2120 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd212c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2130 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2138 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd2144 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2148 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd214c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2150 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2154 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2160 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2164 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd216c | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2178 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd217c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2180 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2184 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2194 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2198 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd21ac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd21b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd21b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd21b8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd21c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd21cc | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd21e0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd21e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd21ec | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd21f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd21f4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd21f8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd21fc | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd2200 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2204 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd2208 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd220c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd221c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2230 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2238 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2244 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd224c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2250 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2254 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd225c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2260 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2274 | store | 4 | `sp -0x54` | `0x9` |
| `str` at 0xd22b4 | store | 4 | `sp -0x54` | `0xc` |
| `ldr` at 0xd22b8 | load | 4 | `sp -0x54` | `-` |
| `str` at 0xd22bc | store | 4 | `sp -0x24` | `[sp-84]` |
| `ldr` at 0xd22c0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd22c8 | store | 4 | `[sp-64] +0x0` | `0x8` |
| `ldr` at 0xd22cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd22d4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd22d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd22dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd22e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd22e4 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd22f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd22f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd22fc | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd2308 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd230c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2310 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2314 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2318 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd2324 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2328 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2330 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd233c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2340 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2344 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2348 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd234c | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2358 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd235c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2364 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2370 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2374 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2378 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd237c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd238c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2390 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd23a4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd23a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd23ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd23b0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd23c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd23c4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd23d8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd23dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd23e0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd23e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd23e8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd23ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd23f0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd23f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd23f8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd2408 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd241c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2424 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2430 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2438 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd243c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2440 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd244c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2450 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2454 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2458 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd245c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd2464 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2468 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd246c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2470 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd247c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2480 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd2484 | load | 4 | `abs 0xd2298` | `-` |
| `ldr` at 0xd2488 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2490 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2498 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd24a4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd24ac | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd24c0 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xd24c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd24c8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd24d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd24d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd24d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd24dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd24e0 | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd24ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd24f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd24f8 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd2504 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2508 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd250c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2510 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2514 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd2520 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2524 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd252c | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd2538 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd253c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2540 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2544 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2548 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2554 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2558 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2560 | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd256c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2570 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2574 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2578 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2588 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd258c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd25a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd25a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd25a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd25ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd25bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd25c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd25d4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd25dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd25e8 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xd25ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd25f4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd25fc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2608 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `str` at 0xd2614 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0xd2618 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd2628 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd262c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2634 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd263c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2640 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd264c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd2658 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2660 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd266c | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0xd2670 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2678 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd2680 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd268c | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0xd2690 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2694 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2698 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd269c | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd26a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd26ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd26b4 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd26c0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd26c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd26c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd26cc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd26d0 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd26dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd26e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd26e8 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd26f4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd26f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd26fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2700 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2704 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2710 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2714 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd271c | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2728 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd272c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2730 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2734 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2744 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2748 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd275c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2760 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2764 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2768 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2778 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd277c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd2790 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2794 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd2798 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd279c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd27a0 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd27a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd27a8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd27ac | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd27b0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd27c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd27d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd27dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd27e8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd27f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd27fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2800 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2804 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd280c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2818 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2824 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd282c | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0xd2838 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0xd283c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd284c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd2860 | store | 4 | `sp -0x58` | `0x9` |
| `str` at 0xd286c | store | 4 | `sp -0x58` | `0xc` |
| `ldr` at 0xd2870 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xd2874 | store | 4 | `sp -0x24` | `[sp-88]` |
| `ldr` at 0xd2878 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2880 | store | 4 | `[sp-64] +0x0` | `0x1` |
| `ldr` at 0xd2884 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd288c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd2890 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2894 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2898 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd289c | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd28a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd28ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd28b4 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd28c0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd28c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd28c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd28cc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd28d0 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd28dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd28e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd28e8 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd28f4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd28f8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd28fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2900 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2904 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2910 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2914 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd291c | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2928 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd292c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2930 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2934 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2944 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2948 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd295c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2960 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2964 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2968 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2978 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd297c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd2990 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2994 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd2998 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd299c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd29a0 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd29a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd29a8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd29ac | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd29b0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd29c0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd29d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd29dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd29e8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd29f0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd29f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd29f8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd29fc | store | 4 | `[sp-68] +0x0` | `[[sp-68]]` |
| `str` at 0xd2a04 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xd2a08 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2a10 | store | 4 | `[sp-64] +0x0` | `0x0` |
| `ldr` at 0xd2a14 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2a1c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0xd2a20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a28 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2a2c | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd2a38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a3c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2a44 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd2a50 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2a54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a5c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2a60 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd2a6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a70 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2a78 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd2a84 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2a88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2a90 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2a94 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2aa0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2aa4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2aac | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2ab8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2abc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2ac0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2ac4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2ad4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2ad8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd2aec | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2af0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2af4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2af8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd2b08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2b0c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd2b20 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd2b24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd2b28 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0xd2b2c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2b30 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd2b34 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2b38 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd2b3c | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd2b40 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd2b50 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2b64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2b6c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2b78 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2b80 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2b84 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2b88 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2b8c | load | 4 | `abs 0xd229c` | `-` |
| `ldr` at 0xd2b98 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2b9c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2ba4 | load | 4 | `abs 0xd229c` | `-` |
| `str` at 0xd2bb0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2bb4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2bb8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2bbc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2bc0 | load | 4 | `abs 0xd22a0` | `-` |
| `ldr` at 0xd2bcc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2bd0 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2bd8 | load | 4 | `abs 0xd22a0` | `-` |
| `str` at 0xd2be4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2be8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2bec | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2bf0 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2bf4 | load | 4 | `abs 0xd22a4` | `-` |
| `ldr` at 0xd2c00 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c04 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2c0c | load | 4 | `abs 0xd22a4` | `-` |
| `str` at 0xd2c18 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2c1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c20 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c24 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2c34 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c38 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd2c4c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2c50 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c54 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c58 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0xd2c68 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c6c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd2c80 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0xd2c84 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd2c88 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2c8c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0xd2c90 | store | 4 | `[sp+8] +0x0` | `[[sp-68]]` |
| `str` at 0xd2c98 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xd2c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2cb0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2cb4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd2cc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2cd0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2cdc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2ce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2cf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2d04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2d1c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2d20 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd2d34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2d3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2d48 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2d60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2d78 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2d80 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd2d94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2d9c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2da8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd2dc0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2dc8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2dd4 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xd2de0 | store | 4 | `sp -0x38` | `0x100` |
| `ldr` at 0xd2de4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd2df4 | store | 4 | `sp -0x3c` | `0x300` |
| `ldr` at 0xd2df8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd2e0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd2e18 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd2e20 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd2e24 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2e28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2e34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2e3c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2e48 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2e4c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd2e54 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd2e5c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2e64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2e70 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2e78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2e84 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd2e88 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd2e90 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd2e98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2ea8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2eb4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2ebc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2ec8 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2ecc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd2ed4 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd2edc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd2ee0 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xd2ee4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2ee8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd2eec | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2ef0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd2ef4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd2ef8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd2f08 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd2f1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2f24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2f30 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd2f38 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd2f3c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd2f40 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd2f44 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xd2f58 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd2f5c | load | 4 | `abs 0xd22a8` | `-` |
| `ldr` at 0xd2f64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd2f6c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd2f78 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xd2f84 | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0xd2f88 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd2f98 | store | 4 | `sp -0x3c` | `0xc0` |
| `ldr` at 0xd2f9c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd2fb0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd2fbc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0xd2fc4 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd2fc8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2fcc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd2fd8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd2fe0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd2fec | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd2ff0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd2ff8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd3000 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3008 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3014 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd301c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3028 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd302c | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd3034 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd303c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd304c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3058 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3060 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd306c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd3070 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd3078 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xd3080 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3084 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xd3088 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd308c | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0xd3090 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3094 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0xd3098 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xd309c | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0xd30ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd30c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd30c8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd30d4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd30dc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd30e0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd30e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd30e8 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xd30fc | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0xd3100 | load | 4 | `abs 0xd22ac` | `-` |
| `ldr` at 0xd3108 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3110 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd311c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xd3124 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd312c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3138 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xd313c | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xd3144 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd3144 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd3144 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd12c8 | `(unknown callee) 0x1943c` | `0x60` | `r1` | `r2` | `r3` |
| `bl` at 0xd12e4 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x121158` | `?` | `ret(0x1943c)` |
| `bl` at 0xd12f8 | `(unknown callee) 0x1943c` | `0x60` | `?` | `?` | `ret(0x1943c)` |
| `bl` at 0xd1314 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x121158` | `?` | `ret(0x1943c)` |
| `bl` at 0xd131c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd1464 | `(unknown callee) 0xd0754` | `0x0` | `0x1` | `[sp+16]` | `?` |
| `bl` at 0xd1478 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd1480 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd149c | `(unknown callee) 0xd0754` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xd14b0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd14b8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1644 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xd1658 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd1660 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd17dc | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd17f0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd17f8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd19a4 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd19b8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd19c0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1b64 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd1b78 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd1b80 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1d68 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd1d7c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd1d84 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1de8 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x1211bc` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xd1df0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd1df8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1e58 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd1e6c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd1e74 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd1ea8 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x121184` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xd1eb0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd1eb8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd202c | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd2040 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd2048 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd20ac | `(unknown callee) 0x197a8` | `[sp+16]` | `0x1211bc` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xd20b4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd20bc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2220 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd2234 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd223c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd240c | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd2420 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd2428 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd248c | `(unknown callee) 0x197a8` | `[sp+16]` | `0x1211bc` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0xd2494 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd249c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd27c4 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd27d8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd27e0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd29c4 | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0xd29d8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd29e0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2b54 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd2b68 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd2b70 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2cb8 | `(unknown callee) 0xd0754` | `0x0` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0xd2ccc | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd2cd4 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2d24 | `(unknown callee) 0xd0754` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0xd2d38 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd2d40 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2d84 | `(unknown callee) 0xd093c` | `0x0` | `[sp-52]` | `0x0` | `[sp+16]` |
| `bl` at 0xd2d98 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd093c)` |
| `bl` at 0xd2da0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2dc4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd2dcc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2f0c | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd2f20 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd2f28 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd2f48 | `(unknown callee) 0xd497c` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+8]` |
| `bl` at 0xd2f60 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x1211e8` | `?` | `ret(0xd497c)` |
| `bl` at 0xd2f68 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd2f70 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd30b0 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0xd30c4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd30cc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd30ec | `(unknown callee) 0xd497c` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+12]` |
| `bl` at 0xd3104 | `(unknown callee) 0x197a8` | `[sp+16]` | `0x121210` | `?` | `ret(0xd497c)` |
| `bl` at 0xd310c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3114 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3128 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `ret(0xd497c)` |
| `bl` at 0xd3130 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd12d8 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xd1308 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `beq` at 0xd1338 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd1454 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd1470 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd14a8 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `beq` at 0xd14d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd1650 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd17e8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd1828 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd1838 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd19b0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd1b70 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd1b98 | `[sp-36] != 0x18` | `[sp-36] == 0x18` |
| `beq` at 0xd1bfc | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd1d74 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd1dd4 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xd1e64 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd1e94 | `[[sp-68]] == 0x54007ff5` | `[[sp-68]] != 0x54007ff5` |
| `beq` at 0xd1ed4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd2038 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd2098 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0xd222c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd226c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd2418 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd2478 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `beq` at 0xd24b8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd2624 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd27d0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd27f8 | `[sp-36] != 0x8` | `[sp-36] == 0x8` |
| `beq` at 0xd2848 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd2858 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd29d0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd2b60 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd2ca8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd2cc4 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd2cf0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd2d00 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd2d10 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd2d30 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd2d5c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd2d6c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd2d90 | `ret(0xd093c) != 0x0` | `ret(0xd093c) == 0x0` |
| `bne` at 0xd2dbc | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xd2e30 | `C == 1` | `not (C == 1)` |
| `bls` at 0xd2e6c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd2eb0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd2f18 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd2f54 | `ret(0xd497c) != 0x0` | `ret(0xd497c) == 0x0` |
| `bhs` at 0xd2fd4 | `C == 1` | `not (C == 1)` |
| `bls` at 0xd3010 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd3054 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd30bc | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd30f8 | `ret(0xd497c) != 0x0` | `ret(0xd497c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd3144 | `r0` | `?` |

## `jmstr_sel_sic_multi`

- `jmstr_sel_sic_multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd3148 to 0xd497c.
  Established. Zero `[r0+16]` takes `0xd3198`, while nonzero falls through (`beq` at 0xd3184).
  Unsigned `[r0+16] > 0x8` also takes `0xd3198`, while smaller or equal values fall through (`bhi` at 0xd3190).
  The unnamed `0xd4b48` call carries `sp-136`, `0xc`, `[sp-104]`, unknown (`bl` at 0xd3370).
  Multi-device SIC selection policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd3158 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd315c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd3160 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd3164 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd316c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd3174 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xd3178 | store | 4 | `sp -0x68` | `[r0+16]` |
| `ldr` at 0xd317c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3188 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3198 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd319c | load | 4 | `abs 0xd4064` | `-` |
| `ldr` at 0xd31a0 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xd31ac | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd31bc | store | 4 | `sp -0x3c` | `ret(0x1943c)` |
| `ldr` at 0xd31c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd31cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd31d0 | load | 4 | `abs 0xd4068` | `-` |
| `str` at 0xd31dc | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd31ec | store | 4 | `sp -0x40` | `ret(0x1943c)` |
| `ldr` at 0xd31f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd31fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd3200 | load | 4 | `abs 0xd4068` | `-` |
| `ldr` at 0xd3208 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3214 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd3224 | store | 4 | `sp -0x44` | `ret(0x1943c)` |
| `ldr` at 0xd3228 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xd3234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd3238 | load | 4 | `abs 0xd4068` | `-` |
| `ldr` at 0xd3240 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3248 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3254 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd3260 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3264 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3268 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3274 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3288 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd328c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd32a4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd32b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd32bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xd32c0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd32c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd32cc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd32d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd32dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd32e0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd32ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3300 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd3304 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd331c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd3328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd3334 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3348 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd334c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3354 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3368 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd336c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3378 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xd337c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd3390 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3394 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd33a8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd33b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd33b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd33c4 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd33d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd33d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd33e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd33f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd33f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3404 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd340c | load | 4 | `sp -0x68` | `-` |
| `str` at 0xd3414 | store | 4 | `sp -0x94` | `[sp-104]-1` |
| `ldr` at 0xd3418 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3428 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd342c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3430 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd343c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3444 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3450 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3454 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd345c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd3468 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd3478 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3480 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3488 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd348c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3490 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd34a0 | load | 4 | `abs 0xd406c` | `-` |
| `ldr` at 0xd34a8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd34b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd34b8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd34c0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd34c4 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd34c8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd34d0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xd34dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd34e0 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd34e4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd34e8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd34ec | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd34f0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd34f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd34f8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3508 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd351c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3524 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd352c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3538 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd3540 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3544 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd3548 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd354c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3550 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3554 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd3558 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd355c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd356c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3580 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3588 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3590 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd359c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd35a8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd35ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd35b0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd35bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd35c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd35d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd35d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd35dc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd35e8 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd35ec | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3600 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd3604 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd3610 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd361c | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3620 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3628 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3630 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3638 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3640 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd3644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3648 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3660 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3668 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3670 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3678 | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd367c | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd3680 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3684 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3690 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd3694 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3698 | store | 4 | `sp -0xb0` | `[sp-36]` |
| `ldr` at 0xd369c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd36a0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd36a4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd36a8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd36ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd36b0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd36c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd36d4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd36dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd36e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd36f0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd36f8 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xd3710 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3724 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3728 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd372c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3738 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3740 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd374c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3750 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3758 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd3764 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd3774 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd377c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3784 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd3788 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd378c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd379c | load | 4 | `abs 0xd406c` | `-` |
| `ldr` at 0xd37a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd37ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd37b4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd37bc | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd37c0 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd37c4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd37c8 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd37cc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd37d0 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd37d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd37d8 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd37dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd37e0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd37f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3804 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd380c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3814 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3820 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd382c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3830 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3834 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3840 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3848 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3854 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3858 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3860 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd386c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd3870 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3884 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd3888 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd3894 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd38a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd38a4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd38ac | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd38b4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd38bc | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd38c4 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd38c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd38cc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd38e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd38ec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd38f4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd38fc | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd3900 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd3904 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3908 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd390c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3910 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3914 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3918 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd391c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd3920 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3930 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3944 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd394c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3954 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3960 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd3968 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0xd3980 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3994 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3998 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd399c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd39a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd39b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd39bc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd39c0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd39c8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd39d4 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd39e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd39ec | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd39f4 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd39f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd39fc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3a0c | load | 4 | `abs 0xd406c` | `-` |
| `ldr` at 0xd3a14 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3a1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3a24 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3a2c | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd3a30 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd3a34 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3a38 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd3a3c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3a40 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3a44 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3a48 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd3a4c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd3a50 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3a60 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3a74 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3a7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3a84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3a90 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd3a9c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3aa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3aa4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3ab0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3ab8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3ac4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3ac8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3ad0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd3adc | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd3ae0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3af4 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd3af8 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd3b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd3b10 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3b14 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3b1c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3b24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3b2c | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3b34 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0xd3b38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3b3c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3b54 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3b5c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3b64 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3b6c | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd3b70 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0xd3b74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3b78 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0xd3b7c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3b80 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3b84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3b88 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd3b8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd3b90 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3ba0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3bb4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3bbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3bc4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3bd0 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd3bdc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3be0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3be4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3bf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3bf8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3c04 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3c08 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3c10 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd3c1c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0xd3c20 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3c24 | store | 4 | `sp -0x2c` | `[sp-148]` |
| `ldr` at 0xd3c28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3c2c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3c3c | load | 4 | `abs 0xd406c` | `-` |
| `ldr` at 0xd3c44 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3c4c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3c54 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3c5c | load | 4 | `sp -0x98` | `-` |
| `str` at 0xd3c60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xd3c64 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3c68 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xd3c6c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3c70 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3c74 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3c78 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd3c7c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd3c80 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3c90 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3ca4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3cac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3cb4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3cc0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd3cc8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xd3cd0 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `str` at 0xd3cdc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3ce0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3ce4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3cf0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3d08 | load | 4 | `abs 0xd4070` | `-` |
| `str` at 0xd3d0c | store | 4 | `(unknown)` | `0x800ffe0` |
| `ldr` at 0xd3d10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3d18 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3d24 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3d28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3d30 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3d3c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd3d40 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3d48 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3d5c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3d60 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3d68 | store | 4 | `sp -0x24` | `ret(0xd4b48)` |
| `ldr` at 0xd3d6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd3d70 | store | 4 | `sp -0xb0` | `ret(0xd4b48)` |
| `ldr` at 0xd3d74 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd3d78 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd3d7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd3d80 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd3d84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd3d88 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd3d98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd3dac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3db4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3dbc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3dc8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd3dd0 | load | 4 | `abs 0xd4070` | `-` |
| `str` at 0xd3dd4 | store | 4 | `sp -0x88` | `0x800ffe0` |
| `ldr` at 0xd3dd8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3ddc | load | 4 | `abs 0xd4078` | `-` |
| `ldr` at 0xd3de8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3df0 | load | 4 | `abs 0xd4078` | `-` |
| `str` at 0xd3dfc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3e00 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e04 | load | 4 | `abs 0xd407c` | `-` |
| `ldr` at 0xd3e10 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e18 | load | 4 | `abs 0xd407c` | `-` |
| `str` at 0xd3e24 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3e28 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e2c | load | 4 | `abs 0xd4080` | `-` |
| `ldr` at 0xd3e38 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e40 | load | 4 | `abs 0xd4080` | `-` |
| `str` at 0xd3e4c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3e50 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e60 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xd3e74 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3e78 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3e88 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xd3e9c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0xd3ea4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xd3eac | store | 4 | `sp -0x9c` | `?` |
| `str` at 0xd3eb4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xd3eb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3ebc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3ec8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3ee0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3ee4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3eec | load | 4 | `sp -0x9c` | `-` |
| `str` at 0xd3ef4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd3ef8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3f10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3f28 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3f30 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3f38 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0xd3f44 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd3f4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd3f50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3f58 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd3f60 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xd3f64 | load | 4 | `sp -0x68` | `-` |
| `str` at 0xd3f6c | store | 4 | `sp -0x88` | `?` |
| `str` at 0xd3f74 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd3f78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3f7c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd3f88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3f90 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3f98 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd3fb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd3fb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd3fc0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd3fc4 | load | 4 | `abs 0xd4074` | `-` |
| `ldr` at 0xd3fcc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd3fd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd3fdc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd3fe8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd3ff0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd3ff8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xd4004 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xd4008 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd400c | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xd4010 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4014 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xd4018 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd401c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd4020 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd4040 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4048 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4050 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd405c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd4090 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd4094 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4098 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd40a4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd40b8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0xd40bc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0xd40c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd40d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd40d8 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0xd40dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd40f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xd4100 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xd410c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4110 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4118 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4124 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd4128 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4130 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd413c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd4140 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4148 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd415c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd4160 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4168 | store | 4 | `sp -0x24` | `ret(0xd4b48)` |
| `ldr` at 0xd416c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd4170 | store | 4 | `sp -0xb0` | `ret(0xd4b48)` |
| `ldr` at 0xd4174 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4178 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd417c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd4180 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd4184 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd4188 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd4198 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd41ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd41b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd41bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd41c8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd41d0 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xd41e4 | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0xd41e8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd41ec | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0xd41f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd41f4 | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0xd41f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd41fc | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd4200 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd4220 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4228 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4230 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd423c | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0xd4248 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd424c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4250 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd425c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4278 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd427c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4284 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4290 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd4294 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd429c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd42a8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd42ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd42b4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd42c8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xd42cc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd42d4 | store | 4 | `sp -0x24` | `ret(0xd4b48)` |
| `ldr` at 0xd42d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd42dc | store | 4 | `sp -0xb0` | `ret(0xd4b48)` |
| `ldr` at 0xd42e0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd42e4 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd42e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd42ec | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd42f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd42f4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd4304 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4318 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4328 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4334 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd433c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4340 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4344 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd4348 | load | 4 | `abs 0xd4078` | `-` |
| `ldr` at 0xd4354 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4358 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd4360 | load | 4 | `abs 0xd4078` | `-` |
| `str` at 0xd436c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd4370 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4374 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4378 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd437c | load | 4 | `abs 0xd407c` | `-` |
| `ldr` at 0xd4388 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd438c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd4394 | load | 4 | `abs 0xd407c` | `-` |
| `str` at 0xd43a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd43a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43ac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd43b0 | load | 4 | `abs 0xd4080` | `-` |
| `ldr` at 0xd43bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43c0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd43c8 | load | 4 | `abs 0xd4080` | `-` |
| `str` at 0xd43d4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd43d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43e0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd43f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd43f4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd4408 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd440c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4410 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4414 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0xd4424 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4428 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd443c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0xd4440 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4444 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4448 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0xd444c | store | 4 | `[sp-24] +0x0` | `[[sp-64]]` |
| `str` at 0xd4454 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xd4458 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd446c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd4470 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4484 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd448c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4494 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd44a0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd44a8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd44b8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd44c8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd44e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd44e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd44f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4500 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4508 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4514 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd451c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd452c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd4544 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd454c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4560 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4568 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4570 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd457c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd4584 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xd4594 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd459c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd45a4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd45b0 | store | 4 | `sp -0xa0` | `0x1` |
| `str` at 0xd45bc | store | 4 | `sp -0x34` | `0x100` |
| `ldr` at 0xd45c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd45d0 | store | 4 | `sp -0x38` | `0x300` |
| `ldr` at 0xd45d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd45e8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd45f4 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xd45fc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd4600 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4604 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4610 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4618 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4624 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4628 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4630 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd4638 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4640 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd464c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4654 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4660 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd4664 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd466c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd4674 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4690 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4698 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd46a4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd46a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd46b0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd46b8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd46bc | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xd46c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd46c4 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd46c8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd46cc | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd46d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd46d4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd46e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd46f8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4700 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4708 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4714 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd4720 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd4724 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd4728 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd473c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd4740 | load | 4 | `abs 0xd4084` | `-` |
| `ldr` at 0xd4748 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4750 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4758 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4764 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd476c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd4778 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd477c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd4780 | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `str` at 0xd4788 | store | 4 | `sp -0x34` | `0x40` |
| `ldr` at 0xd478c | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd479c | store | 4 | `sp -0x38` | `0xc0` |
| `ldr` at 0xd47a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd47b4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xd47c0 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xd47c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xd47cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd47d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd47dc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd47e4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd47f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd47f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd47fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd4804 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd480c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4818 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4820 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd482c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xd4830 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd4838 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd4840 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xd4850 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd485c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xd4864 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4870 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xd4874 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd487c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xd4884 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd4888 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0xd488c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xd4890 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0xd4894 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd4898 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0xd489c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd48a0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0xd48b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xd48c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd48cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd48d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd48e0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd48ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd48f0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd48f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4908 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd490c | load | 4 | `abs 0xd4088` | `-` |
| `ldr` at 0xd4914 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd491c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4924 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd4930 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0xd4938 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4944 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd4948 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd494c | store | 4 | `[sp-28] +0x0` | `[sp-40]` |
| `ldr` at 0xd4950 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd4958 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xd4960 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xd496c | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0xd4970 | load | 4 | `sp -0xa0` | `-` |
| `ldm` at 0xd4978 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd4978 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd4978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd31a4 | `(unknown callee) 0x197a8` | `r3` | `0x121240` | `[r0+16]` | `[r0+16]` |
| `bl` at 0xd31b8 | `(unknown callee) 0x1943c` | `0x60` | `r1` | `0x10` | `[r0+16]` |
| `bl` at 0xd31d4 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x121158` | `?` | `ret(0x1943c)` |
| `bl` at 0xd31e8 | `(unknown callee) 0x1943c` | `0x60` | `?` | `?` | `ret(0x1943c)` |
| `bl` at 0xd3204 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x121158` | `?` | `ret(0x1943c)` |
| `bl` at 0xd320c | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd3220 | `(unknown callee) 0x1943c` | `0x60` | `?` | `?` | `ret(0x1943c)` |
| `bl` at 0xd323c | `(unknown callee) 0x197a8` | `[sp-32]` | `0x121158` | `?` | `ret(0x1943c)` |
| `bl` at 0xd3244 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd324c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3370 | `(unknown callee) 0xd4b48` | `sp-136` | `0xc` | `[sp-104]` | `?` |
| `bl` at 0xd3398 | `(unknown callee) 0xd0754` | `0x0` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0xd33ac | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd33b4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd33bc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd33d8 | `(unknown callee) 0xd0754` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd33ec | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd33f4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd33fc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd347c | `(unknown callee) 0xd4da0` | `0x4` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd34ac | `(unknown callee) 0xd4da0` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd350c | `(unknown callee) 0xd0d10` | `0x0` | `0x0` | `0x1` | `[sp-48]` |
| `bl` at 0xd3520 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3528 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3530 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3570 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3584 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd358c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3594 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3634 | `(unknown callee) 0xd4da0` | `[sp-136]` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3664 | `(unknown callee) 0xd4da0` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xd36c4 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd36d8 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd36e0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd36e8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3778 | `(unknown callee) 0xd4da0` | `0xe` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd37a8 | `(unknown callee) 0xd4da0` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd37f4 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3808 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3810 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3818 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd38b8 | `(unknown callee) 0xd4da0` | `[sp-136]` | `0x18` | `[sp-60]` | `sp-152` |
| `bl` at 0xd38e8 | `(unknown callee) 0xd4da0` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3934 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3948 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3950 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3958 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd39e8 | `(unknown callee) 0xd4da0` | `0x9` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3a18 | `(unknown callee) 0xd4da0` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3a64 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3a78 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3a80 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3a88 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3b28 | `(unknown callee) 0xd4da0` | `[sp-136]` | `0x8` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3b58 | `(unknown callee) 0xd4da0` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3ba4 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3bb8 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3bc0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3bc8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3c48 | `(unknown callee) 0xd4da0` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0xd3c94 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3ca8 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3cb0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3cb8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3d64 | `(unknown callee) 0xd4b48` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xd3d9c | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd3db0 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd3db8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3dc0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd3fc8 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x12125c` | `?` | `?` |
| `bl` at 0xd3fd0 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd3fd8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd3fe0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4030 | `(unknown callee) 0xd4ff0` | `[sp-104]` | `0x8` | `0xc` | `0x1` |
| `bl` at 0xd4044 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd4ff0)` |
| `bl` at 0xd404c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4054 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4164 | `(unknown callee) 0xd4b48` | `sp-136` | `0x8` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xd419c | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd41b0 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd41b8 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd41c0 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4210 | `(unknown callee) 0xd4ff0` | `[sp-104]` | `0x1` | `0xc` | `0x1` |
| `bl` at 0xd4224 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd4ff0)` |
| `bl` at 0xd422c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4234 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd42d0 | `(unknown callee) 0xd4b48` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0xd4308 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd431c | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd4324 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd432c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4474 | `(unknown callee) 0xd0754` | `0x0` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0xd4488 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd4490 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4498 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd44e8 | `(unknown callee) 0xd0754` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0xd44fc | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0754)` |
| `bl` at 0xd4504 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd450c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4550 | `(unknown callee) 0xd093c` | `0x0` | `[sp-48]` | `0x0` | `[sp-32]` |
| `bl` at 0xd4564 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd093c)` |
| `bl` at 0xd456c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4574 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4598 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd45a0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd45a8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd46e8 | `(unknown callee) 0xd0d10` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd46fc | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd4704 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd470c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd472c | `(unknown callee) 0xd497c` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xd4744 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x1211e8` | `?` | `ret(0xd497c)` |
| `bl` at 0xd474c | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd4754 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd475c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd48b4 | `(unknown callee) 0xd0d10` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0xd48c8 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xd48d0 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd48d8 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd48f8 | `(unknown callee) 0xd497c` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0xd4910 | `(unknown callee) 0x197a8` | `[sp-32]` | `0x121210` | `?` | `ret(0xd497c)` |
| `bl` at 0xd4918 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd4920 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4928 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xd4954 | `(unknown callee) 0x1970c` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0xd495c | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xd4964 | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd3184 | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bhi` at 0xd3190 | `[r0+16] > 0x8 (unsigned, 32-bit)` | `[r0+16] <= 0x8 (unsigned, 32-bit)` |
| `bne` at 0xd31c8 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xd31f8 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xd3230 | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bhs` at 0xd3270 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd32e8 | `C == 1` | `not (C == 1)` |
| `beq` at 0xd3388 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd33a4 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd33e4 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `blt` at 0xd3420 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0xd3438 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3498 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd34d8 | `[sp-148] != [sp-104]-1` | `[sp-148] == [sp-104]-1` |
| `bne` at 0xd3518 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd357c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd35b8 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3650 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd36d0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd371c | `? == 0x0` | `? != 0x0` |
| `bhs` at 0xd3734 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3794 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3800 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd383c | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd38d4 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3940 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd398c | `? == 0x0` | `? != 0x0` |
| `bhs` at 0xd39a4 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3a04 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3a70 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd3aac | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3b44 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3bb0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd3bec | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3c34 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3ca0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd3cec | `C == 1` | `not (C == 1)` |
| `bne` at 0xd3da8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xd3ec4 | `C == 1` | `not (C == 1)` |
| `bhs` at 0xd3f84 | `C == 1` | `not (C == 1)` |
| `beq` at 0xd3fbc | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xd403c | `ret(0xd4ff0) != 0x0` | `ret(0xd4ff0) == 0x0` |
| `bhs` at 0xd40a0 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd41a8 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd41dc | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd421c | `ret(0xd4ff0) != 0x0` | `ret(0xd4ff0) == 0x0` |
| `bhs` at 0xd4258 | `C == 1` | `not (C == 1)` |
| `bne` at 0xd4314 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `beq` at 0xd4464 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xd4480 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd44b4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd44c4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd44d4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd44f4 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xd4528 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd4538 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xd455c | `ret(0xd093c) != 0x0` | `ret(0xd093c) == 0x0` |
| `bne` at 0xd4590 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0xd460c | `C == 1` | `not (C == 1)` |
| `bls` at 0xd4648 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd468c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd46f4 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd4738 | `ret(0xd497c) != 0x0` | `ret(0xd497c) == 0x0` |
| `beq` at 0xd4774 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bhs` at 0xd47d8 | `C == 1` | `not (C == 1)` |
| `bls` at 0xd4814 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0xd4858 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xd48c0 | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bne` at 0xd4904 | `ret(0xd497c) != 0x0` | `ret(0xd497c) == 0x0` |
| `beq` at 0xd4940 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd4978 | `r0` | `?` |

## `scancmd_dump`

- `scancmd_dump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd0168 to 0xd0714.
  Established. Nonzero entry `r3` takes `0xd0198`, while zero falls through (`bne` at 0xd0190).
  The unnamed `0x222f0` call carries unknown, `0x120e38`, entry `r1`, entry `r1` (`bl` at 0xd01d8).
  An included word assignment places `[sp-32]+16` at `sp -0x3c` (`str` at 0xd04b4).
  Scan dump field interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd0178 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd017c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd0180 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd0184 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd0188 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd019c | store | 4 | `sp -0x40` | `r0` |
| `ldr` at 0xd01a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd01ac | load | 4 | `abs 0xd06b0` | `-` |
| `ldr` at 0xd01b0 | load | 4 | `abs 0x12eeb4` | `-` |
| `str` at 0xd01b4 | store | 4 | `sp -0x40` | `[0x12eeb4]` |
| `ldr` at 0xd01b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xd01bc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xd01c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd01cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd01d0 | load | 4 | `abs 0xd06b4` | `-` |
| `ldr` at 0xd01d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xd01e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd01e4 | load | 4 | `abs 0xd06b8` | `-` |
| `ldr` at 0xd01ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd01f0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd01fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0200 | load | 4 | `abs 0xd06bc` | `-` |
| `ldr` at 0xd0210 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0218 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd0224 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0228 | load | 4 | `abs 0xd06c0` | `-` |
| `ldr` at 0xd0234 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0238 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd0244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0248 | load | 4 | `abs 0xd06c4` | `-` |
| `ldr` at 0xd0258 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0260 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd026c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0270 | load | 4 | `abs 0xd06c8` | `-` |
| `ldr` at 0xd027c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0280 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd028c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0290 | load | 4 | `abs 0xd06cc` | `-` |
| `ldr` at 0xd02a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd02a8 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd02b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd02b8 | load | 4 | `abs 0xd06d0` | `-` |
| `ldr` at 0xd02c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd02c8 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xd02d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd02d4 | load | 4 | `abs 0xd06d4` | `-` |
| `ldr` at 0xd02e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd02ec | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd02f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd02fc | load | 4 | `abs 0xd06d8` | `-` |
| `ldr` at 0xd030c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0314 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xd0320 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0324 | load | 4 | `abs 0xd06dc` | `-` |
| `ldr` at 0xd0334 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd033c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd0348 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd034c | load | 4 | `abs 0xd06e0` | `-` |
| `ldr` at 0xd035c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0364 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xd0370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0374 | load | 4 | `abs 0xd06e4` | `-` |
| `ldr` at 0xd0384 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd038c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd0398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd039c | load | 4 | `abs 0xd06e8` | `-` |
| `ldr` at 0xd03ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd03b4 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xd03c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd03c4 | load | 4 | `abs 0xd06ec` | `-` |
| `ldr` at 0xd03d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd03dc | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd03e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd03ec | load | 4 | `abs 0xd06f0` | `-` |
| `ldr` at 0xd03fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0404 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xd040c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0410 | load | 4 | `abs 0xd06f4` | `-` |
| `ldr` at 0xd0420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0428 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd0434 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0438 | load | 4 | `abs 0xd06f8` | `-` |
| `ldr` at 0xd0448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0450 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0454 | load | 4 | `abs 0xd06fc` | `-` |
| `ldr` at 0xd0458 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xd0464 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd046c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xd0474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0478 | load | 4 | `abs 0xd0700` | `-` |
| `ldr` at 0xd0488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd0490 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xd049c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd04ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd04b4 | store | 4 | `sp -0x3c` | `[sp-32]+16` |
| `ldr` at 0xd04bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd04c4 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xd04dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xd04e4 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0xd04f0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd04f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd04fc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd0500 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd0508 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd050c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0510 | load | 4 | `abs 0xd0704` | `-` |
| `str` at 0xd051c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd0520 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0524 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd0530 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0538 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd0540 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd054c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd0558 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0564 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd0570 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0574 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xd0578 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0584 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd0590 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0594 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xd0598 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd05a4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd05b0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd05b4 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xd05b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd05c4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd05d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd05d4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xd05d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd05e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd05f0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd05f4 | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xd05f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0604 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd0610 | load | 4 | `(unknown)` | `-` |
| `str` at 0xd0614 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xd0618 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd061c | load | 4 | `abs 0xd0708` | `-` |
| `ldr` at 0xd0620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd0624 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd062c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0634 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xd063c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd064c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xd0650 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0654 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd0660 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xd0668 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xd0674 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xd067c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd0680 | load | 4 | `abs 0xd070c` | `-` |
| `ldr` at 0xd0684 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xd068c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd0694 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xd069c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd06a0 | load | 4 | `abs 0xd0710` | `-` |
| `ldm` at 0xd06ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd06ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd06ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd01d8 | `(unknown callee) 0x222f0` | `?` | `0x120e38` | `r1` | `r1` |
| `bl` at 0xd01e8 | `(unknown callee) 0x222f0` | `?` | `0x120e3c` | `r2` | `r1` |
| `bl` at 0xd0208 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120e58` | `?` | `?` |
| `bl` at 0xd0230 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120e74` | `?` | `?` |
| `bl` at 0xd0250 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120e84` | `?` | `?` |
| `bl` at 0xd0278 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120ea0` | `?` | `?` |
| `bl` at 0xd0298 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120eb0` | `?` | `?` |
| `bl` at 0xd02c0 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120ecc` | `?` | `?` |
| `bl` at 0xd02dc | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120edc` | `?` | `?` |
| `bl` at 0xd0304 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120ef8` | `?` | `?` |
| `bl` at 0xd032c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f08` | `?` | `?` |
| `bl` at 0xd0354 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f24` | `?` | `?` |
| `bl` at 0xd037c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f34` | `?` | `?` |
| `bl` at 0xd03a4 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f50` | `?` | `?` |
| `bl` at 0xd03cc | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f60` | `?` | `?` |
| `bl` at 0xd03f4 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f7c` | `?` | `?` |
| `bl` at 0xd0418 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120f8c` | `?` | `?` |
| `bl` at 0xd0440 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120fa8` | `?` | `?` |
| `bl` at 0xd045c | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120fbc` | `[[sp-32]+12]` | `[sp-32]+12` |
| `bl` at 0xd0480 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120fd4` | `?` | `?` |
| `bl` at 0xd0514 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x120fe8` | `?` | `?` |
| `bl` at 0xd0628 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x12100c` | `?` | `?` |
| `bl` at 0xd0688 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x121040` | `?` | `?` |
| `bl` at 0xd06a4 | `(unknown callee) 0x222f0` | `[sp-20]` | `0x121048` | `?` | `[sp-56]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd0190 | `r3 != 0x0` | `r3 == 0x0` |
| `bne` at 0xd01a8 | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xd01c8 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xd0498 | `[[sp-32]+12] == 0x0` | `[[sp-32]+12] != 0x0` |
| `bne` at 0xd04a4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0xd052c | `C == 1` | `not (C == 1)` |
| `beq` at 0xd0644 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bhs` at 0xd065c | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd06ac | `r0` | `?` |

## `smpcmd_scan_ids`

- `smpcmd_scan_ids`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbe34 to 0xdc330.
  Established. Unsigned `[[sp-24]+40] <= 0x40` takes `0xdbec4`, while greater values fall through (`bls` at 0xdbea0).
  The unnamed `0xd0754` call carries `0x1`, `[sp-40]`, `[[sp-28]]`, `[sp-28]` (`bl` at 0xdc020).
  The unnamed `0xd0d10` call carries `0x1`, `0x0`, `0x1`, `[sp-40]` (`bl` at 0xdc090).
  Scan identification response semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbe44 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdbe48 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdbe4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdbe5c | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdbe60 | load | 4 | `abs 0xdc2f4` | `-` |
| `ldr` at 0xdbe64 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdbe70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbe74 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbe78 | load | 4 | `abs 0xdc2f8` | `-` |
| `str` at 0xdbe84 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdbe8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbe90 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xdbe94 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xdbe98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbea4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbea8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbeac | load | 4 | `abs 0xdc2fc` | `-` |
| `str` at 0xdbebc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdbec4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbed0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbed4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbed8 | load | 4 | `abs 0xdc300` | `-` |
| `str` at 0xdbee4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdbeec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdbef4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xdbef8 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdbf0c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdbf18 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdbf1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdbf30 | store | 4 | `sp -0x40` | `ret(0x1943c)` |
| `ldr` at 0xdbf34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdbf40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbf44 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbf48 | load | 4 | `abs 0xdc304` | `-` |
| `str` at 0xdbf54 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdbf5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdbf70 | store | 4 | `sp -0x44` | `ret(0x1943c)` |
| `ldr` at 0xdbf74 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdbf80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbf84 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbf88 | load | 4 | `abs 0xdc304` | `-` |
| `ldr` at 0xdbf90 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdbf9c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xdbfa8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdbfac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdbfb0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdbfbc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdbfc4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdbfd0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdbfd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdbfdc | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdbfe8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xdbfec | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdbff4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0xdc00c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xdc010 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc018 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc01c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc030 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc034 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc038 | load | 4 | `abs 0xdc2f8` | `-` |
| `ldr` at 0xdc040 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdc048 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdc054 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdc05c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xdc060 | store | 4 | `sp -0x58` | `[sp-56]` |
| `ldr` at 0xdc064 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xdc068 | store | 4 | `sp -0x54` | `[sp-64]` |
| `ldr` at 0xdc06c | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdc070 | store | 4 | `sp -0x50` | `[sp-68]` |
| `ldr` at 0xdc074 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc078 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xdc07c | store | 4 | `sp -0x4c` | `[[sp-28]]` |
| `ldr` at 0xdc08c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xdc0a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc0a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc0a8 | load | 4 | `abs 0xdc2f8` | `-` |
| `ldr` at 0xdc0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdc0b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdc0c4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdc0cc | load | 4 | `abs 0xdc308` | `-` |
| `ldr` at 0xdc0d4 | load | 4 | `abs 0xdc30c` | `-` |
| `ldr` at 0xdc0dc | load | 4 | `abs 0xdc310` | `-` |
| `ldr` at 0xdc0e4 | load | 4 | `abs 0xdc314` | `-` |
| `str` at 0xdc0f0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdc0f8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xdc0fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdc100 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc10c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc110 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdc124 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdc12c | store | 4 | `sp -0x3c` | `[sp-60]+1` |
| `ldr` at 0xdc130 | load | 4 | `abs 0xdc318` | `-` |
| `ldr` at 0xdc134 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdc144 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0xdc150 | store | 4 | `sp -0x58` | `0x20` |
| `ldr` at 0xdc154 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xdc158 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xdc168 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xdc170 | store | 4 | `sp -0x3c` | `[sp-60]+32` |
| `ldr` at 0xdc174 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc178 | load | 4 | `abs 0xdc31c` | `-` |
| `ldr` at 0xdc184 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc18c | load | 4 | `abs 0xdc31c` | `-` |
| `str` at 0xdc198 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc19c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc1a0 | load | 4 | `abs 0xdc320` | `-` |
| `ldr` at 0xdc1ac | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc1b4 | load | 4 | `abs 0xdc320` | `-` |
| `str` at 0xdc1c0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc1c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc1c8 | load | 4 | `abs 0xdc324` | `-` |
| `ldr` at 0xdc1d4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc1dc | load | 4 | `abs 0xdc324` | `-` |
| `str` at 0xdc1e8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc1ec | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc1fc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc210 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc214 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xdc224 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc238 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xdc240 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xdc24c | load | 1 | `sp -0x30` | `-` |
| `ldr` at 0xdc254 | load | 4 | `sp -0x30` | `-` |
| `ldrh` at 0xdc264 | load | 2 | `sp -0x2e` | `-` |
| `str` at 0xdc27c | store | 4 | `sp -0x58` | `?` |
| `ldrb` at 0xdc280 | load | 1 | `sp -0x2d` | `-` |
| `str` at 0xdc28c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xdc290 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdc294 | store | 4 | `sp -0x50` | `[sp-52]` |
| `ldr` at 0xdc298 | load | 4 | `abs 0xdc328` | `-` |
| `ldr` at 0xdc29c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdc2a8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xdc2b0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xdc2b8 | load | 4 | `abs 0xdc2f8` | `-` |
| `ldr` at 0xdc2c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc2c4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc2c8 | load | 4 | `abs 0xdc32c` | `-` |
| `ldr` at 0xdc2d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xdc2d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xdc2e4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xdc2e8 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0xdc2f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdc2f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc2f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc2f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbe54 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdbe7c | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdbeb4 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121b64` | `0x40` | `[sp-28]` |
| `bl` at 0xdbedc | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121b8c` | `?` | `[sp-28]` |
| `bl` at 0xdbf28 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xdbf4c | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121bb0` | `?` | `[sp-28]` |
| `bl` at 0xdbf68 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0xdbf8c | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121bb0` | `?` | `[sp-28]` |
| `bl` at 0xdbf94 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdbffc | `(unknown callee) 0xd9588` | `ret(0x1943c)` | `?` | `?` | `[sp-36]` |
| `bl` at 0xdc020 | `(unknown callee) 0xd0754` | `0x1` | `[sp-40]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xdc03c | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdc044 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdc04c | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdc090 | `(unknown callee) 0xd0d10` | `0x1` | `0x0` | `0x1` | `[sp-40]` |
| `bl` at 0xdc0ac | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdc0b4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdc0bc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0xdc0d0 | `(unknown callee) 0x221f0` | `0x121bdc` | `?` | `?` | `ret(0xd0d10)` |
| `bl` at 0xdc0d8 | `(unknown callee) 0x221f0` | `0x121be0` | `?` | `?` | `?` |
| `bl` at 0xdc0e0 | `(unknown callee) 0x221f0` | `0x121bf8` | `?` | `?` | `?` |
| `bl` at 0xdc0e8 | `(unknown callee) 0x221f0` | `0x121c28` | `?` | `?` | `?` |
| `bl` at 0xdc114 | `(unknown callee) 0xd5a08` | `?` | `[sp-68]` | `?` | `[sp-32]` |
| `bl` at 0xdc138 | `(unknown callee) 0x221f0` | `0x121c58` | `[sp-44]` | `?` | `[sp-60]+1` |
| `bl` at 0xdc164 | `(unknown callee) 0xd5e9c` | `[sp-60]` | `[sp-68]` | `0x0` | `sp-52` |
| `bl` at 0xdc248 | `(unknown callee) 0xdc584` | `[sp-52]` | `sp-48` | `?` | `sp-48` |
| `bl` at 0xdc2a4 | `(unknown callee) 0x221f0` | `0x121c84` | `[sp-44]` | `?` | `?` |
| `bl` at 0xdc2bc | `(unknown callee) 0x221f0` | `0x121a90` | `?` | `?` | `[sp-32]` |
| `bl` at 0xdc2cc | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121cbc` | `?` | `[sp-28]` |
| `bl` at 0xdc2d4 | `(unknown callee) 0x1970c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xdc2dc | `(unknown callee) 0x1970c` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdbe6c | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `bls` at 0xdbea0 | `[[sp-24]+40] <= 0x40 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x40 (unsigned, 32-bit)` |
| `bne` at 0xdbecc | `[[sp-24]+40] != 0x0` | `[[sp-24]+40] == 0x0` |
| `bne` at 0xdbf3c | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bne` at 0xdbf7c | `ret(0x1943c) != 0x0` | `ret(0x1943c) == 0x0` |
| `bhs` at 0xdbfb8 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdc02c | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `bne` at 0xdc09c | `ret(0xd0d10) != 0x0` | `ret(0xd0d10) == 0x0` |
| `bhs` at 0xdc108 | `C == 1` | `not (C == 1)` |
| `bne` at 0xdc120 | `ret(0xd5a08) != 0x0` | `ret(0xd5a08) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc2f0 | `r0` | `0x0` |

## `sn1sc_reset_sic`

- `sn1sc_reset_sic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd90fc to 0xd9588.
  Established. Nonzero `[0x297378]` takes `0xd9140`, while zero falls through (`bne` at 0xd9130).
  Included halfword stores write unknown values to `[0x29c464] +0x0` (`strh` at 0xd92c8, `strh` at 0xd9350, `strh` at 0xd93d4, `strh` at 0xd9458, `strh` at 0xd94dc, `strh` at 0xd9560).
  SIC reset pin sequence interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd910c | load | 4 | `abs 0xd9578` | `-` |
| `ldr` at 0xd9110 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0xd911c | load | 4 | `abs 0xd957c` | `-` |
| `ldr` at 0xd9124 | load | 4 | `abs 0xd9580` | `-` |
| `ldr` at 0xd9128 | load | 4 | `abs 0x297378` | `-` |
| `str` at 0xd9138 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xd9140 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9144 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9148 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9164 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9168 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd916c | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9190 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9194 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9198 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd91bc | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd91c0 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd91c4 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd91e8 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd91ec | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd91f0 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9214 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9218 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd921c | load | 2 | `[0x29c464] +0x0` | `-` |
| `str` at 0xd9240 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xd9244 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd924c | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xd9250 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9254 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd9258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9264 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9278 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd928c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd92a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd92b4 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd92c8 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xd92cc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd92d4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xd92d8 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd92dc | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd92e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd92ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9300 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9314 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9328 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd933c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd9350 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xd9358 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xd935c | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9360 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd9364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9370 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9384 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9398 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd93ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd93c0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd93d4 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xd93dc | store | 4 | `sp -0x14` | `0x28` |
| `ldr` at 0xd93e0 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd93e4 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd93e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd93f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9408 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd941c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9444 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd9458 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xd9460 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xd9464 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd9468 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd946c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9478 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd948c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94c8 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd94dc | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xd94e4 | store | 4 | `sp -0x14` | `0x30` |
| `ldr` at 0xd94e8 | load | 4 | `abs 0xd9584` | `-` |
| `ldr` at 0xd94ec | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd94f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd94fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9510 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd954c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd9560 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xd9568 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xd956c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xd9574 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9574 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9574 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9120 | `(unknown callee) 0x221f0` | `0x12169c` | `r1` | `r2` | `[0x29737c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9118 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `bne` at 0xd9130 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9574 | `r0` | `?` |

## `tdrdata_reset`

- `tdrdata_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe5150 to 0xe532c.
  Established. Zero `[r0+8]` takes `0xe5234`, while nonzero falls through (`beq` at 0xe5170).
  Included word and byte stores write `0x0` to unknown effective targets (`str` at 0xe52c8, `strb` at 0xe5310).
  A word assignment increments `sp -0x18` with `[sp-24]+1` (`str` at 0xe531c).
  Scan data reset bounds remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe5160 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xe5164 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5168 | load | 4 | `r0 +0x8` | `-` |
| `str` at 0xe5178 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe517c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5180 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe5184 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe519c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe51a0 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe51a4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe51b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe51c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe51c8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe51cc | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe51e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe51f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe51f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe51fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5204 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5208 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe520c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5214 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe521c | load | 4 | `(unknown)` | `-` |
| `str` at 0xe5220 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe5224 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe522c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xe5238 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe523c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5240 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe5244 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe525c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5260 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe5264 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe5274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5288 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe528c | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe52a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe52b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe52b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe52bc | load | 4 | `(unknown)` | `-` |
| `str` at 0xe52c8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xe52cc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe52d4 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0xe52e0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xe52e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe52e8 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0xe52ec | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe52f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe52fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xe5300 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe5304 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xe5310 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xe5314 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xe531c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0xe5328 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe5328 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe5328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xe5170 | `[r0+8] == 0x0` | `[r0+8] != 0x0` |
| `bne` at 0xe5198 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe51bc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xe51e8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0xe5258 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0xe527c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bhi` at 0xe52a8 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bls` at 0xe52f8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe5328 | `r0` | `?` |

## `tdrdata_reset_field`

- `tdrdata_reset_field`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe532c to 0xe53fc.
  Established. The unnamed `0xd5e9c` call carries unknown, `[[r0+8]+8]`, unknown, `[r0+16]` (`bl` at 0xe53d8).
  The byte store clears `([[sp-24]+12] + [sp-28]) +0x0` (`strb` at 0xe53f0);
  its integer return candidate is `ret(0xd5e9c)` (`ldm` at 0xe53f8).
  Field reset indexing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xe533c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xe5340 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xe5344 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5348 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe534c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5360 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xe5368 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe536c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5370 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe5374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe5378 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe537c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe5390 | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xe5398 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xe539c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe53a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe53a4 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe53a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xe53bc | load | 4 | `[r0+8] +0x4` | `-` |
| `ldrh` at 0xe53c4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xe53c8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xe53cc | load | 4 | `[r0+8] +0x8` | `-` |
| `ldr` at 0xe53d4 | load | 4 | `r0 +0x10` | `-` |
| `ldr` at 0xe53dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xe53e0 | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xe53e4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0xe53f0 | store | 1 | `([[sp-24]+12] + [sp-28]) +0x0` | `0x0` |
| `ldm` at 0xe53f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xe53f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe53f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe53f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xe53d8 | `(unknown callee) 0xd5e9c` | `?` | `[[r0+8]+8]` | `?` | `[r0+16]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe53f8 | `r0` | `ret(0xd5e9c)` |

## `smpcmd_scan_imp`

- `smpcmd_scan_imp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc4d4 to 0xdc52c.
  Established. An included word assignment stores `ret(0x1943c)` at entry `r1 +0x0` (`str` at 0xdc4fc).
  The unnamed `0x197a8` call carries `[[sp-28]]`, `0x121d3c`, unknown, `[sp-28]` (`bl` at 0xdc514).
  The integer return record contains `0x0` (`ldm` at 0xdc524).
  Scan implementation command meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc4e4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc4e8 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdc4ec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc4fc | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `str` at 0xdc504 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xdc508 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc50c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc510 | load | 4 | `abs 0xdc528` | `-` |
| `ldm` at 0xdc524 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdc524 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc524 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc524 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc4f4 | `(unknown callee) 0x1943c` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xdc514 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121d3c` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc524 | `r0` | `0x0` |

## `smpcmd_scan_test`

- `smpcmd_scan_test`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc52c to 0xdc584.
  Established. The unnamed `0x1943c` call carries `0xa0`, entry `r1`, entry `r2`, entry `r3` (`bl` at 0xdc54c).
  A word assignment clears `sp -0x20` (`str` at 0xdc55c).
  The unnamed `0x197a8` call carries `[[sp-28]]`, `0x121d74`, unknown, `[sp-28]` (`bl` at 0xdc56c).
  Scan test command policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc53c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc540 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdc544 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc554 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `str` at 0xdc55c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xdc560 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc564 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc568 | load | 4 | `abs 0xdc580` | `-` |
| `ldm` at 0xdc57c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdc57c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc57c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc57c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc54c | `(unknown callee) 0x1943c` | `0xa0` | `r1` | `r2` | `r3` |
| `bl` at 0xdc56c | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121d74` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc57c | `r0` | `0x0` |

## `JTAG_Cfg_set_ports_stub`

- `JTAG_Cfg_set_ports_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb888 to 0xdb8b0.
  Established. Word assignments preserve entry `r0` and `r1` at `sp -0x14` and `sp -0x18` (`str` at 0xdb898, `str` at 0xdb89c).
  The integer return candidate is `0x0` (`ldm` at 0xdb8ac).
  Stub port selection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb898 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdb89c | store | 4 | `sp -0x18` | `r1` |
| `ldm` at 0xdb8ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb8ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb8ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb8ac | `r0` | `0x0` |

## `JTAG_Cfg_display_stub`

- `JTAG_Cfg_display_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb8b0 to 0xdb8c0.
  Established. The return record retains entry `r0` (`ldm` at 0xdb8bc).
  Display callback behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0xdb8bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb8bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb8bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb8bc | `r0` | `r0` |

## `JTAG_Cfg_L3SetCfg_stub`

- `JTAG_Cfg_L3SetCfg_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb868 to 0xdb888.
  Established. The word at `sp -0x14` receives entry `r0` (`str` at 0xdb878);
  its integer return candidate contains `0x1` (`ldm` at 0xdb884).
  Configuration stub result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb878 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0xdb884 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdb884 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb884 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb884 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb884 | `r0` | `0x1` |

## `JTAG_LLP_SC_stub`

- `JTAG_LLP_SC_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb8e0 to 0xdb900.
  Established. A word store saves entry `r0` at `sp -0x14` (`str` at 0xdb8f0).
  The integer return record contains `0x0` (`ldm` at 0xdb8fc).
  LLP stub completion meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb8f0 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0xdb8fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdb8fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdb8fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdb8fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdb8fc | `r0` | `0x0` |

## `PPPReset`

- `PPPReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xadd98 to 0xaddfc.
  Established. Included word stores clear entry `r0 +0xc` and `[sp-20] +0x30` (`str` at 0xaddc8, `str` at 0xaddd4).
  Another word store writes `0x0` to an unknown effective target (`str` at 0xaddf0).
  The integer return candidate preserves entry `r0` (`ldm` at 0xaddf8).
  PPP reset field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xadda8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaddac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xaddb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaddc0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaddc8 | store | 4 | `r0 +0xc` | `0x0` |
| `ldr` at 0xaddcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaddd4 | store | 4 | `[sp-20] +0x30` | `0x0` |
| `ldr` at 0xaddd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xadde8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xaddf0 | store | 4 | `(unknown)` | `0x0` |
| `ldm` at 0xaddf8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xaddf8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xaddf8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaddbc | `? == 0x0` | `? != 0x0` |
| `beq` at 0xadde4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xaddf8 | `r0` | `r0` |
