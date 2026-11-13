# ARM router packet paths in 1.38.4

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
No unresolved computed transfers were reported for these routines. Undecodable byte ranges remain visible as fact-sheet warnings.

[Fact sheets and full listings](disasm/arm-1.38.4-package4/router-packet-paths-facts.txt) retain all tool sections.

## `IRouterInit`

- `IRouterInit`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb546c to 0xb553c.
  Established. The byte test takes the arm that stores word `0x2` at `0x149ae0` for `[sp-21]:1 ==
  0x0`; nonzero falls through to store word `0x1` there (`beq` at 0xb5490, `str` at 0xb54ac, `str`
  at 0xb549c). A later word store writes an unknown value at `0x2b1d00` (`str` at 0xb54d8). The call
  to `0xb553c` has `0x2b1ce0`, `0x1`, `0x3`, unknown (`bl` at 0xb54e8). The r0 return is `[sp-24]`,
  while router initialization semantics remain unknown (`ldmdb` at 0xb551c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb547c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5484 | store | 4 | `sp -0x18` | `0x1` |
| `ldrb` at 0xb5488 | load | 1 | `sp -0x15` | `-` |
| `ldr` at 0xb5494 | load | 4 | `abs 0xb5520` | `-` |
| `str` at 0xb549c | store | 4 | `abs 0x149ae0` | `0x1` |
| `ldr` at 0xb54a4 | load | 4 | `abs 0xb5520` | `-` |
| `str` at 0xb54ac | store | 4 | `abs 0x149ae0` | `0x2` |
| `str` at 0xb54b4 | store | 4 | `sp -0x18` | `0x0` |
| `ldr` at 0xb54b8 | load | 4 | `abs 0xb5524` | `-` |
| `ldr` at 0xb54bc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb54d8 | store | 4 | `abs 0x2b1d00` | `?` |
| `ldr` at 0xb54dc | load | 4 | `abs 0xb5528` | `-` |
| `ldr` at 0xb54ec | load | 4 | `abs 0xb552c` | `-` |
| `ldr` at 0xb54f0 | load | 4 | `abs 0xb5530` | `-` |
| `ldr` at 0xb5500 | load | 4 | `abs 0xb5534` | `-` |
| `ldr` at 0xb5504 | load | 4 | `abs 0xb5538` | `-` |
| `ldr` at 0xb5514 | load | 4 | `sp -0x18` | `-` |
| `ldmdb` at 0xb551c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb551c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb551c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb54e8 | `(unknown callee) 0xb553c` | `0x2b1ce0` | `0x1` | `0x3` | `?` |
| `bl` at 0xb54fc | `(unknown callee) 0x22920` | `0x149b04` | `0x1281a0` | `0x1` | `0x9` |
| `bl` at 0xb5510 | `(unknown callee) 0x22920` | `0x149b58` | `0x1281ac` | `0x1` | `0x9` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb5490 | `[sp-21]:1 == 0x0` | `[sp-21]:1 != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb551c | `r0` | `[sp-24]` |

## `IRouterInitID`

- `IRouterInitID`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb553c to 0xb5624.
  Established. The signed test takes its alternate arm for `[[sp-20]] >= 0x0` and falls through for
  less (`bge` at 0xb559c). After the join, a byte store writes `([sp-28]:1 & 0xff)` at `[sp-20]
  +0x4`, and a halfword store writes `0x0` at `[sp-20] +0x6` (`strb` at 0xb5604, `strh` at 0xb5610).
  The returned r0 value is `0x0` (`ldmdb` at 0xb561c). Identity encoding and authority semantics
  remain unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb554c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5550 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb5554 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb5558 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb555c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5560 | load | 4 | `r0 +0x0` | `-` |
| `ldr` at 0xb556c | load | 4 | `abs 0xb5620` | `-` |
| `ldr` at 0xb5570 | load | 4 | `abs 0x2b1d00` | `-` |
| `str` at 0xb5584 | store | 4 | `r0 +0x0` | `?` |
| `ldr` at 0xb5588 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb558c | store | 4 | `sp -0x20` | `[sp-20]` |
| `ldr` at 0xb5590 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb5594 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb55a0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb55a4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb55a8 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb55b0 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb55c0 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xb55c4 | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb55cc | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb55d0 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb55d4 | load | 4 | `[sp-20] +0x0` | `-` |
| `ldr` at 0xb55dc | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb55e8 | store | 4 | `[sp-20] +0x0` | `?` |
| `str` at 0xb55ec | store | 4 | `sp -0x24` | `?` |
| `ldr` at 0xb55f0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb55f4 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb55f8 | store | 4 | `[sp-32] +0x0` | `[sp-36]` |
| `ldr` at 0xb55fc | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb5600 | load | 1 | `sp -0x1c` | `-` |
| `strb` at 0xb5604 | store | 1 | `[sp-20] +0x4` | `([sp-28]:1 & 0xff)` |
| `ldr` at 0xb5608 | load | 4 | `sp -0x14` | `-` |
| `strh` at 0xb5610 | store | 2 | `[sp-20] +0x6` | `0x0` |
| `ldmdb` at 0xb561c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb561c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb561c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bge` at 0xb559c | `[[sp-20]] >= 0x0 (signed, 32-bit)` | `[[sp-20]] < 0x0 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb561c | `r0` | `0x0` |

## `IRouterSetAuthority`

- `IRouterSetAuthority`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5624 to 0xb5654.
  Established. The explicit byte update writes `([sp-24]:1 & 0xff)` at entry `r0 +0x4` (`strb` at
  0xb5644). The return has r0 `0x0` (`ldmdb` at 0xb5650). Earlier word stores save entry r0 at `sp
  -0x14` and entry r1 at `sp -0x18` (`str` at 0xb5634, `str` at 0xb5638). No calls or conditional
  branches are reported. The byte's authority interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5634 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5638 | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb563c | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb5640 | load | 1 | `sp -0x18` | `-` |
| `strb` at 0xb5644 | store | 1 | `r0 +0x4` | `([sp-24]:1 & 0xff)` |
| `ldmdb` at 0xb5650 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5650 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5650 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5650 | `r0` | `0x0` |

## `IRouterCreateQueue`

- `IRouterCreateQueue`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5654 to 0xb5698.
  Established. A word store places `0x1` at `sp -0x20` before the only call (`str` at 0xb5674). That
  call reaches `0x22f84` with entry r0, entry r1, entry r2, `0x7` (`bl` at 0xb5688); its result
  supplies r0 at the return (`ldmdb` at 0xb5694). Queue allocation semantics and the number of stack
  arguments are not established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb5664 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb5668 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb566c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb5674 | store | 4 | `sp -0x20` | `0x1` |
| `ldr` at 0xb5678 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb567c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb5680 | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb5694 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb5694 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb5694 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb5688 | `(unknown callee) 0x22f84` | `r0` | `r1` | `r2` | `0x7` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb5694 | `r0` | `ret(0x22f84)` |

## `IRouterGetPacket`

- `IRouterGetPacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb5698 to 0xb56f4.
  Established. The call to `0x23340` carries entry r1, entry r0, entry r2, entry r3 (`bl` at
  0xb56c0); its zero result takes the arm that stores word `0x0` at `sp -0x28`, while nonzero falls
  through to store word `ret(0x23340)` there (`beq` at 0xb56d4, `str` at 0xb56e8, `str` at 0xb56dc).
  The r0 return is `[sp-40]` (`ldmdb` at 0xb56f0). The referenced packet object and queue behavior
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb56a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb56ac | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb56b0 | store | 4 | `sp -0x1c` | `r2` |
| `ldr` at 0xb56b4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb56b8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb56bc | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb56c8 | store | 4 | `sp -0x24` | `ret(0x23340)` |
| `ldr` at 0xb56cc | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb56d8 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb56dc | store | 4 | `sp -0x28` | `ret(0x23340)` |
| `str` at 0xb56e8 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb56ec | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb56f0 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb56f0 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb56f0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb56c0 | `(unknown callee) 0x23340` | `r1` | `r0` | `r2` | `r3` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb56d4 | `ret(0x23340) == 0x0` | `ret(0x23340) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb56f0 | `r0` | `[sp-40]` |

## `IRouterSendPacket`

- `IRouterSendPacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6798 to 0xb686c.
  Established. The call to `0xb5d98` uses entry r0, entry r1, `0x1`, `[r1+22]:1` (`bl` at 0xb67f0);
  its zero result takes the final path and nonzero falls through (`beq` at 0xb6804). Another arm
  calls `0xb6408`, then stores word `ret(0xb6408)` at `sp -0x1c` before an unknown word overwrites
  that target (`bl` at 0xb6840, `str` at 0xb6848, `str` at 0xb6850). The final call reaches
  `0xb56f4` with `[sp-24]` and three unknown candidates (`bl` at 0xb6858). The r0 return is
  `[sp-28]`; external transmission and ownership effects are not proved (`ldmdb` at 0xb6864).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb67a8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb67ac | store | 4 | `sp -0x18` | `r1` |
| `ldr` at 0xb67b0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb67b4 | load | 1 | `r1 +0x16` | `-` |
| `str` at 0xb67b8 | store | 4 | `sp -0x20` | `[r1+22]:1` |
| `ldr` at 0xb67bc | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb67c8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb67d4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb67e4 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb67e8 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb67f8 | store | 4 | `sp -0x1c` | `ret(0xb5d98)` |
| `ldr` at 0xb67fc | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6808 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb680c | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xb681c | store | 4 | `sp -0x24` | `0x0` |
| `ldr` at 0xb6820 | load | 4 | `abs 0xb6868` | `-` |
| `ldr` at 0xb6824 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6828 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6838 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb683c | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6848 | store | 4 | `sp -0x1c` | `ret(0xb6408)` |
| `str` at 0xb6850 | store | 4 | `sp -0x1c` | `?` |
| `ldr` at 0xb6854 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb685c | load | 4 | `sp -0x1c` | `-` |
| `ldmdb` at 0xb6864 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6864 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6864 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb67f0 | `(unknown callee) 0xb5d98` | `r0` | `r1` | `0x1` | `[r1+22]:1` |
| `bl` at 0xb6830 | `(unknown callee) 0xb6bdc` | `0x2b1ce0` | `ret(0xb5d98)` | `[sp-24]` | `0x0` |
| `bl` at 0xb6840 | `(unknown callee) 0xb6408` | `r0` | `r1` | `r2` | `[r1+22]:1` |
| `bl` at 0xb6858 | `(unknown callee) 0xb56f4` | `[sp-24]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `blt` at 0xb67c4 | `[r1+22]:1 < 0x0 (signed, 32-bit)` | `[r1+22]:1 >= 0x0 (signed, 32-bit)` |
| `ble` at 0xb67d0 | `[r1+22]:1 <= 0x1 (signed, 32-bit)` | `[r1+22]:1 > 0x1 (signed, 32-bit)` |
| `beq` at 0xb67dc | `[r1+22]:1 == 0x2` | `[r1+22]:1 != 0x2` |
| `beq` at 0xb6804 | `ret(0xb5d98) == 0x0` | `ret(0xb5d98) != 0x0` |
| `bne` at 0xb6814 | `[[sp-24]+22]:1 != 0x0` | `[[sp-24]+22]:1 == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6864 | `r0` | `[sp-28]` |

## `IRouterQPacket`

- `IRouterQPacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb686c to 0xb6a9c.
  Established. The first selector takes the alternate arm for `[[sp-24]+22]:1 != 0x2` and falls
  through on equality (`bne` at 0xb68c0). Equality reaches `0x23aa0` with `[sp-20]`, `sp-24`,
  unknown, unknown (`bl` at 0xb691c). The alternate arm calls `0x236b4` with `[sp-20]`, `sp-24`,
  `[0x1429ac]`, `0xb5b40` (`bl` at 0xb695c). Later word stores place `0xfffffd9d` and `0xfffffd9e`
  at `sp -0x28` on separate paths (`str` at 0xb69b8, `str` at 0xb6a38). The r0 return is `[sp-40]`;
  queue and timeout semantics remain unknown (`ldmdb` at 0xb6a70).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb687c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6880 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6888 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb689c | store | 4 | `(unknown)` | `0x0` |
| `str` at 0xb68b0 | store | 4 | `(unknown)` | `0x0` |
| `ldr` at 0xb68b4 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb68b8 | load | 1 | `[sp-24] +0x16` | `-` |
| `str` at 0xb68c8 | store | 4 | `sp -0x54` | `sp-24` |
| `ldr` at 0xb68cc | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb68d0 | load | 2 | `[sp-24] +0x6` | `-` |
| `ldr` at 0xb68e8 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb68ec | load | 2 | `[sp-24] +0x6` | `-` |
| `str` at 0xb68fc | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xb6904 | load | 4 | `abs 0xb6a74` | `-` |
| `ldr` at 0xb6908 | load | 4 | `abs 0x1429b0` | `-` |
| `str` at 0xb690c | store | 4 | `sp -0x58` | `[0x1429b0]` |
| `ldr` at 0xb6910 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6914 | load | 4 | `sp -0x54` | `-` |
| `ldr` at 0xb6918 | load | 4 | `sp -0x58` | `-` |
| `str` at 0xb6924 | store | 4 | `sp -0x28` | `ret(0x23aa0)` |
| `ldr` at 0xb6930 | load | 4 | `abs 0xb6a78` | `-` |
| `ldr` at 0xb6934 | load | 4 | `abs 0xb6a7c` | `-` |
| `str` at 0xb6938 | store | 4 | `sp -0x74` | `0xb5c84` |
| `str` at 0xb6940 | store | 4 | `sp -0x70` | `sp-32` |
| `str` at 0xb6948 | store | 4 | `sp -0x6c` | `0x1` |
| `ldr` at 0xb694c | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6954 | load | 4 | `abs 0x1429ac` | `-` |
| `ldr` at 0xb6958 | load | 4 | `abs 0xb6a80` | `-` |
| `str` at 0xb6964 | store | 4 | `sp -0x28` | `ret(0x236b4)` |
| `ldr` at 0xb6968 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6974 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6978 | store | 4 | `sp -0x68` | `?` |
| `ldr` at 0xb697c | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb6988 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb6994 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb69a4 | load | 4 | `sp -0x68` | `-` |
| `ldr` at 0xb69b4 | load | 4 | `abs 0xb6a84` | `-` |
| `str` at 0xb69b8 | store | 4 | `sp -0x28` | `0xfffffd9d` |
| `str` at 0xb69c0 | store | 4 | `sp -0x5c` | `sp-80` |
| `ldr` at 0xb69c4 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb69cc | store | 4 | `sp -0x60` | `[sp-20]+120` |
| `ldr` at 0xb69d0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb69d4 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb69e0 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb69e4 | load | 1 | `[sp-24] +0x16` | `-` |
| `ldr` at 0xb69f0 | load | 4 | `abs 0xb6a88` | `-` |
| `str` at 0xb69f4 | store | 4 | `sp -0x64` | `0x128230` |
| `ldr` at 0xb69fc | load | 4 | `abs 0xb6a8c` | `-` |
| `str` at 0xb6a00 | store | 4 | `sp -0x64` | `0x1282d8` |
| `ldr` at 0xb6a08 | load | 4 | `abs 0xb6a90` | `-` |
| `str` at 0xb6a0c | store | 4 | `sp -0x64` | `0x12822c` |
| `ldr` at 0xb6a10 | load | 4 | `sp -0x5c` | `-` |
| `ldr` at 0xb6a14 | load | 4 | `abs 0xb6a94` | `-` |
| `ldr` at 0xb6a18 | load | 4 | `sp -0x60` | `-` |
| `ldr` at 0xb6a1c | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb6a34 | load | 4 | `abs 0xb6a98` | `-` |
| `str` at 0xb6a38 | store | 4 | `sp -0x28` | `0xfffffd9e` |
| `ldr` at 0xb6a48 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6a60 | load | 4 | `(unknown)` | `-` |
| `ldr` at 0xb6a68 | load | 4 | `sp -0x28` | `-` |
| `ldmdb` at 0xb6a70 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6a70 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6a70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb691c | `(unknown callee) 0x23aa0` | `[sp-20]` | `sp-24` | `?` | `?` |
| `bl` at 0xb695c | `(unknown callee) 0x236b4` | `[sp-20]` | `sp-24` | `[0x1429ac]` | `0xb5b40` |
| `bl` at 0xb6a20 | `(unknown callee) 0x1b49c` | `sp-80` | `0x1282dc` | `[sp-20]+120` | `?` |
| `bl` at 0xb6a2c | `(unknown callee) 0x20240` | `sp-80` | `?` | `?` | `sp-80` |
| `bl` at 0xb6a64 | `(unknown callee) 0xb56f4` | `?` | `sp-16` | `sp-16` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb68c0 | `[[sp-24]+22]:1 != 0x2` | `[[sp-24]+22]:1 == 0x2` |
| `beq` at 0xb68e4 | `? == 0x0` | `? != 0x0` |
| `beq` at 0xb6970 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb6984 | `Z == 1` | `not (Z == 1)` |
| `bgt` at 0xb6990 | `gt flags condition (operands unknown)` | `not (gt flags condition (operands unknown))` |
| `beq` at 0xb699c | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb69ac | `[sp-104] == 0x6e` | `[sp-104] != 0x6e` |
| `beq` at 0xb69dc | `[[sp-24]+22]:1 == 0x0` | `[[sp-24]+22]:1 != 0x0` |
| `bne` at 0xb69ec | `[[sp-24]+22]:1 != 0x1` | `[[sp-24]+22]:1 == 0x1` |
| `beq` at 0xb6a50 | `? == 0x0` | `? != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6a70 | `r0` | `[sp-40]` |

## `IRouterRequest`

- `IRouterRequest`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6a9c to 0xb6bdc.
  Established. The entry stack-word test takes the bypass for `[sp+8] == 0x0` and falls through
  otherwise (`beq` at 0xb6ac4). Fall-through stores word `[r0+6]:2` at `[sp+8] +0x0` (`str` at
  0xb6ad4). The call to `0xb4c58` has `[sp-48]`, `0x0`, `0x0`, `[sp-52]` (`bl` at 0xb6b88); its zero
  result takes the path to `0xb5d98` with `[sp-20]`, `[sp-40]`, `0x1`, `ret(0xb4c58)`; nonzero falls
  through (`beq` at 0xb6b9c, `bl` at 0xb6bb8). Protocol fields and the joined r0 return remain
  unknown (`ldmdb` at 0xb6bd8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6aac | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6ab0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6ab4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6ab8 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb6abc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6ac8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6acc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6ad0 | load | 2 | `r0 +0x6` | `-` |
| `str` at 0xb6ad4 | store | 4 | `[sp+8] +0x0` | `[r0+6]:2` |
| `str` at 0xb6adc | store | 4 | `sp -0x30` | `sp-40` |
| `ldr` at 0xb6ae0 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6ae4 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb6ae8 | store | 4 | `sp -0x34` | `?` |
| `ldr` at 0xb6aec | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6af0 | store | 4 | `sp -0x5c` | `?` |
| `str` at 0xb6af8 | store | 4 | `sp -0x58` | `0x0` |
| `ldr` at 0xb6afc | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6b00 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb6b08 | store | 2 | `(unknown)` | `?` |
| `str` at 0xb6b14 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb6b18 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6b1c | store | 4 | `sp -0x50` | `[sp-24]` |
| `ldr` at 0xb6b20 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6b2c | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6b30 | load | 4 | `[sp-28] +0x0` | `-` |
| `ldr` at 0xb6b3c | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6b40 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb6b48 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6b4c | store | 4 | `sp -0x38` | `[sp-28]` |
| `ldr` at 0xb6b50 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb6b54 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb6b58 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb6b5c | store | 4 | `sp -0x48` | `[sp]` |
| `ldr` at 0xb6b60 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb6b64 | store | 4 | `sp -0x44` | `[sp+4]` |
| `ldr` at 0xb6b68 | load | 4 | `sp +0xc` | `-` |
| `str` at 0xb6b6c | store | 4 | `sp -0x40` | `[sp+12]` |
| `str` at 0xb6b74 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xb6b78 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb6b84 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb6b90 | store | 4 | `sp -0x24` | `ret(0xb4c58)` |
| `ldr` at 0xb6b94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6ba0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6ba4 | store | 4 | `sp -0x2c` | `ret(0xb4c58)` |
| `ldr` at 0xb6bac | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6bb0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6bc0 | store | 4 | `sp -0x24` | `ret(0xb5d98)` |
| `ldr` at 0xb6bc4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6bcc | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6bd0 | store | 4 | `sp -0x2c` | `[sp-36]` |
| `ldr` at 0xb6bd4 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb6bd8 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6bd8 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6bd8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6b88 | `(unknown callee) 0xb4c58` | `[sp-48]` | `0x0` | `0x0` | `[sp-52]` |
| `bl` at 0xb6bb8 | `(unknown callee) 0xb5d98` | `[sp-20]` | `[sp-40]` | `0x1` | `ret(0xb4c58)` |
| `bl` at 0xb6bc8 | `(unknown callee) 0xb56f4` | `[sp-40]` | `?` | `?` | `ret(0xb5d98)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6ac4 | `[sp+8] == 0x0` | `[sp+8] != 0x0` |
| `beq` at 0xb6b28 | `[sp-28] == 0x0` | `[sp-28] != 0x0` |
| `bne` at 0xb6b38 | `[[sp-28]] != 0x0` | `[[sp-28]] == 0x0` |
| `beq` at 0xb6b9c | `ret(0xb4c58) == 0x0` | `ret(0xb4c58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6bd8 | `r0` | `?` |

## `IRouterResponse`

- `IRouterResponse`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6bdc to 0xb6d54.
  Established. The first branch takes the alternate byte-read arm for `r0 == 0x0` and falls through
  otherwise (`beq` at 0xb6c0c). Those reads address entry `r2 +0x17` and entry `r0 +0x4`,
  respectively (`ldrb` at 0xb6c24, `ldrb` at 0xb6c14). The call to `0xb4c58` uses `[sp-48]`, `0x1`,
  `0x0`, `[sp-52]` (`bl` at 0xb6cfc); its zero result takes the later-call path and nonzero falls
  through (`beq` at 0xb6d10). A following call to `0xb56f4` has `[sp-36]`, unknown, unknown,
  `ret(0xb5d98)` (`bl` at 0xb6d3c). Response encoding and the returned r0 value are unproved
  (`ldmdb` at 0xb6d4c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6bec | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6bf0 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6bf4 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6bf8 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb6c00 | store | 4 | `sp -0x30` | `sp-36` |
| `ldr` at 0xb6c04 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6c10 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6c14 | load | 1 | `r0 +0x4` | `-` |
| `str` at 0xb6c18 | store | 4 | `sp -0x34` | `[r0+4]:1` |
| `ldr` at 0xb6c20 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb6c24 | load | 1 | `r2 +0x17` | `-` |
| `str` at 0xb6c28 | store | 4 | `sp -0x34` | `[r2+23]:1` |
| `ldr` at 0xb6c2c | load | 4 | `sp -0x1c` | `-` |
| `ldrh` at 0xb6c30 | load | 2 | `r2 +0x22` | `-` |
| `str` at 0xb6c34 | store | 4 | `sp -0x5c` | `[r2+34]:2` |
| `ldr` at 0xb6c38 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6c44 | load | 4 | `abs 0xb6d50` | `-` |
| `str` at 0xb6c74 | store | 4 | `sp -0x58` | `?` |
| `ldr` at 0xb6c78 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb6c7c | load | 1 | `[sp-28] +0x14` | `-` |
| `ldr` at 0xb6c80 | load | 4 | `sp -0x1c` | `-` |
| `ldrb` at 0xb6c84 | load | 1 | `[sp-28] +0x21` | `-` |
| `str` at 0xb6c90 | store | 4 | `sp -0x54` | `?` |
| `ldr` at 0xb6c94 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6c9c | store | 4 | `sp -0x50` | `[sp-28]+28` |
| `ldr` at 0xb6ca0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6ca4 | store | 4 | `sp -0x38` | `[sp-20]` |
| `ldr` at 0xb6ca8 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6cb4 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6cbc | store | 4 | `sp -0x38` | `[sp-28]+24` |
| `ldr` at 0xb6cc0 | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb6cc4 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb6cc8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6ccc | store | 4 | `sp -0x48` | `[sp-32]` |
| `ldr` at 0xb6cd0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb6cd4 | store | 4 | `sp -0x44` | `[sp]` |
| `ldr` at 0xb6cd8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6cdc | load | 4 | `[sp-28] +0xc` | `-` |
| `str` at 0xb6ce0 | store | 4 | `sp -0x40` | `[[sp-28]+12]` |
| `str` at 0xb6ce8 | store | 4 | `sp -0x3c` | `0x0` |
| `ldr` at 0xb6cec | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb6cf8 | load | 4 | `sp -0x34` | `-` |
| `str` at 0xb6d04 | store | 4 | `sp -0x28` | `ret(0xb4c58)` |
| `ldr` at 0xb6d08 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6d14 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6d18 | store | 4 | `sp -0x2c` | `ret(0xb4c58)` |
| `ldr` at 0xb6d20 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6d24 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6d34 | store | 4 | `sp -0x28` | `ret(0xb5d98)` |
| `ldr` at 0xb6d38 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6d40 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6d44 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xb6d48 | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb6d4c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6d4c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6d4c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6cfc | `(unknown callee) 0xb4c58` | `[sp-48]` | `0x1` | `0x0` | `[sp-52]` |
| `bl` at 0xb6d2c | `(unknown callee) 0xb5d98` | `[sp-20]` | `[sp-36]` | `0x1` | `ret(0xb4c58)` |
| `bl` at 0xb6d3c | `(unknown callee) 0xb56f4` | `[sp-36]` | `?` | `?` | `ret(0xb5d98)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6c0c | `r0 == 0x0` | `r0 != 0x0` |
| `bne` at 0xb6cb0 | `[sp-20] != 0x0` | `[sp-20] == 0x0` |
| `beq` at 0xb6d10 | `ret(0xb4c58) == 0x0` | `ret(0xb4c58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6d4c | `r0` | `?` |

## `IRouterEvent`

- `IRouterEvent`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6d54 to 0xb6e38.
  Established. A word store advances the absolute word at `0x149b00` with `[0x149b00]+1` (`str` at
  0xb6da4). The call to `0xb4c58` carries `sp-36`, `0x2`, `0x0`, `[r0+4]:1` (`bl` at 0xb6de4); its
  zero result takes the arm calling `0xb6408` with `[sp-20]`, `[sp-36]`, unknown, `ret(0xb4c58)`,
  while nonzero falls through (`beq` at 0xb6df8, `bl` at 0xb6e10). Event delivery and the joined r0
  value remain unknown (`ldmdb` at 0xb6e30).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6d64 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6d68 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6d6c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6d70 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb6d78 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6d7c | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xb6d80 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6d84 | store | 4 | `sp -0x50` | `r1` |
| `str` at 0xb6d8c | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xb6d90 | load | 4 | `abs 0xb6e34` | `-` |
| `ldr` at 0xb6d94 | load | 4 | `abs 0x149b00` | `-` |
| `str` at 0xb6d9c | store | 4 | `sp -0x48` | `[0x149b00]` |
| `str` at 0xb6da4 | store | 4 | `abs 0x149b00` | `[0x149b00]+1` |
| `str` at 0xb6dac | store | 4 | `sp -0x44` | `0x0` |
| `ldr` at 0xb6db0 | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6db4 | store | 4 | `sp -0x40` | `[sp-20]` |
| `ldr` at 0xb6db8 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6dbc | store | 4 | `sp -0x3c` | `[sp-28]` |
| `ldr` at 0xb6dc0 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6dc4 | store | 4 | `sp -0x38` | `[sp-32]` |
| `str` at 0xb6dcc | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb6dd0 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb6dd4 | store | 4 | `sp -0x30` | `[sp]` |
| `str` at 0xb6dec | store | 4 | `sp -0x28` | `ret(0xb4c58)` |
| `ldr` at 0xb6df0 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6dfc | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6e00 | store | 4 | `sp -0x2c` | `ret(0xb4c58)` |
| `ldr` at 0xb6e08 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6e0c | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6e18 | store | 4 | `sp -0x28` | `ret(0xb6408)` |
| `ldr` at 0xb6e1c | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6e24 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb6e28 | store | 4 | `sp -0x2c` | `[sp-40]` |
| `ldr` at 0xb6e2c | load | 4 | `sp -0x2c` | `-` |
| `ldmdb` at 0xb6e30 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6e30 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6e30 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6de4 | `(unknown callee) 0xb4c58` | `sp-36` | `0x2` | `0x0` | `[r0+4]:1` |
| `bl` at 0xb6e10 | `(unknown callee) 0xb6408` | `[sp-20]` | `[sp-36]` | `?` | `ret(0xb4c58)` |
| `bl` at 0xb6e20 | `(unknown callee) 0xb56f4` | `[sp-36]` | `?` | `?` | `ret(0xb6408)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6df8 | `ret(0xb4c58) == 0x0` | `ret(0xb4c58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6e30 | `r0` | `?` |

## `IRouterCommand`

- `IRouterCommand`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6e38 to 0xb6f64.
  Established. The opening call targets `0xb6a9c` with entry r0, entry r1, entry r2, entry r3 (`bl`
  at 0xb6e88); its zero result takes the continuation and nonzero falls through (`beq` at 0xb6e9c).
  Continuation calls `0xb5698` with `[sp+8]`, `[sp+12]`, `[sp+16]`, `ret(0xb6a9c)` (`bl` at
  0xb6eb8). Two later paths store word `0xfffffd96` at `sp -0x30` (`str` at 0xb6f2c, `str` at
  0xb6f48). Command matching, record fields and the merged return remain unknown (`ldmdb` at
  0xb6f5c).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6e48 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6e4c | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6e50 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6e54 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb6e58 | load | 4 | `sp +0x0` | `-` |
| `str` at 0xb6e5c | store | 4 | `sp -0x40` | `[sp]` |
| `ldr` at 0xb6e60 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb6e64 | store | 4 | `sp -0x3c` | `[sp+4]` |
| `str` at 0xb6e6c | store | 4 | `sp -0x38` | `sp-40` |
| `str` at 0xb6e74 | store | 4 | `sp -0x34` | `0x0` |
| `ldr` at 0xb6e78 | load | 4 | `sp -0x14` | `-` |
| `ldr` at 0xb6e7c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb6e80 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb6e84 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb6e90 | store | 4 | `sp -0x24` | `ret(0xb6a9c)` |
| `ldr` at 0xb6e94 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6ea0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6ea4 | store | 4 | `sp -0x30` | `ret(0xb6a9c)` |
| `ldr` at 0xb6eac | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6eb0 | load | 4 | `sp +0xc` | `-` |
| `ldr` at 0xb6eb4 | load | 4 | `sp +0x10` | `-` |
| `str` at 0xb6ec0 | store | 4 | `sp -0x24` | `ret(0xb5698)` |
| `ldr` at 0xb6ec4 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb6ed0 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb6ed4 | store | 4 | `sp -0x30` | `ret(0xb5698)` |
| `ldr` at 0xb6edc | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6ee0 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb6ee4 | load | 1 | `[[sp+8]] +0x14` | `-` |
| `ldr` at 0xb6ee8 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6eec | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb6ef0 | load | 1 | `[[sp+8]] +0x21` | `-` |
| `str` at 0xb6efc | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb6f00 | load | 4 | `sp +0x8` | `-` |
| `ldr` at 0xb6f04 | load | 4 | `[sp+8] +0x0` | `-` |
| `ldrb` at 0xb6f08 | load | 1 | `[[sp+8]] +0x12` | `-` |
| `ldr` at 0xb6f14 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6f1c | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6f28 | load | 4 | `abs 0xb6f60` | `-` |
| `str` at 0xb6f2c | store | 4 | `sp -0x30` | `0xfffffd96` |
| `ldr` at 0xb6f34 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb6f38 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb6f44 | load | 4 | `abs 0xb6f60` | `-` |
| `str` at 0xb6f48 | store | 4 | `sp -0x30` | `0xfffffd96` |
| `str` at 0xb6f54 | store | 4 | `sp -0x30` | `0x0` |
| `ldr` at 0xb6f58 | load | 4 | `sp -0x30` | `-` |
| `ldmdb` at 0xb6f5c | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb6f5c | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb6f5c | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb6e88 | `(unknown callee) 0xb6a9c` | `r0` | `r1` | `r2` | `r3` |
| `bl` at 0xb6eb8 | `(unknown callee) 0xb5698` | `[sp+8]` | `[sp+12]` | `[sp+16]` | `ret(0xb6a9c)` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb6e9c | `ret(0xb6a9c) == 0x0` | `ret(0xb6a9c) != 0x0` |
| `beq` at 0xb6ecc | `ret(0xb5698) == 0x0` | `ret(0xb5698) != 0x0` |
| `bne` at 0xb6f10 | `[[[sp+8]]+18]:1 != 0x0` | `[[[sp+8]]+18]:1 == 0x0` |
| `beq` at 0xb6f24 | `? == [sp-44]` | `? != [sp-44]` |
| `beq` at 0xb6f40 | `[sp-40] == [sp-44]` | `[sp-40] != [sp-44]` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb6f5c | `r0` | `?` |

## `IRouterMakePacket`

- `IRouterMakePacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb6f64 to 0xb702c.
  Established. The halfword update at entry `r0 +0x6` writes `([r0+6]:2+1 & 0xffff)` (`strh` at
  0xb6fb4). The only call reaches `0xb4c58` with `sp-36`, `[sp-32]`, `[sp-40]`, `[r0+4]:1` (`bl` at
  0xb7000); its zero result takes the continuation, while nonzero falls through to store word `0x0`
  at `sp -0x24` (`beq` at 0xb700c, `str` at 0xb7014). The r0 return is `[sp-36]`; packet allocation
  and field meanings are not established (`ldmdb` at 0xb7028).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb6f74 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb6f78 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb6f7c | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb6f80 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb6f88 | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb6f90 | load | 4 | `sp -0x14` | `-` |
| `ldrb` at 0xb6f94 | load | 1 | `r0 +0x4` | `-` |
| `ldr` at 0xb6f98 | load | 4 | `sp -0x1c` | `-` |
| `str` at 0xb6f9c | store | 4 | `sp -0x4c` | `r2` |
| `str` at 0xb6fa4 | store | 4 | `sp -0x48` | `0x0` |
| `ldr` at 0xb6fa8 | load | 4 | `sp -0x14` | `-` |
| `ldrh` at 0xb6fac | load | 2 | `r0 +0x6` | `-` |
| `strh` at 0xb6fb4 | store | 2 | `r0 +0x6` | `([r0+6]:2+1 & 0xffff)` |
| `str` at 0xb6fc0 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb6fc4 | load | 4 | `sp -0x18` | `-` |
| `str` at 0xb6fc8 | store | 4 | `sp -0x40` | `[sp-24]` |
| `ldr` at 0xb6fcc | load | 4 | `sp -0x14` | `-` |
| `str` at 0xb6fd0 | store | 4 | `sp -0x3c` | `[sp-20]` |
| `ldr` at 0xb6fd4 | load | 4 | `sp +0x4` | `-` |
| `str` at 0xb6fd8 | store | 4 | `sp -0x38` | `[sp+4]` |
| `ldr` at 0xb6fdc | load | 4 | `sp +0x8` | `-` |
| `str` at 0xb6fe0 | store | 4 | `sp -0x34` | `[sp+8]` |
| `str` at 0xb6fe8 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xb6ff0 | store | 4 | `sp -0x2c` | `0x0` |
| `ldr` at 0xb6ff4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb6ff8 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7014 | store | 4 | `sp -0x24` | `0x0` |
| `str` at 0xb701c | store | 4 | `sp -0x28` | `0x0` |
| `ldr` at 0xb7020 | load | 4 | `sp -0x24` | `-` |
| `ldmdb` at 0xb7028 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7028 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7028 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7000 | `(unknown callee) 0xb4c58` | `sp-36` | `[sp-32]` | `[sp-40]` | `[r0+4]:1` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `beq` at 0xb700c | `ret(0xb4c58) == 0x0` | `ret(0xb4c58) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7028 | `r0` | `[sp-36]` |

## `IRouterReadPacket`

- `IRouterReadPacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb702c to 0xb74a8.
  Established. The indirect call has unknown target, a known continuation, and candidates `[sp-28]`,
  `sp-44`, `[sp-64]`, `sp-44` (`mov` at 0xb70f8); its signed result test takes the later processing
  arm for `ret(unknown) > 0x0` and falls through for less or equal (`bgt` at 0xb710c). Later word
  stores write `[sp-44]` at `[sp-48] +0x0`, `0x0` at `[sp-44] +0x0`, and `[sp-40]` at `[sp-32] +0x0`
  (`str` at 0xb73d0, `str` at 0xb7400, `str` at 0xb745c). A final call to `0xb4a04` carries `0x1`,
  `([[sp-40]+4]:2 & 0xffff)`, `[sp-32]`, `[sp-40]` (`bl` at 0xb7464). Physical receive behavior,
  packet ownership and the joined return remain unknown (`ldmdb` at 0xb7474).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb703c | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb7040 | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb7044 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb7048 | store | 4 | `sp -0x20` | `r3` |
| `str` at 0xb7050 | store | 4 | `sp -0x28` | `0x0` |
| `str` at 0xb7058 | store | 4 | `sp -0x2c` | `0x0` |
| `str` at 0xb7060 | store | 4 | `sp -0x30` | `0x0` |
| `str` at 0xb7068 | store | 4 | `sp -0x3c` | `0x410` |
| `ldr` at 0xb706c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7074 | store | 4 | `sp -0x40` | `0x400` |
| `ldr` at 0xb7078 | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7084 | store | 4 | `sp -0x2c` | `ret(0x1b124)` |
| `ldr` at 0xb7088 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7094 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb70a4 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb70a8 | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb70ac | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb70b0 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb70b4 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb70b8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb70c4 | load | 4 | `abs 0xb7478` | `-` |
| `str` at 0xb70c8 | store | 4 | `sp -0x60` | `0xfffffda4` |
| `ldr` at 0xb70d0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb70e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb70e8 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb70f0 | load | 4 | `sp -0x40` | `-` |
| `str` at 0xb7100 | store | 4 | `sp -0x24` | `ret(unknown)` |
| `ldr` at 0xb7104 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7110 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7114 | store | 4 | `sp -0x64` | `ret(unknown)` |
| `ldr` at 0xb7118 | load | 4 | `sp -0x64` | `-` |
| `ldr` at 0xb7124 | load | 4 | `sp -0x64` | `-` |
| `str` at 0xb7138 | store | 4 | `sp -0x44` | `?` |
| `ldr` at 0xb7140 | load | 4 | `sp -0x24` | `-` |
| `str` at 0xb7144 | store | 4 | `sp -0x44` | `[sp-36]` |
| `ldr` at 0xb714c | load | 4 | `abs 0xb747c` | `-` |
| `str` at 0xb7150 | store | 4 | `sp -0x44` | `0xfffffd9c` |
| `ldr` at 0xb7154 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7160 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7164 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7168 | store | 4 | `sp -0x48` | `[[sp-40]]` |
| `ldr` at 0xb716c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb717c | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7180 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7184 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb7188 | load | 4 | `sp -0x48` | `-` |
| `ldr` at 0xb7190 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb7194 | store | 4 | `sp -0x48` | `[sp-76]` |
| `ldr` at 0xb719c | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb71a4 | store | 4 | `[sp-40] +0x0` | `0x0` |
| `ldr` at 0xb71a8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb71b4 | load | 4 | `sp -0x28` | `-` |
| `ldrb` at 0xb71b8 | load | 1 | `[sp-40] +0x16` | `-` |
| `str` at 0xb71c8 | store | 4 | `sp -0x70` | `0x0` |
| `ldr` at 0xb71cc | load | 4 | `abs 0xb7480` | `-` |
| `ldr` at 0xb71d0 | load | 4 | `sp -0x44` | `-` |
| `ldr` at 0xb71d4 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb71e0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb71ec | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb71f8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7208 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb720c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb7210 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7214 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7218 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb721c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7228 | load | 4 | `sp -0x44` | `-` |
| `str` at 0xb722c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xb7234 | load | 4 | `abs 0xb7484` | `-` |
| `ldr` at 0xb7238 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb723c | load | 4 | `abs 0x149ae0` | `-` |
| `ldr` at 0xb7244 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7250 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb7254 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xb7268 | load | 4 | `abs 0xb7488` | `-` |
| `ldr` at 0xb726c | load | 4 | `abs 0x149ae8` | `-` |
| `ldr` at 0xb7284 | load | 4 | `abs 0xb748c` | `-` |
| `ldr` at 0xb7288 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb7290 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb729c | store | 4 | `sp -0x60` | `?` |
| `ldr` at 0xb72a4 | load | 4 | `abs 0xb7490` | `-` |
| `ldr` at 0xb72a8 | load | 4 | `abs 0x149ae4` | `-` |
| `ldr` at 0xb72b4 | load | 4 | `abs 0xb7494` | `-` |
| `ldr` at 0xb72b8 | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb72bc | load | 4 | `abs 0xb7498` | `-` |
| `ldr` at 0xb72c0 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb72c8 | load | 4 | `abs 0xb7494` | `-` |
| `ldr` at 0xb72cc | load | 4 | `abs 0x144dc8` | `-` |
| `ldr` at 0xb72d4 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb72d8 | store | 4 | `sp -0x28` | `[sp-44]` |
| `ldr` at 0xb72e0 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb72e4 | load | 1 | `[sp-44] +0x15` | `-` |
| `ldr` at 0xb72ec | load | 4 | `sp -0x38` | `-` |
| `str` at 0xb72fc | store | 4 | `sp -0x4c` | `0x0` |
| `ldr` at 0xb7300 | load | 4 | `abs 0xb7488` | `-` |
| `ldr` at 0xb7304 | load | 4 | `abs 0x149ae8` | `-` |
| `ldr` at 0xb7320 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7330 | load | 4 | `abs 0xb7488` | `-` |
| `ldr` at 0xb7334 | load | 4 | `abs 0x149ae8` | `-` |
| `ldr` at 0xb7348 | load | 4 | `sp -0x4c` | `-` |
| `str` at 0xb7354 | store | 4 | `sp -0x4c` | `?` |
| `ldr` at 0xb735c | load | 4 | `abs 0xb749c` | `-` |
| `ldr` at 0xb7370 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7378 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb737c | store | 4 | `sp -0x30` | `[sp-40]` |
| `ldr` at 0xb7380 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7384 | load | 4 | `[sp-40] +0x0` | `-` |
| `str` at 0xb7388 | store | 4 | `sp -0x28` | `[[sp-40]]` |
| `ldr` at 0xb738c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7398 | load | 4 | `abs 0xb7488` | `-` |
| `ldr` at 0xb739c | load | 4 | `abs 0x149ae8` | `-` |
| `ldr` at 0xb73a8 | load | 4 | `abs 0xb74a0` | `-` |
| `ldr` at 0xb73ac | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb73b4 | load | 4 | `sp -0x2c` | `-` |
| `ldr` at 0xb73bc | load | 4 | `abs 0xb74a4` | `-` |
| `str` at 0xb73c0 | store | 4 | `sp -0x60` | `0xfffffd9a` |
| `ldr` at 0xb73c8 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb73cc | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb73d0 | store | 4 | `[sp-48] +0x0` | `[sp-44]` |
| `ldr` at 0xb73d4 | load | 4 | `sp -0x2c` | `-` |
| `ldrh` at 0xb73d8 | load | 2 | `[sp-44] +0x4` | `-` |
| `strh` at 0xb73dc | store | 2 | `sp -0x68` | `([[sp-44]+4]:2 & 0xffff)` |
| `ldrh` at 0xb73e0 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xb73ec | store | 2 | `sp -0x68` | `?` |
| `ldrh` at 0xb73f0 | load | 2 | `sp -0x68` | `-` |
| `strh` at 0xb73f4 | store | 2 | `[sp-44] +0x4` | `([sp-104]:2 & 0xffff)` |
| `ldr` at 0xb73f8 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb7400 | store | 4 | `[sp-44] +0x0` | `0x0` |
| `ldr` at 0xb7404 | load | 4 | `sp -0x2c` | `-` |
| `ldrb` at 0xb7408 | load | 1 | `[sp-44] +0x15` | `-` |
| `str` at 0xb7414 | store | 4 | `sp -0x38` | `?` |
| `ldr` at 0xb7418 | load | 4 | `sp -0x2c` | `-` |
| `str` at 0xb741c | store | 4 | `sp -0x30` | `[sp-44]` |
| `ldr` at 0xb7420 | load | 4 | `sp -0x38` | `-` |
| `ldr` at 0xb742c | load | 4 | `sp -0x28` | `-` |
| `ldrh` at 0xb7430 | load | 2 | `[sp-40] +0x4` | `-` |
| `strh` at 0xb7434 | store | 2 | `sp -0x6c` | `([[sp-40]+4]:2 & 0xffff)` |
| `ldrh` at 0xb7438 | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xb7448 | store | 2 | `sp -0x6c` | `?` |
| `ldrh` at 0xb744c | load | 2 | `sp -0x6c` | `-` |
| `strh` at 0xb7450 | store | 2 | `[sp-40] +0x4` | `([sp-108]:2 & 0xffff)` |
| `ldr` at 0xb7454 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7458 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb745c | store | 4 | `[sp-32] +0x0` | `[sp-40]` |
| `str` at 0xb746c | store | 4 | `sp -0x60` | `0x0` |
| `ldr` at 0xb7470 | load | 4 | `sp -0x60` | `-` |
| `ldmdb` at 0xb7474 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7474 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7474 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb707c | `(unknown callee) 0x1b124` | `0x410` | `?` | `?` | `0x400` |
| `bl` at 0xb70bc | `(unknown callee) 0x1b400` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb70dc | `(unknown callee) 0x1b43c` | `ret(0x1b124)` | `0x0` | `0x2c` | `ret(0x1b124)` |
| `mov` at 0xb70f8 | `(unknown callee) (unknown target)` | `[sp-28]` | `sp-44` | `[sp-64]` | `sp-44` |
| `bl` at 0xb718c | `(unknown callee) 0x1b400` | `?` | `?` | `?` | `?` |
| `bl` at 0xb71dc | `(unknown callee) 0xb6bdc` | `0x2b1ce0` | `?` | `[sp-40]` | `0x0` |
| `bl` at 0xb71f0 | `(unknown callee) 0x1b400` | `[sp-44]` | `?` | `?` | `[sp-44]` |
| `bl` at 0xb7220 | `(unknown callee) 0x1b400` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb7240 | `(unknown callee) 0xb7674` | `[sp-44]` | `[0x149ae0]` | `?` | `0x149ae0` |
| `bl` at 0xb7280 | `(unknown callee) 0xb80fc` | `0x0` | `0x0` | `?` | `[0x149ae8]` |
| `bl` at 0xb728c | `(unknown callee) 0xb80fc` | `0x1282f4` | `[sp-44]` | `?` | `?` |
| `bl` at 0xb7294 | `(unknown callee) 0x1b400` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb72c4 | `(unknown callee) 0x24028` | `[0x144dc8]` | `0x1282fc` | `[sp-44]` | `0x144dc8` |
| `bl` at 0xb72d0 | `(unknown callee) 0x1b01c` | `[0x144dc8]` | `?` | `?` | `0x144dc8` |
| `bl` at 0xb7318 | `(unknown callee) 0xb80fc` | `0x0` | `0x0` | `?` | `[0x149ae8]` |
| `bl` at 0xb7364 | `(unknown callee) 0x1b49c` | `sp-92` | `0x128308` | `?` | `?` |
| `bl` at 0xb7374 | `(unknown callee) 0xb80fc` | `sp-92` | `[sp-40]` | `?` | `sp-92` |
| `bl` at 0xb7390 | `(unknown callee) 0x1b400` | `[sp-40]` | `?` | `?` | `[[sp-40]]` |
| `bl` at 0xb73b0 | `(unknown callee) 0xb80fc` | `0x128310` | `[sp-44]` | `?` | `[0x149ae8]` |
| `bl` at 0xb73b8 | `(unknown callee) 0x1b400` | `[sp-44]` | `?` | `?` | `?` |
| `bl` at 0xb7464 | `(unknown callee) 0xb4a04` | `0x1` | `([[sp-40]+4]:2 & 0xffff)` | `[sp-32]` | `[sp-40]` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb7090 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xb709c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bgt` at 0xb710c | `ret(unknown) > 0x0 (signed, 32-bit)` | `ret(unknown) <= 0x0 (signed, 32-bit)` |
| `beq` at 0xb7120 | `Z == 1` | `not (Z == 1)` |
| `beq` at 0xb712c | `[sp-100] == 0x0` | `[sp-100] != 0x0` |
| `beq` at 0xb715c | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xb7174 | `Z == 0` | `not (Z == 0)` |
| `beq` at 0xb71b0 | `[sp-40] == 0x0` | `[sp-40] != 0x0` |
| `bne` at 0xb71c0 | `[[sp-40]+22]:1 != 0x0` | `[[sp-40]+22]:1 == 0x0` |
| `beq` at 0xb71e8 | `[sp-44] == 0x0` | `[sp-44] != 0x0` |
| `bne` at 0xb7200 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb724c | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `bne` at 0xb7264 | `? != 0x0` | `? == 0x0` |
| `beq` at 0xb7274 | `[0x149ae8] == 0x0` | `[0x149ae8] != 0x0` |
| `beq` at 0xb72b0 | `[0x149ae4] == 0x0` | `[0x149ae4] != 0x0` |
| `beq` at 0xb72f4 | `? == [sp-56]` | `? != [sp-56]` |
| `beq` at 0xb730c | `[0x149ae8] == 0x0` | `[0x149ae8] != 0x0` |
| `bne` at 0xb7328 | `[sp-40] != 0x0` | `[sp-40] == 0x0` |
| `beq` at 0xb733c | `[0x149ae8] == 0x0` | `[0x149ae8] != 0x0` |
| `beq` at 0xb73a4 | `[0x149ae8] == 0x0` | `[0x149ae8] != 0x0` |
| `bne` at 0xb7428 | `[sp-56] != 0x0` | `[sp-56] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7474 | `r0` | `?` |

## `IRouterWritePacket`

- `IRouterWritePacket`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb74a8 to 0xb7674.
  Established. The indirect call has unknown target with `[sp-28]`, `[sp-52]+16`, `[sp-40]`,
  `[sp-52]+16` and a known continuation (`mov` at 0xb75a8); its result comparison takes the
  continuation for `ret(unknown) == [sp-40]` and falls through on inequality (`beq` at 0xb75c0). The
  equality arm calls `0x1b400` with `[sp-52]`, unknown, `ret(unknown)`, `[sp-40]` (`bl` at 0xb763c).
  Another path stores word `0xfffffd92` at `sp -0x3c` (`str` at 0xb761c). External write completion
  and the merged r0 value remain unproved (`ldmdb` at 0xb7664).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb74b8 | store | 4 | `sp -0x14` | `r0` |
| `str` at 0xb74bc | store | 4 | `sp -0x18` | `r1` |
| `str` at 0xb74c0 | store | 4 | `sp -0x1c` | `r2` |
| `str` at 0xb74c4 | store | 4 | `sp -0x20` | `r3` |
| `ldr` at 0xb74c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb74cc | store | 4 | `sp -0x24` | `r3` |
| `ldr` at 0xb74d0 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb74e0 | load | 4 | `sp -0x24` | `-` |
| `ldrh` at 0xb74e4 | load | 2 | `(unknown)` | `-` |
| `str` at 0xb74e8 | store | 4 | `sp -0x28` | `?` |
| `ldr` at 0xb74ec | load | 4 | `sp -0x24` | `-` |
| `ldrb` at 0xb74f0 | load | 1 | `(unknown)` | `-` |
| `str` at 0xb7500 | store | 4 | `sp -0x2c` | `?` |
| `ldr` at 0xb7504 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb7518 | store | 4 | `sp -0x34` | `ret(0x1b124)` |
| `ldr` at 0xb751c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7528 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb752c | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xb753c | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb7540 | load | 4 | `abs 0xb7668` | `-` |
| `ldr` at 0xb7544 | load | 4 | `abs 0xb766c` | `-` |
| `ldr` at 0xb7548 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7554 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb755c | load | 4 | `abs 0xb766c` | `-` |
| `str` at 0xb7560 | store | 4 | `sp -0x38` | `0xfffffda4` |
| `ldr` at 0xb7568 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb7570 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7574 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7580 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb758c | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7594 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7598 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb75a0 | load | 4 | `sp -0x28` | `-` |
| `str` at 0xb75b0 | store | 4 | `sp -0x30` | `ret(unknown)` |
| `ldr` at 0xb75b4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb75b8 | load | 4 | `sp -0x28` | `-` |
| `ldr` at 0xb75c4 | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb75d0 | load | 4 | `sp -0x20` | `-` |
| `ldrb` at 0xb75d4 | load | 1 | `[sp-32] +0x16` | `-` |
| `str` at 0xb75e4 | store | 4 | `sp -0x40` | `0x0` |
| `ldr` at 0xb75e8 | load | 4 | `abs 0xb7668` | `-` |
| `ldr` at 0xb75f0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb75fc | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7604 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb760c | load | 4 | `sp -0x30` | `-` |
| `ldr` at 0xb7618 | load | 4 | `abs 0xb7670` | `-` |
| `str` at 0xb761c | store | 4 | `sp -0x3c` | `0xfffffd92` |
| `str` at 0xb7628 | store | 4 | `sp -0x3c` | `?` |
| `ldr` at 0xb762c | load | 4 | `sp -0x3c` | `-` |
| `str` at 0xb7630 | store | 4 | `sp -0x38` | `[sp-60]` |
| `ldr` at 0xb7638 | load | 4 | `sp -0x34` | `-` |
| `ldr` at 0xb7640 | load | 4 | `sp -0x24` | `-` |
| `ldr` at 0xb7644 | load | 4 | `[sp-36] +0x0` | `-` |
| `str` at 0xb7648 | store | 4 | `sp -0x24` | `[[sp-36]]` |
| `ldr` at 0xb7650 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb765c | store | 4 | `sp -0x38` | `0x0` |
| `ldr` at 0xb7660 | load | 4 | `sp -0x38` | `-` |
| `ldmdb` at 0xb7664 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7664 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7664 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb7510 | `(unknown callee) 0x1b124` | `?` | `?` | `?` | `?` |
| `bl` at 0xb7550 | `(unknown callee) 0xb6bdc` | `0x2b1ce0` | `0xfffffda4` | `[sp-32]` | `0x0` |
| `bl` at 0xb7558 | `(unknown callee) 0xb56f4` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xb757c | `(unknown callee) 0x1b304` | `ret(0x1b124)` | `[sp-36]` | `[sp-40]+16` | `[sp-40]+16` |
| `bl` at 0xb7588 | `(unknown callee) 0xb7674` | `[sp-52]` | `0x1` | `?` | `?` |
| `mov` at 0xb75a8 | `(unknown callee) (unknown target)` | `[sp-28]` | `[sp-52]+16` | `[sp-40]` | `[sp-52]+16` |
| `bl` at 0xb75f8 | `(unknown callee) 0xb6bdc` | `0x2b1ce0` | `?` | `[sp-32]` | `0x0` |
| `bl` at 0xb7600 | `(unknown callee) 0x1b400` | `[sp-52]` | `?` | `?` | `?` |
| `bl` at 0xb7608 | `(unknown callee) 0xb56f4` | `[sp-32]` | `?` | `?` | `?` |
| `bl` at 0xb763c | `(unknown callee) 0x1b400` | `[sp-52]` | `?` | `ret(unknown)` | `[sp-40]` |
| `bl` at 0xb7654 | `(unknown callee) 0xb56f4` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb74d8 | `Z == 0` | `not (Z == 0)` |
| `bne` at 0xb7524 | `ret(0x1b124) != 0x0` | `ret(0x1b124) == 0x0` |
| `bne` at 0xb7534 | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `beq` at 0xb75c0 | `ret(unknown) == [sp-40]` | `ret(unknown) != [sp-40]` |
| `bge` at 0xb75cc | `ret(unknown) >= 0x0 (signed, 32-bit)` | `ret(unknown) < 0x0 (signed, 32-bit)` |
| `bne` at 0xb75dc | `[[sp-32]+22]:1 != 0x0` | `[[sp-32]+22]:1 == 0x0` |
| `bne` at 0xb7614 | `[sp-48] != 0x0` | `[sp-48] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7664 | `r0` | `?` |

## `IRouterSetFrameByteOrder`

- `IRouterSetFrameByteOrder`, `6.5.28/usr/cpu/firmware/sysco/l1.bin`:
  Package `6.5.28:l1#4`, inner executable `/stand/sysco/bin/l1`.
  Analysed span 0xb7674 to 0xb7944.
  Established. The first comparison takes the update path when an unknown value is not equal to
  entry r1 and falls through when equal (`bne` at 0xb76a8). The next branch takes the alternate
  update arm for `r1 != 0x2` and falls through on equality (`bne` at 0xb76b8). The equality arm
  stores halfword `(ret(0x1b208) & 0xffff)` at entry `r0 +0x10`; the alternate arm stores halfword
  `(ret(0x1b424) & 0xffff)` there (`strh` at 0xb76d4, `strh` at 0xb7818). Their later byte stores
  write `([sp-36]:1 & 0xff)` and `([sp-40]:1 & 0xff)` at `[sp-24] +0x6`, respectively (`strb` at
  0xb77f8, `strb` at 0xb793c). Byte-order semantics, indexed layout and the r0 return remain unknown
  (`ldmdb` at 0xb7940).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `str` at 0xb7684 | store | 4 | `sp -0x18` | `r0` |
| `str` at 0xb7688 | store | 4 | `sp -0x1c` | `r1` |
| `ldr` at 0xb768c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7690 | load | 1 | `r0 +0x6` | `-` |
| `ldr` at 0xb76a0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb76b0 | load | 4 | `sp -0x1c` | `-` |
| `ldr` at 0xb76bc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb76c0 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb76c4 | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xb76d4 | store | 2 | `r0 +0x10` | `(ret(0x1b208) & 0xffff)` |
| `ldr` at 0xb76d8 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb76dc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb76e0 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xb76ec | store | 4 | `[sp-24] +0x18` | `ret(0x1af68)` |
| `ldr` at 0xb76f0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb76f4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb76f8 | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xb7704 | store | 4 | `[sp-24] +0x1c` | `ret(0x1af68)` |
| `ldr` at 0xb7708 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb770c | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb7710 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xb7720 | store | 2 | `[sp-24] +0x22` | `(ret(0x1b208) & 0xffff)` |
| `str` at 0xb7728 | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb772c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7730 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb7734 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7744 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7748 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb775c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7760 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb7774 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb7784 | store | 2 | `(unknown)` | `(ret(0x1b208) & 0xffff)` |
| `ldr` at 0xb7788 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb778c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb77a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb77a4 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb77b8 | load | 4 | `(unknown)` | `-` |
| `str` at 0xb77c4 | store | 4 | `(unknown)` | `ret(0x1af68)` |
| `ldr` at 0xb77c8 | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb77d0 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb77d8 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb77dc | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xb77e0 | store | 1 | `sp -0x24` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xb77e4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xb77f0 | store | 1 | `sp -0x24` | `?` |
| `ldrb` at 0xb77f4 | load | 1 | `sp -0x24` | `-` |
| `strb` at 0xb77f8 | store | 1 | `[sp-24] +0x6` | `([sp-36]:1 & 0xff)` |
| `ldr` at 0xb7800 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7804 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb7808 | load | 2 | `r0 +0x10` | `-` |
| `strh` at 0xb7818 | store | 2 | `r0 +0x10` | `(ret(0x1b424) & 0xffff)` |
| `ldr` at 0xb781c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7820 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7824 | load | 4 | `[sp-24] +0x18` | `-` |
| `str` at 0xb7830 | store | 4 | `[sp-24] +0x18` | `ret(0x1b070)` |
| `ldr` at 0xb7834 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7838 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb783c | load | 4 | `[sp-24] +0x1c` | `-` |
| `str` at 0xb7848 | store | 4 | `[sp-24] +0x1c` | `ret(0x1b070)` |
| `ldr` at 0xb784c | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb7850 | load | 4 | `sp -0x18` | `-` |
| `ldrh` at 0xb7854 | load | 2 | `[sp-24] +0x22` | `-` |
| `strh` at 0xb7864 | store | 2 | `[sp-24] +0x22` | `(ret(0x1b424) & 0xffff)` |
| `str` at 0xb786c | store | 4 | `sp -0x20` | `0x0` |
| `ldr` at 0xb7870 | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7874 | load | 1 | `[sp-24] +0x20` | `-` |
| `ldr` at 0xb7878 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb7888 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb788c | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb78a0 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb78a4 | load | 4 | `sp -0x20` | `-` |
| `ldrh` at 0xb78b8 | load | 2 | `(unknown)` | `-` |
| `strh` at 0xb78c8 | store | 2 | `(unknown)` | `(ret(0x1b424) & 0xffff)` |
| `ldr` at 0xb78cc | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb78d0 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb78e4 | load | 4 | `sp -0x18` | `-` |
| `ldr` at 0xb78e8 | load | 4 | `sp -0x20` | `-` |
| `ldr` at 0xb78fc | load | 4 | `(unknown)` | `-` |
| `str` at 0xb7908 | store | 4 | `(unknown)` | `ret(0x1b070)` |
| `ldr` at 0xb790c | load | 4 | `sp -0x20` | `-` |
| `str` at 0xb7914 | store | 4 | `sp -0x20` | `[sp-32]+1` |
| `ldr` at 0xb791c | load | 4 | `sp -0x18` | `-` |
| `ldrb` at 0xb7920 | load | 1 | `[sp-24] +0x6` | `-` |
| `strb` at 0xb7924 | store | 1 | `sp -0x28` | `([[sp-24]+6]:1 & 0xff)` |
| `ldrb` at 0xb7928 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xb7934 | store | 1 | `sp -0x28` | `?` |
| `ldrb` at 0xb7938 | load | 1 | `sp -0x28` | `-` |
| `strb` at 0xb793c | store | 1 | `[sp-24] +0x6` | `([sp-40]:1 & 0xff)` |
| `ldmdb` at 0xb7940 | load | 4 | `sp -0x14` | `-` |
| `ldmdb` at 0xb7940 | load | 4 | `sp -0x10` | `-` |
| `ldmdb` at 0xb7940 | load | 4 | `sp -0xc` | `-` |
| `ldmdb` at 0xb7940 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 |
|---|---|---|---|---|---|
| `bl` at 0xb76cc | `(unknown callee) 0x1b208` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xb76e4 | `(unknown callee) 0x1af68` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb76fc | `(unknown callee) 0x1af68` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb7718 | `(unknown callee) 0x1b208` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xb777c | `(unknown callee) 0x1b208` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb77bc | `(unknown callee) 0x1af68` | `?` | `[sp-24]` | `0x28` | `?` |
| `bl` at 0xb7810 | `(unknown callee) 0x1b424` | `[r0+16]:2` | `r1` | `?` | `[r0+16]:2` |
| `bl` at 0xb7828 | `(unknown callee) 0x1b070` | `[[sp-24]+24]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb7840 | `(unknown callee) 0x1b070` | `[[sp-24]+28]` | `?` | `?` | `[sp-24]` |
| `bl` at 0xb785c | `(unknown callee) 0x1b424` | `[[sp-24]+34]:2` | `?` | `?` | `[[sp-24]+34]:2` |
| `bl` at 0xb78c0 | `(unknown callee) 0x1b424` | `?` | `[sp-24]` | `0x24` | `?` |
| `bl` at 0xb7900 | `(unknown callee) 0x1b070` | `?` | `[sp-24]` | `0x28` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `bne` at 0xb76a8 | `? != r1` | `? == r1` |
| `bne` at 0xb76b8 | `r1 != 0x2` | `r1 == 0x2` |
| `blt` at 0xb773c | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |
| `blt` at 0xb7880 | `lt flags condition (operands unknown)` | `not (lt flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ldmdb` at 0xb7940 | `r0` | `?` |
