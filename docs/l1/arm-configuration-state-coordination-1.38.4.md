# ARM configuration state coordination in 1.38.4

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
L1CfgRequest and L1CfgGetCfg have unresolved computed selectors. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/configuration-state-coordination-facts.txt) retain all tool sections.

## `L1CfgLock`

- `L1CfgLock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c0d0 to 0x6c168.
  Established. The call to `0x22a38` has `0x1488cc`, `0xc8`, entry r2 and entry r3 as candidates
  (`bl` at 0x6c0ec); its zero result takes the branch and nonzero falls through (`beq` at 0x6c100).
  The return records `ret(0x1b508)` (`ldmdb` at 0x6c14c). Lock ownership and timeout units remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6c0e0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6c0e4 | load | 4 | `abs 0x6c150` | `-` |
| `str` at 0x6c0f4 | store | 4 | `sp -0x18` | `ret(0x22a38)` |
| `ldr` at 0x6c0f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6c104 | load | 4 | `abs 0x6c154` | `-` |
| `str` at 0x6c108 | store | 4 | `sp -0x2c` | `0x10f1d0` |
| `ldr` at 0x6c10c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6c110 | store | 4 | `sp -0x28` | `ret(0x22a38)` |
| `ldr` at 0x6c114 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6c118 | store | 4 | `sp -0x24` | `[sp-20]` |
| `ldr` at 0x6c11c | load | 4 | `abs 0x6c158` | `-` |
| `str` at 0x6c120 | store | 4 | `sp -0x20` | `0x2b1628` |
| `ldr` at 0x6c124 | load | 4 | `abs 0x6c15c` | `-` |
| `str` at 0x6c128 | store | 4 | `sp -0x1c` | `0x2b1650` |
| `ldr` at 0x6c130 | load | 4 | `abs 0x6c160` | `-` |
| `ldr` at 0x6c134 | load | 4 | `abs 0x6c164` | `-` |
| `ldr` at 0x6c140 | load | 4 | `abs 0x6c158` | `-` |
| `ldr` at 0x6c144 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6c14c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c14c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c14c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c0ec | `(unknown callee) 0x22a38` | `0x1488cc` | `0xc8` | `r2` | `r3` |
| `bl` at 0x6c13c | `(unknown callee) 0x21e28` | `0x9` | `0x10f0d4` | `0x27b` | `0x0` |
| `bl` at 0x6c148 | `(unknown callee) 0x1b508` | `0x2b1628` | `[sp-20]` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c100 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c14c | `r0` | `ret(0x1b508)` |

## `L1CfgUnlock`

- `L1CfgUnlock`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c168 to 0x6c1f0.
  Established. The opening call to `0x1b508` receives `0x2b1650`, `0x2b1628`, entry r2 and entry r3
  as candidates (`bl` at 0x6c180). The later branch takes `ret(0x22dc0) == 0x0` and falls through on
  nonzero (`beq` at 0x6c1a8). The final r0 expression and external release semantics remain unknown
  (`ldmdb` at 0x6c1d0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6c178 | load | 4 | `abs 0x6c1d4` | `-` |
| `ldr` at 0x6c17c | load | 4 | `abs 0x6c1d8` | `-` |
| `ldr` at 0x6c184 | load | 4 | `abs 0x6c1d8` | `-` |
| `ldr` at 0x6c188 | load | 4 | `abs 0x6c1dc` | `-` |
| `ldr` at 0x6c190 | load | 4 | `abs 0x6c1e0` | `-` |
| `str` at 0x6c19c | store | 4 | `sp -0x14` | `ret(0x22dc0)` |
| `ldr` at 0x6c1a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6c1ac | load | 4 | `abs 0x6c1e4` | `-` |
| `str` at 0x6c1b0 | store | 4 | `sp -0x1c` | `0x10f214` |
| `ldr` at 0x6c1b4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6c1b8 | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0x6c1c0 | load | 4 | `abs 0x6c1e8` | `-` |
| `ldr` at 0x6c1c4 | load | 4 | `abs 0x6c1ec` | `-` |
| `ldmdb` at 0x6c1d0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c1d0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c1d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c180 | `(unknown callee) 0x1b508` | `0x2b1650` | `0x2b1628` | `r2` | `r3` |
| `bl` at 0x6c18c | `(unknown callee) 0x1b508` | `0x2b1628` | `0x10f208` | `?` | `?` |
| `bl` at 0x6c194 | `(unknown callee) 0x22dc0` | `0x1488cc` | `?` | `?` | `?` |
| `bl` at 0x6c1cc | `(unknown callee) 0x21e28` | `0x9` | `0x10f0d4` | `0x28b` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c1a8 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c1d0 | `r0` | `?` |

## `L1CfgProcessResponse`

- `L1CfgProcessResponse`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x727a8 to 0x72878.
  Established. The first branch takes `[r0+19]:1 != 0x0` and falls through on zero (`bne` at
  0x727c8). The call to `0xb7944` has entry r0, `0x0`, entry r2 and unknown as candidates (`bl` at
  0x727f8). The common return value is unknown (`ldmdb` at 0x72864). Response-type handling remains
  unproved beyond the listed predicates and calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x727b8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x727bc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x727c0 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x727cc | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x727d0 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x727e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x727e4 | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x727e8 | store | 4 | `sp -0x24` | `[r0+40]` |
| `ldr` at 0x727f0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x727fc | store | 4 | `sp -0x24` | `ret(0xb7944)` |
| `ldr` at 0x72800 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x72804 | store | 4 | `sp -0x20` | `[sp-36]` |
| `ldr` at 0x72808 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x72818 | load | 4 | `abs 0x72868` | `-` |
| `ldr` at 0x7281c | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x7282c | load | 4 | `abs 0x7286c` | `-` |
| `ldr` at 0x72830 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x72834 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x72838 | load | 4 | `abs 0x72870` | `-` |
| `ldr` at 0x7283c | load | 4 | `[sp-24] +0xc` | `-` |
| `ldr` at 0x72844 | load | 4 | `abs 0x72874` | `-` |
| `str` at 0x72850 | store | 4 | `abs 0x148670` | `ret(0x1b1e4)` |
| `ldr` at 0x72858 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x72864 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x72864 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x72864 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x72864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x727f8 | `(unknown callee) 0xb7944` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x72840 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f918` | `[[sp-24]+12]` | `0x144dc8` |
| `bl` at 0x7284c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x72860 | `(unknown callee) 0x6eac8` | `[sp-24]` | `0x0` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x727c8 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `bne` at 0x727dc | `? != 0x0` | `? == 0x0` |
| `beq` at 0x72810 | `[sp-36] == 0x3e8` | `[sp-36] != 0x3e8` |
| `beq` at 0x72828 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x72864 | `r0` | `?` |

## `L1CfgProcessEvent`

- `L1CfgProcessEvent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x72878 to 0x729ec.
  Established. The entry-byte guard takes `[r0+19]:1 != 0x0` and falls through on zero (`bne` at
  0x72898). One call to `0x75594` carries entry r0, entry r1, `[r0+34]:2` and `[r0+34]:2` as
  candidates (`bl` at 0x72968). The final value is unknown (`ldmdb` at 0x729e4). Event-code meanings
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x72888 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7288c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x72890 | load | 1 | `r0 +0x13` | `-` |
| `ldr` at 0x7289c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x728a0 | load | 2 | `r0 +0x22` | `-` |
| `str` at 0x728a4 | store | 4 | `sp -0x8c` | `[r0+34]:2` |
| `ldr` at 0x728a8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728b4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728c0 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728cc | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728d8 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728e4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x728f4 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72900 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72910 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x7291c | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72928 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72934 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72944 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72950 | load | 4 | `abs 0x729e8` | `-` |
| `ldr` at 0x72954 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x72964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x7297c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72988 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x72994 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x729dc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x729e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x729e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x729e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x72968 | `(unknown callee) 0x75594` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x72974 | `(unknown callee) 0x752f4` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x72980 | `(unknown callee) 0x75798` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x7298c | `(unknown callee) 0x75908` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x72998 | `(unknown callee) 0x75a10` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729a4 | `(unknown callee) 0x75b18` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729b0 | `(unknown callee) 0x75c20` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729bc | `(unknown callee) 0x75d30` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729c8 | `(unknown callee) 0x76528` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729d4 | `(unknown callee) 0x76c10` | `r0` | `r1` | `[r0+34]:2` | `[r0+34]:2` |
| `bl` at 0x729e0 | `(unknown callee) 0x71880` | `r0` | `r1` | `[r0+34]:2` | `0x3e9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x72898 | `[r0+19]:1 != 0x0` | `[r0+19]:1 == 0x0` |
| `beq` at 0x728b0 | `[r0+34]:2 == 0x6` | `[r0+34]:2 != 0x6` |
| `bgt` at 0x728bc | `[r0+34]:2 > 0x6 (signed, 32-bit)` | `[r0+34]:2 <= 0x6 (signed, 32-bit)` |
| `beq` at 0x728c8 | `[r0+34]:2 == 0x3` | `[r0+34]:2 != 0x3` |
| `bgt` at 0x728d4 | `[r0+34]:2 > 0x3 (signed, 32-bit)` | `[r0+34]:2 <= 0x3 (signed, 32-bit)` |
| `beq` at 0x728e0 | `[r0+34]:2 == 0x1` | `[r0+34]:2 != 0x1` |
| `beq` at 0x728ec | `[r0+34]:2 == 0x2` | `[r0+34]:2 != 0x2` |
| `beq` at 0x728fc | `[r0+34]:2 == 0x4` | `[r0+34]:2 != 0x4` |
| `beq` at 0x72908 | `[r0+34]:2 == 0x5` | `[r0+34]:2 != 0x5` |
| `beq` at 0x72918 | `[r0+34]:2 == 0x64` | `[r0+34]:2 != 0x64` |
| `bgt` at 0x72924 | `[r0+34]:2 > 0x64 (signed, 32-bit)` | `[r0+34]:2 <= 0x64 (signed, 32-bit)` |
| `beq` at 0x72930 | `[r0+34]:2 == 0x7` | `[r0+34]:2 != 0x7` |
| `beq` at 0x7293c | `[r0+34]:2 == 0x8` | `[r0+34]:2 != 0x8` |
| `beq` at 0x7294c | `[r0+34]:2 == 0x66` | `[r0+34]:2 != 0x66` |
| `beq` at 0x7295c | `[r0+34]:2 == 0x3e9` | `[r0+34]:2 != 0x3e9` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x729e4 | `r0` | `?` |

## `L1CfgRingEvent`

- `L1CfgRingEvent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e7f0 to 0x6e9f0.
  Established. The first branch retains an unknown operand (`bne` at 0x6e844). The call to `0xb7944`
  receives entry r0, `0x0`, entry r2 and unknown as candidates (`bl` at 0x6e860). The common exit
  has an unknown r0 expression (`ldmdb` at 0x6e9d4). Ring-event field meanings and external effects
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6e800 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6e804 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6e808 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e80c | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e81c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6e820 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e824 | load | 4 | `r0 +0x1c` | `-` |
| `str` at 0x6e830 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6e834 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e838 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x6e848 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e84c | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x6e850 | store | 4 | `sp -0x30` | `[r0+40]` |
| `ldr` at 0x6e858 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e864 | store | 4 | `sp -0x30` | `ret(0xb7944)` |
| `ldr` at 0x6e868 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x6e878 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e87c | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6e88c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e890 | load | 4 | `[sp-20] +0x30` | `-` |
| `ldr` at 0x6e8a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e8b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e8bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6e8c0 | store | 4 | `[sp-20] +0x38` | `[sp-24]` |
| `ldr` at 0x6e8c4 | load | 4 | `abs 0x6e9d8` | `-` |
| `ldr` at 0x6e8c8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e8d8 | load | 4 | `abs 0x6e9dc` | `-` |
| `ldr` at 0x6e8dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e8e0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e8f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e8f4 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6e900 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e904 | load | 4 | `abs 0x6e9e0` | `-` |
| `ldr` at 0x6e90c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e914 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x6e91c | store | 4 | `[sp-20] +0x30` | `[[sp-20]+48]+1` |
| `ldr` at 0x6e920 | load | 4 | `abs 0x6e9e4` | `-` |
| `ldr` at 0x6e924 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6e928 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x6e934 | load | 4 | `abs 0x6e9e4` | `-` |
| `ldr` at 0x6e938 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6e93c | load | 4 | `abs 0x148938` | `-` |
| `ldr` at 0x6e948 | load | 4 | `abs 0x6e9e8` | `-` |
| `str` at 0x6e94c | store | 4 | `sp -0x28` | `0x148658` |
| `ldr` at 0x6e950 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e954 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6e964 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e968 | load | 4 | `[sp-20] +0x30` | `-` |
| `str` at 0x6e96c | store | 4 | `sp -0x2c` | `[[sp-20]+48]` |
| `ldr` at 0x6e974 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e980 | store | 4 | `sp -0x2c` | `ret(0xb7944)` |
| `ldr` at 0x6e984 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6e988 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6e98c | store | 4 | `[sp-40] +0x0` | `[sp-44]` |
| `ldr` at 0x6e990 | load | 4 | `abs 0x6e9d8` | `-` |
| `ldr` at 0x6e994 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6e9a4 | load | 4 | `abs 0x6e9dc` | `-` |
| `ldr` at 0x6e9a8 | load | 4 | `abs 0x6e9e8` | `-` |
| `ldr` at 0x6e9ac | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6e9b0 | load | 4 | `abs 0x6e9ec` | `-` |
| `ldr` at 0x6e9b4 | load | 4 | `abs 0x148658` | `-` |
| `str` at 0x6e9c0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x6e9cc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6e9d0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x6e9d4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6e9d4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6e9d4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6e860 | `(unknown callee) 0xb7944` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x6e8a8 | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `[sp-48]` | `?` |
| `bl` at 0x6e908 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f5d4` | `?` | `?` |
| `bl` at 0x6e97c | `(unknown callee) 0xb7944` | `[sp-20]` | `0x1` | `[sp-32]` | `?` |
| `bl` at 0x6e9b8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f5fc` | `[0x148658]` | `0x144dc8` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6e844 | `? != 0x0` | `? == 0x0` |
| `bhi` at 0x6e874 | `[sp-48]-1 > 0x2 (unsigned, 32-bit)` | `[sp-48]-1 <= 0x2 (unsigned, 32-bit)` |
| `bne` at 0x6e888 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6e898 | `[[sp-20]+48] != 0x0` | `[[sp-20]+48] == 0x0` |
| `bne` at 0x6e8b4 | `ret(0xb7944) != 0x0` | `ret(0xb7944) == 0x0` |
| `beq` at 0x6e8d4 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x6e930 | `[sp-28] != [0x148934]` | `[sp-28] == [0x148934]` |
| `bne` at 0x6e944 | `[sp-32] != [0x148938]` | `[sp-32] == [0x148938]` |
| `bne` at 0x6e960 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e9a0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6e9d4 | `r0` | `?` |

## `L1CfgFindState`

- `L1CfgFindState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6f8a0 to 0x6f9f8.
  Established. The opening branch takes `r2 != [0x148934]` and falls through on equality (`bne` at
  0x6f8d0). The final r0 value is unknown (`ldmdb` at 0x6f9f0). The lookup record geometry and state
  identity remain unknown beyond the explicit accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6f8b0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6f8b4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6f8b8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x6f8bc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x6f8c0 | load | 4 | `abs 0x6f9f4` | `-` |
| `ldr` at 0x6f8c4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f8c8 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x6f8d4 | load | 4 | `abs 0x6f9f4` | `-` |
| `ldr` at 0x6f8d8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6f8dc | load | 4 | `abs 0x148938` | `-` |
| `ldr` at 0x6f8e8 | load | 4 | `abs 0x6f9f4` | `-` |
| `str` at 0x6f8ec | store | 4 | `sp -0x2c` | `0x14892c` |
| `str` at 0x6f8f8 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x6f900 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x6f904 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f91c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f920 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6f930 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f944 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f94c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f950 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6f95c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6f978 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6f97c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x6f98c | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x6f990 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6f998 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x6f9a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x6f9ac | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6f9c0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6f9c8 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x6f9cc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6f9d4 | store | 4 | `sp -0x30` | `[sp-48]-112` |
| `str` at 0x6f9e0 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x6f9e4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0x6f9e8 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x6f9ec | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x6f9f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6f9f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6f9f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6f8d0 | `r2 != [0x148934]` | `r2 == [0x148934]` |
| `bne` at 0x6f8e4 | `r3 != [0x148938]` | `r3 == [0x148938]` |
| `bne` at 0x6f90c | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x6f918 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x6f928 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6f958 | `? != [sp-28]` | `? == [sp-28]` |
| `bne` at 0x6f984 | `? != [sp-32]` | `? == [sp-32]` |
| `beq` at 0x6f9a8 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6f9f0 | `r0` | `?` |

## `L1CfgUpdateState`

- `L1CfgUpdateState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71880 to 0x71a6c.
  Established. The initial decision has an unknown operand (`bne` at 0x718a4). The first call to
  `0xb7944` carries entry r0, `0x0`, entry r2 and unknown as candidates (`bl` at 0x718c0). The
  recorded final r0 value is unknown (`ldmdb` at 0x71a50). State-update propagation remains
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71890 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x71894 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x71898 | load | 1 | `r0 +0x24` | `-` |
| `ldr` at 0x718a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x718ac | load | 4 | `r0 +0x28` | `-` |
| `str` at 0x718b0 | store | 4 | `sp -0x1a0` | `[r0+40]` |
| `ldr` at 0x718b8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x718c4 | store | 4 | `sp -0x1a0` | `ret(0xb7944)` |
| `ldr` at 0x718c8 | load | 4 | `sp -0x1a0` | `-` |
| `str` at 0x718cc | store | 4 | `sp -0x120` | `[sp-416]` |
| `ldr` at 0x718d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x718d4 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x718e4 | store | 4 | `sp -0x124` | `?` |
| `ldr` at 0x718e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x718ec | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0x718f8 | store | 4 | `sp -0x128` | `?` |
| `ldr` at 0x718fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71900 | load | 4 | `[sp-24] +0xc` | `-` |
| `str` at 0x71904 | store | 4 | `sp -0x12c` | `[[sp-24]+12]` |
| `ldr` at 0x71908 | load | 4 | `abs 0x71a54` | `-` |
| `ldr` at 0x7190c | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x71910 | load | 4 | `abs 0x148934` | `-` |
| `ldr` at 0x7191c | load | 4 | `abs 0x71a54` | `-` |
| `ldr` at 0x71920 | load | 4 | `sp -0x128` | `-` |
| `ldr` at 0x71924 | load | 4 | `abs 0x148938` | `-` |
| `ldr` at 0x71934 | load | 4 | `abs 0x71a58` | `-` |
| `str` at 0x71940 | store | 4 | `abs 0x148678` | `ret(0x1b1e4)` |
| `ldr` at 0x71944 | load | 4 | `abs 0x71a5c` | `-` |
| `ldr` at 0x7194c | load | 4 | `abs 0x71a60` | `-` |
| `ldr` at 0x71950 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x7196c | load | 4 | `abs 0x71a64` | `-` |
| `ldr` at 0x71970 | load | 4 | `sp -0x12c` | `-` |
| `str` at 0x71974 | store | 4 | `sp -0x1ac` | `[sp-300]` |
| `str` at 0x71978 | store | 4 | `sp -0x1a8` | `ret(0x1b1e4)` |
| `ldr` at 0x7197c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x71980 | load | 4 | `abs 0x71a68` | `-` |
| `ldr` at 0x71984 | load | 4 | `sp -0x124` | `-` |
| `ldr` at 0x71988 | load | 4 | `sp -0x128` | `-` |
| `str` at 0x71994 | store | 4 | `sp -0x11c` | `0x0` |
| `ldr` at 0x71998 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x7199c | load | 4 | `sp -0x120` | `-` |
| `ldr` at 0x719ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x719b0 | load | 4 | `sp -0x11c` | `-` |
| `ldrb` at 0x719c4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x719d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x719d8 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x719ec | load | 4 | `(unknown)` | `-` |
| `str` at 0x719f0 | store | 4 | `sp -0x1a4` | `?` |
| `ldr` at 0x719f8 | load | 4 | `sp -0x11c` | `-` |
| `ldr` at 0x71a00 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x71a0c | store | 4 | `sp -0x1a4` | `ret(0xb7944)` |
| `ldr` at 0x71a18 | load | 4 | `sp -0x1a4` | `-` |
| `ldr` at 0x71a28 | load | 4 | `sp -0x194` | `-` |
| `ldr` at 0x71a2c | load | 4 | `sp -0x190` | `-` |
| `ldr` at 0x71a3c | load | 4 | `sp -0x11c` | `-` |
| `str` at 0x71a44 | store | 4 | `sp -0x11c` | `[sp-284]+1` |
| `ldmdb` at 0x71a50 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x71a50 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x71a50 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x71a50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x718c0 | `(unknown callee) 0xb7944` | `r0` | `0x0` | `r2` | `?` |
| `bl` at 0x7193c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x71948 | `(unknown callee) 0x6c0d0` | `0x10f7b4` | `?` | `?` | `?` |
| `bl` at 0x71964 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x7198c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f7c8` | `[sp-292]` | `[sp-296]` |
| `bl` at 0x71a08 | `(unknown callee) 0xb7944` | `[sp-24]` | `?` | `0x2c` | `?` |
| `bl` at 0x71a20 | `(unknown callee) 0x71cbc` | `sp-412` | `?` | `0x70` | `sp-412` |
| `bl` at 0x71a38 | `(unknown callee) 0x711f8` | `[sp-404]` | `[sp-400]` | `sp-412` | `0x1` |
| `bl` at 0x71a4c | `(unknown callee) 0x6c168` | `?` | `?` | `?` | `[sp-288]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x718a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x71918 | `[sp-292] != [0x148934]` | `[sp-292] == [0x148934]` |
| `bne` at 0x7192c | `[sp-296] != [0x148938]` | `[sp-296] == [0x148938]` |
| `beq` at 0x7195c | `? == 0x0` | `? != 0x0` |
| `blt` at 0x719a4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x719d0 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x71a50 | `r0` | `?` |

## `L1CfgRequest`

- `L1CfgRequest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71ea8 to 0x720ec.
  Established. The call to `0x79dcc` has `[sp-84]`, entry r1, `0x1489d0` and unknown as candidates
  (`bl` at 0x71ed8). Nonzero takes the branch and zero falls through (`bne` at 0x71ee4). The
  subsequent selector remains unresolved (`ldrls` at 0x71f10). Selected request construction and the
  final r0 value remain unknown beyond known edges.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71eb8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x71ec0 | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x71ec4 | load | 4 | `abs 0x720c4` | `-` |
| `ldr` at 0x71ec8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71ecc | load | 4 | `(unknown)` | `-` |
| `str` at 0x71ed0 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x71ed4 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x71efc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71f04 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x71f10 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x720c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x720c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x720c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71ed8 | `(unknown callee) 0x79dcc` | `[sp-84]` | `r1` | `0x1489d0` | `?` |
| `bl` at 0x71f00 | `(unknown callee) 0x6907c` | `sp-76` | `sp-72` | `[sp-20]` | `sp-76` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x71ee4 | `ret(0x79dcc) != 0x0` | `ret(0x79dcc) == 0x0` |
| `ldrls` at 0x71f10 | `[sp-20]-1 <= 0x18 (unsigned, 32-bit)` | `[sp-20]-1 > 0x18 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x720c0 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x71f10).

## `L1CfgGetCfg`

- `L1CfgGetCfg`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74908 to 0x74ab8.
  Established. The selector takes `r0 <= 0x28 (unsigned, 32-bit)` and falls through above the bound
  (`ldrls` at 0x74924); its taken edges remain unresolved. The known fall-through exit carries
  `0x148bd0` (`ldmdb` at 0x74aac). Configuration mappings on selected edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74918 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x7491c | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x74924 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x74a6c | load | 4 | `abs 0x74ab4` | `-` |
| `str` at 0x74a70 | store | 4 | `sp -0x28` | `0x148bd0` |
| `ldr` at 0x74aa8 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0x74aac | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x74aac | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x74aac | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x74924 | `r0 <= 0x28 (unsigned, 32-bit)` | `r0 > 0x28 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x74aac | `r0` | `0x148bd0` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x74924).

## `L1CfgEventSystemSN`

- `L1CfgEventSystemSN`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x751c4 to 0x75270.
  Established. The opening call to `0x74908` receives `0x0`, entry r1, entry r2 and entry r3 as
  candidates (`bl` at 0x751dc). The branch takes `[sp-20] == 0x0` and falls through on nonzero
  (`beq` at 0x75218). The return records `ret(0x6b2b0)` (`ldmdb` at 0x75268). System-serial-number
  representation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x751d4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x751e4 | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `str` at 0x751ec | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x751fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75210 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x75228 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x75248 | load | 4 | `abs 0x7526c` | `-` |
| `ldmdb` at 0x75268 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75268 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75268 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x751dc | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x751f4 | `(unknown callee) 0x1f1dc` | `0xd` | `?` | `?` | `0x0` |
| `bl` at 0x7520c | `(unknown callee) 0x1b508` | `[sp-24]+76` | `ret(0x1f1dc)` | `ret(0x1f1dc)` | `[sp-24]+76` |
| `bl` at 0x75220 | `(unknown callee) 0x1f1dc` | `0xd` | `?` | `?` | `[sp-20]` |
| `bl` at 0x75238 | `(unknown callee) 0x1b508` | `[sp-24]+100` | `ret(0x1f1dc)` | `[sp-24]+100` | `ret(0x1f1dc)` |
| `bl` at 0x75240 | `(unknown callee) 0x1f1dc` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x75250 | `(unknown callee) 0x1b508` | `0x2b1b48` | `ret(0x1f1dc)` | `?` | `ret(0x1f1dc)` |
| `bl` at 0x75260 | `(unknown callee) 0x74c78` | `0x7` | `0x0` | `0x0` | `?` |
| `bl` at 0x75264 | `(unknown callee) 0x6b2b0` | `ret(0x74c78)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x75218 | `[sp-20] == 0x0` | `[sp-20] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75268 | `r0` | `ret(0x6b2b0)` |

## `L1CfgMeshTask`

- `L1CfgMeshTask`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b320 to 0x6b538.
  Established. The call to `0x226f0` has `0x64` followed by three unknown candidates (`bl` at
  0x6b354). The first branch takes `[0x148680] == 0x0` and falls through on nonzero (`beq` at
  0x6b364). No exit is recorded on known edges. Mesh-task timing units and coordination policy
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6b330 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6b334 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x6b33c | store | 4 | `sp -0x1c` | `0x0` |
| `str` at 0x6b344 | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0x6b34c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x6b358 | load | 4 | `abs 0x6b510` | `-` |
| `ldr` at 0x6b35c | load | 4 | `abs 0x148680` | `-` |
| `ldr` at 0x6b374 | load | 4 | `abs 0x6b510` | `-` |
| `ldr` at 0x6b378 | load | 4 | `abs 0x148680` | `-` |
| `ldr` at 0x6b388 | load | 4 | `abs 0x6b514` | `-` |
| `ldr` at 0x6b390 | load | 4 | `abs 0x6b510` | `-` |
| `str` at 0x6b398 | store | 4 | `abs 0x148680` | `0x0` |
| `ldr` at 0x6b39c | load | 4 | `abs 0x6b518` | `-` |
| `ldr` at 0x6b3a0 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6b3b0 | load | 4 | `abs 0x6b51c` | `-` |
| `ldr` at 0x6b3b4 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6b3b8 | load | 4 | `abs 0x6b520` | `-` |
| `ldr` at 0x6b3c4 | load | 4 | `abs 0x6b524` | `-` |
| `str` at 0x6b3cc | store | 4 | `abs 0x142874` | `0x5` |
| `ldr` at 0x6b3d4 | load | 4 | `abs 0x6b528` | `-` |
| `ldr` at 0x6b3d8 | load | 4 | `abs 0x148670` | `-` |
| `ldr` at 0x6b3f0 | load | 4 | `abs 0x6b528` | `-` |
| `ldr` at 0x6b3f4 | load | 4 | `abs 0x148670` | `-` |
| `str` at 0x6b3fc | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x6b400 | load | 4 | `abs 0x6b52c` | `-` |
| `ldr` at 0x6b404 | load | 4 | `abs 0x148674` | `-` |
| `ldr` at 0x6b41c | load | 4 | `abs 0x6b52c` | `-` |
| `ldr` at 0x6b420 | load | 4 | `abs 0x148674` | `-` |
| `str` at 0x6b428 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x6b42c | load | 4 | `abs 0x6b530` | `-` |
| `ldr` at 0x6b430 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x6b43c | load | 4 | `abs 0x6b524` | `-` |
| `ldr` at 0x6b440 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b444 | load | 4 | `abs 0x142874` | `-` |
| `str` at 0x6b458 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x6b460 | load | 4 | `abs 0x6b534` | `-` |
| `ldr` at 0x6b464 | load | 4 | `abs 0x14866c` | `-` |
| `ldr` at 0x6b470 | load | 4 | `abs 0x6b534` | `-` |
| `ldr` at 0x6b474 | load | 4 | `abs 0x6b528` | `-` |
| `ldr` at 0x6b478 | load | 4 | `abs 0x148670` | `-` |
| `str` at 0x6b47c | store | 4 | `abs 0x14866c` | `[0x148670]` |
| `str` at 0x6b488 | store | 4 | `sp -0x28` | `ret(0x1f1dc)` |
| `ldr` at 0x6b48c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x6b490 | store | 4 | `sp -0x2c` | `ret(0x1f1dc)` |
| `ldr` at 0x6b494 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6b4a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x6b4b0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6b4bc | load | 4 | `abs 0x6b524` | `-` |
| `ldr` at 0x6b4c0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x6b4c4 | load | 4 | `abs 0x142874` | `-` |
| `ldr` at 0x6b4d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x6b4e4 | load | 4 | `abs 0x6b524` | `-` |
| `str` at 0x6b4ec | store | 4 | `abs 0x142874` | `0xf` |
| `ldr` at 0x6b4f4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x6b508 | store | 4 | `sp -0x24` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b354 | `(unknown callee) 0x226f0` | `0x64` | `?` | `?` | `?` |
| `bl` at 0x6b36c | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x148680]` |
| `bl` at 0x6b38c | `(unknown callee) 0x6c0d0` | `0x10f150` | `?` | `[0x148680]` | `ret(0x1b1e4)` |
| `bl` at 0x6b3bc | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f160` | `0x148680` | `0x144dc8` |
| `bl` at 0x6b3c0 | `(unknown callee) 0x6d51c` | `?` | `?` | `?` | `?` |
| `bl` at 0x6b3e8 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x148670]` |
| `bl` at 0x6b414 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x148674]` |
| `bl` at 0x6b484 | `(unknown callee) 0x1f1dc` | `0x9a` | `?` | `?` | `?` |
| `bl` at 0x6b4d4 | `(unknown callee) 0x6cf78` | `ret(0x1f1dc)` | `?` | `?` | `?` |
| `bl` at 0x6b500 | `(unknown callee) 0x6cf78` | `ret(0x1f1dc)` | `?` | `?` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6b364 | `[0x148680] == 0x0` | `[0x148680] != 0x0` |
| `bge` at 0x6b380 | `ret(0x1b1e4) >= [0x148680] (signed, 32-bit)` | `ret(0x1b1e4) < [0x148680] (signed, 32-bit)` |
| `beq` at 0x6b3ac | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6b3e0 | `[0x148670] == 0x0` | `[0x148670] != 0x0` |
| `beq` at 0x6b40c | `[0x148674] == 0x0` | `[0x148674] != 0x0` |
| `beq` at 0x6b438 | `[0x1489b8] == 0x1` | `[0x1489b8] != 0x1` |
| `blt` at 0x6b44c | `[sp-28] < [0x142874] (signed, 32-bit)` | `[sp-28] >= [0x142874] (signed, 32-bit)` |
| `bne` at 0x6b46c | `[0x14866c] != 0x0` | `[0x14866c] == 0x0` |
| `beq` at 0x6b49c | `ret(0x1f1dc) == 0x0` | `ret(0x1f1dc) != 0x0` |
| `beq` at 0x6b4a8 | `ret(0x1f1dc) == 0x1` | `ret(0x1f1dc) != 0x1` |
| `beq` at 0x6b4b8 | `[sp-32] == 0x0` | `[sp-32] != 0x0` |
| `bge` at 0x6b4cc | `[sp-32] >= [0x142874] (signed, 32-bit)` | `[sp-32] < [0x142874] (signed, 32-bit)` |
| `ble` at 0x6b4e0 | `[sp-28] <= 0x3c (signed, 32-bit)` | `[sp-28] > 0x3c (signed, 32-bit)` |
| `beq` at 0x6b4fc | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|

## `L1CfgAddState`

- `L1CfgAddState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x711f8 to 0x71544.
  Established. The entry-r3 guard takes zero and falls through on nonzero (`beq` at 0x71228). The
  first call to `0x6f784` receives `[0x1489c0]`, `[0x1489b8]`, entry r0 and entry r1 as candidates
  (`bl` at 0x71244). The common r0 value is unknown (`ldmdb` at 0x71528). State-record insertion
  geometry remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71208 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x7120c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x71210 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0x71214 | store | 4 | `sp -0x24` | `r3` |
| `str` at 0x7121c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x71220 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7122c | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71230 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71234 | load | 4 | `abs 0x1489c0` | `-` |
| `ldr` at 0x71238 | load | 4 | `abs 0x1489b8` | `-` |
| `ldr` at 0x7123c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71240 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71254 | load | 4 | `abs 0x71530` | `-` |
| `ldr` at 0x71258 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x71268 | load | 4 | `abs 0x71534` | `-` |
| `ldr` at 0x7126c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x71270 | load | 4 | `abs 0x71538` | `-` |
| `ldr` at 0x71274 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71278 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71284 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71288 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7128c | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x71290 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x71294 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71298 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x712a4 | store | 4 | `sp -0x28` | `ret(0x6f8a0)` |
| `ldr` at 0x712a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x712b4 | load | 4 | `abs 0x71530` | `-` |
| `ldr` at 0x712b8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x712c8 | load | 4 | `abs 0x71534` | `-` |
| `ldr` at 0x712cc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x712d0 | load | 4 | `abs 0x7153c` | `-` |
| `ldr` at 0x712d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x712d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x712e0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x712e4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x712f4 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x712f8 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x71300 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71304 | load | 4 | `abs 0x1489c8` | `-` |
| `ldr` at 0x71310 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71314 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71318 | load | 4 | `abs 0x1489c8` | `-` |
| `str` at 0x71320 | store | 4 | `abs 0x1489c8` | `[0x1489c8]+16` |
| `ldr` at 0x71324 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71328 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7132c | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71330 | load | 4 | `abs 0x1489c8` | `-` |
| `ldr` at 0x71344 | load | 4 | `abs 0x1489cc` | `-` |
| `str` at 0x71354 | store | 4 | `abs 0x1489cc` | `ret(0x1b2c8)` |
| `str` at 0x7135c | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x71360 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71364 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71368 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x71378 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71380 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71388 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7138c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x713a0 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x713a8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x713b0 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x713b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x713c8 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x713d0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x713e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x713e8 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `ldr` at 0x713f0 | load | 4 | `abs 0x71530` | `-` |
| `ldr` at 0x713f4 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x71404 | load | 4 | `abs 0x71534` | `-` |
| `ldr` at 0x71408 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x7140c | load | 4 | `abs 0x71540` | `-` |
| `ldr` at 0x71410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71414 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7141c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71428 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7142c | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x71434 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71438 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7143c | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x71454 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x71464 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71468 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x7146c | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x71478 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7147c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x71490 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x7149c | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x714a0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x714b4 | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x714bc | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x714c0 | load | 4 | `abs 0x1489c4` | `-` |
| `ldr` at 0x714c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x714e4 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x714e8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x714fc | load | 4 | `abs 0x1489cc` | `-` |
| `ldr` at 0x71508 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71514 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x71518 | load | 4 | `abs 0x7152c` | `-` |
| `ldr` at 0x7151c | load | 4 | `abs 0x1489c4` | `-` |
| `str` at 0x71524 | store | 4 | `abs 0x1489c4` | `[0x1489c4]+1` |
| `ldmdb` at 0x71528 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x71528 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x71528 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x71528 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71244 | `(unknown callee) 0x6f784` | `[0x1489c0]` | `[0x1489b8]` | `r0` | `r1` |
| `bl` at 0x7127c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f724` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x7129c | `(unknown callee) 0x6f8a0` | `[0x1489cc]` | `[0x1489c4]` | `?` | `?` |
| `bl` at 0x712dc | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f744` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x712ec | `(unknown callee) 0x1b304` | `?` | `[sp-32]` | `0x70` | `?` |
| `bl` at 0x7134c | `(unknown callee) 0x1b2c8` | `[0x1489cc]` | `?` | `[0x1489c8]` | `?` |
| `bl` at 0x71418 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f75c` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x7145c | `(unknown callee) 0x1b16c` | `[0x1489cc]+112` | `[0x1489cc]` | `?` | `?` |
| `bl` at 0x714e0 | `(unknown callee) 0x1b16c` | `?` | `?` | `?` | `?` |
| `bl` at 0x71510 | `(unknown callee) 0x1b304` | `?` | `[sp-32]` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71228 | `r3 == 0x0` | `r3 != 0x0` |
| `bne` at 0x71250 | `ret(0x6f784) != 0x0` | `ret(0x6f784) == 0x0` |
| `beq` at 0x71264 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x712b0 | `ret(0x6f8a0) == 0x0` | `ret(0x6f8a0) != 0x0` |
| `beq` at 0x712c4 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x7130c | `[0x1489c4]+1 <= [0x1489c8] (signed, 32-bit)` | `[0x1489c4]+1 > [0x1489c8] (signed, 32-bit)` |
| `blt` at 0x71370 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x713dc | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `beq` at 0x71400 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x71424 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x71474 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x71528 | `r0` | `?` |

## `L1CfgDelete`

- `L1CfgDelete`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x71544 to 0x71880.
  Established. The entry-r0 guard takes zero and falls through on nonzero (`beq` at 0x71574). The
  first call to `0x24028` carries `[0x144dc8]`, `0x10f774`, `[sp]` and `[sp+4]` as candidates (`bl`
  at 0x71618). The final r0 expression is unknown (`ldmdb` at 0x7186c). Deletion identity and record
  layouts remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x71554 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x71558 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x7155c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x71560 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x71568 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x7156c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71578 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7157c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71580 | load | 4 | `r1 +0x0` | `-` |
| `ldr` at 0x71590 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x715a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x715ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x715b0 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x715bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x715d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x715d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x715dc | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x715e8 | load | 4 | `abs 0x71870` | `-` |
| `ldr` at 0x715ec | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x715fc | load | 4 | `abs 0x71874` | `-` |
| `ldr` at 0x71600 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71604 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x71608 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x7160c | load | 4 | `abs 0x71878` | `-` |
| `ldr` at 0x71610 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71614 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7161c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71630 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71638 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7164c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x71658 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x7165c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71660 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x7168c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x71690 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldr` at 0x716a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x716c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x716c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x716c8 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0x716d0 | store | 4 | `[sp-24] +0x0` | `[[sp-24]]-1` |
| `ldr` at 0x716d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x716e0 | store | 4 | `sp -0x24` | `?` |
| `str` at 0x716ec | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x716f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x716fc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71700 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71704 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71714 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71728 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71730 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71734 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71740 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71754 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x7175c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x71760 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x7176c | load | 4 | `abs 0x71870` | `-` |
| `ldr` at 0x71770 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x71780 | load | 4 | `abs 0x71874` | `-` |
| `ldr` at 0x71784 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71788 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x7178c | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x71790 | load | 4 | `abs 0x7187c` | `-` |
| `ldr` at 0x71794 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0x71798 | load | 4 | `sp +0x4` | `-` |
| `ldr` at 0x717a0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x717b4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x717bc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x717d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x717dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x717e0 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x717e4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x71810 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71814 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0x71828 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x71844 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x71848 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x7184c | load | 4 | `[sp-32] +0x0` | `-` |
| `str` at 0x71854 | store | 4 | `[sp-32] +0x0` | `[[sp-32]]-1` |
| `ldr` at 0x7185c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x71864 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldmdb` at 0x7186c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x7186c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x7186c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x71618 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f774` | `[sp]` | `[sp+4]` |
| `bl` at 0x71688 | `(unknown callee) 0x1b16c` | `?` | `?` | `?` | `?` |
| `bl` at 0x716bc | `(unknown callee) 0x1b43c` | `?` | `0x0` | `0x18` | `?` |
| `bl` at 0x7179c | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f794` | `[sp]` | `[sp+4]` |
| `bl` at 0x7180c | `(unknown callee) 0x1b16c` | `?` | `?` | `?` | `?` |
| `bl` at 0x71840 | `(unknown callee) 0x1b43c` | `?` | `0x0` | `0x70` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x71574 | `r0 == 0x0` | `r0 != 0x0` |
| `blt` at 0x71588 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x715b8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x715e4 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x715f8 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x716f8 | `Z == 1` | `not (Z == 1)` |
| `blt` at 0x7170c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x7173c | `? != [sp]` | `? == [sp]` |
| `bne` at 0x71768 | `? != [sp+4]` | `? == [sp+4]` |
| `beq` at 0x7177c | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x7186c | `r0` | `?` |

## `L1CfgUpdate`

- `L1CfgUpdate`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6cf78 to 0x6d240.
  Established. The opening call to `0x6c0d0` has `0x10f378`, entry r1, entry r2 and `0x0` as
  candidates (`bl` at 0x6cf94). The loop decision retains unknown operands (`blt` at 0x6cff8). The
  return carries `ret(0xb6d54)` (`ldmdb` at 0x6d21c). Update publication and packet contents remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6cf8c | store | 4 | `sp -0x98` | `0x0` |
| `ldr` at 0x6cf90 | load | 4 | `abs 0x6d220` | `-` |
| `strb` at 0x6cf9c | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x6cfa4 | store | 1 | `sp -0x93` | `0x0` |
| `str` at 0x6cfac | store | 4 | `sp -0x90` | `0x1` |
| `strh` at 0x6cfb4 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x6cfb8 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6cfc0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6cfd4 | load | 4 | `abs 0x6d224` | `-` |
| `str` at 0x6cfe4 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x6cfe8 | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6cfec | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6cff0 | load | 4 | `abs 0x1489b0` | `-` |
| `ldr` at 0x6d000 | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6d004 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d020 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d028 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d034 | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6d038 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d054 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d05c | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d06c | store | 4 | `sp -0xa0` | `0x0` |
| `ldr` at 0x6d070 | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6d074 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d090 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d098 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d09c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d0ac | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6d0b0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d0cc | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d0d4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d0e8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d0f0 | load | 4 | `abs 0x6d224` | `-` |
| `ldr` at 0x6d0f4 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6d110 | load | 4 | `abs 0x1489b4` | `-` |
| `ldr` at 0x6d118 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6d12c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d134 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d138 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6d140 | store | 4 | `sp -0xa4` | `ret(0x74708)` |
| `ldr` at 0x6d144 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6d160 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6d16c | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6d174 | store | 4 | `sp -0xa0` | `[sp-160]+1` |
| `ldr` at 0x6d17c | load | 4 | `sp -0x9c` | `-` |
| `str` at 0x6d184 | store | 4 | `sp -0x9c` | `[sp-156]+1` |
| `ldr` at 0x6d18c | load | 4 | `abs 0x6d228` | `-` |
| `str` at 0x6d198 | store | 4 | `abs 0x148674` | `ret(0x1b1e4)` |
| `strb` at 0x6d1a4 | store | 1 | `sp -0x94` | `0x0` |
| `strb` at 0x6d1ac | store | 1 | `sp -0x93` | `0x0` |
| `ldr` at 0x6d1b0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6d1b8 | store | 4 | `sp -0x90` | `[sp-152]-1` |
| `strh` at 0x6d1c0 | store | 2 | `sp -0x92` | `0x0` |
| `ldr` at 0x6d1c4 | load | 4 | `abs 0x6d22c` | `-` |
| `ldr` at 0x6d1c8 | load | 4 | `abs 0x148654` | `-` |
| `ldr` at 0x6d1e4 | load | 4 | `abs 0x6d230` | `-` |
| `ldr` at 0x6d1e8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0x6d1ec | load | 4 | `abs 0x6d234` | `-` |
| `ldr` at 0x6d1f0 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6d204 | store | 4 | `sp -0xa8` | `0x0` |
| `ldr` at 0x6d208 | load | 4 | `abs 0x6d238` | `-` |
| `ldr` at 0x6d20c | load | 4 | `abs 0x6d23c` | `-` |
| `ldr` at 0x6d210 | load | 4 | `sp -0x98` | `-` |
| `ldmdb` at 0x6d21c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6d21c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6d21c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6d21c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6cf94 | `(unknown callee) 0x6c0d0` | `0x10f378` | `r1` | `r2` | `0x0` |
| `bl` at 0x6cfdc | `(unknown callee) 0x71a6c` | `sp-152` | `sp-148` | `0x14892c` | `0x1` |
| `bl` at 0x6d13c | `(unknown callee) 0x74708` | `?` | `?` | `?` | `?` |
| `bl` at 0x6d168 | `(unknown callee) 0x71a6c` | `sp-152` | `sp-148` | `ret(0x74708)` | `0x1` |
| `bl` at 0x6d194 | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `[0x1489b0]` |
| `bl` at 0x6d19c | `(unknown callee) 0x6c168` | `ret(0x1b1e4)` | `?` | `?` | `?` |
| `bl` at 0x6d1dc | `(unknown callee) 0x1b1e4` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x6d1f8 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x10f384` | `[sp-152]` | `ret(0x1b1e4)` |
| `bl` at 0x6d218 | `(unknown callee) 0xb6d54` | `0x148924` | `0x3e9` | `[sp-152]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6cff8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x6d030 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6d064 | `? == 0x0` | `? != 0x0` |
| `blt` at 0x6d0a4 | `[sp-160] < ? (signed, 32-bit)` | `[sp-160] >= ? (signed, 32-bit)` |
| `beq` at 0x6d14c | `ret(0x74708) == 0x0` | `ret(0x74708) != 0x0` |
| `beq` at 0x6d1d4 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6d21c | `r0` | `ret(0xb6d54)` |

## `SysCom_2m_Init`

- `SysCom_2m_Init`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xaf5ac to 0xaf708.
  Established. The first call targets `0x4f090` with entry r0 through r3 as candidates (`bl` at
  0xaf5bc). The first selector branch takes `[sp-20] == 0x43` and falls through on inequality (`beq`
  at 0xaf5d4). The recorded exit carries `ret(0x4f090)` (`ldmdb` at 0xaf6e4). The selected
  system-communication setup remains unknown beyond the listed stores.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xaf5c8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xaf5cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf5d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf5e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf5f0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf5fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf608 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf614 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf620 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf62c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf638 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf644 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf650 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf65c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf668 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf674 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf680 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf68c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf698 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf6a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaf6b4 | load | 4 | `abs 0xaf6e8` | `-` |
| `ldr` at 0xaf6b8 | load | 4 | `abs 0xaf6ec` | `-` |
| `str` at 0xaf6bc | store | 4 | `abs 0x149718` | `0xaf134` |
| `ldr` at 0xaf6c0 | load | 4 | `abs 0xaf6f0` | `-` |
| `ldr` at 0xaf6c4 | load | 4 | `abs 0xaf6f4` | `-` |
| `str` at 0xaf6c8 | store | 4 | `abs 0x14971c` | `0xaf268` |
| `ldr` at 0xaf6cc | load | 4 | `abs 0xaf6f8` | `-` |
| `ldr` at 0xaf6d0 | load | 4 | `abs 0xaf6fc` | `-` |
| `str` at 0xaf6d4 | store | 4 | `abs 0x149720` | `0xaf2c4` |
| `ldr` at 0xaf6d8 | load | 4 | `abs 0xaf700` | `-` |
| `ldr` at 0xaf6dc | load | 4 | `abs 0xaf704` | `-` |
| `str` at 0xaf6e0 | store | 4 | `abs 0x149724` | `0xaf518` |
| `ldmdb` at 0xaf6e4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xaf6e4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xaf6e4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xaf5bc | `(unknown callee) 0x4f090` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaf5d4 | `[sp-20] == 0x43` | `[sp-20] != 0x43` |
| `beq` at 0xaf5e0 | `[sp-20] == 0x33` | `[sp-20] != 0x33` |
| `beq` at 0xaf5ec | `[sp-20] == 0x35` | `[sp-20] != 0x35` |
| `beq` at 0xaf5f8 | `[sp-20] == 0x34` | `[sp-20] != 0x34` |
| `beq` at 0xaf604 | `[sp-20] == 0x41` | `[sp-20] != 0x41` |
| `beq` at 0xaf610 | `[sp-20] == 0x36` | `[sp-20] != 0x36` |
| `beq` at 0xaf61c | `[sp-20] == 0x39` | `[sp-20] != 0x39` |
| `beq` at 0xaf628 | `[sp-20] == 0x37` | `[sp-20] != 0x37` |
| `beq` at 0xaf634 | `[sp-20] == 0x38` | `[sp-20] != 0x38` |
| `beq` at 0xaf640 | `[sp-20] == 0x30` | `[sp-20] != 0x30` |
| `beq` at 0xaf64c | `[sp-20] == 0x51` | `[sp-20] != 0x51` |
| `beq` at 0xaf658 | `[sp-20] == 0x40` | `[sp-20] != 0x40` |
| `beq` at 0xaf664 | `[sp-20] == 0x4f` | `[sp-20] != 0x4f` |
| `beq` at 0xaf670 | `[sp-20] == 0x42` | `[sp-20] != 0x42` |
| `beq` at 0xaf67c | `[sp-20] == 0x46` | `[sp-20] != 0x46` |
| `beq` at 0xaf688 | `[sp-20] == 0x48` | `[sp-20] != 0x48` |
| `beq` at 0xaf694 | `[sp-20] == 0x4c` | `[sp-20] != 0x4c` |
| `beq` at 0xaf6a0 | `[sp-20] == 0x4d` | `[sp-20] != 0x4d` |
| `beq` at 0xaf6ac | `[sp-20] == 0x7c` | `[sp-20] != 0x7c` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xaf6e4 | `r0` | `ret(0x4f090)` |
