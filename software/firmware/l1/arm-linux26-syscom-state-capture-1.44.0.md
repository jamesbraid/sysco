# ARM Linux 2.6 communication state and captures in 1.44.0

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
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory accesses reported on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
A computed transfer in SysComConfig leaves unresolved edges; its tables cover known edges only.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-syscom-state-capture-facts.txt) retain all tool sections.

## `SysComConfig`

- `SysComConfig`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7358 to 0xb7aa0.
  Established. Known-edge word stores write call results to absolute 0x1b0ce0, 0x31b5b0 and 0x31b5b8 (`str` at 0xb73b8, `str` at 0xb73c4, `str` at 0xb73e0).
  Other word stores write 0x3e8 to 0x31b5bc and 3, 0x50 and 5 to 0x31b61c, 0x31b620 and 0x31b624 (`str` at 0xb7414, `str` at 0xb7434, `str` at 0xb7440, `str` at 0xb744c).
  The call to unnamed 0xfb5cc prints candidate r0/r1/r2/r3 values 0x31b5a0/5/0/1 (`bl` at 0xb7468).
  The computed transfer is unresolved (`ldrls` at 0xb7480), so effects on paths not reached by known edges and their hardware selection remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xb7368 | load | 4 | `abs 0xb7a1c` | `-` |
| `ldr` at 0xb736c | load | 4 | `abs 0x1b0c88` | `-` |
| `ldr` at 0xb7378 | load | 4 | `abs 0xb7a20` | `-` |
| `str` at 0xb737c | store | 4 | `sp -0x24` | `0x16d36c` |
| `ldr` at 0xb7384 | load | 4 | `abs 0xb7a24` | `-` |
| `ldr` at 0xb7388 | load | 4 | `abs 0xb7a28` | `-` |
| `ldr` at 0xb7394 | load | 4 | `abs 0xb7a1c` | `-` |
| `ldr` at 0xb7398 | load | 4 | `abs 0xb7a1c` | `-` |
| `ldr` at 0xb739c | load | 4 | `abs 0x1b0c88` | `-` |
| `str` at 0xb73a4 | store | 4 | `abs 0x1b0c88` | `[0x1b0c88]+1` |
| `ldr` at 0xb73b4 | load | 4 | `abs 0xb7a2c` | `-` |
| `str` at 0xb73b8 | store | 4 | `abs 0x1b0ce0` | `ret(0xa3c7c)` |
| `ldr` at 0xb73bc | load | 4 | `abs 0xb7a30` | `-` |
| `str` at 0xb73c4 | store | 4 | `abs 0x31b5b0` | `ret(0x6e140)` |
| `ldr` at 0xb73c8 | load | 4 | `abs 0xb7a30` | `-` |
| `ldr` at 0xb73cc | load | 4 | `abs 0xb7a30` | `-` |
| `ldr` at 0xb73d0 | load | 4 | `abs 0x31b5b0` | `-` |
| `str` at 0xb73e0 | store | 4 | `abs 0x31b5b8` | `ret(0x204d8)` |
| `ldr` at 0xb73e4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb73f0 | load | 4 | `abs 0xb7a34` | `-` |
| `str` at 0xb73f4 | store | 4 | `sp -0x24` | `0x16d398` |
| `ldr` at 0xb73fc | load | 4 | `abs 0xb7a24` | `-` |
| `ldr` at 0xb740c | load | 4 | `abs 0xb7a30` | `-` |
| `str` at 0xb7414 | store | 4 | `abs 0x31b5bc` | `0x3e8` |
| `ldr` at 0xb7418 | load | 4 | `abs 0xb7a38` | `-` |
| `ldr` at 0xb741c | load | 4 | `abs 0xb7a3c` | `-` |
| `ldr` at 0xb742c | load | 4 | `abs 0xb7a38` | `-` |
| `str` at 0xb7434 | store | 4 | `abs 0x31b61c` | `0x3` |
| `ldr` at 0xb7438 | load | 4 | `abs 0xb7a38` | `-` |
| `str` at 0xb7440 | store | 4 | `abs 0x31b620` | `0x50` |
| `ldr` at 0xb7444 | load | 4 | `abs 0xb7a38` | `-` |
| `str` at 0xb744c | store | 4 | `abs 0x31b624` | `0x5` |
| `ldr` at 0xb7450 | load | 4 | `abs 0xb7a40` | `-` |
| `str` at 0xb7458 | store | 4 | `abs 0x1b0c8c` | `0x1` |
| `ldr` at 0xb745c | load | 4 | `abs 0xb7a30` | `-` |
| `ldrls` at 0xb7480 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xb7a18 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7a18 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7a18 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7a18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7390 | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0x26d` | `0x0` |
| `bl` at 0xb73ac | `(unknown callee) 0xa3c7c` | `0x0` | `?` | `0x1b0c88` | `[0x1b0c88]+1` |
| `bl` at 0xb73c0 | `(unknown callee) 0x6e140` | `ret(0xa3c7c)` | `?` | `ret(0xa3c7c)` | `0x1b0ce0` |
| `bl` at 0xb73d8 | `(unknown callee) 0x204d8` | `[0x31b5b0]` | `0x10` | `?` | `0x31b5a0` |
| `bl` at 0xb7408 | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0x27c` | `0x0` |
| `bl` at 0xb7428 | `(unknown callee) 0x22b84` | `0x31b5c0` | `0x16d3bc` | `0x1` | `0x9` |
| `bl` at 0xb7468 | `(unknown callee) 0xfb5cc` | `0x31b5a0` | `0x5` | `0x0` | `0x1` |
| `bl` at 0xb746c | `(unknown callee) 0x6dc0c` | `ret(0xfb5cc)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb7374 | `[0x1b0c88] <= 0x0 (signed, 32-bit)` | `[0x1b0c88] > 0x0 (signed, 32-bit)` |
| `bne` at 0xb73ec | `[0x31b5b8] != 0x0` | `[0x31b5b8] == 0x0` |
| `ldrls` at 0xb7480 | `? <= 0x59 (unsigned, 32-bit)` | `? > 0x59 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7a18 | `r0` | `ret(0x6dc0c)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xb7480).

## `SysComResetSlab`

- `SysComResetSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5e80 to 0xb6078.
  Established. Word loads read absolute 0x31b5b8, while later computed loads have unknown effective addresses (`ldr` at 0xb5ea4, `ldr` at 0xb5ed8, `ldr` at 0xb5f78).
  Two calls to unnamed 0x100354 and one to unnamed 0x10f7f8 each print candidate r1=3 with r0/r2/r3 unknown (`bl` at 0xb5f34, `bl` at 0xb601c, `bl` at 0xb5fcc).
  The final call to unnamed 0x1012c0 prints candidate r0/r1/r2/r3 values unknown/[sp-24]/unknown/unknown (`bl` at 0xb6068).
  These unknown addresses and arguments leave the reset hardware resources unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5e90 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5e94 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb5e98 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb5e9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5ea4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb5eac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5ed0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5ed8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5ee8 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb5eec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5ef4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb5efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5f20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5f38 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb5f3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5f44 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb5f4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5f70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5f78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5f84 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb5f88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5f90 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb5f98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5fbc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5fc4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb5fd0 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb5fd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5fdc | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb5fe4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6008 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6020 | load | 4 | `abs 0xb6074` | `-` |
| `ldr` at 0xb6024 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb602c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb6034 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6058 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xb6070 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb6070 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb6070 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5f34 | `(unknown callee) 0x100354` | `?` | `0x3` | `?` | `?` |
| `bl` at 0xb5fcc | `(unknown callee) 0x10f7f8` | `?` | `0x3` | `?` | `?` |
| `bl` at 0xb601c | `(unknown callee) 0x100354` | `?` | `0x3` | `?` | `?` |
| `bl` at 0xb6068 | `(unknown callee) 0x1012c0` | `?` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb5ee0 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xb5f80 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb6070 | `r0` | `?` |

## `SysComReset`

- `SysComReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6078 to 0xb6148.
  Established. One path stores [0x31b5b0] at sp-0x20, while another stores [sp-20]+1 there (`str` at 0xb60a8, `str` at 0xb60c0).
  The call to unnamed 0xb82a4 prints r0/r1/r2 unknown and candidate r3=[sp-32], while the call to unnamed 0xb5e80 prints candidate r0=[sp-20] with the remaining registers unknown (`bl` at 0xb60e0, `bl` at 0xb6118).
  Backward transfers return to 0xb60ec and 0xb60cc, and the corresponding loop-limit predicates have unknown operands (`b` at 0xb6128, `b` at 0xb6138, `bge` at 0xb60d8, `ble` at 0xb610c).
  The covered reset targets therefore remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6088 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb608c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb609c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb60a0 | load | 4 | `abs 0xb6144` | `-` |
| `ldr` at 0xb60a4 | load | 4 | `abs 0x31b5b0` | `-` |
| `str` at 0xb60a8 | store | 4 | `sp -0x20` | `[0x31b5b0]` |
| `ldr` at 0xb60b0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb60b4 | store | 4 | `sp -0x1c` | `[sp-20]` |
| `ldr` at 0xb60b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb60c0 | store | 4 | `sp -0x20` | `[sp-20]+1` |
| `ldr` at 0xb60c4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb60c8 | store | 4 | `sp -0x14` | `[sp-28]` |
| `ldr` at 0xb60cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb60d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb60dc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb60e8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb60ec | load | 4 | `abs 0xb6144` | `-` |
| `ldr` at 0xb60f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb60f8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb6100 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6104 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6110 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6114 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb611c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6124 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb612c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6134 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldm` at 0xb6140 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb6140 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb6140 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb60e0 | `(unknown callee) 0xb82a4` | `?` | `?` | `?` | `[sp-32]` |
| `bl` at 0xb6118 | `(unknown callee) 0xb5e80` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb6094 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xb60d8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `ble` at 0xb610c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb6140 | `r0` | `?` |

## `SysComThrottle`

- `SysComThrottle`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6148 to 0xb6348.
  Established. A signed comparison takes the branch to 0xb6274 when [0x31b620] exceeds ret(0x23f8c), and falls through otherwise (`bgt` at 0xb61b8).
  Word stores write [sp-20] to absolute 0x31b614 and [0x31b618]+1 to absolute 0x31b618 (`str` at 0xb6218, `str` at 0xb622c).
  On a later path, equality between [0x31b614] and [sp-20] falls through to a store of 0 at 0x31b614 (`bne` at 0xb62dc, `str` at 0xb62e8).
  The fields are resolved as addresses, but their hardware control meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6158 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb615c | load | 4 | `abs 0xb6334` | `-` |
| `ldr` at 0xb6160 | load | 4 | `abs 0x1b0c8c` | `-` |
| `ldr` at 0xb6170 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6174 | load | 4 | `abs 0x31b614` | `-` |
| `ldr` at 0xb6180 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6184 | load | 4 | `abs 0x31b614` | `-` |
| `ldr` at 0xb6188 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb61a4 | store | 4 | `sp -0x1c` | `ret(0x23f8c)` |
| `ldr` at 0xb61a8 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb61ac | load | 4 | `abs 0x31b620` | `-` |
| `ldr` at 0xb61b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb61bc | load | 4 | `abs 0xb6338` | `-` |
| `str` at 0xb61cc | store | 4 | `sp -0x18` | `ret(0x22ca0)` |
| `ldr` at 0xb61d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb61dc | load | 4 | `abs 0xb633c` | `-` |
| `str` at 0xb61e0 | store | 4 | `sp -0x24` | `0x16cf78` |
| `ldr` at 0xb61e4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb61e8 | store | 4 | `sp -0x20` | `ret(0x22ca0)` |
| `ldr` at 0xb61f0 | load | 4 | `abs 0xb6340` | `-` |
| `ldr` at 0xb6200 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6204 | load | 4 | `abs 0x31b614` | `-` |
| `ldr` at 0xb6210 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6214 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6218 | store | 4 | `abs 0x31b614` | `[sp-20]` |
| `ldr` at 0xb621c | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6220 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6224 | load | 4 | `abs 0x31b618` | `-` |
| `str` at 0xb622c | store | 4 | `abs 0x31b618` | `[0x31b618]+1` |
| `ldr` at 0xb6230 | load | 4 | `abs 0xb6338` | `-` |
| `str` at 0xb623c | store | 4 | `sp -0x18` | `ret(0x2302c)` |
| `ldr` at 0xb6240 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb624c | load | 4 | `abs 0xb6344` | `-` |
| `str` at 0xb6250 | store | 4 | `sp -0x24` | `0x16cfbc` |
| `ldr` at 0xb6254 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6258 | store | 4 | `sp -0x20` | `ret(0x2302c)` |
| `ldr` at 0xb6260 | load | 4 | `abs 0xb6340` | `-` |
| `ldr` at 0xb6274 | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb6278 | load | 4 | `abs 0x31b624` | `-` |
| `ldr` at 0xb627c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6288 | load | 4 | `abs 0xb6338` | `-` |
| `str` at 0xb6298 | store | 4 | `sp -0x18` | `ret(0x22ca0)` |
| `ldr` at 0xb629c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb62a8 | load | 4 | `abs 0xb633c` | `-` |
| `str` at 0xb62ac | store | 4 | `sp -0x24` | `0x16cf78` |
| `ldr` at 0xb62b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb62b4 | store | 4 | `sp -0x20` | `ret(0x22ca0)` |
| `ldr` at 0xb62bc | load | 4 | `abs 0xb6340` | `-` |
| `ldr` at 0xb62cc | load | 4 | `abs 0xb6338` | `-` |
| `ldr` at 0xb62d0 | load | 4 | `abs 0x31b614` | `-` |
| `ldr` at 0xb62d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb62e0 | load | 4 | `abs 0xb6338` | `-` |
| `str` at 0xb62e8 | store | 4 | `abs 0x31b614` | `0x0` |
| `ldr` at 0xb62ec | load | 4 | `abs 0xb6338` | `-` |
| `str` at 0xb62f8 | store | 4 | `sp -0x18` | `ret(0x2302c)` |
| `ldr` at 0xb62fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6308 | load | 4 | `abs 0xb6344` | `-` |
| `str` at 0xb630c | store | 4 | `sp -0x24` | `0x16cfbc` |
| `ldr` at 0xb6310 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6314 | store | 4 | `sp -0x20` | `ret(0x2302c)` |
| `ldr` at 0xb631c | load | 4 | `abs 0xb6340` | `-` |
| `ldm` at 0xb6330 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb6330 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb6330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb619c | `(unknown callee) 0x23f8c` | `r0` | `r1` | `?` | `?` |
| `bl` at 0xb61c4 | `(unknown callee) 0x22ca0` | `0x31b5c0` | `?` | `[0x31b620]` | `ret(0x23f8c)` |
| `bl` at 0xb61fc | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0xe3` | `0x0` |
| `bl` at 0xb6234 | `(unknown callee) 0x2302c` | `0x31b5c0` | `?` | `?` | `?` |
| `bl` at 0xb626c | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0xeb` | `0x0` |
| `bl` at 0xb6290 | `(unknown callee) 0x22ca0` | `0x31b5c0` | `?` | `[0x31b624]` | `ret(0x23f8c)` |
| `bl` at 0xb62c8 | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0xf0` | `0x0` |
| `bl` at 0xb62f0 | `(unknown callee) 0x2302c` | `0x31b5c0` | `?` | `?` | `?` |
| `bl` at 0xb6328 | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0xf6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb6168 | `[0x1b0c8c] != 0x0` | `[0x1b0c8c] == 0x0` |
| `beq` at 0xb617c | `[0x31b614] == 0x0` | `[0x31b614] != 0x0` |
| `beq` at 0xb6190 | `[0x31b614] == r0` | `[0x31b614] != r0` |
| `bgt` at 0xb61b8 | `[0x31b620] > ret(0x23f8c) (signed, 32-bit)` | `[0x31b620] <= ret(0x23f8c) (signed, 32-bit)` |
| `beq` at 0xb61d8 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `bne` at 0xb620c | `[0x31b614] != 0x0` | `[0x31b614] == 0x0` |
| `beq` at 0xb6248 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |
| `ble` at 0xb6284 | `[0x31b624] <= ret(0x23f8c) (signed, 32-bit)` | `[0x31b624] > ret(0x23f8c) (signed, 32-bit)` |
| `beq` at 0xb62a4 | `ret(0x22ca0) == 0x0` | `ret(0x22ca0) != 0x0` |
| `bne` at 0xb62dc | `[0x31b614] != [sp-20]` | `[0x31b614] == [sp-20]` |
| `beq` at 0xb6304 | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb6330 | `r0` | `?` |

## `SysComEventEnabled`

- `SysComEventEnabled`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb7d8 to 0xbb818.
  Established. A word load reads absolute 0x31b5a8 (`ldr` at 0xbb7ec).
  The branch takes the path to 0xbb804 when Z is clear, reaching a word store of 1 at sp-0x14, while the other path stores 0 (`bne` at 0xbb7f4, `str` at 0xbb808, `str` at 0xbb7fc).
  No calls are emitted and the recorded r0 return is [sp-20], but unsupported cmn effects leave its relation to the loaded value Unknown (`cmn` at 0xbb7f0, `ldm` at 0xbb810).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbb7e8 | load | 4 | `abs 0xbb814` | `-` |
| `ldr` at 0xbb7ec | load | 4 | `abs 0x31b5a8` | `-` |
| `str` at 0xbb7fc | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0xbb808 | store | 4 | `sp -0x14` | `0x1` |
| `ldr` at 0xbb80c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbb810 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xbb810 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb810 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb810 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbb7f4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb810 | `r0` | `[sp-20]` |

## `SysComEventDisable`

- `SysComEventDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb818 to 0xbb844.
  Established. A word store writes an unknown value to absolute 0x31b5a8 after unsupported mvn effects (`mvn` at 0xbb828, `str` at 0xbb82c).
  The sole call to unnamed 0x2624c prints candidate r0/r1/r2/r3 values 0x13/0/0x31b5a0/unknown (`bl` at 0xbb838);
  its result is the recorded r0 return, while the device-level effect of that call remains Unknown (`ldm` at 0xbb83c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xbb824 | load | 4 | `abs 0xbb840` | `-` |
| `str` at 0xbb82c | store | 4 | `abs 0x31b5a8` | `?` |
| `ldm` at 0xbb83c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xbb83c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xbb83c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb838 | `(unknown callee) 0x2624c` | `0x13` | `0x0` | `0x31b5a0` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xbb83c | `r0` | `ret(0x2624c)` |

## `SysComLogInit_2m`

- `SysComLogInit_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xee9d4 to 0xeeb4c.
  Established. When [0x31b5b0] is 0, the fall-through path reaches word stores of 0 to absolute 0x1b0cc4, 0x1b0cc8 and 0x1b0ccc (`bne` at 0xee9f0, `str` at 0xee9fc, `str` at 0xeea08, `str` at 0xeea14).
  A later call to unnamed 0x204d8 prints candidate r0/r1 values [0x1b0f58]/4, and its result is stored at absolute 0x1b0f5c (`bl` at 0xeea70, `str` at 0xeea78).
  The call to unnamed 0x51b28 prints candidate r0/r1 values 0x2000/1, followed by a store of its result at an unknown effective address (`bl` at 0xeeadc, `str` at 0xeeaf8).
  The returned objects and hardware relationship remain unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xee9e4 | load | 4 | `abs 0xeeb2c` | `-` |
| `ldr` at 0xee9e8 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xee9f4 | load | 4 | `abs 0xeeb30` | `-` |
| `str` at 0xee9fc | store | 4 | `abs 0x1b0cc4` | `0x0` |
| `ldr` at 0xeea00 | load | 4 | `abs 0xeeb34` | `-` |
| `str` at 0xeea08 | store | 4 | `abs 0x1b0cc8` | `0x0` |
| `ldr` at 0xeea0c | load | 4 | `abs 0xeeb38` | `-` |
| `str` at 0xeea14 | store | 4 | `abs 0x1b0ccc` | `0x0` |
| `ldr` at 0xeea1c | load | 4 | `abs 0xeeb3c` | `-` |
| `str` at 0xeea20 | store | 4 | `sp -0x1c` | `0x1b0f58` |
| `ldr` at 0xeea24 | load | 4 | `abs 0xeeb2c` | `-` |
| `ldr` at 0xeea28 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xeea34 | load | 4 | `abs 0xeeb2c` | `-` |
| `ldr` at 0xeea38 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xeea3c | load | 4 | `[0x31b5b8] +0x4` | `-` |
| `str` at 0xeea40 | store | 4 | `sp -0x20` | `[[0x31b5b8]+4]` |
| `ldr` at 0xeea48 | load | 4 | `abs 0xeeb2c` | `-` |
| `ldr` at 0xeea4c | load | 4 | `abs 0x31b5b0` | `-` |
| `str` at 0xeea50 | store | 4 | `sp -0x20` | `[0x31b5b0]` |
| `ldr` at 0xeea54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeea58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeea5c | store | 4 | `abs 0x1b0f58` | `?` |
| `ldr` at 0xeea60 | load | 4 | `abs 0xeeb40` | `-` |
| `ldr` at 0xeea64 | load | 4 | `abs 0xeeb3c` | `-` |
| `ldr` at 0xeea68 | load | 4 | `abs 0x1b0f58` | `-` |
| `str` at 0xeea78 | store | 4 | `abs 0x1b0f5c` | `ret(0x204d8)` |
| `ldr` at 0xeea7c | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeea88 | load | 4 | `abs 0xeeb44` | `-` |
| `ldr` at 0xeea90 | load | 4 | `abs 0xeeb30` | `-` |
| `str` at 0xeea98 | store | 4 | `abs 0x1b0cc4` | `0x0` |
| `ldr` at 0xeea9c | load | 4 | `abs 0xeeb34` | `-` |
| `str` at 0xeeaa4 | store | 4 | `abs 0x1b0cc8` | `0x0` |
| `ldr` at 0xeeaa8 | load | 4 | `abs 0xeeb38` | `-` |
| `str` at 0xeeab0 | store | 4 | `abs 0x1b0ccc` | `0x0` |
| `str` at 0xeeabc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xeeac0 | load | 4 | `abs 0xeeb3c` | `-` |
| `ldr` at 0xeeac4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeac8 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeeae4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeaec | load | 4 | `abs 0xeeb40` | `-` |
| `ldr` at 0xeeaf0 | load | 4 | `abs 0x1b0f5c` | `-` |
| `str` at 0xeeaf8 | store | 4 | `(unknown)` | `ret(0x51b28)` |
| `ldr` at 0xeeafc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeeb08 | load | 4 | `abs 0xeeb48` | `-` |
| `ldr` at 0xeeb0c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeeb14 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xeeb1c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldm` at 0xeeb28 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeeb28 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeeb28 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeeb28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeea70 | `(unknown callee) 0x204d8` | `[0x1b0f58]` | `0x4` | `?` | `0x1b0f58` |
| `bl` at 0xeea8c | `(unknown callee) 0x29908` | `0x184e90` | `?` | `?` | `[0x1b0f5c]` |
| `bl` at 0xeeadc | `(unknown callee) 0x51b28` | `0x2000` | `0x1` | `?` | `[0x1b0f58]` |
| `bl` at 0xeeb10 | `(unknown callee) 0x29908` | `0x184eb8` | `[sp-24]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xee9f0 | `[0x31b5b0] != 0x0` | `[0x31b5b0] == 0x0` |
| `bne` at 0xeea30 | `[0x31b5b0] != 0x1` | `[0x31b5b0] == 0x1` |
| `bne` at 0xeea84 | `[0x1b0f5c] != 0x0` | `[0x1b0f5c] == 0x0` |
| `bge` at 0xeead0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xeeb04 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeeb28 | `r0` | `?` |

## `SysComLogInsert_2m`

- `SysComLogInsert_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeeb4c to 0xeebc8.
  Established. Word loads read absolute 0x1b0f5c and 0x1b0f58 (`ldr` at 0xeeb6c, `ldr` at 0xeeb84).
  A signed comparison takes the branch to 0xeeb94 when entry r0 is less than [0x1b0f58], while its other path jumps to 0xeebb8 (`blt` at 0xeeb8c, `b` at 0xeeb90).
  The sole call to unnamed 0x51cb8 prints candidate r1/r2 values entry r1/entry r2, with r0/r3 unknown (`bl` at 0xeebb4).
  The prior computed load has unknown effective address, so the selected object remains Unknown (`ldr` at 0xeeba8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeeb5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xeeb60 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xeeb64 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xeeb68 | load | 4 | `abs 0xeebc0` | `-` |
| `ldr` at 0xeeb6c | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeeb7c | load | 4 | `abs 0xeebc4` | `-` |
| `ldr` at 0xeeb80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeb84 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeeb94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeb9c | load | 4 | `abs 0xeebc0` | `-` |
| `ldr` at 0xeeba0 | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeeba8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeebac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeebb0 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xeebbc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeebbc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeebbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeebb4 | `(unknown callee) 0x51cb8` | `?` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeeb74 | `[0x1b0f5c] != 0x0` | `[0x1b0f5c] == 0x0` |
| `blt` at 0xeeb8c | `r0 < [0x1b0f58] (signed, 32-bit)` | `r0 >= [0x1b0f58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeebbc | `r0` | `?` |

## `SysComLogFetch_2m`

- `SysComLogFetch_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeebc8 to 0xeee78.
  Established. A byte store writes 0 at ret(0x201fc), and word loads read [sp-24] +8, +0x10 and +0xc (`strb` at 0xeecf4, `ldr` at 0xeed44, `ldr` at 0xeed48, `ldr` at 0xeed4c).
  The call to unnamed 0x96878 prints candidate r0/r1/r2/r3 values ret(0x2031c)+[sp-32]/[[sp-24]+8]/[[sp-24]+16]/[[sp-24]+12] (`bl` at 0xeed50).
  Another path calls unnamed 0x521bc with candidate r1=0 and other register candidates unknown, then stores its result at sp-0x28 (`bl` at 0xeee40, `str` at 0xeee48).
  The returned content and the meanings of the loaded fields remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeebd8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xeebdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeebf4 | store | 4 | `sp -0x18` | `ret(0xa3c7c)` |
| `ldr` at 0xeebf8 | load | 4 | `abs 0xeee64` | `-` |
| `ldr` at 0xeebfc | load | 4 | `abs 0x1b0f58` | `-` |
| `str` at 0xeec10 | store | 4 | `sp -0x24` | `ret(0x201fc)` |
| `str` at 0xeec18 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xeec1c | load | 4 | `abs 0xeee64` | `-` |
| `ldr` at 0xeec20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeec24 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeec30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeec38 | load | 4 | `abs 0xeee68` | `-` |
| `ldr` at 0xeec3c | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeec44 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeec54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeec5c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xeec64 | store | 4 | `(unknown)` | `ret(0x521bc)` |
| `ldr` at 0xeec68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeec70 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0xeec7c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xeec84 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xeec88 | load | 4 | `abs 0xeee64` | `-` |
| `ldr` at 0xeec8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeec90 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeec9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeca4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeecac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeecbc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeecc8 | store | 4 | `sp -0x1c` | `(ret(0x2031c) + [sp-28])+80` |
| `ldr` at 0xeeccc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeecd4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xeecdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xeece8 | store | 4 | `sp -0x20` | `ret(0x201fc)` |
| `ldr` at 0xeecec | load | 4 | `sp -0x20` | `-` |
| `strb` at 0xeecf4 | store | 1 | `ret(0x201fc) +0x0` | `0x0` |
| `str` at 0xeecfc | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0xeed04 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xeed08 | load | 4 | `abs 0xeee64` | `-` |
| `ldr` at 0xeed0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeed10 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeed1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeed2c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeed34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeed44 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xeed48 | load | 4 | `[sp-24] +0x10` | `-` |
| `ldr` at 0xeed4c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0xeed54 | load | 4 | `abs 0xeee6c` | `-` |
| `ldr` at 0xeed58 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xeed64 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeed74 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeed80 | load | 4 | `abs 0xeee70` | `-` |
| `ldr` at 0xeed84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeed90 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeeda0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeedac | load | 4 | `abs 0xeee74` | `-` |
| `ldr` at 0xeedb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeedb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeedc0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeedc8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeedcc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeedd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeddc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeede4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeedec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeedf4 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xeedfc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeee04 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xeee08 | store | 4 | `sp -0x28` | `[sp-32]` |
| `ldr` at 0xeee10 | load | 4 | `abs 0xeee64` | `-` |
| `ldr` at 0xeee14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeee18 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeee24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeee2c | load | 4 | `abs 0xeee68` | `-` |
| `ldr` at 0xeee30 | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeee38 | load | 4 | `(unknown)` | `-` |
| `str` at 0xeee48 | store | 4 | `sp -0x28` | `ret(0x521bc)` |
| `str` at 0xeee54 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xeee58 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xeee60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeee60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeee60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeebec | `(unknown callee) 0xa3c7c` | `0x0` | `r1` | `r2` | `r0` |
| `bl` at 0xeec08 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xeec4c | `(unknown callee) 0x521bc` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xeecb0 | `(unknown callee) 0x2031c` | `?` | `?` | `?` | `?` |
| `bl` at 0xeece0 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `[0x1b0f58]` |
| `bl` at 0xeed20 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `[0x1b0f58]` |
| `bl` at 0xeed50 | `(unknown callee) 0x96878` | `(ret(0x2031c) + [sp-32])` | `[[sp-24]+8]` | `[[sp-24]+16]` | `[[sp-24]+12]` |
| `bl` at 0xeed68 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `[0x31b5b0]` |
| `bl` at 0xeed88 | `(unknown callee) 0x205bc` | `(ret(0x2031c) + [sp-32])` | `0x184ee0` | `[sp-20]` | `(ret(0x2031c) + [sp-32])` |
| `bl` at 0xeed94 | `(unknown callee) 0x2031c` | `[sp-32]` | `?` | `?` | `[0x31b5b0]` |
| `bl` at 0xeedb4 | `(unknown callee) 0x205bc` | `(ret(0x2031c) + [sp-32])` | `0x184eec` | `[sp-20]` | `(ret(0x2031c) + [sp-32])` |
| `bl` at 0xeedd0 | `(unknown callee) 0x203c4` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xeede8 | `(unknown callee) 0x20514` | `?` | `?` | `?` | `?` |
| `bl` at 0xeee00 | `(unknown callee) 0x20514` | `[sp-36]` | `?` | `?` | `[0x1b0f58]` |
| `bl` at 0xeee40 | `(unknown callee) 0x521bc` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeebe4 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xeec2c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xeec98 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xeed18 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xeed60 | `[0x31b5b0] != 0x1` | `[0x31b5b0] == 0x1` |
| `bge` at 0xeee20 | `[sp-20] >= [0x1b0f58] (signed, 32-bit)` | `[sp-20] < [0x1b0f58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeee60 | `r0` | `?` |

## `SysComLogReset_2m`

- `SysComLogReset_2m`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeee78 to 0xeef1c.
  Established. Word loads read absolute 0x1b0f58 and 0x1b0f5c (`ldr` at 0xeeee8, `ldr` at 0xeef00).
  A signed comparison branches to the return at 0xeef10 when [sp-20] is greater than or equal to [0x1b0f58], falling through to a call to unnamed 0x51c0c otherwise (`bge` at 0xeeef0, `bl` at 0xeef0c).
  A separate loop calls the same unnamed callee with all four register candidates unknown and updates sp-0x14 to [sp-20]+1 (`bl` at 0xeeecc, `str` at 0xeeed8).
  Neither call establishes which hardware or log object is reset.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeee88 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xeee8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeee9c | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xeeea0 | load | 4 | `abs 0xeef14` | `-` |
| `ldr` at 0xeeea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeea8 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeeeb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeebc | load | 4 | `abs 0xeef18` | `-` |
| `ldr` at 0xeeec0 | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeeec8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeeed0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xeeed8 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0xeeee0 | load | 4 | `abs 0xeef14` | `-` |
| `ldr` at 0xeeee4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeee8 | load | 4 | `abs 0x1b0f58` | `-` |
| `ldr` at 0xeeef4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeefc | load | 4 | `abs 0xeef18` | `-` |
| `ldr` at 0xeef00 | load | 4 | `abs 0x1b0f5c` | `-` |
| `ldr` at 0xeef08 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0xeef10 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xeef10 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xeef10 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xeef10 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeeecc | `(unknown callee) 0x51c0c` | `?` | `?` | `?` | `?` |
| `bl` at 0xeef0c | `(unknown callee) 0x51c0c` | `?` | `r1` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xeee94 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xeeeb0 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xeeef0 | `[sp-20] >= [0x1b0f58] (signed, 32-bit)` | `[sp-20] < [0x1b0f58] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xeef10 | `r0` | `?` |

## `SysCom_2m_Init`

- `SysCom_2m_Init`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xeef1c to 0xef09c.
  Established. The sole call to unnamed 0x6dc0c prints entry r0/r1/r2/r3 as candidates, and an ensuing word store saves an unknown value at sp-0x14 (`bl` at 0xeef2c, `str` at 0xeef38).
  Equality branches for [sp-20] values 0x43 and 0x33 both target 0xef048 (`beq` at 0xeef44, `beq` at 0xeef50).
  On the common store path, word stores write 0xee9d4, 0xeeb4c, 0xeebc8 and 0xeee78 to absolute 0x1b0cc0, 0x1b0cc4, 0x1b0cc8 and 0x1b0ccc (`str` at 0xef050, `str` at 0xef05c, `str` at 0xef068, `str` at 0xef074).
  The tested value does not establish a hardware product identity.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xeef38 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xeef3c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeef9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeefe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeff0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeeffc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef008 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef014 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef02c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef038 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef048 | load | 4 | `abs 0xef07c` | `-` |
| `ldr` at 0xef04c | load | 4 | `abs 0xef080` | `-` |
| `str` at 0xef050 | store | 4 | `abs 0x1b0cc0` | `0xee9d4` |
| `ldr` at 0xef054 | load | 4 | `abs 0xef084` | `-` |
| `ldr` at 0xef058 | load | 4 | `abs 0xef088` | `-` |
| `str` at 0xef05c | store | 4 | `abs 0x1b0cc4` | `0xeeb4c` |
| `ldr` at 0xef060 | load | 4 | `abs 0xef08c` | `-` |
| `ldr` at 0xef064 | load | 4 | `abs 0xef090` | `-` |
| `str` at 0xef068 | store | 4 | `abs 0x1b0cc8` | `0xeebc8` |
| `ldr` at 0xef06c | load | 4 | `abs 0xef094` | `-` |
| `ldr` at 0xef070 | load | 4 | `abs 0xef098` | `-` |
| `str` at 0xef074 | store | 4 | `abs 0x1b0ccc` | `0xeee78` |
| `ldm` at 0xef078 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xef078 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xef078 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xef078 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeef2c | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xeef44 | `[sp-20] == 0x43` | `[sp-20] != 0x43` |
| `beq` at 0xeef50 | `[sp-20] == 0x33` | `[sp-20] != 0x33` |
| `beq` at 0xeef5c | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0xeef68 | `[sp-20] == 0x34` | `[sp-20] != 0x34` |
| `beq` at 0xeef74 | `[sp-20] == 0x41` | `[sp-20] != 0x41` |
| `beq` at 0xeef80 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0xeef8c | `[sp-20] == 0x39` | `[sp-20] != 0x39` |
| `beq` at 0xeef98 | `[sp-20] == 0x37` | `[sp-20] != 0x37` |
| `beq` at 0xeefa4 | `[sp-20] == 0x38` | `[sp-20] != 0x38` |
| `beq` at 0xeefb0 | `[sp-20] == 0x30` | `[sp-20] != 0x30` |
| `beq` at 0xeefbc | `[sp-20] == 0x51` | `[sp-20] != 0x51` |
| `beq` at 0xeefc8 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0xeefd4 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0xeefe0 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0xeefec | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0xeeff8 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0xef004 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0xef010 | `[sp-20] == 0x24` | `[sp-20] != 0x24` |
| `beq` at 0xef01c | `[sp-20] == 0x2c` | `[sp-20] != 0x2c` |
| `beq` at 0xef028 | `[sp-20] == 0x7d` | `[sp-20] != 0x7d` |
| `beq` at 0xef034 | `[sp-20] == 0x2e` | `[sp-20] != 0x2e` |
| `beq` at 0xef040 | `[sp-20] == 0x7b` | `[sp-20] != 0x7b` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xef078 | `r0` | `ret(0x6dc0c)` |

## `SysComCaptureIn`

- `SysComCaptureIn`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xef370 to 0xefb00.
  Established. Word loads read absolute 0x31b5b8 and 0x31b5b0 (`ldr` at 0xef39c, `ldr` at 0xef410).
  The sole call to unnamed 0x203f4 prints candidate r0/r1/r2/r3 values unknown/[sp-32]/[sp]/unknown (`bl` at 0xef7c8).
  Later byte and halfword stores write ([sp] & 0xff) and ([sp-40] & 0xffff) at unknown effective addresses (`strb` at 0xef854, `strh` at 0xef8e4).
  Their destinations and field meanings remain Unknown, and the merged predicated move does not establish a definite value for [sp] (`movhs` at 0xef528).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xef380 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xef384 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xef388 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xef38c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xef390 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef39c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef3a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef3c8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef3d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef3d4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef3e4 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef3e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef3f0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef3f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef3fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef40c | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef410 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xef414 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef424 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef428 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef430 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef438 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef45c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef464 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef490 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef498 | load | 1 | `(unknown)` | `-` |
| `str` at 0xef49c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xef4a0 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef4a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef4ac | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef4b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef4d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef4e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef50c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef514 | load | 4 | `abs 0xefaf0` | `-` |
| `ldrh` at 0xef518 | load | 2 | `(unknown)` | `-` |
| `str` at 0xef51c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xef520 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xef52c | store | 4 | `sp +0x0` | `?` |
| `ldr` at 0xef530 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef534 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef53c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef544 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef568 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef59c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef5a4 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef5a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef5b0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef5b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef5dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef5e4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef610 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xef618 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xef638 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xef63c | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef640 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef648 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef650 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef674 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef67c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef6a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef6b0 | load | 4 | `abs 0xefaf0` | `-` |
| `ldr` at 0xef6b4 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef6b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef6c0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef6c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef6ec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef6f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef720 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef728 | load | 4 | `abs 0xefaf0` | `-` |
| `ldrh` at 0xef72c | load | 2 | `(unknown)` | `-` |
| `strh` at 0xef734 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xef738 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef764 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef770 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef774 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef77c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef784 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef7a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef7b0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef7c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xef7c4 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xef7cc | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef7d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef7d8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef7e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef804 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef80c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef838 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef844 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef850 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xef854 | store | 1 | `(unknown)` | `([sp] & 0xff)` |
| `ldr` at 0xef858 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef85c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef864 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef86c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef890 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef898 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef8c4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef8cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xef8d0 | load | 4 | `abs 0xefaf4` | `-` |
| `ldr` at 0xef8e0 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0xef8e4 | store | 2 | `(unknown)` | `([sp-40] & 0xffff)` |
| `ldr` at 0xef8e8 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef8ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef8f4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef8fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef920 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef928 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef954 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef95c | load | 4 | `abs 0xefaf8` | `-` |
| `ldr` at 0xef960 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef96c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef974 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xef998 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef9a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xef9cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xef9d4 | load | 4 | `abs 0xefaf8` | `-` |
| `ldr` at 0xef9d8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xef9e0 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xef9e4 | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xef9e8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xef9f0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xef9f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefa1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefa24 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefa50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefa58 | load | 4 | `abs 0xefafc` | `-` |
| `ldr` at 0xefa5c | load | 4 | `abs 0xefaec` | `-` |
| `ldr` at 0xefa60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefa68 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefa70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefa94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefa9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefac8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefad0 | load | 4 | `abs 0xefafc` | `-` |
| `ldr` at 0xefad4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefad8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xefae0 | store | 4 | `(unknown)` | `?` |
| `ldm` at 0xefae8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xefae8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xefae8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xef7c8 | `(unknown callee) 0x203f4` | `?` | `[sp-32]` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xef3dc | `? > [sp-28] (signed, 32-bit)` | `? <= [sp-28] (signed, 32-bit)` |
| `bgt` at 0xef404 | `? > [sp-24] (signed, 32-bit)` | `? <= [sp-24] (signed, 32-bit)` |
| `bgt` at 0xef41c | `[0x31b5b0] > [sp-20] (signed, 32-bit)` | `[0x31b5b0] <= [sp-20] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xefae8 | `r0` | `?` |

## `SysComCaptureOut`

- `SysComCaptureOut`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xefb00 to 0xf008c.
  Established. The signed comparison branches to 0xefbb4 when [0x31b5b0] exceeds [sp-20], while its other path jumps to 0xf0078 (`bgt` at 0xefbac, `b` at 0xefbb0).
  Byte and halfword loads have unknown effective addresses, and the later byte store also has unknown address and value (`ldrb` at 0xefc28, `ldrh` at 0xefca8, `strb` at 0xefdc8).
  The call to unnamed 0x203f4 prints candidate r1=[sp-32] and r2=[sp], with r0/r3 unknown (`bl` at 0xeff58).
  Further stores carry ([sp] & 0xff) and ([sp-40] & 0xffff), but the output record layout remains Unknown (`strb` at 0xeffe4, `strh` at 0xf0074).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xefb10 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xefb14 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xefb18 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xefb1c | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xefb20 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefb24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefb2c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefb34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefb58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefb60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefb64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefb74 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefb78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefb80 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefb88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefb8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefb9c | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefba0 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xefba4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefbb4 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefbb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefbc0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefbc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefbec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefbf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefc20 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xefc28 | load | 1 | `(unknown)` | `-` |
| `str` at 0xefc2c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xefc30 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefc34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefc3c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefc44 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefc68 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefc70 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefc9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefca4 | load | 4 | `abs 0xf0084` | `-` |
| `ldrh` at 0xefca8 | load | 2 | `(unknown)` | `-` |
| `str` at 0xefcac | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xefcb0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xefcbc | store | 4 | `sp +0x0` | `?` |
| `ldr` at 0xefcc0 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefcc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefccc | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefcd4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefcf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefd00 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefd2c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefd34 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefd40 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefd48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefd6c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefd74 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefda0 | load | 4 | `(unknown)` | `-` |
| `ldrb` at 0xefda8 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xefdc8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xefdcc | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefdd0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefdd8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefde0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefe04 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefe0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefe38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefe40 | load | 4 | `abs 0xf0084` | `-` |
| `ldr` at 0xefe44 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xefe48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefe50 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefe58 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xefe7c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefe84 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefeb0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xefeb8 | load | 4 | `abs 0xf0084` | `-` |
| `ldrh` at 0xefebc | load | 2 | `(unknown)` | `-` |
| `strh` at 0xefec4 | store | 2 | `(unknown)` | `?` |
| `ldr` at 0xefec8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xefef4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeff00 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xeff04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff0c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xeff14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeff38 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeff40 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeff50 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xeff54 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xeff5c | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xeff60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xeff68 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xeff70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xeff94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeff9c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xeffc8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xeffd4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xeffe0 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xeffe4 | store | 1 | `(unknown)` | `([sp] & 0xff)` |
| `ldr` at 0xeffe8 | load | 4 | `abs 0xf0080` | `-` |
| `ldr` at 0xeffec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xefff4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xefffc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0020 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0054 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf005c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0060 | load | 4 | `abs 0xf0088` | `-` |
| `ldr` at 0xf0070 | load | 4 | `sp -0x28` | `-` |
| `strh` at 0xf0074 | store | 2 | `(unknown)` | `([sp-40] & 0xffff)` |
| `ldm` at 0xf007c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf007c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf007c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xeff58 | `(unknown callee) 0x203f4` | `?` | `[sp-32]` | `[sp]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xefb6c | `? > [sp-28] (signed, 32-bit)` | `? <= [sp-28] (signed, 32-bit)` |
| `bgt` at 0xefb94 | `? > [sp-24] (signed, 32-bit)` | `? <= [sp-24] (signed, 32-bit)` |
| `bgt` at 0xefbac | `[0x31b5b0] > [sp-20] (signed, 32-bit)` | `[0x31b5b0] <= [sp-20] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf007c | `r0` | `?` |

## `SysComCaptureReset`

- `SysComCaptureReset`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf008c to 0xf04c4.
  Established. A nonzero word at absolute 0x1b0f64 takes the branch to 0xf02c4, while the other path reaches a store of 1 to that address (`ldr` at 0xf00a0, `bne` at 0xf00a8, `str` at 0xf00b4).
  Word stores later write 0xef370 to absolute 0x1b0cb8 and 0xefb00 to absolute 0x1b0cbc (`str` at 0xf02b4, `str` at 0xf02c0).
  Two calls to unnamed 0x201fc are followed by stores of their results at unknown effective addresses (`bl` at 0xf0204, `str` at 0xf020c, `bl` at 0xf0280, `str` at 0xf0288).
  The sizes and destinations of the capture objects remain Unknown despite the resolved function-address stores.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xf009c | load | 4 | `abs 0xf04ac` | `-` |
| `ldr` at 0xf00a0 | load | 4 | `abs 0x1b0f64` | `-` |
| `ldr` at 0xf00ac | load | 4 | `abs 0xf04ac` | `-` |
| `str` at 0xf00b4 | store | 4 | `abs 0x1b0f64` | `0x1` |
| `str` at 0xf00c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xf00c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf00d4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf00e8 | store | 4 | `sp -0x20` | `0x14` |
| `str` at 0xf00f4 | store | 4 | `sp -0x20` | `0x20` |
| `str` at 0xf0100 | store | 4 | `sp -0x20` | `0x20` |
| `str` at 0xf0108 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf010c | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0110 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xf0114 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf0124 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf0128 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf012c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0134 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf013c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf014c | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0150 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0158 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf0160 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0184 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf018c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0190 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xf0194 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0198 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf01a0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf01a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf01cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf01d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf020c | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xf0210 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0214 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf021c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf0224 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0248 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0250 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xf0288 | store | 4 | `(unknown)` | `ret(0x201fc)` |
| `ldr` at 0xf028c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf0294 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf029c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf02a4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xf02ac | load | 4 | `abs 0xf04b4` | `-` |
| `ldr` at 0xf02b0 | load | 4 | `abs 0xf04b8` | `-` |
| `str` at 0xf02b4 | store | 4 | `abs 0x1b0cb8` | `0xef370` |
| `ldr` at 0xf02b8 | load | 4 | `abs 0xf04bc` | `-` |
| `ldr` at 0xf02bc | load | 4 | `abs 0xf04c0` | `-` |
| `str` at 0xf02c0 | store | 4 | `abs 0x1b0cbc` | `0xefb00` |
| `str` at 0xf02c8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xf02cc | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf02d0 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xf02d4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf02e4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xf02e8 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf02ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf02f4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf02fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0300 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf030c | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0310 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0318 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf0320 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0344 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf034c | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf0350 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0358 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf0360 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0384 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf038c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf03b8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf03c8 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf03cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf03d4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf03dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0400 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0408 | load | 4 | `abs 0xf04b0` | `-` |
| `ldr` at 0xf040c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0414 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf041c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0440 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0448 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0474 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0484 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xf048c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldr` at 0xf0494 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf049c | store | 4 | `sp -0x18` | `?` |
| `ldm` at 0xf04a8 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xf04a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf04a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf04a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf00b8 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `0x1` | `0x1b0f64` |
| `bl` at 0xf0204 | `(unknown callee) 0x201fc` | `?` | `[sp-28]` | `[sp-32]` | `?` |
| `bl` at 0xf0280 | `(unknown callee) 0x201fc` | `?` | `[sp-28]` | `[sp-32]` | `?` |
| `bl` at 0xf03c4 | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |
| `bl` at 0xf0480 | `(unknown callee) 0x20550` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xf00a8 | `[0x1b0f64] != 0x0` | `[0x1b0f64] == 0x0` |
| `beq` at 0xf00d0 | `[sp-36] == 0x2e` | `[sp-36] != 0x2e` |
| `beq` at 0xf00dc | `[sp-36] == 0x40` | `[sp-36] != 0x40` |
| `ble` at 0xf011c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xf0148 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xf02dc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xf0308 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf04a8 | `r0` | `?` |

## `SysComCaptureDump`

- `SysComCaptureDump`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xf04c4 to 0xf0b40.
  Established. A signed comparison takes the branch to 0xf052c when [0x31b5b0] exceeds entry r0, and its fall-through reaches a call to unnamed 0x201fc with candidate r0=0x50 (`bgt` at 0xf04f4, `bl` at 0xf0500).
  A later word store writes ret(0x201fc) at [sp-36], and another saves [[sp-36]] at sp-0x40 (`str` at 0xf077c, `str` at 0xf0784).
  The call to unnamed 0x6e1b0 prints candidate r0/r1/r2/r3 values [sp-24]/1/0/0 (`bl` at 0xf07b0).
  A byte and a halfword load have unknown effective addresses (`ldrb` at 0xf095c, `ldrh` at 0xf09b8), leaving the dumped record fields Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xf04d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xf04d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xf04dc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xf04e0 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xf04e4 | load | 4 | `abs 0xf0b10` | `-` |
| `ldr` at 0xf04e8 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xf04ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf04f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0508 | store | 4 | `r3 +0x0` | `ret(0x201fc)` |
| `ldr` at 0xf050c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0510 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf0514 | load | 4 | `abs 0xf0b14` | `-` |
| `ldr` at 0xf0518 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf0524 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xf052c | load | 4 | `abs 0xf0b10` | `-` |
| `ldr` at 0xf0530 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0538 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf0540 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0544 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0550 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf0560 | store | 4 | `r3 +0x0` | `ret(0x201fc)` |
| `ldr` at 0xf0564 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0568 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf056c | load | 4 | `abs 0xf0b18` | `-` |
| `ldr` at 0xf0570 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0574 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf0580 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xf0588 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xf0594 | load | 4 | `abs 0xf0b10` | `-` |
| `ldr` at 0xf0598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf05a0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf05a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf05cc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf05d4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf05d8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf05e0 | load | 4 | `abs 0xf0b10` | `-` |
| `ldr` at 0xf05e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf05ec | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf05f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0618 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0620 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf0624 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xf0628 | load | 4 | `abs 0xf0b10` | `-` |
| `ldr` at 0xf062c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xf0634 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xf063c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf0660 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xf0668 | load | 4 | `(unknown)` | `-` |
| `str` at 0xf066c | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xf0670 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf067c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xf068c | store | 4 | `[sp-36] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xf0690 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0694 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf0698 | load | 4 | `abs 0xf0b1c` | `-` |
| `str` at 0xf06a4 | store | 4 | `sp -0x50` | `?` |
| `str` at 0xf06b0 | store | 4 | `sp -0x3c` | `0x0` |
| `str` at 0xf06b8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf06bc | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf06c0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xf06d0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xf06d4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf06e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf070c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0718 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xf0724 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf0734 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf0740 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xf0744 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf074c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf0754 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf075c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xf0764 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0768 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xf077c | store | 4 | `[sp-36] +0x0` | `ret(0x201fc)` |
| `ldr` at 0xf0780 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xf0784 | store | 4 | `sp -0x40` | `[[sp-36]]` |
| `str` at 0xf078c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xf0790 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0794 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xf07a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xf07b4 | store | 4 | `sp -0x44` | `ret(0x6e1b0)` |
| `ldr` at 0xf07b8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xf07c8 | store | 4 | `sp -0x48` | `ret(0x2031c)+1` |
| `ldr` at 0xf07cc | load | 4 | `sp -0x48` | `-` |
| `str` at 0xf07f4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf07f8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf07fc | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xf0800 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xf080c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf0810 | store | 4 | `(unknown)` | `[sp-44]` |
| `ldr` at 0xf0814 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0840 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0848 | load | 4 | `abs 0xf0b20` | `-` |
| `ldr` at 0xf084c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf0850 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf0854 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0880 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0888 | load | 4 | `abs 0xf0b24` | `-` |
| `ldr` at 0xf088c | load | 4 | `(unknown)` | `-` |
| `str` at 0xf0890 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xf0894 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0898 | load | 4 | `abs 0xf0b28` | `-` |
| `ldr` at 0xf08a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xf08a8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf08b8 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf08c0 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x2031c))` |
| `ldr` at 0xf08c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf08f0 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xf08f8 | load | 1 | `(unknown)` | `-` |
| `str` at 0xf0900 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xf0908 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xf090c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xf0918 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0944 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0950 | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xf095c | load | 1 | `(unknown)` | `-` |
| `str` at 0xf0960 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xf0964 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf0970 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf099c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf09a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf09a8 | load | 4 | `abs 0xf0b2c` | `-` |
| `ldrh` at 0xf09b8 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0xf09bc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf09c0 | load | 4 | `abs 0xf0b30` | `-` |
| `ldr` at 0xf09cc | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf09dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf09e4 | store | 4 | `sp -0x40` | `([sp-64] + ret(0x2031c))` |
| `str` at 0xf09ec | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xf09f0 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xf09f4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0xf0a00 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xf0a2c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xf0a34 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xf0a40 | load | 4 | `sp -0x48` | `-` |
| `ldrb` at 0xf0a48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xf0a4c | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0a50 | load | 4 | `abs 0xf0b34` | `-` |
| `ldr` at 0xf0a5c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf0a64 | store | 4 | `sp -0x40` | `[sp-64]+3` |
| `ldr` at 0xf0a68 | load | 4 | `sp -0x48` | `-` |
| `str` at 0xf0a70 | store | 4 | `sp -0x48` | `[sp-72]+1` |
| `ldr` at 0xf0a78 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0a7c | load | 4 | `abs 0xf0b38` | `-` |
| `ldr` at 0xf0a84 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0a94 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xf0a9c | store | 4 | `sp -0x40` | `([sp-64] + ret(0x2031c))` |
| `ldr` at 0xf0aa0 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xf0ac0 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xf0ac4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xf0acc | store | 4 | `sp -0x34` | `[sp-52]+1` |
| `ldr` at 0xf0ad4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xf0adc | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0xf0ae4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xf0ae8 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0xf0aec | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0af8 | load | 4 | `sp -0x40` | `-` |
| `ldr` at 0xf0afc | load | 4 | `abs 0xf0b3c` | `-` |
| `ldr` at 0xf0b04 | load | 4 | `sp -0x50` | `-` |
| `ldm` at 0xf0b0c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xf0b0c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xf0b0c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xf0b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xf0500 | `(unknown callee) 0x201fc` | `0x50` | `r1` | `[0x31b5b0]` | `r0` |
| `bl` at 0xf051c | `(unknown callee) 0x205bc` | `[[sp-36]]` | `0x184fb4` | `[sp-24]` | `[sp-36]` |
| `bl` at 0xf0558 | `(unknown callee) 0x201fc` | `0x50` | `0x31b5a0` | `?` | `r1` |
| `bl` at 0xf0578 | `(unknown callee) 0x205bc` | `[[sp-36]]` | `0x184fd4` | `[sp-28]` | `[sp-24]` |
| `bl` at 0xf0684 | `(unknown callee) 0x201fc` | `0x50` | `[sp-28]` | `?` | `[sp-40]` |
| `bl` at 0xf069c | `(unknown callee) 0x20634` | `[[sp-36]]` | `0x185000` | `?` | `[sp-36]` |
| `bl` at 0xf0774 | `(unknown callee) 0x201fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xf07b0 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x1` | `0x0` | `0x0` |
| `bl` at 0xf07bc | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xf0804 | `(unknown callee) 0x203f4` | `[sp-76]` | `[sp-68]` | `[sp-72]` | `?` |
| `bl` at 0xf08a4 | `(unknown callee) 0x205bc` | `[sp-64]` | `0x185034` | `ret(0x203f4)` | `[sp-28]` |
| `bl` at 0xf08ac | `(unknown callee) 0x2031c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xf09c8 | `(unknown callee) 0x205bc` | `?` | `0x18505c` | `?` | `?` |
| `bl` at 0xf09d0 | `(unknown callee) 0x2031c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xf0a58 | `(unknown callee) 0x205bc` | `?` | `0x185064` | `?` | `?` |
| `bl` at 0xf0a80 | `(unknown callee) 0x20634` | `?` | `0x18506c` | `?` | `[sp-76]` |
| `bl` at 0xf0a88 | `(unknown callee) 0x2031c` | `[sp-64]` | `?` | `?` | `?` |
| `bl` at 0xf0b00 | `(unknown callee) 0x203c4` | `?` | `0x185070` | `[[sp-36]]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bgt` at 0xf04f4 | `[0x31b5b0] > r0 (signed, 32-bit)` | `[0x31b5b0] <= r0 (signed, 32-bit)` |
| `bgt` at 0xf054c | `? > r1 (signed, 32-bit)` | `? <= r1 (signed, 32-bit)` |
| `bne` at 0xf0590 | `r2 != 0x0` | `r2 == 0x0` |
| `bne` at 0xf0678 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bge` at 0xf06c8 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0xf06dc | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `bge` at 0xf079c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bgt` at 0xf0914 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xf096c | `[sp-76] == 0x0` | `[sp-76] != 0x0` |
| `bge` at 0xf09fc | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bne` at 0xf0af4 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xf0b0c | `r0` | `[sp-80]` |
