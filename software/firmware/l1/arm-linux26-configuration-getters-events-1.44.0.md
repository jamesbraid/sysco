# ARM Linux 2.6 configuration getters and events in 1.44.0

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
An indirect transfer in L1CfgGetCfg remains unresolved; its tables cover known edges only.
No undecodable byte ranges were reported in these spans.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-configuration-getters-events-facts.txt) retain all tool sections.

## `L1CfgGetBrickCfg`

- `L1CfgGetBrickCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3a70 to 0xa3ac4.
  Established. Word loads read absolute addresses 0x1a69a0 and 0x1a6998 (`ldr` at 0xa3a98, `ldr` at 0xa3a9c).
  The call to unnamed 0x9e7ec has those loaded values followed by entry r0 and entry r1 as printed r0-r3 candidates (`bl` at 0xa3aa8);
  its result is saved at entry sp minus 0x1c, but the meaning of that value remains unknown (`bl` at 0xa3aa8, `str` at 0xa3aac).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3a80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa3a84 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa3a8c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa3a90 | load | 4 | `abs 0xa3ac0` | `-` |
| `ldr` at 0xa3a94 | load | 4 | `abs 0xa3ac0` | `-` |
| `ldr` at 0xa3a98 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa3a9c | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa3aa0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3aa4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3aac | store | 4 | `sp -0x1c` | `ret(0x9e7ec)` |
| `ldr` at 0xa3ab0 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa3abc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3abc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3abc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3aa8 | `(unknown callee) 0x9e7ec` | `[0x1a69a0]` | `[0x1a6998]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3abc | `r0` | `ret(0x9e7ec)` |

## `L1CfgGetBrickIndex`

- `L1CfgGetBrickIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3ac4 to 0xa3bbc.
  Established. Two computed word loads have unknown effective addresses (`ldr` at 0xa3b2c, `ldr` at 0xa3b5c).
  Stores at entry sp minus 0x20 and minus 0x1c initially write zero, while a later store writes one at entry sp minus 0x1c (`str` at 0xa3ae0, `str` at 0xa3ae8, `str` at 0xa3b70).
  Both stores at entry sp minus 0x24 have unknown values, including the path following the unsupported mvn instruction (`str` at 0xa3b98, `mvn` at 0xa3ba0, `str` at 0xa3ba4).
  The final r0 value is the word at entry sp minus 36, whose meaning is unknown (`ldm` at 0xa3bb4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3ad4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa3ad8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa3ae0 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xa3ae8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xa3aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3af8 | load | 4 | `abs 0xa3bb8` | `-` |
| `ldr` at 0xa3afc | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa3b00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3b0c | load | 4 | `abs 0xa3bb8` | `-` |
| `ldr` at 0xa3b10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3b24 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa3b2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3b30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3b3c | load | 4 | `abs 0xa3bb8` | `-` |
| `ldr` at 0xa3b40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3b54 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa3b5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3b60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3b70 | store | 4 | `sp -0x1c` | `0x1` |
| `ldr` at 0xa3b74 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3b7c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa3b84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3b90 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3b98 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xa3ba4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa3ba8 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xa3bb4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3bb4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3bb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa3af4 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xa3b08 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa3b38 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa3b68 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa3b8c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3bb4 | `r0` | `[sp-36]` |

## `L1CfgGetIndexBrick`

- `L1CfgGetIndexBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3bbc to 0xa3c34.
  Established. The first conditional branch goes to 0xa3bf4 when entry r0 is negative under a signed comparison (`blt` at 0xa3bd8).
  A later branch reaches the same address when the word loaded at absolute 0x1a698c is at most entry r0 signed (`ldr` at 0xa3be0, `ble` at 0xa3bec).
  The store at entry sp minus 0x18 writes zero on that path, while the alternative store there has an unknown value (`str` at 0xa3bf8, `str` at 0xa3c20).
  The returned r0 value and the meaning of the computed value remain unknown (`ldm` at 0xa3c2c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3bcc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3bd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3bdc | load | 4 | `abs 0xa3c30` | `-` |
| `ldr` at 0xa3be0 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa3be4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa3bf8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa3c00 | load | 4 | `abs 0xa3c30` | `-` |
| `ldr` at 0xa3c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3c18 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0xa3c20 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xa3c24 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa3c2c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3c2c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3c2c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xa3bd8 | `r0 < 0x0 (signed, 32-bit)` | `r0 >= 0x0 (signed, 32-bit)` |
| `ble` at 0xa3bec | `[0x1a698c] <= r0 (signed, 32-bit)` | `[0x1a698c] > r0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3c2c | `r0` | `?` |

## `L1CfgGetBrick`

- `L1CfgGetBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3c34 to 0xa3c7c.
  Established. Word stores save entry r0 and entry r1 at entry sp minus 0x14 and minus 0x18 (`str` at 0xa3c44, `str` at 0xa3c48).
  Word loads read absolute addresses 0x1a6994 and 0x1a698c (`ldr` at 0xa3c54, `ldr` at 0xa3c58).
  The call to unnamed 0x9e6d0 has those loaded values followed by entry r0 and entry r1 as printed candidates, with the callee responsibility unknown (`bl` at 0xa3c64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3c44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa3c48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa3c4c | load | 4 | `abs 0xa3c78` | `-` |
| `ldr` at 0xa3c50 | load | 4 | `abs 0xa3c78` | `-` |
| `ldr` at 0xa3c54 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa3c58 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa3c5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3c60 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa3c74 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3c74 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3c74 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3c64 | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3c74 | `r0` | `ret(0x9e6d0)` |

## `L1CfgGetCfg`

- `L1CfgGetCfg`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3c7c to 0xa3e30.
  Established. The conditional indirect transfer at 0xa3c98 is unresolved when entry r0 is at most 0x28 unsigned (`ldrls` at 0xa3c98);
  its fall-through path jumps to 0xa3de0 and stores 0x1a6da4 at entry sp minus 0x28 (`b` at 0xa3c9c, `str` at 0xa3de4).
  The known return record gives r0 as 0x1a6da4, whose physical meaning is unknown (`ldm` at 0xa3e24).
  Effects along omitted edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3c8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3c90 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xa3c98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3de0 | load | 4 | `abs 0xa3e2c` | `-` |
| `str` at 0xa3de4 | store | 4 | `sp -0x28` | `0x1a6da4` |
| `ldr` at 0xa3e1c | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xa3e24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3e24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3e24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa3c98 | `r0 <= 0x28 (unsigned, 32-bit)` | `r0 > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3e24 | `r0` | `0x1a6da4` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa3c98).

## `L1CfgGetL2`

- `L1CfgGetL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3e30 to 0xa3eac.
  Established. A word store writes the loaded value at 0x1a6974 into memory at entry r0 (`ldr` at 0xa3e60, `str` at 0xa3e64).
  Two later word stores write the loaded values at 0x1a6978 and 0x1a697c to unknown effective addresses (`str` at 0xa3e80, `str` at 0xa3e9c).
  The return record preserves entry r0, while the meanings of these three software fields remain unknown (`ldm` at 0xa3ea4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3e40 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa3e44 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa3e48 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xa3e4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3e58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3e5c | load | 4 | `abs 0xa3ea8` | `-` |
| `ldr` at 0xa3e60 | load | 4 | `abs 0x1a6974` | `-` |
| `str` at 0xa3e64 | store | 4 | `r0 +0x0` | `[0x1a6974]` |
| `ldr` at 0xa3e68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa3e78 | load | 4 | `abs 0xa3ea8` | `-` |
| `ldr` at 0xa3e7c | load | 4 | `abs 0x1a6978` | `-` |
| `str` at 0xa3e80 | store | 4 | `(unknown)` | `[0x1a6978]` |
| `ldr` at 0xa3e84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3e90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3e94 | load | 4 | `abs 0xa3ea8` | `-` |
| `ldr` at 0xa3e98 | load | 4 | `abs 0x1a697c` | `-` |
| `str` at 0xa3e9c | store | 4 | `(unknown)` | `[0x1a697c]` |
| `ldm` at 0xa3ea4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3ea4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3ea4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa3e54 | `r0 == 0x0` | `r0 != 0x0` |
| `beq` at 0xa3e70 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa3e8c | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3ea4 | `r0` | `r0` |

## `L1CfgGetPort`

- `L1CfgGetPort`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3eac to 0xa3ef4.
  Established. A word store saves entry r0 at entry sp minus 0x14 (`str` at 0xa3ebc).
  A later word load reads absolute address 0x1a6988 (`ldr` at 0xa3ee0).
  The rsb instruction has unsupported effects, and the returned r0 value and computed address meaning remain unknown (`rsb` at 0xa3ed8, `ldm` at 0xa3eec).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3ebc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3ec0 | load | 4 | `abs 0xa3ef0` | `-` |
| `ldr` at 0xa3ec4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3ee0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldm` at 0xa3eec | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa3eec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3eec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3eec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3eec | `r0` | `?` |

## `L1CfgBrickCount`

- `L1CfgBrickCount`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3ef4 to 0xa3f14.
  Established. The literal word load reads absolute address 0xa3f10 (`ldr` at 0xa3f00).
  A word load reads absolute address 0x1a698c, and the return record gives that loaded word as r0 (`ldr` at 0xa3f04, `ldm` at 0xa3f0c).
  The physical meaning of that stored software value remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa3f00 | load | 4 | `abs 0xa3f10` | `-` |
| `ldr` at 0xa3f04 | load | 4 | `abs 0x1a698c` | `-` |
| `ldm` at 0xa3f0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3f0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3f0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3f0c | `r0` | `[0x1a698c]` |

## `L1CfgTypeNonMesh`

- `L1CfgTypeNonMesh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3f14 to 0xa3fa0.
  Established. A word store saves entry r0 at entry sp minus 0x14 (`str` at 0xa3f24).
  The first test branches to 0xa3f80 when entry r0 equals 0x40 (`ldr` at 0xa3f28, `beq` at 0xa3f30).
  Stores at entry sp minus 0x18 write zero at 0xa3f84 and one at 0xa3f90 (`str` at 0xa3f84, `str` at 0xa3f90).
  The meaning of these alternatives and the final r0 value remain unknown (`ldm` at 0xa3f9c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3f24 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3f28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3f70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa3f84 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0xa3f90 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xa3f94 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa3f9c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3f9c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3f9c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa3f30 | `r0 == 0x40` | `r0 != 0x40` |
| `beq` at 0xa3f3c | `r0 == 0x44` | `r0 != 0x44` |
| `beq` at 0xa3f48 | `r0 == 0x24` | `r0 != 0x24` |
| `beq` at 0xa3f54 | `r0 == 0x2c` | `r0 != 0x2c` |
| `beq` at 0xa3f60 | `r0 == 0x7d` | `r0 != 0x7d` |
| `beq` at 0xa3f6c | `r0 == 0x2e` | `r0 != 0x2e` |
| `beq` at 0xa3f78 | `r0 == 0x2d` | `r0 != 0x2d` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3f9c | `r0` | `?` |

## `L1CfgPortNonMesh`

- `L1CfgPortNonMesh`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa3fa0 to 0xa4014.
  Established. A word load reads absolute address 0x1a6984, and the following signed comparison branches to 0xa3ffc when that word is at most entry r0 (`ldr` at 0xa3fb8, `ble` at 0xa3fc4).
  A computed word load has an unknown effective address, and its following stack store has an unknown value (`ldr` at 0xa3ff0, `str` at 0xa3ff4).
  The alternative store at entry sp minus 0x18 writes zero (`str` at 0xa4000).
  The software field meaning and returned r0 value remain unknown (`ldm` at 0xa400c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa3fb0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa3fb4 | load | 4 | `abs 0xa4010` | `-` |
| `ldr` at 0xa3fb8 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa3fbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3fc8 | load | 4 | `abs 0xa4010` | `-` |
| `ldr` at 0xa3fcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3fe8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3ff0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa3ff4 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xa4000 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa4004 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa400c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa400c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa400c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xa3fc4 | `[0x1a6984] <= r0 (signed, 32-bit)` | `[0x1a6984] > r0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa400c | `r0` | `?` |

## `L1CfgGetPortSummary`

- `L1CfgGetPortSummary`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4014 to 0xa4298.
  Established. The call to unnamed 0x201fc has its result stored at entry sp minus 0x24 (`bl` at 0xa4038, `str` at 0xa4040).
  Another store writes that result into memory addressed by the value at entry sp minus 0x18 (`bl` at 0xa4038, `str` at 0xa404c).
  Five computed word loads have unknown effective addresses (`ldr` at 0xa40f0, `ldr` at 0xa4124, `ldr` at 0xa4170, `ldr` at 0xa41a8, `ldr` at 0xa41e0).
  One later store has an unknown target and value, while another writes an unknown value into memory addressed by the word at entry sp minus 36 (`str` at 0xa4200, `str` at 0xa4274).
  The final r0 value is the word at entry sp minus 40, whose meaning remains unknown (`ldm` at 0xa4288).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4024 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa4028 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa402c | load | 4 | `abs 0x1a6984` | `-` |
| `str` at 0xa4040 | store | 4 | `sp -0x24` | `ret(0x201fc)` |
| `ldr` at 0xa4044 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4048 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa404c | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa4050 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa4060 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa4068 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa406c | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa4074 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4084 | load | 4 | `abs 0xa4290` | `-` |
| `str` at 0xa4090 | store | 4 | `sp -0x1c` | `0x1` |
| `str` at 0xa4098 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xa409c | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa40a0 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa40a4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa40b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa40c8 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa40cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa40e8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa40f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa40fc | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa4100 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa411c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa4124 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa4134 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4140 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4148 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa414c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa4168 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa4170 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa4178 | load | 4 | `abs 0xa4294` | `-` |
| `ldr` at 0xa4180 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa4184 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa41a0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa41a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa41b8 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa41bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa41d8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa41e0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa41f0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4200 | store | 4 | `(unknown)` | `?` |
| `str` at 0xa4208 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xa420c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa4214 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xa4220 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa4224 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa4228 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0xa4230 | load | 4 | `abs 0xa4294` | `-` |
| `ldr` at 0xa4238 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa423c | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0xa424c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa4260 | load | 4 | `abs 0xa428c` | `-` |
| `ldr` at 0xa4264 | load | 4 | `abs 0x1a6910` | `-` |
| `str` at 0xa4274 | store | 4 | `[sp-36] +0x0` | `?` |
| `ldr` at 0xa4278 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa427c | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0xa4280 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xa4288 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa4288 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4288 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4038 | `(unknown callee) 0x201fc` | `?` | `r1` | `r2` | `?` |
| `bl` at 0xa4080 | `(unknown callee) 0x20550` | `[sp-36]` | `0x0` | `?` | `?` |
| `bl` at 0xa4088 | `(unknown callee) 0x9adfc` | `0x16b31c` | `?` | `?` | `?` |
| `bl` at 0xa40b4 | `(unknown callee) 0xa9384` | `?` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa421c | `(unknown callee) 0x9afe8` | `?` | `?` | `[0x1a6984]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa4058 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `ble` at 0xa40ac | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa40c0 | `ret(0xa9384) != 0x0` | `ret(0xa9384) == 0x0` |
| `beq` at 0xa40f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa412c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4288 | `r0` | `[sp-40]` |

## `L1CfgSyncL2SSN`

- `L1CfgSyncL2SSN`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa451c to 0xa4564.
  Established. The call to unnamed 0x28230 has printed r0-r3 candidates 0xd and entry r1-r3 (`bl` at 0xa4530).
  The call to unnamed 0x205bc has entry sp minus 144 and 0x16b330 as its first two printed candidates (`bl` at 0xa4544).
  The later call to unnamed 0x201a8 has printed candidates entry sp minus 144, unknown, unknown and entry sp minus 144 (`bl` at 0xa4550).
  The use of that stack address and returned r0 value remain unknown (`ldm` at 0xa455c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa4540 | load | 4 | `abs 0xa4560` | `-` |
| `ldm` at 0xa455c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa455c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa455c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4530 | `(unknown callee) 0x28230` | `0xd` | `r1` | `r2` | `r3` |
| `bl` at 0xa4544 | `(unknown callee) 0x205bc` | `sp-144` | `0x16b330` | `ret(0x28230)` | `sp-144` |
| `bl` at 0xa4550 | `(unknown callee) 0x201a8` | `sp-144` | `?` | `?` | `sp-144` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa455c | `r0` | `?` |

## `L1CfgEventConsoleSubchannel`

- `L1CfgEventConsoleSubchannel`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa4564 to 0xa45c0.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa457c, `str` at 0xa4584).
  A word load reads memory at that result plus 0x14 (`bl` at 0xa457c, `str` at 0xa4584, `ldr` at 0xa458c).
  The equality branch goes to 0xa45b8 when the loaded word equals the word at entry sp minus 20 (`ldr` at 0xa458c, `ldr` at 0xa4590, `beq` at 0xa4598).
  On the fall-through path, a word store writes the word at entry sp minus 20 into memory at the call result plus 0x14 (`bl` at 0xa457c, `str` at 0xa45a4).
  The call to unnamed 0xa4298 has printed candidates two, zero, zero and the word at entry sp minus 20, whose physical meaning is unknown (`bl` at 0xa45b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa4574 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa4584 | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xa4588 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa458c | load | 4 | `ret(0xa3c7c) +0x14` | `-` |
| `ldr` at 0xa4590 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa459c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa45a0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa45a4 | store | 4 | `ret(0xa3c7c) +0x14` | `[sp-20]` |
| `ldm` at 0xa45bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa45bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa45bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa457c | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa45b4 | `(unknown callee) 0xa4298` | `0x2` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa4598 | `[ret(0xa3c7c)+20] == [sp-20]` | `[ret(0xa3c7c)+20] != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa45bc | `r0` | `?` |

## `L1CfgEventPowerState`

- `L1CfgEventPowerState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa45c0 to 0xa461c.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa45d8, `str` at 0xa45e0).
  A word load reads memory at that result plus 0x34 (`bl` at 0xa45d8, `str` at 0xa45e0, `ldr` at 0xa45e8).
  The equality branch goes to 0xa4614 when the loaded word equals the word at entry sp minus 20 (`ldr` at 0xa45e8, `ldr` at 0xa45ec, `beq` at 0xa45f4).
  On the fall-through path, a word store writes the word at entry sp minus 20 into memory at the call result plus 0x34 (`bl` at 0xa45d8, `str` at 0xa4600).
  The call to unnamed 0xa4298 has printed candidates four, zero, zero and the word at entry sp minus 20, whose physical meaning is unknown (`bl` at 0xa4610).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa45d0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa45e0 | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xa45e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa45e8 | load | 4 | `ret(0xa3c7c) +0x34` | `-` |
| `ldr` at 0xa45ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa45f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa45fc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa4600 | store | 4 | `ret(0xa3c7c) +0x34` | `[sp-20]` |
| `ldm` at 0xa4618 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4618 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4618 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa45d8 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa4610 | `(unknown callee) 0xa4298` | `0x4` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa45f4 | `[ret(0xa3c7c)+52] == [sp-20]` | `[ret(0xa3c7c)+52] != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4618 | `r0` | `?` |

## `L1CfgEventMarginState`

- `L1CfgEventMarginState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa461c to 0xa4678.
  Established. The call to unnamed 0xa3c7c has its result saved at entry sp minus 0x18 (`bl` at 0xa4634, `str` at 0xa463c).
  A word load reads memory at that result plus 0x38 (`bl` at 0xa4634, `str` at 0xa463c, `ldr` at 0xa4644).
  The equality branch goes to 0xa4670 when the loaded word equals the word at entry sp minus 20 (`ldr` at 0xa4644, `ldr` at 0xa4648, `beq` at 0xa4650).
  On the fall-through path, a word store writes the word at entry sp minus 20 into memory at the call result plus 0x38 (`bl` at 0xa4634, `str` at 0xa465c).
  The call to unnamed 0xa4298 has printed candidates five, zero, zero and the word at entry sp minus 20, whose physical meaning is unknown (`bl` at 0xa466c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa462c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa463c | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xa4640 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4644 | load | 4 | `ret(0xa3c7c) +0x38` | `-` |
| `ldr` at 0xa4648 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa4654 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa4658 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa465c | store | 4 | `ret(0xa3c7c) +0x38` | `[sp-20]` |
| `ldm` at 0xa4674 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa4674 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa4674 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa4634 | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0xa466c | `(unknown callee) 0xa4298` | `0x5` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa4650 | `[ret(0xa3c7c)+56] == [sp-20]` | `[ret(0xa3c7c)+56] != [sp-20]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa4674 | `r0` | `?` |
