# ARM Linux 2.6 LLP and jmstr routines in 1.44.0

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
Computed transfers with unresolved edges are identified under their routines. Effects on omitted paths remain unknown.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-jmstr-llp-helpers-facts.txt) retain all tool sections.

## `JTAG_LLP_SC_stub`

- `JTAG_LLP_SC_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139d8c to 0x139dac.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x139d9c).
  The final ldm reads words at entry sp minus 0x14, 0x10, 0xc and 8 (`ldm` at 0x139da8).
  The return record places 0 in r0 (`ldm` at 0x139da8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139d9c | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0x139da8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x139da8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139da8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139da8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139da8 | `r0` | `0x0` |

## `JTAG_LLP_Setup`

- `JTAG_LLP_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x146cd0 to 0x146d1c.
  Established. A word store writes 0x1480d8 to absolute address 0x1a0880 (`str` at 0x146ce4).
  A separate word store writes 0x147c00 to absolute address 0x1a0884 (`str` at 0x146cf0).
  The routine calls unnamed 0x14dcb8 (`bl` at 0x146cf8).
  The return record places 0 in r0 (`ldm` at 0x146d04).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x146cdc | load | 4 | `abs 0x146d08` | `-` |
| `ldr` at 0x146ce0 | load | 4 | `abs 0x146d0c` | `-` |
| `str` at 0x146ce4 | store | 4 | `abs 0x1a0880` | `0x1480d8` |
| `ldr` at 0x146ce8 | load | 4 | `abs 0x146d10` | `-` |
| `ldr` at 0x146cec | load | 4 | `abs 0x146d14` | `-` |
| `str` at 0x146cf0 | store | 4 | `abs 0x1a0884` | `0x147c00` |
| `ldr` at 0x146cf4 | load | 4 | `abs 0x146d18` | `-` |
| `ldm` at 0x146d04 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x146d04 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x146d04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x146cf8 | `(unknown callee) 0x14dcb8` | `0x1482ac` | `r1` | `0x1a0884` | `0x147c00` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x146d04 | `r0` | `0x0` |

## `JTAG_LLP_command`

- `JTAG_LLP_command`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1482ac to 0x148c60.
  Established. Word stores preserve entry r0, r1 and r2 at entry sp minus 0x14, 0x18 and 0x1c respectively (`str` at 0x1482bc, `str` at 0x1482c0, `str` at 0x1482c4).
  The predicated word load into pc has an unknown effective address and an unresolved target when an unknown 32-bit operand is unsigned less than or equal to 5 (`ldrls` at 0x148324).
  On the known fallthrough path, a word store writes 0xfffff7c0 at entry sp minus 0x44 (`str` at 0x1483e8).
  The known-edge return record places 0xfffff7c0 in r0, while effects through the unresolved target remain unknown (`ldm` at 0x148c38).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1482bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1482c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x1482c4 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x1482c8 | load | 4 | `abs 0x148c3c` | `-` |
| `ldr` at 0x1482cc | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1482d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1482e4 | load | 4 | `abs 0x148c40` | `-` |
| `ldr` at 0x1482ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1482f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1482f8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1482fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x14830c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x148310 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x148324 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1483e4 | load | 4 | `abs 0x148c44` | `-` |
| `str` at 0x1483e8 | store | 4 | `sp -0x44` | `0xfffff7c0` |
| `ldr` at 0x148c30 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0x148c38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x148c38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x148c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1482dc | `(unknown callee) 0x13ccc0` | `r0` | `r1` | `r2` | `[0x315034]` |
| `bl` at 0x1482f4 | `(unknown callee) 0x243d4` | `0x1952d0` | `ret(0x13ccc0)` | `[sp-20]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1482d4 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `ldrls` at 0x148324 | `? <= 0x5 (unsigned, 32-bit)` | `? > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x148c38 | `r0` | `0xfffff7c0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x148324).

## `JTAG_LLP_SC_f`

- `JTAG_LLP_SC_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1480d8 to 0x1482ac.
  Established. The result from unnamed 0x13e5b0 is stored as a word at entry sp minus 0x18 (`bl` at 0x1481fc, `str` at 0x148204).
  A later word store writes [sp-20] plus 16 at entry sp minus 0x20 (`str` at 0x14823c).
  The result from a separate call to unnamed 0x1482ac is stored as a word at entry sp minus 0x2c (`bl` at 0x148280, `str` at 0x148288).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1480e8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1480ec | load | 4 | `abs 0x148298` | `-` |
| `ldr` at 0x1480f0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x148100 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x148108 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x148114 | load | 4 | `abs 0x14829c` | `-` |
| `ldr` at 0x148124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14812c | load | 4 | `(unknown)` | `-` |
| `str` at 0x148134 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x14813c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x148144 | load | 4 | `(unknown)` | `-` |
| `str` at 0x148150 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x148154 | load | 4 | `abs 0x148298` | `-` |
| `ldr` at 0x148158 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x148164 | load | 4 | `abs 0x1482a0` | `-` |
| `ldr` at 0x148168 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x14816c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x148174 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x148184 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x14818c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x14819c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1481a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x1481b0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x1481b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1481c8 | load | 4 | `abs 0x1482a4` | `-` |
| `str` at 0x1481cc | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0x1481d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1481e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1481ec | load | 4 | `sp -0x28` | `-` |
| `str` at 0x148204 | store | 4 | `sp -0x18` | `ret(0x13e5b0)` |
| `ldr` at 0x148208 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x148214 | load | 4 | `abs 0x1482a4` | `-` |
| `str` at 0x148218 | store | 4 | `sp -0x2c` | `0xfffff7c6` |
| `ldr` at 0x148220 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x148224 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x148230 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x148234 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x14823c | store | 4 | `sp -0x20` | `[sp-20]+16` |
| `ldr` at 0x148240 | load | 4 | `abs 0x148298` | `-` |
| `ldr` at 0x148244 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x148250 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x148254 | load | 4 | `[sp-20] +0x10` | `-` |
| `ldr` at 0x148264 | load | 4 | `abs 0x1482a8` | `-` |
| `ldr` at 0x148268 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x148274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x148278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x14827c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x148288 | store | 4 | `sp -0x2c` | `ret(0x1482ac)` |
| `ldr` at 0x14828c | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x148294 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x148294 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x148294 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x14811c | `(unknown callee) 0x243d4` | `0x195278` | `?` | `0x4` | `?` |
| `bl` at 0x148170 | `(unknown callee) 0x243d4` | `0x195298` | `[sp-36]` | `[sp-40]` | `[0x315034]` |
| `bl` at 0x1481b8 | `(unknown callee) 0x13ccc0` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x1481fc | `(unknown callee) 0x13e5b0` | `?` | `?` | `?` | `?` |
| `bl` at 0x148270 | `(unknown callee) 0x243d4` | `0x1952b4` | `[sp-24]` | `?` | `?` |
| `bl` at 0x148280 | `(unknown callee) 0x1482ac` | `[sp-24]` | `[sp-28]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1480f8 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bls` at 0x148160 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x14817c | `[sp-36] == 0x0` | `[sp-36] != 0x0` |
| `bne` at 0x1481c4 | `ret(0x13ccc0) != 0x0` | `ret(0x13ccc0) == 0x0` |
| `bne` at 0x148210 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x14824c | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x148294 | `r0` | `?` |

## `JTAG_read_dmcout`

- `JTAG_read_dmcout`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x146d1c to 0x147024.
  Established. The result from unnamed 0x13e5b0 is stored as a word at entry sp minus 0x18 (`bl` at 0x146dec, `str` at 0x146df4).
  A separate call to unnamed 0x147024 has its result stored as a word at entry sp minus 0x1c (`bl` at 0x146e40, `str` at 0x146e48).
  A later word store writes [sp-64] to an unknown effective target (`str` at 0x146fd0).
  Another writes 0x160 to the address formed by [sp-20] plus 0xc (`str` at 0x147000).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x146d2c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x146d34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146d3c | load | 4 | `r0 +0x4` | `-` |
| `str` at 0x146d44 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x146d4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146d54 | load | 4 | `r0 +0x4` | `-` |
| `str` at 0x146d60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x146d64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x146d74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146d7c | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x146d8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x146d98 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x146da0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x146da4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x146db8 | load | 4 | `abs 0x147020` | `-` |
| `str` at 0x146dbc | store | 4 | `sp -0x44` | `0xfffff7c6` |
| `ldr` at 0x146dc8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146dd0 | load | 4 | `r0 +0x4` | `-` |
| `ldr` at 0x146ddc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x146df4 | store | 4 | `sp -0x18` | `ret(0x13e5b0)` |
| `ldr` at 0x146df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x146e04 | load | 4 | `abs 0x147020` | `-` |
| `str` at 0x146e08 | store | 4 | `sp -0x44` | `0xfffff7c6` |
| `ldr` at 0x146e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146e14 | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x146e20 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x146e30 | store | 4 | `sp -0x48` | `sp-52` |
| `ldr` at 0x146e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x146e38 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x146e48 | store | 4 | `sp -0x1c` | `ret(0x147024)` |
| `ldr` at 0x146e4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x146e58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x146e5c | store | 4 | `sp -0x44` | `ret(0x147024)` |
| `str` at 0x146e68 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x146e6c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x146e78 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146e84 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146e88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x146e94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146ea0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146ea4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x146ea8 | load | 1 | `[sp-52] +0x0` | `-` |
| `ldr` at 0x146eb0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146ebc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146ec4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0x146ecc | load | 1 | `[sp-52] +0x1` | `-` |
| `ldr` at 0x146ed0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146edc | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146ee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146eec | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146ef0 | store | 4 | `[sp-20] +0x10` | `[sp-64]` |
| `str` at 0x146ef8 | store | 4 | `sp -0x38` | `0x1` |
| `str` at 0x146f00 | store | 4 | `sp -0x3c` | `0x2` |
| `ldr` at 0x146f04 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x146f10 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x146f14 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x146f1c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x146f24 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146f30 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146f38 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x146f3c | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x146f48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x146f50 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146f5c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146f64 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x146f68 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x146f74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x146f7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146f88 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146f90 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x146f94 | load | 4 | `sp -0x3c` | `-` |
| `ldrb` at 0x146fa0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x146fa4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146fb0 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x146fb4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x146fc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x146fcc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x146fd0 | store | 4 | `(unknown)` | `[sp-64]` |
| `ldr` at 0x146fd4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x146fdc | store | 4 | `sp -0x38` | `[sp-56]+1` |
| `ldr` at 0x146fe0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x146fe8 | store | 4 | `sp -0x3c` | `[sp-60]+4` |
| `ldr` at 0x146ff4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x147000 | store | 4 | `[sp-20] +0xc` | `0x160` |
| `ldr` at 0x147004 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x147010 | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x147014 | load | 4 | `sp -0x44` | `-` |
| `ldm` at 0x14701c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x14701c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x14701c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x146da8 | `(unknown callee) 0x13ccc0` | `[sp-24]` | `r1` | `?` | `?` |
| `bl` at 0x146dec | `(unknown callee) 0x13e5b0` | `?` | `?` | `?` | `?` |
| `bl` at 0x146e40 | `(unknown callee) 0x147024` | `[sp-24]` | `[sp-40]` | `sp-44` | `sp-48` |
| `bl` at 0x147008 | `(unknown callee) 0x20514` | `[sp-52]` | `?` | `[sp-20]+12` | `0x160` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x146d6c | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bne` at 0x146db4 | `ret(0x13ccc0) != 0x0` | `ret(0x13ccc0) == 0x0` |
| `bne` at 0x146e00 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x146e54 | `ret(0x147024) >= 0x0 (signed, 32-bit)` | `ret(0x147024) < 0x0 (signed, 32-bit)` |
| `bhi` at 0x146f0c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x14701c | `r0` | `?` |

## `jmstr_set_functions`

- `jmstr_set_functions`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12cc4c to 0x12cc8c.
  Established. A word store writes entry r0 to absolute address 0x315020 (`str` at 0x12cc6c).
  Another writes [sp-24] as a word to absolute address 0x315024 (`str` at 0x12cc78).
  The return record preserves entry r0 (`ldm` at 0x12cc80).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12cc5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12cc60 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x12cc64 | load | 4 | `abs 0x12cc84` | `-` |
| `ldr` at 0x12cc68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12cc6c | store | 4 | `abs 0x315020` | `r0` |
| `ldr` at 0x12cc70 | load | 4 | `abs 0x12cc88` | `-` |
| `ldr` at 0x12cc74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12cc78 | store | 4 | `abs 0x315024` | `[sp-24]` |
| `ldm` at 0x12cc80 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12cc80 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12cc80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12cc80 | `r0` | `r0` |

## `jmstr_reset`

- `jmstr_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12cc8c to 0x12cdbc.
  Established. A word store writes 1 at entry sp minus 0x18 (`str` at 0x12ccb4).
  Word stores include 0x20 and 0x28 at entry sp minus 0x24 (`str` at 0x12ccbc, `str` at 0x12ccd4).
  The call at 0x12cd10 has an unknown target (`mov` at 0x12cd10).
  The return record places [sp-44] in r0 (`ldm` at 0x12cdb0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12cc9c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x12cca0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x12cca4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x12ccac | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x12ccb4 | store | 4 | `sp -0x18` | `0x1` |
| `str` at 0x12ccbc | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0x12ccc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12cccc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ccd4 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x12ccd8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12cce4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ccec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12ccf0 | load | 4 | `abs 0x12cdb4` | `-` |
| `ldr` at 0x12ccfc | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12cd00 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cd20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cd24 | load | 4 | `abs 0x12cdb8` | `-` |
| `str` at 0x12cd30 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12cd38 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12cd48 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x12cd4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12cd54 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12cd58 | load | 4 | `abs 0x12cdb4` | `-` |
| `ldr` at 0x12cd64 | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12cd68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cd88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cd8c | load | 4 | `abs 0x12cdb8` | `-` |
| `str` at 0x12cd98 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x12cda4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x12cda8 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x12cdb0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12cdb0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12cdb0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12cdb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x12cd10 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0x12cd28 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x1907a4` | `?` | `ret(unknown)` |
| `mov` at 0x12cd78 | `(unknown callee) (unknown target)` | `[sp-36]` | `0x1` | `sp-40` | `sp-40` |
| `bl` at 0x12cd90 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x1907a4` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12ccc8 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x12cce0 | `0x1 != 0x1` | `0x1 == 0x1` |
| `bne` at 0x12cd1c | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `bne` at 0x12cd40 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x12cd84 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12cdb0 | `r0` | `[sp-44]` |

## `jmstr_reset_and_idle`

- `jmstr_reset_and_idle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12cdbc to 0x12cee4.
  Established. Word stores include 0x20 and 0x28 at entry sp minus 0x24 (`str` at 0x12cde4, `str` at 0x12cdfc).
  Later word stores include 0x10 and 0x18 at that same destination (`str` at 0x12ce64, `str` at 0x12ce7c).
  A word store writes 1 at entry sp minus 0x2c (`str` at 0x12cecc).
  The return record retains an unknown value in r0 (`ldm` at 0x12ced8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12cdcc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x12cdd0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x12cdd4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x12cddc | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x12cde4 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0x12cde8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12cdf4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12cdfc | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0x12ce00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12ce0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ce14 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12ce18 | load | 4 | `abs 0x12cedc` | `-` |
| `ldr` at 0x12ce24 | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12ce28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ce48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12ce4c | load | 4 | `abs 0x12cee0` | `-` |
| `str` at 0x12ce58 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x12ce64 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0x12ce68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12ce74 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ce7c | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0x12ce80 | load | 4 | `abs 0x12cedc` | `-` |
| `ldr` at 0x12ce8c | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12ce90 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12ceb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12ceb4 | load | 4 | `abs 0x12cee0` | `-` |
| `str` at 0x12cec0 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x12cecc | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x12ced0 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x12ced8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12ced8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12ced8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12ced8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x12ce38 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0x12ce50 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x1907a4` | `?` | `ret(unknown)` |
| `mov` at 0x12cea0 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0x12ceb8 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x1907a4` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12cdf0 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x12ce08 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0x12ce44 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0x12ce70 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0x12ceac | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12ced8 | `r0` | `?` |

## `jmstr_move_to_idle`

- `jmstr_move_to_idle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12cee4 to 0x12d168.
  Established. The result from unnamed 0x201fc is stored as a word at entry sp minus 0x30 (`bl` at 0x12cf44, `str` at 0x12cf48).
  The predicated word load into pc has an unknown effective address and an unresolved target under an ls flags condition whose operands remain unknown (`ldrls` at 0x12cfc4).
  The known fallthrough path writes 0xff000000 and 5 as words at entry sp minus 0x2c and 0x38 respectively (`str` at 0x12d058, `str` at 0x12d060).
  A subsequent known-path call has an unknown target, while effects through the unresolved transfer remain unknown (`mov` at 0x12d09c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12cef4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x12cef8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x12cefc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x12cf00 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x12cf08 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x12cf10 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0x12cf14 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cf20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cf34 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12cf48 | store | 4 | `sp -0x30` | `ret(0x201fc)` |
| `ldr` at 0x12cf4c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12cf58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12cf5c | load | 4 | `abs 0x12d158` | `-` |
| `str` at 0x12cf68 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x12cf70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cf84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12cf90 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12cfa0 | store | 4 | `sp -0x28` | `0x10` |
| `ldr` at 0x12cfa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12cfb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x12cfb8 | store | 4 | `sp -0x28` | `0x18` |
| `ldr` at 0x12cfbc | load | 4 | `sp -0x18` | `-` |
| `ldrls` at 0x12cfc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12d058 | store | 4 | `sp -0x2c` | `0xff000000` |
| `str` at 0x12d060 | store | 4 | `sp -0x38` | `0x5` |
| `ldr` at 0x12d064 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12d070 | load | 4 | `abs 0x12d15c` | `-` |
| `str` at 0x12d080 | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x12d084 | load | 4 | `abs 0x315020` | `-` |
| `ldr` at 0x12d088 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x12d08c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12d0ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12d0b0 | load | 4 | `abs 0x12d160` | `-` |
| `ldr` at 0x12d0b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12d0c4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12d0d0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x12d0d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12d0e4 | load | 4 | `abs 0x12d164` | `-` |
| `ldr` at 0x12d0e8 | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12d0ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x12d0f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12d0f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12d0f8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12d110 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12d114 | load | 4 | `abs 0x12d160` | `-` |
| `ldr` at 0x12d11c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12d128 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x12d130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12d13c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12d148 | store | 4 | `sp -0x40` | `0x1` |
| `ldr` at 0x12d14c | load | 4 | `sp -0x40` | `-` |
| `ldm` at 0x12d154 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12d154 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12d154 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12d154 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12cf44 | `(unknown callee) 0x201fc` | `?` | `r1` | `?` | `?` |
| `bl` at 0x12cf60 | `(unknown callee) 0x205bc` | `[sp-36]` | `0x1907d8` | `?` | `ret(0x201fc)` |
| `bl` at 0x12cf98 | `(unknown callee) 0x20550` | `ret(0x201fc)` | `0x0` | `?` | `?` |
| `mov` at 0x12d09c | `(unknown callee) (unknown target)` | `?` | `0x5` | `sp-60` | `sp-44` |
| `bl` at 0x12d0b4 | `(unknown callee) 0x205bc` | `[sp-36]` | `0x1907a4` | `?` | `ret(unknown)` |
| `bl` at 0x12d0c8 | `(unknown callee) 0x20514` | `[sp-48]` | `?` | `?` | `[sp-48]` |
| `mov` at 0x12d100 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` |
| `bl` at 0x12d118 | `(unknown callee) 0x205bc` | `[sp-36]` | `0x1907a4` | `?` | `ret(unknown)` |
| `bl` at 0x12d120 | `(unknown callee) 0x20514` | `[sp-48]` | `?` | `?` | `?` |
| `bl` at 0x12d140 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12cf1c | `r2 == 0x0` | `r2 != 0x0` |
| `bne` at 0x12cf54 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x12cfac | `Z == 1` | `not (Z == 1)` |
| `ldrls` at 0x12cfc4 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `beq` at 0x12d06c | `0x5 == 0x0` | `0x5 != 0x0` |
| `bne` at 0x12d0a8 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0x12d0c0 | `[sp-48] == 0x0` | `[sp-48] != 0x0` |
| `beq` at 0x12d0e0 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x12d10c | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0x12d138 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12d154 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x12cfc4).

## `jmstr_run_tck`

- `jmstr_run_tck`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12d168 to 0x12d210.
  Established. Word stores include 0x10 and 0x18 at entry sp minus 0x20 (`str` at 0x12d188, `str` at 0x12d1a0).
  The call at 0x12d1cc has an unknown target (`mov` at 0x12d1cc).
  Later paths write 0 and 1 as words at entry sp minus 0x24 (`str` at 0x12d1ec, `str` at 0x12d1f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12d178 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12d17c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12d180 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x12d188 | store | 4 | `sp -0x20` | `0x10` |
| `ldr` at 0x12d18c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12d198 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12d1a0 | store | 4 | `sp -0x20` | `0x18` |
| `ldr` at 0x12d1a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12d1b0 | load | 4 | `abs 0x12d208` | `-` |
| `ldr` at 0x12d1b4 | load | 4 | `abs 0x315024` | `-` |
| `ldr` at 0x12d1b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12d1bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12d1dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12d1e0 | load | 4 | `abs 0x12d20c` | `-` |
| `str` at 0x12d1ec | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x12d1f8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x12d1fc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x12d204 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12d204 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12d204 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x12d1cc | `(unknown callee) (unknown target)` | `?` | `r1` | `0x0` | `0x0` |
| `bl` at 0x12d1e4 | `(unknown callee) 0x205bc` | `[sp-28]` | `0x1907a4` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12d194 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0x12d1ac | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0x12d1d8 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12d204 | `r0` | `?` |

## `jmstr_custom_seq`

- `jmstr_custom_seq`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12d210 to 0x12d2b8.
  Established. A word loaded from entry sp is stored at entry sp minus 0x2c (`ldr` at 0x12d254, `str` at 0x12d258).
  The call at 0x12d274 has an unknown target (`mov` at 0x12d274).
  Separate later paths write 0 and 1 as words at entry sp minus 0x28 (`str` at 0x12d294, `str` at 0x12d2a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12d220 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12d224 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12d228 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x12d22c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x12d234 | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0x12d238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12d244 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12d24c | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0x12d250 | load | 4 | `abs 0x12d2b0` | `-` |
| `ldr` at 0x12d254 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x12d258 | store | 4 | `sp -0x2c` | `[sp]` |
| `ldr` at 0x12d25c | load | 4 | `abs 0x315020` | `-` |
| `ldr` at 0x12d260 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12d264 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x12d268 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12d26c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12d284 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x12d288 | load | 4 | `abs 0x12d2b4` | `-` |
| `str` at 0x12d294 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x12d2a0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x12d2a4 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x12d2ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12d2ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12d2ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x12d274 | `(unknown callee) (unknown target)` | `?` | `r1` | `r2` | `r3` |
| `bl` at 0x12d28c | `(unknown callee) 0x205bc` | `[sp+4]` | `0x1907a4` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12d240 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x12d280 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12d2ac | `r0` | `?` |

## `jmstr_scan`

- `jmstr_scan`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12d2b8 to 0x12d84c.
  Established. One known path writes 1 as a word at entry sp minus 0x48 (`str` at 0x12d2f0).
  The predicated word load into pc has an unknown effective address and an unresolved target when entry r1 is unsigned less than or equal to 0xd (`ldrls` at 0x12d320).
  A known fallthrough path calls unnamed 0x20634 and later writes 0 as a word at entry sp minus 0x48 (`bl` at 0x12d480, `str` at 0x12d488).
  Effects through the unresolved target remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12d2c8 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x12d2cc | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x12d2d0 | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x12d2d4 | store | 4 | `sp -0x28` | `r3` |
| `str` at 0x12d2dc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x12d2e0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x12d2f0 | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x12d2fc | store | 4 | `sp -0x34` | `0x10` |
| `ldr` at 0x12d300 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x12d30c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x12d314 | store | 4 | `sp -0x34` | `0x18` |
| `ldr` at 0x12d318 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0x12d320 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x12d478 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x12d47c | load | 4 | `abs 0x12d83c` | `-` |
| `str` at 0x12d488 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12d830 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0x12d838 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x12d838 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x12d838 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12d838 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12d838 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12d480 | `(unknown callee) 0x20634` | `[sp+12]` | `0x1907fc` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12d2e8 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0x12d308 | `r3 == 0x0` | `r3 != 0x0` |
| `ldrls` at 0x12d320 | `r1 <= 0xd (unsigned, 32-bit)` | `r1 > 0xd (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12d838 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x12d320).

## `jmstr_sel_sic`

- `jmstr_sel_sic`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12d84c to 0x12f6f0.
  Established. The result from one call to unnamed 0x201fc is stored as a word at entry sp minus 0x40 (`bl` at 0x12d870, `str` at 0x12d874).
  A separate call to the same unnamed address has its result stored as a word at entry sp minus 0x44 (`bl` at 0x12d8a0, `str` at 0x12d8a4).
  A later word store writes 0xa800ffea to the address held in [sp-64] (`str` at 0x12e3c0).
  Another word store writes [[sp-68]] to the address held in [sp+8] (`str` at 0x12f238).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12d85c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12d860 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12d864 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x12d868 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x12d874 | store | 4 | `sp -0x40` | `ret(0x201fc)` |
| `ldr` at 0x12d878 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12d884 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12d888 | load | 4 | `abs 0x12e824` | `-` |
| `str` at 0x12d894 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x12d8a4 | store | 4 | `sp -0x44` | `ret(0x201fc)` |
| `ldr` at 0x12d8a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12d8b4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12d8b8 | load | 4 | `abs 0x12e824` | `-` |
| `ldr` at 0x12d8c0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12d8cc | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12d8d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12d8e4 | load | 4 | `abs 0x12e828` | `-` |
| `str` at 0x12d8e8 | store | 4 | `sp -0x28` | `0xff800000` |
| `str` at 0x12d8f0 | store | 4 | `sp -0x2c` | `0x80000000` |
| `ldr` at 0x12d8f8 | load | 4 | `abs 0x12e82c` | `-` |
| `str` at 0x12d8fc | store | 4 | `sp -0x28` | `0xfff00000` |
| `ldr` at 0x12d900 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12d904 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `ldr` at 0x12d908 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d90c | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12d918 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d920 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12d92c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x12d930 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d934 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12d940 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d948 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12d954 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x12d958 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d95c | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12d968 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d970 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12d97c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x12d980 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d990 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12d9a4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x12d9a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d9b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12d9cc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x12d9d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12d9dc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x12d9e4 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x12d9ec | store | 4 | `sp -0x34` | `0x1` |
| `ldr` at 0x12d9f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12da04 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12da08 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12da1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12da24 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12da30 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12da3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12da40 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12da54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12da5c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12da68 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12da70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12da84 | store | 4 | `sp -0x24` | `0x9` |
| `ldr` at 0x12da88 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12da90 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `str` at 0x12da9c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0x12daa0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12daa8 | store | 4 | `[sp-64] +0x0` | `0x4` |
| `ldr` at 0x12daac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dab0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dab4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dab8 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12dac4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dac8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dad0 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12dadc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dae0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dae4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dae8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12daec | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12daf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dafc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12db04 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12db10 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12db14 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db1c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12db20 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12db2c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db30 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12db38 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12db44 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12db48 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db50 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12db60 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db64 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12db78 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12db7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db84 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12db94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12db98 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12dbac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dbb0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dbb8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12dbbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12dbc0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12dbc4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12dbc8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12dbcc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dbd0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12dbd4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12dbd8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12dbe8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12dbfc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc04 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dc10 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12dc18 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12dc1c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12dc20 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12dc2c | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0x12dc34 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0x12dc38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x12dc40 | store | 4 | `[sp-64] +0x0` | `[sp-24]` |
| `ldr` at 0x12dc44 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dc4c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12dc50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc54 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc58 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dc5c | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12dc68 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc6c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dc74 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12dc80 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dc84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc88 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dc8c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dc90 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12dc9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dca0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dca8 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12dcb4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dcb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dcbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dcc0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dcc4 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12dcd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dcd4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dcdc | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12dce8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dcec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dcf0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dcf4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dd04 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dd08 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12dd1c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dd20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dd24 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dd28 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dd38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dd3c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12dd50 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dd54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12dd58 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12dd5c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12dd60 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12dd64 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dd68 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12dd6c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12dd70 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12dd80 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12dd94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dd9c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12dda8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12ddb0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ddb4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ddb8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12ddc0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12ddc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12ddd4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12dde8 | store | 4 | `sp -0x4c` | `0x9` |
| `str` at 0x12ddf4 | store | 4 | `sp -0x4c` | `0xc` |
| `ldr` at 0x12ddf8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x12ddfc | store | 4 | `sp -0x24` | `[sp-76]` |
| `ldr` at 0x12de00 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12de08 | store | 4 | `[sp-64] +0x0` | `0xe` |
| `ldr` at 0x12de0c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12de14 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12de18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de20 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12de24 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12de30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de34 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12de3c | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12de48 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12de4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12de58 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12de64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de68 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12de70 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12de7c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12de80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de88 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12de8c | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12de98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12de9c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dea4 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12deb0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12deb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12deb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12debc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12decc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ded0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12dee4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dee8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12deec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12def0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12df00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12df04 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12df18 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12df1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12df20 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12df24 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12df28 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12df2c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12df30 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12df34 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12df38 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12df48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12df5c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12df64 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12df70 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12df78 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12df7c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12df80 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12df8c | store | 4 | `[sp-68] +0x0` | `?` |
| `str` at 0x12df94 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0x12df98 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12df9c | load | 4 | `sp +0x0` | `-` |
| `str` at 0x12dfa0 | store | 4 | `[sp-64] +0x0` | `[sp]` |
| `ldr` at 0x12dfa4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dfa8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dfac | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dfb0 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12dfbc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dfc0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dfc8 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12dfd4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12dfd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dfdc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dfe0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dfe4 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12dff0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12dff4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12dffc | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12e008 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e00c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e010 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e014 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e018 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12e024 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e028 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e030 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12e03c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e040 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e044 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e048 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e058 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e05c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e070 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e074 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e078 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e07c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e08c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e090 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e0a4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e0ac | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e0b8 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0x12e0bc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e0c4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e0cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e0d8 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0x12e0dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e0e0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e0e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e0e8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e0ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e0f0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e0f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e0f8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e108 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e11c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e124 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e130 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e138 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12e144 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e148 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e14c | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12e154 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e15c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e168 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0x12e174 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e180 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e188 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0x12e194 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0x12e198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12e1ac | store | 4 | `sp -0x50` | `0x9` |
| `str` at 0x12e1b8 | store | 4 | `sp -0x50` | `0xc` |
| `ldr` at 0x12e1bc | load | 4 | `sp -0x50` | `-` |
| `str` at 0x12e1c0 | store | 4 | `sp -0x24` | `[sp-80]` |
| `ldr` at 0x12e1c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e1c8 | load | 4 | `abs 0x12e830` | `-` |
| `str` at 0x12e1cc | store | 4 | `[sp-64] +0x0` | `0xfff` |
| `ldr` at 0x12e1d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e1d8 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e1dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e1e0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e1e4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e1e8 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12e1f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e1f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e200 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12e20c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e210 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e214 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e218 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e21c | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12e228 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e22c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e234 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12e240 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e244 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e248 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e24c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e250 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12e25c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e260 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e268 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12e274 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e278 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e27c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e280 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e290 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e294 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e2a8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e2ac | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e2b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e2b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e2c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e2c8 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e2dc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e2e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e2e4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e2e8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e2ec | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e2f0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e2f4 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e2f8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e2fc | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e30c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e320 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e328 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e334 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e33c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e340 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e344 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12e350 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e354 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e358 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e35c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e360 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x12e368 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e36c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e370 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e374 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12e380 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e384 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12e388 | load | 4 | `abs 0x12e840` | `-` |
| `ldr` at 0x12e38c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e394 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e39c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e3a8 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x12e3b4 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0x12e3b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e3bc | load | 4 | `abs 0x12e834` | `-` |
| `str` at 0x12e3c0 | store | 4 | `[sp-64] +0x0` | `0xa800ffea` |
| `ldr` at 0x12e3c4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e3cc | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e3d0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e3d4 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e3d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e3dc | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e3e0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e3e4 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e3e8 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e3ec | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e3fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e410 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e418 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e424 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e42c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e430 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e434 | load | 4 | `abs 0x12e838` | `-` |
| `ldr` at 0x12e440 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e444 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12e448 | load | 4 | `abs 0x12e83c` | `-` |
| `ldr` at 0x12e44c | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e454 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e45c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e468 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e470 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12e484 | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0x12e488 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e490 | store | 4 | `[sp-64] +0x0` | `0x9` |
| `ldr` at 0x12e494 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e49c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e4a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4a4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4a8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e4ac | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12e4b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e4c4 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12e4d0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e4d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4dc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e4e0 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12e4ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e4f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e4f8 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12e504 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e508 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e50c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e510 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e514 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12e520 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e524 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e52c | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12e538 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e53c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e540 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e544 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e554 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e558 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e56c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e570 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e574 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e578 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e588 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e58c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e5a0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e5a4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e5a8 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e5ac | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e5b0 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e5b4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e5b8 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e5bc | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e5c0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e5d0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e5e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e5ec | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e5f8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e600 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e604 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e608 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12e614 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e618 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e61c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e620 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e624 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x12e62c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e630 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e634 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e638 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12e644 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e648 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12e64c | load | 4 | `abs 0x12e840` | `-` |
| `ldr` at 0x12e650 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12e658 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e660 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e66c | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x12e678 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0x12e67c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e680 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x12e684 | store | 4 | `[sp-64] +0x0` | `[sp+4]` |
| `ldr` at 0x12e688 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e68c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e690 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e694 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12e6a0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6a4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e6ac | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12e6b8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e6bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6c0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6c4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e6c8 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12e6d4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6d8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e6e0 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12e6ec | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e6f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6f4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e6f8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e6fc | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12e708 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e70c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e714 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12e720 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e724 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e728 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e72c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e73c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e740 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e754 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e758 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e75c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e760 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e770 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e774 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e788 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e78c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e794 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e798 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e79c | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e7a0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e7a4 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e7a8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e7ac | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e7b0 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e7b4 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e7c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e7d8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e7e0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e7ec | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e7f4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e7f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e7fc | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12e804 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e808 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12e81c | store | 4 | `sp -0x54` | `0x9` |
| `str` at 0x12e85c | store | 4 | `sp -0x54` | `0xc` |
| `ldr` at 0x12e860 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x12e864 | store | 4 | `sp -0x24` | `[sp-84]` |
| `ldr` at 0x12e868 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e870 | store | 4 | `[sp-64] +0x0` | `0x8` |
| `ldr` at 0x12e874 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e87c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12e880 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e884 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e888 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e88c | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12e898 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e89c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e8a4 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12e8b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e8b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e8b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e8bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e8c0 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12e8cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e8d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e8d8 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12e8e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e8e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e8ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e8f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e8f4 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12e900 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e904 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e90c | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12e918 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e91c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e920 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e924 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e934 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e938 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e94c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e950 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e954 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e958 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12e968 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e96c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12e980 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12e984 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12e988 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12e98c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12e990 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12e994 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e998 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12e99c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12e9a0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12e9b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12e9c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12e9cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12e9d8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12e9e0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e9e4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e9e8 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12e9f4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12e9f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12e9fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ea00 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12ea04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x12ea0c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12ea10 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ea14 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12ea18 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12ea24 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ea28 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12ea2c | load | 4 | `abs 0x12e840` | `-` |
| `ldr` at 0x12ea30 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12ea38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ea40 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ea4c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12ea54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12ea68 | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0x12ea6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ea70 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x12ea78 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ea7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ea80 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ea84 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ea88 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12ea94 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ea98 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eaa0 | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12eaac | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eab0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eab4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eab8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eabc | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12eac8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eacc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ead4 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12eae0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eae4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eae8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eaec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eaf0 | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12eafc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb00 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eb08 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12eb14 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eb18 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb1c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb20 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eb30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb34 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12eb48 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eb4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eb64 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eb68 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12eb7c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eb84 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12eb90 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0x12eb94 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12eb9c | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12eba4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ebb0 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `str` at 0x12ebbc | store | 4 | `sp -0x24` | `0x8` |
| `ldr` at 0x12ebc0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12ebd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ebd4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x12ebdc | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ebe4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ebe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x12ebf4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x12ec00 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ec08 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12ec14 | store | 4 | `[sp-64] +0x4` | `0x0` |
| `ldr` at 0x12ec18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ec20 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12ec28 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ec34 | store | 4 | `[sp-68] +0x4` | `0x0` |
| `ldr` at 0x12ec38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec40 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ec44 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12ec50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ec5c | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12ec68 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ec6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec74 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ec78 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12ec84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ec88 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ec90 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12ec9c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eca0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eca4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eca8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ecac | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12ecb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ecbc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ecc4 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12ecd0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ecd4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ecd8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ecdc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ecec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ecf0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12ed04 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ed08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ed0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ed10 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ed20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ed24 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12ed38 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ed3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ed40 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12ed44 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12ed48 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12ed4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ed50 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12ed54 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12ed58 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12ed68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12ed7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ed84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ed90 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12ed98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x12eda4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12eda8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12edac | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12edb4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12edc0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12edcc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12edd4 | load | 4 | `[sp-68] +0x4` | `-` |
| `str` at 0x12ede0 | store | 4 | `[sp-68] +0x4` | `?` |
| `ldr` at 0x12ede4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12edf4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x12ee08 | store | 4 | `sp -0x58` | `0x9` |
| `str` at 0x12ee14 | store | 4 | `sp -0x58` | `0xc` |
| `ldr` at 0x12ee18 | load | 4 | `sp -0x58` | `-` |
| `str` at 0x12ee1c | store | 4 | `sp -0x24` | `[sp-88]` |
| `ldr` at 0x12ee20 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12ee28 | store | 4 | `[sp-64] +0x0` | `0x1` |
| `ldr` at 0x12ee2c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ee34 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12ee38 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee3c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee40 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ee44 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12ee50 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee54 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ee5c | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12ee68 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ee6c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee70 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee74 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ee78 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12ee84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ee88 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ee90 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12ee9c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eea0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eea4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eea8 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eeac | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12eeb8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eebc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eec4 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12eed0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12eed4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eed8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eedc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12eeec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12eef0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12ef04 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ef08 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ef0c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ef10 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12ef20 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ef24 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12ef38 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12ef3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ef40 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12ef44 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12ef48 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12ef4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ef50 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12ef54 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12ef58 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12ef68 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12ef7c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12ef84 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ef90 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12ef98 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12ef9c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12efa0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12efa4 | store | 4 | `[sp-68] +0x0` | `[[sp-68]]` |
| `str` at 0x12efac | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0x12efb0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12efb8 | store | 4 | `[sp-64] +0x0` | `0x0` |
| `ldr` at 0x12efbc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12efc4 | store | 4 | `[sp-68] +0x0` | `0x0` |
| `ldr` at 0x12efc8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12efcc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12efd0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12efd4 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12efe0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12efe4 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12efec | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12eff8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12effc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f000 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f004 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f008 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12f014 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f018 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f020 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12f02c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12f030 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f034 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f038 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f03c | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12f048 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f04c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f054 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12f060 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12f064 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f068 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f06c | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f07c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f080 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12f094 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12f098 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f09c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f0a0 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x12f0b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f0b4 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x12f0c8 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x12f0cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12f0d0 | store | 4 | `sp -0x68` | `[sp-36]` |
| `ldr` at 0x12f0d4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f0d8 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12f0dc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f0e0 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12f0e4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12f0e8 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12f0f8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f10c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f114 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f120 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f128 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f12c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f130 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f134 | load | 4 | `abs 0x12e844` | `-` |
| `ldr` at 0x12f140 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f144 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f14c | load | 4 | `abs 0x12e844` | `-` |
| `str` at 0x12f158 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12f15c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f160 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f164 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f168 | load | 4 | `abs 0x12e848` | `-` |
| `ldr` at 0x12f174 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f178 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f180 | load | 4 | `abs 0x12e848` | `-` |
| `str` at 0x12f18c | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12f190 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f194 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f198 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f19c | load | 4 | `abs 0x12e84c` | `-` |
| `ldr` at 0x12f1a8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f1ac | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f1b4 | load | 4 | `abs 0x12e84c` | `-` |
| `str` at 0x12f1c0 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12f1c4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f1c8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f1cc | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f1dc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f1e0 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12f1f4 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12f1f8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f1fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f200 | load | 4 | `[sp-68] +0x0` | `-` |
| `ldr` at 0x12f210 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f214 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12f228 | store | 4 | `[sp-68] +0x0` | `?` |
| `ldr` at 0x12f22c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x12f230 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f234 | load | 4 | `[sp-68] +0x0` | `-` |
| `str` at 0x12f238 | store | 4 | `[sp+8] +0x0` | `[[sp-68]]` |
| `str` at 0x12f240 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x12f244 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f258 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f25c | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12f270 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f278 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f284 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f28c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f29c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f2ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f2c4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f2c8 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12f2dc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f2e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f2f0 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f2f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f308 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f320 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f328 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12f33c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f344 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f350 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f368 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f370 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f37c | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0x12f388 | store | 4 | `sp -0x38` | `0x100` |
| `ldr` at 0x12f38c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x12f39c | store | 4 | `sp -0x3c` | `0x300` |
| `ldr` at 0x12f3a0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12f3b4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x12f3c0 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x12f3c8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x12f3cc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f3d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f3dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f3e4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f3f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f3f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f3fc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f404 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f40c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f418 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f420 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f42c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12f430 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f438 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f440 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f450 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f45c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f464 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f470 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f474 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f47c | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f484 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12f488 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x12f48c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f490 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12f494 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f498 | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12f49c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12f4a0 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12f4b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f4c4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f4cc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f4d8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f4e0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12f4e4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f4e8 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f4ec | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x12f500 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12f504 | load | 4 | `abs 0x12e850` | `-` |
| `ldr` at 0x12f50c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f514 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f520 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0x12f52c | store | 4 | `sp -0x38` | `0x40` |
| `ldr` at 0x12f530 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x12f540 | store | 4 | `sp -0x3c` | `0xc0` |
| `ldr` at 0x12f544 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12f558 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x12f564 | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x12f56c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x12f570 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f574 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f580 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f588 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f594 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f598 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f5a0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f5a8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f5b0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f5bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f5c4 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f5d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12f5d4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f5dc | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f5e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f5f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f600 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f608 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f614 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f618 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x12f620 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0x12f628 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12f62c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x12f630 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f634 | store | 4 | `sp -0x64` | `[sp-64]` |
| `ldr` at 0x12f638 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f63c | store | 4 | `sp -0x60` | `[sp-68]` |
| `ldr` at 0x12f640 | load | 4 | `sp +0x10` | `-` |
| `str` at 0x12f644 | store | 4 | `sp -0x5c` | `[sp+16]` |
| `ldr` at 0x12f654 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x12f668 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f670 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f67c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f684 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x12f688 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f68c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f690 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x12f6a4 | load | 4 | `sp +0x10` | `-` |
| `ldr` at 0x12f6a8 | load | 4 | `abs 0x12e854` | `-` |
| `ldr` at 0x12f6b0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f6b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f6c4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x12f6cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f6d4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f6e0 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0x12f6e4 | load | 4 | `sp -0x48` | `-` |
| `ldm` at 0x12f6ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x12f6ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x12f6ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12d870 | `(unknown callee) 0x201fc` | `0x60` | `r1` | `r2` | `r3` |
| `bl` at 0x12d88c | `(unknown callee) 0x205bc` | `[sp+16]` | `0x190870` | `?` | `ret(0x201fc)` |
| `bl` at 0x12d8a0 | `(unknown callee) 0x201fc` | `0x60` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0x12d8bc | `(unknown callee) 0x205bc` | `[sp+16]` | `0x190870` | `?` | `ret(0x201fc)` |
| `bl` at 0x12d8c4 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12da0c | `(unknown callee) 0x12cc8c` | `0x0` | `0x1` | `[sp+16]` | `?` |
| `bl` at 0x12da20 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12da28 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12da44 | `(unknown callee) 0x12cc8c` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0x12da58 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12da60 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12dbec | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0x12dc00 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12dc08 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12dd84 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12dd98 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12dda0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12df4c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12df60 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12df68 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e10c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e120 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e128 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e310 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e324 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e32c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e390 | `(unknown callee) 0x205bc` | `[sp+16]` | `0x1908d4` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0x12e398 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12e3a0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e400 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e414 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e41c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e450 | `(unknown callee) 0x205bc` | `[sp+16]` | `0x19089c` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0x12e458 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12e460 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e5d4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e5e8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e5f0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e654 | `(unknown callee) 0x205bc` | `[sp+16]` | `0x1908d4` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0x12e65c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12e664 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e7c8 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e7dc | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e7e4 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12e9b4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12e9c8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12e9d0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12ea34 | `(unknown callee) 0x205bc` | `[sp+16]` | `0x1908d4` | `[[sp-68]]` | `[sp-68]` |
| `bl` at 0x12ea3c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12ea44 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12ed6c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12ed80 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12ed88 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12ef6c | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `[sp-52]` |
| `bl` at 0x12ef80 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12ef88 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f0fc | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12f110 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12f118 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f260 | `(unknown callee) 0x12cc8c` | `0x0` | `0x0` | `[sp+16]` | `?` |
| `bl` at 0x12f274 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12f27c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f2cc | `(unknown callee) 0x12cc8c` | `0x1` | `?` | `[sp+16]` | `?` |
| `bl` at 0x12f2e0 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12f2e8 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f32c | `(unknown callee) 0x12cee4` | `0x0` | `[sp-52]` | `0x0` | `[sp+16]` |
| `bl` at 0x12f340 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12cee4)` |
| `bl` at 0x12f348 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f36c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f374 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f4b4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12f4c8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12f4d0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f4f0 | `(unknown callee) 0x130f24` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+8]` |
| `bl` at 0x12f508 | `(unknown callee) 0x205bc` | `[sp+16]` | `0x190900` | `?` | `ret(0x130f24)` |
| `bl` at 0x12f510 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f518 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f658 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-52]` |
| `bl` at 0x12f66c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12f674 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f694 | `(unknown callee) 0x130f24` | `[sp-56]` | `[sp-60]` | `[sp-68]` | `[sp+12]` |
| `bl` at 0x12f6ac | `(unknown callee) 0x205bc` | `[sp+16]` | `0x190928` | `?` | `ret(0x130f24)` |
| `bl` at 0x12f6b4 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f6bc | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f6d0 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `ret(0x130f24)` |
| `bl` at 0x12f6d8 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x12d880 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x12d8b0 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `beq` at 0x12d8e0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12d9fc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12da18 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x12da50 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `beq` at 0x12da7c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12dbf8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12dd90 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12ddd0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12dde0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12df58 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12e118 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12e140 | `[sp-36] != 0x18` | `[sp-36] == 0x18` |
| `beq` at 0x12e1a4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12e31c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12e37c | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0x12e40c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12e43c | `[[sp-68]] == 0x54007ff5` | `[[sp-68]] != 0x54007ff5` |
| `beq` at 0x12e47c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12e5e0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12e640 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `bne` at 0x12e7d4 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12e814 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12e9c0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12ea20 | `[[sp-68]] == [sp-44]` | `[[sp-68]] != [sp-44]` |
| `beq` at 0x12ea60 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12ebcc | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12ed78 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12eda0 | `[sp-36] != 0x8` | `[sp-36] == 0x8` |
| `beq` at 0x12edf0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x12ee00 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12ef78 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12f108 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12f250 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12f26c | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x12f298 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12f2a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12f2b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12f2d8 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x12f304 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12f314 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x12f338 | `ret(0x12cee4) != 0x0` | `ret(0x12cee4) == 0x0` |
| `bne` at 0x12f364 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x12f3d8 | `C == 1` | `not (C == 1)` |
| `bls` at 0x12f414 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x12f458 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x12f4c0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12f4fc | `ret(0x130f24) != 0x0` | `ret(0x130f24) == 0x0` |
| `bhs` at 0x12f57c | `C == 1` | `not (C == 1)` |
| `bls` at 0x12f5b8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x12f5fc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x12f664 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12f6a0 | `ret(0x130f24) != 0x0` | `ret(0x130f24) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x12f6ec | `r0` | `?` |

## `jmstr_sel_sic_multi`

- `jmstr_sel_sic_multi`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x12f6f0 to 0x130f24.
  Established. A word loaded from entry r0 plus 0x10 is stored at entry sp minus 0x68 (`ldr` at 0x12f71c, `str` at 0x12f720).
  The result from unnamed 0x201fc is stored as a word at entry sp minus 0x3c (`bl` at 0x12f760, `str` at 0x12f764).
  A later word store writes 0x0800ffe0 to an unknown effective target (`str` at 0x1302b4).
  Another writes [sp-40] as a word to the address held in [sp-24] (`str` at 0x130d28).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x12f700 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x12f704 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x12f708 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x12f70c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x12f714 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f71c | load | 4 | `r0 +0x10` | `-` |
| `str` at 0x12f720 | store | 4 | `sp -0x68` | `[r0+16]` |
| `ldr` at 0x12f724 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f730 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f740 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f744 | load | 4 | `abs 0x13060c` | `-` |
| `ldr` at 0x12f748 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x12f754 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12f764 | store | 4 | `sp -0x3c` | `ret(0x201fc)` |
| `ldr` at 0x12f768 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f774 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f778 | load | 4 | `abs 0x130610` | `-` |
| `str` at 0x12f784 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12f794 | store | 4 | `sp -0x40` | `ret(0x201fc)` |
| `ldr` at 0x12f798 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f7a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f7a8 | load | 4 | `abs 0x130610` | `-` |
| `ldr` at 0x12f7b0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12f7bc | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12f7cc | store | 4 | `sp -0x44` | `ret(0x201fc)` |
| `ldr` at 0x12f7d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x12f7dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f7e0 | load | 4 | `abs 0x130610` | `-` |
| `ldr` at 0x12f7e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f7f0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12f7fc | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12f808 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12f80c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f810 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f81c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12f830 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x12f834 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f84c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x12f858 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f864 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12f868 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f86c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12f874 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12f880 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12f884 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f888 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f894 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12f8a8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x12f8ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f8c4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x12f8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12f8dc | load | 4 | `(unknown)` | `-` |
| `str` at 0x12f8f0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x12f8f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12f8fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12f910 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f914 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f920 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x12f924 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x12f938 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f93c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f950 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f958 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f960 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f96c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x12f978 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12f97c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x12f990 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12f998 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12f9a0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12f9ac | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x12f9b4 | load | 4 | `sp -0x68` | `-` |
| `str` at 0x12f9bc | store | 4 | `sp -0x94` | `[sp-104]-1` |
| `ldr` at 0x12f9c0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12f9d0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12f9d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f9d8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12f9e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12f9ec | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12f9f8 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12f9fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fa04 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12fa10 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x12fa20 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fa28 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fa30 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x12fa34 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fa38 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fa48 | load | 4 | `abs 0x130614` | `-` |
| `ldr` at 0x12fa50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fa58 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fa60 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12fa68 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x12fa6c | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x12fa70 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fa78 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x12fa84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12fa88 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x12fa8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fa90 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12fa94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fa98 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fa9c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12faa0 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x12fab0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12fac4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12facc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12fad4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12fae0 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x12fae8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12faec | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x12faf0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12faf4 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12faf8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fafc | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fb00 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12fb04 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x12fb14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12fb28 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fb30 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12fb38 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12fb44 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12fb50 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12fb54 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fb58 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fb64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fb6c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fb78 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12fb7c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fb84 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12fb90 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x12fb94 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fba8 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x12fbac | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x12fbb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12fbc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12fbc8 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x12fbd0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x12fbd8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fbe0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fbe8 | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x12fbec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fbf0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fc08 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fc10 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fc18 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12fc20 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x12fc24 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x12fc28 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fc2c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fc38 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x12fc3c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12fc40 | store | 4 | `sp -0xb0` | `[sp-36]` |
| `ldr` at 0x12fc44 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fc48 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12fc4c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fc50 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fc54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12fc58 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x12fc68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12fc7c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fc84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12fc8c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12fc98 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x12fca0 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x12fcb8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12fccc | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12fcd0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fcd4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fce0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fce8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fcf4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12fcf8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fd00 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12fd0c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x12fd1c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fd24 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fd2c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x12fd30 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fd34 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fd44 | load | 4 | `abs 0x130614` | `-` |
| `ldr` at 0x12fd4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fd54 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fd5c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12fd64 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x12fd68 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x12fd6c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12fd70 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x12fd74 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fd78 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12fd7c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fd80 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fd84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12fd88 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x12fd98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12fdac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fdb4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12fdbc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12fdc8 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x12fdd4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12fdd8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fddc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fde8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fdf0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12fdfc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12fe00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fe08 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12fe14 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x12fe18 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fe2c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x12fe30 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x12fe3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x12fe48 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12fe4c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x12fe54 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x12fe5c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fe64 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12fe6c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x12fe70 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12fe74 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12fe8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fe94 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12fe9c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12fea4 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x12fea8 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x12feac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12feb0 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x12feb4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12feb8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12febc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fec0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fec4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12fec8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x12fed8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x12feec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12fef4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x12fefc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x12ff08 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x12ff10 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x12ff28 | load | 4 | `(unknown)` | `-` |
| `str` at 0x12ff3c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x12ff40 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12ff44 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12ff50 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12ff58 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12ff64 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x12ff68 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12ff70 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x12ff7c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x12ff8c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12ff94 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x12ff9c | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x12ffa0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x12ffa4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x12ffb4 | load | 4 | `abs 0x130614` | `-` |
| `ldr` at 0x12ffbc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x12ffc4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x12ffcc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x12ffd4 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x12ffd8 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x12ffdc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x12ffe0 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x12ffe4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x12ffe8 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x12ffec | load | 4 | `sp -0x40` | `-` |
| `str` at 0x12fff0 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x12fff4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x12fff8 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130008 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x13001c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130024 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13002c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130038 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x130044 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130048 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13004c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130058 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130060 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x13006c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x130070 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130078 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x130084 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x130088 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x13009c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x1300a0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x1300ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1300b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x1300bc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x1300c4 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1300cc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1300d4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x1300dc | store | 4 | `sp -0x2c` | `[sp-148]+1` |
| `ldr` at 0x1300e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1300e4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1300fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130104 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x13010c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130114 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x130118 | store | 4 | `sp -0x24` | `[sp-152]` |
| `ldr` at 0x13011c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x130120 | store | 4 | `sp -0xb0` | `[sp-152]` |
| `ldr` at 0x130124 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130128 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x13012c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130130 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x130134 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130138 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130148 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x13015c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130164 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13016c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130178 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x130184 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130188 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13018c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130198 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1301a0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1301ac | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1301b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1301b8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x1301c4 | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x1301c8 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x1301cc | store | 4 | `sp -0x2c` | `[sp-148]` |
| `ldr` at 0x1301d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1301d4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1301e4 | load | 4 | `abs 0x130614` | `-` |
| `ldr` at 0x1301ec | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1301f4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1301fc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130204 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x130208 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x13020c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x130210 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x130214 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130218 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x13021c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130220 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x130224 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130228 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130238 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x13024c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130254 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13025c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130268 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130270 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x130278 | store | 4 | `sp -0x94` | `[sp-148]-1` |
| `str` at 0x130284 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130288 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13028c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130298 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1302b0 | load | 4 | `abs 0x130618` | `-` |
| `str` at 0x1302b4 | store | 4 | `(unknown)` | `0x800ffe0` |
| `ldr` at 0x1302b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1302c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1302cc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1302d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1302d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1302e4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1302e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1302f0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130304 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130308 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130310 | store | 4 | `sp -0x24` | `ret(0x1310f0)` |
| `ldr` at 0x130314 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x130318 | store | 4 | `sp -0xb0` | `ret(0x1310f0)` |
| `ldr` at 0x13031c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130320 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x130324 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130328 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x13032c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130330 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130340 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130354 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x13035c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130364 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130370 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130378 | load | 4 | `abs 0x130618` | `-` |
| `str` at 0x13037c | store | 4 | `sp -0x88` | `0x800ffe0` |
| `ldr` at 0x130380 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x130384 | load | 4 | `abs 0x130620` | `-` |
| `ldr` at 0x130390 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x130398 | load | 4 | `abs 0x130620` | `-` |
| `str` at 0x1303a4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x1303a8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1303ac | load | 4 | `abs 0x130624` | `-` |
| `ldr` at 0x1303b8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1303c0 | load | 4 | `abs 0x130624` | `-` |
| `str` at 0x1303cc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x1303d0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1303d4 | load | 4 | `abs 0x130628` | `-` |
| `ldr` at 0x1303e0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1303e8 | load | 4 | `abs 0x130628` | `-` |
| `str` at 0x1303f4 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x1303f8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x130408 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x13041c | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x130420 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x130430 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x130444 | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x13044c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x130454 | store | 4 | `sp -0x9c` | `?` |
| `str` at 0x13045c | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0x130460 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130464 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130470 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130488 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x13048c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130494 | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x13049c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1304a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1304b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1304d0 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1304d8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x1304e0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x1304ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x1304f4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1304f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130500 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130508 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x13050c | load | 4 | `sp -0x68` | `-` |
| `str` at 0x130514 | store | 4 | `sp -0x88` | `?` |
| `str` at 0x13051c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130520 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130524 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130530 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130538 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130540 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130558 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13055c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x130568 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x13056c | load | 4 | `abs 0x13061c` | `-` |
| `ldr` at 0x130574 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x13057c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130584 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130590 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130598 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1305a0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x1305ac | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0x1305b0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x1305b4 | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0x1305b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1305bc | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0x1305c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1305c4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x1305c8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1305e8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1305f0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1305f8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130604 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x130638 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x13063c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130640 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x13064c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130660 | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x130664 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x130670 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13067c | load | 4 | `(unknown)` | `-` |
| `str` at 0x130680 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x130684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13069c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x1306a8 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x1306b4 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x1306b8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1306c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x1306cc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1306d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1306d8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x1306e4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1306e8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x1306f0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130704 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130708 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130710 | store | 4 | `sp -0x24` | `ret(0x1310f0)` |
| `ldr` at 0x130714 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x130718 | store | 4 | `sp -0xb0` | `ret(0x1310f0)` |
| `ldr` at 0x13071c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130720 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x130724 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130728 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x13072c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130730 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130740 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130754 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x13075c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130764 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130770 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130778 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x13078c | store | 4 | `sp -0xb0` | `0x1` |
| `ldr` at 0x130790 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x130794 | store | 4 | `sp -0xac` | `[sp-48]` |
| `ldr` at 0x130798 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x13079c | store | 4 | `sp -0xa8` | `[sp-68]` |
| `ldr` at 0x1307a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1307a4 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x1307a8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x1307c8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1307d0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1307d8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1307e4 | store | 4 | `sp -0xa0` | `0x0` |
| `str` at 0x1307f0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x1307f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1307f8 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130804 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130820 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x130824 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x13082c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130838 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x13083c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130844 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130850 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x130854 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x13085c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130870 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x130874 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x13087c | store | 4 | `sp -0x24` | `ret(0x1310f0)` |
| `ldr` at 0x130880 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x130884 | store | 4 | `sp -0xb0` | `ret(0x1310f0)` |
| `ldr` at 0x130888 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x13088c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x130890 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130894 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x130898 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13089c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x1308ac | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x1308c0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x1308c8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1308d0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x1308dc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x1308e4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1308e8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1308ec | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x1308f0 | load | 4 | `abs 0x130620` | `-` |
| `ldr` at 0x1308fc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130900 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x130908 | load | 4 | `abs 0x130620` | `-` |
| `str` at 0x130914 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x130918 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13091c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130920 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x130924 | load | 4 | `abs 0x130624` | `-` |
| `ldr` at 0x130930 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130934 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x13093c | load | 4 | `abs 0x130624` | `-` |
| `str` at 0x130948 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x13094c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130950 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130954 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x130958 | load | 4 | `abs 0x130628` | `-` |
| `ldr` at 0x130964 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130968 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x130970 | load | 4 | `abs 0x130628` | `-` |
| `str` at 0x13097c | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x130980 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130984 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130988 | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x130998 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x13099c | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x1309b0 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x1309b4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1309b8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1309bc | load | 4 | `[sp-64] +0x0` | `-` |
| `ldr` at 0x1309cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1309d0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x1309e4 | store | 4 | `[sp-64] +0x0` | `?` |
| `ldr` at 0x1309e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1309ec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x1309f0 | load | 4 | `[sp-64] +0x0` | `-` |
| `str` at 0x1309f4 | store | 4 | `[sp-24] +0x0` | `[[sp-64]]` |
| `str` at 0x1309fc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x130a00 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130a14 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130a18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x130a2c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130a34 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130a3c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130a48 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130a50 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130a60 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130a70 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130a88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130a8c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x130aa0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130aa8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130ab0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130abc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130ac4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130ad4 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130aec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130af4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x130b08 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130b10 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130b18 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130b24 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130b2c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x130b3c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130b44 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130b4c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130b58 | store | 4 | `sp -0xa0` | `0x1` |
| `str` at 0x130b64 | store | 4 | `sp -0x34` | `0x100` |
| `ldr` at 0x130b68 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x130b78 | store | 4 | `sp -0x38` | `0x300` |
| `ldr` at 0x130b7c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x130b90 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x130b9c | store | 4 | `sp -0x28` | `?` |
| `str` at 0x130ba4 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130ba8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130bac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130bb8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130bc0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130bcc | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x130bd0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130bd8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130be0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130be8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130bf4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130bfc | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130c08 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x130c0c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130c14 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130c1c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130c2c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130c38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130c40 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130c4c | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x130c50 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130c58 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130c60 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x130c64 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x130c68 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130c6c | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x130c70 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130c74 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x130c78 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130c7c | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130c8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130ca0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130ca8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130cb0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130cbc | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130cc8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x130ccc | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x130cd0 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130ce4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x130ce8 | load | 4 | `abs 0x13062c` | `-` |
| `ldr` at 0x130cf0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130cf8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130d00 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130d0c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130d14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x130d20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x130d24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x130d28 | store | 4 | `[sp-24] +0x0` | `[sp-40]` |
| `str` at 0x130d30 | store | 4 | `sp -0x34` | `0x40` |
| `ldr` at 0x130d34 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x130d44 | store | 4 | `sp -0x38` | `0xc0` |
| `ldr` at 0x130d48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x130d5c | load | 4 | `sp -0x34` | `-` |
| `str` at 0x130d68 | store | 4 | `sp -0x28` | `?` |
| `str` at 0x130d70 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x130d74 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130d78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130d84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130d8c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130d98 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x130d9c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130da4 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130dac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130db4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130dc0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130dc8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130dd4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x130dd8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130de0 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130de8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x130df8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130e04 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x130e0c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130e18 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x130e1c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x130e24 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x130e2c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x130e30 | store | 4 | `sp -0xb0` | `?` |
| `ldr` at 0x130e34 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x130e38 | store | 4 | `sp -0xac` | `[sp-60]` |
| `ldr` at 0x130e3c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x130e40 | store | 4 | `sp -0xa8` | `[sp-64]` |
| `ldr` at 0x130e44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x130e48 | store | 4 | `sp -0xa4` | `[sp-32]` |
| `ldr` at 0x130e58 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x130e6c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130e74 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130e7c | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130e88 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130e94 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x130e98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x130e9c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130eb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x130eb4 | load | 4 | `abs 0x130630` | `-` |
| `ldr` at 0x130ebc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130ec4 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130ecc | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130ed8 | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x130ee0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x130eec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x130ef0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x130ef4 | store | 4 | `[sp-28] +0x0` | `[sp-40]` |
| `ldr` at 0x130ef8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x130f00 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x130f08 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x130f14 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x130f18 | load | 4 | `sp -0xa0` | `-` |
| `ldm` at 0x130f20 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x130f20 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x130f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x12f74c | `(unknown callee) 0x205bc` | `r3` | `0x190958` | `[r0+16]` | `[r0+16]` |
| `bl` at 0x12f760 | `(unknown callee) 0x201fc` | `0x60` | `r1` | `0x10` | `[r0+16]` |
| `bl` at 0x12f77c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190870` | `?` | `ret(0x201fc)` |
| `bl` at 0x12f790 | `(unknown callee) 0x201fc` | `0x60` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0x12f7ac | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190870` | `?` | `ret(0x201fc)` |
| `bl` at 0x12f7b4 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x12f7c8 | `(unknown callee) 0x201fc` | `0x60` | `?` | `?` | `ret(0x201fc)` |
| `bl` at 0x12f7e4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190870` | `?` | `ret(0x201fc)` |
| `bl` at 0x12f7ec | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x12f7f4 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f918 | `(unknown callee) 0x1310f0` | `sp-136` | `0xc` | `[sp-104]` | `?` |
| `bl` at 0x12f940 | `(unknown callee) 0x12cc8c` | `0x0` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0x12f954 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12f95c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f964 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12f980 | `(unknown callee) 0x12cc8c` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0x12f994 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x12f99c | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12f9a4 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12fa24 | `(unknown callee) 0x131348` | `0x4` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fa54 | `(unknown callee) 0x131348` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fab4 | `(unknown callee) 0x12d2b8` | `0x0` | `0x0` | `0x1` | `[sp-48]` |
| `bl` at 0x12fac8 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12fad0 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12fad8 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12fb18 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x12fb2c | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12fb34 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12fb3c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12fbdc | `(unknown callee) 0x131348` | `[sp-136]` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fc0c | `(unknown callee) 0x131348` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fc6c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x12fc80 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12fc88 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12fc90 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12fd20 | `(unknown callee) 0x131348` | `0xe` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fd50 | `(unknown callee) 0x131348` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fd9c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x12fdb0 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12fdb8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12fdc0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12fe60 | `(unknown callee) 0x131348` | `[sp-136]` | `0x18` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fe90 | `(unknown callee) 0x131348` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0x12fedc | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x12fef0 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x12fef8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x12ff00 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x12ff90 | `(unknown callee) 0x131348` | `0x9` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x12ffc0 | `(unknown callee) 0x131348` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x13000c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130020 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130028 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130030 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x1300d0 | `(unknown callee) 0x131348` | `[sp-136]` | `0x8` | `[sp-60]` | `sp-152` |
| `bl` at 0x130100 | `(unknown callee) 0x131348` | `0x0` | `0x1` | `[sp-60]` | `sp-152` |
| `bl` at 0x13014c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130160 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130168 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130170 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x1301f0 | `(unknown callee) 0x131348` | `0xfff` | `0xc` | `[sp-60]` | `sp-152` |
| `bl` at 0x13023c | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130250 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130258 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130260 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x13030c | `(unknown callee) 0x1310f0` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0x130344 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130358 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130360 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130368 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130570 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190974` | `?` | `?` |
| `bl` at 0x130578 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x130580 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130588 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x1305d8 | `(unknown callee) 0x131598` | `[sp-104]` | `0x8` | `0xc` | `0x1` |
| `bl` at 0x1305ec | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x131598)` |
| `bl` at 0x1305f4 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x1305fc | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x13070c | `(unknown callee) 0x1310f0` | `sp-136` | `0x8` | `[sp-104]` | `[sp-60]` |
| `bl` at 0x130744 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130758 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130760 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130768 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x1307b8 | `(unknown callee) 0x131598` | `[sp-104]` | `0x1` | `0xc` | `0x1` |
| `bl` at 0x1307cc | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x131598)` |
| `bl` at 0x1307d4 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x1307dc | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130878 | `(unknown callee) 0x1310f0` | `sp-136` | `0x20` | `[sp-104]` | `[sp-60]` |
| `bl` at 0x1308b0 | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x1308c4 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x1308cc | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x1308d4 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130a1c | `(unknown callee) 0x12cc8c` | `0x0` | `0x0` | `[sp-32]` | `?` |
| `bl` at 0x130a30 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x130a38 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130a40 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130a90 | `(unknown callee) 0x12cc8c` | `0x1` | `?` | `[sp-32]` | `?` |
| `bl` at 0x130aa4 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12cc8c)` |
| `bl` at 0x130aac | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130ab4 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130af8 | `(unknown callee) 0x12cee4` | `0x0` | `[sp-48]` | `0x0` | `[sp-32]` |
| `bl` at 0x130b0c | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12cee4)` |
| `bl` at 0x130b14 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130b1c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130b40 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x130b48 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130b50 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130c90 | `(unknown callee) 0x12d2b8` | `0x0` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130ca4 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130cac | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130cb4 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130cd4 | `(unknown callee) 0x130f24` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0x130cec | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190900` | `?` | `ret(0x130f24)` |
| `bl` at 0x130cf4 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x130cfc | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130d04 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130e5c | `(unknown callee) 0x12d2b8` | `0x1` | `0x1` | `0x1` | `[sp-48]` |
| `bl` at 0x130e70 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `ret(0x12d2b8)` |
| `bl` at 0x130e78 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130e80 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130ea0 | `(unknown callee) 0x130f24` | `[sp-52]` | `[sp-56]` | `[sp-64]` | `sp-40` |
| `bl` at 0x130eb8 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x190928` | `?` | `ret(0x130f24)` |
| `bl` at 0x130ec0 | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x130ec8 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130ed0 | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |
| `bl` at 0x130efc | `(unknown callee) 0x20514` | `[sp-60]` | `?` | `?` | `?` |
| `bl` at 0x130f04 | `(unknown callee) 0x20514` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0x130f0c | `(unknown callee) 0x20514` | `[sp-68]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x12f72c | `[r0+16] == 0x0` | `[r0+16] != 0x0` |
| `bhi` at 0x12f738 | `[r0+16] > 0x8 (unsigned, 32-bit)` | `[r0+16] <= 0x8 (unsigned, 32-bit)` |
| `bne` at 0x12f770 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x12f7a0 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bne` at 0x12f7d8 | `ret(0x201fc) != 0x0` | `ret(0x201fc) == 0x0` |
| `bhs` at 0x12f818 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12f890 | `C == 1` | `not (C == 1)` |
| `beq` at 0x12f930 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x12f94c | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x12f98c | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `blt` at 0x12f9c8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0x12f9e0 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12fa40 | `C == 1` | `not (C == 1)` |
| `bne` at 0x12fa80 | `[sp-148] != [sp-104]-1` | `[sp-148] == [sp-104]-1` |
| `bne` at 0x12fac0 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x12fb24 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x12fb60 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12fbf8 | `C == 1` | `not (C == 1)` |
| `bne` at 0x12fc78 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12fcc4 | `? == 0x0` | `? != 0x0` |
| `bhs` at 0x12fcdc | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12fd3c | `C == 1` | `not (C == 1)` |
| `bne` at 0x12fda8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x12fde4 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12fe7c | `C == 1` | `not (C == 1)` |
| `bne` at 0x12fee8 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x12ff34 | `? == 0x0` | `? != 0x0` |
| `bhs` at 0x12ff4c | `C == 1` | `not (C == 1)` |
| `bhs` at 0x12ffac | `C == 1` | `not (C == 1)` |
| `bne` at 0x130018 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x130054 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x1300ec | `C == 1` | `not (C == 1)` |
| `bne` at 0x130158 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x130194 | `C == 1` | `not (C == 1)` |
| `bhs` at 0x1301dc | `C == 1` | `not (C == 1)` |
| `bne` at 0x130248 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x130294 | `C == 1` | `not (C == 1)` |
| `bne` at 0x130350 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bhs` at 0x13046c | `C == 1` | `not (C == 1)` |
| `bhs` at 0x13052c | `C == 1` | `not (C == 1)` |
| `beq` at 0x130564 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x1305e4 | `ret(0x131598) != 0x0` | `ret(0x131598) == 0x0` |
| `bhs` at 0x130648 | `C == 1` | `not (C == 1)` |
| `bne` at 0x130750 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x130784 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1307c4 | `ret(0x131598) != 0x0` | `ret(0x131598) == 0x0` |
| `bhs` at 0x130800 | `C == 1` | `not (C == 1)` |
| `bne` at 0x1308bc | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `beq` at 0x130a0c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x130a28 | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x130a5c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x130a6c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x130a7c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x130a9c | `ret(0x12cc8c) != 0x0` | `ret(0x12cc8c) == 0x0` |
| `bne` at 0x130ad0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x130ae0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x130b04 | `ret(0x12cee4) != 0x0` | `ret(0x12cee4) == 0x0` |
| `bne` at 0x130b38 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x130bb4 | `C == 1` | `not (C == 1)` |
| `bls` at 0x130bf0 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x130c34 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x130c9c | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x130ce0 | `ret(0x130f24) != 0x0` | `ret(0x130f24) == 0x0` |
| `beq` at 0x130d1c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `bhs` at 0x130d80 | `C == 1` | `not (C == 1)` |
| `bls` at 0x130dbc | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bls` at 0x130e00 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x130e68 | `ret(0x12d2b8) != 0x0` | `ret(0x12d2b8) == 0x0` |
| `bne` at 0x130eac | `ret(0x130f24) != 0x0` | `ret(0x130f24) == 0x0` |
| `beq` at 0x130ee8 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x130f20 | `r0` | `?` |

## `scanpktInit`

- `scanpktInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13abf8 to 0x13ad88.
  Established. A word loaded from absolute address 0x31505c is incremented by 1 and stored as a word at that same address (`ldr` at 0x13ac40, `add` at 0x13ac44, `str` at 0x13ac48).
  The result from unnamed 0xfb6ec is stored as a word at entry sp minus 0x14 (`bl` at 0x13ac58, `str` at 0x13ac60).
  Word stores write 0 and 0x800 at entry sp minus 0x2c and 0x28 respectively (`str` at 0x13acb4, `str` at 0x13acbc).
  Further word stores write 0x19, 0xa and 4 at entry sp minus 0x24, 0x20 and 0x1c respectively (`str` at 0x13acc4, `str` at 0x13accc, `str` at 0x13acd4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x13ac08 | load | 4 | `abs 0x13ad5c` | `-` |
| `ldr` at 0x13ac0c | load | 4 | `abs 0x31505c` | `-` |
| `ldr` at 0x13ac18 | load | 4 | `abs 0x13ad60` | `-` |
| `str` at 0x13ac1c | store | 4 | `sp -0x2c` | `0x191790` |
| `ldr` at 0x13ac24 | load | 4 | `abs 0x13ad64` | `-` |
| `ldr` at 0x13ac38 | load | 4 | `abs 0x13ad5c` | `-` |
| `ldr` at 0x13ac3c | load | 4 | `abs 0x13ad5c` | `-` |
| `ldr` at 0x13ac40 | load | 4 | `abs 0x31505c` | `-` |
| `str` at 0x13ac48 | store | 4 | `abs 0x31505c` | `[0x31505c]+1` |
| `ldr` at 0x13ac4c | load | 4 | `abs 0x13ad68` | `-` |
| `ldr` at 0x13ac50 | load | 4 | `abs 0x13ad6c` | `-` |
| `str` at 0x13ac60 | store | 4 | `sp -0x14` | `ret(0xfb6ec)` |
| `ldr` at 0x13ac64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13ac70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13ac7c | load | 4 | `abs 0x13ad70` | `-` |
| `str` at 0x13ac80 | store | 4 | `sp -0x2c` | `0x1917e0` |
| `str` at 0x13ac84 | store | 4 | `sp -0x28` | `ret(0x219a4)` |
| `ldr` at 0x13ac8c | load | 4 | `abs 0x13ad64` | `-` |
| `ldr` at 0x13aca0 | load | 4 | `abs 0x13ad74` | `-` |
| `str` at 0x13acb4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x13acbc | store | 4 | `sp -0x28` | `0x800` |
| `str` at 0x13acc4 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0x13accc | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0x13acd4 | store | 4 | `sp -0x1c` | `0x4` |
| `ldr` at 0x13acd8 | load | 4 | `abs 0x13ad78` | `-` |
| `ldr` at 0x13acdc | load | 4 | `abs 0x13ad7c` | `-` |
| `ldr` at 0x13ace0 | load | 4 | `abs 0x13ad80` | `-` |
| `ldr` at 0x13acfc | load | 4 | `abs 0x13ad74` | `-` |
| `ldr` at 0x13ad00 | load | 4 | `abs 0x13ad74` | `-` |
| `ldr` at 0x13ad04 | load | 4 | `abs 0x31e714` | `-` |
| `ldr` at 0x13ad0c | load | 4 | `abs 0x13ad68` | `-` |
| `str` at 0x13ad18 | store | 4 | `sp -0x14` | `ret(0xfa264)` |
| `ldr` at 0x13ad1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13ad28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13ad34 | load | 4 | `abs 0x13ad84` | `-` |
| `str` at 0x13ad38 | store | 4 | `sp -0x2c` | `0x191814` |
| `str` at 0x13ad3c | store | 4 | `sp -0x28` | `ret(0x219a4)` |
| `ldr` at 0x13ad44 | load | 4 | `abs 0x13ad64` | `-` |
| `ldm` at 0x13ad58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13ad58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13ad58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x13ac30 | `(unknown callee) 0x22070` | `0x4` | `0x1917bc` | `0x42` | `0x0` |
| `bl` at 0x13ac58 | `(unknown callee) 0xfb6ec` | `0x31e720` | `0x1917d4` | `0x14` | `[0x31505c]+1` |
| `bl` at 0x13ac74 | `(unknown callee) 0x219a4` | `ret(0xfb6ec)` | `?` | `?` | `ret(0xfb6ec)` |
| `bl` at 0x13ac98 | `(unknown callee) 0x22070` | `0x4` | `0x1917bc` | `0x5a` | `0x0` |
| `bl` at 0x13acac | `(unknown callee) 0xfb5cc` | `0x31e714` | `0xf` | `0x0` | `ret(0xfb6ec)` |
| `bl` at 0x13ace8 | `(unknown callee) 0x22540` | `0x31e71c` | `0x191808` | `0x13ad88` | `0x0` |
| `bl` at 0x13ad10 | `(unknown callee) 0xfa264` | `0x31e714` | `[0x31e714]` | `0x1` | `0x31e720` |
| `bl` at 0x13ad2c | `(unknown callee) 0x219a4` | `ret(0xfa264)` | `?` | `?` | `ret(0xfa264)` |
| `bl` at 0x13ad50 | `(unknown callee) 0x22070` | `0x4` | `0x1917bc` | `0x74` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x13ac14 | `[0x31505c] == 0x0` | `[0x31505c] != 0x0` |
| `beq` at 0x13ac6c | `ret(0xfb6ec) == 0x0` | `ret(0xfb6ec) != 0x0` |
| `beq` at 0x13acf4 | `ret(0x22540) == 0x0` | `ret(0x22540) != 0x0` |
| `beq` at 0x13ad24 | `ret(0xfa264) == 0x0` | `ret(0xfa264) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13ad58 | `r0` | `?` |
