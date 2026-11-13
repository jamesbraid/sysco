# ARM Linux 2.6 TIO and PCI-X setup routines in 1.44.0

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
register locations r0 through r3 with unknown arity. Custom calling conventions remain unknown.
Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include the
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-tio-pcix-setup-facts.txt) retain all tool sections.

## `TIOResetRelease`

- `TIOResetRelease`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x73c5c to 0x73ff8.
  Established. The result from unnamed 0x7af1c is stored as a word at entry sp minus 0x1c (`bl` at 0x73d14, `str` at 0x73d1c).
  A later call to unnamed 0x7b064 has its result stored as a word at that same destination (`bl` at 0x73d40, `str` at 0x73d48).
  A word store writes 1 to the address formed by [sp-36] plus 0xc (`str` at 0x73f40).
  Another word store writes 0xfffff768 at entry sp minus 0x34 (`str` at 0x73cc4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x73c6c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x73c70 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x73c78 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x73c80 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x73c84 | load | 4 | `abs 0x73fec` | `-` |
| `ldr` at 0x73c88 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x73c94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73ca8 | load | 4 | `abs 0x73ff0` | `-` |
| `ldr` at 0x73cac | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x73cb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x73cc0 | load | 4 | `abs 0x73ff4` | `-` |
| `str` at 0x73cc4 | store | 4 | `sp -0x34` | `0xfffff768` |
| `ldr` at 0x73ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x73ce0 | load | 4 | `abs 0x73ff0` | `-` |
| `ldr` at 0x73ce4 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x73cec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x73cf0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73d08 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x73d10 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73d1c | store | 4 | `sp -0x1c` | `ret(0x7af1c)` |
| `ldr` at 0x73d20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73d2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73d30 | store | 4 | `sp -0x34` | `ret(0x7af1c)` |
| `ldr` at 0x73d38 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73d48 | store | 4 | `sp -0x1c` | `ret(0x7b064)` |
| `ldr` at 0x73d4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73d58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73d5c | store | 4 | `sp -0x34` | `ret(0x7b064)` |
| `str` at 0x73d68 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x73d6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73d70 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x73d74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73d84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73d8c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x73d94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73da4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73da8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73dbc | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x73dc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73dc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73dd0 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x73de4 | store | 4 | `sp -0x1c` | `ret(0x75440)` |
| `ldr` at 0x73dec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73df8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73dfc | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x73e04 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73e0c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x73e14 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73e18 | load | 4 | `[sp-36] +0x10` | `-` |
| `str` at 0x73e30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x73e34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73e38 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x73e3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73e4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e54 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x73e5c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73e70 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e84 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x73e8c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73e90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73e98 | load | 4 | `[sp-36] +0x4` | `-` |
| `str` at 0x73eac | store | 4 | `sp -0x1c` | `ret(0x7655c)` |
| `ldr` at 0x73eb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73ec0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73ec4 | store | 4 | `sp -0x34` | `[sp-28]` |
| `ldr` at 0x73ecc | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73ed4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x73edc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73eec | store | 4 | `sp -0x1c` | `ret(0x7b064)` |
| `ldr` at 0x73ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73efc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73f00 | store | 4 | `sp -0x34` | `ret(0x7b064)` |
| `ldr` at 0x73f08 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x73f1c | store | 4 | `sp -0x1c` | `ret(0x7a808)` |
| `ldr` at 0x73f20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x73f2c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x73f30 | store | 4 | `sp -0x34` | `ret(0x7a808)` |
| `ldr` at 0x73f38 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x73f40 | store | 4 | `[sp-36] +0xc` | `0x1` |
| `str` at 0x73f48 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x73f4c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73f50 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x73f54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f60 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73f64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f6c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x73f74 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73f80 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73f84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73f8c | load | 4 | `[sp-36] +0x4` | `-` |
| `ldrb` at 0x73f94 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73f98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x73f9c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x73fb0 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldrb` at 0x73fb8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x73fc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x73fd0 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x73fdc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x73fe0 | load | 4 | `sp -0x34` | `-` |
| `ldm` at 0x73fe8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x73fe8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x73fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x73cf4 | `(unknown callee) 0x558d8` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x73d14 | `(unknown callee) 0x7af1c` | `[sp-20]` | `?` | `?` | `ret(0x558d8)` |
| `bl` at 0x73d40 | `(unknown callee) 0x7b064` | `[sp-20]` | `0x0` | `?` | `ret(0x7af1c)` |
| `bl` at 0x73da0 | `(unknown callee) 0x134634` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x73ddc | `(unknown callee) 0x75440` | `?` | `?` | `?` | `?` |
| `bl` at 0x73de8 | `(unknown callee) 0x1346fc` | `ret(0x75440)` | `?` | `?` | `ret(0x75440)` |
| `bl` at 0x73e28 | `(unknown callee) 0x73ff8` | `0x64` | `?` | `[[sp-36]+8]` | `[[sp-36]+16]` |
| `bl` at 0x73e68 | `(unknown callee) 0x134634` | `?` | `[sp-36]` | `?` | `?` |
| `bl` at 0x73ea4 | `(unknown callee) 0x7655c` | `?` | `?` | `?` | `?` |
| `bl` at 0x73eb0 | `(unknown callee) 0x1346fc` | `ret(0x7655c)` | `?` | `?` | `ret(0x7655c)` |
| `bl` at 0x73ee4 | `(unknown callee) 0x7b064` | `[sp-20]` | `0x1` | `[[sp-36]+8]` | `?` |
| `bl` at 0x73f14 | `(unknown callee) 0x7a808` | `[sp-20]` | `0xff` | `0xff` | `ret(0x7b064)` |
| `bl` at 0x73fc4 | `(unknown callee) 0xb8924` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x73c90 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x73cbc | `? != 0x0` | `? == 0x0` |
| `bne` at 0x73d00 | `ret(0x558d8) != 0x0` | `ret(0x558d8) == 0x0` |
| `beq` at 0x73d28 | `ret(0x7af1c) == 0x0` | `ret(0x7af1c) != 0x0` |
| `beq` at 0x73d54 | `ret(0x7b064) == 0x0` | `ret(0x7b064) != 0x0` |
| `ble` at 0x73d7c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x73d9c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x73df4 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x73e20 | `[[sp-36]+16] == 0x0` | `[[sp-36]+16] != 0x0` |
| `ble` at 0x73e44 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x73e64 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x73ebc | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `beq` at 0x73ef8 | `ret(0x7b064) == 0x0` | `ret(0x7b064) != 0x0` |
| `beq` at 0x73f28 | `ret(0x7a808) == 0x0` | `ret(0x7a808) != 0x0` |
| `ble` at 0x73f5c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x73f7c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x73fe8 | `r0` | `?` |

## `TIOActive`

- `TIOActive`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74020 to 0x740c4.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x74030).
  A word store writes 0xfffff768 at entry sp minus 0x1c (`str` at 0x74074).
  A word loaded from [sp-24] plus 0xc is stored at entry sp minus 0x1c (`ldr` at 0x740a4, `str` at 0x740a8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74030 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x74034 | load | 4 | `abs 0x740b8` | `-` |
| `ldr` at 0x74038 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x74044 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74058 | load | 4 | `abs 0x740bc` | `-` |
| `ldr` at 0x7405c | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x74064 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74070 | load | 4 | `abs 0x740c0` | `-` |
| `str` at 0x74074 | store | 4 | `sp -0x1c` | `0xfffff768` |
| `ldr` at 0x7407c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74090 | load | 4 | `abs 0x740bc` | `-` |
| `ldr` at 0x74094 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x7409c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x740a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x740a4 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x740a8 | store | 4 | `sp -0x1c` | `[[sp-24]+12]` |
| `ldr` at 0x740ac | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x740b4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x740b4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x740b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74040 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x7406c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x740b4 | `r0` | `?` |

## `TIO_Target`

- `TIO_Target`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x740c4 to 0x74230.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x740d4, `str` at 0x740d8).
  Other word stores write 0xfffff768 and 0xfffff766 at entry sp minus 0x20 (`str` at 0x7411c, `str` at 0x74160).
  The computed word load at 0x74204 has an unknown effective address (`ldr` at 0x74204).
  The return record retains an unknown value in r0 (`ldm` at 0x7421c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x740d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x740d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x740dc | load | 4 | `abs 0x74220` | `-` |
| `ldr` at 0x740e0 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x740ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74100 | load | 4 | `abs 0x74224` | `-` |
| `ldr` at 0x74104 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x7410c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74118 | load | 4 | `abs 0x74228` | `-` |
| `str` at 0x7411c | store | 4 | `sp -0x20` | `0xfffff768` |
| `ldr` at 0x74124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74138 | load | 4 | `abs 0x74224` | `-` |
| `ldr` at 0x7413c | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x74144 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x74148 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7414c | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x74150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7415c | load | 4 | `abs 0x7422c` | `-` |
| `str` at 0x74160 | store | 4 | `sp -0x20` | `0xfffff766` |
| `ldr` at 0x74180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7418c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74190 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x741a4 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x741ac | load | 4 | `(unknown)` | `-` |
| `str` at 0x741b4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x741b8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x741c0 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x741c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x741d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x741dc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x741e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x741e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x741fc | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0x74204 | load | 4 | `(unknown)` | `-` |
| `str` at 0x74208 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x7420c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x74210 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x74214 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7421c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7421c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7421c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74168 | `(unknown callee) 0x6e140` | `r0` | `r1` | `[[sp-28]+8]` | `r1` |
| `bl` at 0x74178 | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x740e8 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x74114 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x74158 | `[[sp-28]+8] > r1 (signed, 32-bit)` | `[[sp-28]+8] <= r1 (signed, 32-bit)` |
| `ble` at 0x74174 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0x74188 | `ret(0x6e190) == [sp-20]` | `ret(0x6e190) != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7421c | `r0` | `?` |

## `TIO_SetLinkEnable`

- `TIO_SetLinkEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74230 to 0x743d4.
  Established. The result from unnamed 0x6e190 is stored as a word at entry sp minus 0x34 (`bl` at 0x74270, `str` at 0x74274).
  A word store writes 0x90060780 at entry sp minus 0x24 (`str` at 0x74388).
  The call to unnamed 0x13d2b8 has printed r0-r3 candidates [sp-20], [sp-36], 0 and 1 (`bl` at 0x743a0).
  An earlier word store writes 0xfffff768 at entry sp minus 0x30 (`str` at 0x74318).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74240 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74244 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x74248 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x74274 | store | 4 | `sp -0x34` | `ret(0x6e190)` |
| `ldr` at 0x7427c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74284 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x74288 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x74294 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x74298 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x7429c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x742a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x742b8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x742c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x742d0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x742d8 | load | 4 | `abs 0x743bc` | `-` |
| `ldr` at 0x742dc | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x742e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x742fc | load | 4 | `abs 0x743c0` | `-` |
| `ldr` at 0x74300 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x74308 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74314 | load | 4 | `abs 0x743c4` | `-` |
| `str` at 0x74318 | store | 4 | `sp -0x30` | `0xfffff768` |
| `ldr` at 0x74320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x74334 | load | 4 | `abs 0x743c0` | `-` |
| `ldr` at 0x74338 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x74340 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x74344 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x74348 | load | 4 | `[sp-44] +0xc` | `-` |
| `str` at 0x74358 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x74360 | load | 4 | `abs 0x743c8` | `-` |
| `ldr` at 0x74364 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x74370 | load | 4 | `abs 0x743cc` | `-` |
| `ldr` at 0x74374 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7437c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x74384 | load | 4 | `abs 0x743d0` | `-` |
| `str` at 0x74388 | store | 4 | `sp -0x24` | `0x90060780` |
| `ldr` at 0x74390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74394 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x743ac | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x743b0 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x743b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x743b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x743b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x7424c | `(unknown callee) 0x6e140` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x7425c | `(unknown callee) 0x3e38c` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0x74270 | `(unknown callee) 0x6e190` | `?` | `?` | `?` | `?` |
| `bl` at 0x74380 | `(unknown callee) 0x243d4` | `0x165504` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x7438c | `(unknown callee) 0x134634` | `?` | `?` | `?` | `0x90060780` |
| `bl` at 0x743a0 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `[sp-36]` | `0x0` | `0x1` |
| `bl` at 0x743a4 | `(unknown callee) 0x1346fc` | `ret(0x13d2b8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74258 | `ret(0x6e140) == 0x1` | `ret(0x6e140) != 0x1` |
| `bne` at 0x74268 | `ret(0x3e38c) != 0x0` | `ret(0x3e38c) == 0x0` |
| `beq` at 0x742b0 | `? == 0x4` | `? != 0x4` |
| `bne` at 0x742c8 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0x742e4 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x74310 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x74350 | `[[sp-44]+12] != 0x0` | `[[sp-44]+12] == 0x0` |
| `beq` at 0x7436c | `[0x315034] == 0x0` | `[0x315034] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x743b8 | `r0` | `?` |

## `TIOSysComInt`

- `TIOSysComInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x743d4 to 0x745b8.
  Established. A byte store preserves the low eight bits of entry r1 at entry sp minus 0x15 (`strb` at 0x743f0).
  Another preserves the low eight bits of entry r2 at entry sp minus 0x16 (`strb` at 0x743f8).
  A third preserves the low eight bits of entry r3 at entry sp minus 0x17 (`strb` at 0x74400).
  The result from unnamed 0x6e1b0 is stored as a word at entry sp minus 0x28 (`bl` at 0x74494, `str` at 0x74498).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x743e4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x743f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x743f8 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x74400 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x74408 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x7440c | load | 4 | `abs 0x745a4` | `-` |
| `ldr` at 0x74410 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x7441c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74430 | load | 4 | `abs 0x745a8` | `-` |
| `ldr` at 0x74434 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x7443c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7444c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74460 | load | 4 | `abs 0x745a8` | `-` |
| `ldr` at 0x74464 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x7446c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x74470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74474 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x74478 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74484 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74498 | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x7449c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x744ac | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x744b0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x744d8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x744dc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x744e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x744e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x744f0 | load | 4 | `abs 0x745ac` | `-` |
| `str` at 0x744f4 | store | 4 | `(unknown)` | `0x16553c` |
| `str` at 0x744f8 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x744fc | load | 1 | `sp -0x15` | `-` |
| `str` at 0x74500 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x74508 | load | 4 | `abs 0x745b0` | `-` |
| `ldr` at 0x7450c | load | 4 | `abs 0x745b4` | `-` |
| `ldr` at 0x7451c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74524 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74540 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x7454c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x74550 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x74554 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x74558 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x7455c | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x7456c | load | 1 | `sp -0x17` | `-` |
| `ldrb` at 0x74570 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x74588 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7458c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x74590 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x745a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x745a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x745a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74494 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x744a0 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x744e8 | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x74514 | `(unknown callee) 0x22070` | `0x1` | `0x165450` | `0x287` | `0x0` |
| `bl` at 0x74598 | `(unknown callee) 0xb8420` | `r0` | `?` | `(r3 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74418 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x74444 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x74480 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x74564 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x74584 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x745a0 | `r0` | `?` |

## `TIOPciResetInt`

- `TIOPciResetInt`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x745b8 to 0x747b4.
  Established. An initial word store writes 0 at entry sp minus 0x20 (`str` at 0x745ec).
  The result from unnamed 0x6e1b0 is stored as a word at entry sp minus 0x28 (`bl` at 0x74678, `str` at 0x7467c).
  The result from unnamed 0x2031c is increased by 1 and stored as a word at entry sp minus 0x2c (`bl` at 0x74684, `add` at 0x7468c, `str` at 0x74690).
  The result from unnamed 0x203f4 is stored as a word at an unknown effective target (`bl` at 0x746cc, `str` at 0x746dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x745c8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x745d4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x745dc | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0x745e4 | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `str` at 0x745ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x745f0 | load | 4 | `abs 0x747a0` | `-` |
| `ldr` at 0x745f4 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x74600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74614 | load | 4 | `abs 0x747a4` | `-` |
| `ldr` at 0x74618 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x74620 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74644 | load | 4 | `abs 0x747a4` | `-` |
| `ldr` at 0x74648 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x74650 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x74654 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74658 | load | 4 | `[sp-36] +0x8` | `-` |
| `ldr` at 0x7465c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74668 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7467c | store | 4 | `sp -0x28` | `ret(0x6e1b0)` |
| `ldr` at 0x74680 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x74690 | store | 4 | `sp -0x2c` | `ret(0x2031c)+1` |
| `ldr` at 0x74694 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x746bc | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x746c0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x746c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x746c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x746d4 | load | 4 | `abs 0x747a8` | `-` |
| `str` at 0x746d8 | store | 4 | `(unknown)` | `0x165560` |
| `str` at 0x746dc | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldrb` at 0x746e0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x746e4 | store | 4 | `(unknown)` | `[sp-21]:1` |
| `ldr` at 0x746ec | load | 4 | `abs 0x747ac` | `-` |
| `ldr` at 0x746f0 | load | 4 | `abs 0x747b0` | `-` |
| `ldr` at 0x74700 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x74708 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x74724 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x74730 | store | 4 | `sp -0x20` | `?` |
| `str` at 0x74734 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x74738 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7473c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x74740 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x74750 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x74760 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x74764 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x74768 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x74774 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x74784 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x74788 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x7478c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x7479c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7479c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7479c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74678 | `(unknown callee) 0x6e1b0` | `r0` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x74684 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x746cc | `(unknown callee) 0x203f4` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `?` |
| `bl` at 0x746f8 | `(unknown callee) 0x22070` | `0x1` | `0x165450` | `0x2b1` | `0x0` |
| `bl` at 0x74770 | `(unknown callee) 0x77678` | `r0` | `?` | `?` | `?` |
| `bl` at 0x74794 | `(unknown callee) 0x77678` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x745fc | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x74628 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0x74664 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x74748 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x7475c | `? == 0x0` | `? != 0x0` |
| `beq` at 0x74780 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7479c | `r0` | `?` |

## `TIOPresent`

- `TIOPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x747b4 to 0x74860.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x747c4).
  A byte store preserves the low eight bits of entry r1 at entry sp minus 0x15 (`strb` at 0x747cc).
  A later computed byte store writes 1 to an unknown effective target (`strb` at 0x7484c).
  The return record preserves entry r0 (`ldm` at 0x74854).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x747c4 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x747cc | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x747d0 | load | 4 | `abs 0x74858` | `-` |
| `ldr` at 0x747d4 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x747e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x747f4 | load | 4 | `abs 0x7485c` | `-` |
| `ldr` at 0x747f8 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x74800 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74810 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74824 | load | 4 | `abs 0x7485c` | `-` |
| `ldr` at 0x74828 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x74830 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x74834 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x74838 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x74840 | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x7484c | store | 1 | `(unknown)` | `0x1` |
| `ldm` at 0x74854 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x74854 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x74854 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x747dc | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x74808 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x74854 | `r0` | `r0` |

## `TIONotPresent`

- `TIONotPresent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74860 to 0x7490c.
  Established. A byte store preserves the low eight bits of entry r1 at entry sp minus 0x15 (`strb` at 0x74878).
  A later word load reads the address formed by [sp-28] plus 4 (`ldr` at 0x748ec).
  The computed byte store writes 0 to an unknown effective target (`strb` at 0x748f8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74870 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x74878 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x7487c | load | 4 | `abs 0x74904` | `-` |
| `ldr` at 0x74880 | load | 4 | `abs 0x1a6214` | `-` |
| `ldr` at 0x7488c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x748a0 | load | 4 | `abs 0x74908` | `-` |
| `ldr` at 0x748a4 | load | 4 | `abs 0x1a6210` | `-` |
| `ldr` at 0x748ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x748bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x748d0 | load | 4 | `abs 0x74908` | `-` |
| `ldr` at 0x748d4 | load | 4 | `abs 0x1a6210` | `-` |
| `str` at 0x748dc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x748e0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x748e4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x748ec | load | 4 | `[sp-28] +0x4` | `-` |
| `strb` at 0x748f8 | store | 1 | `(unknown)` | `0x0` |
| `ldm` at 0x74900 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x74900 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x74900 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74888 | `[0x1a6214] == 0x0` | `[0x1a6214] != 0x0` |
| `bne` at 0x748b4 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x74900 | `r0` | `r0` |

## `TIO_Issue_CX_Probe`

- `TIO_Issue_CX_Probe`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x75a8c to 0x7655c.
  Established. A byte store preserves the low eight bits of entry r1 at entry sp minus 0x15 (`strb` at 0x75aa4).
  A halfword store preserves the low 16 bits of entry r2 at entry sp minus 0x18 (`strh` at 0x75aa8).
  The result from one call to unnamed 0x86eac is stored as a word at entry sp minus 0x24 (`bl` at 0x75f6c, `str` at 0x75f74).
  A separate call to that unnamed address has its result stored as a word at entry sp minus 0x20 (`bl` at 0x75fb8, `str` at 0x75fc0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x75a9c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x75aa4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0x75aa8 | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `ldr` at 0x75aac | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75abc | store | 4 | `sp -0x1c` | `?` |
| `str` at 0x75ac4 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x75acc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x75ad0 | load | 4 | `abs 0x76494` | `-` |
| `str` at 0x75ad4 | store | 4 | `sp -0x94` | `0x165754` |
| `ldrb` at 0x75ad8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x75ae4 | load | 4 | `abs 0x76498` | `-` |
| `ldr` at 0x75ae8 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x75af4 | load | 4 | `abs 0x7649c` | `-` |
| `ldr` at 0x75afc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75b10 | store | 4 | `sp -0x28` | `ret(0x13f880)` |
| `ldr` at 0x75b14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75b20 | load | 4 | `abs 0x764a0` | `-` |
| `str` at 0x75b24 | store | 4 | `sp -0x94` | `0x165758` |
| `ldr` at 0x75b2c | load | 4 | `abs 0x76498` | `-` |
| `ldr` at 0x75b30 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x75b3c | load | 4 | `abs 0x764a4` | `-` |
| `ldr` at 0x75b44 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x75b58 | store | 4 | `sp -0x28` | `ret(0x13f880)` |
| `ldr` at 0x75b5c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75b68 | load | 4 | `abs 0x764a8` | `-` |
| `str` at 0x75b6c | store | 4 | `sp -0x94` | `0x165770` |
| `str` at 0x75b78 | store | 4 | `sp -0x38` | `?` |
| `str` at 0x75b80 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x75b84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75b88 | load | 4 | `abs 0x764ac` | `-` |
| `ldr` at 0x75b8c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x75b90 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x75b9c | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75ba0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75bac | load | 4 | `abs 0x764b0` | `-` |
| `str` at 0x75bb0 | store | 4 | `sp -0x94` | `0x165784` |
| `ldr` at 0x75bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75bbc | load | 4 | `abs 0x764b4` | `-` |
| `ldr` at 0x75bc0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x75bc4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x75bd0 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75bd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75be0 | load | 4 | `abs 0x764b8` | `-` |
| `str` at 0x75be4 | store | 4 | `sp -0x94` | `0x1657a4` |
| `ldr` at 0x75bec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75bf0 | load | 4 | `abs 0x764bc` | `-` |
| `ldr` at 0x75bf4 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x75bf8 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x75c04 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75c08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75c14 | load | 4 | `abs 0x764c0` | `-` |
| `str` at 0x75c18 | store | 4 | `sp -0x94` | `0x1657c4` |
| `ldr` at 0x75c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75c24 | load | 4 | `abs 0x764c4` | `-` |
| `ldr` at 0x75c28 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x75c2c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x75c38 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75c3c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75c48 | load | 4 | `abs 0x764c8` | `-` |
| `str` at 0x75c4c | store | 4 | `sp -0x94` | `0x1657e4` |
| `ldr` at 0x75c5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75c60 | load | 4 | `abs 0x764cc` | `-` |
| `str` at 0x75c74 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75c78 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75c84 | load | 4 | `abs 0x764d0` | `-` |
| `str` at 0x75c88 | store | 4 | `sp -0x94` | `0x165804` |
| `ldr` at 0x75c90 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x75c9c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x75ca0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x75cac | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x75cb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75cbc | load | 4 | `abs 0x764d4` | `-` |
| `str` at 0x75cd0 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75cd4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75ce0 | load | 4 | `abs 0x764d8` | `-` |
| `str` at 0x75ce4 | store | 4 | `sp -0x94` | `0x165818` |
| `ldr` at 0x75cf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75cf8 | load | 4 | `abs 0x764dc` | `-` |
| `str` at 0x75d0c | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75d10 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75d1c | load | 4 | `abs 0x764e0` | `-` |
| `str` at 0x75d20 | store | 4 | `sp -0x94` | `0x165834` |
| `ldr` at 0x75d30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75d34 | load | 4 | `abs 0x764e4` | `-` |
| `str` at 0x75d48 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75d4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75d58 | load | 4 | `abs 0x764e8` | `-` |
| `str` at 0x75d5c | store | 4 | `sp -0x94` | `0x165854` |
| `ldr` at 0x75d64 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x75d6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75d70 | load | 4 | `abs 0x764d4` | `-` |
| `ldr` at 0x75d78 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x75d84 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75d88 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75d94 | load | 4 | `abs 0x764ec` | `-` |
| `str` at 0x75d98 | store | 4 | `sp -0x94` | `0x165874` |
| `ldr` at 0x75da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75da4 | load | 4 | `abs 0x764e4` | `-` |
| `str` at 0x75db8 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75dbc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75dc8 | load | 4 | `abs 0x764f0` | `-` |
| `str` at 0x75dcc | store | 4 | `sp -0x94` | `0x165890` |
| `ldr` at 0x75dd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75dd8 | load | 4 | `abs 0x764dc` | `-` |
| `str` at 0x75dec | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75df0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75dfc | load | 4 | `abs 0x764f4` | `-` |
| `str` at 0x75e00 | store | 4 | `sp -0x94` | `0x1658b0` |
| `str` at 0x75e0c | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x75e10 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x75e24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75e28 | load | 4 | `abs 0x764f8` | `-` |
| `str` at 0x75e3c | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75e40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75e4c | load | 4 | `abs 0x764fc` | `-` |
| `ldr` at 0x75e54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75e58 | load | 4 | `abs 0x76500` | `-` |
| `str` at 0x75e6c | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75e70 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75e7c | load | 4 | `abs 0x76504` | `-` |
| `ldr` at 0x75e84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75e88 | load | 4 | `abs 0x76500` | `-` |
| `str` at 0x75e9c | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x75ea0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x75eac | load | 4 | `abs 0x76508` | `-` |
| `str` at 0x75eb8 | store | 4 | `sp -0x8c` | `0x0` |
| `str` at 0x75ec0 | store | 4 | `sp -0x90` | `0x0` |
| `ldr` at 0x75ec4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x75ed0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x75ed4 | load | 4 | `abs 0x7650c` | `-` |
| `ldr` at 0x75ee8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75eec | load | 4 | `abs 0x764f8` | `-` |
| `str` at 0x75f00 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x75f04 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x75f14 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x75f18 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x75f20 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x75f2c | load | 4 | `sp -0x90` | `-` |
| `str` at 0x75f34 | store | 4 | `sp -0x90` | `[sp-144]+100` |
| `str` at 0x75f40 | store | 4 | `sp -0xa8` | `0x0` |
| `str` at 0x75f48 | store | 4 | `sp -0xa4` | `0x30000000` |
| `str` at 0x75f50 | store | 4 | `sp -0xa0` | `sp-56` |
| `str` at 0x75f58 | store | 4 | `sp -0x9c` | `sp-60` |
| `ldr` at 0x75f5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75f60 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x75f64 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x75f74 | store | 4 | `sp -0x24` | `ret(0x86eac)` |
| `ldrh` at 0x75f78 | load | 2 | `sp -0x18` | `-` |
| `ldr` at 0x75f88 | load | 4 | `abs 0x76510` | `-` |
| `str` at 0x75f8c | store | 4 | `sp -0xa8` | `0xc0000040` |
| `ldr` at 0x75f90 | load | 4 | `abs 0x76514` | `-` |
| `str` at 0x75f94 | store | 4 | `sp -0xa4` | `0x30004070` |
| `str` at 0x75f9c | store | 4 | `sp -0xa0` | `sp-64` |
| `str` at 0x75fa4 | store | 4 | `sp -0x9c` | `sp-68` |
| `ldr` at 0x75fa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75fac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x75fb0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x75fc0 | store | 4 | `sp -0x20` | `ret(0x86eac)` |
| `ldr` at 0x75fc4 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x75fc8 | load | 4 | `abs 0x76518` | `-` |
| `ldr` at 0x75fd8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x75fe8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x75ff0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x75ff8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x76000 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x76004 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76010 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x7601c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76020 | load | 4 | `abs 0x764d4` | `-` |
| `ldr` at 0x76024 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x76028 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x76034 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x76038 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76044 | load | 4 | `abs 0x764ec` | `-` |
| `str` at 0x76048 | store | 4 | `sp -0x94` | `0x165874` |
| `ldr` at 0x76050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76054 | load | 4 | `abs 0x764dc` | `-` |
| `ldr` at 0x76058 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x7605c | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x76068 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x7606c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76078 | load | 4 | `abs 0x764f4` | `-` |
| `str` at 0x7607c | store | 4 | `sp -0x94` | `0x1658b0` |
| `ldr` at 0x76084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76088 | load | 4 | `abs 0x764e4` | `-` |
| `ldr` at 0x7608c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x76090 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x7609c | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x760a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x760ac | load | 4 | `abs 0x764f0` | `-` |
| `str` at 0x760b0 | store | 4 | `sp -0x94` | `0x165890` |
| `str` at 0x760bc | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x760c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x760d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x760e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x760f0 | load | 4 | `abs 0x76498` | `-` |
| `ldr` at 0x760f4 | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x76100 | load | 4 | `abs 0x7649c` | `-` |
| `ldr` at 0x76108 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7611c | store | 4 | `sp -0x28` | `ret(0x13f880)` |
| `ldr` at 0x76120 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7612c | load | 4 | `abs 0x764a0` | `-` |
| `str` at 0x76130 | store | 4 | `sp -0x94` | `0x165758` |
| `ldr` at 0x76138 | load | 4 | `abs 0x76498` | `-` |
| `ldr` at 0x7613c | load | 4 | `abs 0x315034` | `-` |
| `ldr` at 0x76148 | load | 4 | `abs 0x764a4` | `-` |
| `ldr` at 0x76150 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76164 | store | 4 | `sp -0x28` | `ret(0x13f880)` |
| `ldr` at 0x76168 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76174 | load | 4 | `abs 0x764a8` | `-` |
| `str` at 0x76178 | store | 4 | `sp -0x94` | `0x165770` |
| `ldr` at 0x76180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76184 | load | 4 | `abs 0x76500` | `-` |
| `str` at 0x76198 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x7619c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x761a8 | load | 4 | `abs 0x7651c` | `-` |
| `str` at 0x761ac | store | 4 | `sp -0x94` | `0x165938` |
| `ldr` at 0x761b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x761b8 | load | 4 | `abs 0x76500` | `-` |
| `str` at 0x761cc | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x761d0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x761dc | load | 4 | `abs 0x76520` | `-` |
| `str` at 0x761e0 | store | 4 | `sp -0x94` | `0x165954` |
| `ldr` at 0x761f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x761f4 | load | 4 | `abs 0x76524` | `-` |
| `str` at 0x76208 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x7620c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76218 | load | 4 | `abs 0x76528` | `-` |
| `str` at 0x7621c | store | 4 | `sp -0x94` | `0x165970` |
| `str` at 0x76228 | store | 4 | `sp -0x84` | `0x0` |
| `ldr` at 0x7622c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x7623c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x76244 | load | 4 | `sp -0x6c` | `-` |
| `ldr` at 0x76254 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x7626c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76280 | store | 4 | `sp -0x28` | `ret(0x13d1a0)` |
| `ldr` at 0x76284 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76290 | load | 4 | `abs 0x7652c` | `-` |
| `str` at 0x76294 | store | 4 | `sp -0x94` | `0x165994` |
| `ldr` at 0x7629c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x762a4 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x762a8 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x762b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x762c0 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0x762c4 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x762d0 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x762d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x762e0 | load | 4 | `abs 0x76530` | `-` |
| `str` at 0x762e4 | store | 4 | `sp -0x94` | `0x1659b4` |
| `ldr` at 0x762ec | load | 4 | `sp -0x84` | `-` |
| `str` at 0x762f4 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x762fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0x76314 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x76318 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76324 | load | 4 | `abs 0x76534` | `-` |
| `str` at 0x76328 | store | 4 | `sp -0x94` | `0x1659d4` |
| `ldr` at 0x76330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76334 | load | 4 | `abs 0x76538` | `-` |
| `str` at 0x76348 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x7634c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76358 | load | 4 | `abs 0x7653c` | `-` |
| `str` at 0x7635c | store | 4 | `sp -0x94` | `0x1659f4` |
| `ldr` at 0x76364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76368 | load | 4 | `abs 0x76540` | `-` |
| `str` at 0x7637c | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x76380 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x7638c | load | 4 | `abs 0x76544` | `-` |
| `str` at 0x76390 | store | 4 | `sp -0x94` | `0x165a14` |
| `ldr` at 0x76398 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x763a0 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0x763a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x763ac | load | 4 | `abs 0x764d4` | `-` |
| `ldr` at 0x763b0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x763b4 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x763c0 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x763c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x763d0 | load | 4 | `abs 0x76548` | `-` |
| `str` at 0x763d4 | store | 4 | `sp -0x94` | `0x165a38` |
| `ldr` at 0x763dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x763e0 | load | 4 | `abs 0x764dc` | `-` |
| `ldr` at 0x763e4 | load | 4 | `sp -0x58` | `-` |
| `ldr` at 0x763e8 | load | 4 | `sp -0x5c` | `-` |
| `str` at 0x763f4 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x763f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76404 | load | 4 | `abs 0x7654c` | `-` |
| `str` at 0x76408 | store | 4 | `sp -0x94` | `0x165a5c` |
| `ldr` at 0x76410 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x76414 | load | 4 | `abs 0x764e4` | `-` |
| `ldr` at 0x76418 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x7641c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x76428 | store | 4 | `sp -0x28` | `ret(0x13d2b8)` |
| `ldr` at 0x7642c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x76438 | load | 4 | `abs 0x76550` | `-` |
| `str` at 0x7643c | store | 4 | `sp -0x94` | `0x165a84` |
| `ldr` at 0x76444 | load | 4 | `abs 0x76554` | `-` |
| `ldr` at 0x76448 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x76450 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x76454 | store | 4 | `sp -0x98` | `[sp-36]` |
| `ldr` at 0x7645c | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x76468 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7646c | load | 4 | `abs 0x76558` | `-` |
| `ldr` at 0x76474 | load | 4 | `sp -0x94` | `-` |
| `ldr` at 0x76480 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x76484 | store | 4 | `sp -0x98` | `[sp-40]` |
| `ldr` at 0x76488 | load | 4 | `sp -0x98` | `-` |
| `ldm` at 0x76490 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x76490 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x76490 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x75af8 | `(unknown callee) 0x243d4` | `0x1656a4` | `r1` | `r2` | `[0x315034]` |
| `bl` at 0x75b08 | `(unknown callee) 0x13f880` | `?` | `0x20` | `0x1` | `?` |
| `bl` at 0x75b40 | `(unknown callee) 0x243d4` | `0x1656c4` | `?` | `?` | `[0x315034]` |
| `bl` at 0x75b50 | `(unknown callee) 0x13f880` | `[sp-20]` | `0x20` | `0x2` | `?` |
| `bl` at 0x75b94 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb00088a0` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x75bc8 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb00088a8` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x75bfc | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb00088b0` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x75c30 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb00088b8` | `[sp-56]` | `[sp-60]` |
| `bl` at 0x75c6c | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0x90060e80` | `sp-56` | `sp-60` |
| `bl` at 0x75cc8 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb0000028` | `sp-72` | `sp-76` |
| `bl` at 0x75d04 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb0000100` | `sp-88` | `sp-92` |
| `bl` at 0x75d40 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb0000108` | `sp-80` | `sp-84` |
| `bl` at 0x75d7c | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000028` | `?` | `[sp-76]` |
| `bl` at 0x75db0 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000108` | `0x0` | `0x0` |
| `bl` at 0x75de4 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000100` | `0x0` | `0xc8` |
| `bl` at 0x75e34 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb000be18` | `sp-120` | `sp-124` |
| `bl` at 0x75e50 | `(unknown callee) 0x29908` | `0x1658d0` | `?` | `?` | `ret(0x13d1a0)` |
| `bl` at 0x75e64 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb000b100` | `0x0` | `0x8` |
| `bl` at 0x75e80 | `(unknown callee) 0x29908` | `0x1658f0` | `?` | `?` | `ret(0x13d2b8)` |
| `bl` at 0x75e94 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb000b100` | `0x0` | `0x0` |
| `bl` at 0x75eb0 | `(unknown callee) 0x29908` | `0x165914` | `?` | `?` | `ret(0x13d2b8)` |
| `bl` at 0x75ef8 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb000be18` | `sp-120` | `sp-124` |
| `bl` at 0x75f28 | `(unknown callee) 0x73ff8` | `0xa` | `?` | `?` | `?` |
| `bl` at 0x75f6c | `(unknown callee) 0x86eac` | `[sp-20]` | `[sp-44]` | `[sp-48]` | `0xff` |
| `bl` at 0x75fb8 | `(unknown callee) 0x86eac` | `[sp-20]` | `[sp-44]` | `[sp-48]` | `0xff` |
| `bl` at 0x7602c | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000028` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x76060 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000100` | `[sp-88]` | `[sp-92]` |
| `bl` at 0x76094 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000108` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x760c8 | `(unknown callee) 0x3deec` | `[sp-28]` | `?` | `?` | `?` |
| `bl` at 0x760ec | `(unknown callee) 0x4747c` | `[sp-28]` | `?` | `0xa` | `?` |
| `bl` at 0x76104 | `(unknown callee) 0x243d4` | `0x1656a4` | `?` | `?` | `[0x315034]` |
| `bl` at 0x76114 | `(unknown callee) 0x13f880` | `[sp-20]` | `0x20` | `0x1` | `?` |
| `bl` at 0x7614c | `(unknown callee) 0x243d4` | `0x1656c4` | `?` | `?` | `[0x315034]` |
| `bl` at 0x7615c | `(unknown callee) 0x13f880` | `[sp-20]` | `0x20` | `0x2` | `?` |
| `bl` at 0x76190 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb000b100` | `0x0` | `0x8` |
| `bl` at 0x761c4 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb000b100` | `0x0` | `0x0` |
| `bl` at 0x76200 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `0xb0008118` | `sp-104` | `sp-108` |
| `bl` at 0x76278 | `(unknown callee) 0x13d1a0` | `[sp-20]` | `?` | `sp-112` | `sp-116` |
| `bl` at 0x762c8 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `?` | `[sp-112]` | `[sp-116]` |
| `bl` at 0x7630c | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000000` | `0x0` | `0x0` |
| `bl` at 0x76340 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0008000` | `0x0` | `0x0` |
| `bl` at 0x76374 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0008060` | `0x0` | `0x0` |
| `bl` at 0x763b8 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000028` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x763ec | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000100` | `[sp-88]` | `[sp-92]` |
| `bl` at 0x76420 | `(unknown callee) 0x13d2b8` | `[sp-20]` | `0xb0000108` | `[sp-80]` | `[sp-84]` |
| `bl` at 0x7644c | `(unknown callee) 0x243d4` | `0x165aac` | `[sp-36]` | `?` | `ret(0x13d2b8)` |
| `bl` at 0x76460 | `(unknown callee) 0x219a4` | `?` | `?` | `?` | `?` |
| `bl` at 0x7647c | `(unknown callee) 0x29908` | `0x165ac8` | `[sp-21]:1` | `[sp-148]` | `ret(0x219a4)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x75ae0 | `(r1 & 0xff) != 0x1` | `(r1 & 0xff) == 0x1` |
| `bls` at 0x75af0 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x75b1c | `ret(0x13f880) >= 0x0 (signed, 32-bit)` | `ret(0x13f880) < 0x0 (signed, 32-bit)` |
| `bls` at 0x75b38 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x75b64 | `ret(0x13f880) == 0x0` | `ret(0x13f880) != 0x0` |
| `beq` at 0x75ba8 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75bdc | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75c10 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75c44 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75c80 | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75cdc | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75d18 | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75d54 | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75d90 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75dc4 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75df8 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `bhi` at 0x75e18 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x75e48 | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75e78 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75ea8 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x75ecc | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0x75edc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0x75f0c | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x75f84 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x75fd4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x75fe0 | `ret(0x86eac) != 0x0` | `ret(0x86eac) == 0x0` |
| `bne` at 0x7600c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x76018 | `[sp-52] != 0x0` | `[sp-52] == 0x0` |
| `beq` at 0x76040 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76074 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x760a8 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `bne` at 0x760d4 | `ret(0x3deec) != 0x700` | `ret(0x3deec) == 0x700` |
| `bls` at 0x760fc | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `bge` at 0x76128 | `ret(0x13f880) >= 0x0 (signed, 32-bit)` | `ret(0x13f880) < 0x0 (signed, 32-bit)` |
| `bls` at 0x76144 | `[0x315034] <= 0x1 (unsigned, 32-bit)` | `[0x315034] > 0x1 (unsigned, 32-bit)` |
| `beq` at 0x76170 | `ret(0x13f880) == 0x0` | `ret(0x13f880) != 0x0` |
| `beq` at 0x761a4 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x761d8 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76214 | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `bhi` at 0x76234 | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |
| `beq` at 0x76250 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x7628c | `ret(0x13d1a0) == 0x0` | `ret(0x13d1a0) != 0x0` |
| `beq` at 0x762dc | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76320 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76354 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76388 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x763cc | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76400 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |
| `beq` at 0x76434 | `ret(0x13d2b8) == 0x0` | `ret(0x13d2b8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x76490 | `r0` | `?` |

## `pcixInit`

- `pcixInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78b38 to 0x78fc4.
  Established. The result from unnamed 0x6e140 is stored as a word at absolute address 0x1a6220 (`bl` at 0x78bc0, `str` at 0x78bcc).
  The result from a call to unnamed 0x204d8 is stored as a word at absolute address 0x1a621c (`bl` at 0x78bf8, `str` at 0x78c00).
  A separate call to unnamed 0x204d8 has its result stored as a word at the address formed by [sp-48] plus 0x14 (`bl` at 0x78cb4, `str` at 0x78cbc).
  The result from a later call through an unknown target is stored as a word at entry sp minus 0x28 (`mov` at 0x78ed0, `str` at 0x78ed8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78b48 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x78b4c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x78b50 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x78b54 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x78b58 | load | 4 | `abs 0x78fa4` | `-` |
| `ldr` at 0x78b5c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x78b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78b7c | load | 4 | `abs 0x78fa8` | `-` |
| `str` at 0x78b80 | store | 4 | `sp -0x50` | `0x166014` |
| `ldr` at 0x78b84 | load | 4 | `abs 0x78fa4` | `-` |
| `ldr` at 0x78b88 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x78b8c | store | 4 | `sp -0x4c` | `[0x1a621c]` |
| `ldr` at 0x78b90 | load | 4 | `abs 0x78fac` | `-` |
| `ldr` at 0x78b94 | load | 4 | `abs 0x1a6220` | `-` |
| `str` at 0x78b98 | store | 4 | `sp -0x48` | `[0x1a6220]` |
| `ldr` at 0x78ba0 | load | 4 | `abs 0x78fb0` | `-` |
| `ldr` at 0x78bb0 | load | 4 | `abs 0x78fac` | `-` |
| `ldr` at 0x78bb4 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x78bc8 | load | 4 | `abs 0x78fac` | `-` |
| `str` at 0x78bcc | store | 4 | `abs 0x1a6220` | `ret(0x6e140)` |
| `ldr` at 0x78bd0 | load | 4 | `abs 0x78fac` | `-` |
| `ldr` at 0x78bd4 | load | 4 | `abs 0x1a6220` | `-` |
| `str` at 0x78be8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0x78bec | load | 4 | `abs 0x78fa4` | `-` |
| `ldr` at 0x78bf0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78c00 | store | 4 | `abs 0x1a621c` | `ret(0x204d8)` |
| `ldr` at 0x78c04 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x78c10 | load | 4 | `abs 0x78fb4` | `-` |
| `str` at 0x78c14 | store | 4 | `sp -0x50` | `0x166058` |
| `ldr` at 0x78c18 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78c1c | store | 4 | `sp -0x4c` | `[sp-52]` |
| `ldr` at 0x78c24 | load | 4 | `abs 0x78fb0` | `-` |
| `ldr` at 0x78c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78c4c | load | 4 | `abs 0x78fa4` | `-` |
| `ldr` at 0x78c50 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x78c58 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x78c5c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x78c64 | store | 4 | `[sp-48] +0x0` | `[sp-28]` |
| `ldr` at 0x78c68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x78c70 | store | 4 | `[sp-48] +0x4` | `[sp-32]` |
| `ldr` at 0x78c74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c78 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x78c7c | store | 4 | `[sp-48] +0x8` | `[sp-36]` |
| `ldr` at 0x78c80 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c84 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x78c88 | store | 4 | `[sp-48] +0xc` | `[sp]` |
| `ldr` at 0x78c8c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c90 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x78c94 | store | 4 | `[sp-48] +0x10` | `[sp+4]` |
| `ldr` at 0x78c98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78c9c | load | 4 | `sp +0x8` | `-` |
| `str` at 0x78ca0 | store | 4 | `[sp-48] +0x18` | `[sp+8]` |
| `ldr` at 0x78ca4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78ca8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78cac | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x78cbc | store | 4 | `[sp-48] +0x14` | `ret(0x204d8)` |
| `ldr` at 0x78cc0 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x78ccc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x78ce0 | store | 4 | `sp -0x34` | `ret(0x6e1b0)` |
| `ldr` at 0x78ce4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78cf4 | store | 4 | `sp -0x38` | `ret(0x2031c)+1` |
| `ldr` at 0x78cf8 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x78d20 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x78d24 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x78d28 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78d2c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x78d38 | load | 4 | `abs 0x78fb8` | `-` |
| `str` at 0x78d3c | store | 4 | `(unknown)` | `0x166090` |
| `str` at 0x78d40 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x78d44 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78d48 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x78d4c | store | 4 | `(unknown)` | `[[sp-48]+8]` |
| `ldr` at 0x78d54 | load | 4 | `abs 0x78fb0` | `-` |
| `ldr` at 0x78d64 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78d68 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78d6c | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x78d74 | load | 4 | `[sp-48] +0x8` | `-` |
| `str` at 0x78d80 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x78d84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78d88 | load | 4 | `[sp-48] +0x8` | `-` |
| `ldr` at 0x78d8c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78d98 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78d9c | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x78db0 | load | 4 | `abs 0x78fbc` | `-` |
| `str` at 0x78db8 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x78dbc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78dc0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78dc8 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x78dd0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78dd4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78de8 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x78df0 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x78df4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x78df8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78dfc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78e04 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x78e0c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78e10 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78e24 | load | 4 | `[sp-48] +0x4` | `-` |
| `ldrb` at 0x78e2c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x78e30 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x78e34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78e38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78e40 | load | 4 | `[sp-48] +0x14` | `-` |
| `strb` at 0x78e4c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x78e50 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x78e54 | load | 4 | `[sp-60] +0x0` | `-` |
| `ldr` at 0x78e58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x78e70 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x78e74 | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x78e80 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x78e84 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78e88 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78e9c | load | 4 | `[sp-48] +0x4` | `-` |
| `ldr` at 0x78ea4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x78ea8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x78eb0 | load | 4 | `[sp-48] +0x14` | `-` |
| `ldr` at 0x78ebc | load | 4 | `[sp-60] +0x8` | `-` |
| `ldr` at 0x78ec0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x78ed8 | store | 4 | `sp -0x28` | `ret(unknown)` |
| `ldr` at 0x78edc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x78ee8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x78efc | store | 4 | `sp -0x38` | `ret(0x6e1b0)` |
| `ldr` at 0x78f00 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x78f10 | store | 4 | `sp -0x34` | `ret(0x2031c)+1` |
| `ldr` at 0x78f14 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x78f3c | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x78f40 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0x78f44 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x78f48 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x78f54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x78f60 | load | 4 | `abs 0x78fc0` | `-` |
| `str` at 0x78f64 | store | 4 | `(unknown)` | `0x1660cc` |
| `str` at 0x78f68 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0x78f6c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x78f70 | store | 4 | `(unknown)` | `[sp-44]` |
| `str` at 0x78f74 | store | 4 | `(unknown)` | `ret(0x21ce0)` |
| `ldr` at 0x78f7c | load | 4 | `abs 0x78fb0` | `-` |
| `ldr` at 0x78f8c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x78f94 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldm` at 0x78fa0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x78fa0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x78fa0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x78fa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x78b68 | `(unknown callee) 0x6e190` | `r0` | `r1` | `r2` | `[0x1a621c]` |
| `bl` at 0x78bac | `(unknown callee) 0x22070` | `0x9` | `0x166044` | `0xaa` | `0x0` |
| `bl` at 0x78bc0 | `(unknown callee) 0x6e140` | `?` | `?` | `?` | `[0x1a6220]` |
| `bl` at 0x78bf8 | `(unknown callee) 0x204d8` | `[sp-52]` | `0x1` | `[0x1a6220]` | `?` |
| `bl` at 0x78c30 | `(unknown callee) 0x22070` | `0x9` | `0x166044` | `0xb5` | `0x0` |
| `bl` at 0x78c34 | `(unknown callee) 0xe0fc8` | `?` | `?` | `?` | `?` |
| `bl` at 0x78cb4 | `(unknown callee) 0x204d8` | `[[sp-48]+8]` | `0x8` | `[sp-48]` | `[sp-48]` |
| `bl` at 0x78cdc | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x78ce8 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x78d30 | `(unknown callee) 0x203f4` | `[sp-60]` | `[sp-52]` | `[sp-56]` | `?` |
| `bl` at 0x78d60 | `(unknown callee) 0x22070` | `0x9` | `0x166044` | `0xcc` | `0x0` |
| `bl` at 0x78d78 | `(unknown callee) 0x20550` | `[[sp-48]+20]` | `0x1` | `[[sp-48]+8]` | `[sp-48]` |
| `mov` at 0x78e60 | `(unknown callee) (unknown target)` | `[sp-24]` | `[sp-48]` | `?` | `[[sp-60]]` |
| `mov` at 0x78ed0 | `(unknown callee) (unknown target)` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x78ef8 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0x78f04 | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0x78f4c | `(unknown callee) 0x203f4` | `[sp-64]` | `[sp-56]` | `[sp-52]` | `?` |
| `bl` at 0x78f58 | `(unknown callee) 0x21ce0` | `[sp-40]` | `?` | `?` | `?` |
| `bl` at 0x78f88 | `(unknown callee) 0x22070` | `0x1` | `0x166044` | `0xd9` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x78b64 | `[0x1a621c] == 0x0` | `[0x1a621c] != 0x0` |
| `bne` at 0x78b78 | `ret(0x6e190) != [sp-24]` | `ret(0x6e190) == [sp-24]` |
| `bne` at 0x78bbc | `[0x1a6220] != 0x0` | `[0x1a6220] == 0x0` |
| `bne` at 0x78c0c | `[0x1a621c] != 0x0` | `[0x1a621c] == 0x0` |
| `bne` at 0x78cc8 | `[[sp-48]+20] != 0x0` | `[[sp-48]+20] == 0x0` |
| `ble` at 0x78d94 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x78e6c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `beq` at 0x78e7c | `[[sp-60]+8] == 0x0` | `[[sp-60]+8] != 0x0` |
| `beq` at 0x78ee4 | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x78fa0 | `r0` | `?` |

## `pcixRemove`

- `pcixRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x78fc4 to 0x790b4.
  Established. A word store writes 0 to the address held in [sp-24] (`str` at 0x79048).
  Four word stores write 0 to [sp-24] plus 0x10, 0xc, 8 and 4 respectively (`str` at 0x79054, `str` at 0x79060, `str` at 0x7906c, `str` at 0x79078).
  A word loaded from [sp-24] plus 0x14 supplies the printed r0 candidate for a call to unnamed 0x20514 (`ldr` at 0x79090, `bl` at 0x79094).
  A subsequent word store writes 0 to the address formed by [sp-24] plus 0x14 (`str` at 0x790a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x78fd4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x78fd8 | load | 4 | `abs 0x790ac` | `-` |
| `ldr` at 0x78fdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x78fe0 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x78fec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79000 | load | 4 | `abs 0x790b0` | `-` |
| `ldr` at 0x79004 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x7900c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x7901c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79030 | load | 4 | `abs 0x790b0` | `-` |
| `ldr` at 0x79034 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x7903c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x79040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x79048 | store | 4 | `[sp-24] +0x0` | `0x0` |
| `ldr` at 0x7904c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x79054 | store | 4 | `[sp-24] +0x10` | `0x0` |
| `ldr` at 0x79058 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x79060 | store | 4 | `[sp-24] +0xc` | `0x0` |
| `ldr` at 0x79064 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x7906c | store | 4 | `[sp-24] +0x8` | `0x0` |
| `ldr` at 0x79070 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x79078 | store | 4 | `[sp-24] +0x4` | `0x0` |
| `ldr` at 0x7907c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79080 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x7908c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79090 | load | 4 | `[sp-24] +0x14` | `-` |
| `ldr` at 0x79098 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x790a0 | store | 4 | `[sp-24] +0x14` | `0x0` |
| `ldm` at 0x790a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x790a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x790a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79094 | `(unknown callee) 0x20514` | `[[sp-24]+20]` | `r1` | `[sp-24]` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x78fe8 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x79014 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79088 | `[[sp-24]+20] == 0x0` | `[[sp-24]+20] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x790a8 | `r0` | `?` |

## `pcixASICInit`

- `pcixASICInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x790b4 to 0x791b4.
  Established. A computed word store writes entry r1 to an unknown effective target (`str` at 0x790f4).
  Separate computed word stores write [sp-28] and [sp-32] to unknown effective targets (`str` at 0x79120, `str` at 0x7914c).
  Further computed word stores write [sp] and [sp+4] to unknown effective targets (`str` at 0x79178, `str` at 0x791a4).
  The return record places 0x10 in r0 (`ldm` at 0x791ac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x790c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x790c8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x790cc | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x790d0 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x790d4 | load | 4 | `abs 0x791b0` | `-` |
| `ldr` at 0x790d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x790f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x790f4 | store | 4 | `(unknown)` | `r1` |
| `ldr` at 0x790f8 | load | 4 | `abs 0x791b0` | `-` |
| `ldr` at 0x790fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7911c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x79120 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x79124 | load | 4 | `abs 0x791b0` | `-` |
| `ldr` at 0x79128 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79148 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7914c | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x79150 | load | 4 | `abs 0x791b0` | `-` |
| `ldr` at 0x79154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79174 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x79178 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0x7917c | load | 4 | `abs 0x791b0` | `-` |
| `ldr` at 0x79180 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x791a0 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x791a4 | store | 4 | `(unknown)` | `[sp+4]` |
| `ldm` at 0x791ac | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x791ac | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x791ac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x791ac | `r0` | `0x10` |

## `pcixGetSlotCfg`

- `pcixGetSlotCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x791b4 to 0x79278.
  Established. A word loaded from [sp-28] plus 0xc is stored as a word at the address held in [sp-24] (`ldr` at 0x79250, `str` at 0x79254).
  A separate word loaded from [sp-28] plus 0x10 is stored at entry sp minus 0x20 (`ldr` at 0x7925c, `str` at 0x79260).
  Another word store writes 0 at entry sp minus 0x20 (`str` at 0x79210).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x791c4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x791c8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x791cc | load | 4 | `abs 0x79270` | `-` |
| `ldr` at 0x791d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x791d4 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x791e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x791f4 | load | 4 | `abs 0x79274` | `-` |
| `ldr` at 0x791f8 | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x79200 | load | 4 | `(unknown)` | `-` |
| `str` at 0x79210 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x79218 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7922c | load | 4 | `abs 0x79274` | `-` |
| `ldr` at 0x79230 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x79238 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x7923c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79248 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7924c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79250 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x79254 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+12]` |
| `ldr` at 0x79258 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7925c | load | 4 | `[sp-28] +0x10` | `-` |
| `str` at 0x79260 | store | 4 | `sp -0x20` | `[[sp-28]+16]` |
| `ldr` at 0x79264 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x7926c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x7926c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x7926c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x791dc | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x79208 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79244 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x7926c | `r0` | `?` |

## `pcixGetBusCfg`

- `pcixGetBusCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79278 to 0x7933c.
  Established. Word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0x79288, `str` at 0x7928c).
  A word loaded from [sp-28] plus 4 is stored as a word at the address held in [sp-24] (`ldr` at 0x79314, `str` at 0x79318).
  A separate word loaded from [sp-28] plus 8 is stored at entry sp minus 0x20 (`ldr` at 0x79320, `str` at 0x79324).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79288 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x7928c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x79290 | load | 4 | `abs 0x79334` | `-` |
| `ldr` at 0x79294 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79298 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x792a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792b8 | load | 4 | `abs 0x79338` | `-` |
| `ldr` at 0x792bc | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x792c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x792d4 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x792dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x792f0 | load | 4 | `abs 0x79338` | `-` |
| `ldr` at 0x792f4 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x792fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x79300 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7930c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x79310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79314 | load | 4 | `[sp-28] +0x4` | `-` |
| `str` at 0x79318 | store | 4 | `[sp-24] +0x0` | `[[sp-28]+4]` |
| `ldr` at 0x7931c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79320 | load | 4 | `[sp-28] +0x8` | `-` |
| `str` at 0x79324 | store | 4 | `sp -0x20` | `[[sp-28]+8]` |
| `ldr` at 0x79328 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x79330 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79330 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x792a0 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x792cc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x79308 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79330 | `r0` | `?` |

## `pcixUp`

- `pcixUp`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x7933c to 0x79670.
  Established. The result from unnamed 0x7cd40 is stored as a word at entry sp minus 0x20 (`bl` at 0x794dc, `str` at 0x794e4).
  The result from a separate call to unnamed 0x7ce1c is stored as a word at entry sp minus 0x24 (`bl` at 0x79584, `str` at 0x7958c).
  A later call to unnamed 0x7cffc has its result stored as a word at entry sp minus 0x30 (`bl` at 0x795c8, `str` at 0x795d0).
  A computed byte store writes 0 to an unknown effective target (`strb` at 0x7964c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x7934c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x79354 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0x7935c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0x79360 | load | 4 | `abs 0x79664` | `-` |
| `ldr` at 0x79364 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79368 | load | 4 | `abs 0x1a6220` | `-` |
| `ldr` at 0x79374 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79388 | load | 4 | `abs 0x79668` | `-` |
| `ldr` at 0x7938c | load | 4 | `abs 0x1a621c` | `-` |
| `ldr` at 0x79394 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0x793a0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x793b0 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x793b8 | load | 4 | `abs 0x7966c` | `-` |
| `str` at 0x793bc | store | 4 | `sp -0x34` | `0xfffff703` |
| `ldr` at 0x793c0 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x793c4 | store | 4 | `sp -0x30` | `[sp-52]` |
| `ldr` at 0x793cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x793e0 | load | 4 | `abs 0x79668` | `-` |
| `ldr` at 0x793e4 | load | 4 | `abs 0x1a621c` | `-` |
| `str` at 0x793ec | store | 4 | `sp -0x1c` | `?` |
| `ldrb` at 0x793f0 | load | 1 | `sp -0x15` | `-` |
| `str` at 0x79400 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x79404 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79408 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x7940c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7941c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79424 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79430 | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0x79434 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x7943c | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x79444 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x79450 | store | 4 | `sp -0x28` | `ret(0x7a2bc)` |
| `str` at 0x79458 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x7945c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79460 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x79464 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x79470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x79474 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7947c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79488 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x7948c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x79494 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x7949c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x794a0 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0x794a8 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x794b4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x794b8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x794c0 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x794cc | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x794d0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x794d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x794e4 | store | 4 | `sp -0x20` | `ret(0x7cd40)` |
| `ldr` at 0x794e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x794f4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x794f8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79500 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x7950c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x79510 | load | 4 | `abs 0x7966c` | `-` |
| `str` at 0x79514 | store | 4 | `sp -0x30` | `0xfffff703` |
| `ldr` at 0x7951c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79520 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x7952c | store | 4 | `sp -0x28` | `ret(0x7b8f0)` |
| `ldr` at 0x79530 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x79534 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x7953c | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79548 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x7954c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79550 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x79558 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x7955c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79564 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79570 | store | 1 | `(unknown)` | `0x1` |
| `ldrb` at 0x79574 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x79578 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x7957c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x7958c | store | 4 | `sp -0x24` | `ret(0x7ce1c)` |
| `ldr` at 0x79590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7959c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x795a0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x795a8 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x795b4 | store | 1 | `(unknown)` | `0x0` |
| `ldrb` at 0x795b8 | load | 1 | `sp -0x15` | `-` |
| `ldrb` at 0x795bc | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0x795c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x795d0 | store | 4 | `sp -0x30` | `ret(0x7cffc)` |
| `ldr` at 0x795d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x795dc | load | 4 | `sp -0x24` | `-` |
| `str` at 0x795e8 | store | 4 | `sp -0x28` | `ret(0x7ba0c)` |
| `ldr` at 0x795ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x795f8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x795fc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79604 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x79610 | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x79614 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79618 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x79620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79624 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x79630 | store | 4 | `sp -0x28` | `ret(0x7c7e0)` |
| `ldr` at 0x79634 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x79638 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x79640 | load | 4 | `[sp-28] +0x14` | `-` |
| `strb` at 0x7964c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0x79650 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x79654 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x79658 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0x79660 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x79660 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x79660 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x79448 | `(unknown callee) 0x7a2bc` | `[sp-20]` | `?` | `[[sp-28]+8]` | `?` |
| `bl` at 0x794dc | `(unknown callee) 0x7cd40` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0x79524 | `(unknown callee) 0x7b8f0` | `[sp-20]` | `ret(0x7cd40)` | `?` | `ret(0x7cd40)` |
| `bl` at 0x79584 | `(unknown callee) 0x7ce1c` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x795c8 | `(unknown callee) 0x7cffc` | `[sp-20]` | `[sp-21]:1` | `[sp-22]:1` | `[sp-21]:1` |
| `bl` at 0x795e0 | `(unknown callee) 0x7ba0c` | `[sp-20]` | `ret(0x7ce1c)` | `?` | `ret(0x7ce1c)` |
| `bl` at 0x79628 | `(unknown callee) 0x7c7e0` | `[sp-20]` | `[sp-36]` | `?` | `ret(0x7ba0c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x79370 | `r0 >= [0x1a6220] (signed, 32-bit)` | `r0 < [0x1a6220] (signed, 32-bit)` |
| `bne` at 0x7939c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x793a8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x793f8 | `[sp-21]:1 != 0xff` | `[sp-21]:1 == 0xff` |
| `ble` at 0x79414 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0x7946c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x794b0 | `[sp-22]:1 != 0xff` | `[sp-22]:1 == 0xff` |
| `bne` at 0x794f0 | `ret(0x7cd40) != 0x0` | `ret(0x7cd40) == 0x0` |
| `bne` at 0x79598 | `ret(0x7ce1c) != 0x0` | `ret(0x7ce1c) == 0x0` |
| `bge` at 0x795f4 | `ret(0x7ba0c) >= 0x0 (signed, 32-bit)` | `ret(0x7ba0c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x79660 | `r0` | `?` |
