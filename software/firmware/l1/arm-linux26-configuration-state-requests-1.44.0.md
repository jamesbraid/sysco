# ARM Linux 2.6 configuration state and requests in 1.44.0

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
Computed transfers in L1CfgRequest and L1CfgQuery leave unresolved edges. Their tables
cover known edges only.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-configuration-state-requests-facts.txt) retain all tool sections.

## `L1CfgPortDisable`

- `L1CfgPortDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9e2d4 to 0x9e6d0.
  Established. The word stores at 0x1a6634, 0x1a6638 and 0x1a6630 write their loaded values minus one (`str` at 0x9e38c, `str` at 0x9e3d8, `str` at 0x9e3ec).
  The call to unnamed 0xfd4a8 has printed r0-r3 candidates 0x1a68f8, zero, zero and 0x64 (`bl` at 0x9e600);
  its result is saved at entry sp minus 0x1c (`bl` at 0x9e600, `str` at 0x9e608).
  A later word store targets that result plus 0x18 and writes the word loaded at 0x1a6fa8 (`bl` at 0x9e600, `str` at 0x9e628).
  The physical meaning of these software fields and the final r0 value remain unknown (`ldm` at 0x9e6a0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9e2e4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x9e2e8 | load | 4 | `abs 0x9e6a4` | `-` |
| `ldr` at 0x9e2f0 | load | 4 | `abs 0x9e6a8` | `-` |
| `ldr` at 0x9e2f4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0x9e304 | load | 4 | `abs 0x9e6ac` | `-` |
| `ldr` at 0x9e308 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0x9e30c | load | 4 | `abs 0x9e6b0` | `-` |
| `ldr` at 0x9e310 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e318 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e31c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e338 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e340 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9e344 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x9e348 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e34c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e368 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e370 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e37c | load | 4 | `abs 0x9e6b8` | `-` |
| `ldr` at 0x9e380 | load | 4 | `abs 0x9e6b8` | `-` |
| `ldr` at 0x9e384 | load | 4 | `abs 0x1a6634` | `-` |
| `str` at 0x9e38c | store | 4 | `abs 0x1a6634` | `[0x1a6634]-1` |
| `ldr` at 0x9e390 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e3b0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e3b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e3c8 | load | 4 | `abs 0x9e6bc` | `-` |
| `ldr` at 0x9e3cc | load | 4 | `abs 0x9e6bc` | `-` |
| `ldr` at 0x9e3d0 | load | 4 | `abs 0x1a6638` | `-` |
| `str` at 0x9e3d8 | store | 4 | `abs 0x1a6638` | `[0x1a6638]-1` |
| `ldr` at 0x9e3dc | load | 4 | `abs 0x9e6c0` | `-` |
| `ldr` at 0x9e3e0 | load | 4 | `abs 0x9e6c0` | `-` |
| `ldr` at 0x9e3e4 | load | 4 | `abs 0x1a6630` | `-` |
| `str` at 0x9e3ec | store | 4 | `abs 0x1a6630` | `[0x1a6630]-1` |
| `ldr` at 0x9e3f0 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e3f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e410 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9e41c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9e420 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e424 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e440 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9e44c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9e450 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e470 | load | 4 | `abs 0x1a6988` | `-` |
| `str` at 0x9e47c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0x9e480 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e484 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e4a0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9e4bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e4d4 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e4d8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e4e4 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e4e8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e4f4 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e4f8 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e504 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e508 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e514 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e518 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e524 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e528 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e534 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e538 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e544 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e548 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e554 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e558 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e564 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e568 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0x9e574 | load | 4 | `abs 0x9e6b4` | `-` |
| `ldr` at 0x9e578 | load | 4 | `abs 0x1a6910` | `-` |
| `str` at 0x9e5d4 | store | 4 | `sp -0x50` | `0x2` |
| `str` at 0x9e5dc | store | 4 | `sp -0x4c` | `0x2` |
| `ldr` at 0x9e5e0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9e5e4 | store | 4 | `sp -0x48` | `[sp-64]` |
| `str` at 0x9e5ec | store | 4 | `sp -0x44` | `sp-60` |
| `ldr` at 0x9e5f0 | load | 4 | `abs 0x9e6c4` | `-` |
| `str` at 0x9e608 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0x9e60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e61c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e620 | load | 4 | `abs 0x9e6c8` | `-` |
| `ldr` at 0x9e624 | load | 4 | `abs 0x1a6fa8` | `-` |
| `str` at 0x9e628 | store | 4 | `ret(0xfd4a8) +0x18` | `[0x1a6fa8]` |
| `ldr` at 0x9e62c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e64c | load | 4 | `abs 0x9e6cc` | `-` |
| `ldr` at 0x9e650 | load | 4 | `abs 0x31b514` | `-` |
| `ldr` at 0x9e65c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e664 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e684 | load | 4 | `abs 0x9e6cc` | `-` |
| `ldr` at 0x9e688 | load | 4 | `abs 0x31b510` | `-` |
| `ldr` at 0x9e694 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0x9e6a0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9e6a0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9e6a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9e2ec | `(unknown callee) 0x9adfc` | `0x16acec` | `r1` | `r2` | `r3` |
| `bl` at 0x9e314 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ad00` | `[sp-20]` | `0x1a288c` |
| `bl` at 0x9e4b8 | `(unknown callee) 0x20550` | `?` | `0x0` | `0x70` | `?` |
| `bl` at 0x9e4c8 | `(unknown callee) 0x9c3e0` | `ret(0x20550)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9e4d0 | `(unknown callee) 0x9afe8` | `ret(0x20550)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x9e59c | `(unknown callee) 0x958d4` | `sp-64` | `sp-60` | `0x1` | `sp-64` |
| `bl` at 0x9e5b4 | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9e5cc | `(unknown callee) 0x95934` | `sp-64` | `sp-60` | `0x0` | `sp-64` |
| `bl` at 0x9e600 | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `0x0` | `0x0` | `0x64` |
| `bl` at 0x9e63c | `(unknown callee) 0xa92d8` | `0x2` | `?` | `ret(0xfd4a8)` | `[sp-20]` |
| `bl` at 0x9e660 | `(unknown callee) 0xfcb5c` | `[0x31b514]+20` | `[sp-28]` | `?` | `[0x31b514]+20` |
| `bl` at 0x9e674 | `(unknown callee) 0xa92d8` | `0x1` | `?` | `?` | `[sp-20]` |
| `bl` at 0x9e698 | `(unknown callee) 0xfcb5c` | `[0x31b510]+20` | `[sp-28]` | `?` | `[0x31b510]+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9e300 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9e378 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9e3c0 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x9e4c4 | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x9e4e0 | `[0x1a6910] == 0x35` | `[0x1a6910] != 0x35` |
| `beq` at 0x9e4f0 | `[0x1a6910] == 0x4f` | `[0x1a6910] != 0x4f` |
| `beq` at 0x9e500 | `[0x1a6910] == 0x42` | `[0x1a6910] != 0x42` |
| `beq` at 0x9e510 | `[0x1a6910] == 0x46` | `[0x1a6910] != 0x46` |
| `beq` at 0x9e520 | `[0x1a6910] == 0x4a` | `[0x1a6910] != 0x4a` |
| `beq` at 0x9e530 | `[0x1a6910] == 0x4b` | `[0x1a6910] != 0x4b` |
| `beq` at 0x9e540 | `[0x1a6910] == 0x21` | `[0x1a6910] != 0x21` |
| `beq` at 0x9e550 | `[0x1a6910] == 0x5e` | `[0x1a6910] != 0x5e` |
| `beq` at 0x9e560 | `[0x1a6910] == 0x2b` | `[0x1a6910] != 0x2b` |
| `beq` at 0x9e570 | `[0x1a6910] == 0x4c` | `[0x1a6910] != 0x4c` |
| `beq` at 0x9e580 | `[0x1a6910] == 0x4d` | `[0x1a6910] != 0x4d` |
| `bne` at 0x9e614 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `bne` at 0x9e634 | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `beq` at 0x9e648 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |
| `bne` at 0x9e66c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |
| `beq` at 0x9e680 | `ret(0xa92d8) == 0x0` | `ret(0xa92d8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9e6a0 | `r0` | `?` |

## `L1CfgFindBrick`

- `L1CfgFindBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9e6d0 to 0x9e7ec.
  Established. Two computed word loads have unknown effective addresses (`ldr` at 0x9e744, `ldr` at 0x9e770).
  One word store writes one at entry sp minus 0x28, while another writes zero at entry sp minus 0x2c (`str` at 0x9e784, `str` at 0x9e7d8).
  The store at entry sp minus 0x24 has an unknown value, and the final r0 value is also unknown (`str` at 0x9e790, `ldm` at 0x9e7e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9e6e0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9e6e4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9e6e8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9e6ec | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x9e6f4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9e6fc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9e700 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e70c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e718 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e71c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e728 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e73c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e744 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e748 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e768 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e774 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9e784 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x9e788 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9e790 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x9e798 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e7a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e7b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9e7c0 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9e7c4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9e7cc | store | 4 | `sp -0x2c` | `?` |
| `str` at 0x9e7d8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x9e7dc | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x9e7e8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9e7e8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9e7e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9e708 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9e714 | `r0 == 0x0` | `r0 != 0x0` |
| `bge` at 0x9e724 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9e750 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9e77c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9e7a0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9e7e8 | `r0` | `?` |

## `L1CfgFindState`

- `L1CfgFindState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9e7ec to 0x9e944.
  Established. Word loads read absolute addresses 0x1a6908 and 0x1a690c (`ldr` at 0x9e810, `ldr` at 0x9e824).
  A word store writes 0x1a6900 at entry sp minus 0x2c, while two later computed loads have unknown effective addresses (`str` at 0x9e838, `ldr` at 0x9e894, `ldr` at 0x9e8c0).
  Stores at entry sp minus 0x30 write the loaded value there minus 112 and zero on their respective paths, but the returned r0 value remains unknown (`str` at 0x9e91c, `str` at 0x9e928, `ldm` at 0x9e93c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9e7fc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9e800 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9e804 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9e808 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9e80c | load | 4 | `abs 0x9e940` | `-` |
| `ldr` at 0x9e810 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0x9e814 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e820 | load | 4 | `abs 0x9e940` | `-` |
| `ldr` at 0x9e824 | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0x9e828 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9e834 | load | 4 | `abs 0x9e940` | `-` |
| `str` at 0x9e838 | store | 4 | `sp -0x2c` | `0x1a6900` |
| `str` at 0x9e844 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9e84c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9e850 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e868 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9e878 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e88c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e894 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e8a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e8b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e8c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e8c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9e8d4 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x9e8d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9e8e0 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x9e8e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9e8f4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9e908 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9e910 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x9e914 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9e91c | store | 4 | `sp -0x30` | `[sp-48]-112` |
| `str` at 0x9e928 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9e92c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x9e930 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9e934 | load | 4 | `sp -0x2c` | `-` |
| `ldm` at 0x9e93c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x9e93c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x9e93c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9e81c | `[0x1a6908] != r2` | `[0x1a6908] == r2` |
| `bne` at 0x9e830 | `[0x1a690c] != r3` | `[0x1a690c] == r3` |
| `bne` at 0x9e858 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9e864 | `Z == 1` | `not (Z == 1)` |
| `bge` at 0x9e874 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0x9e8a0 | `? != [sp-28]` | `? == [sp-28]` |
| `bne` at 0x9e8cc | `? != [sp-32]` | `? == [sp-32]` |
| `beq` at 0x9e8f0 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x9e93c | `r0` | `?` |

## `L1CfgFindL2`

- `L1CfgFindL2`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9e944 to 0xa00bc.
  Established. The lone call to unnamed 0x6dc0c has entry r0-r3 as printed candidates (`bl` at 0x9e954).
  Word loads read 0x1a698c and 0x1a6994, while another computed load has an unknown effective address (`ldr` at 0x9e978, `ldr` at 0x9e9a0, `ldr` at 0x9e9a8).
  A later comparison branches to 0x9f3e8 when the word loaded from the value at entry sp minus 0x1c plus eight equals 0x43 (`ldr` at 0x9f264, `beq` at 0x9f26c).
  The software field meanings and returned r0 value remain unknown (`ldm` at 0xa00b8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9e960 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x9e968 | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x9e970 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x9e974 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9e978 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9e97c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e988 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9e98c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e9a0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9e9a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9e9b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9e9c0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9e9c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9e9d8 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9e9e0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9e9e8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9e9ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ea00 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ea08 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ea0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ea10 | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x9ea1c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ea20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ea34 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9ea3c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9ea44 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ea48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ea5c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ea64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ea68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ea6c | load | 4 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x9ea78 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ea7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ea80 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9ea9c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9eaa4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eab0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eab4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9eab8 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9ead4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9eadc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eae8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eaec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9eaf0 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9eb0c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9eb14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eb24 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eb28 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eb2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eb40 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9eb48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eb64 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9eb6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eb78 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eb7c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eb80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eb94 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9eb9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ebb8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ebc0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ebcc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ebd0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ebd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ebe8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ebf0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ec0c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ec14 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ec20 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ec24 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ec28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ec3c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ec44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ec60 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ec68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ec74 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ec78 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ec7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ec90 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ec98 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ecb4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ecbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ecc8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eccc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ecd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ece4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ecec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ed08 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ed10 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ed1c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ed20 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ed24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ed38 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ed40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ed5c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ed64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ed70 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ed74 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ed78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ed8c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ed94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9edb0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9edb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9edc4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9edc8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9edcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ede0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ede8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ee04 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ee0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ee18 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ee1c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ee20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ee34 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ee3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ee58 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ee60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ee6c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ee70 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ee74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ee88 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ee90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eeac | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9eeb4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9eec0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eec4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9eec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9eedc | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9eee4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ef00 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ef08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ef14 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ef18 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ef1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ef30 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ef38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ef54 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9ef5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ef68 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ef6c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ef70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ef84 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ef8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9efa8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9efb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9efbc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9efc0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9efc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9efd8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9efe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9effc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f004 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f010 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f014 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f018 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f02c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f034 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f050 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f058 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f064 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f068 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f06c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f080 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f088 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f0a4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f0ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f0b8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f0bc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f0d4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f0dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f0f8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f10c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f110 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f114 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f128 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f130 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f14c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f154 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f160 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f164 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f168 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f17c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f184 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f1a0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f1a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f1b4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f1b8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f1bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f1d0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f1d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f1f4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f1fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f208 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f20c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f224 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f22c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f248 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f250 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f260 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f264 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f270 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f274 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f284 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f290 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f294 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2a4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2b4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2c4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2d4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2e4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f2f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f2f4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f300 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f304 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f310 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f314 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f324 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f330 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f334 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f344 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f350 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f354 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f364 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f374 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f384 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f390 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f394 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f3a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f3a4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f3b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f3b4 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9f3c0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f3d8 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9f3e0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f3e8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f3ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f400 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f408 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f40c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f410 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9f41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f434 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f444 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f448 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f45c | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9f464 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f46c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f470 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f484 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f48c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f490 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f494 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9f4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9f4c8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f4cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f4e0 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9f4e8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f4ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9f4f4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x9f4fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f508 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9f50c | store | 4 | `sp -0x20` | `?` |
| `str` at 0x9f518 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x9f51c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f520 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0x9f524 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f530 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f548 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f550 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f55c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f568 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f56c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f580 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9f588 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f590 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f594 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f5a8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f5b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f5b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f5b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f5c4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f5c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f5dc | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9f5e4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9f5ec | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f5f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f604 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f60c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f610 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f614 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f620 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f624 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f628 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f644 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f64c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f658 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f65c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f660 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9f67c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f684 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f690 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f694 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9f698 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9f6b4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f6bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f6d0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f6d4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f6d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f6ec | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f6f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f710 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f718 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f724 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f728 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f72c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f740 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f748 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f764 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f76c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f778 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f77c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f780 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f794 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f79c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f7b8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f7c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f7cc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f7d0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f7d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f7e8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f7f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f80c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f820 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f824 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f83c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f844 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f860 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f874 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f878 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f87c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f890 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f898 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f8b4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f8bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f8c8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f8cc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f8d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f8e4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f8ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f908 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f910 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f91c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f920 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f924 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f938 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f940 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f95c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f970 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f974 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f978 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f98c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f994 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f9b0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9f9b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9f9c4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f9c8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9f9cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9f9e0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9f9e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fa04 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fa0c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fa18 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fa1c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fa20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fa34 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fa3c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fa58 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fa60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fa6c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fa70 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fa74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fa88 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fa90 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9faac | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fab4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fac0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fac4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fac8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fadc | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fae4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fb00 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fb08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fb14 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fb18 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fb1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fb30 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fb38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fb54 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fb5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fb68 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fb6c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fb70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fb84 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fb8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fba8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fbb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fbbc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fbc0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fbc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fbd8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fbe0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fbfc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fc04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fc10 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fc14 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fc18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fc2c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fc34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fc50 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fc58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fc64 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fc68 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fc6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fc80 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fc88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fca4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fcac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fcb8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fcbc | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fcc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fcd4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fcdc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fcf8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fd00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fd0c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fd10 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fd14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fd28 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fd30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fd4c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fd54 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fd60 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fd64 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fd68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fd7c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fd84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fda0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fda8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fdb4 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fdb8 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9fdd0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9fdd8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fdf4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0x9fdfc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9fe0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe10 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe20 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe30 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe40 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe50 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe60 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe6c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe70 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe80 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fe90 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fe9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fea0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9feac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9feb0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9febc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fec0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fecc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fed0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fedc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fee0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9feec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9fef0 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9fefc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff00 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff10 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff20 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff30 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff40 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff50 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ff60 | load | 4 | `[sp-28] +0x8` | `-` |
| `ldr` at 0x9ff6c | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ff70 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ff84 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0x9ff8c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x9ff94 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9ff98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9ffac | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0x9ffb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9ffb8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x9ffbc | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0x9ffc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ffd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ffe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9fff0 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0x9fff4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0008 | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0xa0010 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa0018 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0xa001c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0030 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa0038 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa003c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0040 | load | 4 | `[sp-28] +0x10` | `-` |
| `ldr` at 0xa004c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0058 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0074 | load | 4 | `abs 0x9f6cc` | `-` |
| `ldr` at 0xa0078 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa008c | load | 4 | `abs 0x1a6994` | `-` |
| `str` at 0xa0094 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa0098 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa00a0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xa00a8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa00ac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa00b0 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xa00b8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa00b8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa00b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9e954 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0x9e984 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x9e9b0 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9e9bc | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `bhs` at 0x9ea18 | `? >= [[sp-28]+20] (unsigned, 32-bit)` | `? < [[sp-28]+20] (unsigned, 32-bit)` |
| `bne` at 0x9ea74 | `? != [[sp-28]+20]` | `? == [[sp-28]+20]` |
| `beq` at 0x9eaac | `? == 0x52` | `? != 0x52` |
| `beq` at 0x9eae4 | `? == 0x44` | `? != 0x44` |
| `beq` at 0x9eb1c | `? == 0x2d` | `? != 0x2d` |
| `beq` at 0x9eb74 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x9ebc8 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x9ec1c | `? == 0x35` | `? != 0x35` |
| `beq` at 0x9ec70 | `? == 0x34` | `? != 0x34` |
| `beq` at 0x9ecc4 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x9ed18 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x9ed6c | `? == 0x39` | `? != 0x39` |
| `beq` at 0x9edc0 | `? == 0x37` | `? != 0x37` |
| `beq` at 0x9ee14 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x9ee68 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x9eebc | `? == 0x51` | `? != 0x51` |
| `beq` at 0x9ef10 | `? == 0x40` | `? != 0x40` |
| `beq` at 0x9ef64 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x9efb8 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x9f00c | `? == 0x46` | `? != 0x46` |
| `beq` at 0x9f060 | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x9f0b4 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x9f108 | `? == 0x24` | `? != 0x24` |
| `beq` at 0x9f15c | `? == 0x2c` | `? != 0x2c` |
| `beq` at 0x9f1b0 | `? == 0x7d` | `? != 0x7d` |
| `beq` at 0x9f204 | `? == 0x2e` | `? != 0x2e` |
| `beq` at 0x9f258 | `? == 0x7b` | `? != 0x7b` |
| `beq` at 0x9f26c | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x9f27c | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x9f28c | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x9f29c | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x9f2ac | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x9f2bc | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x9f2cc | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x9f2dc | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x9f2ec | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x9f2fc | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x9f30c | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x9f31c | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x9f32c | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x9f33c | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x9f34c | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x9f35c | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x9f36c | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x9f37c | `[[sp-28]+8] == 0x24` | `[[sp-28]+8] != 0x24` |
| `beq` at 0x9f38c | `[[sp-28]+8] == 0x2c` | `[[sp-28]+8] != 0x2c` |
| `beq` at 0x9f39c | `[[sp-28]+8] == 0x7d` | `[[sp-28]+8] != 0x7d` |
| `beq` at 0x9f3ac | `[[sp-28]+8] == 0x2e` | `[[sp-28]+8] != 0x2e` |
| `beq` at 0x9f3bc | `[[sp-28]+8] == 0x7b` | `[[sp-28]+8] != 0x7b` |
| `bhs` at 0x9f418 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x9f424 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x9f430 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x9f43c | `[sp-24] == 0x2d` | `[sp-24] != 0x2d` |
| `bhs` at 0x9f49c | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x9f4a8 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x9f4b4 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x9f4c0 | `[sp-24] == 0x2d` | `[sp-24] != 0x2d` |
| `beq` at 0x9f504 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0x9f52c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0x9f558 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0x9f564 | `Z == 0` | `not (Z == 0)` |
| `bhs` at 0x9f5c0 | `C == 1` | `not (C == 1)` |
| `bne` at 0x9f61c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9f654 | `? == 0x52` | `? != 0x52` |
| `beq` at 0x9f68c | `? == 0x44` | `? != 0x44` |
| `beq` at 0x9f6c4 | `? == 0x2d` | `? != 0x2d` |
| `beq` at 0x9f720 | `? == 0x43` | `? != 0x43` |
| `beq` at 0x9f774 | `? == 0x33` | `? != 0x33` |
| `beq` at 0x9f7c8 | `? == 0x35` | `? != 0x35` |
| `beq` at 0x9f81c | `? == 0x34` | `? != 0x34` |
| `beq` at 0x9f870 | `? == 0x41` | `? != 0x41` |
| `beq` at 0x9f8c4 | `? == 0x36` | `? != 0x36` |
| `beq` at 0x9f918 | `? == 0x39` | `? != 0x39` |
| `beq` at 0x9f96c | `? == 0x37` | `? != 0x37` |
| `beq` at 0x9f9c0 | `? == 0x38` | `? != 0x38` |
| `beq` at 0x9fa14 | `? == 0x30` | `? != 0x30` |
| `beq` at 0x9fa68 | `? == 0x51` | `? != 0x51` |
| `beq` at 0x9fabc | `? == 0x40` | `? != 0x40` |
| `beq` at 0x9fb10 | `? == 0x4f` | `? != 0x4f` |
| `beq` at 0x9fb64 | `? == 0x42` | `? != 0x42` |
| `beq` at 0x9fbb8 | `? == 0x46` | `? != 0x46` |
| `beq` at 0x9fc0c | `? == 0x4c` | `? != 0x4c` |
| `beq` at 0x9fc60 | `? == 0x4d` | `? != 0x4d` |
| `beq` at 0x9fcb4 | `? == 0x24` | `? != 0x24` |
| `beq` at 0x9fd08 | `? == 0x2c` | `? != 0x2c` |
| `beq` at 0x9fd5c | `? == 0x7d` | `? != 0x7d` |
| `beq` at 0x9fdb0 | `? == 0x2e` | `? != 0x2e` |
| `beq` at 0x9fe04 | `? == 0x7b` | `? != 0x7b` |
| `beq` at 0x9fe18 | `[[sp-28]+8] == 0x43` | `[[sp-28]+8] != 0x43` |
| `beq` at 0x9fe28 | `[[sp-28]+8] == 0x33` | `[[sp-28]+8] != 0x33` |
| `beq` at 0x9fe38 | `[[sp-28]+8] == 0x35` | `[[sp-28]+8] != 0x35` |
| `beq` at 0x9fe48 | `[[sp-28]+8] == 0x34` | `[[sp-28]+8] != 0x34` |
| `beq` at 0x9fe58 | `[[sp-28]+8] == 0x41` | `[[sp-28]+8] != 0x41` |
| `beq` at 0x9fe68 | `[[sp-28]+8] == 0x36` | `[[sp-28]+8] != 0x36` |
| `beq` at 0x9fe78 | `[[sp-28]+8] == 0x39` | `[[sp-28]+8] != 0x39` |
| `beq` at 0x9fe88 | `[[sp-28]+8] == 0x37` | `[[sp-28]+8] != 0x37` |
| `beq` at 0x9fe98 | `[[sp-28]+8] == 0x38` | `[[sp-28]+8] != 0x38` |
| `beq` at 0x9fea8 | `[[sp-28]+8] == 0x30` | `[[sp-28]+8] != 0x30` |
| `beq` at 0x9feb8 | `[[sp-28]+8] == 0x51` | `[[sp-28]+8] != 0x51` |
| `beq` at 0x9fec8 | `[[sp-28]+8] == 0x40` | `[[sp-28]+8] != 0x40` |
| `beq` at 0x9fed8 | `[[sp-28]+8] == 0x4f` | `[[sp-28]+8] != 0x4f` |
| `beq` at 0x9fee8 | `[[sp-28]+8] == 0x42` | `[[sp-28]+8] != 0x42` |
| `beq` at 0x9fef8 | `[[sp-28]+8] == 0x46` | `[[sp-28]+8] != 0x46` |
| `beq` at 0x9ff08 | `[[sp-28]+8] == 0x4c` | `[[sp-28]+8] != 0x4c` |
| `beq` at 0x9ff18 | `[[sp-28]+8] == 0x4d` | `[[sp-28]+8] != 0x4d` |
| `beq` at 0x9ff28 | `[[sp-28]+8] == 0x24` | `[[sp-28]+8] != 0x24` |
| `beq` at 0x9ff38 | `[[sp-28]+8] == 0x2c` | `[[sp-28]+8] != 0x2c` |
| `beq` at 0x9ff48 | `[[sp-28]+8] == 0x7d` | `[[sp-28]+8] != 0x7d` |
| `beq` at 0x9ff58 | `[[sp-28]+8] == 0x2e` | `[[sp-28]+8] != 0x2e` |
| `beq` at 0x9ff68 | `[[sp-28]+8] == 0x7b` | `[[sp-28]+8] != 0x7b` |
| `bhs` at 0x9ffc4 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0x9ffd0 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0x9ffdc | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0x9ffe8 | `[sp-24] == 0x2d` | `[sp-24] != 0x2d` |
| `bhs` at 0xa0048 | `? >= [[sp-28]+16] (unsigned, 32-bit)` | `? < [[sp-28]+16] (unsigned, 32-bit)` |
| `beq` at 0xa0054 | `[sp-24] == 0x52` | `[sp-24] != 0x52` |
| `beq` at 0xa0060 | `[sp-24] == 0x44` | `[sp-24] != 0x44` |
| `beq` at 0xa006c | `[sp-24] == 0x2d` | `[sp-24] != 0x2d` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa00b8 | `r0` | `?` |

## `L1CfgAddBrick`

- `L1CfgAddBrick`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa00bc to 0xa0438.
  Established. The call to unnamed 0x9e6d0 has printed r0-r3 candidates the words in memory at entry r0 and entry r1, entry r3 and the word at entry sp (`bl` at 0xa00f4);
  its result is stored at entry sp minus 0x30 (`bl` at 0xa00f4, `str` at 0xa00fc).
  A later word store writes the word at entry sp plus eight into memory at that result plus 0xc (`bl` at 0xa00f4, `str` at 0xa0114).
  Another store adds 16 to the word in memory addressed by the value at entry sp minus 0x20 (`str` at 0xa0148).
  The word stored at absolute 0x1a6620 comes from the word addressed by the value at entry sp minus 0x1c, whose physical meaning is unknown (`str` at 0xa041c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa00cc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa00d0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa00d4 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa00d8 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xa00dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa00e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa00e4 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xa00e8 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0xa00ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa00f0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xa00fc | store | 4 | `sp -0x30` | `ret(0x9e6d0)` |
| `ldr` at 0xa0100 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa010c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa0110 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xa0114 | store | 4 | `ret(0x9e6d0) +0xc` | `[sp+8]` |
| `ldr` at 0xa011c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0120 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa0128 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa012c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa0138 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa013c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0140 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xa0148 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]+16` |
| `ldr` at 0xa014c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0154 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0158 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa016c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa017c | store | 4 | `[sp-24] +0x0` | `ret(0x203b8)` |
| `ldr` at 0xa0180 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0184 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa0188 | store | 4 | `sp -0x2c` | `[[sp-24]]` |
| `str` at 0xa0190 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa0194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0198 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa019c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa01a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa01b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa01b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa01cc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa01d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa01dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa01f0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa01f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0208 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa0210 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa0218 | load | 4 | `abs 0xa0428` | `-` |
| `ldr` at 0xa021c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa022c | load | 4 | `abs 0xa042c` | `-` |
| `ldr` at 0xa0230 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xa0234 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0xa0238 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa023c | store | 4 | `sp -0x40` | `[sp+16]` |
| `ldr` at 0xa0240 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa0244 | store | 4 | `sp -0x3c` | `[sp+12]` |
| `ldr` at 0xa0248 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa024c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa0250 | load | 4 | `sp +0x8` | `-` |
| `str` at 0xa0254 | store | 4 | `sp -0x34` | `[sp+8]` |
| `ldr` at 0xa0258 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa025c | load | 4 | `abs 0xa0430` | `-` |
| `ldr` at 0xa0260 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0264 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa026c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa0278 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0280 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0284 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa029c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa02ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa02b0 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa02b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa02c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa02d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa02e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa02f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa02fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0300 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xa0304 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa0324 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa0338 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0340 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa0344 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xa0348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa035c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0364 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xa0368 | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0xa036c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa0380 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0388 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xa038c | store | 4 | `(unknown)` | `[sp+4]` |
| `ldr` at 0xa0390 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa03a4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa03ac | load | 4 | `sp +0x8` | `-` |
| `str` at 0xa03b0 | store | 4 | `(unknown)` | `[sp+8]` |
| `ldr` at 0xa03b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa03c8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa03d0 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xa03d4 | store | 4 | `(unknown)` | `[sp+12]` |
| `ldr` at 0xa03d8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa03ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa03f4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xa03f8 | store | 4 | `(unknown)` | `[sp+16]` |
| `ldr` at 0xa03fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0400 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0404 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa040c | store | 4 | `[sp-28] +0x0` | `[[sp-28]]+1` |
| `ldr` at 0xa0410 | load | 4 | `abs 0xa0434` | `-` |
| `ldr` at 0xa0414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0418 | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xa041c | store | 4 | `abs 0x1a6620` | `[[sp-28]]` |
| `ldm` at 0xa0424 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa0424 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0424 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0424 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa00f4 | `(unknown callee) 0x9e6d0` | `[r0]` | `[r1]` | `r3` | `[sp]` |
| `bl` at 0xa0174 | `(unknown callee) 0x203b8` | `[[sp-24]]` | `?` | `[[sp-32]]` | `?` |
| `bl` at 0xa0268 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ad18` | `[[sp-24]]` | `[sp-36]` |
| `bl` at 0xa02a4 | `(unknown callee) 0x20250` | `?` | `?` | `?` | `?` |
| `bl` at 0xa0320 | `(unknown callee) 0x20250` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa0108 | `ret(0x9e6d0) == 0x0` | `ret(0x9e6d0) != 0x0` |
| `ble` at 0xa0134 | `[[sp-28]]+1 <= [[sp-32]] (signed, 32-bit)` | `[[sp-28]]+1 > [[sp-32]] (signed, 32-bit)` |
| `ble` at 0xa01a4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blo` at 0xa0204 | `? < ? (unsigned, 32-bit)` | `? >= ? (unsigned, 32-bit)` |
| `beq` at 0xa0228 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa0274 | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xa02bc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0424 | `r0` | `?` |

## `L1CfgAddState`

- `L1CfgAddState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa0438 to 0xa07b0.
  Established. The call to unnamed 0x9e7ec has its result saved at entry sp minus 0x28 (`bl` at 0xa04dc, `str` at 0xa04e4).
  A word store increases the loaded value at absolute 0x1a699c by 16 (`str` at 0xa058c).
  The call to unnamed 0x203b8 has its result stored at absolute 0x1a69a0, and a later store increases the word at 0x1a6998 by one (`bl` at 0xa05b8, `str` at 0xa05c0, `str` at 0xa078c).
  The allocation or physical role of these software fields remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa0448 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa044c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xa0450 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xa0454 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0xa045c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xa0460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa046c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0470 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0474 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa0478 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa047c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0480 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0494 | load | 4 | `abs 0xa079c` | `-` |
| `ldr` at 0xa0498 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa04a8 | load | 4 | `abs 0xa07a0` | `-` |
| `ldr` at 0xa04ac | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa04b0 | load | 4 | `abs 0xa07a4` | `-` |
| `ldr` at 0xa04b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa04b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa04c4 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa04c8 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa04cc | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa04d0 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa04d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa04d8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa04e4 | store | 4 | `sp -0x28` | `ret(0x9e7ec)` |
| `ldr` at 0xa04e8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa04f4 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa04f8 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0xa04fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0508 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa050c | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0xa0510 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0520 | load | 4 | `abs 0xa079c` | `-` |
| `ldr` at 0xa0524 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa0534 | load | 4 | `abs 0xa07a0` | `-` |
| `ldr` at 0xa0538 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa053c | load | 4 | `abs 0xa07a8` | `-` |
| `ldr` at 0xa0540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa054c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa0550 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0560 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0564 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa056c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0570 | load | 4 | `abs 0x1a699c` | `-` |
| `ldr` at 0xa057c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0580 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0584 | load | 4 | `abs 0x1a699c` | `-` |
| `str` at 0xa058c | store | 4 | `abs 0x1a699c` | `[0x1a699c]+16` |
| `ldr` at 0xa0590 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0594 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0598 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa059c | load | 4 | `abs 0x1a699c` | `-` |
| `ldr` at 0xa05b0 | load | 4 | `abs 0x1a69a0` | `-` |
| `str` at 0xa05c0 | store | 4 | `abs 0x1a69a0` | `ret(0x203b8)` |
| `str` at 0xa05c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xa05cc | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa05d0 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa05d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa05e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa05e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa05f0 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa05f4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0608 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa0610 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0618 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa061c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0630 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa0638 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0648 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa0650 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xa0658 | load | 4 | `abs 0xa079c` | `-` |
| `ldr` at 0xa065c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa066c | load | 4 | `abs 0xa07a0` | `-` |
| `ldr` at 0xa0670 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa0674 | load | 4 | `abs 0xa07ac` | `-` |
| `ldr` at 0xa0678 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa067c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0684 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0690 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0694 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa069c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa06a0 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa06a4 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa06bc | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa06cc | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa06d0 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa06d4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa06e0 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa06e4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa06f8 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa0704 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0708 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa071c | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa0724 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0728 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa072c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa074c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0750 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa0764 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa0770 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa077c | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0780 | load | 4 | `abs 0xa0798` | `-` |
| `ldr` at 0xa0784 | load | 4 | `abs 0x1a6998` | `-` |
| `str` at 0xa078c | store | 4 | `abs 0x1a6998` | `[0x1a6998]+1` |
| `ldm` at 0xa0794 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa0794 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0794 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0794 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa0484 | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `r0` | `r1` |
| `bl` at 0xa04bc | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ad50` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xa04dc | `(unknown callee) 0x9e7ec` | `[0x1a69a0]` | `[0x1a6998]` | `?` | `?` |
| `bl` at 0xa0548 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ad70` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xa0558 | `(unknown callee) 0x203f4` | `?` | `[sp-32]` | `0x70` | `?` |
| `bl` at 0xa05b8 | `(unknown callee) 0x203b8` | `[0x1a69a0]` | `?` | `[0x1a699c]` | `?` |
| `bl` at 0xa0680 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ad88` | `[sp-24]` | `[sp-28]` |
| `bl` at 0xa06c4 | `(unknown callee) 0x20250` | `[0x1a69a0]+112` | `[0x1a69a0]` | `?` | `?` |
| `bl` at 0xa0748 | `(unknown callee) 0x20250` | `?` | `?` | `?` | `?` |
| `bl` at 0xa0778 | `(unknown callee) 0x203f4` | `?` | `[sp-32]` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa0468 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0xa0490 | `ret(0x9e6d0) != 0x0` | `ret(0x9e6d0) == 0x0` |
| `beq` at 0xa04a4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa04f0 | `ret(0x9e7ec) == 0x0` | `ret(0x9e7ec) != 0x0` |
| `bne` at 0xa0504 | `[0x1a6908] != [sp-24]` | `[0x1a6908] == [sp-24]` |
| `bne` at 0xa0518 | `[0x1a690c] != [sp-28]` | `[0x1a690c] == [sp-28]` |
| `beq` at 0xa0530 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xa0578 | `[0x1a6998]+1 <= [0x1a699c] (signed, 32-bit)` | `[0x1a6998]+1 > [0x1a699c] (signed, 32-bit)` |
| `ble` at 0xa05dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `blt` at 0xa0644 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `beq` at 0xa0668 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa068c | `Z == 0` | `not (Z == 0)` |
| `ble` at 0xa06dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0794 | `r0` | `?` |

## `L1CfgDelete`

- `L1CfgDelete`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa07b0 to 0xa0ae8.
  Established. The call to unnamed 0x20550 at 0xa0924 has printed candidates unknown, zero, 0x18 and unknown (`bl` at 0xa0924).
  A later call to the same numeric callee changes the third printed candidate to 0x70 (`bl` at 0xa0aa4).
  Separate word stores subtract one from the words addressed by the values at entry sp minus 0x18 and entry sp minus 0x20 (`str` at 0xa0938, `str` at 0xa0ab8).
  The record meanings and returned r0 value remain unknown (`ldm` at 0xa0ad4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa07c0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa07c4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xa07c8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xa07cc | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xa07d4 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa07d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa07e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa07e8 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0xa07ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa07f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa080c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0814 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0818 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa0824 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0840 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0844 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa0850 | load | 4 | `abs 0xa0ad8` | `-` |
| `ldr` at 0xa0854 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa0864 | load | 4 | `abs 0xa0adc` | `-` |
| `ldr` at 0xa0868 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa086c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xa0870 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa0874 | load | 4 | `abs 0xa0ae0` | `-` |
| `ldr` at 0xa0878 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa087c | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa0884 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0898 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa08a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa08b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa08c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa08c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa08c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa08f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa08f8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xa090c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa0928 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa092c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0930 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa0938 | store | 4 | `[sp-24] +0x0` | `[[sp-24]]-1` |
| `ldr` at 0xa0940 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa0948 | store | 4 | `sp -0x24` | `?` |
| `str` at 0xa0954 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa0958 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0964 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0968 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa096c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0978 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa098c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0994 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa0998 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa09a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa09b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa09c0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa09c4 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa09d0 | load | 4 | `abs 0xa0ad8` | `-` |
| `ldr` at 0xa09d4 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa09e4 | load | 4 | `abs 0xa0adc` | `-` |
| `ldr` at 0xa09e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa09ec | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0xa09f0 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa09f4 | load | 4 | `abs 0xa0ae4` | `-` |
| `ldr` at 0xa09f8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xa09fc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xa0a04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0a18 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0a20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0a34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0a40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0a44 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa0a48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa0a74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0a78 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa0a8c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa0aa8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa0ab0 | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0xa0ab8 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]-1` |
| `ldr` at 0xa0ac0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa0ac8 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldm` at 0xa0ad4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0ad4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0ad4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa0880 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ada0` | `[sp]` | `[sp+4]` |
| `bl` at 0xa08f0 | `(unknown callee) 0x20250` | `?` | `?` | `?` | `?` |
| `bl` at 0xa0924 | `(unknown callee) 0x20550` | `?` | `0x0` | `0x18` | `?` |
| `bl` at 0xa0a00 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16adc0` | `[sp]` | `[sp+4]` |
| `bl` at 0xa0a70 | `(unknown callee) 0x20250` | `?` | `?` | `?` | `?` |
| `bl` at 0xa0aa4 | `(unknown callee) 0x20550` | `?` | `0x0` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa07e0 | `r0 == 0x0` | `r0 != 0x0` |
| `ble` at 0xa07f4 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa0820 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa084c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa0860 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa0960 | `Z == 1` | `not (Z == 1)` |
| `ble` at 0xa0974 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa09a0 | `? != [sp]` | `? == [sp]` |
| `bne` at 0xa09cc | `? != [sp+4]` | `? == [sp+4]` |
| `beq` at 0xa09e0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0ad4 | `r0` | `?` |

## `L1CfgUpdateState`

- `L1CfgUpdateState`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa0ae8 to 0xa0cd4.
  Established. A word load from the value at entry sp minus 0x18 plus 0xc is saved at entry sp minus 0x12c (`ldr` at 0xa0b68, `str` at 0xa0b6c).
  The call to unnamed 0x202c8 has zero as its printed r0 candidate and its result is stored at absolute 0x1a664c (`bl` at 0xa0ba4, `str` at 0xa0ba8).
  The call to unnamed 0xa0438 has printed candidates the words at entry sp minus 404 and minus 400, entry sp minus 412, and one (`bl` at 0xa0c9c).
  The contents represented by those values and the final r0 value remain unknown (`ldm` at 0xa0cb8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa0af8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa0afc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa0b00 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0xa0b10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0b14 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xa0b18 | store | 4 | `sp -0x1a0` | `[r0+40]` |
| `ldr` at 0xa0b20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa0b2c | store | 4 | `sp -0x1a0` | `ret(0xfde7c)` |
| `ldr` at 0xa0b30 | load | 4 | `sp -0x1a0` | `-` |
| `str` at 0xa0b34 | store | 4 | `sp -0x120` | `[sp-416]` |
| `ldr` at 0xa0b38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0b3c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xa0b4c | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0xa0b50 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0b54 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xa0b60 | store | 4 | `sp -0x128` | `?` |
| `ldr` at 0xa0b64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0b68 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xa0b6c | store | 4 | `sp -0x12c` | `[[sp-24]+12]` |
| `ldr` at 0xa0b70 | load | 4 | `abs 0xa0cbc` | `-` |
| `ldr` at 0xa0b74 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0xa0b78 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0xa0b84 | load | 4 | `abs 0xa0cbc` | `-` |
| `ldr` at 0xa0b88 | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0xa0b8c | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0xa0b9c | load | 4 | `abs 0xa0cc0` | `-` |
| `str` at 0xa0ba8 | store | 4 | `abs 0x1a664c` | `ret(0x202c8)` |
| `ldr` at 0xa0bac | load | 4 | `abs 0xa0cc4` | `-` |
| `ldr` at 0xa0bb4 | load | 4 | `abs 0xa0cc8` | `-` |
| `ldr` at 0xa0bb8 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa0bd4 | load | 4 | `abs 0xa0ccc` | `-` |
| `ldr` at 0xa0bd8 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0xa0bdc | store | 4 | `sp -0x1ac` | `[sp-300]` |
| `str` at 0xa0be0 | store | 4 | `sp -0x1a8` | `ret(0x202c8)` |
| `ldr` at 0xa0be4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa0be8 | load | 4 | `abs 0xa0cd0` | `-` |
| `ldr` at 0xa0bec | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0xa0bf0 | load | 4 | `sp -0x128` | `-` |
| `str` at 0xa0bfc | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xa0c00 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0xa0c04 | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0xa0c10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0c14 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0xa0c28 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xa0c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa0c3c | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0xa0c50 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa0c54 | store | 4 | `sp -0x1a4` | `?` |
| `ldr` at 0xa0c5c | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0xa0c64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa0c70 | store | 4 | `sp -0x1a4` | `ret(0xfde7c)` |
| `ldr` at 0xa0c7c | load | 4 | `sp -0x1a4` | `-` |
| `ldr` at 0xa0c8c | load | 4 | `sp -0x194` | `-` |
| `ldr` at 0xa0c90 | load | 4 | `sp -0x190` | `-` |
| `ldr` at 0xa0ca0 | load | 4 | `sp -0x11c` | `-` |
| `str` at 0xa0ca8 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldm` at 0xa0cb8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa0cb8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa0cb8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa0cb8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa0b28 | `(unknown callee) 0xfde7c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0xa0ba4 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa0bb0 | `(unknown callee) 0x9adfc` | `0x16ade0` | `?` | `?` | `?` |
| `bl` at 0xa0bcc | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa0bf4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16adf4` | `[sp-292]` | `[sp-296]` |
| `bl` at 0xa0c6c | `(unknown callee) 0xfde7c` | `[sp-24]` | `?` | `[sp-24]` | `?` |
| `bl` at 0xa0c84 | `(unknown callee) 0xa0f3c` | `sp-412` | `?` | `0x70` | `sp-412` |
| `bl` at 0xa0c9c | `(unknown callee) 0xa0438` | `[sp-404]` | `[sp-400]` | `sp-412` | `0x1` |
| `bl` at 0xa0cb0 | `(unknown callee) 0x9afe8` | `?` | `?` | `?` | `[sp-288]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa0b0c | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa0b80 | `[0x1a6908] != [sp-292]` | `[0x1a6908] == [sp-292]` |
| `bne` at 0xa0b94 | `[0x1a690c] != [sp-296]` | `[0x1a690c] == [sp-296]` |
| `beq` at 0xa0bc4 | `? == 0x0` | `? != 0x0` |
| `bge` at 0xa0c0c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xa0c34 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa0cb8 | `r0` | `?` |

## `L1CfgRequest`

- `L1CfgRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1144 to 0xa1424.
  Established. A word store initializes entry sp minus 0x4c to zero, while a computed word load has an unknown effective address (`str` at 0xa115c, `ldr` at 0xa1168).
  The call to unnamed 0xa92d8 has printed candidates the word at entry sp minus 84, entry r1, 0x1a69a4 and unknown (`bl` at 0xa1174).
  A conditional indirect transfer at 0xa11d8 is unresolved when the word at entry sp minus 20 minus one is at most 0x30 unsigned (`ldrls` at 0xa11d8).
  Effects along omitted edges and the returned r0 value remain unknown (`ldm` at 0xa13f4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1154 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa115c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xa1160 | load | 4 | `abs 0xa13f8` | `-` |
| `ldr` at 0xa1164 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1168 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa116c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa1170 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa1184 | load | 4 | `abs 0xa13fc` | `-` |
| `ldr` at 0xa1188 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa1198 | load | 4 | `abs 0xa1400` | `-` |
| `ldr` at 0xa119c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa11a0 | load | 4 | `abs 0xa1404` | `-` |
| `ldr` at 0xa11a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa11a8 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xa11c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa11cc | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xa11d8 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xa13f4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa13f4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa13f4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1174 | `(unknown callee) 0xa92d8` | `[sp-84]` | `r1` | `0x1a69a4` | `?` |
| `bl` at 0xa11ac | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16ae30` | `[sp-20]` | `[sp-84]` |
| `bl` at 0xa11c8 | `(unknown callee) 0x958d4` | `sp-76` | `sp-72` | `[sp-20]` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1180 | `ret(0xa92d8) != 0x0` | `ret(0xa92d8) == 0x0` |
| `beq` at 0xa1194 | `? == 0x0` | `? != 0x0` |
| `ldrls` at 0xa11d8 | `[sp-20]-1 <= 0x30 (unsigned, 32-bit)` | `[sp-20]-1 > 0x30 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa13f4 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa11d8).

## `L1CfgProcessRequest`

- `L1CfgProcessRequest`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1424 to 0xa1ab8.
  Established. The call to unnamed 0x202c8 has its result stored at absolute 0x1a6644 (`bl` at 0xa151c, `str` at 0xa1520).
  A word store copies the loaded value at 0x1a698c into memory at 0x1a6894 (`str` at 0xa1674).
  Two calls to unnamed 0x201fc have their results stored at 0x1a6898 and 0x1a689c respectively (`bl` at 0xa1684, `str` at 0xa168c, `bl` at 0xa16b0, `str` at 0xa16b8).
  A byte store writes ([sp-128] & 0xff) into memory at ([0x1a6898] + [sp-136]), whose physical role is unknown (`strb` at 0xa1780).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1434 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa143c | store | 4 | `sp -0x64` | `0x0` |
| `ldr` at 0xa1440 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa1444 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0xa1454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1458 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xa145c | store | 4 | `sp -0x8c` | `[r0+40]` |
| `ldr` at 0xa1464 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa1470 | store | 4 | `sp -0x8c` | `ret(0xfde7c)` |
| `ldr` at 0xa1474 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0xa1478 | store | 4 | `sp -0x74` | `[sp-140]` |
| `ldr` at 0xa148c | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0xa1494 | load | 4 | `sp -0x74` | `-` |
| `str` at 0xa1498 | store | 4 | `sp -0x98` | `[sp-116]` |
| `ldr` at 0xa149c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14a8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14b4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14c0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14cc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14dc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14ec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa14f8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa1504 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0xa1514 | load | 4 | `abs 0xa1a7c` | `-` |
| `str` at 0xa1520 | store | 4 | `abs 0x1a6644` | `ret(0x202c8)` |
| `ldr` at 0xa1524 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1530 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa1534 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xa1540 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa1544 | load | 1 | `[sp-24] +0x2c` | `-` |
| `ldr` at 0xa1554 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1558 | load | 4 | `[sp-24] +0x30` | `-` |
| `str` at 0xa155c | store | 4 | `sp -0x90` | `[[sp-24]+48]` |
| `ldr` at 0xa1564 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa1570 | store | 4 | `sp -0x90` | `ret(0xfde7c)` |
| `ldr` at 0xa1574 | load | 4 | `sp -0x90` | `-` |
| `str` at 0xa1578 | store | 4 | `sp -0x78` | `[sp-144]` |
| `ldr` at 0xa157c | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa158c | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0xa1590 | load | 4 | `sp -0x78` | `-` |
| `str` at 0xa15ac | store | 4 | `sp -0x7c` | `0x1` |
| `ldr` at 0xa15b0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa15c4 | load | 4 | `abs 0xa1a80` | `-` |
| `ldr` at 0xa15cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa15d0 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xa15dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa15e0 | load | 1 | `[sp-24] +0x34` | `-` |
| `ldr` at 0xa15f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa15f4 | load | 4 | `[sp-24] +0x38` | `-` |
| `str` at 0xa15f8 | store | 4 | `sp -0x94` | `[[sp-24]+56]` |
| `ldr` at 0xa1600 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa160c | store | 4 | `sp -0x94` | `ret(0xfde7c)` |
| `ldr` at 0xa1610 | load | 4 | `sp -0x94` | `-` |
| `str` at 0xa1614 | store | 4 | `sp -0x7c` | `[sp-148]` |
| `ldr` at 0xa1618 | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa161c | load | 4 | `abs 0xa1a88` | `-` |
| `ldr` at 0xa1620 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa1624 | load | 4 | `abs 0x1a6894` | `-` |
| `ldr` at 0xa1630 | load | 4 | `abs 0xa1a8c` | `-` |
| `ldr` at 0xa1634 | load | 4 | `abs 0x1a6898` | `-` |
| `ldr` at 0xa1640 | load | 4 | `abs 0xa1a8c` | `-` |
| `ldr` at 0xa1644 | load | 4 | `abs 0x1a6898` | `-` |
| `ldr` at 0xa164c | load | 4 | `abs 0xa1a90` | `-` |
| `ldr` at 0xa1650 | load | 4 | `abs 0x1a689c` | `-` |
| `ldr` at 0xa165c | load | 4 | `abs 0xa1a90` | `-` |
| `ldr` at 0xa1660 | load | 4 | `abs 0x1a689c` | `-` |
| `ldr` at 0xa1668 | load | 4 | `abs 0xa1a88` | `-` |
| `ldr` at 0xa166c | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa1670 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0xa1674 | store | 4 | `abs 0x1a6894` | `[0x1a698c]` |
| `ldr` at 0xa1678 | load | 4 | `abs 0xa1a8c` | `-` |
| `ldr` at 0xa167c | load | 4 | `abs 0xa1a88` | `-` |
| `ldr` at 0xa1680 | load | 4 | `abs 0x1a6894` | `-` |
| `str` at 0xa168c | store | 4 | `abs 0x1a6898` | `ret(0x201fc)` |
| `ldr` at 0xa1690 | load | 4 | `abs 0xa1a90` | `-` |
| `ldr` at 0xa1694 | load | 4 | `abs 0xa1a88` | `-` |
| `ldr` at 0xa1698 | load | 4 | `abs 0x1a6894` | `-` |
| `str` at 0xa16b8 | store | 4 | `abs 0x1a689c` | `ret(0x201fc)` |
| `ldr` at 0xa16d4 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0xa16e4 | store | 4 | `sp -0x84` | `0x0` |
| `str` at 0xa16ec | store | 4 | `sp -0x88` | `0x0` |
| `str` at 0xa16f4 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0xa16f8 | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa16fc | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa1700 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa170c | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa1710 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa1724 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa172c | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa1730 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0xa1744 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa174c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1750 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa175c | store | 4 | `sp -0x78` | `ret(0xa3a70)` |
| `ldr` at 0xa1760 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xa176c | load | 4 | `abs 0xa1a8c` | `-` |
| `ldr` at 0xa1770 | load | 4 | `abs 0x1a6898` | `-` |
| `ldr` at 0xa1774 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa177c | load | 4 | `sp -0x80` | `-` |
| `strb` at 0xa1780 | store | 1 | `([0x1a6898] + [sp-136]) +0x0` | `([sp-128] & 0xff)` |
| `ldr` at 0xa1784 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa1798 | load | 4 | `abs 0xa1a90` | `-` |
| `ldr` at 0xa179c | load | 4 | `abs 0x1a689c` | `-` |
| `ldr` at 0xa17a8 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0xa17b4 | load | 4 | `sp -0x88` | `-` |
| `str` at 0xa17bc | store | 4 | `sp -0x88` | `[sp-136]+1` |
| `ldr` at 0xa17c0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0xa17c8 | store | 4 | `sp -0x80` | `[sp-128]+1` |
| `ldr` at 0xa17e0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0xa17f8 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa1820 | load | 4 | `abs 0xa1a8c` | `-` |
| `ldr` at 0xa182c | load | 4 | `abs 0x1a6898` | `-` |
| `ldr` at 0xa1830 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa1840 | load | 4 | `abs 0xa1a90` | `-` |
| `ldr` at 0xa184c | load | 4 | `abs 0x1a689c` | `-` |
| `ldr` at 0xa1850 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0xa186c | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa1888 | load | 4 | `abs 0xa1a94` | `-` |
| `ldr` at 0xa18a4 | load | 4 | `abs 0xa1a94` | `-` |
| `ldr` at 0xa18c0 | load | 4 | `abs 0xa1a94` | `-` |
| `str` at 0xa18d4 | store | 4 | `sp -0xa8` | `sp-96` |
| `ldr` at 0xa18d8 | load | 4 | `abs 0xa1a98` | `-` |
| `ldr` at 0xa18e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa18e4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa18f0 | store | 4 | `sp -0x68` | `ret(0xfd0dc)` |
| `ldr` at 0xa18f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1914 | load | 4 | `abs 0xa1a84` | `-` |
| `ldr` at 0xa1930 | load | 4 | `abs 0xa1a94` | `-` |
| `ldr` at 0xa193c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1940 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xa1944 | store | 4 | `sp -0x20` | `[[sp-24]+28]` |
| `ldr` at 0xa1948 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa194c | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0xa1950 | store | 4 | `sp -0x70` | `[[sp-24]+12]` |
| `ldr` at 0xa1954 | load | 4 | `abs 0xa1a9c` | `-` |
| `ldr` at 0xa1958 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa1968 | load | 4 | `abs 0xa1aa0` | `-` |
| `ldr` at 0xa196c | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa1970 | load | 4 | `abs 0xa1aa4` | `-` |
| `ldr` at 0xa1974 | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa1984 | store | 4 | `sp -0xa8` | `0x1` |
| `str` at 0xa198c | store | 4 | `sp -0xa4` | `0x2` |
| `ldr` at 0xa1990 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xa1994 | store | 4 | `sp -0xa0` | `[sp-100]` |
| `str` at 0xa199c | store | 4 | `sp -0x9c` | `sp-96` |
| `ldr` at 0xa19a0 | load | 4 | `abs 0xa1a98` | `-` |
| `ldr` at 0xa19ac | load | 4 | `sp -0x70` | `-` |
| `str` at 0xa19b8 | store | 4 | `sp -0x1c` | `ret(0xfd4a8)` |
| `ldr` at 0xa19bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa19c8 | load | 4 | `abs 0xa1aa0` | `-` |
| `ldr` at 0xa19cc | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa19d0 | load | 4 | `abs 0xa1aa8` | `-` |
| `ldr` at 0xa19dc | load | 4 | `abs 0xa1a9c` | `-` |
| `ldr` at 0xa19e0 | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa19f0 | load | 4 | `abs 0xa1aa0` | `-` |
| `ldr` at 0xa19f4 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa19f8 | load | 4 | `abs 0xa1aac` | `-` |
| `ldr` at 0xa19fc | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa1a04 | load | 4 | `abs 0xa1ab0` | `-` |
| `ldr` at 0xa1a08 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa1a1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1a28 | load | 4 | `abs 0xa1ab0` | `-` |
| `ldr` at 0xa1a2c | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa1a40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1a4c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa1a60 | load | 4 | `abs 0xa1ab4` | `-` |
| `ldr` at 0xa1a64 | load | 4 | `sp -0x70` | `-` |
| `ldr` at 0xa1a6c | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xa1a78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa1a78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1a78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1a78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa146c | `(unknown callee) 0xfde7c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0xa1490 | `(unknown callee) 0x958d4` | `sp-100` | `sp-96` | `[sp-140]` | `sp-100` |
| `bl` at 0xa151c | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[sp-116]` |
| `bl` at 0xa1528 | `(unknown callee) 0x9b070` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xa156c | `(unknown callee) 0xfde7c` | `[sp-24]` | `0x1` | `?` | `?` |
| `bl` at 0xa1598 | `(unknown callee) 0xee1bc` | `[sp-144]` | `0x8000` | `?` | `0x0` |
| `bl` at 0xa15c0 | `(unknown callee) 0xa485c` | `0x1` | `?` | `?` | `[sp-124]` |
| `bl` at 0xa15c8 | `(unknown callee) 0x9adfc` | `0x16aeb4` | `?` | `?` | `?` |
| `bl` at 0xa1608 | `(unknown callee) 0xfde7c` | `[sp-24]` | `0x2` | `?` | `?` |
| `bl` at 0xa1648 | `(unknown callee) 0x20514` | `[0x1a6898]` | `[0x1a698c]` | `0x1a6894` | `0x1a6898` |
| `bl` at 0xa1664 | `(unknown callee) 0x20514` | `[0x1a689c]` | `?` | `?` | `0x1a689c` |
| `bl` at 0xa1684 | `(unknown callee) 0x201fc` | `[0x1a6894]` | `?` | `0x1a6894` | `0x1a6894` |
| `bl` at 0xa16b0 | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a6894]` | `?` |
| `bl` at 0xa16d0 | `(unknown callee) 0x958d4` | `sp-100` | `sp-96` | `0x7` | `sp-100` |
| `bl` at 0xa1754 | `(unknown callee) 0xa3a70` | `?` | `?` | `?` | `?` |
| `bl` at 0xa17b0 | `(unknown callee) 0x203f4` | `?` | `[sp-120]` | `0x70` | `?` |
| `bl` at 0xa17e4 | `(unknown callee) 0x95934` | `sp-100` | `sp-96` | `[sp-124]` | `sp-100` |
| `bl` at 0xa17fc | `(unknown callee) 0x95934` | `sp-100` | `sp-96` | `[sp-136]` | `sp-100` |
| `bl` at 0xa1814 | `(unknown callee) 0x95934` | `sp-100` | `sp-96` | `0x70` | `sp-100` |
| `bl` at 0xa1834 | `(unknown callee) 0x95ab0` | `sp-100` | `sp-96` | `[0x1a6898]` | `[sp-136]` |
| `bl` at 0xa1854 | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `[0x1a689c]` | `[sp-136]` |
| `bl` at 0xa1874 | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6900` | `0x1` |
| `bl` at 0xa1890 | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6da4` | `0x1` |
| `bl` at 0xa18ac | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6da4` | `0x1` |
| `bl` at 0xa18c8 | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6da4` | `0x1` |
| `bl` at 0xa18cc | `(unknown callee) 0x9afe8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa18e8 | `(unknown callee) 0xfd0dc` | `0x1a68f8` | `0x0` | `[sp-24]` | `[sp-100]` |
| `bl` at 0xa1900 | `(unknown callee) 0x9d974` | `[sp-24]` | `0x1` | `?` | `[sp-116]` |
| `bl` at 0xa191c | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6900` | `0x1` |
| `bl` at 0xa1938 | `(unknown callee) 0xa0cd4` | `sp-100` | `sp-96` | `0x1a6da4` | `0x1` |
| `bl` at 0xa1978 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aecc` | `[[sp-24]+12]` | `0x1a288c` |
| `bl` at 0xa19b0 | `(unknown callee) 0xfd4a8` | `0x1a68f8` | `sp-32` | `0x0` | `?` |
| `bl` at 0xa19d4 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16aeec` | `?` | `0x1a288c` |
| `bl` at 0xa1a00 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16af24` | `[sp-112]` | `0x1a288c` |
| `bl` at 0xa1a50 | `(unknown callee) 0xfcb5c` | `?` | `?` | `0x31b45c` | `?` |
| `bl` at 0xa1a68 | `(unknown callee) 0x29908` | `0x16af48` | `[sp-112]` | `?` | `ret(0xfcb5c)` |
| `bl` at 0xa1a70 | `(unknown callee) 0xfb794` | `[sp-28]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1450 | `? != 0x0` | `? == 0x0` |
| `bgt` at 0xa14a4 | `[sp-116] > 0xf (signed, 32-bit)` | `[sp-116] <= 0xf (signed, 32-bit)` |
| `bge` at 0xa14b0 | `[sp-116] >= 0xc (signed, 32-bit)` | `[sp-116] < 0xc (signed, 32-bit)` |
| `beq` at 0xa14bc | `[sp-116] == 0x6` | `[sp-116] != 0x6` |
| `bgt` at 0xa14c8 | `[sp-116] > 0x6 (signed, 32-bit)` | `[sp-116] <= 0x6 (signed, 32-bit)` |
| `blt` at 0xa14d4 | `[sp-116] < 0x1 (signed, 32-bit)` | `[sp-116] >= 0x1 (signed, 32-bit)` |
| `beq` at 0xa14e4 | `[sp-116] == 0x9` | `[sp-116] != 0x9` |
| `blt` at 0xa14f4 | `[sp-116] < 0x14 (signed, 32-bit)` | `[sp-116] >= 0x14 (signed, 32-bit)` |
| `ble` at 0xa1500 | `[sp-116] <= 0x31 (signed, 32-bit)` | `[sp-116] > 0x31 (signed, 32-bit)` |
| `beq` at 0xa150c | `[sp-116] == 0x3e8` | `[sp-116] != 0x3e8` |
| `bls` at 0xa153c | `[[sp-24]+32]:1 <= 0x1 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xa1550 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa1584 | `[sp-144] == 0x0` | `[sp-144] != 0x0` |
| `beq` at 0xa15a4 | `ret(0xee1bc) == 0x0` | `ret(0xee1bc) != 0x0` |
| `beq` at 0xa15b8 | `[sp-124] == 0x0` | `[sp-124] != 0x0` |
| `bls` at 0xa15d8 | `[[sp-24]+32]:1 <= 0x2 (unsigned, 32-bit)` | `[[sp-24]+32]:1 > 0x2 (unsigned, 32-bit)` |
| `bne` at 0xa15ec | `? != 0x0` | `? == 0x0` |
| `ble` at 0xa162c | `[0x1a698c] <= [0x1a6894] (signed, 32-bit)` | `[0x1a698c] > [0x1a6894] (signed, 32-bit)` |
| `beq` at 0xa163c | `[0x1a6898] == 0x0` | `[0x1a6898] != 0x0` |
| `beq` at 0xa1658 | `[0x1a689c] == 0x0` | `[0x1a689c] != 0x0` |
| `bne` at 0xa16dc | `[sp-124] != 0x0` | `[sp-124] == 0x0` |
| `ble` at 0xa1708 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa1768 | `ret(0xa3a70) == 0x0` | `ret(0xa3a70) != 0x0` |
| `beq` at 0xa1964 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa19c4 | `ret(0xfd4a8) != 0x0` | `ret(0xfd4a8) == 0x0` |
| `beq` at 0xa19ec | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa1a24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa1a5c | `ret(0xfcb5c) == 0x0` | `ret(0xfcb5c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1a78 | `r0` | `?` |

## `L1CfgProcessResponse`

- `L1CfgProcessResponse`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1ab8 to 0xa1b8c.
  Established. A byte load reads memory at entry r0 plus 0x13, and the following branch goes to 0xa1b74 when that value is nonzero (`ldrb` at 0xa1ad0, `bne` at 0xa1ad8).
  A word loaded from memory at entry r0 plus 0x28 is stored at entry sp minus 0x24 (`ldr` at 0xa1af4, `str` at 0xa1af8).
  The call to unnamed 0x202c8 has its result stored at absolute 0x1a6644 (`bl` at 0xa1b5c, `str` at 0xa1b60).
  The field meanings and returned r0 value remain unknown (`ldm` at 0xa1b78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1ac8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xa1acc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa1ad0 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0xa1adc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xa1ae0 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0xa1af0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1af4 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0xa1af8 | store | 4 | `sp -0x24` | `[r0+40]` |
| `ldr` at 0xa1b00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa1b0c | store | 4 | `sp -0x24` | `ret(0xfde7c)` |
| `ldr` at 0xa1b10 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa1b14 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0xa1b18 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1b28 | load | 4 | `abs 0xa1b7c` | `-` |
| `ldr` at 0xa1b2c | load | 4 | `abs 0x1a6628` | `-` |
| `ldr` at 0xa1b3c | load | 4 | `abs 0xa1b80` | `-` |
| `ldr` at 0xa1b40 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1b44 | load | 4 | `abs 0x1a288c` | `-` |
| `ldr` at 0xa1b48 | load | 4 | `abs 0xa1b84` | `-` |
| `ldr` at 0xa1b4c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xa1b54 | load | 4 | `abs 0xa1b88` | `-` |
| `str` at 0xa1b60 | store | 4 | `abs 0x1a6644` | `ret(0x202c8)` |
| `ldr` at 0xa1b68 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xa1b78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa1b78 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1b78 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1b78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1b08 | `(unknown callee) 0xfde7c` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0xa1b50 | `(unknown callee) 0x244d4` | `[0x1a288c]` | `0x16af74` | `[[sp-24]+12]` | `0x1a288c` |
| `bl` at 0xa1b5c | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa1b70 | `(unknown callee) 0x9d974` | `[sp-24]` | `0x0` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1ad8 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `bne` at 0xa1aec | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa1b20 | `[sp-36] == 0x3e8` | `[sp-36] != 0x3e8` |
| `beq` at 0xa1b38 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1b78 | `r0` | `?` |

## `L1CfgProcessEvent`

- `L1CfgProcessEvent`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1b8c to 0xa1cec.
  Established. A halfword load reads memory at entry r0 plus 0x22 and saves the value at entry sp minus 0x8c (`ldrh` at 0xa1bb4, `str` at 0xa1bb8).
  Comparisons send the value six to 0xa1c9c and the value 0x3e9 to 0xa1cd8 (`beq` at 0xa1bc4, `beq` at 0xa1c64).
  The call to unnamed 0xa0ae8 has printed r0-r3 candidates entry r0, entry r1, that loaded halfword and 0x3e9 (`bl` at 0xa1cdc).
  The halfword meaning and returned r0 value remain unknown (`ldm` at 0xa1ce4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1b9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa1ba0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1ba4 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0xa1bb0 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xa1bb4 | load | 2 | `r0 +0x22` | `-` |
| `str` at 0xa1bb8 | store | 4 | `sp -0x8c` | `[r0+34]:2` |
| `ldr` at 0xa1bbc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1bc8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1bd4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1be0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1bec | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1bfc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c08 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c18 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c24 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c30 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c3c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c4c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c58 | load | 4 | `abs 0xa1ce8` | `-` |
| `ldr` at 0xa1c5c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0xa1c6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1c78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1c84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1c90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1cb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1cc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1ccc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1cd8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa1ce4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1ce4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1ce4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1c70 | `(unknown callee) 0xa499c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1c7c | `(unknown callee) 0xa4ab0` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1c88 | `(unknown callee) 0xa4c24` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1c94 | `(unknown callee) 0xa4d30` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1ca0 | `(unknown callee) 0xa4e3c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1cac | `(unknown callee) 0xa4f48` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1cb8 | `(unknown callee) 0xa505c` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1cc4 | `(unknown callee) 0xa5858` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1cd0 | `(unknown callee) 0xa5efc` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0xa1cdc | `(unknown callee) 0xa0ae8` | `r0` | `r1` | `[r0+34]:2` | `0x3e9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1bac | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `beq` at 0xa1bc4 | `[r0+34]:2 == 0x6` | `[r0+34]:2 != 0x6` |
| `bgt` at 0xa1bd0 | `[r0+34]:2 > 0x6 (signed, 32-bit)` | `[r0+34]:2 <= 0x6 (signed, 32-bit)` |
| `beq` at 0xa1bdc | `[r0+34]:2 == 0x3` | `[r0+34]:2 != 0x3` |
| `bgt` at 0xa1be8 | `[r0+34]:2 > 0x3 (signed, 32-bit)` | `[r0+34]:2 <= 0x3 (signed, 32-bit)` |
| `beq` at 0xa1bf4 | `[r0+34]:2 == 0x2` | `[r0+34]:2 != 0x2` |
| `beq` at 0xa1c04 | `[r0+34]:2 == 0x4` | `[r0+34]:2 != 0x4` |
| `beq` at 0xa1c10 | `[r0+34]:2 == 0x5` | `[r0+34]:2 != 0x5` |
| `beq` at 0xa1c20 | `[r0+34]:2 == 0x64` | `[r0+34]:2 != 0x64` |
| `bgt` at 0xa1c2c | `[r0+34]:2 > 0x64 (signed, 32-bit)` | `[r0+34]:2 <= 0x64 (signed, 32-bit)` |
| `beq` at 0xa1c38 | `[r0+34]:2 == 0x7` | `[r0+34]:2 != 0x7` |
| `beq` at 0xa1c44 | `[r0+34]:2 == 0x8` | `[r0+34]:2 != 0x8` |
| `beq` at 0xa1c54 | `[r0+34]:2 == 0x66` | `[r0+34]:2 != 0x66` |
| `beq` at 0xa1c64 | `[r0+34]:2 == 0x3e9` | `[r0+34]:2 != 0x3e9` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1ce4 | `r0` | `?` |

## `L1CfgIRouter`

- `L1CfgIRouter`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1cec to 0xa1dac.
  Established. Byte loads read memory at entry r0 plus 0x16 (`ldrb` at 0xa1d0c, `ldrb` at 0xa1d6c, `ldrb` at 0xa1d88).
  Word loads read memory at entry r0 plus 0x1c (`ldr` at 0xa1d1c, `ldr` at 0xa1d2c, `ldr` at 0xa1d48).
  The branch at 0xa1d74 falls through to the call to unnamed 0xa1424 when the byte loaded at 0xa1d6c is zero (`ldrb` at 0xa1d6c, `bne` at 0xa1d74, `bl` at 0xa1d7c);
  its taken edge reaches a separate byte load (`ldrb` at 0xa1d88).
  The branch at 0xa1d90 falls through to the call to unnamed 0xa1ab8 when this second value equals one (`bne` at 0xa1d90, `bl` at 0xa1d98).
  The final call to unnamed 0xfb794 has printed candidates the word at entry sp minus 20 followed by three unknowns (`bl` at 0xa1da0).
  The byte and word fields have no established physical meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1cfc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa1d04 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xa1d08 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1d0c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0xa1d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d1c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa1d28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d2c | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa1d44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d48 | load | 4 | `r0 +0x1c` | `-` |
| `ldr` at 0xa1d5c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d68 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1d6c | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0xa1d78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d84 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xa1d88 | load | 1 | `r0 +0x16` | `-` |
| `ldr` at 0xa1d94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa1d9c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa1da8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa1da8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa1da8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1d60 | `(unknown callee) 0xa1b8c` | `r0` | `r1` | `r2` | `?` |
| `bl` at 0xa1d7c | `(unknown callee) 0xa1424` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0xa1d98 | `(unknown callee) 0xa1ab8` | `r0` | `r1` | `r2` | `[r0+22]:1` |
| `bl` at 0xa1da0 | `(unknown callee) 0xfb794` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xa1d14 | `[r0+22]:1 != 0x2` | `[r0+22]:1 == 0x2` |
| `bge` at 0xa1d24 | `[r0+28] >= 0x0 (signed, 32-bit)` | `[r0+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0xa1d3c | `? == 0xe` | `? != 0xe` |
| `beq` at 0xa1d54 | `? == 0xe` | `? != 0xe` |
| `bne` at 0xa1d74 | `[r0+22]:1 != 0x0` | `[r0+22]:1 == 0x0` |
| `bne` at 0xa1d90 | `[r0+22]:1 != 0x1` | `[r0+22]:1 == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa1da8 | `r0` | `ret(0xfb794)` |

## `L1CfgQuery`

- `L1CfgQuery`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa1dac to 0xa39e4.
  Established. The call to unnamed 0x201fc has its result stored in the memory addressed by the value at entry sp minus 0x18 (`bl` at 0xa1e10, `str` at 0xa1e18).
  A later call to unnamed 0xa3a70 has its result saved at entry sp minus 0x3c (`bl` at 0xa2170, `str` at 0xa2178).
  The conditional indirect transfer at 0xa2294 is unresolved when the word loaded from the value at entry sp minus 0x3c plus 0x38 is at most five unsigned (`ldrls` at 0xa2294).
  A word store writes zero at absolute 0x1a664c, but the software field meaning and effects along omitted edges remain unknown (`str` at 0xa2c00).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa1dbc | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xa1dc0 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xa1dc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa1dd0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa1de4 | load | 4 | `abs 0xa2d5c` | `-` |
| `ldr` at 0xa1dec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa1df0 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1df4 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0xa1e18 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa1e1c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa1e20 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0xa1e28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa1e2c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1e30 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa1e34 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1e40 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1e44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1e58 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1e60 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa1e64 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa1e68 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1e6c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1e80 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1e88 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa1e8c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa1e90 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1e94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1ea8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1eb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1ebc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa1ed0 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1ed4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1ee8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1ef0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1f08 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1f0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1f20 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1f28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1f34 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1f38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1f4c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1f54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1f58 | load | 4 | `abs 0xa2d64` | `-` |
| `ldr` at 0xa1f5c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1f68 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1f6c | load | 4 | `abs 0xa2d68` | `-` |
| `ldr` at 0xa1f74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1f84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa1f8c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa1f90 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1f94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1fa8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1fb0 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1fb4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1fc8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1fd0 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa1fd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa1fe8 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa1ff0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa1ff4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1ff8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa1ffc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2004 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2014 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa201c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2020 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2024 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2038 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2040 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa204c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2050 | load | 4 | `abs 0xa2d6c` | `-` |
| `ldr` at 0xa205c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2060 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2064 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2078 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2080 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa2084 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2094 | load | 4 | `abs 0xa2d70` | `-` |
| `ldr` at 0xa20a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa20b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa20b8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa20bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa20cc | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa20d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa20e4 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa20ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa20f0 | load | 4 | `abs 0xa2d74` | `-` |
| `ldr` at 0xa20f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa20fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa210c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2114 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2118 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2128 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa212c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2140 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2148 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa214c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2160 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2168 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa216c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa2178 | store | 4 | `sp -0x3c` | `ret(0xa3a70)` |
| `ldr` at 0xa217c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2180 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2194 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa219c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa21a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa21ac | load | 4 | `abs 0xa2d78` | `-` |
| `ldr` at 0xa21b8 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa21bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa21d0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa21d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa21e4 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa21e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa21fc | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2204 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2208 | load | 4 | `abs 0xa2d7c` | `-` |
| `ldr` at 0xa220c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2218 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa221c | load | 4 | `abs 0xa2d80` | `-` |
| `ldr` at 0xa2224 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2234 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa223c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2240 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa224c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2250 | load | 4 | `abs 0xa2d84` | `-` |
| `ldr` at 0xa225c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2260 | load | 4 | `[sp-60] +0x34` | `-` |
| `ldr` at 0xa226c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2270 | load | 4 | `abs 0xa2d88` | `-` |
| `ldr` at 0xa227c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2280 | load | 4 | `abs 0xa2d8c` | `-` |
| `ldr` at 0xa2288 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa228c | load | 4 | `[sp-60] +0x38` | `-` |
| `ldrls` at 0xa2294 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2314 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2318 | load | 4 | `abs 0xa2da8` | `-` |
| `ldr` at 0xa2320 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2324 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa2334 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2338 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa2348 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa234c | load | 4 | `abs 0xa2dac` | `-` |
| `ldr` at 0xa2358 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa235c | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa236c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2370 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa2380 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2384 | load | 4 | `abs 0xa2db0` | `-` |
| `ldr` at 0xa2390 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2394 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa23a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa23a8 | load | 4 | `abs 0xa2db4` | `-` |
| `ldr` at 0xa23b4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa23b8 | load | 4 | `[sp-60] +0x5c` | `-` |
| `ldr` at 0xa23c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa23cc | load | 4 | `abs 0xa2db8` | `-` |
| `ldr` at 0xa23d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa23dc | load | 4 | `abs 0xa2dbc` | `-` |
| `ldr` at 0xa23e4 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa23e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa23fc | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2404 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2410 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2428 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2430 | load | 4 | `abs 0xa2dc0` | `-` |
| `ldr` at 0xa2434 | load | 4 | `abs 0x1a6924` | `-` |
| `ldr` at 0xa243c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2440 | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa2448 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2458 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2460 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2464 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa246c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa2474 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2484 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2488 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa248c | load | 4 | `abs 0xa2dc4` | `-` |
| `ldr` at 0xa2490 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa2498 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa24a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa24b0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa24b4 | load | 4 | `abs 0xa2dc8` | `-` |
| `ldr` at 0xa24b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa24bc | load | 4 | `abs 0xa2dcc` | `-` |
| `ldr` at 0xa24c0 | load | 4 | `abs 0x1a6630` | `-` |
| `ldr` at 0xa24c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa24d8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa24e0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa24e4 | load | 4 | `abs 0xa2dd0` | `-` |
| `ldr` at 0xa24e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa24ec | load | 4 | `abs 0xa2dd4` | `-` |
| `ldr` at 0xa24f0 | load | 4 | `abs 0x1a6634` | `-` |
| `ldr` at 0xa24f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2508 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2510 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2514 | load | 4 | `abs 0xa2dd8` | `-` |
| `ldr` at 0xa2518 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa251c | load | 4 | `abs 0xa2ddc` | `-` |
| `ldr` at 0xa2520 | load | 4 | `abs 0x1a6638` | `-` |
| `ldr` at 0xa2528 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2538 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2540 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa254c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa255c | load | 4 | `abs 0xa2de0` | `-` |
| `ldr` at 0xa2564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2568 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa256c | load | 4 | `abs 0x1a6984` | `-` |
| `str` at 0xa2598 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa259c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa25a0 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xa25a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa25a8 | load | 4 | `abs 0xa2de4` | `-` |
| `ldr` at 0xa25b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa25c0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa25c8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa25cc | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa25d0 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa25d4 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa25d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa25dc | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0xa25e0 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa25e4 | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0xa25ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa25f0 | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa25f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2608 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2610 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `str` at 0xa2618 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xa261c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2620 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa2624 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2630 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2648 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa264c | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa2650 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa265c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2660 | load | 4 | `abs 0xa2de8` | `-` |
| `ldr` at 0xa2664 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa266c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa267c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2684 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2688 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa268c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa26a8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa26b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa26bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa26c0 | load | 4 | `abs 0xa2dec` | `-` |
| `ldr` at 0xa26cc | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa26d0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa26ec | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa26f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2700 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2704 | load | 4 | `abs 0xa2df0` | `-` |
| `ldr` at 0xa2710 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2714 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2730 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa2738 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa273c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2758 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa2760 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2764 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2780 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa2788 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa278c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2790 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2794 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa279c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa27a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa27bc | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa27c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa27d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa27d4 | load | 4 | `abs 0xa2df4` | `-` |
| `ldr` at 0xa27e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa27e4 | load | 4 | `abs 0xa2df8` | `-` |
| `ldr` at 0xa27ec | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa27f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa280c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa2814 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2818 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2834 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa283c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2840 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa284c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2858 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa285c | load | 4 | `abs 0xa2df8` | `-` |
| `ldr` at 0xa2864 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2874 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa287c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2880 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2884 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa28a0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa28a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa28ac | load | 4 | `abs 0xa2dfc` | `-` |
| `ldr` at 0xa28b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa28b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa28c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa28d0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa28d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa28d8 | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa28e0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa28f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa28f8 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa28fc | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa2904 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa2914 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2924 | load | 4 | `abs 0xa2e00` | `-` |
| `ldr` at 0xa292c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa2930 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2934 | load | 4 | `abs 0x1a698c` | `-` |
| `str` at 0xa2954 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa2958 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa295c | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xa2960 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2964 | load | 4 | `abs 0xa2de4` | `-` |
| `ldr` at 0xa296c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa297c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2984 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2988 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa298c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2990 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2994 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2998 | load | 4 | `abs 0x1a6908` | `-` |
| `ldr` at 0xa299c | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa29a0 | load | 4 | `abs 0x1a690c` | `-` |
| `ldr` at 0xa29a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa29ac | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa29b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa29c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa29cc | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `str` at 0xa29d4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xa29d8 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa29dc | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa29e0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa29ec | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa29f0 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa29f4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2a00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2a04 | load | 4 | `abs 0xa2e64` | `-` |
| `ldr` at 0xa2a08 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2a10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2a20 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2a28 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `str` at 0xa2a30 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xa2a34 | load | 4 | `abs 0xa2e04` | `-` |
| `ldr` at 0xa2a38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa2a3c | load | 4 | `abs 0x1a0350` | `-` |
| `str` at 0xa2a4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa2a50 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2a54 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa2a58 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2a64 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2a68 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2a7c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2a84 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2a88 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa2a98 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2a9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2ab0 | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2ab8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2abc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa2acc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa2ad4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2ad8 | load | 4 | `abs 0xa2e08` | `-` |
| `ldr` at 0xa2ae0 | load | 4 | `abs 0xa2e0c` | `-` |
| `ldr` at 0xa2ae8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2af8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2b00 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2b04 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2b08 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2b1c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2b24 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2b28 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2b3c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2b44 | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2b48 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa2b5c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa2b64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2b68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2b6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2b70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa2b78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2b7c | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa2b84 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2b94 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2b9c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2ba0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa2ba8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa2bb0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xa2bb8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xa2bc0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa2bc8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa2bd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2bdc | load | 4 | `abs 0xa2e10` | `-` |
| `ldr` at 0xa2be8 | load | 4 | `abs 0xa2e28` | `-` |
| `str` at 0xa2bf4 | store | 4 | `abs 0x1a6644` | `ret(0x202c8)` |
| `ldr` at 0xa2bf8 | load | 4 | `abs 0xa2e30` | `-` |
| `str` at 0xa2c00 | store | 4 | `abs 0x1a664c` | `0x0` |
| `ldr` at 0xa2c04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa2c14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa2c24 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xa2c28 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa2c2c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa2c3c | load | 4 | `abs 0xa2e24` | `-` |
| `ldr` at 0xa2c40 | load | 4 | `abs 0x1a663c` | `-` |
| `str` at 0xa2c48 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa2c4c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2c50 | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa2c68 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2c6c | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa2ca0 | load | 4 | `abs 0xa2e38` | `-` |
| `ldr` at 0xa2cb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa2cbc | load | 4 | `abs 0xa2e38` | `-` |
| `str` at 0xa2ce8 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa2cec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2cf0 | load | 4 | `abs 0xa2e14` | `-` |
| `ldr` at 0xa2d00 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2d10 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa2d18 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa2d1c | load | 4 | `abs 0xa2d60` | `-` |
| `ldr` at 0xa2d20 | load | 4 | `abs 0x1a6970` | `-` |
| `ldr` at 0xa2d2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2d30 | load | 4 | `abs 0xa2e18` | `-` |
| `ldr` at 0xa2d3c | load | 4 | `abs 0xa2e20` | `-` |
| `ldr` at 0xa2d40 | load | 4 | `abs 0x1a6640` | `-` |
| `ldr` at 0xa2d4c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa2d50 | load | 4 | `abs 0xa2e1c` | `-` |
| `ldr` at 0xa2ea4 | load | 4 | `abs 0xa2e20` | `-` |
| `ldr` at 0xa2ea8 | load | 4 | `abs 0xa2e24` | `-` |
| `ldr` at 0xa2eac | load | 4 | `abs 0x1a6640` | `-` |
| `ldr` at 0xa2eb0 | load | 4 | `abs 0x1a663c` | `-` |
| `str` at 0xa2eb8 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xa2ec8 | load | 4 | `abs 0xa2e28` | `-` |
| `ldr` at 0xa2ecc | load | 4 | `abs 0x1a6644` | `-` |
| `str` at 0xa2ed4 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa2ed8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa2edc | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa2ef4 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa2ef8 | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa2f2c | load | 4 | `abs 0xa2e38` | `-` |
| `ldr` at 0xa2f44 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xa2f48 | load | 4 | `abs 0xa2e38` | `-` |
| `str` at 0xa2f74 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa2f78 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa2f7c | load | 4 | `abs 0xa2e34` | `-` |
| `str` at 0xa2f94 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xa2f98 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa2f9c | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa2fd0 | load | 4 | `abs 0xa2e38` | `-` |
| `str` at 0xa2fe8 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xa2fec | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xa2ff0 | load | 4 | `abs 0xa2e38` | `-` |
| `str` at 0xa301c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xa3020 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3024 | load | 4 | `abs 0xa2e2c` | `-` |
| `ldr` at 0xa3034 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3044 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa304c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa3064 | load | 4 | `abs 0xa2e30` | `-` |
| `ldr` at 0xa3068 | load | 4 | `abs 0x1a664c` | `-` |
| `ldr` at 0xa3080 | load | 4 | `abs 0xa2e30` | `-` |
| `ldr` at 0xa3084 | load | 4 | `abs 0x1a664c` | `-` |
| `str` at 0xa308c | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa3090 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa3094 | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa30ac | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa30b0 | load | 4 | `abs 0xa2e34` | `-` |
| `ldr` at 0xa30e4 | load | 4 | `abs 0xa2e38` | `-` |
| `ldr` at 0xa30fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa3100 | load | 4 | `abs 0xa2e38` | `-` |
| `str` at 0xa312c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xa3130 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3134 | load | 4 | `abs 0xa2e3c` | `-` |
| `ldr` at 0xa3148 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa314c | load | 4 | `abs 0xa2e40` | `-` |
| `ldr` at 0xa3158 | load | 4 | `abs 0xa2e44` | `-` |
| `ldr` at 0xa315c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3160 | load | 4 | `abs 0xa2e48` | `-` |
| `ldr` at 0xa3164 | load | 4 | `abs 0x1a0358` | `-` |
| `ldr` at 0xa316c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3170 | load | 4 | `abs 0xa2e4c` | `-` |
| `ldr` at 0xa317c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa318c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa319c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xa31a0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xa31a4 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa31a8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa31ac | load | 4 | `abs 0xa2e50` | `-` |
| `ldr` at 0xa31b0 | load | 4 | `abs 0xa2e54` | `-` |
| `ldr` at 0xa31bc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa31cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa31dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa31e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa31f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa31fc | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3200 | load | 4 | `abs 0x1a6998` | `-` |
| `str` at 0xa3220 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa3224 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa3228 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0xa3230 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xa3234 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3238 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa323c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3248 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa324c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3260 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa3268 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa326c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xa3280 | load | 4 | `abs 0x1a69a0` | `-` |
| `ldr` at 0xa3288 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa328c | load | 4 | `abs 0xa2e58` | `-` |
| `ldr` at 0xa3290 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3294 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa329c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa32ac | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa32b4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa32b8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xa32c0 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xa32c8 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa32cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa32d0 | load | 4 | `abs 0xa2e5c` | `-` |
| `ldr` at 0xa32d4 | load | 4 | `abs 0x1a6998` | `-` |
| `ldr` at 0xa32dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa32ec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa32f4 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa32fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa331c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa333c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xa3350 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xa3358 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xa335c | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3360 | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa3364 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3370 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3374 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3390 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3398 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa339c | load | 4 | `(unknown)` | `-` |
| `str` at 0xa33a4 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xa33a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa33b0 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa33b8 | load | 4 | `abs 0xa2e60` | `-` |
| `ldr` at 0xa33c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa33c4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xa33e4 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa33e8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa33ec | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `str` at 0xa33f4 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xa33f8 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa33fc | load | 4 | `abs 0x1a6984` | `-` |
| `ldr` at 0xa3400 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa340c | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3410 | load | 4 | `abs 0x1a6910` | `-` |
| `ldr` at 0xa3414 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3420 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3424 | load | 4 | `abs 0xa2e64` | `-` |
| `ldr` at 0xa3428 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3430 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3440 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa3448 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `str` at 0xa3450 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa3454 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3458 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3474 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa347c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3480 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa348c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3490 | load | 4 | `abs 0xa2e68` | `-` |
| `ldr` at 0xa3498 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa34a8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa34b0 | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa34b4 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa34b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa34d4 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa34dc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa34f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa34f8 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa34fc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3518 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3520 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3534 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa353c | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3540 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa355c | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa3578 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3580 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3584 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3588 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa358c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3594 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa35a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa35ac | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa35b0 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa35b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa35d0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa35d8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa35ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa35f4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3600 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3604 | load | 4 | `abs 0xa2e6c` | `-` |
| `ldr` at 0xa3610 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3614 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3630 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3638 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa364c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3654 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3660 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3664 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3680 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3688 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa369c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa36a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa36a8 | load | 4 | `abs 0xa2e70` | `-` |
| `ldr` at 0xa36ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa36b8 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa36bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa36d8 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa36e0 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa36e4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3700 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa371c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3724 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3728 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3738 | load | 4 | `abs 0xa2e74` | `-` |
| `ldr` at 0xa3744 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3754 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa375c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa3760 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3764 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3780 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3788 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa379c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa37a4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa37b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa37b4 | load | 4 | `abs 0xa2e78` | `-` |
| `ldr` at 0xa37c0 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa37c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa37e0 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa37e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa37fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3804 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3810 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3814 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xa3830 | load | 4 | `abs 0x1a6988` | `-` |
| `ldr` at 0xa3838 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xa384c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3854 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3858 | load | 4 | `abs 0xa2e7c` | `-` |
| `ldr` at 0xa385c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa3868 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa386c | load | 4 | `abs 0xa2e80` | `-` |
| `ldr` at 0xa3874 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3884 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa388c | store | 4 | `sp -0x20` | `([sp-32] + ret(0x2031c))` |
| `ldr` at 0xa3890 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa3898 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xa38a0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xa38a8 | store | 4 | `sp -0x30` | `[sp-48]+1` |
| `ldr` at 0xa38b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa38c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa38d8 | store | 4 | `[sp-24] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xa38dc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xa38e0 | store | 4 | `sp -0x20` | `[[sp-24]]` |
| `ldr` at 0xa38e4 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa38e8 | load | 4 | `abs 0x1a6974` | `-` |
| `ldr` at 0xa38f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa38f8 | load | 4 | `abs 0xa2e84` | `-` |
| `ldr` at 0xa3904 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3908 | load | 4 | `abs 0x1a6974` | `-` |
| `ldr` at 0xa3914 | load | 4 | `abs 0xa2e88` | `-` |
| `ldr` at 0xa391c | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3920 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3924 | load | 4 | `abs 0x1a6978` | `-` |
| `ldr` at 0xa3928 | load | 4 | `abs 0x1a697c` | `-` |
| `str` at 0xa3934 | store | 4 | `sp -0x3c` | `ret(0xa3c34)` |
| `ldr` at 0xa3938 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa3944 | load | 4 | `abs 0xa2e98` | `-` |
| `str` at 0xa3948 | store | 4 | `sp -0x40` | `0x1a6900` |
| `ldr` at 0xa394c | load | 4 | `abs 0xa2e98` | `-` |
| `str` at 0xa3950 | store | 4 | `sp -0x44` | `0x1a6900` |
| `ldr` at 0xa3954 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa3958 | load | 4 | `abs 0x1a6980` | `-` |
| `str` at 0xa395c | store | 4 | `sp -0x58` | `[0x1a6980]` |
| `ldr` at 0xa3960 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xa3964 | load | 4 | `ret(0xa3c34) +0xc` | `-` |
| `ldr` at 0xa3970 | load | 4 | `abs 0xa2e8c` | `-` |
| `str` at 0xa3974 | store | 4 | `sp -0x48` | `0x16b2b4` |
| `ldr` at 0xa397c | load | 4 | `abs 0xa2e90` | `-` |
| `str` at 0xa3980 | store | 4 | `sp -0x48` | `0x16b2b8` |
| `ldr` at 0xa3984 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xa3988 | store | 4 | `sp -0x54` | `[sp-72]` |
| `ldr` at 0xa398c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa3990 | load | 4 | `abs 0xa2e94` | `-` |
| `ldr` at 0xa3994 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xa3998 | load | 4 | `[sp-64] +0x78` | `-` |
| `ldr` at 0xa399c | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xa39a0 | load | 4 | `[sp-68] +0x7c` | `-` |
| `ldr` at 0xa39ac | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa39b0 | load | 4 | `abs 0xa2e98` | `-` |
| `ldr` at 0xa39b4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa39b8 | load | 4 | `abs 0xa2e9c` | `-` |
| `ldr` at 0xa39bc | load | 4 | `abs 0x1a6978` | `-` |
| `ldr` at 0xa39c0 | load | 4 | `abs 0x1a697c` | `-` |
| `ldr` at 0xa39d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa39d4 | load | 4 | `abs 0xa2ea0` | `-` |
| `ldm` at 0xa39e0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xa39e0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa39e0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa39e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa1de8 | `(unknown callee) 0x9adfc` | `0x16af98` | `r1` | `r2` | `?` |
| `bl` at 0xa1e10 | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a698c]` | `?` |
| `bl` at 0xa1ef4 | `(unknown callee) 0xa9384` | `?` | `0x1a6900` | `?` | `?` |
| `bl` at 0xa1f60 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16afa4` | `?` | `?` |
| `bl` at 0xa1f70 | `(unknown callee) 0x20634` | `[sp-32]` | `0x16afac` | `?` | `?` |
| `bl` at 0xa1f78 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2000 | `(unknown callee) 0x96878` | `([sp-32] + ret(0x2031c))` | `?` | `?` | `?` |
| `bl` at 0xa2008 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2054 | `(unknown callee) 0x20634` | `([sp-32] + ret(0x2031c))` | `0x16afb4` | `?` | `?` |
| `bl` at 0xa2088 | `(unknown callee) 0x96990` | `[0x1a6910]` | `?` | `?` | `?` |
| `bl` at 0xa209c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16afbc` | `ret(0x96990)` | `ret(0x96990)` |
| `bl` at 0xa20a4 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa20f8 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x16afc0` | `?` | `?` |
| `bl` at 0xa2100 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2170 | `(unknown callee) 0xa3a70` | `?` | `?` | `?` | `?` |
| `bl` at 0xa21b0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16afc4` | `?` | `?` |
| `bl` at 0xa2210 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16afd0` | `?` | `?` |
| `bl` at 0xa2220 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16afdc` | `?` | `?` |
| `bl` at 0xa2228 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2254 | `(unknown callee) 0x203c4` | `([sp-32] + ret(0x2031c))` | `0x16afe8` | `ret(0x2031c)` | `[sp-60]` |
| `bl` at 0xa2274 | `(unknown callee) 0x203c4` | `([sp-32] + ret(0x2031c))` | `0x16aff4` | `ret(0x2031c)` | `[[sp-60]+52]` |
| `bl` at 0xa2284 | `(unknown callee) 0x203c4` | `([sp-32] + ret(0x2031c))` | `0x16affc` | `ret(0x2031c)` | `[[sp-60]+52]` |
| `bl` at 0xa231c | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b034` | `?` | `[[sp-60]+56]` |
| `bl` at 0xa2350 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b03c` | `?` | `?` |
| `bl` at 0xa2388 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b048` | `?` | `?` |
| `bl` at 0xa23ac | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b054` | `?` | `?` |
| `bl` at 0xa23d0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b060` | `?` | `?` |
| `bl` at 0xa23e0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b06c` | `?` | `?` |
| `bl` at 0xa2414 | `(unknown callee) 0x2031c` | `[sp-32]` | `0x1a6900` | `?` | `?` |
| `bl` at 0xa2438 | `(unknown callee) 0x205bc` | `(ret(0x2031c) + [sp-32])` | `0x16b078` | `[0x1a6924]` | `(ret(0x2031c) + [sp-32])` |
| `bl` at 0xa2444 | `(unknown callee) 0x203c4` | `?` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa244c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2494 | `(unknown callee) 0x205bc` | `?` | `0x16b084` | `[0x1a698c]` | `0x1a6900` |
| `bl` at 0xa249c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa24c4 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x16b090` | `[0x1a6630]` | `0x1a6630` |
| `bl` at 0xa24cc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa24f4 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x16b0a8` | `[0x1a6634]` | `0x1a6634` |
| `bl` at 0xa24fc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2524 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x16b0c0` | `[0x1a6638]` | `0x1a6638` |
| `bl` at 0xa252c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2544 | `(unknown callee) 0x9afe8` | `?` | `?` | `?` | `?` |
| `bl` at 0xa2560 | `(unknown callee) 0x9adfc` | `0x16b0d8` | `r1` | `r2` | `?` |
| `bl` at 0xa2590 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xa25ac | `(unknown callee) 0x205bc` | `[[sp-24]]` | `0x16b0e4` | `?` | `[[sp-24]]` |
| `bl` at 0xa25b4 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa25e8 | `(unknown callee) 0x96878` | `([sp-32] + ret(0x2031c))` | `[0x1a6908]` | `[0x1a6910]` | `[0x1a690c]` |
| `bl` at 0xa25f4 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa25fc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2634 | `(unknown callee) 0xa9384` | `?` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa2654 | `(unknown callee) 0x96990` | `[0x1a6910]` | `[sp-48]` | `?` | `0x1a6900` |
| `bl` at 0xa2668 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b0f0` | `[sp-48]` | `ret(0x96990)` |
| `bl` at 0xa2670 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa26c4 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b0fc` | `?` | `?` |
| `bl` at 0xa2708 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b10c` | `?` | `?` |
| `bl` at 0xa2798 | `(unknown callee) 0x96878` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa27d8 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b128` | `?` | `?` |
| `bl` at 0xa27e8 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b130` | `?` | `?` |
| `bl` at 0xa2844 | `(unknown callee) 0x96990` | `?` | `?` | `?` | `?` |
| `bl` at 0xa2854 | `(unknown callee) 0x203c4` | `[sp-32]` | `ret(0x96990)` | `?` | `ret(0x96990)` |
| `bl` at 0xa2860 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16b130` | `?` | `?` |
| `bl` at 0xa2868 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa28b4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b134` | `?` | `?` |
| `bl` at 0xa28bc | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa28dc | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa28e4 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa290c | `(unknown callee) 0x9afe8` | `?` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa2928 | `(unknown callee) 0x9adfc` | `0x16b140` | `r1` | `r2` | `?` |
| `bl` at 0xa294c | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a698c]` | `?` |
| `bl` at 0xa2968 | `(unknown callee) 0x205bc` | `[[sp-24]]` | `0x16b0e4` | `?` | `[[sp-24]]` |
| `bl` at 0xa2970 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa29a4 | `(unknown callee) 0x96878` | `([sp-32] + ret(0x2031c))` | `[0x1a6908]` | `[0x1a6910]` | `[0x1a690c]` |
| `bl` at 0xa29b0 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa29b8 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa29f8 | `(unknown callee) 0x96990` | `[0x1a6910]` | `?` | `[0x1a6984]` | `0x1a6900` |
| `bl` at 0xa2a0c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b24c` | `[sp-48]` | `ret(0x96990)` |
| `bl` at 0xa2a14 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2ae4 | `(unknown callee) 0x205bc` | `([sp-32] + ret(0x2031c))` | `0x16b14c` | `?` | `0x16b150` |
| `bl` at 0xa2aec | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2b74 | `(unknown callee) 0x96878` | `([sp-32] + ret(0x2031c))` | `?` | `?` | `?` |
| `bl` at 0xa2b80 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa2b88 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2bd0 | `(unknown callee) 0x9afe8` | `ret(0x2031c)` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa2bf0 | `(unknown callee) 0x202c8` | `0x0` | `r1` | `r1` | `0x1010` |
| `bl` at 0xa2c1c | `(unknown callee) 0x201fc` | `0xc8` | `?` | `?` | `?` |
| `bl` at 0xa2c34 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `?` |
| `bl` at 0xa2cfc | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b154` | `?` | `?` |
| `bl` at 0xa2d04 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa2d34 | `(unknown callee) 0x20634` | `([sp-32] + ret(0x2031c))` | `0x16b16c` | `ret(0x2031c)` | `[0x1a6970]` |
| `bl` at 0xa2d54 | `(unknown callee) 0x20634` | `([sp-32] + ret(0x2031c))` | `0x16b184` | `ret(0x2031c)` | `[0x1a6640]` |
| `bl` at 0xa2ec0 | `(unknown callee) 0x202c8` | `0x0` | `[0x1a6640]` | `0x1a663c` | `?` |
| `bl` at 0xa3030 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b19c` | `?` | `?` |
| `bl` at 0xa3038 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3054 | `(unknown callee) 0x28230` | `0x9a` | `?` | `ret(0x2031c)` | `([sp-32] + ret(0x2031c))` |
| `bl` at 0xa3078 | `(unknown callee) 0x202c8` | `0x0` | `?` | `?` | `[0x1a664c]` |
| `bl` at 0xa3140 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b1c8` | `?` | `?` |
| `bl` at 0xa3150 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b1e0` | `?` | `[0x1a664c]` |
| `bl` at 0xa3168 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b1f4` | `[0x1a0358]` | `0x1a0358` |
| `bl` at 0xa3174 | `(unknown callee) 0x203c4` | `[sp-32]` | `0x16a8d8` | `?` | `?` |
| `bl` at 0xa3194 | `(unknown callee) 0x201fc` | `0x64` | `?` | `?` | `?` |
| `bl` at 0xa31b4 | `(unknown callee) 0x205bc` | `?` | `0x16b210` | `0x31b40c` | `?` |
| `bl` at 0xa31c8 | `(unknown callee) 0x9c1bc` | `?` | `?` | `?` | `?` |
| `bl` at 0xa31e0 | `(unknown callee) 0x9c23c` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0xa3218 | `(unknown callee) 0x201fc` | `?` | `?` | `[0x1a6998]` | `?` |
| `bl` at 0xa3298 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b228` | `?` | `?` |
| `bl` at 0xa32a0 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa32d8 | `(unknown callee) 0x205bc` | `?` | `0x16b234` | `[0x1a6998]` | `0x1a6900` |
| `bl` at 0xa32e0 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3314 | `(unknown callee) 0x2624c` | `0x9a` | `0x0` | `?` | `?` |
| `bl` at 0xa3334 | `(unknown callee) 0x2624c` | `0x9a` | `0x1` | `?` | `?` |
| `bl` at 0xa33bc | `(unknown callee) 0x9adfc` | `0x16b240` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa33dc | `(unknown callee) 0x201fc` | `?` | `?` | `[sp-60]` | `?` |
| `bl` at 0xa3418 | `(unknown callee) 0x96990` | `[0x1a6910]` | `?` | `[0x1a6984]` | `0x1a6900` |
| `bl` at 0xa342c | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b24c` | `[sp-48]` | `ret(0x96990)` |
| `bl` at 0xa3434 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3494 | `(unknown callee) 0x20634` | `[sp-32]` | `0x16b258` | `?` | `[sp-36]` |
| `bl` at 0xa349c | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3590 | `(unknown callee) 0x96878` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3598 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa3608 | `(unknown callee) 0x20634` | `[sp-32]` | `0x16b260` | `?` | `?` |
| `bl` at 0xa36b0 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b268` | `?` | `?` |
| `bl` at 0xa372c | `(unknown callee) 0x96990` | `?` | `?` | `?` | `?` |
| `bl` at 0xa3740 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b274` | `ret(0x96990)` | `ret(0x96990)` |
| `bl` at 0xa3748 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa37b8 | `(unknown callee) 0x20634` | `[sp-32]` | `0x16b280` | `?` | `?` |
| `bl` at 0xa3860 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b288` | `?` | `?` |
| `bl` at 0xa3870 | `(unknown callee) 0x20634` | `[sp-32]` | `0x16b290` | `?` | `?` |
| `bl` at 0xa3878 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xa38b0 | `(unknown callee) 0x9afe8` | `?` | `?` | `[0x1a6984]` | `?` |
| `bl` at 0xa38d0 | `(unknown callee) 0x201fc` | `0x50` | `?` | `?` | `?` |
| `bl` at 0xa38fc | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b29c` | `?` | `[0x1a6974]` |
| `bl` at 0xa3918 | `(unknown callee) 0x9adfc` | `0x16b2a8` | `?` | `?` | `[0x1a6974]` |
| `bl` at 0xa392c | `(unknown callee) 0xa3c34` | `[0x1a6978]` | `[0x1a697c]` | `0x1a6900` | `0x1a6900` |
| `bl` at 0xa39a4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b2c4` | `[[sp-64]+120]` | `[[sp-68]+124]` |
| `bl` at 0xa39c4 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b2e4` | `[0x1a6978]` | `[0x1a697c]` |
| `bl` at 0xa39c8 | `(unknown callee) 0x9afe8` | `ret(0x205bc)` | `?` | `?` | `?` |
| `bl` at 0xa39d8 | `(unknown callee) 0x205bc` | `[sp-32]` | `0x16b2fc` | `?` | `[0x1a6974]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa1dcc | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xa1ddc | `? != 0x0` | `? == 0x0` |
| `ble` at 0xa1e3c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bls` at 0xa1eb8 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xa1ec8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa1f00 | `ret(0xa9384) != 0x0` | `ret(0xa9384) == 0x0` |
| `beq` at 0xa1f30 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xa2048 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa20c8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2124 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa21a4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa21e0 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa2248 | `[sp-60] != 0x0` | `[sp-60] == 0x0` |
| `beq` at 0xa2268 | `[[sp-60]+52] == 0x0` | `[[sp-60]+52] != 0x0` |
| `ldrls` at 0xa2294 | `[[sp-60]+56] <= 0x5 (unsigned, 32-bit)` | `[[sp-60]+56] > 0x5 (unsigned, 32-bit)` |
| `beq` at 0xa2330 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa2344 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa2368 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa237c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa23a0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa23c4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa240c | `? != 0x0` | `? == 0x0` |
| `beq` at 0xa2480 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2558 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xa262c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xa2640 | `ret(0xa9384) != 0x0` | `ret(0xa9384) == 0x0` |
| `bne` at 0xa26b8 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa26fc | `? != 0x0` | `? == 0x0` |
| `bne` at 0xa27cc | `? != 0x1` | `? == 0x1` |
| `beq` at 0xa2920 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xa29e8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bge` at 0xa2a44 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xa2a60 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa2a90 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xa2ac4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xa2be4 | `r1 != 0x1010` | `r1 == 0x1010` |
| `beq` at 0xa2c10 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa2d28 | `[0x1a6970] == 0x0` | `[0x1a6970] != 0x0` |
| `bne` at 0xa2d48 | `[0x1a6640] != 0x0` | `[0x1a6640] == 0x0` |
| `beq` at 0xa3060 | `ret(0x28230) == 0x0` | `ret(0x28230) != 0x0` |
| `beq` at 0xa3070 | `[0x1a664c] == 0x0` | `[0x1a664c] != 0x0` |
| `beq` at 0xa3188 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa31c4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa31d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa31f4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xa3244 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xa3308 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa3328 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa3348 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xa336c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xa3408 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xa3488 | `? <= [sp-36] (signed, 32-bit)` | `? > [sp-36] (signed, 32-bit)` |
| `bne` at 0xa35fc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa365c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xa37ac | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa380c | `? == 0x0` | `? != 0x0` |
| `beq` at 0xa38c4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xa38f0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xa3910 | `[0x1a6974] == 0x0` | `[0x1a6974] != 0x0` |
| `beq` at 0xa3940 | `ret(0xa3c34) == 0x0` | `ret(0xa3c34) != 0x0` |
| `bne` at 0xa396c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa39e0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa2294).

## `L1CfgGetBrickType`

- `L1CfgGetBrickType`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa39e4 to 0xa3a70.
  Established. The call to unnamed 0x9e6d0 has printed candidates the loaded values at 0x1a6994 and 0x1a698c followed by the words at entry sp minus 20 and minus 24 (`bl` at 0xa3a1c);
  its result is saved at entry sp minus 0x1c (`bl` at 0xa3a1c, `str` at 0xa3a24).
  A word loaded from that result plus eight is saved at entry sp minus 0x24 (`bl` at 0xa3a1c, `ldr` at 0xa3a38, `str` at 0xa3a3c).
  An alternative store writes 0x3f at entry sp minus 0x24 (`str` at 0xa3a48).
  The final r0 value is the word at entry sp minus 32, whose physical meaning is unknown (`ldm` at 0xa3a64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa39f4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa39f8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa39fc | load | 4 | `abs 0xa3a68` | `-` |
| `ldr` at 0xa3a04 | load | 4 | `abs 0xa3a6c` | `-` |
| `ldr` at 0xa3a08 | load | 4 | `abs 0xa3a6c` | `-` |
| `ldr` at 0xa3a0c | load | 4 | `abs 0x1a6994` | `-` |
| `ldr` at 0xa3a10 | load | 4 | `abs 0x1a698c` | `-` |
| `ldr` at 0xa3a14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa3a18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xa3a24 | store | 4 | `sp -0x1c` | `ret(0x9e6d0)` |
| `ldr` at 0xa3a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3a34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa3a38 | load | 4 | `ret(0x9e6d0) +0x8` | `-` |
| `str` at 0xa3a3c | store | 4 | `sp -0x24` | `[ret(0x9e6d0)+8]` |
| `str` at 0xa3a48 | store | 4 | `sp -0x24` | `0x3f` |
| `ldr` at 0xa3a4c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa3a50 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xa3a58 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xa3a64 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xa3a64 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xa3a64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa3a00 | `(unknown callee) 0x9adfc` | `0x16b308` | `r1` | `r2` | `r3` |
| `bl` at 0xa3a1c | `(unknown callee) 0x9e6d0` | `[0x1a6994]` | `[0x1a698c]` | `[sp-20]` | `[sp-24]` |
| `bl` at 0xa3a54 | `(unknown callee) 0x9afe8` | `ret(0x9e6d0)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa3a30 | `ret(0x9e6d0) == 0x0` | `ret(0x9e6d0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xa3a64 | `r0` | `[sp-32]` |
