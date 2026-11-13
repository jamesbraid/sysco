# ARM Linux 2.6 GPIO and I2C transfer paths in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#6` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0xa936a8`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `e57ab8ecd48858e39ecdd0050a23e444a491c35e87f9d8ebb48881fc25229a06`.
Inner executable SHA-256: `b33154e53b00531623db6ff365e80b1c1825bc9427961c59fdc132c24f0c819e`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1629656 bytes.
Segment SHA-256: `f6491062ef0a2afd254f8510f82b4814c563c9256691f9a2b393cf6e84ba7fd8`. Analysed routine spans use the exact
ELF function symbols and sizes for named routines. The unnamed call-target span ends at
the next named function and its end is inferred. Spans can include trailing literal words.

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

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-gpio-transfer-facts.txt) retain all tool sections.

## `0x10af88`

- `0x10af88`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10af88 to 0x10b338.
  Inferred. An unnamed `0x20520` call carries `[0x1a04bc]`, `0x703`, `[sp-21]:1`, `0x1a04bc` (`bl` at 0x10b044);
  its signed nonnegative result takes `0x10b084`, while negative falls through (`bge` at 0x10b050).
  A second unnamed `0x20520` call carries `[0x1a04bc]`, `0x707`, `sp-76`, `0x1a04bc` (`bl` at 0x10b24c).
  Included halfword assignments write `0x4000` and `0x1` to `sp -0x6a` on separate paths (`strh` at 0x10b1bc, `strh` at 0x10b1d4).
  The transfer record contract remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10af98 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10af9c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x10afa0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x10afa4 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x10afac | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0x10afb0 | store | 2 | `sp -0x22` | `([sp] & 0xffff)` |
| `ldr` at 0x10afb4 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x10afb8 | store | 1 | `sp -0x29` | `(r2 & 0xff)` |
| `str` at 0x10afc8 | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x10afd0 | load | 4 | `sp -0x60` | `-` |
| `str` at 0x10afd4 | store | 4 | `[sp-96] +0x0` | `sp-68` |
| `ldr` at 0x10afd8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x10afe8 | store | 4 | `sp -0x64` | `0x2` |
| `str` at 0x10aff4 | store | 4 | `sp -0x64` | `0x1` |
| `ldr` at 0x10aff8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x10affc | load | 4 | `sp -0x60` | `-` |
| `str` at 0x10b000 | store | 4 | `[sp-96] +0x4` | `[sp-100]` |
| `str` at 0x10b014 | store | 4 | `(unknown)` | `sp-68` |
| `str` at 0x10b01c | store | 4 | `(unknown)` | `0x1` |
| `str` at 0x10b024 | store | 4 | `sp -0x58` | `0x0` |
| `ldrb` at 0x10b028 | load | 1 | `sp -0x15` | `-` |
| `strb` at 0x10b030 | store | 1 | `sp -0x15` | `?` |
| `ldr` at 0x10b034 | load | 4 | `abs 0x10b318` | `-` |
| `ldrb` at 0x10b038 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b03c | load | 4 | `abs 0x1a04bc` | `-` |
| `ldr` at 0x10b040 | load | 4 | `abs 0x10b31c` | `-` |
| `ldr` at 0x10b060 | load | 4 | `abs 0x10b320` | `-` |
| `ldrb` at 0x10b064 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x10b068 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x10b06c | load | 4 | `abs 0x10b324` | `-` |
| `ldr` at 0x10b070 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10b078 | load | 4 | `abs 0x10b328` | `-` |
| `str` at 0x10b07c | store | 4 | `sp -0x5c` | `0xfffffc16` |
| `ldr` at 0x10b084 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10b090 | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x10b0b4 | load | 1 | `sp -0x15` | `-` |
| `strh` at 0x10b0b8 | store | 2 | `(unknown)` | `([sp-21]:1 & 0xffff)` |
| `ldr` at 0x10b0bc | load | 4 | `sp -0x58` | `-` |
| `strh` at 0x10b0e4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x10b0e8 | load | 4 | `sp -0x58` | `-` |
| `strh` at 0x10b110 | store | 2 | `(unknown)` | `0x1` |
| `ldr` at 0x10b114 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x10b13c | store | 4 | `(unknown)` | `sp-41` |
| `ldr` at 0x10b140 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x10b148 | store | 4 | `sp -0x58` | `[sp-88]+1` |
| `ldr` at 0x10b14c | load | 4 | `sp -0x58` | `-` |
| `ldrb` at 0x10b170 | load | 1 | `sp -0x15` | `-` |
| `strh` at 0x10b174 | store | 2 | `(unknown)` | `([sp-21]:1 & 0xffff)` |
| `ldr` at 0x10b178 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x10b19c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x10b1a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10b1ac | load | 4 | `sp -0x58` | `-` |
| `strh` at 0x10b1bc | store | 2 | `sp -0x6a` | `0x4000` |
| `strh` at 0x10b1c8 | store | 2 | `sp -0x6a` | `0x0` |
| `strh` at 0x10b1d4 | store | 2 | `sp -0x6a` | `0x1` |
| `ldrh` at 0x10b1d8 | load | 2 | `sp -0x6a` | `-` |
| `ldr` at 0x10b1dc | load | 4 | `sp -0x68` | `-` |
| `strh` at 0x10b1e0 | store | 2 | `[sp-104] +0x2` | `?` |
| `ldr` at 0x10b1e4 | load | 4 | `sp -0x58` | `-` |
| `ldrh` at 0x10b208 | load | 2 | `sp -0x22` | `-` |
| `strh` at 0x10b20c | store | 2 | `(unknown)` | `([sp-34]:2 & 0xffff)` |
| `ldr` at 0x10b210 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x10b234 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10b238 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x10b23c | load | 4 | `abs 0x10b318` | `-` |
| `ldr` at 0x10b244 | load | 4 | `abs 0x1a04bc` | `-` |
| `ldr` at 0x10b248 | load | 4 | `abs 0x10b32c` | `-` |
| `ldr` at 0x10b264 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10b268 | store | 4 | `sp -0x70` | `[ret(0x20184)]` |
| `ldr` at 0x10b26c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b278 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b284 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b290 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b2a0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b2ac | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x10b2bc | load | 4 | `abs 0x10b330` | `-` |
| `str` at 0x10b2c0 | store | 4 | `sp -0x5c` | `0xfffffc18` |
| `str` at 0x10b2cc | store | 4 | `sp -0x5c` | `?` |
| `ldr` at 0x10b2d4 | load | 4 | `abs 0x10b334` | `-` |
| `str` at 0x10b2d8 | store | 4 | `sp -0x5c` | `0xfffffc14` |
| `ldr` at 0x10b2e0 | load | 4 | `abs 0x10b334` | `-` |
| `str` at 0x10b2e4 | store | 4 | `sp -0x5c` | `0xfffffc14` |
| `ldr` at 0x10b2ec | load | 4 | `abs 0x10b334` | `-` |
| `str` at 0x10b2f0 | store | 4 | `sp -0x5c` | `0xfffffc14` |
| `ldr` at 0x10b2f8 | load | 4 | `abs 0x10b328` | `-` |
| `str` at 0x10b2fc | store | 4 | `sp -0x5c` | `0xfffffc16` |
| `str` at 0x10b308 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0x10b30c | load | 4 | `sp -0x5c` | `-` |
| `ldm` at 0x10b314 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10b314 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10b314 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10b044 | `(unknown callee) 0x20520` | `[0x1a04bc]` | `0x703` | `[sp-21]:1` | `0x1a04bc` |
| `bl` at 0x10b054 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10b074 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x18c9cc` | `[sp-21]:1` | `[ret(0x20184)]` |
| `bl` at 0x10b24c | `(unknown callee) 0x20520` | `[0x1a04bc]` | `0x707` | `sp-76` | `0x1a04bc` |
| `bl` at 0x10b25c | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x10afe0 | `[sp-28] < 0x0 (signed, 32-bit)` | `[sp-28] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x10b050 | `ret(0x20520) >= 0x0 (signed, 32-bit)` | `ret(0x20520) < 0x0 (signed, 32-bit)` |
| `blt` at 0x10b08c | `[sp-28] < 0x0 (signed, 32-bit)` | `[sp-28] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x10b1a8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x10b1b4 | `[sp-88] == 0x0` | `[sp-88] != 0x0` |
| `bge` at 0x10b258 | `ret(0x20520) >= 0x0 (signed, 32-bit)` | `ret(0x20520) < 0x0 (signed, 32-bit)` |
| `beq` at 0x10b274 | `[ret(0x20184)] == 0x6e` | `[ret(0x20184)] != 0x6e` |
| `bgt` at 0x10b280 | `[ret(0x20184)] > 0x6e (signed, 32-bit)` | `[ret(0x20184)] <= 0x6e (signed, 32-bit)` |
| `beq` at 0x10b28c | `[ret(0x20184)] == 0x6` | `[ret(0x20184)] != 0x6` |
| `beq` at 0x10b298 | `[ret(0x20184)] == 0x47` | `[ret(0x20184)] != 0x47` |
| `beq` at 0x10b2a8 | `[ret(0x20184)] == 0x6f` | `[ret(0x20184)] != 0x6f` |
| `beq` at 0x10b2b4 | `[ret(0x20184)] == 0x79` | `[ret(0x20184)] != 0x79` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10b314 | `r0` | `?` |

## `i2cGetSwitch`

- `i2cGetSwitch`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a840 to 0x10a87c.
  Established. Included word reads address `0x31beec` and `[0x31beec] +0x0` (`ldr` at 0x10a85c, `ldr` at 0x10a860).
  A byte assignment writes an unknown value to entry `r0 +0x0` (`strb` at 0x10a868).
  The return candidate is zero (`ldm` at 0x10a874).
  Switch-bit ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a850 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x10a854 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10a858 | load | 4 | `abs 0x10a878` | `-` |
| `ldr` at 0x10a85c | load | 4 | `abs 0x31beec` | `-` |
| `ldr` at 0x10a860 | load | 4 | `[0x31beec] +0x0` | `-` |
| `strb` at 0x10a868 | store | 1 | `r0 +0x0` | `?` |
| `ldm` at 0x10a874 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10a874 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a874 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a874 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a874 | `r0` | `0x0` |

## `i2cSetBusSelect`

- `i2cSetBusSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a5a8 to 0x10a5d0.
  Established. A word assignment stores entry `r0` at `0x1a04c0` (`str` at 0x10a5c4).
  The integer return record preserves entry `r0` (`ldm` at 0x10a5c8).
  Bus-select callback ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a5b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x10a5bc | load | 4 | `abs 0x10a5cc` | `-` |
| `ldr` at 0x10a5c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10a5c4 | store | 4 | `abs 0x1a04c0` | `r0` |
| `ldm` at 0x10a5c8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10a5c8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a5c8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a5c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a5c8 | `r0` | `r0` |

## `i2cSetBusDeselect`

- `i2cSetBusDeselect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a5d0 to 0x10a5f8.
  Established. The word at `0x314688` receives entry `r0` (`str` at 0x10a5ec).
  Entry `r0` survives as the return candidate (`ldm` at 0x10a5f0).
  The registered value and bus-deselection hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a5e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x10a5e4 | load | 4 | `abs 0x10a5f4` | `-` |
| `ldr` at 0x10a5e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10a5ec | store | 4 | `abs 0x314688` | `r0` |
| `ldm` at 0x10a5f0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10a5f0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a5f0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a5f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a5f0 | `r0` | `r0` |

## `i2cSetBusSlotMap`

- `i2cSetBusSlotMap`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a5f8 to 0x10a620.
  Established. Entry `r0` is assigned to the word at `0x314694` (`str` at 0x10a614).
  The integer return candidate retains that entry register (`ldm` at 0x10a618).
  The slot-map record layout remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10a608 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x10a60c | load | 4 | `abs 0x10a61c` | `-` |
| `ldr` at 0x10a610 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10a614 | store | 4 | `abs 0x314694` | `r0` |
| `ldm` at 0x10a618 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x10a618 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a618 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a618 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a618 | `r0` | `r0` |

## `i2cResetBusSelect`

- `i2cResetBusSelect`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10a620 to 0x10a688.
  Established. Zero `[0x1a04c0]` takes `0x10a670`, while nonzero falls through (`beq` at 0x10a63c).
  The unnamed `0x10a87c` call carries `0x1`, entry `r1`, entry `r2` and `[0x1a04c0]` (`bl` at 0x10a644).
  A subsequent unnamed `0x10a9c0` call carries `0x0`, unknown, unknown and `ret(0x10a87c)` (`bl` at 0x10a66c).
  Bus-selection reset effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x10a630 | load | 4 | `abs 0x10a684` | `-` |
| `ldr` at 0x10a634 | load | 4 | `abs 0x1a04c0` | `-` |
| `str` at 0x10a64c | store | 4 | `sp -0x14` | `ret(0x10a87c)` |
| `ldr` at 0x10a650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x10a65c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x10a660 | store | 4 | `sp -0x18` | `ret(0x10a87c)` |
| `str` at 0x10a674 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x10a678 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x10a680 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10a680 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10a680 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10a644 | `(unknown callee) 0x10a87c` | `0x1` | `r1` | `r2` | `[0x1a04c0]` |
| `bl` at 0x10a66c | `(unknown callee) 0x10a9c0` | `0x0` | `?` | `?` | `ret(0x10a87c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x10a63c | `[0x1a04c0] == 0x0` | `[0x1a04c0] != 0x0` |
| `beq` at 0x10a658 | `ret(0x10a87c) == 0x0` | `ret(0x10a87c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10a680 | `r0` | `?` |

## `l1ioGpioInit`

- `l1ioGpioInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e47c to 0x10e500.
  Established. The unnamed `0x2046c` call carries `0x18d558`, `0x2`, entry `r2` and entry `r3` (`bl` at 0x10e498);
  its result is assigned to the word at `0x1a04f8` (`str` at 0x10e4a4).
  The signed condition `[0x1a04f8] >= 0` takes `0x10e4e8`, while negative falls through (`bge` at 0x10e4b0).
  GPIO device initialization responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e48c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x10e490 | load | 4 | `abs 0x10e4f0` | `-` |
| `ldr` at 0x10e4a0 | load | 4 | `abs 0x10e4f4` | `-` |
| `str` at 0x10e4a4 | store | 4 | `abs 0x1a04f8` | `ret(0x2046c)` |
| `ldr` at 0x10e4a8 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e4bc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10e4c8 | load | 4 | `abs 0x10e4f8` | `-` |
| `str` at 0x10e4cc | store | 4 | `sp -0x1c` | `0x18d568` |
| `str` at 0x10e4d0 | store | 4 | `sp -0x18` | `ret(0x20154)` |
| `ldr` at 0x10e4d8 | load | 4 | `abs 0x10e4fc` | `-` |
| `ldm` at 0x10e4ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e4ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e4ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e498 | `(unknown callee) 0x2046c` | `0x18d558` | `0x2` | `r2` | `r3` |
| `bl` at 0x10e4b4 | `(unknown callee) 0x20184` | `ret(0x2046c)` | `?` | `ret(0x2046c)` | `[0x1a04f8]` |
| `bl` at 0x10e4c0 | `(unknown callee) 0x20154` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10e4e4 | `(unknown callee) 0x22070` | `0x1` | `0x18d580` | `0x34` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e4b0 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e4ec | `r0` | `?` |

## `l1ioGpioReset`

- `l1ioGpioReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e500 to 0x10e6ec.
  Established. A byte read addresses entry `r1 +0x3` (`ldrb` at 0x10e540).
  Included unnamed `0x20520` calls carry `[0x1a04f8]`, request candidates `0x40046706` and `0x80046708` respectively, `sp-28` and `0x1a04f8` (`bl` at 0x10e560, `bl` at 0x10e670).
  Separate byte reads address `[sp-24] +0x6`, `[sp-24] +0x5` and `[sp-24] +0x4` (`ldrb` at 0x10e584, `ldrb` at 0x10e5cc, `ldrb` at 0x10e618).
  GPIO reset bit definitions and request semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e510 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e514 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x10e51c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x10e520 | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e524 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e530 | load | 4 | `abs 0x10e6d4` | `-` |
| `str` at 0x10e534 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0x10e53c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e540 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e544 | store | 1 | `sp -0x1c` | `([r1+3]:1 & 0xff)` |
| `strb` at 0x10e54c | store | 1 | `sp -0x1b` | `0x0` |
| `ldr` at 0x10e550 | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e558 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e55c | load | 4 | `abs 0x10e6d8` | `-` |
| `ldr` at 0x10e578 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e57c | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `ldr` at 0x10e580 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e584 | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0x10e588 | store | 1 | `sp -0x1b` | `([[sp-24]+6]:1 & 0xff)` |
| `ldr` at 0x10e58c | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e594 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e598 | load | 4 | `abs 0x10e6dc` | `-` |
| `ldr` at 0x10e5ac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10e5c0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e5c4 | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `ldr` at 0x10e5c8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e5cc | load | 1 | `[sp-24] +0x5` | `-` |
| `strb` at 0x10e5d4 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x10e5d8 | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e5e0 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e5e4 | load | 4 | `abs 0x10e6e0` | `-` |
| `ldr` at 0x10e5f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10e60c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e610 | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `ldr` at 0x10e614 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e618 | load | 1 | `[sp-24] +0x4` | `-` |
| `strb` at 0x10e620 | store | 1 | `sp -0x1b` | `?` |
| `ldr` at 0x10e624 | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e62c | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e630 | load | 4 | `abs 0x10e6e4` | `-` |
| `ldr` at 0x10e644 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10e658 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e65c | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `ldr` at 0x10e660 | load | 4 | `abs 0x10e6d0` | `-` |
| `ldr` at 0x10e668 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e66c | load | 4 | `abs 0x10e6e8` | `-` |
| `ldr` at 0x10e680 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10e694 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e698 | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `ldr` at 0x10e69c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x10e6a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x10e6b4 | store | 4 | `sp -0x24` | `ret(0x10eb40)` |
| `str` at 0x10e6c0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10e6c4 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10e6cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e6cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e6cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e560 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046706` | `sp-28` | `0x1a04f8` |
| `bl` at 0x10e570 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10e59c | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046700` | `sp-28` | `0x1a04f8` |
| `bl` at 0x10e5b8 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `[sp-32]` |
| `bl` at 0x10e5e8 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046704` | `sp-28` | `0x1a04f8` |
| `bl` at 0x10e604 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `[sp-32]` |
| `bl` at 0x10e634 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046702` | `sp-28` | `0x1a04f8` |
| `bl` at 0x10e650 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `[sp-32]` |
| `bl` at 0x10e670 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x80046708` | `sp-28` | `0x1a04f8` |
| `bl` at 0x10e68c | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `[sp-32]` |
| `bl` at 0x10e6ac | `(unknown callee) 0x10eb40` | `[sp-32]` | `?` | `?` | `[sp-32]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e52c | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `beq` at 0x10e56c | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `beq` at 0x10e5a8 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `bne` at 0x10e5b4 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x10e5f4 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `bne` at 0x10e600 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x10e640 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `bne` at 0x10e64c | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x10e67c | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `bne` at 0x10e688 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `beq` at 0x10e6a4 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e6cc | `r0` | `?` |

## `l1ioGpioWrite`

- `l1ioGpioWrite`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e6ec to 0x10e798.
  Established. A byte assignment places `((r2 & 0xff) & 0xff)` at `sp -0x1f` (`strb` at 0x10e738).
  The unnamed `0x20520` call carries `[0x1a04f8]`, `0x40046700`, `sp-32` and `0x1a04f8` (`bl` at 0x10e74c);
  its zero result takes `0x10e778`, while nonzero falls through (`beq` at 0x10e758).
  Pin-write hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e6fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e700 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10e708 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x10e70c | load | 4 | `abs 0x10e78c` | `-` |
| `ldr` at 0x10e710 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e71c | load | 4 | `abs 0x10e790` | `-` |
| `str` at 0x10e720 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0x10e728 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e72c | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e730 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0x10e734 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x10e738 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0x10e73c | load | 4 | `abs 0x10e78c` | `-` |
| `ldr` at 0x10e744 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e748 | load | 4 | `abs 0x10e794` | `-` |
| `ldr` at 0x10e764 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e770 | store | 4 | `sp -0x24` | `ret(0x10eb40)` |
| `str` at 0x10e77c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10e780 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10e788 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e788 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e788 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e74c | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046700` | `sp-32` | `0x1a04f8` |
| `bl` at 0x10e75c | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10e768 | `(unknown callee) 0x10eb40` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e718 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `beq` at 0x10e758 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e788 | `r0` | `?` |

## `l1ioGpioRead`

- `l1ioGpioRead`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e798 to 0x10e844.
  Established. An unnamed `0x20520` call carries `[0x1a04f8]`, `0xc0046701`, `sp-32` and `0x1a04f8` (`bl` at 0x10e7ec).
  A zero result takes `0x10e818`, while nonzero falls through (`beq` at 0x10e7f8).
  The taken path assigns `([sp-31]:1 & 0xff)` to the byte at `[sp-28] +0x0` (`strb` at 0x10e820).
  GPIO sample meaning remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e7a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e7ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x10e7b0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x10e7b4 | load | 4 | `abs 0x10e838` | `-` |
| `ldr` at 0x10e7b8 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e7c4 | load | 4 | `abs 0x10e83c` | `-` |
| `str` at 0x10e7c8 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0x10e7d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e7d4 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e7d8 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0x10e7dc | load | 4 | `abs 0x10e838` | `-` |
| `ldr` at 0x10e7e4 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e7e8 | load | 4 | `abs 0x10e840` | `-` |
| `ldr` at 0x10e804 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e810 | store | 4 | `sp -0x24` | `ret(0x10eb40)` |
| `ldr` at 0x10e818 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10e81c | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x10e820 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0x10e828 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10e82c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10e834 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e834 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e7ec | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0xc0046701` | `sp-32` | `0x1a04f8` |
| `bl` at 0x10e7fc | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10e808 | `(unknown callee) 0x10eb40` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e7c0 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `beq` at 0x10e7f8 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e834 | `r0` | `?` |

## `l1ioGpioIntSet`

- `l1ioGpioIntSet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e844 to 0x10e8f0.
  Established. The byte at `sp -0x20` is assigned `([r1+3]:1 & 0xff)` (`strb` at 0x10e888).
  The unnamed `0x20520` call carries `[0x1a04f8]`, `0x40046706`, `sp-32` and `0x1a04f8` (`bl` at 0x10e8a4).
  A separate unnamed `0x10eb40` call carries `[ret(0x20184)]`, unknown, unknown and `ret(0x20184)` (`bl` at 0x10e8c0).
  Interrupt-mask hardware behavior remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e854 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e858 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x10e860 | store | 1 | `sp -0x19` | `(r2 & 0xff)` |
| `ldr` at 0x10e864 | load | 4 | `abs 0x10e8e4` | `-` |
| `ldr` at 0x10e868 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e874 | load | 4 | `abs 0x10e8e8` | `-` |
| `str` at 0x10e878 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0x10e880 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e884 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e888 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldrb` at 0x10e88c | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x10e890 | store | 1 | `sp -0x1f` | `((r2 & 0xff) & 0xff)` |
| `ldr` at 0x10e894 | load | 4 | `abs 0x10e8e4` | `-` |
| `ldr` at 0x10e89c | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e8a0 | load | 4 | `abs 0x10e8ec` | `-` |
| `ldr` at 0x10e8bc | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e8c8 | store | 4 | `sp -0x24` | `ret(0x10eb40)` |
| `str` at 0x10e8d4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10e8d8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10e8e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e8e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e8e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e8a4 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x40046706` | `sp-32` | `0x1a04f8` |
| `bl` at 0x10e8b4 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10e8c0 | `(unknown callee) 0x10eb40` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e870 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `beq` at 0x10e8b0 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e8e0 | `r0` | `?` |

## `l1ioGpioIntGet`

- `l1ioGpioIntGet`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e8f0 to 0x10e99c.
  Established. A word assignment writes `0xfffff3e4` to `sp -0x24` on one path (`str` at 0x10e920).
  The unnamed `0x20520` call carries `[0x1a04f8]`, `0xc0046707`, `sp-32` and `0x1a04f8` (`bl` at 0x10e944).
  A later byte assignment writes `([sp-31]:1 & 0xff)` to `[sp-28] +0x0` (`strb` at 0x10e978).
  Interrupt-state encoding remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e900 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e904 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x10e908 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x10e90c | load | 4 | `abs 0x10e990` | `-` |
| `ldr` at 0x10e910 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e91c | load | 4 | `abs 0x10e994` | `-` |
| `str` at 0x10e920 | store | 4 | `sp -0x24` | `0xfffff3e4` |
| `ldr` at 0x10e928 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e92c | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e930 | store | 1 | `sp -0x20` | `([r1+3]:1 & 0xff)` |
| `ldr` at 0x10e934 | load | 4 | `abs 0x10e990` | `-` |
| `ldr` at 0x10e93c | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e940 | load | 4 | `abs 0x10e998` | `-` |
| `ldr` at 0x10e95c | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x10e968 | store | 4 | `sp -0x24` | `ret(0x10eb40)` |
| `ldr` at 0x10e970 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10e974 | load | 1 | `sp -0x1f` | `-` |
| `strb` at 0x10e978 | store | 1 | `[sp-28] +0x0` | `([sp-31]:1 & 0xff)` |
| `str` at 0x10e980 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x10e984 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x10e98c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10e98c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10e98c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10e944 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0xc0046707` | `sp-32` | `0x1a04f8` |
| `bl` at 0x10e954 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10e960 | `(unknown callee) 0x10eb40` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e918 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `beq` at 0x10e950 | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10e98c | `r0` | `?` |

## `l1ioGpioInt`

- `l1ioGpioInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x10e99c to 0x10eb40.
  Established. The unnamed `0x20520` call carries `[0x1a04f8]`, `0x80046708`, `sp-36` and `0x1a04f8` (`bl` at 0x10ea10).
  Another unnamed `0x1261c4` call carries `0x0`, `[sp-24]`, `[sp-28]` and `[sp-29]:1` (`bl` at 0x10eab0).
  A word assignment updates `sp -0x28` with `([sp-40] + ret(0x1261c4))` (`str` at 0x10eac0).
  Interrupt acknowledgment and callback roles remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x10e9ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x10e9b0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x10e9b4 | store | 4 | `sp -0x1c` | `r2` |
| `strb` at 0x10e9b8 | store | 1 | `sp -0x1d` | `(r3 & 0xff)` |
| `str` at 0x10e9c0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x10e9c4 | load | 4 | `abs 0x10eb28` | `-` |
| `ldr` at 0x10e9c8 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10e9d4 | load | 4 | `abs 0x10eb2c` | `-` |
| `str` at 0x10e9d8 | store | 4 | `sp -0x30` | `0xfffff3e4` |
| `ldr` at 0x10e9e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x10e9e4 | load | 1 | `r1 +0x3` | `-` |
| `strb` at 0x10e9e8 | store | 1 | `sp -0x24` | `([r1+3]:1 & 0xff)` |
| `str` at 0x10e9f0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x10e9f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x10ea00 | load | 4 | `abs 0x10eb28` | `-` |
| `ldr` at 0x10ea08 | load | 4 | `abs 0x1a04f8` | `-` |
| `ldr` at 0x10ea0c | load | 4 | `abs 0x10eb30` | `-` |
| `ldr` at 0x10ea28 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x10ea40 | load | 4 | `abs 0x10eb34` | `-` |
| `str` at 0x10ea44 | store | 4 | `sp -0x3c` | `0x18d58c` |
| `ldr` at 0x10ea48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10ea4c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x10ea50 | store | 4 | `sp -0x38` | `[[sp-24]+12]` |
| `str` at 0x10ea54 | store | 4 | `sp -0x34` | `ret(0x21ce0)` |
| `ldr` at 0x10ea5c | load | 4 | `abs 0x10eb38` | `-` |
| `ldr` at 0x10ea6c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x10ea70 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x10ea78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x10ea7c | load | 1 | `sp -0x23` | `-` |
| `ldrb` at 0x10ea80 | load | 1 | `[sp-28] +0x3` | `-` |
| `ldrb` at 0x10ea94 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x10ea98 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x10ea9c | store | 4 | `sp -0x3c` | `[sp-35]:1` |
| `ldr` at 0x10eaa4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10eaa8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x10eab8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x10eac0 | store | 4 | `sp -0x28` | `([sp-40] + ret(0x1261c4))` |
| `ldr` at 0x10eac8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x10eacc | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x10ead4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x10eadc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x10eae4 | load | 4 | `abs 0x10eb3c` | `-` |
| `str` at 0x10eae8 | store | 4 | `sp -0x3c` | `0x18d5b8` |
| `ldr` at 0x10eaec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x10eaf0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x10eaf4 | store | 4 | `sp -0x38` | `?` |
| `ldrb` at 0x10eaf8 | load | 1 | `sp -0x23` | `-` |
| `str` at 0x10eafc | store | 4 | `sp -0x34` | `[sp-35]:1` |
| `ldr` at 0x10eb04 | load | 4 | `abs 0x10eb38` | `-` |
| `ldr` at 0x10eb14 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x10eb18 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x10eb1c | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x10eb24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x10eb24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x10eb24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x10ea10 | `(unknown callee) 0x20520` | `[0x1a04f8]` | `0x80046708` | `sp-36` | `0x1a04f8` |
| `bl` at 0x10ea20 | `(unknown callee) 0x20184` | `ret(0x20520)` | `?` | `?` | `ret(0x20520)` |
| `bl` at 0x10ea2c | `(unknown callee) 0x10eb40` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x10ea38 | `(unknown callee) 0x21ce0` | `ret(0x10eb40)` | `?` | `?` | `ret(0x10eb40)` |
| `bl` at 0x10ea68 | `(unknown callee) 0x22070` | `0x3` | `0x18d580` | `0xda` | `0x0` |
| `bl` at 0x10eab0 | `(unknown callee) 0x1261c4` | `0x0` | `[sp-24]` | `[sp-28]` | `[sp-29]:1` |
| `bl` at 0x10eb10 | `(unknown callee) 0x22070` | `0x3` | `0x18d580` | `0xe5` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x10e9d0 | `[0x1a04f8] >= 0x0 (signed, 32-bit)` | `[0x1a04f8] < 0x0 (signed, 32-bit)` |
| `bgt` at 0x10e9fc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x10ea1c | `ret(0x20520) == 0x0` | `ret(0x20520) != 0x0` |
| `beq` at 0x10ea90 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x10eb24 | `r0` | `?` |

## `l1ioRegs`

- `l1ioRegs`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9171c to 0x9173c.
  Established. The word read resolves to `0x1a6450` (`ldr` at 0x9172c);
  its value `[0x1a6450]` is the integer return candidate (`ldm` at 0x91734).
  The register-window identity remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x91728 | load | 4 | `abs 0x91738` | `-` |
| `ldr` at 0x9172c | load | 4 | `abs 0x1a6450` | `-` |
| `ldm` at 0x91734 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x91734 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x91734 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x91734 | `r0` | `[0x1a6450]` |

## `i2cTest`

- `i2cTest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf8cac to 0xf8f68.
  Established. The unnamed `0x1234f4` call carries `0x0`, `sp-28`, entry `r2` and `sp-28` (`bl` at 0xf8cd4).
  A word assignment updates `sp -0x3c` with `([sp-60] + ret(0xf8f68))` (`str` at 0xf8da8).
  Equality `[sp-60] == 0` takes `0xf8f44`, while nonzero falls through (`beq` at 0xf8f34).
  Hardware test coverage and pass criteria remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf8cbc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xf8cc4 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xf8cdc | store | 4 | `sp -0x18` | `ret(0x1234f4)` |
| `str` at 0xf8cf4 | store | 4 | `sp -0x20` | `ret(0x12576c)` |
| `str` at 0xf8d0c | store | 4 | `sp -0x28` | `ret(0x1202e8)` |
| `str` at 0xf8d24 | store | 4 | `sp -0x30` | `ret(0x111db8)` |
| `str` at 0xf8d2c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xf8d30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf8d34 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xf8d44 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xf8d48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8d4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf8d58 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8d78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8d80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf8d88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8da0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf8da8 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xf8f68))` |
| `ldr` at 0xf8dac | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf8db4 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xf8dc0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xf8dc4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8dc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf8dd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8df4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8dfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf8e04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8e1c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf8e24 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xf90a0))` |
| `ldr` at 0xf8e28 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf8e30 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xf8e3c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xf8e40 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8e44 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf8e50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8e70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8e84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf8e8c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8ea4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf8eac | store | 4 | `sp -0x3c` | `([sp-60] + ret(0xf91dc))` |
| `ldr` at 0xf8eb0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf8eb8 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `str` at 0xf8ec4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xf8ec8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8ecc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf8ed8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8ef8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf8f10 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf8f18 | store | 4 | `sp -0x3c` | `([sp-60] + ret(0x114558))` |
| `ldr` at 0xf8f1c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf8f24 | store | 4 | `sp -0x40` | `[sp-64]+1` |
| `ldr` at 0xf8f2c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf8f3c | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xf8f44 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf8f4c | store | 4 | `sp -0x38` | `?` |
| `str` at 0xf8f58 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xf8f5c | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0xf8f64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf8f64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf8f64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf8cd4 | `(unknown callee) 0x1234f4` | `0x0` | `sp-28` | `r2` | `sp-28` |
| `bl` at 0xf8cec | `(unknown callee) 0x12576c` | `0x0` | `sp-36` | `?` | `sp-36` |
| `bl` at 0xf8d04 | `(unknown callee) 0x1202e8` | `0x0` | `sp-44` | `?` | `sp-44` |
| `bl` at 0xf8d1c | `(unknown callee) 0x111db8` | `0x0` | `sp-52` | `?` | `sp-52` |
| `bl` at 0xf8d68 | `(unknown callee) 0x1236e8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xf8d98 | `(unknown callee) 0xf8f68` | `?` | `?` | `?` | `?` |
| `bl` at 0xf8de4 | `(unknown callee) 0x125958` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xf8e14 | `(unknown callee) 0xf90a0` | `?` | `?` | `?` | `?` |
| `bl` at 0xf8e60 | `(unknown callee) 0x120398` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xf8e9c | `(unknown callee) 0xf91dc` | `?` | `?` | `?` | `?` |
| `bl` at 0xf8ee8 | `(unknown callee) 0x111978` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xf8f08 | `(unknown callee) 0x114558` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xf8d3c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xf8d54 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xf8d74 | `ret(0x1236e8) == 0x0` | `ret(0x1236e8) != 0x0` |
| `bge` at 0xf8dd0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xf8df0 | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |
| `bge` at 0xf8e4c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xf8e6c | `ret(0x120398) == 0x0` | `ret(0x120398) != 0x0` |
| `bge` at 0xf8ed4 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `beq` at 0xf8ef4 | `ret(0x111978) == 0x0` | `ret(0x111978) != 0x0` |
| `beq` at 0xf8f34 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf8f64 | `r0` | `?` |
