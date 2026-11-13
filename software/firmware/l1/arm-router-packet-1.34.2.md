# ARM router registration and packet entry paths in 1.34.2

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
Indirect calls can have unknown targets while retaining their continuation and argument candidates. No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.34.2-package3/router-packet-facts.txt) retain all tool sections.

## `IRouterPacketUseCount`

- `IRouterPacketUseCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x98d34 to 0x98e9c.
  Established. The halfword at `[sp-20] +0x4` receives `([sp-32]:2 & 0xffff)` (`strh` at 0x98de0).
  Calls to `0x1fb00` (`bl` at 0x98d54) and `0x1fe64` (`bl` at 0x98e34) both have first candidate
  `0x11b4b0`. The latter result being zero takes the branch past the following call, while nonzero
  falls through (`beq` at 0x98e48). The result is unknown at the exit (`ldmdb` at 0x98e78).
  Reference counting and synchronization meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x98d44 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x98d48 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x98d4c | load | 4 | `abs 0x98e7c` | `-` |
| `str` at 0x98d5c | store | 4 | `sp -0x1c` | `ret(0x1fb00)` |
| `ldr` at 0x98d60 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x98d6c | load | 4 | `abs 0x98e80` | `-` |
| `str` at 0x98d70 | store | 4 | `sp -0x2c` | `0xfce5c` |
| `ldr` at 0x98d74 | load | 4 | `abs 0x98e7c` | `-` |
| `str` at 0x98d78 | store | 4 | `sp -0x28` | `0x11b4b0` |
| `ldr` at 0x98d7c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x98d80 | store | 4 | `sp -0x24` | `ret(0x1fb00)` |
| `ldr` at 0x98d88 | load | 4 | `abs 0x98e84` | `-` |
| `ldr` at 0x98d98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98d9c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x98da0 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldrh` at 0x98dac | load | 2 | `sp -0x18` | `-` |
| `ldrh` at 0x98db4 | load | 2 | `[sp-20] +0x4` | `-` |
| `strh` at 0x98db8 | store | 2 | `sp -0x20` | `([[sp-20]+4]:2 & 0xffff)` |
| `ldr` at 0x98dbc | load | 4 | `abs 0x98e88` | `-` |
| `ldrh` at 0x98dc8 | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x98dd8 | store | 2 | `sp -0x20` | `?` |
| `ldrh` at 0x98ddc | load | 2 | `sp -0x20` | `-` |
| `strh` at 0x98de0 | store | 2 | `[sp-20] +0x4` | `([sp-32]:2 & 0xffff)` |
| `ldr` at 0x98de4 | load | 4 | `abs 0x98e8c` | `-` |
| `ldr` at 0x98de8 | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x98df4 | load | 4 | `abs 0x98e8c` | `-` |
| `ldr` at 0x98df8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98dfc | load | 4 | `abs 0x11b454` | `-` |
| `ldr` at 0x98e08 | load | 4 | `abs 0x98e90` | `-` |
| `ldr` at 0x98e0c | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x98e10 | load | 2 | `[sp-20] +0x4` | `-` |
| `ldr` at 0x98e20 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x98e24 | load | 4 | `abs 0x98e94` | `-` |
| `ldr` at 0x98e28 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x98e30 | load | 4 | `abs 0x98e7c` | `-` |
| `str` at 0x98e3c | store | 4 | `sp -0x1c` | `ret(0x1fe64)` |
| `ldr` at 0x98e40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x98e4c | load | 4 | `abs 0x98e98` | `-` |
| `str` at 0x98e50 | store | 4 | `sp -0x2c` | `0xfceb4` |
| `ldr` at 0x98e54 | load | 4 | `abs 0x98e7c` | `-` |
| `str` at 0x98e58 | store | 4 | `sp -0x28` | `0x11b4b0` |
| `ldr` at 0x98e5c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x98e60 | store | 4 | `sp -0x24` | `ret(0x1fe64)` |
| `ldr` at 0x98e68 | load | 4 | `abs 0x98e84` | `-` |
| `ldmdb` at 0x98e78 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x98e78 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x98e78 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x98d54 | `(unknown callee) 0x1fb00` | `0x11b4b0` | `?` | `r2` | `r3` |
| `bl` at 0x98d94 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x52` | `0x0` |
| `bl` at 0x98e2c | `(unknown callee) 0x21010` | `[0x116928]` | `0xfcea8` | `[sp-20]` | `?` |
| `bl` at 0x98e34 | `(unknown callee) 0x1fe64` | `0x11b4b0` | `?` | `?` | `?` |
| `bl` at 0x98e74 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x5a` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x98d68 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x98df0 | `[0x11b454] == 0x0` | `[0x11b454] != 0x0` |
| `bne` at 0x98e04 | `[sp-20] != [0x11b454]` | `[sp-20] == [0x11b454]` |
| `beq` at 0x98e48 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x98e78 | `r0` | `?` |

## `IRouterRegisterFunction`

- `IRouterRegisterFunction`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99478 to 0x99520.
  Established. Two calls to `0x98e9c` have candidates `(entry r0, entry r1, 0x1, 0x0)` (`bl` at
  0x994c8) and `(unknown, unknown, 0x2, 0x0)` (`bl` at 0x99508). Entry `r3` is stored in the stack
  word at `sp -0x28` before the first call (`str` at 0x994b4). The intermediate branch retains only
  `Z == 0` as its taken predicate (`bne` at 0x994dc). The callee target is numeric, but registration
  policy and callable-argument roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99488 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9948c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x99490 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x99494 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x9949c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x994a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x994b0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x994b4 | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x994b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x994bc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x994d0 | store | 4 | `sp -0x24` | `ret(0x98e9c)` |
| `ldr` at 0x994d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x994e0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x994f0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x994f4 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x994f8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x994fc | load | 4 | `sp -0x18` | `-` |
| `str` at 0x99510 | store | 4 | `sp -0x24` | `ret(0x98e9c)` |
| `ldr` at 0x99514 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x9951c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9951c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9951c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x994c8 | `(unknown callee) 0x98e9c` | `r0` | `r1` | `0x1` | `0x0` |
| `bl` at 0x99508 | `(unknown callee) 0x98e9c` | `?` | `?` | `0x2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x994ac | `? == 0x0` | `? != 0x0` |
| `bne` at 0x994dc | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x994ec | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9951c | `r0` | `?` |

## `IRouterRegisterQueue`

- `IRouterRegisterQueue`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99520 to 0x995c8.
  Established. The first call to `0x98e9c` uses candidates `(entry r0, entry r1, 0x1, 0x1)` (`bl` at
  0x99570). A second call to that target has candidates `(unknown, unknown, 0x2, 0x1)` (`bl` at
  0x995b0). The intermediate branch is taken for `Z == 0` and falls through otherwise (`bne` at
  0x99584). The return remains unknown (`ldmdb` at 0x995c4). Queue registration semantics are not
  established by the candidate values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99530 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x99534 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x99538 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9953c | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x99544 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0x99548 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99558 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9955c | store | 4 | `sp -0x28` | `r3` |
| `ldr` at 0x99560 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99564 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x99578 | store | 4 | `sp -0x24` | `ret(0x98e9c)` |
| `ldr` at 0x9957c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99588 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99598 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9959c | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x995a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x995a4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x995b8 | store | 4 | `sp -0x24` | `ret(0x98e9c)` |
| `ldr` at 0x995bc | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x995c4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x995c4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x995c4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99570 | `(unknown callee) 0x98e9c` | `r0` | `r1` | `0x1` | `0x1` |
| `bl` at 0x995b0 | `(unknown callee) 0x98e9c` | `?` | `?` | `0x2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x99554 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x99584 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x99594 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x995c4 | `r0` | `?` |

## `IRouterEventRegisterFunction`

- `IRouterEventRegisterFunction`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99904 to 0x9994c.
  Established. A stack word at `sp -0x24` receives entry `r3` (`str` at 0x99928). The call to
  `0x995c8` has candidates `(entry r0, entry r1, entry r2, 0x0)` (`bl` at 0x9993c); its result
  reaches the return (`ldmdb` at 0x99948). No conditional branch appears, and the callable role of
  the stored entry value remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99914 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x99918 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9991c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x99920 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x99924 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x99928 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x9992c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99930 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99934 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x99948 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x99948 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x99948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9993c | `(unknown callee) 0x995c8` | `r0` | `r1` | `r2` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x99948 | `r0` | `ret(0x995c8)` |

## `IRouterEventRegisterQueue`

- `IRouterEventRegisterQueue`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9994c to 0x99994.
  Established. Entry `r3` is saved in the word at `sp -0x24` (`str` at 0x99970). The call carries
  candidates `(entry r0, entry r1, entry r2, 0x1)` (`bl` at 0x99984), and `ret(0x995c8)` reaches the
  exit (`ldmdb` at 0x99990). The body records no conditional branch. Event queue ownership and
  matching rules are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9995c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x99960 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x99964 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x99968 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9996c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x99970 | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x99974 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99978 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9997c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0x99990 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x99990 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x99990 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99984 | `(unknown callee) 0x995c8` | `r0` | `r1` | `r2` | `0x1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x99990 | `r0` | `ret(0x995c8)` |

## `IRouterUnregister`

- `IRouterUnregister`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99994 to 0x99d04.
  Established. Word writes include `[[sp-36]+40]` at `0x282860` (`str` at 0x99b58) and
  `[[sp-48]+24]` at `0x2824a0` (`str` at 0x99c34). Calls to `0x18be4` have first candidates
  `[sp-36]` (`bl` at 0x99b88) and `[sp-48]` (`bl` at 0x99c64). The final call to `0x1fe64` uses
  first candidate `0x11b45c` (`bl` at 0x99c88); its zero result takes the branch past the following
  call, while nonzero falls through (`beq` at 0x99c9c). Removal, allocation and ownership semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x999a4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x999a8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x999ac | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0x999b0 | load | 4 | `abs 0x99cd8` | `-` |
| `ldr` at 0x999b4 | load | 4 | `abs 0x282860` | `-` |
| `str` at 0x999b8 | store | 4 | `sp -0x24` | `[0x282860]` |
| `str` at 0x999c0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x999c8 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x999cc | load | 4 | `abs 0x99cdc` | `-` |
| `ldr` at 0x999d0 | load | 4 | `abs 0x2824a0` | `-` |
| `str` at 0x999d4 | store | 4 | `sp -0x30` | `[0x2824a0]` |
| `str` at 0x999dc | store | 4 | `sp -0x34` | `0x0` |
| `str` at 0x999e4 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x999e8 | load | 4 | `abs 0x99ce0` | `-` |
| `str` at 0x999f8 | store | 4 | `sp -0x3c` | `ret(0x1fb00)` |
| `ldr` at 0x999fc | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x99a08 | load | 4 | `abs 0x99ce4` | `-` |
| `str` at 0x99a0c | store | 4 | `sp -0x48` | `0xfce5c` |
| `ldr` at 0x99a10 | load | 4 | `abs 0x99ce0` | `-` |
| `str` at 0x99a14 | store | 4 | `sp -0x44` | `0x11b45c` |
| `ldr` at 0x99a18 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x99a1c | store | 4 | `sp -0x40` | `ret(0x1fb00)` |
| `ldr` at 0x99a24 | load | 4 | `abs 0x99ce8` | `-` |
| `ldr` at 0x99a28 | load | 4 | `abs 0x99cec` | `-` |
| `ldr` at 0x99a34 | load | 4 | `abs 0x99cf0` | `-` |
| `ldr` at 0x99a38 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99a3c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99a40 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99a48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x99a5c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0x99a60 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99a70 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99a7c | load | 4 | `abs 0x99cf4` | `-` |
| `ldr` at 0x99a80 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99a98 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99a9c | load | 4 | `(unknown)` | `-` |
| `ldr` at 0x99aa0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x99aac | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x99ac0 | load | 4 | `abs 0x99cf4` | `-` |
| `ldr` at 0x99ad8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x99ae0 | store | 4 | `sp -0x20` | `?` |
| `ldr` at 0x99aec | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99afc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99b04 | load | 4 | `[sp-36] +0x0` | `-` |
| `ldr` at 0x99b08 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x99b14 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99b20 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b24 | load | 4 | `[sp-36] +0x4` | `-` |
| `ldr` at 0x99b28 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99b38 | load | 4 | `abs 0x99cd8` | `-` |
| `ldr` at 0x99b3c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b40 | load | 4 | `abs 0x282860` | `-` |
| `ldr` at 0x99b4c | load | 4 | `abs 0x99cd8` | `-` |
| `ldr` at 0x99b50 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b54 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0x99b58 | store | 4 | `abs 0x282860` | `[[sp-36]+40]` |
| `ldr` at 0x99b60 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x99b64 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b68 | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0x99b6c | store | 4 | `[sp-40] +0x28` | `[[sp-36]+40]` |
| `ldr` at 0x99b70 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x99b74 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0x99b78 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b7c | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0x99b80 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0x99b84 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x99b90 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x99b94 | store | 4 | `sp -0x28` | `[sp-36]` |
| `ldr` at 0x99b98 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x99b9c | load | 4 | `[sp-36] +0x28` | `-` |
| `str` at 0x99ba0 | store | 4 | `sp -0x24` | `[[sp-36]+40]` |
| `ldr` at 0x99ba8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99bb4 | load | 4 | `abs 0x99cf8` | `-` |
| `ldr` at 0x99bb8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99bbc | load | 4 | `abs 0x282460` | `-` |
| `ldr` at 0x99bc0 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x99bcc | load | 4 | `abs 0x99cf8` | `-` |
| `ldr` at 0x99bdc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x99bec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99bfc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c00 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x99c04 | load | 4 | `[sp-48] +0x0` | `-` |
| `ldr` at 0x99c08 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0x99c14 | load | 4 | `abs 0x99cdc` | `-` |
| `ldr` at 0x99c18 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c1c | load | 4 | `abs 0x2824a0` | `-` |
| `ldr` at 0x99c28 | load | 4 | `abs 0x99cdc` | `-` |
| `ldr` at 0x99c2c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c30 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0x99c34 | store | 4 | `abs 0x2824a0` | `[[sp-48]+24]` |
| `ldr` at 0x99c3c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x99c40 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c44 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0x99c48 | store | 4 | `[sp-52] +0x18` | `[[sp-48]+24]` |
| `ldr` at 0x99c4c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99c50 | store | 4 | `sp -0x38` | `[sp-48]` |
| `ldr` at 0x99c54 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c58 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0x99c5c | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0x99c60 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x99c6c | load | 4 | `sp -0x30` | `-` |
| `str` at 0x99c70 | store | 4 | `sp -0x34` | `[sp-48]` |
| `ldr` at 0x99c74 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x99c78 | load | 4 | `[sp-48] +0x18` | `-` |
| `str` at 0x99c7c | store | 4 | `sp -0x30` | `[[sp-48]+24]` |
| `ldr` at 0x99c84 | load | 4 | `abs 0x99ce0` | `-` |
| `str` at 0x99c90 | store | 4 | `sp -0x3c` | `ret(0x1fe64)` |
| `ldr` at 0x99c94 | load | 4 | `sp -0x3c` | `-` |
| `ldr` at 0x99ca0 | load | 4 | `abs 0x99cfc` | `-` |
| `str` at 0x99ca4 | store | 4 | `sp -0x48` | `0xfceb4` |
| `ldr` at 0x99ca8 | load | 4 | `abs 0x99ce0` | `-` |
| `str` at 0x99cac | store | 4 | `sp -0x44` | `0x11b45c` |
| `ldr` at 0x99cb0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x99cb4 | store | 4 | `sp -0x40` | `ret(0x1fe64)` |
| `ldr` at 0x99cbc | load | 4 | `abs 0x99ce8` | `-` |
| `ldr` at 0x99cc0 | load | 4 | `abs 0x99d00` | `-` |
| `ldmdb` at 0x99cd4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x99cd4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x99cd4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x999f0 | `(unknown callee) 0x1fb00` | `0x11b45c` | `?` | `r2` | `0x0` |
| `bl` at 0x99a30 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x167` | `0x0` |
| `bl` at 0x99a44 | `(unknown callee) 0x1dba4` | `0xfcee4` | `[sp-20]` | `[sp-24]` | `[sp-28]` |
| `bl` at 0x99ad4 | `(unknown callee) 0x18c20` | `?` | `0x0` | `0x24` | `?` |
| `bl` at 0x99b88 | `(unknown callee) 0x18be4` | `[sp-36]` | `[[sp-36]]` | `[[sp-36]+40]` | `[[sp-36]+40]` |
| `bl` at 0x99bd8 | `(unknown callee) 0x18c20` | `0x282460` | `0x0` | `0x24` | `[[sp-20]]` |
| `bl` at 0x99c64 | `(unknown callee) 0x18be4` | `[sp-48]` | `[[sp-48]]` | `[[sp-48]+24]` | `[[sp-48]+24]` |
| `bl` at 0x99c88 | `(unknown callee) 0x1fe64` | `0x11b45c` | `?` | `?` | `?` |
| `bl` at 0x99cc8 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x1a6` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x99a04 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `beq` at 0x99a54 | `? == 0x0` | `? != 0x0` |
| `ble` at 0x99a68 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `bne` at 0x99a78 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x99aa8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x99af4 | `[sp-36] != 0x0` | `[sp-36] == 0x0` |
| `bne` at 0x99b10 | `[[sp-36]] != [[sp-20]]` | `[[sp-36]] == [[sp-20]]` |
| `beq` at 0x99b1c | `[sp-24] == 0x0` | `[sp-24] != 0x0` |
| `beq` at 0x99b30 | `[[sp-36]+4] == [sp-24]` | `[[sp-36]+4] != [sp-24]` |
| `bne` at 0x99b48 | `[sp-36] != [0x282860]` | `[sp-36] == [0x282860]` |
| `bne` at 0x99bb0 | `[sp-24] != 0x0` | `[sp-24] == 0x0` |
| `bne` at 0x99bc8 | `[0x282460] != [[sp-20]]` | `[0x282460] == [[sp-20]]` |
| `beq` at 0x99be8 | `? == 0x0` | `? != 0x0` |
| `bne` at 0x99bf4 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |
| `bne` at 0x99c10 | `[[sp-48]] != [[sp-20]]` | `[[sp-48]] == [[sp-20]]` |
| `bne` at 0x99c24 | `[sp-48] != [0x2824a0]` | `[sp-48] == [0x2824a0]` |
| `beq` at 0x99c9c | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x99cd4 | `r0` | `0x0` |

## `IRouterPacketCount`

- `IRouterPacketCount`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x99d04 to 0x99e2c.
  Established. Two word updates write `([0x11b448] + [sp-20])` at `0x11b448` (`str` at 0x99d78) and
  `([0x11b44c] + [sp-20])` at `0x11b44c` (`str` at 0x99d90). The signed comparison takes its branch
  when `[0x11b44c] <= [0x11b450]`, and falls through when greater (`ble` at 0x99da8). The
  fall-through copies the word `[0x11b44c]` to `0x11b450` (`str` at 0x99db8). The quantities counted
  by these words are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x99d14 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x99d18 | load | 4 | `abs 0x99e08` | `-` |
| `str` at 0x99d28 | store | 4 | `sp -0x18` | `ret(0x1fb00)` |
| `ldr` at 0x99d2c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99d38 | load | 4 | `abs 0x99e0c` | `-` |
| `str` at 0x99d3c | store | 4 | `sp -0x24` | `0xfce5c` |
| `ldr` at 0x99d40 | load | 4 | `abs 0x99e08` | `-` |
| `str` at 0x99d44 | store | 4 | `sp -0x20` | `0x11b4b0` |
| `ldr` at 0x99d48 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x99d4c | store | 4 | `sp -0x1c` | `ret(0x1fb00)` |
| `ldr` at 0x99d54 | load | 4 | `abs 0x99e10` | `-` |
| `ldr` at 0x99d58 | load | 4 | `abs 0x99e14` | `-` |
| `ldr` at 0x99d64 | load | 4 | `abs 0x99e18` | `-` |
| `ldr` at 0x99d68 | load | 4 | `abs 0x99e18` | `-` |
| `ldr` at 0x99d6c | load | 4 | `abs 0x11b448` | `-` |
| `ldr` at 0x99d70 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x99d78 | store | 4 | `abs 0x11b448` | `([0x11b448] + [sp-20])` |
| `ldr` at 0x99d7c | load | 4 | `abs 0x99e1c` | `-` |
| `ldr` at 0x99d80 | load | 4 | `abs 0x99e1c` | `-` |
| `ldr` at 0x99d84 | load | 4 | `abs 0x11b44c` | `-` |
| `ldr` at 0x99d88 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x99d90 | store | 4 | `abs 0x11b44c` | `([0x11b44c] + [sp-20])` |
| `ldr` at 0x99d94 | load | 4 | `abs 0x99e1c` | `-` |
| `ldr` at 0x99d98 | load | 4 | `abs 0x99e20` | `-` |
| `ldr` at 0x99d9c | load | 4 | `abs 0x11b44c` | `-` |
| `ldr` at 0x99da0 | load | 4 | `abs 0x11b450` | `-` |
| `ldr` at 0x99dac | load | 4 | `abs 0x99e20` | `-` |
| `ldr` at 0x99db0 | load | 4 | `abs 0x99e1c` | `-` |
| `ldr` at 0x99db4 | load | 4 | `abs 0x11b44c` | `-` |
| `str` at 0x99db8 | store | 4 | `abs 0x11b450` | `[0x11b44c]` |
| `ldr` at 0x99dbc | load | 4 | `abs 0x99e08` | `-` |
| `str` at 0x99dc8 | store | 4 | `sp -0x18` | `ret(0x1fe64)` |
| `ldr` at 0x99dcc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x99dd8 | load | 4 | `abs 0x99e24` | `-` |
| `str` at 0x99ddc | store | 4 | `sp -0x24` | `0xfceb4` |
| `ldr` at 0x99de0 | load | 4 | `abs 0x99e08` | `-` |
| `str` at 0x99de4 | store | 4 | `sp -0x20` | `0x11b4b0` |
| `ldr` at 0x99de8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x99dec | store | 4 | `sp -0x1c` | `ret(0x1fe64)` |
| `ldr` at 0x99df4 | load | 4 | `abs 0x99e10` | `-` |
| `ldr` at 0x99df8 | load | 4 | `abs 0x99e28` | `-` |
| `ldmdb` at 0x99e04 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x99e04 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x99e04 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x99d20 | `(unknown callee) 0x1fb00` | `0x11b4b0` | `?` | `r2` | `r3` |
| `bl` at 0x99d60 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x1b5` | `0x0` |
| `bl` at 0x99dc0 | `(unknown callee) 0x1fe64` | `0x11b4b0` | `[0x11b44c]` | `?` | `?` |
| `bl` at 0x99e00 | `(unknown callee) 0x1eed0` | `0x9` | `0xfce88` | `0x1bd` | `0x0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x99d34 | `ret(0x1fb00) == 0x0` | `ret(0x1fb00) != 0x0` |
| `ble` at 0x99da8 | `[0x11b44c] <= [0x11b450] (signed, 32-bit)` | `[0x11b44c] > [0x11b450] (signed, 32-bit)` |
| `beq` at 0x99dd4 | `ret(0x1fe64) == 0x0` | `ret(0x1fe64) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x99e04 | `r0` | `?` |

## `IRouterSetQueueSuspendTime`

- `IRouterSetQueueSuspendTime`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ad60 to 0x9ad9c.
  Established. The word at `0x1156f0` receives entry `r0` (`str` at 0x9ad80), and the word at
  `0x1156f4` receives `[sp-24]` (`str` at 0x9ad8c). Entry `r0` is preserved at the exit (`ldmdb` at
  0x9ad90). There is no call or conditional branch record. Time units and suspension behavior remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9ad70 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9ad74 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9ad78 | load | 4 | `abs 0x9ad94` | `-` |
| `ldr` at 0x9ad7c | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9ad80 | store | 4 | `abs 0x1156f0` | `r0` |
| `ldr` at 0x9ad84 | load | 4 | `abs 0x9ad98` | `-` |
| `ldr` at 0x9ad88 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9ad8c | store | 4 | `abs 0x1156f4` | `[sp-24]` |
| `ldmdb` at 0x9ad90 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9ad90 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9ad90 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9ad90 | `r0` | `r0` |

## `IRouterGetQueueSuspendTime`

- `IRouterGetQueueSuspendTime`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9ad9c to 0x9ade0.
  Established. The word `[0x1156f0]` is stored at entry `r0 +0x0` (`str` at 0x9adc0). Another word
  `[0x1156f4]` is written at `[sp-24] +0x0` (`str` at 0x9add0). The exit preserves entry `r0`
  (`ldmdb` at 0x9add4). The body has no call or conditional branch, and the units represented by the
  two loaded words are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9adac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9adb0 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0x9adb4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9adb8 | load | 4 | `abs 0x9add8` | `-` |
| `ldr` at 0x9adbc | load | 4 | `abs 0x1156f0` | `-` |
| `str` at 0x9adc0 | store | 4 | `r0 +0x0` | `[0x1156f0]` |
| `ldr` at 0x9adc4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9adc8 | load | 4 | `abs 0x9addc` | `-` |
| `ldr` at 0x9adcc | load | 4 | `abs 0x1156f4` | `-` |
| `str` at 0x9add0 | store | 4 | `[sp-24] +0x0` | `[0x1156f4]` |
| `ldmdb` at 0x9add4 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9add4 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9add4 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9add4 | `r0` | `r0` |

## `IRouterResponse`

- `IRouterResponse`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9bd94 to 0x9bf04.
  Established. The first equality branch is taken when entry `r0 == 0`, and falls through when
  nonzero (`beq` at 0x9bdc4). A call to `0x99f58` carries candidates `([sp-48], 0x1, 0x0, [sp-52])`
  (`bl` at 0x9beac); its zero result takes the branch into the remaining call sequence, while
  nonzero falls through (`beq` at 0x9bec0). A later call to `0x9b02c` has third candidate one (`bl`
  at 0x9bedc). Response formatting and matching remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9bda4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9bda8 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9bdac | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9bdb0 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x9bdb8 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0x9bdbc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bdc8 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9bdcc | load | 1 | `r0 +0x4` | `-` |
| `str` at 0x9bdd0 | store | 4 | `sp -0x34` | `[r0+4]:1` |
| `ldr` at 0x9bdd8 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9bddc | load | 1 | `r2 +0x17` | `-` |
| `str` at 0x9bde0 | store | 4 | `sp -0x34` | `[r2+23]:1` |
| `ldr` at 0x9bde4 | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0x9bde8 | load | 2 | `r2 +0x22` | `-` |
| `str` at 0x9bdec | store | 4 | `sp -0x58` | `[r2+34]:2` |
| `ldr` at 0x9bdf0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9bdfc | load | 4 | `abs 0x9bf00` | `-` |
| `str` at 0x9be2c | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0x9be30 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9be34 | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0x9be38 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0x9be3c | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0x9be48 | store | 4 | `sp -0x50` | `?` |
| `ldr` at 0x9be4c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9be54 | store | 4 | `sp -0x4c` | `[sp-28]+28` |
| `ldr` at 0x9be58 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9be5c | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0x9be60 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9be6c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9be74 | store | 4 | `sp -0x38` | `[sp-28]+24` |
| `ldr` at 0x9be78 | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9be7c | store | 4 | `sp -0x48` | `?` |
| `ldr` at 0x9be80 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9be84 | store | 4 | `sp -0x44` | `[sp-32]` |
| `ldr` at 0x9be88 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9be8c | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0x9be90 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9be94 | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0x9be98 | store | 4 | `sp -0x3c` | `[[sp-28]+12]` |
| `ldr` at 0x9be9c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9bea8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0x9beb4 | store | 4 | `sp -0x28` | `ret(0x99f58)` |
| `ldr` at 0x9beb8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9bec4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bec8 | store | 4 | `sp -0x2c` | `ret(0x99f58)` |
| `ldr` at 0x9bed0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bed4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9bee4 | store | 4 | `sp -0x28` | `ret(0x9b02c)` |
| `ldr` at 0x9bee8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bef0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bef4 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x9bef8 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x9befc | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9befc | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9befc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9beac | `(unknown callee) 0x99f58` | `[sp-48]` | `0x1` | `0x0` | `[sp-52]` |
| `bl` at 0x9bedc | `(unknown callee) 0x9b02c` | `[sp-20]` | `[sp-36]` | `0x1` | `ret(0x99f58)` |
| `bl` at 0x9beec | `(unknown callee) 0x9a998` | `[sp-36]` | `?` | `?` | `ret(0x9b02c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9bdc4 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0x9be68 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0x9bec0 | `ret(0x99f58) == 0x0` | `ret(0x99f58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9befc | `r0` | `?` |

## `IRouterEvent`

- `IRouterEvent`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9bf04 to 0x9bfe0.
  Established. A word at `0x11b458` receives `[0x11b458]+1` (`str` at 0x9bf54). The call to
  `0x99f58` has candidates `(sp-36, 0x2, 0x0, [r0+4]:1)` (`bl` at 0x9bf8c). A zero result takes the
  following branch, with nonzero falling through (`beq` at 0x9bfa0). The merged return is unknown
  (`ldmdb` at 0x9bfd8). Event numbering and delivery effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9bf14 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9bf18 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9bf1c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9bf20 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9bf28 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9bf2c | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0x9bf30 | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9bf34 | store | 4 | `sp -0x4c` | `r1` |
| `str` at 0x9bf3c | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0x9bf40 | load | 4 | `abs 0x9bfdc` | `-` |
| `ldr` at 0x9bf44 | load | 4 | `abs 0x11b458` | `-` |
| `str` at 0x9bf4c | store | 4 | `sp -0x44` | `[0x11b458]` |
| `str` at 0x9bf54 | store | 4 | `abs 0x11b458` | `[0x11b458]+1` |
| `str` at 0x9bf5c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x9bf60 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9bf64 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0x9bf68 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9bf6c | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0x9bf70 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9bf74 | store | 4 | `sp -0x34` | `[sp-32]` |
| `str` at 0x9bf7c | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x9bf94 | store | 4 | `sp -0x28` | `ret(0x99f58)` |
| `ldr` at 0x9bf98 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9bfa4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bfa8 | store | 4 | `sp -0x2c` | `ret(0x99f58)` |
| `ldr` at 0x9bfb0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9bfb4 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9bfc0 | store | 4 | `sp -0x28` | `ret(0x9b67c)` |
| `ldr` at 0x9bfc4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9bfcc | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9bfd0 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0x9bfd4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0x9bfd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9bfd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9bfd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9bf8c | `(unknown callee) 0x99f58` | `sp-36` | `0x2` | `0x0` | `[r0+4]:1` |
| `bl` at 0x9bfb8 | `(unknown callee) 0x9b67c` | `[sp-20]` | `[sp-36]` | `?` | `ret(0x99f58)` |
| `bl` at 0x9bfc8 | `(unknown callee) 0x9a998` | `[sp-36]` | `?` | `?` | `ret(0x9b67c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9bfa0 | `ret(0x99f58) == 0x0` | `ret(0x99f58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9bfd8 | `r0` | `?` |

## `IRouterCommand`

- `IRouterCommand`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9bfe0 to 0x9c10c.
  Established. The first call uses the four entry register values as candidates (`bl` at 0x9c030);
  its zero result takes the branch to a call to `0x9a93c` (`beq` at 0x9c044, `bl` at 0x9c060). Nonzero falls through to the exit. The second call has candidates `([sp+8], [sp+12], [sp+16],
  ret(0x9bc5c))` (`bl` at 0x9c060). Later word stores include `0xfffffd96` at `sp -0x30` (`str` at
  0x9c0d4, `str` at 0x9c0f0). Command sequencing and error-code meanings remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9bff0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9bff4 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9bff8 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9bffc | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9c000 | load | 4 | `sp +0x0` | `-` |
| `str` at 0x9c004 | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0x9c008 | load | 4 | `sp +0x4` | `-` |
| `str` at 0x9c00c | store | 4 | `sp -0x3c` | `[sp+4]` |
| `str` at 0x9c014 | store | 4 | `sp -0x38` | `sp-40` |
| `str` at 0x9c01c | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0x9c020 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0x9c024 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c028 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c02c | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c038 | store | 4 | `sp -0x24` | `ret(0x9bc5c)` |
| `ldr` at 0x9c03c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c048 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c04c | store | 4 | `sp -0x30` | `ret(0x9bc5c)` |
| `ldr` at 0x9c054 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9c058 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0x9c05c | load | 4 | `sp +0x10` | `-` |
| `str` at 0x9c068 | store | 4 | `sp -0x24` | `ret(0x9a93c)` |
| `ldr` at 0x9c06c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c078 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c07c | store | 4 | `sp -0x30` | `ret(0x9a93c)` |
| `ldr` at 0x9c084 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9c088 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0x9c08c | load | 1 | `[[sp+8]] +0x14` | `-` |
| `ldr` at 0x9c090 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9c094 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0x9c098 | load | 1 | `[[sp+8]] +0x21` | `-` |
| `str` at 0x9c0a4 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9c0a8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0x9c0ac | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0x9c0b0 | load | 1 | `[[sp+8]] +0x12` | `-` |
| `ldr` at 0x9c0bc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c0c4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c0d0 | load | 4 | `abs 0x9c108` | `-` |
| `str` at 0x9c0d4 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `ldr` at 0x9c0dc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c0e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c0ec | load | 4 | `abs 0x9c108` | `-` |
| `str` at 0x9c0f0 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `str` at 0x9c0fc | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0x9c100 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0x9c104 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9c104 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9c104 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c030 | `(unknown callee) 0x9bc5c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0x9c060 | `(unknown callee) 0x9a93c` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ret(0x9bc5c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9c044 | `ret(0x9bc5c) == 0x0` | `ret(0x9bc5c) != 0x0` |
| `beq` at 0x9c074 | `ret(0x9a93c) == 0x0` | `ret(0x9a93c) != 0x0` |
| `bne` at 0x9c0b8 | `[[[sp+8]]+18]:1 != 0x0` | `[[[sp+8]]+18]:1 == 0x0` |
| `beq` at 0x9c0cc | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0x9c0e8 | `[sp-40] == [sp-44]` | `[sp-40] != [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9c104 | `r0` | `?` |

## `IRouterMakePacket`

- `IRouterMakePacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c10c to 0x9c1cc.
  Established. A halfword at entry `r0 +0x6` receives `([r0+6]:2+1 & 0xffff)` (`strh` at 0x9c15c).
  The call to `0x99f58` uses candidates `(sp-36, [sp-32], [sp-40], [r0+4]:1)` (`bl` at 0x9c1a0); its
  zero result takes the branch past a stack-word clear, while nonzero falls through (`beq` at
  0x9c1ac) to zero at `sp -0x24` (`str` at 0x9c1b4). The exit reports `[sp-36]` (`ldmdb` at
  0x9c1c8). Packet allocation and field ownership are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c11c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9c120 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9c124 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9c128 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x9c130 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9c138 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0x9c13c | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0x9c140 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0x9c144 | store | 4 | `sp -0x48` | `r2` |
| `str` at 0x9c14c | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0x9c150 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0x9c154 | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0x9c15c | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0x9c168 | store | 4 | `sp -0x40` | `?` |
| `ldr` at 0x9c16c | load | 4 | `sp -0x18` | `-` |
| `str` at 0x9c170 | store | 4 | `sp -0x3c` | `[sp-24]` |
| `ldr` at 0x9c174 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x9c178 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0x9c17c | load | 4 | `sp +0x4` | `-` |
| `str` at 0x9c180 | store | 4 | `sp -0x34` | `[sp+4]` |
| `ldr` at 0x9c184 | load | 4 | `sp +0x8` | `-` |
| `str` at 0x9c188 | store | 4 | `sp -0x30` | `[sp+8]` |
| `str` at 0x9c190 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0x9c194 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c198 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c1b4 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0x9c1bc | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0x9c1c0 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0x9c1c8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9c1c8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9c1c8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c1a0 | `(unknown callee) 0x99f58` | `sp-36` | `[sp-32]` | `[sp-40]` | `[r0+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x9c1ac | `ret(0x99f58) == 0x0` | `ret(0x99f58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9c1c8 | `r0` | `[sp-36]` |

## `IRouterReadPacket`

- `IRouterReadPacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c1cc to 0x9c648.
  Established. The call to `0x18920` has first candidate `0x410` (`bl` at 0x9c21c); its nonzero
  result takes the branch, while zero falls through (`bne` at 0x9c230). The nonzero arm calls
  `0x18c20` with candidates `(ret(0x18920), 0x0, 0x2c, ret(0x18920))` (`bl` at 0x9c27c). The
  subsequent indirect call has candidates `([sp-28], sp-44, [sp-64], sp-44)` and an unknown target
  (`mov` at 0x9c298); its signed positive result takes the later branch, with zero or negative
  values falling through (`bgt` at 0x9c2ac). Frame layout and the called reader behavior remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c1dc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9c1e0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9c1e4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9c1e8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0x9c1f0 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0x9c1f8 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0x9c200 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0x9c208 | store | 4 | `sp -0x3c` | `0x410` |
| `ldr` at 0x9c20c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9c214 | store | 4 | `sp -0x40` | `0x400` |
| `ldr` at 0x9c218 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9c224 | store | 4 | `sp -0x2c` | `ret(0x18920)` |
| `ldr` at 0x9c228 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c234 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c244 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c248 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x9c24c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c250 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x9c254 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0x9c258 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c264 | load | 4 | `abs 0x9c618` | `-` |
| `str` at 0x9c268 | store | 4 | `sp -0x60` | `0xfffffda4` |
| `ldr` at 0x9c270 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c284 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c288 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c290 | load | 4 | `sp -0x40` | `-` |
| `str` at 0x9c2a0 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0x9c2a4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c2b0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c2b4 | store | 4 | `sp -0x64` | `ret(unknown)` |
| `ldr` at 0x9c2b8 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0x9c2c4 | load | 4 | `sp -0x64` | `-` |
| `str` at 0x9c2d8 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0x9c2e0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0x9c2e4 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0x9c2ec | load | 4 | `abs 0x9c61c` | `-` |
| `str` at 0x9c2f0 | store | 4 | `sp -0x44` | `0xfffffd9c` |
| `ldr` at 0x9c2f4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c300 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c304 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x9c308 | store | 4 | `sp -0x48` | `[[sp-40]]` |
| `ldr` at 0x9c30c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9c31c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9c320 | load | 4 | `(unknown)` | `-` |
| `str` at 0x9c324 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x9c328 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0x9c330 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9c334 | store | 4 | `sp -0x48` | `[sp-76]` |
| `ldr` at 0x9c33c | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c344 | store | 4 | `[sp-40] +0x0` | `0x0` |
| `ldr` at 0x9c348 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c354 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0x9c358 | load | 1 | `[sp-40] +0x16` | `-` |
| `str` at 0x9c368 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0x9c36c | load | 4 | `abs 0x9c620` | `-` |
| `ldr` at 0x9c370 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0x9c374 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c380 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c38c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c398 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c3a8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c3ac | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x9c3b0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c3b4 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x9c3b8 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0x9c3bc | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c3c8 | load | 4 | `sp -0x44` | `-` |
| `str` at 0x9c3cc | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x9c3d4 | load | 4 | `abs 0x9c624` | `-` |
| `ldr` at 0x9c3d8 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c3dc | load | 4 | `abs 0x11b438` | `-` |
| `ldr` at 0x9c3e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c3f0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9c3f4 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0x9c408 | load | 4 | `abs 0x9c628` | `-` |
| `ldr` at 0x9c40c | load | 4 | `abs 0x11b440` | `-` |
| `ldr` at 0x9c424 | load | 4 | `abs 0x9c62c` | `-` |
| `ldr` at 0x9c428 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c430 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c43c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0x9c444 | load | 4 | `abs 0x9c630` | `-` |
| `ldr` at 0x9c448 | load | 4 | `abs 0x11b43c` | `-` |
| `ldr` at 0x9c454 | load | 4 | `abs 0x9c634` | `-` |
| `ldr` at 0x9c458 | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9c45c | load | 4 | `abs 0x9c638` | `-` |
| `ldr` at 0x9c460 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c468 | load | 4 | `abs 0x9c634` | `-` |
| `ldr` at 0x9c46c | load | 4 | `abs 0x116928` | `-` |
| `ldr` at 0x9c474 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c478 | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0x9c480 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9c484 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0x9c48c | load | 4 | `sp -0x38` | `-` |
| `str` at 0x9c49c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0x9c4a0 | load | 4 | `abs 0x9c628` | `-` |
| `ldr` at 0x9c4a4 | load | 4 | `abs 0x11b440` | `-` |
| `ldr` at 0x9c4c0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c4d0 | load | 4 | `abs 0x9c628` | `-` |
| `ldr` at 0x9c4d4 | load | 4 | `abs 0x11b440` | `-` |
| `ldr` at 0x9c4e8 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0x9c4f4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0x9c4fc | load | 4 | `abs 0x9c63c` | `-` |
| `ldr` at 0x9c510 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c518 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c51c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0x9c520 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c524 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0x9c528 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0x9c52c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c538 | load | 4 | `abs 0x9c628` | `-` |
| `ldr` at 0x9c53c | load | 4 | `abs 0x11b440` | `-` |
| `ldr` at 0x9c548 | load | 4 | `abs 0x9c640` | `-` |
| `ldr` at 0x9c54c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c554 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0x9c55c | load | 4 | `abs 0x9c644` | `-` |
| `str` at 0x9c560 | store | 4 | `sp -0x60` | `0xfffffd9a` |
| `ldr` at 0x9c568 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c56c | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c570 | store | 4 | `[sp-48] +0x0` | `[sp-44]` |
| `ldr` at 0x9c574 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0x9c578 | load | 2 | `[sp-44] +0x4` | `-` |
| `strh` at 0x9c57c | store | 2 | `sp -0x68` | `([[sp-44]+4]:2 & 0xffff)` |
| `ldrh` at 0x9c580 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0x9c58c | store | 2 | `sp -0x68` | `?` |
| `ldrh` at 0x9c590 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0x9c594 | store | 2 | `[sp-44] +0x4` | `([sp-104]:2 & 0xffff)` |
| `ldr` at 0x9c598 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c5a0 | store | 4 | `[sp-44] +0x0` | `0x0` |
| `ldr` at 0x9c5a4 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0x9c5a8 | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0x9c5b4 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0x9c5b8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0x9c5bc | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0x9c5c0 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0x9c5cc | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0x9c5d0 | load | 2 | `[sp-40] +0x4` | `-` |
| `strh` at 0x9c5d4 | store | 2 | `sp -0x6c` | `([[sp-40]+4]:2 & 0xffff)` |
| `ldrh` at 0x9c5d8 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0x9c5e8 | store | 2 | `sp -0x6c` | `?` |
| `ldrh` at 0x9c5ec | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0x9c5f0 | store | 2 | `[sp-40] +0x4` | `([sp-108]:2 & 0xffff)` |
| `ldr` at 0x9c5f4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c5f8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c5fc | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `str` at 0x9c60c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0x9c610 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0x9c614 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9c614 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9c614 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c21c | `(unknown callee) 0x18920` | `0x410` | `?` | `?` | `0x400` |
| `bl` at 0x9c25c | `(unknown callee) 0x18be4` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0x9c27c | `(unknown callee) 0x18c20` | `ret(0x18920)` | `0x0` | `0x2c` | `ret(0x18920)` |
| `mov` at 0x9c298 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-44` | `[sp-64]` | `sp-44` |
| `bl` at 0x9c32c | `(unknown callee) 0x18be4` | `?` | `?` | `?` | `?` |
| `bl` at 0x9c37c | `(unknown callee) 0x9bd94` | `0x2824c0` | `?` | `[sp-40]` | `0x0` |
| `bl` at 0x9c390 | `(unknown callee) 0x18be4` | `[sp-44]` | `?` | `?` | `[sp-44]` |
| `bl` at 0x9c3c0 | `(unknown callee) 0x18be4` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0x9c3e0 | `(unknown callee) 0x9c808` | `[sp-44]` | `[0x11b438]` | `?` | `0x11b438` |
| `bl` at 0x9c420 | `(unknown callee) 0x9d240` | `0x0` | `0x0` | `?` | `[0x11b440]` |
| `bl` at 0x9c42c | `(unknown callee) 0x9d240` | `0xfd034` | `[sp-44]` | `?` | `?` |
| `bl` at 0x9c434 | `(unknown callee) 0x18be4` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x9c464 | `(unknown callee) 0x21010` | `[0x116928]` | `0xfd03c` | `[sp-44]` | `0x116928` |
| `bl` at 0x9c470 | `(unknown callee) 0x18818` | `[0x116928]` | `?` | `?` | `0x116928` |
| `bl` at 0x9c4b8 | `(unknown callee) 0x9d240` | `0x0` | `0x0` | `?` | `[0x11b440]` |
| `bl` at 0x9c504 | `(unknown callee) 0x18c74` | `sp-92` | `0xfd048` | `?` | `?` |
| `bl` at 0x9c514 | `(unknown callee) 0x9d240` | `sp-92` | `[sp-40]` | `?` | `sp-92` |
| `bl` at 0x9c530 | `(unknown callee) 0x18be4` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0x9c550 | `(unknown callee) 0x9d240` | `0xfd050` | `[sp-44]` | `?` | `[0x11b440]` |
| `bl` at 0x9c558 | `(unknown callee) 0x18be4` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0x9c604 | `(unknown callee) 0x99d04` | `0x1` | `([[sp-40]+4]:2 & 0xffff)` | `[sp-32]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c230 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0x9c23c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bgt` at 0x9c2ac | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `beq` at 0x9c2c0 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0x9c2cc | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0x9c2fc | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x9c314 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0x9c350 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0x9c360 | `[[sp-40]+22]:1 != 0x0` | `[[sp-40]+22]:1 == 0x0` |
| `beq` at 0x9c388 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0x9c3a0 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x9c3ec | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0x9c404 | `? != 0x0` | `? == 0x0` |
| `beq` at 0x9c414 | `[0x11b440] == 0x0` | `[0x11b440] != 0x0` |
| `beq` at 0x9c450 | `[0x11b43c] == 0x0` | `[0x11b43c] != 0x0` |
| `beq` at 0x9c494 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0x9c4ac | `[0x11b440] == 0x0` | `[0x11b440] != 0x0` |
| `bne` at 0x9c4c8 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0x9c4dc | `[0x11b440] == 0x0` | `[0x11b440] != 0x0` |
| `beq` at 0x9c544 | `[0x11b440] == 0x0` | `[0x11b440] != 0x0` |
| `bne` at 0x9c5c8 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9c614 | `r0` | `?` |

## `IRouterWritePacket`

- `IRouterWritePacket`, `6.5.27/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.27:l1#3`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x9c648 to 0x9c808.
  Established. The word at `sp -0x38` receives `0xfffffda4` on one path (`str` at 0x9c6f4), and zero
  on another (`str` at 0x9c7f0). An indirect call has candidates `([sp-28], [sp-52]+16, [sp-40],
  [sp-52]+16)` (`mov` at 0x9c73c), with an unknown target. Equality between its result and `[sp-40]`
  takes the following branch, while inequality falls through (`beq` at 0x9c754). A later call to
  `0x9a998` uses `[sp-32]` as its first candidate (`bl` at 0x9c79c). Transmission completion and
  ownership semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x9c658 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0x9c65c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0x9c660 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0x9c664 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0x9c668 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c66c | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0x9c670 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c680 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x9c684 | load | 2 | `(unknown)` | `-` |
| `str` at 0x9c688 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0x9c68c | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0x9c690 | load | 2 | `(unknown)` | `-` |
| `str` at 0x9c694 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0x9c698 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c6ac | store | 4 | `sp -0x34` | `ret(0x18920)` |
| `ldr` at 0x9c6b0 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c6bc | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9c6c0 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0x9c6d0 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x9c6d4 | load | 4 | `abs 0x9c7fc` | `-` |
| `ldr` at 0x9c6d8 | load | 4 | `abs 0x9c800` | `-` |
| `ldr` at 0x9c6dc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c6e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c6f0 | load | 4 | `abs 0x9c800` | `-` |
| `str` at 0x9c6f4 | store | 4 | `sp -0x38` | `0xfffffda4` |
| `ldr` at 0x9c6fc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c704 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c708 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c714 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c720 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c728 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0x9c72c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0x9c734 | load | 4 | `sp -0x28` | `-` |
| `str` at 0x9c744 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0x9c748 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c74c | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0x9c758 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c764 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0x9c768 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0x9c778 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0x9c77c | load | 4 | `abs 0x9c7fc` | `-` |
| `ldr` at 0x9c784 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c790 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c798 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0x9c7a0 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0x9c7ac | load | 4 | `abs 0x9c804` | `-` |
| `str` at 0x9c7b0 | store | 4 | `sp -0x3c` | `0xfffffd92` |
| `str` at 0x9c7bc | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0x9c7c0 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0x9c7c4 | store | 4 | `sp -0x38` | `[sp-60]` |
| `ldr` at 0x9c7cc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0x9c7d4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0x9c7d8 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0x9c7dc | store | 4 | `sp -0x24` | `[[sp-36]]` |
| `ldr` at 0x9c7e4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0x9c7f0 | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0x9c7f4 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0x9c7f8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0x9c7f8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0x9c7f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x9c6a4 | `(unknown callee) 0x18920` | `?` | `?` | `?` | `?` |
| `bl` at 0x9c6e4 | `(unknown callee) 0x9bd94` | `0x2824c0` | `0xfffffda4` | `[sp-32]` | `0x0` |
| `bl` at 0x9c6ec | `(unknown callee) 0x9a998` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x9c710 | `(unknown callee) 0x18ae8` | `ret(0x18920)` | `[sp-36]` | `[sp-40]+16` | `[sp-40]+16` |
| `bl` at 0x9c71c | `(unknown callee) 0x9c808` | `[sp-52]` | `0x1` | `?` | `?` |
| `mov` at 0x9c73c | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-52]+16` | `[sp-40]` | `[sp-52]+16` |
| `bl` at 0x9c78c | `(unknown callee) 0x9bd94` | `0x2824c0` | `?` | `[sp-32]` | `0x0` |
| `bl` at 0x9c794 | `(unknown callee) 0x18be4` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0x9c79c | `(unknown callee) 0x9a998` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0x9c7d0 | `(unknown callee) 0x18be4` | `[sp-52]` | `?` | `ret(unknown)` | `[sp-40]` |
| `bl` at 0x9c7e8 | `(unknown callee) 0x9a998` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0x9c678 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0x9c6b8 | `ret(0x18920) != 0x0` | `ret(0x18920) == 0x0` |
| `bne` at 0x9c6c8 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `beq` at 0x9c754 | `ret(unknown) == [sp-40]` | `ret(unknown) != [sp-40]` |
| `bge` at 0x9c760 | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `bne` at 0x9c770 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `bne` at 0x9c7a8 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0x9c7f8 | `r0` | `?` |
