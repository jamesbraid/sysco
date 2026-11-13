# ARM NMI, PROM and configuration helpers in 1.38.4

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
Accesses and return values cover known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/nmi-prom-configuration-helpers-facts.txt) retain all tool sections.

## `nmi`

- `nmi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e4d8 to 0x4e558.
  Established. The first call targets `0x4e650` with entry r0, entry r1, entry r2 and entry r1 as
  candidates (`bl` at 0x4e4f8). A word read uses absolute `0x2ae5d4` (`ldr` at 0x4e510). Zero takes
  the branch and nonzero falls through (`beq` at 0x4e518). NMI delivery and the merged return remain
  unknown (`ldmdb` at 0x4e54c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e4e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x4e4f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x4e4f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e500 | load | 4 | `abs 0x4e550` | `-` |
| `ldr` at 0x4e50c | load | 4 | `abs 0x4e554` | `-` |
| `ldr` at 0x4e510 | load | 4 | `abs 0x2ae5d4` | `-` |
| `ldr` at 0x4e51c | load | 4 | `abs 0x4e554` | `-` |
| `ldrb` at 0x4e520 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x4e524 | load | 4 | `abs 0x2ae5d4` | `-` |
| `str` at 0x4e538 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4e544 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4e548 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x4e54c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e54c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e54c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x4e4f8 | `(unknown callee) 0x4e650` | `r0` | `r1` | `r2` | `r1` |
| `bl` at 0x4e508 | `(unknown callee) 0x2070c` | `0x10a5e0` | `ret(0x4e650)` | `?` | `ret(0x4e650)` |
| `mov` at 0x4e530 | `(unknown callee) (unknown target)` | `[sp-21]:1` | `?` | `[sp-21]:1` | `[0x2ae5d4]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x4e518 | `[0x2ae5d4] == 0x0` | `[0x2ae5d4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e54c | `r0` | `?` |

## `nmiMaskSet`

- `nmiMaskSet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e558 to 0x4e5dc.
  Established. The first branch takes `Z == 0` and falls through on `not (Z == 0)` (`bne` at
  0x4e578). Word stores at absolute `0x148240` and `0x148244` all retain unknown values (`str` at
  0x4e584, `str` at 0x4e59c, `str` at 0x4e5b4, `str` at 0x4e5cc). The mask-bit definitions and
  sentinel interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e568 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4e56c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4e570 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e57c | load | 4 | `abs 0x4e5d4` | `-` |
| `str` at 0x4e584 | store | 4 | `abs 0x148240` | `?` |
| `ldr` at 0x4e58c | load | 4 | `abs 0x4e5d4` | `-` |
| `ldr` at 0x4e594 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x4e59c | store | 4 | `abs 0x148240` | `?` |
| `ldr` at 0x4e5a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e5ac | load | 4 | `abs 0x4e5d8` | `-` |
| `str` at 0x4e5b4 | store | 4 | `abs 0x148244` | `?` |
| `ldr` at 0x4e5bc | load | 4 | `abs 0x4e5d8` | `-` |
| `ldr` at 0x4e5c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4e5cc | store | 4 | `abs 0x148244` | `?` |
| `ldmdb` at 0x4e5d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e5d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e5d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x4e578 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x4e5a8 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e5d0 | `r0` | `r0` |

## `nmiMaskClr`

- `nmiMaskClr`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e5dc to 0x4e60c.
  Established. Two word stores clear absolute `0x148240` and `0x148244` (`str` at 0x4e5f0, `str` at
  0x4e5fc). Entry r0 is preserved at the recorded return (`ldmdb` at 0x4e600). The two cleared
  words' hardware masking effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x4e5e8 | load | 4 | `abs 0x4e604` | `-` |
| `str` at 0x4e5f0 | store | 4 | `abs 0x148240` | `0x0` |
| `ldr` at 0x4e5f4 | load | 4 | `abs 0x4e608` | `-` |
| `str` at 0x4e5fc | store | 4 | `abs 0x148244` | `0x0` |
| `ldmdb` at 0x4e600 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e600 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e600 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e600 | `r0` | `r0` |

## `nmiMaskGet`

- `nmiMaskGet`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4e60c to 0x4e650.
  Established. Word stores copy `[0x148240]` to `r0 +0x0` and `[0x148244]` to `[sp-24] +0x0` (`str`
  at 0x4e630, `str` at 0x4e640). The return carries entry r0 (`ldmdb` at 0x4e644). The mask
  encodings are unknown despite the established copies.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x4e61c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x4e620 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x4e624 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4e628 | load | 4 | `abs 0x4e648` | `-` |
| `ldr` at 0x4e62c | load | 4 | `abs 0x148240` | `-` |
| `str` at 0x4e630 | store | 4 | `r0 +0x0` | `[0x148240]` |
| `ldr` at 0x4e634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4e638 | load | 4 | `abs 0x4e64c` | `-` |
| `ldr` at 0x4e63c | load | 4 | `abs 0x148244` | `-` |
| `str` at 0x4e640 | store | 4 | `[sp-24] +0x0` | `[0x148244]` |
| `ldmdb` at 0x4e644 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4e644 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4e644 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4e644 | `r0` | `r0` |

## `promIceInit`

- `promIceInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44a5c to 0x44b28.
  Established. The word read accesses absolute `0x2b0720` (`ldr` at 0x44a78). Zero takes the branch
  and nonzero falls through (`beq` at 0x44a80). The opening call to `0x21e28` carries `0x9`,
  `0x109110`, `0x39` and `0x0` (`bl` at 0x44a9c). The external initialisation effect remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x44a6c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x44a70 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0x44a74 | load | 4 | `abs 0x44b10` | `-` |
| `ldr` at 0x44a78 | load | 4 | `abs 0x2b0720` | `-` |
| `ldr` at 0x44a84 | load | 4 | `abs 0x44b14` | `-` |
| `str` at 0x44a88 | store | 4 | `sp -0x20` | `0x1090ec` |
| `ldr` at 0x44a90 | load | 4 | `abs 0x44b18` | `-` |
| `ldr` at 0x44aa0 | load | 4 | `abs 0x44b1c` | `-` |
| `ldr` at 0x44aa4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44ab4 | store | 4 | `abs 0x2b0740` | `ret(0x1b3c4)` |
| `ldr` at 0x44ab8 | load | 4 | `abs 0x2b0740` | `-` |
| `ldr` at 0x44ac4 | load | 4 | `abs 0x44b20` | `-` |
| `str` at 0x44ac8 | store | 4 | `sp -0x20` | `0x109128` |
| `ldr` at 0x44ad0 | load | 4 | `abs 0x44b18` | `-` |
| `ldr` at 0x44ae0 | load | 4 | `abs 0x44b1c` | `-` |
| `ldr` at 0x44ae4 | load | 4 | `abs 0x2b0740` | `-` |
| `ldr` at 0x44aec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44af4 | load | 4 | `abs 0x44b10` | `-` |
| `ldr` at 0x44af8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x44afc | store | 4 | `abs 0x2b0720` | `[sp-24]` |
| `ldr` at 0x44b00 | load | 4 | `abs 0x44b24` | `-` |
| `ldr` at 0x44b04 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44b08 | store | 4 | `abs 0x2b0700` | `[sp-28]` |
| `ldmdb` at 0x44b0c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x44b0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44b0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44b0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44a9c | `(unknown callee) 0x21e28` | `0x9` | `0x109110` | `0x39` | `0x0` |
| `bl` at 0x44aac | `(unknown callee) 0x1b3c4` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x44adc | `(unknown callee) 0x21e28` | `0x9` | `0x109110` | `0x40` | `0x0` |
| `bl` at 0x44af0 | `(unknown callee) 0x1b43c` | `[0x2b0740]` | `0x1` | `[sp-28]` | `0x2b0740` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x44a80 | `[0x2b0720] == 0x0` | `[0x2b0720] != 0x0` |
| `bne` at 0x44ac0 | `[0x2b0740] != 0x0` | `[0x2b0740] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44b0c | `r0` | `ret(0x1b43c)` |

## `promIceNotPresent`

- `promIceNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44b28 to 0x44b78.
  Established. The signed bound check takes `(r0 & 0xff) >= [0x2b0700] (signed, 32-bit)` and falls
  through below the bound (`bge` at 0x44b50). A fall-through byte store clears `((r0 & 0xff) +
  [0x2b0740]) +0x0` (`strb` at 0x44b68). The physical presence meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44b3c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x44b40 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44b44 | load | 4 | `abs 0x44b70` | `-` |
| `ldr` at 0x44b48 | load | 4 | `abs 0x2b0700` | `-` |
| `ldrb` at 0x44b54 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44b58 | load | 4 | `abs 0x44b74` | `-` |
| `ldr` at 0x44b5c | load | 4 | `abs 0x2b0740` | `-` |
| `strb` at 0x44b68 | store | 1 | `((r0 & 0xff) + [0x2b0740]) +0x0` | `0x0` |
| `ldmdb` at 0x44b6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44b6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44b6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x44b50 | `(r0 & 0xff) >= [0x2b0700] (signed, 32-bit)` | `(r0 & 0xff) < [0x2b0700] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44b6c | `r0` | `r0` |

## `promIceGetCfg`

- `promIceGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44ca8 to 0x44ce4.
  Established. A word store writes `[0x2b0720]` at `r0 +0x0` (`str` at 0x44cc8). The exit carries
  `[0x2b0700]` (`ldmdb` at 0x44cd8). The configuration representation and count meaning remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x44cb8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x44cbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x44cc0 | load | 4 | `abs 0x44cdc` | `-` |
| `ldr` at 0x44cc4 | load | 4 | `abs 0x2b0720` | `-` |
| `str` at 0x44cc8 | store | 4 | `r0 +0x0` | `[0x2b0720]` |
| `ldr` at 0x44ccc | load | 4 | `abs 0x44ce0` | `-` |
| `ldr` at 0x44cd0 | load | 4 | `abs 0x2b0700` | `-` |
| `ldmdb` at 0x44cd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44cd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44cd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44cd8 | `r0` | `[0x2b0700]` |

## `promIceCount`

- `promIceCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44ce4 to 0x44d04.
  Established. The word read uses absolute `0x2b0700` (`ldr` at 0x44cf4). The same global expression
  is returned as `[0x2b0700]` (`ldmdb` at 0x44cfc). The body alone does not establish what objects
  it counts.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x44cf0 | load | 4 | `abs 0x44d00` | `-` |
| `ldr` at 0x44cf4 | load | 4 | `abs 0x2b0700` | `-` |
| `ldmdb` at 0x44cfc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44cfc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44cfc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44cfc | `r0` | `[0x2b0700]` |

## `promIcePresent`

- `promIcePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44d04 to 0x44d64.
  Established. A word load reads `0x2b0700` (`ldr` at 0x44d24). The signed branch takes
  `(r0 & 0xff) >= [0x2b0700] (signed, 32-bit)` and falls through below the bound (`bge` at 0x44d2c).
  The merged return and physical presence encoding remain unknown (`ldmdb` at 0x44d58).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44d18 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x44d1c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44d20 | load | 4 | `abs 0x44d5c` | `-` |
| `ldr` at 0x44d24 | load | 4 | `abs 0x2b0700` | `-` |
| `ldrb` at 0x44d30 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x44d34 | load | 4 | `abs 0x44d60` | `-` |
| `ldr` at 0x44d38 | load | 4 | `abs 0x2b0740` | `-` |
| `ldrb` at 0x44d40 | load | 1 | `((r0 & 0xff) + [0x2b0740]) +0x0` | `-` |
| `str` at 0x44d44 | store | 4 | `sp -0x18` | `[((r0 & 0xff) + [0x2b0740])]:1` |
| `str` at 0x44d50 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x44d54 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x44d58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44d58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44d58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x44d2c | `(r0 & 0xff) >= [0x2b0700] (signed, 32-bit)` | `(r0 & 0xff) < [0x2b0700] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44d58 | `r0` | `?` |

## `promIceEnable`

- `promIceEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44e78 to 0x44f4c.
  Established. The selector takes `(r0 & 0xff) != 0xff` and falls through on equality (`bne` at
  0x44ea0). The first call to `0x44d04` has unknown, unknown, `[0x2b0700]` and unknown as candidates
  (`bl` at 0x44ecc). The enable mechanism and final r0 expression remain unknown (`ldmdb` at
  0x44f44).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44e8c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x44e94 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x44e98 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x44ea8 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x44eac | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x44eb0 | load | 4 | `abs 0x44f48` | `-` |
| `ldr` at 0x44eb4 | load | 4 | `abs 0x2b0700` | `-` |
| `ldrb` at 0x44ec4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x44edc | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x44eec | store | 4 | `sp -0x1c` | `ret(0x45020)` |
| `ldr` at 0x44ef0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44efc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x44f08 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44f0c | store | 4 | `sp -0x18` | `ret(0x45020)` |
| `ldrb` at 0x44f10 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x44f18 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x44f20 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x44f24 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x44f2c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x44f3c | store | 4 | `sp -0x24` | `ret(0x45020)` |
| `ldr` at 0x44f40 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x44f44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x44f44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x44f44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44ecc | `(unknown callee) 0x44d04` | `?` | `?` | `[0x2b0700]` | `?` |
| `bl` at 0x44ee4 | `(unknown callee) 0x45020` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x44f34 | `(unknown callee) 0x45020` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x44ea0 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x44ebc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x44ed8 | `ret(0x44d04) == 0x0` | `ret(0x44d04) != 0x0` |
| `beq` at 0x44ef8 | `ret(0x45020) == 0x0` | `ret(0x45020) != 0x0` |
| `bne` at 0x44f04 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x44f44 | `r0` | `?` |

## `promIceDisable`

- `promIceDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x44f4c to 0x45020.
  Established. The initial selector takes `(r0 & 0xff) != 0xff` and falls through on equality (`bne`
  at 0x44f74). The first call targets `0x44d04` with unknown, unknown, `[0x2b0700]` and unknown
  (`bl` at 0x44fa0). Disable semantics and the common exit value remain unknown (`ldmdb` at
  0x45018).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x44f60 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x44f68 | store | 4 | `sp -0x18` | `0x0` |
| `ldrb` at 0x44f6c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x44f7c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x44f80 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x44f84 | load | 4 | `abs 0x4501c` | `-` |
| `ldr` at 0x44f88 | load | 4 | `abs 0x2b0700` | `-` |
| `ldrb` at 0x44f98 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x44fb0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x44fc0 | store | 4 | `sp -0x1c` | `ret(0x450d0)` |
| `ldr` at 0x44fc4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x44fd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x44fdc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x44fe0 | store | 4 | `sp -0x18` | `ret(0x450d0)` |
| `ldrb` at 0x44fe4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x44fec | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x44ff4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x44ff8 | store | 4 | `sp -0x24` | `?` |
| `ldrb` at 0x45000 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45010 | store | 4 | `sp -0x24` | `ret(0x450d0)` |
| `ldr` at 0x45014 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x45018 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45018 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45018 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x44fa0 | `(unknown callee) 0x44d04` | `?` | `?` | `[0x2b0700]` | `?` |
| `bl` at 0x44fb8 | `(unknown callee) 0x450d0` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x45008 | `(unknown callee) 0x450d0` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x44f74 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x44f90 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x44fac | `ret(0x44d04) == 0x0` | `ret(0x44d04) != 0x0` |
| `beq` at 0x44fcc | `ret(0x450d0) == 0x0` | `ret(0x450d0) != 0x0` |
| `bne` at 0x44fd8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45018 | `r0` | `?` |

## `IP57_Init`

- `IP57_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5f690 to 0x5f708.
  Established. The call to `0x5dec8` receives `(r0 & 0xff)`, entry r1, entry r2 and `(r0 & 0xff)`
  (`bl` at 0x5f6b0). A zero result takes the branch and nonzero falls through (`beq` at 0x5f6bc).
  The two result-word stores write `0x0` and `0x1` at `sp -0x18` on their respective paths (`str` at
  0x5f6f0, `str` at 0x5f6fc). The board-initialisation contract is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5f6a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5f6a8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f6c0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5f6d8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5f6f0 | store | 4 | `sp -0x18` | `0x0` |
| `str` at 0x5f6fc | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x5f700 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5f704 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5f704 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5f704 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5f6b0 | `(unknown callee) 0x5dec8` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x5f6c8 | `(unknown callee) 0x5df4c` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |
| `bl` at 0x5f6e8 | `(unknown callee) 0x5f978` | `[sp-17]:1` | `0x0` | `0x1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5f6bc | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |
| `bne` at 0x5f6d4 | `ret(0x5df4c) != 0x0` | `ret(0x5df4c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5f704 | `r0` | `?` |

## `IP57_Get_Cfg`

- `IP57_Get_Cfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60e98 to 0x60ef8.
  Established. A word read accesses absolute `0x148428` (`ldr` at 0x60eb0). Nonzero takes the branch
  and zero falls through (`bne` at 0x60eb8). The fall-through store places `0xfffff510` at `sp
  -0x18` (`str` at 0x60ec0). The configuration record and merged return value remain unknown
  (`ldmdb` at 0x60ee8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x60ea8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x60eac | load | 4 | `abs 0x60eec` | `-` |
| `ldr` at 0x60eb0 | load | 4 | `abs 0x148428` | `-` |
| `ldr` at 0x60ebc | load | 4 | `abs 0x60ef0` | `-` |
| `str` at 0x60ec0 | store | 4 | `sp -0x18` | `0xfffff510` |
| `ldr` at 0x60ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x60ecc | load | 4 | `abs 0x60eec` | `-` |
| `ldr` at 0x60ed0 | load | 4 | `abs 0x148428` | `-` |
| `str` at 0x60ed4 | store | 4 | `r0 +0x0` | `[0x148428]` |
| `ldr` at 0x60ed8 | load | 4 | `abs 0x60ef4` | `-` |
| `ldr` at 0x60edc | load | 4 | `abs 0x148424` | `-` |
| `str` at 0x60ee0 | store | 4 | `sp -0x18` | `[0x148424]` |
| `ldr` at 0x60ee4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x60ee8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60ee8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60ee8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x60eb8 | `[0x148428] != 0x0` | `[0x148428] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60ee8 | `r0` | `?` |

## `IP65_Get_Cfg`

- `IP65_Get_Cfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x648e8 to 0x64948.
  Established. The word load reads absolute `0x1484f4` (`ldr` at 0x64900). Nonzero takes the first
  branch and zero falls through (`bne` at 0x64908). The fall-through word store writes `0xfffff254`
  at `sp -0x18` (`str` at 0x64910). The record contents and final value are unknown (`ldmdb` at
  0x64938).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x648f8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x648fc | load | 4 | `abs 0x6493c` | `-` |
| `ldr` at 0x64900 | load | 4 | `abs 0x1484f4` | `-` |
| `ldr` at 0x6490c | load | 4 | `abs 0x64940` | `-` |
| `str` at 0x64910 | store | 4 | `sp -0x18` | `0xfffff254` |
| `ldr` at 0x64918 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6491c | load | 4 | `abs 0x6493c` | `-` |
| `ldr` at 0x64920 | load | 4 | `abs 0x1484f4` | `-` |
| `str` at 0x64924 | store | 4 | `r0 +0x0` | `[0x1484f4]` |
| `ldr` at 0x64928 | load | 4 | `abs 0x64944` | `-` |
| `ldr` at 0x6492c | load | 4 | `abs 0x1484f0` | `-` |
| `str` at 0x64930 | store | 4 | `sp -0x18` | `[0x1484f0]` |
| `ldr` at 0x64934 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x64938 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64938 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64938 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x64908 | `[0x1484f4] != 0x0` | `[0x1484f4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64938 | `r0` | `?` |

## `IP65_Cfg_Init`

- `IP65_Cfg_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x63be8 to 0x63c70.
  Established. A byte store sets absolute `0x1484ec` to `0x1` (`strb` at 0x63c3c). The following
  word stores write `[sp-24]` at `0x1484f0` and `[sp-20]` at `0x1484f4` (`str` at 0x63c48, `str` at
  0x63c54). The earlier guard takes `[0x1484ec]:1 == 0x0` and falls through on nonzero (`beq` at
  0x63c14). The configuration ownership and hardware meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x63bf8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x63bfc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x63c00 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x63c04 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x63c08 | load | 4 | `abs 0x63c5c` | `-` |
| `ldrb` at 0x63c0c | load | 1 | `abs 0x1484ec` | `-` |
| `ldr` at 0x63c18 | load | 4 | `abs 0x63c60` | `-` |
| `str` at 0x63c1c | store | 4 | `sp -0x28` | `0x10ce04` |
| `ldr` at 0x63c24 | load | 4 | `abs 0x63c64` | `-` |
| `ldr` at 0x63c34 | load | 4 | `abs 0x63c5c` | `-` |
| `strb` at 0x63c3c | store | 1 | `abs 0x1484ec` | `0x1` |
| `ldr` at 0x63c40 | load | 4 | `abs 0x63c68` | `-` |
| `ldr` at 0x63c44 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x63c48 | store | 4 | `abs 0x1484f0` | `[sp-24]` |
| `ldr` at 0x63c4c | load | 4 | `abs 0x63c6c` | `-` |
| `ldr` at 0x63c50 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x63c54 | store | 4 | `abs 0x1484f4` | `[sp-20]` |
| `ldmdb` at 0x63c58 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x63c58 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x63c58 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x63c30 | `(unknown callee) 0x21e28` | `0x9` | `0x10ce20` | `0x90` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x63c14 | `[0x1484ec]:1 == 0x0` | `[0x1484ec]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x63c58 | `r0` | `?` |
