# ARM IPMB packet and checksum helpers in 1.38.4

Read from `6.5.28/usr/cpu/firmware/sysco/l1.bin`, package `6.5.28:l1#4` (version 1.38.4),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53cfa0`.
Carrier SHA-256: `e5b65e5f28d7fba6013a8618f55c34bed7f60f7c6f46d7e9c5b8b7b2eabbe7b3`.
Package SHA-256: `c6693d19c6eb8c3fcf00cbb84d6974d1b5b17b930fb5375c1df2df20894edec9`.
Inner executable SHA-256: `f652c04c77cd7c53ca7ae6d6d6f581f11200efa211ff089bfba0d8bd4f718eb8`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1254136 bytes.
Segment SHA-256: `a373bb774d119f083e52d394fbd2701ebfbf35210301a40dcdb4ebdc39bf6bdc`. Analysed routine spans use the exact
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
SdrDecode has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/ipmb-packet-checksum-helpers-facts.txt) retain all tool sections.

## `ipmiInit`

- `ipmiInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21cb0 to 0x21cc0.
  Established. The only recorded exit preserves entry r0 (`ldmdb` at 0x21cbc). The body establishes
  no external IPMI initialisation effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x21cbc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21cbc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21cbc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21cbc | `r0` | `r0` |

## `ipmiCheckCSum`

- `ipmiCheckCSum`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21cc0 to 0x21d34.
  Established. The byte store clears `sp -0x19` (`strb` at 0x21cdc). The loop branch takes `Z == 0`
  and falls through on `not (Z == 0)` (`bne` at 0x21cf8). A later byte load has an unknown effective
  target (`ldrb` at 0x21d10). The checksum result and input bounds remain unknown (`ldmdb` at
  0x21d30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21cd0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x21cd4 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x21cdc | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x21ce4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x21cf0 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x21d04 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x21d0c | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x21d10 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x21d18 | store | 1 | `sp -0x19` | `?` |
| `str` at 0x21d20 | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x21d28 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x21d30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21d30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21d30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x21cf8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21d30 | `r0` | `?` |

## `ipmiCalcCSum`

- `ipmiCalcCSum`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x21d34 to 0x21dac.
  Established. The stack byte at `sp -0x19` is initialised to `0x0` (`strb` at 0x21d50). The loop
  decision takes `Z == 0` and falls through on `not (Z == 0)` (`bne` at 0x21d64). The later input
  byte load has an unknown target (`ldrb` at 0x21d7c). The final checksum expression is unknown
  (`ldmdb` at 0x21da8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x21d44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x21d48 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0x21d50 | store | 1 | `sp -0x19` | `0x0` |
| `ldr` at 0x21d54 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x21d5c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x21d70 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x21d78 | load | 1 | `sp -0x19` | `-` |
| `ldrb` at 0x21d7c | load | 1 | `(unknown)` | `-` |
| `strb` at 0x21d84 | store | 1 | `sp -0x19` | `?` |
| `str` at 0x21d8c | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x21d94 | load | 1 | `sp -0x19` | `-` |
| `ldmdb` at 0x21da8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x21da8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x21da8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x21d64 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x21da8 | `r0` | `?` |

## `IcmbIpmbRqBuild`

- `IcmbIpmbRqBuild`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae1bc to 0xae24c.
  Established. A byte store writes `((r0 & 0xff) & 0xff)` at `[sp+8] +0x0` (`strb` at 0xae1e8). The
  call to `0x1b304` carries `[sp+8]+3`, `[sp]`, `[sp+4]` and `[sp+8]+3` (`bl` at 0xae238). The
  recorded return is `[sp+4]+3` (`ldmdb` at 0xae248). Packet field semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae1cc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae1d0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xae1d4 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xae1dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xae1e0 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xae1e4 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xae1e8 | store | 1 | `[sp+8] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xae1f0 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xae1f8 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xae204 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xae20c | store | 1 | `[sp+8] +0x1` | `?` |
| `ldr` at 0xae214 | load | 4 | `sp +0x8` | `-` |
| `ldrb` at 0xae21c | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xae220 | store | 1 | `[sp+8] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xae224 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xae230 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae234 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xae23c | load | 4 | `sp +0x4` | `-` |
| `ldmdb` at 0xae248 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae248 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae248 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae238 | `(unknown callee) 0x1b304` | `[sp+8]+3` | `[sp]` | `[sp+4]` | `[sp+8]+3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae248 | `r0` | `[sp+4]+3` |

## `IcmbIpmbRsBuild`

- `IcmbIpmbRsBuild`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae24c to 0xae2f4.
  Established. The opening output byte store targets `[sp+12] +0x0` with `((r0 & 0xff) & 0xff)`
  (`strb` at 0xae278). The call to `0x1b304` uses `[sp+12]+4`, `[sp+4]`, `[sp+8]` and `[sp+12]+4`
  (`bl` at 0xae2e0); its enclosing routine returns `[sp+8]+4` (`ldmdb` at 0xae2f0). Response-field
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae25c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xae260 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xae264 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xae26c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xae270 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xae274 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0xae278 | store | 1 | `[sp+12] +0x0` | `((r0 & 0xff) & 0xff)` |
| `ldr` at 0xae280 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xae288 | load | 1 | `sp -0x1c` | `-` |
| `ldrb` at 0xae294 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xae2a0 | store | 1 | `[sp+12] +0x1` | `?` |
| `ldr` at 0xae2a8 | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xae2b0 | load | 1 | `sp -0x20` | `-` |
| `strb` at 0xae2b4 | store | 1 | `[sp+12] +0x2` | `([sp-32]:1 & 0xff)` |
| `ldr` at 0xae2bc | load | 4 | `sp +0xc` | `-` |
| `ldrb` at 0xae2c4 | load | 1 | `sp +0x0` | `-` |
| `strb` at 0xae2c8 | store | 1 | `[sp+12] +0x3` | `([sp]:1 & 0xff)` |
| `ldr` at 0xae2cc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xae2d8 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0xae2dc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xae2e4 | load | 4 | `sp +0x8` | `-` |
| `ldmdb` at 0xae2f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae2f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae2f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae2e0 | `(unknown callee) 0x1b304` | `[sp+12]+4` | `[sp+4]` | `[sp+8]` | `[sp+12]+4` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae2f0 | `r0` | `[sp+8]+4` |

## `IcmbIpmbGetNetFn`

- `IcmbIpmbGetNetFn`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae2f4 to 0xae334.
  Established. A byte is loaded from `r0 +0x1` (`ldrb` at 0xae314). The word store at `sp -0x18` has
  unknown value (`str` at 0xae324), and the return carries `[sp-24]` (`ldmdb` at 0xae330). The
  network-function encoding remains unknown in the supported facts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae304 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae30c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae314 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xae324 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xae328 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae330 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae330 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae330 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae330 | `r0` | `[sp-24]` |

## `IcmbIpmbGetRqRsId`

- `IcmbIpmbGetRqRsId`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae334 to 0xae374.
  Established. The byte read targets `r0 +0x1` (`ldrb` at 0xae354). A word store records an unknown
  value at `sp -0x18` (`str` at 0xae364). The return expression is `[sp-24]` (`ldmdb` at 0xae370).
  The request/response identifier mapping remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae344 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae34c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae354 | load | 1 | `r0 +0x1` | `-` |
| `str` at 0xae364 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xae368 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae370 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae370 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae370 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae370 | `r0` | `[sp-24]` |

## `IcmbIpmbGetCmd`

- `IcmbIpmbGetCmd`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae374 to 0xae3a8.
  Established. The byte at `r0 +0x2` is loaded and saved as a word at `sp -0x18` (`ldrb` at 0xae394,
  `str` at 0xae398). The recorded return carries `[r0+2]:1` (`ldmdb` at 0xae3a4). The command
  interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae384 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae38c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae394 | load | 1 | `r0 +0x2` | `-` |
| `str` at 0xae398 | store | 4 | `sp -0x18` | `[r0+2]:1` |
| `ldr` at 0xae39c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae3a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae3a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae3a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae3a4 | `r0` | `[r0+2]:1` |

## `IcmbIpmbGetCmpCode`

- `IcmbIpmbGetCmpCode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae3a8 to 0xae3dc.
  Established. The input read fetches a byte at `r0 +0x3` (`ldrb` at 0xae3c8). A word store saves
  `[r0+3]:1` at `sp -0x18`, and the exit retains that byte expression (`str` at 0xae3cc, `ldmdb` at
  0xae3d8). Completion-code meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae3b8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae3c0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae3c8 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xae3cc | store | 4 | `sp -0x18` | `[r0+3]:1` |
| `ldr` at 0xae3d0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xae3d8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae3d8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae3d8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae3d8 | `r0` | `[r0+3]:1` |

## `IcmbIpmbGetData`

- `IcmbIpmbGetData`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae3dc to 0xae434.
  Established. The call to `0xae334` receives entry r0 through r3 as candidates (`bl` at 0xae3f4).
  Nonzero takes the branch and zero falls through (`bne` at 0xae408). The fall-through word store
  writes `[sp-20]+3` at `sp -0x1c`, while the taken arm writes `[sp-20]+4` there (`str` at 0xae414,
  `str` at 0xae424). Message layout semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae3ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae3f0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae3fc | store | 4 | `sp -0x18` | `ret(0xae334)` |
| `ldr` at 0xae400 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xae40c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae414 | store | 4 | `sp -0x1c` | `[sp-20]+3` |
| `ldr` at 0xae41c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae424 | store | 4 | `sp -0x1c` | `[sp-20]+4` |
| `ldr` at 0xae428 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xae430 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae430 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae430 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae3f4 | `(unknown callee) 0xae334` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xae408 | `ret(0xae334) != 0x0` | `ret(0xae334) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae430 | `r0` | `[sp-28]` |

## `IcmbPktBuild`

- `IcmbPktBuild`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae4c8 to 0xae5a0.
  Established. Halfword stores save `(r0 & 0xffff)` at `sp -0x12` and `(r1 & 0xffff)` at `sp -0x14`
  (`strh` at 0xae4dc, `strh` at 0xae4e0). The first call to `0xae6c4` carries `[sp]`, `(r1 &
  0xffff)`, entry r2 and `(r1 & 0xffff)` (`bl` at 0xae500). The return is `[sp-26]:1` (`ldmdb` at
  0xae59c). Packet length and checksum interpretations remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae4d8 | store | 4 | `sp -0x18` | `r2` |
| `strh` at 0xae4dc | store | 2 | `sp -0x12` | `(r0 & 0xffff)` |
| `strh` at 0xae4e0 | store | 2 | `sp -0x14` | `(r1 & 0xffff)` |
| `strb` at 0xae4e4 | store | 1 | `sp -0x19` | `(r3 & 0xff)` |
| `ldrb` at 0xae4e8 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0xae4f0 | store | 1 | `sp -0x1a` | `((r3 & 0xff)+7 & 0xff)` |
| `ldrh` at 0xae4f4 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xae4f8 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xae504 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xae50c | load | 2 | `sp -0x12` | `-` |
| `ldr` at 0xae520 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xae52c | store | 1 | `[sp] +0x4` | `0x0` |
| `ldr` at 0xae530 | load | 4 | `sp +0x0` | `-` |
| `ldrb` at 0xae538 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0xae540 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xae548 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xae54c | load | 4 | `sp +0x0` | `-` |
| `strh` at 0xae55c | store | 2 | `sp -0x1c` | `(ret(0xae434) & 0xffff)` |
| `ldrb` at 0xae560 | load | 1 | `sp -0x1a` | `-` |
| `ldr` at 0xae564 | load | 4 | `sp +0x0` | `-` |
| `ldrh` at 0xae578 | load | 2 | `sp -0x1c` | `-` |
| `ldrb` at 0xae594 | load | 1 | `sp -0x1a` | `-` |
| `ldmdb` at 0xae59c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae59c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae59c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae500 | `(unknown callee) 0xae6c4` | `[sp]` | `(r1 & 0xffff)` | `r2` | `(r1 & 0xffff)` |
| `bl` at 0xae518 | `(unknown callee) 0xae6c4` | `[sp]+2` | `[sp-18]:2` | `[sp-18]:2` | `[sp]+2` |
| `bl` at 0xae544 | `(unknown callee) 0x1b304` | `[sp]+5` | `[sp-24]` | `[sp-25]:1` | `[sp]+5` |
| `bl` at 0xae554 | `(unknown callee) 0xae434` | `[sp]` | `[sp-26]:1` | `?` | `[sp-26]:1` |
| `bl` at 0xae590 | `(unknown callee) 0xae6c4` | `([sp-26]:1 + [sp])-2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae59c | `r0` | `[sp-26]:1` |

## `IcmbPktDecode`

- `IcmbPktDecode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae5a0 to 0xae66c.
  Established. The first call targets `0xae434` with entry r0, `(r1 & 0xff)`, entry r2 and `(r1 &
  0xff)` (`bl` at 0xae5d0). The branch takes `([sp-36]:2 + (ret(0xae66c) & 0xffff)) == 0xffff` and
  falls through on inequality (`beq` at 0xae610). The final r0 value and validity semantics remain
  unknown (`ldmdb` at 0xae664).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae5b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xae5b4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xae5b8 | store | 4 | `sp -0x20` | `r3` |
| `strb` at 0xae5c0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xae5c4 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xae5c8 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xae5d8 | store | 2 | `sp -0x24` | `(ret(0xae434) & 0xffff)` |
| `ldrb` at 0xae5dc | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xae5e0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xae5f8 | store | 2 | `sp -0x26` | `(ret(0xae66c) & 0xffff)` |
| `ldrh` at 0xae5fc | load | 2 | `sp -0x24` | `-` |
| `ldrh` at 0xae600 | load | 2 | `sp -0x26` | `-` |
| `ldr` at 0xae608 | load | 4 | `abs 0xae668` | `-` |
| `str` at 0xae618 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xae620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xae634 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xae638 | store | 2 | `[sp-28] +0x0` | `(ret(0xae66c) & 0xffff)` |
| `ldrb` at 0xae63c | load | 1 | `sp -0x15` | `-` |
| `strb` at 0xae644 | store | 1 | `sp -0x21` | `([sp-21]:1-7 & 0xff)` |
| `ldr` at 0xae648 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xae64c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xae654 | store | 4 | `[sp-32] +0x0` | `[sp-20]+5` |
| `ldrb` at 0xae658 | load | 1 | `sp -0x21` | `-` |
| `str` at 0xae65c | store | 4 | `sp -0x2c` | `[sp-33]:1` |
| `ldr` at 0xae660 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xae664 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae664 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae664 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xae5d0 | `(unknown callee) 0xae434` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xae5f0 | `(unknown callee) 0xae66c` | `([sp-21]:1 + [sp-20])-2` | `?` | `[sp-21]:1` | `([sp-21]:1 + [sp-20])-2` |
| `bl` at 0xae62c | `(unknown callee) 0xae66c` | `[sp-20]+2` | `?` | `([sp-36]:2 + (ret(0xae66c) & 0xffff))` | `[sp-20]+2` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xae610 | `([sp-36]:2 + (ret(0xae66c) & 0xffff)) == 0xffff` | `([sp-36]:2 + (ret(0xae66c) & 0xffff)) != 0xffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae664 | `r0` | `?` |

## `IcmbCvtToU16`

- `IcmbCvtToU16`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xae66c to 0xae6c4.
  Established. Byte reads access `r0 +0x1` and `r0 +0x0` (`ldrb` at 0xae68c, `ldrb` at 0xae6a4). A
  halfword store at `sp -0x16` has unknown value after the shift (`strh` at 0xae69c). The return
  carries `[sp-22]:2` (`ldmdb` at 0xae6c0). The reconstructed numeric value remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xae67c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xae684 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae68c | load | 1 | `r0 +0x1` | `-` |
| `strh` at 0xae690 | store | 2 | `sp -0x16` | `([r0+1]:1 & 0xffff)` |
| `ldrh` at 0xae694 | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xae69c | store | 2 | `sp -0x16` | `?` |
| `ldr` at 0xae6a0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xae6a4 | load | 1 | `r0 +0x0` | `-` |
| `ldrh` at 0xae6ac | load | 2 | `sp -0x16` | `-` |
| `strh` at 0xae6b4 | store | 2 | `sp -0x16` | `?` |
| `ldrh` at 0xae6b8 | load | 2 | `sp -0x16` | `-` |
| `ldmdb` at 0xae6c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xae6c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xae6c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xae6c0 | `r0` | `[sp-22]:2` |

## `SdrDecode`

- `SdrDecode`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacf68 to 0xad01c.
  Established. The byte read fetches `r0 +0x3`, then its value is saved as a word at `sp -0x1c`
  (`ldrb` at 0xacf88, `str` at 0xacf8c). The selector takes `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)`
  and falls through above that bound (`ldrls` at 0xacf9c); its taken edges remain unresolved. Entry
  r0 is the known-edge exit value (`ldmdb` at 0xad018). Selected decode effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacf78 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xacf7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacf80 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xacf84 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xacf88 | load | 1 | `r0 +0x3` | `-` |
| `str` at 0xacf8c | store | 4 | `sp -0x1c` | `[r0+3]:1` |
| `ldr` at 0xacf90 | load | 4 | `sp -0x1c` | `-` |
| `ldrls` at 0xacf9c | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xad018 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xad018 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xad018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xacf9c | `[r0+3]:1-1 <= 0x11 (unsigned, 32-bit)` | `[r0+3]:1-1 > 0x11 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xad018 | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xacf9c).

## `SysComEventDisable`

- `SysComEventDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x85a0c to 0x85a38.
  Established. The word store at absolute `0x2b172c` has unknown value (`str` at 0x85a20). The call
  to `0x1d38c` receives `0x13`, `0x0`, `0x2b1720` and unknown (`bl` at 0x85a2c). The recorded exit
  carries `ret(0x1d38c)` (`ldmdb` at 0x85a30). The event-disable effect and polarity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x85a18 | load | 4 | `abs 0x85a34` | `-` |
| `str` at 0x85a20 | store | 4 | `abs 0x2b172c` | `?` |
| `ldmdb` at 0x85a30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x85a30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x85a30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x85a2c | `(unknown callee) 0x1d38c` | `0x13` | `0x0` | `0x2b1720` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x85a30 | `r0` | `ret(0x1d38c)` |
