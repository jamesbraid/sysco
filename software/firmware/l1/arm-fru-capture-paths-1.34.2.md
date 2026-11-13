# ARM FRU capture and status paths in 1.34.2

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
The initialization selector has an unresolved computed transfer; its excluded destinations supply no known-edge arguments or effects. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/fru-capture-paths-facts.txt) retain all tool sections.

## `charioClose`

- `charioClose`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x20d7c to 0x20eac.
  Established. The branch takes its alternate path for `[0x1170e4] != 0x0` and falls through on zero
  to `0x18be4` (`bne` at 0x20de4, `bl` at 0x20df0). That call carries `[0x1170e8]`, unknown,
  unknown, `0x1170e8` (`bl` at 0x20df0). The fall-through path clears words `0x1170e8` and
  `0x1170ec` (`str` at 0x20dfc, `str` at 0x20e08). On the alternate path, `[0x1170ec] == 0x0` takes
  the bypass, while nonzero falls through to the call to `0xadb18` and the word clear at `0x1170ec` (`beq`
  at 0x20e1c, `bl` at 0x20e2c, `str` at 0x20e40). The r0 return is `[sp-20]`, and the physical close
  operation is unknown (`ldmdb` at 0x20e8c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x20d90 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x20d94 | load | 4 | `abs 0x20e90` | `-` |
| `str` at 0x20da4 | store | 4 | `sp -0x18` | `ret(0x1fb00)` |
| `ldr` at 0x20da8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20db4 | load | 4 | `abs 0x20e94` | `-` |
| `str` at 0x20db8 | store | 4 | `sp -0x20` | `0xd956c` |
| `ldr` at 0x20dbc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x20dc0 | store | 4 | `sp -0x1c` | `ret(0x1fb00)` |
| `ldr` at 0x20dc8 | load | 4 | `abs 0x20e98` | `-` |
| `ldr` at 0x20dd8 | load | 4 | `abs 0x20e9c` | `-` |
| `ldr` at 0x20ddc | load | 4 | `abs 0x1170e4` | `-` |
| `ldr` at 0x20de8 | load | 4 | `abs 0x20ea0` | `-` |
| `ldr` at 0x20dec | load | 4 | `abs 0x1170e8` | `-` |
| `ldr` at 0x20df4 | load | 4 | `abs 0x20ea0` | `-` |
| `str` at 0x20dfc | store | 4 | `abs 0x1170e8` | `0x0` |
| `ldr` at 0x20e00 | load | 4 | `abs 0x20ea4` | `-` |
| `str` at 0x20e08 | store | 4 | `abs 0x1170ec` | `0x0` |
| `ldr` at 0x20e10 | load | 4 | `abs 0x20ea4` | `-` |
| `ldr` at 0x20e14 | load | 4 | `abs 0x1170ec` | `-` |
| `ldr` at 0x20e20 | load | 4 | `abs 0x20ea0` | `-` |
| `ldr` at 0x20e24 | load | 4 | `abs 0x1170e8` | `-` |
| `str` at 0x20e34 | store | 4 | `sp -0x14` | `ret(0xadb18)` |
| `ldr` at 0x20e38 | load | 4 | `abs 0x20ea4` | `-` |
| `str` at 0x20e40 | store | 4 | `abs 0x1170ec` | `0x0` |
| `ldr` at 0x20e44 | load | 4 | `abs 0x20e90` | `-` |
| `str` at 0x20e50 | store | 4 | `sp -0x18` | `ret(0x1fe64)` |
| `ldr` at 0x20e54 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x20e60 | load | 4 | `abs 0x20ea8` | `-` |
| `str` at 0x20e64 | store | 4 | `sp -0x20` | `0xd9590` |
| `ldr` at 0x20e68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x20e6c | store | 4 | `sp -0x1c` | `ret(0x1fe64)` |
| `ldr` at 0x20e74 | load | 4 | `abs 0x20e98` | `-` |
| `ldr` at 0x20e84 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x20e8c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x20e8c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x20e8c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x20d9c | `(unknown callee) 0x1fb00` | `0x117090` | `?` | `r2` | `0x0` |
| `bl` at 0x20dd4 | `(unknown callee) 0x1eed0` | `0x9` | `0xd9544` | `0xcd` | `0x0` |
| `bl` at 0x20df0 | `(unknown callee) 0x18be4` | `[0x1170e8]` | `?` | `?` | `0x1170e8` |
| `bl` at 0x20e2c | `(unknown callee) 0xadb18` | `[0x1170e8]` | `0x1` | `?` | `0x1170e8` |
| `bl` at 0x20e48 | `(unknown callee) 0x1fe64` | `0x117090` | `?` | `?` | `?` |
| `bl` at 0x20e80 | `(unknown callee) 0x1eed0` | `0x9` | `0xd9544` | `0xe4` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x20db0 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `bne` at 0x20de4 | `[0x1170e4] != 0x0` | `[0x1170e4] == 0x0` |
| `beq` at 0x20e1c | `[0x1170ec] == 0x0` | `[0x1170ec] != 0x0` |
| `beq` at 0x20e5c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x20e8c | `r0` | `[sp-20]` |

## `L1FRU_2m_Init`

- `L1FRU_2m_Init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8c824 to 0x8ca9c.
  Established. The first branch takes the existing-state path for `[0x11b2f0] != 0x0` and falls
  through when zero (`bne` at 0x8c840). The zero path calls `0x18c20` with `0x2822e0`, `0x0`,
  `0x2c`, `[0x11b2f0]` (`bl` at 0x8c850); its later computed transfer takes an unresolved edge for
  `? <= 0x1a` unsigned and falls through above that bound (`ldrls` at 0x8c868). On the
  existing-state path, two word stores write `[0x11b2e8]+1` at absolute `0x11b2e8` (`str` at
  0x8ca0c, `str` at 0x8ca40). Between them, `[0x2822e8] != 0x0` takes a bypass, while zero falls
  through to a word store of `0x3` at `0x2822e8` (`bne` at 0x8ca1c, `str` at 0x8ca28). Excluded
  initialization destinations and their hardware actions remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8c834 | load | 4 | `abs 0x8ca48` | `-` |
| `ldr` at 0x8c838 | load | 4 | `abs 0x11b2f0` | `-` |
| `ldr` at 0x8c844 | load | 4 | `abs 0x8ca4c` | `-` |
| `ldrls` at 0x8c868 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8c9d0 | load | 4 | `abs 0x8ca90` | `-` |
| `ldr` at 0x8c9d4 | load | 4 | `abs 0x11b2e8` | `-` |
| `ldr` at 0x8c9e0 | load | 4 | `abs 0x8ca94` | `-` |
| `str` at 0x8c9e4 | store | 4 | `sp -0x1c` | `0xfb428` |
| `ldr` at 0x8c9ec | load | 4 | `abs 0x8ca98` | `-` |
| `ldr` at 0x8c9fc | load | 4 | `abs 0x8ca90` | `-` |
| `ldr` at 0x8ca00 | load | 4 | `abs 0x8ca90` | `-` |
| `ldr` at 0x8ca04 | load | 4 | `abs 0x11b2e8` | `-` |
| `str` at 0x8ca0c | store | 4 | `abs 0x11b2e8` | `[0x11b2e8]+1` |
| `ldr` at 0x8ca10 | load | 4 | `abs 0x8ca4c` | `-` |
| `ldr` at 0x8ca14 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8ca20 | load | 4 | `abs 0x8ca4c` | `-` |
| `str` at 0x8ca28 | store | 4 | `abs 0x2822e8` | `0x3` |
| `ldr` at 0x8ca30 | load | 4 | `abs 0x8ca90` | `-` |
| `ldr` at 0x8ca34 | load | 4 | `abs 0x8ca90` | `-` |
| `ldr` at 0x8ca38 | load | 4 | `abs 0x11b2e8` | `-` |
| `str` at 0x8ca40 | store | 4 | `abs 0x11b2e8` | `[0x11b2e8]+1` |
| `ldmdb` at 0x8ca44 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8ca44 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8ca44 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8ca44 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8c850 | `(unknown callee) 0x18c20` | `0x2822e0` | `0x0` | `0x2c` | `[0x11b2f0]` |
| `bl` at 0x8c854 | `(unknown callee) 0x2b368` | `ret(0x18c20)` | `?` | `?` | `?` |
| `bl` at 0x8c9f8 | `(unknown callee) 0x1eed0` | `0x9` | `0xfb448` | `0x7e` | `0x0` |
| `bl` at 0x8ca2c | `(unknown callee) 0xa1130` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8c840 | `[0x11b2f0] != 0x0` | `[0x11b2f0] == 0x0` |
| `ldrls` at 0x8c868 | `? <= 0x1a (unsigned, 32-bit)` | `? > 0x1a (unsigned, 32-bit)` |
| `beq` at 0x8c9dc | `[0x11b2e8] == 0x0` | `[0x11b2e8] != 0x0` |
| `bne` at 0x8ca1c | `[0x2822e8] != 0x0` | `[0x2822e8] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8ca44 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x8c868).

## `L1FRU_2m_Capture`

- `L1FRU_2m_Capture`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8cb00 to 0x8ccd4.
  Established. The result of `0x189c8` is stored as a word at absolute `0x2822ec` (`bl` at 0x8cb1c,
  `str` at 0x8cb20). Two alternative stores write word `0x1` or `0x2` at `0x2822f0`, followed by
  word `0x0` at `0x2822e8` (`str` at 0x8cba8, `str` at 0x8cbb8, `str` at 0x8cbc4). The selector
  branches take their call arms for `[sp-36] == 0x2` and `[sp-36] == 0x1`, falling through on
  inequality (`beq` at 0x8cc28, `beq` at 0x8cc40). The corresponding calls reach `0x8f354` and
  `0x8e24c` with unknown, `0x2822e0`, unknown, unknown (`bl` at 0x8cc9c, `bl` at 0x8cc70).
  Captured-data meaning is unknown, and the reported return retains `[sp-32]` (`ldmdb` at 0x8ccc8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8cb10 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x8cb14 | load | 4 | `abs 0x8cccc` | `-` |
| `str` at 0x8cb20 | store | 4 | `abs 0x2822ec` | `ret(0x189c8)` |
| `ldr` at 0x8cb24 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cb28 | load | 4 | `abs 0x8ccd0` | `-` |
| `ldr` at 0x8cb2c | load | 4 | `abs 0x2822ec` | `-` |
| `ldr` at 0x8cb30 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb38 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cb3c | load | 4 | `abs 0x282300` | `-` |
| `ldr` at 0x8cb48 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cb4c | load | 4 | `abs 0x282304` | `-` |
| `str` at 0x8cb60 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8cb68 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cb88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cba0 | load | 4 | `abs 0x8cccc` | `-` |
| `str` at 0x8cba8 | store | 4 | `abs 0x2822f0` | `0x1` |
| `ldr` at 0x8cbb0 | load | 4 | `abs 0x8cccc` | `-` |
| `str` at 0x8cbb8 | store | 4 | `abs 0x2822f0` | `0x2` |
| `ldr` at 0x8cbbc | load | 4 | `abs 0x8cccc` | `-` |
| `str` at 0x8cbc4 | store | 4 | `abs 0x2822e8` | `0x0` |
| `str` at 0x8cbcc | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x8cbd0 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cbd4 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8cbe0 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cbe4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cbe8 | load | 4 | `abs 0x2822f4` | `-` |
| `ldr` at 0x8cbf8 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cbfc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cc10 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cc18 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cc1c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8cc20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8cc2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8cc38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8cc4c | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cc50 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cc64 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cc78 | load | 4 | `abs 0x8cccc` | `-` |
| `ldr` at 0x8cc7c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cc90 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cca0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8cca8 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `str` at 0x8ccc0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8ccc4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0x8ccc8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8ccc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8ccc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8ccc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cb1c | `(unknown callee) 0x189c8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x8cb34 | `(unknown callee) 0x1dba4` | `0xfb464` | `[0x2822ec]` | `[sp-24]` | `0x2822e0` |
| `bl` at 0x8cc70 | `(unknown callee) 0x8e24c` | `?` | `0x2822e0` | `?` | `?` |
| `bl` at 0x8cc9c | `(unknown callee) 0x8f354` | `?` | `0x2822e0` | `?` | `?` |
| `bl` at 0x8ccb0 | `(unknown callee) 0x8e1f0` | `?` | `?` | `?` | `?` |
| `bl` at 0x8ccb4 | `(unknown callee) 0x8e20c` | `ret(0x8e1f0)` | `?` | `?` | `?` |
| `bl` at 0x8ccb8 | `(unknown callee) 0x8e22c` | `ret(0x8e20c)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8cb44 | `[0x282300] != 0x0` | `[0x282300] == 0x0` |
| `bne` at 0x8cb54 | `[0x282304] != 0x0` | `[0x282304] == 0x0` |
| `blt` at 0x8cb70 | `[sp-24] < 0x0 (signed, 32-bit)` | `[sp-24] >= 0x0 (signed, 32-bit)` |
| `bne` at 0x8cb84 | `? != 0x0` | `? == 0x0` |
| `bne` at 0x8cb9c | `? != 0x5` | `? == 0x5` |
| `bne` at 0x8cbdc | `[0x2822e8] != 0x0` | `[0x2822e8] == 0x0` |
| `blt` at 0x8cbf0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8cc28 | `[sp-36] == 0x2` | `[sp-36] != 0x2` |
| `bgt` at 0x8cc34 | `[sp-36] > 0x2 (signed, 32-bit)` | `[sp-36] <= 0x2 (signed, 32-bit)` |
| `beq` at 0x8cc40 | `[sp-36] == 0x1` | `[sp-36] != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8ccc8 | `r0` | `[sp-32]` |

## `L1FRU_2m_Deadlock`

- `L1FRU_2m_Deadlock`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8ccd4 to 0x8cda0.
  Established. Entry r0 and r1 are saved as words at `sp -0x14` and `sp -0x18` (`str` at 0x8cce4,
  `str` at 0x8cce8). The selector takes its call arm for `[sp-32] == 0x1` and falls through
  otherwise (`beq` at 0x8cd3c). That call reaches `0x8e7d0` with unknown, `[sp-20]`, `[sp-24]`,
  unknown (`bl` at 0x8cd7c). The r0 return is `0x0`, but the deadlock-handling role and callee
  effects are unproven (`ldmdb` at 0x8cd98).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8cce4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8cce8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8ccf0 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x8ccf4 | load | 4 | `abs 0x8cd9c` | `-` |
| `ldr` at 0x8ccf8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8ccfc | load | 4 | `abs 0x2822f4` | `-` |
| `ldr` at 0x8cd0c | load | 4 | `abs 0x8cd9c` | `-` |
| `ldr` at 0x8cd10 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cd24 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cd2c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cd30 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8cd34 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8cd40 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8cd50 | load | 4 | `abs 0x8cd9c` | `-` |
| `ldr` at 0x8cd54 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8cd68 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cd70 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8cd74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cd78 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8cd80 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8cd88 | store | 4 | `sp -0x1c` | `[sp-28]+1` |
| `ldmdb` at 0x8cd98 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8cd98 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8cd98 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cd7c | `(unknown callee) 0x8e7d0` | `?` | `[sp-20]` | `[sp-24]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x8cd04 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8cd3c | `[sp-32] == 0x1` | `[sp-32] != 0x1` |
| `blt` at 0x8cd48 | `[sp-32] < 0x1 (signed, 32-bit)` | `[sp-32] >= 0x1 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8cd98 | `r0` | `0x0` |

## `L1FRU_2m_Dump`

- `L1FRU_2m_Dump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8cda0 to 0x8cdcc.
  Established. Two explicit word stores preserve entry r0 and r1 at `sp -0x14` and `sp -0x18` (`str`
  at 0x8cdb0, `str` at 0x8cdb4). The only call targets `0xa8200` with entry r0, `0x0`, entry r1,
  entry r3 (`bl` at 0x8cdc4); its result supplies r0 at the return (`ldmdb` at 0x8cdc8). No
  conditional branches are reported, and dump formatting is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8cdb0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8cdb4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x8cdb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cdc0 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x8cdc8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8cdc8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8cdc8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cdc4 | `(unknown callee) 0xa8200` | `r0` | `0x0` | `r1` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8cdc8 | `r0` | `ret(0xa8200)` |

## `L1FRU_2m_PortStatus`

- `L1FRU_2m_PortStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e1f0 to 0x8e20c.
  Established. A literal word is loaded from absolute `0x8e208` before the only call (`ldr` at
  0x8e1fc). The call reaches `0x3f818` with `0x282300`, entry r1, entry r2, entry r3, and its result
  supplies r0 (`bl` at 0x8e200, `ldmdb` at 0x8e204). No explicit stores or conditional branches are
  reported. The port-status representation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8e1fc | load | 4 | `abs 0x8e208` | `-` |
| `ldmdb` at 0x8e204 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e204 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e204 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e200 | `(unknown callee) 0x3f818` | `0x282300` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e204 | `r0` | `ret(0x3f818)` |

## `L1FRU_2m_EnvStatus`

- `L1FRU_2m_EnvStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e20c to 0x8e22c.
  Established. The call to `0x6b2a0` carries `0x282304`, `0x1`, entry r2, entry r3 (`bl` at
  0x8e220); its result reaches the r0 return as `ret(0x6b2a0)` (`ldmdb` at 0x8e224). The sole
  non-return explicit memory operand is a word load from literal address `0x8e228` (`ldr` at
  0x8e218). Environmental units and the destination object are not identified.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8e218 | load | 4 | `abs 0x8e228` | `-` |
| `ldmdb` at 0x8e224 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e224 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e224 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e220 | `(unknown callee) 0x6b2a0` | `0x282304` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e224 | `r0` | `ret(0x6b2a0)` |

## `L1FRU_2m_MgnStatus`

- `L1FRU_2m_MgnStatus`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e22c to 0x8e24c.
  Established. A word load reads the literal at absolute `0x8e248` (`ldr` at 0x8e238). The only call
  has target `0x7cb00` and candidates `0x282308`, `0x1`, entry r2, entry r3 (`bl` at 0x8e240). The
  returned r0 value is `ret(0x7cb00)` (`ldmdb` at 0x8e244). No explicit stores are reported, and
  margin-status interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0x8e238 | load | 4 | `abs 0x8e248` | `-` |
| `ldmdb` at 0x8e244 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e244 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e244 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e240 | `(unknown callee) 0x7cb00` | `0x282308` | `0x1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e244 | `r0` | `ret(0x7cb00)` |

## `L1FRU_2m_Attach`

- `L1FRU_2m_Attach`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e050 to 0x8e1f0.
  Established. The first selector takes its bypass for `[sp-20] != 0x1` and falls through on
  equality (`bne` at 0x8e09c); its equality arm calls `0xa5aec` with `[sp-48]`, `[sp-40]`,
  `[sp-44]`, `[0x2822e8]` (`bl` at 0x8e124). The other selector takes its bypass for `[sp-20] !=
  0x2`; its equality arm calls `0xa6814` with the same four printed candidates (`bne` at 0x8e15c,
  `bl` at 0x8e1b4). Both arms store word `[sp-24]` at `sp -0x34`, which supplies the reported return
  (`str` at 0x8e14c, `str` at 0x8e1d4, `ldmdb` at 0x8e1e0). The attachment format and object roles
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e060 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8e064 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8e068 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x8e074 | store | 4 | `sp -0x28` | `ret(0x1c79c)` |
| `str` at 0x8e080 | store | 4 | `sp -0x2c` | `ret(0x1c79c)` |
| `str` at 0x8e090 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0x8e094 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e0a0 | load | 4 | `abs 0x8e1e4` | `-` |
| `ldr` at 0x8e0a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e0ac | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0b0 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0b4 | load | 4 | `abs 0x2822ec` | `-` |
| `str` at 0x8e0b8 | store | 4 | `sp -0x58` | `[0x2822ec]` |
| `ldr` at 0x8e0bc | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0c0 | load | 4 | `abs 0x2822f0` | `-` |
| `str` at 0x8e0c4 | store | 4 | `sp -0x54` | `[0x2822f0]` |
| `ldr` at 0x8e0c8 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0cc | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8e0d0 | store | 4 | `sp -0x50` | `[0x2822f4]` |
| `ldr` at 0x8e0d4 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0d8 | load | 4 | `abs 0x2822f8` | `-` |
| `str` at 0x8e0dc | store | 4 | `sp -0x4c` | `[0x2822f8]` |
| `ldr` at 0x8e0e0 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0e4 | load | 4 | `abs 0x282300` | `-` |
| `str` at 0x8e0e8 | store | 4 | `sp -0x48` | `[0x282300]` |
| `ldr` at 0x8e0ec | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0f0 | load | 4 | `abs 0x282304` | `-` |
| `str` at 0x8e0f4 | store | 4 | `sp -0x44` | `[0x282304]` |
| `ldr` at 0x8e0f8 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e0fc | load | 4 | `abs 0x282308` | `-` |
| `str` at 0x8e100 | store | 4 | `sp -0x40` | `[0x282308]` |
| `str` at 0x8e108 | store | 4 | `sp -0x3c` | `sp-32` |
| `str` at 0x8e110 | store | 4 | `sp -0x38` | `sp-36` |
| `ldr` at 0x8e114 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e118 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8e11c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8e120 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8e12c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8e130 | store | 4 | `sp -0x58` | `[sp-36]` |
| `ldr` at 0x8e13c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e140 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e148 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e14c | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0x8e154 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e160 | load | 4 | `abs 0x8e1ec` | `-` |
| `ldr` at 0x8e164 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e16c | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e170 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e174 | load | 4 | `abs 0x2822ec` | `-` |
| `str` at 0x8e178 | store | 4 | `sp -0x58` | `[0x2822ec]` |
| `ldr` at 0x8e17c | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e180 | load | 4 | `abs 0x2822f0` | `-` |
| `str` at 0x8e184 | store | 4 | `sp -0x54` | `[0x2822f0]` |
| `ldr` at 0x8e188 | load | 4 | `abs 0x8e1e8` | `-` |
| `ldr` at 0x8e18c | load | 4 | `abs 0x2822fc` | `-` |
| `str` at 0x8e190 | store | 4 | `sp -0x50` | `[0x2822fc]` |
| `str` at 0x8e198 | store | 4 | `sp -0x4c` | `sp-32` |
| `str` at 0x8e1a0 | store | 4 | `sp -0x48` | `sp-36` |
| `ldr` at 0x8e1a4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x8e1a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8e1ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8e1b0 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8e1c0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8e1c4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8e1c8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e1d0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e1d4 | store | 4 | `sp -0x34` | `[sp-24]` |
| `ldr` at 0x8e1dc | load | 4 | `sp -0x34` | `-` |
| `ldmdb` at 0x8e1e0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e1e0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e1e0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e070 | `(unknown callee) 0x1c79c` | `0x1` | `r1` | `r2` | `r3` |
| `bl` at 0x8e07c | `(unknown callee) 0x1c79c` | `0x2` | `?` | `?` | `?` |
| `bl` at 0x8e084 | `(unknown callee) 0x2b368` | `ret(0x1c79c)` | `?` | `?` | `?` |
| `bl` at 0x8e0a8 | `(unknown callee) 0x1dba4` | `0xfb5ac` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0x8e124 | `(unknown callee) 0xa5aec` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2822e8]` |
| `bl` at 0x8e144 | `(unknown callee) 0x54570` | `0x3` | `sp-24` | `[sp-28]` | `[sp-32]` |
| `bl` at 0x8e168 | `(unknown callee) 0x1dba4` | `0xfb5b8` | `[sp-24]` | `?` | `[sp-20]` |
| `bl` at 0x8e1b4 | `(unknown callee) 0xa6814` | `[sp-48]` | `[sp-40]` | `[sp-44]` | `[0x2822e8]` |
| `bl` at 0x8e1cc | `(unknown callee) 0x542d4` | `sp-24` | `[sp-28]` | `[sp-32]` | `[sp-36]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8e09c | `[sp-20] != 0x1` | `[sp-20] == 0x1` |
| `bne` at 0x8e15c | `[sp-20] != 0x2` | `[sp-20] == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e1e0 | `r0` | `[sp-52]` |

## `L1FRUSHUBCapture`

- `L1FRUSHUBCapture`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8e24c to 0x8e524.
  Established. The first branch takes the continuation for `[r0+24] != 0x0` and falls through on
  zero to a word store of `0x1` at absolute `0x2822e8` (`bne` at 0x8e26c, `str` at 0x8e278). A later
  call to `0xc97a8` has unknown, unknown, `sp-48`, `sp-52` (`bl` at 0x8e3b4). Eight byte-store
  targets are unresolved: six have unknown values, while the stores at `0x8e450` and `0x8e4cc` write
  `([sp-48]:1 & 0xff)` and `([sp-52]:1 & 0xff)`, respectively (`strb` at 0x8e3f4, `strb` at 0x8e414,
  `strb` at 0x8e434, `strb` at 0x8e450, `strb` at 0x8e470, `strb` at 0x8e490, `strb` at 0x8e4b0,
  `strb` at 0x8e4cc). Another arm stores word `0x4` at `0x2822e8` (`str` at 0x8e4dc). The captured
  register layout and physical effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8e25c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8e260 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e264 | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x8e270 | load | 4 | `abs 0x8e51c` | `-` |
| `str` at 0x8e278 | store | 4 | `abs 0x2822e8` | `0x1` |
| `ldr` at 0x8e284 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e288 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e28c | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x8e294 | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x8e2a8 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8e2ac | load | 4 | `abs 0x8e51c` | `-` |
| `ldr` at 0x8e2b0 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8e2bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e2c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e2c4 | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x8e2d8 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x8e2dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e2e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e2f4 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0x8e2fc | load | 4 | `(unknown)` | `-` |
| `str` at 0x8e300 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x8e304 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e308 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e31c | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0x8e324 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8e32c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x8e330 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8e33c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e340 | load | 4 | `[sp-20] +0x4` | `-` |
| `str` at 0x8e34c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x8e350 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8e354 | store | 4 | `sp -0x40` | `?` |
| `str` at 0x8e35c | store | 4 | `sp -0x3c` | `sp-48` |
| `str` at 0x8e364 | store | 4 | `sp -0x38` | `sp-52` |
| `str` at 0x8e38c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8e394 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e398 | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x8e3ac | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8e3c8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8e3cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8e3d8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e3dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e3e4 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e3ec | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8e3f4 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e3f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e3fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e404 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e40c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8e414 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e418 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e424 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e42c | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8e434 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e43c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e444 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0x8e44c | load | 1 | `sp -0x30` | `-` |
| `strb` at 0x8e450 | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0x8e454 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e458 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e460 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e468 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8e470 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e474 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e478 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e480 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e488 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8e490 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e494 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e4a0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8e4a8 | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8e4b0 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8e4b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e4b8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e4c0 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0x8e4c8 | load | 1 | `sp -0x34` | `-` |
| `strb` at 0x8e4cc | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0x8e4d4 | load | 4 | `abs 0x8e51c` | `-` |
| `str` at 0x8e4dc | store | 4 | `abs 0x2822e8` | `0x4` |
| `ldr` at 0x8e4e0 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8e4e4 | store | 4 | `sp -0x44` | `[sp-44]` |
| `ldr` at 0x8e4e8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8e4ec | store | 4 | `sp -0x40` | `[sp-36]` |
| `ldr` at 0x8e4f0 | load | 4 | `abs 0x8e520` | `-` |
| `ldr` at 0x8e4f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8e4f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8e4fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8e504 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8e50c | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x8e518 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e518 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e518 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8e280 | `(unknown callee) 0xc2c9c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0x8e2a0 | `(unknown callee) 0x18c20` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0x8e378 | `(unknown callee) 0x4faf4` | `?` | `?` | `?` | `?` |
| `bl` at 0x8e3b4 | `(unknown callee) 0xc97a8` | `?` | `?` | `sp-48` | `sp-52` |
| `bl` at 0x8e500 | `(unknown callee) 0x1dba4` | `0xfb5c8` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0x8e514 | `(unknown callee) 0xc2d08` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8e26c | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0x8e2b8 | `[0x2822e8] != 0x0` | `[0x2822e8] == 0x0` |
| `blt` at 0x8e2cc | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x8e338 | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `beq` at 0x8e3d4 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e518 | `r0` | `?` |

## `L1FRUSHUBInit`

- `L1FRUSHUBInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f12c to 0x8f32c.
  Established. The pointer test takes its continuation for `[0x2822f8] != 0x0` and falls through on
  zero (`bne` at 0x8f158). Stores at unknown indexed targets include words `0x1`, `[0x1156f8]`,
  `0x1002b8`, `[0x1156fc]`, and `0x1037c8` (`str` at 0x8f1a0, `str` at 0x8f1f4, `str` at 0x8f21c,
  `str` at 0x8f248, `str` at 0x8f270). The call to `0x18920` carries `[sp-28]`, `0x2822e0`, unknown,
  `[0x2822f8]` (`bl` at 0x8f2a8). An absolute word store later writes `[0x2822f4]+1` at `0x2822f4`
  (`str` at 0x8f2f4). The initialized-record meaning and r0 return remain unknown (`ldmdb` at
  0x8f314).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f13c | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x8f140 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f144 | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8f148 | store | 4 | `sp -0x24` | `[0x2822f4]` |
| `ldr` at 0x8f14c | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f150 | load | 4 | `abs 0x2822f8` | `-` |
| `str` at 0x8f164 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8f168 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8f16c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f17c | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f180 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f194 | load | 4 | `abs 0x2822f8` | `-` |
| `str` at 0x8f1a0 | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0x8f1a4 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f1a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f1bc | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f1c4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8f1c8 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x8f1cc | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f1d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f1e4 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f1ec | load | 4 | `abs 0x8f31c` | `-` |
| `ldr` at 0x8f1f0 | load | 4 | `abs 0x1156f8` | `-` |
| `str` at 0x8f1f4 | store | 4 | `(unknown)` | `[0x1156f8]` |
| `ldr` at 0x8f1f8 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f1fc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f210 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f218 | load | 4 | `abs 0x8f320` | `-` |
| `str` at 0x8f21c | store | 4 | `(unknown)` | `0x1002b8` |
| `ldr` at 0x8f220 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f224 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f238 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f240 | load | 4 | `abs 0x8f324` | `-` |
| `ldr` at 0x8f244 | load | 4 | `abs 0x1156fc` | `-` |
| `str` at 0x8f248 | store | 4 | `(unknown)` | `[0x1156fc]` |
| `ldr` at 0x8f24c | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f250 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f264 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f26c | load | 4 | `abs 0x8f328` | `-` |
| `str` at 0x8f270 | store | 4 | `(unknown)` | `0x1037c8` |
| `ldr` at 0x8f274 | load | 4 | `abs 0x8f31c` | `-` |
| `ldr` at 0x8f278 | load | 4 | `abs 0x1156f8` | `-` |
| `str` at 0x8f280 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f284 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f288 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f29c | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f2a4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8f2b0 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x8f2b4 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f2b8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f2cc | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f2d4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f2dc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f2e4 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f2e8 | load | 4 | `abs 0x8f318` | `-` |
| `ldr` at 0x8f2ec | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8f2f4 | store | 4 | `abs 0x2822f4` | `[0x2822f4]+1` |
| `ldr` at 0x8f2f8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f300 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x8f304 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8f30c | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0x8f314 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8f314 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f314 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f314 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8f2a8 | `(unknown callee) 0x18920` | `[sp-28]` | `0x2822e0` | `?` | `[0x2822f8]` |
| `bl` at 0x8f2e0 | `(unknown callee) 0x18c20` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8f158 | `[0x2822f8] != 0x0` | `[0x2822f8] == 0x0` |
| `blt` at 0x8f174 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f314 | `r0` | `?` |

## `L1FRUNL4RCapture`

- `L1FRUNL4RCapture`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f354 to 0x8f5c8.
  Established. The entry pointer read addresses `r0 +0x18`, and its nonzero result takes the
  continuation and zero falls through to store word `0x1` at absolute `0x2822e8` (`ldr` at 0x8f36c,
  `bne` at 0x8f374, `str` at 0x8f380). The call to `0xc97a8` carries unknown, unknown, `sp-48`,
  `sp-52` (`bl` at 0x8f458). Later byte stores include `([sp-48]:1 & 0xff)` and `([sp-52]:1 & 0xff)`
  at unknown targets (`strb` at 0x8f4f4, `strb` at 0x8f570). A separate arm writes word `0x4` at
  `0x2822e8` (`str` at 0x8f580). Capture destination layout and the joined r0 return are unproven
  (`ldmdb` at 0x8f5bc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f364 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x8f368 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f36c | load | 4 | `r0 +0x18` | `-` |
| `ldr` at 0x8f378 | load | 4 | `abs 0x8f5c0` | `-` |
| `str` at 0x8f380 | store | 4 | `abs 0x2822e8` | `0x1` |
| `ldr` at 0x8f38c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f390 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f394 | load | 4 | `[sp-20] +0x8` | `-` |
| `ldr` at 0x8f39c | load | 4 | `[sp-20] +0x18` | `-` |
| `str` at 0x8f3b0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8f3b4 | load | 4 | `abs 0x8f5c0` | `-` |
| `ldr` at 0x8f3b8 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8f3c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f3c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f3cc | load | 4 | `[sp-20] +0x8` | `-` |
| `str` at 0x8f3e0 | store | 4 | `sp -0x24` | `0x1` |
| `ldr` at 0x8f3e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f3e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f3fc | load | 4 | `[sp-20] +0xc` | `-` |
| `ldr` at 0x8f404 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8f408 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x8f40c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f410 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f424 | load | 4 | `[sp-20] +0xc` | `-` |
| `ldrb` at 0x8f42c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8f434 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x8f438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f43c | load | 4 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x8f450 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8f46c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0x8f470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f47c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f488 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f490 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8f498 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f49c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f4a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f4a8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f4b0 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8f4b8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f4bc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f4c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f4c8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f4d0 | load | 4 | `sp -0x30` | `-` |
| `strb` at 0x8f4d8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f4dc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f4e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f4e8 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0x8f4f0 | load | 1 | `sp -0x30` | `-` |
| `strb` at 0x8f4f4 | store | 1 | `(unknown)` | `([sp-48]:1 & 0xff)` |
| `ldr` at 0x8f4f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f4fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f504 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f50c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8f514 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f518 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f51c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f524 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f52c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8f534 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f538 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f53c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f544 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldr` at 0x8f54c | load | 4 | `sp -0x34` | `-` |
| `strb` at 0x8f554 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x8f558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f55c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f564 | load | 4 | `[sp-20] +0x18` | `-` |
| `ldrb` at 0x8f56c | load | 1 | `sp -0x34` | `-` |
| `strb` at 0x8f570 | store | 1 | `(unknown)` | `([sp-52]:1 & 0xff)` |
| `ldr` at 0x8f578 | load | 4 | `abs 0x8f5c0` | `-` |
| `str` at 0x8f580 | store | 4 | `abs 0x2822e8` | `0x4` |
| `ldr` at 0x8f584 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8f588 | store | 4 | `sp -0x3c` | `[sp-44]` |
| `ldr` at 0x8f58c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f590 | store | 4 | `sp -0x38` | `[sp-36]` |
| `ldr` at 0x8f594 | load | 4 | `abs 0x8f5c4` | `-` |
| `ldr` at 0x8f598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8f59c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f5a0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8f5a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8f5b0 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldmdb` at 0x8f5bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f5bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f5bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8f388 | `(unknown callee) 0xc2c9c` | `r0` | `r1` | `r2` | `[r0+24]` |
| `bl` at 0x8f3a8 | `(unknown callee) 0x18c20` | `[[sp-20]+24]` | `0x0` | `?` | `?` |
| `bl` at 0x8f458 | `(unknown callee) 0xc97a8` | `?` | `?` | `sp-48` | `sp-52` |
| `bl` at 0x8f5a4 | `(unknown callee) 0x1dba4` | `0xfbcd8` | `[sp-20]` | `[sp-24]` | `[sp-40]` |
| `bl` at 0x8f5b8 | `(unknown callee) 0xc2d08` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8f374 | `[r0+24] != 0x0` | `[r0+24] == 0x0` |
| `bne` at 0x8f3c0 | `[0x2822e8] != 0x0` | `[0x2822e8] == 0x0` |
| `blt` at 0x8f3d4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8f478 | `[sp-36] == 0x0` | `[sp-36] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f5bc | `r0` | `?` |

## `L1FRUNL4RInit`

- `L1FRUNL4RInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8f5c8 to 0x8f7c8.
  Established. The first indexed word store writes `0x2` at an unknown effective target (`str` at
  0x8f63c). Other unknown-target word stores include `[0x115700]`, `0x105ac0`, `[0x115704]`, and
  `0x106668` (`str` at 0x8f690, `str` at 0x8f6b8, `str` at 0x8f6e4, `str` at 0x8f70c). The call to
  `0x18c20` has unknown, `0x0`, `[sp-28]`, unknown (`bl` at 0x8f77c). An absolute word store writes
  `[0x2822f4]+1` at `0x2822f4` (`str` at 0x8f790). Hardware initialization and the r0 return remain
  unknown (`ldmdb` at 0x8f7b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8f5d8 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0x8f5dc | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f5e0 | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8f5e4 | store | 4 | `sp -0x24` | `[0x2822f4]` |
| `ldr` at 0x8f5e8 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f5ec | load | 4 | `abs 0x2822f8` | `-` |
| `str` at 0x8f600 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8f604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8f608 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8f618 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f61c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f630 | load | 4 | `abs 0x2822f8` | `-` |
| `str` at 0x8f63c | store | 4 | `(unknown)` | `0x2` |
| `ldr` at 0x8f640 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f644 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f658 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f660 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8f664 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0x8f668 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f66c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f680 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f688 | load | 4 | `abs 0x8f7b8` | `-` |
| `ldr` at 0x8f68c | load | 4 | `abs 0x115700` | `-` |
| `str` at 0x8f690 | store | 4 | `(unknown)` | `[0x115700]` |
| `ldr` at 0x8f694 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f698 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f6ac | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f6b4 | load | 4 | `abs 0x8f7bc` | `-` |
| `str` at 0x8f6b8 | store | 4 | `(unknown)` | `0x105ac0` |
| `ldr` at 0x8f6bc | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f6c0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f6d4 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f6dc | load | 4 | `abs 0x8f7c0` | `-` |
| `ldr` at 0x8f6e0 | load | 4 | `abs 0x115704` | `-` |
| `str` at 0x8f6e4 | store | 4 | `(unknown)` | `[0x115704]` |
| `ldr` at 0x8f6e8 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f6ec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f700 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f708 | load | 4 | `abs 0x8f7c4` | `-` |
| `str` at 0x8f70c | store | 4 | `(unknown)` | `0x106668` |
| `ldr` at 0x8f710 | load | 4 | `abs 0x8f7b8` | `-` |
| `ldr` at 0x8f714 | load | 4 | `abs 0x115700` | `-` |
| `str` at 0x8f71c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8f720 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f724 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f738 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f740 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8f74c | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0x8f750 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f754 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x8f768 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8f770 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8f778 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8f780 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f784 | load | 4 | `abs 0x8f7b4` | `-` |
| `ldr` at 0x8f788 | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8f790 | store | 4 | `abs 0x2822f4` | `[0x2822f4]+1` |
| `ldr` at 0x8f794 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x8f79c | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0x8f7a0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8f7a8 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldmdb` at 0x8f7b0 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0x8f7b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8f7b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8f7b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8f744 | `(unknown callee) 0x18920` | `[sp-28]` | `0x2822e0` | `?` | `[0x2822f8]` |
| `bl` at 0x8f77c | `(unknown callee) 0x18c20` | `?` | `0x0` | `[sp-28]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x8f5f4 | `[0x2822f8] != 0x0` | `[0x2822f8] == 0x0` |
| `blt` at 0x8f610 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8f7b0 | `r0` | `?` |

## `L1FRU_2m_Examine`

- `L1FRU_2m_Examine`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8d9a8 to 0x8e050.
  Established. The call to `0xa7794` carries `[sp-180]`, `sp-36`, `sp-40`, `sp-44` (`bl` at
  0x8dbb4). Two signed comparisons take their bypasses for `[sp-40] >= [sp-72]` and `[sp-40] <=
  [sp-76]`, respectively, and fall through for the opposite relation (`bge` at 0x8dc40, `ble` at
  0x8dc58). Near the end, a word store puts `[0x11b2dc]` at `[sp-24] +0x0`, followed by a word clear
  of absolute `0x11b2dc` (`str` at 0x8dff4, `str` at 0x8e000). The final call to `0xa4510` carries
  `0x2822fc`, unknown, `0x11b2dc`, `0x0` and supplies r0 at return (`bl` at 0x8e008, `ldmdb` at
  0x8e00c). Event interpretation and hardware diagnosis remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8d9b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x8d9bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x8d9c4 | store | 4 | `sp -0x44` | `0x0` |
| `str` at 0x8d9cc | store | 4 | `sp -0x48` | `?` |
| `str` at 0x8d9d4 | store | 4 | `sp -0x4c` | `0x0` |
| `str` at 0x8d9dc | store | 4 | `sp -0x50` | `0x0` |
| `str` at 0x8d9e4 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0x8d9f4 | store | 4 | `sp -0xac` | `ret(0x5e7a8)` |
| `ldr` at 0x8d9fc | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da00 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8da10 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da14 | load | 4 | `abs 0x2822f0` | `-` |
| `ldr` at 0x8da20 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x8da28 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0x8da30 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da34 | load | 4 | `abs 0x2822f0` | `-` |
| `ldr` at 0x8da40 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8da48 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x8da4c | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da50 | load | 4 | `abs 0x2822ec` | `-` |
| `str` at 0x8da54 | store | 4 | `sp -0x48` | `[0x2822ec]` |
| `ldr` at 0x8da58 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da5c | load | 4 | `abs 0x2822ec` | `-` |
| `str` at 0x8da60 | store | 4 | `sp -0x4c` | `[0x2822ec]` |
| `ldr` at 0x8da64 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8da6c | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x8da74 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8da78 | load | 4 | `abs 0x2822e8` | `-` |
| `ldr` at 0x8da88 | load | 4 | `sp -0xac` | `-` |
| `ldr` at 0x8da8c | load | 4 | `sp -0xac` | `-` |
| `str` at 0x8da90 | store | 4 | `sp -0xcc` | `ret(0xa7c14)` |
| `ldr` at 0x8da98 | load | 4 | `abs 0x8e014` | `-` |
| `ldr` at 0x8da9c | load | 4 | `[sp-172] +0x8` | `-` |
| `ldr` at 0x8daa0 | load | 4 | `[sp-172] +0xc` | `-` |
| `str` at 0x8dac0 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x8dac4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dad0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8dad4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8dad8 | load | 4 | `[sp-20] +0x414` | `-` |
| `ldr` at 0x8dae8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8daec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x8daf0 | load | 4 | `abs 0x8e018` | `-` |
| `ldr` at 0x8db00 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8db04 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x8db08 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x8db0c | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8db1c | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x8db20 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8db34 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x8db38 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8db48 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8db4c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8db50 | store | 4 | `sp -0xb4` | `?` |
| `ldr` at 0x8db58 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x8db64 | store | 4 | `sp -0xb4` | `ret(0x9ca84)` |
| `str` at 0x8db78 | store | 4 | `sp -0xcc` | `sp-48` |
| `str` at 0x8db80 | store | 4 | `sp -0xc8` | `sp-52` |
| `str` at 0x8db88 | store | 4 | `sp -0xc4` | `sp-56` |
| `str` at 0x8db90 | store | 4 | `sp -0xc0` | `sp-60` |
| `str` at 0x8db98 | store | 4 | `sp -0xbc` | `sp-64` |
| `str` at 0x8dba0 | store | 4 | `sp -0xb8` | `sp-168` |
| `ldr` at 0x8dba4 | load | 4 | `sp -0xb4` | `-` |
| `ldr` at 0x8dbb8 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8dbc4 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8dbd4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dbd8 | load | 4 | `[sp-28] +0x1c` | `-` |
| `ldr` at 0x8dbe8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8dbec | load | 4 | `[sp-28] +0x1c` | `-` |
| `str` at 0x8dbf8 | store | 4 | `sp -0xcc` | `ret(0xa7c14)` |
| `ldr` at 0x8dbfc | load | 4 | `abs 0x8e014` | `-` |
| `ldr` at 0x8dc18 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x8dc28 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8dc30 | store | 4 | `sp -0x44` | `[sp-68]+1` |
| `ldr` at 0x8dc34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8dc38 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8dc44 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8dc48 | store | 4 | `sp -0x48` | `[sp-40]` |
| `ldr` at 0x8dc4c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x8dc50 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8dc5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8dc60 | store | 4 | `sp -0x4c` | `[sp-40]` |
| `ldr` at 0x8dc64 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8dc70 | load | 4 | `sp -0x50` | `-` |
| `str` at 0x8dc78 | store | 4 | `sp -0x50` | `[sp-80]+1` |
| `ldr` at 0x8dc80 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x8dc8c | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8dc94 | store | 4 | `sp -0x54` | `[sp-84]+1` |
| `ldr` at 0x8dc98 | load | 4 | `abs 0x8e01c` | `-` |
| `ldr` at 0x8dc9c | load | 4 | `sp -0xa8` | `-` |
| `ldr` at 0x8dca4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8dcac | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8dcb4 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8dcb8 | load | 4 | `abs 0x2822fc` | `-` |
| `ldr` at 0x8dcc0 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8dcc8 | load | 4 | `abs 0x8e020` | `-` |
| `ldr` at 0x8dcd0 | load | 4 | `abs 0x2822fc` | `-` |
| `ldr` at 0x8dcd8 | load | 4 | `abs 0x8e010` | `-` |
| `ldr` at 0x8dce0 | load | 4 | `abs 0x8e020` | `-` |
| `ldr` at 0x8dce8 | load | 4 | `abs 0x2822fc` | `-` |
| `ldr` at 0x8dcf0 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8dd04 | store | 4 | `sp -0xb0` | `ret(0x189c8)` |
| `ldr` at 0x8dd08 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x8dd14 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0x8dd24 | load | 4 | `abs 0x8e024` | `-` |
| `ldr` at 0x8dd34 | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x8dd44 | load | 4 | `abs 0x8e028` | `-` |
| `ldr` at 0x8dd58 | load | 4 | `abs 0x8e02c` | `-` |
| `ldr` at 0x8dd80 | load | 4 | `abs 0x8e030` | `-` |
| `ldr` at 0x8ddbc | load | 4 | `abs 0x8e034` | `-` |
| `ldr` at 0x8dde0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8dde4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8ddec | load | 4 | `abs 0x8e038` | `-` |
| `ldr` at 0x8de04 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8de08 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8de10 | load | 4 | `abs 0x8e03c` | `-` |
| `ldr` at 0x8de28 | load | 4 | `abs 0x8e03c` | `-` |
| `ldr` at 0x8de54 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8de58 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8de60 | load | 4 | `abs 0x8e03c` | `-` |
| `str` at 0x8de8c | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x8de90 | load | 4 | `abs 0x8e040` | `-` |
| `ldr` at 0x8deb4 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8deb8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8dec4 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8dec8 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8dedc | load | 4 | `sp -0xb0` | `-` |
| `str` at 0x8dee0 | store | 4 | `sp -0xcc` | `[sp-176]` |
| `ldr` at 0x8dee8 | load | 4 | `abs 0x8e044` | `-` |
| `ldr` at 0x8deec | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x8def0 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8df14 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8df18 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8df20 | load | 4 | `abs 0x8e038` | `-` |
| `ldr` at 0x8df38 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8df3c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8df44 | load | 4 | `abs 0x8e03c` | `-` |
| `ldr` at 0x8df5c | load | 4 | `abs 0x8e03c` | `-` |
| `ldr` at 0x8df88 | load | 4 | `sp -0xb0` | `-` |
| `ldr` at 0x8df8c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8df94 | load | 4 | `abs 0x8e03c` | `-` |
| `str` at 0x8dfc0 | store | 4 | `sp -0xcc` | `?` |
| `ldr` at 0x8dfc4 | load | 4 | `abs 0x8e048` | `-` |
| `ldr` at 0x8dfe8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8dfec | load | 4 | `abs 0x8e04c` | `-` |
| `ldr` at 0x8dff0 | load | 4 | `abs 0x11b2dc` | `-` |
| `str` at 0x8dff4 | store | 4 | `[sp-24] +0x0` | `[0x11b2dc]` |
| `ldr` at 0x8dff8 | load | 4 | `abs 0x8e04c` | `-` |
| `str` at 0x8e000 | store | 4 | `abs 0x11b2dc` | `0x0` |
| `ldr` at 0x8e004 | load | 4 | `abs 0x8e01c` | `-` |
| `ldmdb` at 0x8e00c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8e00c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8e00c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8d9ec | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `0x0` |
| `bl` at 0x8d9f8 | `(unknown callee) 0x8d858` | `ret(0x5e7a8)` | `?` | `?` | `ret(0x5e7a8)` |
| `bl` at 0x8da7c | `(unknown callee) 0xa7c14` | `[0x2822e8]` | `?` | `?` | `0x2822e0` |
| `bl` at 0x8daa4 | `(unknown callee) 0x18c74` | `sp-164` | `0xfb470` | `[[sp-172]+8]` | `[[sp-172]+12]` |
| `bl` at 0x8dab8 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8db60 | `(unknown callee) 0x9ca84` | `?` | `0x1` | `0x418` | `?` |
| `bl` at 0x8dbb4 | `(unknown callee) 0xa7794` | `[sp-180]` | `sp-36` | `sp-40` | `sp-44` |
| `bl` at 0x8dbc8 | `(unknown callee) 0xa7c14` | `[sp-60]` | `?` | `?` | `[sp-60]` |
| `bl` at 0x8dc00 | `(unknown callee) 0x18c74` | `sp-164` | `0xfb470` | `?` | `?` |
| `bl` at 0x8dc14 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8dca0 | `(unknown callee) 0xa467c` | `0x2822fc` | `[sp-168]` | `[sp-40]` | `?` |
| `bl` at 0x8dcbc | `(unknown callee) 0xa487c` | `[0x2822fc]` | `?` | `?` | `0x2822e0` |
| `bl` at 0x8dcd4 | `(unknown callee) 0xa4b74` | `0x0` | `0x8d8a0` | `0x0` | `[0x2822fc]` |
| `bl` at 0x8dcec | `(unknown callee) 0xa4b74` | `0x2` | `0x8d8a0` | `0x0` | `[0x2822fc]` |
| `bl` at 0x8dd00 | `(unknown callee) 0x189c8` | `0x0` | `?` | `?` | `[sp-68]` |
| `bl` at 0x8dd2c | `(unknown callee) 0x8d8a0` | `0x0` | `0xfb488` | `?` | `[sp-84]` |
| `bl` at 0x8dd4c | `(unknown callee) 0x8d8a0` | `0x0` | `0xfb4c0` | `?` | `[sp-80]` |
| `bl` at 0x8dd60 | `(unknown callee) 0x8d8a0` | `0x0` | `0xfb4e8` | `?` | `?` |
| `bl` at 0x8dd6c | `(unknown callee) 0x187d0` | `sp-72` | `?` | `?` | `sp-72` |
| `bl` at 0x8dd88 | `(unknown callee) 0x18938` | `sp-164` | `0x3c` | `0xfb510` | `ret(0x187d0)` |
| `bl` at 0x8dd9c | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8dda8 | `(unknown callee) 0x187d0` | `sp-76` | `?` | `?` | `sp-76` |
| `bl` at 0x8ddc4 | `(unknown callee) 0x18938` | `sp-164` | `0x3c` | `0xfb530` | `ret(0x187d0)` |
| `bl` at 0x8ddd8 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8de9c | `(unknown callee) 0x18c74` | `sp-164` | `0xfb550` | `?` | `?` |
| `bl` at 0x8deb0 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8def4 | `(unknown callee) 0x18c74` | `sp-164` | `0xfb56c` | `[sp-72]` | `[sp-76]` |
| `bl` at 0x8df08 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8dfd0 | `(unknown callee) 0x18c74` | `sp-164` | `0xfb590` | `?` | `?` |
| `bl` at 0x8dfe4 | `(unknown callee) 0x8d8a0` | `0x0` | `sp-164` | `?` | `sp-164` |
| `bl` at 0x8e008 | `(unknown callee) 0xa4510` | `0x2822fc` | `?` | `0x11b2dc` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x8da08 | `[0x2822e8] == 0x0` | `[0x2822e8] != 0x0` |
| `bne` at 0x8da1c | `[0x2822f0] != 0x1` | `[0x2822f0] == 0x1` |
| `bne` at 0x8da3c | `[0x2822f0] != 0x2` | `[0x2822f0] == 0x2` |
| `beq` at 0x8dacc | `[sp-20] == 0x0` | `[sp-20] != 0x0` |
| `blt` at 0x8dae0 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8db14 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x8db2c | `? == 0x1` | `? != 0x1` |
| `bne` at 0x8db44 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x8dbc0 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `beq` at 0x8dc20 | `[sp-60] == 0x0` | `[sp-60] != 0x0` |
| `bge` at 0x8dc40 | `[sp-40] >= [sp-72] (signed, 32-bit)` | `[sp-40] < [sp-72] (signed, 32-bit)` |
| `ble` at 0x8dc58 | `[sp-40] <= [sp-76] (signed, 32-bit)` | `[sp-40] > [sp-76] (signed, 32-bit)` |
| `bne` at 0x8dc6c | `[sp-44] != 0x1` | `[sp-44] == 0x1` |
| `bne` at 0x8dc88 | `[sp-44] != 0x2` | `[sp-44] == 0x2` |
| `beq` at 0x8dcf8 | `[sp-68] == 0x0` | `[sp-68] != 0x0` |
| `beq` at 0x8dd10 | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `beq` at 0x8dd1c | `[sp-84] == 0x0` | `[sp-84] != 0x0` |
| `beq` at 0x8dd3c | `[sp-80] == 0x0` | `[sp-80] != 0x0` |
| `bgt` at 0x8dec0 | `[sp-76] > [sp-176] (signed, 32-bit)` | `[sp-76] <= [sp-176] (signed, 32-bit)` |
| `bgt` at 0x8ded0 | `[sp-72] > [sp-176] (signed, 32-bit)` | `[sp-72] <= [sp-176] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8e00c | `r0` | `ret(0xa4510)` |

## `L1FRU_2m_Events`

- `L1FRU_2m_Events`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x8cdcc to 0x8d858.
  Established. Two calls to `0xa1140` carry unknown, `0x2a`, `0x20`, unknown and unknown, `0x15`,
  `0x8`, unknown, respectively (`bl` at 0x8cfc4, `bl` at 0x8d02c). The unsigned test takes its
  bypass for `ret(0xa1484) > [sp-132]` and falls through for less-or-equal (`bhi` at 0x8d168). In
  the fall-through block, byte stores at `[sp-104] +0x0` through `+0x6` copy `([[sp-104]+1]:1 &
  0xff)` through `([[sp-104]+7]:1 & 0xff)`, respectively, and `[sp-104] +0x7` receives `0x0` (`strb`
  at 0x8d194, `strb` at 0x8d1b4, `strb` at 0x8d1d4, `strb` at 0x8d1f4, `strb` at 0x8d214, `strb` at
  0x8d234, `strb` at 0x8d254, `strb` at 0x8d268). Two later calls to `0xa46d8` use `0x2822fc`,
  `[sp-104]`, `[sp-28]`, `[sp-116]` (`bl` at 0x8d474, `bl` at 0x8d824). The event encoding and
  returned r0 value remain unknown (`ldmdb` at 0x8d848).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x8cde8 | store | 4 | `sp -0x98` | `ret(0x5e7a8)` |
| `ldr` at 0x8cdec | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8cdf0 | load | 4 | `ret(0x5e7a8) +0x8` | `-` |
| `str` at 0x8cdf4 | store | 4 | `sp -0x1c` | `[ret(0x5e7a8)+8]` |
| `ldr` at 0x8cdf8 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8cdfc | load | 4 | `ret(0x5e7a8) +0xc` | `-` |
| `str` at 0x8ce00 | store | 4 | `sp -0x74` | `[ret(0x5e7a8)+12]` |
| `ldr` at 0x8ce04 | load | 4 | `sp -0x98` | `-` |
| `ldr` at 0x8ce08 | load | 4 | `ret(0x5e7a8) +0x10` | `-` |
| `str` at 0x8ce0c | store | 4 | `sp -0x78` | `[ret(0x5e7a8)+16]` |
| `ldr` at 0x8ce10 | load | 4 | `abs 0x8d84c` | `-` |
| `strb` at 0x8ce1c | store | 1 | `sp -0x60` | `?` |
| `strb` at 0x8ce24 | store | 1 | `sp -0x5f` | `?` |
| `strb` at 0x8ce2c | store | 1 | `sp -0x5e` | `?` |
| `strb` at 0x8ce34 | store | 1 | `sp -0x5d` | `?` |
| `strb` at 0x8ce3c | store | 1 | `sp -0x5c` | `?` |
| `strb` at 0x8ce44 | store | 1 | `sp -0x5b` | `?` |
| `strb` at 0x8ce4c | store | 1 | `sp -0x5a` | `?` |
| `strb` at 0x8ce54 | store | 1 | `sp -0x59` | `?` |
| `ldr` at 0x8ce58 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8ce5c | load | 4 | `abs 0x2822f4` | `-` |
| `str` at 0x8ce60 | store | 4 | `sp -0x80` | `[0x2822f4]` |
| `str` at 0x8ce68 | store | 4 | `sp -0x7c` | `0x0` |
| `ldr` at 0x8ce6c | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8ce70 | load | 4 | `sp -0x80` | `-` |
| `ldr` at 0x8ce80 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8ce84 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8ce98 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cea0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cea4 | store | 4 | `sp -0x84` | `?` |
| `ldr` at 0x8cea8 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8ceac | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8cec0 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cec8 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cecc | store | 4 | `sp -0x88` | `?` |
| `ldr` at 0x8ced0 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8ced4 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8cee8 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cef0 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cef4 | store | 4 | `sp -0x8c` | `?` |
| `ldr` at 0x8cef8 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8cefc | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8cf10 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cf18 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cf1c | store | 4 | `sp -0x90` | `?` |
| `ldr` at 0x8cf20 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8cf24 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8cf38 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8cf40 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8cf44 | store | 4 | `sp -0x94` | `?` |
| `ldr` at 0x8cf48 | load | 4 | `abs 0x8d854` | `-` |
| `str` at 0x8cf4c | store | 4 | `sp -0x2c` | `0xffff` |
| `str` at 0x8cf54 | store | 4 | `sp -0x14` | `0x0` |
| `ldr` at 0x8cf58 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf5c | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x8cf6c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cf80 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x8cf88 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8cfa4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cfac | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8cfb4 | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x8cfb8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8cfcc | store | 4 | `sp -0x2c` | `ret(0xa1140)` |
| `ldr` at 0x8cfd4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8cfe8 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x8cff0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d00c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x8d014 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d01c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x8d020 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8d034 | store | 4 | `sp -0x2c` | `ret(0xa1140)` |
| `ldr` at 0x8d038 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x8d040 | store | 4 | `sp -0x14` | `[sp-20]+1` |
| `str` at 0x8d04c | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x8d050 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d054 | load | 4 | `sp -0x8c` | `-` |
| `ldr` at 0x8d064 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d06c | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d074 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d084 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d08c | store | 4 | `sp -0x64` | `?` |
| `ldr` at 0x8d090 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d098 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d0a0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d0ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d0b4 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d0bc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x8d0c8 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x8d0d8 | store | 4 | `sp -0x50` | `ret(0xa1140)` |
| `ldr` at 0x8d0dc | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x8d0e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d0f0 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x8d0f8 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x8d10c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d114 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x8d11c | load | 1 | `(unknown)` | `-` |
| `str` at 0x8d134 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0x8d138 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d13c | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0x8d140 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d144 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d148 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x8d14c | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d158 | store | 4 | `sp -0x4c` | `ret(0xa1484)` |
| `ldr` at 0x8d15c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8d160 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x8d16c | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8d174 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d17c | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x8d180 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d188 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d190 | load | 1 | `[sp-104] +0x1` | `-` |
| `strb` at 0x8d194 | store | 1 | `[sp-104] +0x0` | `([[sp-104]+1]:1 & 0xff)` |
| `ldr` at 0x8d19c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d1a8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d1b0 | load | 1 | `[sp-104] +0x2` | `-` |
| `strb` at 0x8d1b4 | store | 1 | `[sp-104] +0x1` | `([[sp-104]+2]:1 & 0xff)` |
| `ldr` at 0x8d1bc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d1c8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d1d0 | load | 1 | `[sp-104] +0x3` | `-` |
| `strb` at 0x8d1d4 | store | 1 | `[sp-104] +0x2` | `([[sp-104]+3]:1 & 0xff)` |
| `ldr` at 0x8d1dc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d1e8 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d1f0 | load | 1 | `[sp-104] +0x4` | `-` |
| `strb` at 0x8d1f4 | store | 1 | `[sp-104] +0x3` | `([[sp-104]+4]:1 & 0xff)` |
| `ldr` at 0x8d1fc | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d208 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d210 | load | 1 | `[sp-104] +0x5` | `-` |
| `strb` at 0x8d214 | store | 1 | `[sp-104] +0x4` | `([[sp-104]+5]:1 & 0xff)` |
| `ldr` at 0x8d21c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d228 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d230 | load | 1 | `[sp-104] +0x6` | `-` |
| `strb` at 0x8d234 | store | 1 | `[sp-104] +0x5` | `([[sp-104]+6]:1 & 0xff)` |
| `ldr` at 0x8d23c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d248 | load | 4 | `sp -0x68` | `-` |
| `ldrb` at 0x8d250 | load | 1 | `[sp-104] +0x7` | `-` |
| `strb` at 0x8d254 | store | 1 | `[sp-104] +0x6` | `([[sp-104]+7]:1 & 0xff)` |
| `ldr` at 0x8d25c | load | 4 | `sp -0x68` | `-` |
| `strb` at 0x8d268 | store | 1 | `[sp-104] +0x7` | `0x0` |
| `ldr` at 0x8d26c | load | 4 | `abs 0x8d854` | `-` |
| `str` at 0x8d270 | store | 4 | `sp -0x28` | `0xffff` |
| `str` at 0x8d278 | store | 4 | `sp -0x30` | `0x3` |
| `str` at 0x8d280 | store | 4 | `sp -0x34` | `0x7` |
| `str` at 0x8d288 | store | 4 | `sp -0x38` | `0x3` |
| `str` at 0x8d290 | store | 4 | `sp -0x3c` | `0x7f` |
| `str` at 0x8d298 | store | 4 | `sp -0x40` | `0x7f` |
| `str` at 0x8d2a0 | store | 4 | `sp -0x44` | `0x7f` |
| `str` at 0x8d2a8 | store | 4 | `sp -0x48` | `0x7f` |
| `ldr` at 0x8d2ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d2b4 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d2bc | load | 2 | `(unknown)` | `-` |
| `str` at 0x8d2d0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8d2d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d2dc | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d2e4 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d300 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x8d308 | load | 2 | `(unknown)` | `-` |
| `str` at 0x8d31c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x8d320 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d328 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d330 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d34c | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x8d354 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d358 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x8d35c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d364 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d36c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d370 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x8d374 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d37c | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x8d384 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8d390 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x8d394 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x8d398 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0x8d39c | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8d3a0 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8d3b4 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8d3bc | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d3c0 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x8d3c4 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8d3c8 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8d3dc | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8d3e4 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d3e8 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x8d3ec | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d3f0 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0x8d3f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8d3f8 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0x8d3fc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8d400 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0x8d404 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8d408 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0x8d40c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8d410 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0x8d414 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8d418 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0x8d41c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8d420 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0x8d424 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8d428 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0x8d42c | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8d430 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0x8d434 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8d438 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0x8d43c | load | 4 | `sp -0x48` | `-` |
| `str` at 0x8d440 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0x8d444 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8d448 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0x8d44c | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8d450 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0x8d454 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x8d458 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0x8d45c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x8d460 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0x8d464 | load | 4 | `abs 0x8d84c` | `-` |
| `ldr` at 0x8d468 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d46c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d470 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x8d47c | load | 4 | `sp -0x50` | `-` |
| `ldr` at 0x8d488 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d490 | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x8d498 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8d4b0 | store | 4 | `sp -0x68` | `sp-96` |
| `ldr` at 0x8d4b4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d4b8 | store | 4 | `sp -0xe0` | `[sp-148]` |
| `ldr` at 0x8d4bc | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d4c0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d4c4 | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x8d4c8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d4d4 | store | 4 | `sp -0x4c` | `ret(0xa1484)` |
| `ldr` at 0x8d4d8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8d4dc | load | 4 | `sp -0x84` | `-` |
| `ldr` at 0x8d4e8 | load | 4 | `sp -0x4c` | `-` |
| `ldr` at 0x8d4f0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d4f8 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0x8d4fc | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d500 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d504 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d508 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d510 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d514 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d518 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d524 | store | 4 | `sp -0x28` | `ret(0xa32a4)` |
| `ldr` at 0x8d528 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d52c | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d530 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d534 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d53c | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d540 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d544 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d550 | store | 4 | `sp -0x30` | `ret(0xa32a4)` |
| `ldr` at 0x8d554 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d558 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d55c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d560 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d568 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d56c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d570 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d57c | store | 4 | `sp -0x34` | `ret(0xa32a4)` |
| `ldr` at 0x8d580 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d584 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d588 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d58c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d594 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d598 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d59c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d5a8 | store | 4 | `sp -0x38` | `ret(0xa32a4)` |
| `ldr` at 0x8d5ac | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d5b0 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d5b4 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d5b8 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d5c0 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d5c4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d5c8 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d5d4 | store | 4 | `sp -0x3c` | `ret(0xa32a4)` |
| `ldr` at 0x8d5d8 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d5dc | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d5e0 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d5e4 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d5ec | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d5f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d5f4 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d600 | store | 4 | `sp -0x40` | `ret(0xa32a4)` |
| `ldr` at 0x8d604 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d608 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d60c | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d610 | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d618 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d61c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d620 | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d62c | store | 4 | `sp -0x44` | `ret(0xa32a4)` |
| `ldr` at 0x8d630 | load | 4 | `sp -0x88` | `-` |
| `str` at 0x8d634 | store | 4 | `sp -0xe0` | `[sp-136]` |
| `ldr` at 0x8d638 | load | 4 | `sp -0x94` | `-` |
| `str` at 0x8d63c | store | 4 | `sp -0xdc` | `[sp-148]` |
| `ldr` at 0x8d644 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d648 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d64c | load | 4 | `sp -0x84` | `-` |
| `str` at 0x8d658 | store | 4 | `sp -0x48` | `ret(0xa32a4)` |
| `ldr` at 0x8d65c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d664 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d66c | load | 2 | `(unknown)` | `-` |
| `str` at 0x8d680 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x8d684 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d68c | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d694 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d6b0 | load | 4 | `sp -0x88` | `-` |
| `ldrh` at 0x8d6b8 | load | 2 | `(unknown)` | `-` |
| `str` at 0x8d6cc | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x8d6d0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d6d8 | load | 4 | `sp -0x90` | `-` |
| `ldrh` at 0x8d6e0 | load | 2 | `(unknown)` | `-` |
| `ldr` at 0x8d6fc | load | 4 | `sp -0x88` | `-` |
| `ldr` at 0x8d704 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d708 | store | 4 | `sp -0x6c` | `?` |
| `ldr` at 0x8d70c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d714 | load | 4 | `sp -0x90` | `-` |
| `ldr` at 0x8d71c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d720 | store | 4 | `sp -0x70` | `?` |
| `ldr` at 0x8d724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x8d72c | load | 4 | `sp -0x90` | `-` |
| `ldrb` at 0x8d734 | load | 1 | `(unknown)` | `-` |
| `str` at 0x8d740 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0x8d744 | load | 4 | `sp -0x78` | `-` |
| `str` at 0x8d748 | store | 4 | `sp -0xe0` | `[sp-120]` |
| `ldr` at 0x8d74c | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8d750 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8d764 | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8d76c | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d770 | store | 4 | `sp -0xdc` | `?` |
| `ldr` at 0x8d774 | load | 4 | `abs 0x8d850` | `-` |
| `ldr` at 0x8d778 | load | 4 | `sp -0x7c` | `-` |
| `ldr` at 0x8d78c | load | 4 | `abs 0x2822f8` | `-` |
| `ldr` at 0x8d794 | load | 4 | `(unknown)` | `-` |
| `str` at 0x8d798 | store | 4 | `sp -0xd8` | `?` |
| `ldr` at 0x8d79c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d7a0 | store | 4 | `sp -0xd4` | `[sp-24]` |
| `ldr` at 0x8d7a4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x8d7a8 | store | 4 | `sp -0xd0` | `[sp-32]` |
| `ldr` at 0x8d7ac | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x8d7b0 | store | 4 | `sp -0xcc` | `[sp-44]` |
| `ldr` at 0x8d7b4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x8d7b8 | store | 4 | `sp -0xc8` | `[sp-40]` |
| `ldr` at 0x8d7bc | load | 4 | `sp -0x30` | `-` |
| `str` at 0x8d7c0 | store | 4 | `sp -0xc4` | `[sp-48]` |
| `ldr` at 0x8d7c4 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x8d7c8 | store | 4 | `sp -0xc0` | `[sp-52]` |
| `ldr` at 0x8d7cc | load | 4 | `sp -0x38` | `-` |
| `str` at 0x8d7d0 | store | 4 | `sp -0xbc` | `[sp-56]` |
| `ldr` at 0x8d7d4 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x8d7d8 | store | 4 | `sp -0xb8` | `[sp-60]` |
| `ldr` at 0x8d7dc | load | 4 | `sp -0x40` | `-` |
| `str` at 0x8d7e0 | store | 4 | `sp -0xb4` | `[sp-64]` |
| `ldr` at 0x8d7e4 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x8d7e8 | store | 4 | `sp -0xb0` | `[sp-68]` |
| `ldr` at 0x8d7ec | load | 4 | `sp -0x48` | `-` |
| `str` at 0x8d7f0 | store | 4 | `sp -0xac` | `[sp-72]` |
| `ldr` at 0x8d7f4 | load | 4 | `sp -0x54` | `-` |
| `str` at 0x8d7f8 | store | 4 | `sp -0xa8` | `[sp-84]` |
| `ldr` at 0x8d7fc | load | 4 | `sp -0x58` | `-` |
| `str` at 0x8d800 | store | 4 | `sp -0xa4` | `[sp-88]` |
| `ldr` at 0x8d804 | load | 4 | `sp -0x6c` | `-` |
| `str` at 0x8d808 | store | 4 | `sp -0xa0` | `[sp-108]` |
| `ldr` at 0x8d80c | load | 4 | `sp -0x70` | `-` |
| `str` at 0x8d810 | store | 4 | `sp -0x9c` | `[sp-112]` |
| `ldr` at 0x8d814 | load | 4 | `abs 0x8d84c` | `-` |
| `ldr` at 0x8d818 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0x8d81c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x8d820 | load | 4 | `sp -0x74` | `-` |
| `ldr` at 0x8d828 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x8d830 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `ldr` at 0x8d838 | load | 4 | `sp -0x7c` | `-` |
| `str` at 0x8d840 | store | 4 | `sp -0x7c` | `[sp-124]+1` |
| `ldmdb` at 0x8d848 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x8d848 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x8d848 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x8cde0 | `(unknown callee) 0x5e7a8` | `0x0` | `r1` | `r2` | `r3` |
| `bl` at 0x8ce14 | `(unknown callee) 0xa4510` | `0x2822fc` | `?` | `?` | `[ret(0x5e7a8)+16]` |
| `bl` at 0x8cfc4 | `(unknown callee) 0xa1140` | `?` | `0x2a` | `0x20` | `?` |
| `bl` at 0x8d02c | `(unknown callee) 0xa1140` | `?` | `0x15` | `0x8` | `?` |
| `bl` at 0x8d0d0 | `(unknown callee) 0xa1140` | `[sp-100]` | `?` | `?` | `?` |
| `bl` at 0x8d150 | `(unknown callee) 0xa1484` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0x8d474 | `(unknown callee) 0xa46d8` | `0x2822fc` | `[sp-104]` | `[sp-28]` | `[sp-116]` |
| `bl` at 0x8d4cc | `(unknown callee) 0xa1484` | `[sp-144]` | `[sp-24]` | `[sp-132]` | `[sp-136]` |
| `bl` at 0x8d51c | `(unknown callee) 0xa32a4` | `0x0` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d548 | `(unknown callee) 0xa32a4` | `0x1` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d574 | `(unknown callee) 0xa32a4` | `0x2` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d5a0 | `(unknown callee) 0xa32a4` | `0x7` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d5cc | `(unknown callee) 0xa32a4` | `0x3` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d5f8 | `(unknown callee) 0xa32a4` | `0x4` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d624 | `(unknown callee) 0xa32a4` | `0x5` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d650 | `(unknown callee) 0xa32a4` | `0x6` | `[sp-144]` | `[sp-24]` | `[sp-132]` |
| `bl` at 0x8d824 | `(unknown callee) 0xa46d8` | `0x2822fc` | `[sp-104]` | `[sp-28]` | `[sp-116]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x8ce78 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `ble` at 0x8cf64 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x8cfa0 | `? != 0x2` | `? == 0x2` |
| `bne` at 0x8d008 | `? != 0x34` | `? == 0x34` |
| `blt` at 0x8d05c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x8d0e4 | `ret(0xa1140) == 0x0` | `ret(0xa1140) != 0x0` |
| `ble` at 0x8d108 | `? <= 0x1b (signed, 32-bit)` | `? > 0x1b (signed, 32-bit)` |
| `bgt` at 0x8d12c | `? > 0x24 (signed, 32-bit)` | `? <= 0x24 (signed, 32-bit)` |
| `bhi` at 0x8d168 | `ret(0xa1484) > [sp-132] (unsigned, 32-bit)` | `ret(0xa1484) <= [sp-132] (unsigned, 32-bit)` |
| `beq` at 0x8d484 | `ret(0xa1140) == 0x0` | `ret(0xa1140) != 0x0` |
| `beq` at 0x8d4a8 | `? == 0x0` | `? != 0x0` |
| `bhi` at 0x8d4e4 | `ret(0xa1484) > [sp-132] (unsigned, 32-bit)` | `ret(0xa1484) <= [sp-132] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x8d848 | `r0` | `?` |
