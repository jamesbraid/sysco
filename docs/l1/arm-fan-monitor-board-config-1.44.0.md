# ARM fan, EEPROM and board configuration in 1.44.0

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
Computed transfers in EEPROM records, automatic fan paths and monitor testing remain
unresolved. Each affected entry identifies its computed transfer. Those tables cover known edges only.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/fan-monitor-board-config-facts.txt) retain all tool sections.

## `eepromRead`

- `eepromRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc580c to 0xc5a00.
  Established. A halfword at `sp -0x2c` receives `0x20` on an included path (`strh` at 0xc592c). The
  unnamed `0xbb3a8` call carries unknown, `[sp-42]:1`, `[sp-28]`, `[sp-44]:2` (`bl` at 0xc59a0). A
  zero `ret(0xbaf50)` takes `0xc5974`, while nonzero falls through (`beq` at 0xc5964). EEPROM read
  transaction semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc581c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5820 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xc5824 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc582c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xc5830 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xc5834 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xc583c | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xc5840 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5844 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5854 | store | 4 | `sp -0x24` | `ret(0xc5cc8)` |
| `ldrh` at 0xc5858 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xc585c | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xc5864 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc5868 | load | 2 | `ret(0xc5cc8) +0x2` | `-` |
| `ldr` at 0xc5874 | load | 4 | `abs 0xc59f8` | `-` |
| `str` at 0xc5878 | store | 4 | `sp -0x44` | `0x120028` |
| `ldr` at 0xc587c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5880 | load | 4 | `ret(0xc5cc8) +0x8` | `-` |
| `str` at 0xc5884 | store | 4 | `sp -0x40` | `[ret(0xc5cc8)+8]` |
| `ldrh` at 0xc5888 | load | 2 | `sp -0x18` | `-` |
| `str` at 0xc588c | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xc5890 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xc5894 | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xc5898 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc589c | load | 2 | `ret(0xc5cc8) +0x2` | `-` |
| `str` at 0xc58a0 | store | 4 | `sp -0x34` | `[ret(0xc5cc8)+2]:2` |
| `ldr` at 0xc58a8 | load | 4 | `abs 0xc59fc` | `-` |
| `str` at 0xc58bc | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xc58c4 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xc58d0 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc58dc | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xc58e8 | store | 1 | `sp -0x29` | `?` |
| `ldrh` at 0xc58ec | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xc58f0 | store | 1 | `sp -0x2a` | `([sp-24]:2 & 0xff)` |
| `ldrb` at 0xc58f4 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xc58fc | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xc5908 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xc5914 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xc5918 | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldrh` at 0xc591c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xc592c | store | 2 | `sp -0x2c` | `0x20` |
| `ldr` at 0xc5930 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc5934 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5938 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5958 | store | 4 | `sp -0x28` | `ret(0xbaf50)` |
| `ldr` at 0xc595c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5968 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc596c | store | 4 | `sp -0x30` | `ret(0xbaf50)` |
| `ldr` at 0xc5974 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc5978 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xc597c | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xc5988 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xc598c | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xc5998 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc59a8 | store | 4 | `sp -0x28` | `ret(0xbb3a8)` |
| `ldrh` at 0xc59b0 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xc59b4 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xc59bc | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xc59c0 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xc59c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc59cc | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xc59d0 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xc59d4 | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xc59dc | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xc59e4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc59e8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc59ec | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc59f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc59f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc59f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc584c | `(unknown callee) 0xc5cc8` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc58b4 | `(unknown callee) 0x200a0` | `0x6` | `0x11ff14` | `0x1e4` | `0x0` |
| `bl` at 0xc5940 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xc5950 | `(unknown callee) 0xbaf50` | `?` | `?` | `?` | `?` |
| `bl` at 0xc59a0 | `(unknown callee) 0xbb3a8` | `?` | `[sp-42]:1` | `[sp-28]` | `[sp-44]:2` |
| `bl` at 0xc59ac | `(unknown callee) 0xbb094` | `ret(0xbb3a8)` | `?` | `?` | `ret(0xbb3a8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc5870 | `([sp-24]:2 + [sp-30]:2) <= [ret(0xc5cc8)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xc5cc8)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xc58cc | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xc58d8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xc5910 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |
| `bls` at 0xc5924 | `[sp-44]:2 <= 0x20 (unsigned, 32-bit)` | `[sp-44]:2 > 0x20 (unsigned, 32-bit)` |
| `beq` at 0xc5964 | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc59f4 | `r0` | `?` |

## `eepromWrite`

- `eepromWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5a00 to 0xc5bbc.
  Established. The word at `sp -0x44` receives `[sp-44]:2` before an included call (`str` at 0xc5b48). The unnamed `0xc7e14` call carries unknown, unknown, `[sp-42]:1`, `[sp-28]` (`bl` at 0xc5b5c). The halfword at `sp -0x18` receives `(([sp-24]:2 + [sp-44]:2) & 0xffff)` (`strh` at 0xc5b74). EEPROM write protocol remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5a10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5a14 | store | 4 | `sp -0x1c` | `r3` |
| `ldr` at 0xc5a18 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xc5a20 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xc5a24 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `strh` at 0xc5a28 | store | 2 | `sp -0x1e` | `([sp] & 0xffff)` |
| `str` at 0xc5a30 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xc5a34 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5a38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5a48 | store | 4 | `sp -0x24` | `ret(0xc5cc8)` |
| `ldrh` at 0xc5a4c | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xc5a50 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xc5a58 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc5a5c | load | 2 | `ret(0xc5cc8) +0x2` | `-` |
| `ldr` at 0xc5a68 | load | 4 | `abs 0xc5bb0` | `-` |
| `str` at 0xc5a6c | store | 4 | `sp -0x44` | `0x120028` |
| `ldr` at 0xc5a70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc5a74 | load | 4 | `ret(0xc5cc8) +0x8` | `-` |
| `str` at 0xc5a78 | store | 4 | `sp -0x40` | `[ret(0xc5cc8)+8]` |
| `ldrh` at 0xc5a7c | load | 2 | `sp -0x18` | `-` |
| `str` at 0xc5a80 | store | 4 | `sp -0x3c` | `[sp-24]:2` |
| `ldrh` at 0xc5a84 | load | 2 | `sp -0x1e` | `-` |
| `str` at 0xc5a88 | store | 4 | `sp -0x38` | `[sp-30]:2` |
| `ldr` at 0xc5a8c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xc5a90 | load | 2 | `ret(0xc5cc8) +0x2` | `-` |
| `str` at 0xc5a94 | store | 4 | `sp -0x34` | `[ret(0xc5cc8)+2]:2` |
| `ldr` at 0xc5a9c | load | 4 | `abs 0xc5bb4` | `-` |
| `ldr` at 0xc5aa0 | load | 4 | `abs 0xc5bb8` | `-` |
| `str` at 0xc5ab0 | store | 4 | `sp -0x30` | `0x0` |
| `ldrh` at 0xc5ab8 | load | 2 | `sp -0x1e` | `-` |
| `ldr` at 0xc5ac4 | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xc5ad0 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xc5adc | store | 1 | `sp -0x29` | `?` |
| `ldrh` at 0xc5ae0 | load | 2 | `sp -0x18` | `-` |
| `strb` at 0xc5ae4 | store | 1 | `sp -0x2a` | `([sp-24]:2 & 0xff)` |
| `ldrb` at 0xc5ae8 | load | 1 | `sp -0x2a` | `-` |
| `strh` at 0xc5af0 | store | 2 | `sp -0x2c` | `?` |
| `ldrh` at 0xc5afc | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xc5b08 | load | 2 | `sp -0x1e` | `-` |
| `strh` at 0xc5b0c | store | 2 | `sp -0x2c` | `([sp-30]:2 & 0xffff)` |
| `ldr` at 0xc5b10 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc5b14 | load | 1 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xc5b18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc5b2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc5b30 | load | 1 | `[sp-36] +0x1` | `-` |
| `ldrb` at 0xc5b34 | load | 1 | `sp -0x29` | `-` |
| `ldrb` at 0xc5b40 | load | 1 | `sp -0x2a` | `-` |
| `ldrh` at 0xc5b44 | load | 2 | `sp -0x2c` | `-` |
| `str` at 0xc5b48 | store | 4 | `sp -0x44` | `[sp-44]:2` |
| `ldr` at 0xc5b58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5b64 | store | 4 | `sp -0x28` | `ret(0xc7e14)` |
| `ldrh` at 0xc5b68 | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0xc5b6c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xc5b74 | store | 2 | `sp -0x18` | `(([sp-24]:2 + [sp-44]:2) & 0xffff)` |
| `ldrh` at 0xc5b78 | load | 2 | `sp -0x2c` | `-` |
| `ldr` at 0xc5b7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5b84 | store | 4 | `sp -0x1c` | `([sp-28] + [sp-44]:2)` |
| `ldrh` at 0xc5b88 | load | 2 | `sp -0x1e` | `-` |
| `ldrh` at 0xc5b8c | load | 2 | `sp -0x2c` | `-` |
| `strh` at 0xc5b94 | store | 2 | `sp -0x1e` | `?` |
| `ldr` at 0xc5b9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc5ba0 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xc5ba4 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xc5bac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5bac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5bac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5a40 | `(unknown callee) 0xc5cc8` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc5aa8 | `(unknown callee) 0x200a0` | `0x6` | `0x11ff14` | `0x22d` | `0x0` |
| `bl` at 0xc5b20 | `(unknown callee) 0xbac78` | `[sp-20]` | `[[sp-36]]:1` | `?` | `[[sp-36]]:1` |
| `bl` at 0xc5b5c | `(unknown callee) 0xc7e14` | `?` | `?` | `[sp-42]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc5a64 | `([sp-24]:2 + [sp-30]:2) <= [ret(0xc5cc8)+2]:2 (unsigned, 32-bit)` | `([sp-24]:2 + [sp-30]:2) > [ret(0xc5cc8)+2]:2 (unsigned, 32-bit)` |
| `beq` at 0xc5ac0 | `[sp-30]:2 == 0x0` | `[sp-30]:2 != 0x0` |
| `bne` at 0xc5acc | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bls` at 0xc5b04 | `? <= [sp-30]:2 (unsigned, 32-bit)` | `? > [sp-30]:2 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5bac | `r0` | `?` |

## `eepromProbe`

- `eepromProbe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc51dc to 0xc5388.
  Established. A nonzero `ret(0xc5790)` takes `0xc5240`, while zero falls through (`bne` at 0xc5230). The unnamed `0xc580c` call carries `[sp-28]`, `[sp-29]:1`, `0x0`, `sp-48` (`bl` at 0xc5260). An included byte assignment through `([sp-29]:1 + [[sp-36]+8]) +0x0` writes zero (`strb`
  at 0xc5354). EEPROM probing policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc51ec | store | 4 | `sp -0x1c` | `r0` |
| `strb` at 0xc51f4 | store | 1 | `sp -0x1d` | `(r1 & 0xff)` |
| `ldr` at 0xc51f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5200 | load | 4 | `abs 0xc536c` | `-` |
| `ldr` at 0xc5204 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc520c | store | 4 | `sp -0x24` | `?` |
| `str` at 0xc5214 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0xc5218 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc521c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5234 | load | 4 | `abs 0xc5370` | `-` |
| `str` at 0xc5238 | store | 4 | `sp -0x38` | `0xfffffbb4` |
| `ldrb` at 0xc5240 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0xc524c | store | 4 | `sp -0x48` | `0x8` |
| `ldr` at 0xc5250 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc5268 | store | 4 | `sp -0x28` | `ret(0xc580c)` |
| `ldr` at 0xc526c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5278 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5284 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc528c | load | 4 | `abs 0xc5374` | `-` |
| `ldr` at 0xc5290 | load | 4 | `abs 0xc5378` | `-` |
| `str` at 0xc52cc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc52d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc52d8 | load | 4 | `abs 0xc5374` | `-` |
| `ldr` at 0xc52dc | load | 4 | `abs 0xc5378` | `-` |
| `ldr` at 0xc52e8 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xc52f8 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc52fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc530c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc5318 | load | 4 | `abs 0xc537c` | `-` |
| `str` at 0xc531c | store | 4 | `(unknown)` | `0x120008` |
| `str` at 0xc5320 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `str` at 0xc5324 | store | 4 | `(unknown)` | `ret(0xc5bbc)` |
| `str` at 0xc5328 | store | 4 | `(unknown)` | `ret(0x1f8e0)` |
| `ldr` at 0xc5330 | load | 4 | `abs 0xc5380` | `-` |
| `ldr` at 0xc5334 | load | 4 | `abs 0xc5384` | `-` |
| `ldr` at 0xc5340 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xc5344 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0xc5348 | load | 4 | `[sp-36] +0x8` | `-` |
| `strb` at 0xc5354 | store | 1 | `([sp-29]:1 + [[sp-36]+8]) +0x0` | `0x0` |
| `ldr` at 0xc5358 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc535c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xc5360 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0xc5368 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xc5368 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc5368 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5368 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5368 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5224 | `(unknown callee) 0xc5790` | `r0` | `(r1 & 0xff)` | `?` | `(r1 & 0xff)` |
| `bl` at 0xc5260 | `(unknown callee) 0xc580c` | `[sp-28]` | `[sp-29]:1` | `0x0` | `sp-48` |
| `bl` at 0xc5294 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc52a0 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc52e0 | `(unknown callee) 0x48da8` | `[sp-28]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc52f0 | `(unknown callee) 0x19808` | `[sp-52]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc5304 | `(unknown callee) 0xc5bbc` | `[sp-28]` | `[sp-29]:1` | `?` | `[sp-29]:1` |
| `bl` at 0xc5310 | `(unknown callee) 0x1f8e0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0xc533c | `(unknown callee) 0x200a0` | `0x1` | `0x11ff14` | `0x11d` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc5230 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `beq` at 0xc5274 | `ret(0xc580c) == 0x0` | `ret(0xc580c) != 0x0` |
| `beq` at 0xc5280 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5368 | `r0` | `?` |

## `eepromReadData`

- `eepromReadData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc541c to 0xc554c.
  Established. The unnamed `0x19748` call carries `[sp-28]`, `0x0`, `0x78`, `ret(0xc5cc8)` (`bl` at 0xc5460). The word through `[sp-28] +0x0` receives `[[sp-32]+4]:1` (`str` at 0xc5470). An included
  word assignment at `sp -0x24` writes `0xfffffbb1` (`str` at 0xc5534). EEPROM record dispatch
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc542c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc5434 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0xc5438 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc543c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5440 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc5450 | store | 4 | `sp -0x20` | `ret(0xc5cc8)` |
| `ldr` at 0xc5454 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc5468 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc546c | load | 1 | `[sp-32] +0x4` | `-` |
| `str` at 0xc5470 | store | 4 | `[sp-28] +0x0` | `[[sp-32]+4]:1` |
| `ldrb` at 0xc5474 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0xc5480 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc5484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc549c | load | 4 | `abs 0xc5544` | `-` |
| `str` at 0xc54a0 | store | 4 | `sp -0x24` | `0xfffffbb4` |
| `ldr` at 0xc54a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc54ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc54b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc5530 | load | 4 | `abs 0xc5548` | `-` |
| `str` at 0xc5534 | store | 4 | `sp -0x24` | `0xfffffbb1` |
| `ldr` at 0xc5538 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xc5540 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5540 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc5448 | `(unknown callee) 0xc5cc8` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc5460 | `(unknown callee) 0x19748` | `[sp-28]` | `0x0` | `0x78` | `ret(0xc5cc8)` |
| `bl` at 0xc548c | `(unknown callee) 0xc5790` | `[sp-20]` | `[sp-21]:1` | `[sp-28]` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0xc547c | `[sp-21]:1 > 0xf4 (unsigned, 32-bit)` | `[sp-21]:1 <= 0xf4 (unsigned, 32-bit)` |
| `bne` at 0xc5498 | `ret(0xc5790) != 0x0` | `ret(0xc5790) == 0x0` |
| `ldrls` at 0xc54b8 | `[[sp-28]]-1 <= 0x3 (unsigned, 32-bit)` | `[[sp-28]]-1 > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5540 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc54b8).

## `eepromReadAllData`

- `eepromReadAllData`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc5388 to 0xc541c.
  Established. The absolute word at `0x297328` is read (`ldr` at 0xc53a8). The byte at `sp -0x19`
  receives zero before the loop (`strb` at 0xc53b8). The unnamed `0xc541c` call carries `[sp-20]`,
  unknown, unknown, unknown (`bl` at 0xc53fc). The loop updates that byte to `([sp-25]:1+1 & 0xff)`
  (`strb` at 0xc5408). EEPROM traversal meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc5398 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xc539c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc53a4 | load | 4 | `abs 0xc5418` | `-` |
| `ldr` at 0xc53a8 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc53b0 | store | 4 | `sp -0x18` | `?` |
| `strb` at 0xc53b8 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0xc53bc | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc53c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc53c4 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldrb` at 0xc53d0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc53d4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc53d8 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xc53ec | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xc53f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xc5400 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xc5408 | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldm` at 0xc5414 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5414 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5414 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc53fc | `(unknown callee) 0xc541c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xc53cc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5414 | `r0` | `?` |

## `autofanInit`

- `autofanInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30c94 to 0x30e74.
  Established. The absolute word at `0x1320f0` receives `[sp-28]` (`str` at 0x30d08). An included
  assignment to the word at `0x132108` writes `0x1` (`str` at 0x30d28). The unnamed `0xeb174` call
  carries `0x3c`, `[sp-32]`, unknown, unknown (`bl` at 0x30d60). Automatic fan initialization
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30ca4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x30ca8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x30cac | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x30cb0 | load | 4 | `abs 0x30e4c` | `-` |
| `ldr` at 0x30cb4 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x30cc0 | load | 4 | `abs 0x30e50` | `-` |
| `str` at 0x30cc4 | store | 4 | `sp -0x40` | `0xf459c` |
| `ldr` at 0x30cc8 | load | 4 | `abs 0x30e4c` | `-` |
| `ldr` at 0x30ccc | load | 4 | `abs 0x1320ec` | `-` |
| `str` at 0x30cd0 | store | 4 | `sp -0x3c` | `[0x1320ec]` |
| `ldr` at 0x30cd4 | load | 4 | `abs 0x30e54` | `-` |
| `ldr` at 0x30cd8 | load | 4 | `abs 0x1320f0` | `-` |
| `str` at 0x30cdc | store | 4 | `sp -0x38` | `[0x1320f0]` |
| `ldr` at 0x30ce4 | load | 4 | `abs 0x30e58` | `-` |
| `ldr` at 0x30cf4 | load | 4 | `abs 0x30e4c` | `-` |
| `ldr` at 0x30cf8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x30cfc | store | 4 | `abs 0x1320ec` | `?` |
| `ldr` at 0x30d00 | load | 4 | `abs 0x30e54` | `-` |
| `ldr` at 0x30d04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x30d08 | store | 4 | `abs 0x1320f0` | `[sp-28]` |
| `ldr` at 0x30d0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x30d18 | load | 4 | `abs 0x30e5c` | `-` |
| `ldr` at 0x30d20 | load | 4 | `abs 0x30e60` | `-` |
| `str` at 0x30d28 | store | 4 | `abs 0x132108` | `0x1` |
| `ldr` at 0x30d30 | load | 4 | `abs 0x30e60` | `-` |
| `ldr` at 0x30d34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x30d50 | store | 4 | `abs 0x132108` | `?` |
| `ldr` at 0x30d54 | load | 4 | `abs 0x30e64` | `-` |
| `ldr` at 0x30d5c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x30d68 | store | 4 | `abs 0x132104` | `ret(0xeb174)` |
| `strb` at 0x30d84 | store | 1 | `sp -0x2d` | `0x1` |
| `strb` at 0x30da0 | store | 1 | `sp -0x2d` | `0x2` |
| `strb` at 0x30dac | store | 1 | `sp -0x2d` | `0x0` |
| `str` at 0x30db4 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x30db8 | load | 4 | `abs 0x30e54` | `-` |
| `ldr` at 0x30dbc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x30dc0 | load | 4 | `abs 0x1320f0` | `-` |
| `ldr` at 0x30dcc | load | 4 | `abs 0x30e68` | `-` |
| `ldr` at 0x30dd0 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x30ddc | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x30de0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x30de8 | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0x30df0 | load | 4 | `abs 0x30e6c` | `-` |
| `ldr` at 0x30df4 | load | 4 | `abs 0x30e54` | `-` |
| `ldr` at 0x30df8 | load | 4 | `abs 0x1320f0` | `-` |
| `str` at 0x30e0c | store | 4 | `abs 0x1320f4` | `ret(0x1943c)` |
| `ldr` at 0x30e10 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x30e1c | load | 4 | `abs 0x30e70` | `-` |
| `ldr` at 0x30e24 | load | 4 | `abs 0x30e6c` | `-` |
| `ldr` at 0x30e28 | load | 4 | `abs 0x30e54` | `-` |
| `ldr` at 0x30e2c | load | 4 | `abs 0x1320f0` | `-` |
| `ldr` at 0x30e34 | load | 4 | `abs 0x1320f4` | `-` |
| `ldm` at 0x30e48 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x30e48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x30e48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x30e48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30cf0 | `(unknown callee) 0x200a0` | `0x9` | `0xf45cc` | `0x49` | `0x0` |
| `bl` at 0x30d1c | `(unknown callee) 0x1e4ac` | `0xf45e4` | `?` | `0x1320f0` | `[sp-32]` |
| `bl` at 0x30d60 | `(unknown callee) 0xeb174` | `0x3c` | `[sp-32]` | `?` | `?` |
| `bl` at 0x30d70 | `(unknown callee) 0x1d444` | `0x6` | `?` | `?` | `ret(0xeb174)` |
| `bl` at 0x30d8c | `(unknown callee) 0x375e4` | `ret(0x1d444)` | `?` | `?` | `ret(0x1d444)` |
| `bl` at 0x30e04 | `(unknown callee) 0x1943c` | `?` | `?` | `?` | `?` |
| `bl` at 0x30e20 | `(unknown callee) 0x1e4ac` | `0xf4614` | `?` | `?` | `[0x1320f4]` |
| `bl` at 0x30e40 | `(unknown callee) 0x19748` | `[0x1320f4]` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30cbc | `[0x1320ec] == 0x0` | `[0x1320ec] != 0x0` |
| `bgt` at 0x30d14 | `[sp-32] > 0x0 (signed, 32-bit)` | `[sp-32] <= 0x0 (signed, 32-bit)` |
| `bne` at 0x30d7c | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `beq` at 0x30d98 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bge` at 0x30dc8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x30e18 | `[0x1320f4] != 0x0` | `[0x1320f4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x30e48 | `r0` | `ret(0x19748)` |

## `autofanMonitor`

- `autofanMonitor`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x30e74 to 0x3226c.
  Established. A zero byte at `0x132208` takes `0x30ec4`, while nonzero falls through (`beq` at 0x30eb4). An included word assignment at `0x12d680` writes zero (`str` at 0x30f00). The byte at
  `0x2996e9` receives `0x1` on an included path (`strb` at 0x32014). The unnamed `0x404c4` call
  carries `0x0`, unknown, unknown, unknown (`bl` at 0x321bc). Fan adjustment policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x30e84 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x30e88 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x30e8c | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x30e94 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x30e9c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x30ea4 | store | 4 | `sp -0x13c` | `sp-312` |
| `ldr` at 0x30ea8 | load | 4 | `abs 0x318f8` | `-` |
| `ldrb` at 0x30eac | load | 1 | `abs 0x132208` | `-` |
| `str` at 0x30ebc | store | 4 | `sp -0x140` | `0x0` |
| `ldr` at 0x30ec4 | load | 4 | `abs 0x318fc` | `-` |
| `ldr` at 0x30ec8 | load | 4 | `abs 0x132104` | `-` |
| `ldr` at 0x30ed0 | load | 4 | `abs 0x31900` | `-` |
| `ldr` at 0x30ed4 | load | 4 | `abs 0x12d680` | `-` |
| `ldr` at 0x30ee0 | load | 4 | `abs 0x31900` | `-` |
| `ldr` at 0x30ee4 | load | 4 | `abs 0x31900` | `-` |
| `ldr` at 0x30ee8 | load | 4 | `abs 0x12d680` | `-` |
| `str` at 0x30ef0 | store | 4 | `abs 0x12d680` | `[0x12d680]+1` |
| `ldr` at 0x30ef8 | load | 4 | `abs 0x31900` | `-` |
| `str` at 0x30f00 | store | 4 | `abs 0x12d680` | `0x0` |
| `str` at 0x30f08 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x30f0c | load | 4 | `abs 0x31904` | `-` |
| `ldr` at 0x30f10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30f14 | load | 4 | `abs 0x1320f0` | `-` |
| `str` at 0x30f24 | store | 4 | `sp -0x48` | `0x2` |
| `ldr` at 0x30f28 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30f3c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x30f40 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x30f48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30f54 | load | 4 | `abs 0x31908` | `-` |
| `str` at 0x30f58 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x30f60 | store | 4 | `sp -0x38` | `0x0` |
| `str` at 0x30f68 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x30f6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30f80 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x30f84 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x30f8c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x30f90 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x30f9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x30fb0 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x30fb4 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x30fbc | load | 4 | `sp -0x44` | `-` |
| `ldrb` at 0x30fc4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x30fc8 | store | 1 | `sp -0x4d` | `?` |
| `ldrb` at 0x30fcc | load | 1 | `sp -0x4d` | `-` |
| `ldrb` at 0x30fe8 | load | 1 | `sp -0x4d` | `-` |
| `ldr` at 0x30ffc | load | 4 | `sp -0x34` | `-` |
| `ldc` at 0x31004 | load | unknown | `sp -0x60` | `-` |
| `stc` at 0x3100c | store | unknown | `sp -0x60` | `?` |
| `ldr` at 0x31010 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x31014 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x31020 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x31024 | store | 4 | `sp -0x38` | `[sp-52]` |
| `ldr` at 0x31028 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x31030 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x31038 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3104c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31050 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31058 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31068 | load | 4 | `sp -0x38` | `-` |
| `stc` at 0x31070 | store | unknown | `sp -0x5c` | `?` |
| `ldr` at 0x31078 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3108c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31090 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31098 | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x310a0 | load | unknown | `sp -0x60` | `-` |
| `stc` at 0x310a8 | store | unknown | `sp -0x5c` | `?` |
| `ldr` at 0x310ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x310b4 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x310b8 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x310c0 | load | 4 | `abs 0x31900` | `-` |
| `ldr` at 0x310c4 | load | 4 | `abs 0x12d680` | `-` |
| `ldr` at 0x310d8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x310dc | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x310e0 | load | 4 | `abs 0x31908` | `-` |
| `str` at 0x310e4 | store | 4 | `sp -0x60` | `0x0` |
| `str` at 0x310ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x310f0 | load | 4 | `abs 0x318fc` | `-` |
| `ldr` at 0x310f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x310f8 | load | 4 | `abs 0x132104` | `-` |
| `ldr` at 0x31104 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3110c | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31110 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31118 | load | 4 | `sp -0x24` | `-` |
| `ldc` at 0x3112c | load | unknown | `sp -0x60` | `-` |
| `ldc` at 0x31130 | load | unknown | `(unknown)` | `-` |
| `stc` at 0x31138 | store | unknown | `sp -0x60` | `?` |
| `ldr` at 0x3113c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x31144 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x3114c | load | 4 | `abs 0x318fc` | `-` |
| `ldr` at 0x31150 | load | 4 | `abs 0x132104` | `-` |
| `ldc` at 0x31158 | load | unknown | `sp -0x60` | `-` |
| `stc` at 0x31160 | store | unknown | `sp -0x5c` | `?` |
| `ldr` at 0x31164 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31178 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x3117c | load | 4 | `abs 0x1320ec` | `-` |
| `ldc` at 0x31184 | load | unknown | `sp -0x5c` | `-` |
| `stc` at 0x3118c | store | unknown | `(unknown)` | `?` |
| `str` at 0x31198 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x3119c | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x311a0 | load | 4 | `abs 0x3190c` | `-` |
| `ldr` at 0x311a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x311ac | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x311bc | load | 4 | `sp -0x13c` | `-` |
| `str` at 0x311c4 | store | 4 | `sp -0x13c` | `([sp-316] + ret(0x19538))` |
| `ldr` at 0x311c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x311dc | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x311e0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x311e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x311fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31210 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31214 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x3121c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3122c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31240 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31244 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x3124c | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x31250 | load | 4 | `abs 0x31910` | `-` |
| `ldr` at 0x31254 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x3125c | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x3126c | load | 4 | `sp -0x13c` | `-` |
| `str` at 0x31274 | store | 4 | `sp -0x13c` | `([sp-316] + ret(0x19538))` |
| `ldr` at 0x31278 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3128c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31290 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31298 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x312ac | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x312b0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x312b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x312cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x312e0 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x312e4 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x312ec | load | 1 | `sp -0x50` | `-` |
| `ldc` at 0x312f4 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x312fc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31310 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31314 | load | 4 | `abs 0x1320ec` | `-` |
| `ldc` at 0x3131c | load | unknown | `(unknown)` | `-` |
| `str` at 0x31328 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x3132c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31338 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x3134c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31360 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `ldr` at 0x31364 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31370 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31384 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31398 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x3139c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x313a4 | load | 1 | `(unknown)` | `-` |
| `str` at 0x313a8 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x313b0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x313c4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x313c8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x313d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x313e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x313f4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x313f8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31400 | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x31408 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x31418 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3142c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31430 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31438 | load | 1 | `(unknown)` | `-` |
| `str` at 0x3143c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x31444 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31458 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x3145c | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x31464 | load | 4 | `(unknown)` | `-` |
| `ldc` at 0x3146c | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x3147c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31490 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `ldr` at 0x31498 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x314ac | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x314b0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldc` at 0x314b8 | load | unknown | `(unknown)` | `-` |
| `ldc` at 0x314bc | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x314c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x314d8 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x314dc | load | 4 | `abs 0x1320ec` | `-` |
| `ldc` at 0x314e4 | load | unknown | `(unknown)` | `-` |
| `str` at 0x314f0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x314f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31500 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31514 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31528 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `ldr` at 0x31530 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31544 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31548 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31550 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31564 | store | 4 | `sp -0x144` | `?` |
| `str` at 0x31570 | store | 4 | `sp -0x144` | `0x1` |
| `ldr` at 0x31574 | load | 4 | `sp -0x144` | `-` |
| `str` at 0x31578 | store | 4 | `sp -0x30` | `[sp-324]` |
| `ldr` at 0x3157c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31584 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31588 | load | 4 | `abs 0x1320f4` | `-` |
| `ldc` at 0x31590 | load | unknown | `sp -0x5c` | `-` |
| `ldc` at 0x31594 | load | unknown | `(unknown)` | `-` |
| `stc` at 0x3159c | store | unknown | `sp -0x64` | `?` |
| `ldc` at 0x315a0 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x315a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x315bc | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x315c0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x315c8 | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x315d0 | load | unknown | `abs 0x318f0` | `-` |
| `ldr` at 0x315e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x315ec | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x315f0 | load | 4 | `abs 0x1320f4` | `-` |
| `ldrb` at 0x315f8 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31604 | load | 1 | `sp -0x4f` | `-` |
| `str` at 0x31608 | store | 4 | `sp -0x4c` | `[sp-79]:1` |
| `ldc` at 0x31610 | load | unknown | `sp -0x64` | `-` |
| `ldr` at 0x31620 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31628 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x3162c | load | 4 | `abs 0x1320f4` | `-` |
| `ldc` at 0x31634 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x31644 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3164c | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31650 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31658 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31660 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31664 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x3166c | load | 4 | `(unknown)` | `-` |
| `str` at 0x31674 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31678 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3168c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31690 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31698 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x316a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x316ac | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x316b0 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x316b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x316cc | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x316d0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x316d8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x316dc | load | 4 | `(unknown)` | `-` |
| `strb` at 0x316fc | store | 1 | `sp -0x4e` | `?` |
| `strb` at 0x31708 | store | 1 | `sp -0x4e` | `0x0` |
| `ldrb` at 0x3170c | load | 1 | `sp -0x4f` | `-` |
| `ldr` at 0x31710 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31724 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31728 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31730 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31734 | load | 1 | `sp -0x4e` | `-` |
| `ldr` at 0x3173c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31748 | store | 4 | `sp -0x4c` | `?` |
| `ldrb` at 0x31750 | load | 1 | `sp -0x4f` | `-` |
| `str` at 0x31754 | store | 4 | `sp -0x4c` | `[sp-79]:1` |
| `ldr` at 0x3175c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31764 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31768 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31770 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31778 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x3177c | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31784 | load | 4 | `(unknown)` | `-` |
| `str` at 0x3178c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x317a4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x317a8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x317b0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x317bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x317c4 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x317c8 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x317d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x317e4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x317e8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x317f0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x317f4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31814 | store | 1 | `sp -0x4e` | `?` |
| `strb` at 0x31820 | store | 1 | `sp -0x4e` | `0x0` |
| `ldrb` at 0x31824 | load | 1 | `sp -0x4f` | `-` |
| `ldr` at 0x31828 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3183c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31840 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31848 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x3184c | load | 1 | `sp -0x4e` | `-` |
| `ldr` at 0x31854 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31860 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x31868 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3187c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31880 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31888 | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x31890 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x318a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x318a8 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x318ac | load | 4 | `abs 0x1320f4` | `-` |
| `strb` at 0x318b8 | store | 1 | `(unknown)` | `0x1` |
| `ldc` at 0x318bc | load | unknown | `sp -0x64` | `-` |
| `ldr` at 0x318cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x318d4 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x318d8 | load | 4 | `abs 0x1320f4` | `-` |
| `ldc` at 0x318e0 | load | unknown | `(unknown)` | `-` |
| `ldr` at 0x31954 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3195c | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31960 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31968 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31970 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31974 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x3197c | load | 4 | `(unknown)` | `-` |
| `str` at 0x31984 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31988 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3199c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x319a0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x319a8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x319b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x319bc | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x319c0 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x319c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x319dc | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x319e0 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x319e8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x319ec | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31a0c | store | 1 | `sp -0x4e` | `?` |
| `strb` at 0x31a18 | store | 1 | `sp -0x4e` | `0x0` |
| `ldrb` at 0x31a1c | load | 1 | `sp -0x4f` | `-` |
| `ldr` at 0x31a20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31a34 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31a38 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31a40 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x31a44 | load | 1 | `sp -0x4e` | `-` |
| `ldr` at 0x31a4c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31a58 | store | 4 | `sp -0x4c` | `?` |
| `ldrb` at 0x31a60 | load | 1 | `sp -0x4f` | `-` |
| `str` at 0x31a64 | store | 4 | `sp -0x4c` | `[sp-79]:1` |
| `ldr` at 0x31a6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31a74 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31a78 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31a80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31a88 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31a8c | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31a94 | load | 4 | `(unknown)` | `-` |
| `str` at 0x31a9c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x31aa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31ab4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31ab8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31ac0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31acc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31ad4 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31ad8 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31ae0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31af4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31af8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31b00 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31b04 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x31b24 | store | 1 | `sp -0x4e` | `?` |
| `strb` at 0x31b30 | store | 1 | `sp -0x4e` | `0x0` |
| `ldc` at 0x31b34 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x31b3c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31b50 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31b54 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31b5c | load | 1 | `(unknown)` | `-` |
| `stc` at 0x31b68 | store | unknown | `sp -0x58` | `?` |
| `ldc` at 0x31b6c | load | unknown | `sp -0x58` | `-` |
| `ldc` at 0x31b70 | load | unknown | `sp -0x58` | `-` |
| `ldc` at 0x31b78 | load | unknown | `abs 0x31950` | `-` |
| `stc` at 0x31b80 | store | unknown | `sp -0x68` | `?` |
| `ldc` at 0x31b84 | load | unknown | `sp -0x68` | `-` |
| `ldr` at 0x31b94 | load | 4 | `abs 0x31914` | `-` |
| `str` at 0x31b98 | store | 4 | `sp -0x68` | `0x3f800000` |
| `ldr` at 0x31b9c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31bb0 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31bb4 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31bbc | load | 1 | `(unknown)` | `-` |
| `ldc` at 0x31bc4 | load | unknown | `sp -0x68` | `-` |
| `strb` at 0x31bd0 | store | 1 | `sp -0x51` | `?` |
| `ldrb` at 0x31bd4 | load | 1 | `sp -0x4f` | `-` |
| `ldrb` at 0x31bd8 | load | 1 | `sp -0x51` | `-` |
| `ldrb` at 0x31bdc | load | 1 | `sp -0x4e` | `-` |
| `ldr` at 0x31be4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x31bf0 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x31bf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31c00 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31c14 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31c28 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31c2c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31c34 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31c38 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x31c40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31c4c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31c60 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31c74 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `str` at 0x31c94 | store | 4 | `sp -0x3c` | `0x5` |
| `str` at 0x31c9c | store | 4 | `sp -0x40` | `0xa` |
| `str` at 0x31ca8 | store | 4 | `sp -0x3c` | `0x9` |
| `str` at 0x31cb0 | store | 4 | `sp -0x40` | `0xd` |
| `ldr` at 0x31cb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31cc8 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31ccc | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31cd4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31cd8 | load | 4 | `sp -0x40` | `-` |
| `ldc` at 0x31ce4 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x31cf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31d00 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x31d18 | store | 4 | `sp -0x48` | `0x10` |
| `ldr` at 0x31d20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31d34 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31d38 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31d40 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31d44 | load | 4 | `sp -0x3c` | `-` |
| `ldc` at 0x31d50 | load | unknown | `sp -0x5c` | `-` |
| `ldr` at 0x31d60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31d6c | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x31d84 | store | 4 | `sp -0x48` | `0x8` |
| `ldr` at 0x31d8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31d94 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x31db8 | store | 4 | `sp -0x48` | `0x4` |
| `ldrb` at 0x31dc0 | load | 1 | `sp -0x4f` | `-` |
| `str` at 0x31dc4 | store | 4 | `sp -0x4c` | `[sp-79]:1` |
| `ldr` at 0x31dc8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31dd0 | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31dd4 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31ddc | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x31de0 | store | 4 | `(unknown)` | `[sp-92]` |
| `ldr` at 0x31de4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31dec | load | 4 | `abs 0x31918` | `-` |
| `ldr` at 0x31df0 | load | 4 | `abs 0x1320f4` | `-` |
| `ldr` at 0x31df8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x31dfc | store | 4 | `(unknown)` | `[sp-100]` |
| `ldr` at 0x31e00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31e0c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31e20 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31e34 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31e38 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31e40 | load | 1 | `(unknown)` | `-` |
| `str` at 0x31e44 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x31e4c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31e58 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31e6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31e80 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `ldr` at 0x31e84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x31e88 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x31e94 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x31e98 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x31e9c | store | 4 | `[sp-24] +0x0` | `[sp-44]` |
| `ldr` at 0x31ea0 | load | 4 | `sp -0x1c` | `-` |
| `ldc` at 0x31ea4 | load | unknown | `sp -0x5c` | `-` |
| `str` at 0x31eac | store | 4 | `[sp-28] +0x0` | `?` |
| `ldr` at 0x31eb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x31eb4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31ebc | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x31ed4 | store | 4 | `[sp-32] +0x0` | `?` |
| `ldr` at 0x31ed8 | load | 4 | `sp -0x48` | `-` |
| `str` at 0x31edc | store | 4 | `sp -0x28` | `[sp-72]` |
| `ldr` at 0x31ee0 | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x31ee4 | load | 4 | `abs 0x3191c` | `-` |
| `ldr` at 0x31ee8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x31ef0 | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x31f00 | load | 4 | `sp -0x13c` | `-` |
| `str` at 0x31f08 | store | 4 | `sp -0x13c` | `([sp-316] + ret(0x19538))` |
| `ldr` at 0x31f0c | load | 4 | `abs 0x31930` | `-` |
| `ldr` at 0x31f10 | load | 4 | `abs 0x12d67c` | `-` |
| `str` at 0x31f20 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x31f24 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31f38 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31f3c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x31f44 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x31f48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x31f54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31f68 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31f6c | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x31f78 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x31f84 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x31f88 | store | 1 | `sp -0x69` | `?` |
| `ldrb` at 0x31f8c | load | 1 | `sp -0x69` | `-` |
| `ldr` at 0x31fa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x31fb4 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x31fb8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x31fc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x31fc4 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x31fd0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x31fdc | load | 4 | `abs 0x3192c` | `-` |
| `ldr` at 0x31fe0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x31fe8 | load | 1 | `[sp-44] +0x13210c` | `-` |
| `ldr` at 0x31ff4 | load | 4 | `abs 0x3192c` | `-` |
| `ldr` at 0x31ff8 | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x32004 | store | 1 | `[sp-44] +0x13210c` | `0x1` |
| `ldr` at 0x3200c | load | 4 | `abs 0x31934` | `-` |
| `strb` at 0x32014 | store | 1 | `abs 0x2996e9` | `0x1` |
| `ldrb` at 0x32018 | load | 1 | `sp -0x69` | `-` |
| `ldr` at 0x3202c | load | 4 | `abs 0x31920` | `-` |
| `str` at 0x32030 | store | 4 | `(unknown)` | `0xf464c` |
| `ldr` at 0x32034 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x32038 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0x3203c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32040 | store | 4 | `(unknown)` | `[sp-36]` |
| `str` at 0x32044 | store | 4 | `(unknown)` | `ret(0x40cd8)` |
| `ldr` at 0x32048 | load | 4 | `sp -0x70` | `-` |
| `str` at 0x3204c | store | 4 | `(unknown)` | `[sp-112]` |
| `ldr` at 0x32054 | load | 4 | `abs 0x31924` | `-` |
| `ldr` at 0x32058 | load | 4 | `abs 0x31928` | `-` |
| `ldr` at 0x32068 | load | 4 | `abs 0x3192c` | `-` |
| `ldr` at 0x3206c | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x32078 | store | 1 | `[sp-44] +0x13210c` | `0x0` |
| `ldr` at 0x3207c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32084 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x3208c | load | 4 | `abs 0x31930` | `-` |
| `ldr` at 0x32090 | load | 4 | `abs 0x31930` | `-` |
| `ldr` at 0x32094 | load | 4 | `abs 0x12d67c` | `-` |
| `str` at 0x3209c | store | 4 | `abs 0x12d67c` | `[0x12d67c]-1` |
| `ldr` at 0x320a0 | load | 4 | `abs 0x31934` | `-` |
| `ldrb` at 0x320a4 | load | 1 | `abs 0x2996e9` | `-` |
| `ldr` at 0x320b0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x320c0 | store | 4 | `sp -0x28` | `0x20` |
| `str` at 0x320c8 | store | 4 | `sp -0x48` | `0x20` |
| `ldr` at 0x320cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x320e0 | store | 4 | `sp -0x4c` | `ret(0x32ec0)` |
| `ldrb` at 0x320e4 | load | 1 | `sp -0x4f` | `-` |
| `ldr` at 0x320e8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x320f4 | load | 4 | `abs 0x31948` | `-` |
| `ldr` at 0x320f8 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x32108 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3211c | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x32120 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32128 | load | 1 | `sp -0x4f` | `-` |
| `ldr` at 0x3212c | load | 4 | `abs 0x31938` | `-` |
| `ldr` at 0x32130 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32134 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x32140 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x32144 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x32158 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x3215c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32164 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32168 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32174 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x32188 | load | 4 | `abs 0x3193c` | `-` |
| `ldr` at 0x3218c | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x32198 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x321a4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x321a8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x321c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x321c8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x321d0 | load | 4 | `abs 0x31940` | `-` |
| `ldr` at 0x321d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x321dc | load | 4 | `sp -0x48` | `-` |
| `strb` at 0x321e0 | store | 1 | `[sp-44] +0x1320f8` | `([sp-72] & 0xff)` |
| `ldr` at 0x321e4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x321ec | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x321f4 | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x321f8 | load | 4 | `abs 0x31944` | `-` |
| `ldr` at 0x32200 | load | 4 | `sp -0x13c` | `-` |
| `ldr` at 0x32210 | load | 4 | `sp -0x13c` | `-` |
| `str` at 0x32218 | store | 4 | `sp -0x13c` | `([sp-316] + ret(0x19538))` |
| `ldr` at 0x3221c | load | 4 | `abs 0x31948` | `-` |
| `ldr` at 0x32220 | load | 4 | `abs 0x132f14` | `-` |
| `ldr` at 0x32234 | load | 4 | `abs 0x3194c` | `-` |
| `ldr` at 0x32240 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32250 | store | 4 | `sp -0x140` | `0x2` |
| `ldr` at 0x32258 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x3225c | store | 4 | `sp -0x140` | `[sp-40]` |
| `ldr` at 0x32260 | load | 4 | `sp -0x140` | `-` |
| `ldm` at 0x32268 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x32268 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x32268 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x32268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x30fd8 | `(unknown callee) 0x243dc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x30ff8 | `(unknown callee) 0x2457c` | `0x0` | `[sp-77]:1` | `sp-52` | `[sp-77]:1` |
| `bl` at 0x311a8 | `(unknown callee) 0x197a8` | `[sp-316]` | `0xf4630` | `?` | `?` |
| `bl` at 0x311b0 | `(unknown callee) 0x19538` | `[sp-316]` | `?` | `?` | `?` |
| `bl` at 0x311f8 | `(unknown callee) 0x4096c` | `0x0` | `?` | `sp-79` | `?` |
| `bl` at 0x31258 | `(unknown callee) 0x197a8` | `[sp-316]` | `0xf463c` | `?` | `?` |
| `bl` at 0x31260 | `(unknown callee) 0x19538` | `[sp-316]` | `?` | `?` | `?` |
| `bl` at 0x312c8 | `(unknown callee) 0x4096c` | `0x0` | `?` | `sp-80` | `?` |
| `bl` at 0x3133c | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31358 | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31374 | `(unknown callee) 0x33010` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31488 | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31504 | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31520 | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x316e8 | `(unknown callee) 0xeb368` | `?` | `?` | `?` | `?` |
| `bl` at 0x31800 | `(unknown callee) 0xeb368` | `?` | `?` | `?` | `?` |
| `bl` at 0x319f8 | `(unknown callee) 0xeb368` | `?` | `?` | `?` | `?` |
| `bl` at 0x31b10 | `(unknown callee) 0xeb368` | `?` | `?` | `?` | `?` |
| `bl` at 0x31c04 | `(unknown callee) 0x33010` | `?` | `?` | `?` | `?` |
| `bl` at 0x31c50 | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31c6c | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31c78 | `(unknown callee) 0x488cc` | `?` | `?` | `?` | `?` |
| `bl` at 0x31d04 | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31d70 | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31da4 | `(unknown callee) 0x331d8` | `?` | `?` | `?` | `?` |
| `bl` at 0x31e10 | `(unknown callee) 0x33010` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31e5c | `(unknown callee) 0x32f4c` | `?` | `[sp-76]` | `?` | `?` |
| `bl` at 0x31e78 | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x31ecc | `(unknown callee) 0x330f8` | `?` | `?` | `?` | `?` |
| `bl` at 0x31eec | `(unknown callee) 0x197a8` | `[sp-316]` | `0xf4644` | `[sp-76]` | `?` |
| `bl` at 0x31ef4 | `(unknown callee) 0x19538` | `[sp-316]` | `?` | `?` | `?` |
| `bl` at 0x31f9c | `(unknown callee) 0x40afc` | `0x0` | `[sp-105]:1` | `sp-112` | `[sp-105]:1` |
| `bl` at 0x32024 | `(unknown callee) 0x40cd8` | `0x0` | `[sp-105]:1` | `0x2996e9` | `[sp-105]:1` |
| `bl` at 0x32060 | `(unknown callee) 0x200a0` | `0x1` | `0xf45cc` | `0x189` | `0x0` |
| `bl` at 0x320d8 | `(unknown callee) 0x32ec0` | `?` | `?` | `?` | `?` |
| `bl` at 0x32138 | `(unknown callee) 0x1e958` | `0xf4674` | `?` | `[sp-79]:1` | `[sp-76]` |
| `bl` at 0x321bc | `(unknown callee) 0x404c4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x321fc | `(unknown callee) 0x197a8` | `[sp-316]` | `0xf4684` | `?` | `[0x1320f0]` |
| `bl` at 0x32204 | `(unknown callee) 0x19538` | `[sp-316]` | `?` | `?` | `?` |
| `bl` at 0x3223c | `(unknown callee) 0x221f0` | `0xf4688` | `sp-312` | `ret(0x19538)` | `sp-312` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30eb4 | `[0x132208]:1 == 0x0` | `[0x132208]:1 != 0x0` |
| `ble` at 0x30edc | `[0x132104]-1 <= [0x12d680] (signed, 32-bit)` | `[0x132104]-1 > [0x12d680] (signed, 32-bit)` |
| `bge` at 0x30f1c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x30f50 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x30f98 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x30fe4 | `ret(0x243dc) == 0x0` | `ret(0x243dc) != 0x0` |
| `ble` at 0x3101c | `[sp-52] <= [sp-56] (signed, 32-bit)` | `[sp-52] > [sp-56] (signed, 32-bit)` |
| `beq` at 0x31064 | `? == 0x0` | `? != 0x0` |
| `bge` at 0x31100 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x31228 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31348 | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `beq` at 0x31380 | `ret(0x33010) == 0x0` | `ret(0x33010) != 0x0` |
| `beq` at 0x313dc | `? == 0x0` | `? != 0x0` |
| `bge` at 0x31410 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bmi` at 0x31474 | `N == 1` | `not (N == 1)` |
| `beq` at 0x31510 | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `beq` at 0x3155c | `? == 0x0` | `? != 0x0` |
| `bmi` at 0x315dc | `N == 1` | `not (N == 1)` |
| `bne` at 0x31600 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x31618 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bgt` at 0x3163c | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x316a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x317b8 | `? == 0x0` | `? != 0x0` |
| `bmi` at 0x31898 | `N == 1` | `not (N == 1)` |
| `bmi` at 0x318c4 | `N == 1` | `not (N == 1)` |
| `bmi` at 0x318e8 | `N == 1` | `not (N == 1)` |
| `beq` at 0x319b0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x31ac8 | `? == 0x0` | `? != 0x0` |
| `bls` at 0x31b8c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x31c10 | `ret(0x33010) == 0x0` | `ret(0x33010) != 0x0` |
| `beq` at 0x31c5c | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `bhi` at 0x31c8c | `? > 0x1 (unsigned, 32-bit)` | `? <= 0x1 (unsigned, 32-bit)` |
| `bmi` at 0x31cec | `N == 1` | `not (N == 1)` |
| `beq` at 0x31d10 | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `bmi` at 0x31d58 | `N == 1` | `not (N == 1)` |
| `beq` at 0x31d7c | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `beq` at 0x31db0 | `ret(0x331d8) == 0x0` | `ret(0x331d8) != 0x0` |
| `beq` at 0x31e1c | `ret(0x33010) == 0x0` | `ret(0x33010) != 0x0` |
| `beq` at 0x31e68 | `ret(0x32f4c) == 0x0` | `ret(0x32f4c) != 0x0` |
| `bge` at 0x31e90 | `[sp-40] >= [sp-72] (signed, 32-bit)` | `[sp-40] < [sp-72] (signed, 32-bit)` |
| `bgt` at 0x31f18 | `[0x12d67c] > 0x0 (signed, 32-bit)` | `[0x12d67c] <= 0x0 (signed, 32-bit)` |
| `ble` at 0x31f50 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0x31fcc | `? <= [sp-112] (unsigned, 32-bit)` | `? > [sp-112] (unsigned, 32-bit)` |
| `bne` at 0x31fd8 | `[sp-112] != 0x0` | `[sp-112] == 0x0` |
| `bne` at 0x31ff0 | `[[sp-44]+1253644]:1 != 0x0` | `[[sp-44]+1253644]:1 == 0x0` |
| `beq` at 0x320ac | `[0x2996e9]:1 == 0x0` | `[0x2996e9]:1 != 0x0` |
| `bgt` at 0x320b8 | `[sp-40] > 0x3 (signed, 32-bit)` | `[sp-40] <= 0x3 (signed, 32-bit)` |
| `beq` at 0x320f0 | `[sp-79]:1 == [sp-76]` | `[sp-79]:1 != [sp-76]` |
| `beq` at 0x32104 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x32170 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x3222c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x32248 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x32268 | `r0` | `?` |

## `autofanEnv`

- `autofanEnv`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3226c to 0x32744.
  Established. A zero `ret(0x375e4)` takes `0x32624`, while nonzero falls through (`beq` at 0x32298). Included assignments write byte zero at `0x2996e9` and word `0x5` at `0x12d67c` (`strb`
  at 0x3262c, `str` at 0x32638). The unnamed `0x30e74` call carries `sp-32`, `sp-36`, `sp-40`,
  `sp-32` (`bl` at 0x322c8). Environmental fan policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x3227c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x32280 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x32284 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x32288 | store | 4 | `sp -0x80` | `r1` |
| `str` at 0x322d0 | store | 4 | `sp -0x1c` | `ret(0x30e74)` |
| `ldr` at 0x322d4 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0x322e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32608 | load | 4 | `abs 0x3272c` | `-` |
| `ldr` at 0x3260c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x32614 | load | 4 | `abs 0x326fc` | `-` |
| `ldr` at 0x32618 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x3261c | store | 4 | `abs 0x12d684` | `[sp-28]` |
| `ldr` at 0x32624 | load | 4 | `abs 0x32730` | `-` |
| `strb` at 0x3262c | store | 1 | `abs 0x2996e9` | `0x0` |
| `ldr` at 0x32630 | load | 4 | `abs 0x32734` | `-` |
| `str` at 0x32638 | store | 4 | `abs 0x12d67c` | `0x5` |
| `ldr` at 0x3263c | load | 4 | `abs 0x326fc` | `-` |
| `str` at 0x32644 | store | 4 | `abs 0x12d684` | `0x2` |
| `str` at 0x32660 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x32664 | load | 4 | `abs 0x32738` | `-` |
| `ldr` at 0x32668 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x3266c | load | 4 | `abs 0x1320f0` | `-` |
| `ldr` at 0x32678 | load | 4 | `abs 0x3273c` | `-` |
| `ldr` at 0x3267c | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x32688 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x3268c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x32694 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x326a0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x326a4 | load | 4 | `abs 0x32738` | `-` |
| `ldr` at 0x326a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x326ac | load | 4 | `abs 0x1320f0` | `-` |
| `ldr` at 0x326b8 | load | 4 | `abs 0x3273c` | `-` |
| `ldr` at 0x326bc | load | 4 | `sp -0x2c` | `-` |
| `strb` at 0x326c8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x326cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x326d4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x326dc | load | 4 | `abs 0x32740` | `-` |
| `ldr` at 0x326e0 | load | 4 | `abs 0x132108` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3228c | `(unknown callee) 0x375e4` | `?` | `?` | `?` | `?` |
| `bl` at 0x322a0 | `(unknown callee) 0x1d444` | `0x6` | `?` | `?` | `ret(0x375e4)` |
| `bl` at 0x322c8 | `(unknown callee) 0x30e74` | `sp-32` | `sp-36` | `sp-40` | `sp-32` |
| `bl` at 0x32610 | `(unknown callee) 0x1e958` | `0xf4760` | `ret(0x30e74)` | `?` | `ret(0x30e74)-2` |
| `bl` at 0x3264c | `(unknown callee) 0x1d444` | `0x6` | `?` | `0x12d684` | `0x2` |
| `bl` at 0x326e4 | `(unknown callee) 0x32744` | `[0x132108]` | `?` | `?` | `0x132108` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x32298 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `beq` at 0x322ac | `ret(0x1d444) == 0x0` | `ret(0x1d444) != 0x0` |
| `ldrls` at 0x322e0 | `ret(0x30e74)-2 <= 0x1e (unsigned, 32-bit)` | `ret(0x30e74)-2 > 0x1e (unsigned, 32-bit)` |
| `bne` at 0x32658 | `ret(0x1d444) != 0x0` | `ret(0x1d444) == 0x0` |
| `bge` at 0x32674 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0x326b4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x322e0).

## `autofanEnvQuery`

- `autofanEnvQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3276c to 0x32e54.
  Established. The word at `sp -0x54` receives `sp-80` (`str` at 0x32788). The unnamed `0x197a8`
  call at `0x32928` carries `sp-80`, `0xf48b4`, `0xf48b8`, `sp-80` (`bl` at 0x32928). Coprocessor
  stores at unknown targets have unknown widths and values (`stc` at 0x32d9c, `stc` at 0x32da0).
  Environmental query formatting remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `stc` at 0x32774 | store | unknown | `sp -0x1c` | `?` |
| `str` at 0x32780 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x32788 | store | 4 | `sp -0x54` | `sp-80` |
| `str` at 0x32790 | store | 4 | `sp -0x64` | `sp-96` |
| `ldr` at 0x32794 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x327a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x327ac | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x327b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x327b4 | load | 4 | `abs 0x32e0c` | `-` |
| `ldr` at 0x327bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x327cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x327d4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `str` at 0x327dc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x327e0 | load | 4 | `abs 0x32e10` | `-` |
| `ldr` at 0x327e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x327e8 | load | 4 | `abs 0x1320f0` | `-` |
| `ldr` at 0x327f4 | load | 4 | `abs 0x32e14` | `-` |
| `ldr` at 0x327f8 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32800 | load | 1 | `(unknown)` | `-` |
| `ldrls` at 0x32808 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x328e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x328f8 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x328fc | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32904 | load | 1 | `(unknown)` | `-` |
| `str` at 0x32918 | store | 4 | `sp -0x54` | `sp-80` |
| `ldr` at 0x3291c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x32920 | load | 4 | `abs 0x32e38` | `-` |
| `ldr` at 0x32924 | load | 4 | `abs 0x32e3c` | `-` |
| `ldr` at 0x32934 | load | 4 | `abs 0x32e38` | `-` |
| `ldr` at 0x32938 | load | 4 | `abs 0x32e3c` | `-` |
| `ldr` at 0x32948 | load | 4 | `abs 0x32e38` | `-` |
| `ldr` at 0x3294c | load | 4 | `abs 0x32e3c` | `-` |
| `str` at 0x3295c | store | 4 | `sp -0x54` | `sp-80` |
| `str` at 0x32964 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x32968 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x3297c | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32980 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32988 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x3298c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x32998 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x329a4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x329a8 | load | 4 | `abs 0x32e40` | `-` |
| `ldr` at 0x329b0 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x329c0 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x329c8 | store | 4 | `sp -0x54` | `([sp-84] + ret(0x19538))` |
| `ldr` at 0x329cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x329e0 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x329e4 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x329ec | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x329f4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x329f8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x329fc | load | 4 | `abs 0x32e44` | `-` |
| `ldr` at 0x32a08 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x32a18 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x32a20 | store | 4 | `sp -0x54` | `([sp-84] + ret(0x19538))` |
| `ldr` at 0x32a24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32a2c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x32a34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32a48 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32a4c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32a54 | load | 1 | `(unknown)` | `-` |
| `str` at 0x32a58 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x32a60 | load | 4 | `sp -0x38` | `-` |
| `ldc` at 0x32a68 | load | unknown | `abs 0x32dfc` | `-` |
| `ldc` at 0x32a70 | load | unknown | `abs 0x32e04` | `-` |
| `ldr` at 0x32a80 | load | 4 | `abs 0x32e48` | `-` |
| `ldr` at 0x32a84 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x32a90 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x32a98 | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x32aa0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x32aa4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32ab8 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32abc | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32ac4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32ac8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x32ad4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32ae8 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32aec | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x32af4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x32afc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32b10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x32b14 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x32b1c | store | 4 | `sp -0x2c` | `([sp-44] + [sp-48])` |
| `ldr` at 0x32b20 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x32b24 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x32b30 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x32b34 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0x32b38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32b40 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x32b48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32b5c | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32b60 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32b68 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32b78 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x32b7c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x32b84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32b98 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32b9c | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32ba4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32ba8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x32bb8 | store | 4 | `sp -0x3c` | `ret(0xeb174)` |
| `ldr` at 0x32bc0 | load | 4 | `sp -0x3c` | `-` |
| `ldc` at 0x32bc8 | load | unknown | `abs 0x32dfc` | `-` |
| `ldc` at 0x32bd0 | load | unknown | `abs 0x32e04` | `-` |
| `ldr` at 0x32be0 | load | 4 | `abs 0x32e48` | `-` |
| `ldr` at 0x32be4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x32bf0 | store | 4 | `sp -0x64` | `sp-96` |
| `str` at 0x32bf8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x32bfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32c10 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32c14 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32c1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32c20 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x32c2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x32c38 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x32c3c | load | 4 | `abs 0x32e40` | `-` |
| `ldr` at 0x32c44 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x32c54 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x32c5c | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0x32c60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32c74 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32c78 | load | 4 | `abs 0x1320ec` | `-` |
| `ldr` at 0x32c84 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x32c90 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32c94 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x32c98 | load | 4 | `abs 0x32e44` | `-` |
| `ldr` at 0x32ca4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x32cb4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x32cbc | store | 4 | `sp -0x64` | `([sp-100] + ret(0x19538))` |
| `ldr` at 0x32cc0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x32cc8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x32cd0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32ce4 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32ce8 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32cf0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32d04 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x32d0c | load | 1 | `sp -0x3d` | `-` |
| `ldr` at 0x32d20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32d28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32d3c | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32d40 | load | 4 | `abs 0x1320ec` | `-` |
| `ldrb` at 0x32d48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x32d5c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x32d70 | load | 4 | `abs 0x32e34` | `-` |
| `ldr` at 0x32d74 | load | 4 | `abs 0x1320ec` | `-` |
| `str` at 0x32d80 | store | 4 | `(unknown)` | `sp-80` |
| `str` at 0x32d88 | store | 4 | `(unknown)` | `sp-124` |
| `str` at 0x32d90 | store | 4 | `(unknown)` | `sp-112` |
| `str` at 0x32d98 | store | 4 | `(unknown)` | `sp-96` |
| `stc` at 0x32d9c | store | unknown | `(unknown)` | `?` |
| `stc` at 0x32da0 | store | unknown | `(unknown)` | `?` |
| `ldr` at 0x32da4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32da8 | load | 4 | `abs 0x32e4c` | `-` |
| `ldr` at 0x32dac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x32db0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x32db8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32dc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x32dd0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x19538))` |
| `ldr` at 0x32dd4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x32ddc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x32de4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x32de8 | load | 4 | `abs 0x32e50` | `-` |
| `ldc` at 0x32df0 | load | unknown | `sp -0x1c` | `-` |
| `ldm` at 0x32df8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x32df8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x32df8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x32798 | `(unknown callee) 0x19538` | `r0` | `r1` | `r2` | `sp-96` |
| `bl` at 0x327b8 | `(unknown callee) 0x197a8` | `([sp-32] + ret(0x19538))` | `0xf4784` | `ret(0x19538)` | `([sp-32] + ret(0x19538))` |
| `bl` at 0x327c0 | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x32928 | `(unknown callee) 0x197a8` | `sp-80` | `0xf48b4` | `0xf48b8` | `sp-80` |
| `bl` at 0x3293c | `(unknown callee) 0x197a8` | `sp-124` | `0xf48b4` | `0xf48b8` | `sp-124` |
| `bl` at 0x32950 | `(unknown callee) 0x197a8` | `sp-112` | `0xf48b4` | `0xf48b8` | `sp-112` |
| `bl` at 0x329ac | `(unknown callee) 0x197a8` | `?` | `0xf48bc` | `?` | `?` |
| `bl` at 0x329b4 | `(unknown callee) 0x19538` | `[sp-84]` | `?` | `?` | `?` |
| `bl` at 0x32a04 | `(unknown callee) 0x197a8` | `?` | `0xf48c0` | `?` | `?` |
| `bl` at 0x32a0c | `(unknown callee) 0x19538` | `[sp-84]` | `?` | `?` | `?` |
| `bl` at 0x32a88 | `(unknown callee) 0x197a8` | `sp-124` | `0xf48c4` | `[sp-56]` | `?` |
| `bl` at 0x32b0c | `(unknown callee) 0x2457c` | `0x0` | `?` | `sp-48` | `?` |
| `bl` at 0x32bb0 | `(unknown callee) 0xeb174` | `?` | `?` | `?` | `?` |
| `bl` at 0x32be8 | `(unknown callee) 0x197a8` | `sp-112` | `0xf48c4` | `[sp-60]` | `?` |
| `bl` at 0x32c40 | `(unknown callee) 0x197a8` | `?` | `0xf48bc` | `?` | `?` |
| `bl` at 0x32c48 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x32ca0 | `(unknown callee) 0x197a8` | `?` | `0xf48c0` | `?` | `?` |
| `bl` at 0x32ca8 | `(unknown callee) 0x19538` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x32d00 | `(unknown callee) 0x4096c` | `0x0` | `?` | `sp-61` | `?` |
| `bl` at 0x32d18 | `(unknown callee) 0x330f8` | `?` | `[sp-61]:1` | `[sp-61]:1` | `?` |
| `bl` at 0x32d54 | `(unknown callee) 0x330f8` | `?` | `?` | `?` | `?` |
| `bl` at 0x32db4 | `(unknown callee) 0x197a8` | `[sp-32]` | `0xf48d0` | `?` | `[sp-68]` |
| `bl` at 0x32dbc | `(unknown callee) 0x19538` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x32dec | `(unknown callee) 0x19808` | `([sp-32] + ret(0x19538))` | `0xf4684` | `?` | `[0x1320f0]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x327f0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ldrls` at 0x32808 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x32910 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x32994 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x329a0 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x32ad0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x32b2c | `[sp-48] <= [sp-52] (signed, 32-bit)` | `[sp-48] > [sp-52] (signed, 32-bit)` |
| `beq` at 0x32b74 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x32c28 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x32c34 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x32df8 | `r0` | `ret(0x19808)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x32808).

## `monitorTest`

- `monitorTest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc16ec to 0xc183c.
  Established. The unnamed `0xc183c` call carries entry `r0`, `(r1 & 0xff)`, entry `r2`, `(r1 & 0xff)` (`bl` at 0xc1714). A nonzero byte through `ret(0xc1970) +0x0` takes `0xc1754`, while zero
  falls through (`bne` at 0xc1744). A word at `sp -0x24` receives `0xfffffc11` on an included path
  (`str` at 0xc174c). Monitor test dispatch remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc16fc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc1704 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xc1708 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc170c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc171c | store | 4 | `sp -0x1c` | `ret(0xc183c)` |
| `ldrb` at 0xc1720 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc1724 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc1734 | store | 4 | `sp -0x20` | `ret(0xc1970)` |
| `ldr` at 0xc1738 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xc173c | load | 1 | `ret(0xc1970) +0x0` | `-` |
| `ldr` at 0xc1748 | load | 4 | `abs 0xc1828` | `-` |
| `str` at 0xc174c | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xc1754 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc1758 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xc1764 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc1814 | load | 4 | `abs 0xc1838` | `-` |
| `str` at 0xc1818 | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0xc181c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xc1824 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc1824 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc1824 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc1714 | `(unknown callee) 0xc183c` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xc172c | `(unknown callee) 0xc1970` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc1744 | `[ret(0xc1970)]:1 != 0x0` | `[ret(0xc1970)]:1 == 0x0` |
| `ldrls` at 0xc1764 | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc1824 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc1764).

## `monitorVSC055Int`

- `monitorVSC055Int`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4c84 to 0xc4e80.
  Established. The absolute word at `0x297320` is read (`ldr` at 0xc4cc0). The unnamed `0x7294c`
  call carries `[sp-20]`, `[sp-33]:1`, `[sp-34]:1`, `[sp-33]:1` (`bl` at 0xc4e54). The included
  return record gives integer `r0` as `0x1` (`ldm` at 0xc4e64). Interrupt interpretation remains
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4c94 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xc4ca0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xc4ca8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xc4cb0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0xc4cb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4cbc | load | 4 | `abs 0xc4e68` | `-` |
| `ldr` at 0xc4cc0 | load | 4 | `abs 0x297320` | `-` |
| `str` at 0xc4cc8 | store | 4 | `sp -0x1c` | `?` |
| `strb` at 0xc4cd0 | store | 1 | `sp -0x21` | `?` |
| `strb` at 0xc4cd8 | store | 1 | `sp -0x22` | `?` |
| `str` at 0xc4ce0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc4ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4ce8 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0xc4cec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc4cf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4cfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc4d10 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xc4d18 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc4d1c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xc4d28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4d2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc4d40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0xc4d48 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0xc4d4c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xc4d58 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xc4d5c | store | 1 | `sp -0x21` | `?` |
| `ldr` at 0xc4d60 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc4d68 | store | 4 | `sp -0x20` | `?` |
| `ldrb` at 0xc4d70 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0xc4d7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d84 | load | 4 | `abs 0xc4e6c` | `-` |
| `ldr` at 0xc4d88 | load | 4 | `abs 0xc4e70` | `-` |
| `str` at 0xc4dc4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc4dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4dd0 | load | 4 | `abs 0xc4e6c` | `-` |
| `ldr` at 0xc4dd4 | load | 4 | `abs 0xc4e70` | `-` |
| `ldr` at 0xc4de0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc4df0 | load | 4 | `abs 0xc4e74` | `-` |
| `str` at 0xc4df4 | store | 4 | `(unknown)` | `0x11fdd4` |
| `str` at 0xc4df8 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldrb` at 0xc4dfc | load | 1 | `sp -0x15` | `-` |
| `str` at 0xc4e00 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0xc4e04 | load | 1 | `sp -0x16` | `-` |
| `str` at 0xc4e08 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0xc4e0c | load | 1 | `sp -0x17` | `-` |
| `str` at 0xc4e10 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0xc4e18 | load | 4 | `abs 0xc4e78` | `-` |
| `ldr` at 0xc4e1c | load | 4 | `abs 0xc4e7c` | `-` |
| `ldrb` at 0xc4e28 | load | 1 | `sp -0x17` | `-` |
| `strb` at 0xc4e40 | store | 1 | `sp -0x22` | `?` |
| `ldrb` at 0xc4e44 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0xc4e48 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0xc4e4c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc4e64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc4e64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc4e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4d8c | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xc4d98 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc4dd8 | `(unknown callee) 0x48da8` | `[sp-20]` | `0x0` | `0x11fb44` | `0x11fb4c` |
| `bl` at 0xc4de8 | `(unknown callee) 0x19808` | `[sp-40]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc4e24 | `(unknown callee) 0x200a0` | `0x1` | `0x11faec` | `0xbc9` | `0x0` |
| `bl` at 0xc4e54 | `(unknown callee) 0x7294c` | `[sp-20]` | `[sp-33]:1` | `[sp-34]:1` | `[sp-33]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xc4cf4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xc4d24 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc4d54 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc4d78 | `[sp-33]:1 != 0xff` | `[sp-33]:1 == 0xff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc4e64 | `r0` | `0x1` |

## `eepromInit`

- `eepromInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4e80 to 0xc5140.
  Established. The absolute word at `0x29732c` receives `0x1` (`str` at 0xc4eec). The word at
  `0x297328` receives `ret(0x196d0)` (`str` at 0xc4f14). An included word assignment through
  `[sp-44] +0xc` writes `ret(0x196d0)` (`str` at 0xc5068). EEPROM allocation roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc4e90 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc4e94 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc4e98 | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0xc4e9c | load | 4 | `abs 0xc511c` | `-` |
| `ldr` at 0xc4ea0 | load | 4 | `abs 0x297328` | `-` |
| `ldr` at 0xc4eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4eb8 | load | 4 | `abs 0xc5120` | `-` |
| `str` at 0xc4ebc | store | 4 | `sp -0x3c` | `0x11fef0` |
| `ldr` at 0xc4ec4 | load | 4 | `abs 0xc5124` | `-` |
| `ldr` at 0xc4ed4 | load | 4 | `abs 0xc5128` | `-` |
| `ldr` at 0xc4ed8 | load | 4 | `abs 0x29732c` | `-` |
| `ldr` at 0xc4ee4 | load | 4 | `abs 0xc5128` | `-` |
| `str` at 0xc4eec | store | 4 | `abs 0x29732c` | `0x1` |
| `ldr` at 0xc4ef0 | load | 4 | `abs 0xc5128` | `-` |
| `ldr` at 0xc4ef4 | load | 4 | `abs 0x29732c` | `-` |
| `str` at 0xc4efc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc4f00 | load | 4 | `abs 0xc511c` | `-` |
| `ldr` at 0xc4f04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4f14 | store | 4 | `abs 0x297328` | `ret(0x196d0)` |
| `ldr` at 0xc4f18 | load | 4 | `abs 0x297328` | `-` |
| `ldr` at 0xc4f24 | load | 4 | `abs 0xc512c` | `-` |
| `str` at 0xc4f28 | store | 4 | `sp -0x3c` | `0x11ff30` |
| `ldr` at 0xc4f2c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc4f30 | store | 4 | `sp -0x38` | `[sp-40]` |
| `ldr` at 0xc4f38 | load | 4 | `abs 0xc5124` | `-` |
| `ldr` at 0xc4f48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4f50 | load | 4 | `abs 0xc511c` | `-` |
| `ldr` at 0xc4f54 | load | 4 | `abs 0x297328` | `-` |
| `str` at 0xc4f5c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc4f60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4f64 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc4f68 | store | 4 | `[sp-44] +0x0` | `?` |
| `ldr` at 0xc4f6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4f70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc4f74 | store | 4 | `[sp-44] +0x4` | `[sp-32]` |
| `ldr` at 0xc4f78 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4f7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc4f80 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xc4f8c | store | 4 | `[sp-44] +0x8` | `ret(0x1943c)` |
| `ldr` at 0xc4f90 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xc4f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4fa4 | load | 4 | `abs 0xc5130` | `-` |
| `ldr` at 0xc4fa8 | load | 4 | `abs 0xc5134` | `-` |
| `str` at 0xc4fe4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc4fe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4ff0 | load | 4 | `abs 0xc5130` | `-` |
| `ldr` at 0xc4ff4 | load | 4 | `abs 0xc5134` | `-` |
| `ldr` at 0xc5000 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc5010 | load | 4 | `abs 0xc5138` | `-` |
| `str` at 0xc5014 | store | 4 | `(unknown)` | `0x11ff74` |
| `str` at 0xc5018 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xc501c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc5020 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xc5028 | load | 4 | `abs 0xc5124` | `-` |
| `ldr` at 0xc5038 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc503c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5040 | load | 4 | `[sp-44] +0x8` | `-` |
| `ldr` at 0xc5048 | load | 4 | `[sp-44] +0x4` | `-` |
| `ldr` at 0xc5050 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5054 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc5058 | load | 4 | `[sp-44] +0x4` | `-` |
| `str` at 0xc5068 | store | 4 | `[sp-44] +0xc` | `ret(0x196d0)` |
| `ldr` at 0xc506c | load | 4 | `[sp-44] +0xc` | `-` |
| `ldr` at 0xc5078 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc5080 | load | 4 | `abs 0xc5130` | `-` |
| `ldr` at 0xc5084 | load | 4 | `abs 0xc5134` | `-` |
| `str` at 0xc50c0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc50c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc50cc | load | 4 | `abs 0xc5130` | `-` |
| `ldr` at 0xc50d0 | load | 4 | `abs 0xc5134` | `-` |
| `ldr` at 0xc50dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xc50ec | load | 4 | `abs 0xc513c` | `-` |
| `str` at 0xc50f0 | store | 4 | `(unknown)` | `0x11ffb0` |
| `str` at 0xc50f4 | store | 4 | `(unknown)` | `ret(0x19808)` |
| `ldr` at 0xc50f8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc50fc | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xc5104 | load | 4 | `abs 0xc5124` | `-` |
| `ldm` at 0xc5118 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xc5118 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xc5118 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xc5118 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4ed0 | `(unknown callee) 0x200a0` | `0x9` | `0x11ff14` | `0xe3` | `0x0` |
| `bl` at 0xc4f0c | `(unknown callee) 0x196d0` | `[sp-40]` | `0x1` | `0x29732c` | `?` |
| `bl` at 0xc4f44 | `(unknown callee) 0x200a0` | `0x9` | `0x11ff14` | `0xed` | `0x0` |
| `bl` at 0xc4f84 | `(unknown callee) 0x1943c` | `[[sp-44]+4]` | `?` | `[sp-44]` | `[sp-44]` |
| `bl` at 0xc4fac | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc4fb8 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc4ff8 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc5008 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc5034 | `(unknown callee) 0x200a0` | `0x9` | `0x11ff14` | `0xf9` | `0x0` |
| `bl` at 0xc504c | `(unknown callee) 0x19748` | `[[sp-44]+8]` | `0x1` | `[[sp-44]+4]` | `[sp-44]` |
| `bl` at 0xc5060 | `(unknown callee) 0x196d0` | `[[sp-44]+4]` | `0x78` | `?` | `[sp-44]` |
| `bl` at 0xc5088 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc5094 | `(unknown callee) 0x19538` | `ret(0x48da8)` | `?` | `?` | `ret(0x48da8)` |
| `bl` at 0xc50d4 | `(unknown callee) 0x48da8` | `[sp-24]` | `0x0` | `0x11ff68` | `0x11ff70` |
| `bl` at 0xc50e4 | `(unknown callee) 0x19808` | `[sp-48]` | `ret(0x48da8)` | `?` | `ret(0x48da8)` |
| `bl` at 0xc5110 | `(unknown callee) 0x200a0` | `0x9` | `0x11ff14` | `0xfe` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4ea8 | `[0x297328] == 0x0` | `[0x297328] != 0x0` |
| `bne` at 0xc4eb4 | `r0 != 0x0` | `r0 == 0x0` |
| `bne` at 0xc4ee0 | `[0x29732c] != 0x0` | `[0x29732c] == 0x0` |
| `bne` at 0xc4f20 | `[0x297328] != 0x0` | `[0x297328] == 0x0` |
| `bne` at 0xc4f98 | `[[sp-44]+8] != 0x0` | `[[sp-44]+8] == 0x0` |
| `bne` at 0xc5074 | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xc5118 | `r0` | `?` |

## `Dorado_i2cConfig`

- `Dorado_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x307a4 to 0x30a2c.
  Established. The unnamed `0xbf638` call carries `0x0`, `0xf3ee4`, `0x7`, unknown (`bl` at 0x30810). The unnamed `0x30c94` call carries `0xf4288`, `0x5`, `0x14`, unknown (`bl` at 0x3087c).
  A zero byte at `0x1320e9` takes `0x309a0`, while nonzero falls through (`beq` at 0x30974). Board
  configuration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x307b4 | load | 4 | `abs 0x309bc` | `-` |
| `ldr` at 0x307c0 | load | 4 | `abs 0x309c0` | `-` |
| `ldr` at 0x307d8 | load | 4 | `abs 0x309c4` | `-` |
| `ldr` at 0x307e8 | load | 4 | `abs 0x309c8` | `-` |
| `ldr` at 0x307f8 | load | 4 | `abs 0x309cc` | `-` |
| `ldr` at 0x30808 | load | 4 | `abs 0x309d0` | `-` |
| `ldr` at 0x30814 | load | 4 | `abs 0x309d4` | `-` |
| `str` at 0x30818 | store | 4 | `sp -0x1c` | `0x2ef68` |
| `ldr` at 0x30820 | load | 4 | `abs 0x309d8` | `-` |
| `ldr` at 0x30828 | load | 4 | `abs 0x309dc` | `-` |
| `ldr` at 0x30834 | load | 4 | `abs 0x309e0` | `-` |
| `ldr` at 0x30844 | load | 4 | `abs 0x309e4` | `-` |
| `ldr` at 0x30854 | load | 4 | `abs 0x309e8` | `-` |
| `ldr` at 0x30864 | load | 4 | `abs 0x309ec` | `-` |
| `ldr` at 0x30870 | load | 4 | `abs 0x309f0` | `-` |
| `ldr` at 0x30880 | load | 4 | `abs 0x309f4` | `-` |
| `str` at 0x30890 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x30898 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x3089c | load | 4 | `abs 0x309f8` | `-` |
| `ldr` at 0x308a4 | load | 4 | `abs 0x309fc` | `-` |
| `ldr` at 0x308b4 | load | 4 | `abs 0x30a00` | `-` |
| `ldr` at 0x308d4 | load | 4 | `abs 0x30a04` | `-` |
| `ldr` at 0x308e0 | load | 4 | `abs 0x30a08` | `-` |
| `ldr` at 0x308f4 | load | 4 | `abs 0x30a0c` | `-` |
| `str` at 0x308f8 | store | 4 | `sp -0x1c` | `0xf4440` |
| `str` at 0x30900 | store | 4 | `sp -0x18` | `0x7` |
| `ldr` at 0x30904 | load | 4 | `abs 0x30a10` | `-` |
| `str` at 0x30908 | store | 4 | `sp -0x14` | `0x43520000` |
| `ldr` at 0x30914 | load | 4 | `abs 0x30a14` | `-` |
| `ldr` at 0x30920 | load | 4 | `abs 0x30a18` | `-` |
| `ldr` at 0x3092c | load | 4 | `abs 0x30a1c` | `-` |
| `ldr` at 0x30940 | load | 4 | `abs 0x30a20` | `-` |
| `ldr` at 0x30954 | load | 4 | `abs 0x30a24` | `-` |
| `ldr` at 0x30968 | load | 4 | `abs 0x30a28` | `-` |
| `ldrb` at 0x3096c | load | 1 | `abs 0x1320e9` | `-` |
| `ldr` at 0x30978 | load | 4 | `abs 0x30a1c` | `-` |
| `ldr` at 0x3098c | load | 4 | `abs 0x30a20` | `-` |
| `ldr` at 0x309a0 | load | 4 | `abs 0x30a24` | `-` |
| `ldm` at 0x309b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x309b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x309b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x307b8 | `(unknown callee) 0xbaca4` | `0x305c4` | `r1` | `r2` | `r3` |
| `bl` at 0x307c8 | `(unknown callee) 0xc7fdc` | `0x0` | `0xf3a70` | `0x2` | `?` |
| `bl` at 0x307d0 | `(unknown callee) 0xbc854` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x307e0 | `(unknown callee) 0xcbf9c` | `0x0` | `0xf3a80` | `0x2` | `?` |
| `bl` at 0x307f0 | `(unknown callee) 0xc9658` | `0x0` | `0xf3bc0` | `0x16` | `?` |
| `bl` at 0x30800 | `(unknown callee) 0xc4e80` | `0x0` | `0xf3db8` | `0x14` | `?` |
| `bl` at 0x30810 | `(unknown callee) 0xbf638` | `0x0` | `0xf3ee4` | `0x7` | `?` |
| `bl` at 0x3082c | `(unknown callee) 0x35e44` | `0x0` | `0xf3fbc` | `0xe` | `0x2ef34` |
| `bl` at 0x3083c | `(unknown callee) 0x24214` | `0x0` | `0xf4378` | `0x7` | `?` |
| `bl` at 0x3084c | `(unknown callee) 0x24884` | `0x0` | `0xf4144` | `0x4` | `?` |
| `bl` at 0x3085c | `(unknown callee) 0x3f100` | `0x0` | `0xf4164` | `0x5` | `?` |
| `bl` at 0x3086c | `(unknown callee) 0x3ee60` | `0x0` | `0xf41f4` | `0xb` | `?` |
| `bl` at 0x3087c | `(unknown callee) 0x30c94` | `0xf4288` | `0x5` | `0x14` | `?` |
| `bl` at 0x30888 | `(unknown callee) 0xdceac` | `0x12d64c` | `0x2` | `?` | `?` |
| `bl` at 0x308ac | `(unknown callee) 0x5590c` | `0xf4548` | `0x1` | `0x30374` | `0x0` |
| `bl` at 0x308bc | `(unknown callee) 0x3acb8` | `0x0` | `0xf44b0` | `0x6` | `?` |
| `bl` at 0x308c4 | `(unknown callee) 0x229c8` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x308c8 | `(unknown callee) 0x2ed34` | `ret(0x229c8)` | `?` | `?` | `?` |
| `bl` at 0x308cc | `(unknown callee) 0x3a9c8` | `ret(0x2ed34)` | `?` | `?` | `?` |
| `bl` at 0x308d0 | `(unknown callee) 0x2ee84` | `ret(0x3a9c8)` | `?` | `?` | `?` |
| `bl` at 0x308dc | `(unknown callee) 0x4ab64` | `0xf455c` | `0x3` | `?` | `?` |
| `bl` at 0x308ec | `(unknown callee) 0x4b108` | `0xf43e8` | `0x2` | `0x1` | `?` |
| `bl` at 0x308f0 | `(unknown callee) 0x4b358` | `ret(0x4b108)` | `?` | `?` | `?` |
| `bl` at 0x3091c | `(unknown callee) 0x4f384` | `0x0` | `0x1` | `0xf4410` | `0x4` |
| `bl` at 0x30928 | `(unknown callee) 0xbba24` | `0x12d65c` | `0x3` | `?` | `?` |
| `bl` at 0x3093c | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x0` | `0x0` | `0x0` |
| `bl` at 0x30950 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x0` | `0x0` | `0x2` |
| `bl` at 0x30964 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x2` | `0x0` | `0x3` |
| `bl` at 0x30988 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x3099c | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x309b0 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x0` | `0x15` | `0x15` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x30974 | `[0x1320e9]:1 == 0x0` | `[0x1320e9]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x309b8 | `r0` | `ret(0xbbbbc)` |

## `Gemini_i2cConfig`

- `Gemini_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x28c04 to 0x28e64.
  Established. The unnamed `0xc4e80` call carries `0x0`, `0xf09d4`, `0x15`, unknown (`bl` at 0x28c60). A zero byte at `0x132089` takes `0x28dd0`, while nonzero falls through (`beq` at 0x28da4). The word at `sp -0x18` receives `0x27b48` (`str` at 0x28c78). Board-specific I2C setup
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x28c14 | load | 4 | `abs 0x28e00` | `-` |
| `ldr` at 0x28c20 | load | 4 | `abs 0x28e04` | `-` |
| `ldr` at 0x28c38 | load | 4 | `abs 0x28e08` | `-` |
| `ldr` at 0x28c48 | load | 4 | `abs 0x28e0c` | `-` |
| `ldr` at 0x28c58 | load | 4 | `abs 0x28e10` | `-` |
| `ldr` at 0x28c68 | load | 4 | `abs 0x28e14` | `-` |
| `ldr` at 0x28c74 | load | 4 | `abs 0x28e18` | `-` |
| `str` at 0x28c78 | store | 4 | `sp -0x18` | `0x27b48` |
| `ldr` at 0x28c80 | load | 4 | `abs 0x28e1c` | `-` |
| `ldr` at 0x28c88 | load | 4 | `abs 0x28e20` | `-` |
| `ldr` at 0x28c94 | load | 4 | `abs 0x28e24` | `-` |
| `ldr` at 0x28ca4 | load | 4 | `abs 0x28e28` | `-` |
| `ldr` at 0x28cb4 | load | 4 | `abs 0x28e2c` | `-` |
| `ldr` at 0x28cc4 | load | 4 | `abs 0x28e30` | `-` |
| `ldr` at 0x28cd0 | load | 4 | `abs 0x28e34` | `-` |
| `str` at 0x28ce0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x28ce8 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x28cec | load | 4 | `abs 0x28e38` | `-` |
| `ldr` at 0x28cf4 | load | 4 | `abs 0x28e3c` | `-` |
| `ldr` at 0x28d04 | load | 4 | `abs 0x28e40` | `-` |
| `ldr` at 0x28d10 | load | 4 | `abs 0x28e44` | `-` |
| `ldr` at 0x28d30 | load | 4 | `abs 0x28e48` | `-` |
| `ldr` at 0x28d44 | load | 4 | `abs 0x28e4c` | `-` |
| `ldr` at 0x28d50 | load | 4 | `abs 0x28e50` | `-` |
| `ldr` at 0x28d5c | load | 4 | `abs 0x28e54` | `-` |
| `ldr` at 0x28d70 | load | 4 | `abs 0x28e58` | `-` |
| `ldr` at 0x28d84 | load | 4 | `abs 0x28e54` | `-` |
| `ldr` at 0x28d98 | load | 4 | `abs 0x28e5c` | `-` |
| `ldrb` at 0x28d9c | load | 1 | `abs 0x132089` | `-` |
| `ldr` at 0x28da8 | load | 4 | `abs 0x28e60` | `-` |
| `ldr` at 0x28dbc | load | 4 | `abs 0x28e58` | `-` |
| `ldr` at 0x28dd0 | load | 4 | `abs 0x28e60` | `-` |
| `ldr` at 0x28de4 | load | 4 | `abs 0x28e58` | `-` |
| `ldm` at 0x28dfc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x28dfc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x28dfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x28c18 | `(unknown callee) 0xbaca4` | `0x28a24` | `r1` | `r2` | `r3` |
| `bl` at 0x28c28 | `(unknown callee) 0xc7fdc` | `0x0` | `0xf06f0` | `0x2` | `?` |
| `bl` at 0x28c30 | `(unknown callee) 0xbc854` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x28c40 | `(unknown callee) 0xcbf9c` | `0x0` | `0xf0708` | `0x2` | `?` |
| `bl` at 0x28c50 | `(unknown callee) 0xc9658` | `0x0` | `0xf081c` | `0x12` | `?` |
| `bl` at 0x28c60 | `(unknown callee) 0xc4e80` | `0x0` | `0xf09d4` | `0x15` | `?` |
| `bl` at 0x28c70 | `(unknown callee) 0xbf638` | `0x0` | `0xf0b0c` | `0x7` | `?` |
| `bl` at 0x28c8c | `(unknown callee) 0x35e44` | `0x0` | `0xf0c24` | `0x14` | `0x27b14` |
| `bl` at 0x28c9c | `(unknown callee) 0x24214` | `0x0` | `0xf0f6c` | `0x7` | `?` |
| `bl` at 0x28cac | `(unknown callee) 0x24884` | `0x0` | `0xf0e54` | `0x5` | `?` |
| `bl` at 0x28cbc | `(unknown callee) 0x3f100` | `0x0` | `0xf0e7c` | `0x4` | `?` |
| `bl` at 0x28ccc | `(unknown callee) 0x3ee60` | `0x0` | `0xf0f00` | `0x9` | `?` |
| `bl` at 0x28cd8 | `(unknown callee) 0xdceac` | `0x12d4e4` | `0x2` | `?` | `?` |
| `bl` at 0x28cfc | `(unknown callee) 0x5590c` | `0xf10ec` | `0x1` | `0x287cc` | `0x0` |
| `bl` at 0x28d0c | `(unknown callee) 0x3acb8` | `0x0` | `0xf1050` | `0x6` | `?` |
| `bl` at 0x28d18 | `(unknown callee) 0x3e690` | `0xf10e0` | `0x1` | `?` | `?` |
| `bl` at 0x28d20 | `(unknown callee) 0x229c8` | `0x11` | `?` | `?` | `?` |
| `bl` at 0x28d24 | `(unknown callee) 0x27754` | `ret(0x229c8)` | `?` | `?` | `?` |
| `bl` at 0x28d28 | `(unknown callee) 0x3a9c8` | `ret(0x27754)` | `?` | `?` | `?` |
| `bl` at 0x28d2c | `(unknown callee) 0x27a64` | `ret(0x3a9c8)` | `?` | `?` | `?` |
| `bl` at 0x28d3c | `(unknown callee) 0x4b108` | `0xf0fd4` | `0x2` | `0x1` | `?` |
| `bl` at 0x28d40 | `(unknown callee) 0x4b358` | `ret(0x4b108)` | `?` | `?` | `?` |
| `bl` at 0x28d4c | `(unknown callee) 0xcea34` | `0xf1008` | `0x1` | `?` | `?` |
| `bl` at 0x28d58 | `(unknown callee) 0xbba24` | `0x12d4f4` | `0x4` | `?` | `?` |
| `bl` at 0x28d6c | `(unknown callee) 0xbbbbc` | `0xca960` | `0x0` | `0x11` | `0x11` |
| `bl` at 0x28d80 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x0` | `0x0` | `0x1` |
| `bl` at 0x28d94 | `(unknown callee) 0xbbbbc` | `0xca960` | `0x3` | `0x0` | `0x3` |
| `bl` at 0x28db8 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x1` | `0x1` | `0x1` |
| `bl` at 0x28dcc | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x1` | `0x3` | `0x6` |
| `bl` at 0x28de0 | `(unknown callee) 0xbbbbc` | `0xcd1a0` | `0x2` | `0x0` | `0x0` |
| `bl` at 0x28df4 | `(unknown callee) 0xbbbbc` | `0xc1428` | `0x2` | `0x2` | `0x2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x28da4 | `[0x132089]:1 == 0x0` | `[0x132089]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x28dfc | `r0` | `ret(0xbbbbc)` |

## `Unknown_i2cConfig`

- `Unknown_i2cConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x227d8 to 0x228e8.
  Established. The unnamed `0xc4e80` call carries `0x0`, `0xee180`, `0x9`, unknown (`bl` at 0x22880). An included byte assignment at `sp -0x11` writes zero (`strb` at 0x22888). The unnamed
  `0xc51dc` call carries `0x0`, unknown, unknown, unknown (`bl` at 0x228a4). Fallback board
  configuration remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x227ec | load | 4 | `abs 0x228c4` | `-` |
| `str` at 0x227fc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x22804 | load | 4 | `abs 0x228c8` | `-` |
| `ldr` at 0x22818 | load | 4 | `abs 0x228cc` | `-` |
| `ldr` at 0x22828 | load | 4 | `abs 0x228d0` | `-` |
| `ldr` at 0x22838 | load | 4 | `abs 0x228d4` | `-` |
| `ldr` at 0x22848 | load | 4 | `abs 0x228d8` | `-` |
| `ldr` at 0x22858 | load | 4 | `abs 0x228dc` | `-` |
| `ldr` at 0x22868 | load | 4 | `abs 0x228e0` | `-` |
| `ldr` at 0x22878 | load | 4 | `abs 0x228e4` | `-` |
| `strb` at 0x22888 | store | 1 | `sp -0x11` | `0x0` |
| `ldrb` at 0x2288c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x22898 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x228a8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x228b0 | store | 1 | `sp -0x11` | `([sp-17]:1+1 & 0xff)` |
| `ldm` at 0x228c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x228c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x228c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x227f4 | `(unknown callee) 0xc9658` | `0x0` | `0xee128` | `0x0` | `r3` |
| `bl` at 0x22810 | `(unknown callee) 0x35e44` | `0x0` | `0xee1ec` | `0x0` | `0x0` |
| `bl` at 0x22820 | `(unknown callee) 0x3f100` | `0x0` | `0xee214` | `0x0` | `?` |
| `bl` at 0x22830 | `(unknown callee) 0x3ee60` | `0x0` | `0xee208` | `0x0` | `?` |
| `bl` at 0x22840 | `(unknown callee) 0xbf638` | `0x0` | `0xee220` | `0x0` | `?` |
| `bl` at 0x22850 | `(unknown callee) 0x24884` | `0x0` | `0xee22c` | `0x0` | `?` |
| `bl` at 0x22860 | `(unknown callee) 0x3acb8` | `0x0` | `0xee234` | `0x0` | `?` |
| `bl` at 0x22870 | `(unknown callee) 0x24214` | `0x0` | `0xee24c` | `0x0` | `?` |
| `bl` at 0x22880 | `(unknown callee) 0xc4e80` | `0x0` | `0xee180` | `0x9` | `?` |
| `bl` at 0x228a4 | `(unknown callee) 0xc51dc` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x228b8 | `(unknown callee) 0x3a9c8` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x22894 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x228c0 | `r0` | `ret(0x3a9c8)` |
