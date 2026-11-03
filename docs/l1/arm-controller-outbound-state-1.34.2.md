# ARM outbound controller and state paths in 1.34.2

Read from `6.5.27/usr/cpu/firmware/sysco/l1.bin`, package `6.5.27:l1#3` (version 1.34.2),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x3b0c1c`.
Carrier SHA-256: `45c872a661d4a4ad20a4cb482fe2aee921ee9ae52784382be125b3d7b7836572`.
Package SHA-256: `d8561deedcf6bae5bef085e0420886534c72bf6c59c54b742d086b9998264128`.
Inner executable SHA-256: `65c32185ef6f284717fd705e751157059ffed1b028bd6195f92eac73749012d3`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1069932 bytes.
Segment SHA-256: `c92d999ce63930093f87780073854fbb74f634d99098b88b7830a55634c9de6c`. Analysed routine spans use the exact
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
Indirect calls can have unknown targets while retaining their continuation and argument candidates.
SysComInit reports one unresolved computed transfer on known edges. The other fourteen routines
report none. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/controller-outbound-state-facts.txt) retain all tool sections.

## `SysComReset`

- `SysComReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6bd04 to 0x6be80.
  Established. The loop begins with a word store of `0x0` at `sp -0x14` and later stores `[sp-20]+1`
  there (`str` at 0x6bd18, `str` at 0x6be70); its repeat branch retains only a signed less-than
  flags condition with unknown operands (`blt` at 0x6bd2c). The body includes two calls to `0x9ee24`
  around a call to `0xae02c`, each with first candidate unknown, second `0x3` and the remaining
  candidates unknown (`bl` at 0x6bde8, `bl` at 0x6be24, `bl` at 0x6be64). Their reset semantics and
  the iteration bound are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6bd18 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x6bd1c | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6bd20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6bd24 | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x6bd34 | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6bd38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6bd5c | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6bd64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6bd70 | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6bd74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6bd98 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6bda0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6bdac | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6bdb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6bdd4 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6bdec | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6bdf0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6be14 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6be1c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6be28 | load | 4 | `abs 0x6be7c` | `-` |
| `ldr` at 0x6be2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6be50 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6be68 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6be70 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldmdb` at 0x6be78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6be78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6be78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6bde8 | `(unknown callee) 0x9ee24` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x6be24 | `(unknown callee) 0xae02c` | `?` | `0x3` | `?` | `?` |
| `bl` at 0x6be64 | `(unknown callee) 0x9ee24` | `?` | `0x3` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6bd2c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x6bd6c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6bda8 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6be78 | `r0` | `?` |

## `SysComInit`

- `SysComInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c6ec to 0x6c9ec.
  Established. A word store increments absolute `0x11b05c` (`str` at 0x6c738). A call to `0x9a7e0`
  has candidates `0x281fc0`, `0x5`, `0x0`, `[0x11b05c]+1` (`bl` at 0x6c748). Two equality branches
  take their edges for `[0x281fc8] == 0x31` and `0x32`, respectively (`beq` at 0x6c770, `beq` at
  0x6c77c). The later computed transfer has taken condition `[0x281fc8]-53 <= 0x29` unsigned and
  falls through for a greater value, but its computed destinations are unresolved (`ldrls` at
  0x6c7a8). Operations decoded beyond that transfer have no proven selection mapping.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x6c6fc | load | 4 | `abs 0x6c9a0` | `-` |
| `ldr` at 0x6c700 | load | 4 | `abs 0x11b05c` | `-` |
| `ldr` at 0x6c70c | load | 4 | `abs 0x6c9a4` | `-` |
| `str` at 0x6c710 | store | 4 | `sp -0x38` | `0xe8a44` |
| `ldr` at 0x6c718 | load | 4 | `abs 0x6c9a8` | `-` |
| `ldr` at 0x6c71c | load | 4 | `abs 0x6c9ac` | `-` |
| `ldr` at 0x6c728 | load | 4 | `abs 0x6c9a0` | `-` |
| `ldr` at 0x6c72c | load | 4 | `abs 0x6c9a0` | `-` |
| `ldr` at 0x6c730 | load | 4 | `abs 0x11b05c` | `-` |
| `str` at 0x6c738 | store | 4 | `abs 0x11b05c` | `[0x11b05c]+1` |
| `ldr` at 0x6c73c | load | 4 | `abs 0x6c9b0` | `-` |
| `ldr` at 0x6c74c | load | 4 | `abs 0x6c9b0` | `-` |
| `str` at 0x6c75c | store | 4 | `abs 0x281fc8` | `?` |
| `ldr` at 0x6c760 | load | 4 | `abs 0x281fc8` | `-` |
| `str` at 0x6c764 | store | 4 | `sp -0x24` | `[0x281fc8]` |
| `ldr` at 0x6c768 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6c774 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x6c784 | load | 4 | `abs 0x6c9b0` | `-` |
| `str` at 0x6c794 | store | 4 | `abs 0x281fc8` | `?` |
| `ldr` at 0x6c798 | load | 4 | `abs 0x6c9b0` | `-` |
| `ldr` at 0x6c79c | load | 4 | `abs 0x281fc8` | `-` |
| `ldrls` at 0x6c7a8 | load | 4 | `(unknown)` | `-` |
| `ldmdb` at 0x6c99c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6c99c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6c99c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6c99c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6c724 | `(unknown callee) 0x1eed0` | `0x9` | `0xe89f8` | `0x179` | `0x0` |
| `bl` at 0x6c748 | `(unknown callee) 0x9a7e0` | `0x281fc0` | `0x5` | `0x0` | `[0x11b05c]+1` |
| `bl` at 0x6c750 | `(unknown callee) 0x2b31c` | `ret(0x9a7e0)` | `?` | `?` | `?` |
| `bl` at 0x6c788 | `(unknown callee) 0x2b368` | `ret(0x2b31c)` | `?` | `?` | `[0x281fc8]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6c708 | `[0x11b05c] == 0x0` | `[0x11b05c] != 0x0` |
| `beq` at 0x6c770 | `[0x281fc8] == 0x31` | `[0x281fc8] != 0x31` |
| `beq` at 0x6c77c | `[0x281fc8] == 0x32` | `[0x281fc8] != 0x32` |
| `ldrls` at 0x6c7a8 | `[0x281fc8]-53 <= 0x29 (unsigned, 32-bit)` | `[0x281fc8]-53 > 0x29 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6c99c | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x6c7a8).

## `syscom_process_outbound_request`

- `syscom_process_outbound_request`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6d6b8 to 0x6db34.
  Established. A result of `0x1` from `0x3a8c0` takes the edge into the longer path, while other
  results fall through (`bl` at 0x6d6cc, `beq` at 0x6d6d8). That longer path includes an
  explicit-link call with unknown target and candidates `[sp-136]`, `sp-124`, `0x1`, unknown (`mov`
  at 0x6d8c8); its result unequal to `0x1` takes the bypass edge, while equality falls through
  to a word store of `0x0` at `sp -0x80` (`bne` at 0x6d8dc, `str` at 0x6d8e4). Another call to
  `0x9ee84` has candidates unknown, unknown, `[sp-136]`, `sp-124` (`bl` at 0x6d9e4). Both final
  paths call `0x9bd94` with `0x281fc0`, `0x0`, `[sp-20]`, `[sp-120]`, and the return reports its
  result (`bl` at 0x6d764, `bl` at 0x6db08, `ldmdb` at 0x6db0c). Hardware transport and
  request-field meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6d6c8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6d6f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d6f8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6d70c | load | 4 | `abs 0x6db10` | `-` |
| `str` at 0x6d738 | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x6d750 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x6d754 | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6d75c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d760 | load | 4 | `sp -0x78` | `-` |
| `ldr` at 0x6d76c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d770 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6d77c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6d780 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6d788 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6d78c | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6d794 | store | 4 | `sp -0x8c` | `?` |
| `str` at 0x6d7a0 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x6d7a4 | load | 4 | `sp -0x8c` | `-` |
| `str` at 0x6d7a8 | store | 4 | `sp -0x84` | `[sp-140]` |
| `ldr` at 0x6d7ac | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6d7b8 | store | 4 | `sp -0x88` | `ret(0x6c9ec)` |
| `ldr` at 0x6d7bc | load | 4 | `sp -0x84` | `-` |
| `str` at 0x6d7c8 | store | 4 | `sp -0x84` | `ret(0x6ca20)` |
| `ldr` at 0x6d7cc | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6d7d0 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d7f4 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6d7fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d808 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6d80c | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6d81c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6d820 | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x6d824 | store | 1 | `sp -0x8d` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x6d82c | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x6d83c | store | 1 | `sp -0x8d` | `(ret(0x9ca84) & 0xff)` |
| `ldrb` at 0x6d840 | load | 1 | `sp -0x8d` | `-` |
| `strb` at 0x6d844 | store | 1 | `sp -0x7c` | `?` |
| `strb` at 0x6d84c | store | 1 | `sp -0x7b` | `0x0` |
| `ldr` at 0x6d850 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d86c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d870 | load | 4 | `abs 0x6db18` | `-` |
| `ldr` at 0x6d880 | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6d884 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d8a8 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6d8b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d8b8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6d8d0 | store | 4 | `sp -0x80` | `ret(unknown)` |
| `ldr` at 0x6d8d4 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6d8e4 | store | 4 | `sp -0x80` | `0x0` |
| `ldrb` at 0x6d8ec | load | 1 | `sp -0x84` | `-` |
| `strb` at 0x6d8f0 | store | 1 | `sp -0x7c` | `([sp-132]:1 & 0xff)` |
| `ldr` at 0x6d8f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6d8f8 | load | 1 | `[sp-20] +0x2c` | `-` |
| `ldr` at 0x6d908 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6d90c | load | 1 | `[sp-20] +0x30` | `-` |
| `strb` at 0x6d910 | store | 1 | `sp -0x8e` | `([[sp-20]+48]:1 & 0xff)` |
| `ldr` at 0x6d918 | load | 4 | `sp -0x14` | `-` |
| `strb` at 0x6d928 | store | 1 | `sp -0x8e` | `(ret(0x9ca84) & 0xff)` |
| `ldrb` at 0x6d92c | load | 1 | `sp -0x8e` | `-` |
| `strb` at 0x6d930 | store | 1 | `sp -0x7b` | `?` |
| `ldr` at 0x6d934 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d950 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d954 | load | 4 | `abs 0x6db1c` | `-` |
| `ldr` at 0x6d964 | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6d968 | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d98c | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6d998 | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6d99c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x6d9c0 | load | 4 | `abs 0x281fd4` | `-` |
| `str` at 0x6d9d0 | store | 4 | `sp -0x94` | `0x2` |
| `ldr` at 0x6d9d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6d9dc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x6d9ec | store | 4 | `sp -0x80` | `ret(0x9ee84)` |
| `ldr` at 0x6d9f0 | load | 4 | `sp -0x80` | `-` |
| `str` at 0x6da00 | store | 4 | `sp -0x80` | `0x0` |
| `ldr` at 0x6da14 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6da1c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6da28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6da2c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6da40 | load | 4 | `abs 0x6db20` | `-` |
| `ldr` at 0x6da48 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6da4c | load | 4 | `abs 0x6db24` | `-` |
| `ldr` at 0x6da60 | load | 4 | `abs 0x6db28` | `-` |
| `ldr` at 0x6da74 | load | 4 | `abs 0x6db2c` | `-` |
| `ldr` at 0x6da7c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6da80 | load | 4 | `abs 0x6db24` | `-` |
| `ldr` at 0x6da8c | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x6dab0 | load | 4 | `abs 0x6db30` | `-` |
| `str` at 0x6dadc | store | 4 | `sp -0x94` | `0x1` |
| `str` at 0x6daf4 | store | 4 | `sp -0x94` | `sp-116` |
| `ldr` at 0x6daf8 | load | 4 | `abs 0x6db14` | `-` |
| `ldr` at 0x6db00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6db04 | load | 4 | `sp -0x78` | `-` |
| `ldmdb` at 0x6db0c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6db0c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6db0c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6d6cc | `(unknown callee) 0x3a8c0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x6d6f0 | `(unknown callee) 0x541d4` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x6d6fc | `(unknown callee) 0x5461c` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x6d710 | `(unknown callee) 0x18c74` | `sp-100` | `0xe8bc8` | `ret(0x5461c)` | `sp-100` |
| `bl` at 0x6d71c | `(unknown callee) 0x18a1c` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x6d748 | `(unknown callee) 0x54450` | `sp-120` | `sp-116` | `sp-100` | `ret(0x18a1c)` |
| `bl` at 0x6d764 | `(unknown callee) 0x9bd94` | `0x281fc0` | `0x0` | `[sp-20]` | `[sp-120]` |
| `bl` at 0x6d7b0 | `(unknown callee) 0x6c9ec` | `[sp-140]` | `?` | `?` | `[sp-140]` |
| `bl` at 0x6d7c0 | `(unknown callee) 0x6ca20` | `[sp-132]` | `?` | `?` | `ret(0x6c9ec)` |
| `bl` at 0x6d834 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x6d858 | `(unknown callee) 0x6bc14` | `[sp-136]` | `0x8` | `?` | `0x0` |
| `bl` at 0x6d87c | `(unknown callee) 0x6bcac` | `[sp-136]` | `0xe8bec` | `sp-124` | `0x1` |
| `mov` at 0x6d8c8 | `(unknown callee) (unknown target)` | `[sp-136]` | `sp-124` | `0x1` | `?` |
| `bl` at 0x6d920 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x1` | `?` | `?` |
| `bl` at 0x6d93c | `(unknown callee) 0x6bc14` | `[sp-136]` | `0x10` | `?` | `?` |
| `bl` at 0x6d960 | `(unknown callee) 0x6bcac` | `[sp-136]` | `0xe8bfc` | `sp-124` | `0x2` |
| `bl` at 0x6d9e4 | `(unknown callee) 0x9ee84` | `?` | `?` | `[sp-136]` | `sp-124` |
| `bl` at 0x6da18 | `(unknown callee) 0x541d4` | `sp-120` | `sp-116` | `?` | `sp-120` |
| `bl` at 0x6da30 | `(unknown callee) 0x54938` | `[[sp-20]+24]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x6da44 | `(unknown callee) 0x18c74` | `sp-100` | `0xe8c10` | `ret(0x54938)` | `sp-100` |
| `bl` at 0x6da64 | `(unknown callee) 0x18ab8` | `sp-100` | `0xe8c24` | `[sp-128]` | `sp-100` |
| `bl` at 0x6da78 | `(unknown callee) 0x18ab8` | `sp-100` | `0xe8c3c` | `[sp-128]` | `sp-100` |
| `bl` at 0x6da90 | `(unknown callee) 0x1e71c` | `[sp-128]` | `?` | `[sp-128]` | `0xfffff69e` |
| `bl` at 0x6daa4 | `(unknown callee) 0x18ab8` | `sp-100` | `ret(0x1e71c)` | `ret(0x1e71c)` | `sp-100` |
| `bl` at 0x6dab4 | `(unknown callee) 0x18ab8` | `sp-100` | `0xe8c48` | `?` | `sp-100` |
| `bl` at 0x6dac0 | `(unknown callee) 0x18a1c` | `sp-100` | `?` | `?` | `sp-100` |
| `bl` at 0x6daec | `(unknown callee) 0x54450` | `sp-120` | `sp-116` | `sp-100` | `ret(0x18a1c)` |
| `bl` at 0x6db08 | `(unknown callee) 0x9bd94` | `0x281fc0` | `0x0` | `[sp-20]` | `[sp-120]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x6d6d8 | `ret(0x3a8c0) == 0x1` | `ret(0x3a8c0) != 0x1` |
| `blt` at 0x6d778 | `[[sp-20]+24] < 0x0 (signed, 32-bit)` | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x6d804 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6d818 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6d864 | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `bne` at 0x6d8dc | `ret(unknown) != 0x1` | `ret(unknown) == 0x1` |
| `bne` at 0x6d904 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6d948 | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `bne` at 0x6d9f8 | `ret(0x9ee84) != 0x2` | `ret(0x9ee84) == 0x2` |
| `beq` at 0x6da24 | `[sp-128] == 0x0` | `[sp-128] != 0x0` |
| `bne` at 0x6da54 | `[sp-128] != 0xfffff69e` | `[sp-128] == 0xfffff69e` |
| `beq` at 0x6da88 | `[sp-128] == 0xfffff69e` | `[sp-128] != 0xfffff69e` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6db0c | `r0` | `ret(0x9bd94)` |

## `syscom_process_outbound_response`

- `syscom_process_outbound_response`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6e214 to 0x6e984.
  Established. A signed-negative word at `r0 +0x18` takes the early branch, while nonnegative falls
  through (`ldr` at 0x6e23c, `blt` at 0x6e244). The body includes equality branches selecting paths
  for `[sp-208] == 0x1`, `0x0` and `0x2` (`beq` at 0x6e45c, `beq` at 0x6e474, `beq` at 0x6e484). One
  explicit-link call has unknown target and candidates `[sp-180]`, `[sp-176]`, `sp-148`, `[sp-152]`
  (`mov` at 0x6e860). A second has unknown target and candidates `[sp-180]`, `sp-148`, `[sp-152]`,
  unknown (`mov` at 0x6e8e8). Byte stores include `0x0` and `0x1` at unresolved effective targets
  (`strb` at 0x6e4c8, `strb` at 0x6e758). The constructed byte sequence’s protocol meaning and
  merged return value are unknown (`ldmdb` at 0x6e974).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6e224 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6e22c | store | 4 | `sp -0x98` | `0x0` |
| `str` at 0x6e234 | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x6e238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e23c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x6e248 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e24c | load | 4 | `r0 +0x18` | `-` |
| `str` at 0x6e254 | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x6e258 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x6e260 | store | 4 | `sp -0xb8` | `?` |
| `str` at 0x6e26c | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x6e270 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x6e274 | store | 4 | `sp -0xb0` | `[sp-184]` |
| `ldr` at 0x6e278 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x6e284 | store | 4 | `sp -0xb4` | `ret(0x6c9ec)` |
| `ldr` at 0x6e288 | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x6e294 | store | 4 | `sp -0xb0` | `ret(0x6ca20)` |
| `ldr` at 0x6e298 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e29c | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x6e2a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e2ac | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6e2b4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6e2b8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x6e2c0 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6e2c8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e2cc | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x6e2d4 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6e2d8 | load | 4 | `sp -0xbc` | `-` |
| `str` at 0x6e2dc | store | 4 | `sp -0xac` | `[sp-188]` |
| `ldr` at 0x6e2e0 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x6e2ec | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e2f0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e2fc | load | 1 | `sp -0xb0` | `-` |
| `strb` at 0x6e304 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x6e308 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e30c | load | 1 | `[sp-20] +0x13` | `-` |
| `ldr` at 0x6e31c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e320 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x6e330 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e334 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x6e338 | store | 4 | `sp -0xc0` | `[[sp-20]+40]` |
| `ldr` at 0x6e340 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6e34c | store | 4 | `sp -0xc0` | `ret(0x9ca84)` |
| `ldr` at 0x6e350 | load | 4 | `sp -0xc0` | `-` |
| `str` at 0x6e354 | store | 4 | `sp -0xa8` | `[sp-192]` |
| `ldr` at 0x6e358 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e360 | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x6e364 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e36c | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x6e370 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e378 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x6e37c | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x6e380 | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x6e388 | store | 1 | `sp -0x8f` | `0x0` |
| `str` at 0x6e390 | store | 4 | `sp -0x98` | `0x6` |
| `strb` at 0x6e39c | store | 1 | `sp -0x93` | `?` |
| `strb` at 0x6e3a4 | store | 1 | `sp -0x92` | `?` |
| `strb` at 0x6e3ac | store | 1 | `sp -0x91` | `?` |
| `strb` at 0x6e3b4 | store | 1 | `sp -0x90` | `?` |
| `ldr` at 0x6e3b8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e3bc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x6e3c4 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x6e3c8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e3cc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x6e3d4 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x6e3d8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e3dc | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x6e3e4 | store | 1 | `sp -0x8d` | `?` |
| `ldr` at 0x6e3e8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e3ec | load | 1 | `[sp-20] +0x13` | `-` |
| `strb` at 0x6e3f0 | store | 1 | `sp -0x8c` | `([[sp-20]+19]:1 & 0xff)` |
| `str` at 0x6e3f8 | store | 4 | `sp -0x98` | `0x9` |
| `str` at 0x6e400 | store | 4 | `sp -0x9c` | `0x1` |
| `str` at 0x6e408 | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x6e40c | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6e418 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6e41c | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x6e420 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e430 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e434 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6e448 | load | 1 | `(unknown)` | `-` |
| `str` at 0x6e450 | store | 4 | `sp -0xd0` | `?` |
| `ldr` at 0x6e454 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x6e460 | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x6e46c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x6e47c | load | 4 | `sp -0xd0` | `-` |
| `ldr` at 0x6e48c | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6e4a0 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6e4ac | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x6e4c8 | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x6e4d0 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6e4d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e4d8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6e4ec | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6e4fc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e500 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e514 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e518 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x6e520 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e524 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e52c | store | 4 | `sp -0xc4` | `ret(0x9ca84)` |
| `ldr` at 0x6e530 | load | 4 | `sp -0xc4` | `-` |
| `str` at 0x6e534 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x6e53c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e554 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e55c | store | 1 | `(unknown)` | `?` |
| `str` at 0x6e564 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6e56c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e584 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e58c | store | 1 | `(unknown)` | `?` |
| `str` at 0x6e594 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x6e59c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e5b4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6e5bc | store | 1 | `(unknown)` | `?` |
| `str` at 0x6e5c4 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x6e5cc | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x6e5e4 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x6e5e8 | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x6e5f0 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x6e5f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e5fc | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x6e610 | load | 2 | `(unknown)` | `-` |
| `str` at 0x6e614 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6e618 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e61c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e630 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6e63c | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x6e654 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x6e660 | store | 1 | `(unknown)` | `?` |
| `str` at 0x6e668 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6e66c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e670 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6e684 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6e694 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e698 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e6ac | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e6b0 | store | 4 | `sp -0xc8` | `?` |
| `ldr` at 0x6e6b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e6bc | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e6c4 | store | 4 | `sp -0xc8` | `ret(0x9ca84)` |
| `ldr` at 0x6e6cc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e6d8 | load | 4 | `sp -0xc8` | `-` |
| `ldr` at 0x6e6dc | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6e6e4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e6e8 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e6f0 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x6e6f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e6fc | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x6e710 | load | 2 | `(unknown)` | `-` |
| `str` at 0x6e714 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6e718 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e71c | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e730 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6e73c | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x6e758 | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x6e760 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6e764 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e768 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6e77c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6e78c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e790 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e7a4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e7a8 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x6e7b0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e7b4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e7bc | store | 4 | `sp -0xcc` | `ret(0x9ca84)` |
| `ldr` at 0x6e7c4 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e7d0 | load | 4 | `sp -0xcc` | `-` |
| `ldr` at 0x6e7d8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e7dc | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e7e4 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x6e7e8 | load | 1 | `sp -0x8f` | `-` |
| `strb` at 0x6e7f0 | store | 1 | `sp -0x8f` | `([sp-143]:1+1 & 0xff)` |
| `ldr` at 0x6e7f4 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e7fc | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6e804 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e820 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e824 | load | 4 | `abs 0x6e978` | `-` |
| `ldr` at 0x6e82c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e834 | load | 4 | `abs 0x6e97c` | `-` |
| `ldr` at 0x6e838 | load | 4 | `abs 0x11b088` | `-` |
| `ldr` at 0x6e844 | load | 4 | `abs 0x6e97c` | `-` |
| `ldr` at 0x6e84c | load | 4 | `abs 0x11b088` | `-` |
| `ldr` at 0x6e850 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e854 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e858 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e864 | load | 4 | `abs 0x6e980` | `-` |
| `ldr` at 0x6e868 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e88c | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e894 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e8a0 | load | 4 | `abs 0x6e980` | `-` |
| `ldr` at 0x6e8a4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e8c8 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e8d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e8d8 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e8e0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e8f0 | load | 4 | `abs 0x6e980` | `-` |
| `ldr` at 0x6e8f4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e918 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e924 | load | 4 | `abs 0x6e980` | `-` |
| `ldr` at 0x6e928 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x6e94c | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e958 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6e95c | store | 4 | `sp -0xd4` | `[sp-152]` |
| `ldr` at 0x6e964 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e968 | load | 4 | `sp -0xb4` | `-` |
| `ldmdb` at 0x6e974 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6e974 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6e974 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6e27c | `(unknown callee) 0x6c9ec` | `[sp-184]` | `r1` | `r2` | `?` |
| `bl` at 0x6e28c | `(unknown callee) 0x6ca20` | `[sp-176]` | `?` | `?` | `ret(0x6c9ec)` |
| `bl` at 0x6e2f4 | `(unknown callee) 0x6f8e0` | `[sp-180]` | `[sp-20]` | `?` | `[sp-188]` |
| `bl` at 0x6e348 | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x6e528 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6e6c0 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6e6e0 | `(unknown callee) 0x18ae8` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x6e7b8 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6e7d4 | `(unknown callee) 0x18cd4` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x6e80c | `(unknown callee) 0x6bc14` | `[sp-180]` | `0x40` | `?` | `?` |
| `bl` at 0x6e830 | `(unknown callee) 0x6bcac` | `[sp-180]` | `0xe8c5c` | `sp-148` | `[sp-152]` |
| `mov` at 0x6e860 | `(unknown callee) (unknown target)` | `[sp-180]` | `[sp-176]` | `sp-148` | `[sp-152]` |
| `mov` at 0x6e8e8 | `(unknown callee) (unknown target)` | `[sp-180]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x6e970 | `(unknown callee) 0x9ee84` | `?` | `?` | `[sp-180]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x6e244 | `[r0+24] < 0x0 (signed, 32-bit)` | `[r0+24] >= 0x0 (signed, 32-bit)` |
| `bge` at 0x6e2a4 | `[[sp-20]+24] >= 0x0 (signed, 32-bit)` | `[[sp-20]+24] < 0x0 (signed, 32-bit)` |
| `bne` at 0x6e2e8 | `[sp-188] != 0x15` | `[sp-188] == 0x15` |
| `beq` at 0x6e314 | `[[sp-20]+19]:1 == 0x0` | `[[sp-20]+19]:1 != 0x0` |
| `bne` at 0x6e32c | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6e414 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x6e428 | `C == 0` | `not (C == 0)` |
| `beq` at 0x6e45c | `[sp-208] == 0x1` | `[sp-208] != 0x1` |
| `bgt` at 0x6e468 | `[sp-208] > 0x1 (signed, 32-bit)` | `[sp-208] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6e474 | `[sp-208] == 0x0` | `[sp-208] != 0x0` |
| `beq` at 0x6e484 | `[sp-208] == 0x2` | `[sp-208] != 0x2` |
| `bls` at 0x6e498 | `[sp-152]+5 <= 0x7f (unsigned, 32-bit)` | `[sp-152]+5 > 0x7f (unsigned, 32-bit)` |
| `bne` at 0x6e4f8 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x6e628 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6e690 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x6e728 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6e788 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e818 | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `beq` at 0x6e840 | `[0x11b088] == 0x0` | `[0x11b088] != 0x0` |
| `bne` at 0x6e89c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6e974 | `r0` | `?` |

## `syscom_process_outbound_event`

- `syscom_process_outbound_event`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6db34 to 0x6e214.
  Established. The body loads the absolute word at `0x281fcc` before its first branch, whose taken
  predicate remains `Z == 0` (`ldr` at 0x6db54, `bne` at 0x6db5c). Later calls to `0x6c9ec` and
  `0x6ca20` use first candidate `[0x281fcc]` (`bl` at 0x6db6c, `bl` at 0x6db80). A stack byte at `sp
  -0x8b` receives `0x0`, while a stack word at `sp -0x98` receives `0xa` (`strb` at 0x6dcbc, `str`
  at 0x6dcc4). An explicit-link call has unknown target and candidates `[sp-176]`, `sp-148`,
  `[sp-152]`, unknown (`mov` at 0x6e17c). A separate call to `0x9ee84` has first and second
  candidates unknown, third `[sp-176]`, fourth `sp-148` (`bl` at 0x6e204). Event selection and
  output encoding remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6db44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6db4c | store | 4 | `sp -0x9c` | `0x0` |
| `ldr` at 0x6db50 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6db54 | load | 4 | `abs 0x281fcc` | `-` |
| `ldr` at 0x6db64 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6db68 | load | 4 | `abs 0x281fcc` | `-` |
| `str` at 0x6db74 | store | 4 | `sp -0xb0` | `ret(0x6c9ec)` |
| `ldr` at 0x6db78 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6db7c | load | 4 | `abs 0x281fcc` | `-` |
| `str` at 0x6db88 | store | 4 | `sp -0xac` | `ret(0x6ca20)` |
| `ldr` at 0x6db8c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6db90 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6db9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dba0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6dbb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbbc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6dbcc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbd0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6dbdc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbe0 | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldr` at 0x6dbf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dbfc | load | 4 | `[sp-20] +0x1c` | `-` |
| `ldrb` at 0x6dc0c | load | 1 | `sp -0xac` | `-` |
| `strb` at 0x6dc14 | store | 1 | `sp -0x94` | `?` |
| `ldr` at 0x6dc18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc1c | load | 4 | `[sp-20] +0x1c` | `-` |
| `str` at 0x6dc20 | store | 4 | `sp -0xa8` | `[[sp-20]+28]` |
| `ldr` at 0x6dc24 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dc2c | store | 1 | `sp -0x93` | `?` |
| `ldr` at 0x6dc30 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dc38 | store | 1 | `sp -0x92` | `?` |
| `ldr` at 0x6dc3c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dc44 | store | 1 | `sp -0x91` | `?` |
| `ldrb` at 0x6dc48 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x6dc4c | store | 1 | `sp -0x90` | `([sp-168]:1 & 0xff)` |
| `ldr` at 0x6dc50 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6dc54 | load | 1 | `[sp-20] +0x24` | `-` |
| `ldr` at 0x6dc64 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dc68 | load | 4 | `[sp-20] +0x28` | `-` |
| `str` at 0x6dc6c | store | 4 | `sp -0xb4` | `[[sp-20]+40]` |
| `ldr` at 0x6dc74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6dc80 | store | 4 | `sp -0xb4` | `ret(0x9ca84)` |
| `ldr` at 0x6dc84 | load | 4 | `sp -0xb4` | `-` |
| `str` at 0x6dc88 | store | 4 | `sp -0xa8` | `[sp-180]` |
| `ldr` at 0x6dc8c | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dc94 | store | 1 | `sp -0x8f` | `?` |
| `ldr` at 0x6dc98 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dca0 | store | 1 | `sp -0x8e` | `?` |
| `ldr` at 0x6dca4 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6dcac | store | 1 | `sp -0x8d` | `?` |
| `ldrb` at 0x6dcb0 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x6dcb4 | store | 1 | `sp -0x8c` | `([sp-168]:1 & 0xff)` |
| `strb` at 0x6dcbc | store | 1 | `sp -0x8b` | `0x0` |
| `str` at 0x6dcc4 | store | 4 | `sp -0x98` | `0xa` |
| `str` at 0x6dccc | store | 4 | `sp -0xa0` | `0x1` |
| `ldr` at 0x6dcd0 | load | 4 | `sp -0x9c` | `-` |
| `ldr` at 0x6dcdc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x6dce0 | load | 1 | `[sp-20] +0x20` | `-` |
| `ldr` at 0x6dce4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6dcf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dcf8 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6dd0c | load | 1 | `(unknown)` | `-` |
| `str` at 0x6dd14 | store | 4 | `sp -0xc4` | `?` |
| `ldr` at 0x6dd18 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x6dd24 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x6dd30 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x6dd40 | load | 4 | `sp -0xc4` | `-` |
| `ldr` at 0x6dd50 | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6dd64 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6dd70 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x6dd8c | store | 1 | `(unknown)` | `0x0` |
| `str` at 0x6dd94 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6dd98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dd9c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6ddb0 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6ddc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ddc4 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6ddd8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6dddc | store | 4 | `sp -0xb8` | `?` |
| `ldr` at 0x6dde4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dde8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6ddf0 | store | 4 | `sp -0xb8` | `ret(0x9ca84)` |
| `ldr` at 0x6ddf4 | load | 4 | `sp -0xb8` | `-` |
| `str` at 0x6ddf8 | store | 4 | `sp -0xa8` | `?` |
| `ldr` at 0x6de00 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6de18 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6de20 | store | 1 | `(unknown)` | `?` |
| `str` at 0x6de28 | store | 4 | `sp -0x98` | `[sp-152]+1` |
| `ldr` at 0x6de30 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6de48 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6de50 | store | 1 | `(unknown)` | `?` |
| `str` at 0x6de58 | store | 4 | `sp -0x98` | `[sp-152]+2` |
| `ldr` at 0x6de60 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6de78 | load | 4 | `sp -0xa8` | `-` |
| `strb` at 0x6de80 | store | 1 | `(unknown)` | `?` |
| `str` at 0x6de88 | store | 4 | `sp -0x98` | `[sp-152]+3` |
| `ldr` at 0x6de90 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x6dea8 | load | 1 | `sp -0xa8` | `-` |
| `strb` at 0x6deac | store | 1 | `(unknown)` | `([sp-168]:1 & 0xff)` |
| `str` at 0x6deb4 | store | 4 | `sp -0x98` | `[sp-152]+4` |
| `ldr` at 0x6debc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dec0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x6ded4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x6ded8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6dedc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6dee0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6def4 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6df00 | load | 4 | `sp -0x98` | `-` |
| `ldrb` at 0x6df18 | load | 1 | `sp -0xa4` | `-` |
| `strb` at 0x6df24 | store | 1 | `(unknown)` | `?` |
| `str` at 0x6df2c | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6df30 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6df34 | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6df48 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6df58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6df5c | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6df70 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6df74 | store | 4 | `sp -0xbc` | `?` |
| `ldr` at 0x6df7c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6df80 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6df88 | store | 4 | `sp -0xbc` | `ret(0x9ca84)` |
| `ldr` at 0x6df90 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6df9c | load | 4 | `sp -0xbc` | `-` |
| `ldr` at 0x6dfa0 | load | 4 | `sp -0xa4` | `-` |
| `ldr` at 0x6dfa8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6dfac | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6dfb4 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldr` at 0x6dfbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6dfc0 | load | 4 | `sp -0xa0` | `-` |
| `ldrh` at 0x6dfd4 | load | 2 | `(unknown)` | `-` |
| `str` at 0x6dfd8 | store | 4 | `sp -0xa4` | `?` |
| `ldr` at 0x6dfdc | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6dfe0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6dff4 | store | 4 | `sp -0x9c` | `0x1` |
| `ldr` at 0x6e000 | load | 4 | `sp -0x98` | `-` |
| `strb` at 0x6e01c | store | 1 | `(unknown)` | `0x1` |
| `str` at 0x6e024 | store | 4 | `sp -0x98` | `?` |
| `ldr` at 0x6e028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e02c | load | 4 | `sp -0xa0` | `-` |
| `ldrb` at 0x6e040 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x6e050 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e054 | load | 4 | `sp -0xa0` | `-` |
| `ldr` at 0x6e068 | load | 4 | `(unknown)` | `-` |
| `str` at 0x6e06c | store | 4 | `sp -0xc0` | `?` |
| `ldr` at 0x6e074 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6e078 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e080 | store | 4 | `sp -0xc0` | `ret(0x9ca84)` |
| `ldr` at 0x6e088 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e094 | load | 4 | `sp -0xc0` | `-` |
| `ldr` at 0x6e09c | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e0a0 | load | 4 | `sp -0xa4` | `-` |
| `str` at 0x6e0a8 | store | 4 | `sp -0x98` | `([sp-152] + [sp-164])` |
| `ldrb` at 0x6e0ac | load | 1 | `sp -0x8b` | `-` |
| `strb` at 0x6e0b4 | store | 1 | `sp -0x8b` | `?` |
| `ldr` at 0x6e0b8 | load | 4 | `sp -0xa0` | `-` |
| `str` at 0x6e0c0 | store | 4 | `sp -0xa0` | `?` |
| `ldr` at 0x6e0c8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e0e4 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e0e8 | load | 4 | `abs 0x6e210` | `-` |
| `ldr` at 0x6e0f0 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e0f8 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6e0fc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e120 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e128 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e134 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6e138 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e15c | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e168 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e16c | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e174 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x6e184 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6e188 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e1ac | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e1b8 | load | 4 | `abs 0x6e20c` | `-` |
| `ldr` at 0x6e1bc | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x6e1e0 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6e1ec | load | 4 | `sp -0x98` | `-` |
| `str` at 0x6e1f0 | store | 4 | `sp -0xc8` | `[sp-152]` |
| `ldr` at 0x6e1f8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6e1fc | load | 4 | `sp -0xb0` | `-` |
| `ldmdb` at 0x6e208 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6e208 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6e208 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6db6c | `(unknown callee) 0x6c9ec` | `[0x281fcc]` | `r1` | `r2` | `0x281fc0` |
| `bl` at 0x6db80 | `(unknown callee) 0x6ca20` | `[0x281fcc]` | `?` | `?` | `0x281fc0` |
| `bl` at 0x6dc7c | `(unknown callee) 0x9ca84` | `[sp-20]` | `0x0` | `?` | `?` |
| `bl` at 0x6ddec | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6df84 | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6dfa4 | `(unknown callee) 0x18ae8` | `(sp-148 + [sp-152])` | `?` | `[sp-164]` | `(sp-148 + [sp-152])` |
| `bl` at 0x6e07c | `(unknown callee) 0x9ca84` | `[sp-20]` | `[sp-160]` | `0x24` | `?` |
| `bl` at 0x6e098 | `(unknown callee) 0x18cd4` | `(sp-148 + [sp-152])` | `?` | `sp-148` | `(sp-148 + [sp-152])` |
| `bl` at 0x6e0d0 | `(unknown callee) 0x6bc14` | `[sp-176]` | `0x20` | `?` | `?` |
| `bl` at 0x6e0f4 | `(unknown callee) 0x6bcac` | `[sp-176]` | `0xe8c4c` | `sp-148` | `[sp-152]` |
| `mov` at 0x6e17c | `(unknown callee) (unknown target)` | `[sp-176]` | `sp-148` | `[sp-152]` | `?` |
| `bl` at 0x6e204 | `(unknown callee) 0x9ee84` | `?` | `?` | `[sp-176]` | `sp-148` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x6db5c | `Z == 0` | `not (Z == 0)` |
| `bge` at 0x6db98 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `beq` at 0x6dbb0 | `? == 0x5` | `? != 0x5` |
| `beq` at 0x6dbc8 | `? == 0x5` | `? != 0x5` |
| `bge` at 0x6dbd8 | `[[sp-20]+28] >= 0x0 (signed, 32-bit)` | `[[sp-20]+28] < 0x0 (signed, 32-bit)` |
| `bne` at 0x6dbf0 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x6dc08 | `? != 0x4` | `? == 0x4` |
| `bne` at 0x6dc60 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x6dcd8 | `[sp-156] != 0x0` | `[sp-156] == 0x0` |
| `blo` at 0x6dcec | `C == 0` | `not (C == 0)` |
| `beq` at 0x6dd20 | `[sp-196] == 0x1` | `[sp-196] != 0x1` |
| `bgt` at 0x6dd2c | `[sp-196] > 0x1 (signed, 32-bit)` | `[sp-196] <= 0x1 (signed, 32-bit)` |
| `beq` at 0x6dd38 | `[sp-196] == 0x0` | `[sp-196] != 0x0` |
| `beq` at 0x6dd48 | `[sp-196] == 0x2` | `[sp-196] != 0x2` |
| `bls` at 0x6dd5c | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6ddbc | `? != 0x0` | `? == 0x0` |
| `bls` at 0x6deec | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6df54 | `? != 0x0` | `? == 0x0` |
| `bls` at 0x6dfec | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0x6e04c | `? != 0x0` | `? == 0x0` |
| `beq` at 0x6e0dc | `ret(0x6bc14) == 0x0` | `ret(0x6bc14) != 0x0` |
| `bne` at 0x6e130 | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6e208 | `r0` | `?` |

## `SysComSubchan`

- `SysComSubchan`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ca20 to 0x6ca54.
  Established. The wrapper loads the word at absolute `0x281fc8` (`ldr` at 0x6ca38) and calls
  `0x70738` with candidates `[0x281fc8]`, entry r0, entry r2 and `0x281fc0` (`bl` at 0x6ca40). The
  return reports `ret(0x70738)` in r0 (`ldmdb` at 0x6ca4c). The numeric callee’s subchannel mapping
  is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ca30 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ca34 | load | 4 | `abs 0x6ca50` | `-` |
| `ldr` at 0x6ca38 | load | 4 | `abs 0x281fc8` | `-` |
| `ldr` at 0x6ca3c | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6ca4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ca4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ca4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ca40 | `(unknown callee) 0x70738` | `[0x281fc8]` | `r0` | `r2` | `0x281fc0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ca4c | `r0` | `ret(0x70738)` |

## `SysComSlab`

- `SysComSlab`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6c9ec to 0x6ca20.
  Established. Entry r0 is saved as a word at `sp -0x14` (`str` at 0x6c9fc). The call to `0x70694`
  has candidates `[0x281fc8]`, entry r0, entry r2 and `0x281fc0` (`bl` at 0x6ca0c). That result
  reaches the return in r0 (`ldmdb` at 0x6ca18). A slab interpretation of the result is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6c9fc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6ca00 | load | 4 | `abs 0x6ca1c` | `-` |
| `ldr` at 0x6ca04 | load | 4 | `abs 0x281fc8` | `-` |
| `ldr` at 0x6ca08 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x6ca18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ca18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ca18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ca0c | `(unknown callee) 0x70694` | `[0x281fc8]` | `r0` | `r2` | `0x281fc0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ca18 | `r0` | `ret(0x70694)` |

## `SysComTasknum`

- `SysComTasknum`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ca54 to 0x6ca90.
  Established. The wrapper saves entry r0 and r1 at `sp -0x14` and `sp -0x18` (`str` at 0x6ca64,
  `str` at 0x6ca68); its call to `0x7083c` uses candidates `[0x281fc8]`, entry r0, entry r1 and
  `0x281fc0` (`bl` at 0x6ca7c). The return reports `ret(0x7083c)` (`ldmdb` at 0x6ca88). The result’s
  task-number meaning remains unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6ca64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x6ca68 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x6ca6c | load | 4 | `abs 0x6ca8c` | `-` |
| `ldr` at 0x6ca70 | load | 4 | `abs 0x281fc8` | `-` |
| `ldr` at 0x6ca74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6ca78 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x6ca88 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6ca88 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6ca88 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6ca7c | `(unknown callee) 0x7083c` | `[0x281fc8]` | `r0` | `r1` | `0x281fc0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6ca88 | `r0` | `ret(0x7083c)` |

## `SysComJTagReadReady`

- `SysComJTagReadReady`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x6ca90 to 0x6cb94.
  Established. The first branch takes its edge for `r0 >= [0x281fd0]` signed and falls through for
  less-than (`bge` at 0x6cab4). A later zero word at absolute `0x281fd4` takes another edge to
  the return (`beq` at 0x6cac4). The longer path includes a call to `0x1fe64` with first candidate
  unknown, second `[sp-20]`, and the rest unknown (`bl` at 0x6cb3c). A zero result takes the edge
  past the call to `0x1eed0`, while a nonzero result falls through to it (`beq` at 0x6cb50, `bl`
  at 0x6cb7c). Readiness signalling semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x6caa0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x6caa4 | load | 4 | `abs 0x6cb84` | `-` |
| `ldr` at 0x6caa8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6caac | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x6cab8 | load | 4 | `abs 0x6cb84` | `-` |
| `ldr` at 0x6cabc | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6cac8 | load | 4 | `abs 0x6cb84` | `-` |
| `ldr` at 0x6cacc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6caf0 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x6caf8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x6cb04 | load | 4 | `abs 0x6cb84` | `-` |
| `ldr` at 0x6cb08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x6cb2c | load | 4 | `abs 0x281fd4` | `-` |
| `str` at 0x6cb44 | store | 4 | `sp -0x18` | `ret(0x1fe64)` |
| `ldr` at 0x6cb48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x6cb54 | load | 4 | `abs 0x6cb88` | `-` |
| `str` at 0x6cb58 | store | 4 | `sp -0x24` | `0xe8aa4` |
| `ldr` at 0x6cb5c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x6cb60 | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0x6cb64 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x6cb68 | store | 4 | `sp -0x1c` | `ret(0x1fe64)` |
| `ldr` at 0x6cb70 | load | 4 | `abs 0x6cb8c` | `-` |
| `ldr` at 0x6cb74 | load | 4 | `abs 0x6cb90` | `-` |
| `ldmdb` at 0x6cb80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x6cb80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x6cb80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x6cb3c | `(unknown callee) 0x1fe64` | `?` | `[sp-20]` | `?` | `?` |
| `bl` at 0x6cb7c | `(unknown callee) 0x1eed0` | `0x9` | `0xe89f8` | `0x22e` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0x6cab4 | `r0 >= [0x281fd0] (signed, 32-bit)` | `r0 < [0x281fd0] (signed, 32-bit)` |
| `beq` at 0x6cac4 | `[0x281fd4] == 0x0` | `[0x281fd4] != 0x0` |
| `beq` at 0x6cb00 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x6cb50 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x6cb80 | `r0` | `?` |

## `SysComSimStop`

- `SysComSimStop`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x95268 to 0x95288.
  Established. The only explicit word store writes `0x0` at absolute `0x11b080` (`str` at 0x9527c).
  The return retains entry r0 (`ldmdb` at 0x95280). No calls or conditional branches are reported.
  The absolute word’s simulation-control role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x95274 | load | 4 | `abs 0x95284` | `-` |
| `str` at 0x9527c | store | 4 | `abs 0x11b080` | `0x0` |
| `ldmdb` at 0x95280 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x95280 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x95280 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x95280 | `r0` | `r0` |

## `SysComSimStart`

- `SysComSimStart`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x95188 to 0x95268.
  Established. A nonzero word at absolute `0x11b3a4` takes the edge that bypasses the first two
  calls, while zero falls through (`ldr` at 0x9519c, `bne` at 0x951a4). Those calls target `0x1f9e8`
  with `0x11b3b0`, `0xfc2e4`, `0x0`, `0x9` and `0x1f498` with `0x11b3ac`, `0xfc2f0`, `0x95288`,
  `0x0` (`bl` at 0x951b8, `bl` at 0x951f4). A word store then writes `0x1` at absolute `0x11b3a4`
  (`str` at 0x95200). The joined path calls `0x1fe64` with first candidate `0x11b3b0` (`bl` at
  0x95208). The resource and task semantics of these numeric callees are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x95198 | load | 4 | `abs 0x95248` | `-` |
| `ldr` at 0x9519c | load | 4 | `abs 0x11b3a4` | `-` |
| `ldr` at 0x951a8 | load | 4 | `abs 0x9524c` | `-` |
| `ldr` at 0x951ac | load | 4 | `abs 0x95250` | `-` |
| `str` at 0x951c0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x951c8 | store | 4 | `sp -0x24` | `0x800` |
| `str` at 0x951d0 | store | 4 | `sp -0x20` | `0x19` |
| `str` at 0x951d8 | store | 4 | `sp -0x1c` | `0xa` |
| `str` at 0x951e0 | store | 4 | `sp -0x18` | `0x9` |
| `ldr` at 0x951e4 | load | 4 | `abs 0x95254` | `-` |
| `ldr` at 0x951e8 | load | 4 | `abs 0x95258` | `-` |
| `ldr` at 0x951ec | load | 4 | `abs 0x9525c` | `-` |
| `ldr` at 0x951f8 | load | 4 | `abs 0x95248` | `-` |
| `str` at 0x95200 | store | 4 | `abs 0x11b3a4` | `0x1` |
| `ldr` at 0x95204 | load | 4 | `abs 0x9524c` | `-` |
| `str` at 0x95210 | store | 4 | `sp -0x14` | `ret(0x1fe64)` |
| `ldr` at 0x95214 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x95220 | load | 4 | `abs 0x95260` | `-` |
| `str` at 0x95224 | store | 4 | `sp -0x28` | `0xfc2fc` |
| `ldr` at 0x95228 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9522c | store | 4 | `sp -0x24` | `ret(0x1fe64)` |
| `ldr` at 0x95234 | load | 4 | `abs 0x95264` | `-` |
| `ldmdb` at 0x95244 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x95244 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x95244 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x951b8 | `(unknown callee) 0x1f9e8` | `0x11b3b0` | `0xfc2e4` | `0x0` | `0x9` |
| `bl` at 0x951f4 | `(unknown callee) 0x1f498` | `0x11b3ac` | `0xfc2f0` | `0x95288` | `0x0` |
| `bl` at 0x95208 | `(unknown callee) 0x1fe64` | `0x11b3b0` | `?` | `?` | `?` |
| `bl` at 0x95240 | `(unknown callee) 0x1eed0` | `0x9` | `0xfc328` | `0x3f` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x951a4 | `[0x11b3a4] != 0x0` | `[0x11b3a4] == 0x0` |
| `beq` at 0x9521c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x95244 | `r0` | `?` |

## `TIOSysComInt`

- `TIOSysComInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x45e44 to 0x45f30.
  Established. The body stores the low bytes of entry r0, r1 and r2 at `sp -0x11`, `sp -0x12` and
  `sp -0x13` (`strb` at 0x45e58, `strb` at 0x45e60, `strb` at 0x45e68); its loop comparison retains
  a signed less-than flags predicate with unknown operands (`blt` at 0x45e84). A later branch takes
  the bypass edge when an unknown value equals zero and falls through to the call to `0x6ca90`
  for nonzero (`beq` at 0x45f04, `bl` at 0x45f14). That call has first and second candidates
  unknown, third `(r2 & 0xff)` and fourth unknown (`bl` at 0x45f14). The interrupt source and mask
  interpretation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x45e58 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x45e60 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x45e68 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `str` at 0x45e70 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x45e74 | load | 4 | `abs 0x45f1c` | `-` |
| `ldrb` at 0x45e78 | load | 1 | `abs 0x2811d8` | `-` |
| `ldr` at 0x45e7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45e88 | load | 4 | `abs 0x45f20` | `-` |
| `str` at 0x45e8c | store | 4 | `sp -0x24` | `0xe2f34` |
| `ldrb` at 0x45e90 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x45e94 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0x45e9c | load | 4 | `abs 0x45f24` | `-` |
| `ldr` at 0x45ea0 | load | 4 | `abs 0x45f28` | `-` |
| `ldr` at 0x45eb0 | load | 4 | `abs 0x45f2c` | `-` |
| `ldr` at 0x45eb8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x45ec4 | load | 4 | `abs 0x119e78` | `-` |
| `str` at 0x45ecc | store | 4 | `sp -0x18` | `?` |
| `str` at 0x45ed4 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x45ed8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45edc | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x45ee0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x45eec | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x45ef0 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x45f08 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x45f0c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x45f18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x45f18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x45f18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x45ea8 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2e3c` | `0x242` | `0x0` |
| `bl` at 0x45f14 | `(unknown callee) 0x6ca90` | `?` | `?` | `(r2 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x45e84 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x45ee8 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x45f04 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x45f18 | `r0` | `?` |

## `NL4R_SysComInt`

- `NL4R_SysComInt`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x333a4 to 0x33490.
  Established. Three byte stores retain `(r0 & 0xff)`, `(r1 & 0xff)` and `(r2 & 0xff)` at `sp
  -0x11`, `sp -0x12`, `sp -0x13` (`strb` at 0x333b8, `strb` at 0x333c0, `strb` at 0x333c8). A loop
  branch reports `Z == 0` for its taken repeat edge, with operand identities unknown (`bne` at
  0x33448). A call to `0x6ca90` has candidates unknown, unknown, `(r2 & 0xff)`, unknown (`bl` at
  0x33474). The branch before it takes the bypass for `? == 0x0` and falls through for `? != 0x0`
  (`beq` at 0x33464). No particular interrupt acknowledgement is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x333b8 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x333c0 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `strb` at 0x333c8 | store | 1 | `sp -0x13` | `(r2 & 0xff)` |
| `str` at 0x333d0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x333d4 | load | 4 | `abs 0x3347c` | `-` |
| `ldr` at 0x333d8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x333dc | load | 4 | `abs 0x119d80` | `-` |
| `ldr` at 0x333e8 | load | 4 | `abs 0x33480` | `-` |
| `str` at 0x333ec | store | 4 | `sp -0x24` | `0xde664` |
| `ldrb` at 0x333f0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x333f4 | store | 4 | `sp -0x20` | `(r0 & 0xff)` |
| `ldr` at 0x333fc | load | 4 | `abs 0x33484` | `-` |
| `ldr` at 0x33400 | load | 4 | `abs 0x33488` | `-` |
| `ldr` at 0x33410 | load | 4 | `abs 0x3348c` | `-` |
| `ldr` at 0x33418 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x33424 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x3342c | store | 4 | `sp -0x18` | `?` |
| `str` at 0x33434 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x33438 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3343c | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x33440 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x3344c | load | 1 | `sp -0x13` | `-` |
| `ldrb` at 0x33450 | load | 1 | `sp -0x12` | `-` |
| `ldr` at 0x33468 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x3346c | load | 1 | `(unknown)` | `-` |
| `ldmdb` at 0x33478 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x33478 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x33478 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x33408 | `(unknown callee) 0x1eed0` | `0x1` | `0xde624` | `0x11b` | `0x0` |
| `bl` at 0x33474 | `(unknown callee) 0x6ca90` | `?` | `?` | `(r2 & 0xff)` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x333e4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x33448 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x33464 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x33478 | `r0` | `?` |

## `NL4R_SysComEnable`

- `NL4R_SysComEnable`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x332b0 to 0x333a4.
  Established. Entry r0 unequal to `0xff` takes the single-selection path, while equality falls
  through into a loop (`bne` at 0x332cc). The single-selection comparison takes its edge for `r0 <
  [0x119d80]` signed and falls through otherwise (`blt` at 0x33344). Both paths include a call to
  `0xb7c74` with candidates `[[sp-24]+2]:1`, `0x2`, unknown, `[[sp-24]+2]:1` (`bl` at 0x33320, `bl`
  at 0x33384). The joined path stores word `0x0` at `sp -0x1c` (`str` at 0x3338c), and the return
  reports `[sp-28]` (`ldmdb` at 0x33394). The value `0x2` has no established electrical meaning.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x332c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x332c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x332d4 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x332d8 | load | 4 | `abs 0x33398` | `-` |
| `ldr` at 0x332dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x332e0 | load | 4 | `abs 0x119d80` | `-` |
| `ldr` at 0x332f0 | load | 4 | `abs 0x3339c` | `-` |
| `ldr` at 0x332f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x332fc | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x33304 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x33308 | load | 4 | `abs 0x333a0` | `-` |
| `ldr` at 0x33310 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x33314 | load | 1 | `[sp-24] +0x2` | `-` |
| `ldr` at 0x33324 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x3332c | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `ldr` at 0x33334 | load | 4 | `abs 0x33398` | `-` |
| `ldr` at 0x33338 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x3333c | load | 4 | `abs 0x119d80` | `-` |
| `str` at 0x3334c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x33354 | load | 4 | `abs 0x3339c` | `-` |
| `ldr` at 0x33358 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x33360 | load | 4 | `abs 0x119d7c` | `-` |
| `str` at 0x33368 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0x3336c | load | 4 | `abs 0x333a0` | `-` |
| `ldr` at 0x33374 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x33378 | load | 1 | `[sp-24] +0x2` | `-` |
| `str` at 0x3338c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x33390 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x33394 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x33394 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x33394 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x3330c | `(unknown callee) 0x1dba4` | `0xde654` | `0x119d7c` | `?` | `?` |
| `bl` at 0x33320 | `(unknown callee) 0xb7c74` | `[[sp-24]+2]:1` | `0x2` | `?` | `[[sp-24]+2]:1` |
| `bl` at 0x33370 | `(unknown callee) 0x1dba4` | `0xde654` | `0x119d7c` | `?` | `?` |
| `bl` at 0x33384 | `(unknown callee) 0xb7c74` | `[[sp-24]+2]:1` | `0x2` | `?` | `[[sp-24]+2]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x332cc | `r0 != 0xff` | `r0 == 0xff` |
| `blt` at 0x332e8 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x33344 | `r0 < [0x119d80] (signed, 32-bit)` | `r0 >= [0x119d80] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x33394 | `r0` | `[sp-28]` |

## `SysComCaptureReset`

- `SysComCaptureReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x95d18 to 0x96070.
  Established. A nonzero absolute word at `0x11b3a8` takes the edge past the initial setup path,
  while zero falls through to a word store of `0x1` there (`ldr` at 0x95d2c, `bne` at 0x95d34,
  `str` at 0x95d40). Two later word stores put `0x95458` at absolute `0x11b084` and `0x958b8` at
  absolute `0x11b088` (`str` at 0x95ee8, `str` at 0x95ef4). Other stores include `ret(0x18920)` at
  unresolved effective targets (`str` at 0x95e2c, `str` at 0x95e90). The body calls `0x18c20` twice
  with first candidate unknown, second `0x0` and the remaining candidates unknown (`bl` at 0x95fa8,
  `bl` at 0x9603c). The stored numeric addresses are not calls, and their capture role is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x95d28 | load | 4 | `abs 0x96054` | `-` |
| `ldr` at 0x95d2c | load | 4 | `abs 0x11b3a8` | `-` |
| `ldr` at 0x95d38 | load | 4 | `abs 0x96054` | `-` |
| `str` at 0x95d40 | store | 4 | `abs 0x11b3a8` | `0x1` |
| `ldr` at 0x95d44 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95d48 | load | 4 | `abs 0x281fc8` | `-` |
| `str` at 0x95d5c | store | 4 | `sp -0x1c` | `0x14` |
| `str` at 0x95d68 | store | 4 | `sp -0x1c` | `0x20` |
| `str` at 0x95d70 | store | 4 | `sp -0x1c` | `0x8` |
| `str` at 0x95d78 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x95d7c | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95d80 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95d84 | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x95d94 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95d98 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95dbc | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95dc4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95dc8 | store | 4 | `(unknown)` | `[sp-28]` |
| `ldr` at 0x95dcc | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95dd0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95df4 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95dfc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95e2c | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x95e30 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95e34 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95e58 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95e60 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x95e90 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x95e94 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x95e9c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x95ea4 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95ea8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x95eac | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x95ed4 | load | 4 | `abs 0x9605c` | `-` |
| `ldr` at 0x95ee0 | load | 4 | `abs 0x96060` | `-` |
| `ldr` at 0x95ee4 | load | 4 | `abs 0x96064` | `-` |
| `str` at 0x95ee8 | store | 4 | `abs 0x11b084` | `0x95458` |
| `ldr` at 0x95eec | load | 4 | `abs 0x96068` | `-` |
| `ldr` at 0x95ef0 | load | 4 | `abs 0x9606c` | `-` |
| `str` at 0x95ef4 | store | 4 | `abs 0x11b088` | `0x958b8` |
| `str` at 0x95efc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x95f00 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95f04 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95f08 | load | 4 | `abs 0x281fd0` | `-` |
| `ldr` at 0x95f18 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95f1c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95f40 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95f48 | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95f4c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95f70 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95f78 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x95f9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x95fac | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95fb0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x95fd4 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x95fdc | load | 4 | `abs 0x96058` | `-` |
| `ldr` at 0x95fe0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x96004 | load | 4 | `abs 0x281fd4` | `-` |
| `ldr` at 0x9600c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x96030 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x96040 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x96048 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x96050 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x96050 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x96050 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x96050 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x95e24 | `(unknown callee) 0x18920` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0x95e88 | `(unknown callee) 0x18920` | `?` | `[sp-24]` | `[sp-28]` | `?` |
| `bl` at 0x95edc | `(unknown callee) 0x1dba4` | `0xfc3a0` | `?` | `?` | `?` |
| `bl` at 0x95fa8 | `(unknown callee) 0x18c20` | `?` | `0x0` | `?` | `?` |
| `bl` at 0x9603c | `(unknown callee) 0x18c20` | `?` | `0x0` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x95d34 | `[0x11b3a8] != 0x0` | `[0x11b3a8] == 0x0` |
| `beq` at 0x95d50 | `[0x281fc8] == 0x40` | `[0x281fc8] != 0x40` |
| `blt` at 0x95d8c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0x95f10 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x96050 | `r0` | `?` |
