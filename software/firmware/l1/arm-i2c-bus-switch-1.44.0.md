# ARM I2C bus and switch paths in 1.44.0

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
Unknown data values and instruction effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/i2c-bus-switch-facts.txt) retain all tool sections.

## `i2cInit`

- `i2cInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaaa0 to 0xbac78.
  Established. The word at `0x29aeec` receives `ret(0x193b8)` (`str` at 0xbab60). A later word
  assignment to that address writes `[0x29aeec]+36` (`str` at 0xbabb8). The unnamed `0x193b8` call
  carries `0x0`, `0x100`, `0x3`, `0x1` (`bl` at 0xbab5c). I2C initialization effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbaab0 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xbaab4 | load | 4 | `abs 0xbac50` | `-` |
| `ldr` at 0xbaab8 | load | 4 | `abs 0x12d8dc` | `-` |
| `ldr` at 0xbaac4 | load | 4 | `abs 0xbac54` | `-` |
| `str` at 0xbaac8 | store | 4 | `sp -0x28` | `0x11ec74` |
| `ldr` at 0xbaad0 | load | 4 | `abs 0xbac58` | `-` |
| `ldr` at 0xbaae0 | load | 4 | `abs 0xbac5c` | `-` |
| `ldr` at 0xbaae4 | load | 4 | `abs 0xbac60` | `-` |
| `str` at 0xbaaf0 | store | 4 | `sp -0x1c` | `ret(0x1967c)` |
| `ldr` at 0xbaaf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbab0c | load | 4 | `abs 0xbac64` | `-` |
| `str` at 0xbab10 | store | 4 | `sp -0x28` | `0x11eca0` |
| `ldr` at 0xbab14 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbab18 | store | 4 | `sp -0x24` | `[ret(0x193d0)]` |
| `ldr` at 0xbab20 | load | 4 | `abs 0xbac58` | `-` |
| `ldr` at 0xbab38 | load | 4 | `abs 0xbac68` | `-` |
| `ldr` at 0xbab3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xbab40 | store | 4 | `sp -0x28` | `?` |
| `str` at 0xbab48 | store | 4 | `sp -0x24` | `0x56000000` |
| `str` at 0xbab60 | store | 4 | `abs 0x29aeec` | `ret(0x193b8)` |
| `ldr` at 0xbab64 | load | 4 | `abs 0x29aeec` | `-` |
| `ldr` at 0xbab7c | load | 4 | `abs 0xbac6c` | `-` |
| `str` at 0xbab80 | store | 4 | `sp -0x28` | `0x11ecc0` |
| `ldr` at 0xbab84 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbab88 | store | 4 | `sp -0x24` | `[ret(0x193d0)]` |
| `ldr` at 0xbab90 | load | 4 | `abs 0xbac58` | `-` |
| `ldr` at 0xbaba8 | load | 4 | `abs 0xbac68` | `-` |
| `ldr` at 0xbabac | load | 4 | `abs 0xbac68` | `-` |
| `ldr` at 0xbabb0 | load | 4 | `abs 0x29aeec` | `-` |
| `str` at 0xbabb8 | store | 4 | `abs 0x29aeec` | `[0x29aeec]+36` |
| `ldr` at 0xbabbc | load | 4 | `abs 0xbac70` | `-` |
| `ldr` at 0xbabcc | load | 4 | `abs 0xbac50` | `-` |
| `str` at 0xbabd0 | store | 4 | `abs 0x12d8dc` | `ret(0x1967c)` |
| `ldr` at 0xbabd4 | load | 4 | `abs 0x12d8dc` | `-` |
| `ldr` at 0xbabe8 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbabf4 | load | 4 | `abs 0xbac74` | `-` |
| `str` at 0xbabf8 | store | 4 | `sp -0x28` | `0x11ecf8` |
| `str` at 0xbabfc | store | 4 | `sp -0x24` | `ret(0x1f8e0)` |
| `ldr` at 0xbac04 | load | 4 | `abs 0xbac58` | `-` |
| `strb` at 0xbac18 | store | 1 | `sp -0x1d` | `0x1` |
| `ldrb` at 0xbac1c | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xbac28 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0xbac38 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0xbac40 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldm` at 0xbac4c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbac4c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbac4c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbac4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbaadc | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x4d` | `0x0` |
| `bl` at 0xbaae8 | `(unknown callee) 0x1967c` | `0x11ec94` | `0x1002` | `?` | `?` |
| `bl` at 0xbab00 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `?` | `ret(0x1967c)` |
| `bl` at 0xbab2c | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x53` | `0x0` |
| `bl` at 0xbab34 | `(unknown callee) 0x196b8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xbab5c | `(unknown callee) 0x193b8` | `0x0` | `0x100` | `0x3` | `0x1` |
| `bl` at 0xbab70 | `(unknown callee) 0x193d0` | `ret(0x193b8)` | `?` | `?` | `[0x29aeec]` |
| `bl` at 0xbab9c | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x5b` | `0x0` |
| `bl` at 0xbaba4 | `(unknown callee) 0x196b8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xbabc4 | `(unknown callee) 0x1967c` | `0x11ecec` | `0x2` | `0x29aeec` | `[0x29aeec]+36` |
| `bl` at 0xbabe0 | `(unknown callee) 0x193d0` | `ret(0x1967c)` | `?` | `ret(0x1967c)` | `[0x12d8dc]` |
| `bl` at 0xbabec | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbac10 | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x65` | `0x0` |
| `bl` at 0xbac30 | `(unknown callee) 0xc9238` | `?` | `?` | `?` | `?` |
| `bl` at 0xbac34 | `(unknown callee) 0xbb220` | `ret(0xc9238)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xbaac0 | `[0x12d8dc] < 0x0 (signed, 32-bit)` | `[0x12d8dc] >= 0x0 (signed, 32-bit)` |
| `bge` at 0xbaafc | `ret(0x1967c) >= 0x0 (signed, 32-bit)` | `ret(0x1967c) < 0x0 (signed, 32-bit)` |
| `bne` at 0xbab6c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xbabdc | `[0x12d8dc] >= 0x0 (signed, 32-bit)` | `[0x12d8dc] < 0x0 (signed, 32-bit)` |
| `bhi` at 0xbac24 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbac4c | `r0` | `?` |

## `i2cBus`

- `i2cBus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbac78 to 0xbaca4.
  Established. The byte at `sp -0x15` receives `(r1 & 0xff)` (`strb` at 0xbac90). That byte is read
  (`ldrb` at 0xbac94). The return reports `(r1 & 0xff)` (`ldm` at 0xbaca0). Bus identity
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbac88 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xbac90 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xbac94 | load | 1 | `sp -0x15` | `-` |
| `ldm` at 0xbaca0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbaca0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbaca0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbaca0 | `r0` | `(r1 & 0xff)` |

## `i2cSetBusSelect`

- `i2cSetBusSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaca4 to 0xbaccc.
  Established. The absolute word at `0x12d8e0` receives entry `r0` (`str` at 0xbacc0). The return
  reports entry `r0` (`ldm` at 0xbacc4). Stored selector semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbacb4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbacb8 | load | 4 | `abs 0xbacc8` | `-` |
| `ldr` at 0xbacbc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbacc0 | store | 4 | `abs 0x12d8e0` | `r0` |
| `ldm` at 0xbacc4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbacc4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbacc4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbacc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbacc4 | `r0` | `r0` |

## `i2cSetBusDeselect`

- `i2cSetBusDeselect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaccc to 0xbacf4.
  Established. Entry `r0` is assigned to the absolute word at `0x2972a4` (`str` at 0xbace8). The
  return reports that entry value (`ldm` at 0xbacec). The stored deselector role remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbacdc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbace0 | load | 4 | `abs 0xbacf0` | `-` |
| `ldr` at 0xbace4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbace8 | store | 4 | `abs 0x2972a4` | `r0` |
| `ldm` at 0xbacec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbacec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbacec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbacec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbacec | `r0` | `r0` |

## `i2cResetBusSelect`

- `i2cResetBusSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbacf4 to 0xbad5c.
  Established. The absolute word at `0x12d8e0` is read (`ldr` at 0xbad08); its zero value takes
  `0xbad44`, while nonzero falls through (`beq` at 0xbad10). The unnamed `0xbaf50` call carries
  `0x1`, entry `r1`, entry `r2`, `[0x12d8e0]` (`bl` at 0xbad18). Bus reset policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbad04 | load | 4 | `abs 0xbad58` | `-` |
| `ldr` at 0xbad08 | load | 4 | `abs 0x12d8e0` | `-` |
| `str` at 0xbad20 | store | 4 | `sp -0x14` | `ret(0xbaf50)` |
| `ldr` at 0xbad24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbad30 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbad34 | store | 4 | `sp -0x18` | `ret(0xbaf50)` |
| `str` at 0xbad48 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbad4c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbad54 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbad54 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbad54 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbad18 | `(unknown callee) 0xbaf50` | `0x1` | `r1` | `r2` | `[0x12d8e0]` |
| `bl` at 0xbad40 | `(unknown callee) 0xbb094` | `0x0` | `?` | `?` | `ret(0xbaf50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbad10 | `[0x12d8e0] == 0x0` | `[0x12d8e0] != 0x0` |
| `beq` at 0xbad2c | `ret(0xbaf50) == 0x0` | `ret(0xbaf50) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbad54 | `r0` | `?` |

## `i2cSetSwitch`

- `i2cSetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaeac to 0xbaf14.
  Established. The absolute word at `0x29aeec` is read (`ldr` at 0xbaef0). A word through
  `[0x29aeec] +0x0` receives `(r0 & 0xff)` (`str` at 0xbaef8). The return reports zero (`ldm` at 0xbaf08). Switch destination meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbaec0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0xbaec8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbaecc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaed0 | load | 4 | `abs 0xbaf0c` | `-` |
| `ldr` at 0xbaedc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbaee4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xbaeec | load | 4 | `abs 0xbaf10` | `-` |
| `ldr` at 0xbaef0 | load | 4 | `abs 0x29aeec` | `-` |
| `ldrb` at 0xbaef4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbaef8 | store | 4 | `[0x29aeec] +0x0` | `(r0 & 0xff)` |
| `ldm` at 0xbaf08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbaf08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbaf08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xbaed8 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbaf08 | `r0` | `0x0` |

## `i2cGetSwitch`

- `i2cGetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaf14 to 0xbaf50.
  Established. Included word reads use absolute `0x29aeec` and `[0x29aeec] +0x0` (`ldr` at 0xbaf30, `ldr` at 0xbaf34). A byte through entry `r0 +0x0` receives an unknown value (`strb` at 0xbaf3c).
  The return reports `0x0` (`ldm` at 0xbaf48). Switch readback interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbaf24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbaf28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbaf2c | load | 4 | `abs 0xbaf4c` | `-` |
| `ldr` at 0xbaf30 | load | 4 | `abs 0x29aeec` | `-` |
| `ldr` at 0xbaf34 | load | 4 | `[0x29aeec] +0x0` | `-` |
| `strb` at 0xbaf3c | store | 1 | `r0 +0x0` | `?` |
| `ldm` at 0xbaf48 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbaf48 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbaf48 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbaf48 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbaf48 | `r0` | `0x0` |

## `i2cLock`

- `i2cLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbaf50 to 0xbb094.
  Established. The absolute word at `0x2972a8` receives `ret(0x1958c)` (`str` at 0xbafc4). The
  indirect call has an unknown target and candidates `[sp-17]:1`, unknown, `[sp-17]:1`, `[0x12d8e0]`
  (`mov` at 0xbb01c). The byte at `0x2972ac` receives `([sp-17]:1 & 0xff)` (`strb` at 0xbb058). Lock
  ownership semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbaf64 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbaf68 | load | 4 | `abs 0xbb070` | `-` |
| `str` at 0xbaf74 | store | 4 | `sp -0x18` | `ret(0x194fc)` |
| `ldr` at 0xbaf78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbaf8c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbaf98 | load | 4 | `abs 0xbb074` | `-` |
| `str` at 0xbaf9c | store | 4 | `sp -0x24` | `0x11ed80` |
| `str` at 0xbafa0 | store | 4 | `sp -0x20` | `ret(0x1f8e0)` |
| `ldr` at 0xbafa8 | load | 4 | `abs 0xbb078` | `-` |
| `ldr` at 0xbafac | load | 4 | `abs 0xbb07c` | `-` |
| `ldr` at 0xbafc0 | load | 4 | `abs 0xbb080` | `-` |
| `str` at 0xbafc4 | store | 4 | `abs 0x2972a8` | `ret(0x1958c)` |
| `ldrb` at 0xbafc8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbafd4 | load | 4 | `abs 0xbb084` | `-` |
| `ldr` at 0xbafd8 | load | 4 | `abs 0x12d8e0` | `-` |
| `ldr` at 0xbafe4 | load | 4 | `abs 0xbb088` | `-` |
| `str` at 0xbafe8 | store | 4 | `sp -0x24` | `0x11ed9c` |
| `ldrb` at 0xbafec | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbaff0 | store | 4 | `sp -0x20` | `[sp-17]:1` |
| `ldr` at 0xbaff8 | load | 4 | `abs 0xbb078` | `-` |
| `ldr` at 0xbaffc | load | 4 | `abs 0xbb08c` | `-` |
| `ldr` at 0xbb008 | load | 4 | `abs 0xbb084` | `-` |
| `ldrb` at 0xbb00c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbb010 | load | 4 | `abs 0x12d8e0` | `-` |
| `str` at 0xbb024 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xbb028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb040 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `str` at 0xbb048 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xbb050 | load | 4 | `abs 0xbb090` | `-` |
| `ldrb` at 0xbb054 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xbb058 | store | 1 | `abs 0x2972ac` | `([sp-17]:1 & 0xff)` |
| `str` at 0xbb060 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xbb064 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xbb06c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb06c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb06c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbaf6c | `(unknown callee) 0x194fc` | `0x29728c` | `r1` | `r2` | `r0` |
| `bl` at 0xbaf84 | `(unknown callee) 0x193d0` | `ret(0x194fc)` | `?` | `?` | `ret(0x194fc)` |
| `bl` at 0xbaf90 | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbafb4 | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x141` | `0x0` |
| `bl` at 0xbafb8 | `(unknown callee) 0x1958c` | `?` | `?` | `?` | `?` |
| `bl` at 0xbb004 | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x14a` | `0x0` |
| `mov` at 0xbb01c | `(unknown callee) (unknown target)` | `[sp-17]:1` | `?` | `[sp-17]:1` | `[0x12d8e0]` |
| `bl` at 0xbb034 | `(unknown callee) 0xbb094` | `ret(unknown)` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xbb038 | `(unknown callee) 0x193d0` | `ret(0xbb094)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xbaf80 | `ret(0x194fc) >= 0x0 (signed, 32-bit)` | `ret(0x194fc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xbafd0 | `[sp-17]:1 == 0x0` | `[sp-17]:1 != 0x0` |
| `bne` at 0xbafe0 | `[0x12d8e0] != 0x0` | `[0x12d8e0] == 0x0` |
| `beq` at 0xbb030 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb06c | `r0` | `?` |

## `i2cUnlock`

- `i2cUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb094 to 0xbb18c.
  Established. The absolute word at `0x2972a8` receives zero (`str` at 0xbb0ac). A zero byte
  `[0x2972ac]:1` takes `0xbb114`, while nonzero falls through (`beq` at 0xbb0bc). The byte at
  `0x2972ac` is assigned zero on an included path (`strb` at 0xbb110). Release side effects remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbb0a4 | load | 4 | `abs 0xbb16c` | `-` |
| `str` at 0xbb0ac | store | 4 | `abs 0x2972a8` | `0x0` |
| `ldr` at 0xbb0b0 | load | 4 | `abs 0xbb170` | `-` |
| `ldrb` at 0xbb0b4 | load | 1 | `abs 0x2972ac` | `-` |
| `ldr` at 0xbb0c0 | load | 4 | `abs 0xbb174` | `-` |
| `ldr` at 0xbb0c4 | load | 4 | `abs 0x2972a4` | `-` |
| `ldr` at 0xbb0d0 | load | 4 | `abs 0xbb174` | `-` |
| `ldr` at 0xbb0d4 | load | 4 | `abs 0xbb170` | `-` |
| `ldrb` at 0xbb0d8 | load | 1 | `abs 0x2972ac` | `-` |
| `ldr` at 0xbb0dc | load | 4 | `abs 0x2972a4` | `-` |
| `str` at 0xbb0f0 | store | 4 | `sp -0x14` | `ret(unknown)` |
| `ldr` at 0xbb0f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb100 | load | 4 | `abs 0xbb178` | `-` |
| `ldr` at 0xbb108 | load | 4 | `abs 0xbb170` | `-` |
| `strb` at 0xbb110 | store | 1 | `abs 0x2972ac` | `0x0` |
| `ldr` at 0xbb114 | load | 4 | `abs 0xbb17c` | `-` |
| `str` at 0xbb120 | store | 4 | `sp -0x14` | `ret(0x19778)` |
| `ldr` at 0xbb124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb138 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbb144 | load | 4 | `abs 0xbb180` | `-` |
| `str` at 0xbb148 | store | 4 | `sp -0x1c` | `0x11edcc` |
| `str` at 0xbb14c | store | 4 | `sp -0x18` | `ret(0x1f8e0)` |
| `ldr` at 0xbb154 | load | 4 | `abs 0xbb184` | `-` |
| `ldr` at 0xbb158 | load | 4 | `abs 0xbb188` | `-` |
| `ldm` at 0xbb168 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb168 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb168 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbb0e8 | `(unknown callee) (unknown target)` | `[0x2972ac]:1` | `r1` | `[0x2972a4]` | `[0x2972ac]:1` |
| `bl` at 0xbb104 | `(unknown callee) 0x1e4ac` | `0x11edb4` | `?` | `?` | `ret(unknown)` |
| `bl` at 0xbb118 | `(unknown callee) 0x19778` | `0x29728c` | `?` | `?` | `?` |
| `bl` at 0xbb130 | `(unknown callee) 0x193d0` | `ret(0x19778)` | `?` | `?` | `ret(0x19778)` |
| `bl` at 0xbb13c | `(unknown callee) 0x1f8e0` | `[ret(0x193d0)]` | `?` | `?` | `ret(0x193d0)` |
| `bl` at 0xbb160 | `(unknown callee) 0x200a0` | `0x9` | `0x11ec8c` | `0x175` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb0bc | `[0x2972ac]:1 == 0x0` | `[0x2972ac]:1 != 0x0` |
| `beq` at 0xbb0cc | `[0x2972a4] == 0x0` | `[0x2972a4] != 0x0` |
| `beq` at 0xbb0fc | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `bge` at 0xbb12c | `ret(0x19778) >= 0x0 (signed, 32-bit)` | `ret(0x19778) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb168 | `r0` | `?` |

## `i2cSelectBus`

- `i2cSelectBus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb18c to 0xbb220.
  Established. A zero absolute word `[0x12d8e0]` takes `0xbb1d8`, while nonzero falls through (`beq` at 0xbb1b0). The indirect call has an unknown target and candidates `(r0 & 0xff)`, entry `r1`,
  `(r0 & 0xff)`, `[0x12d8e0]` (`mov` at 0xbb1c8). The word at `sp -0x18` receives `ret(unknown)`
  (`str` at 0xbb1d0). Bus selection effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbb1a0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xbb1a4 | load | 4 | `abs 0xbb210` | `-` |
| `ldr` at 0xbb1a8 | load | 4 | `abs 0x12d8e0` | `-` |
| `ldr` at 0xbb1b4 | load | 4 | `abs 0xbb210` | `-` |
| `ldrb` at 0xbb1b8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xbb1bc | load | 4 | `abs 0x12d8e0` | `-` |
| `str` at 0xbb1d0 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldr` at 0xbb1d8 | load | 4 | `abs 0xbb214` | `-` |
| `str` at 0xbb1dc | store | 4 | `sp -0x20` | `0x11ede8` |
| `ldrb` at 0xbb1e0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb1e4 | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0xbb1ec | load | 4 | `abs 0xbb218` | `-` |
| `ldr` at 0xbb1f0 | load | 4 | `abs 0xbb21c` | `-` |
| `str` at 0xbb200 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xbb204 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbb20c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb20c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb20c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbb1c8 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x12d8e0]` |
| `bl` at 0xbb1f8 | `(unknown callee) 0x200a0` | `0x1` | `0x11ec8c` | `0x187` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbb1b0 | `[0x12d8e0] == 0x0` | `[0x12d8e0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb20c | `r0` | `?` |

## `i2cReset`

- `i2cReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb220 to 0xbb2a8.
  Established. The unnamed `0x19718` call carries `[0x12d8dc]`, `0x782`, `0x0`, `0x12d8dc` (`bl` at 0xbb254); its nonnegative signed result takes `0xbb284`, while a negative result falls through
  (`bge` at 0xbb260). The word at `sp -0x14` receives `[sp-20]+1` (`str` at 0xbb28c). Reset request
  interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb234 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xbb238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbb244 | load | 4 | `abs 0xbb298` | `-` |
| `ldr` at 0xbb248 | load | 4 | `abs 0x12d8dc` | `-` |
| `ldr` at 0xbb24c | load | 4 | `abs 0xbb29c` | `-` |
| `ldr` at 0xbb270 | load | 4 | `abs 0xbb2a0` | `-` |
| `ldr` at 0xbb274 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xbb278 | load | 4 | `abs 0xbb2a4` | `-` |
| `ldr` at 0xbb27c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xbb284 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbb28c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0xbb294 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbb294 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb294 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb294 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb254 | `(unknown callee) 0x19718` | `[0x12d8dc]` | `0x782` | `0x0` | `0x12d8dc` |
| `bl` at 0xbb264 | `(unknown callee) 0x193d0` | `ret(0x19718)` | `?` | `?` | `ret(0x19718)` |
| `bl` at 0xbb280 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x11ee04` | `[ret(0x193d0)]` | `0x12eeb8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xbb240 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0xbb260 | `ret(0x19718) >= 0x0 (signed, 32-bit)` | `ret(0x19718) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb294 | `r0` | `?` |

## `i2cWrite`

- `i2cWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb2a8 to 0xbb2f8.
  Established. The halfword at `sp -0x1a` receives `(r2 & 0xffff)` (`strh` at 0xbb2c4). A word at
  `sp -0x20` receives that expression (`str` at 0xbb2d0). The unnamed `0xbb65c` call carries `0x0`,
  `(r0 & 0xff)`, unknown, `[sp-24]` (`bl` at 0xbb2e4). Transfer behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb2bc | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbb2c0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xbb2c4 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xbb2c8 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xbb2cc | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xbb2d0 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xbb2e0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbb2f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb2f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb2f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb2e4 | `(unknown callee) 0xbb65c` | `0x0` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb2f4 | `r0` | `ret(0xbb65c)` |

## `i2cWriteOffset`

- `i2cWriteOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb2f8 to 0xbb358.
  Established. Included byte assignments at `sp -0x11` and `sp -0x12` write `(r0 & 0xff)` and `(r1 & 0xff)` respectively (`strb` at 0xbb314, `strb` at 0xbb31c).
  The word at `sp -0x20` receives `(r3 & 0xffff)` (`str` at 0xbb330). The unnamed `0xbb65c` call carries `0x0`, `(r0 & 0xff)`, `(r1 & 0xff)`, entry `r2` (`bl` at 0xbb344). Offset transfer semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb308 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xbb314 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb31c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xbb320 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xbb324 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbb328 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xbb32c | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xbb330 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xbb340 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbb354 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb354 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb354 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb344 | `(unknown callee) 0xbb65c` | `0x0` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb354 | `r0` | `ret(0xbb65c)` |

## `i2cRead`

- `i2cRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb358 to 0xbb3a8.
  Established. A word at `sp -0x20` receives `(r2 & 0xffff)` (`str` at 0xbb380). The unnamed
  `0xbb65c` call carries `0x1`, `(r0 & 0xff)`, unknown, `[sp-24]` (`bl` at 0xbb394). The return
  reports `ret(0xbb65c)` (`ldm` at 0xbb3a4). Read operation effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb36c | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xbb370 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0xbb374 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldrb` at 0xbb378 | load | 1 | `sp -0x11` | `-` |
| `ldrh` at 0xbb37c | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xbb380 | store | 4 | `sp -0x20` | `(r2 & 0xffff)` |
| `ldr` at 0xbb390 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbb3a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb3a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb3a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb394 | `(unknown callee) 0xbb65c` | `0x1` | `(r0 & 0xff)` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb3a4 | `r0` | `ret(0xbb65c)` |

## `i2cReadOffset`

- `i2cReadOffset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb3a8 to 0xbb408.
  Established. The halfword at `sp -0x1a` receives `(r3 & 0xffff)` (`strh` at 0xbb3d0). The unnamed
  `0xbb65c` call carries `0x1`, `(r0 & 0xff)`, `(r1 & 0xff)`, entry `r2` (`bl` at 0xbb3f4); its
  result is reported by the return (`ldm` at 0xbb404). Offset read effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbb3b8 | store | 4 | `sp -0x18` | `r2` |
| `strb` at 0xbb3c4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0xbb3cc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0xbb3d0 | store | 2 | `sp -0x1a` | `(r3 & 0xffff)` |
| `ldrb` at 0xbb3d4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xbb3d8 | load | 1 | `sp -0x12` | `-` |
| `ldrh` at 0xbb3dc | load | 2 | `sp -0x1a` | `-` |
| `str` at 0xbb3e0 | store | 4 | `sp -0x20` | `(r3 & 0xffff)` |
| `ldr` at 0xbb3f0 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xbb404 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb404 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb404 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb3f4 | `(unknown callee) 0xbb65c` | `0x1` | `(r0 & 0xff)` | `(r1 & 0xff)` | `r2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb404 | `r0` | `ret(0xbb65c)` |
