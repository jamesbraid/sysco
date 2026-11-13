# ARM Linux 2.6 I/O expander interrupts and transfers in 1.44.0

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
Computed transfers in ioExpInt, ioExpWriteExp and ioExpReadExp leave unresolved edges.
Their tables cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-expander-interrupt-transfers-facts.txt) retain all tool sections.

## `ioExpGetBit`

- `ioExpGetBit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125b08 to 0x125b8c.
  Established. The sole call to unnamed 0x125b8c has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/sp-29/(entry r1 & 0xff) (`bl` at 0x125b3c).
  A zero call result takes the branch to 0x125b60, while the other path saves ret(0x125b8c) at sp-0x24 (`beq` at 0x125b50, `str` at 0x125b58).
  The later store at sp-0x24 and the recorded r0 return are unknown after the merged predicated moves, leaving the result's bit interpretation unestablished (`moveq` at 0x125b74, `movne` at 0x125b78, `str` at 0x125b7c, `ldm` at 0x125b88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125b18 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125b20 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125b28 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x125b2c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125b34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125b44 | store | 4 | `sp -0x1c` | `ret(0x125b8c)` |
| `ldr` at 0x125b48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125b54 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x125b58 | store | 4 | `sp -0x24` | `ret(0x125b8c)` |
| `ldrb` at 0x125b60 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x125b64 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x125b7c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x125b80 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x125b88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125b88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125b88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125b3c | `(unknown callee) 0x125b8c` | `r0` | `(r1 & 0xff)` | `sp-29` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125b50 | `ret(0x125b8c) == 0x0` | `ret(0x125b8c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125b88 | `r0` | `?` |

## `ioExpRead`

- `ioExpRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125b8c to 0x125c20.
  Established. A byte load reads ret(0x126748), and a nonzero loaded byte takes the branch to 0x125bf8 (`ldrb` at 0x125be0, `bne` at 0x125be8);
  its other path stores 0xfffffc11 at sp-0x28 (`str` at 0x125bf0).
  The call to unnamed 0x126e4c has candidate r0/r1/r2/r3 values [sp-20]/[sp-32]/[sp-28]/[ret(0x126748)]:1 (`bl` at 0x125c04).
  The byte and call arguments do not identify a physical transfer target.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125b9c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x125ba4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x125ba8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x125bac | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125bb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125bc0 | store | 4 | `sp -0x20` | `ret(0x126630)` |
| `ldrb` at 0x125bc4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125bc8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125bd8 | store | 4 | `sp -0x24` | `ret(0x126748)` |
| `ldr` at 0x125bdc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x125be0 | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125bec | load | 4 | `abs 0x125c1c` | `-` |
| `str` at 0x125bf0 | store | 4 | `sp -0x28` | `0xfffffc11` |
| `ldr` at 0x125bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125bfc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x125c00 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x125c0c | store | 4 | `sp -0x28` | `ret(0x126e4c)` |
| `ldr` at 0x125c10 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x125c18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125c18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125c18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125bb8 | `(unknown callee) 0x126630` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0x125bd0 | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x125c04 | `(unknown callee) 0x126e4c` | `[sp-20]` | `[sp-32]` | `[sp-28]` | `[ret(0x126748)]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x125be8 | `[ret(0x126748)]:1 != 0x0` | `[ret(0x126748)]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125c18 | `r0` | `?` |

## `ioExpWriteValue`

- `ioExpWriteValue`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125c20 to 0x125c68.
  Established. A byte store saves (entry r1 & 0xff) at sp-0x15 (`strb` at 0x125c38).
  The call to unnamed 0x126748 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/entry r2/(entry r1 & 0xff) (`bl` at 0x125c48).
  A byte load reads ret(0x126748) +1, and the recorded r0 return is [ret(0x126748)+1]:1, whose field meaning remains Unknown (`ldrb` at 0x125c58, `ldm` at 0x125c64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125c30 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125c38 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x125c3c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125c40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125c50 | store | 4 | `sp -0x1c` | `ret(0x126748)` |
| `ldr` at 0x125c54 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125c58 | load | 1 | `ret(0x126748) +0x1` | `-` |
| `ldm` at 0x125c64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125c64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125c64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125c48 | `(unknown callee) 0x126748` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125c64 | `r0` | `[ret(0x126748)+1]:1` |

## `ioExpIntCount`

- `ioExpIntCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125c68 to 0x125cb0.
  Established. The sole call to unnamed 0x126748 has candidate r1=(entry r1 & 0xff), with entry r0/r2 and the same masked r1 value as r3 candidates (`bl` at 0x125c90).
  A word store saves its result at sp-0x1c, followed by a word load from ret(0x126748) +8 (`str` at 0x125c98, `ldr` at 0x125ca0).
  The recorded r0 return is [ret(0x126748)+8], but the body does not establish what that word counts (`ldm` at 0x125cac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125c78 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125c80 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x125c84 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125c88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125c98 | store | 4 | `sp -0x1c` | `ret(0x126748)` |
| `ldr` at 0x125c9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125ca0 | load | 4 | `ret(0x126748) +0x8` | `-` |
| `ldm` at 0x125cac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125cac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125cac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125c90 | `(unknown callee) 0x126748` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125cac | `r0` | `[ret(0x126748)+8]` |

## `ioExpIntMask`

- `ioExpIntMask`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125cb0 to 0x125cf8.
  Established. A word store saves entry r0 at sp-0x14 and a byte store saves (entry r1 & 0xff) at sp-0x15 (`str` at 0x125cc0, `strb` at 0x125cc8).
  The call to unnamed 0x126748 supplies the address for a byte load at ret(0x126748) +3 (`bl` at 0x125cd8, `ldrb` at 0x125ce8).
  The recorded r0 return is [ret(0x126748)+3]:1, with no established physical mask assignment (`ldm` at 0x125cf4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125cc0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125cc8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x125ccc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x125cd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125ce0 | store | 4 | `sp -0x1c` | `ret(0x126748)` |
| `ldr` at 0x125ce4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125ce8 | load | 1 | `ret(0x126748) +0x3` | `-` |
| `ldm` at 0x125cf4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125cf4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125cf4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125cd8 | `(unknown callee) 0x126748` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125cf4 | `r0` | `[ret(0x126748)+3]:1` |

## `ioExpIntDisable`

- `ioExpIntDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125cf8 to 0x125d44.
  Established. Byte stores save masked entry r1 and r2 at sp-0x15 and sp-0x16 (`strb` at 0x125d10, `strb` at 0x125d18).
  The sole call to unnamed 0x127058 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/(entry r2 & 0xff)/0 (`bl` at 0x125d30);
  its result is the recorded r0 return, while the physical meaning of the zero candidate remains Unknown (`ldm` at 0x125d40).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125d08 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125d10 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125d18 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x125d1c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x125d20 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x125d24 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x125d40 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125d40 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125d40 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125d30 | `(unknown callee) 0x127058` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125d40 | `r0` | `ret(0x127058)` |

## `ioExpIntEnable`

- `ioExpIntEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125d44 to 0x125d90.
  Established. A word store saves entry r0 at sp-0x14 (`str` at 0x125d54).
  The call to unnamed 0x127058 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/(entry r2 & 0xff)/1 (`bl` at 0x125d7c).
  The recorded r0 return is ret(0x127058), so this wrapper alone leaves any enable operation unestablished (`ldm` at 0x125d8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125d54 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125d5c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125d64 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x125d68 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x125d6c | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x125d70 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x125d8c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125d8c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125d8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125d7c | `(unknown callee) 0x127058` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125d8c | `r0` | `ret(0x127058)` |

## `ioExpIntSet`

- `ioExpIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125d90 to 0x125de0.
  Established. Word stores save entry r0 and r3 at sp-0x14 and sp-0x1c (`str` at 0x125da0, `str` at 0x125da4).
  The sole call to unnamed 0x127058 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/(entry r2 & 0xff)/entry r3 (`bl` at 0x125dcc).
  The recorded r0 return is that call result, and no physical interrupt-field assignment follows from these candidates (`ldm` at 0x125ddc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125da0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x125da4 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0x125dac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125db4 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0x125db8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x125dbc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x125dc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125dc8 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x125ddc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125ddc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125ddc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125dcc | `(unknown callee) 0x127058` | `r0` | `(r1 & 0xff)` | `(r2 & 0xff)` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125ddc | `r0` | `ret(0x127058)` |

## `ioExpInt`

- `ioExpInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125de0 to 0x125f90.
  Established. Known-edge byte loads read ret(0x126748) and ret(0x126748) +3 (`ldrb` at 0x125e58, `ldrb` at 0x125e68).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 1/0x18fb58/0x301/0 (`bl` at 0x125f60).
  A byte store writes ([sp-33]:1+1 & 0xff) at sp-0x21 before the backward transfer to 0x125e14 (`strb` at 0x125f6c, `b` at 0x125f70).
  The computed transfer is unresolved (`ldrls` at 0x125e80), so omitted effects and any physical interrupt interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125df0 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x125df8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125e00 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `str` at 0x125e08 | store | 4 | `sp -0x28` | `0x0` |
| `ldrb` at 0x125e0c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x125e10 | store | 1 | `sp -0x21` | `((r1 & 0xff) & 0xff)` |
| `ldrb` at 0x125e14 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x125e18 | load | 1 | `sp -0x16` | `-` |
| `ldrb` at 0x125e24 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x125e28 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125e38 | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x125e3c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x125e40 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x125e50 | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x125e54 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x125e58 | load | 1 | `ret(0x126748) +0x0` | `-` |
| `ldr` at 0x125e64 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x125e68 | load | 1 | `ret(0x126748) +0x3` | `-` |
| `ldr` at 0x125e74 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x125e78 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0x125e80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x125f3c | load | 4 | `abs 0x125f84` | `-` |
| `str` at 0x125f40 | store | 4 | `sp -0x30` | `0x18fce8` |
| `ldr` at 0x125f44 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x125f48 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x125f4c | store | 4 | `sp -0x2c` | `[[sp-28]+12]` |
| `ldr` at 0x125f54 | load | 4 | `abs 0x125f88` | `-` |
| `ldr` at 0x125f58 | load | 4 | `abs 0x125f8c` | `-` |
| `ldrb` at 0x125f64 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x125f6c | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x125f74 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x125f80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x125f80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x125f80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x125e30 | `(unknown callee) 0x126630` | `?` | `?` | `?` | `?` |
| `bl` at 0x125e48 | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0x125f60 | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x301` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhi` at 0x125e20 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x125e60 | `[ret(0x126748)]:1 == 0x0` | `[ret(0x126748)]:1 != 0x0` |
| `beq` at 0x125e70 | `[ret(0x126748)+3]:1 == 0x0` | `[ret(0x126748)+3]:1 != 0x0` |
| `ldrls` at 0x125e80 | `[[sp-28]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-28]]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x125f80 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x125e80).

## `ioExpPortInt`

- `ioExpPortInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x125f90 to 0x1261c4.
  Established. Byte loads read [sp-28] +1, +2 and +3 (`ldrb` at 0x126088, `ldrb` at 0x12609c, `ldrb` at 0x1260b0).
  The first comparison branches to 0x1260e8 when [[sp-28]+1]:1 is not equal to [sp-21]:1, and falls through on equality (`bne` at 0x126094).
  The call to unnamed 0x1261c4 has candidate r0/r1/r2/r3 values [sp-20]/[sp-28]/ret(0x126748)/[sp-33]:1 (`bl` at 0x1260d8).
  The compared software fields remain unidentified as physical ports or interrupts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x125fa0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x125fa8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x125fb0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x125fb8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x125fc0 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0x125fc8 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldr` at 0x125fcc | load | 4 | `abs 0x1261b0` | `-` |
| `ldr` at 0x125fd0 | load | 4 | `abs 0x314fd8` | `-` |
| `ldr` at 0x125fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x125ff0 | load | 4 | `abs 0x1261b4` | `-` |
| `ldr` at 0x125ff4 | load | 4 | `abs 0x314fd4` | `-` |
| `ldr` at 0x125ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x126008 | load | 4 | `abs 0x1261b8` | `-` |
| `str` at 0x12600c | store | 4 | `sp -0x38` | `0xfffffc11` |
| `ldr` at 0x126014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x126028 | load | 4 | `abs 0x1261b4` | `-` |
| `ldr` at 0x12602c | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x126034 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0x12603c | store | 1 | `sp -0x21` | `0x0` |
| `ldrb` at 0x126040 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x126044 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x126048 | load | 4 | `[sp-40] +0x4` | `-` |
| `ldrb` at 0x126054 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x126058 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126068 | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldrb` at 0x12606c | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x126070 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126080 | store | 4 | `sp -0x20` | `ret(0x126748)` |
| `ldr` at 0x126084 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x126088 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0x12608c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x126098 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12609c | load | 1 | `[sp-28] +0x2` | `-` |
| `ldrb` at 0x1260a0 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x1260ac | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1260b0 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0x1260b4 | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x1260c0 | load | 1 | `sp -0x21` | `-` |
| `ldrb` at 0x1260c4 | load | 1 | `sp -0x18` | `-` |
| `str` at 0x1260c8 | store | 4 | `sp -0x4c` | `[sp-24]:1` |
| `ldr` at 0x1260cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1260d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1260d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1260e0 | store | 4 | `sp -0x38` | `ret(0x1261c4)` |
| `ldrb` at 0x1260e8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x1260f0 | store | 1 | `sp -0x21` | `([sp-33]:1+1 & 0xff)` |
| `ldr` at 0x1260f8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12610c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x126110 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x126120 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x126124 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12614c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x126150 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x126154 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x126158 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x126164 | load | 4 | `abs 0x1261bc` | `-` |
| `str` at 0x126168 | store | 4 | `(unknown)` | `0x18fd14` |
| `str` at 0x12616c | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x126170 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x126174 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0x126178 | load | 1 | `sp -0x16` | `-` |
| `str` at 0x12617c | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0x126180 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x126184 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0x12618c | load | 4 | `abs 0x1261c0` | `-` |
| `str` at 0x1261a0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x1261a4 | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x1261ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1261ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1261ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x126060 | `(unknown callee) 0x126630` | `?` | `?` | `[[sp-40]+4]` | `?` |
| `bl` at 0x126078 | `(unknown callee) 0x126748` | `[sp-20]` | `[sp-33]:1` | `?` | `[sp-33]:1` |
| `bl` at 0x1260d8 | `(unknown callee) 0x1261c4` | `[sp-20]` | `[sp-28]` | `ret(0x126748)` | `[sp-33]:1` |
| `bl` at 0x126108 | `(unknown callee) 0x6e1b0` | `?` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126114 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x12615c | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x126198 | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x324` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x125fd8 | `[0x314fd8] == 0x0` | `[0x314fd8] != 0x0` |
| `bne` at 0x126004 | `? != 0x0` | `? == 0x0` |
| `bge` at 0x126050 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x126094 | `[[sp-28]+1]:1 != [sp-21]:1` | `[[sp-28]+1]:1 == [sp-21]:1` |
| `bne` at 0x1260a8 | `[[sp-28]+2]:1 != [sp-22]:1` | `[[sp-28]+2]:1 == [sp-22]:1` |
| `bne` at 0x1260bc | `[[sp-28]+3]:1 != [sp-23]:1` | `[[sp-28]+3]:1 == [sp-23]:1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1261ac | `r0` | `?` |

## `ioExpIntHandler`

- `ioExpIntHandler`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1261c4 to 0x1264dc.
  Established. A word store increments [sp-32] +8 using [[sp-32]+8]+1, and a separate byte store writes ([r2+5]:1-1 & 0xff) at entry r2 +5 (`str` at 0x12630c, `strb` at 0x126390).
  The unknown-target call has candidate r0/r1/r2/r3 values [sp-24]/[sp-33]:1/unknown/[[sp-32]+2]:1 (`mov` at 0x126344).
  A later call to unnamed 0x125cf8 has candidate r0/r1/r2 values [sp-24]/[sp-33]:1/[sp-68] (`bl` at 0x1264b0).
  These accesses do not establish counter, retry or physical interrupt meanings for the fields.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1261d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x1261d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x1261dc | store | 4 | `sp -0x20` | `r2` |
| `ldr` at 0x1261e0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x1261e4 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `strb` at 0x1261ec | store | 1 | `sp -0x22` | `([sp] & 0xff)` |
| `ldr` at 0x1261f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1261f4 | load | 4 | `r1 +0x8` | `-` |
| `ldr` at 0x126200 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126204 | load | 1 | `r2 +0x3` | `-` |
| `ldrb` at 0x126208 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x12621c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126224 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x126228 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x126238 | store | 4 | `sp -0x28` | `ret(0x126e4c)` |
| `ldr` at 0x12623c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x126248 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12625c | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x126260 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x126270 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x126274 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12629c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x1262a0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1262a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1262a8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1262b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1262c0 | load | 4 | `abs 0x1264c8` | `-` |
| `str` at 0x1262c4 | store | 4 | `(unknown)` | `0x18fd4c` |
| `str` at 0x1262c8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x1262cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1262d0 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x1262d4 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `str` at 0x1262d8 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x1262e0 | load | 4 | `abs 0x1264cc` | `-` |
| `str` at 0x1262f4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x1262fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126300 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126304 | load | 4 | `[sp-32] +0x8` | `-` |
| `str` at 0x12630c | store | 4 | `[sp-32] +0x8` | `[[sp-32]+8]+1` |
| `ldr` at 0x126310 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x126314 | load | 1 | `sp -0x21` | `-` |
| `ldr` at 0x126318 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x12631c | load | 1 | `[sp-32] +0x3` | `-` |
| `ldrb` at 0x126320 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x12632c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126330 | load | 1 | `[sp-32] +0x2` | `-` |
| `ldr` at 0x126334 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x126338 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12634c | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x126354 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126358 | load | 1 | `r2 +0x4` | `-` |
| `ldrb` at 0x12635c | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x126370 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126374 | load | 1 | `r2 +0x5` | `-` |
| `ldr` at 0x126380 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126384 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126388 | load | 1 | `r2 +0x5` | `-` |
| `strb` at 0x126390 | store | 1 | `r2 +0x5` | `([r2+5]:1-1 & 0xff)` |
| `str` at 0x126398 | store | 4 | `sp -0x38` | `0x1` |
| `ldr` at 0x1263a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1263b4 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x1263b8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1263c8 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x1263cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1263f4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1263f8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1263fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x126400 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12640c | load | 4 | `abs 0x1264d0` | `-` |
| `str` at 0x126410 | store | 4 | `(unknown)` | `0x18fd70` |
| `str` at 0x126414 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x126418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12641c | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x126420 | store | 4 | `(unknown)` | `[[sp-28]+12]` |
| `ldrb` at 0x126424 | load | 1 | `sp -0x22` | `-` |
| `str` at 0x126428 | store | 4 | `(unknown)` | `[sp-34]:1` |
| `ldr` at 0x12642c | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x126430 | load | 1 | `[sp-32] +0x3` | `-` |
| `str` at 0x126434 | store | 4 | `(unknown)` | `[[sp-32]+3]:1` |
| `ldr` at 0x126438 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12643c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x126448 | load | 4 | `abs 0x1264d4` | `-` |
| `str` at 0x12644c | store | 4 | `sp -0x3c` | `0x18fdb4` |
| `ldr` at 0x126454 | load | 4 | `abs 0x1264d8` | `-` |
| `str` at 0x126458 | store | 4 | `sp -0x3c` | `0x18fdb8` |
| `ldr` at 0x12645c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x126460 | store | 4 | `(unknown)` | `[sp-60]` |
| `ldr` at 0x126468 | load | 4 | `abs 0x1264cc` | `-` |
| `ldrb` at 0x126478 | load | 1 | `sp -0x21` | `-` |
| `str` at 0x12647c | store | 4 | `sp -0x40` | `[sp-33]:1` |
| `ldr` at 0x126480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126484 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldrb` at 0x126490 | load | 1 | `sp -0x22` | `-` |
| `str` at 0x126494 | store | 4 | `sp -0x44` | `[sp-34]:1` |
| `str` at 0x1264a0 | store | 4 | `sp -0x44` | `0xff` |
| `ldr` at 0x1264a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1264a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1264ac | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1264b8 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x1264bc | load | 4 | `sp -0x38` | `-` |
| `ldm` at 0x1264c4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1264c4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1264c4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1264c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x126230 | `(unknown callee) 0x126e4c` | `r0` | `r1` | `r2+2` | `r2+2` |
| `bl` at 0x126258 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126264 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1262ac | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x1262b8 | `(unknown callee) 0x21ce0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x1262ec | `(unknown callee) 0x22070` | `0x3` | `0x18fb58` | `0x338` | `0x0` |
| `mov` at 0x126344 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-33]:1` | `?` | `[[sp-32]+2]:1` |
| `bl` at 0x1263b0 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1263bc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126404 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0x126474 | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x354` | `0x0` |
| `bl` at 0x1264b0 | `(unknown callee) 0x125cf8` | `[sp-24]` | `[sp-33]:1` | `[sp-68]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1261fc | `[r1+8] == 0x0` | `[r1+8] != 0x0` |
| `beq` at 0x126218 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x126244 | `ret(0x126e4c) == 0x0` | `ret(0x126e4c) != 0x0` |
| `beq` at 0x12636c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12637c | `[r2+5]:1 == 0x0` | `[r2+5]:1 != 0x0` |
| `beq` at 0x126444 | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |
| `beq` at 0x12648c | `[[sp-28]+8] == 0x0` | `[[sp-28]+8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1264c4 | `r0` | `?` |

## `ioExpIgnoreInt`

- `ioExpIgnoreInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1264dc to 0x1265c4.
  Established. A word store writes ret(0x2031c)+1 at sp-0x20, while a later word store has unknown effective address and carries 0x18fdc8 (`str` at 0x126534, `str` at 0x12657c).
  The call to unnamed 0x203f4 has candidate r0/r1/r2/r3 values [sp-36]/[sp-28]/[sp-32]/unknown (`bl` at 0x126570).
  The final call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 6/0x18fb58/0x367/0 and supplies the recorded r0 return (`bl` at 0x1265ac, `ldm` at 0x1265b4).
  No physical ignore operation is established by those calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1264ec | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1264f8 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x126500 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x126508 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `ldr` at 0x12650c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126520 | store | 4 | `sp -0x1c` | `ret(0x6e1b0)` |
| `ldr` at 0x126524 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x126534 | store | 4 | `sp -0x20` | `ret(0x2031c)+1` |
| `ldr` at 0x126538 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x126560 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x126564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x126568 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12656c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126578 | load | 4 | `abs 0x1265b8` | `-` |
| `str` at 0x12657c | store | 4 | `(unknown)` | `0x18fdc8` |
| `str` at 0x126580 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x126584 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x126588 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldrb` at 0x12658c | load | 1 | `sp -0x16` | `-` |
| `str` at 0x126590 | store | 4 | `(unknown)` | `[sp-22]:1` |
| `ldrb` at 0x126594 | load | 1 | `sp -0x17` | `-` |
| `str` at 0x126598 | store | 4 | `(unknown)` | `[sp-23]:1` |
| `ldr` at 0x1265a0 | load | 4 | `abs 0x1265bc` | `-` |
| `ldr` at 0x1265a4 | load | 4 | `abs 0x1265c0` | `-` |
| `ldm` at 0x1265b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1265b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1265b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12651c | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126528 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126570 | `(unknown callee) 0x203f4` | `[sp-36]` | `[sp-28]` | `[sp-32]` | `?` |
| `bl` at 0x1265ac | `(unknown callee) 0x22070` | `0x6` | `0x18fb58` | `0x367` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1265b4 | `r0` | `ret(0x22070)` |

## `ioExpName`

- `ioExpName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1265c4 to 0x126630.
  Established. The call to unnamed 0x126630 has candidate r0/r1/r2/r3 values entry r0/(entry r1 & 0xff)/entry r2/(entry r1 & 0xff) (`bl` at 0x1265ec).
  A word load reads ret(0x126630) +0xc (`ldr` at 0x1265fc).
  The branch targets 0x12661c when [ret(0x126630)+12] is nonzero, while the other path stores 0x18fe08 at sp-0x20 (`bne` at 0x126610, `str` at 0x126618).
  The selected word's content and object identity remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1265d4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x1265dc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0x1265e0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x1265e4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1265f4 | store | 4 | `sp -0x1c` | `ret(0x126630)` |
| `ldr` at 0x1265f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1265fc | load | 4 | `ret(0x126630) +0xc` | `-` |
| `str` at 0x126600 | store | 4 | `sp -0x20` | `[ret(0x126630)+12]` |
| `ldr` at 0x126604 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126608 | load | 4 | `ret(0x126630) +0xc` | `-` |
| `ldr` at 0x126614 | load | 4 | `abs 0x12662c` | `-` |
| `str` at 0x126618 | store | 4 | `sp -0x20` | `0x18fe08` |
| `ldr` at 0x12661c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x126628 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x126628 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x126628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1265ec | `(unknown callee) 0x126630` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x126610 | `[ret(0x126630)+12] != 0x0` | `[ret(0x126630)+12] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x126628 | `r0` | `?` |

## `ioExpGetCfgIndex`

- `ioExpGetCfgIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x126630 to 0x126748.
  Established. Word loads read absolute 0x314fd4 and [sp-28] +4 (`ldr` at 0x126664, `ldr` at 0x126678).
  A signed comparison takes the branch to 0x126718 when (entry r1 & 0xff) is less than [[sp-28]+4], and falls through otherwise (`blt` at 0x126680).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 9/0x18fb58/0x387/0, and a later word load reads [sp-28] (`bl` at 0x126714, `ldr` at 0x126724).
  The recorded r0 return remains unknown, so no concrete indexed object address is established (`ldm` at 0x126734).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x126640 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x126648 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x12664c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x126660 | load | 4 | `abs 0x126738` | `-` |
| `ldr` at 0x126664 | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x12666c | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x126670 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x126674 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126678 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x126684 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126698 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x12669c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1266ac | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x1266b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1266d8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1266dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1266e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1266e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1266f0 | load | 4 | `abs 0x12673c` | `-` |
| `str` at 0x1266f4 | store | 4 | `(unknown)` | `0x18fe10` |
| `str` at 0x1266f8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x1266fc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x126700 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x126708 | load | 4 | `abs 0x126740` | `-` |
| `ldr` at 0x12670c | load | 4 | `abs 0x126744` | `-` |
| `ldr` at 0x126718 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x12671c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x126724 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldm` at 0x126734 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x126734 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x126734 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x126694 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1266a0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1266e8 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x126714 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x387` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x126680 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x126734 | `r0` | `?` |

## `ioExpGetValIndex`

- `ioExpGetValIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x126748 to 0x12686c.
  Established. A word load reads [sp-28] +4, and the signed comparison branches to 0x126830 when (entry r1 & 0xff) is less than that loaded word (`ldr` at 0x126790, `blt` at 0x126798).
  The other path calls unnamed 0x6e1b0 with candidate r0/r1/r2/r3 values entry r0/1/0/0 (`bl` at 0x1267ac).
  A later word load reads [sp-28] +8, but the recorded r0 return is unknown (`ldr` at 0x126848, `ldm` at 0x126858).
  The selected software object and index units remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x126758 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x126760 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x126764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x126778 | load | 4 | `abs 0x12685c` | `-` |
| `ldr` at 0x12677c | load | 4 | `abs 0x314fd4` | `-` |
| `str` at 0x126784 | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x126788 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x12678c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126790 | load | 4 | `[sp-28] +0x4` | `-` |
| `ldr` at 0x12679c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1267b0 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x1267b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1267c4 | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x1267c8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1267f0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x1267f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1267f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1267fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x126808 | load | 4 | `abs 0x126860` | `-` |
| `str` at 0x12680c | store | 4 | `(unknown)` | `0x18fe10` |
| `str` at 0x126810 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x126814 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x126818 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x126820 | load | 4 | `abs 0x126864` | `-` |
| `ldr` at 0x126824 | load | 4 | `abs 0x126868` | `-` |
| `ldr` at 0x126830 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x126834 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x126848 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldm` at 0x126858 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x126858 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x126858 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1267ac | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1267b8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126800 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x12682c | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x395` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x126798 | `(r1 & 0xff) < [[sp-28]+4] (signed, 32-bit)` | `(r1 & 0xff) >= [[sp-28]+4] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x126858 | `r0` | `?` |

## `ioExpWriteExp`

- `ioExpWriteExp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12686c to 0x126e4c.
  Established. Known-edge word stores write ret(0x20370) and later 0 at absolute 0x314fdc (`str` at 0x126a9c, `str` at 0x126da4).
  Byte stores write unknown values at [sp-28] +1 (`strb` at 0x126ab8, `strb` at 0x126ad0).
  Calls to unnamed 0x202d4 and 0x2058c both have candidate r0=0x314fbc, with the latter also having r2=0x314fdc and r3=0 (`bl` at 0x126a44, `bl` at 0x126dac).
  The computed transfer is unresolved (`ldrls` at 0x126ae0), leaving omitted effects and the byte field's physical meaning unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12687c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x126880 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x126884 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x126888 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x12688c | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `strb` at 0x126894 | store | 1 | `sp -0x1e` | `([sp] & 0xff)` |
| `str` at 0x12689c | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x1268a4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x1268a8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1268ac | load | 1 | `r1 +0x4` | `-` |
| `ldrb` at 0x1268b0 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x1268c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1268d8 | store | 4 | `sp -0x2c` | `ret(0x6e1b0)` |
| `ldr` at 0x1268dc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1268ec | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x1268f0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x126918 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x12691c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x126920 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x126924 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x126930 | load | 4 | `abs 0x126e18` | `-` |
| `str` at 0x126934 | store | 4 | `(unknown)` | `0x18fe34` |
| `str` at 0x126938 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x12693c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x126940 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x126944 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0x126948 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x12694c | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldr` at 0x126954 | load | 4 | `abs 0x126e1c` | `-` |
| `ldr` at 0x126958 | load | 4 | `abs 0x126e20` | `-` |
| `ldrb` at 0x126964 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x12696c | load | 1 | `sp -0x1e` | `-` |
| `ldr` at 0x12697c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126990 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x126994 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x1269a4 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x1269a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1269d0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x1269d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1269d8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x1269dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1269e8 | load | 4 | `abs 0x126e24` | `-` |
| `str` at 0x1269ec | store | 4 | `(unknown)` | `0x18fe6c` |
| `str` at 0x1269f0 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x1269f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1269f8 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x1269fc | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldrb` at 0x126a00 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x126a04 | store | 4 | `(unknown)` | `[sp-29]:1` |
| `ldrb` at 0x126a08 | load | 1 | `sp -0x1e` | `-` |
| `str` at 0x126a0c | store | 4 | `(unknown)` | `[sp-30]:1` |
| `ldr` at 0x126a14 | load | 4 | `abs 0x126e1c` | `-` |
| `ldr` at 0x126a18 | load | 4 | `abs 0x126e28` | `-` |
| `ldr` at 0x126a24 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x126a28 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x126a3c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x126a40 | load | 4 | `abs 0x126e2c` | `-` |
| `str` at 0x126a4c | store | 4 | `sp -0x24` | `ret(0x202d4)` |
| `ldr` at 0x126a50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x126a64 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x126a70 | load | 4 | `abs 0x126e30` | `-` |
| `str` at 0x126a74 | store | 4 | `(unknown)` | `0x18fea4` |
| `str` at 0x126a78 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x126a80 | load | 4 | `abs 0x126e1c` | `-` |
| `ldr` at 0x126a84 | load | 4 | `abs 0x126e34` | `-` |
| `ldr` at 0x126a98 | load | 4 | `abs 0x126e38` | `-` |
| `str` at 0x126a9c | store | 4 | `abs 0x314fdc` | `ret(0x20370)` |
| `ldr` at 0x126aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126aa4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x126aa8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x126ab0 | load | 1 | `[sp-28] +0x1` | `-` |
| `strb` at 0x126ab8 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0x126abc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x126ac0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x126ac4 | load | 1 | `[sp-28] +0x1` | `-` |
| `ldrb` at 0x126ac8 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x126ad0 | store | 1 | `[sp-28] +0x1` | `?` |
| `ldr` at 0x126ad4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x126ad8 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0x126ae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x126cf8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126d0c | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x126d10 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x126d20 | store | 4 | `sp -0x30` | `ret(0x2031c)+1` |
| `ldr` at 0x126d24 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x126d4c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x126d50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x126d54 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x126d58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x126d64 | load | 4 | `abs 0x126e3c` | `-` |
| `str` at 0x126d68 | store | 4 | `(unknown)` | `0x18fec0` |
| `ldr` at 0x126d6c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x126d70 | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x126d74 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0x126d78 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x126d7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x126d80 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x126d84 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0x126d8c | load | 4 | `abs 0x126e1c` | `-` |
| `ldr` at 0x126d90 | load | 4 | `abs 0x126e40` | `-` |
| `ldr` at 0x126d9c | load | 4 | `abs 0x126e38` | `-` |
| `str` at 0x126da4 | store | 4 | `abs 0x314fdc` | `0x0` |
| `ldr` at 0x126da8 | load | 4 | `abs 0x126e2c` | `-` |
| `str` at 0x126db4 | store | 4 | `sp -0x24` | `ret(0x2058c)` |
| `ldr` at 0x126db8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x126dcc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x126dd8 | load | 4 | `abs 0x126e44` | `-` |
| `str` at 0x126ddc | store | 4 | `(unknown)` | `0x18fef0` |
| `str` at 0x126de0 | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0x126de8 | load | 4 | `abs 0x126e1c` | `-` |
| `ldr` at 0x126dec | load | 4 | `abs 0x126e48` | `-` |
| `ldr` at 0x126df8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x126e08 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x126e14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x126e14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x126e14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1268d4 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x1268e0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126928 | `(unknown callee) 0x203f4` | `[sp-52]` | `[sp-44]` | `[sp-48]` | `?` |
| `bl` at 0x126960 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x3ae` | `0x0` |
| `bl` at 0x12698c | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126998 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x1269e0 | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0x126a20 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x3b2` | `0x0` |
| `bl` at 0x126a34 | `(unknown callee) 0x134634` | `?` | `?` | `?` | `[[sp-24]]:1` |
| `bl` at 0x126a44 | `(unknown callee) 0x202d4` | `0x314fbc` | `?` | `?` | `?` |
| `bl` at 0x126a5c | `(unknown callee) 0x20184` | `ret(0x202d4)` | `?` | `?` | `ret(0x202d4)` |
| `bl` at 0x126a68 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x126a8c | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x3c9` | `0x0` |
| `bl` at 0x126a90 | `(unknown callee) 0x20370` | `?` | `?` | `?` | `?` |
| `bl` at 0x126d08 | `(unknown callee) 0x6e1b0` | `[sp-20]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126d14 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126d5c | `(unknown callee) 0x203f4` | `[sp-44]` | `[sp-52]` | `[sp-48]` | `?` |
| `bl` at 0x126d98 | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x3fe` | `0x0` |
| `bl` at 0x126dac | `(unknown callee) 0x2058c` | `0x314fbc` | `?` | `0x314fdc` | `0x0` |
| `bl` at 0x126dc4 | `(unknown callee) 0x20184` | `ret(0x2058c)` | `?` | `?` | `ret(0x2058c)` |
| `bl` at 0x126dd0 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x126df4 | `(unknown callee) 0x22070` | `0x9` | `0x18fb58` | `0x40b` | `0x0` |
| `bl` at 0x126e04 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1268c0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x126978 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x126a30 | `[[sp-24]]:1 != 0x4` | `[[sp-24]]:1 == 0x4` |
| `beq` at 0x126a58 | `ret(0x202d4) == 0x0` | `ret(0x202d4) != 0x0` |
| `ldrls` at 0x126ae0 | `[[sp-24]]:1 <= 0x4 (unsigned, 32-bit)` | `[[sp-24]]:1 > 0x4 (unsigned, 32-bit)` |
| `beq` at 0x126dc0 | `ret(0x2058c) == 0x0` | `ret(0x2058c) != 0x0` |
| `beq` at 0x126e00 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x126e14 | `r0` | `[sp-36]` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x126ae0).

## `ioExpReadExp`

- `ioExpReadExp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x126e4c to 0x127058.
  Established. A known-edge byte load reads entry r1, while later loads read [sp-24] and [sp-24] +0xc (`ldrb` at 0x126e6c, `ldrb` at 0x12700c, `ldr` at 0x12701c).
  The call to unnamed 0x22070 has candidate r0/r1/r2/r3 values 1/0x18fb58/0x438/0 (`bl` at 0x127034).
  A word store writes 0 at sp-0x2c, and the recorded known-edge r0 return is 0 (`str` at 0x12703c, `ldm` at 0x127048).
  The computed transfer is unresolved (`ldrls` at 0x126e74), so omitted effects and any physical read target remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x126e5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x126e60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x126e64 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x126e68 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x126e6c | load | 1 | `r1 +0x0` | `-` |
| `ldrls` at 0x126e74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x126f94 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x126fa8 | store | 4 | `sp -0x20` | `ret(0x6e1b0)` |
| `ldr` at 0x126fac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x126fbc | store | 4 | `sp -0x24` | `ret(0x2031c)+1` |
| `ldr` at 0x126fc0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x126fe8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x126fec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x126ff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x126ff4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x127000 | load | 4 | `abs 0x12704c` | `-` |
| `str` at 0x127004 | store | 4 | `(unknown)` | `0x18fec0` |
| `ldr` at 0x127008 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x12700c | load | 1 | `[sp-24] +0x0` | `-` |
| `str` at 0x127010 | store | 4 | `(unknown)` | `[[sp-24]]:1` |
| `str` at 0x127014 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x127018 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12701c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x127020 | store | 4 | `(unknown)` | `[[sp-24]+12]` |
| `ldr` at 0x127028 | load | 4 | `abs 0x127050` | `-` |
| `ldr` at 0x12702c | load | 4 | `abs 0x127054` | `-` |
| `str` at 0x12703c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x127040 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x127048 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x127048 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x127048 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x126fa4 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x126fb0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x126ff8 | `(unknown callee) 0x203f4` | `[sp-40]` | `[sp-32]` | `[sp-36]` | `?` |
| `bl` at 0x127034 | `(unknown callee) 0x22070` | `0x1` | `0x18fb58` | `0x438` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x126e74 | `[r1]:1 <= 0x4 (unsigned, 32-bit)` | `[r1]:1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x127048 | `r0` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x126e74).
