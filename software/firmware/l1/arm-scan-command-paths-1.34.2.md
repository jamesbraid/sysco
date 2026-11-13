# ARM scan-command and port paths in 1.34.2

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
`jmstr_scan`, `sn1sc_process_scancmd` and `sn1sc_reset_brick` each report one unresolved computed
transfer on known edges. The other twelve routines report none. Undecodable byte ranges remain
visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/scan-command-paths-facts.txt) retain all tool sections.

## `jmstr_reset`

- `jmstr_reset`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc308 to 0xbc3c4.
  Established. A word at `sp -0x24` receives `0x20`, with a nonzero entry r1 selecting a later store
  of `0x28` (`str` at 0xbc330, `beq` at 0xbc33c, `str` at 0xbc348). The explicit-link call has
  unknown target and candidates unknown, `0x20`, `sp-40`, `sp-40` (`mov` at 0xbc384). A nonzero
  result takes the edge to word `0x1` at `sp -0x2c`, while zero falls through to a call to
  `0x18c74` and word `0x0` there (`bne` at 0xbc390, `str` at 0xbc3b0, `bl` at 0xbc39c, `str` at
  0xbc3a4). The command bits’ scan-reset meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc318 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbc31c | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbc320 | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xbc328 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xbc330 | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xbc334 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc340 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc348 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xbc34c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc358 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc360 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbc364 | load | 4 | `abs 0xbc3bc` | `-` |
| `ldr` at 0xbc370 | load | 4 | `abs 0x27eb88` | `-` |
| `ldr` at 0xbc374 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc394 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc398 | load | 4 | `abs 0xbc3c0` | `-` |
| `str` at 0xbc3a4 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xbc3b0 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xbc3b4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xbc3b8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbc3b8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc3b8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc3b8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbc384 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xbc39c | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109b14` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbc33c | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xbc354 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xbc390 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc3b8 | `r0` | `?` |

## `jmstr_reset_and_idle`

- `jmstr_reset_and_idle`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc3c4 to 0xbc4e8.
  Established. The first explicit-link call has unknown target and candidates unknown, `0x20`,
  `sp-40`, `sp-40` (`mov` at 0xbc440). A nonzero result takes the edge to a word store of `0x10`
  at `sp -0x24`, while zero falls through to the earlier zero-result exit path (`bne` at
  0xbc44c, `str` at 0xbc46c, `str` at 0xbc460). A second explicit-link call uses the same reported
  candidates (`mov` at 0xbc4a8); its nonzero result takes the edge to a word store of `0x1` at
  `sp -0x2c`, while zero falls through to a call to `0x18c74` and word `0x0` there (`bne` at
  0xbc4b4, `str` at 0xbc4d4, `bl` at 0xbc4c0, `str` at 0xbc4c8). An idle-state transition is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc3d4 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xbc3d8 | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xbc3dc | store | 4 | `sp -0x20` | `r2` |
| `str` at 0xbc3e4 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xbc3ec | store | 4 | `sp -0x24` | `0x20` |
| `ldr` at 0xbc3f0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc3fc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc404 | store | 4 | `sp -0x24` | `0x28` |
| `ldr` at 0xbc408 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbc414 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc41c | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbc420 | load | 4 | `abs 0xbc4e0` | `-` |
| `ldr` at 0xbc42c | load | 4 | `abs 0x27eb88` | `-` |
| `ldr` at 0xbc430 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc450 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc454 | load | 4 | `abs 0xbc4e4` | `-` |
| `str` at 0xbc460 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xbc46c | store | 4 | `sp -0x24` | `0x10` |
| `ldr` at 0xbc470 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc47c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc484 | store | 4 | `sp -0x24` | `0x18` |
| `ldr` at 0xbc488 | load | 4 | `abs 0xbc4e0` | `-` |
| `ldr` at 0xbc494 | load | 4 | `abs 0x27eb88` | `-` |
| `ldr` at 0xbc498 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc4b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc4bc | load | 4 | `abs 0xbc4e4` | `-` |
| `str` at 0xbc4c8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xbc4d4 | store | 4 | `sp -0x2c` | `0x1` |
| `ldr` at 0xbc4d8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xbc4dc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbc4dc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc4dc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc4dc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `mov` at 0xbc440 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xbc458 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109b14` | `?` | `ret(unknown)` |
| `mov` at 0xbc4a8 | `(unknown callee) (unknown target)` | `?` | `0x20` | `sp-40` | `sp-40` |
| `bl` at 0xbc4c0 | `(unknown callee) 0x18c74` | `[sp-32]` | `0x109b14` | `?` | `ret(unknown)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xbc3f8 | `r1 == 0x0` | `r1 != 0x0` |
| `bne` at 0xbc410 | `r0 != 0x1` | `r0 == 0x1` |
| `bne` at 0xbc44c | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |
| `beq` at 0xbc478 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xbc4b4 | `ret(unknown) != 0x0` | `ret(unknown) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc4dc | `r0` | `?` |

## `jmstr_scan`

- `jmstr_scan`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbc8b4 to 0xbce44.
  Established. A nonzero word at entry `sp +0x0` takes the branch to further processing, while
  zero falls through to word `0x1` at `sp -0x48` (`bne` at 0xbc8e4, `str` at 0xbc8ec). The
  conditional computed transfer takes its unresolved edge for `r1 <= 0xd` unsigned and falls through
  for a larger value (`ldrls` at 0xbc91c). The known fall-through path calls `0x18cd4` with
  candidates `[sp+12]`, `0x109b6c`, entry r2 and entry r1 (`bl` at 0xbca7c). It then stores word
  `0x0` at `sp -0x48` (`str` at 0xbca84). The selected scan operation and arguments on excluded
  destinations remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbc8c4 | store | 4 | `sp -0x1c` | `r0` |
| `str` at 0xbc8c8 | store | 4 | `sp -0x20` | `r1` |
| `str` at 0xbc8cc | store | 4 | `sp -0x24` | `r2` |
| `str` at 0xbc8d0 | store | 4 | `sp -0x28` | `r3` |
| `str` at 0xbc8d8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xbc8dc | load | 4 | `sp +0x0` | `-` |
| `str` at 0xbc8ec | store | 4 | `sp -0x48` | `0x1` |
| `str` at 0xbc8f8 | store | 4 | `sp -0x34` | `0x10` |
| `ldr` at 0xbc8fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xbc908 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xbc910 | store | 4 | `sp -0x34` | `0x18` |
| `ldr` at 0xbc914 | load | 4 | `sp -0x20` | `-` |
| `ldrls` at 0xbc91c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbca74 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xbca78 | load | 4 | `abs 0xbce34` | `-` |
| `str` at 0xbca84 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xbce2c | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xbce30 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xbce30 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbce30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbce30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbce30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbca7c | `(unknown callee) 0x18cd4` | `[sp+12]` | `0x109b6c` | `r2` | `r1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbc8e4 | `[sp] != 0x0` | `[sp] == 0x0` |
| `beq` at 0xbc904 | `r3 == 0x0` | `r3 != 0x0` |
| `ldrls` at 0xbc91c | `r1 <= 0xd (unsigned, 32-bit)` | `r1 > 0xd (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbce30 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xbc91c).

## `scancmd_create`

- `scancmd_create`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbb8c to 0xbbc50.
  Established. The call to `0x18920` has candidates `[sp-32]`, entry r1, entry r2 and unknown (`bl`
  at 0xbbbf0). A nonzero result takes the edge to `0xbbc50`, while zero falls through to a
  stack store of `0x0` at `sp -0x1c` (`bne` at 0xbbc04, `bl` at 0xbbc1c, `str` at 0xbbc0c). The call
  to `0xbbc50` has candidates `[sp-20]`, `ret(0x18920)`, unknown, `ret(0x18920)` (`bl` at 0xbbc1c);
  its nonzero result takes the edge to storing `[sp-24]` at `sp -0x1c`, whereas zero falls
  through to a call to `0x18be4` (`bne` at 0xbbc28, `str` at 0xbbc44, `bl` at 0xbbc30). The
  candidate size and object type are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbb9c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbbba0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbbb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbbb8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbbbc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbbbc8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbbd0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbbd8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbbdc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbbbe8 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0xbbbec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbbbf8 | store | 4 | `sp -0x18` | `ret(0x18920)` |
| `ldr` at 0xbbbfc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbc0c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xbbc14 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbc18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbc2c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbc38 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xbbc40 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbc44 | store | 4 | `sp -0x1c` | `[sp-24]` |
| `ldr` at 0xbbc48 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xbbc4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbbc4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbbc4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbbf0 | `(unknown callee) 0x18920` | `[sp-32]` | `r1` | `r2` | `?` |
| `bl` at 0xbbc1c | `(unknown callee) 0xbbc50` | `[sp-20]` | `ret(0x18920)` | `?` | `ret(0x18920)` |
| `bl` at 0xbbc30 | `(unknown callee) 0x18be4` | `[sp-24]` | `?` | `?` | `ret(0xbbc50)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbbbac | `? != 0x0` | `? == 0x0` |
| `bne` at 0xbbc04 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0xbbc28 | `ret(0xbbc50) != 0x0` | `ret(0xbbc50) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbbc4c | `r0` | `?` |

## `scancmd_init`

- `scancmd_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbc50 to 0xbbcec.
  Established. Entry r1 unequal to zero takes the edge into the body, while zero falls through
  to word `0x0` at `sp -0x20` (`bne` at 0xbbc70, `str` at 0xbbc78). The body calls `0x18c20`
  with candidates entry r1, `0x0`, `[sp-36]`, unknown (`bl` at 0xbbcd8). It subsequently writes word
  `0x1` at `sp -0x20` (`str` at 0xbbce0). The return value is unknown after the paths merge (`ldmdb`
  at 0xbbce8). No command-record size or initialisation semantics are established for the numeric
  callee.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbc60 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbbc64 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xbbc68 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xbbc78 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xbbc80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbc90 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbc98 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbbc9c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbbca8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbbcb0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbcb8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbbcbc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbbcc8 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xbbccc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbcd4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbbce0 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xbbce4 | load | 4 | `sp -0x20` | `-` |
| `ldmdb` at 0xbbce8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbbce8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbbce8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbcd8 | `(unknown callee) 0x18c20` | `r1` | `0x0` | `[sp-36]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbbc70 | `r1 != 0x0` | `r1 == 0x0` |
| `bne` at 0xbbc8c | `? != 0x0` | `? == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbbce8 | `r0` | `?` |

## `scancmd_destroy`

- `scancmd_destroy`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbcec to 0xbbd1c.
  Established. A nonzero entry r0 takes the edge to the sole call, while zero falls through to
  the return (`bne` at 0xbbd08). The call targets `0x18be4` with candidates entry r0, entry r1,
  entry r2 and entry r0 (`bl` at 0xbbd14). The reported return value is unknown after the join
  (`ldmdb` at 0xbbd18). Destruction or deallocation behavior is not established from this body
  alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbcfc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xbbd00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbd10 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xbbd18 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbbd18 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbbd18 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbd14 | `(unknown callee) 0x18be4` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbbd08 | `r0 != 0x0` | `r0 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbbd18 | `r0` | `?` |

## `scancmd_dump`

- `scancmd_dump`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xbbd1c to 0xbc2cc.
  Established. A nonzero entry r3 takes the edge into the longer body, while zero falls through
  to the return (`bne` at 0xbbd44). Calls include `0x21010` with candidates `[sp-20]`,
  `0x109a44`, `[[sp-32]+12]`, `[sp-32]+12` (`bl` at 0xbc010). A zero word at `[sp-32] +0xc` takes
  the edge past the later loop bodies (`ldr` at 0xbc044, `beq` at 0xbc04c). The loop stores include
  `[sp-36]+1` at `sp -0x24` (`str` at 0xbc1ec, `str` at 0xbc250). No device role is established for
  the numeric call targets or the loaded record fields.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xbbd2c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xbbd30 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xbbd34 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xbbd38 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xbbd3c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbd4c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xbbd50 | store | 4 | `sp -0x40` | `r0` |
| `ldr` at 0xbbd54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbd60 | load | 4 | `abs 0xbc268` | `-` |
| `ldr` at 0xbbd64 | load | 4 | `abs 0x116924` | `-` |
| `str` at 0xbbd68 | store | 4 | `sp -0x40` | `[0x116924]` |
| `ldr` at 0xbbd6c | load | 4 | `sp -0x40` | `-` |
| `str` at 0xbbd70 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xbbd74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbd80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbd84 | load | 4 | `abs 0xbc26c` | `-` |
| `ldr` at 0xbbd88 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xbbd94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbd98 | load | 4 | `abs 0xbc270` | `-` |
| `ldr` at 0xbbda0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbda4 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xbbdb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbdb4 | load | 4 | `abs 0xbc274` | `-` |
| `ldr` at 0xbbdc4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbdcc | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbdd8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbddc | load | 4 | `abs 0xbc278` | `-` |
| `ldr` at 0xbbde8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbdec | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xbbdf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbdfc | load | 4 | `abs 0xbc27c` | `-` |
| `ldr` at 0xbbe0c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbe14 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbe20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbe24 | load | 4 | `abs 0xbc280` | `-` |
| `ldr` at 0xbbe30 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbe34 | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xbbe40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbe44 | load | 4 | `abs 0xbc284` | `-` |
| `ldr` at 0xbbe54 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbe5c | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbe68 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbe6c | load | 4 | `abs 0xbc288` | `-` |
| `ldr` at 0xbbe78 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbe7c | load | 4 | `[sp-32] +0x0` | `-` |
| `ldr` at 0xbbe84 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbe88 | load | 4 | `abs 0xbc28c` | `-` |
| `ldr` at 0xbbe98 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbea0 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbeac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbeb0 | load | 4 | `abs 0xbc290` | `-` |
| `ldr` at 0xbbec0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbec8 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xbbed4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbed8 | load | 4 | `abs 0xbc294` | `-` |
| `ldr` at 0xbbee8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbef0 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbefc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbf00 | load | 4 | `abs 0xbc298` | `-` |
| `ldr` at 0xbbf10 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbf18 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xbbf24 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbf28 | load | 4 | `abs 0xbc29c` | `-` |
| `ldr` at 0xbbf38 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbf40 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbf4c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbf50 | load | 4 | `abs 0xbc2a0` | `-` |
| `ldr` at 0xbbf60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbf68 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xbbf74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbf78 | load | 4 | `abs 0xbc2a4` | `-` |
| `ldr` at 0xbbf88 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbf90 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbf9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbfa0 | load | 4 | `abs 0xbc2a8` | `-` |
| `ldr` at 0xbbfb0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbfb8 | load | 4 | `[sp-32] +0x4` | `-` |
| `ldr` at 0xbbfc0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbfc4 | load | 4 | `abs 0xbc2ac` | `-` |
| `ldr` at 0xbbfd4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbbfdc | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbbfe8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbbfec | load | 4 | `abs 0xbc2b0` | `-` |
| `ldr` at 0xbbffc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc004 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc008 | load | 4 | `abs 0xbc2b4` | `-` |
| `ldr` at 0xbc00c | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xbc018 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc020 | load | 4 | `[sp-32] +0x8` | `-` |
| `ldr` at 0xbc028 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc02c | load | 4 | `abs 0xbc2b8` | `-` |
| `ldr` at 0xbc03c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc044 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xbc050 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xbc060 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xbc068 | store | 4 | `sp -0x3c` | `[sp-32]+16` |
| `ldr` at 0xbc070 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc078 | load | 4 | `[sp-32] +0xc` | `-` |
| `ldr` at 0xbc090 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xbc098 | load | 4 | `[sp-32] +0xc` | `-` |
| `str` at 0xbc0a4 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xbc0a8 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc0b0 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xbc0b4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xbc0bc | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xbc0c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc0c4 | load | 4 | `abs 0xbc2bc` | `-` |
| `str` at 0xbc0d0 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbc0d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc0d8 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbc0e8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc0f0 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc0f8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc104 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc110 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc11c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc128 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc12c | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xbc130 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc13c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc148 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc14c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xbc150 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc15c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc168 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc16c | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0xbc170 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc17c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc188 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc18c | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xbc190 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc19c | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc1a8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc1ac | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0xbc1b0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc1bc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc1c8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xbc1cc | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xbc1d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc1d4 | load | 4 | `abs 0xbc2c0` | `-` |
| `ldr` at 0xbc1d8 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbc1dc | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbc1e4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc1ec | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xbc1f4 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xbc204 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xbc208 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc20c | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xbc21c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xbc224 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xbc230 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0xbc238 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc23c | load | 4 | `abs 0xbc2c4` | `-` |
| `ldr` at 0xbc240 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xbc248 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xbc250 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xbc258 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xbc25c | load | 4 | `abs 0xbc2c8` | `-` |
| `ldmdb` at 0xbc264 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xbc264 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xbc264 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xbbd8c | `(unknown callee) 0x21010` | `?` | `0x1098c0` | `r1` | `r1` |
| `bl` at 0xbbd9c | `(unknown callee) 0x21010` | `?` | `0x1098c4` | `r2` | `r1` |
| `bl` at 0xbbdbc | `(unknown callee) 0x21010` | `[sp-20]` | `0x1098e0` | `?` | `?` |
| `bl` at 0xbbde4 | `(unknown callee) 0x21010` | `[sp-20]` | `0x1098fc` | `?` | `?` |
| `bl` at 0xbbe04 | `(unknown callee) 0x21010` | `[sp-20]` | `0x10990c` | `?` | `?` |
| `bl` at 0xbbe2c | `(unknown callee) 0x21010` | `[sp-20]` | `0x109928` | `?` | `?` |
| `bl` at 0xbbe4c | `(unknown callee) 0x21010` | `[sp-20]` | `0x109938` | `?` | `?` |
| `bl` at 0xbbe74 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109954` | `?` | `?` |
| `bl` at 0xbbe90 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109964` | `?` | `?` |
| `bl` at 0xbbeb8 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109980` | `?` | `?` |
| `bl` at 0xbbee0 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109990` | `?` | `?` |
| `bl` at 0xbbf08 | `(unknown callee) 0x21010` | `[sp-20]` | `0x1099ac` | `?` | `?` |
| `bl` at 0xbbf30 | `(unknown callee) 0x21010` | `[sp-20]` | `0x1099bc` | `?` | `?` |
| `bl` at 0xbbf58 | `(unknown callee) 0x21010` | `[sp-20]` | `0x1099d8` | `?` | `?` |
| `bl` at 0xbbf80 | `(unknown callee) 0x21010` | `[sp-20]` | `0x1099e8` | `?` | `?` |
| `bl` at 0xbbfa8 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a04` | `?` | `?` |
| `bl` at 0xbbfcc | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a14` | `?` | `?` |
| `bl` at 0xbbff4 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a30` | `?` | `?` |
| `bl` at 0xbc010 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a44` | `[[sp-32]+12]` | `[sp-32]+12` |
| `bl` at 0xbc034 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a5c` | `?` | `?` |
| `bl` at 0xbc0c8 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a70` | `?` | `?` |
| `bl` at 0xbc1e0 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109a94` | `?` | `?` |
| `bl` at 0xbc244 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109ac8` | `?` | `?` |
| `bl` at 0xbc260 | `(unknown callee) 0x21010` | `[sp-20]` | `0x109ad0` | `?` | `[sp-56]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xbbd44 | `r3 != 0x0` | `r3 == 0x0` |
| `bne` at 0xbbd5c | `r0 != 0x0` | `r0 == 0x0` |
| `beq` at 0xbbd7c | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xbc04c | `[[sp-32]+12] == 0x0` | `[[sp-32]+12] != 0x0` |
| `bne` at 0xbc058 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `blo` at 0xbc0e0 | `C == 0` | `not (C == 0)` |
| `beq` at 0xbc1fc | `[sp-56] == 0x0` | `[sp-56] != 0x0` |
| `blo` at 0xbc214 | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xbc264 | `r0` | `?` |

## `scan_multi_sic_ir`

- `scan_multi_sic_ir`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc0c28 to 0xc1004.
  Established. The opening call to `0xc0778` has candidates `sp-328`, unknown, unknown, `sp-136`
  (`bl` at 0xc0cec). Another call targets `0xbc8b4` with `0x0`, `[sp-32]`, `[sp]`, `[sp+4]` (`bl` at
  0xc0d28); its nonzero result takes the edge into further processing, while zero falls through
  to word `0x0` at `sp -0x158` (`bne` at 0xc0d34, `str` at 0xc0d3c). Two equality branches later
  compare unknown operands, each following its equality edge (`beq` at 0xc0db8, `beq` at 0xc0f74).
  The input loads are four bytes wide (`ldr` at 0xc0dac, `ldr` at 0xc0db0, `ldr` at 0xc0f48,
  `ldr` at 0xc0f64). The scan sequence and merged return meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc0c38 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc0c3c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc0c40 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xc0c44 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xc0c4c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0c50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0c54 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc0c64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0c68 | load | 4 | `abs 0xc0fec` | `-` |
| `ldr` at 0xc0c7c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc0c80 | store | 4 | `(unknown)` | `?` |
| `ldr` at 0xc0c84 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0ca0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc0ca4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0cc0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc0cc4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0ccc | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `ldr` at 0xc0ce0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc0ce4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc0cf4 | store | 4 | `sp -0x28` | `ret(0xc0778)` |
| `ldr` at 0xc0cf8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0cfc | store | 4 | `sp -0x168` | `ret(0xc0778)` |
| `str` at 0xc0d04 | store | 4 | `sp -0x164` | `sp-136` |
| `str` at 0xc0d0c | store | 4 | `sp -0x160` | `sp-232` |
| `ldr` at 0xc0d10 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xc0d14 | store | 4 | `sp -0x15c` | `[sp+12]` |
| `ldr` at 0xc0d1c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc0d20 | load | 4 | `sp +0x0` | `-` |
| `ldr` at 0xc0d24 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xc0d3c | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xc0d44 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc0d50 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0d58 | store | 4 | `sp -0x14c` | `?` |
| `ldr` at 0xc0d5c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc0d64 | store | 4 | `sp -0x150` | `?` |
| `str` at 0xc0d6c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0d70 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0d74 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xc0d84 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0d9c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0da4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc0dac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0db0 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0dbc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0dc4 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc0dcc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0de4 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0de8 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xc0dec | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc0df0 | load | 4 | `abs 0xc0ff0` | `-` |
| `ldr` at 0xc0df4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0df8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xc0e04 | store | 4 | `sp -0x158` | `0x0` |
| `ldr` at 0xc0e0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0e14 | store | 4 | `sp -0x24` | `[sp-36]+1` |
| `str` at 0xc0e20 | store | 4 | `sp -0x154` | `0x0` |
| `str` at 0xc0e28 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc0e2c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0e30 | load | 4 | `sp -0x150` | `-` |
| `ldr` at 0xc0e44 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xc0e4c | load | 4 | `sp -0x154` | `-` |
| `str` at 0xc0e54 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0e58 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xc0e60 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xc0e68 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0e6c | load | 4 | `abs 0xc0ff4` | `-` |
| `ldr` at 0xc0e78 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0e80 | load | 4 | `abs 0xc0ff4` | `-` |
| `str` at 0xc0e8c | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0e90 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0e94 | load | 4 | `abs 0xc0ff8` | `-` |
| `ldr` at 0xc0ea0 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0ea8 | load | 4 | `abs 0xc0ff8` | `-` |
| `str` at 0xc0eb4 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0eb8 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0ebc | load | 4 | `abs 0xc0ffc` | `-` |
| `ldr` at 0xc0ec8 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0ed0 | load | 4 | `abs 0xc0ffc` | `-` |
| `str` at 0xc0edc | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0ee0 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0ef0 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xc0f04 | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0f08 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0f18 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xc0f2c | store | 4 | `sp -0x154` | `?` |
| `ldr` at 0xc0f30 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xc0f48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0f4c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0f54 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xc0f5c | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc0f64 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0f68 | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0f78 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xc0f80 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xc0f88 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0f8c | load | 4 | `sp -0x154` | `-` |
| `ldr` at 0xc0f94 | load | 4 | `sp -0x14c` | `-` |
| `ldr` at 0xc0fac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc0fb0 | load | 4 | `sp -0x154` | `-` |
| `str` at 0xc0fb8 | store | 4 | `sp -0x168` | `?` |
| `ldr` at 0xc0fbc | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xc0fc0 | load | 4 | `abs 0xc1000` | `-` |
| `ldr` at 0xc0fc4 | load | 4 | `sp -0x14c` | `-` |
| `str` at 0xc0fd4 | store | 4 | `sp -0x158` | `0x0` |
| `str` at 0xc0fe0 | store | 4 | `sp -0x158` | `0x1` |
| `ldr` at 0xc0fe4 | load | 4 | `sp -0x158` | `-` |
| `ldmdb` at 0xc0fe8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc0fe8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc0fe8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc0cec | `(unknown callee) 0xc0778` | `sp-328` | `?` | `?` | `sp-136` |
| `bl` at 0xc0d28 | `(unknown callee) 0xbc8b4` | `0x0` | `[sp-32]` | `[sp]` | `[sp+4]` |
| `bl` at 0xc0dfc | `(unknown callee) 0x18c74` | `[sp+12]` | `0x109cdc` | `[sp-36]` | `?` |
| `bl` at 0xc0fcc | `(unknown callee) 0x18c74` | `[sp+12]` | `0x109d2c` | `[sp-332]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blo` at 0xc0c5c | `C == 0` | `not (C == 0)` |
| `bne` at 0xc0d34 | `ret(0xbc8b4) != 0x0` | `ret(0xbc8b4) == 0x0` |
| `beq` at 0xc0d4c | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `blo` at 0xc0d7c | `C == 0` | `not (C == 0)` |
| `beq` at 0xc0db8 | `? == ?` | `? != ?` |
| `blo` at 0xc0e38 | `C == 0` | `not (C == 0)` |
| `beq` at 0xc0f74 | `? == ?` | `? != ?` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc0fe8 | `r0` | `?` |

## `sn1sc_process_scancmd`

- `sn1sc_process_scancmd`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc2d70 to 0xc44c0.
  Established. An unsigned branch takes its bypass for `[0x27eb98] <= 0x2` and falls through to
  a call to `0xbbd1c` for a larger value (`bls` at 0xc2d90, `bl` at 0xc2da4). A later explicit-link
  call has unknown target and candidates unknown, unknown, `[0x11584c]`, unknown (`mov` at 0xc2e60);
  its zero result takes the edge to the computed-transfer path, while nonzero falls through
  to calls to `0x18c74` and `0xc2d08` (`beq` at 0xc2e6c, `bl` at 0xc2eac, `bl` at 0xc2eb0). The
  computed transfer has taken condition `? <= 0x13` unsigned and falls through for a greater value,
  with destinations unresolved (`ldrls` at 0xc2eec). Known-edge word stores include unknown values
  at `[sp-24] +0x0` (`str` at 0xc2ec4, `str` at 0xc4438). Complete command selection and
  excluded-path call arguments are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc2d80 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xc2d84 | load | 4 | `abs 0xc3d1c` | `-` |
| `ldr` at 0xc2d88 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc2d98 | load | 4 | `abs 0xc3ce8` | `-` |
| `ldr` at 0xc2da0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2dac | load | 4 | `abs 0xc3d1c` | `-` |
| `ldr` at 0xc2db0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc2dc0 | load | 4 | `abs 0xc3cec` | `-` |
| `ldr` at 0xc2dc8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2dd0 | load | 4 | `abs 0xc3cf0` | `-` |
| `ldr` at 0xc2dd4 | load | 4 | `abs 0x27eb94` | `-` |
| `str` at 0xc2de4 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc2de8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2dec | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc2df8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2dfc | load | 4 | `(unknown)` | `-` |
| `str` at 0xc2e08 | store | 4 | `(unknown)` | `?` |
| `str` at 0xc2e10 | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xc2e1c | load | 4 | `abs 0xc3cf4` | `-` |
| `ldr` at 0xc2e24 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2e2c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc2e3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2e44 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc2e50 | load | 4 | `abs 0x11584c` | `-` |
| `ldr` at 0xc2e74 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2e7c | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc2e8c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2e94 | load | 4 | `[sp-24] +0x4` | `-` |
| `ldr` at 0xc2ea0 | load | 4 | `abs 0xc3d14` | `-` |
| `ldr` at 0xc2ea4 | load | 4 | `abs 0xc3cf8` | `-` |
| `ldr` at 0xc2eb4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2eb8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2ebc | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xc2ec4 | store | 4 | `[sp-24] +0x0` | `?` |
| `str` at 0xc2ecc | store | 4 | `sp -0x5c` | `0x0` |
| `ldr` at 0xc2ed4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc2ed8 | load | 4 | `[sp-24] +0x0` | `-` |
| `ldrls` at 0xc2eec | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc4400 | load | 4 | `abs 0xc3d14` | `-` |
| `ldr` at 0xc4404 | load | 4 | `abs 0xc3d0c` | `-` |
| `str` at 0xc4410 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xc4418 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc441c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4428 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc442c | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xc4438 | store | 4 | `[sp-24] +0x0` | `?` |
| `ldr` at 0xc443c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4448 | load | 4 | `abs 0xc3d1c` | `-` |
| `ldr` at 0xc444c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc4458 | load | 4 | `abs 0xc3d10` | `-` |
| `ldr` at 0xc445c | load | 4 | `abs 0xc3d14` | `-` |
| `ldr` at 0xc4464 | load | 4 | `abs 0xc3d1c` | `-` |
| `ldr` at 0xc4468 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc4478 | load | 4 | `abs 0xc3d18` | `-` |
| `ldr` at 0xc4480 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc448c | load | 4 | `abs 0xc3d1c` | `-` |
| `ldr` at 0xc4490 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc44a0 | load | 4 | `abs 0xc3d20` | `-` |
| `ldr` at 0xc44a8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc44b0 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xc44b4 | store | 4 | `sp -0x5c` | `[sp-28]` |
| `ldr` at 0xc44b8 | load | 4 | `sp -0x5c` | `-` |
| `ldmdb` at 0xc44bc | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc44bc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc44bc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc44bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc2da4 | `(unknown callee) 0xbbd1c` | `0x0` | `0x109ecc` | `0x1` | `r0` |
| `bl` at 0xc2dcc | `(unknown callee) 0xbbd1c` | `0x0` | `0x109ef0` | `0x0` | `r0` |
| `bl` at 0xc2e18 | `(unknown callee) 0xc2c9c` | `?` | `?` | `?` | `[0x27eb94]` |
| `mov` at 0xc2e60 | `(unknown callee) (unknown target)` | `?` | `?` | `[0x11584c]` | `?` |
| `bl` at 0xc2eac | `(unknown callee) 0x18c74` | `0x2842a8` | `0x109f14` | `?` | `?` |
| `bl` at 0xc2eb0 | `(unknown callee) 0xc2d08` | `ret(0x18c74)` | `?` | `?` | `?` |
| `bl` at 0xc4408 | `(unknown callee) 0x18cd4` | `0x2842a8` | `0x109f58` | `?` | `?` |
| `bl` at 0xc4414 | `(unknown callee) 0xc2d08` | `ret(0x18cd4)` | `?` | `?` | `0x0` |
| `bl` at 0xc4460 | `(unknown callee) 0x20f14` | `0x109f84` | `0x2842a8` | `?` | `[0x27eb98]` |
| `bl` at 0xc4484 | `(unknown callee) 0xbbd1c` | `0x0` | `0x109f98` | `0x1` | `[sp-24]` |
| `bl` at 0xc44ac | `(unknown callee) 0xbbd1c` | `0x0` | `0x109fc0` | `0x0` | `[sp-24]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc2d90 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xc2db8 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |
| `bne` at 0xc2ddc | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `beq` at 0xc2e6c | `ret(unknown) == 0x0` | `ret(unknown) != 0x0` |
| `ldrls` at 0xc2eec | `? <= 0x13 (unsigned, 32-bit)` | `? > 0x13 (unsigned, 32-bit)` |
| `bne` at 0xc4444 | `[sp-28] != 0x0` | `[sp-28] == 0x0` |
| `beq` at 0xc4454 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bls` at 0xc4470 | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `bls` at 0xc4498 | `[0x27eb98] <= 0x1 (unsigned, 32-bit)` | `[0x27eb98] > 0x1 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc44bc | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc2eec).

## `sn1sc_reset_sic`

- `sn1sc_reset_sic`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc4a90 to 0xc4f18.
  Established. A nonzero word at absolute `0x27eb94` takes the edge into the longer body, while zero
  falls through to a stack store of `0x0` (`bne` at 0xc4ac4, `str` at 0xc4acc). The longer body
  includes six halfword stores at `[0x2842a0] +0x0`, each with unknown value (`strh` at 0xc4c5c,
  `strh` at 0xc4ce4, `strh` at 0xc4d68, `strh` at 0xc4dec, `strh` at 0xc4e70, `strh` at 0xc4ef4).
  Intermediate stack word stores include `0x38`, `0x28`, `0x38` and `0x30` at `sp -0x14` (`str` at
  0xc4cec, `str` at 0xc4d70, `str` at 0xc4df4, `str` at 0xc4e78). Their bit-to-signal mapping and
  electrical reset effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc4aa0 | load | 4 | `abs 0xc4f08` | `-` |
| `ldr` at 0xc4aa4 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc4ab0 | load | 4 | `abs 0xc4f0c` | `-` |
| `ldr` at 0xc4ab8 | load | 4 | `abs 0xc4f10` | `-` |
| `ldr` at 0xc4abc | load | 4 | `abs 0x27eb94` | `-` |
| `str` at 0xc4acc | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xc4ad4 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4ad8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4adc | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4af8 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4afc | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4b00 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4b24 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4b28 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4b2c | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4b50 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4b54 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4b58 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4b7c | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4b80 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4b84 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `ldr` at 0xc4ba8 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4bac | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4bb0 | load | 2 | `[0x2842a0] +0x0` | `-` |
| `str` at 0xc4bd4 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc4bd8 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4be0 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc4be4 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4be8 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4bec | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4bf8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4c0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4c20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4c34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4c48 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4c5c | store | 2 | `[0x2842a0] +0x0` | `?` |
| `ldr` at 0xc4c60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc4c68 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc4c6c | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4c70 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4c74 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4c80 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4c94 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4cbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4cd0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4ce4 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `str` at 0xc4cec | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xc4cf0 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4cf4 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4cf8 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4d04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d18 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d2c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d40 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d54 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4d68 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `str` at 0xc4d70 | store | 4 | `sp -0x14` | `0x28` |
| `ldr` at 0xc4d74 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4d78 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4d7c | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4d88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4d9c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4db0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4dc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4dd8 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4dec | store | 2 | `[0x2842a0] +0x0` | `?` |
| `str` at 0xc4df4 | store | 4 | `sp -0x14` | `0x38` |
| `ldr` at 0xc4df8 | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4dfc | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4e00 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4e0c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4e20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4e34 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4e48 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4e5c | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4e70 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `str` at 0xc4e78 | store | 4 | `sp -0x14` | `0x30` |
| `ldr` at 0xc4e7c | load | 4 | `abs 0xc4f14` | `-` |
| `ldr` at 0xc4e80 | load | 4 | `abs 0x2842a0` | `-` |
| `ldrh` at 0xc4e84 | load | 2 | `sp -0x14` | `-` |
| `ldr` at 0xc4e90 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ea4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4eb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ecc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc4ee0 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xc4ef4 | store | 2 | `[0x2842a0] +0x0` | `?` |
| `str` at 0xc4efc | store | 4 | `sp -0x18` | `0x1` |
| `ldr` at 0xc4f00 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xc4f04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4f04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4f04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc4ab4 | `(unknown callee) 0x20f14` | `0x10a004` | `r1` | `r2` | `[0x27eb98]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc4aac | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bne` at 0xc4ac4 | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4f04 | `r0` | `?` |

## `sn1sc_reset_brick`

- `sn1sc_reset_brick`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc478c to 0xc4a90.
  Established. A nonzero word at absolute `0x27eb94` takes the branch to further processing,
  while zero falls through to a word store of `0xfffff7cc` at an unresolved effective target
  (`bne` at 0xc47d4, `str` at 0xc47e0). An explicit-link call has unknown target and candidates
  unknown, `0x1`, `0x2842a8`, `[0x27eea8]` (`mov` at 0xc481c); its result is written at `[sp-24]
  +0x0` (`str` at 0xc4828). The later computed transfer takes its unresolved edge for `[sp-20]-37 <=
  0x34` unsigned and falls through for a greater value (`ldrls` at 0xc485c). Brick-specific
  selection and electrical reset semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc479c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc47a0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xc47a8 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xc47ac | load | 4 | `abs 0xc4a74` | `-` |
| `ldr` at 0xc47b0 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc47bc | load | 4 | `abs 0xc4a78` | `-` |
| `ldr` at 0xc47c0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc47c8 | load | 4 | `abs 0xc4a7c` | `-` |
| `ldr` at 0xc47cc | load | 4 | `abs 0x27eb94` | `-` |
| `ldr` at 0xc47d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc47dc | load | 4 | `abs 0xc4a80` | `-` |
| `str` at 0xc47e0 | store | 4 | `(unknown)` | `0xfffff7cc` |
| `str` at 0xc47e8 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xc47f4 | load | 4 | `abs 0xc4a84` | `-` |
| `ldr` at 0xc47f8 | load | 4 | `abs 0x27eea8` | `-` |
| `ldr` at 0xc4804 | load | 4 | `abs 0xc4a84` | `-` |
| `ldr` at 0xc4808 | load | 4 | `abs 0x27eea8` | `-` |
| `ldr` at 0xc4814 | load | 4 | `abs 0xc4a88` | `-` |
| `ldr` at 0xc4824 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc4828 | store | 4 | `[sp-24] +0x0` | `ret(unknown)` |
| `ldr` at 0xc482c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4830 | load | 4 | `[sp-24] +0x0` | `-` |
| `str` at 0xc4840 | store | 4 | `sp -0x48` | `?` |
| `str` at 0xc484c | store | 4 | `sp -0x1c` | `0x18` |
| `ldr` at 0xc4850 | load | 4 | `sp -0x14` | `-` |
| `ldrls` at 0xc485c | load | 4 | `(unknown)` | `-` |
| `str` at 0xc49f0 | store | 4 | `sp -0x30` | `0x1` |
| `ldr` at 0xc49f4 | load | 4 | `sp -0x30` | `-` |
| `str` at 0xc4a04 | store | 4 | `sp -0x5c` | `0x0` |
| `str` at 0xc4a0c | store | 4 | `sp -0x58` | `0x0` |
| `str` at 0xc4a14 | store | 4 | `sp -0x54` | `0x0` |
| `str` at 0xc4a1c | store | 4 | `sp -0x50` | `0x0` |
| `ldr` at 0xc4a20 | load | 4 | `abs 0xc4a88` | `-` |
| `str` at 0xc4a24 | store | 4 | `sp -0x4c` | `0x2842a8` |
| `ldr` at 0xc4a28 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xc4a2c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xc4a30 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xc4a48 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc4a4c | load | 4 | `abs 0xc4a8c` | `-` |
| `str` at 0xc4a50 | store | 4 | `[sp-24] +0x0` | `0xfffff7cb` |
| `str` at 0xc4a58 | store | 4 | `sp -0x48` | `0x0` |
| `str` at 0xc4a68 | store | 4 | `sp -0x48` | `0x1` |
| `ldr` at 0xc4a6c | load | 4 | `sp -0x48` | `-` |
| `ldmdb` at 0xc4a70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc4a70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc4a70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc47c4 | `(unknown callee) 0x20f14` | `0x109fe4` | `r0` | `r2` | `[0x27eb98]` |
| `bl` at 0xc47f0 | `(unknown callee) 0xc4a90` | `?` | `?` | `?` | `[0x27eb94]` |
| `mov` at 0xc481c | `(unknown callee) (unknown target)` | `?` | `0x1` | `0x2842a8` | `[0x27eea8]` |
| `bl` at 0xc4a38 | `(unknown callee) 0xbce44` | `0x18` | `[sp-52]` | `[sp-56]` | `0x0` |
| `bl` at 0xc4a60 | `(unknown callee) 0xc4a90` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xc47b8 | `[0x27eb98] == 0x0` | `[0x27eb98] != 0x0` |
| `bne` at 0xc47d4 | `[0x27eb94] != 0x0` | `[0x27eb94] == 0x0` |
| `beq` at 0xc4800 | `[0x27eea8] == 0x0` | `[0x27eea8] != 0x0` |
| `ldrls` at 0xc485c | `[sp-20]-37 <= 0x34 (unsigned, 32-bit)` | `[sp-20]-37 > 0x34 (unsigned, 32-bit)` |
| `bne` at 0xc49fc | `0x1 != 0x0` | `0x1 == 0x0` |
| `bne` at 0xc4a44 | `ret(0xbce44) != 0x0` | `ret(0xbce44) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc4a70 | `r0` | `?` |

Coverage excludes unresolved edges from the computed transfer (`ldrls` at 0xc485c).

## `JTAG_Cfg_port_init`

- `JTAG_Cfg_port_init`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc65e8 to 0xc69e8.
  Established. The opening conditional branch takes its edge for entry `r2 != 0x0` and falls through
  for zero (`bne` at 0xc6610). Explicit stores include zero words and a word `0x1` at unresolved
  effective targets (`str` at 0xc668c, `str` at 0xc66ec, `str` at 0xc6774, `str` at 0xc68fc, `str`
  at 0xc6928). A call to `0x18920` has candidates unknown, `0xc`, `[sp-24]`, unknown, and its result
  is stored as a word at an unresolved target (`bl` at 0xc67d8, `str` at 0xc67e0). Two byte stores
  also have unknown effective targets and unknown values (`strb` at 0xc687c, `strb` at 0xc68c8). The
  final calls target `0xc69e8` with `[sp-24]`, `0x14`, unknown, `[sp]`, then `0xc91a0` with first
  candidate `ret(0xc69e8)` and the rest unknown (`bl` at 0xc69b4, `bl` at 0xc69b8). The configured
  port records and physical effects are unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc65f8 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xc65fc | store | 4 | `sp -0x1c` | `r1` |
| `str` at 0xc6600 | store | 4 | `sp -0x20` | `r2` |
| `strb` at 0xc6604 | store | 1 | `sp -0x21` | `(r3 & 0xff)` |
| `ldr` at 0xc6608 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6614 | load | 4 | `abs 0xc69cc` | `-` |
| `str` at 0xc6618 | store | 4 | `sp -0x38` | `0x10a0dc` |
| `ldr` at 0xc6620 | load | 4 | `abs 0xc69d0` | `-` |
| `ldr` at 0xc6630 | load | 4 | `abs 0xc69d4` | `-` |
| `ldr` at 0xc6634 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6640 | load | 4 | `abs 0xc69d8` | `-` |
| `str` at 0xc6644 | store | 4 | `sp -0x38` | `0x10a108` |
| `ldr` at 0xc6648 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc664c | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc6650 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6654 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xc665c | load | 4 | `abs 0xc69d0` | `-` |
| `ldr` at 0xc666c | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6670 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc668c | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc6690 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6694 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc66b4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc66b8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc66c4 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc66c8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc66ec | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc66f0 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc66f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6714 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6720 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6724 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6744 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc674c | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6750 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6774 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc6778 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc677c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc679c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc67a8 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc67ac | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc67cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc67e0 | store | 4 | `(unknown)` | `ret(0x18920)` |
| `ldr` at 0xc67e4 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc67f0 | load | 4 | `abs 0xc69e0` | `-` |
| `str` at 0xc67f4 | store | 4 | `sp -0x38` | `0x10a0ac` |
| `ldr` at 0xc67fc | load | 4 | `abs 0xc69d0` | `-` |
| `ldr` at 0xc680c | load | 4 | `abs 0xc69e4` | `-` |
| `str` at 0xc6810 | store | 4 | `sp -0x2c` | `0xfffff7cc` |
| `str` at 0xc681c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xc6820 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc6824 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xc6834 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6858 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc6860 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc6868 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc6870 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc6878 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc687c | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc6880 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6884 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc68a4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc68ac | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xc68b4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xc68bc | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xc68c4 | load | 1 | `(unknown)` | `-` |
| `strb` at 0xc68c8 | store | 1 | `(unknown)` | `?` |
| `ldr` at 0xc68cc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc68d4 | store | 4 | `sp -0x28` | `[sp-40]+1` |
| `ldr` at 0xc68dc | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc68e0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc68fc | store | 4 | `(unknown)` | `0x1` |
| `ldr` at 0xc6900 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6904 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6928 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xc692c | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6950 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xc6954 | store | 4 | `(unknown)` | `[sp-32]` |
| `ldr` at 0xc6958 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc695c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xc697c | load | 1 | `sp -0x21` | `-` |
| `strb` at 0xc6980 | store | 1 | `(unknown)` | `([sp-33]:1 & 0xff)` |
| `ldr` at 0xc6984 | load | 4 | `abs 0xc69dc` | `-` |
| `ldr` at 0xc6988 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc69a8 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xc69ac | store | 4 | `(unknown)` | `[sp]` |
| `ldr` at 0xc69b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc69c0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xc69c4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xc69c8 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xc69c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc69c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc69c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc662c | `(unknown callee) 0x1eed0` | `0x9` | `0x10a068` | `0xd2` | `0x0` |
| `bl` at 0xc6668 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0xd5` | `0x0` |
| `bl` at 0xc6748 | `(unknown callee) 0x18be4` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xc67d8 | `(unknown callee) 0x18920` | `?` | `0xc` | `[sp-24]` | `?` |
| `bl` at 0xc6808 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a068` | `0xea` | `0x0` |
| `bl` at 0xc69b4 | `(unknown callee) 0xc69e8` | `[sp-24]` | `0x14` | `?` | `[sp]` |
| `bl` at 0xc69b8 | `(unknown callee) 0xc91a0` | `ret(0xc69e8)` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc6610 | `r2 != 0x0` | `r2 == 0x0` |
| `bls` at 0xc663c | `[0x27eb98] <= 0x2 (unsigned, 32-bit)` | `[0x27eb98] > 0x2 (unsigned, 32-bit)` |
| `beq` at 0xc66c0 | `? == [sp-32]` | `? != [sp-32]` |
| `beq` at 0xc671c | `? == 0x0` | `? != 0x0` |
| `bne` at 0xc67a4 | `? != 0x0` | `? == 0x0` |
| `bne` at 0xc67ec | `? != 0x0` | `? == 0x0` |
| `blo` at 0xc682c | `C == 0` | `not (C == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc69c8 | `r0` | `?` |

## `JTAG_Cfg_set_ports_f`

- `JTAG_Cfg_set_ports_f`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc6b70 to 0xc6d3c.
  Established. The body writes unknown word value at absolute `0x115844` and `[sp-24]` at absolute
  `0x27eba4` (`str` at 0xc6c98, `str` at 0xc6ca4). The following call to `0xc2c3c` has candidates
  `[0x115844]:1`, `[0x27eba4]:1`, `[0x115844]:1`, `[0x27eba4]:1` (`bl` at 0xc6cc0). A prior
  inequality branch takes its edge for `[0x115844] != [sp-20]` and falls through for equality (`bne`
  at 0xc6bfc). The port-mask interpretation and the call’s hardware effect remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc6b80 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc6b84 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xc6b88 | load | 4 | `abs 0xc6d18` | `-` |
| `ldr` at 0xc6b8c | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6b98 | load | 4 | `abs 0xc6d1c` | `-` |
| `str` at 0xc6b9c | store | 4 | `sp -0x38` | `0x10a150` |
| `ldr` at 0xc6ba0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6ba4 | store | 4 | `sp -0x34` | `r0` |
| `ldr` at 0xc6ba8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6bac | store | 4 | `sp -0x30` | `r1` |
| `ldr` at 0xc6bb4 | load | 4 | `abs 0xc6d20` | `-` |
| `ldr` at 0xc6bc4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6bd4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6bdc | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc6be0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6be8 | store | 4 | `sp -0x14` | `?` |
| `ldr` at 0xc6bec | load | 4 | `abs 0xc6d24` | `-` |
| `ldr` at 0xc6bf0 | load | 4 | `abs 0x115844` | `-` |
| `ldr` at 0xc6bf4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6c00 | load | 4 | `abs 0xc6d28` | `-` |
| `ldr` at 0xc6c04 | load | 4 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc6c08 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6c18 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc6c20 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xc6c2c | load | 4 | `abs 0xc6d2c` | `-` |
| `str` at 0xc6c30 | store | 4 | `sp -0x38` | `0x10a184` |
| `ldr` at 0xc6c34 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6c38 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xc6c3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6c40 | store | 4 | `sp -0x30` | `[sp-20]` |
| `ldr` at 0xc6c48 | load | 4 | `abs 0xc6d20` | `-` |
| `ldr` at 0xc6c4c | load | 4 | `abs 0xc6d30` | `-` |
| `ldr` at 0xc6c58 | load | 4 | `abs 0xc6d34` | `-` |
| `str` at 0xc6c5c | store | 4 | `sp -0x24` | `0xfffff7c3` |
| `ldr` at 0xc6c64 | load | 4 | `abs 0xc6d24` | `-` |
| `str` at 0xc6c68 | store | 4 | `sp -0x28` | `0x115844` |
| `ldr` at 0xc6c6c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xc6c70 | store | 4 | `sp -0x2c` | `[sp-20]` |
| `ldr` at 0xc6c74 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc6c84 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6c8c | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xc6c90 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xc6c94 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xc6c98 | store | 4 | `abs 0x115844` | `?` |
| `ldr` at 0xc6c9c | load | 4 | `abs 0xc6d28` | `-` |
| `ldr` at 0xc6ca0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xc6ca4 | store | 4 | `abs 0x27eba4` | `[sp-24]` |
| `ldr` at 0xc6ca8 | load | 4 | `abs 0xc6d24` | `-` |
| `ldrb` at 0xc6cac | load | 1 | `abs 0x115844` | `-` |
| `ldr` at 0xc6cb0 | load | 4 | `abs 0xc6d28` | `-` |
| `ldrb` at 0xc6cb4 | load | 1 | `abs 0x27eba4` | `-` |
| `ldr` at 0xc6cc4 | load | 4 | `abs 0xc6d18` | `-` |
| `ldr` at 0xc6cc8 | load | 4 | `abs 0x27eb98` | `-` |
| `ldr` at 0xc6cd4 | load | 4 | `abs 0xc6d38` | `-` |
| `str` at 0xc6cd8 | store | 4 | `sp -0x38` | `0x10a1c4` |
| `ldr` at 0xc6cdc | load | 4 | `abs 0xc6d24` | `-` |
| `ldr` at 0xc6ce0 | load | 4 | `abs 0x115844` | `-` |
| `str` at 0xc6ce4 | store | 4 | `sp -0x34` | `[0x115844]` |
| `ldr` at 0xc6ce8 | load | 4 | `abs 0xc6d28` | `-` |
| `ldr` at 0xc6cec | load | 4 | `abs 0x27eba4` | `-` |
| `str` at 0xc6cf0 | store | 4 | `sp -0x30` | `[0x27eba4]` |
| `ldr` at 0xc6cf8 | load | 4 | `abs 0xc6d20` | `-` |
| `str` at 0xc6d0c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xc6d10 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xc6d14 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc6d14 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc6d14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc6bc0 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x128` | `0x0` |
| `bl` at 0xc6c54 | `(unknown callee) 0x1eed0` | `0x3` | `0x10a068` | `0x139` | `0x0` |
| `bl` at 0xc6cc0 | `(unknown callee) 0xc2c3c` | `[0x115844]:1` | `[0x27eba4]:1` | `[0x115844]:1` | `[0x27eba4]:1` |
| `bl` at 0xc6d04 | `(unknown callee) 0x1eed0` | `0x6` | `0x10a068` | `0x158` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bls` at 0xc6b94 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |
| `bne` at 0xc6bcc | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xc6bfc | `[0x115844] != [sp-20]` | `[0x115844] == [sp-20]` |
| `bne` at 0xc6c10 | `Z == 0` | `not (Z == 0)` |
| `bls` at 0xc6c28 | `ls flags condition (operands unknown)` | `not (ls flags condition (operands unknown))` |
| `bne` at 0xc6c7c | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `bls` at 0xc6cd0 | `[0x27eb98] <= 0x3 (unsigned, 32-bit)` | `[0x27eb98] > 0x3 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc6d14 | `r0` | `?` |

## `JTAG_Cfg_set_ports_stub`

- `JTAG_Cfg_set_ports_stub`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc7aa0 to 0xc7ac4.
  Established. The body saves entry r0 and r1 as words at `sp -0x14` and `sp -0x18` (`str` at
  0xc7ab0, `str` at 0xc7ab4). It sets r0 to `0x0` and returns that value (`mov` at 0xc7abc, `ldmdb`
  at 0xc7ac0). No calls or conditional branches are reported. A physical port-setting operation is
  not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xc7ab0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xc7ab4 | store | 4 | `sp -0x18` | `r1` |
| `ldmdb` at 0xc7ac0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc7ac0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc7ac0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc7ac0 | `r0` | `0x0` |

## `scanpktInit`

- `scanpktInit`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xc8724 to 0xc88c8.
  Established. The body increments the word at absolute `0x27ebb0` (`str` at 0xc878c). A call to
  `0x9a8f8` has candidates `0x28448c`, `0x10a724`, `0x14`, `[0x27ebb0]+1` (`bl` at 0xc879c). A later
  call to `0x1f498` uses `0x284488`, `0x10a758`, `0xc88c8`, `0x0` (`bl` at 0xc882c); its zero result
  takes the edge to a call to `0x99520`, while nonzero falls through to the return (`beq` at
  0xc8838, `bl` at 0xc8854). The numeric addresses’ queue and task roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldr` at 0xc874c | load | 4 | `abs 0xc889c` | `-` |
| `ldr` at 0xc8750 | load | 4 | `abs 0x27ebb0` | `-` |
| `ldr` at 0xc875c | load | 4 | `abs 0xc88a0` | `-` |
| `str` at 0xc8760 | store | 4 | `sp -0x2c` | `0x10a6e0` |
| `ldr` at 0xc8768 | load | 4 | `abs 0xc88a4` | `-` |
| `ldr` at 0xc877c | load | 4 | `abs 0xc889c` | `-` |
| `ldr` at 0xc8780 | load | 4 | `abs 0xc889c` | `-` |
| `ldr` at 0xc8784 | load | 4 | `abs 0x27ebb0` | `-` |
| `str` at 0xc878c | store | 4 | `abs 0x27ebb0` | `[0x27ebb0]+1` |
| `ldr` at 0xc8790 | load | 4 | `abs 0xc88a8` | `-` |
| `ldr` at 0xc8794 | load | 4 | `abs 0xc88ac` | `-` |
| `str` at 0xc87a4 | store | 4 | `sp -0x14` | `ret(0x9a8f8)` |
| `ldr` at 0xc87a8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc87b4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc87c0 | load | 4 | `abs 0xc88b0` | `-` |
| `str` at 0xc87c4 | store | 4 | `sp -0x2c` | `0x10a730` |
| `str` at 0xc87c8 | store | 4 | `sp -0x28` | `ret(0x1e71c)` |
| `ldr` at 0xc87d0 | load | 4 | `abs 0xc88a4` | `-` |
| `ldr` at 0xc87e4 | load | 4 | `abs 0xc88b4` | `-` |
| `str` at 0xc87f8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xc8800 | store | 4 | `sp -0x28` | `0x800` |
| `str` at 0xc8808 | store | 4 | `sp -0x24` | `0x19` |
| `str` at 0xc8810 | store | 4 | `sp -0x20` | `0xa` |
| `str` at 0xc8818 | store | 4 | `sp -0x1c` | `0x4` |
| `ldr` at 0xc881c | load | 4 | `abs 0xc88b8` | `-` |
| `ldr` at 0xc8820 | load | 4 | `abs 0xc88bc` | `-` |
| `ldr` at 0xc8824 | load | 4 | `abs 0xc88c0` | `-` |
| `ldr` at 0xc8840 | load | 4 | `abs 0xc88b4` | `-` |
| `ldr` at 0xc8844 | load | 4 | `abs 0xc88b4` | `-` |
| `ldr` at 0xc8848 | load | 4 | `abs 0x284480` | `-` |
| `ldr` at 0xc8850 | load | 4 | `abs 0xc88a8` | `-` |
| `str` at 0xc885c | store | 4 | `sp -0x14` | `ret(0x99520)` |
| `ldr` at 0xc8860 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc886c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xc8878 | load | 4 | `abs 0xc88c4` | `-` |
| `str` at 0xc887c | store | 4 | `sp -0x2c` | `0x10a764` |
| `str` at 0xc8880 | store | 4 | `sp -0x28` | `ret(0x1e71c)` |
| `ldr` at 0xc8888 | load | 4 | `abs 0xc88a4` | `-` |
| `ldmdb` at 0xc8898 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xc8898 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xc8898 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xc8734 | `(unknown callee) 0x2b31c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xc8774 | `(unknown callee) 0x1eed0` | `0x4` | `0x10a70c` | `0x45` | `0x0` |
| `bl` at 0xc879c | `(unknown callee) 0x9a8f8` | `0x28448c` | `0x10a724` | `0x14` | `[0x27ebb0]+1` |
| `bl` at 0xc87b8 | `(unknown callee) 0x1e71c` | `ret(0x9a8f8)` | `?` | `?` | `ret(0x9a8f8)` |
| `bl` at 0xc87dc | `(unknown callee) 0x1eed0` | `0x4` | `0x10a70c` | `0x5d` | `0x0` |
| `bl` at 0xc87f0 | `(unknown callee) 0x9a7e0` | `0x284480` | `0xf` | `0x0` | `ret(0x9a8f8)` |
| `bl` at 0xc882c | `(unknown callee) 0x1f498` | `0x284488` | `0x10a758` | `0xc88c8` | `0x0` |
| `bl` at 0xc8854 | `(unknown callee) 0x99520` | `0x284480` | `[0x284480]` | `0x1` | `0x28448c` |
| `bl` at 0xc8870 | `(unknown callee) 0x1e71c` | `ret(0x99520)` | `?` | `?` | `ret(0x99520)` |
| `bl` at 0xc8894 | `(unknown callee) 0x1eed0` | `0x4` | `0x10a70c` | `0x77` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xc8744 | `? != 0x32` | `? == 0x32` |
| `beq` at 0xc8758 | `[0x27ebb0] == 0x0` | `[0x27ebb0] != 0x0` |
| `beq` at 0xc87b0 | `ret(0x9a8f8) == 0x0` | `ret(0x9a8f8) != 0x0` |
| `beq` at 0xc8838 | `ret(0x1f498) == 0x0` | `ret(0x1f498) != 0x0` |
| `beq` at 0xc8868 | `ret(0x99520) == 0x0` | `ret(0x99520) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xc8898 | `r0` | `?` |
