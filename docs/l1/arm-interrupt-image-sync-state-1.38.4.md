# ARM interrupt and image synchronisation state in 1.38.4

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
monitorTest has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/interrupt-image-sync-state-facts.txt) retain all tool sections.

## `extIntInit`

- `extIntInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc84f8 to 0xc8698.
  Established. A byte read accesses absolute `0x2ad194` (`ldrb` at 0xc8514). Zero takes the branch
  and nonzero falls through (`beq` at 0xc851c). The opening call to `0x21e28` has candidate values `0x9`,
  `0x133148`, `0x5f` and `0x0` (`bl` at 0xc8544). Interrupt-table geometry and the merged return
  remain unknown (`ldmdb` at 0xc867c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8508 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc850c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xc8510 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc8514 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8520 | load | 4 | `abs 0xc8684` | `-` |
| `str` at 0xc8524 | store | 4 | `sp -0x2c` | `0x13311c` |
| `ldr` at 0xc8528 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc852c | load | 1 | `abs 0x2ad194` | `-` |
| `str` at 0xc8530 | store | 4 | `sp -0x28` | `[0x2ad194]:1` |
| `ldr` at 0xc8538 | load | 4 | `abs 0xc8688` | `-` |
| `ldr` at 0xc8548 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8554 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc8560 | load | 4 | `abs 0xc868c` | `-` |
| `str` at 0xc8564 | store | 4 | `sp -0x18` | `0x1429d0` |
| `str` at 0xc856c | store | 4 | `sp -0x1c` | `0x6` |
| `ldr` at 0xc8570 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc8574 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xc8578 | store | 1 | `abs 0x2ad194` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xc857c | load | 4 | `abs 0xc8690` | `-` |
| `ldr` at 0xc8580 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc8584 | load | 1 | `abs 0x2ad194` | `-` |
| `str` at 0xc8598 | store | 4 | `abs 0x2ad190` | `ret(0x1b3c4)` |
| `ldr` at 0xc859c | load | 4 | `abs 0x2ad190` | `-` |
| `ldr` at 0xc85a8 | load | 4 | `abs 0xc8694` | `-` |
| `str` at 0xc85ac | store | 4 | `sp -0x2c` | `0x133154` |
| `ldr` at 0xc85b0 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc85b4 | load | 1 | `abs 0x2ad194` | `-` |
| `str` at 0xc85d0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xc85d8 | load | 4 | `abs 0xc8688` | `-` |
| `str` at 0xc85ec | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xc85f0 | load | 4 | `abs 0xc8680` | `-` |
| `ldrb` at 0xc85f4 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc85f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8608 | load | 4 | `abs 0xc8690` | `-` |
| `ldr` at 0xc860c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8628 | load | 4 | `abs 0x2ad190` | `-` |
| `str` at 0xc8630 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc8634 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8638 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc8640 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc8648 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc864c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc8650 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc8654 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc865c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8664 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc8668 | store | 4 | `[sp-36] +0x8` | `?` |
| `ldr` at 0xc866c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc8674 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0xc867c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc867c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc867c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc867c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8544 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0x5f` | `0x0` |
| `bl` at 0xc8590 | `(unknown callee) 0x1b3c4` | `[0x2ad194]:1` | `0x34` | `0x2ad194` | `[0x2ad194]:1` |
| `bl` at 0xc85e4 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0x6f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc851c | `[0x2ad194]:1 == 0x0` | `[0x2ad194]:1 != 0x0` |
| `beq` at 0xc8550 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xc855c | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc85a4 | `[0x2ad190] != 0x0` | `[0x2ad190] == 0x0` |
| `blt` at 0xc8600 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc867c | `r0` | `?` |

## `extIntEnable`

- `extIntEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc87f4 to 0xc88d8.
  Established. A word store clears `sp -0x14` (`str` at 0xc8808). The loop decision retains unknown
  operands (`blt` at 0xc881c). The call to `0x223f0` has `[sp-24]+12`, `[[sp-24]+8]`, `0xc8cac` and
  `0x1` as candidates (`bl` at 0xc88b4). Task ownership and interrupt-enable behavior remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8808 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0xc880c | load | 4 | `abs 0xc88cc` | `-` |
| `ldrb` at 0xc8810 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8814 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8824 | load | 4 | `abs 0xc88d0` | `-` |
| `ldr` at 0xc8828 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8844 | load | 4 | `abs 0x2ad190` | `-` |
| `str` at 0xc884c | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc8850 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc8854 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc8864 | load | 4 | `sp -0x18` | `-` |
| `strb` at 0xc886c | store | 1 | `(unknown)` | `0x1` |
| `ldr` at 0xc8870 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8878 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc887c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8880 | store | 4 | `sp -0x2c` | `[sp-24]` |
| `str` at 0xc8888 | store | 4 | `sp -0x28` | `0x1000` |
| `str` at 0xc8890 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xc8898 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xc88a0 | store | 4 | `sp -0x1c` | `0x9` |
| `ldr` at 0xc88a8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xc88ac | load | 4 | `abs 0xc88d4` | `-` |
| `ldr` at 0xc88b8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc88c0 | store | 4 | `sp -0x14` | `?` |
| `ldmdb` at 0xc88c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc88c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc88c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc88b4 | `(unknown callee) 0x223f0` | `[sp-24]+12` | `[[sp-24]+8]` | `0xc8cac` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc881c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0xc885c | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc88c8 | `r0` | `?` |

## `extIntMaxInts`

- `extIntMaxInts`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc88d8 to 0xc88f8.
  Established. The byte load reads absolute `0x2ad194` (`ldrb` at 0xc88e8); its value reaches the
  return as `[0x2ad194]:1` (`ldmdb` at 0xc88f0). The meaning of the byte as an interrupt limit is
  unknown in this span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc88e4 | load | 4 | `abs 0xc88f4` | `-` |
| `ldrb` at 0xc88e8 | load | 1 | `abs 0x2ad194` | `-` |
| `ldmdb` at 0xc88f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc88f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc88f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc88f0 | `r0` | `[0x2ad194]:1` |

## `extIntCount`

- `extIntCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc88f8 to 0xc898c.
  Established. The unsigned branch takes `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` and falls
  through at or above the bound (`blo` at 0xc8920). The call to `0x21e28` has candidate values `0x9`, `0x133148`,
  `0xc2` and `0x0` (`bl` at 0xc8944). The counter source and final r0 expression remain unknown
  (`ldmdb` at 0xc8978).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc890c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8910 | load | 4 | `abs 0xc897c` | `-` |
| `ldrb` at 0xc8914 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc8918 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8924 | load | 4 | `abs 0xc8980` | `-` |
| `str` at 0xc8928 | store | 4 | `sp -0x1c` | `0x1331f4` |
| `ldrb` at 0xc892c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8930 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc8938 | load | 4 | `abs 0xc8984` | `-` |
| `ldrb` at 0xc8948 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8964 | load | 4 | `abs 0xc8988` | `-` |
| `ldr` at 0xc8968 | load | 4 | `abs 0x2ad190` | `-` |
| `ldr` at 0xc8970 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xc8978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8944 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0xc2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc8920 | `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8978 | `r0` | `?` |

## `extIntName`

- `extIntName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc898c to 0xc8a20.
  Established. The first decision takes `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` and falls
  through at or above the bound (`blo` at 0xc89b4). The call to `0x21e28` uses `0x9`, `0x133148`,
  `0xcd` and `0x0` as candidates (`bl` at 0xc89d8). The selected name and record geometry remain
  unknown (`ldmdb` at 0xc8a0c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc89a0 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc89a4 | load | 4 | `abs 0xc8a10` | `-` |
| `ldrb` at 0xc89a8 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc89ac | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc89b8 | load | 4 | `abs 0xc8a14` | `-` |
| `str` at 0xc89bc | store | 4 | `sp -0x1c` | `0x133230` |
| `ldrb` at 0xc89c0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc89c4 | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc89cc | load | 4 | `abs 0xc8a18` | `-` |
| `ldrb` at 0xc89dc | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc89f8 | load | 4 | `abs 0xc8a1c` | `-` |
| `ldr` at 0xc89fc | load | 4 | `abs 0x2ad190` | `-` |
| `ldr` at 0xc8a04 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0xc8a0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8a0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8a0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc89d8 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0xcd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc89b4 | `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8a0c | `r0` | `?` |

## `extIntIsPolling`

- `extIntIsPolling`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8ab4 to 0xc8b48.
  Established. The unsigned comparison takes `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` and
  falls through otherwise (`blo` at 0xc8adc). The call to `0x21e28` has `0x9`, `0x133148`, `0xe2`
  and `0x0` as candidates (`bl` at 0xc8b00). Polling-state encoding and the final value remain
  unknown (`ldmdb` at 0xc8b34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xc8ac8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0xc8acc | load | 4 | `abs 0xc8b38` | `-` |
| `ldrb` at 0xc8ad0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0xc8ad4 | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8ae0 | load | 4 | `abs 0xc8b3c` | `-` |
| `str` at 0xc8ae4 | store | 4 | `sp -0x1c` | `0x1332a4` |
| `ldrb` at 0xc8ae8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xc8aec | store | 4 | `sp -0x18` | `(r0 & 0xff)` |
| `ldr` at 0xc8af4 | load | 4 | `abs 0xc8b40` | `-` |
| `ldrb` at 0xc8b04 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0xc8b20 | load | 4 | `abs 0xc8b44` | `-` |
| `ldr` at 0xc8b24 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc8b2c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0xc8b34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8b34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8b34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8b00 | `(unknown callee) 0x21e28` | `0x9` | `0x133148` | `0xe2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc8adc | `(r0 & 0xff) < [0x2ad194]:1 (unsigned, 32-bit)` | `(r0 & 0xff) >= [0x2ad194]:1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8b34 | `r0` | `?` |

## `extIntPollingMask`

- `extIntPollingMask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8b48 to 0xc8be4.
  Established. Word stores clear `sp -0x14` and `sp -0x18` (`str` at 0xc8b5c, `str` at 0xc8b64). The
  loop comparison has unknown operands (`blt` at 0xc8b78). The return expression is unknown (`ldmdb`
  at 0xc8bd8). The polling mask cannot be established from the supported values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc8b5c | store | 4 | `sp -0x14` | `0x0` |
| `str` at 0xc8b64 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc8b68 | load | 4 | `abs 0xc8bdc` | `-` |
| `ldrb` at 0xc8b6c | load | 1 | `abs 0x2ad194` | `-` |
| `ldr` at 0xc8b70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8b80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8b9c | load | 4 | `abs 0xc8be0` | `-` |
| `ldr` at 0xc8ba0 | load | 4 | `abs 0x2ad190` | `-` |
| `ldrb` at 0xc8ba8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0xc8bac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc8bb4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc8bbc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc8bc0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc8bc8 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xc8bd0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc8bd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8bd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8bd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xc8b78 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8bd8 | `r0` | `?` |

## `imageSyncInit`

- `imageSyncInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdad0c to 0xdad7c.
  Established. The global word read uses absolute `0x2ad234` (`ldr` at 0xdad28). Zero takes the
  branch and nonzero falls through (`beq` at 0xdad30). The opening call to `0x21e28` has candidate values `0x9`,
  `0x134ea4`, `0x35` and `0x0` (`bl` at 0xdad4c). Image-synchronisation ownership and the final r0
  value remain unknown (`ldmdb` at 0xdad68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdad1c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xdad20 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xdad24 | load | 4 | `abs 0xdad6c` | `-` |
| `ldr` at 0xdad28 | load | 4 | `abs 0x2ad234` | `-` |
| `ldr` at 0xdad34 | load | 4 | `abs 0xdad70` | `-` |
| `str` at 0xdad38 | store | 4 | `sp -0x20` | `0x134e84` |
| `ldr` at 0xdad40 | load | 4 | `abs 0xdad74` | `-` |
| `ldr` at 0xdad50 | load | 4 | `abs 0xdad78` | `-` |
| `ldr` at 0xdad54 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xdad58 | store | 4 | `abs 0x2b3740` | `?` |
| `ldr` at 0xdad5c | load | 4 | `abs 0xdad6c` | `-` |
| `ldr` at 0xdad60 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdad64 | store | 4 | `abs 0x2ad234` | `[sp-24]` |
| `ldmdb` at 0xdad68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdad68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdad68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdad4c | `(unknown callee) 0x21e28` | `0x9` | `0x134ea4` | `0x35` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xdad30 | `[0x2ad234] == 0x0` | `[0x2ad234] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdad68 | `r0` | `?` |

## `imageSyncGetCfg`

- `imageSyncGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdad7c to 0xdadb8.
  Established. A word store copies `[0x2b3740]` to `r0 +0x0` (`str` at 0xdad9c). The return carries
  `[0x2ad234]` (`ldmdb` at 0xdadac). The configuration fields and their physical use remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdad8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xdad90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xdad94 | load | 4 | `abs 0xdadb0` | `-` |
| `ldr` at 0xdad98 | load | 4 | `abs 0x2b3740` | `-` |
| `str` at 0xdad9c | store | 4 | `r0 +0x0` | `[0x2b3740]` |
| `ldr` at 0xdada0 | load | 4 | `abs 0xdadb4` | `-` |
| `ldr` at 0xdada4 | load | 4 | `abs 0x2ad234` | `-` |
| `ldmdb` at 0xdadac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdadac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdadac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdadac | `r0` | `[0x2ad234]` |

## `imageSyncCount`

- `imageSyncCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdadb8 to 0xdadd8.
  Established. The word load reads absolute `0x2ad234` (`ldr` at 0xdadc8). The recorded return is
  `[0x2ad234]` (`ldmdb` at 0xdadd0). The object count implied by the symbol is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xdadc4 | load | 4 | `abs 0xdadd4` | `-` |
| `ldr` at 0xdadc8 | load | 4 | `abs 0x2ad234` | `-` |
| `ldmdb` at 0xdadd0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdadd0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdadd0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdadd0 | `r0` | `[0x2ad234]` |

## `imageSyncClear`

- `imageSyncClear`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaec8 to 0xdaf80.
  Established. The call to `0xdb07c` has candidate values `(r0 & 0xff)`, entry r1, entry r2 and `(r0 & 0xff)`
  (`bl` at 0xdaee8). Nonzero takes the branch and zero falls through (`bne` at 0xdaefc). The
  fall-through store writes `0xfffffc10` at `sp -0x20` (`str` at 0xdaf04). The external clear action
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdaedc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdaee0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdaef0 | store | 4 | `sp -0x18` | `ret(0xdb07c)` |
| `ldr` at 0xdaef4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdaf00 | load | 4 | `abs 0xdaf7c` | `-` |
| `str` at 0xdaf04 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xdaf0c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaf10 | load | 1 | `ret(0xdb07c) +0x0` | `-` |
| `ldr` at 0xdaf14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaf18 | load | 1 | `ret(0xdb07c) +0x1` | `-` |
| `str` at 0xdaf30 | store | 4 | `sp -0x1c` | `ret(0xc7ef0)` |
| `ldr` at 0xdaf34 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdaf40 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xdaf44 | store | 4 | `sp -0x20` | `ret(0xc7ef0)` |
| `ldr` at 0xdaf4c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaf50 | load | 1 | `[sp-24] +0x0` | `-` |
| `ldr` at 0xdaf54 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xdaf58 | load | 1 | `[sp-24] +0x1` | `-` |
| `str` at 0xdaf70 | store | 4 | `sp -0x20` | `ret(0xc7ef0)` |
| `ldr` at 0xdaf74 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdaf78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaf78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaf78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdaee8 | `(unknown callee) 0xdb07c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdaf28 | `(unknown callee) 0xc7ef0` | `[ret(0xdb07c)]:1` | `[ret(0xdb07c)+1]:1` | `0x1` | `[ret(0xdb07c)+1]:1` |
| `bl` at 0xdaf68 | `(unknown callee) 0xc7ef0` | `[[sp-24]]:1` | `[[sp-24]+1]:1` | `0x0` | `[[sp-24]+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdaefc | `ret(0xdb07c) != 0x0` | `ret(0xdb07c) == 0x0` |
| `beq` at 0xdaf3c | `ret(0xc7ef0) == 0x0` | `ret(0xc7ef0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaf78 | `r0` | `?` |

## `imageSyncRead`

- `imageSyncRead`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaf80 to 0xdaffc.
  Established. The first call targets `0xdb07c` with candidate values `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0xdafa4); its nonzero result takes the branch and zero falls through (`bne` at
  0xdafb8). The final r0 value is unknown (`ldmdb` at 0xdaff4). The read source and synchronisation
  protocol remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xdaf94 | store | 4 | `sp -0x18` | `r1` |
| `strb` at 0xdaf98 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdaf9c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdafac | store | 4 | `sp -0x1c` | `ret(0xdb07c)` |
| `ldr` at 0xdafb0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xdafbc | load | 4 | `abs 0xdaff8` | `-` |
| `str` at 0xdafc0 | store | 4 | `sp -0x20` | `0xfffffc10` |
| `ldr` at 0xdafc8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdafcc | load | 1 | `ret(0xdb07c) +0x0` | `-` |
| `ldr` at 0xdafd0 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xdafd4 | load | 1 | `ret(0xdb07c) +0x1` | `-` |
| `ldr` at 0xdafe0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xdafec | store | 4 | `sp -0x20` | `ret(0xc801c)` |
| `ldr` at 0xdaff0 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xdaff4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdaff4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdaff4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdafa4 | `(unknown callee) 0xdb07c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xdafe4 | `(unknown callee) 0xc801c` | `[ret(0xdb07c)]:1` | `[ret(0xdb07c)+1]:1` | `[sp-24]` | `[ret(0xdb07c)+1]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdafb8 | `ret(0xdb07c) != 0x0` | `ret(0xdb07c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdaff4 | `r0` | `?` |

## `imageSyncName`

- `imageSyncName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xdaffc to 0xdb07c.
  Established. The call to `0xdb07c` has candidate values `(r0 & 0xff)`, entry r1, entry r2 and `(r0 & 0xff)`
  (`bl` at 0xdb01c). Nonzero takes the branch and zero falls through (`bne` at 0xdb030). The
  fall-through word store writes `0x134ec4` at `sp -0x1c` (`str` at 0xdb038). The name
  representation and common result are unknown (`ldmdb` at 0xdb070).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xdb010 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xdb014 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xdb024 | store | 4 | `sp -0x18` | `ret(0xdb07c)` |
| `ldr` at 0xdb028 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb034 | load | 4 | `abs 0xdb074` | `-` |
| `str` at 0xdb038 | store | 4 | `sp -0x1c` | `0x134ec4` |
| `ldr` at 0xdb040 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb044 | load | 4 | `ret(0xdb07c) +0x4` | `-` |
| `str` at 0xdb048 | store | 4 | `sp -0x20` | `[ret(0xdb07c)+4]` |
| `ldr` at 0xdb04c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xdb050 | load | 4 | `ret(0xdb07c) +0x4` | `-` |
| `ldr` at 0xdb05c | load | 4 | `abs 0xdb078` | `-` |
| `str` at 0xdb060 | store | 4 | `sp -0x20` | `0x134ecc` |
| `ldr` at 0xdb064 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xdb068 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xdb06c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xdb070 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xdb070 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xdb070 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xdb01c | `(unknown callee) 0xdb07c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xdb030 | `ret(0xdb07c) != 0x0` | `ret(0xdb07c) == 0x0` |
| `bne` at 0xdb058 | `[ret(0xdb07c)+4] != 0x0` | `[ret(0xdb07c)+4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xdb070 | `r0` | `?` |

## `monitorName`

- `monitorName`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdf64 to 0xcdfcc.
  Established. A word store saves `[ret(0xce118)+8]` at `sp -0x20` (`str` at 0xcdfa0). The deciding
  branch takes `[ret(0xce118)+8] != 0x0` and falls through on zero (`bne` at 0xcdfb0). The earlier
  call to `0xce118` has candidate values entry r0, `(r1 & 0xff)`, entry r2 and `(r1 & 0xff)` (`bl` at 0xcdf8c). The
  selected name and merged return remain unknown (`ldmdb` at 0xcdfc4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdf74 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdf7c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcdf80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdf84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdf94 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldr` at 0xcdf98 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcdf9c | load | 4 | `ret(0xce118) +0x8` | `-` |
| `str` at 0xcdfa0 | store | 4 | `sp -0x20` | `[ret(0xce118)+8]` |
| `ldr` at 0xcdfa4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xcdfa8 | load | 4 | `ret(0xce118) +0x8` | `-` |
| `ldr` at 0xcdfb4 | load | 4 | `abs 0xcdfc8` | `-` |
| `str` at 0xcdfb8 | store | 4 | `sp -0x20` | `0x133ed8` |
| `ldr` at 0xcdfbc | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcdfc4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdfc4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdfc4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdf8c | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdfb0 | `[ret(0xce118)+8] != 0x0` | `[ret(0xce118)+8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdfc4 | `r0` | `?` |

## `monitorTest`

- `monitorTest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdfcc to 0xce118.
  Established. The opening call to `0xce118` has entry r0, `(r1 & 0xff)`, entry r2 and `(r1 & 0xff)`
  as candidates (`bl` at 0xcdff4). The branch takes `[ret(0xce238)]:1 != 0x0` and falls through on
  zero (`bne` at 0xce024). The later selector remains unresolved (`ldrls` at 0xce044). Selected test
  effects are unknown beyond known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdfdc | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdfe4 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldrb` at 0xcdfe8 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdfec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdffc | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldrb` at 0xce000 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xce004 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xce014 | store | 4 | `sp -0x20` | `ret(0xce238)` |
| `ldr` at 0xce018 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xce01c | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xce028 | load | 4 | `abs 0xce104` | `-` |
| `str` at 0xce02c | store | 4 | `sp -0x24` | `0xfffffc11` |
| `ldr` at 0xce034 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xce038 | load | 1 | `[sp-28] +0x0` | `-` |
| `ldrls` at 0xce044 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xce0f4 | load | 4 | `abs 0xce114` | `-` |
| `str` at 0xce0f8 | store | 4 | `sp -0x24` | `0xfffff830` |
| `ldr` at 0xce0fc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xce100 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xce100 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xce100 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdff4 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xce00c | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xce024 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `ldrls` at 0xce044 | `[[sp-28]]:1-1 <= 0x5 (unsigned, 32-bit)` | `[[sp-28]]:1-1 > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xce100 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xce044).
