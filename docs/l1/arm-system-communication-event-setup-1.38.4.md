# ARM system communication and event setup in 1.38.4

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
SysComInit has an unresolved computed selector. Accesses and return values cover only known control-flow edges. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/system-communication-event-setup-facts.txt) retain all tool sections.

## `SysComSlab`

- `SysComSlab`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83734 to 0x8376c.
  Established. A word read accesses absolute `0x2b1728` (`ldr` at 0x8374c). The call to `0x87568`
  has `[[0x2b1728]+16]`, entry r0, entry r2 and `[0x2b1728]` as candidates (`bl` at 0x83758). The
  return carries `ret(0x87568)` (`ldmdb` at 0x83764). The loaded values' meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83744 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x83748 | load | 4 | `abs 0x83768` | `-` |
| `ldr` at 0x8374c | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0x83750 | load | 4 | `[0x2b1728] +0x10` | `-` |
| `ldr` at 0x83754 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x83764 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83764 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83764 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83758 | `(unknown callee) 0x87568` | `[[0x2b1728]+16]` | `r0` | `r2` | `[0x2b1728]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83764 | `r0` | `ret(0x87568)` |

## `SysComSubchan`

- `SysComSubchan`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8376c to 0x837a4.
  Established. The global word load reads `0x2b1728` (`ldr` at 0x83784). The call to `0x8760c`
  receives `[[0x2b1728]+16]`, entry r0, entry r2 and `[0x2b1728]` as candidates (`bl` at 0x83790).
  The recorded exit is `ret(0x8760c)` (`ldmdb` at 0x8379c). Subchannel interpretation remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8377c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x83780 | load | 4 | `abs 0x837a0` | `-` |
| `ldr` at 0x83784 | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0x83788 | load | 4 | `[0x2b1728] +0x10` | `-` |
| `ldr` at 0x8378c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8379c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8379c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8379c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83790 | `(unknown callee) 0x8760c` | `[[0x2b1728]+16]` | `r0` | `r2` | `[0x2b1728]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8379c | `r0` | `ret(0x8760c)` |

## `SysComTasknum`

- `SysComTasknum`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x837a4 to 0x837e4.
  Established. The word read accesses absolute `0x2b1728` (`ldr` at 0x837c0). The call to `0x87710`
  has `[[0x2b1728]+16]`, entry r0, entry r1 and `[0x2b1728]` as candidates (`bl` at 0x837d0); its result is
  retained at the exit (`ldmdb` at 0x837dc). Task-number encoding remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x837b4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x837b8 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x837bc | load | 4 | `abs 0x837e0` | `-` |
| `ldr` at 0x837c0 | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0x837c4 | load | 4 | `[0x2b1728] +0x10` | `-` |
| `ldr` at 0x837c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x837cc | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x837dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x837dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x837dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x837d0 | `(unknown callee) 0x87710` | `[[0x2b1728]+16]` | `r0` | `r1` | `[0x2b1728]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x837dc | `r0` | `ret(0x87710)` |

## `SysComEnable`

- `SysComEnable`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x838e8 to 0x83984.
  Established. A word load reads absolute `0x2b1730` (`ldr` at 0x83908). The loop comparison has
  unknown operands (`blt` at 0x83910). The call to `0x2070c` has `0x111cc4`, unknown, unknown and
  `[0x2b1730]` as candidates (`bl` at 0x83920). The enable mechanism and merged return remain
  unknown (`ldmdb` at 0x83978).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x838fc | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x83900 | load | 4 | `abs 0x8397c` | `-` |
| `ldr` at 0x83904 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83908 | load | 4 | `abs 0x2b1730` | `-` |
| `ldr` at 0x83918 | load | 4 | `abs 0x83980` | `-` |
| `ldr` at 0x8391c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x83924 | load | 4 | `abs 0x8397c` | `-` |
| `ldr` at 0x83928 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8394c | load | 4 | `abs 0x2b1734` | `-` |
| `ldrb` at 0x83954 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x83968 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x83970 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x83978 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83978 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83978 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83920 | `(unknown callee) 0x2070c` | `0x111cc4` | `?` | `?` | `[0x2b1730]` |
| `bl` at 0x83964 | `(unknown callee) 0xd6d40` | `0x0` | `?` | `0x2` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x83910 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83978 | `r0` | `?` |

## `SysComThrottle`

- `SysComThrottle`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x82878 to 0x82a78.
  Established. The global word read accesses `0x14972c` (`ldr` at 0x82890). Nonzero takes the branch
  and zero falls through (`bne` at 0x82898). The call to `0x23af4` receives entry r0, entry r1,
  unknown and unknown (`bl` at 0x828cc). Throttling thresholds and final r0 value remain unknown
  (`ldmdb` at 0x82a5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x82888 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8288c | load | 4 | `abs 0x82a60` | `-` |
| `ldr` at 0x82890 | load | 4 | `abs 0x14972c` | `-` |
| `ldr` at 0x828a0 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x828a4 | load | 4 | `abs 0x2b1790` | `-` |
| `ldr` at 0x828b0 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x828b4 | load | 4 | `abs 0x2b1790` | `-` |
| `ldr` at 0x828b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x828c8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x828d4 | store | 4 | `sp -0x1c` | `ret(0x23af4)` |
| `ldr` at 0x828d8 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x828dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x828e0 | load | 4 | `abs 0x2b179c` | `-` |
| `ldr` at 0x828ec | load | 4 | `abs 0x82a68` | `-` |
| `str` at 0x828fc | store | 4 | `sp -0x18` | `ret(0x22a38)` |
| `ldr` at 0x82900 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8290c | load | 4 | `abs 0x82a6c` | `-` |
| `str` at 0x82910 | store | 4 | `sp -0x24` | `0x111804` |
| `ldr` at 0x82914 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82918 | store | 4 | `sp -0x20` | `ret(0x22a38)` |
| `ldr` at 0x82920 | load | 4 | `abs 0x82a70` | `-` |
| `ldr` at 0x82930 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x82934 | load | 4 | `abs 0x2b1790` | `-` |
| `ldr` at 0x82940 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x82944 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x82948 | store | 4 | `abs 0x2b1790` | `[sp-20]` |
| `ldr` at 0x8294c | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x82950 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x82954 | load | 4 | `abs 0x2b1794` | `-` |
| `str` at 0x8295c | store | 4 | `abs 0x2b1794` | `[0x2b1794]+1` |
| `ldr` at 0x82960 | load | 4 | `abs 0x82a68` | `-` |
| `str` at 0x8296c | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0x82970 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8297c | load | 4 | `abs 0x82a74` | `-` |
| `str` at 0x82980 | store | 4 | `sp -0x24` | `0x111848` |
| `ldr` at 0x82984 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82988 | store | 4 | `sp -0x20` | `ret(0x22dc0)` |
| `ldr` at 0x82990 | load | 4 | `abs 0x82a70` | `-` |
| `ldr` at 0x829a4 | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x829a8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x829ac | load | 4 | `abs 0x2b17a0` | `-` |
| `ldr` at 0x829b8 | load | 4 | `abs 0x82a68` | `-` |
| `str` at 0x829c8 | store | 4 | `sp -0x18` | `ret(0x22a38)` |
| `ldr` at 0x829cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x829d8 | load | 4 | `abs 0x82a6c` | `-` |
| `str` at 0x829dc | store | 4 | `sp -0x24` | `0x111804` |
| `ldr` at 0x829e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x829e4 | store | 4 | `sp -0x20` | `ret(0x22a38)` |
| `ldr` at 0x829ec | load | 4 | `abs 0x82a70` | `-` |
| `ldr` at 0x829fc | load | 4 | `abs 0x82a64` | `-` |
| `ldr` at 0x82a00 | load | 4 | `abs 0x2b1790` | `-` |
| `ldr` at 0x82a04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x82a10 | load | 4 | `abs 0x82a64` | `-` |
| `str` at 0x82a18 | store | 4 | `abs 0x2b1790` | `0x0` |
| `ldr` at 0x82a1c | load | 4 | `abs 0x82a68` | `-` |
| `str` at 0x82a28 | store | 4 | `sp -0x18` | `ret(0x22dc0)` |
| `ldr` at 0x82a2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x82a38 | load | 4 | `abs 0x82a74` | `-` |
| `str` at 0x82a3c | store | 4 | `sp -0x24` | `0x111848` |
| `ldr` at 0x82a40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x82a44 | store | 4 | `sp -0x20` | `ret(0x22dc0)` |
| `ldr` at 0x82a4c | load | 4 | `abs 0x82a70` | `-` |
| `ldmdb` at 0x82a5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x82a5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x82a5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x828cc | `(unknown callee) 0x23af4` | `r0` | `r1` | `?` | `?` |
| `bl` at 0x828f4 | `(unknown callee) 0x22a38` | `0x2b173c` | `?` | `ret(0x23af4)` | `[0x2b179c]` |
| `bl` at 0x8292c | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0xa8` | `0x0` |
| `bl` at 0x82964 | `(unknown callee) 0x22dc0` | `0x2b173c` | `?` | `?` | `?` |
| `bl` at 0x8299c | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0xb0` | `0x0` |
| `bl` at 0x829c0 | `(unknown callee) 0x22a38` | `0x2b173c` | `?` | `ret(0x23af4)` | `[0x2b17a0]` |
| `bl` at 0x829f8 | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0xb5` | `0x0` |
| `bl` at 0x82a20 | `(unknown callee) 0x22dc0` | `0x2b173c` | `?` | `?` | `?` |
| `bl` at 0x82a58 | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0xbb` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x82898 | `[0x14972c] != 0x0` | `[0x14972c] == 0x0` |
| `beq` at 0x828ac | `[0x2b1790] == 0x0` | `[0x2b1790] != 0x0` |
| `beq` at 0x828c0 | `[0x2b1790] == r0` | `[0x2b1790] != r0` |
| `blt` at 0x828e8 | `ret(0x23af4) < [0x2b179c] (signed, 32-bit)` | `ret(0x23af4) >= [0x2b179c] (signed, 32-bit)` |
| `beq` at 0x82908 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x8293c | `[0x2b1790] != 0x0` | `[0x2b1790] == 0x0` |
| `beq` at 0x82978 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |
| `bge` at 0x829b4 | `ret(0x23af4) >= [0x2b17a0] (signed, 32-bit)` | `ret(0x23af4) < [0x2b17a0] (signed, 32-bit)` |
| `beq` at 0x829d4 | `ret(0x22a38) == 0x0` | `ret(0x22a38) != 0x0` |
| `bne` at 0x82a0c | `[0x2b1790] != [sp-20]` | `[0x2b1790] == [sp-20]` |
| `beq` at 0x82a34 | `ret(0x22dc0) == 0x0` | `ret(0x22dc0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x82a5c | `r0` | `?` |

## `SysComInit`

- `SysComInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x832e8 to 0x83734.
  Established. A word load reads absolute `0x1496e8` (`ldr` at 0x832fc). Zero takes the branch and
  nonzero falls through (`beq` at 0x83304). The first call to `0x21e28` has `0x9`, `0x111830`,
  `0x1ca` and `0x0` as candidates (`bl` at 0x83320). The later selector is unresolved (`ldrls` at
  0x833cc). Initialisation effects on selected edges remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x832f8 | load | 4 | `abs 0x836c4` | `-` |
| `ldr` at 0x832fc | load | 4 | `abs 0x1496e8` | `-` |
| `ldr` at 0x83308 | load | 4 | `abs 0x836c8` | `-` |
| `str` at 0x8330c | store | 4 | `sp -0x34` | `0x111c2c` |
| `ldr` at 0x83314 | load | 4 | `abs 0x836cc` | `-` |
| `ldr` at 0x83318 | load | 4 | `abs 0x836d0` | `-` |
| `ldr` at 0x83324 | load | 4 | `abs 0x836c4` | `-` |
| `ldr` at 0x83328 | load | 4 | `abs 0x836c4` | `-` |
| `ldr` at 0x8332c | load | 4 | `abs 0x1496e8` | `-` |
| `str` at 0x83334 | store | 4 | `abs 0x1496e8` | `[0x1496e8]+1` |
| `ldr` at 0x83338 | load | 4 | `abs 0x836d4` | `-` |
| `ldr` at 0x83348 | load | 4 | `abs 0x836d8` | `-` |
| `ldr` at 0x8334c | load | 4 | `abs 0x836dc` | `-` |
| `ldr` at 0x8335c | load | 4 | `abs 0x836d4` | `-` |
| `str` at 0x83364 | store | 4 | `abs 0x2b1738` | `0xc8` |
| `ldr` at 0x83368 | load | 4 | `abs 0x836d4` | `-` |
| `str` at 0x83370 | store | 4 | `abs 0x2b1798` | `0x3` |
| `ldr` at 0x83374 | load | 4 | `abs 0x836d4` | `-` |
| `str` at 0x8337c | store | 4 | `abs 0x2b179c` | `0x50` |
| `ldr` at 0x83380 | load | 4 | `abs 0x836d4` | `-` |
| `str` at 0x83388 | store | 4 | `abs 0x2b17a0` | `0x5` |
| `ldr` at 0x8338c | load | 4 | `abs 0x836e0` | `-` |
| `str` at 0x83394 | store | 4 | `abs 0x14972c` | `0x1` |
| `ldr` at 0x83398 | load | 4 | `abs 0x836d4` | `-` |
| `ldr` at 0x833a8 | load | 4 | `abs 0x836d4` | `-` |
| `str` at 0x833b4 | store | 4 | `abs 0x2b1728` | `ret(0x74908)` |
| `ldr` at 0x833b8 | load | 4 | `abs 0x836d4` | `-` |
| `ldr` at 0x833bc | load | 4 | `abs 0x2b1728` | `-` |
| `ldr` at 0x833c0 | load | 4 | `[0x2b1728] +0x10` | `-` |
| `ldrls` at 0x833cc | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x836c0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x836c0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x836c0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x836c0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x83320 | `(unknown callee) 0x21e28` | `0x9` | `0x111830` | `0x1ca` | `0x0` |
| `bl` at 0x83344 | `(unknown callee) 0x1b43c` | `0x2b1720` | `0x0` | `0x84` | `[0x1496e8]+1` |
| `bl` at 0x83358 | `(unknown callee) 0x22920` | `0x2b173c` | `0x111c50` | `0x1` | `0x9` |
| `bl` at 0x833a4 | `(unknown callee) 0xb553c` | `0x2b1720` | `0x5` | `0x0` | `0x1` |
| `bl` at 0x833b0 | `(unknown callee) 0x74908` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x83304 | `[0x1496e8] == 0x0` | `[0x1496e8] != 0x0` |
| `ldrls` at 0x833cc | `[[0x2b1728]+16]-43 <= 0x51 (unsigned, 32-bit)` | `[[0x2b1728]+16]-43 > 0x51 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x836c0 | `r0` | `ret(0x74908)` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x833cc).

## `syscom_process_inbound_data`

- `syscom_process_inbound_data`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x83984 to 0x83ac4.
  Established. The opening call to `0x6907c` uses `sp-80`, `sp-76`, `0x0` and `sp-80` as candidates
  (`bl` at 0x839b4). The branch takes `ret(0x8260c) == 0x0` and falls through on nonzero (`beq` at
  0x839ec). The final r0 value is unknown (`ldmdb` at 0x83ab4). Inbound-data routing remains
  unproved beyond the recorded calls.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x83994 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0x83998 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0x8399c | store | 4 | `sp -0x24` | `r2` |
| `str` at 0x839c4 | store | 4 | `sp -0x58` | `0x1` |
| `ldr` at 0x839cc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x839d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x839d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x839f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x839f4 | load | 4 | `abs 0x83ab8` | `-` |
| `ldr` at 0x839f8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x839fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x83a10 | load | 4 | `abs 0x83abc` | `-` |
| `ldr` at 0x83a14 | load | 4 | `abs 0x2b1720` | `-` |
| `str` at 0x83a18 | store | 4 | `(unknown)` | `[0x2b1720]` |
| `ldr` at 0x83a34 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x83a3c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x83a58 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0x83a64 | load | 4 | `abs 0x83abc` | `-` |
| `ldr` at 0x83a68 | load | 4 | `abs 0x2b1738` | `-` |
| `str` at 0x83a6c | store | 4 | `sp -0x58` | `[0x2b1738]` |
| `ldr` at 0x83a78 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x83a88 | store | 4 | `sp -0x54` | `ret(0xb6d54)` |
| `ldr` at 0x83a8c | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x83a98 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x83aa4 | load | 4 | `abs 0x83ac0` | `-` |
| `ldr` at 0x83aa8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x83ab4 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x83ab4 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x83ab4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x83ab4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x83ab4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x839b4 | `(unknown callee) 0x6907c` | `sp-80` | `sp-76` | `0x0` | `sp-80` |
| `bl` at 0x839d4 | `(unknown callee) 0x692f8` | `sp-80` | `sp-76` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x839e0 | `(unknown callee) 0x8260c` | `[sp-28]` | `0x1` | `?` | `?` |
| `bl` at 0x83a00 | `(unknown callee) 0x826a4` | `[sp-28]` | `0x111cd4` | `[sp-32]` | `[sp-36]` |
| `bl` at 0x83a44 | `(unknown callee) 0x837a4` | `[sp-28]` | `0x4` | `sp-24` | `?` |
| `bl` at 0x83a80 | `(unknown callee) 0xb6d54` | `sp-44` | `0x0` | `[sp-80]` | `sp-76` |
| `bl` at 0x83a9c | `(unknown callee) 0x21674` | `ret(0xb6d54)` | `?` | `?` | `ret(0xb6d54)` |
| `bl` at 0x83ab0 | `(unknown callee) 0x2070c` | `0x111ce4` | `[sp-28]` | `ret(0x21674)` | `ret(0x21674)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x839ec | `ret(0x8260c) == 0x0` | `ret(0x8260c) != 0x0` |
| `beq` at 0x83a94 | `ret(0xb6d54) == 0x0` | `ret(0xb6d54) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x83ab4 | `r0` | `?` |

## `L1ReqInit`

- `L1ReqInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x881b0 to 0x882a0.
  Established. The word read accesses absolute `0x1497cc` (`ldr` at 0x881c4). Zero takes the first
  branch and nonzero falls through (`beq` at 0x881cc). The call to `0x21e28` carries `0x9`,
  `0x1120a0`, `0x1c6` and `0x0` as candidates (`bl` at 0x881e8). Request initialisation ownership
  remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x881c0 | load | 4 | `abs 0x88274` | `-` |
| `ldr` at 0x881c4 | load | 4 | `abs 0x1497cc` | `-` |
| `ldr` at 0x881d0 | load | 4 | `abs 0x88278` | `-` |
| `str` at 0x881d4 | store | 4 | `sp -0x1c` | `0x112084` |
| `ldr` at 0x881dc | load | 4 | `abs 0x8827c` | `-` |
| `ldr` at 0x881e0 | load | 4 | `abs 0x88280` | `-` |
| `ldr` at 0x881ec | load | 4 | `abs 0x88284` | `-` |
| `ldr` at 0x881f0 | load | 4 | `abs 0x88288` | `-` |
| `ldr` at 0x88200 | load | 4 | `abs 0x8828c` | `-` |
| `ldr` at 0x88210 | load | 4 | `abs 0x8828c` | `-` |
| `ldr` at 0x88214 | load | 4 | `abs 0x88290` | `-` |
| `ldr` at 0x8821c | load | 4 | `abs 0x88294` | `-` |
| `str` at 0x88228 | store | 4 | `sp -0x14` | `ret(0xb4178)` |
| `ldr` at 0x8822c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x88238 | load | 4 | `abs 0x88298` | `-` |
| `str` at 0x8823c | store | 4 | `sp -0x1c` | `0x1120c4` |
| `ldr` at 0x88240 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x88244 | store | 4 | `sp -0x18` | `ret(0xb4178)` |
| `ldr` at 0x8824c | load | 4 | `abs 0x8827c` | `-` |
| `ldr` at 0x88250 | load | 4 | `abs 0x8829c` | `-` |
| `ldr` at 0x8825c | load | 4 | `abs 0x88274` | `-` |
| `ldr` at 0x88260 | load | 4 | `abs 0x88274` | `-` |
| `ldr` at 0x88264 | load | 4 | `abs 0x1497cc` | `-` |
| `str` at 0x8826c | store | 4 | `abs 0x1497cc` | `[0x1497cc]+1` |
| `ldmdb` at 0x88270 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x88270 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x88270 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x881e8 | `(unknown callee) 0x21e28` | `0x9` | `0x1120a0` | `0x1c6` | `0x0` |
| `bl` at 0x881fc | `(unknown callee) 0x22920` | `0x2b17c0` | `0x1120b8` | `0x1` | `0x9` |
| `bl` at 0x8820c | `(unknown callee) 0xb553c` | `0x2b1814` | `0x6` | `0x0` | `?` |
| `bl` at 0x88220 | `(unknown callee) 0xb4178` | `0x2b1814` | `0xffff006` | `0x1` | `0x882a0` |
| `bl` at 0x88258 | `(unknown callee) 0x21e28` | `0x9` | `0x1120a0` | `0x1da` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x881cc | `[0x1497cc] == 0x0` | `[0x1497cc] != 0x0` |
| `beq` at 0x88234 | `ret(0xb4178) == 0x0` | `ret(0xb4178) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x88270 | `r0` | `?` |

## `main_task`

- `main_task`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6538c to 0x65564.
  Established. The first call to `0x1bc64` receives entry r0, entry r1, entry r2 and `0x1e` as
  candidates (`bl` at 0x653bc). The first guard takes `[0x2ae500] == 0x0` and falls through on
  nonzero (`beq` at 0x653e0). The task's final r0 value and external startup contract remain unknown
  (`ldmdb` at 0x65540).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6539c | store | 4 | `sp -0x18` | `r0` |
| `str` at 0x653a0 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0x653a8 | store | 4 | `sp -0x20` | `0x2` |
| `str` at 0x653b0 | store | 4 | `sp -0x24` | `0x6` |
| `str` at 0x653b8 | store | 4 | `sp -0x30` | `0x1e` |
| `ldr` at 0x653c0 | load | 4 | `abs 0x65544` | `-` |
| `ldr` at 0x653c4 | load | 4 | `abs 0x65548` | `-` |
| `ldr` at 0x653d4 | load | 4 | `abs 0x6554c` | `-` |
| `ldr` at 0x653d8 | load | 4 | `abs 0x2ae500` | `-` |
| `ldr` at 0x653e4 | load | 4 | `abs 0x6554c` | `-` |
| `ldr` at 0x653e8 | load | 4 | `abs 0x2ae500` | `-` |
| `ldr` at 0x6542c | load | 4 | `abs 0x6554c` | `-` |
| `ldr` at 0x65430 | load | 4 | `abs 0x2ae504` | `-` |
| `ldr` at 0x65490 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x654a8 | load | 4 | `abs 0x65550` | `-` |
| `ldr` at 0x654ac | load | 4 | `abs 0x149994` | `-` |
| `ldr` at 0x654b8 | load | 4 | `abs 0x65550` | `-` |
| `ldr` at 0x654bc | load | 4 | `abs 0x149994` | `-` |
| `ldr` at 0x654ec | load | 4 | `abs 0x65554` | `-` |
| `ldr` at 0x654fc | load | 4 | `abs 0x65558` | `-` |
| `ldr` at 0x65514 | load | 4 | `abs 0x6555c` | `-` |
| `ldr` at 0x65534 | load | 4 | `abs 0x65560` | `-` |
| `ldmdb` at 0x65540 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x65540 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x65540 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x65540 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x653bc | `(unknown callee) 0x1bc64` | `r0` | `r1` | `r2` | `0x1e` |
| `bl` at 0x653c8 | `(unknown callee) 0x2070c` | `0x10d184` | `0x10d194` | `?` | `?` |
| `bl` at 0x653cc | `(unknown callee) 0x5e4d8` | `ret(0x2070c)` | `?` | `?` | `?` |
| `bl` at 0x653d0 | `(unknown callee) 0x4e7b0` | `ret(0x5e4d8)` | `?` | `?` | `?` |
| `mov` at 0x653f0 | `(unknown callee) (unknown target)` | `ret(0x4e7b0)` | `?` | `?` | `[0x2ae500]` |
| `bl` at 0x653f8 | `(unknown callee) 0x64a1c` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x65408 | `(unknown callee) 0xc99c4` | `0x1` | `0x2` | `0x9600` | `?` |
| `bl` at 0x65410 | `(unknown callee) 0x23b48` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6541c | `(unknown callee) 0x21dac` | `0x0` | `0x0` | `?` | `?` |
| `bl` at 0x65424 | `(unknown callee) 0xcae70` | `0x0` | `?` | `?` | `?` |
| `bl` at 0x65428 | `(unknown callee) 0xe312c` | `ret(0xcae70)` | `?` | `?` | `?` |
| `mov` at 0x65438 | `(unknown callee) (unknown target)` | `ret(0xe312c)` | `?` | `?` | `[0x2ae504]` |
| `bl` at 0x6543c | `(unknown callee) 0x50d2c` | `ret(unknown)` | `?` | `?` | `?` |
| `bl` at 0x65440 | `(unknown callee) 0x20da4` | `ret(0x50d2c)` | `?` | `?` | `?` |
| `bl` at 0x65448 | `(unknown callee) 0x1f1dc` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x65460 | `(unknown callee) 0x1f1dc` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x65474 | `(unknown callee) 0xb546c` | `?` | `?` | `?` | `?` |
| `bl` at 0x65478 | `(unknown callee) 0x881b0` | `ret(0xb546c)` | `?` | `?` | `?` |
| `bl` at 0x6547c | `(unknown callee) 0x98c1c` | `ret(0x881b0)` | `?` | `?` | `?` |
| `bl` at 0x65480 | `(unknown callee) 0xa3040` | `ret(0x98c1c)` | `?` | `?` | `?` |
| `bl` at 0x65484 | `(unknown callee) 0xb10a0` | `ret(0xa3040)` | `?` | `?` | `?` |
| `bl` at 0x65488 | `(unknown callee) 0x68db0` | `ret(0xb10a0)` | `?` | `?` | `?` |
| `bl` at 0x6548c | `(unknown callee) 0x6a8f4` | `ret(0x68db0)` | `?` | `?` | `?` |
| `bl` at 0x65494 | `(unknown callee) 0x66314` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x65498 | `(unknown callee) 0x7a91c` | `ret(0x66314)` | `?` | `?` | `?` |
| `bl` at 0x6549c | `(unknown callee) 0xe8980` | `ret(0x7a91c)` | `?` | `?` | `?` |
| `bl` at 0x654a0 | `(unknown callee) 0xa5048` | `ret(0xe8980)` | `?` | `?` | `?` |
| `bl` at 0x654a4 | `(unknown callee) 0xc87f4` | `ret(0xa5048)` | `?` | `?` | `?` |
| `mov` at 0x654c4 | `(unknown callee) (unknown target)` | `ret(0xc87f4)` | `?` | `?` | `[0x149994]` |
| `bl` at 0x654c8 | `(unknown callee) 0x65564` | `?` | `?` | `?` | `?` |
| `bl` at 0x654cc | `(unknown callee) 0x76d04` | `ret(0x65564)` | `?` | `?` | `?` |
| `bl` at 0x654d0 | `(unknown callee) 0x832e8` | `ret(0x76d04)` | `?` | `?` | `?` |
| `bl` at 0x654d4 | `(unknown callee) 0x4f090` | `ret(0x832e8)` | `?` | `?` | `?` |
| `bl` at 0x654f4 | `(unknown callee) 0x51580` | `0x31` | `0x10d19c` | `0x2` | `?` |
| `bl` at 0x65504 | `(unknown callee) 0x51580` | `0x32` | `0x10d1ac` | `0x2` | `?` |
| `bl` at 0x6550c | `(unknown callee) 0x69af8` | `ret(0x4f090)` | `?` | `?` | `?` |
| `bl` at 0x6551c | `(unknown callee) 0x51580` | `0x32` | `0x10d1bc` | `0x0` | `?` |
| `bl` at 0x65520 | `(unknown callee) 0x20c60` | `ret(0x51580)` | `?` | `?` | `?` |
| `bl` at 0x6553c | `(unknown callee) 0x51580` | `0x32` | `0x10d1c8` | `0x1` | `ret(0x20c60)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x653e0 | `[0x2ae500] == 0x0` | `[0x2ae500] != 0x0` |
| `beq` at 0x654b4 | `[0x149994] == 0x0` | `[0x149994] != 0x0` |
| `bne` at 0x654e4 | `? != 0x3f` | `? == 0x3f` |
| `beq` at 0x6552c | `ret(0x20c60) == 0x0` | `ret(0x20c60) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x65540 | `r0` | `?` |

## `L1NetInit`

- `L1NetInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x65564 to 0x656f8.
  Established. A word read uses absolute `0x148500` (`ldr` at 0x65578). Zero takes the branch and
  nonzero falls through (`beq` at 0x65580). The opening call to `0x21e28` has `0x9`, `0x10d1f0`,
  `0x43` and `0x0` as candidates (`bl` at 0x6559c). The network initialisation contract remains
  unknown beyond these calls and accesses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x65574 | load | 4 | `abs 0x656b4` | `-` |
| `ldr` at 0x65578 | load | 4 | `abs 0x148500` | `-` |
| `ldr` at 0x65584 | load | 4 | `abs 0x656b8` | `-` |
| `str` at 0x65588 | store | 4 | `sp -0x30` | `0x10d1d4` |
| `ldr` at 0x65590 | load | 4 | `abs 0x656bc` | `-` |
| `ldr` at 0x655a0 | load | 4 | `abs 0x656b4` | `-` |
| `ldr` at 0x655a4 | load | 4 | `abs 0x656b4` | `-` |
| `ldr` at 0x655a8 | load | 4 | `abs 0x148500` | `-` |
| `str` at 0x655b0 | store | 4 | `abs 0x148500` | `[0x148500]+1` |
| `ldr` at 0x655b4 | load | 4 | `abs 0x656c0` | `-` |
| `ldr` at 0x655c4 | load | 4 | `abs 0x656c0` | `-` |
| `ldr` at 0x655d4 | load | 4 | `abs 0x656c4` | `-` |
| `ldr` at 0x655d8 | load | 4 | `abs 0x656c8` | `-` |
| `str` at 0x655e8 | store | 4 | `sp -0x14` | `ret(0xb5654)` |
| `ldr` at 0x655ec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x655f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x65604 | load | 4 | `abs 0x656cc` | `-` |
| `str` at 0x65608 | store | 4 | `sp -0x30` | `0x10d204` |
| `str` at 0x6560c | store | 4 | `sp -0x2c` | `ret(0x21674)` |
| `ldr` at 0x65614 | load | 4 | `abs 0x656bc` | `-` |
| `ldr` at 0x65624 | load | 4 | `abs 0x656d0` | `-` |
| `ldr` at 0x65628 | load | 4 | `abs 0x656d4` | `-` |
| `str` at 0x6563c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x65640 | load | 4 | `abs 0x656d8` | `-` |
| `str` at 0x65644 | store | 4 | `sp -0x2c` | `0x5dc` |
| `str` at 0x6564c | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x65654 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x6565c | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x65660 | load | 4 | `abs 0x656dc` | `-` |
| `ldr` at 0x65664 | load | 4 | `abs 0x656e0` | `-` |
| `ldr` at 0x65668 | load | 4 | `abs 0x656e4` | `-` |
| `str` at 0x65678 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x6567c | load | 4 | `abs 0x656e8` | `-` |
| `str` at 0x65680 | store | 4 | `sp -0x2c` | `0xc18` |
| `str` at 0x65688 | store | 4 | `sp -0x28` | `0xf` |
| `str` at 0x65690 | store | 4 | `sp -0x24` | `0xa` |
| `str` at 0x65698 | store | 4 | `sp -0x20` | `0x9` |
| `ldr` at 0x6569c | load | 4 | `abs 0x656ec` | `-` |
| `ldr` at 0x656a0 | load | 4 | `abs 0x656f0` | `-` |
| `ldr` at 0x656a4 | load | 4 | `abs 0x656f4` | `-` |
| `ldmdb` at 0x656b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x656b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x656b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6559c | `(unknown callee) 0x21e28` | `0x9` | `0x10d1f0` | `0x43` | `0x0` |
| `bl` at 0x655c0 | `(unknown callee) 0x1b43c` | `0x2b0960` | `0x0` | `0x104` | `[0x148500]+1` |
| `bl` at 0x655d0 | `(unknown callee) 0xb553c` | `0x2b0960` | `0x11` | `0x0` | `?` |
| `bl` at 0x655e0 | `(unknown callee) 0xb5654` | `0x2b0970` | `0x10d1f8` | `0x64` | `?` |
| `bl` at 0x655fc | `(unknown callee) 0x21674` | `ret(0xb5654)` | `?` | `?` | `ret(0xb5654)` |
| `bl` at 0x65620 | `(unknown callee) 0x21e28` | `0x9` | `0x10d1f0` | `0x54` | `0x0` |
| `bl` at 0x65634 | `(unknown callee) 0x22920` | `0x2b0a08` | `0x10d228` | `0x1` | `0x9` |
| `bl` at 0x65670 | `(unknown callee) 0x223f0` | `0x2b0a60` | `0x10d234` | `0x65cfc` | `0x0` |
| `bl` at 0x656ac | `(unknown callee) 0x223f0` | `0x2b0a5c` | `0x10d240` | `0x65718` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x65580 | `[0x148500] == 0x0` | `[0x148500] != 0x0` |
| `beq` at 0x655f4 | `ret(0xb5654) == 0x0` | `ret(0xb5654) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x656b0 | `r0` | `ret(0x223f0)` |

## `EvtInit`

- `EvtInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68db0 to 0x68e88.
  Established. The opening call to `0xb553c` carries `0x1485fc`, `0x1`, `0x0` and entry r3 as
  candidates (`bl` at 0x68dcc). The branch takes `[0x142860] == 0x0` and falls through on nonzero
  (`beq` at 0x68ddc). The final r0 expression and event-routing setup remain unknown (`ldmdb` at
  0x68e64).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x68dc0 | load | 4 | `abs 0x68e68` | `-` |
| `ldr` at 0x68dd0 | load | 4 | `abs 0x68e6c` | `-` |
| `ldr` at 0x68dd4 | load | 4 | `abs 0x142860` | `-` |
| `ldr` at 0x68de0 | load | 4 | `abs 0x68e70` | `-` |
| `str` at 0x68de4 | store | 4 | `sp -0x18` | `0x68fb4` |
| `str` at 0x68df0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x68df4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x68dfc | load | 4 | `abs 0x68e74` | `-` |
| `ldr` at 0x68e00 | load | 4 | `abs 0x1485f8` | `-` |
| `ldr` at 0x68e0c | load | 4 | `abs 0x68e68` | `-` |
| `ldr` at 0x68e10 | load | 4 | `abs 0x68e78` | `-` |
| `ldr` at 0x68e18 | load | 4 | `abs 0x68e7c` | `-` |
| `str` at 0x68e24 | store | 4 | `sp -0x14` | `ret(0xb4178)` |
| `ldr` at 0x68e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68e34 | load | 4 | `abs 0x68e80` | `-` |
| `str` at 0x68e38 | store | 4 | `sp -0x20` | `0x10ee94` |
| `ldr` at 0x68e3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68e40 | store | 4 | `sp -0x1c` | `ret(0xb4178)` |
| `ldr` at 0x68e48 | load | 4 | `abs 0x68e84` | `-` |
| `ldr` at 0x68e58 | load | 4 | `abs 0x68e74` | `-` |
| `str` at 0x68e60 | store | 4 | `abs 0x1485f8` | `0x1` |
| `ldmdb` at 0x68e64 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68e64 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68e64 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68dcc | `(unknown callee) 0xb553c` | `0x1485fc` | `0x1` | `0x0` | `r3` |
| `bl` at 0x68df8 | `(unknown callee) 0x244b8` | `[sp-24]` | `?` | `?` | `?` |
| `bl` at 0x68e1c | `(unknown callee) 0xb4178` | `0x1485fc` | `0xffff001` | `0x1` | `0x68e88` |
| `bl` at 0x68e54 | `(unknown callee) 0x21e28` | `0x9` | `0x10eec0` | `0x43` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x68ddc | `[0x142860] == 0x0` | `[0x142860] != 0x0` |
| `bne` at 0x68e08 | `[0x1485f8] != 0x0` | `[0x1485f8] == 0x0` |
| `beq` at 0x68e30 | `ret(0xb4178) == 0x0` | `ret(0xb4178) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68e64 | `r0` | `?` |

## `EvtPrintf`

- `EvtPrintf`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x68f7c to 0x68fb4.
  Established. The first call to `0x1b22c` receives entry r0 through r3 as candidates (`bl` at
  0x68f94). The recorded return carries `ret(0x68fb4)` (`ldmdb` at 0x68fb0). The output destination
  and event format remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x68f8c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x68f90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x68f9c | store | 4 | `sp -0x18` | `ret(0x1b22c)` |
| `ldr` at 0x68fa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x68fa8 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x68fb0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x68fb0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x68fb0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x68f94 | `(unknown callee) 0x1b22c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x68fac | `(unknown callee) 0x68fb4` | `0x0` | `[sp-20]` | `ret(0x1b22c)` | `ret(0x1b22c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x68fb0 | `r0` | `ret(0x68fb4)` |

## `B2BActive`

- `B2BActive`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x79dcc to 0x79e74.
  Established. The initial branch retains an unknown operand (`beq` at 0x79e08). The final r0
  expression is unknown (`ldmdb` at 0x79e6c). The body's calculated loads do not establish a
  physical active-state contract.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x79ddc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x79de4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x79de8 | load | 4 | `abs 0x79e70` | `-` |
| `ldr` at 0x79dec | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e00 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79e0c | load | 4 | `abs 0x79e70` | `-` |
| `ldr` at 0x79e10 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e24 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79e28 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79e34 | load | 4 | `abs 0x79e70` | `-` |
| `ldr` at 0x79e38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x79e4c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x79e50 | load | 4 | `(unknown)` | `-` |
| `str` at 0x79e60 | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0x79e64 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x79e6c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x79e6c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x79e6c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x79e08 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79e30 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x79e58 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x79e6c | `r0` | `?` |

## `L1CfgRefresh`

- `L1CfgRefresh`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6b2b0 to 0x6b320.
  Established. The first call to `0x71ea8` has candidates `0x2`, entry r1, entry r2 and entry r3
  (`bl` at 0x6b2c0). The last call result reaches the recorded exit as `ret(0x71ea8)` (`ldmdb` at
  0x6b31c). The selector values and their external effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x6b31c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6b31c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6b31c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6b2c0 | `(unknown callee) 0x71ea8` | `0x2` | `r1` | `r2` | `r3` |
| `bl` at 0x6b2c8 | `(unknown callee) 0x71ea8` | `0xd` | `?` | `?` | `?` |
| `bl` at 0x6b2d0 | `(unknown callee) 0x71ea8` | `0x14` | `?` | `?` | `?` |
| `bl` at 0x6b2d8 | `(unknown callee) 0x71ea8` | `0x15` | `?` | `?` | `?` |
| `bl` at 0x6b2e0 | `(unknown callee) 0x71ea8` | `0x16` | `?` | `?` | `?` |
| `bl` at 0x6b2e8 | `(unknown callee) 0x71ea8` | `0x17` | `?` | `?` | `?` |
| `bl` at 0x6b2f0 | `(unknown callee) 0x71ea8` | `0x18` | `?` | `?` | `?` |
| `bl` at 0x6b2f8 | `(unknown callee) 0x71ea8` | `0x19` | `?` | `?` | `?` |
| `bl` at 0x6b300 | `(unknown callee) 0x71ea8` | `0x1` | `?` | `?` | `?` |
| `bl` at 0x6b308 | `(unknown callee) 0x71ea8` | `0x3` | `?` | `?` | `?` |
| `bl` at 0x6b310 | `(unknown callee) 0x71ea8` | `0xe` | `?` | `?` | `?` |
| `bl` at 0x6b318 | `(unknown callee) 0x71ea8` | `0xf` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6b31c | `r0` | `ret(0x71ea8)` |

## `L1CfgEventMarginState`

- `L1CfgEventMarginState`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x74fbc to 0x75014.
  Established. The call to `0x74908` carries `0x0`, entry r1, entry r2 and entry r3 as candidates
  (`bl` at 0x74fd4). The branch takes `[sp-20] == [ret(0x74908)+56]` and falls through on inequality
  (`beq` at 0x74ff0). The final r0 value and margin-state encoding remain unknown (`ldmdb` at
  0x75010).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x74fcc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x74fdc | store | 4 | `sp -0x18` | `ret(0x74908)` |
| `ldr` at 0x74fe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74fe4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x74fe8 | load | 4 | `ret(0x74908) +0x38` | `-` |
| `ldr` at 0x74ff4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x74ff8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x74ffc | store | 4 | `ret(0x74908) +0x38` | `[sp-20]` |
| `ldmdb` at 0x75010 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x75010 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x75010 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x74fd4 | `(unknown callee) 0x74908` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x7500c | `(unknown callee) 0x74c78` | `0x5` | `0x0` | `0x0` | `[sp-20]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x74ff0 | `[sp-20] == [ret(0x74908)+56]` | `[sp-20] != [ret(0x74908)+56]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x75010 | `r0` | `?` |
