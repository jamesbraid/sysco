# ARM power margin and reset callers in 1.40.5

Read from `6.5.29/usr/cpu/firmware/sysco/l1.bin`, package `6.5.29:l1#4` (version 1.40.5),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d448`.
Carrier SHA-256: `80f4a616601d187bc3839a2758f2e7f2d0db962ad7fd8f2d96d41bcd15bdf88c`.
Package SHA-256: `4ccd1a6e7054bc3cd56e962a60737d1c90339ffa93fcc8af0912b6c9c7d385dd`.
Inner executable SHA-256: `08ebc648bb298017c7e3d6049983f2a54a8fe56f1bb5738df88b82c38f75ac3a`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1257088 bytes.
Segment SHA-256: `dc7cc5d18fbdf816d328411825c9c01e26c3f8852f1012c7a642aea0767e2665`. Analysed routine spans use the exact
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
These routines have no unresolved computed transfers in the fact sheets. Numeric callee effects remain unknown.

[Fact sheets and full listings](disasm/arm-1.40.5-package4/power-margin-state-facts.txt) retain all tool sections.

## `powerResetStart`

- `powerResetStart`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40bc8 to 0x40be0.
  Established. The only explicit store retains four-byte `r0` at `sp -0x14` (`str` at 0x40bd8). The
  return carries entry `r0` (`ldmdb` at 0x40bdc). A reset-start action is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40bd8 | store | 4 | `sp -0x14` | `r0` |
| `ldmdb` at 0x40bdc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40bdc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40bdc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40bdc | `r0` | `r0` |

## `powerResetDone`

- `powerResetDone`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40be0 to 0x40c24.
  Established. A four-byte load reads absolute `0x148d00` (`ldr` at 0x40c0c). The later byte store
  writes `0x0` to an unresolved effective address (`strb` at 0x40c18). Entry `r0` survives as the
  return expression (`ldmdb` at 0x40c1c). Reset-completion flag semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40bf0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x40bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40c08 | load | 4 | `abs 0x40c20` | `-` |
| `ldr` at 0x40c0c | load | 4 | `abs 0x148d00` | `-` |
| `strb` at 0x40c18 | store | 1 | `(unknown)` | `0x0` |
| `ldmdb` at 0x40c1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40c1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40c1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40c1c | `r0` | `r0` |

## `powerUpInProgress`

- `powerUpInProgress`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40c24 to 0x40c68.
  Established. The absolute four-byte read at `0x148d00` precedes a byte load with unresolved
  effective address (`ldr` at 0x40c50, `ldrb` at 0x40c58). The returned value is unknown (`ldmdb` at
  0x40c60). The body does not establish a power-progress flag address.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40c34 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x40c38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40c4c | load | 4 | `abs 0x40c64` | `-` |
| `ldr` at 0x40c50 | load | 4 | `abs 0x148d00` | `-` |
| `ldrb` at 0x40c58 | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x40c60 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40c60 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40c60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40c60 | `r0` | `?` |

## `powerUpFirstReset`

- `powerUpFirstReset`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40c68 to 0x40cac.
  Established. The four-byte absolute load reads `0x148d00` (`ldr` at 0x40c94). A later byte read
  has unknown effective address (`ldrb` at 0x40c9c), and the exit value remains unknown (`ldmdb` at
  0x40ca4). A first-reset interpretation is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40c78 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x40c7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40c90 | load | 4 | `abs 0x40ca8` | `-` |
| `ldr` at 0x40c94 | load | 4 | `abs 0x148d00` | `-` |
| `ldrb` at 0x40c9c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x40ca4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40ca4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40ca4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40ca4 | `r0` | `?` |

## `powerGetMarginValue`

- `powerGetMarginValue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4121c to 0x41254.
  Established. The byte read targets `r1 +0x10` (`ldrb` at 0x41238). The unnamed call to `0x1f26c`
  carries `[r1+16]:1+130`, `r1`, `r2`, `[r1+16]:1+130` (`bl` at 0x41244). The return preserves
  `ret(0x1f26c)` (`ldmdb` at 0x41250). The selected value and storage medium have Unknown roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4122c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41230 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x41234 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41238 | load | 1 | `r1 +0x10` | `-` |
| `ldmdb` at 0x41250 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41250 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41250 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41244 | `(unknown callee) 0x1f26c` | `[r1+16]:1+130` | `r1` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41250 | `r0` | `ret(0x1f26c)` |

## `powerSetMarginValue`

- `powerSetMarginValue`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41254 to 0x4128c.
  Established. After reading a byte at `r1 +0x10`, the routine calls unnamed `0x1d41c` with
  `[r1+16]:1+130`, `r2`, `r2`, `[r1+16]:1+130` (`ldrb` at 0x41274, `bl` at 0x41284); its exit
  carries `ret(0x1d41c)` (`ldmdb` at 0x41288). A persistent margin update is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x41264 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x41268 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x4126c | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x41270 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x41274 | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x41280 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x41288 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41288 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41288 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41284 | `(unknown callee) 0x1d41c` | `[r1+16]:1+130` | `r2` | `r2` | `[r1+16]:1+130` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41288 | `r0` | `ret(0x1d41c)` |

## `powerGetMarginState`

- `powerGetMarginState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4128c to 0x412c4.
  Established. A byte load reads `r1 +0x10` (`ldrb` at 0x412a8). The call to unnamed `0x1f26c` uses
  `[r1+16]:1+20`, `r1`, `r2`, `[r1+16]:1+20` (`bl` at 0x412b4). The exit retains `ret(0x1f26c)`
  (`ldmdb` at 0x412c0). State numbering and backing storage are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4129c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x412a0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x412a4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x412a8 | load | 1 | `r1 +0x10` | `-` |
| `ldmdb` at 0x412c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x412c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x412c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x412b4 | `(unknown callee) 0x1f26c` | `[r1+16]:1+20` | `r1` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x412c0 | `r0` | `ret(0x1f26c)` |

## `powerSetMarginState`

- `powerSetMarginState`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x412c4 to 0x412fc.
  Established. The explicit byte read is at `r1 +0x10` (`ldrb` at 0x412e4). The unnamed call to
  `0x1d41c` has candidates `[r1+16]:1+20`, `r2`, `r2`, `[r1+16]:1+20` (`bl` at 0x412f4). The
  returned expression is `ret(0x1d41c)` (`ldmdb` at 0x412f8). The destination effect of that call
  remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x412d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x412d8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x412dc | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x412e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x412e4 | load | 1 | `r1 +0x10` | `-` |
| `ldr` at 0x412f0 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x412f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x412f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x412f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x412f4 | `(unknown callee) 0x1d41c` | `[r1+16]:1+20` | `r2` | `r2` | `[r1+16]:1+20` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x412f8 | `r0` | `ret(0x1d41c)` |

## `powerLowMargin`

- `powerLowMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40874 to 0x4089c.
  Established. A byte load at `r1 +0xf` supplies return expression `[r1+15]:1` (`ldrb` at 0x40890,
  `ldmdb` at 0x40898). The explicit stack stores retain four-byte `r0` and `r1` at `sp -0x14` and
  `sp -0x18` (`str` at 0x40884, `str` at 0x40888). The byte's low-margin unit is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40884 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40888 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4088c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x40890 | load | 1 | `r1 +0xf` | `-` |
| `ldmdb` at 0x40898 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40898 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40898 | `r0` | `[r1+15]:1` |

## `powerNormalMargin`

- `powerNormalMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4089c to 0x408c4.
  Established. The byte read at `r1 +0xd` leads to return expression `[r1+13]:1` (`ldrb` at 0x408b8,
  `ldmdb` at 0x408c0). Four-byte entry values `r0` and `r1` are retained at
  `sp -0x14` and `sp -0x18` (`str` at 0x408ac, `str` at 0x408b0). Nominal-margin semantics remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x408ac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x408b0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x408b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x408b8 | load | 1 | `r1 +0xd` | `-` |
| `ldmdb` at 0x408c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x408c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x408c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x408c0 | `r0` | `[r1+13]:1` |

## `powerHighMargin`

- `powerHighMargin`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x408c4 to 0x408ec.
  Established. The retained entry values occupy four-byte slots at `sp -0x14` and `sp -0x18` (`str` at
  0x408d4, `str` at 0x408d8). A byte read at `r1 +0xe` supplies `[r1+14]:1` to the exit (`ldrb` at
  0x408e0, `ldmdb` at 0x408e8). The high-margin field role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x408d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x408d8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x408dc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x408e0 | load | 1 | `r1 +0xe` | `-` |
| `ldmdb` at 0x408e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x408e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x408e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x408e8 | `r0` | `[r1+14]:1` |

## `powerLowVoltage`

- `powerLowVoltage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x405f4 to 0x406a4.
  Established. The unnamed call to `0x40754` uses entry `r0`, `r1`, `r2`, `r3` (`bl` at 0x40614). A
  later call to `0x407e0` carries `[sp-32]`, `[sp-36]`, unknown, unknown (`bl` at 0x40634). The two
  four-byte accesses at the multiple-load instruction have unresolved effective addresses (`ldm` at
  0x40650). The last call to `0xfbcec` carries `ret(0xfb690)`, unknown, unknown, `ret(0xfb690)`
  (`bl` at 0x4068c). Voltage units and conversion semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40604 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x40608 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x4060c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x40610 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4062c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x40630 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x40650 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x40650 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40698 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x40614 | `(unknown callee) 0x40754` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x40620 | `(unknown callee) 0xfb644` | `ret(0x40754)` | `?` | `?` | `ret(0x40754)` |
| `bl` at 0x40634 | `(unknown callee) 0x407e0` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x40640 | `(unknown callee) 0xfb644` | `ret(0x407e0)` | `?` | `?` | `ret(0x407e0)` |
| `bl` at 0x4065c | `(unknown callee) 0xfb308` | `?` | `?` | `ret(0xfb644)` | `?` |
| `bl` at 0x40678 | `(unknown callee) 0xfb690` | `ret(0xfb644)` | `?` | `ret(0xfb308)` | `?` |
| `bl` at 0x4068c | `(unknown callee) 0xfbcec` | `ret(0xfb690)` | `?` | `?` | `ret(0xfb690)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40698 | `r0` | `ret(0xfbcec)` |

## `powerNominalVoltage`

- `powerNominalVoltage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x40754 to 0x407e0.
  Established. The four-byte absolute load reads `0x148d00` (`ldr` at 0x40784). Subsequent byte and
  four-byte reads both have unresolved effective addresses (`ldrb` at 0x407b4, `ldr` at 0x407cc).
  The exit value is unknown (`ldmdb` at 0x407d4). A voltage table or its units cannot be established
  from those unresolved reads.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x40764 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x40768 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4076c | load | 4 | `abs 0x407d8` | `-` |
| `ldr` at 0x40770 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40784 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x4078c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x40790 | load | 4 | `abs 0x407dc` | `-` |
| `ldr` at 0x40794 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40798 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x407ac | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x407b4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x407cc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x407d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x407d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x407d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x407d4 | `r0` | `?` |

## `powerHighVoltage`

- `powerHighVoltage`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x406a4 to 0x40754.
  Established. The first unnamed call targets `0x40754` with `r0`, `r1`, `r2`, `r3` (`bl` at
  0x406c4). A later unnamed call to `0xfb30c` has `ret(0xfb644)` followed by three unknown
  candidates (`bl` at 0x4070c). Both four-byte reads at the multiple-load instruction have
  unresolved effective addresses (`ldm` at 0x40708). The final return carries `ret(0xfbcec)`
  (`ldmdb` at 0x40748). High-voltage calculations remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x406b4 | store | 4 | `sp -0x20` | `r0` |
| `str` at 0x406b8 | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0x406bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x406c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x406dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x406e0 | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0x40708 | load | 4 | `(unknown)` | `-` |
| `ldm` at 0x40708 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40748 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x406c4 | `(unknown callee) 0x40754` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x406d0 | `(unknown callee) 0xfb644` | `ret(0x40754)` | `?` | `?` | `ret(0x40754)` |
| `bl` at 0x406e4 | `(unknown callee) 0x407e0` | `[sp-32]` | `[sp-36]` | `?` | `?` |
| `bl` at 0x406f0 | `(unknown callee) 0xfb644` | `ret(0x407e0)` | `?` | `?` | `ret(0x407e0)` |
| `bl` at 0x4070c | `(unknown callee) 0xfb30c` | `ret(0xfb644)` | `?` | `?` | `?` |
| `bl` at 0x40728 | `(unknown callee) 0xfb690` | `ret(0xfb644)` | `?` | `ret(0xfb30c)` | `?` |
| `bl` at 0x4073c | `(unknown callee) 0xfbcec` | `ret(0xfb690)` | `?` | `?` | `ret(0xfb690)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40748 | `r0` | `ret(0xfbcec)` |

## `powerMarginPercent`

- `powerMarginPercent`, `6.5.29/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.29:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x407e0 to 0x40874.
  Established. The branch takes `? == 0xe` to `0x4084c` (`beq` at 0x40844); its taken arm assigns
  four-byte `0x3cdd67ba` at `sp -0x20`, while the other arm assigns four-byte `0x3d4ccccd` there
  (`str` at 0x40850, `str` at 0x4085c). The exit reports `[sp-32]` (`ldmdb` at 0x40864). The
  constants' units and percentage interpretation are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x407f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x407f4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x407f8 | load | 4 | `abs 0x40868` | `-` |
| `ldr` at 0x407fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x40810 | load | 4 | `abs 0x148d00` | `-` |
| `str` at 0x40818 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x4081c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x40820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x40834 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldrb` at 0x4083c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x4084c | load | 4 | `abs 0x4086c` | `-` |
| `str` at 0x40850 | store | 4 | `sp -0x20` | `0x3cdd67ba` |
| `ldr` at 0x40858 | load | 4 | `abs 0x40870` | `-` |
| `str` at 0x4085c | store | 4 | `sp -0x20` | `0x3d4ccccd` |
| `ldr` at 0x40860 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x40864 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x40864 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x40864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x40844 | `? == 0xe` | `? != 0xe` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x40864 | `r0` | `[sp-32]` |
