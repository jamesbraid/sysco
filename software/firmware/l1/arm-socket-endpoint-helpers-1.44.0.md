# ARM socket endpoint and character I/O helpers in 1.44.0

Read from `6.5.30/usr/cpu/firmware/sysco/l1.bin`, package `6.5.30:l1#4` (version 1.44.0),
inner executable `/stand/sysco/bin/l1`. The package begins at carrier offset `0x53d154`.
Carrier SHA-256: `c355c542554a41b23b69dca4c201c566e8352e8f9fb870d2365c74cdd8ab0c0f`.
Package SHA-256: `cb84af5c8150913d0bd30673dc3efc39264f208fa18336cceb20c66eff4c56d6`.
Inner executable SHA-256: `8d492b1d00a2fa93aebb48890caf8cacc5a9747e79209e93b841378aabd75162`.
The ELF identifies 32-bit little-endian ARM; its executable PT_LOAD segment begins at
virtual address `0x8000`, file offset `0x0`, and contains 1166540 bytes.
Segment SHA-256: `7f2caa3b9c656f2cc1caf5bdaf139038c25b4a58ac8ff8a26ea044818245a46d`. Analysed routine spans use the exact
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
No unresolved computed transfers were reported for these routines.
Undecodable byte ranges remain visible in the fact sheets.
Coprocessor transfer widths and values are unknown. Integer return records do not establish
coprocessor results. Unknown values and effects remain explicit in the fact sheets.

[Fact sheets and full listings](disasm/arm-1.44.0-package4/socket-endpoint-helpers-facts.txt) retain all tool sections.

## `SocketClose`

- `SocketClose`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac56c to 0xac5c0.
  Established. Nonnegative entry `r0` takes `0xac598`, while negative falls through (`bge` at 0xac588).
  The unnamed `0x19418` call carries entry `r0`, `0x2`, entry `r2` and entry `r0` (`bl` at 0xac5a0).
  A subsequent unnamed `0x1931c` call carries `[sp-20]` followed by three unknown candidates (`bl` at 0xac5a8).
  Hardware shutdown responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac57c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xac580 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac590 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xac598 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac5a4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac5b0 | store | 4 | `sp -0x18` | `ret(0x1931c)` |
| `ldr` at 0xac5b4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xac5bc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac5bc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac5bc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac5a0 | `(unknown callee) 0x19418` | `r0` | `0x2` | `r2` | `r0` |
| `bl` at 0xac5a8 | `(unknown callee) 0x1931c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xac588 | `r0 >= 0x0 (signed, 32-bit)` | `r0 < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac5bc | `r0` | `[sp-24]` |

## `SocketCloseOnly`

- `SocketCloseOnly`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac5c0 to 0xac608.
  Established. The signed condition entry `r0 >= 0` takes `0xac5ec`, while negative falls through (`bge` at 0xac5dc).
  An unnamed `0x1931c` call carries entry `r0`, entry `r1`, entry `r2` and entry `r0` (`bl` at 0xac5f0).
  Device-resource ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac5d0 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xac5d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac5e4 | store | 4 | `sp -0x18` | `?` |
| `ldr` at 0xac5ec | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac5f8 | store | 4 | `sp -0x18` | `ret(0x1931c)` |
| `ldr` at 0xac5fc | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xac604 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac604 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac604 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac5f0 | `(unknown callee) 0x1931c` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xac5dc | `r0 >= 0x0 (signed, 32-bit)` | `r0 < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac604 | `r0` | `[sp-24]` |

## `SocketCloseUnixServer`

- `SocketCloseUnixServer`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac608 to 0xac658.
  Established. The unnamed `0xacaf0` call carries `sp-136`, entry `r1`, entry `r2` and `sp-136` (`bl` at 0xac62c).
  Another unnamed `0x19358` call carries `sp-134`, unknown, unknown and `sp-134` (`bl` at 0xac63c).
  The return candidate is `ret(0x1931c)` (`ldm` at 0xac654).
  Unix-server hardware responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac618 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac61c | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xac628 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac640 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xac654 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac654 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac654 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac62c | `(unknown callee) 0xacaf0` | `sp-136` | `r1` | `r2` | `sp-136` |
| `bl` at 0xac63c | `(unknown callee) 0x19358` | `sp-134` | `?` | `?` | `sp-134` |
| `bl` at 0xac644 | `(unknown callee) 0x1931c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac654 | `r0` | `ret(0x1931c)` |

## `SocketGetAddr`

- `SocketGetAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac658 to 0xac718.
  Established. Zero entry `r0` takes `0xac690`, while nonzero falls through (`beq` at 0xac678).
  A word assignment writes `ret(0x19574)` to `[sp-28] +0x0` (`str` at 0xac6a8).
  The unnamed `0x19610` call carries `[sp-28]`, `[[ret(0x19694)+16]]`, `0x4` and `[ret(0x19694)+16]` (`bl` at 0xac700).
  Address resolution at the device boundary remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac668 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xac66c | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xac670 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac67c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac694 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac69c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac6a0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac6a8 | store | 4 | `[sp-28] +0x0` | `ret(0x19574)` |
| `ldr` at 0xac6ac | load | 4 | `[sp-28] +0x0` | `-` |
| `str` at 0xac6bc | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xac6c4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac6d0 | store | 4 | `sp -0x20` | `ret(0x19694)` |
| `ldr` at 0xac6d4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac6e4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xac6ec | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xac6f0 | load | 4 | `ret(0x19694) +0x10` | `-` |
| `ldr` at 0xac6f4 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac6f8 | load | 4 | `[ret(0x19694)+16] +0x0` | `-` |
| `str` at 0xac708 | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xac70c | load | 4 | `sp -0x24` | `-` |
| `ldm` at 0xac714 | load | 4 | `sp -0x14` | `-` |
| `ldm` at 0xac714 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac714 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac714 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac680 | `(unknown callee) 0x19538` | `r0` | `r1` | `r2` | `r0` |
| `bl` at 0xac6a4 | `(unknown callee) 0x19574` | `[sp-24]` | `?` | `?` | `ret(0x19538)` |
| `bl` at 0xac6c8 | `(unknown callee) 0x19694` | `[sp-24]` | `?` | `?` | `[[sp-28]]` |
| `bl` at 0xac700 | `(unknown callee) 0x19610` | `[sp-28]` | `[[ret(0x19694)+16]]` | `0x4` | `[ret(0x19694)+16]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xac678 | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xac68c | `ret(0x19538) != 0x0` | `ret(0x19538) == 0x0` |
| `beq` at 0xac6b4 | `Z == 1` | `not (Z == 1)` |
| `bne` at 0xac6dc | `ret(0x19694) != 0x0` | `ret(0x19694) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac714 | `r0` | `[sp-36]` |

## `SocketGetPeerAddr`

- `SocketGetPeerAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac718 to 0xac7ac.
  Established. A word assignment initializes `sp -0x2c` to `0x10` (`str` at 0xac734).
  The unnamed `0x194e4` call carries entry `r0`, `sp-40`, `sp-44` and `sp-40` (`bl` at 0xac748);
  its signed nonnegative result takes `0xac780`, while negative falls through (`bge` at 0xac754).
  The hardware representation of the peer address remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac728 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac72c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac734 | store | 4 | `sp -0x2c` | `0x10` |
| `ldr` at 0xac740 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac760 | load | 4 | `abs 0xac7a0` | `-` |
| `ldr` at 0xac764 | load | 4 | `abs 0xac7a4` | `-` |
| `ldr` at 0xac768 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac76c | load | 4 | `abs 0xac7a8` | `-` |
| `str` at 0xac778 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xac780 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac784 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac788 | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `str` at 0xac790 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xac794 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xac79c | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac79c | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac79c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac748 | `(unknown callee) 0x194e4` | `r0` | `sp-40` | `sp-44` | `sp-40` |
| `bl` at 0xac758 | `(unknown callee) 0x193d0` | `ret(0x194e4)` | `?` | `?` | `ret(0x194e4)` |
| `bl` at 0xac770 | `(unknown callee) 0x1f538` | `0xfffffe69` | `0x114414` | `[ret(0x193d0)]` | `0x11430c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xac754 | `ret(0x194e4) >= 0x0 (signed, 32-bit)` | `ret(0x194e4) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac79c | `r0` | `?` |

## `SocketGetSockAddr`

- `SocketGetSockAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac7ac to 0xac840.
  Established. The unnamed `0x19628` call carries entry `r0`, `sp-40`, `sp-44` and `sp-40` (`bl` at 0xac7dc).
  A nonnegative result takes `0xac814`, while negative falls through (`bge` at 0xac7e8).
  The taken path assigns `[sp-36]` to `[sp-24] +0x0` (`str` at 0xac81c).
  Local-endpoint address ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac7bc | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac7c0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac7c8 | store | 4 | `sp -0x2c` | `0x10` |
| `ldr` at 0xac7d4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac7f4 | load | 4 | `abs 0xac834` | `-` |
| `ldr` at 0xac7f8 | load | 4 | `abs 0xac838` | `-` |
| `ldr` at 0xac7fc | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac800 | load | 4 | `abs 0xac83c` | `-` |
| `str` at 0xac80c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xac814 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac818 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac81c | store | 4 | `[sp-24] +0x0` | `[sp-36]` |
| `str` at 0xac824 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xac828 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xac830 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac830 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac830 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac7dc | `(unknown callee) 0x19628` | `r0` | `sp-40` | `sp-44` | `sp-40` |
| `bl` at 0xac7ec | `(unknown callee) 0x193d0` | `ret(0x19628)` | `?` | `?` | `ret(0x19628)` |
| `bl` at 0xac804 | `(unknown callee) 0x1f538` | `0xfffffe68` | `0x114420` | `[ret(0x193d0)]` | `0x11430c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xac7e8 | `ret(0x19628) >= 0x0 (signed, 32-bit)` | `ret(0x19628) < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac830 | `r0` | `?` |

## `SocketGetName`

- `SocketGetName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac840 to 0xac8d4.
  Established. The unnamed `0x193e8` call carries `sp-32`, `0x4`, `0x2` and `sp-32` (`bl` at 0xac884).
  Nonzero `ret(0x193e8)` takes `0xac8a8`, while zero falls through (`bne` at 0xac898).
  A later unnamed `0x19760` call carries `[sp-24]`, `[ret(0x193e8)]`, `[sp-28]-1` and `ret(0x193e8)` (`bl` at 0xac8bc).
  The source and hardware meaning of the returned name remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac850 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac854 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac858 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xac85c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac860 | store | 4 | `sp -0x20` | `r0` |
| `ldr` at 0xac864 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac86c | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac88c | store | 4 | `sp -0x24` | `ret(0x193e8)` |
| `ldr` at 0xac890 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xac8a0 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xac8a8 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac8ac | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac8b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac8b8 | load | 4 | `ret(0x193e8) +0x0` | `-` |
| `str` at 0xac8c4 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xac8c8 | load | 4 | `sp -0x28` | `-` |
| `ldm` at 0xac8d0 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac8d0 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac8d0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac870 | `(unknown callee) 0x19748` | `r1` | `0x0` | `r2` | `r0` |
| `bl` at 0xac884 | `(unknown callee) 0x193e8` | `sp-32` | `0x4` | `0x2` | `sp-32` |
| `bl` at 0xac8bc | `(unknown callee) 0x19760` | `[sp-24]` | `[ret(0x193e8)]` | `[sp-28]-1` | `ret(0x193e8)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xac898 | `ret(0x193e8) != 0x0` | `ret(0x193e8) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac8d0 | `r0` | `[sp-40]` |

## `SocketGetHostName`

- `SocketGetHostName`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac94c to 0xac9e0.
  Established. An unnamed `0x19748` call carries `0x133ccc`, `0x0`, `0x42` and entry `r3` (`bl` at 0xac96c).
  A word assignment writes `0x133ccc` to `[sp-20] +0x0` (`str` at 0xac978).
  Zero `ret(0x19670)` takes `0xac9bc`, while nonzero falls through (`beq` at 0xac990).
  Host-name storage ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac95c | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xac960 | load | 4 | `abs 0xac9d0` | `-` |
| `ldr` at 0xac970 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac974 | load | 4 | `abs 0xac9d0` | `-` |
| `str` at 0xac978 | store | 4 | `[sp-20] +0x0` | `0x133ccc` |
| `ldr` at 0xac97c | load | 4 | `abs 0xac9d0` | `-` |
| `ldr` at 0xac99c | load | 4 | `abs 0xac9d4` | `-` |
| `ldr` at 0xac9a0 | load | 4 | `abs 0xac9d8` | `-` |
| `ldr` at 0xac9a4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac9a8 | load | 4 | `abs 0xac9dc` | `-` |
| `str` at 0xac9b4 | store | 4 | `sp -0x18` | `?` |
| `str` at 0xac9c0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xac9c4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xac9cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac9cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac9cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac96c | `(unknown callee) 0x19748` | `0x133ccc` | `0x0` | `0x42` | `r3` |
| `bl` at 0xac984 | `(unknown callee) 0x19670` | `0x133ccc` | `0x41` | `0x133ccc` | `[sp-20]` |
| `bl` at 0xac994 | `(unknown callee) 0x193d0` | `ret(0x19670)` | `?` | `?` | `ret(0x19670)` |
| `bl` at 0xac9ac | `(unknown callee) 0x1f538` | `0xfffffe68` | `0x11442c` | `[ret(0x193d0)]` | `0x11430c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xac990 | `ret(0x19670) == 0x0` | `ret(0x19670) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac9cc | `r0` | `?` |

## `SocketIsLocalHost`

- `SocketIsLocalHost`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac8d4 to 0xac94c.
  Established. A word assignment initializes `sp -0x20` to zero (`str` at 0xac8ec).
  The unnamed `0x19310` call carries `[sp-20]`, `[sp-28]`, unknown and `ret(0xac94c)` (`bl` at 0xac91c);
  its nonzero result takes `0xac934`, while zero falls through to a word assignment of `0x1` to `sp -0x20` (`bne` at 0xac928, `str` at 0xac930).
  Hardware-locality policy remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac8e4 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac8ec | store | 4 | `sp -0x20` | `0x0` |
| `str` at 0xac8f8 | store | 4 | `sp -0x18` | `ret(0x1f4d8)` |
| `ldr` at 0xac914 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac918 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac930 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xac934 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac93c | load | 4 | `sp -0x20` | `-` |
| `ldm` at 0xac948 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac948 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac948 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac8f0 | `(unknown callee) 0x1f4d8` | `r0` | `r1` | `r2` | `0x0` |
| `bl` at 0xac904 | `(unknown callee) 0xac94c` | `sp-28` | `?` | `?` | `sp-28` |
| `bl` at 0xac91c | `(unknown callee) 0x19310` | `[sp-20]` | `[sp-28]` | `?` | `ret(0xac94c)` |
| `bl` at 0xac938 | `(unknown callee) 0x1f510` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xac910 | `ret(0xac94c) != 0x0` | `ret(0xac94c) == 0x0` |
| `bne` at 0xac928 | `ret(0x19310) != 0x0` | `ret(0x19310) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac948 | `r0` | `[sp-32]` |

## `SocketGetHostAddr`

- `SocketGetHostAddr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac9e0 to 0xacabc.
  Established. Included word accesses read `0x133d50` and assign `[0x133d50]` to `[sp-20] +0x0` (`ldr` at 0xaca90, `str` at 0xaca94).
  The unnamed `0xac658` call carries `0x133d0e`, `0x133d50` and two unknown candidates (`bl` at 0xaca48);
  its zero result takes `0xaca88`, while nonzero falls through (`beq` at 0xaca54).
  Host-address cache ownership remains Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac9f0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xaca10 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xaca18 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaca1c | load | 4 | `abs 0xacaac` | `-` |
| `ldr` at 0xaca34 | load | 4 | `abs 0xacaac` | `-` |
| `ldr` at 0xaca38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xaca40 | load | 4 | `abs 0xacaac` | `-` |
| `ldr` at 0xaca44 | load | 4 | `abs 0xacab0` | `-` |
| `str` at 0xaca5c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xaca60 | load | 4 | `abs 0xacab4` | `-` |
| `str` at 0xaca64 | store | 4 | `sp -0x20` | `0x11430c` |
| `ldr` at 0xaca68 | load | 4 | `abs 0xacab8` | `-` |
| `ldr` at 0xaca6c | load | 4 | `abs 0xacab4` | `-` |
| `ldr` at 0xaca74 | load | 4 | `abs 0xacaac` | `-` |
| `str` at 0xaca80 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xaca88 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xaca8c | load | 4 | `abs 0xacab0` | `-` |
| `ldr` at 0xaca90 | load | 4 | `abs 0x133d50` | `-` |
| `str` at 0xaca94 | store | 4 | `[sp-20] +0x0` | `[0x133d50]` |
| `str` at 0xaca9c | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xacaa0 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xacaa8 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xacaa8 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xacaa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac9fc | `(unknown callee) 0xac94c` | `sp-24` | `r1` | `r2` | `sp-24` |
| `bl` at 0xaca24 | `(unknown callee) 0x1955c` | `[sp-24]` | `0x133d0e` | `0x41` | `ret(0xac94c)` |
| `bl` at 0xaca3c | `(unknown callee) 0x19808` | `0x133d0e` | `[sp-24]` | `?` | `ret(0x1955c)` |
| `bl` at 0xaca48 | `(unknown callee) 0xac658` | `0x133d0e` | `0x133d50` | `?` | `?` |
| `bl` at 0xaca78 | `(unknown callee) 0x1f5a0` | `0xfffffe6a` | `0x11430c` | `0x0` | `0x133d0e` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xaca08 | `ret(0xac94c) == 0x0` | `ret(0xac94c) != 0x0` |
| `beq` at 0xaca30 | `ret(0x1955c) == 0x0` | `ret(0x1955c) != 0x0` |
| `beq` at 0xaca54 | `ret(0xac658) == 0x0` | `ret(0xac658) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xacaa8 | `r0` | `[sp-28]` |

## `SocketAddrStr`

- `SocketAddrStr`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xacabc to 0xacaf0.
  Established. A local word assignment preserves entry `r0` at `sp -0x18` (`str` at 0xacad4).
  The unnamed `0x1940c` call carries entry `r0`, entry `r1`, entry `r2` and entry `r0` (`bl` at 0xacadc);
  its result is the integer return candidate (`ldm` at 0xacaec).
  Hardware address-to-text responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xacacc | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0xacad0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xacad4 | store | 4 | `sp -0x18` | `r0` |
| `ldr` at 0xacad8 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0xacaec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xacaec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xacaec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xacadc | `(unknown callee) 0x1940c` | `r0` | `r1` | `r2` | `r0` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xacaec | `r0` | `ret(0x1940c)` |

## `SocketNodelayMode`

- `SocketNodelayMode`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xabde0 to 0xabe70.
  Established. Zero entry `r1` takes `0xabe50`, while nonzero falls through (`beq` at 0xabe00).
  The unnamed `0x19604` call carries entry `r0`, `0x4`, `0x800` and entry `r1` (`bl` at 0xabe10).
  A separate word assignment sets `sp -0x1c` to zero (`str` at 0xabe54).
  Hardware timing and buffering effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xabdf0 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xabdf4 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xabdf8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xabe04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xabe28 | load | 4 | `abs 0xabe64` | `-` |
| `ldr` at 0xabe2c | load | 4 | `abs 0xabe68` | `-` |
| `ldr` at 0xabe30 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xabe34 | load | 4 | `abs 0xabe6c` | `-` |
| `ldr` at 0xabe3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xabe48 | store | 4 | `sp -0x1c` | `?` |
| `str` at 0xabe54 | store | 4 | `sp -0x1c` | `0x0` |
| `ldr` at 0xabe58 | load | 4 | `sp -0x1c` | `-` |
| `ldm` at 0xabe60 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xabe60 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xabe60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xabe10 | `(unknown callee) 0x19604` | `r0` | `0x4` | `0x800` | `r1` |
| `bl` at 0xabe20 | `(unknown callee) 0x193d0` | `ret(0x19604)` | `?` | `?` | `ret(0x19604)` |
| `bl` at 0xabe38 | `(unknown callee) 0x1f538` | `0xfffffe6c` | `0x114348` | `[ret(0x193d0)]` | `0x114350` |
| `bl` at 0xabe40 | `(unknown callee) 0x1931c` | `[sp-20]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xabe00 | `r1 == 0x0` | `r1 != 0x0` |
| `beq` at 0xabe1c | `ret(0x19604) == 0x0` | `ret(0x19604) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xabe60 | `r0` | `?` |

## `SocketSetBufferSize`

- `SocketSetBufferSize`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xac230 to 0xac56c.
  Established. The unnamed `0x194cc` call carries entry `r0`, `0x1`, `0x7` and `sp-24` (`bl` at 0xac27c).
  Another unnamed `0x1949c` call carries `[sp-20]`, `0x1`, `0x8` and `sp-40` (`bl` at 0xac3c8).
  Later word assignments set `sp -0x2c` to `0x1` and then `0x10` (`str` at 0xac43c, `str` at 0xac4bc).
  Equality `[ret(0x193d0)] == 0x5f` takes `0xac538`, while inequality falls through (`beq` at 0xac50c).
  The hardware buffer limits remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xac240 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xac244 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xac248 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xac24c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xac25c | store | 4 | `sp -0x20` | `0x4` |
| `ldr` at 0xac264 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac268 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0xac26c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac284 | store | 4 | `sp -0x24` | `ret(0x194cc)` |
| `ldr` at 0xac288 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac2a0 | load | 4 | `abs 0xac54c` | `-` |
| `ldr` at 0xac2a4 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac2a8 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac2b4 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xac2c0 | store | 4 | `sp -0x20` | `0x4` |
| `str` at 0xac2cc | store | 4 | `sp -0x34` | `sp-32` |
| `ldr` at 0xac2d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac2e4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac2e8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac2f4 | load | 4 | `abs 0xac554` | `-` |
| `ldr` at 0xac2f8 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xac2fc | load | 4 | `abs 0xac558` | `-` |
| `ldr` at 0xac300 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac304 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac318 | load | 4 | `abs 0xac54c` | `-` |
| `ldr` at 0xac31c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac320 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac32c | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xac334 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac344 | store | 4 | `sp -0x20` | `0x4` |
| `ldr` at 0xac34c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac350 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0xac354 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac36c | store | 4 | `sp -0x24` | `ret(0x194cc)` |
| `ldr` at 0xac370 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac388 | load | 4 | `abs 0xac55c` | `-` |
| `ldr` at 0xac38c | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac390 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac39c | store | 4 | `sp -0x30` | `?` |
| `str` at 0xac3a8 | store | 4 | `sp -0x20` | `0x4` |
| `str` at 0xac3b4 | store | 4 | `sp -0x34` | `sp-32` |
| `ldr` at 0xac3b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xac3cc | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac3d0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac3dc | load | 4 | `abs 0xac554` | `-` |
| `ldr` at 0xac3e0 | load | 4 | `abs 0x12eeb8` | `-` |
| `ldr` at 0xac3e4 | load | 4 | `abs 0xac560` | `-` |
| `ldr` at 0xac3e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xac3ec | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xac400 | load | 4 | `abs 0xac55c` | `-` |
| `ldr` at 0xac404 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac408 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac414 | store | 4 | `sp -0x30` | `?` |
| `ldr` at 0xac41c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xac428 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xac43c | store | 4 | `sp -0x2c` | `0x1` |
| `str` at 0xac444 | store | 4 | `sp -0x20` | `0x4` |
| `ldr` at 0xac44c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac450 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0xac454 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac46c | store | 4 | `sp -0x24` | `ret(0x194cc)` |
| `ldr` at 0xac470 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac484 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac49c | load | 4 | `abs 0xac564` | `-` |
| `ldr` at 0xac4a0 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac4a4 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac4b0 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xac4bc | store | 4 | `sp -0x2c` | `0x10` |
| `str` at 0xac4c4 | store | 4 | `sp -0x20` | `0x4` |
| `ldr` at 0xac4cc | load | 4 | `sp -0x20` | `-` |
| `str` at 0xac4d0 | store | 4 | `sp -0x34` | `0x4` |
| `ldr` at 0xac4d4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xac4ec | store | 4 | `sp -0x24` | `ret(0x194cc)` |
| `ldr` at 0xac4f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xac504 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac51c | load | 4 | `abs 0xac568` | `-` |
| `ldr` at 0xac520 | load | 4 | `ret(0x193d0) +0x0` | `-` |
| `ldr` at 0xac524 | load | 4 | `abs 0xac550` | `-` |
| `str` at 0xac530 | store | 4 | `sp -0x30` | `?` |
| `str` at 0xac53c | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xac540 | load | 4 | `sp -0x30` | `-` |
| `ldm` at 0xac548 | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0xac548 | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0xac548 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xac27c | `(unknown callee) 0x194cc` | `r0` | `0x1` | `0x7` | `sp-24` |
| `bl` at 0xac294 | `(unknown callee) 0x193d0` | `ret(0x194cc)` | `?` | `?` | `ret(0x194cc)` |
| `bl` at 0xac2ac | `(unknown callee) 0x1f538` | `?` | `0x114364` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac2e0 | `(unknown callee) 0x1949c` | `[sp-20]` | `0x1` | `0x7` | `sp-40` |
| `bl` at 0xac308 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x114374` | `[sp-24]` | `[sp-40]` |
| `bl` at 0xac30c | `(unknown callee) 0x193d0` | `ret(0x222f0)` | `?` | `?` | `?` |
| `bl` at 0xac324 | `(unknown callee) 0x1f538` | `?` | `0x114364` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac364 | `(unknown callee) 0x194cc` | `?` | `0x1` | `0x8` | `sp-28` |
| `bl` at 0xac37c | `(unknown callee) 0x193d0` | `ret(0x194cc)` | `?` | `?` | `ret(0x194cc)` |
| `bl` at 0xac394 | `(unknown callee) 0x1f538` | `?` | `0x1143a8` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac3c8 | `(unknown callee) 0x1949c` | `[sp-20]` | `0x1` | `0x8` | `sp-40` |
| `bl` at 0xac3f0 | `(unknown callee) 0x222f0` | `[0x12eeb8]` | `0x1143b8` | `[sp-28]` | `[sp-40]` |
| `bl` at 0xac3f4 | `(unknown callee) 0x193d0` | `ret(0x222f0)` | `?` | `?` | `?` |
| `bl` at 0xac40c | `(unknown callee) 0x1f538` | `?` | `0x1143a8` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac464 | `(unknown callee) 0x194cc` | `?` | `0x6` | `0x1` | `sp-44` |
| `bl` at 0xac47c | `(unknown callee) 0x193d0` | `ret(0x194cc)` | `?` | `?` | `ret(0x194cc)` |
| `bl` at 0xac490 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xac4a8 | `(unknown callee) 0x1f538` | `?` | `0x1143ec` | `[ret(0x193d0)]` | `0x11430c` |
| `bl` at 0xac4e4 | `(unknown callee) 0x194cc` | `[sp-20]` | `0x0` | `0x1` | `sp-44` |
| `bl` at 0xac4fc | `(unknown callee) 0x193d0` | `ret(0x194cc)` | `?` | `?` | `ret(0x194cc)` |
| `bl` at 0xac510 | `(unknown callee) 0x193d0` | `ret(0x193d0)` | `?` | `?` | `[ret(0x193d0)]` |
| `bl` at 0xac528 | `(unknown callee) 0x1f538` | `?` | `0x114404` | `[ret(0x193d0)]` | `0x11430c` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xac254 | `r1 == 0x0` | `r1 != 0x0` |
| `bge` at 0xac290 | `ret(0x194cc) >= 0x0 (signed, 32-bit)` | `ret(0x194cc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xac2f0 | `[sp-40] == [sp-24]` | `[sp-40] != [sp-24]` |
| `beq` at 0xac33c | `Z == 1` | `not (Z == 1)` |
| `bge` at 0xac378 | `ret(0x194cc) >= 0x0 (signed, 32-bit)` | `ret(0x194cc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xac3d8 | `[sp-40] == [sp-28]` | `[sp-40] != [sp-28]` |
| `bne` at 0xac424 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xac430 | `Z == 0` | `not (Z == 0)` |
| `bge` at 0xac478 | `ret(0x194cc) >= 0x0 (signed, 32-bit)` | `ret(0x194cc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xac48c | `[ret(0x193d0)] == 0x5f` | `[ret(0x193d0)] != 0x5f` |
| `bge` at 0xac4f8 | `ret(0x194cc) >= 0x0 (signed, 32-bit)` | `ret(0x194cc) < 0x0 (signed, 32-bit)` |
| `beq` at 0xac50c | `[ret(0x193d0)] == 0x5f` | `[ret(0x193d0)] != 0x5f` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0xac548 | `r0` | `?` |

## `charioLock`

- `charioLock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x22184 to 0x221d4.
  Established. The unnamed `0x22518` call carries the four entry registers `r0`, `r1`, `r2` and `r3` (`bl` at 0x2219c);
  its zero result takes `0x221b8`, while nonzero falls through (`beq` at 0x221a8).
  The zero path reads a word from `0x12f678` and assigns `[0x12f678]` to `sp -0x18` (`ldr` at 0x221bc, `str` at 0x221c0).
  Character-I/O locking responsibilities remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0x22194 | store | 4 | `sp -0x14` | `r0` |
| `ldr` at 0x22198 | load | 4 | `sp -0x14` | `-` |
| `str` at 0x221b0 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0x221b8 | load | 4 | `abs 0x221d0` | `-` |
| `ldr` at 0x221bc | load | 4 | `abs 0x12f678` | `-` |
| `str` at 0x221c0 | store | 4 | `sp -0x18` | `[0x12f678]` |
| `ldr` at 0x221c4 | load | 4 | `sp -0x18` | `-` |
| `ldm` at 0x221cc | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x221cc | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x221cc | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x2219c | `(unknown callee) 0x22518` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0x221a8 | `ret(0x22518) == 0x0` | `ret(0x22518) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x221cc | `r0` | `[sp-24]` |

## `charioUnlock`

- `charioUnlock`, `6.5.30/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.30:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0x221d4 to 0x221f0.
  Established. An unnamed `0x2263c` call carries entry `r0`, entry `r1`, entry `r2` and entry `r3` (`bl` at 0x221e0).
  The integer return candidate preserves `ret(0x2263c)` (`ldm` at 0x221ec).
  Lock release policy and hardware effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `ldm` at 0x221ec | load | 4 | `sp -0x10` | `-` |
| `ldm` at 0x221ec | load | 4 | `sp -0xc` | `-` |
| `ldm` at 0x221ec | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0x221e0 | `(unknown callee) 0x2263c` | `r0` | `r1` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldm` at 0x221ec | `r0` | `ret(0x2263c)` |
