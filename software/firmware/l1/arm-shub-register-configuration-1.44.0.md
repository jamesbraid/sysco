# ARM SHUB register and configuration paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/shub-register-configuration-facts.txt) retain all tool sections.

## `SHUB_Init`

- `SHUB_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x412f4 to 0x417a4.
  Established. An included byte store writes `(r1 & 0xff)` to absolute `0x132224` (`strb` at 0x41324).
  The halfword at `0x2997b0` receives `0x320` (`strh` at 0x416c0).
  The unnamed `0x417a4` call carries `[sp-32]`, `[sp]`, `0x132240`, `[sp+12]` (`bl` at 0x4136c).
  Initialization record and count meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41304 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41308 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4130c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x41310 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x41314 | load | 4 | `sp +0x10` | `-` |
| `strb` at 0x41318 | store | 1 | `sp -0x21` | `([sp+16] & 0xff)` |
| `ldr` at 0x4131c | load | 4 | `abs 0x4177c` | `-` |
| `ldr` at 0x41320 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x41324 | store | 1 | `abs 0x132224` | `(r1 & 0xff)` |
| `ldr` at 0x41328 | load | 4 | `abs 0x41780` | `-` |
| `ldr` at 0x4132c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x41330 | store | 4 | `abs 0x132228` | `[sp-20]` |
| `ldr` at 0x41334 | load | 4 | `abs 0x41784` | `-` |
| `ldr` at 0x41338 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x4133c | store | 1 | `abs 0x13222c` | `([sp-28] & 0xff)` |
| `ldr` at 0x41340 | load | 4 | `abs 0x41788` | `-` |
| `ldr` at 0x41344 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x41348 | store | 4 | `abs 0x132238` | `[sp+4]` |
| `ldr` at 0x4134c | load | 4 | `abs 0x4178c` | `-` |
| `ldr` at 0x41350 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x41354 | store | 4 | `abs 0x13223c` | `[sp+8]` |
| `ldr` at 0x41358 | load | 4 | `abs 0x41790` | `-` |
| `ldr` at 0x4135c | load | 4 | `sp +0xc` | `-` |
| `str` at 0x41360 | store | 4 | `abs 0x132240` | `[sp+12]` |
| `ldr` at 0x41364 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x41368 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x41374 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x41378 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41384 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41388 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x413a4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x413a8 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x413ac | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x413c8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x413cc | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x413d0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x413ec | store | 1 | `(unknown)` | `0x2` |
| `ldr` at 0x413f0 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x413f4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x4140c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x41410 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x41414 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41418 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4143c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x41440 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41444 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x41468 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x4146c | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41470 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x41494 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x41498 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x4149c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x414c0 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x414c4 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x414c8 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x414ec | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x414f0 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x414f4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x41518 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x4151c | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41520 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x41544 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x41548 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x4154c | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x41570 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x41574 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41578 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4159c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x415a0 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x415a4 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x415c8 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x415cc | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x415d0 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x415f4 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x415f8 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x415fc | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x41620 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x41624 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41628 | load | 4 | `sp -0x28` | `-` |
| `strb` at 0x4164c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x41650 | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41654 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x41678 | store | 2 | `(unknown)` | `0x5` |
| `ldr` at 0x4167c | load | 4 | `abs 0x41794` | `-` |
| `ldr` at 0x41680 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0x416a4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x416a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x416b0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x416b8 | load | 4 | `abs 0x41798` | `-` |
| `strh` at 0x416c0 | store | 2 | `abs 0x2997b0` | `0x320` |
| `str` at 0x416cc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x416d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x416d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x416e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x416e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x416f0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x416f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41700 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x41704 | load | 4 | `(unknown)` | `-` |
| `str` at 0x41710 | store | 4 | `sp -0x2c` | `ret(0xdcfd0)` |
| `ldr` at 0x41714 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x41720 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x4172c | load | 4 | `abs 0x4179c` | `-` |
| `str` at 0x41730 | store | 4 | `sp -0x38` | `0xf67d4` |
| `ldr` at 0x41734 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x4173c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41744 | load | 4 | `(unknown)` | `-` |
| `str` at 0x41748 | store | 4 | `sp -0x34` | `?` |
| `str` at 0x4174c | store | 4 | `sp -0x30` | `ret(0x1f8e0)` |
| `ldr` at 0x41754 | load | 4 | `abs 0x417a0` | `-` |
| `ldr` at 0x41764 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x4176c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldm` at 0x41778 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41778 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41778 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4136c | `(unknown callee) 0x417a4` | `[sp-32]` | `[sp]` | `0x132240` | `[sp+12]` |
| `bl` at 0x416c4 | `(unknown callee) 0x47a80` | `?` | `?` | `0x2997b0` | `0x320` |
| `bl` at 0x41708 | `(unknown callee) 0xdcfd0` | `?` | `?` | `?` | `?` |
| `bl` at 0x41724 | `(unknown callee) 0x1f8e0` | `ret(0xdcfd0)` | `?` | `?` | `ret(0xdcfd0)` |
| `bl` at 0x41760 | `(unknown callee) 0x200a0` | `0x9` | `0xf67f0` | `0xad` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x41380 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0x416dc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0x4171c | `ret(0xdcfd0) == 0x0` | `ret(0xdcfd0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41778 | `r0` | `?` |

## `SHUB_Set_MemCfg`

- `SHUB_Set_MemCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x417a4 to 0x4182c.
  Established. The absolute word at `0x132230` receives entry `r0` (`str` at 0x417dc).
  A byte store writes `([sp-24] & 0xff)` to `0x132234` (`strb` at 0x417e8).
  Inequality `[0x132230] != [sp-28]` takes `0x41818`, while equality falls through (`bne` at 0x417fc).
  Memory configuration cache semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x417b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x417b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x417bc | load | 4 | `abs 0x41824` | `-` |
| `ldr` at 0x417c0 | load | 4 | `abs 0x132230` | `-` |
| `str` at 0x417c4 | store | 4 | `sp -0x1c` | `[0x132230]` |
| `ldr` at 0x417c8 | load | 4 | `abs 0x41828` | `-` |
| `ldrb` at 0x417cc | load | 1 | `abs 0x132234` | `-` |
| `str` at 0x417d0 | store | 4 | `sp -0x20` | `[0x132234]:1` |
| `ldr` at 0x417d4 | load | 4 | `abs 0x41824` | `-` |
| `ldr` at 0x417d8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x417dc | store | 4 | `abs 0x132230` | `r0` |
| `ldr` at 0x417e0 | load | 4 | `abs 0x41828` | `-` |
| `ldr` at 0x417e4 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0x417e8 | store | 1 | `abs 0x132234` | `([sp-24] & 0xff)` |
| `ldr` at 0x417ec | load | 4 | `abs 0x41824` | `-` |
| `ldr` at 0x417f0 | load | 4 | `abs 0x132230` | `-` |
| `ldr` at 0x417f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41800 | load | 4 | `abs 0x41828` | `-` |
| `ldrb` at 0x41804 | load | 1 | `abs 0x132234` | `-` |
| `ldr` at 0x41808 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x41820 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41820 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41820 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41818 | `(unknown callee) 0x41cd8` | `r0` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x417fc | `[0x132230] != [sp-28]` | `[0x132230] == [sp-28]` |
| `bne` at 0x41810 | `[0x132234]:1 != [sp-32]` | `[0x132234]:1 == [sp-32]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41820 | `r0` | `?` |

## `SHUB_MMR_Write`

- `SHUB_MMR_Write`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4182c to 0x418a4.
  Established. The unnamed `0xde274` call carries unknown and entry `r1`, `r2`, `r3` (`bl` at 0x41870).
  A nonzero result takes `0x4188c`, while zero falls through (`bne` at 0x4187c).
  The word at `sp -0x24` receives `0x1` on an included path (`str` at 0x41884).
  Register-write transport behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4183c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41840 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x41844 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x41848 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x4184c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41854 | load | 4 | `abs 0x418a0` | `-` |
| `ldr` at 0x41858 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x41860 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x41864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41868 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4186c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41884 | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x41890 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x41894 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x4189c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x4189c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x4189c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41870 | `(unknown callee) 0xde274` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4187c | `ret(0xde274) != 0x0` | `ret(0xde274) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x4189c | `r0` | `[sp-36]` |

## `SHUB_MMR_Read`

- `SHUB_MMR_Read`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x418a4 to 0x4191c.
  Established. The word at absolute `0x132228` is read (`ldr` at 0x418d0).
  The unnamed `0xde16c` call carries unknown and entry `r1`, `r2`, `r3` (`bl` at 0x418e8);
  its nonzero result takes `0x41904`, while zero falls through (`bne` at 0x418f4).
  Register-read transport contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x418b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x418b8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x418bc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x418c0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x418c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x418cc | load | 4 | `abs 0x41918` | `-` |
| `ldr` at 0x418d0 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x418d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x418dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x418e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x418e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x418fc | store | 4 | `sp -0x24` | `0x1` |
| `str` at 0x41908 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x4190c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x41914 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41914 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41914 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x418e8 | `(unknown callee) 0xde16c` | `?` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x418f4 | `ret(0xde16c) != 0x0` | `ret(0xde16c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41914 | `r0` | `[sp-36]` |

## `SHUB_Set_NICfg`

- `SHUB_Set_NICfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x419c8 to 0x41ac8.
  Established. The halfword through `[sp-36] +0x0` receives `([sp-26]:2 & 0xffff)` on an included path (`strh` at 0x41a98).
  Equality `[sp-32] == 0x3` takes `0x41a90`, while inequality falls through (`beq` at 0x41a4c).
  An included word store writes `0x1` to an unknown effective target (`str` at 0x41ab4).
  NI configuration update mode meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x419d8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x419dc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x419e0 | store | 4 | `sp -0x20` | `r3` |
| `strh` at 0x419e4 | store | 2 | `sp -0x1a` | `(r2 & 0xffff)` |
| `ldr` at 0x419e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x419fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41a08 | load | 4 | `abs 0x41ac0` | `-` |
| `str` at 0x41a10 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x41a14 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x41a18 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0x41a1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41a28 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41a34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41a44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x41a54 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41a58 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x41a5c | load | 2 | `[sp-36] +0x0` | `-` |
| `ldrh` at 0x41a60 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x41a68 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x41a70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41a74 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x41a78 | load | 2 | `sp -0x1a` | `-` |
| `ldrh` at 0x41a80 | load | 2 | `[sp-36] +0x0` | `-` |
| `strh` at 0x41a88 | store | 2 | `[sp-36] +0x0` | `?` |
| `ldr` at 0x41a90 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x41a94 | load | 2 | `sp -0x1a` | `-` |
| `strh` at 0x41a98 | store | 2 | `[sp-36] +0x0` | `([sp-26]:2 & 0xffff)` |
| `ldr` at 0x41a9c | load | 4 | `abs 0x41ac4` | `-` |
| `ldr` at 0x41aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x41aa8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x41ab4 | store | 4 | `(unknown)` | `0x1` |
| `ldm` at 0x41abc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41abc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41abc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x41a24 | `[sp-32] == 0x2` | `[sp-32] != 0x2` |
| `bgt` at 0x41a30 | `[sp-32] > 0x2 (signed, 32-bit)` | `[sp-32] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x41a3c | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `beq` at 0x41a4c | `[sp-32] == 0x3` | `[sp-32] != 0x3` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41abc | `r0` | `r0` |

## `SHUB_Get_NICfg`

- `SHUB_Get_NICfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4196c to 0x419c8.
  Established. Entry `r0` and `r1` are saved in words at `sp -0x14` and `sp -0x18` (`str` at 0x4197c, `str` at 0x41980).
  The halfword load has an unknown effective target (`ldrh` at 0x419b4).
  The integer return value is unknown (`ldm` at 0x419c0).
  NI configuration record indexing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4197c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41980 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x41984 | load | 4 | `abs 0x419c4` | `-` |
| `ldr` at 0x41988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4199c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x419b4 | load | 2 | `(unknown)` | `-` |
| `ldm` at 0x419c0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x419c0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x419c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x419c0 | `r0` | `?` |

## `SHUB_Set_MCI`

- `SHUB_Set_MCI`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41ac8 to 0x41bb0.
  Established. The absolute word at `0x132244` receives `0x1` (`str` at 0x41af4).
  Included byte and halfword stores write `([sp-17]:1 & 0xff)` and `([sp-20]:2 & 0xffff)` to unknown effective targets (`strb` at 0x41b34, `strh` at 0x41b60).
  MCI configuration field roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41adc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strh` at 0x41ae0 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0x41ae8 | store | 1 | `sp -0x15` | `(r2 & 0xff)` |
| `ldr` at 0x41aec | load | 4 | `abs 0x41ba8` | `-` |
| `str` at 0x41af4 | store | 4 | `abs 0x132244` | `0x1` |
| `str` at 0x41afc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x41b00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x41b0c | load | 4 | `abs 0x41bac` | `-` |
| `ldr` at 0x41b10 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41b30 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x41b34 | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x41b38 | load | 4 | `abs 0x41bac` | `-` |
| `ldr` at 0x41b3c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x41b5c | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x41b60 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x41b64 | load | 4 | `abs 0x41bac` | `-` |
| `ldr` at 0x41b68 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x41b88 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x41b8c | store | 1 | `(unknown)` | `([sp-21]:1 & 0xff)` |
| `ldr` at 0x41b90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x41b98 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x41ba4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41ba4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41ba4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x41b08 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41ba4 | `r0` | `?` |

## `SHUB_Set_WRT90`

- `SHUB_Set_WRT90`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41bd0 to 0x41cb8.
  Established. An absolute word assignment writes `0x1` to `0x132248` (`str` at 0x41bfc).
  Included byte stores write `([sp-17]:1 & 0xff)` and `([sp-18]:1 & 0xff)` to unknown targets (`strb` at 0x41c3c, `strb` at 0x41c68).
  Write-timing record semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41be4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x41bec | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strh` at 0x41bf0 | store | 2 | `sp -0x14` | `(r2 & 0xffff)` |
| `ldr` at 0x41bf4 | load | 4 | `abs 0x41cb0` | `-` |
| `str` at 0x41bfc | store | 4 | `abs 0x132248` | `0x1` |
| `str` at 0x41c04 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x41c08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41c14 | load | 4 | `abs 0x41cb4` | `-` |
| `ldr` at 0x41c18 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41c38 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x41c3c | store | 1 | `(unknown)` | `([sp-17]:1 & 0xff)` |
| `ldr` at 0x41c40 | load | 4 | `abs 0x41cb4` | `-` |
| `ldr` at 0x41c44 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41c64 | load | 1 | `sp -0x12` | `-` |
| `strb` at 0x41c68 | store | 1 | `(unknown)` | `([sp-18]:1 & 0xff)` |
| `ldr` at 0x41c6c | load | 4 | `abs 0x41cb4` | `-` |
| `ldr` at 0x41c70 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0x41c90 | load | 2 | `sp -0x14` | `-` |
| `strh` at 0x41c94 | store | 2 | `(unknown)` | `([sp-20]:2 & 0xffff)` |
| `ldr` at 0x41c98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x41ca0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x41cac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x41cac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x41cac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x41c10 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x41cac | `r0` | `?` |

## `SHUB_Set_FSBCfg`

- `SHUB_Set_FSBCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41e68 to 0x4201c.
  Established. The word at absolute `0x13224c` receives `0x1` (`str` at 0x41ec8).
  Words at entry `sp +0x0`, `sp +0x4`, and `sp +0x8` are loaded (`ldr` at 0x41e7c, `ldr` at 0x41e80, `ldr` at 0x41e84).
  An included byte store writes `([sp-31]:1 & 0xff)` to an unknown target (`strb` at 0x41ff8).
  FSB configuration argument roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x41e7c | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x41e80 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x41e84 | load | 4 | `sp +0x8` | `-` |
| `strb` at 0x41e8c | store | 1 | `sp -0x19` | `(r0 & 0xff)` |
| `strb` at 0x41e94 | store | 1 | `sp -0x1a` | `(r1 & 0xff)` |
| `strb` at 0x41e9c | store | 1 | `sp -0x1b` | `(r2 & 0xff)` |
| `strb` at 0x41ea4 | store | 1 | `sp -0x1c` | `(r3 & 0xff)` |
| `strb` at 0x41eac | store | 1 | `sp -0x1d` | `([sp] & 0xff)` |
| `strb` at 0x41eb4 | store | 1 | `sp -0x1e` | `([sp+4] & 0xff)` |
| `strb` at 0x41ebc | store | 1 | `sp -0x1f` | `([sp+8] & 0xff)` |
| `ldr` at 0x41ec0 | load | 4 | `abs 0x42014` | `-` |
| `str` at 0x41ec8 | store | 4 | `abs 0x13224c` | `0x1` |
| `str` at 0x41ed0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x41ed4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x41ee0 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41ee4 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41efc | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x41f00 | store | 1 | `(unknown)` | `([sp-25]:1 & 0xff)` |
| `ldr` at 0x41f04 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41f08 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41f20 | load | 1 | `sp -0x1a` | `-` |
| `strb` at 0x41f24 | store | 1 | `(unknown)` | `([sp-26]:1 & 0xff)` |
| `ldr` at 0x41f28 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41f2c | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41f4c | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x41f50 | store | 1 | `(unknown)` | `([sp-27]:1 & 0xff)` |
| `ldr` at 0x41f54 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41f58 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41f78 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0x41f7c | store | 1 | `(unknown)` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0x41f80 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41f84 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41fa4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x41fa8 | store | 1 | `(unknown)` | `([sp-29]:1 & 0xff)` |
| `ldr` at 0x41fac | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41fb0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41fd0 | load | 1 | `sp -0x1e` | `-` |
| `strb` at 0x41fd4 | store | 1 | `(unknown)` | `([sp-30]:1 & 0xff)` |
| `ldr` at 0x41fd8 | load | 4 | `abs 0x42018` | `-` |
| `ldr` at 0x41fdc | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0x41ff4 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x41ff8 | store | 1 | `(unknown)` | `([sp-31]:1 & 0xff)` |
| `ldr` at 0x41ffc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x42004 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0x42010 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x42010 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x42010 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42010 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x41edc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42010 | `r0` | `?` |

## `SHUBTarget`

- `SHUBTarget`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4203c to 0x42078.
  Established. Entry `r0` is narrowed into the byte at `sp -0x11` (`strb` at 0x42050).
  A word load reads absolute `0x132228` (`ldr` at 0x42060).
  The later word load has unknown effective target (`ldr` at 0x42068).
  Target record interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x42050 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x42054 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x4205c | load | 4 | `abs 0x42074` | `-` |
| `ldr` at 0x42060 | load | 4 | `abs 0x132228` | `-` |
| `ldr` at 0x42068 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x42070 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x42070 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x42070 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x42070 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x42070 | `r0` | `?` |

## `SHUB_GetClkstop`

- `SHUB_GetClkstop`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45f60 to 0x45f80.
  Established. A byte load reads absolute `0x29974d` (`ldrb` at 0x45f70).
  The integer return record carries `[0x29974d]:1` (`ldm` at 0x45f78).
  Clock-stop state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x45f6c | load | 4 | `abs 0x45f7c` | `-` |
| `ldrb` at 0x45f70 | load | 1 | `abs 0x29974d` | `-` |
| `ldm` at 0x45f78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45f78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45f78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45f78 | `r0` | `[0x29974d]:1` |

## `SHUB_SetClkstop`

- `SHUB_SetClkstop`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45ef0 to 0x45f60.
  Established. An included byte store has unknown target and value (`strb` at 0x45f40).
  The word at `sp -0x18` begins at `0x0` and later receives `[sp-24]+1` (`str` at 0x45f08, `str` at 0x45f4c).
  Clock-stop table update meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45f00 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x45f08 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x45f0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45f18 | load | 4 | `abs 0x45f5c` | `-` |
| `ldr` at 0x45f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45f3c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x45f40 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x45f44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45f4c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x45f58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45f58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45f58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x45f14 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45f58 | `r0` | `?` |

## `SHUB_GetTP`

- `SHUB_GetTP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45ff0 to 0x46010.
  Established. The byte at absolute `0x29974c` is read (`ldrb` at 0x46000);
  its value `[0x29974c]:1` appears in the integer return record (`ldm` at 0x46008).
  Test-point selector encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x45ffc | load | 4 | `abs 0x4600c` | `-` |
| `ldrb` at 0x46000 | load | 1 | `abs 0x29974c` | `-` |
| `ldm` at 0x46008 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x46008 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x46008 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x46008 | `r0` | `[0x29974c]:1` |

## `SHUB_SetTP`

- `SHUB_SetTP`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45f80 to 0x45ff0.
  Established. Entry `r0` is saved in the word at `sp -0x14` (`str` at 0x45f90).
  An included byte store has unknown target and value (`strb` at 0x45fd0).
  The word at `sp -0x18` receives `[sp-24]+1` (`str` at 0x45fdc).
  Test-point configuration layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x45f90 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x45f98 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x45f9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45fa8 | load | 4 | `abs 0x45fec` | `-` |
| `ldr` at 0x45fac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x45fcc | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x45fd0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x45fd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x45fdc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0x45fe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x45fe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x45fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0x45fa4 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x45fe8 | `r0` | `?` |

## `shub_set_link_enable`

- `shub_set_link_enable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x47458 to 0x4753c.
  Established. The unnamed `0xde274` call carries `[sp-20]`, `0x10060300`, `0x0`, `0x1` (`bl` at 0x47508).
  A zero `[0x132630]` takes `0x474dc`, while nonzero falls through (`beq` at 0x474a4).
  The integer return record gives `0x0` (`ldm` at 0x47520).
  Link-enable operation semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x47468 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4746c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x47470 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x47474 | load | 4 | `abs 0x47524` | `-` |
| `ldr` at 0x47478 | load | 4 | `abs 0x29737c` | `-` |
| `ldr` at 0x47484 | load | 4 | `abs 0x47528` | `-` |
| `ldr` at 0x47488 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4748c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x47490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x47498 | load | 4 | `abs 0x4752c` | `-` |
| `ldr` at 0x4749c | load | 4 | `abs 0x132630` | `-` |
| `ldr` at 0x474a8 | load | 4 | `abs 0x47528` | `-` |
| `str` at 0x474ac | store | 4 | `sp -0x30` | `0xf7774` |
| `ldr` at 0x474b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x474b4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x474b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x474bc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x474c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x474c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x474cc | load | 4 | `abs 0x47530` | `-` |
| `ldr` at 0x474d0 | load | 4 | `abs 0x47534` | `-` |
| `ldr` at 0x474dc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x474ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x474f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x474fc | load | 4 | `abs 0x47538` | `-` |
| `str` at 0x47514 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x47518 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x47520 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x47520 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x47520 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x47494 | `(unknown callee) 0x221f0` | `0xf7774` | `r0` | `r1` | `r2` |
| `bl` at 0x474d8 | `(unknown callee) 0x200a0` | `0x6` | `0xf67f0` | `0x7ef` | `0x0` |
| `bl` at 0x474f4 | `(unknown callee) 0xd708c` | `?` | `?` | `?` | `?` |
| `bl` at 0x47508 | `(unknown callee) 0xde274` | `[sp-20]` | `0x10060300` | `0x0` | `0x1` |
| `bl` at 0x4750c | `(unknown callee) 0xd70fc` | `ret(0xde274)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x47480 | `[0x29737c] == 0x0` | `[0x29737c] != 0x0` |
| `beq` at 0x474a4 | `[0x132630] == 0x0` | `[0x132630] != 0x0` |
| `bne` at 0x474e4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x47520 | `r0` | `0x0` |
