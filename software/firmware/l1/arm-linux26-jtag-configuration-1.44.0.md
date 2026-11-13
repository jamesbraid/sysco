# ARM Linux 2.6 JTAG_Cfg routines in 1.44.0

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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-jtag-configuration-facts.txt) retain all tool sections.

## `JTAG_Cfg_create`

- `JTAG_Cfg_create`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137984 to 0x137b78.
  Established. Word stores write 0 to absolute address 0x31504c and 1 to absolute address 0x315048 (`str` at 0x137a54, `str` at 0x137a60).
  Four later word stores write 0 to unknown effective targets (`str` at 0x137ab4, `str` at 0x137ae0, `str` at 0x137b0c, `str` at 0x137b38).
  A call to unnamed 0x20514 has printed r0-r3 candidates unknown, 0x31e5c4, [sp-24] and unknown (`bl` at 0x137a38).
  The recorded return value in r0 is 1 (`ldm` at 0x137b5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137994 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x137998 | load | 4 | `abs 0x137b60` | `-` |
| `ldr` at 0x13799c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1379a8 | load | 4 | `abs 0x137b64` | `-` |
| `str` at 0x1379ac | store | 4 | `sp -0x24` | `0x190ff8` |
| `ldr` at 0x1379b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1379b4 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x1379bc | load | 4 | `abs 0x137b68` | `-` |
| `str` at 0x1379d0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1379d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1379e0 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x1379e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137a04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137a10 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x137a14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137a34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137a3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137a44 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x137a4c | load | 4 | `abs 0x137b70` | `-` |
| `str` at 0x137a54 | store | 4 | `abs 0x31504c` | `0x0` |
| `ldr` at 0x137a58 | load | 4 | `abs 0x137b74` | `-` |
| `str` at 0x137a60 | store | 4 | `abs 0x315048` | `0x1` |
| `ldr` at 0x137a64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x137a74 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0x137a80 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x137a84 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137a88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x137a94 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x137a98 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137ab4 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137ab8 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x137abc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137ae0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137ae4 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x137ae8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137b0c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137b10 | load | 4 | `abs 0x137b6c` | `-` |
| `ldr` at 0x137b14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137b38 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137b3c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137b44 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x137b50 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0x137b54 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x137b5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x137b5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x137b5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1379c8 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x7c` | `0x0` |
| `bl` at 0x137a38 | `(unknown callee) 0x20514` | `?` | `0x31e5c4` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1379a4 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bhi` at 0x1379dc | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x137a0c | `? == 0x0` | `? != 0x0` |
| `bne` at 0x137a6c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bhs` at 0x137a90 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x137b5c | `r0` | `0x1` |

## `JTAG_Cfg_info_init`

- `JTAG_Cfg_info_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137b78 to 0x137eec.
  Established. The result from unnamed 0x201fc is stored as a word at an unknown effective target (`bl` at 0x137d2c, `str` at 0x137d34).
  A halfword store writes 0 to an unknown effective target (`strh` at 0x137dbc).
  Two byte stores also write 0 to unknown effective targets (`strb` at 0x137df8, `strb` at 0x137e34).
  A later word store writes [sp-28] to an unknown effective target (`str` at 0x137ec0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137b88 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x137b8c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x137b90 | load | 4 | `abs 0x137ed8` | `-` |
| `ldr` at 0x137b94 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x137ba0 | load | 4 | `abs 0x137edc` | `-` |
| `str` at 0x137ba4 | store | 4 | `sp -0x30` | `0x19102c` |
| `ldr` at 0x137ba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137bac | store | 4 | `sp -0x2c` | `r0` |
| `ldr` at 0x137bb0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137bb4 | store | 4 | `sp -0x28` | `r1` |
| `ldr` at 0x137bbc | load | 4 | `abs 0x137ee0` | `-` |
| `ldr` at 0x137bcc | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137bd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137bf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x137c00 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137c04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137c28 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137c2c | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137c30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137c50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137c5c | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137c60 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137c80 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137c88 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137c8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137cb0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137cb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137cc4 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x137ccc | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137cd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137cf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137cfc | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137d00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137d20 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137d34 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0x137d38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137d44 | load | 4 | `abs 0x137ee8` | `-` |
| `str` at 0x137d48 | store | 4 | `sp -0x30` | `0x191054` |
| `ldr` at 0x137d50 | load | 4 | `abs 0x137ee0` | `-` |
| `str` at 0x137d64 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x137d70 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x137d74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137d78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x137d84 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137d88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137da8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137db0 | load | 4 | `(unknown)` | `-` |
| `strh` at 0x137dbc | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0x137dc0 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137de4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137dec | load | 4 | `(unknown)` | `-` |
| `strb` at 0x137df8 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x137dfc | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137e00 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137e20 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137e28 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x137e34 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x137e38 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x137e40 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0x137e48 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137e4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137e68 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137e6c | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137e70 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137e94 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137e98 | load | 4 | `abs 0x137ee4` | `-` |
| `ldr` at 0x137e9c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137ebc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x137ec0 | store | 4 | `(unknown)` | `[sp-28]` |
| `str` at 0x137ec8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x137ecc | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x137ed4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x137ed4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x137ed4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x137ed4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x137bc8 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0xa4` | `0x0` |
| `bl` at 0x137c84 | `(unknown callee) 0x20514` | `?` | `0x31e5c4` | `[sp-24]` | `?` |
| `bl` at 0x137d2c | `(unknown callee) 0x201fc` | `?` | `0x31e5c4` | `[sp-24]` | `?` |
| `bl` at 0x137d5c | `(unknown callee) 0x22070` | `0x3` | `0x191010` | `0xb8` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x137b9c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x137bfc | `? == [sp-28]` | `? != [sp-28]` |
| `beq` at 0x137c58 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x137cbc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bne` at 0x137cf8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x137d40 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x137d80 | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x137ed4 | `r0` | `?` |

## `JTAG_Cfg_port_init`

- `JTAG_Cfg_port_init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x137eec to 0x13831c.
  Established. A byte store preserves the low eight bits of entry r3 at entry sp minus 0x21 (`strb` at 0x137f08).
  The result from unnamed 0x201fc is stored as a word at an unknown effective target (`bl` at 0x1380dc, `str` at 0x1380e4).
  A word store writes 0xfffff7cc at entry sp minus 0x2c (`str` at 0x138114).
  Later stores write the low eight bits of [sp-33]:1 as a byte and [sp] as a word to unknown effective targets (`strb` at 0x1382ac, `str` at 0x1382d8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x137efc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x137f00 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x137f04 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0x137f08 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0x137f0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137f18 | load | 4 | `abs 0x138300` | `-` |
| `str` at 0x137f1c | store | 4 | `sp -0x38` | `0x191084` |
| `ldr` at 0x137f24 | load | 4 | `abs 0x138304` | `-` |
| `ldr` at 0x137f34 | load | 4 | `abs 0x138308` | `-` |
| `ldr` at 0x137f38 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x137f44 | load | 4 | `abs 0x13830c` | `-` |
| `str` at 0x137f48 | store | 4 | `sp -0x38` | `0x1910b0` |
| `ldr` at 0x137f4c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137f50 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x137f54 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x137f58 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x137f60 | load | 4 | `abs 0x138304` | `-` |
| `ldr` at 0x137f70 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x137f74 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137f90 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137f94 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x137f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x137fb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x137fbc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x137fc8 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x137fcc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x137ff0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x137ff4 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x137ff8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138018 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138024 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138048 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138050 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138054 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138078 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x13807c | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138080 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1380a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1380ac | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x1380b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1380d0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1380e4 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0x1380e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1380f4 | load | 4 | `abs 0x138314` | `-` |
| `str` at 0x1380f8 | store | 4 | `sp -0x38` | `0x191054` |
| `ldr` at 0x138100 | load | 4 | `abs 0x138304` | `-` |
| `ldr` at 0x138110 | load | 4 | `abs 0x138318` | `-` |
| `str` at 0x138114 | store | 4 | `sp -0x2c` | `0xfffff7cc` |
| `ldr` at 0x13811c | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138120 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138140 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x138144 | store | 4 | `(unknown)` | `[sp-32]` |
| `str` at 0x13814c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x138150 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x138154 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x138160 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138184 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x13818c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138194 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x13819c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1381a4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1381a8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1381ac | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x1381b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1381d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1381d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1381e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x1381e8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x1381f0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x1381f4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1381f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x138200 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x138208 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x13820c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138228 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x13822c | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138230 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138254 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x138258 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x13825c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13827c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x138280 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x138284 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x138288 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x1382a8 | load | 1 | `sp -0x21` | `-` |
| `strb` at 0x1382ac | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0x1382b0 | load | 4 | `abs 0x138310` | `-` |
| `ldr` at 0x1382b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1382d4 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x1382d8 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x1382dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1382e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1382f0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x1382f4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x1382fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1382fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1382fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1382fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x137f30 | `(unknown callee) 0x22070` | `0x9` | `0x191010` | `0xd5` | `0x0` |
| `bl` at 0x137f6c | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0xd8` | `0x0` |
| `bl` at 0x13804c | `(unknown callee) 0x20514` | `?` | `0x31e5c4` | `[sp-24]` | `?` |
| `bl` at 0x1380dc | `(unknown callee) 0x201fc` | `?` | `0x31e5c4` | `[sp-24]` | `?` |
| `bl` at 0x13810c | `(unknown callee) 0x22070` | `0x3` | `0x191010` | `0xed` | `0x0` |
| `bl` at 0x1382e0 | `(unknown callee) 0x1384bc` | `[sp-24]` | `0x31e5c4` | `?` | `[sp]` |
| `bl` at 0x1382e8 | `(unknown callee) 0x13c2dc` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x137f14 | `r2 != 0x0` | `r2 == 0x0` |
| `bls` at 0x137f40 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0x137fc4 | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0x138020 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x1380a8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x1380f0 | `? != 0x0` | `? == 0x0` |
| `bhs` at 0x13815c | `C == 1` | `not (C == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1382fc | `r0` | `?` |

## `JTAG_Cfg_port_reset`

- `JTAG_Cfg_port_reset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x13831c to 0x1384bc.
  Established. Word stores place 0 and 0xc at entry sp minus 0x20 and 0x24 respectively (`str` at 0x138374, `str` at 0x13837c).
  A separate path writes [sp-20] plus 1 at entry sp minus 0x24 (`str` at 0x138394).
  A byte store writes 0 to an unknown effective target (`strb` at 0x138454).
  The call to unnamed 0x13c2dc has printed r0-r3 candidates [sp-28], 0x31e5c4, unknown and 0 (`bl` at 0x138488).
  The return record places 0 in r0 (`ldm` at 0x1384a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x13832c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x138330 | load | 4 | `abs 0x1384ac` | `-` |
| `ldr` at 0x138334 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138340 | load | 4 | `abs 0x1384b0` | `-` |
| `str` at 0x138344 | store | 4 | `sp -0x2c` | `0x1910d8` |
| `ldr` at 0x138348 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x13834c | store | 4 | `sp -0x28` | `r0` |
| `ldr` at 0x138354 | load | 4 | `abs 0x1384b4` | `-` |
| `ldr` at 0x138364 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138374 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x13837c | store | 4 | `sp -0x24` | `0xc` |
| `ldr` at 0x138384 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138388 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x13838c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138394 | store | 4 | `sp -0x24` | `[sp-20]+1` |
| `ldr` at 0x138398 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x13839c | store | 4 | `sp -0x1c` | `[sp-32]` |
| `ldr` at 0x1383a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1383a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1383b0 | load | 4 | `abs 0x1384b8` | `-` |
| `ldr` at 0x1383b4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1383d0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x1383d4 | load | 4 | `abs 0x1384b8` | `-` |
| `ldr` at 0x1383d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1383fc | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x138400 | load | 4 | `abs 0x1384b8` | `-` |
| `ldr` at 0x138404 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138428 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x13842c | load | 4 | `abs 0x1384b8` | `-` |
| `ldr` at 0x138430 | load | 4 | `sp -0x1c` | `-` |
| `strb` at 0x138454 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x138458 | load | 4 | `abs 0x1384b8` | `-` |
| `ldr` at 0x13845c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138480 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x138484 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x13848c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138494 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldm` at 0x1384a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1384a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1384a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x138360 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x114` | `0x0` |
| `bl` at 0x138488 | `(unknown callee) 0x13c2dc` | `[sp-28]` | `0x31e5c4` | `?` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x13833c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bne` at 0x13836c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x1383ac | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1384a8 | `r0` | `0x0` |

## `JTAG_Cfg_calc_ir_vals`

- `JTAG_Cfg_calc_ir_vals`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1384bc to 0x138644.
  Established. A halfword store writes the low 16 bits of [sp-28] to an unknown effective target (`strh` at 0x138580).
  The later word store writes [sp-28] to an unknown effective target (`str` at 0x138600).
  Another word store writes [sp-24] plus 1 at entry sp minus 0x18 (`str` at 0x1385d0).
  The call through an unknown target has printed r0-r3 candidates 4, 0x31e5c4, unknown and [0x1a0878] (`mov` at 0x138620).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1384cc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1384d0 | load | 4 | `abs 0x13862c` | `-` |
| `ldr` at 0x1384d4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1384e0 | load | 4 | `abs 0x138630` | `-` |
| `str` at 0x1384e4 | store | 4 | `sp -0x24` | `0x1910f8` |
| `ldr` at 0x1384e8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1384ec | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0x1384f4 | load | 4 | `abs 0x138634` | `-` |
| `ldr` at 0x1384f8 | load | 4 | `abs 0x138638` | `-` |
| `str` at 0x138508 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x138510 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x138514 | load | 4 | `abs 0x13863c` | `-` |
| `ldr` at 0x138518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138538 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13853c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138548 | load | 4 | `abs 0x13863c` | `-` |
| `ldr` at 0x13854c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13856c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138574 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13857c | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0x138580 | store | 2 | `(unknown)` | `([sp-28] & 0xffff)` |
| `ldr` at 0x138584 | load | 4 | `abs 0x13863c` | `-` |
| `ldr` at 0x138588 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1385a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1385b0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x1385b8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x1385bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1385c4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x1385c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1385d0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x1385d8 | load | 4 | `abs 0x13863c` | `-` |
| `ldr` at 0x1385dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1385fc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138600 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x138604 | load | 4 | `abs 0x13862c` | `-` |
| `ldr` at 0x138608 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138614 | load | 4 | `abs 0x138640` | `-` |
| `ldr` at 0x138618 | load | 4 | `abs 0x1a0878` | `-` |
| `ldm` at 0x138628 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x138628 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x138628 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x138500 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x147` | `0x0` |
| `mov` at 0x138620 | `(unknown callee) (unknown target)` | `0x4` | `0x31e5c4` | `?` | `[0x1a0878]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x1384dc | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0x138544 | `? <= [sp-24] (unsigned, 32-bit)` | `? > [sp-24] (unsigned, 32-bit)` |
| `bls` at 0x138610 | `[0x315034] <= 0x3 (unsigned, 32-bit)` | `[0x315034] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x138628 | `r0` | `?` |

## `JTAG_Cfg_set_ports_f`

- `JTAG_Cfg_set_ports_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x138644 to 0x138850.
  Established. The word loaded from absolute address 0x31504c is stored at entry sp minus 0x18 (`ldr` at 0x1386a8, `str` at 0x1386ac).
  A separate load from absolute address 0x315048 is stored at entry sp minus 0x14 (`ldr` at 0x1386b4, `str` at 0x1386b8).
  A later word store writes an unknown value to absolute address 0x315048 (`str` at 0x138798).
  Another writes [sp-24] to absolute address 0x31504c (`str` at 0x1387a4).
  A word store on a different path writes 0xfffff7c3 at entry sp minus 0x24 (`str` at 0x13875c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x138654 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x138658 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x13865c | load | 4 | `abs 0x138828` | `-` |
| `ldr` at 0x138660 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x13866c | load | 4 | `abs 0x13882c` | `-` |
| `str` at 0x138670 | store | 4 | `sp -0x38` | `0x191118` |
| `ldr` at 0x138674 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138678 | store | 4 | `sp -0x34` | `r0` |
| `ldr` at 0x13867c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138680 | store | 4 | `sp -0x30` | `r1` |
| `ldr` at 0x138688 | load | 4 | `abs 0x138830` | `-` |
| `ldr` at 0x13868c | load | 4 | `abs 0x138834` | `-` |
| `ldr` at 0x138698 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1386a4 | load | 4 | `abs 0x138838` | `-` |
| `ldr` at 0x1386a8 | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x1386ac | store | 4 | `sp -0x18` | `[0x31504c]` |
| `ldr` at 0x1386b0 | load | 4 | `abs 0x13883c` | `-` |
| `ldr` at 0x1386b4 | load | 4 | `abs 0x315048` | `-` |
| `str` at 0x1386b8 | store | 4 | `sp -0x14` | `[0x315048]` |
| `ldr` at 0x1386c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1386d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1386d8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x1386dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1386e8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0x1386ec | load | 4 | `abs 0x13883c` | `-` |
| `ldr` at 0x1386f0 | load | 4 | `abs 0x315048` | `-` |
| `ldr` at 0x1386f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138700 | load | 4 | `abs 0x138838` | `-` |
| `ldr` at 0x138704 | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x138708 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138718 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x138720 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x13872c | load | 4 | `abs 0x138840` | `-` |
| `str` at 0x138730 | store | 4 | `sp -0x38` | `0x19114c` |
| `ldr` at 0x138734 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138738 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x13873c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138740 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x138748 | load | 4 | `abs 0x138830` | `-` |
| `ldr` at 0x13874c | load | 4 | `abs 0x138844` | `-` |
| `ldr` at 0x138758 | load | 4 | `abs 0x138848` | `-` |
| `str` at 0x13875c | store | 4 | `sp -0x24` | `0xfffff7c3` |
| `ldr` at 0x138764 | load | 4 | `abs 0x13883c` | `-` |
| `str` at 0x138768 | store | 4 | `sp -0x28` | `0x315048` |
| `ldr` at 0x13876c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138770 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138784 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x13878c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x138794 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x138798 | store | 4 | `abs 0x315048` | `?` |
| `ldr` at 0x13879c | load | 4 | `abs 0x138838` | `-` |
| `ldr` at 0x1387a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x1387a4 | store | 4 | `abs 0x31504c` | `[sp-24]` |
| `ldr` at 0x1387a8 | load | 4 | `abs 0x13883c` | `-` |
| `ldr` at 0x1387ac | load | 4 | `abs 0x315048` | `-` |
| `ldr` at 0x1387b8 | load | 4 | `abs 0x138838` | `-` |
| `ldr` at 0x1387bc | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x1387d0 | load | 4 | `abs 0x138828` | `-` |
| `ldr` at 0x1387d4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x1387e0 | load | 4 | `abs 0x13884c` | `-` |
| `str` at 0x1387e4 | store | 4 | `sp -0x38` | `0x19118c` |
| `ldr` at 0x1387e8 | load | 4 | `abs 0x13883c` | `-` |
| `ldr` at 0x1387ec | load | 4 | `abs 0x315048` | `-` |
| `str` at 0x1387f0 | store | 4 | `sp -0x34` | `[0x315048]` |
| `ldr` at 0x1387f4 | load | 4 | `abs 0x138838` | `-` |
| `ldr` at 0x1387f8 | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x1387fc | store | 4 | `sp -0x30` | `[0x31504c]` |
| `ldr` at 0x138804 | load | 4 | `abs 0x138830` | `-` |
| `str` at 0x138818 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x13881c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x138824 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x138824 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x138824 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x138694 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x15e` | `0x0` |
| `bl` at 0x138754 | `(unknown callee) 0x22070` | `0x3` | `0x191010` | `0x175` | `0x0` |
| `bl` at 0x1387cc | `(unknown callee) 0x134374` | `?` | `?` | `?` | `?` |
| `bl` at 0x138810 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x194` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x138668 | `[0x315034] <= 0x3 (unsigned, 32-bit)` | `[0x315034] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0x1386a0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x1386c8 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bne` at 0x1386fc | `[0x315048] != [sp-20]` | `[0x315048] == [sp-20]` |
| `bne` at 0x138710 | `[0x31504c] != [sp-24]` | `[0x31504c] == [sp-24]` |
| `bls` at 0x138728 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x13877c | `Z == 0` | `not (Z == 0)` |
| `bls` at 0x1387dc | `[0x315034] <= 0x3 (unsigned, 32-bit)` | `[0x315034] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x138824 | `r0` | `?` |

## `JTAG_Cfg_find_chip`

- `JTAG_Cfg_find_chip`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x138850 to 0x138ae4.
  Established. The call to unnamed 0x22070 has printed r0-r3 candidates 6, 0x191010, 0x1af and 0 (`bl` at 0x1388b8).
  Computed byte loads retain unknown effective addresses (`ldrb` at 0x1389d0, `ldrb` at 0x138a74).
  Two later word stores at entry sp minus 0x24 write [sp-36] plus 1 on their respective paths (`str` at 0x1389f4, `str` at 0x138ab4).
  The word store at entry sp minus 0x2c after the final unsupported mvn has an unknown value (`mvn` at 0x138abc, `str` at 0x138ac0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x138860 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x138864 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x138868 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x13886c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x138870 | load | 4 | `abs 0x138ad0` | `-` |
| `ldr` at 0x138874 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138880 | load | 4 | `abs 0x138ad4` | `-` |
| `str` at 0x138884 | store | 4 | `sp -0x40` | `0x1911c0` |
| `ldr` at 0x138888 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x13888c | store | 4 | `sp -0x3c` | `r0` |
| `ldr` at 0x138890 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138894 | store | 4 | `sp -0x38` | `r1` |
| `ldr` at 0x138898 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x13889c | store | 4 | `sp -0x34` | `r2` |
| `ldr` at 0x1388a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x1388a4 | store | 4 | `sp -0x30` | `r3` |
| `ldr` at 0x1388ac | load | 4 | `abs 0x138ad8` | `-` |
| `ldr` at 0x1388b0 | load | 4 | `abs 0x138adc` | `-` |
| `ldr` at 0x1388bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1388c8 | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x1388cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1388ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x1388fc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138904 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x138910 | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x138914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138934 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x138938 | load | 4 | `(unknown)` | `-` |
| `str` at 0x13894c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138954 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138960 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138964 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x138968 | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x13896c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13898c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x138990 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x13899c | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x1389a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1389c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x1389c8 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x1389d0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x1389d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x1389e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1389e4 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x1389ec | load | 4 | `sp -0x24` | `-` |
| `str` at 0x1389f4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x138a00 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x138a08 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x138a0c | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x138a10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138a30 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x138a34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138a40 | load | 4 | `abs 0x138ae0` | `-` |
| `ldr` at 0x138a44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138a64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x138a6c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x138a74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x138a78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x138a84 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x138a88 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x138a94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x138a98 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x138aa0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x138aa8 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x138aac | load | 4 | `sp -0x24` | `-` |
| `str` at 0x138ab4 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0x138ac0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138ac4 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x138acc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x138acc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x138acc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1388b8 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x1af` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x13887c | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x1388c4 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x1388f4 | `? != 0x0` | `? == 0x0` |
| `blt` at 0x13890c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bhs` at 0x138940 | `[sp-28] >= ? (unsigned, 32-bit)` | `[sp-28] < ? (unsigned, 32-bit)` |
| `bne` at 0x13895c | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bhs` at 0x138998 | `[sp-36] >= ? (unsigned, 32-bit)` | `[sp-36] < ? (unsigned, 32-bit)` |
| `bne` at 0x1389dc | `? != [sp-32]` | `? == [sp-32]` |
| `bhs` at 0x138a3c | `[sp-36] >= ? (unsigned, 32-bit)` | `[sp-36] < ? (unsigned, 32-bit)` |
| `bne` at 0x138a80 | `? != [sp-32]` | `? == [sp-32]` |
| `bne` at 0x138a90 | `[sp-40] != [sp-28]` | `[sp-40] == [sp-28]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x138acc | `r0` | `[sp-44]` |

## `JTAG_Cfg_pid`

- `JTAG_Cfg_pid`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x138ae4 to 0x138b24.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x138af4).
  The computed word load has an unknown effective address (`ldr` at 0x138b14).
  The return record retains an unknown value in r0 (`ldm` at 0x138b1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x138af4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x138af8 | load | 4 | `abs 0x138b20` | `-` |
| `ldr` at 0x138afc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138b14 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x138b1c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x138b1c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x138b1c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x138b1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x138b1c | `r0` | `?` |

## `JTAG_Cfg_display_f`

- `JTAG_Cfg_display_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x138b24 to 0x138e3c.
  Established. One call to unnamed 0x243d4 has printed r0-r3 candidates 0x191224, [0x315048], unknown and 0x315048 (`bl` at 0x138b74).
  A separate call has candidates 0x191234, [0x31504c], unknown and 0x31504c (`bl` at 0x138b84).
  Word stores at entry sp minus 0x20 and 0x1c have unknown values (`str` at 0x138c38, `str` at 0x138c64).
  A computed halfword load retains an unknown effective address (`ldrh` at 0x138d48).
  Later word stores at entry sp minus 0x18 and 0x14 write [sp-24] plus 1 and [sp-20] plus 1 respectively (`str` at 0x138ddc, `str` at 0x138dec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x138b34 | load | 4 | `abs 0x138e04` | `-` |
| `ldr` at 0x138b38 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138b44 | load | 4 | `abs 0x138e08` | `-` |
| `str` at 0x138b48 | store | 4 | `sp -0x20` | `0x1911f8` |
| `ldr` at 0x138b50 | load | 4 | `abs 0x138e0c` | `-` |
| `ldr` at 0x138b54 | load | 4 | `abs 0x138e10` | `-` |
| `ldr` at 0x138b60 | load | 4 | `abs 0x138e14` | `-` |
| `ldr` at 0x138b68 | load | 4 | `abs 0x138e18` | `-` |
| `ldr` at 0x138b6c | load | 4 | `abs 0x138e1c` | `-` |
| `ldr` at 0x138b70 | load | 4 | `abs 0x315048` | `-` |
| `ldr` at 0x138b78 | load | 4 | `abs 0x138e20` | `-` |
| `ldr` at 0x138b7c | load | 4 | `abs 0x138e24` | `-` |
| `ldr` at 0x138b80 | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x138b8c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x138b90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138b9c | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138ba0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138bc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138bd0 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138bd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138bec | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138bf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138c10 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138c14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138c34 | load | 4 | `(unknown)` | `-` |
| `str` at 0x138c38 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x138c3c | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138c40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138c60 | load | 4 | `(unknown)` | `-` |
| `str` at 0x138c64 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x138c68 | load | 4 | `abs 0x138e2c` | `-` |
| `ldr` at 0x138c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138c70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138c74 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138c7c | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138c80 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x138ca0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x138ca4 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138cc8 | load | 4 | `abs 0x138e30` | `-` |
| `ldr` at 0x138cd0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x138cdc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x138ce0 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138ce4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138d04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138d08 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138d14 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138d38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138d40 | load | 4 | `(unknown)` | `-` |
| `ldrh` at 0x138d48 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x138d4c | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138d50 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138d70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138d78 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x138d80 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x138d84 | load | 4 | `abs 0x138e28` | `-` |
| `ldr` at 0x138d88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138da8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138db0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x138db8 | load | 1 | `(unknown)` | `-` |
| `str` at 0x138dbc | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x138dc0 | load | 4 | `abs 0x138e34` | `-` |
| `ldr` at 0x138dc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138dd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138ddc | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x138de4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x138dec | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x138df4 | load | 4 | `abs 0x138e38` | `-` |
| `ldm` at 0x138e00 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x138e00 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x138e00 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x138b5c | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x1e5` | `0x0` |
| `bl` at 0x138b64 | `(unknown callee) 0x243d4` | `0x19120c` | `?` | `?` | `?` |
| `bl` at 0x138b74 | `(unknown callee) 0x243d4` | `0x191224` | `[0x315048]` | `?` | `0x315048` |
| `bl` at 0x138b84 | `(unknown callee) 0x243d4` | `0x191234` | `[0x31504c]` | `?` | `0x31504c` |
| `bl` at 0x138c78 | `(unknown callee) 0x243d4` | `0x191244` | `[sp-20]` | `?` | `?` |
| `bl` at 0x138cd4 | `(unknown callee) 0x243d4` | `0x191280` | `?` | `?` | `?` |
| `bl` at 0x138dd0 | `(unknown callee) 0x243d4` | `0x1912b0` | `[sp-24]` | `?` | `?` |
| `bl` at 0x138df8 | `(unknown callee) 0x243d4` | `0x1912e4` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0x138b40 | `[0x315034] <= 0x2 (unsigned, 32-bit)` | `[0x315034] > 0x2 (unsigned, 32-bit)` |
| `bhi` at 0x138b98 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `bne` at 0x138bc8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x138d10 | `? <= [sp-24] (unsigned, 32-bit)` | `? > [sp-24] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x138e00 | `r0` | `ret(0x243d4)` |

## `JTAG_Cfg_L3SetCfg_f`

- `JTAG_Cfg_L3SetCfg_f`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x138e3c to 0x139234.
  Established. A byte store writes 0 at entry sp minus 0x22 (`strb` at 0x139040).
  A later byte store writes 3 to an unknown effective target (`strb` at 0x1391d0).
  Another byte store at entry sp minus 0x22 writes ([sp-34]:1 + 1) & 0xff (`strb` at 0x1391dc).
  The call to unnamed 0x13df54 has printed r0-r3 candidates 3, unknown, unknown and unknown (`bl` at 0x139184).
  A later word store writes 1 at entry sp minus 0x28 (`str` at 0x139200).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x138e4c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x138e50 | load | 4 | `abs 0x139210` | `-` |
| `ldr` at 0x138e54 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138e60 | load | 4 | `abs 0x139214` | `-` |
| `str` at 0x138e64 | store | 4 | `sp -0x3c` | `0x1912e8` |
| `ldr` at 0x138e6c | load | 4 | `abs 0x139218` | `-` |
| `ldr` at 0x138e70 | load | 4 | `abs 0x13921c` | `-` |
| `ldr` at 0x138e80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138e88 | load | 4 | `(unknown)` | `-` |
| `str` at 0x138e8c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x138e94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138e9c | load | 4 | `(unknown)` | `-` |
| `str` at 0x138ea0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x138ea4 | load | 4 | `abs 0x139210` | `-` |
| `ldr` at 0x138ea8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138eb4 | load | 4 | `abs 0x139220` | `-` |
| `str` at 0x138eb8 | store | 4 | `sp -0x3c` | `0x1912fc` |
| `ldr` at 0x138ebc | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138ec0 | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x138ec4 | store | 4 | `sp -0x38` | `[0x31504c]` |
| `ldr` at 0x138ec8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138ecc | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x138ed0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138ed4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x138edc | load | 4 | `abs 0x139218` | `-` |
| `ldr` at 0x138ee0 | load | 4 | `abs 0x139228` | `-` |
| `ldr` at 0x138eec | load | 4 | `abs 0x139210` | `-` |
| `ldr` at 0x138ef0 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x138efc | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x138f00 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138f04 | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x138f1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138f20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138f2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x138f38 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x138f3c | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138f40 | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x138f60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x138f64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x138f70 | load | 4 | `abs 0x139230` | `-` |
| `str` at 0x138f74 | store | 4 | `sp -0x3c` | `0x19132c` |
| `ldr` at 0x138f78 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138f7c | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x138f80 | store | 4 | `sp -0x38` | `[0x31504c]` |
| `ldr` at 0x138f84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x138f88 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0x138f8c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x138f90 | store | 4 | `sp -0x30` | `[sp-28]` |
| `ldr` at 0x138f94 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x138f98 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138f9c | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x138fbc | load | 4 | `(unknown)` | `-` |
| `str` at 0x138fc0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x138fc8 | load | 4 | `abs 0x139218` | `-` |
| `ldr` at 0x138fd8 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x138fdc | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x138fe0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x138ff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138ff8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x138ffc | load | 4 | `[sp-20] +0x0` | `-` |
| `str` at 0x139004 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0x13900c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x139014 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x139018 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x13901c | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x139034 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x139038 | store | 4 | `(unknown)` | `[sp-24]` |
| `strb` at 0x139040 | store | 1 | `sp -0x22` | `0x0` |
| `ldrb` at 0x139044 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x139048 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x139054 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x139060 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x13906c | load | 4 | `(unknown)` | `-` |
| `str` at 0x139070 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x139074 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x139078 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x13907c | load | 4 | `abs 0x31504c` | `-` |
| `ldrb` at 0x13909c | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x1390a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1390ac | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x1390b4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x1390b8 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x1390bc | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x1390c0 | load | 4 | `abs 0x31504c` | `-` |
| `ldrb` at 0x1390e0 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x1390e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1390f0 | load | 4 | `sp -0x20` | `-` |
| `strb` at 0x1390f4 | store | 1 | `(unknown)` | `([sp-32] & 0xff)` |
| `ldr` at 0x1390f8 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x1390fc | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x139100 | load | 4 | `abs 0x31504c` | `-` |
| `ldrb` at 0x139120 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x139128 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x139130 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x13913c | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x139140 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x139144 | load | 4 | `abs 0x31504c` | `-` |
| `ldrb` at 0x139164 | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x13916c | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x139174 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x139194 | load | 4 | `abs 0x13922c` | `-` |
| `ldr` at 0x139198 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x13919c | load | 4 | `abs 0x31504c` | `-` |
| `ldrb` at 0x1391bc | load | 1 | `sp -0x22` | `-` |
| `ldr` at 0x1391c4 | load | 4 | `(unknown)` | `-` |
| `strb` at 0x1391d0 | store | 1 | `(unknown)` | `0x3` |
| `ldrb` at 0x1391d4 | load | 1 | `sp -0x22` | `-` |
| `strb` at 0x1391dc | store | 1 | `sp -0x22` | `([sp-34]:1+1 & 0xff)` |
| `ldr` at 0x1391e4 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x1391e8 | load | 4 | `abs 0x31504c` | `-` |
| `ldr` at 0x1391f0 | load | 4 | `abs 0x139224` | `-` |
| `ldr` at 0x1391f4 | load | 4 | `abs 0x31504c` | `-` |
| `str` at 0x139200 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x139204 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0x13920c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13920c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13920c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x138e78 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x20a` | `0x0` |
| `bl` at 0x138ee8 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x213` | `0x0` |
| `bl` at 0x138fd4 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x218` | `0x0` |
| `bl` at 0x138fe4 | `(unknown callee) 0x137b78` | `[0x31504c]` | `?` | `?` | `0x31504c` |
| `bl` at 0x139184 | `(unknown callee) 0x13df54` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x1391ec | `(unknown callee) 0x1384bc` | `[0x31504c]` | `?` | `?` | `0x31504c` |
| `bl` at 0x1391f8 | `(unknown callee) 0x13c2dc` | `[0x31504c]` | `?` | `?` | `0x31504c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x138e5c | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x138eb0 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x138ef8 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `beq` at 0x138f28 | `? == [sp-24]` | `? != [sp-24]` |
| `beq` at 0x138f34 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x138f6c | `? == [sp-28]` | `? != [sp-28]` |
| `bne` at 0x138ff0 | `ret(0x137b78) != 0x0` | `ret(0x137b78) == 0x0` |
| `bhs` at 0x139050 | `C == 1` | `not (C == 1)` |
| `bne` at 0x139138 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x13917c | `? != 0x8` | `? == 0x8` |
| `beq` at 0x139190 | `ret(0x13df54) == 0x0` | `ret(0x13df54) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13920c | `r0` | `?` |

## `JTAG_Cfg_L3SelSIC`

- `JTAG_Cfg_L3SelSIC`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139234 to 0x139344.
  Established. A halfword store preserves the low 16 bits of entry r0 at entry sp minus 0x12 (`strh` at 0x13924c).
  A byte store preserves the low eight bits of entry r1 at entry sp minus 0x13 (`strb` at 0x139254).
  A later byte store writes the low eight bits of [sp-19]:1 to an unknown effective target (`strb` at 0x1392d8).
  Another store writes [sp-24] as a word to an unknown effective target (`str` at 0x139304).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139248 | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0x13924c | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strb` at 0x139254 | store | 1 | `sp -0x13` | `(r1 & 0xff)` |
| `ldr` at 0x139258 | load | 4 | `abs 0x139330` | `-` |
| `ldr` at 0x13925c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x139268 | load | 4 | `abs 0x139334` | `-` |
| `str` at 0x13926c | store | 4 | `sp -0x30` | `0x191370` |
| `ldr` at 0x139274 | load | 4 | `abs 0x139338` | `-` |
| `ldr` at 0x139278 | load | 4 | `abs 0x13933c` | `-` |
| `str` at 0x139288 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x13928c | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x139298 | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0x13929c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x1392b0 | load | 4 | `abs 0x139340` | `-` |
| `ldr` at 0x1392b4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x1392d4 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x1392d8 | store | 1 | `(unknown)` | `([sp-19]:1 & 0xff)` |
| `ldr` at 0x1392dc | load | 4 | `abs 0x139340` | `-` |
| `ldr` at 0x1392e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x139300 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x139304 | store | 4 | `(unknown)` | `[sp-24]` |
| `ldr` at 0x139308 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x139310 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x139318 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x13932c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13932c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13932c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x139280 | `(unknown callee) 0x22070` | `0x6` | `0x191010` | `0x257` | `0x0` |
| `bl` at 0x13930c | `(unknown callee) 0x13c2dc` | `[sp-44]` | `0x31e5c4` | `?` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x139264 | `[0x315034] == 0x0` | `[0x315034] != 0x0` |
| `bgt` at 0x139294 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x1392ac | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13932c | `r0` | `0x1` |

## `JTAG_Cfg_Setup`

- `JTAG_Cfg_Setup`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139344 to 0x139398.
  Established. Three word stores write 0x138e3c, 0x138644 and 0x138b24 to absolute addresses 0x1a0870, 0x1a0874 and 0x1a0878 respectively (`str` at 0x139358, `str` at 0x139364, `str` at 0x139370).
  Included implicit word saves (PC omitted) place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0x139348).
  The recorded return value in r0 is 0 (`ldm` at 0x13937c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x139350 | load | 4 | `abs 0x139380` | `-` |
| `ldr` at 0x139354 | load | 4 | `abs 0x139384` | `-` |
| `str` at 0x139358 | store | 4 | `abs 0x1a0870` | `0x138e3c` |
| `ldr` at 0x13935c | load | 4 | `abs 0x139388` | `-` |
| `ldr` at 0x139360 | load | 4 | `abs 0x13938c` | `-` |
| `str` at 0x139364 | store | 4 | `abs 0x1a0874` | `0x138644` |
| `ldr` at 0x139368 | load | 4 | `abs 0x139390` | `-` |
| `ldr` at 0x13936c | load | 4 | `abs 0x139394` | `-` |
| `str` at 0x139370 | store | 4 | `abs 0x1a0878` | `0x138b24` |
| `ldm` at 0x13937c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x13937c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x13937c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x13937c | `r0` | `0x0` |

## `JTAG_Cfg_L3SetCfg_stub`

- `JTAG_Cfg_L3SetCfg_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139d14 to 0x139d34.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x139d24).
  The final ldm reads words at entry sp minus 0x14, 0x10, 0xc and 8 (`ldm` at 0x139d30).
  The return record places 1 in r0 (`ldm` at 0x139d30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139d24 | store | 4 | `sp -0x14` | `r0` |
| `ldm` at 0x139d30 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x139d30 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139d30 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139d30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139d30 | `r0` | `0x1` |

## `JTAG_Cfg_set_ports_stub`

- `JTAG_Cfg_set_ports_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139d34 to 0x139d5c.
  Established. One word store writes entry r0 at entry sp minus 0x14 (`str` at 0x139d44).
  A separate word store writes entry r1 at entry sp minus 0x18 (`str` at 0x139d48).
  The return record places 0 in r0 (`ldm` at 0x139d58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x139d44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x139d48 | store | 4 | `sp -0x18` | `r1` |
| `ldm` at 0x139d58 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139d58 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139d58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139d58 | `r0` | `0x0` |

## `JTAG_Cfg_display_stub`

- `JTAG_Cfg_display_stub`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x139d5c to 0x139d6c.
  Established. Included implicit word saves (PC omitted) place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0x139d60).
  The ldm reads words at those three stack locations (`ldm` at 0x139d68).
  The return record preserves entry r0 (`ldm` at 0x139d68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x139d68 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x139d68 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x139d68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x139d68 | `r0` | `r0` |
