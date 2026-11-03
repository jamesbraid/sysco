# ARM Linux 2.6 communication endpoints in 1.44.0

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
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package6/linux26-syscom-endpoints-facts.txt) retain all tool sections.

## `SysComInit`

- `SysComInit`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7aa0 to 0xb7b2c.
  Established. Word loads read absolute 0x31b5b0 and 0x31b5b8 (`ldr` at 0xb7abc, `ldr` at 0xb7ad8).
  The sole call to unnamed 0xb6bd4 prints candidate r0/r1/r2/r3 values unknown/0x31b5a0/unknown/unknown (`bl` at 0xb7b10).
  Two further word loads have unknown effective addresses, leaving the represented hardware entries Unknown (`ldr` at 0xb7ae0, `ldr` at 0xb7b00).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7ab4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xb7ab8 | load | 4 | `abs 0xb7b28` | `-` |
| `ldr` at 0xb7abc | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xb7ac0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7acc | load | 4 | `abs 0xb7b28` | `-` |
| `ldr` at 0xb7ad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7ad8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7ae0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7aec | load | 4 | `abs 0xb7b28` | `-` |
| `ldr` at 0xb7af0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7af8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7b00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7b0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7b14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7b1c | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0xb7b24 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7b24 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7b24 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7b24 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7b10 | `(unknown callee) 0xb6bd4` | `?` | `0x31b5a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb7ac8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb7ae8 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xb7b08 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7b24 | `r0` | `?` |

## `SysComAdd`

- `SysComAdd`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7b2c to 0xb7b9c.
  Established. A word load reads absolute 0x31b5b8, followed by a word load with unknown effective address (`ldr` at 0xb7b54, `ldr` at 0xb7b5c).
  The call to unnamed 0xb6348 prints candidate r0/r1/r2 values entry r0/entry r2/entry r1 (`bl` at 0xb7b80).
  A subsequent word store writes 0 to sp-0x20, while the other branch stores an unknown value after unsupported mvn effects (`str` at 0xb7b88, `mvn` at 0xb7b68, `str` at 0xb7b6c).
  The hardware meaning of the tested entry remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7b3c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7b40 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7b44 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb7b48 | load | 4 | `abs 0xb7b98` | `-` |
| `ldr` at 0xb7b4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7b54 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7b5c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7b6c | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb7b74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7b78 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7b7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7b88 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb7b8c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xb7b94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7b94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7b94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7b80 | `(unknown callee) 0xb6348` | `r0` | `r2` | `r1` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb7b64 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7b94 | `r0` | `[sp-32]` |

## `SysComRemove`

- `SysComRemove`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7b9c to 0xb7e54.
  Established. Two calls to unnamed 0x227b4 are followed by stores of their results at sp-0x1c (`bl` at 0xb7c50, `str` at 0xb7c58, `bl` at 0xb7cec, `str` at 0xb7cf4).
  Later word stores write 0 at three unknown effective addresses (`str` at 0xb7df0, `str` at 0xb7e0c, `str` at 0xb7e28).
  The loop also calls unnamed 0x23480 with candidate r0/r1 values unknown/[sp-24] (`bl` at 0xb7d88).
  Which hardware resources these computed addresses represent remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7bac | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb7bb0 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7bb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7bbc | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7bc4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7bd4 | store | 4 | `sp -0x20` | `?` |
| `str` at 0xb7be0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb7be4 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7be8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7bf0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7bf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7bfc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7c08 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7c14 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7c1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7c40 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7c58 | store | 4 | `sp -0x1c` | `ret(0x227b4)` |
| `ldr` at 0xb7c5c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7c68 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7c74 | load | 4 | `abs 0xb7e44` | `-` |
| `str` at 0xb7c78 | store | 4 | `sp -0x30` | `0x16d46c` |
| `ldr` at 0xb7c7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7c80 | store | 4 | `sp -0x2c` | `[sp-20]` |
| `ldr` at 0xb7c84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7c88 | store | 4 | `sp -0x28` | `[sp-24]` |
| `str` at 0xb7c8c | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0xb7c94 | load | 4 | `abs 0xb7e48` | `-` |
| `ldr` at 0xb7c98 | load | 4 | `abs 0xb7e4c` | `-` |
| `ldr` at 0xb7ca4 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7cb0 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7cb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7cdc | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7cf4 | store | 4 | `sp -0x1c` | `ret(0x227b4)` |
| `ldr` at 0xb7cf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7d04 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb7d10 | load | 4 | `abs 0xb7e50` | `-` |
| `str` at 0xb7d14 | store | 4 | `sp -0x30` | `0x16d4a4` |
| `ldr` at 0xb7d18 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb7d1c | store | 4 | `sp -0x2c` | `[sp-20]` |
| `ldr` at 0xb7d20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7d24 | store | 4 | `sp -0x28` | `[sp-24]` |
| `str` at 0xb7d28 | store | 4 | `sp -0x24` | `ret(0x219a4)` |
| `ldr` at 0xb7d30 | load | 4 | `abs 0xb7e48` | `-` |
| `ldr` at 0xb7d40 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7d44 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7d4c | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7d54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7d78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7d8c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb7d94 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb7d9c | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7da0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7da8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7dbc | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7dc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7dc8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb7dd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb7dd8 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7ddc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7de4 | load | 4 | `abs 0x31b5b8` | `-` |
| `str` at 0xb7df0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb7df4 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e00 | load | 4 | `abs 0x31b5b8` | `-` |
| `str` at 0xb7e0c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb7e10 | load | 4 | `abs 0xb7e40` | `-` |
| `ldr` at 0xb7e14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb7e1c | load | 4 | `abs 0x31b5b8` | `-` |
| `str` at 0xb7e28 | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xb7e30 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb7e34 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xb7e3c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7e3c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7e3c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7c50 | `(unknown callee) 0x227b4` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7c6c | `(unknown callee) 0x219a4` | `ret(0x227b4)` | `?` | `?` | `ret(0x227b4)` |
| `bl` at 0xb7ca0 | `(unknown callee) 0x22070` | `0x1` | `0x16cfa4` | `0x38e` | `0x0` |
| `bl` at 0xb7cec | `(unknown callee) 0x227b4` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xb7d08 | `(unknown callee) 0x219a4` | `ret(0x227b4)` | `?` | `?` | `ret(0x227b4)` |
| `bl` at 0xb7d3c | `(unknown callee) 0x22070` | `0x1` | `0x16cfa4` | `0x394` | `0x0` |
| `bl` at 0xb7d88 | `(unknown callee) 0x23480` | `?` | `[sp-24]` | `?` | `?` |
| `bl` at 0xb7dd4 | `(unknown callee) 0x20514` | `?` | `0x31b5a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7bcc | `? != 0x0` | `? == 0x0` |
| `ble` at 0xb7c04 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `beq` at 0xb7c64 | `ret(0x227b4) == 0x0` | `ret(0x227b4) != 0x0` |
| `beq` at 0xb7d00 | `ret(0x227b4) == 0x0` | `ret(0x227b4) != 0x0` |
| `beq` at 0xb7db8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7e3c | `r0` | `[sp-32]` |

## `SysComGetSlab`

- `SysComGetSlab`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7e54 to 0xb7e8c.
  Established. Entry r0 is saved by a word store at sp-0x14 before a call to unnamed 0x6dc0c (`str` at 0xb7e64, `bl` at 0xb7e68).
  The second call, to unnamed 0xb7e8c, prints candidate r0/r1/r2/r3 values unknown/[sp-20]/unknown/unknown (`bl` at 0xb7e7c).
  The recorded r0 return is that second call result, whose hardware interpretation remains Unknown (`ldm` at 0xb7e88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7e64 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb7e78 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7e88 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7e88 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7e88 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7e88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7e68 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb7e7c | `(unknown callee) 0xb7e8c` | `?` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7e88 | `r0` | `ret(0xb7e8c)` |

## `SysComGetSubc`

- `SysComGetSubc`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7f6c to 0xb7fa4.
  Established. The first call to unnamed 0x6dc0c prints entry r0/r1/r2/r3 as candidates (`bl` at 0xb7f80).
  Entry r0 was saved at sp-0x14, and the second call to unnamed 0xb7fa4 prints candidate r1=[sp-20] with r0/r2/r3 unknown (`str` at 0xb7f7c, `bl` at 0xb7f94);
  its result is the recorded r0 return, with no established hardware meaning for that value (`ldm` at 0xb7fa0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7f7c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb7f90 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7fa0 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb7fa0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb7fa0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb7fa0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7f80 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb7f94 | `(unknown callee) 0xb7fa4` | `?` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb7fa0 | `r0` | `ret(0xb7fa4)` |

## `SysComSetTasknum`

- `SysComSetTasknum`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb80fc to 0xb8140.
  Established. Word stores save entry r0 and r1 at sp-0x14 and sp-0x18 (`str` at 0xb810c, `str` at 0xb8110).
  After a call to unnamed 0x6dc0c, the call to unnamed 0xb8140 prints candidate r0/r1/r2/r3 values unknown/[sp-20]/[sp-24]/unknown (`bl` at 0xb8114, `bl` at 0xb812c).
  The recorded r0 return comes from 0xb8140, so the body alone leaves the affected task or hardware state Unknown (`ldm` at 0xb813c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb810c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8110 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8124 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8128 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xb813c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb813c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb813c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8114 | `(unknown callee) 0x6dc0c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb812c | `(unknown callee) 0xb8140` | `?` | `[sp-20]` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb813c | `r0` | `ret(0xb8140)` |

## `SysComPurge`

- `SysComPurge`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb82a4 to 0xb83b0.
  Established. A word load reads absolute 0x31b5a0, and a word store writes that loaded value at an unknown effective address (`ldr` at 0xb82c8, `str` at 0xb82cc).
  Byte and halfword stores write 0 at other unknown effective addresses (`strb` at 0xb82e0, `strh` at 0xb82f4, `strb` at 0xb8338, `strb` at 0xb834c, `strh` at 0xb8374).
  The call to unnamed 0xfd284 prints candidate r0/r1/r2/r3 values sp-28/[sp-20]/1/1 and supplies the recorded r0 return (`bl` at 0xb83a0, `ldm` at 0xb83a8).
  The destination and hardware meaning of the constructed values remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb82b4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xb82c4 | load | 4 | `abs 0xb83ac` | `-` |
| `ldr` at 0xb82c8 | load | 4 | `abs 0x31b5a0` | `-` |
| `str` at 0xb82cc | store | 4 | `(unknown)` | `[0x31b5a0]` |
| `strb` at 0xb82e0 | store | 1 | `(unknown)` | `0x0` |
| `strh` at 0xb82f4 | store | 2 | `(unknown)` | `0x0` |
| `ldr` at 0xb8304 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8308 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb830c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8314 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8324 | store | 4 | `sp -0x28` | `?` |
| `strb` at 0xb8338 | store | 1 | `(unknown)` | `0x0` |
| `strb` at 0xb834c | store | 1 | `(unknown)` | `0x0` |
| `ldr` at 0xb835c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8360 | store | 4 | `(unknown)` | `[sp-40]` |
| `strh` at 0xb8374 | store | 2 | `(unknown)` | `0x0` |
| `str` at 0xb8380 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xb8384 | load | 4 | `abs 0xb83ac` | `-` |
| `ldr` at 0xb8388 | load | 4 | `abs 0x31b5bc` | `-` |
| `str` at 0xb838c | store | 4 | `sp -0x2c` | `[0x31b5bc]` |
| `ldr` at 0xb8394 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb83a8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb83a8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb83a8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb83a0 | `(unknown callee) 0xfd284` | `sp-28` | `[sp-20]` | `0x1` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb83a8 | `r0` | `ret(0xfd284)` |

## `SysComPurgeAll`

- `SysComPurgeAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb83b0 to 0xb8420.
  Established. Word loads read absolute 0x31b5b0 and 0x31b5b8 (`ldr` at 0xb83cc, `ldr` at 0xb83e8).
  A conditional branch takes the path to 0xb8400 when Z is clear, while its other path jumps to 0xb8408 (`bne` at 0xb83f8, `b` at 0xb83fc).
  The sole call to unnamed 0xb82a4 prints candidate r0/r1/r2/r3 values unknown/0x31b5a0/unknown/unknown (`bl` at 0xb8404).
  The prior load has unknown effective address, so the hardware condition behind that branch remains Unknown (`ldr` at 0xb83f0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb83c4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xb83c8 | load | 4 | `abs 0xb841c` | `-` |
| `ldr` at 0xb83cc | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xb83d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb83dc | load | 4 | `abs 0xb841c` | `-` |
| `ldr` at 0xb83e0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb83e8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb83f0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8400 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8408 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8410 | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0xb8418 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8418 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8418 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8418 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8404 | `(unknown callee) 0xb82a4` | `?` | `0x31b5a0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb83d8 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb83f8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8418 | `r0` | `?` |

## `SysComJTagReadReady`

- `SysComJTagReadReady`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8420 to 0xb8924.
  Established. Word loads read [sp-40] +0x28, +0x30 and +0x20 (`ldr` at 0xb8510, `ldr` at 0xb851c, `ldr` at 0xb8588).
  A byte load reads [sp-76], and the call to unnamed 0x1012ec prints candidate r0/r1 values [sp-40]+368/[[sp-76]]:1 (`ldrb` at 0xb8770, `bl` at 0xb877c).
  The positive-result path reaches a word store of that call result at [sp-40] +0x3a4 (`bge` at 0xb8790, `bne` at 0xb87a0, `str` at 0xb87b0).
  These object fields and the two earlier callees 0x139398 and 0x139664 remain unnamed in the analysis, leaving their hardware role Unknown (`bl` at 0xb85e0, `bl` at 0xb85fc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8430 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb8434 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb8438 | load | 4 | `abs 0xb8904` | `-` |
| `ldr` at 0xb843c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8444 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb844c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8458 | load | 4 | `abs 0xb8904` | `-` |
| `ldr` at 0xb845c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8464 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb846c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb847c | load | 4 | `abs 0xb8904` | `-` |
| `ldr` at 0xb8480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8488 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8490 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb84b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb84bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb84c8 | load | 4 | `abs 0xb8904` | `-` |
| `ldr` at 0xb84cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb84d4 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb84dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8500 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8508 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb850c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8510 | load | 4 | `[sp-40] +0x28` | `-` |
| `str` at 0xb8514 | store | 4 | `sp -0x2c` | `[[sp-40]+40]` |
| `ldr` at 0xb8518 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb851c | load | 4 | `[sp-40] +0x30` | `-` |
| `str` at 0xb8520 | store | 4 | `sp -0x30` | `[[sp-40]+48]` |
| `ldr` at 0xb853c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8548 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb854c | load | 4 | `[sp-40] +0x20` | `-` |
| `str` at 0xb8554 | store | 4 | `sp -0xe8` | `?` |
| `ldr` at 0xb8558 | load | 4 | `sp -0xe8` | `-` |
| `str` at 0xb8560 | store | 4 | `sp -0xe8` | `?` |
| `ldr` at 0xb8564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8574 | load | 4 | `sp -0xe8` | `-` |
| `str` at 0xb857c | store | 4 | `sp -0xe8` | `?` |
| `ldr` at 0xb8584 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8588 | load | 4 | `[sp-40] +0x20` | `-` |
| `str` at 0xb858c | store | 4 | `sp -0xe8` | `[[sp-40]+32]` |
| `ldr` at 0xb8590 | load | 4 | `sp -0xe8` | `-` |
| `str` at 0xb8594 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb8598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb859c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb85ac | store | 4 | `sp -0x38` | `ret(0xb5c90)` |
| `str` at 0xb85b4 | store | 4 | `sp -0x4c` | `sp-72` |
| `str` at 0xb85bc | store | 4 | `sp -0x54` | `0x0` |
| `ldr` at 0xb85c4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb85c8 | load | 4 | `[sp-40] +0x0` | `-` |
| `ldr` at 0xb85d8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb85e8 | store | 4 | `sp -0x24` | `ret(0x139398)` |
| `ldr` at 0xb85f4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb8604 | store | 4 | `sp -0x24` | `ret(0x139664)` |
| `ldr` at 0xb860c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb861c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb8628 | load | 4 | `abs 0xb8908` | `-` |
| `ldr` at 0xb8634 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8638 | store | 4 | `sp -0xec` | `[sp-24]` |
| `ldr` at 0xb8640 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb8648 | store | 4 | `sp -0xec` | `?` |
| `ldr` at 0xb864c | load | 4 | `sp -0xec` | `-` |
| `ldr` at 0xb8654 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8660 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb866c | load | 4 | `abs 0xb890c` | `-` |
| `str` at 0xb8674 | store | 4 | `sp -0xd8` | `ret(0x6e1b0)` |
| `ldr` at 0xb8678 | load | 4 | `sp -0xd8` | `-` |
| `str` at 0xb8688 | store | 4 | `sp -0xdc` | `ret(0x2031c)+1` |
| `ldr` at 0xb868c | load | 4 | `sp -0xdc` | `-` |
| `str` at 0xb86b4 | store | 4 | `sp -0xe0` | `?` |
| `ldr` at 0xb86b8 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0xb86bc | load | 4 | `sp -0xd8` | `-` |
| `ldr` at 0xb86c0 | load | 4 | `sp -0xdc` | `-` |
| `ldr` at 0xb86cc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb86dc | store | 4 | `(unknown)` | `ret(0x219a4)` |
| `ldr` at 0xb86e4 | load | 4 | `abs 0xb8910` | `-` |
| `ldr` at 0xb86ec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8700 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb8710 | load | 4 | `abs 0xb8914` | `-` |
| `ldr` at 0xb8720 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb8730 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb8734 | store | 4 | `sp -0x100` | `[sp-36]` |
| `ldr` at 0xb8738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb873c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8740 | load | 4 | `abs 0xb8918` | `-` |
| `str` at 0xb8750 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb8754 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8758 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8764 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb876c | load | 4 | `sp -0x4c` | `-` |
| `ldrb` at 0xb8770 | load | 1 | `[sp-76] +0x0` | `-` |
| `str` at 0xb8784 | store | 4 | `sp -0xe0` | `ret(0x1012ec)` |
| `ldr` at 0xb8788 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0xb8798 | load | 4 | `sp -0xe0` | `-` |
| `ldr` at 0xb87a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb87ac | load | 4 | `sp -0xe0` | `-` |
| `str` at 0xb87b0 | store | 4 | `[sp-40] +0x3a4` | `ret(0x1012ec)` |
| `ldr` at 0xb87b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb87b8 | load | 4 | `[sp-40] +0x28` | `-` |
| `ldr` at 0xb87c8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb87cc | load | 4 | `[sp-40] +0x28` | `-` |
| `ldr` at 0xb87d8 | load | 4 | `abs 0xb8908` | `-` |
| `ldr` at 0xb87e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb87e8 | load | 4 | `[sp-40] +0x28` | `-` |
| `str` at 0xb87f0 | store | 4 | `sp -0xf0` | `?` |
| `ldr` at 0xb87f8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb87fc | store | 4 | `sp -0xf0` | `[sp-24]` |
| `ldr` at 0xb8800 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb8804 | load | 4 | `[sp-40] +0x30` | `-` |
| `ldr` at 0xb8810 | load | 4 | `sp -0xf0` | `-` |
| `ldr` at 0xb881c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb8830 | store | 4 | `sp -0x24` | `ret(0x2302c)` |
| `ldr` at 0xb8834 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8840 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb884c | load | 4 | `abs 0xb890c` | `-` |
| `str` at 0xb8854 | store | 4 | `sp -0xdc` | `ret(0x6e1b0)` |
| `ldr` at 0xb8858 | load | 4 | `sp -0xdc` | `-` |
| `str` at 0xb8868 | store | 4 | `sp -0xd8` | `ret(0x2031c)+1` |
| `ldr` at 0xb886c | load | 4 | `sp -0xd8` | `-` |
| `str` at 0xb8894 | store | 4 | `sp -0xe4` | `?` |
| `ldr` at 0xb8898 | load | 4 | `sp -0xe4` | `-` |
| `ldr` at 0xb889c | load | 4 | `sp -0xdc` | `-` |
| `ldr` at 0xb88a0 | load | 4 | `sp -0xd8` | `-` |
| `ldr` at 0xb88ac | load | 4 | `abs 0xb891c` | `-` |
| `str` at 0xb88b0 | store | 4 | `(unknown)` | `0x16d50c` |
| `str` at 0xb88b4 | store | 4 | `(unknown)` | `ret(0x203f4)` |
| `ldr` at 0xb88b8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb88bc | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0xb88c0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb88c4 | store | 4 | `(unknown)` | `[sp-36]` |
| `ldr` at 0xb88cc | load | 4 | `abs 0xb8920` | `-` |
| `ldr` at 0xb88e0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb88e8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb88ec | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb88f4 | store | 4 | `sp -0x4c` | `[sp-76]+1` |
| `ldm` at 0xb8900 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8900 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8900 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8900 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8524 | `(unknown callee) 0x6e140` | `?` | `[sp-28]` | `?` | `[[sp-40]+48]` |
| `bl` at 0xb8534 | `(unknown callee) 0x6e190` | `ret(0x6e140)` | `?` | `?` | `ret(0x6e140)` |
| `bl` at 0xb85a4 | `(unknown callee) 0xb5c90` | `[sp-24]` | `[sp-28]` | `0xa00` | `?` |
| `bl` at 0xb85c0 | `(unknown callee) 0x134634` | `ret(0xb5c90)` | `?` | `?` | `0x0` |
| `bl` at 0xb85e0 | `(unknown callee) 0x139398` | `[sp-52]` | `sp-72` | `?` | `sp-72` |
| `bl` at 0xb85fc | `(unknown callee) 0x139664` | `[sp-52]` | `sp-72` | `?` | `sp-72` |
| `bl` at 0xb8608 | `(unknown callee) 0x1346fc` | `?` | `?` | `?` | `?` |
| `bl` at 0xb8650 | `(unknown callee) 0x10d500` | `[sp-236]` | `?` | `?` | `?` |
| `bl` at 0xb8670 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x0` | `0x0` | `0x16d4dc` |
| `bl` at 0xb867c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xb86c4 | `(unknown callee) 0x203f4` | `[sp-224]` | `[sp-216]` | `[sp-220]` | `?` |
| `bl` at 0xb86d0 | `(unknown callee) 0x219a4` | `[sp-36]` | `?` | `?` | `?` |
| `bl` at 0xb86f0 | `(unknown callee) 0x205bc` | `sp-212` | `0x16d4e0` | `ret(0x203f4)` | `[sp-28]` |
| `bl` at 0xb86fc | `(unknown callee) 0x293f8` | `sp-212` | `?` | `?` | `sp-212` |
| `bl` at 0xb8718 | `(unknown callee) 0x243d4` | `0x16d4fc` | `sp-212` | `?` | `sp-212` |
| `bl` at 0xb8748 | `(unknown callee) 0xb5db0` | `[sp-24]` | `[sp-28]` | `0x16d500` | `sp-72` |
| `bl` at 0xb877c | `(unknown callee) 0x1012ec` | `[sp-40]+368` | `[[sp-76]]:1` | `[sp-40]+368` | `[[sp-76]]:1` |
| `bl` at 0xb8818 | `(unknown callee) 0x10d2bc` | `?` | `?` | `0xfff` | `?` |
| `bl` at 0xb8828 | `(unknown callee) 0x2302c` | `[sp-40]+56` | `?` | `?` | `[sp-40]+56` |
| `bl` at 0xb8850 | `(unknown callee) 0x6e1b0` | `[sp-24]` | `0x0` | `0x0` | `0x16d4dc` |
| `bl` at 0xb885c | `(unknown callee) 0x2031c` | `ret(0x6e1b0)` | `?` | `?` | `?` |
| `bl` at 0xb88a4 | `(unknown callee) 0x203f4` | `[sp-228]` | `[sp-220]` | `[sp-216]` | `?` |
| `bl` at 0xb88d8 | `(unknown callee) 0x22070` | `0x9` | `0x16cfa4` | `0x440` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8454 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xb8478 | `? <= r1 (signed, 32-bit)` | `? > r1 (signed, 32-bit)` |
| `beq` at 0xb84c4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0xb8530 | `ret(0x6e140) <= 0x1 (signed, 32-bit)` | `ret(0x6e140) > 0x1 (signed, 32-bit)` |
| `beq` at 0xb8544 | `ret(0x6e190) == [sp-24]` | `ret(0x6e190) != [sp-24]` |
| `bne` at 0xb85d0 | `[[sp-40]] != 0x1` | `[[sp-40]] == 0x1` |
| `beq` at 0xb8618 | `? == 0x0` | `? != 0x0` |
| `bne` at 0xb8630 | `? != 0xfff` | `? == 0xfff` |
| `bge` at 0xb865c | `[sp-36] >= 0x0 (signed, 32-bit)` | `[sp-36] < 0x0 (signed, 32-bit)` |
| `beq` at 0xb8708 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `beq` at 0xb8728 | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `bge` at 0xb8760 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `bge` at 0xb8790 | `ret(0x1012ec) >= 0x0 (signed, 32-bit)` | `ret(0x1012ec) < 0x0 (signed, 32-bit)` |
| `bne` at 0xb87a0 | `ret(0x1012ec) != 0x0` | `ret(0x1012ec) == 0x0` |
| `beq` at 0xb87c4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb87e0 | `? == 0xfff` | `? != 0xfff` |
| `beq` at 0xb883c | `ret(0x2302c) == 0x0` | `ret(0x2302c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8900 | `r0` | `?` |

## `SysComEnable`

- `SysComEnable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8924 to 0xb8a6c.
  Established. Two word loads have unknown effective addresses (`ldr` at 0xb897c, `ldr` at 0xb89c4).
  One path calls unnamed 0x10d214 with candidate r0/r1/r2/r3 values unknown/[sp-32]/0xfff/unknown (`bl` at 0xb8a14).
  Another calls unnamed 0x125958, then branches to 0xb8a5c if its result is 0 and falls through to the call to unnamed 0x125d44 otherwise (`bl` at 0xb8a2c, `beq` at 0xb8a38, `bl` at 0xb8a58).
  The loaded fields do not establish which hardware control is enabled.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8934 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8938 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb893c | load | 4 | `abs 0xb8a64` | `-` |
| `ldr` at 0xb8940 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8948 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8950 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8974 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb897c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8980 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb8984 | load | 4 | `abs 0xb8a64` | `-` |
| `ldr` at 0xb8988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8990 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8998 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb89bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb89c4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb89c8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb89cc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb89dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb89e8 | load | 4 | `abs 0xb8a68` | `-` |
| `ldr` at 0xb89f4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb89fc | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb8a04 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8a08 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0xb8a0c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8a10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8a1c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8a24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8a3c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8a44 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8a4c | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8a60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8a60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8a60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8a14 | `(unknown callee) 0x10d214` | `?` | `[sp-32]` | `0xfff` | `?` |
| `bl` at 0xb8a2c | `(unknown callee) 0x125958` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xb8a58 | `(unknown callee) 0x125d44` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb89d8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb89f0 | `? == 0xfff` | `? != 0xfff` |
| `beq` at 0xb8a38 | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8a60 | `r0` | `?` |

## `SysComEnableAll`

- `SysComEnableAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8a6c to 0xb8afc.
  Established. Word stores initialize sp-0x14 and sp-0x18 to 0 (`str` at 0xb8a80, `str` at 0xb8a9c).
  The sole call to unnamed 0xb8924 prints all four register candidates as unknown, followed by a word store of [sp-24]+1 at sp-0x18 (`bl` at 0xb8acc, `str` at 0xb8ad8).
  Backward transfers return to 0xb8aa0 and 0xb8a84, but the loop-limit predicates have unknown operands (`b` at 0xb8adc, `b` at 0xb8aec, `ble` at 0xb8a94, `ble` at 0xb8ac0).
  Hardware enumeration and enable semantics therefore remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8a80 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xb8a84 | load | 4 | `abs 0xb8af8` | `-` |
| `ldr` at 0xb8a88 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xb8a8c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8a9c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb8aa0 | load | 4 | `abs 0xb8af8` | `-` |
| `ldr` at 0xb8aa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8aac | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8ab4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8ab8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8ac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8ac8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8ad0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8ad8 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb8ae0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8ae8 | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0xb8af4 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8af4 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8af4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8acc | `(unknown callee) 0xb8924` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb8a94 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xb8ac0 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8af4 | `r0` | `?` |

## `SysComDisable`

- `SysComDisable`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8afc to 0xb8c44.
  Established. Two word loads have unknown effective addresses, and their saved stack values are unknown (`ldr` at 0xb8b54, `ldr` at 0xb8b9c, `str` at 0xb8b58, `str` at 0xb8ba0).
  The call to unnamed 0x10d2bc prints candidate r1=[sp-32] and r2=0xfff, with r0/r3 unknown (`bl` at 0xb8bec).
  On the other path, a zero result from unnamed 0x125958 branches to 0xb8c34, while a nonzero result falls through to unnamed 0x125cf8 (`bl` at 0xb8c04, `beq` at 0xb8c10, `bl` at 0xb8c30).
  The particular hardware state disabled is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8b0c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8b10 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb8b14 | load | 4 | `abs 0xb8c3c` | `-` |
| `ldr` at 0xb8b18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8b20 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8b4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8b54 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8b58 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb8b5c | load | 4 | `abs 0xb8c3c` | `-` |
| `ldr` at 0xb8b60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8b68 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8b94 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8b9c | load | 4 | `(unknown)` | `-` |
| `str` at 0xb8ba0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb8ba4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8bb4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8bc0 | load | 4 | `abs 0xb8c40` | `-` |
| `ldr` at 0xb8bcc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8bd4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb8bdc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8be0 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0xb8be4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb8be8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8bf4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8bfc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c14 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8c1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb8c24 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xb8c38 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8c38 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8c38 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8bec | `(unknown callee) 0x10d2bc` | `?` | `[sp-32]` | `0xfff` | `?` |
| `bl` at 0xb8c04 | `(unknown callee) 0x125958` | `[sp-20]` | `?` | `?` | `?` |
| `bl` at 0xb8c30 | `(unknown callee) 0x125cf8` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb8bb0 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb8bc8 | `? == 0xfff` | `? != 0xfff` |
| `beq` at 0xb8c10 | `ret(0x125958) == 0x0` | `ret(0x125958) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8c38 | `r0` | `?` |

## `SysComDisableAll`

- `SysComDisableAll`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8c44 to 0xb8cd4.
  Established. Word loads read absolute 0x31b5b0 and 0x31b5b8, while the following word load has unknown effective address (`ldr` at 0xb8c60, `ldr` at 0xb8c84, `ldr` at 0xb8c8c).
  The sole call to unnamed 0xb8afc prints unknown candidates for r0/r1/r2/r3 (`bl` at 0xb8ca4).
  A word store updates sp-0x18 to [sp-24]+1 before a backward transfer to 0xb8c78 (`str` at 0xb8cb0, `b` at 0xb8cb4).
  The unknown loop-limit operands prevent establishing which hardware entries are disabled (`ble` at 0xb8c6c, `ble` at 0xb8c98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8c58 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xb8c5c | load | 4 | `abs 0xb8cd0` | `-` |
| `ldr` at 0xb8c60 | load | 4 | `abs 0x31b5b0` | `-` |
| `ldr` at 0xb8c64 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8c74 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb8c78 | load | 4 | `abs 0xb8cd0` | `-` |
| `ldr` at 0xb8c7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8c84 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8c8c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8c90 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8c9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8ca0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8ca8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb8cb0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0xb8cb8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb8cc0 | store | 4 | `sp -0x14` | `?` |
| `ldm` at 0xb8ccc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8ccc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8ccc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb8ca4 | `(unknown callee) 0xb8afc` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb8c6c | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `ble` at 0xb8c98 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8ccc | `r0` | `?` |

## `SysComIOExpIndex`

- `SysComIOExpIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8cd4 to 0xb8d9c.
  Established. A word load reads absolute 0x31b5b8, while the later word load has unknown effective address (`ldr` at 0xb8d00, `ldr` at 0xb8d58).
  The comparison branch goes to 0xb8d74 when its unknown operand is not equal to [sp-24], and its equal path reaches a store of [sp-28] at sp-0x20 (`bne` at 0xb8d64, `str` at 0xb8d6c).
  The other return path stores an unknown value after unsupported mvn effects (`mvn` at 0xb8d84, `str` at 0xb8d88).
  With no calls emitted, the matching field and any hardware index meaning remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8ce4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8ce8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8cf0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb8cf4 | load | 4 | `abs 0xb8d98` | `-` |
| `ldr` at 0xb8cf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8d00 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8d08 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8d0c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8d18 | load | 4 | `abs 0xb8d98` | `-` |
| `ldr` at 0xb8d1c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8d24 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8d2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8d50 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8d58 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8d5c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8d68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8d6c | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xb8d74 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8d7c | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xb8d88 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb8d8c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xb8d94 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8d94 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8d94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb8d14 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb8d64 | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8d94 | `r0` | `[sp-32]` |

## `SysComDeviceIndex`

- `SysComDeviceIndex`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#6`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb8d9c to 0xb8e64.
  Established. The load at absolute 0x31b5b8 precedes two computed-address loads whose effective addresses are unknown (`ldr` at 0xb8dc8, `ldr` at 0xb8e18, `ldr` at 0xb8e20).
  An unequal comparison takes the branch to 0xb8e3c, where the path increments sp-0x1c using [sp-28]+1 and loops back to 0xb8dbc (`bne` at 0xb8e2c, `str` at 0xb8e44, `b` at 0xb8e48).
  The equal path stores [sp-28] at sp-0x20, and the recorded r0 return is [sp-32] (`str` at 0xb8e34, `ldm` at 0xb8e5c).
  Which device the compared field identifies remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb8dac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb8db0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb8db8 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xb8dbc | load | 4 | `abs 0xb8e60` | `-` |
| `ldr` at 0xb8dc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8dc8 | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8dd0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8dd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8de0 | load | 4 | `abs 0xb8e60` | `-` |
| `ldr` at 0xb8de4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb8dec | load | 4 | `abs 0x31b5b8` | `-` |
| `ldr` at 0xb8df4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb8e18 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8e20 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb8e24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb8e30 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8e34 | store | 4 | `sp -0x20` | `[sp-28]` |
| `ldr` at 0xb8e3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb8e44 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0xb8e50 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xb8e54 | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xb8e5c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xb8e5c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xb8e5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ble` at 0xb8ddc | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0xb8e2c | `? != [sp-24]` | `? == [sp-24]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xb8e5c | `r0` | `[sp-32]` |
