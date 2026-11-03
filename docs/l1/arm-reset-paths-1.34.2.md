# ARM reset wrapper and PCI-X paths in 1.34.2

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
exact tool notation. A `:1` suffix marks a byte load. Call candidates use the assumed ARM AAPCS
register locations r0 through r3 with unknown arity. Stack stores appear in the access tables.
Unknown values and predicates include unsupported instruction effects and merged paths.
Numeric call targets have unknown callee names in the flat-image analysis. Device roles
and pointee meanings remain unknown unless the body establishes them. Tables include all
explicit memory operands on known control-flow edges. Implicit stack saves appear in the
full fact sheets. Separate loads at the same printed address need not observe the same value.
Indirect calls can have unknown targets while retaining their continuation and argument candidates.
`resetSource` reports one unresolved computed transfer on known edges. The other fourteen routines
report none. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/reset-paths-facts.txt) retain all tool sections.

## `resetHold`

- `resetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x433b0 to 0x43400.
  Established. Entry r0 equal to `0x5` takes the edge past the first two calls, while inequality
  falls through to `0x437dc` and `0x1dba4` (`beq` at 0x433cc, `bl` at 0x433d4, `bl` at 0x433e4).
  The joined path calls `0xc4a90` with all candidates unknown, followed by `0x4387c` with first
  candidate `ret(0xc4a90)` and the rest unknown (`bl` at 0x433e8, `bl` at 0x433ec). The return
  reports `ret(0x4387c)` (`ldmdb` at 0x433f8). The callees’ reset effects are not established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x433c0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x433c4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x433d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x433dc | load | 4 | `abs 0x433fc` | `-` |
| `ldmdb` at 0x433f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x433f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x433f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x433d4 | `(unknown callee) 0x437dc` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x433e4 | `(unknown callee) 0x1dba4` | `0xe2600` | `ret(0x437dc)` | `?` | `ret(0x437dc)` |
| `bl` at 0x433e8 | `(unknown callee) 0xc4a90` | `?` | `?` | `?` | `?` |
| `bl` at 0x433ec | `(unknown callee) 0x4387c` | `ret(0xc4a90)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x433cc | `r0 == 0x5` | `r0 != 0x5` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x433f8 | `r0` | `ret(0x4387c)` |

## `resetRelease`

- `resetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x43400 to 0x435d8.
  Established. A call to `0xc478c` uses candidates `[sp-24]`, `sp-32`, unknown, `sp-32` (`bl` at
  0x4345c). A nonzero result takes the edge to further calls, while zero falls through to a word
  store of `[sp-32]` at `sp -0x24` (`bne` at 0x43468, `str` at 0x43470). The longer path includes an
  explicit-link call with unknown target and candidates `0xff`, unknown, unknown, `[0x27ee84]`
  (`mov` at 0x4357c). A subsequent call to `0x3ed00` has candidates `0xff`, `0x5`, unknown and
  unknown (`bl` at 0x43594). Release ordering at external hardware and the merged return value
  remain unknown (`ldmdb` at 0x435cc).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x43410 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x43420 | store | 4 | `sp -0x18` | `?` |
| `str` at 0x43428 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x4342c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43438 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43444 | load | 4 | `abs 0x435d0` | `-` |
| `ldr` at 0x43454 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4346c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x43470 | store | 4 | `sp -0x24` | `[sp-32]` |
| `ldr` at 0x43480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4348c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43498 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434a4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434b0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434c8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434d4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434e0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434ec | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x434f8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43504 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43510 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4351c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43528 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43534 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4355c | load | 4 | `abs 0x435d4` | `-` |
| `ldr` at 0x43560 | load | 4 | `abs 0x27ee84` | `-` |
| `ldr` at 0x4356c | load | 4 | `abs 0x435d4` | `-` |
| `ldr` at 0x43570 | load | 4 | `abs 0x27ee84` | `-` |
| `str` at 0x43584 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x4359c | store | 4 | `sp -0x20` | `ret(0x3ed00)` |
| `ldr` at 0x435a0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x435ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x435b8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x435bc | store | 4 | `sp -0x1c` | `ret(0x3ed00)` |
| `ldr` at 0x435c0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x435c4 | store | 4 | `sp -0x24` | `[sp-28]` |
| `ldr` at 0x435c8 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x435cc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x435cc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x435cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x43414 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x4343c | `(unknown callee) 0x437dc` | `[sp-20]` | `?` | `?` | `[sp-20]` |
| `bl` at 0x4344c | `(unknown callee) 0x1dba4` | `0xe2610` | `ret(0x437dc)` | `?` | `ret(0x437dc)` |
| `bl` at 0x4345c | `(unknown callee) 0xc478c` | `[sp-24]` | `sp-32` | `?` | `sp-32` |
| `bl` at 0x4347c | `(unknown callee) 0x43904` | `0xa` | `?` | `?` | `ret(0xc478c)` |
| `bl` at 0x43544 | `(unknown callee) 0x6bd04` | `ret(0x43904)` | `?` | `?` | `[sp-24]` |
| `bl` at 0x43548 | `(unknown callee) 0x6ebac` | `ret(0x6bd04)` | `?` | `?` | `?` |
| `bl` at 0x4354c | `(unknown callee) 0x531c4` | `?` | `?` | `?` | `?` |
| `bl` at 0x43558 | `(unknown callee) 0x2ece8` | `0xff` | `0x0` | `?` | `?` |
| `mov` at 0x4357c | `(unknown callee) (unknown target)` | `0xff` | `?` | `?` | `[0x27ee84]` |
| `bl` at 0x43588 | `(unknown callee) 0x3dd30` | `?` | `?` | `?` | `?` |
| `bl` at 0x43594 | `(unknown callee) 0x3ed00` | `0xff` | `0x5` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x43434 | `[sp-20] == 0x5` | `[sp-20] != 0x5` |
| `bne` at 0x43468 | `ret(0xc478c) != 0x0` | `ret(0xc478c) == 0x0` |
| `beq` at 0x43488 | `[sp-24] == 0x43` | `[sp-24] != 0x43` |
| `beq` at 0x43494 | `[sp-24] == 0x33` | `[sp-24] != 0x33` |
| `beq` at 0x434a0 | `[sp-24] == 0x35` | `[sp-24] != 0x35` |
| `beq` at 0x434ac | `[sp-24] == 0x34` | `[sp-24] != 0x34` |
| `beq` at 0x434b8 | `[sp-24] == 0x41` | `[sp-24] != 0x41` |
| `beq` at 0x434c4 | `[sp-24] == 0x36` | `[sp-24] != 0x36` |
| `beq` at 0x434d0 | `[sp-24] == 0x39` | `[sp-24] != 0x39` |
| `beq` at 0x434dc | `[sp-24] == 0x37` | `[sp-24] != 0x37` |
| `beq` at 0x434e8 | `[sp-24] == 0x38` | `[sp-24] != 0x38` |
| `beq` at 0x434f4 | `[sp-24] == 0x30` | `[sp-24] != 0x30` |
| `beq` at 0x43500 | `[sp-24] == 0x51` | `[sp-24] != 0x51` |
| `beq` at 0x4350c | `[sp-24] == 0x40` | `[sp-24] != 0x40` |
| `beq` at 0x43518 | `[sp-24] == 0x4f` | `[sp-24] != 0x4f` |
| `beq` at 0x43524 | `[sp-24] == 0x42` | `[sp-24] != 0x42` |
| `beq` at 0x43530 | `[sp-24] == 0x46` | `[sp-24] != 0x46` |
| `beq` at 0x4353c | `[sp-24] == 0x4c` | `[sp-24] != 0x4c` |
| `beq` at 0x43568 | `[0x27ee84] == 0x0` | `[0x27ee84] != 0x0` |
| `beq` at 0x435a8 | `ret(0x3ed00) == 0x0` | `ret(0x3ed00) != 0x0` |
| `bne` at 0x435b4 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x435cc | `r0` | `?` |

## `softReset`

- `softReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x435d8 to 0x43664.
  Established. A zero word at absolute `0x27ee8c` takes the edge to a word store of `0x0` at `sp
  -0x1c`, while nonzero falls through to an explicit-link call (`beq` at 0x43624, `str` at
  0x43650, `mov` at 0x4363c). That call has unknown target and first three candidates unknown,
  fourth `[0x27ee8c]` (`mov` at 0x4363c); its result is stored at `sp -0x1c` on that path (`str` at
  0x43644). The return value is unknown after the paths merge (`ldmdb` at 0x43658). A physical
  soft-reset operation is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x435e8 | store | 4 | `sp -0x14` | `r0` |
| `strb` at 0x435f0 | store | 1 | `sp -0x15` | `(r1 & 0xff)` |
| `ldr` at 0x435f4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x43600 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x4360c | load | 4 | `abs 0x4365c` | `-` |
| `ldr` at 0x43618 | load | 4 | `abs 0x43660` | `-` |
| `ldr` at 0x4361c | load | 4 | `abs 0x27ee8c` | `-` |
| `ldr` at 0x43628 | load | 4 | `abs 0x43660` | `-` |
| `ldrb` at 0x4362c | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0x43630 | load | 4 | `abs 0x27ee8c` | `-` |
| `str` at 0x43644 | store | 4 | `sp -0x1c` | `ret(unknown)` |
| `str` at 0x43650 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0x43654 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x43658 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x43658 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x43658 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x43604 | `(unknown callee) 0x437dc` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x43614 | `(unknown callee) 0x1dba4` | `0xe2624` | `ret(0x437dc)` | `?` | `ret(0x437dc)` |
| `mov` at 0x4363c | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `[0x27ee8c]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x435fc | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x43624 | `[0x27ee8c] == 0x0` | `[0x27ee8c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x43658 | `r0` | `?` |

## `resetSource`

- `resetSource`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x437dc to 0x4387c.
  Established. The conditional computed transfer takes its unresolved edge for `r0-1 <= 0x4`
  unsigned and falls through for a greater value (`ldrls` at 0x437fc). On the known fall-through
  path, a word store writes `0xe2670` at `sp -0x18` (`str` at 0x43858). The reported return value on
  that known path is `0xe2670` (`ldmdb` at 0x43860). The other selectors’ destinations and returned
  values remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x437ec | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x437f0 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0x437fc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x43854 | load | 4 | `abs 0x43878` | `-` |
| `str` at 0x43858 | store | 4 | `sp -0x18` | `0xe2670` |
| `ldr` at 0x4385c | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0x43860 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x43860 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x43860 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ldrls` at 0x437fc | `r0-1 <= 0x4 (unsigned, 32-bit)` | `r0-1 > 0x4 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x43860 | `r0` | `0xe2670` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0x437fc).

## `reset`

- `reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x43298 to 0x433b0.
  Established. The call to `0x3dca8` has all candidates unknown (`bl` at 0x432d0); its zero result
  takes the edge to a stack word store of `0x1`, while nonzero falls through to word `0x2`
  at `sp -0x28` (`beq` at 0x432dc, `str` at 0x432f0, `str` at 0x432e4). Later calls include
  `0x43904` with first candidates `0x190` and `0xa`, and `0x43400` with first candidate `0x5` (`bl`
  at 0x43330, `bl` at 0x4335c, `bl` at 0x43364). A zero result from `0x43400` takes the edge to
  the loop increment, while nonzero falls through to storing that result at `sp -0x24` (`beq` at
  0x43378, `str` at 0x43390, `str` at 0x43380). Time units and the reset mechanism are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x432a8 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x432ac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x432b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x432c4 | load | 4 | `abs 0x433a8` | `-` |
| `str` at 0x432e4 | store | 4 | `sp -0x28` | `0x2` |
| `str` at 0x432f0 | store | 4 | `sp -0x28` | `0x1` |
| `ldr` at 0x432f4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x432f8 | store | 4 | `sp -0x20` | `[sp-40]` |
| `str` at 0x43300 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x43304 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43308 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x43318 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x43324 | load | 4 | `abs 0x433ac` | `-` |
| `str` at 0x4333c | store | 4 | `sp -0x1c` | `ret(0x4387c)` |
| `ldr` at 0x43340 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4334c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x43350 | store | 4 | `sp -0x24` | `ret(0x4387c)` |
| `str` at 0x4336c | store | 4 | `sp -0x1c` | `ret(0x43400)` |
| `ldr` at 0x43370 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x4337c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x43380 | store | 4 | `sp -0x24` | `ret(0x43400)` |
| `ldr` at 0x43388 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x43390 | store | 4 | `sp -0x18` | `[sp-24]+1` |
| `str` at 0x4339c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x433a0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x433a4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x433a4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x433a4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x432bc | `(unknown callee) 0x437dc` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0x432cc | `(unknown callee) 0x1dba4` | `0xe25e0` | `ret(0x437dc)` | `?` | `ret(0x437dc)` |
| `bl` at 0x432d0 | `(unknown callee) 0x3dca8` | `?` | `?` | `?` | `?` |
| `bl` at 0x43328 | `(unknown callee) 0x1dba4` | `0xe25ec` | `?` | `?` | `?` |
| `bl` at 0x43330 | `(unknown callee) 0x43904` | `0x190` | `?` | `?` | `?` |
| `bl` at 0x43334 | `(unknown callee) 0x4387c` | `?` | `?` | `?` | `?` |
| `bl` at 0x4335c | `(unknown callee) 0x43904` | `0xa` | `?` | `?` | `ret(0x4387c)` |
| `bl` at 0x43364 | `(unknown callee) 0x43400` | `0x5` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x432b4 | `r0 == 0x5` | `r0 != 0x5` |
| `beq` at 0x432dc | `ret(0x3dca8) == 0x0` | `ret(0x3dca8) != 0x0` |
| `blt` at 0x43310 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x43320 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x43348 | `ret(0x4387c) == 0x0` | `ret(0x4387c) != 0x0` |
| `beq` at 0x43378 | `ret(0x43400) == 0x0` | `ret(0x43400) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x433a4 | `r0` | `?` |

## `imageSyncResetHold`

- `imageSyncResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb2d0 to 0xbb344.
  Established. The call to `0xbb56c` has candidates `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0xbb2f0). A nonzero result takes the edge to byte loads at `ret(0xbb56c) +0x2` and
  `+0x3`, while zero falls through to word `0xfffffc10` at `sp -0x1c` (`bne` at 0xbb304, `ldrb`
  at 0xbb318, `ldrb` at 0xbb320, `str` at 0xbb30c). The nonzero path calls `0xb7958` with those two
  loaded bytes repeated as four candidates (`bl` at 0xbb32c). Their image-synchronisation meaning is
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbb2e4 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xbb2e8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb2f8 | store | 4 | `sp -0x18` | `ret(0xbb56c)` |
| `ldr` at 0xbb2fc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb308 | load | 4 | `abs 0xbb340` | `-` |
| `str` at 0xbb30c | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xbb314 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbb318 | load | 1 | `ret(0xbb56c) +0x2` | `-` |
| `ldr` at 0xbb31c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbb320 | load | 1 | `ret(0xbb56c) +0x3` | `-` |
| `str` at 0xbb334 | store | 4 | `sp -0x1c` | `ret(0xb7958)` |
| `ldr` at 0xbb338 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xbb33c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb33c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb33c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb2f0 | `(unknown callee) 0xbb56c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb32c | `(unknown callee) 0xb7958` | `[ret(0xbb56c)+2]:1` | `[ret(0xbb56c)+3]:1` | `[ret(0xbb56c)+2]:1` | `[ret(0xbb56c)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbb304 | `ret(0xbb56c) != 0x0` | `ret(0xbb56c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb33c | `r0` | `?` |

## `imageSyncResetRelease`

- `imageSyncResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbb344 to 0xbb3b8.
  Established. A nonzero result from `0xbb56c` takes the branch into a call to `0xb790c` (`bl` at
  0xbb364, `bne` at 0xbb378, `bl` at 0xbb3a0). That call has candidates `[ret(0xbb56c)+2]:1`,
  `[ret(0xbb56c)+3]:1`, `[ret(0xbb56c)+2]:1`, `[ret(0xbb56c)+3]:1` (`bl` at 0xbb3a0). A zero result
  from the earlier call falls through to a stack word store of `0xfffffc10` (`bne` at 0xbb378,
  `str` at 0xbb380). The merged return value and the callee’s release semantics remain unknown
  (`ldmdb` at 0xbb3b0).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0xbb358 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0xbb35c | load | 1 | `sp -0x11` | `-` |
| `str` at 0xbb36c | store | 4 | `sp -0x18` | `ret(0xbb56c)` |
| `ldr` at 0xbb370 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbb37c | load | 4 | `abs 0xbb3b4` | `-` |
| `str` at 0xbb380 | store | 4 | `sp -0x1c` | `0xfffffc10` |
| `ldr` at 0xbb388 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbb38c | load | 1 | `ret(0xbb56c) +0x2` | `-` |
| `ldr` at 0xbb390 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xbb394 | load | 1 | `ret(0xbb56c) +0x3` | `-` |
| `str` at 0xbb3a8 | store | 4 | `sp -0x1c` | `ret(0xb790c)` |
| `ldr` at 0xbb3ac | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xbb3b0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbb3b0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbb3b0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbb364 | `(unknown callee) 0xbb56c` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0xbb3a0 | `(unknown callee) 0xb790c` | `[ret(0xbb56c)+2]:1` | `[ret(0xbb56c)+3]:1` | `[ret(0xbb56c)+2]:1` | `[ret(0xbb56c)+3]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbb378 | `ret(0xbb56c) != 0x0` | `ret(0xbb56c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbb3b0 | `r0` | `?` |

## `vrmReset`

- `vrmReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x2dc8c to 0x2dcb8.
  Established. The first call targets `0x3a8c0` with entry r0 through r3 as candidates (`bl` at
  0x2dc98). A result unequal to `0x1` takes the edge to `0x2de10`, while equality falls through
  to `0x2dcf4` (`bne` at 0x2dca4, `bl` at 0x2dcb0, `bl` at 0x2dca8). Both selected calls have
  first and fourth candidates `ret(0x3a8c0)`, with second and third unknown (`bl` at 0x2dcb0, `bl`
  at 0x2dca8). The affected voltage-regulator operation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldmdb` at 0x2dcb4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x2dcb4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x2dcb4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2dc98 | `(unknown callee) 0x3a8c0` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x2dca8 | `(unknown callee) 0x2dcf4` | `ret(0x3a8c0)` | `?` | `?` | `ret(0x3a8c0)` |
| `bl` at 0x2dcb0 | `(unknown callee) 0x2de10` | `ret(0x3a8c0)` | `?` | `?` | `ret(0x3a8c0)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x2dca4 | `ret(0x3a8c0) != 0x1` | `ret(0x3a8c0) == 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x2dcb4 | `r0` | `?` |

## `pcixReset`

- `pcixReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x4945c to 0x494e0.
  Established. The opening call to `0x49c84` has candidates `(r0 & 0xff)`, `(r1 & 0xff)`, `(r1 &
  0xff)`, `(r0 & 0xff)` (`bl` at 0x49490). A zero result takes the edge to a call to `0x4c4a0` with
  first candidate `0x2`, while nonzero falls through to a stack store of the result (`beq` at
  0x494a4, `bl` at 0x494b8, `str` at 0x494ac). The zero-result path then calls `0x49ef4` with
  candidates `[sp-17]:1`, `[sp-18]:1`, `[sp-18]:1`, `[sp-17]:1` (`bl` at 0x494cc). The unit of candidate `0x2`
  and the PCI-X reset effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x49474 | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x4947c | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `ldrb` at 0x49480 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x49484 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x49498 | store | 4 | `sp -0x18` | `ret(0x49c84)` |
| `ldr` at 0x4949c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x494a8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x494ac | store | 4 | `sp -0x1c` | `ret(0x49c84)` |
| `ldrb` at 0x494bc | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x494c0 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x494d4 | store | 4 | `sp -0x1c` | `ret(0x49ef4)` |
| `ldr` at 0x494d8 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x494dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x494dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x494dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49490 | `(unknown callee) 0x49c84` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x494b8 | `(unknown callee) 0x4c4a0` | `0x2` | `?` | `?` | `ret(0x49c84)` |
| `bl` at 0x494cc | `(unknown callee) 0x49ef4` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x494a4 | `ret(0x49c84) == 0x0` | `ret(0x49c84) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x494dc | `r0` | `?` |

## `pcixResetHold`

- `pcixResetHold`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49c84 to 0x49ef4.
  Established. The first inequality branch takes its edge for `(r0 & 0xff) != 0xff`, with equality
  falling through into a loop (`bne` at 0x49cb8). Another branch takes its edge for `(r1 & 0xff) !=
  0xff`, with equality falling through to `0x4b7e4` (`bne` at 0x49db4, `bl` at 0x49dc0). Calls
  to `0x4ae34` include two with first candidate unknown and second `0x1`, and another with first
  candidate `ret(0x4b89c)` and second `0x0` (`bl` at 0x49cf8, `bl` at 0x49e30, `bl` at 0x49ec8). A
  byte store has unresolved effective target and unknown value (`strb` at 0x49d8c). Selector
  coverage and electrical hold semantics are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x49c9c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x49ca4 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x49cac | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x49cb0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x49cc0 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49cc4 | load | 4 | `abs 0x49edc` | `-` |
| `ldr` at 0x49cc8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49ccc | load | 4 | `abs 0x119ea8` | `-` |
| `ldr` at 0x49cdc | load | 4 | `abs 0x49ee0` | `-` |
| `ldr` at 0x49ce0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49ce8 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49d00 | store | 4 | `sp -0x24` | `ret(0x4ae34)` |
| `ldr` at 0x49d04 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49d10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49d1c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49d20 | store | 4 | `sp -0x20` | `ret(0x4ae34)` |
| `ldr` at 0x49d24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49d2c | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `str` at 0x49d38 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49d3c | load | 4 | `abs 0x49ee4` | `-` |
| `ldr` at 0x49d40 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49d44 | load | 4 | `abs 0x119e9c` | `-` |
| `ldr` at 0x49d54 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49d5c | load | 4 | `abs 0x49ee8` | `-` |
| `ldr` at 0x49d60 | load | 4 | `abs 0x119ea0` | `-` |
| `ldr` at 0x49d68 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49d7c | load | 4 | `abs 0x49eec` | `-` |
| `ldr` at 0x49d80 | load | 4 | `abs 0x119e98` | `-` |
| `ldrb` at 0x49d88 | load | 1 | `(unknown)` | `-` |
| `strb` at 0x49d8c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0x49d90 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49d98 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49da0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49da4 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x49dac | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x49db8 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x49dc8 | store | 4 | `sp -0x18` | `ret(0x4b7e4)` |
| `ldr` at 0x49dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49dd8 | load | 4 | `abs 0x49ef0` | `-` |
| `str` at 0x49ddc | store | 4 | `sp -0x2c` | `0xfffff703` |
| `ldr` at 0x49de4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49de8 | load | 1 | `ret(0x4b7e4) +0x1` | `-` |
| `str` at 0x49dec | store | 4 | `sp -0x28` | `[ret(0x4b7e4)+1]:1` |
| `ldr` at 0x49df0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49df4 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x49df8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49dfc | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x49e04 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49e14 | load | 4 | `abs 0x49ee0` | `-` |
| `ldr` at 0x49e18 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49e20 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49e38 | store | 4 | `sp -0x24` | `ret(0x4ae34)` |
| `ldr` at 0x49e3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49e48 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49e54 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49e58 | store | 4 | `sp -0x20` | `ret(0x4ae34)` |
| `ldr` at 0x49e5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49e64 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49e6c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49e70 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldrb` at 0x49e78 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x49e7c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x49e90 | store | 4 | `sp -0x1c` | `ret(0x4b89c)` |
| `ldr` at 0x49e94 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x49ea0 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x49ea4 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x49eb8 | store | 4 | `sp -0x2c` | `ret(0x4ba38)` |
| `ldr` at 0x49ec0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49ed0 | store | 4 | `sp -0x2c` | `ret(0x4ae34)` |
| `ldr` at 0x49ed4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x49ed8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49ed8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49ed8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49cf8 | `(unknown callee) 0x4ae34` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x49dc0 | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x49e30 | `(unknown callee) 0x4ae34` | `?` | `0x1` | `?` | `?` |
| `bl` at 0x49e88 | `(unknown callee) 0x4b89c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x49eb0 | `(unknown callee) 0x4ba38` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0x49ec8 | `(unknown callee) 0x4ae34` | `ret(0x4b89c)` | `0x0` | `?` | `ret(0x4b89c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x49cb8 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x49cd4 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49d0c | `ret(0x4ae34) == 0x0` | `ret(0x4ae34) != 0x0` |
| `bne` at 0x49d18 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `blt` at 0x49d4c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x49db4 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x49dd4 | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `blt` at 0x49e0c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49e44 | `ret(0x4ae34) == 0x0` | `ret(0x4ae34) != 0x0` |
| `bne` at 0x49e50 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x49e9c | `ret(0x4b89c) != 0x0` | `ret(0x4b89c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49ed8 | `r0` | `?` |

## `pcixResetRelease`

- `pcixResetRelease`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x49ef4 to 0x4a08c.
  Established. Two inequality branches select their taken edges for `(r0 & 0xff) != 0xff` and `(r1 &
  0xff) != 0xff` (`bne` at 0x49f28, `bne` at 0x49fc0). The later call to `0x4b89c` has candidates
  `(r0 & 0xff)`, `(r1 & 0xff)`, `(r1 & 0xff)`, `(r0 & 0xff)` (`bl` at 0x4a030); its nonzero result
  takes the edge to `0x4b340`, while zero falls through to `0x4ba38` (`bne` at 0x4a044, `bl` at
  0x4a06c, `bl` at 0x4a058). A distinct path stores word `0xfffff703` at `sp -0x2c` when `0x4b7e4`
  returns zero (`bne` at 0x49fe0, `str` at 0x49fe8). The callees’ reset-release semantics are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x49f0c | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `strb` at 0x49f14 | store | 1 | `sp -0x12` | `(r1 & 0xff)` |
| `str` at 0x49f1c | store | 4 | `sp -0x20` | `0x0` |
| `ldrb` at 0x49f20 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x49f30 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x49f34 | load | 4 | `abs 0x4a080` | `-` |
| `ldr` at 0x49f38 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49f3c | load | 4 | `abs 0x119e9c` | `-` |
| `ldr` at 0x49f4c | load | 4 | `abs 0x4a084` | `-` |
| `ldr` at 0x49f50 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49f64 | load | 4 | `abs 0x119e98` | `-` |
| `str` at 0x49f78 | store | 4 | `sp -0x24` | `ret(0x4b1bc)` |
| `ldr` at 0x49f7c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x49f88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x49f94 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x49f98 | store | 4 | `sp -0x20` | `ret(0x4b1bc)` |
| `ldr` at 0x49f9c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49fa4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0x49fac | load | 4 | `sp -0x20` | `-` |
| `str` at 0x49fb0 | store | 4 | `sp -0x2c` | `?` |
| `ldrb` at 0x49fb8 | load | 1 | `sp -0x12` | `-` |
| `ldrb` at 0x49fc4 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x49fd4 | store | 4 | `sp -0x1c` | `ret(0x4b7e4)` |
| `ldr` at 0x49fd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x49fe4 | load | 4 | `abs 0x4a088` | `-` |
| `str` at 0x49fe8 | store | 4 | `sp -0x2c` | `0xfffff703` |
| `ldr` at 0x49ff0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49ffc | store | 4 | `sp -0x24` | `ret(0x4b1bc)` |
| `ldr` at 0x4a000 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x4a00c | load | 4 | `sp -0x24` | `-` |
| `str` at 0x4a010 | store | 4 | `sp -0x20` | `ret(0x4b1bc)` |
| `ldr` at 0x4a014 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x4a018 | store | 4 | `sp -0x2c` | `[sp-32]` |
| `ldrb` at 0x4a020 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4a024 | load | 1 | `sp -0x12` | `-` |
| `str` at 0x4a038 | store | 4 | `sp -0x18` | `ret(0x4b89c)` |
| `ldr` at 0x4a03c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x4a048 | load | 1 | `sp -0x11` | `-` |
| `ldrb` at 0x4a04c | load | 1 | `sp -0x12` | `-` |
| `str` at 0x4a060 | store | 4 | `sp -0x2c` | `ret(0x4ba38)` |
| `ldr` at 0x4a068 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x4a074 | store | 4 | `sp -0x2c` | `ret(0x4b340)` |
| `ldr` at 0x4a078 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x4a07c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x4a07c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x4a07c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x49f70 | `(unknown callee) 0x4b1bc` | `?` | `0x119e98` | `?` | `?` |
| `bl` at 0x49fcc | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r1` | `(r0 & 0xff)` |
| `bl` at 0x49ff4 | `(unknown callee) 0x4b1bc` | `ret(0x4b7e4)` | `?` | `?` | `ret(0x4b7e4)` |
| `bl` at 0x4a030 | `(unknown callee) 0x4b89c` | `(r0 & 0xff)` | `(r1 & 0xff)` | `(r1 & 0xff)` | `(r0 & 0xff)` |
| `bl` at 0x4a058 | `(unknown callee) 0x4ba38` | `[sp-17]:1` | `[sp-18]:1` | `[sp-18]:1` | `[sp-17]:1` |
| `bl` at 0x4a06c | `(unknown callee) 0x4b340` | `ret(0x4b89c)` | `?` | `?` | `ret(0x4b89c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x49f28 | `(r0 & 0xff) != 0xff` | `(r0 & 0xff) == 0xff` |
| `blt` at 0x49f44 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x49f84 | `ret(0x4b1bc) == 0x0` | `ret(0x4b1bc) != 0x0` |
| `bne` at 0x49f90 | `[sp-32] != 0x0` | `[sp-32] == 0x0` |
| `bne` at 0x49fc0 | `(r1 & 0xff) != 0xff` | `(r1 & 0xff) == 0xff` |
| `bne` at 0x49fe0 | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `beq` at 0x4a008 | `ret(0x4b1bc) == 0x0` | `ret(0x4b1bc) != 0x0` |
| `bne` at 0x4a044 | `ret(0x4b89c) != 0x0` | `ret(0x4b89c) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x4a07c | `r0` | `?` |

## `pcixBusInReset`

- `pcixBusInReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x499c8 to 0x49b2c.
  Established. The call to `0x4b7e4` uses candidates `(r0 & 0xff)`, entry r1, entry r2 and `(r0 &
  0xff)` (`bl` at 0x499e8). A nonzero result takes the edge into a loop, while zero falls through
  to a word store of `0xfffff703` at `sp -0x30` (`bne` at 0x499fc, `str` at 0x49a04). The loop
  call to `0x4bccc` has candidates `[sp-28]`, `sp-36`, unknown, `sp-36` (`bl` at 0x49a70). A zero
  result takes the edge to later tests, while nonzero falls through to storing that result at
  `sp -0x30` (`beq` at 0x49a84, `str` at 0x49a8c). Other paths store word `0x0` or `0x1` there
  (`str` at 0x49af8, `str` at 0x49b14). A bus-in-reset interpretation of those results is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x499dc | store | 1 | `sp -0x11` | `(r0 & 0xff)` |
| `ldrb` at 0x499e0 | load | 1 | `sp -0x11` | `-` |
| `str` at 0x499f0 | store | 4 | `sp -0x18` | `ret(0x4b7e4)` |
| `ldr` at 0x499f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x49a00 | load | 4 | `abs 0x49b20` | `-` |
| `str` at 0x49a04 | store | 4 | `sp -0x30` | `0xfffff703` |
| `str` at 0x49a10 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x49a14 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49a18 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x49a1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x49a2c | load | 4 | `abs 0x49b24` | `-` |
| `ldr` at 0x49a30 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0x49a34 | load | 1 | `(unknown)` | `-` |
| `ldr` at 0x49a38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x49a44 | load | 4 | `abs 0x119ea4` | `-` |
| `str` at 0x49a4c | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0x49a58 | load | 4 | `abs 0x49b28` | `-` |
| `str` at 0x49a5c | store | 4 | `sp -0x30` | `0xfffff702` |
| `ldr` at 0x49a68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x49a78 | store | 4 | `sp -0x28` | `ret(0x4bccc)` |
| `ldr` at 0x49a7c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x49a88 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x49a8c | store | 4 | `sp -0x30` | `ret(0x4bccc)` |
| `ldrb` at 0x49aa0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x49ac0 | load | 1 | `(unknown)` | `-` |
| `ldrb` at 0x49ae0 | load | 1 | `(unknown)` | `-` |
| `str` at 0x49af8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x49b00 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x49b08 | store | 4 | `sp -0x2c` | `[sp-44]+1` |
| `str` at 0x49b14 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0x49b18 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x49b1c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x49b1c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x49b1c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x499e8 | `(unknown callee) 0x4b7e4` | `(r0 & 0xff)` | `r1` | `r2` | `(r0 & 0xff)` |
| `bl` at 0x49a70 | `(unknown callee) 0x4bccc` | `[sp-28]` | `sp-36` | `?` | `sp-36` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x499fc | `ret(0x4b7e4) != 0x0` | `ret(0x4b7e4) == 0x0` |
| `blt` at 0x49a24 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `bne` at 0x49a54 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x49a84 | `ret(0x4bccc) == 0x0` | `ret(0x4bccc) != 0x0` |
| `beq` at 0x49ab0 | `? == 0x3` | `? != 0x3` |
| `bne` at 0x49ad0 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x49af0 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x49b1c | `r0` | `?` |

## `promIceReset`

- `promIceReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x41350 to 0x41478.
  Established. The body includes a call to `0x1c79c` with first and fourth candidates
  `[sp-25]:1+108`, second `0x2811a0` and third unknown (`bl` at 0x413bc); its nonzero result takes
  the edge to byte `0x0` at `sp -0x1b`, while zero falls through to byte `([[sp-20]+1]:1 &
  0xff)` at that target (`bne` at 0x413c8, `strb` at 0x413e0, `strb` at 0x413d4). A later call to
  `0xb79a0` has candidates `[[sp-20]]:1`, `[[sp-20]+1]:1`, unknown, `[[sp-20]+1]:1` (`bl` at
  0x41408). The PROM/ICE line action and loop bound remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `strb` at 0x41364 | store | 1 | `sp -0x19` | `0x0` |
| `ldrb` at 0x41368 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x4136c | load | 4 | `abs 0x41468` | `-` |
| `ldr` at 0x41370 | load | 4 | `abs 0x281180` | `-` |
| `ldrb` at 0x41380 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x41398 | load | 4 | `abs 0x4146c` | `-` |
| `ldrb` at 0x4139c | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x413a4 | load | 4 | `abs 0x2811a0` | `-` |
| `str` at 0x413ac | store | 4 | `sp -0x14` | `?` |
| `ldrb` at 0x413b0 | load | 1 | `sp -0x19` | `-` |
| `ldr` at 0x413cc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x413d0 | load | 1 | `[sp-20] +0x1` | `-` |
| `strb` at 0x413d4 | store | 1 | `sp -0x1b` | `([[sp-20]+1]:1 & 0xff)` |
| `strb` at 0x413e0 | store | 1 | `sp -0x1b` | `0x0` |
| `ldrb` at 0x413e4 | load | 1 | `sp -0x1b` | `-` |
| `strb` at 0x413e8 | store | 1 | `sp -0x1a` | `?` |
| `ldr` at 0x413ec | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x413f0 | load | 1 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x413f4 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x413f8 | load | 1 | `[sp-20] +0x1` | `-` |
| `ldrb` at 0x413fc | load | 1 | `sp -0x1a` | `-` |
| `str` at 0x41410 | store | 4 | `sp -0x18` | `ret(0xb79a0)` |
| `ldr` at 0x41414 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x41420 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x4142c | load | 4 | `abs 0x41470` | `-` |
| `str` at 0x41430 | store | 4 | `sp -0x28` | `0xe23c4` |
| `ldrb` at 0x41434 | load | 1 | `sp -0x19` | `-` |
| `str` at 0x41438 | store | 4 | `sp -0x24` | `[sp-25]:1` |
| `str` at 0x4143c | store | 4 | `sp -0x20` | `ret(0x1ea50)` |
| `ldr` at 0x41444 | load | 4 | `abs 0x41474` | `-` |
| `ldrb` at 0x41454 | load | 1 | `sp -0x19` | `-` |
| `strb` at 0x4145c | store | 1 | `sp -0x19` | `([sp-25]:1+1 & 0xff)` |
| `ldmdb` at 0x41464 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x41464 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x41464 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x41388 | `(unknown callee) 0x414d4` | `?` | `?` | `[0x281180]` | `?` |
| `bl` at 0x413bc | `(unknown callee) 0x1c79c` | `[sp-25]:1+108` | `0x2811a0` | `?` | `[sp-25]:1+108` |
| `bl` at 0x41408 | `(unknown callee) 0xb79a0` | `[[sp-20]]:1` | `[[sp-20]+1]:1` | `?` | `[[sp-20]+1]:1` |
| `bl` at 0x41424 | `(unknown callee) 0x1ea50` | `ret(0xb79a0)` | `?` | `?` | `ret(0xb79a0)` |
| `bl` at 0x41450 | `(unknown callee) 0x1eed0` | `0x1` | `0xe2390` | `0x66` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0x41378 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `beq` at 0x41394 | `ret(0x414d4) == 0x0` | `ret(0x414d4) != 0x0` |
| `bne` at 0x413c8 | `ret(0x1c79c) != 0x0` | `ret(0x1c79c) == 0x0` |
| `beq` at 0x4141c | `ret(0xb79a0) == 0x0` | `ret(0xb79a0) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x41464 | `r0` | `?` |

## `PPPReset`

- `PPPReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ee24 to 0x9ee84.
  Established. Two conditional branches retain unknown compared values, following their bypass edges
  for `? == 0x0` (`beq` at 0x9ee48, `beq` at 0x9ee70). The first fall-through path includes word
  `0x0` at `r0 +0xc` and at `[sp-20] +0x30` (`str` at 0x9ee54, `str` at 0x9ee60). The second
  fall-through path contains another zero word store at an unresolved effective target (`str` at
  0x9ee7c). Entry r0 is the reported return value (`ldmdb` at 0x9ee80). The fields’ protocol roles
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ee34 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ee38 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9ee3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ee4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ee54 | store | 4 | `r0 +0xc` | `0x0` |
| `ldr` at 0x9ee58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ee60 | store | 4 | `[sp-20] +0x30` | `0x0` |
| `ldr` at 0x9ee64 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9ee74 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ee7c | store | 4 | `(unknown)` | `0x0` |
| `ldmdb` at 0x9ee80 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9ee80 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9ee80 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9ee48 | `? == 0x0` | `? != 0x0` |
| `beq` at 0x9ee70 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9ee80 | `r0` | `r0` |

## `exceptionReset`

- `exceptionReset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x1e174 to 0x1e224.
  Established. The body includes a call to `0xaf3fc` with all candidates unknown (`bl` at 0x1e1dc).
  Later calls to `0x188d8` and `0x1f7b8` have first candidates `0xd6ff8` and `0x1388`, respectively
  (`bl` at 0x1e1e4, `bl` at 0x1e1ec). The last listed direct call targets `0x18b84`, with all
  candidates unknown (`bl` at 0x1e204). No decoded return is reported. Trailing words decode as
  conditional stores in the retained listing, but whether they execute as code is unknown (`strheq`
  at 0x1e210, `strdeq` at 0x1e218). Reset behavior cannot be established from the numeric callees
  alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x1e184 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x1e188 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x1e18c | load | 4 | `abs 0x1e208` | `-` |
| `ldr` at 0x1e190 | load | 4 | `abs 0x1169d8` | `-` |
| `ldr` at 0x1e19c | load | 4 | `abs 0x1e20c` | `-` |
| `ldr` at 0x1e1a0 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x1e1a4 | load | 4 | `abs 0x1e210` | `-` |
| `ldr` at 0x1e1b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x1e1b8 | load | 4 | `abs 0x1e214` | `-` |
| `ldr` at 0x1e1cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1e1d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x1e1e0 | load | 4 | `abs 0x1e218` | `-` |
| `ldr` at 0x1e1e8 | load | 4 | `abs 0x1e21c` | `-` |
| `ldr` at 0x1e1f0 | load | 4 | `abs 0x1e20c` | `-` |
| `ldr` at 0x1e1f4 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x1e1f8 | load | 4 | `abs 0x1e220` | `-` |
| `ldrsbeq` at 0x1e208 | load | 1 | `(unknown)` | `-` |
| `strheq` at 0x1e210 | store | 2 | `sp -0x18` | `?` |
| `strdeq` at 0x1e218 | store | 8 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x1e1a8 | `(unknown callee) 0x21010` | `[0x116928]` | `0xd6fbc` | `r2` | `0x116928` |
| `bl` at 0x1e1b0 | `(unknown callee) 0x18b84` | `?` | `?` | `?` | `?` |
| `bl` at 0x1e1c8 | `(unknown callee) 0x18b84` | `?` | `?` | `?` | `0x3e7` |
| `bl` at 0x1e1dc | `(unknown callee) 0xaf3fc` | `?` | `?` | `?` | `?` |
| `bl` at 0x1e1e4 | `(unknown callee) 0x188d8` | `0xd6ff8` | `?` | `?` | `?` |
| `bl` at 0x1e1ec | `(unknown callee) 0x1f7b8` | `0x1388` | `?` | `?` | `?` |
| `bl` at 0x1e1fc | `(unknown callee) 0x21010` | `[0x116928]` | `0xd7008` | `?` | `0x116928` |
| `bl` at 0x1e204 | `(unknown callee) 0x18b84` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x1e198 | `[0x1169d8] == 0x0` | `[0x1169d8] != 0x0` |
| `bne` at 0x1e1c0 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x1e1d4 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
