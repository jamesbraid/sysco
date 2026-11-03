# ARM TAP drivers and scan command paths in 1.44.0

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

[Fact sheets and full listings](disasm/arm-1.44.0-package4/tap-scan-commands-facts.txt) retain all tool sections.

## `sn1sc_set_tap_val`

- `sn1sc_set_tap_val`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd98e0 to 0xd9978.
  Established. The included halfword store targets `[0x29c464] +0x0` with an unknown value (`strh` at 0xd996c).
  The integer return record retains entry `r0` (`ldm` at 0xd9970).
  TAP pin encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd98f0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd98f4 | load | 4 | `abs 0xd9974` | `-` |
| `ldr` at 0xd98f8 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd98fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9908 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd991c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9930 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9958 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xd996c | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldm` at 0xd9970 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd9970 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9970 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9970 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9970 | `r0` | `r0` |

## `sn1sc_get_tap_val`

- `sn1sc_get_tap_val`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9588 to 0xd96ac.
  Established. Halfword reads include six accesses to `[0x29c464] +0x0` (`ldrh` at 0xd95a0, `ldrh` at 0xd95c4, `ldrh` at 0xd95f0, `ldrh` at 0xd961c, `ldrh` at 0xd9648, `ldrh` at 0xd9674).
  The integer return candidate is `[sp-20]` (`ldm` at 0xd96a4).
  Meaning of the sampled pins remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xd9598 | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd959c | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd95a0 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd95bc | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd95c0 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd95c4 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd95e8 | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd95ec | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd95f0 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9614 | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd9618 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd961c | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9640 | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd9644 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9648 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd966c | load | 4 | `abs 0xd96a8` | `-` |
| `ldr` at 0xd9670 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9674 | load | 2 | `[0x29c464] +0x0` | `-` |
| `str` at 0xd9698 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xd969c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd96a4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xd96a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd96a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd96a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd96a4 | `r0` | `[sp-20]` |

## `sn1sc_get_tap_text`

- `sn1sc_get_tap_text`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd96ac to 0xd98e0.
  Established. Included word assignments place `0x31` and `0x30` at `sp -0x1c` on separate paths (`str` at 0xd97d8, `str` at 0xd97e4).
  The unnamed `0x197a8` call carries entry `r0`, `0x1216b0`, `[sp-28]`, `[sp-32]` (`bl` at 0xd98cc).
  Text field semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd96bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xd96c0 | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd96c4 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd96c8 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd96e4 | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd96e8 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd96ec | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9710 | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd9714 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9718 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd973c | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd9740 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9744 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9768 | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd976c | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9770 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9794 | load | 4 | `abs 0xd98d8` | `-` |
| `ldr` at 0xd9798 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd979c | load | 2 | `[0x29c464] +0x0` | `-` |
| `str` at 0xd97c0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xd97c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd97d8 | store | 4 | `sp -0x1c` | `0x31` |
| `str` at 0xd97e4 | store | 4 | `sp -0x1c` | `0x30` |
| `ldr` at 0xd97e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd97fc | store | 4 | `sp -0x20` | `0x31` |
| `str` at 0xd9808 | store | 4 | `sp -0x20` | `0x30` |
| `ldr` at 0xd980c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9820 | store | 4 | `sp -0x24` | `0x31` |
| `str` at 0xd982c | store | 4 | `sp -0x24` | `0x30` |
| `ldr` at 0xd9830 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xd9834 | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0xd9838 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd984c | store | 4 | `sp -0x28` | `0x31` |
| `str` at 0xd9858 | store | 4 | `sp -0x28` | `0x30` |
| `ldr` at 0xd985c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xd9860 | store | 4 | `sp -0x3c` | `[sp-40]` |
| `ldr` at 0xd9864 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9878 | store | 4 | `sp -0x2c` | `0x31` |
| `str` at 0xd9884 | store | 4 | `sp -0x2c` | `0x30` |
| `ldr` at 0xd9888 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xd988c | store | 4 | `sp -0x38` | `[sp-44]` |
| `ldr` at 0xd9890 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd98a4 | store | 4 | `sp -0x30` | `0x31` |
| `str` at 0xd98b0 | store | 4 | `sp -0x30` | `0x30` |
| `ldr` at 0xd98b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xd98b8 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0xd98bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd98c0 | load | 4 | `abs 0xd98dc` | `-` |
| `ldr` at 0xd98c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd98c8 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xd98d4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd98d4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd98d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd98cc | `(unknown callee) 0x197a8` | `r0` | `0x1216b0` | `[sp-28]` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd97d0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd97f4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9818 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9844 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9870 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd989c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd98d4 | `r0` | `ret(0x197a8)` |

## `sn1sc_drive_tap`

- `sn1sc_drive_tap`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9be0 to 0xd9e0c.
  Established. The halfword store to `[0x29c464] +0x0` has an unknown value (`strh` at 0xd9dc0).
  The unnamed `0xe8b64` call carries `sp-52`, unknown, unknown, `sp-52` (`bl` at 0xd9df4);
  its own integer return candidate is `0x1` (`ldm` at 0xd9e04).
  TAP drive result interpretation remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9bf0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd9bf4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd9bf8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd9bfc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd9c00 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9c04 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9c08 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9c24 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9c28 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9c2c | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9c50 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9c54 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9c58 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9c7c | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9c80 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9c84 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9ca8 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9cac | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9cb0 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9cd4 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9cd8 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9cdc | load | 2 | `[0x29c464] +0x0` | `-` |
| `str` at 0xd9d00 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9d04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9d0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9d1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9d2c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd9d34 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9d3c | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd9d44 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9d48 | load | 4 | `abs 0xd9e08` | `-` |
| `ldr` at 0xd9d4c | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd9d50 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9d5c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9d70 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9d84 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9d98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9dac | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xd9dc0 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xd9dc4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9dc8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd9dcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9dd0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd9dd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd9dd8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd9ddc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xd9de0 | store | 4 | `sp -0x28` | `[sp]` |
| `ldr` at 0xd9de4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9de8 | store | 4 | `sp -0x24` | `?` |
| `ldm` at 0xd9e04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9e04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9df4 | `(unknown callee) 0xe8b64` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xd9d18 | `? == ?` | `? != ?` |
| `beq` at 0xd9d28 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9e04 | `r0` | `0x1` |

## `sn1sc_drive_tap_fixed_tms`

- `sn1sc_drive_tap_fixed_tms`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xd9978 to 0xd9be0.
  Established. Nonzero entry `r1` takes `0xd99b0`, while zero falls through (`bne` at 0xd99a0).
  The included halfword assignment to `[0x29c464] +0x0` has unknown data (`strh` at 0xd9b70).
  The unnamed `0xe8938` call carries `sp-52`, unknown, unknown, `sp-52` (`bl` at 0xd9bb4).
  Fixed-TMS pin policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xd9988 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xd998c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xd9990 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xd9994 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xd9998 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd99a8 | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xd99b0 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd99b4 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd99b8 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd99d4 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd99d8 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd99dc | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9a00 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd9a04 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9a08 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9a2c | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd9a30 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9a34 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9a58 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd9a5c | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9a60 | load | 2 | `[0x29c464] +0x0` | `-` |
| `ldr` at 0xd9a84 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd9a88 | load | 4 | `abs 0x29c464` | `-` |
| `ldrh` at 0xd9a8c | load | 2 | `[0x29c464] +0x0` | `-` |
| `str` at 0xd9ab0 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9ab4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9abc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9acc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xd9adc | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd9ae4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9aec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xd9af4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xd9af8 | load | 4 | `abs 0xd9bdc` | `-` |
| `ldr` at 0xd9afc | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xd9b00 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9b0c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9b20 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9b34 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9b48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xd9b5c | load | 4 | `sp -0x38` | `-` |
| `strh` at 0xd9b70 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xd9b74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xd9b78 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xd9b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xd9b80 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xd9b84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xd9b90 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd9b94 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xd9b98 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xd9b9c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xd9ba0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xd9bcc | store | 4 | `sp -0x3c` | `0x1` |
| `ldr` at 0xd9bd0 | load | 4 | `sp -0x3c` | `-` |
| `ldm` at 0xd9bd8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xd9bd8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xd9bd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xd9bb4 | `(unknown callee) 0xe8938` | `sp-52` | `?` | `?` | `sp-52` |
| `bl` at 0xd9bc4 | `(unknown callee) 0xe8ae4` | `sp-52` | `?` | `?` | `sp-52` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xd99a0 | `r1 != 0x0` | `r1 == 0x0` |
| `beq` at 0xd9ac8 | `? == ?` | `? != ?` |
| `beq` at 0xd9ad8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xd9b8c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xd9ba8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xd9bd8 | `r0` | `0x1` |

## `sn1sc_asm_drive_tap`

- `sn1sc_asm_drive_tap`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8b64 to 0xe8d24.
  Established. Included halfword writes send unknown values to `[0x29c464] +0x0` (`strh` at 0xe8c20, `strh` at 0xe8c3c, `strh` at 0xe8c54, `strh` at 0xe8cd0, `strh` at 0xe8cec, `strh` at 0xe8d04).
  Halfword samples include reads at the same target (`ldrh` at 0xe8c34, `ldrh` at 0xe8ce4).
  The word at `sp -0x2c` receives `[r0+4]` (`str` at 0xe8b7c).
  Assembly scan timing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe8b70 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xe8b78 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xe8b7c | store | 4 | `sp -0x2c` | `[r0+4]` |
| `ldr` at 0xe8b80 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe8b84 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe8b88 | load | 4 | `r0 +0x10` | `-` |
| `str` at 0xe8bcc | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xe8be4 | load | 4 | `abs 0xe8d20` | `-` |
| `ldr` at 0xe8be8 | load | 4 | `abs 0x29c464` | `-` |
| `str` at 0xe8bec | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xe8bf4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8bf8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8bfc | store | 4 | `sp -0x24` | `0x0` |
| `strh` at 0xe8c20 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8c28 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8c30 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0xe8c34 | load | 2 | `[0x29c464] +0x0` | `-` |
| `strh` at 0xe8c3c | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8c48 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8c50 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0xe8c54 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8c64 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xe8c68 | store | 4 | `[sp-48] +0x0` | `?` |
| `ldr` at 0xe8c70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8c7c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xe8c88 | load | 4 | `abs 0xe8d20` | `-` |
| `ldr` at 0xe8c8c | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xe8c90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8c94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xe8c98 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xe8ca4 | store | 4 | `sp -0x24` | `0x0` |
| `strh` at 0xe8cd0 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8cd8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8ce0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0xe8ce4 | load | 2 | `[0x29c464] +0x0` | `-` |
| `strh` at 0xe8cec | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8cf8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8d00 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0xe8d04 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xe8d14 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe8bd8 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xe8c60 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xe8c78 | `C == 0` | `not (C == 0)` |
| `beq` at 0xe8c84 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xe8cb4 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xe8d10 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xe8d1c | `r0` | `?` |

## `sn1sc_asm_drive_tap_fixed_tms`

- `sn1sc_asm_drive_tap_fixed_tms`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe8938 to 0xe8ae4.
  Established. The halfword stores include six unknown-value writes to `[0x29c464] +0x0` (`strh` at 0xe89e8, `strh` at 0xe8a04, `strh` at 0xe8a1c, `strh` at 0xe8a90, `strh` at 0xe8aac, `strh` at 0xe8ac4).
  A word assignment saves `[r0+4]` at `sp -0x28` (`str` at 0xe8950).
  The integer return value is unknown (`pop` at 0xe8adc).
  Fixed-TMS clock timing remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe8944 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xe894c | load | 4 | `r0 +0x4` | `-` |
| `str` at 0xe8950 | store | 4 | `sp -0x28` | `[r0+4]` |
| `ldr` at 0xe8954 | load | 4 | `r0 +0x8` | `-` |
| `ldr` at 0xe8958 | load | 4 | `r0 +0xc` | `-` |
| `ldr` at 0xe89b0 | load | 4 | `abs 0xe8ae0` | `-` |
| `ldr` at 0xe89b4 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xe89c0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe89c4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe89dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe89e4 | store | 4 | `sp -0x24` | `?` |
| `strh` at 0xe89e8 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe89f0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe89f8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0xe89fc | load | 2 | `[0x29c464] +0x0` | `-` |
| `strh` at 0xe8a04 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8a10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8a18 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0xe8a1c | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xe8a2c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xe8a34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8a40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xe8a4c | load | 4 | `abs 0xe8ae0` | `-` |
| `ldr` at 0xe8a50 | load | 4 | `abs 0x29c464` | `-` |
| `ldr` at 0xe8a54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xe8a64 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xe8a84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8a8c | store | 4 | `sp -0x24` | `?` |
| `strh` at 0xe8a90 | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8a98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8aa0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldrh` at 0xe8aa4 | load | 2 | `[0x29c464] +0x0` | `-` |
| `strh` at 0xe8aac | store | 2 | `[0x29c464] +0x0` | `?` |
| `ldr` at 0xe8ab8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xe8ac0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `strh` at 0xe8ac4 | store | 2 | `[0x29c464] +0x0` | `?` |
| `str` at 0xe8ad4 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bhs` at 0xe89a4 | `0x0 >= ? (unsigned, 32-bit)` | `0x0 < ? (unsigned, 32-bit)` |
| `bge` at 0xe8a28 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `blo` at 0xe8a3c | `C == 0` | `not (C == 0)` |
| `beq` at 0xe8a48 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0xe8a74 | `0x1f < ? (signed, 32-bit)` | `0x1f >= ? (signed, 32-bit)` |
| `bge` at 0xe8ad0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `pop` at 0xe8adc | `r0` | `?` |

## `smpcmd_scan_sel`

- `smpcmd_scan_sel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdba98 to 0xdbc80.
  Established. A nonzero `[0x297378]` takes `0xdbaf0`, while zero falls through (`bne` at 0xdbad0).
  The unnamed `0xd12a4` call carries unknown, `[[sp-24]+40]`, `[[sp-24]+44]`, `0x0` (`bl` at 0xdbc10).
  An included word store sets `sp -0x20` to `0x58` (`str` at 0xdbaf4).
  Scan selection request semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbaa8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdbaac | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdbab0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdbac0 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdbac4 | load | 4 | `abs 0xdbc68` | `-` |
| `ldr` at 0xdbac8 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdbad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbad8 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbadc | load | 4 | `abs 0xdbc6c` | `-` |
| `str` at 0xdbae8 | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xdbaf4 | store | 4 | `sp -0x20` | `0x58` |
| `ldr` at 0xdbaf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbafc | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xdbb00 | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xdbb04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdbb10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbb14 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbb18 | load | 4 | `abs 0xdbc70` | `-` |
| `str` at 0xdbb24 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xdbb2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbb30 | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xdbb34 | store | 4 | `sp -0x2c` | `[[sp-24]+44]` |
| `ldr` at 0xdbb38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdbb3c | load | 4 | `abs 0xdbc74` | `-` |
| `ldr` at 0xdbb48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbb4c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbb50 | load | 4 | `abs 0xdbc78` | `-` |
| `str` at 0xdbb5c | store | 4 | `sp -0xd8` | `0x0` |
| `str` at 0xdbb68 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xdbb70 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xdbb74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbb78 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdbb7c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdbb8c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdbb94 | store | 4 | `sp -0x20` | `0x5a` |
| `ldr` at 0xdbb98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbb9c | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xdbba0 | store | 4 | `sp -0x30` | `[[sp-24]+48]` |
| `ldr` at 0xdbba4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbba8 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdbbac | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdbbbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdbbc4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbbc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbbcc | load | 4 | `[sp-24] +0x34` | `-` |
| `str` at 0xdbbd0 | store | 4 | `sp -0x34` | `[[sp-24]+52]` |
| `ldr` at 0xdbbd4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xdbbd8 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0xdbbdc | load | 4 | `sp -0x34` | `-` |
| `str` at 0xdbbe0 | store | 4 | `sp -0xe8` | `?` |
| `str` at 0xdbbe8 | store | 4 | `sp -0xe4` | `sp-56` |
| `str` at 0xdbbf0 | store | 4 | `sp -0xe0` | `sp-40` |
| `ldr` at 0xdbbf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbbf8 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xdbbfc | store | 4 | `sp -0xdc` | `[[sp-28]]` |
| `ldr` at 0xdbc00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbc04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdbc08 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xdbc20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbc24 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbc28 | load | 4 | `abs 0xdbc6c` | `-` |
| `str` at 0xdbc34 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xdbc3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbc40 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbc44 | load | 4 | `abs 0xdbc7c` | `-` |
| `ldr` at 0xdbc48 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xdbc4c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xdbc58 | store | 4 | `sp -0xd8` | `0x0` |
| `ldr` at 0xdbc5c | load | 4 | `sp -0xd8` | `-` |
| `ldm` at 0xdbc64 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdbc64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdbc64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdbc64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbab8 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdbae0 | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdbb1c | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121ab8` | `?` | `[sp-28]` |
| `bl` at 0xdbb54 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121aec` | `[[sp-24]+44]` | `[sp-28]` |
| `bl` at 0xdbc10 | `(unknown callee) 0xd12a4` | `?` | `[[sp-24]+40]` | `[[sp-24]+44]` | `0x0` |
| `bl` at 0xdbc2c | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdbc50 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121b14` | `[sp-56]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdbad0 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `bls` at 0xdbb0c | `[[sp-24]+40] <= 0xff (unsigned, 32-bit)` | `[[sp-24]+40] > 0xff (unsigned, 32-bit)` |
| `bls` at 0xdbb44 | `[[sp-24]+44] <= 0xffff (unsigned, 32-bit)` | `[[sp-24]+44] > 0xffff (unsigned, 32-bit)` |
| `beq` at 0xdbb88 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xdbbb8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdbc1c | `ret(0xd12a4) != 0x0` | `ret(0xd12a4) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdbc64 | `r0` | `0x0` |

## `smpcmd_scan_config`

- `smpcmd_scan_config`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbc80 to 0xdbd18.
  Established. The indirect call carries `ret(0x1943c)`, unknown, unknown, `[0x12da6c]` (`mov` at 0xdbce4).
  Nonzero `[0x297378]` takes `0xdbcd8`, while zero falls through (`bne` at 0xdbcb8).
  A word store assigns `ret(0x1943c)` to entry `r1 +0x0` (`str` at 0xdbca8).
  Configuration callback ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbc90 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdbc94 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdbc98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdbca8 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdbcac | load | 4 | `abs 0xdbd0c` | `-` |
| `ldr` at 0xdbcb0 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdbcbc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbcc0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbcc4 | load | 4 | `abs 0xdbd10` | `-` |
| `str` at 0xdbcd0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdbcd8 | load | 4 | `abs 0xdbd14` | `-` |
| `ldr` at 0xdbcdc | load | 4 | `abs 0x12da6c` | `-` |
| `ldr` at 0xdbce8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbcec | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbcf0 | load | 4 | `abs 0xdbd10` | `-` |
| `str` at 0xdbcfc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdbd00 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xdbd08 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdbd08 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdbd08 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdbd08 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbca0 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdbcc8 | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `mov` at 0xdbce4 | `(unknown callee) (unknown target)` | `ret(0x1943c)` | `?` | `?` | `[0x12da6c]` |
| `bl` at 0xdbcf4 | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdbcb8 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdbd08 | `r0` | `0x0` |

## `smpcmd_scan_reset`

- `smpcmd_scan_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdb954 to 0xdba98.
  Established. The unnamed `0xd0754` calls carry leading candidates `0x0` and `0x1`, each followed by `[sp-32]`, `[[sp-28]]`, `[sp-28]` (`bl` at 0xdb9e8, `bl` at 0xdba3c).
  The first nonzero call result takes `0xdba14`, while zero falls through (`bne` at 0xdb9f4).
  Scan reset request policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdb964 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdb968 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdb96c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdb97c | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdb980 | load | 4 | `abs 0xdba8c` | `-` |
| `ldr` at 0xdb984 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdb990 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb994 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdb998 | load | 4 | `abs 0xdba90` | `-` |
| `str` at 0xdb9a4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xdb9bc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdb9c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb9c4 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdb9c8 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdb9d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb9e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdb9e4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdb9f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdb9fc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdba00 | load | 4 | `abs 0xdba90` | `-` |
| `str` at 0xdba0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdba14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdba18 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdba1c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdba2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdba38 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdba4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba50 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdba54 | load | 4 | `abs 0xdba90` | `-` |
| `str` at 0xdba60 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdba68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdba6c | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdba70 | load | 4 | `abs 0xdba94` | `-` |
| `str` at 0xdba7c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xdba80 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xdba88 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdba88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdba88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdba88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb974 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdb99c | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdb9ac | `(unknown callee) 0xd9588` | `ret(0x1943c)` | `?` | `?` | `[0x297378]` |
| `bl` at 0xdb9e8 | `(unknown callee) 0xd0754` | `0x0` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xdba04 | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdba3c | `(unknown callee) 0xd0754` | `0x1` | `[sp-32]` | `[[sp-28]]` | `[sp-28]` |
| `bl` at 0xdba58 | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdba74 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121a94` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb98c | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `beq` at 0xdb9d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdb9f4 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |
| `beq` at 0xdba28 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xdba48 | `ret(0xd0754) != 0x0` | `ret(0xd0754) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdba88 | `r0` | `0x0` |

## `smpcmd_scan_debug`

- `smpcmd_scan_debug`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc330 to 0xdc3d8.
  Established. An included word store assigns `[[sp-24]+40]` to absolute `0x29737c` (`str` at 0xdc3a0).
  Unsigned `[[sp-24]+40] <= 0x4` takes `0xdc398`, while greater values fall through (`bls` at 0xdc384).
  The unnamed `0x197a8` call carries `[[sp-28]]`, `0x121d24`, `[0x29737c]`, `[sp-28]` (`bl` at 0xdc3b8).
  Debug level effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc340 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc344 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdc348 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc358 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdc35c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc360 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdc364 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdc370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc374 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xdc378 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xdc37c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdc388 | load | 4 | `abs 0xdc3cc` | `-` |
| `ldr` at 0xdc398 | load | 4 | `abs 0xdc3d0` | `-` |
| `ldr` at 0xdc39c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdc3a0 | store | 4 | `abs 0x29737c` | `[[sp-24]+40]` |
| `ldr` at 0xdc3a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc3a8 | load | 4 | `abs 0xdc3d0` | `-` |
| `ldr` at 0xdc3ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc3b0 | load | 4 | `abs 0xdc3d4` | `-` |
| `ldr` at 0xdc3b4 | load | 4 | `abs 0x29737c` | `-` |
| `ldm` at 0xdc3c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdc3c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc3c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc3c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc350 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdc390 | `(unknown callee) 0x221f0` | `0x121ce8` | `0x4` | `?` | `[[sp-24]+40]` |
| `bl` at 0xdc3b8 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121d24` | `[0x29737c]` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc36c | `[[[sp-24]+112]] != 0x1` | `[[[sp-24]+112]] == 0x1` |
| `bls` at 0xdc384 | `[[sp-24]+40] <= 0x4 (unsigned, 32-bit)` | `[[sp-24]+40] > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc3c8 | `r0` | `0x0` |

## `smpcmd_scan_port`

- `smpcmd_scan_port`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdbd18 to 0xdbe34.
  Established. The included indirect call carries `[sp-32]`, `[sp-36]`, unknown, `[0x12da68]` (`mov` at 0xdbde4).
  Nonzero `[0x297378]` takes `0xdbd70`, while zero falls through (`bne` at 0xdbd50).
  Included word reads fetch absolute `0x297388` and `0x29738c` (`ldr` at 0xdbdfc, `ldr` at 0xdbe00).
  Port selection callback semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdbd28 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdbd2c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdbd30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdbd40 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdbd44 | load | 4 | `abs 0xdbe1c` | `-` |
| `ldr` at 0xdbd48 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdbd54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbd58 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbd5c | load | 4 | `abs 0xdbe20` | `-` |
| `str` at 0xdbd68 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdbd70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbd74 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdbd78 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdbd84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbd88 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdbd8c | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdbd98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbd9c | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xdbda0 | store | 4 | `sp -0x20` | `[[sp-24]+40]` |
| `ldr` at 0xdbda4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbda8 | load | 4 | `[sp-24] +0x2c` | `-` |
| `str` at 0xdbdac | store | 4 | `sp -0x24` | `[[sp-24]+44]` |
| `ldr` at 0xdbdb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdbdb8 | load | 4 | `[sp-24] +0x28` | `-` |
| `str` at 0xdbdbc | store | 4 | `sp -0x24` | `[[sp-24]+40]` |
| `ldr` at 0xdbdc4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdbdcc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xdbdd0 | load | 4 | `abs 0xdbe24` | `-` |
| `ldr` at 0xdbdd4 | load | 4 | `abs 0x12da68` | `-` |
| `ldr` at 0xdbdd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xdbddc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdbde8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdbdec | load | 4 | `abs 0xdbe28` | `-` |
| `ldr` at 0xdbdf0 | load | 4 | `abs 0xdbe2c` | `-` |
| `ldr` at 0xdbdf4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdbdf8 | load | 4 | `abs 0xdbe30` | `-` |
| `ldr` at 0xdbdfc | load | 4 | `abs 0x297388` | `-` |
| `ldr` at 0xdbe00 | load | 4 | `abs 0x29738c` | `-` |
| `str` at 0xdbe0c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdbe10 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdbe18 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdbe18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdbe18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdbe18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdbd38 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdbd60 | `(unknown callee) 0x19808` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `mov` at 0xdbde4 | `(unknown callee) (unknown target)` | `[sp-32]` | `[sp-36]` | `?` | `[0x12da68]` |
| `bl` at 0xdbe04 | `(unknown callee) 0x197a8` | `[[sp-28]]` | `0x121b44` | `[0x297388]` | `[0x29738c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdbd50 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `beq` at 0xdbd80 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `bne` at 0xdbd94 | `[[[sp-24]+112]] != 0x2` | `[[[sp-24]+112]] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdbe18 | `r0` | `0x0` |

## `smpcmd_scan_set`

- `smpcmd_scan_set`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc3d8 to 0xdc4d4.
  Established. An included word store preserves `ret(0xd9588)` at `sp -0x24` (`str` at 0xdc44c).
  The unnamed `0xd98e0` call carries `[sp-36]`, unknown, unknown, unknown (`bl` at 0xdc4a8).
  The subsequent unnamed `0xd96ac` call carries `[[sp-28]]`, unknown, unknown, `[sp-28]` (`bl` at 0xdc4b4).
  Requested TAP bit meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdc3e8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xdc3ec | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xdc3f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdc400 | store | 4 | `r1 +0x0` | `ret(0x1943c)` |
| `ldr` at 0xdc404 | load | 4 | `abs 0xdc4cc` | `-` |
| `ldr` at 0xdc408 | load | 4 | `abs 0x297378` | `-` |
| `ldr` at 0xdc414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc418 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xdc41c | load | 4 | `abs 0xdc4d0` | `-` |
| `str` at 0xdc428 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdc430 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc434 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdc438 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xdc44c | store | 4 | `sp -0x24` | `ret(0xd9588)` |
| `ldr` at 0xdc450 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc458 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdc460 | load | 4 | `[[sp-24]+112] +0x4` | `-` |
| `ldr` at 0xdc46c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc470 | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdc474 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc478 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `str` at 0xdc480 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdc48c | load | 4 | `[sp-24] +0x70` | `-` |
| `ldr` at 0xdc490 | load | 4 | `[[sp-24]+112] +0x0` | `-` |
| `ldr` at 0xdc498 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xdc4a0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xdc4a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xdc4ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdc4b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xdc4bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xdc4c0 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xdc4c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xdc4c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc4c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc4c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc3f8 | `(unknown callee) 0x1943c` | `0x50` | `r1` | `r2` | `r3` |
| `bl` at 0xdc420 | `(unknown callee) 0x195e0` | `[[sp-28]]` | `0x121a90` | `?` | `[sp-28]` |
| `bl` at 0xdc444 | `(unknown callee) 0xd9588` | `ret(0x1943c)` | `?` | `?` | `[[[sp-24]+112]]` |
| `bl` at 0xdc4a8 | `(unknown callee) 0xd98e0` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xdc4b4 | `(unknown callee) 0xd96ac` | `[[sp-28]]` | `?` | `?` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdc410 | `[0x297378] != 0x0` | `[0x297378] == 0x0` |
| `beq` at 0xdc440 | `[[[sp-24]+112]] == 0x0` | `[[[sp-24]+112]] != 0x0` |
| `beq` at 0xdc468 | `[[[sp-24]+112]+4] == 0x0` | `[[[sp-24]+112]+4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc4c8 | `r0` | `0x0` |

## `scanpktInit`

- `scanpktInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdc69c to 0xdc82c.
  Established. Zero `[0x29739c]` takes `0xdc6dc`, while nonzero falls through (`beq` at 0xdc6b8).
  An included absolute word assignment writes `[0x29739c]+1` to `0x29739c` (`str` at 0xdc6ec).
  The unnamed `0x2066c` call carries `0x29c608`, `0x121e18`, `0xdc82c`, `0x0` (`bl` at 0xdc78c).
  Scan packet startup dependencies remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdc6ac | load | 4 | `abs 0xdc800` | `-` |
| `ldr` at 0xdc6b0 | load | 4 | `abs 0x29739c` | `-` |
| `ldr` at 0xdc6bc | load | 4 | `abs 0xdc804` | `-` |
| `str` at 0xdc6c0 | store | 4 | `sp -0x2c` | `0x121da0` |
| `ldr` at 0xdc6c8 | load | 4 | `abs 0xdc808` | `-` |
| `ldr` at 0xdc6dc | load | 4 | `abs 0xdc800` | `-` |
| `ldr` at 0xdc6e0 | load | 4 | `abs 0xdc800` | `-` |
| `ldr` at 0xdc6e4 | load | 4 | `abs 0x29739c` | `-` |
| `str` at 0xdc6ec | store | 4 | `abs 0x29739c` | `[0x29739c]+1` |
| `ldr` at 0xdc6f0 | load | 4 | `abs 0xdc80c` | `-` |
| `ldr` at 0xdc6f4 | load | 4 | `abs 0xdc810` | `-` |
| `str` at 0xdc704 | store | 4 | `sp -0x14` | `ret(0xa8af0)` |
| `ldr` at 0xdc708 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc720 | load | 4 | `abs 0xdc814` | `-` |
| `str` at 0xdc724 | store | 4 | `sp -0x2c` | `0x121df0` |
| `str` at 0xdc728 | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0xdc730 | load | 4 | `abs 0xdc808` | `-` |
| `ldr` at 0xdc744 | load | 4 | `abs 0xdc818` | `-` |
| `str` at 0xdc758 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xdc760 | store | 4 | `sp -0x28` | `0x800` |
| `str` at 0xdc768 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xdc770 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xdc778 | store | 4 | `sp -0x1c` | `0x4` |
| `ldr` at 0xdc77c | load | 4 | `abs 0xdc81c` | `-` |
| `ldr` at 0xdc780 | load | 4 | `abs 0xdc820` | `-` |
| `ldr` at 0xdc784 | load | 4 | `abs 0xdc824` | `-` |
| `ldr` at 0xdc7a0 | load | 4 | `abs 0xdc818` | `-` |
| `ldr` at 0xdc7a4 | load | 4 | `abs 0xdc818` | `-` |
| `ldr` at 0xdc7a8 | load | 4 | `abs 0x29c600` | `-` |
| `ldr` at 0xdc7b0 | load | 4 | `abs 0xdc80c` | `-` |
| `str` at 0xdc7bc | store | 4 | `sp -0x14` | `ret(0xa76a4)` |
| `ldr` at 0xdc7c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc7cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdc7d8 | load | 4 | `abs 0xdc828` | `-` |
| `str` at 0xdc7dc | store | 4 | `sp -0x2c` | `0x121e24` |
| `str` at 0xdc7e0 | store | 4 | `sp -0x28` | `ret(0x1f8e0)` |
| `ldr` at 0xdc7e8 | load | 4 | `abs 0xdc808` | `-` |
| `ldm` at 0xdc7fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xdc7fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xdc7fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdc6d4 | `(unknown callee) 0x200a0` | `0x4` | `0x121dcc` | `0x42` | `0x0` |
| `bl` at 0xdc6fc | `(unknown callee) 0xa8af0` | `0x29c60c` | `0x121de4` | `0x14` | `[0x29739c]+1` |
| `bl` at 0xdc718 | `(unknown callee) 0x1f8e0` | `ret(0xa8af0)` | `?` | `?` | `ret(0xa8af0)` |
| `bl` at 0xdc73c | `(unknown callee) 0x200a0` | `0x4` | `0x121dcc` | `0x5a` | `0x0` |
| `bl` at 0xdc750 | `(unknown callee) 0xa89d0` | `0x29c600` | `0xf` | `0x0` | `ret(0xa8af0)` |
| `bl` at 0xdc78c | `(unknown callee) 0x2066c` | `0x29c608` | `0x121e18` | `0xdc82c` | `0x0` |
| `bl` at 0xdc7b4 | `(unknown callee) 0xa76a4` | `0x29c600` | `[0x29c600]` | `0x1` | `0x29c60c` |
| `bl` at 0xdc7d0 | `(unknown callee) 0x1f8e0` | `ret(0xa76a4)` | `?` | `?` | `ret(0xa76a4)` |
| `bl` at 0xdc7f4 | `(unknown callee) 0x200a0` | `0x4` | `0x121dcc` | `0x74` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdc6b8 | `[0x29739c] == 0x0` | `[0x29739c] != 0x0` |
| `beq` at 0xdc710 | `ret(0xa8af0) == 0x0` | `ret(0xa8af0) != 0x0` |
| `beq` at 0xdc798 | `ret(0x2066c) == 0x0` | `ret(0x2066c) != 0x0` |
| `beq` at 0xdc7c8 | `ret(0xa76a4) == 0x0` | `ret(0xa76a4) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xdc7fc | `r0` | `?` |

## `JTAG_LLP_Setup`

- `JTAG_LLP_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xe69f4 to 0xe6a34.
  Established. Absolute word assignments put `0xe7540` at `0x12da74` and `0xe7044` at `0x12da78` (`str` at 0xe6a08, `str` at 0xe6a14).
  The integer return record contains `0x0` (`ldm` at 0xe6a20).
  LLP pointer registration purpose remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xe6a00 | load | 4 | `abs 0xe6a24` | `-` |
| `ldr` at 0xe6a04 | load | 4 | `abs 0xe6a28` | `-` |
| `str` at 0xe6a08 | store | 4 | `abs 0x12da74` | `0xe7540` |
| `ldr` at 0xe6a0c | load | 4 | `abs 0xe6a2c` | `-` |
| `ldr` at 0xe6a10 | load | 4 | `abs 0xe6a30` | `-` |
| `str` at 0xe6a14 | store | 4 | `abs 0x12da78` | `0xe7044` |
| `ldm` at 0xe6a20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xe6a20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xe6a20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xe6a20 | `r0` | `0x0` |
