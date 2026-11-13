# ARM EEPROM, panel and board helpers in 1.44.0

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
Computed transfers in EEPROM release and status selection remain
unresolved. Each affected entry identifies its computed transfer. Those tables cover known edges only.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/eeprom-monitor-board-facts.txt) retain all tool sections.

## `eepromFreeData`

- `eepromFreeData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc554c to 0xc56e0.
  Established. The word through entry `r0 +0x0` is read (`ldr` at 0xc5564). The computed transfer
  tests `[r0]-1 <= 0x3` using unsigned comparison, with larger values falling through (`ldrls` at 0xc5570). The included unnamed `0x19748` call carries entry `r0`, `0x0`, `0x78`, `[r0]-1` (`bl` at 0xc56d8). EEPROM release effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc555c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5564 | load | 4 | `r0 +0x0` | `-` |
| `ldrls` at 0xc5570 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc56cc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc56dc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc56dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc56dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc56dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc56d8 | `(unknown callee) 0x19748` | `r0` | `0x0` | `0x78` | `[r0]-1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc5570 | `[r0]-1 <= 0x3 (unsigned, 32-bit)` | `[r0]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc56dc | `r0` | `ret(0x19748)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc5570).

## `eepromGetData`

- `eepromGetData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5738 to 0xc5790.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]+12]` (`str` at 0xc5774). The
  included return record gives integer `r0` as `[[sp-28]+4]` (`ldm` at 0xc5788). The absolute word
  at `0x297328` is read (`ldr` at 0xc575c). EEPROM data layout meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5748 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc574c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc5750 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5758 | load | 4 | `abs 0xc578c` | `-` |
| `ldr` at 0xc575c | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc5764 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc5768 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc576c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5770 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xc5774 | store | 4 | `r1 +0x0` | `[[sp-28]+12]` |
| `ldr` at 0xc5778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc577c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0xc5788 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5788 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5788 | `r0` | `[[sp-28]+4]` |

## `eepromIsPresent`

- `eepromIsPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5790 to 0xc580c.
  Established. The signed condition `(r1 & 0xff) < [[sp-28]+4]` takes `0xc57e4`, while greater or
  equal values fall through (`blt` at 0xc57d4). The byte through `((r1 & 0xff) + [[sp-28]+8]) +0x0`
  is read on an included path (`ldrb` at 0xc57f4). Presence flag interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc57a0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc57a8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc57ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc57b4 | load | 4 | `abs 0xc5808` | `-` |
| `ldr` at 0xc57b8 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc57c0 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc57c4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc57c8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc57cc | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0xc57dc | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc57e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc57e8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc57ec | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0xc57f4 | load | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `-` |
| `str` at 0xc57f8 | store | 4 | `sp -0x20` | `[((r1 & 0xff) + [[sp-28]+8])]:1` |
| `ldr` at 0xc57fc | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xc5804 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5804 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5804 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc57d4 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5804 | `r0` | `?` |

## `eepromName`

- `eepromName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5bbc to 0xc5c28.
  Established. The unnamed `0xc5cc8` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xc5be4). A nonzero `[ret(0xc5cc8)+8]` takes `0xc5c14`, while zero falls through
  (`bne` at 0xc5c08). The word at `sp -0x20` receives `0x120068` on the included fallback path
  (`str` at 0xc5c10). Name selection meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5bcc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc5bd4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc5bd8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5bdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5bec | store | 4 | `sp -0x1c` | `ret(0xc5cc8)` |
| `ldr` at 0xc5bf0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5bf4 | load | 4 | `ret(0xc5cc8) +0x8` | `-` |
| `str` at 0xc5bf8 | store | 4 | `sp -0x20` | `[ret(0xc5cc8)+8]` |
| `ldr` at 0xc5bfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5c00 | load | 4 | `ret(0xc5cc8) +0x8` | `-` |
| `ldr` at 0xc5c0c | load | 4 | `abs 0xc5c24` | `-` |
| `str` at 0xc5c10 | store | 4 | `sp -0x20` | `0x120068` |
| `ldr` at 0xc5c14 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xc5c20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5c20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5c20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5be4 | `(unknown callee) 0xc5cc8` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc5c08 | `[ret(0xc5cc8)+8] != 0x0` | `[ret(0xc5cc8)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5c20 | `r0` | `?` |

## `eepromGetCfg`

- `eepromGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc56e0 to 0xc5738.
  Established. The word through entry `r1 +0x0` receives `[[sp-28]]` (`str` at 0xc571c). The
  absolute word at `0x297328` is read (`ldr` at 0xc5704). The included return record gives integer
  `r0` as `[[sp-28]+4]` (`ldm` at 0xc5730). EEPROM configuration representation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc56f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc56f4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc56f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5700 | load | 4 | `abs 0xc5734` | `-` |
| `ldr` at 0xc5704 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc570c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xc5710 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5714 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5718 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xc571c | store | 4 | `r1 +0x0` | `[[sp-28]]` |
| `ldr` at 0xc5720 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5724 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldm` at 0xc5730 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5730 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5730 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5730 | `r0` | `[[sp-28]+4]` |

## `eepromStatus`

- `eepromStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5c28 to 0xc5cc8.
  Established. The computed transfer uses unsigned `r0 <= 0x4`, with larger values falling through
  (`ldrls` at 0xc5c44). An included word assignment at `sp -0x18` writes `0x1200bc` (`str` at 0xc5ca0). The known return record carries that integer value in `r0` (`ldm` at 0xc5cac). Status
  label selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5c38 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc5c3c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc5c44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc5c9c | load | 4 | `abs 0xc5cc4` | `-` |
| `str` at 0xc5ca0 | store | 4 | `sp -0x18` | `0x1200bc` |
| `ldr` at 0xc5ca4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc5cac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5cac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5cac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xc5c44 | `r0 <= 0x4 (unsigned, 32-bit)` | `r0 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5cac | `r0` | `0x1200bc` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc5c44).

## `eepromNotPresent`

- `eepromNotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5140 to 0xc51dc.
  Established. The signed condition `(r1 & 0xff) >= [[sp-28]+4]` takes `0xc51a4`, while smaller
  values fall through (`bge` at 0xc5184). The included byte assignment through `((r1 & 0xff) + [[sp-28]+8]) +0x0` writes zero (`strb` at 0xc519c). The unnamed `0x200a0` call carries `0x6`,
  `0x11ff14`, `0x10c`, `0x0` (`bl` at 0xc51c4). EEPROM removal policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5150 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc5158 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0xc515c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5164 | load | 4 | `abs 0xc51d0` | `-` |
| `ldr` at 0xc5168 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc5170 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0xc5174 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5178 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc517c | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0xc5188 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc518c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5190 | load | 4 | `[sp-28] +0x8` | `-` |
| `strb` at 0xc519c | store | 1 | `((r1 & 0xff) + [[sp-28]+8]) +0x0` | `0x0` |
| `ldr` at 0xc51a4 | load | 4 | `abs 0xc51d4` | `-` |
| `str` at 0xc51a8 | store | 4 | `sp -0x24` | `0x11ffec` |
| `ldrb` at 0xc51ac | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc51b0 | store | 4 | `sp -0x20` | `(r1 & 0xff)` |
| `ldr` at 0xc51b8 | load | 4 | `abs 0xc51d8` | `-` |
| `ldm` at 0xc51cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc51cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc51cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc51c4 | `(unknown callee) 0x200a0` | `0x6` | `0x11ff14` | `0x10c` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc5184 | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc51cc | `r0` | `?` |

## `Vortix_i2cConfig`

- `Vortix_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x27128 to 0x2743c.
  Established. The unnamed `0xc4e80` call carries `0x0`, `0xef434`, `0x48`, unknown (`bl` at 0x2717c). A word at `sp -0x18` receives `0x261d4` (`str` at 0x271a0). A zero `ret(0x548e4)` takes
  `0x27324`, while nonzero falls through (`beq` at 0x272f8). Board bus setup effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x27138 | load | 4 | `abs 0x273e0` | `-` |
| `ldr` at 0x27144 | load | 4 | `abs 0x273e4` | `-` |
| `ldr` at 0x27154 | load | 4 | `abs 0x273e8` | `-` |
| `ldr` at 0x27164 | load | 4 | `abs 0x273ec` | `-` |
| `ldr` at 0x27174 | load | 4 | `abs 0x273f0` | `-` |
| `ldr` at 0x27184 | load | 4 | `abs 0x273f4` | `-` |
| `ldr` at 0x2719c | load | 4 | `abs 0x273f8` | `-` |
| `str` at 0x271a0 | store | 4 | `sp -0x18` | `0x261d4` |
| `ldr` at 0x271a8 | load | 4 | `abs 0x273fc` | `-` |
| `ldr` at 0x271b0 | load | 4 | `abs 0x27400` | `-` |
| `ldr` at 0x271bc | load | 4 | `abs 0x27404` | `-` |
| `ldr` at 0x271cc | load | 4 | `abs 0x27408` | `-` |
| `ldr` at 0x271dc | load | 4 | `abs 0x2740c` | `-` |
| `ldr` at 0x271ec | load | 4 | `abs 0x27410` | `-` |
| `ldr` at 0x271f8 | load | 4 | `abs 0x27414` | `-` |
| `ldr` at 0x27204 | load | 4 | `abs 0x27418` | `-` |
| `str` at 0x27214 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x2721c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x27220 | load | 4 | `abs 0x2741c` | `-` |
| `ldr` at 0x27228 | load | 4 | `abs 0x27420` | `-` |
| `ldr` at 0x27238 | load | 4 | `abs 0x27424` | `-` |
| `ldr` at 0x27244 | load | 4 | `abs 0x27428` | `-` |
| `ldr` at 0x27264 | load | 4 | `abs 0x2742c` | `-` |
| `ldr` at 0x27270 | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x27284 | load | 4 | `abs 0x27434` | `-` |
| `ldr` at 0x27298 | load | 4 | `abs 0x27438` | `-` |
| `ldr` at 0x272ac | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x272c0 | load | 4 | `abs 0x27434` | `-` |
| `ldr` at 0x272d4 | load | 4 | `abs 0x27438` | `-` |
| `ldr` at 0x272fc | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x27310 | load | 4 | `abs 0x27438` | `-` |
| `ldr` at 0x27338 | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x2734c | load | 4 | `abs 0x27438` | `-` |
| `ldr` at 0x27374 | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x27388 | load | 4 | `abs 0x27438` | `-` |
| `ldr` at 0x273b0 | load | 4 | `abs 0x27430` | `-` |
| `ldr` at 0x273c4 | load | 4 | `abs 0x27438` | `-` |
| `ldm` at 0x273dc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x273dc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x273dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2713c | `(unknown callee) 0xbaca4` | `0x26f10` | `r1` | `r2` | `r3` |
| `bl` at 0x2714c | `(unknown callee) 0xc7fdc` | `0x0` | `0xeea5c` | `0x5` | `?` |
| `bl` at 0x2715c | `(unknown callee) 0xcbf9c` | `0x0` | `0xeea94` | `0x6` | `?` |
| `bl` at 0x2716c | `(unknown callee) 0xc9658` | `0x0` | `0xeedac` | `0x36` | `?` |
| `bl` at 0x2717c | `(unknown callee) 0xc4e80` | `0x0` | `0xef434` | `0x48` | `?` |
| `bl` at 0x2718c | `(unknown callee) 0xbf638` | `0x0` | `0xef7f4` | `0x13` | `?` |
| `bl` at 0x27198 | `(unknown callee) 0xbf88c` | `0x0` | `0x40` | `?` | `?` |
| `bl` at 0x271b4 | `(unknown callee) 0x35e44` | `0x0` | `0xefa44` | `0x29` | `0x261a4` |
| `bl` at 0x271c4 | `(unknown callee) 0x24884` | `0x0` | `0xefec0` | `0xe` | `?` |
| `bl` at 0x271d4 | `(unknown callee) 0x24214` | `0x0` | `0xeff78` | `0x13` | `?` |
| `bl` at 0x271e4 | `(unknown callee) 0x3f100` | `0x0` | `0xeff30` | `0x1` | `?` |
| `bl` at 0x271f4 | `(unknown callee) 0x3ee60` | `0x0` | `0xeff54` | `0x3` | `?` |
| `bl` at 0x27200 | `(unknown callee) 0xdceac` | `0x12d440` | `0x6` | `?` | `?` |
| `bl` at 0x2720c | `(unknown callee) 0x2906c` | `0xf00c0` | `0x2` | `?` | `?` |
| `bl` at 0x27230 | `(unknown callee) 0x5590c` | `0xf0070` | `0x4` | `0x26ee8` | `0x0` |
| `bl` at 0x27240 | `(unknown callee) 0x3acb8` | `0x0` | `0xf01f0` | `0x1a` | `?` |
| `bl` at 0x2724c | `(unknown callee) 0x3e690` | `0xf0010` | `0x4` | `?` | `?` |
| `bl` at 0x27254 | `(unknown callee) 0x229c8` | `0x9` | `?` | `?` | `?` |
| `bl` at 0x27258 | `(unknown callee) 0x25fd8` | `ret(0x229c8)` | `?` | `?` | `?` |
| `bl` at 0x2725c | `(unknown callee) 0x3a9c8` | `ret(0x25fd8)` | `?` | `?` | `?` |
| `bl` at 0x27260 | `(unknown callee) 0x260c8` | `ret(0x3a9c8)` | `?` | `?` | `?` |
| `bl` at 0x2726c | `(unknown callee) 0xbba24` | `0x12d470` | `0x6` | `?` | `?` |
| `bl` at 0x27280 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x27294 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x272a8 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x272bc | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x272d0 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x1` | `0x9` | `0x9` |
| `bl` at 0x272e4 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x1` | `0x2` | `0x2` |
| `bl` at 0x272ec | `(unknown callee) 0x548e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2730c | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x2` | `0x2` | `0x2` |
| `bl` at 0x27320 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x2` | `0x3` | `0x6` |
| `bl` at 0x27328 | `(unknown callee) 0x548e4` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x27348 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x3` | `0x3` | `0x3` |
| `bl` at 0x2735c | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x3` | `0x7` | `0xa` |
| `bl` at 0x27364 | `(unknown callee) 0x548e4` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x27384 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x4` | `0x4` | `0x4` |
| `bl` at 0x27398 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x4` | `0xb` | `0xe` |
| `bl` at 0x273a0 | `(unknown callee) 0x548e4` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x273c0 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x5` | `0x5` | `0x5` |
| `bl` at 0x273d4 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x5` | `0xf` | `0x12` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x272f8 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `beq` at 0x27334 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `beq` at 0x27370 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `beq` at 0x273ac | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x273dc | `r0` | `?` |

## `OpusTio_i2cConfig`

- `OpusTio_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2d61c to 0x2d8c8.
  Established. The unnamed `0xc4e80` call carries `0x0`, `0xf270c`, `0x16`, unknown (`bl` at 0x2d678). A zero byte at `0x1320a2` takes `0x2d798`, while nonzero falls through (`beq` at 0x2d754). An included word assignment at `sp -0x14` writes `0x43520000` (`str` at 0x2d780). Board
  configuration roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x2d62c | load | 4 | `abs 0x2d858` | `-` |
| `ldr` at 0x2d638 | load | 4 | `abs 0x2d85c` | `-` |
| `ldr` at 0x2d650 | load | 4 | `abs 0x2d860` | `-` |
| `ldr` at 0x2d660 | load | 4 | `abs 0x2d864` | `-` |
| `ldr` at 0x2d670 | load | 4 | `abs 0x2d868` | `-` |
| `ldr` at 0x2d680 | load | 4 | `abs 0x2d86c` | `-` |
| `ldr` at 0x2d68c | load | 4 | `abs 0x2d870` | `-` |
| `str` at 0x2d690 | store | 4 | `sp -0x1c` | `0x2c4f4` |
| `ldr` at 0x2d698 | load | 4 | `abs 0x2d874` | `-` |
| `ldr` at 0x2d6a0 | load | 4 | `abs 0x2d878` | `-` |
| `ldr` at 0x2d6ac | load | 4 | `abs 0x2d87c` | `-` |
| `ldr` at 0x2d6bc | load | 4 | `abs 0x2d880` | `-` |
| `ldr` at 0x2d6cc | load | 4 | `abs 0x2d884` | `-` |
| `ldr` at 0x2d6dc | load | 4 | `abs 0x2d888` | `-` |
| `ldr` at 0x2d6e8 | load | 4 | `abs 0x2d88c` | `-` |
| `str` at 0x2d6f8 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x2d700 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x2d704 | load | 4 | `abs 0x2d890` | `-` |
| `ldr` at 0x2d70c | load | 4 | `abs 0x2d894` | `-` |
| `ldr` at 0x2d71c | load | 4 | `abs 0x2d898` | `-` |
| `ldr` at 0x2d728 | load | 4 | `abs 0x2d89c` | `-` |
| `ldr` at 0x2d748 | load | 4 | `abs 0x2d8a0` | `-` |
| `ldrb` at 0x2d74c | load | 1 | `abs 0x1320a2` | `-` |
| `ldr` at 0x2d758 | load | 4 | `abs 0x2d8a4` | `-` |
| `ldr` at 0x2d76c | load | 4 | `abs 0x2d8a8` | `-` |
| `str` at 0x2d770 | store | 4 | `sp -0x1c` | `0xf2d18` |
| `str` at 0x2d778 | store | 4 | `sp -0x18` | `0x4` |
| `ldr` at 0x2d77c | load | 4 | `abs 0x2d8ac` | `-` |
| `str` at 0x2d780 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x2d78c | load | 4 | `abs 0x2d8b0` | `-` |
| `ldr` at 0x2d798 | load | 4 | `abs 0x2d8b4` | `-` |
| `ldr` at 0x2d7a4 | load | 4 | `abs 0x2d8b8` | `-` |
| `ldr` at 0x2d7b8 | load | 4 | `abs 0x2d8bc` | `-` |
| `ldr` at 0x2d7cc | load | 4 | `abs 0x2d8b8` | `-` |
| `ldr` at 0x2d7e0 | load | 4 | `abs 0x2d8c0` | `-` |
| `ldrb` at 0x2d7e4 | load | 1 | `abs 0x1320a1` | `-` |
| `ldr` at 0x2d7f0 | load | 4 | `abs 0x2d8c4` | `-` |
| `ldr` at 0x2d804 | load | 4 | `abs 0x2d8bc` | `-` |
| `ldr` at 0x2d818 | load | 4 | `abs 0x2d8a0` | `-` |
| `ldrb` at 0x2d81c | load | 1 | `abs 0x1320a2` | `-` |
| `ldr` at 0x2d828 | load | 4 | `abs 0x2d8c4` | `-` |
| `ldr` at 0x2d83c | load | 4 | `abs 0x2d8bc` | `-` |
| `ldm` at 0x2d854 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x2d854 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x2d854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2d630 | `(unknown callee) 0xbaca4` | `0x2d43c` | `r1` | `r2` | `r3` |
| `bl` at 0x2d640 | `(unknown callee) 0xc7fdc` | `0x0` | `0xf23e8` | `0x2` | `?` |
| `bl` at 0x2d648 | `(unknown callee) 0xbc854` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x2d658 | `(unknown callee) 0xcbf9c` | `0x0` | `0xf2400` | `0x2` | `?` |
| `bl` at 0x2d668 | `(unknown callee) 0xc9658` | `0x0` | `0xf252c` | `0x14` | `?` |
| `bl` at 0x2d678 | `(unknown callee) 0xc4e80` | `0x0` | `0xf270c` | `0x16` | `?` |
| `bl` at 0x2d688 | `(unknown callee) 0xbf638` | `0x0` | `0xf2850` | `0x7` | `?` |
| `bl` at 0x2d6a4 | `(unknown callee) 0x35e44` | `0x0` | `0xf2964` | `0x13` | `0x2c4c0` |
| `bl` at 0x2d6b4 | `(unknown callee) 0x24214` | `0x0` | `0xf2c90` | `0x7` | `?` |
| `bl` at 0x2d6c4 | `(unknown callee) 0x24884` | `0x0` | `0xf2b78` | `0x5` | `?` |
| `bl` at 0x2d6d4 | `(unknown callee) 0x3f100` | `0x0` | `0xf2ba0` | `0x4` | `?` |
| `bl` at 0x2d6e4 | `(unknown callee) 0x3ee60` | `0x0` | `0xf2c24` | `0x9` | `?` |
| `bl` at 0x2d6f0 | `(unknown callee) 0xdceac` | `0x12d5bc` | `0x2` | `?` | `?` |
| `bl` at 0x2d714 | `(unknown callee) 0x5590c` | `0xf2dc8` | `0x1` | `0x2d1e4` | `0x0` |
| `bl` at 0x2d724 | `(unknown callee) 0x3acb8` | `0x0` | `0xf2d5c` | `0x4` | `?` |
| `bl` at 0x2d730 | `(unknown callee) 0x3e690` | `0xf2dbc` | `0x1` | `?` | `?` |
| `bl` at 0x2d738 | `(unknown callee) 0x229c8` | `0x13` | `?` | `?` | `?` |
| `bl` at 0x2d73c | `(unknown callee) 0x2c0e0` | `ret(0x229c8)` | `?` | `?` | `?` |
| `bl` at 0x2d740 | `(unknown callee) 0x3a9c8` | `ret(0x2c0e0)` | `?` | `?` | `?` |
| `bl` at 0x2d744 | `(unknown callee) 0x2c40c` | `ret(0x3a9c8)` | `?` | `?` | `?` |
| `bl` at 0x2d764 | `(unknown callee) 0x4b108` | `0xf2cec` | `0x1` | `0x1` | `[0x1320a2]:1` |
| `bl` at 0x2d768 | `(unknown callee) 0x4b358` | `ret(0x4b108)` | `?` | `?` | `?` |
| `bl` at 0x2d794 | `(unknown callee) 0x4f384` | `0x0` | `0x1` | `0xf2d00` | `0x2` |
| `bl` at 0x2d7a0 | `(unknown callee) 0xbba24` | `0x12d5cc` | `0x4` | `?` | `?` |
| `bl` at 0x2d7b4 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x0` | `0x13` | `0x13` |
| `bl` at 0x2d7c8 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x2d7dc | `(unknown callee) 0xbbbbc` | `0xca960` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x2d800 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x2d814 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x2d838 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x2d84c | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x2d754 | `[0x1320a2]:1 == 0x0` | `[0x1320a2]:1 != 0x0` |
| `beq` at 0x2d7ec | `[0x1320a1]:1 == 0x0` | `[0x1320a1]:1 != 0x0` |
| `beq` at 0x2d824 | `[0x1320a2]:1 == 0x0` | `[0x1320a2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x2d854 | `r0` | `ret(0xbbbbc)` |

## `frontpanelLEDInit`

- `frontpanelLEDInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ab64 to 0x4abf0.
  Established. The word at `0x132480` receives `[sp-24]` (`str` at 0x4abd4). A zero word at
  `0x13247c` takes `0x4abc0`, while nonzero falls through (`beq` at 0x4ab88). The unnamed `0x200a0`
  call carries `0x9`, `0xf82d4`, `0x3a`, `0x0` (`bl` at 0x4abbc). Panel LED initialization effects
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ab74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4ab78 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4ab7c | load | 4 | `abs 0x4abe0` | `-` |
| `ldr` at 0x4ab80 | load | 4 | `abs 0x13247c` | `-` |
| `ldr` at 0x4ab8c | load | 4 | `abs 0x4abe4` | `-` |
| `str` at 0x4ab90 | store | 4 | `sp -0x24` | `0xf82a0` |
| `ldr` at 0x4ab94 | load | 4 | `abs 0x4abe0` | `-` |
| `ldr` at 0x4ab98 | load | 4 | `abs 0x13247c` | `-` |
| `str` at 0x4ab9c | store | 4 | `sp -0x20` | `[0x13247c]` |
| `ldr` at 0x4aba0 | load | 4 | `abs 0x4abe8` | `-` |
| `ldr` at 0x4aba4 | load | 4 | `abs 0x132480` | `-` |
| `str` at 0x4aba8 | store | 4 | `sp -0x1c` | `[0x132480]` |
| `ldr` at 0x4abb0 | load | 4 | `abs 0x4abec` | `-` |
| `ldr` at 0x4abc0 | load | 4 | `abs 0x4abe0` | `-` |
| `ldr` at 0x4abc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4abc8 | store | 4 | `abs 0x13247c` | `?` |
| `ldr` at 0x4abcc | load | 4 | `abs 0x4abe8` | `-` |
| `ldr` at 0x4abd0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4abd4 | store | 4 | `abs 0x132480` | `[sp-24]` |
| `ldm` at 0x4abdc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4abdc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4abdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4abbc | `(unknown callee) 0x200a0` | `0x9` | `0xf82d4` | `0x3a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4ab88 | `[0x13247c] == 0x0` | `[0x13247c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4abdc | `r0` | `?` |

## `frontpanelLEDOn`

- `frontpanelLEDOn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4abf0 to 0x4ac60.
  Established. Entry `r0 != 0x4` takes `0x4ac20`, while equality falls through (`bne` at 0x4ac10).
  The absolute word at `0x132470` receives zero on an included path (`str` at 0x4ac28). The unnamed
  `0x4aef8` call carries `[sp-20]`, unknown, unknown, unknown (`bl` at 0x4ac44). Panel illumination
  policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ac00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4ac04 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4ac08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ac18 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4ac20 | load | 4 | `abs 0x4ac5c` | `-` |
| `str` at 0x4ac28 | store | 4 | `abs 0x132470` | `0x0` |
| `ldr` at 0x4ac2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4ac40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ac4c | store | 4 | `sp -0x1c` | `ret(0x4aef8)` |
| `ldr` at 0x4ac50 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x4ac58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4ac58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4ac58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ac3c | `(unknown callee) 0x4ac60` | `0xff` | `r1` | `0x0` | `[sp-24]` |
| `bl` at 0x4ac44 | `(unknown callee) 0x4aef8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4ac10 | `r0 != 0x4` | `r0 == 0x4` |
| `bne` at 0x4ac34 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4ac58 | `r0` | `?` |

## `frontpanelLEDOff`

- `frontpanelLEDOff`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ac60 to 0x4acb8.
  Established. The word at `0x132470` receives zero on an included path (`str` at 0x4ac94). The
  unnamed `0x4b008` call carries `[sp-20]`, entry `r1`, `0x0`, `0x132470` (`bl` at 0x4ac9c).
  Equality of entry `r0` with `0x4` falls through, while inequality takes `0x4ac8c` (`bne` at 0x4ac7c). Panel shutoff semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4ac70 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x4ac74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4ac84 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x4ac8c | load | 4 | `abs 0x4acb4` | `-` |
| `str` at 0x4ac94 | store | 4 | `abs 0x132470` | `0x0` |
| `ldr` at 0x4ac98 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4aca4 | store | 4 | `sp -0x18` | `ret(0x4b008)` |
| `ldr` at 0x4aca8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x4acb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4acb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4acb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ac9c | `(unknown callee) 0x4b008` | `[sp-20]` | `r1` | `0x0` | `0x132470` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4ac7c | `r0 != 0x4` | `r0 == 0x4` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4acb0 | `r0` | `?` |

## `frontpanelLEDBlink`

- `frontpanelLEDBlink`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4acb8 to 0x4ada4.
  Established. The absolute word at `0x132470` receives `0x1` (`str` at 0x4acf0). The byte at
  `0x132484` receives `([sp-20] & 0xff)` (`strb` at 0x4acfc). Separate included arms write byte
  `0x1` and zero at `0x132475` (`strb` at 0x4ad2c, `strb` at 0x4ad68). Blink configuration
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4acc8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4accc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4acd0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x4acd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4ace8 | load | 4 | `abs 0x4ad8c` | `-` |
| `str` at 0x4acf0 | store | 4 | `abs 0x132470` | `0x1` |
| `ldr` at 0x4acf4 | load | 4 | `abs 0x4ad90` | `-` |
| `ldr` at 0x4acf8 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4acfc | store | 1 | `abs 0x132484` | `([sp-20] & 0xff)` |
| `ldr` at 0x4ad00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4ad10 | load | 4 | `abs 0x4ad94` | `-` |
| `ldr` at 0x4ad14 | load | 4 | `abs 0x4ad98` | `-` |
| `ldr` at 0x4ad18 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4ad1c | load | 1 | `[0x13247c] +0x2` | `-` |
| `strb` at 0x4ad20 | store | 1 | `abs 0x132485` | `([[0x13247c]+2]:1 & 0xff)` |
| `ldr` at 0x4ad24 | load | 4 | `abs 0x4ad9c` | `-` |
| `strb` at 0x4ad2c | store | 1 | `abs 0x132475` | `0x1` |
| `ldr` at 0x4ad30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4ad40 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x4ad44 | load | 4 | `abs 0x4ada0` | `-` |
| `ldr` at 0x4ad48 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x4ad50 | store | 1 | `abs 0x132474` | `?` |
| `str` at 0x4ad58 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4ad60 | load | 4 | `abs 0x4ad9c` | `-` |
| `strb` at 0x4ad68 | store | 1 | `abs 0x132475` | `0x0` |
| `ldr` at 0x4ad6c | load | 4 | `abs 0x4ad94` | `-` |
| `ldr` at 0x4ad70 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x4ad74 | store | 1 | `abs 0x132485` | `([sp-20] & 0xff)` |
| `str` at 0x4ad7c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4ad80 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x4ad88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4ad88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4ad88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ace4 | `(unknown callee) 0x4ac60` | `0xff` | `r1` | `r2` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4acdc | `r2 != 0x0` | `r2 == 0x0` |
| `beq` at 0x4ad0c | `? == 0x0` | `? != 0x0` |
| `bgt` at 0x4ad38 | `[sp-24] > 0x0 (signed, 32-bit)` | `[sp-24] <= 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4ad88 | `r0` | `0x0` |

## `frontpanelLEDBlinkTask`

- `frontpanelLEDBlinkTask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4ada4 to 0x4aef8.
  Established. A nonzero word at `0x132470` takes `0x4adc4`, while zero falls through (`bne` at 0x4adbc). The word at `0x12d708` receives `0x3` on an included path (`str` at 0x4aeb8). The
  unnamed `0x4b008` call carries `[0x132485]:1`, `[0x132486]:1`, `0x132485`, `[0x132485]:1` (`bl` at 0x4ae98). Blink task timing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4adb0 | load | 4 | `abs 0x4aed8` | `-` |
| `ldr` at 0x4adb4 | load | 4 | `abs 0x132470` | `-` |
| `ldr` at 0x4adc4 | load | 4 | `abs 0x4aedc` | `-` |
| `ldr` at 0x4adc8 | load | 4 | `abs 0x12d708` | `-` |
| `ldr` at 0x4add4 | load | 4 | `abs 0x4aee0` | `-` |
| `ldrb` at 0x4add8 | load | 1 | `abs 0x132475` | `-` |
| `ldr` at 0x4ade4 | load | 4 | `abs 0x4aee4` | `-` |
| `ldr` at 0x4ade8 | load | 4 | `abs 0x132478` | `-` |
| `ldr` at 0x4adf4 | load | 4 | `abs 0x4aee8` | `-` |
| `ldr` at 0x4adf8 | load | 4 | `abs 0x13247c` | `-` |
| `ldr` at 0x4adfc | load | 4 | `abs 0x4aeec` | `-` |
| `ldrb` at 0x4ae00 | load | 1 | `[0x13247c] +0x2` | `-` |
| `ldrb` at 0x4ae04 | load | 1 | `abs 0x132485` | `-` |
| `ldr` at 0x4ae10 | load | 4 | `abs 0x4aeec` | `-` |
| `ldr` at 0x4ae18 | load | 4 | `abs 0x4aee8` | `-` |
| `ldr` at 0x4ae1c | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4ae24 | load | 1 | `[0x13247c] +0x6` | `-` |
| `strb` at 0x4ae28 | store | 1 | `abs 0x132485` | `([[0x13247c]+6]:1 & 0xff)` |
| `ldr` at 0x4ae30 | load | 4 | `abs 0x4aeec` | `-` |
| `ldr` at 0x4ae34 | load | 4 | `abs 0x4aee8` | `-` |
| `ldr` at 0x4ae38 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4ae3c | load | 1 | `[0x13247c] +0x2` | `-` |
| `strb` at 0x4ae40 | store | 1 | `abs 0x132485` | `([[0x13247c]+2]:1 & 0xff)` |
| `ldr` at 0x4ae44 | load | 4 | `abs 0x4aee4` | `-` |
| `ldr` at 0x4ae48 | load | 4 | `abs 0x4aef0` | `-` |
| `ldrb` at 0x4ae4c | load | 1 | `abs 0x132474` | `-` |
| `str` at 0x4ae50 | store | 4 | `abs 0x132478` | `[0x132474]:1` |
| `ldr` at 0x4ae58 | load | 4 | `abs 0x4aee4` | `-` |
| `ldr` at 0x4ae5c | load | 4 | `abs 0x4aee4` | `-` |
| `ldr` at 0x4ae60 | load | 4 | `abs 0x132478` | `-` |
| `str` at 0x4ae68 | store | 4 | `abs 0x132478` | `[0x132478]-1` |
| `ldr` at 0x4ae6c | load | 4 | `abs 0x4aef4` | `-` |
| `ldr` at 0x4ae70 | load | 4 | `abs 0x4aeec` | `-` |
| `ldrb` at 0x4ae74 | load | 1 | `abs 0x132486` | `-` |
| `ldrb` at 0x4ae78 | load | 1 | `abs 0x132485` | `-` |
| `ldr` at 0x4ae8c | load | 4 | `abs 0x4aeec` | `-` |
| `ldrb` at 0x4ae90 | load | 1 | `abs 0x132485` | `-` |
| `ldr` at 0x4aea0 | load | 4 | `abs 0x4aeec` | `-` |
| `ldrb` at 0x4aea4 | load | 1 | `abs 0x132485` | `-` |
| `ldr` at 0x4aeb0 | load | 4 | `abs 0x4aedc` | `-` |
| `str` at 0x4aeb8 | store | 4 | `abs 0x12d708` | `0x3` |
| `ldr` at 0x4aec0 | load | 4 | `abs 0x4aedc` | `-` |
| `ldr` at 0x4aec4 | load | 4 | `abs 0x4aedc` | `-` |
| `ldr` at 0x4aec8 | load | 4 | `abs 0x12d708` | `-` |
| `str` at 0x4aed0 | store | 4 | `abs 0x12d708` | `[0x12d708]-1` |
| `ldm` at 0x4aed4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4aed4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4aed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4ae98 | `(unknown callee) 0x4b008` | `[0x132485]:1` | `[0x132486]:1` | `0x132485` | `[0x132485]:1` |
| `bl` at 0x4aeac | `(unknown callee) 0x4aef8` | `[0x132485]:1` | `[0x132486]:1` | `0x132485` | `[0x132485]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4adbc | `[0x132470] != 0x0` | `[0x132470] == 0x0` |
| `bge` at 0x4add0 | `[0x12d708] >= 0x0 (signed, 32-bit)` | `[0x12d708] < 0x0 (signed, 32-bit)` |
| `beq` at 0x4ade0 | `[0x132475]:1 == 0x0` | `[0x132475]:1 != 0x0` |
| `bgt` at 0x4adf0 | `[0x132478] > 0x1 (signed, 32-bit)` | `[0x132478] <= 0x1 (signed, 32-bit)` |
| `bne` at 0x4ae0c | `[[0x13247c]+2]:1 != [0x132485]:1` | `[[0x13247c]+2]:1 == [0x132485]:1` |
| `beq` at 0x4ae88 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4aed4 | `r0` | `?` |

## `fp_led_off_raw`

- `fp_led_off_raw`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4b008 to 0x4b108.
  Established. The unnamed `0xca584` call carries `0x0`, unknown, unknown, `0x0` (`bl` at 0x4b09c);
  its zero result takes `0x4b0bc`, while nonzero falls through (`beq` at 0x4b0b0). The byte at
  `0x132486` receives an unknown value on an included path (`strb` at 0x4b0d8). Raw LED control
  behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4b018 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4b020 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x4b024 | load | 4 | `abs 0x4b0fc` | `-` |
| `ldr` at 0x4b028 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b02c | load | 4 | `abs 0x132480` | `-` |
| `ldr` at 0x4b038 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b040 | load | 4 | `abs 0x4b100` | `-` |
| `ldr` at 0x4b044 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4b04c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b060 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b068 | load | 4 | `abs 0x4b100` | `-` |
| `ldr` at 0x4b06c | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4b074 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4b078 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x4b080 | load | 4 | `abs 0x4b100` | `-` |
| `ldr` at 0x4b084 | load | 4 | `abs 0x13247c` | `-` |
| `ldrb` at 0x4b08c | load | 1 | `(unknown)` | `-` |
| `str` at 0x4b0a4 | store | 4 | `sp -0x1c` | `ret(0xca584)` |
| `ldr` at 0x4b0a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4b0b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x4b0b8 | store | 4 | `sp -0x18` | `ret(0xca584)` |
| `ldr` at 0x4b0bc | load | 4 | `abs 0x4b104` | `-` |
| `ldr` at 0x4b0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4b0c8 | load | 4 | `abs 0x4b104` | `-` |
| `ldrb` at 0x4b0d0 | load | 1 | `abs 0x132486` | `-` |
| `strb` at 0x4b0d8 | store | 1 | `abs 0x132486` | `?` |
| `ldr` at 0x4b0dc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4b0e4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x4b0ec | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x4b0f8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4b0f8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4b0f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4b09c | `(unknown callee) 0xca584` | `0x0` | `?` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x4b034 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x4b05c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x4b0b0 | `ret(0xca584) == 0x0` | `ret(0xca584) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4b0f8 | `r0` | `[sp-24]` |
