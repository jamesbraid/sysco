# ARM processor state and mask helpers in 1.38.4

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

[Fact sheets and full listings](disasm/arm-1.38.4-package4/processor-state-mask-helpers-facts.txt) retain all tool sections.

## `cpuCount`

- `cpuCount`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d81c to 0x5d83c.
  Established. A word load reads absolute `0x2ae588` (`ldr` at 0x5d82c). The recorded exit carries
  `[0x2ae588]` (`ldmdb` at 0x5d834). The meaning of this word as a processor count is unknown within
  the span.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x5d828 | load | 4 | `abs 0x5d838` | `-` |
| `ldr` at 0x5d82c | load | 4 | `abs 0x2ae588` | `-` |
| `ldmdb` at 0x5d834 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d834 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d834 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d834 | `r0` | `[0x2ae588]` |

## `cpuPresent`

- `cpuPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d83c to 0x5d8c0.
  Established. The global word read uses absolute `0x2ae590` (`ldr` at 0x5d858). Zero takes the
  branch and nonzero falls through (`beq` at 0x5d860). The indirect call has candidates `(r0 &
  0xff)`, entry r1, `(r0 & 0xff)` and `[0x2ae590]` (`mov` at 0x5d878); its target and the final
  presence result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5d850 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x5d854 | load | 4 | `abs 0x5d8bc` | `-` |
| `ldr` at 0x5d858 | load | 4 | `abs 0x2ae590` | `-` |
| `ldr` at 0x5d864 | load | 4 | `abs 0x5d8bc` | `-` |
| `ldrb` at 0x5d868 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5d86c | load | 4 | `abs 0x2ae590` | `-` |
| `str` at 0x5d880 | store | 4 | `sp -0x18` | `ret(unknown)` |
| `ldrb` at 0x5d888 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5d88c | store | 4 | `sp -0x1c` | `(r0 & 0xff)` |
| `ldr` at 0x5d890 | load | 4 | `abs 0x5d8bc` | `-` |
| `ldr` at 0x5d894 | load | 4 | `abs 0x2ae588` | `-` |
| `ldr` at 0x5d898 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d8a8 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5d8ac | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d8b0 | store | 4 | `sp -0x18` | `[sp-28]` |
| `ldr` at 0x5d8b4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5d8b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d8b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d8b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0x5d878 | `(unknown callee) (unknown target)` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `[0x2ae590]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5d860 | `[0x2ae590] == 0x0` | `[0x2ae590] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d8b8 | `r0` | `?` |

## `cpuDisable`

- `cpuDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d94c to 0x5da40.
  Established. The call to `0x3d8e0` receives entry r0, entry r1, entry r2 and `0x0` (`bl` at
  0x5d96c). A zero result takes the branch and nonzero falls through (`beq` at 0x5d978). The
  subsequent selector takes `[sp-17]:1 != 0xff` and falls through on equality (`bne` at 0x5d990).
  The processor-disable action and merged return are unknown (`ldmdb` at 0x5da34).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5d960 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5d968 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5d97c | load | 4 | `abs 0x5da38` | `-` |
| `str` at 0x5d980 | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x5d988 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5d998 | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5d99c | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5d9a0 | load | 4 | `abs 0x5da3c` | `-` |
| `ldr` at 0x5d9a4 | load | 4 | `abs 0x2ae588` | `-` |
| `ldrb` at 0x5d9b4 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5d9cc | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5d9dc | store | 4 | `sp -0x1c` | `ret(0x5db34)` |
| `ldr` at 0x5d9e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5d9ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5d9f8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d9fc | store | 4 | `sp -0x18` | `ret(0x5db34)` |
| `ldrb` at 0x5da00 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5da08 | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5da10 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5da14 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5da1c | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5da2c | store | 4 | `sp -0x24` | `ret(0x5db34)` |
| `ldr` at 0x5da30 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5da34 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5da34 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5da34 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d96c | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5d9bc | `(unknown callee) 0x5d83c` | `?` | `?` | `[0x2ae588]` | `?` |
| `bl` at 0x5d9d4 | `(unknown callee) 0x5db34` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5da24 | `(unknown callee) 0x5db34` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5d978 | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bne` at 0x5d990 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5d9ac | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5d9c8 | `ret(0x5d83c) == 0x0` | `ret(0x5d83c) != 0x0` |
| `beq` at 0x5d9e8 | `ret(0x5db34) == 0x0` | `ret(0x5db34) != 0x0` |
| `bne` at 0x5d9f4 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5da34 | `r0` | `?` |

## `cpuEnable`

- `cpuEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5da40 to 0x5db34.
  Established. The opening call to `0x3d8e0` carries entry r0, entry r1, entry r2 and `0x0` (`bl` at
  0x5da60); its zero result takes the branch, while nonzero falls through (`beq` at 0x5da6c). The
  later selector takes `[sp-17]:1 != 0xff` and falls through on equality (`bne` at 0x5da84). The
  processor-enable mechanism remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5da54 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x5da5c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5da70 | load | 4 | `abs 0x5db2c` | `-` |
| `str` at 0x5da74 | store | 4 | `sp -0x24` | `0xfffff63a` |
| `ldrb` at 0x5da7c | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5da8c | store | 1 | `sp -0x1d` | `0x0` |
| `ldrb` at 0x5da90 | load | 1 | `sp -0x1d` | `-` |
| `ldr` at 0x5da94 | load | 4 | `abs 0x5db30` | `-` |
| `ldr` at 0x5da98 | load | 4 | `abs 0x2ae588` | `-` |
| `ldrb` at 0x5daa8 | load | 1 | `sp -0x1d` | `-` |
| `ldrb` at 0x5dac0 | load | 1 | `sp -0x1d` | `-` |
| `str` at 0x5dad0 | store | 4 | `sp -0x1c` | `ret(0x5dbf0)` |
| `ldr` at 0x5dad4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5dae0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5daec | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5daf0 | store | 4 | `sp -0x18` | `ret(0x5dbf0)` |
| `ldrb` at 0x5daf4 | load | 1 | `sp -0x1d` | `-` |
| `strb` at 0x5dafc | store | 1 | `sp -0x1d` | `([sp-29]:1+1 & 0xff)` |
| `ldr` at 0x5db04 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5db08 | store | 4 | `sp -0x24` | `[sp-24]` |
| `ldrb` at 0x5db10 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5db20 | store | 4 | `sp -0x24` | `ret(0x5dbf0)` |
| `ldr` at 0x5db24 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x5db28 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5db28 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5db28 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5da60 | `(unknown callee) 0x3d8e0` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0x5dab0 | `(unknown callee) 0x5d83c` | `?` | `?` | `[0x2ae588]` | `?` |
| `bl` at 0x5dac8 | `(unknown callee) 0x5dbf0` | `[sp-29]:1` | `?` | `?` | `[sp-29]:1` |
| `bl` at 0x5db18 | `(unknown callee) 0x5dbf0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5da6c | `ret(0x3d8e0) == 0x0` | `ret(0x3d8e0) != 0x0` |
| `bne` at 0x5da84 | `[sp-17]:1 != 0xff` | `[sp-17]:1 == 0xff` |
| `blt` at 0x5daa0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x5dabc | `ret(0x5d83c) == 0x0` | `ret(0x5d83c) != 0x0` |
| `beq` at 0x5dadc | `ret(0x5dbf0) == 0x0` | `ret(0x5dbf0) != 0x0` |
| `bne` at 0x5dae8 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5db28 | `r0` | `?` |

## `cpuSetState`

- `cpuSetState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dcac to 0x5dd74.
  Established. The initial selector takes `(r0 & 0xff) != 0xff` and falls through on equality (`bne`
  at 0x5dcd8). The loop comparison retains unknown operands (`blt` at 0x5dcf4). A call to `0x5e8d4`
  on the single-index path carries `(r0 & 0xff)+10`, `(r1 & 0xff)`, `(r1 & 0xff)` and `(r0 &
  0xff)+10` (`bl` at 0x5dd58). State encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5dcc4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x5dccc | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x5dcd0 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5dce0 | store | 1 | `sp -0x13` | `0x0` |
| `ldrb` at 0x5dce4 | load | 1 | `sp -0x13` | `-` |
| `ldr` at 0x5dce8 | load | 4 | `abs 0x5dd6c` | `-` |
| `ldr` at 0x5dcec | load | 4 | `abs 0x2ae588` | `-` |
| `ldrb` at 0x5dcfc | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x5dd04 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x5dd14 | load | 1 | `sp -0x13` | `-` |
| `strb` at 0x5dd1c | store | 1 | `sp -0x13` | `([sp-19]:1+1 & 0xff)` |
| `ldrb` at 0x5dd24 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5dd28 | load | 4 | `abs 0x5dd6c` | `-` |
| `ldr` at 0x5dd2c | load | 4 | `abs 0x2ae588` | `-` |
| `ldr` at 0x5dd38 | load | 4 | `abs 0x5dd70` | `-` |
| `str` at 0x5dd3c | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x5dd44 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5dd4c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x5dd60 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5dd64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5dd68 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5dd68 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5dd68 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5dd10 | `(unknown callee) 0x5e8d4` | `?` | `?` | `?` | `?` |
| `bl` at 0x5dd58 | `(unknown callee) 0x5e8d4` | `(r0 & 0xff)+10` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5dcd8 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x5dcf4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x5dd34 | `(r0 & 0xff) < [0x2ae588] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2ae588] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5dd68 | `r0` | `?` |

## `cpuState`

- `cpuState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5dd74 to 0x5ddd8.
  Established. The signed comparison takes `(r0 & 0xff) < [0x2ae588] (signed, 32-bit)` and falls
  through at or above the bound (`blt` at 0x5dd9c). The taken path calls `0x5ea88` with `(r0 &
  0xff)+10`, entry r1, `(r0 & 0xff)` and `(r0 & 0xff)+10` (`bl` at 0x5ddb8). The resulting state and
  merged return remain unknown (`ldmdb` at 0x5ddcc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5dd88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5dd8c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5dd90 | load | 4 | `abs 0x5ddd0` | `-` |
| `ldr` at 0x5dd94 | load | 4 | `abs 0x2ae588` | `-` |
| `ldr` at 0x5dda0 | load | 4 | `abs 0x5ddd4` | `-` |
| `str` at 0x5dda4 | store | 4 | `sp -0x18` | `0xfffff63b` |
| `ldrb` at 0x5ddac | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5ddc4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5ddc8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5ddcc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ddcc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ddcc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ddb8 | `(unknown callee) 0x5ea88` | `(r0 & 0xff)+10` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5dd9c | `(r0 & 0xff) < [0x2ae588] (signed, 32-bit)` | `(r0 & 0xff) >= [0x2ae588] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ddcc | `r0` | `?` |

## `cpuStateString`

- `cpuStateString`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5ddd8 to 0x5dea8.
  Established. The call to `0x5ea88` receives `(r0 & 0xff)+10`, entry r1, entry r2 and `(r0 &
  0xff)+10` (`bl` at 0x5ddfc). A word read accesses absolute `0x14285c` (`ldr` at 0x5de14). The loop
  comparison has unknown operands (`blt` at 0x5de1c). The selected string and state-table
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5ddec | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5ddf0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5de00 | store | 4 | `sp -0x1c` | `ret(0x5ea88)` |
| `str` at 0x5de08 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5de0c | load | 4 | `abs 0x5de9c` | `-` |
| `ldr` at 0x5de10 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5de14 | load | 4 | `abs 0x14285c` | `-` |
| `ldr` at 0x5de24 | load | 4 | `abs 0x5dea0` | `-` |
| `ldr` at 0x5de28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5de2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x5de30 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x5de40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x5de48 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5de50 | load | 4 | `abs 0x5de9c` | `-` |
| `ldr` at 0x5de54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5de58 | load | 4 | `abs 0x14285c` | `-` |
| `ldr` at 0x5de64 | load | 4 | `abs 0x5dea0` | `-` |
| `ldr` at 0x5de68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x5de7c | load | 4 | `(unknown)` | `-` |
| `str` at 0x5de80 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x5de88 | load | 4 | `abs 0x5dea4` | `-` |
| `str` at 0x5de8c | store | 4 | `sp -0x20` | `0x10c2dc` |
| `ldr` at 0x5de90 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x5de98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5de98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5de98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ddfc | `(unknown callee) 0x5ea88` | `(r0 & 0xff)+10` | `r1` | `r2` | `(r0 & 0xff)+10` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x5de1c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x5de38 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x5de60 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5de98 | `r0` | `?` |

## `IP57_CpuPresent`

- `IP57_CpuPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fdfc to 0x5fe74.
  Established. The call to `0x1f1dc` has candidates `0x7b`, entry r1, entry r2 and unknown (`bl` at
  0x5fe30); its result takes the branch when unequal to `0x1` and falls through on equality (`bne`
  at 0x5fe3c). The hardware presence meaning and final r0 value are unknown (`ldmdb` at 0x5fe6c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5fe10 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5fe14 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5fe1c | store | 1 | `sp -0x12` | `?` |
| `ldrb` at 0x5fe20 | load | 1 | `sp -0x11` | `-` |
| `strb` at 0x5fe28 | store | 1 | `sp -0x13` | `?` |
| `str` at 0x5fe44 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5fe4c | load | 4 | `abs 0x5fe70` | `-` |
| `ldrb` at 0x5fe50 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x5fe54 | load | 1 | `[sp-18]:1 +0x148434` | `-` |
| `ldrb` at 0x5fe58 | load | 1 | `sp -0x13` | `-` |
| `str` at 0x5fe64 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5fe68 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5fe6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5fe6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5fe6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5fe30 | `(unknown callee) 0x1f1dc` | `0x7b` | `r1` | `r2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x5fe3c | `ret(0x1f1dc) != 0x1` | `ret(0x1f1dc) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5fe6c | `r0` | `?` |

## `IP57_CpuDisable`

- `IP57_CpuDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fe74 to 0x5fee4.
  Established. The first call targets `0x5fdfc` with `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0x5fe94). Zero takes the branch and nonzero falls through (`beq` at 0x5fea0). The
  later call to `0x5ff54` has four unknown candidates (`bl` at 0x5fec0). The disable effect remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5fe88 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5fe8c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5fea4 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5feb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5fec8 | store | 4 | `sp -0x18` | `ret(0x5ff54)` |
| `ldr` at 0x5fed0 | load | 4 | `abs 0x5fee0` | `-` |
| `str` at 0x5fed4 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x5fed8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5fedc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5fedc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5fedc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5fe94 | `(unknown callee) 0x5fdfc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x5fec0 | `(unknown callee) 0x5ff54` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5fea0 | `ret(0x5fdfc) == 0x0` | `ret(0x5fdfc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5fedc | `r0` | `?` |

## `IP57_CpuEnable`

- `IP57_CpuEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5fee4 to 0x5ff54.
  Established. The call to `0x5fdfc` receives `(r0 & 0xff)`, entry r1, entry r2 and `(r0 & 0xff)`
  (`bl` at 0x5ff04). The branch takes its zero result and falls through on nonzero (`beq` at
  0x5ff10). The later call to `0x60110` has four unknown candidates (`bl` at 0x5ff30).
  Processor-enable behavior is unknown beyond those calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5fef8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5fefc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5ff14 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5ff20 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5ff38 | store | 4 | `sp -0x18` | `ret(0x60110)` |
| `ldr` at 0x5ff40 | load | 4 | `abs 0x5ff50` | `-` |
| `str` at 0x5ff44 | store | 4 | `sp -0x18` | `0xfffff50e` |
| `ldr` at 0x5ff48 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5ff4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5ff4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5ff4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5ff04 | `(unknown callee) 0x5fdfc` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x5ff30 | `(unknown callee) 0x60110` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x5ff10 | `ret(0x5fdfc) == 0x0` | `ret(0x5fdfc) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5ff4c | `r0` | `?` |

## `IP57_GetCpuMask`

- `IP57_GetCpuMask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x60ef8 to 0x60f28.
  Established. The byte read targets `(r0 & 0xff) +0x148434` (`ldrb` at 0x60f18); its value reaches
  the return as `[(r0 & 0xff)+1344564]:1` (`ldmdb` at 0x60f20). The mask-bit meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x60f0c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x60f10 | load | 4 | `abs 0x60f24` | `-` |
| `ldrb` at 0x60f14 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x60f18 | load | 1 | `(r0 & 0xff) +0x148434` | `-` |
| `ldmdb` at 0x60f20 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x60f20 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x60f20 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x60f20 | `r0` | `[(r0 & 0xff)+1344564]:1` |

## `IP65_CpuPresent`

- `IP65_CpuPresent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64474 to 0x644c8.
  Established. The opening call to `0x1f1dc` passes `0x7b`, entry r1, entry r2 and entry r0 (`bl` at
  0x64490); its result takes the branch when unequal to `0x1`, falling through on equality (`bne` at
  0x6449c). The presence result and hardware interpretation remain unknown (`ldmdb` at 0x644c0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64488 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `str` at 0x644a4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x644ac | load | 4 | `abs 0x644c4` | `-` |
| `ldrb` at 0x644b0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x644b4 | load | 1 | `[sp-17]:1 +0x1484fc` | `-` |
| `str` at 0x644b8 | store | 4 | `sp -0x18` | `[[sp-17]:1+1344764]:1` |
| `ldr` at 0x644bc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x644c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x644c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x644c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64490 | `(unknown callee) 0x1f1dc` | `0x7b` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6449c | `ret(0x1f1dc) != 0x1` | `ret(0x1f1dc) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x644c0 | `r0` | `?` |

## `IP65_CpuDisable`

- `IP65_CpuDisable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x644c8 to 0x64524.
  Established. The call to `0x64474` carries `(r0 & 0xff)`, entry r1, entry r2 and `(r0 & 0xff)`
  (`bl` at 0x644e8). A zero result takes the branch and nonzero falls through (`beq` at 0x644f4). A
  later call to `0x64580` has `[sp-17]:1`, unknown, unknown and `[sp-17]:1` as candidates (`bl` at
  0x64500); its disable meaning is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x644dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x644e0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x644f8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64508 | store | 4 | `sp -0x18` | `ret(0x64580)` |
| `ldr` at 0x64510 | load | 4 | `abs 0x64520` | `-` |
| `str` at 0x64514 | store | 4 | `sp -0x18` | `0xfffff252` |
| `ldr` at 0x64518 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6451c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6451c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6451c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x644e8 | `(unknown callee) 0x64474` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x64500 | `(unknown callee) 0x64580` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x644f4 | `ret(0x64474) == 0x0` | `ret(0x64474) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6451c | `r0` | `?` |

## `IP65_CpuEnable`

- `IP65_CpuEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64524 to 0x64580.
  Established. The initial call to `0x64474` uses `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0x64544); its zero result takes the branch, while nonzero falls through (`beq` at
  0x64550). The later `0x646b0` call receives `[sp-17]:1`, unknown, unknown and `[sp-17]:1` (`bl` at
  0x6455c). The actual enable operation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x64538 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x6453c | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64554 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x64564 | store | 4 | `sp -0x18` | `ret(0x646b0)` |
| `ldr` at 0x6456c | load | 4 | `abs 0x6457c` | `-` |
| `str` at 0x64570 | store | 4 | `sp -0x18` | `0xfffff252` |
| `ldr` at 0x64574 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x64578 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64578 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64578 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x64544 | `(unknown callee) 0x64474` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x6455c | `(unknown callee) 0x646b0` | `[sp-17]:1` | `?` | `?` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x64550 | `ret(0x64474) == 0x0` | `ret(0x64474) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64578 | `r0` | `?` |

## `IP65_GetCpuMask`

- `IP65_GetCpuMask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x64948 to 0x64978.
  Established. A byte is read from `(r0 & 0xff) +0x1484fc` (`ldrb` at 0x64968). The return records
  `[(r0 & 0xff)+1344764]:1` (`ldmdb` at 0x64970). This establishes the indexed read but leaves the
  processor-mask encoding unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x6495c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldr` at 0x64960 | load | 4 | `abs 0x64974` | `-` |
| `ldrb` at 0x64964 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x64968 | load | 1 | `(r0 & 0xff) +0x1484fc` | `-` |
| `ldmdb` at 0x64970 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x64970 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x64970 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x64970 | `r0` | `[(r0 & 0xff)+1344764]:1` |
