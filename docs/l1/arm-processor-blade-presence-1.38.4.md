# ARM processor and blade presence helpers in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/processor-blade-presence-facts.txt) retain all tool sections.

## `daytonaBladePresent`

- `daytonaBladePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35f7c to 0x35fac.
  Established. A byte store writes `0x1` at `(r0 & 0xff) +0x2b0680` (`strb` at 0x35fa0). Entry r0 is
  retained at the recorded return (`ldmdb` at 0x35fa4). The physical meaning of the indexed byte
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x35f90 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x35f94 | load | 4 | `abs 0x35fa8` | `-` |
| `ldrb` at 0x35f98 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x35fa0 | store | 1 | `(r0 & 0xff) +0x2b0680` | `0x1` |
| `ldmdb` at 0x35fa4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x35fa4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x35fa4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x35fa4 | `r0` | `r0` |

## `daytonaBladeNotPresent`

- `daytonaBladeNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35fac to 0x35fdc.
  Established. The indexed byte at `(r0 & 0xff) +0x2b0680` is cleared (`strb` at 0x35fd0). The exit
  carries entry r0 (`ldmdb` at 0x35fd4). A hardware absence indication cannot be proved from this
  store alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x35fc0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x35fc4 | load | 4 | `abs 0x35fd8` | `-` |
| `ldrb` at 0x35fc8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x35fd0 | store | 1 | `(r0 & 0xff) +0x2b0680` | `0x0` |
| `ldmdb` at 0x35fd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x35fd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x35fd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x35fd4 | `r0` | `r0` |

## `daytonaBladeIsPresent`

- `daytonaBladeIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x35fdc to 0x3600c.
  Established. The byte read targets `(r0 & 0xff) +0x2b0680` (`ldrb` at 0x35ffc); its value reaches
  the exit as `[(r0 & 0xff)+2819712]:1` (`ldmdb` at 0x36004). The byte interpretation remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x35ff0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x35ff4 | load | 4 | `abs 0x36008` | `-` |
| `ldrb` at 0x35ff8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x35ffc | load | 1 | `(r0 & 0xff) +0x2b0680` | `-` |
| `ldmdb` at 0x36004 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36004 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36004 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36004 | `r0` | `[(r0 & 0xff)+2819712]:1` |

## `daytonaInt`

- `daytonaInt`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3600c to 0x36058.
  Established. Three byte stores save `(r0 & 0xff)`, `(r1 & 0xff)` and `(r2 & 0xff)` at `sp -0x11`,
  `sp -0x12` and `sp -0x13` (`strb` at 0x36020, `strb` at 0x36028, `strb` at 0x36030). At the call to
  `0x2070c`, candidate registers r0 through r3 hold `0x106a58` and those three truncated values
  (`bl` at 0x3604c). The interrupt effect
  is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x36020 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x36028 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x36030 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x36034 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x36038 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x3603c | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x36040 | load | 4 | `abs 0x36054` | `-` |
| `ldmdb` at 0x36050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x36050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x36050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3604c | `(unknown callee) 0x2070c` | `0x106a58` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x36050 | `r0` | `ret(0x2070c)` |

## `ip63_cpu_disable`

- `ip63_cpu_disable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x380dc to 0x38108.
  Established. At the call to `0x20240`, candidate registers r0 through r3 hold `0x1075fc`, entry r1,
  entry r2 and entry r0 (`bl` at 0x380f8). The final r0 expression is unknown (`ldmdb` at 0x38100). Processor-disable behavior
  remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x380f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x380f4 | load | 4 | `abs 0x38104` | `-` |
| `ldmdb` at 0x38100 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38100 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x380f8 | `(unknown callee) 0x20240` | `0x1075fc` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38100 | `r0` | `?` |

## `ip63_cpu_enable`

- `ip63_cpu_enable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38108 to 0x38134.
  Established. A byte save records `(r0 & 0xff)` at `sp -0x11` (`strb` at 0x3811c). At the call to
  `0x20240`, candidate registers r0 through r3 hold `0x107614`, entry r1, entry r2 and entry r0
  (`bl` at 0x38124); its recorded
  exit value is unknown (`ldmdb` at 0x3812c), as is the enable mechanism.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x3811c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38120 | load | 4 | `abs 0x38130` | `-` |
| `ldmdb` at 0x3812c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x3812c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x3812c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x38124 | `(unknown callee) 0x20240` | `0x107614` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x3812c | `r0` | `?` |

## `ip63_cpu_present`

- `ip63_cpu_present`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38134 to 0x38190.
  Established. A stack word at `sp -0x18` is first cleared and later conditionally set to `0x1`
  (`str` at 0x38150, `str` at 0x38178). The branches are taken when the bytes at `0x2b0680` and
  `0x2b06e8`, respectively, are zero, and fall through when nonzero (`beq` at 0x38160, `beq` at
  0x38170). The two globals'
  meanings and the merged return value remain unknown (`ldmdb` at 0x38184).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38148 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x38150 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x38154 | load | 4 | `abs 0x38188` | `-` |
| `ldrb` at 0x38158 | load | 1 | `abs 0x2b0680` | `-` |
| `ldr` at 0x38164 | load | 4 | `abs 0x3818c` | `-` |
| `ldrb` at 0x38168 | load | 1 | `abs 0x2b06e8` | `-` |
| `str` at 0x38178 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x3817c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x38184 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38184 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38184 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x38160 | `[0x2b0680]:1 == 0x0` | `[0x2b0680]:1 != 0x0` |
| `beq` at 0x38170 | `[0x2b06e8]:1 == 0x0` | `[0x2b06e8]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38184 | `r0` | `?` |

## `ip63Int`

- `ip63Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38190 to 0x381dc.
  Established. At the call to `0x2070c`, candidate registers r0 through r3 hold `0x107628`,
  `(r0 & 0xff)`, `(r1 & 0xff)` and `(r2 & 0xff)` (`bl` at 0x381d0). The exit carries `ret(0x2070c)` (`ldmdb` at 0x381d4). A specific
  hardware interrupt action is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x381a4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x381ac | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x381b4 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `ldrb` at 0x381b8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x381bc | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x381c0 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x381c4 | load | 4 | `abs 0x381d8` | `-` |
| `ldmdb` at 0x381d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x381d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x381d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x381d0 | `(unknown callee) 0x2070c` | `0x107628` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r2 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x381d4 | `r0` | `ret(0x2070c)` |

## `ip63BladePresent`

- `ip63BladePresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x381dc to 0x3820c.
  Established. The store places `0x1` in the byte at `(r0 & 0xff) +0x2b0680` (`strb` at 0x38200).
  The return retains entry r0 (`ldmdb` at 0x38204). The corresponding physical blade state remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x381f0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x381f4 | load | 4 | `abs 0x38208` | `-` |
| `ldrb` at 0x381f8 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x38200 | store | 1 | `(r0 & 0xff) +0x2b0680` | `0x1` |
| `ldmdb` at 0x38204 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38204 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38204 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38204 | `r0` | `r0` |

## `ip63BladeNotPresent`

- `ip63BladeNotPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3820c to 0x3823c.
  Established. The byte store writes `0x0` to `(r0 & 0xff) +0x2b0680` (`strb` at 0x38230). Entry r0
  survives to the recorded exit (`ldmdb` at 0x38234). The cleared value's presence meaning is
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38220 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38224 | load | 4 | `abs 0x38238` | `-` |
| `ldrb` at 0x38228 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x38230 | store | 1 | `(r0 & 0xff) +0x2b0680` | `0x0` |
| `ldmdb` at 0x38234 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38234 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38234 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38234 | `r0` | `r0` |

## `ip63BladeIsPresent`

- `ip63BladeIsPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x3823c to 0x3826c.
  Established. The indexed read fetches a byte at `(r0 & 0xff) +0x2b0680` (`ldrb` at 0x3825c),
  returned as `[(r0 & 0xff)+2819712]:1` (`ldmdb` at 0x38264); its interpretation as blade presence
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x38250 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x38254 | load | 4 | `abs 0x38268` | `-` |
| `ldrb` at 0x38258 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3825c | load | 1 | `(r0 & 0xff) +0x2b0680` | `-` |
| `ldmdb` at 0x38264 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x38264 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x38264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x38264 | `r0` | `[(r0 & 0xff)+2819712]:1` |

## `IP63_Nmi`

- `IP63_Nmi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x38294 to 0x382bc.
  Established. A word store saves entry r0 at `sp -0x14` (`str` at 0x382a4). At the only call to
  `0x20240`, candidate registers r0 through r3 hold `0x107664`, entry r1, entry r2 and entry r3
  (`bl` at 0x382ac). Neither the NMI
  effect nor the final r0 value is established (`ldmdb` at 0x382b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x382a4 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x382a8 | load | 4 | `abs 0x382b8` | `-` |
| `ldmdb` at 0x382b4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x382b4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x382b4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x382ac | `(unknown callee) 0x20240` | `0x107664` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x382b4 | `r0` | `?` |

## `IP65_Nmi`

- `IP65_Nmi`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64880 to 0x648a4.
  Established. The entry value is truncated into a byte at `sp -0x11` (`strb` at 0x64894). The
  recorded return is `0x0` (`ldmdb` at 0x648a0). This body does not establish delivery of an NMI.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64894 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldmdb` at 0x648a0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x648a0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x648a0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x648a0 | `r0` | `0x0` |

## `IP65_Int`

- `IP65_Int`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x648a4 to 0x648e8.
  Established. Five byte saves record `(r0 & 0xff)`, `(r1 & 0xff)`, `(r2 & 0xff)`, `(r3 & 0xff)` and
  `([sp] & 0xff)` at `sp -0x11`, `sp -0x12`, `sp -0x13`, `sp -0x14` and `sp -0x15` (`strb` at
  0x648c0, `strb` at 0x648c8, `strb` at 0x648d0, `strb` at 0x648d8, `strb` at 0x648e0). Entry r0
  reaches the exit (`ldmdb` at 0x648e4). The interrupt-handling role remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x648b8 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0x648c0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x648c8 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x648d0 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `strb` at 0x648d8 | store | 1 | `sp -0x14` | `(r3 & 0xff)` |
| `strb` at 0x648e0 | store | 1 | `sp -0x15` | `([sp] & 0xff)` |
| `ldmdb` at 0x648e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x648e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x648e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x648e4 | `r0` | `r0` |

## `PICActive`

- `PICActive`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x34740 to 0x34768.
  Established. The absolute byte read uses `0x147f51` (`ldrb` at 0x34758). The return value is
  `[0x147f51]:1` (`ldmdb` at 0x34760). The byte's interpretation as an active state remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x34750 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x34754 | load | 4 | `abs 0x34764` | `-` |
| `ldrb` at 0x34758 | load | 1 | `abs 0x147f51` | `-` |
| `ldmdb` at 0x34760 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x34760 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x34760 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x34760 | `r0` | `[0x147f51]:1` |
