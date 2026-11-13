# ARM voltage and power-bay paths in 1.38.4

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
Tables and return values cover only known control-flow edges. Each affected entry cites its unresolved transfers. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/voltage-power-bay-facts.txt) retain all tool sections.

## `vrmEnabled`

- `vrmEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5cb50 to 0x5cb9c.
  Established. The indexed byte read has an unknown effective target (`ldrb` at 0x5cb8c). The exit
  r0 expression is `[[sp-24]+9]:1` (`ldmdb` at 0x5cb94). No calls or conditional branches appear in
  the facts. VRM-enable polarity remains unidentified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x5cb60 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x5cb64 | load | 4 | `abs 0x5cb98` | `-` |
| `ldr` at 0x5cb68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x5cb7c | load | 4 | `abs 0x14838c` | `-` |
| `str` at 0x5cb84 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x5cb88 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x5cb8c | load | 1 | `[sp-24] +0x9` | `-` |
| `ldmdb` at 0x5cb94 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5cb94 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5cb94 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5cb94 | `r0` | `[[sp-24]+9]:1` |

## `cpuEnabled`

- `cpuEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5d8c0 to 0x5d94c.
  Established. The signed guard takes its bypass for `(r0 & 0xff) >= [0x2ae588]` and falls through
  for less (`bge` at 0x5d8e8). The call to `0x5d83c` carries `(r0 & 0xff)`, entry r1, `(r0 & 0xff)`,
  `(r0 & 0xff)` (`bl` at 0x5d8f4); its zero result takes the bypass and nonzero falls through (`beq`
  at 0x5d900). Physical CPU-enable interpretation and final r0 remain unknown (`ldmdb` at 0x5d944).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5d8d4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5d8d8 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5d8dc | load | 4 | `abs 0x5d948` | `-` |
| `ldr` at 0x5d8e0 | load | 4 | `abs 0x2ae588` | `-` |
| `ldrb` at 0x5d8ec | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x5d904 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5d914 | store | 4 | `sp -0x1c` | `ret(0x1f1dc)` |
| `ldr` at 0x5d918 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d928 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5d92c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5d930 | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x5d93c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5d940 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5d944 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5d944 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5d944 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5d8f4 | `(unknown callee) 0x5d83c` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x5d910 | `(unknown callee) 0x1f1dc` | `[sp-17]:1+88` | `?` | `?` | `[sp-17]:1+88` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5d8e8 | `(r0 & 0xff) >= [0x2ae588] (signed, 32-bit)` | `(r0 & 0xff) < [0x2ae588] (signed, 32-bit)` |
| `beq` at 0x5d900 | `ret(0x5d83c) == 0x0` | `ret(0x5d83c) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5d944 | `r0` | `?` |

## `nodeEnabled`

- `nodeEnabled`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x5df4c to 0x5dfe8.
  Established. The first call to `0x5dec8` receives `(r0 & 0xff)`, entry r1, `(r0 & 0xff)`, `(r0 &
  0xff)` (`bl` at 0x5df80); its zero result takes the bypass and nonzero falls through (`beq` at
  0x5df8c). The prior signed guard takes that bypass for `(r0 & 0xff) >= [0x2ae5b4]` and falls
  through for less (`bge` at 0x5df74). Node-enable encoding and exit r0 remain unestablished
  (`ldmdb` at 0x5dfe0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x5df60 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x5df64 | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5df68 | load | 4 | `abs 0x5dfe4` | `-` |
| `ldr` at 0x5df6c | load | 4 | `abs 0x2ae5b4` | `-` |
| `ldrb` at 0x5df78 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x5df98 | store | 4 | `sp -0x1c` | `ret(0x1f1dc)` |
| `ldrb` at 0x5df9c | load | 1 | `sp -0x11` | `-` |
| `ldr` at 0x5dfa8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5dfb0 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5dfb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5dfc4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x5dfc8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x5dfcc | store | 4 | `sp -0x18` | `[sp-28]` |
| `str` at 0x5dfd8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x5dfdc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x5dfe0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x5dfe0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x5dfe0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x5df80 | `(unknown callee) 0x5dec8` | `(r0 & 0xff)` | `r1` | `(r0 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x5df94 | `(unknown callee) 0x1f1dc` | `0x70` | `?` | `?` | `ret(0x5dec8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x5df74 | `(r0 & 0xff) >= [0x2ae5b4] (signed, 32-bit)` | `(r0 & 0xff) < [0x2ae5b4] (signed, 32-bit)` |
| `beq` at 0x5df8c | `ret(0x5dec8) == 0x0` | `ret(0x5dec8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x5dfe0 | `r0` | `?` |

## `monitorSetVoltageLimits`

- `monitorSetVoltageLimits`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcc85c to 0xccb28.
  Established. The call to `0xce118` carries entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl`
  at 0xcc8a4). The subsequent result-byte test takes its continuation for `[ret(0xce238)]:1 != 0x0`
  and falls through on zero (`bne` at 0xcc8d4). Voltage-register selections beyond the two
  unresolved computed branches remain unknown (`ldrls` at 0xcc914, 0xcc9d0). R0 at the recognized exit is unresolved (`ldmdb` at
  0xccb1c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcc86c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xcc874 | load | 4 | `sp +0x0` | `-` |
| `strb` at 0xcc87c | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcc884 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `strb` at 0xcc88c | store | 1 | `sp -0x17` | `(r3 & 0xff)` |
| `strb` at 0xcc894 | store | 1 | `sp -0x18` | `([sp] & 0xff)` |
| `ldrb` at 0xcc898 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc89c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc8ac | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldrb` at 0xcc8b0 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcc8b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcc8c4 | store | 4 | `sp -0x20` | `ret(0xce238)` |
| `ldr` at 0xcc8c8 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xcc8cc | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xcc8d8 | load | 4 | `abs 0xccb20` | `-` |
| `str` at 0xcc8dc | store | 4 | `sp -0x2c` | `0xfffffc11` |
| `ldr` at 0xcc8e4 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcc8e8 | load | 1 | `[sp-28] +0x0` | `-` |
| `str` at 0xcc8ec | store | 4 | `sp -0x30` | `[[sp-28]]:1` |
| `ldr` at 0xcc8f0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xcc8fc | load | 4 | `sp -0x30` | `-` |
| `ldrb` at 0xcc90c | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcc914 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcc9c0 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0xcc9c8 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcc9d0 | load | 4 | `(unknown)` | `-` |
| `str` at 0xcca64 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xcca6c | load | 4 | `abs 0xccb24` | `-` |
| `str` at 0xcca70 | store | 4 | `sp -0x2c` | `0xfffff830` |
| `ldr` at 0xccb18 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xccb1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xccb1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xccb1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcc8a4 | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |
| `bl` at 0xcc8bc | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcc8d4 | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xcc8f8 | `[[sp-28]]:1 == 0x1` | `[[sp-28]]:1 != 0x1` |
| `beq` at 0xcc904 | `[[sp-28]]:1 == 0x2` | `[[sp-28]]:1 != 0x2` |
| `ldrls` at 0xcc914 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `ldrls` at 0xcc9d0 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xccb1c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcc914).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcc9d0).

## `monitorReadVoltage`

- `monitorReadVoltage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xccf4c to 0xcd130.
  Established. The entry `r3 +0x0` halfword store writes `0x0` (`strh` at 0xccf7c). The opening call
  to `0xce118` has `[sp-20]`, `[sp-21]:1`, entry r3, `[sp-21]:1` (`bl` at 0xccf8c). The later
  result-byte test takes its continuation for `[ret(0xce238)]:1 != 0x0` and falls through on zero
  (`bne` at 0xccfbc). The selector transfer is unresolved (`ldrls` at 0xccffc). Voltage units remain unknown.
  The final r0 is unknown (`ldmdb` at 0xcd124).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xccf5c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xccf60 | store | 4 | `sp -0x1c` | `r3` |
| `strb` at 0xccf68 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xccf70 | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldr` at 0xccf74 | load | 4 | `sp -0x1c` | `-` |
| `strh` at 0xccf7c | store | 2 | `r3 +0x0` | `0x0` |
| `ldrb` at 0xccf80 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccf84 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xccf94 | store | 4 | `sp -0x20` | `ret(0xce118)` |
| `ldrb` at 0xccf98 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xccf9c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xccfac | store | 4 | `sp -0x24` | `ret(0xce238)` |
| `ldr` at 0xccfb0 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xccfb4 | load | 1 | `ret(0xce238) +0x0` | `-` |
| `ldr` at 0xccfc0 | load | 4 | `abs 0xcd128` | `-` |
| `str` at 0xccfc4 | store | 4 | `sp -0x30` | `0xfffffc11` |
| `ldr` at 0xccfcc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xccfd0 | load | 1 | `[sp-32] +0x0` | `-` |
| `str` at 0xccfd4 | store | 4 | `sp -0x34` | `[[sp-32]]:1` |
| `ldr` at 0xccfd8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xccfe4 | load | 4 | `sp -0x34` | `-` |
| `ldrb` at 0xccff4 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xccffc | load | 4 | `(unknown)` | `-` |
| `str` at 0xcd078 | store | 4 | `sp -0x30` | `?` |
| `ldrb` at 0xcd080 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcd084 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xcd088 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xcd090 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xcd09c | store | 4 | `sp -0x30` | `ret(0xcf940)` |
| `ldr` at 0xcd0a4 | load | 4 | `abs 0xcd12c` | `-` |
| `str` at 0xcd0a8 | store | 4 | `sp -0x30` | `0xfffff830` |
| `ldr` at 0xcd120 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xcd124 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcd124 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcd124 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xccf8c | `(unknown callee) 0xce118` | `[sp-20]` | `[sp-21]:1` | `r3` | `[sp-21]:1` |
| `bl` at 0xccfa4 | `(unknown callee) 0xce238` | `[sp-20]` | `[sp-21]:1` | `?` | `[sp-21]:1` |
| `bl` at 0xcd094 | `(unknown callee) 0xcf940` | `[sp-20]` | `[sp-32]` | `[sp-22]:1` | `[sp-28]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xccfbc | `[ret(0xce238)]:1 != 0x0` | `[ret(0xce238)]:1 == 0x0` |
| `beq` at 0xccfe0 | `[[sp-32]]:1 == 0x1` | `[[sp-32]]:1 != 0x1` |
| `beq` at 0xccfec | `[[sp-32]]:1 == 0x2` | `[[sp-32]]:1 != 0x2` |
| `ldrls` at 0xccffc | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcd124 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xccffc).

## `monitorVoltage`

- `monitorVoltage`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcd93c to 0xcdaf0.
  Established. The sole known-edge call to `0xce118` carries entry r0, `(r1 & 0xff)`, entry r2, `(r1
  & 0xff)` (`bl` at 0xcd96c). The next test takes its branch for `[ret(0xce118)]:1 == 0x1` and falls
  through otherwise (`beq` at 0xcd98c). A separate comparison takes the branch for equality with
  `0x2` and falls through on inequality (`beq` at 0xcd998). Voltage mappings behind the computed
  branches remain unidentified (`ldrls` at 0xcd9a8, 0xcda48). Exit r0 is unknown (`ldmdb` at 0xcdabc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcd94c | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcd954 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strb` at 0xcd95c | store | 1 | `sp -0x16` | `(r2 & 0xff)` |
| `ldrb` at 0xcd960 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcd964 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcd974 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldr` at 0xcd978 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcd97c | load | 1 | `ret(0xce118) +0x0` | `-` |
| `str` at 0xcd980 | store | 4 | `sp -0x24` | `[ret(0xce118)]:1` |
| `ldr` at 0xcd984 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xcd990 | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xcd9a0 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcd9a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcda14 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcda18 | load | 1 | `ret(0xce118) +0x3` | `-` |
| `ldrb` at 0xcda28 | load | 1 | `sp -0x16` | `-` |
| `ldr` at 0xcda34 | load | 4 | `abs 0xcdad4` | `-` |
| `str` at 0xcda38 | store | 4 | `sp -0x20` | `0x3b0ffb8b` |
| `ldrb` at 0xcda40 | load | 1 | `sp -0x16` | `-` |
| `ldrls` at 0xcda48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xcdab0 | load | 4 | `abs 0xcdaec` | `-` |
| `str` at 0xcdab4 | store | 4 | `sp -0x20` | `0xc2c80000` |
| `ldr` at 0xcdab8 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xcdabc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdabc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdabc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcd96c | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xcd98c | `[ret(0xce118)]:1 == 0x1` | `[ret(0xce118)]:1 != 0x1` |
| `beq` at 0xcd998 | `[ret(0xce118)]:1 == 0x2` | `[ret(0xce118)]:1 != 0x2` |
| `ldrls` at 0xcd9a8 | `[sp-22]:1 <= 0x9 (unsigned, 32-bit)` | `[sp-22]:1 > 0x9 (unsigned, 32-bit)` |
| `beq` at 0xcda24 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xcda30 | `[sp-22]:1 == 0x2` | `[sp-22]:1 != 0x2` |
| `ldrls` at 0xcda48 | `[sp-22]:1 <= 0x8 (unsigned, 32-bit)` | `[sp-22]:1 > 0x8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdabc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcd9a8).

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xcda48).

## `monitorVoltageLimit`

- `monitorVoltageLimit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xcdaf0 to 0xcdb7c.
  Established. The call to `0xce118` has entry r0, `(r1 & 0xff)`, entry r2, `(r1 & 0xff)` (`bl` at
  0xcdb1c). The result-byte comparison takes its bypass for `[ret(0xce118)]:1 != 0x2` and falls
  through on equality (`bne` at 0xcdb34). The unsigned halfword test takes its branch for `[sp-24]:2
  <= 0xfe` and falls through for greater (`bls` at 0xcdb4c). The return reports `[sp-24]:1` (`ldmdb`
  at 0xcdb78). Numeric units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xcdb00 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0xcdb08 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `strh` at 0xcdb0c | store | 2 | `sp -0x18` | `(r2 & 0xffff)` |
| `ldrb` at 0xcdb10 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xcdb14 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xcdb24 | store | 4 | `sp -0x1c` | `ret(0xce118)` |
| `ldr` at 0xcdb28 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xcdb2c | load | 1 | `ret(0xce118) +0x0` | `-` |
| `ldrh` at 0xcdb38 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xcdb40 | store | 2 | `sp -0x18` | `?` |
| `ldrh` at 0xcdb44 | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xcdb54 | store | 2 | `sp -0x18` | `0xfe` |
| `ldrh` at 0xcdb5c | load | 2 | `sp -0x18` | `-` |
| `strh` at 0xcdb6c | store | 2 | `sp -0x18` | `0x1` |
| `ldrb` at 0xcdb70 | load | 1 | `sp -0x18` | `-` |
| `ldmdb` at 0xcdb78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xcdb78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xcdb78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xcdb1c | `(unknown callee) 0xce118` | `r0` | `(r1 & 0xff)` | `r2` | `(r1 & 0xff)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xcdb34 | `[ret(0xce118)]:1 != 0x2` | `[ret(0xce118)]:1 == 0x2` |
| `bls` at 0xcdb4c | `[sp-24]:2 <= 0xfe (unsigned, 32-bit)` | `[sp-24]:2 > 0xfe (unsigned, 32-bit)` |
| `bne` at 0xcdb64 | `[sp-24]:2 != 0x0` | `[sp-24]:2 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xcdb78 | `r0` | `[sp-24]:1` |

## `APwrInit`

- `APwrInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb20f0 to 0xb2314.
  Established. The opening call to `0x4f090` carries entry r0 through r3 (`bl` at 0xb2100). The
  first two comparisons take their branches for `[sp-28] == 0x43` and `[sp-28] == 0x33`,
  respectively (`beq` at 0xb2118, `beq` at 0xb2124). Their fall-through paths correspond to
  inequality at each site (`beq` at 0xb2118, 0xb2124). Power-task ownership and r0 at the merged exit remain unresolved (`ldmdb`
  at 0xb22e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb210c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb2110 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb211c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2128 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2134 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2140 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb214c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2158 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2164 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2170 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb217c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2188 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2194 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21b8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb21e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2210 | load | 4 | `abs 0xb22ec` | `-` |
| `ldr` at 0xb2238 | load | 4 | `abs 0xb22f0` | `-` |
| `ldr` at 0xb2240 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb2248 | load | 4 | `abs 0xb22f4` | `-` |
| `ldr` at 0xb224c | load | 4 | `abs 0xb22f8` | `-` |
| `str` at 0xb225c | store | 4 | `sp -0x18` | `ret(0xb5654)` |
| `ldr` at 0xb2260 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb226c | load | 4 | `abs 0xb22fc` | `-` |
| `ldr` at 0xb2278 | load | 4 | `abs 0xb22ec` | `-` |
| `ldr` at 0xb227c | load | 4 | `abs 0xb2300` | `-` |
| `ldr` at 0xb2284 | load | 4 | `abs 0xb22f4` | `-` |
| `str` at 0xb2290 | store | 4 | `sp -0x18` | `ret(0xb4220)` |
| `ldr` at 0xb2294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb22a0 | load | 4 | `abs 0xb2304` | `-` |
| `ldr` at 0xb22ac | load | 4 | `abs 0xb22ec` | `-` |
| `str` at 0xb22b0 | store | 4 | `sp -0x30` | `0x2b1bc0` |
| `str` at 0xb22b8 | store | 4 | `sp -0x2c` | `0x4b0` |
| `str` at 0xb22c0 | store | 4 | `sp -0x28` | `0x19` |
| `str` at 0xb22c8 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0xb22d0 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0xb22d4 | load | 4 | `abs 0xb2308` | `-` |
| `ldr` at 0xb22d8 | load | 4 | `abs 0xb230c` | `-` |
| `ldr` at 0xb22dc | load | 4 | `abs 0xb2310` | `-` |
| `ldmdb` at 0xb22e8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb22e8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb22e8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb22e8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb2100 | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb21fc | `(unknown callee) 0x1f1dc` | `0x32` | `?` | `?` | `[sp-28]` |
| `bl` at 0xb221c | `(unknown callee) 0xb553c` | `0x2b1bc0` | `0x14` | `0x0` | `ret(0x1f1dc)` |
| `bl` at 0xb2224 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0xb2230 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xb2244 | `(unknown callee) 0x69d3c` | `0x149abc` | `ret(0x1f1dc)` | `[sp-28]` | `ret(0x1f1dc)` |
| `bl` at 0xb2254 | `(unknown callee) 0xb5654` | `0x2b1bc8` | `0x127a4c` | `0x1` | `?` |
| `bl` at 0xb2270 | `(unknown callee) 0x2070c` | `0x127a58` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0xb2288 | `(unknown callee) 0xb4220` | `0x2b1bc0` | `0xffff014` | `0x2` | `0x2b1bc8` |
| `bl` at 0xb22a4 | `(unknown callee) 0x2070c` | `0x127a84` | `?` | `?` | `ret(0xb4220)` |
| `bl` at 0xb22e4 | `(unknown callee) 0x223f0` | `0x2b1c60` | `0x127ab0` | `0xb2314` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb2118 | `[sp-28] == 0x43` | `[sp-28] != 0x43` |
| `beq` at 0xb2124 | `[sp-28] == 0x33` | `[sp-28] != 0x33` |
| `beq` at 0xb2130 | `[sp-28] == 0x35` | `[sp-28] != 0x35` |
| `beq` at 0xb213c | `[sp-28] == 0x34` | `[sp-28] != 0x34` |
| `beq` at 0xb2148 | `[sp-28] == 0x41` | `[sp-28] != 0x41` |
| `beq` at 0xb2154 | `[sp-28] == 0x36` | `[sp-28] != 0x36` |
| `beq` at 0xb2160 | `[sp-28] == 0x39` | `[sp-28] != 0x39` |
| `beq` at 0xb216c | `[sp-28] == 0x37` | `[sp-28] != 0x37` |
| `beq` at 0xb2178 | `[sp-28] == 0x38` | `[sp-28] != 0x38` |
| `beq` at 0xb2184 | `[sp-28] == 0x30` | `[sp-28] != 0x30` |
| `beq` at 0xb2190 | `[sp-28] == 0x51` | `[sp-28] != 0x51` |
| `beq` at 0xb219c | `[sp-28] == 0x40` | `[sp-28] != 0x40` |
| `beq` at 0xb21a8 | `[sp-28] == 0x4f` | `[sp-28] != 0x4f` |
| `beq` at 0xb21b4 | `[sp-28] == 0x42` | `[sp-28] != 0x42` |
| `beq` at 0xb21c0 | `[sp-28] == 0x46` | `[sp-28] != 0x46` |
| `beq` at 0xb21cc | `[sp-28] == 0x48` | `[sp-28] != 0x48` |
| `beq` at 0xb21d8 | `[sp-28] == 0x4c` | `[sp-28] != 0x4c` |
| `beq` at 0xb21e4 | `[sp-28] == 0x4d` | `[sp-28] != 0x4d` |
| `beq` at 0xb21f0 | `[sp-28] == 0x7c` | `[sp-28] != 0x7c` |
| `bne` at 0xb2208 | `ret(0x1f1dc) != 0x0` | `ret(0x1f1dc) == 0x0` |
| `beq` at 0xb2268 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |
| `beq` at 0xb229c | `ret(0xb4220) == 0x0` | `ret(0xb4220) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb22e8 | `r0` | `?` |

## `PBayInit`

- `PBayInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9734 to 0xa9834.
  Established. A word store writes `ret(0x1b124)` at absolute `0x1499d0`, and a halfword store
  clears `0x1499c8` (`str` at 0xa975c, `strh` at 0xa978c). A later word store writes `0x1499c0` at
  `[0x1499d0] +0x8` (`str` at 0xa979c). The opening call to `0xa9834` has `0x1`, entry r1, entry r2,
  entry r3 (`bl` at 0xa9748). The result test takes its bypass for `ret(0x223f0) == 0x0` and falls
  through on nonzero (`beq` at 0xa97fc). Power-bay initialization roles and final r0 remain unknown
  (`ldmdb` at 0xa9820).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa974c | load | 4 | `abs 0xa9824` | `-` |
| `str` at 0xa975c | store | 4 | `abs 0x1499d0` | `ret(0x1b124)` |
| `ldr` at 0xa9760 | load | 4 | `abs 0xa9824` | `-` |
| `ldr` at 0xa9764 | load | 4 | `abs 0x1499d0` | `-` |
| `ldr` at 0xa9774 | load | 4 | `abs 0xa9828` | `-` |
| `ldr` at 0xa9784 | load | 4 | `abs 0xa9828` | `-` |
| `strh` at 0xa978c | store | 2 | `abs 0x1499c8` | `0x0` |
| `ldr` at 0xa9790 | load | 4 | `abs 0xa9824` | `-` |
| `ldr` at 0xa9794 | load | 4 | `abs 0x1499d0` | `-` |
| `ldr` at 0xa9798 | load | 4 | `abs 0xa9828` | `-` |
| `str` at 0xa979c | store | 4 | `[0x1499d0] +0x8` | `0x1499c0` |
| `ldr` at 0xa97a0 | load | 4 | `abs 0xa9824` | `-` |
| `ldr` at 0xa97a4 | load | 4 | `abs 0x1499d0` | `-` |
| `ldr` at 0xa97ac | load | 4 | `abs 0xa9824` | `-` |
| `ldr` at 0xa97b0 | load | 4 | `abs 0x1499d0` | `-` |
| `str` at 0xa97b4 | store | 4 | `sp -0x2c` | `[0x1499d0]` |
| `str` at 0xa97bc | store | 4 | `sp -0x28` | `0xa00` |
| `str` at 0xa97c4 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xa97cc | store | 4 | `sp -0x20` | `0x28` |
| `str` at 0xa97d4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xa97dc | load | 4 | `abs 0xa982c` | `-` |
| `ldr` at 0xa97e0 | load | 4 | `abs 0xa9830` | `-` |
| `str` at 0xa97f0 | store | 4 | `sp -0x18` | `ret(0x223f0)` |
| `ldr` at 0xa97f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xa9814 | load | 4 | `abs 0xa9824` | `-` |
| `ldr` at 0xa9818 | load | 4 | `abs 0x1499d0` | `-` |
| `ldmdb` at 0xa9820 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa9820 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9820 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9820 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9748 | `(unknown callee) 0xa9834` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xa9754 | `(unknown callee) 0x1b124` | `0x104` | `?` | `?` | `?` |
| `bl` at 0xa9770 | `(unknown callee) 0x1b43c` | `[0x1499d0]` | `0x0` | `0x104` | `0x1499d0` |
| `bl` at 0xa9780 | `(unknown callee) 0x1b43c` | `0x1499c0` | `0x0` | `0x10` | `?` |
| `bl` at 0xa97e8 | `(unknown callee) 0x223f0` | `[0x1499d0]+4` | `0x126ef4` | `0xaa440` | `0x1` |
| `bl` at 0xa9808 | `(unknown callee) 0xa9ed4` | `?` | `0x7` | `?` | `ret(0x223f0)` |
| `bl` at 0xa9810 | `(unknown callee) 0xa9834` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xa981c | `(unknown callee) 0x1b400` | `[0x1499d0]` | `?` | `?` | `0x1499d0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa97fc | `ret(0x223f0) == 0x0` | `ret(0x223f0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9820 | `r0` | `?` |

## `PBaySetState`

- `PBaySetState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9834 to 0xa9910.
  Established. The word store at absolute `0x1499bc` writes entry r0 (`str` at 0xa9850). The
  computed branch takes its unresolved edge for unsigned `[0x1499bc] <= 0x5` and falls through for
  greater (`ldrls` at 0xa9860). A known-edge word store writes `0x126f68` at absolute `0x142908`
  (`str` at 0xa98e8). The recognized exit retains entry r0 (`ldmdb` at 0xa98ec). State-to-record
  mappings in excluded arms remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9844 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xa9848 | load | 4 | `abs 0xa98f0` | `-` |
| `ldr` at 0xa984c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xa9850 | store | 4 | `abs 0x1499bc` | `r0` |
| `ldr` at 0xa9854 | load | 4 | `abs 0xa98f0` | `-` |
| `ldr` at 0xa9858 | load | 4 | `abs 0x1499bc` | `-` |
| `ldrls` at 0xa9860 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xa98e0 | load | 4 | `abs 0xa98f4` | `-` |
| `ldr` at 0xa98e4 | load | 4 | `abs 0xa990c` | `-` |
| `str` at 0xa98e8 | store | 4 | `abs 0x142908` | `0x126f68` |
| `ldmdb` at 0xa98ec | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa98ec | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa98ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0xa9860 | `[0x1499bc] <= 0x5 (unsigned, 32-bit)` | `[0x1499bc] > 0x5 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa98ec | `r0` | `r0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa9860).

## `PBayGetState`

- `PBayGetState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9910 to 0xa9930.
  Established. The word load reads absolute `0x1499bc` (`ldr` at 0xa9920). The exit carries
  `[0x1499bc]` in r0 (`ldmdb` at 0xa9928). There are no calls or conditional branches in the facts.
  State encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xa991c | load | 4 | `abs 0xa992c` | `-` |
| `ldr` at 0xa9920 | load | 4 | `abs 0x1499bc` | `-` |
| `ldmdb` at 0xa9928 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9928 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9928 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9928 | `r0` | `[0x1499bc]` |

## `PBayCmd`

- `PBayCmd`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9930 to 0xa9ed4.
  Established. A word store clears entry `r1 +0x0` (`str` at 0xa9958). Two later word stores write `ret(0x1b124)` at `[sp-32] +0x0` (`str` at 0xa99c4, `str` at 0xa9a04). The
  initial call to `0x8d2c8` has `[[[sp-28]+112]]`, `[sp-32]`, entry r1, `[[sp-28]+112]` (`bl` at
  0xa996c); its zero result takes the continuation and nonzero falls through (`beq` at 0xa9978).
  Command decoding beyond the unresolved selector remains unidentified (`ldrls` at 0xa9abc).
  Final r0 is unknown (`ldmdb` at 0xa9ea8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9940 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xa9944 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xa994c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xa9950 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9958 | store | 4 | `r1 +0x0` | `0x0` |
| `ldr` at 0xa995c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9960 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xa9964 | load | 4 | `[[sp-28]+112] +0x0` | `-` |
| `ldr` at 0xa9968 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9980 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xa9988 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9990 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xa9998 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xa99b4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa99c4 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa99c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa99cc | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa99d0 | load | 4 | `abs 0xa9eac` | `-` |
| `ldr` at 0xa99d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa99dc | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xa99e4 | load | 4 | `abs 0xa9eb0` | `-` |
| `ldr` at 0xa99e8 | load | 4 | `abs 0x142908` | `-` |
| `ldr` at 0xa99f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9a04 | store | 4 | `[sp-32] +0x0` | `ret(0x1b124)` |
| `ldr` at 0xa9a08 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xa9a0c | load | 4 | `abs 0xa9eb0` | `-` |
| `ldr` at 0xa9a10 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xa9a14 | load | 4 | `abs 0x142908` | `-` |
| `str` at 0xa9a20 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xa9a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9a30 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xa9a38 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xa9a44 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xa9a50 | store | 4 | `sp -0x24` | `ret(0xaa3d0)` |
| `ldr` at 0xa9a54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9a58 | store | 4 | `sp -0x34` | `ret(0xaa3d0)` |
| `ldr` at 0xa9a60 | load | 4 | `abs 0xa9eb4` | `-` |
| `ldr` at 0xa9a64 | load | 4 | `abs 0x1499d0` | `-` |
| `str` at 0xa9a7c | store | 4 | `sp -0x2c` | `ret(0x22a38)` |
| `ldr` at 0xa9a80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xa9a8c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9a90 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xa9aa0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9aa8 | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xa9ab0 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldrls` at 0xa9abc | load | 4 | `(unknown)` | `-` |
| `str` at 0xa9e60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xa9e64 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xa9e6c | load | 4 | `[sp-28] +0x70` | `-` |
| `ldr` at 0xa9e74 | load | 4 | `[[sp-28]+112] +0x4` | `-` |
| `ldr` at 0xa9e88 | load | 4 | `abs 0xa9eb4` | `-` |
| `ldr` at 0xa9e8c | load | 4 | `abs 0x1499d0` | `-` |
| `ldr` at 0xa9e9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xa9ea0 | store | 4 | `sp -0x34` | `[sp-36]` |
| `ldr` at 0xa9ea4 | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0xa9ea8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa9ea8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xa9ea8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9ea8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9ea8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa996c | `(unknown callee) 0x8d2c8` | `[[[sp-28]+112]]` | `[sp-32]` | `r1` | `[[sp-28]+112]` |
| `bl` at 0xa99a4 | `(unknown callee) 0xa9910` | `ret(0x8d2c8)` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xa99bc | `(unknown callee) 0x1b124` | `0x50` | `?` | `?` | `ret(0xa9910)` |
| `bl` at 0xa99d4 | `(unknown callee) 0x1b508` | `[[sp-32]]` | `0x126f7c` | `?` | `[sp-32]` |
| `bl` at 0xa99fc | `(unknown callee) 0x1b124` | `0x50` | `?` | `0x4` | `[0x142908]` |
| `bl` at 0xa9a18 | `(unknown callee) 0x1b508` | `[[sp-32]]` | `[0x142908]` | `0x142908` | `[sp-32]` |
| `bl` at 0xa9a48 | `(unknown callee) 0xaa3d0` | `[sp-32]` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xa9a74 | `(unknown callee) 0x22a38` | `[0x1499d0]+164` | `0x0` | `0x4` | `[0x1499d0]+164` |
| `bl` at 0xa9a9c | `(unknown callee) 0xa9834` | `0x3` | `?` | `?` | `ret(0x22a38)` |
| `bl` at 0xa9e84 | `(unknown callee) 0xa9834` | `0x2` | `?` | `0x4` | `[[[sp-28]+112]+4]` |
| `bl` at 0xa9e98 | `(unknown callee) 0x22dc0` | `[0x1499d0]+164` | `?` | `?` | `[0x1499d0]+164` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xa9978 | `ret(0x8d2c8) == 0x0` | `ret(0x8d2c8) != 0x0` |
| `bne` at 0xa99a0 | `[[[sp-28]+112]+4] != 0xf` | `[[[sp-28]+112]+4] == 0xf` |
| `beq` at 0xa99b0 | `ret(0xa9910) == 0x5` | `ret(0xa9910) != 0x5` |
| `beq` at 0xa99f0 | `[0x142908] == 0x0` | `[0x142908] != 0x0` |
| `bne` at 0xa9a40 | `[[[sp-28]+112]+4] != 0x10` | `[[[sp-28]+112]+4] == 0x10` |
| `beq` at 0xa9a88 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `ldrls` at 0xa9abc | `[[[sp-28]+112]+4]-1 <= 0xf (unsigned, 32-bit)` | `[[[sp-28]+112]+4]-1 > 0xf (unsigned, 32-bit)` |
| `beq` at 0xa9e7c | `[[[sp-28]+112]+4] == 0xf` | `[[[sp-28]+112]+4] != 0xf` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9ea8 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xa9abc).

## `PBayError`

- `PBayError`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xa9ed4 to 0xa9f10.
  Established. The sole call to `0x24028` receives `[0x144dc8]`, `0x126ff4`, entry r0, entry r1
  (`bl` at 0xa9f00). The exit propagates `ret(0x24028)` (`ldmdb` at 0xa9f04). Error formatting and
  hardware consequences are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xa9ee4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xa9ee8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xa9eec | load | 4 | `abs 0xa9f08` | `-` |
| `ldr` at 0xa9ef0 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xa9ef4 | load | 4 | `abs 0xa9f0c` | `-` |
| `ldr` at 0xa9ef8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xa9efc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xa9f04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xa9f04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xa9f04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xa9f00 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x126ff4` | `r0` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xa9f04 | `r0` | `ret(0x24028)` |

## `PBayInitFunc`

- `PBayInitFunc`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa440 to 0xaa500.
  Established. The halfword store at `[[sp-36]+8] +0x4` has unknown value (`strh` at 0xaa4e0). The
  opening call to `0x22920` has `r1+164`, `0x127028`, `0x0`, unknown (`bl` at 0xaa478); its zero
  result takes the continuation and nonzero falls through (`beq` at 0xaa484). The exit reports
  `ret(0xa9834)` (`ldmdb` at 0xaa4f8). Initialization record fields remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa450 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xaa454 | store | 4 | `sp -0x20` | `r1` |
| `ldr` at 0xaa458 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xaa45c | store | 4 | `sp -0x24` | `r1` |
| `ldr` at 0xaa460 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa46c | load | 4 | `abs 0xaa4fc` | `-` |
| `ldr` at 0xaa4a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xaa4ac | load | 4 | `[sp-36] +0x8` | `-` |
| `strh` at 0xaa4e0 | store | 2 | `[[sp-36]+8] +0x4` | `?` |
| `ldmdb` at 0xaa4f8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xaa4f8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaa4f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa4f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa4f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa478 | `(unknown callee) 0x22920` | `r1+164` | `0x127028` | `0x0` | `?` |
| `bl` at 0xaa490 | `(unknown callee) 0xa9ed4` | `?` | `0x11` | `?` | `ret(0x22920)` |
| `bl` at 0xaa498 | `(unknown callee) 0xa9834` | `0x5` | `?` | `?` | `?` |
| `bl` at 0xaa4a4 | `(unknown callee) 0xa9834` | `0x1` | `?` | `?` | `ret(0x22920)` |
| `bl` at 0xaa4b4 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `[sp-36]` |
| `bl` at 0xaa4c8 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0xaa4ec | `(unknown callee) 0xa9ed4` | `?` | `0x2` | `?` | `?` |
| `bl` at 0xaa4f4 | `(unknown callee) 0xa9834` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa484 | `ret(0x22920) == 0x0` | `ret(0x22920) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa4f8 | `r0` | `ret(0xa9834)` |

## `PBayInitIcmb`

- `PBayInitIcmb`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaa500 to 0xaa7dc.
  Established. The initial halfword store clears `[[sp-24]+8] +0x6` (`strh` at 0xaa528).
  Later halfword stores to the same printed target write `(ret(0xae66c) & 0xffff)` and `0x4400`,
  respectively (`strh` at 0xaa5d0, 0xaa640). The test
  after the call to `0xaa7dc` (`bl` at 0xaa5a8) takes its bypass on zero and falls through on
  nonzero (`beq` at 0xaa5bc). The next
  halfword comparison takes the continuation for `[[[sp-24]+8]+6]:2 != 0x0` and falls through on
  zero (`bne` at 0xaa628). The return reports `ret(0xa9834)` (`ldmdb` at 0xaa7d4). ICMB address
  meanings and reply formats remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaa510 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xaa51c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa520 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xaa528 | store | 2 | `[[sp-24]+8] +0x6` | `0x0` |
| `ldr` at 0xaa52c | load | 4 | `abs 0xaa7d8` | `-` |
| `strh` at 0xaa538 | store | 2 | `sp -0x1b4` | `0x0` |
| `str` at 0xaa540 | store | 4 | `sp -0x1b0` | `0x10` |
| `str` at 0xaa548 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xaa550 | store | 4 | `sp -0x118` | `0xff` |
| `str` at 0xaa558 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xaa560 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaa564 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa56c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaa57c | store | 4 | `sp -0x1b0` | `0x2` |
| `str` at 0xaa584 | store | 4 | `sp -0x28` | `0x3` |
| `str` at 0xaa58c | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xaa594 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaa598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa5a0 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaa5b0 | store | 4 | `sp -0x1c` | `ret(0xaa7dc)` |
| `ldr` at 0xaa5b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa5c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa5c4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa5c8 | load | 4 | `sp -0x2c` | `-` |
| `strh` at 0xaa5d0 | store | 2 | `[[sp-24]+8] +0x6` | `(ret(0xae66c) & 0xffff)` |
| `str` at 0xaa5d8 | store | 4 | `sp -0x1b0` | `0x12` |
| `str` at 0xaa5e0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0xaa5e4 | load | 4 | `abs 0xaa7d8` | `-` |
| `ldr` at 0xaa5e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa5ec | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xaa5f0 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `strh` at 0xaa5f8 | store | 2 | `sp -0x1b4` | `([[[sp-24]+8]+6]:2 & 0xffff)` |
| `str` at 0xaa600 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaa604 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa60c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa618 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa61c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrh` at 0xaa620 | load | 2 | `[[sp-24]+8] +0x6` | `-` |
| `ldr` at 0xaa634 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa638 | load | 4 | `[sp-24] +0x8` | `-` |
| `strh` at 0xaa640 | store | 2 | `[[sp-24]+8] +0x6` | `0x4400` |
| `ldr` at 0xaa648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa64c | load | 4 | `[sp-24] +0x8` | `-` |
| `strb` at 0xaa654 | store | 1 | `[[sp-24]+8] +0x0` | `0x0` |
| `str` at 0xaa65c | store | 4 | `sp -0x1b0` | `0x13` |
| `str` at 0xaa664 | store | 4 | `sp -0x24` | `0x3` |
| `str` at 0xaa66c | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0xaa670 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa678 | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaa688 | store | 4 | `sp -0x1c` | `ret(0xaa7dc)` |
| `ldr` at 0xaa68c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa6a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa6a8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa6ac | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xaa6b0 | load | 1 | `[sp-44] +0x0` | `-` |
| `strb` at 0xaa6b4 | store | 1 | `[[sp-24]+8] +0x0` | `([[sp-44]]:1 & 0xff)` |
| `str` at 0xaa6bc | store | 4 | `sp -0x1b0` | `0x20` |
| `str` at 0xaa6c4 | store | 4 | `sp -0x11c` | `0x0` |
| `str` at 0xaa6cc | store | 4 | `sp -0x118` | `0x0` |
| `ldr` at 0xaa6d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa6d8 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xaa6dc | load | 1 | `[[sp-24]+8] +0x0` | `-` |
| `strb` at 0xaa6e4 | store | 1 | `(unknown)` | `([[[sp-24]+8]]:1 & 0xff)` |
| `str` at 0xaa6ec | store | 4 | `sp -0x110` | `0x0` |
| `strb` at 0xaa6f8 | store | 1 | `sp -0x10c` | `0x0` |
| `str` at 0xaa700 | store | 4 | `sp -0xc8` | `0x0` |
| `ldr` at 0xaa704 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa70c | load | 4 | `[sp-24] +0x8` | `-` |
| `str` at 0xaa71c | store | 4 | `sp -0x1c` | `ret(0xaa7dc)` |
| `ldr` at 0xaa720 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xaa72c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xaa734 | store | 4 | `sp -0x1b8` | `ret(0xae3dc)` |
| `ldr` at 0xaa738 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa73c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa740 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xaa744 | load | 1 | `ret(0xae3dc) +0x1` | `-` |
| `strb` at 0xaa748 | store | 1 | `[[sp-24]+8] +0x1` | `([ret(0xae3dc)+1]:1 & 0xff)` |
| `ldr` at 0xaa74c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa750 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa754 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xaa758 | load | 1 | `[sp-440] +0x2` | `-` |
| `strb` at 0xaa75c | store | 1 | `[[sp-24]+8] +0x2` | `([[sp-440]+2]:1 & 0xff)` |
| `ldr` at 0xaa760 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa764 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa768 | load | 4 | `sp -0x1b8` | `-` |
| `ldrb` at 0xaa76c | load | 1 | `[sp-440] +0x3` | `-` |
| `strb` at 0xaa770 | store | 1 | `[[sp-24]+8] +0x3` | `([[sp-440]+3]:1 & 0xff)` |
| `ldr` at 0xaa774 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa778 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xaa77c | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xaa788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa78c | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xaa790 | load | 1 | `[[sp-24]+8] +0x1` | `-` |
| `ldr` at 0xaa79c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa7a0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldrb` at 0xaa7a4 | load | 1 | `[[sp-24]+8] +0x2` | `-` |
| `ldr` at 0xaa7b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa7b4 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldr` at 0xaa7bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaa7c0 | load | 4 | `[sp-24] +0x8` | `-` |
| `ldmdb` at 0xaa7d4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xaa7d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaa7d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaa7d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaa518 | `(unknown callee) 0xa9834` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0xaa574 | `(unknown callee) 0xaa7dc` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaa5a8 | `(unknown callee) 0xaa7dc` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaa5cc | `(unknown callee) 0xae66c` | `[sp-44]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaa614 | `(unknown callee) 0xaa7dc` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaa630 | `(unknown callee) 0xa9834` | `0x5` | `?` | `?` | `[[[sp-24]+8]+6]:2` |
| `bl` at 0xaa680 | `(unknown callee) 0xaa7dc` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaa69c | `(unknown callee) 0xa9834` | `0x5` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xaa714 | `(unknown callee) 0xaa7dc` | `[[sp-24]+8]` | `sp-436` | `sp-436` | `[sp-24]` |
| `bl` at 0xaa730 | `(unknown callee) 0xae3dc` | `[sp-44]` | `?` | `?` | `ret(0xaa7dc)` |
| `bl` at 0xaa798 | `(unknown callee) 0xaddb8` | `[[[sp-24]+8]+1]:1` | `?` | `[[sp-24]+8]` | `[[[sp-24]+8]+1]:1` |
| `bl` at 0xaa7b8 | `(unknown callee) 0xaac6c` | `[[sp-24]+8]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xaa7c8 | `(unknown callee) 0xaca9c` | `[[sp-24]+8]` | `0x0` | `?` | `[sp-24]` |
| `bl` at 0xaa7d0 | `(unknown callee) 0xa9834` | `0x2` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaa5bc | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |
| `bne` at 0xaa628 | `[[[sp-24]+8]+6]:2 != 0x0` | `[[[sp-24]+8]+6]:2 == 0x0` |
| `bne` at 0xaa694 | `ret(0xaa7dc) != 0x0` | `ret(0xaa7dc) == 0x0` |
| `beq` at 0xaa728 | `ret(0xaa7dc) == 0x0` | `ret(0xaa7dc) != 0x0` |
| `beq` at 0xaa784 | `[[[sp-24]+8]+1]:1 == 0x0` | `[[[sp-24]+8]+1]:1 != 0x0` |
| `beq` at 0xaa7ac | `[[[sp-24]+8]+2]:1 == 0x0` | `[[[sp-24]+8]+2]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaa7d4 | `r0` | `ret(0xa9834)` |
