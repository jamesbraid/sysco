# ARM CPU, node and NMI helpers in 1.44.0

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
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/cpu-node-nmi-facts.txt) retain all tool sections.

## `cpuPresent`

- `cpuPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54244 to 0x542cc.
  Established. A zero `[0x297610]` takes `0x54290`, while nonzero falls through (`beq` at 0x54268).
  The indirect call has unknown target and carries `(r0 & 0xff)`, entry `r1`, `(r0 & 0xff)`, `[0x297610]` (`mov` at 0x54280).
  CPU-presence callback semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54258 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5425c | load | 4 | `abs 0x542c8` | `-` |
| `ldr` at 0x54260 | load | 4 | `abs 0x297610` | `-` |
| `ldr` at 0x5426c | load | 4 | `abs 0x542c8` | `-` |
| `ldrb` at 0x54270 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x54274 | load | 4 | `abs 0x297610` | `-` |
| `str` at 0x54288 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x54290 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54294 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x54298 | load | 4 | `abs 0x542c8` | `-` |
| `ldr` at 0x5429c | load | 4 | `abs 0x297608` | `-` |
| `ldr` at 0x542a0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x542b0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x542b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x542b8 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x542bc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x542c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x542c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x542c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x54280 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x297610]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54268 | `[0x297610] == 0x0` | `[0x297610] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x542c4 | `r0` | `?` |

## `cpuEnable`

- `cpuEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54450 to 0x54544.
  Established. The unnamed `0x375e4` call carries entry `r0`, `r1`, `r2` and `0x0` (`bl` at 0x54470).
  A zero result takes `0x5448c`, while nonzero falls through (`beq` at 0x5447c).
  An included word assignment writes `0xfffff63a` to `sp -0x24` (`str` at 0x54484).
  CPU enable error meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54464 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5446c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x54480 | load | 4 | `abs 0x5453c` | `-` |
| `str` at 0x54484 | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x5448c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5449c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x544a0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x544a4 | load | 4 | `abs 0x54540` | `-` |
| `ldr` at 0x544a8 | load | 4 | `abs 0x297608` | `-` |
| `ldrb` at 0x544b4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x544cc | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x544dc | store | 4 | `sp -0x1c` | `ret(0x54604)` |
| `ldr` at 0x544e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x544ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x544f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x544fc | store | 4 | `sp -0x18` | `ret(0x54604)` |
| `ldrb` at 0x54500 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x54508 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x54510 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54514 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5451c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5452c | store | 4 | `sp -0x24` | `ret(0x54604)` |
| `ldr` at 0x54530 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x54538 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54538 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54538 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54470 | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x544bc | `(unknown callee) 0x54244` | `?` | `?` | `[0x297608]` | `?` |
| `bl` at 0x544d4 | `(unknown callee) 0x54604` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x54524 | `(unknown callee) 0x54604` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5447c | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bne` at 0x54494 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `bge` at 0x544b0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x544c8 | `ret(0x54244) == 0x0` | `ret(0x54244) != 0x0` |
| `beq` at 0x544e8 | `ret(0x54604) == 0x0` | `ret(0x54604) != 0x0` |
| `bne` at 0x544f4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54538 | `r0` | `?` |

## `cpuDisable`

- `cpuDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5435c to 0x54450.
  Established. A selector `[sp-17]:1` unequal to `0xff` takes `0x54428`, while equality falls through (`bne` at 0x543a0).
  The unnamed `0x54544` call carries `[sp-17]:1`, two unknown candidates and `[sp-17]:1` (`bl` at 0x54430).
  An included byte store writes `([sp-29]:1+1 & 0xff)` to `sp -0x1d` (`strb` at 0x54414).
  CPU disable policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54370 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x54378 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5438c | load | 4 | `abs 0x54448` | `-` |
| `str` at 0x54390 | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x54398 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x543a8 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x543ac | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x543b0 | load | 4 | `abs 0x5444c` | `-` |
| `ldr` at 0x543b4 | load | 4 | `abs 0x297608` | `-` |
| `ldrb` at 0x543c0 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x543d8 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x543e8 | store | 4 | `sp -0x1c` | `ret(0x54544)` |
| `ldr` at 0x543ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x543f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54404 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54408 | store | 4 | `sp -0x18` | `ret(0x54544)` |
| `ldrb` at 0x5440c | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x54414 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5441c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54420 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x54428 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54438 | store | 4 | `sp -0x24` | `ret(0x54544)` |
| `ldr` at 0x5443c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x54444 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54444 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54444 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5437c | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x543c8 | `(unknown callee) 0x54244` | `?` | `?` | `[0x297608]` | `?` |
| `bl` at 0x543e0 | `(unknown callee) 0x54544` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x54430 | `(unknown callee) 0x54544` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54388 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bne` at 0x543a0 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `bge` at 0x543bc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x543d4 | `ret(0x54244) == 0x0` | `ret(0x54244) != 0x0` |
| `beq` at 0x543f4 | `ret(0x54544) == 0x0` | `ret(0x54544) != 0x0` |
| `bne` at 0x54400 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54444 | `r0` | `?` |

## `cpuSetState`

- `cpuSetState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x546c4 to 0x5478c.
  Established. The signed condition `(r0 & 0xff) < [0x297608]` takes `0x54758`, while greater or equal falls through (`blt` at 0x54748).
  The unnamed `0x55300` call carries `(r0 & 0xff)+10`, `(r1 & 0xff)`, `(r1 & 0xff)`, `(r0 & 0xff)+10` (`bl` at 0x5476c).
  The state selector encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x546dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x546e4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x546e8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x546f8 | store | 1 | `sp -0x13` | `0x0` |
| `ldrb` at 0x546fc | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x54700 | load | 4 | `abs 0x54784` | `-` |
| `ldr` at 0x54704 | load | 4 | `abs 0x297608` | `-` |
| `ldrb` at 0x54710 | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x54718 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x54728 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x54730 | store | 1 | `sp -0x13` | `([sp-19]:1+1 & 0xff)` |
| `ldrb` at 0x54738 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5473c | load | 4 | `abs 0x54784` | `-` |
| `ldr` at 0x54740 | load | 4 | `abs 0x297608` | `-` |
| `ldr` at 0x5474c | load | 4 | `abs 0x54788` | `-` |
| `str` at 0x54750 | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x54758 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x54760 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x54774 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x54778 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x54780 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54780 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54780 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54724 | `(unknown callee) 0x55300` | `?` | `?` | `?` | `?` |
| `bl` at 0x5476c | `(unknown callee) 0x55300` | `(r0 & 0xff)+10` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x546f0 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `bge` at 0x5470c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blt` at 0x54748 | `(r0 & 0xff) < [0x297608] (signed, 32-bit)` | `(r0 & 0xff) >= [0x297608] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54780 | `r0` | `[sp-24]` |

## `cpuState`

- `cpuState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5478c to 0x547f4.
  Established. A word read addresses `0x297608` (`ldr` at 0x547ac).
  The signed condition `(r0 & 0xff) < [0x297608]` takes `0x547c4`, while greater or equal falls through (`blt` at 0x547b4).
  The unnamed `0x554b8` call carries `(r0 & 0xff)+10`, entry `r1`, `(r0 & 0xff)`, `(r0 & 0xff)+10` (`bl` at 0x547d0).
  State-result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x547a0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x547a4 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x547a8 | load | 4 | `abs 0x547ec` | `-` |
| `ldr` at 0x547ac | load | 4 | `abs 0x297608` | `-` |
| `ldr` at 0x547b8 | load | 4 | `abs 0x547f0` | `-` |
| `str` at 0x547bc | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x547c4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x547dc | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x547e0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x547e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x547e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x547e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x547d0 | `(unknown callee) 0x554b8` | `(r0 & 0xff)+10` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x547b4 | `(r0 & 0xff) < [0x297608] (signed, 32-bit)` | `(r0 & 0xff) >= [0x297608] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x547e8 | `r0` | `?` |

## `cpuStateString`

- `cpuStateString`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x547f4 to 0x548c4.
  Established. The unnamed `0x554b8` call carries `(r0 & 0xff)+10`, entry `r1`, entry `r2`, `(r0 & 0xff)+10` (`bl` at 0x54818).
  Included word reads address `0x12d774` (`ldr` at 0x54830, `ldr` at 0x54870).
  A word assignment writes `0xf8fa4` to `sp -0x20` (`str` at 0x548a4).
  The returned label meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54808 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5480c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5481c | store | 4 | `sp -0x1c` | `ret(0x554b8)` |
| `str` at 0x54824 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x54828 | load | 4 | `abs 0x548b8` | `-` |
| `ldr` at 0x5482c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54830 | load | 4 | `abs 0x12d774` | `-` |
| `ldr` at 0x5483c | load | 4 | `abs 0x548bc` | `-` |
| `ldr` at 0x54840 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54844 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x54848 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54858 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54860 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x54868 | load | 4 | `abs 0x548b8` | `-` |
| `ldr` at 0x5486c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54870 | load | 4 | `abs 0x12d774` | `-` |
| `ldr` at 0x5487c | load | 4 | `abs 0x548bc` | `-` |
| `ldr` at 0x54880 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54894 | load | 4 | `(unknown)` | `-` |
| `str` at 0x54898 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x548a0 | load | 4 | `abs 0x548c0` | `-` |
| `str` at 0x548a4 | store | 4 | `sp -0x20` | `0xf8fa4` |
| `ldr` at 0x548a8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x548b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x548b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x548b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54818 | `(unknown callee) 0x554b8` | `(r0 & 0xff)+10` | `r1` | `r2` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x54838 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x54850 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x54878 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x548b4 | `r0` | `?` |

## `nodePresent`

- `nodePresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x548e4 to 0x5496c.
  Established. The zero `[0x297638]` branch takes `0x54930`, while nonzero falls through (`beq` at 0x54908);
  its indirect call carries `(r0 & 0xff)`, entry `r1`, `(r0 & 0xff)`, `[0x297638]` with unknown target (`mov` at 0x54920).
  An included word read addresses `0x297634` (`ldr` at 0x5493c).
  Node-presence fallback meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x548f8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x548fc | load | 4 | `abs 0x54968` | `-` |
| `ldr` at 0x54900 | load | 4 | `abs 0x297638` | `-` |
| `ldr` at 0x5490c | load | 4 | `abs 0x54968` | `-` |
| `ldrb` at 0x54910 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x54914 | load | 4 | `abs 0x297638` | `-` |
| `str` at 0x54928 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x54930 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54934 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x54938 | load | 4 | `abs 0x54968` | `-` |
| `ldr` at 0x5493c | load | 4 | `abs 0x297634` | `-` |
| `ldr` at 0x54940 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54950 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x54954 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54958 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x5495c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x54964 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54964 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54964 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x54920 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x297638]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54908 | `[0x297638] == 0x0` | `[0x297638] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54964 | `r0` | `?` |

## `nodeEnable`

- `nodeEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54b00 to 0x54bf4.
  Established. A nonzero `[sp-24]` takes `0x54bb0`, while zero falls through (`bne` at 0x54ba4).
  An included word assignment writes `ret(0x54e2c)` to `sp -0x18` (`str` at 0x54bac).
  The unnamed `0x54e2c` call carries `[sp-17]:1`, two unknown candidates and `[sp-17]:1` (`bl` at 0x54bd4).
  Node enable status policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54b14 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x54b1c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x54b30 | load | 4 | `abs 0x54bec` | `-` |
| `str` at 0x54b34 | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x54b3c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x54b4c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x54b50 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x54b54 | load | 4 | `abs 0x54bf0` | `-` |
| `ldr` at 0x54b58 | load | 4 | `abs 0x297634` | `-` |
| `ldrb` at 0x54b64 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x54b7c | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x54b8c | store | 4 | `sp -0x1c` | `ret(0x54e2c)` |
| `ldr` at 0x54b90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54b9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54ba8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54bac | store | 4 | `sp -0x18` | `ret(0x54e2c)` |
| `ldrb` at 0x54bb0 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x54bb8 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x54bc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54bc4 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x54bcc | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54bdc | store | 4 | `sp -0x24` | `ret(0x54e2c)` |
| `ldr` at 0x54be0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x54be8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54be8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54be8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54b20 | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x54b6c | `(unknown callee) 0x548e4` | `?` | `?` | `[0x297634]` | `?` |
| `bl` at 0x54b84 | `(unknown callee) 0x54e2c` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x54bd4 | `(unknown callee) 0x54e2c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54b2c | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bne` at 0x54b44 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `bge` at 0x54b60 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x54b78 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `beq` at 0x54b98 | `ret(0x54e2c) == 0x0` | `ret(0x54e2c) != 0x0` |
| `bne` at 0x54ba4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54be8 | `r0` | `?` |

## `nodeDisable`

- `nodeDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54a0c to 0x54b00.
  Established. A zero `ret(0x548e4)` takes `0x54abc`, while nonzero falls through (`beq` at 0x54a84).
  Included word stores write `ret(0x54d58)` to `sp -0x1c` and `sp -0x18` (`str` at 0x54a98, `str` at 0x54ab8).
  Node disable completion conditions remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x54a20 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x54a28 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x54a3c | load | 4 | `abs 0x54af8` | `-` |
| `str` at 0x54a40 | store | 4 | `sp -0x24` | `0xfffff5d6` |
| `ldrb` at 0x54a48 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x54a58 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x54a5c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x54a60 | load | 4 | `abs 0x54afc` | `-` |
| `ldr` at 0x54a64 | load | 4 | `abs 0x297634` | `-` |
| `ldrb` at 0x54a70 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x54a88 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x54a98 | store | 4 | `sp -0x1c` | `ret(0x54d58)` |
| `ldr` at 0x54a9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54aa8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x54ab4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54ab8 | store | 4 | `sp -0x18` | `ret(0x54d58)` |
| `ldrb` at 0x54abc | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x54ac4 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x54acc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x54ad0 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x54ad8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x54ae8 | store | 4 | `sp -0x24` | `ret(0x54d58)` |
| `ldr` at 0x54aec | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x54af4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54af4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54a2c | `(unknown callee) 0x375e4` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x54a78 | `(unknown callee) 0x548e4` | `?` | `?` | `[0x297634]` | `?` |
| `bl` at 0x54a90 | `(unknown callee) 0x54d58` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x54ae0 | `(unknown callee) 0x54d58` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x54a38 | `ret(0x375e4) == 0x0` | `ret(0x375e4) != 0x0` |
| `bne` at 0x54a50 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `bge` at 0x54a6c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x54a84 | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |
| `beq` at 0x54aa4 | `ret(0x54d58) == 0x0` | `ret(0x54d58) != 0x0` |
| `bne` at 0x54ab0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54af4 | `r0` | `?` |

## `nodeOp`

- `nodeOp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x54bf4 to 0x54d58.
  Established. The byte selector `(r1 & 0xff)` unequal to `0xff` takes `0x54cb0`, while equality falls through (`bne` at 0x54c20).
  Two indirect calls each carry `[sp-21]:1`, unknown, `[sp-20]`, `[sp-21]:1` to unknown targets (`mov` at 0x54c68, `mov` at 0x54cf8).
  Included word assignments write `0xfffff5d5` and `0xfffff5d4` to `sp -0x28` (`str` at 0x54d24, `str` at 0x54d30).
  Node operation callback effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x54c04 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x54c0c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `str` at 0x54c14 | store | 4 | `sp -0x1c` | `0x0` |
| `ldrb` at 0x54c18 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x54c28 | store | 1 | `sp -0x15` | `0x0` |
| `ldrb` at 0x54c2c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54c30 | load | 4 | `abs 0x54d48` | `-` |
| `ldr` at 0x54c34 | load | 4 | `abs 0x297634` | `-` |
| `ldrb` at 0x54c40 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x54c58 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54c5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54c70 | store | 4 | `sp -0x20` | `ret(unknown)` |
| `ldr` at 0x54c74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x54c80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x54c8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x54c90 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `ldrb` at 0x54c94 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x54c9c | store | 1 | `sp -0x15` | `([sp-21]:1+1 & 0xff)` |
| `ldr` at 0x54ca4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x54ca8 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x54cb0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54cb4 | load | 4 | `abs 0x54d48` | `-` |
| `ldr` at 0x54cb8 | load | 4 | `abs 0x297634` | `-` |
| `ldr` at 0x54cc4 | load | 4 | `abs 0x54d4c` | `-` |
| `str` at 0x54cc8 | store | 4 | `sp -0x24` | `0xfffff5d7` |
| `ldrb` at 0x54cd0 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x54ce8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54cec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x54d00 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldrb` at 0x54d08 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x54d20 | load | 4 | `abs 0x54d50` | `-` |
| `str` at 0x54d24 | store | 4 | `sp -0x28` | `0xfffff5d5` |
| `ldr` at 0x54d2c | load | 4 | `abs 0x54d54` | `-` |
| `str` at 0x54d30 | store | 4 | `sp -0x28` | `0xfffff5d4` |
| `ldr` at 0x54d34 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x54d38 | store | 4 | `sp -0x24` | `[sp-40]` |
| `ldr` at 0x54d3c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x54d44 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x54d44 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x54d44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x54c48 | `(unknown callee) 0x5496c` | `?` | `?` | `[0x297634]` | `?` |
| `mov` at 0x54c68 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x54cd8 | `(unknown callee) 0x5496c` | `(r1 & 0xff)` | `r1` | `[0x297634]` | `(r1 & 0xff)` |
| `mov` at 0x54cf8 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-20]` | `[sp-21]:1` |
| `bl` at 0x54d10 | `(unknown callee) 0x548e4` | `[sp-21]:1` | `?` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x54c20 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bge` at 0x54c3c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x54c54 | `ret(0x5496c) == 0x0` | `ret(0x5496c) != 0x0` |
| `beq` at 0x54c7c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bne` at 0x54c88 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blt` at 0x54cc0 | `(r1 & 0xff) < [0x297634] (signed, 32-bit)` | `(r1 & 0xff) >= [0x297634] (signed, 32-bit)` |
| `beq` at 0x54ce4 | `ret(0x5496c) == 0x0` | `ret(0x5496c) != 0x0` |
| `beq` at 0x54d1c | `ret(0x548e4) == 0x0` | `ret(0x548e4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x54d44 | `r0` | `?` |

## `nmi`

- `nmi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47ed0 to 0x47f54.
  Established. The unnamed `0x1e958` call carries `0xf7950`, `ret(0x48054)`, unknown, `ret(0x48054)` (`bl` at 0x47f00).
  A zero `[0x297654]` takes `0x47f38`, while nonzero falls through (`beq` at 0x47f10).
  The indirect call target is unknown (`mov` at 0x47f28).
  NMI delivery through that callback remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47ee0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x47ee8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x47eec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47ef8 | load | 4 | `abs 0x47f4c` | `-` |
| `ldr` at 0x47f04 | load | 4 | `abs 0x47f50` | `-` |
| `ldr` at 0x47f08 | load | 4 | `abs 0x297654` | `-` |
| `ldr` at 0x47f14 | load | 4 | `abs 0x47f50` | `-` |
| `ldrb` at 0x47f18 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x47f1c | load | 4 | `abs 0x297654` | `-` |
| `str` at 0x47f30 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x47f3c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x47f40 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x47f48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47f48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47f48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47ef0 | `(unknown callee) 0x48054` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x47f00 | `(unknown callee) 0x1e958` | `0xf7950` | `ret(0x48054)` | `?` | `ret(0x48054)` |
| `mov` at 0x47f28 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-21]:1` | `[0x297654]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47f10 | `[0x297654] == 0x0` | `[0x297654] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47f48 | `r0` | `?` |

## `nmiMaskSet`

- `nmiMaskSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47f54 to 0x47fdc.
  Established. Word stores to `0x132410` and `0x132414` carry unknown values (`str` at 0x47f80, `str` at 0x47f98, `str` at 0x47fb0, `str` at 0x47fc8).
  The `Z == 0` branch takes `0x47f88`, with the complementary predicate falling through (`bne` at 0x47f74).
  Mask bit selection remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47f64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x47f68 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x47f6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x47f78 | load | 4 | `abs 0x47fd4` | `-` |
| `str` at 0x47f80 | store | 4 | `abs 0x132410` | `?` |
| `ldr` at 0x47f88 | load | 4 | `abs 0x47fd4` | `-` |
| `ldr` at 0x47f90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x47f98 | store | 4 | `abs 0x132410` | `?` |
| `ldr` at 0x47f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47fa8 | load | 4 | `abs 0x47fd8` | `-` |
| `str` at 0x47fb0 | store | 4 | `abs 0x132414` | `?` |
| `ldr` at 0x47fb8 | load | 4 | `abs 0x47fd8` | `-` |
| `ldr` at 0x47fc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x47fc8 | store | 4 | `abs 0x132414` | `?` |
| `ldm` at 0x47fd0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47fd0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47fd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x47f74 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x47fa4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47fd0 | `r0` | `r0` |

## `nmiMaskClr`

- `nmiMaskClr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47fdc to 0x4800c.
  Established. Two word assignments clear `0x132410` and `0x132414` to `0x0` (`str` at 0x47ff0, `str` at 0x47ffc).
  The integer return record preserves entry `r0` (`ldm` at 0x48000).
  These locations have Unknown device ownership.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x47fe8 | load | 4 | `abs 0x48004` | `-` |
| `str` at 0x47ff0 | store | 4 | `abs 0x132410` | `0x0` |
| `ldr` at 0x47ff4 | load | 4 | `abs 0x48008` | `-` |
| `str` at 0x47ffc | store | 4 | `abs 0x132414` | `0x0` |
| `ldm` at 0x48000 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48000 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48000 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48000 | `r0` | `r0` |

## `nmiMaskGet`

- `nmiMaskGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4800c to 0x48054.
  Established. Word reads address `0x132410` and `0x132414` (`ldr` at 0x4802c, `ldr` at 0x4803c).
  Their values are stored respectively to `r0 +0x0` and `[sp-24] +0x0` (`str` at 0x48030, `str` at 0x48040).
  Mask consumers beyond these output pointers remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4801c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x48020 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x48024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x48028 | load | 4 | `abs 0x4804c` | `-` |
| `ldr` at 0x4802c | load | 4 | `abs 0x132410` | `-` |
| `str` at 0x48030 | store | 4 | `r0 +0x0` | `[0x132410]` |
| `ldr` at 0x48034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x48038 | load | 4 | `abs 0x48050` | `-` |
| `ldr` at 0x4803c | load | 4 | `abs 0x132414` | `-` |
| `str` at 0x48040 | store | 4 | `[sp-24] +0x0` | `[0x132414]` |
| `ldm` at 0x48048 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x48048 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x48048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x48048 | `r0` | `r0` |

## `IP57_Nmi`

- `IP57_Nmi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x575b0 to 0x57690.
  Established. A zero `ret(0x57964)` takes `0x575fc`, while nonzero falls through (`beq` at 0x575ec).
  A byte read addresses `[sp-24] +0x2` (`ldrb` at 0x57600).
  Included unnamed `0xca538` and `0xca4e8` calls use first candidate `0x0` and third candidate `0x10` (`bl` at 0x5761c, `bl` at 0x57658).
  The NMI line mapping remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x575c4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x575c8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x575e0 | store | 4 | `sp -0x20` | `ret(0x57964)` |
| `ldr` at 0x575e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x575f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x575f4 | store | 4 | `sp -0x24` | `ret(0x57964)` |
| `ldr` at 0x575fc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x57600 | load | 1 | `[sp-24] +0x2` | `-` |
| `strb` at 0x57608 | store | 1 | `sp -0x19` | `([[sp-24]+2]:1+2 & 0xff)` |
| `ldrb` at 0x5760c | load | 1 | `sp -0x19` | `-` |
| `str` at 0x57624 | store | 4 | `sp -0x20` | `ret(0xca538)` |
| `ldr` at 0x57628 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57634 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57638 | store | 4 | `sp -0x24` | `ret(0xca538)` |
| `ldrb` at 0x57648 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x57660 | store | 4 | `sp -0x20` | `ret(0xca4e8)` |
| `ldr` at 0x57664 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x57670 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x57674 | store | 4 | `sp -0x24` | `ret(0xca4e8)` |
| `str` at 0x57680 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x57684 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x5768c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x5768c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x5768c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x575d8 | `(unknown callee) 0x57964` | `(r0 & 0xff)` | `sp-24` | `sp-24` | `(r0 & 0xff)` |
| `bl` at 0x5761c | `(unknown callee) 0xca538` | `0x0` | `([[sp-24]+2]:1+2 & 0xff)` | `0x10` | `([[sp-24]+2]:1+2 & 0xff)` |
| `bl` at 0x57644 | `(unknown callee) 0x570a8` | `0xa` | `?` | `?` | `ret(0xca538)` |
| `bl` at 0x57658 | `(unknown callee) 0xca4e8` | `0x0` | `[sp-25]:1` | `0x10` | `[sp-25]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x575ec | `ret(0x57964) == 0x0` | `ret(0x57964) != 0x0` |
| `beq` at 0x57630 | `ret(0xca538) == 0x0` | `ret(0xca538) != 0x0` |
| `beq` at 0x5766c | `ret(0xca4e8) == 0x0` | `ret(0xca4e8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x5768c | `r0` | `?` |
