# ARM temperature and regulator paths in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/temperature-vrm-facts.txt) retain all tool sections.

## `tempInit`

- `tempInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24214 to 0x24340.
  Established. The word at `0x132064` receives `0x1` (`str` at 0x24298). The word at `0x132060`
  receives `ret(0x196d0)` (`str` at 0x242c0). A byte through `[sp-36] +0x4` receives `([sp-32] & 0xff)` (`strb` at 0x24320). Temperature initialization policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24224 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x24228 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x2422c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x24230 | load | 4 | `abs 0x2432c` | `-` |
| `ldr` at 0x24234 | load | 4 | `abs 0x132060` | `-` |
| `ldr` at 0x24240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x2424c | load | 4 | `abs 0x24330` | `-` |
| `str` at 0x24250 | store | 4 | `sp -0x34` | `0xee4dc` |
| `ldr` at 0x24254 | load | 4 | `abs 0x2432c` | `-` |
| `ldr` at 0x24258 | load | 4 | `abs 0x132060` | `-` |
| `str` at 0x2425c | store | 4 | `sp -0x30` | `[0x132060]` |
| `ldr` at 0x24260 | load | 4 | `abs 0x24334` | `-` |
| `ldr` at 0x24264 | load | 4 | `abs 0x132064` | `-` |
| `str` at 0x24268 | store | 4 | `sp -0x2c` | `[0x132064]` |
| `ldr` at 0x24270 | load | 4 | `abs 0x24338` | `-` |
| `ldr` at 0x24280 | load | 4 | `abs 0x24334` | `-` |
| `ldr` at 0x24284 | load | 4 | `abs 0x132064` | `-` |
| `ldr` at 0x24290 | load | 4 | `abs 0x24334` | `-` |
| `str` at 0x24298 | store | 4 | `abs 0x132064` | `0x1` |
| `ldr` at 0x2429c | load | 4 | `abs 0x24334` | `-` |
| `ldr` at 0x242a0 | load | 4 | `abs 0x132064` | `-` |
| `str` at 0x242a8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x242ac | load | 4 | `abs 0x2432c` | `-` |
| `ldr` at 0x242b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x242c0 | store | 4 | `abs 0x132060` | `ret(0x196d0)` |
| `ldr` at 0x242c4 | load | 4 | `abs 0x132060` | `-` |
| `ldr` at 0x242d0 | load | 4 | `abs 0x2433c` | `-` |
| `str` at 0x242d4 | store | 4 | `sp -0x34` | `0xee520` |
| `ldr` at 0x242d8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x242dc | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x242e4 | load | 4 | `abs 0x24338` | `-` |
| `ldr` at 0x242f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x242fc | load | 4 | `abs 0x2432c` | `-` |
| `ldr` at 0x24300 | load | 4 | `abs 0x132060` | `-` |
| `str` at 0x24308 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x2430c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24310 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x24314 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x24318 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x2431c | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x24320 | store | 1 | `[sp-36] +0x4` | `([sp-32] & 0xff)` |
| `ldm` at 0x24328 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24328 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24328 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24328 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2427c | `(unknown callee) 0x200a0` | `0x9` | `0xee50c` | `0x3a` | `0x0` |
| `bl` at 0x242b8 | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `0x132064` | `?` |
| `bl` at 0x242f0 | `(unknown callee) 0x200a0` | `0x9` | `0xee50c` | `0x46` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2423c | `[0x132060] == 0x0` | `[0x132060] != 0x0` |
| `bne` at 0x24248 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x2428c | `[0x132064] != 0x0` | `[0x132064] == 0x0` |
| `bne` at 0x242cc | `[0x132060] != 0x0` | `[0x132060] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24328 | `r0` | `?` |

## `tempGetCfg`

- `tempGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24340 to 0x24398.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0x2437c). The
  absolute word at `0x132060` is read (`ldr` at 0x24364). The included return record carries
  `[[sp-28]+4]:1` in `r0` (`ldm` at 0x24390). Temperature configuration layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24350 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24354 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x24358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24360 | load | 4 | `abs 0x24394` | `-` |
| `ldr` at 0x24364 | load | 4 | `abs 0x132060` | `-` |
| `str` at 0x2436c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x24370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24374 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24378 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x2437c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x24380 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24384 | load | 1 | `[sp-28] +0x4` | `-` |
| `ldm` at 0x24390 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24390 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24390 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24390 | `r0` | `[[sp-28]+4]:1` |

## `tempCount`

- `tempCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24398 to 0x243dc.
  Established. The byte through `[sp-24] +0x4` is read (`ldrb` at 0x243c8). The absolute word at
  `0x132060` is read (`ldr` at 0x243b8). The included return record carries `[[sp-24]+4]:1` in `r0`
  (`ldm` at 0x243d4). Temperature count representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x243a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x243ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x243b4 | load | 4 | `abs 0x243d8` | `-` |
| `ldr` at 0x243b8 | load | 4 | `abs 0x132060` | `-` |
| `str` at 0x243c0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x243c4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x243c8 | load | 1 | `[sp-24] +0x4` | `-` |
| `ldm` at 0x243d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x243d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x243d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x243d4 | `r0` | `[[sp-24]+4]:1` |

## `tempIsPresent`

- `tempIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x243dc to 0x2442c.
  Established. The unnamed `0xbfb1c` call carries `[sp-20]`, `[ret(0x24760)]:1`, unknown,
  `[ret(0x24760)]:1` (`bl` at 0x24418). The byte through `ret(0x24760) +0x0` is read (`ldrb` at 0x2440c). The included return record carries `ret(0xbfb1c)` in `r0` (`ldm` at 0x24428).
  Temperature presence query remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x243ec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x243f4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x243f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x243fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x2440c | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x24410 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24428 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24428 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24428 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24404 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x24418 | `(unknown callee) 0xbfb1c` | `[sp-20]` | `[ret(0x24760)]:1` | `?` | `[ret(0x24760)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24428 | `r0` | `ret(0xbfb1c)` |

## `tempSetLimits`

- `tempSetLimits`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2442c to 0x244b4.
  Established. The unnamed `0xc01e8` call carries `[sp-20]`, `[ret(0x24760)]:1`,
  `[ret(0x24760)+1]:1`, `[sp-22]:1` (`bl` at 0x244a0). The word at `sp -0x20` receives `[sp-23]:1`
  before that included call (`str` at 0x2448c). The included return record gives `ret(0xc01e8)` in
  `r0` (`ldm` at 0x244b0). Temperature limit semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2443c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x24448 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x24450 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x24458 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldrb` at 0x2445c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24460 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24470 | store | 4 | `sp -0x1c` | `ret(0x24760)` |
| `ldr` at 0x24474 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24478 | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x2447c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24480 | load | 1 | `ret(0x24760) +0x1` | `-` |
| `ldrb` at 0x24484 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x24488 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x2448c | store | 4 | `sp -0x20` | `[sp-23]:1` |
| `ldr` at 0x24490 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x244b0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x244b0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x244b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24468 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x244a0 | `(unknown callee) 0xc01e8` | `[sp-20]` | `[ret(0x24760)]:1` | `[ret(0x24760)+1]:1` | `[sp-22]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x244b0 | `r0` | `ret(0xc01e8)` |

## `tempIntDisable`

- `tempIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x244b4 to 0x24518.
  Established. The unnamed `0xc12c0` call carries `[sp-20]`, `[ret(0x24760)]:1`,
  `[ret(0x24760)+1]:1`, `[ret(0x24760)+1]:1` (`bl` at 0x24504). The byte through `ret(0x24760) +0x1`
  is read (`ldrb` at 0x244f4). The included return record carries `ret(0xc12c0)` in `r0` (`ldm` at 0x24514). Temperature interrupt disable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x244c4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x244cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x244d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x244d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x244e4 | store | 4 | `sp -0x1c` | `ret(0x24760)` |
| `ldr` at 0x244e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x244ec | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x244f0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x244f4 | load | 1 | `ret(0x24760) +0x1` | `-` |
| `ldr` at 0x244f8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24514 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24514 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24514 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x244dc | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x24504 | `(unknown callee) 0xc12c0` | `[sp-20]` | `[ret(0x24760)]:1` | `[ret(0x24760)+1]:1` | `[ret(0x24760)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24514 | `r0` | `ret(0xc12c0)` |

## `tempIntEnable`

- `tempIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24518 to 0x2457c.
  Established. The unnamed `0xc130c` call carries `[sp-20]`, `[ret(0x24760)]:1`,
  `[ret(0x24760)+1]:1`, `[ret(0x24760)+1]:1` (`bl` at 0x24568). The byte through `ret(0x24760) +0x0`
  is read (`ldrb` at 0x24550). The included return record gives `ret(0xc130c)` in `r0` (`ldm` at 0x24578). Temperature interrupt enable meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24528 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x24530 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x24534 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24538 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24548 | store | 4 | `sp -0x1c` | `ret(0x24760)` |
| `ldr` at 0x2454c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24550 | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x24554 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24558 | load | 1 | `ret(0x24760) +0x1` | `-` |
| `ldr` at 0x2455c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24578 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24578 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24540 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x24568 | `(unknown callee) 0xc130c` | `[sp-20]` | `[ret(0x24760)]:1` | `[ret(0x24760)+1]:1` | `[ret(0x24760)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24578 | `r0` | `ret(0xc130c)` |

## `tempRead`

- `tempRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2457c to 0x24628.
  Established. The unnamed `0xc0808` call carries `[sp-20]`, `[ret(0x24760)]:1`,
  `[ret(0x24760)+1]:1`, `sp-33` (`bl` at 0x245d8); its zero result takes `0x24608`, while nonzero
  falls through (`beq` at 0x245ec). An included word assignment through `[sp-28] +0x0` writes zero
  (`str` at 0x245f8). Temperature read result format remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x2458c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24594 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x24598 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x2459c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x245a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245b0 | store | 4 | `sp -0x20` | `ret(0x24760)` |
| `ldr` at 0x245b4 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x245b8 | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x245bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x245c0 | load | 1 | `ret(0x24760) +0x1` | `-` |
| `ldr` at 0x245c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x245e0 | store | 4 | `sp -0x28` | `ret(0xc0808)` |
| `ldr` at 0x245e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x245f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x245f8 | store | 4 | `[sp-28] +0x0` | `0x0` |
| `ldr` at 0x245fc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x24600 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x24608 | load | 4 | `sp -0x1c` | `-` |
| `ldrsb` at 0x2460c | load | 1 | `sp -0x21` | `-` |
| `str` at 0x24610 | store | 4 | `[sp-28] +0x0` | `?` |
| `str` at 0x24618 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x2461c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x24624 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24624 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24624 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x245a8 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x245d8 | `(unknown callee) 0xc0808` | `[sp-20]` | `[ret(0x24760)]:1` | `[ret(0x24760)+1]:1` | `sp-33` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x245ec | `ret(0xc0808) == 0x0` | `ret(0xc0808) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24624 | `r0` | `[sp-44]` |

## `tempLocation`

- `tempLocation`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24628 to 0x246a0.
  Established. A nonzero `[ret(0x24760)+4]` takes `0x24690`, while zero falls through (`bne` at 0x24674). The unnamed `0xc1680` call carries `[sp-20]`, `[ret(0x24760)]:1`, unknown,
  `[ret(0x24760)]:1` (`bl` at 0x24688). The word at `sp -0x20` receives `ret(0xc1680)` on that
  included path (`str` at 0x2468c). Temperature location selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24638 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x24640 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x24644 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24648 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24658 | store | 4 | `sp -0x1c` | `ret(0x24760)` |
| `ldr` at 0x2465c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24660 | load | 4 | `ret(0x24760) +0x4` | `-` |
| `str` at 0x24664 | store | 4 | `sp -0x20` | `[ret(0x24760)+4]` |
| `ldr` at 0x24668 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x2466c | load | 4 | `ret(0x24760) +0x4` | `-` |
| `ldr` at 0x24678 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x2467c | load | 1 | `ret(0x24760) +0x0` | `-` |
| `ldr` at 0x24680 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x2468c | store | 4 | `sp -0x20` | `ret(0xc1680)` |
| `ldr` at 0x24690 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x2469c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2469c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2469c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24650 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x24688 | `(unknown callee) 0xc1680` | `[sp-20]` | `[ret(0x24760)]:1` | `?` | `[ret(0x24760)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x24674 | `[ret(0x24760)+4] != 0x0` | `[ret(0x24760)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2469c | `r0` | `?` |

## `tempType`

- `tempType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x246a0 to 0x24760.
  Established. A zero byte through `ret(0x24760) +0x2` takes `0x246f4`, while nonzero falls through
  (`beq` at 0x246e0). The unnamed `0xbfe74` call carries `[sp-20]`, `sp-32`, unknown, `sp-32` (`bl`
  at 0x24700). An included word assignment at `sp -0x28` writes zero (`str` at 0x24750). Temperature
  type interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x246b0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x246b8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x246bc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x246c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x246d0 | store | 4 | `sp -0x1c` | `ret(0x24760)` |
| `ldr` at 0x246d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x246d8 | load | 1 | `ret(0x24760) +0x2` | `-` |
| `ldr` at 0x246e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x246e8 | load | 1 | `ret(0x24760) +0x2` | `-` |
| `str` at 0x246ec | store | 4 | `sp -0x28` | `[ret(0x24760)+2]:1` |
| `ldr` at 0x246f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x24708 | store | 4 | `sp -0x24` | `ret(0xbfe74)` |
| `ldr` at 0x2470c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24710 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x24714 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24720 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24724 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x24738 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x24740 | load | 1 | `(unknown)` | `-` |
| `str` at 0x24744 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x24750 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x24754 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x2475c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2475c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2475c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x246c8 | `(unknown callee) 0x24760` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x24700 | `(unknown callee) 0xbfe74` | `[sp-20]` | `sp-32` | `?` | `sp-32` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x246e0 | `[ret(0x24760)+2]:1 == 0x0` | `[ret(0x24760)+2]:1 != 0x0` |
| `bge` at 0x2471c | `[[sp-28]]:1 >= ret(0xbfe74) (signed, 32-bit)` | `[[sp-28]]:1 < ret(0xbfe74) (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2475c | `r0` | `?` |

## `vrmInit`

- `vrmInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24884 to 0x24ab4.
  Established. The absolute word at `0x13206c` receives `0x1` (`str` at 0x24908). The word at
  `0x132068` receives `ret(0x196d0)` (`str` at 0x2493c). An included byte assignment through
  `[sp-36] +0x8` writes `([sp-32] & 0xff)` (`strb` at 0x249a8). Regulator initialization effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24894 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x24898 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x2489c | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x248a0 | load | 4 | `abs 0x24a94` | `-` |
| `ldr` at 0x248a4 | load | 4 | `abs 0x132068` | `-` |
| `ldr` at 0x248b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x248bc | load | 4 | `abs 0x24a98` | `-` |
| `str` at 0x248c0 | store | 4 | `sp -0x34` | `0xee588` |
| `ldr` at 0x248c4 | load | 4 | `abs 0x24a94` | `-` |
| `ldr` at 0x248c8 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x248cc | store | 4 | `sp -0x30` | `[0x132068]` |
| `ldr` at 0x248d0 | load | 4 | `abs 0x24a9c` | `-` |
| `ldr` at 0x248d4 | load | 4 | `abs 0x13206c` | `-` |
| `str` at 0x248d8 | store | 4 | `sp -0x2c` | `[0x13206c]` |
| `ldr` at 0x248e0 | load | 4 | `abs 0x24aa0` | `-` |
| `ldr` at 0x248f0 | load | 4 | `abs 0x24a9c` | `-` |
| `ldr` at 0x248f4 | load | 4 | `abs 0x13206c` | `-` |
| `ldr` at 0x24900 | load | 4 | `abs 0x24a9c` | `-` |
| `str` at 0x24908 | store | 4 | `abs 0x13206c` | `0x1` |
| `ldr` at 0x2490c | load | 4 | `abs 0x24a9c` | `-` |
| `ldr` at 0x24910 | load | 4 | `abs 0x13206c` | `-` |
| `str` at 0x24924 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x24928 | load | 4 | `abs 0x24a94` | `-` |
| `ldr` at 0x2492c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x2493c | store | 4 | `abs 0x132068` | `ret(0x196d0)` |
| `ldr` at 0x24940 | load | 4 | `abs 0x132068` | `-` |
| `ldr` at 0x2494c | load | 4 | `abs 0x24aa4` | `-` |
| `str` at 0x24950 | store | 4 | `sp -0x34` | `0xee5cc` |
| `ldr` at 0x24954 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x24958 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x24960 | load | 4 | `abs 0x24aa0` | `-` |
| `ldr` at 0x24970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24984 | load | 4 | `abs 0x24a94` | `-` |
| `ldr` at 0x24988 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24990 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x24994 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24998 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x2499c | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x249a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x249a4 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x249a8 | store | 1 | `[sp-36] +0x8` | `([sp-32] & 0xff)` |
| `ldr` at 0x249ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x249b0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x249b4 | load | 1 | `[sp-36] +0x8` | `-` |
| `str` at 0x249c8 | store | 4 | `[sp-36] +0x4` | `ret(0x196d0)` |
| `ldr` at 0x249cc | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x249d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x249e0 | load | 4 | `abs 0x24aa8` | `-` |
| `ldr` at 0x249e4 | load | 4 | `abs 0x24aac` | `-` |
| `str` at 0x24a20 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x24a24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24a2c | load | 4 | `abs 0x24aa8` | `-` |
| `ldr` at 0x24a30 | load | 4 | `abs 0x24aac` | `-` |
| `ldr` at 0x24a3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x24a4c | load | 4 | `abs 0x24ab0` | `-` |
| `str` at 0x24a50 | store | 4 | `(unknown)` | `0xee610` |
| `str` at 0x24a54 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0x24a58 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x24a5c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x24a64 | load | 4 | `abs 0x24aa0` | `-` |
| `ldr` at 0x24a74 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x24a78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x24a7c | load | 1 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x24a80 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldm` at 0x24a90 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24a90 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24a90 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24a90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x248ec | `(unknown callee) 0x200a0` | `0x9` | `0xee5b8` | `0x36` | `0x0` |
| `bl` at 0x24934 | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `[0x13206c]` | `?` |
| `bl` at 0x2496c | `(unknown callee) 0x200a0` | `0x9` | `0xee5b8` | `0x42` | `0x0` |
| `bl` at 0x249c0 | `(unknown callee) 0x196d0` | `[[sp-36]+8]:1` | `0x1` | `[sp-36]` | `[[sp-36]+8]:1` |
| `bl` at 0x249e8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x249f4 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x24a34 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24a44 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x24a70 | `(unknown callee) 0x200a0` | `0x9` | `0xee5b8` | `0x4d` | `0x0` |
| `bl` at 0x24a88 | `(unknown callee) 0x19748` | `[[sp-36]+4]` | `0x1` | `[[sp-36]+8]:1` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x248ac | `[0x132068] == 0x0` | `[0x132068] != 0x0` |
| `bne` at 0x248b8 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0x248fc | `[0x13206c] != 0x0` | `[0x13206c] == 0x0` |
| `bne` at 0x24948 | `[0x132068] != 0x0` | `[0x132068] == 0x0` |
| `bne` at 0x249d4 | `[[sp-36]+4] != 0x0` | `[[sp-36]+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24a90 | `r0` | `ret(0x19748)` |

## `vrmDisable`

- `vrmDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24ab4 to 0x24bdc.
  Established. The unsigned condition `[[sp-28]+8]:1 <= (r1 & 0xff)` takes `0x24b24`, while greater
  values fall through (`bls` at 0x24b04). The included byte assignment through `((r1 & 0xff) + [[sp-28]+4]) +0x0` writes zero (`strb` at 0x24b1c). The unnamed `0x200a0` call carries `0x6`,
  `0xee5b8`, `0x5c`, `0x0` (`bl` at 0x24bbc). Regulator disable behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24ac4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x24acc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x24ad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24ae4 | load | 4 | `abs 0x24bc8` | `-` |
| `ldr` at 0x24ae8 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24af0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x24af4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24af8 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x24afc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24b08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24b0c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24b10 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x24b1c | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x0` |
| `ldr` at 0x24b24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24b2c | load | 4 | `abs 0x24bcc` | `-` |
| `ldr` at 0x24b30 | load | 4 | `abs 0x24bd0` | `-` |
| `str` at 0x24b6c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x24b70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24b78 | load | 4 | `abs 0x24bcc` | `-` |
| `ldr` at 0x24b7c | load | 4 | `abs 0x24bd0` | `-` |
| `ldr` at 0x24b88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24b98 | load | 4 | `abs 0x24bd4` | `-` |
| `str` at 0x24b9c | store | 4 | `(unknown)` | `0xee64c` |
| `str` at 0x24ba0 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x24ba4 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x24ba8 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x24bb0 | load | 4 | `abs 0x24bd8` | `-` |
| `ldm` at 0x24bc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24bc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24bc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24b34 | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24b40 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x24b80 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24b90 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x24bbc | `(unknown callee) 0x200a0` | `0x6` | `0xee5b8` | `0x5c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x24b04 | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24bc4 | `r0` | `?` |

## `vrmEnable`

- `vrmEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24bdc to 0x24d04.
  Established. An included byte assignment through `((r1 & 0xff) + [[sp-28]+4]) +0x0` writes `0x1`
  (`strb` at 0x24c44). The absolute word at `0x132068` is read (`ldr` at 0x24c10). The unnamed
  `0x200a0` call carries `0x6`, `0xee5b8`, `0x6a`, `0x0` (`bl` at 0x24ce4). Regulator enable
  semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24bec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x24bf4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x24bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24c0c | load | 4 | `abs 0x24cf0` | `-` |
| `ldr` at 0x24c10 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24c18 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x24c1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24c20 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x24c24 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24c30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24c34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x24c38 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x24c44 | store | 1 | `((r1 & 0xff) + [[sp-28]+4]) +0x0` | `0x1` |
| `ldr` at 0x24c4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24c54 | load | 4 | `abs 0x24cf4` | `-` |
| `ldr` at 0x24c58 | load | 4 | `abs 0x24cf8` | `-` |
| `str` at 0x24c94 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x24c98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24ca0 | load | 4 | `abs 0x24cf4` | `-` |
| `ldr` at 0x24ca4 | load | 4 | `abs 0x24cf8` | `-` |
| `ldr` at 0x24cb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x24cc0 | load | 4 | `abs 0x24cfc` | `-` |
| `str` at 0x24cc4 | store | 4 | `(unknown)` | `0xee668` |
| `str` at 0x24cc8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0x24ccc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x24cd0 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x24cd8 | load | 4 | `abs 0x24d00` | `-` |
| `ldm` at 0x24cec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24cec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24cec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24c5c | `(unknown callee) 0x48da8` | `r0` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24c68 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0x24ca8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0xee604` | `0xee60c` |
| `bl` at 0x24cb8 | `(unknown callee) 0x19808` | `[sp-32]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0x24ce4 | `(unknown callee) 0x200a0` | `0x6` | `0xee5b8` | `0x6a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x24c2c | `[[sp-28]+8]:1 <= (r1 & 0xff) (unsigned, 32-bit)` | `[[sp-28]+8]:1 > (r1 & 0xff) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24cec | `r0` | `?` |

## `vrmReset`

- `vrmReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24d04 to 0x24d44.
  Established. The inequality `ret(0x37674) != 0x1` takes `0x24d38`, while equality falls through
  (`bne` at 0x24d28). The unnamed `0x24da8` call carries `[sp-20]`, unknown, unknown, `ret(0x37674)`
  (`bl` at 0x24d30). The alternative unnamed `0x24f6c` call carries those same candidates (`bl` at 0x24d3c). Regulator reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24d14 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x24d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24d38 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24d40 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x24d40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24d40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24d40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x24d1c | `(unknown callee) 0x37674` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x24d30 | `(unknown callee) 0x24da8` | `[sp-20]` | `?` | `?` | `ret(0x37674)` |
| `bl` at 0x24d3c | `(unknown callee) 0x24f6c` | `[sp-20]` | `?` | `?` | `ret(0x37674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x24d28 | `ret(0x37674) != 0x1` | `ret(0x37674) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24d40 | `r0` | `?` |

## `vrmGetCfg`

- `vrmGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x24d44 to 0x24da8.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0x24d8c). The byte
  through `[sp-28] +0x8` is read (`ldrb` at 0x24d94). The included return record carries
  `[[sp-28]+8]:1` in `r0` (`ldm` at 0x24da0). Regulator configuration meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x24d54 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x24d58 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x24d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x24d70 | load | 4 | `abs 0x24da4` | `-` |
| `ldr` at 0x24d74 | load | 4 | `abs 0x132068` | `-` |
| `str` at 0x24d7c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x24d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x24d84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x24d88 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0x24d8c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0x24d90 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x24d94 | load | 1 | `[sp-28] +0x8` | `-` |
| `ldm` at 0x24da0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x24da0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x24da0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x24da0 | `r0` | `[[sp-28]+8]:1` |
