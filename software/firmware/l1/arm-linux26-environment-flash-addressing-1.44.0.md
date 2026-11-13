# ARM Linux 2.6 environment state and flash addressing in 1.44.0

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
No undecodable byte ranges were reported in these spans.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-environment-flash-addressing-facts.txt) retain all tool sections.

## `EnvPowerUpStart`

- `EnvPowerUpStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5a44 to 0xb5a68.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb5a54).
  A later word load reads that stack location into r0 (`ldr` at 0xb5a58).
  The call to unnamed 0xb57f4 has printed r0-r3 candidates entry r0, 2, entry r2 and entry r3 (`bl` at 0xb5a60).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5a54 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5a58 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5a64 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5a64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5a64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5a64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5a60 | `(unknown callee) 0xb57f4` | `r0` | `0x2` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5a64 | `r0` | `ret(0xb57f4)` |

## `EnvPowerUpDone`

- `EnvPowerUpDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5a68 to 0xb5aa0.
  Established. A word store preserves entry r0 at entry sp minus 0x18 (`str` at 0xb5a78).
  The result from unnamed 0x202c8 is stored as a word at absolute address 0x1a6ff0 (`bl` at 0xb5a84, `str` at 0xb5a88).
  The subsequent call to unnamed 0xb57f4 has printed r0-r3 candidates [sp-24], 4, unknown and unknown (`bl` at 0xb5a94).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5a78 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xb5a7c | load | 4 | `abs 0xb5a9c` | `-` |
| `str` at 0xb5a88 | store | 4 | `abs 0x1a6ff0` | `ret(0x202c8)` |
| `ldr` at 0xb5a8c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5a98 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5a98 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5a98 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5a98 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5a98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5a84 | `(unknown callee) 0x202c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xb5a94 | `(unknown callee) 0xb57f4` | `[sp-24]` | `0x4` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5a98 | `r0` | `ret(0xb57f4)` |

## `EnvPowerDownStart`

- `EnvPowerDownStart`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5aa0 to 0xb5ac4.
  Established. The routine writes entry r0 as a word at entry sp minus 0x14 (`str` at 0xb5ab0);
  its reload from that location precedes a call to unnamed 0xb57f4 (`ldr` at 0xb5ab4, `bl` at 0xb5abc).
  That call has printed r0-r3 candidates entry r0, 3, entry r2 and entry r3 (`bl` at 0xb5abc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5ab0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5ab4 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5ac0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5ac0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5ac0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5ac0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5abc | `(unknown callee) 0xb57f4` | `r0` | `0x3` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5ac0 | `r0` | `ret(0xb57f4)` |

## `EnvPowerDownDone`

- `EnvPowerDownDone`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5ac4 to 0xb5aec.
  Established. Entry r0 is saved as a word at entry sp minus 0x14 (`str` at 0xb5ad4).
  A call to unnamed 0xb57f4 has printed r0-r3 candidates entry r0, 4, entry r2 and entry r3 (`bl` at 0xb5ae0).
  The next call goes to unnamed 0xbb818, with unknown printed r1-r3 candidates (`bl` at 0xb5ae4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5ad4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5ad8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5ae8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5ae8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5ae8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5ae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5ae0 | `(unknown callee) 0xb57f4` | `r0` | `0x4` | `r2` | `r3` |
| `bl` at 0xb5ae4 | `(unknown callee) 0xbb818` | `ret(0xb57f4)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5ae8 | `r0` | `ret(0xbb818)` |

## `EnvPowerDown`

- `EnvPowerDown`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5aec to 0xb5b18.
  Established. A word store writes entry r0 at entry sp minus 0x14 (`str` at 0xb5afc).
  A separate word store writes entry r1 at entry sp minus 0x18 (`str` at 0xb5b00).
  The call to unnamed 0xaa248 has printed r0-r3 candidates entry r0, entry r1, entry r2 and entry r3 (`bl` at 0xb5b0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5afc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5b00 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5b04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5b08 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5b14 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5b14 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5b14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5b0c | `(unknown callee) 0xaa248` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5b14 | `r0` | `ret(0xaa248)` |

## `EnvGetStatus`

- `EnvGetStatus`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5b18 to 0xb5b34.
  Established. Included implicit word saves place entry r11, sp and lr at entry sp minus 0x10, 0xc and 8 respectively (`push` at 0xb5b1c).
  A word load reads the literal at absolute address 0xb5b30 (`ldr` at 0xb5b24).
  The recorded return value in r0 is 0x1a7318 (`ldm` at 0xb5b2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb5b24 | load | 4 | `abs 0xb5b30` | `-` |
| `ldm` at 0xb5b2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5b2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5b2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5b2c | `r0` | `0x1a7318` |

## `EnvTempConfigure`

- `EnvTempConfigure`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5b34 to 0xb5b70.
  Established. Two word stores preserve entry r0 and r1 at entry sp minus 0x14 and 0x18 respectively (`str` at 0xb5b44, `str` at 0xb5b48).
  The call to unnamed 0xac1d0 has printed r0-r3 candidates entry r0, entry r1, entry r2 and entry r3 (`bl` at 0xb5b50).
  A later call to unnamed 0xabc84 has printed r0 and r1 candidates [sp-20] and [sp-24] (`bl` at 0xb5b64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5b44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5b48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5b4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5b58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5b5c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5b6c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5b6c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5b6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5b50 | `(unknown callee) 0xac1d0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb5b64 | `(unknown callee) 0xabc84` | `[sp-20]` | `[sp-24]` | `ret(0xac1d0)` | `ret(0xac1d0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5b6c | `r0` | `ret(0xabc84)` |

## `EnvIgnoreLowPower`

- `EnvIgnoreLowPower`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5b70 to 0xb5ba4.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0xb5b80).
  The word store to absolute address 0x1a6fe8 has an unknown value after the predicated effects merge (`moveq` at 0xb5b90, `movne` at 0xb5b94, `str` at 0xb5b98).
  The recorded return value in r0 is entry r0 (`ldm` at 0xb5b9c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5b80 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5b84 | load | 4 | `abs 0xb5ba0` | `-` |
| `ldr` at 0xb5b88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5b98 | store | 4 | `abs 0x1a6fe8` | `?` |
| `ldm` at 0xb5b9c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb5b9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5b9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5b9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5b9c | `r0` | `r0` |

## `EnvAutoFanControl`

- `EnvAutoFanControl`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5ba4 to 0xb5bc4.
  Established. A word load reads the literal at absolute address 0xb5bc0 (`ldr` at 0xb5bb0).
  The word store writes 1 to memory at absolute address 0x1a6fec (`str` at 0xb5bb8).
  The return record preserves entry r0 (`ldm` at 0xb5bbc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb5bb0 | load | 4 | `abs 0xb5bc0` | `-` |
| `str` at 0xb5bb8 | store | 4 | `abs 0x1a6fec` | `0x1` |
| `ldm` at 0xb5bbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5bbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5bbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5bbc | `r0` | `r0` |

## `EnvMessages`

- `EnvMessages`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5bc4 to 0xb5c90.
  Established. The routine stores entry r0 as words at entry sp minus 0x14 and 0x1c (`str` at 0xb5bd4, `str` at 0xb5bdc).
  Separate paths write 1 and 0 to absolute address 0x1a6fe4 (`str` at 0xb5c48, `str` at 0xb5c60).
  Other word stores place 0x16cecc, 0x16cef0, 0x16cf14 or 0x16cf40 at entry sp minus 0x18 (`str` at 0xb5c2c, `str` at 0xb5c38, `str` at 0xb5c50, `str` at 0xb5c68).
  The recorded return value is [sp-24] (`ldm` at 0xb5c78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5bd4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb5bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb5bdc | store | 4 | `sp -0x1c` | `r0` |
| `ldr` at 0xb5be0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5bec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5bf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5c08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb5c18 | load | 4 | `abs 0xb5c7c` | `-` |
| `ldr` at 0xb5c1c | load | 4 | `abs 0x1a6fe4` | `-` |
| `ldr` at 0xb5c28 | load | 4 | `abs 0xb5c80` | `-` |
| `str` at 0xb5c2c | store | 4 | `sp -0x18` | `0x16cecc` |
| `ldr` at 0xb5c34 | load | 4 | `abs 0xb5c84` | `-` |
| `str` at 0xb5c38 | store | 4 | `sp -0x18` | `0x16cef0` |
| `ldr` at 0xb5c40 | load | 4 | `abs 0xb5c7c` | `-` |
| `str` at 0xb5c48 | store | 4 | `abs 0x1a6fe4` | `0x1` |
| `ldr` at 0xb5c4c | load | 4 | `abs 0xb5c88` | `-` |
| `str` at 0xb5c50 | store | 4 | `sp -0x18` | `0x16cf14` |
| `ldr` at 0xb5c58 | load | 4 | `abs 0xb5c7c` | `-` |
| `str` at 0xb5c60 | store | 4 | `abs 0x1a6fe4` | `0x0` |
| `ldr` at 0xb5c64 | load | 4 | `abs 0xb5c8c` | `-` |
| `str` at 0xb5c68 | store | 4 | `sp -0x18` | `0x16cf40` |
| `ldr` at 0xb5c70 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb5c78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb5c78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb5c78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5be8 | `r0 == 0x1` | `r0 != 0x1` |
| `bgt` at 0xb5bf4 | `r0 > 0x1 (signed, 32-bit)` | `r0 <= 0x1 (signed, 32-bit)` |
| `beq` at 0xb5c00 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xb5c10 | `r0 == 0x2` | `r0 != 0x2` |
| `beq` at 0xb5c24 | `[0x1a6fe4] == 0x0` | `[0x1a6fe4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb5c78 | `r0` | `[sp-24]` |

## `flash_lock`

- `flash_lock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1102a0 to 0x110358.
  Established. A word load reads absolute address 0x1a05dc, and its signed value below zero selects the branch to 0x110334 (`ldr` at 0x1102b4, `blt` at 0x1102bc).
  A separate load from the same address supplies the printed r0 candidate of the call to unnamed 0x2052c, whose other printed candidates are 0x1a05c4, 2 and 0x1a05dc (`ldr` at 0x1102c4, `bl` at 0x1102d0).
  The result from unnamed 0x20154 is stored as a word at entry sp minus 0x1c (`bl` at 0x1102f8, `str` at 0x110310).
  Alternative word stores at entry sp minus 0x18 write an unknown value or 0 (`str` at 0x11032c, `str` at 0x110338).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x1102b0 | load | 4 | `abs 0x110348` | `-` |
| `ldr` at 0x1102b4 | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x1102c0 | load | 4 | `abs 0x110348` | `-` |
| `ldr` at 0x1102c4 | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x1102c8 | load | 4 | `abs 0x11034c` | `-` |
| `ldr` at 0x1102f4 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x110300 | load | 4 | `abs 0x110350` | `-` |
| `str` at 0x110304 | store | 4 | `sp -0x24` | `0x18dc28` |
| `ldr` at 0x110308 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x11030c | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `str` at 0x110310 | store | 4 | `sp -0x1c` | `ret(0x20154)` |
| `ldr` at 0x110318 | load | 4 | `abs 0x110354` | `-` |
| `str` at 0x11032c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x110338 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x11033c | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x110344 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x110344 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110344 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110344 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1102d0 | `(unknown callee) 0x2052c` | `[0x1a05dc]` | `0x1a05c4` | `0x2` | `0x1a05dc` |
| `bl` at 0x1102e0 | `(unknown callee) 0x20184` | `ret(0x2052c)` | `?` | `?` | `ret(0x2052c)` |
| `bl` at 0x1102ec | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x1102f8 | `(unknown callee) 0x20154` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x110324 | `(unknown callee) 0x22070` | `0x4` | `0x18dac4` | `0xef` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x1102bc | `[0x1a05dc] < 0x0 (signed, 32-bit)` | `[0x1a05dc] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x1102dc | `ret(0x2052c) >= 0x0 (signed, 32-bit)` | `ret(0x2052c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110344 | `r0` | `?` |

## `flash_unlock`

- `flash_unlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110358 to 0x110410.
  Established. The call to unnamed 0x2052c has printed r0-r3 candidates [0x1a05dc], 0x1a05d4, 1 and 0x1a05dc (`bl` at 0x110388).
  A word store writes 0x18dc64 at entry sp minus 0x24 (`str` at 0x1103bc).
  The result from unnamed 0x20154 is stored as a word at entry sp minus 0x1c (`bl` at 0x1103b0, `str` at 0x1103c8).
  A later call to unnamed 0x22070 has printed r0-r3 candidates 4, 0x18dac4, 0xfa and 0 (`bl` at 0x1103dc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x110368 | load | 4 | `abs 0x110400` | `-` |
| `ldr` at 0x11036c | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x110378 | load | 4 | `abs 0x110400` | `-` |
| `ldr` at 0x11037c | load | 4 | `abs 0x1a05dc` | `-` |
| `ldr` at 0x110380 | load | 4 | `abs 0x110404` | `-` |
| `ldr` at 0x1103ac | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x1103b8 | load | 4 | `abs 0x110408` | `-` |
| `str` at 0x1103bc | store | 4 | `sp -0x24` | `0x18dc64` |
| `ldr` at 0x1103c0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `str` at 0x1103c4 | store | 4 | `sp -0x20` | `[ret(0x20184)]` |
| `str` at 0x1103c8 | store | 4 | `sp -0x1c` | `ret(0x20154)` |
| `ldr` at 0x1103d0 | load | 4 | `abs 0x11040c` | `-` |
| `str` at 0x1103e4 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x1103f0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x1103f4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x1103fc | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0x1103fc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1103fc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1103fc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110388 | `(unknown callee) 0x2052c` | `[0x1a05dc]` | `0x1a05d4` | `0x1` | `0x1a05dc` |
| `bl` at 0x110398 | `(unknown callee) 0x20184` | `ret(0x2052c)` | `?` | `?` | `ret(0x2052c)` |
| `bl` at 0x1103a4 | `(unknown callee) 0x20184` | `ret(0x20184)` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x1103b0 | `(unknown callee) 0x20154` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x1103dc | `(unknown callee) 0x22070` | `0x4` | `0x18dac4` | `0xfa` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x110374 | `[0x1a05dc] < 0x0 (signed, 32-bit)` | `[0x1a05dc] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x110394 | `ret(0x2052c) >= 0x0 (signed, 32-bit)` | `ret(0x2052c) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1103fc | `r0` | `?` |

## `flash_addr_to_index`

- `flash_addr_to_index`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x110410 to 0x1104ac.
  Established. Entry r0 is stored as a word at entry sp minus 0x14 (`str` at 0x110420).
  A word load from absolute address 0x314f74 supplies the nonzero test selecting the branch to 0x11045c (`ldr` at 0x110428, `bne` at 0x110430).
  The result from unnamed 0x110180 is stored as a word at entry sp minus 0x1c (`bl` at 0x110438, `str` at 0x110440).
  A later word store at entry sp minus 0x18 retains an unknown value in the access record (`str` at 0x110468).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x110420 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x110424 | load | 4 | `abs 0x1104a8` | `-` |
| `ldr` at 0x110428 | load | 4 | `abs 0x314f74` | `-` |
| `str` at 0x110440 | store | 4 | `sp -0x1c` | `ret(0x110180)` |
| `ldr` at 0x110444 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110450 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x110454 | store | 4 | `sp -0x20` | `ret(0x110180)` |
| `ldr` at 0x11045c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110468 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x11046c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x110478 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x11048c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110494 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x110498 | store | 4 | `sp -0x20` | `[sp-24]` |
| `ldr` at 0x11049c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x1104a4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x1104a4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x1104a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110438 | `(unknown callee) 0x110180` | `0x0` | `r1` | `r2` | `[0x314f74]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x110430 | `[0x314f74] != 0x0` | `[0x314f74] == 0x0` |
| `bge` at 0x11044c | `ret(0x110180) >= 0x0 (signed, 32-bit)` | `ret(0x110180) < 0x0 (signed, 32-bit)` |
| `blt` at 0x110474 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bgt` at 0x110480 | `[sp-24] > 0x2 (signed, 32-bit)` | `[sp-24] <= 0x2 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x1104a4 | `r0` | `[sp-32]` |

## `flash_offset`

- `flash_offset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1104ac to 0x11053c.
  Established. The result from unnamed 0x110410 is stored as a word at entry sp minus 0x1c (`bl` at 0x1104c4, `str` at 0x1104cc).
  Two indexed word loads have unknown effective addresses (`ldr` at 0x1104f8, `ldr` at 0x11051c).
  The unsigned branch to 0x110510 retains unknown comparison operands (`blo` at 0x110500).
  The word store at entry sp minus 0x20 after the unsupported rsb also has an unknown value (`rsb` at 0x110520, `str` at 0x110524).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1104bc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x1104c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x1104cc | store | 4 | `sp -0x1c` | `ret(0x110410)` |
| `ldr` at 0x1104d0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x1104e0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x1104e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1104f0 | load | 4 | `abs 0x110534` | `-` |
| `ldr` at 0x1104f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x1104f8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x110508 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110510 | load | 4 | `abs 0x110538` | `-` |
| `ldr` at 0x110514 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x110518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x11051c | load | 4 | `(unknown)` | `-` |
| `str` at 0x110524 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x110528 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0x110530 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x110530 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x110530 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1104c4 | `(unknown callee) 0x110410` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x1104d8 | `ret(0x110410) >= 0x0 (signed, 32-bit)` | `ret(0x110410) < 0x0 (signed, 32-bit)` |
| `blo` at 0x110500 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x110530 | `r0` | `[sp-32]` |

## `flash_read_addr`

- `flash_read_addr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x11053c to 0x110654.
  Established. Distinct results from unnamed 0x110410 and 0x1104ac are stored as words at entry sp minus 0x78 and 0x74 respectively (`bl` at 0x110554, `str` at 0x11055c, `bl` at 0x11057c, `str` at 0x110584).
  A call to unnamed 0x15023c has printed r0-r3 candidates unknown, entry sp minus 112, entry sp minus 112 and [sp-120] (`bl` at 0x1105a8).
  The result from unnamed 0x219a4 is stored as a word at entry sp minus 0x80 (`bl` at 0x1105c4, `str` at 0x1105dc).
  The final computed word store at entry sp minus 0x7c has an unknown value, and the prior indexed load has an unknown effective address (`ldr` at 0x110624, `str` at 0x110630).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x11054c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x110550 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x11055c | store | 4 | `sp -0x78` | `ret(0x110410)` |
| `ldr` at 0x110560 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x110570 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x110578 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x110584 | store | 4 | `sp -0x74` | `ret(0x1104ac)` |
| `ldr` at 0x110588 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x110594 | load | 4 | `abs 0x110640` | `-` |
| `ldr` at 0x110598 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x1105a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x1105c0 | load | 4 | `ret(0x20184) +0x0` | `-` |
| `ldr` at 0x1105cc | load | 4 | `abs 0x110644` | `-` |
| `str` at 0x1105d0 | store | 4 | `sp -0x88` | `0x18dca4` |
| `ldr` at 0x1105d4 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x1105d8 | store | 4 | `sp -0x84` | `[sp-120]` |
| `str` at 0x1105dc | store | 4 | `sp -0x80` | `ret(0x219a4)` |
| `ldr` at 0x1105e4 | load | 4 | `abs 0x110648` | `-` |
| `ldr` at 0x1105e8 | load | 4 | `abs 0x11064c` | `-` |
| `str` at 0x1105f8 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x110600 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x110604 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x110614 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x11061c | load | 4 | `abs 0x110650` | `-` |
| `ldr` at 0x110620 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x110624 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x110628 | load | 4 | `sp -0x74` | `-` |
| `str` at 0x110630 | store | 4 | `sp -0x7c` | `?` |
| `ldr` at 0x110634 | load | 4 | `sp -0x7c` | `-` |
| `ldm` at 0x11063c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x11063c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x11063c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x110554 | `(unknown callee) 0x110410` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x11057c | `(unknown callee) 0x1104ac` | `[sp-20]` | `?` | `?` | `ret(0x110410)` |
| `bl` at 0x1105a8 | `(unknown callee) 0x15023c` | `?` | `sp-112` | `sp-112` | `[sp-120]` |
| `bl` at 0x1105b8 | `(unknown callee) 0x20184` | `ret(0x15023c)` | `?` | `?` | `ret(0x15023c)` |
| `bl` at 0x1105c4 | `(unknown callee) 0x219a4` | `[ret(0x20184)]` | `?` | `?` | `ret(0x20184)` |
| `bl` at 0x1105f0 | `(unknown callee) 0x22070` | `0x3` | `0x18dac4` | `0x132` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x110568 | `ret(0x110410) >= 0x0 (signed, 32-bit)` | `ret(0x110410) < 0x0 (signed, 32-bit)` |
| `ble` at 0x110590 | `[sp-120] <= 0x0 (signed, 32-bit)` | `[sp-120] > 0x0 (signed, 32-bit)` |
| `bge` at 0x1105b4 | `ret(0x15023c) >= 0x0 (signed, 32-bit)` | `ret(0x15023c) < 0x0 (signed, 32-bit)` |
| `bhi` at 0x11060c | `hi flags condition (operands unknown)` | `not (hi flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x11063c | `r0` | `[sp-124]` |
