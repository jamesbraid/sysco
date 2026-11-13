# MMSC clock and initialization hooks in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include padding or literals after returns. Statements below concern sites reached by known control-flow edges.
The indirect call targets remain unknown.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-clock-hooks-6.5-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. A four-byte comparison reads absolute `0x1ae7a4` (`cmp` at 0x10849b). The branch
  takes `[0x1ae7a4] == 0x0` and falls through for nonzero (`je` at 0x1084a2); its fall-through
  includes an indirect call with unknown callee and target, presenting leading candidate
  `[0x1ae7a8]` (`call` at 0x1084af). The callback identity and interrupt policy are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1ae7a4` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1ae7a8` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1ae7a4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1ae7a8]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1ae7a4] == 0x0` | `[0x1ae7a4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. Calls `_sysHwInit2` with leading candidates entry `ebx`, `esi`, and `ebp` (`call` at
  0x1084c3). Four-byte stores put `[sp+4]` at absolute `0x1ae7a4` (`mov` at 0x1084c8), `[sp+8]` at
  absolute `0x1ae7a8` (`mov` at 0x1084ce), and `0x1` at absolute `0x1ae7b0` (`mov` at 0x1084d4). EAX
  reaches the return as `0x0` (`ret` at 0x1084e8). The global-state meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1ae7a4` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1ae7a8` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1ae7b0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The branch takes `[0x1ae7ac] == 0x0` and falls through for nonzero (`je` at
  0x108502). The fall-through calls `_sysOutByte` with leading candidates `0x43`, `0x38` (`call` at
  0x108508), then `0x40`, `0x0` (`call` at 0x108514), and again `0x40`, `0x0` (`call` at 0x108520).
  It calls `_sysIntDisablePIC` with leading candidate `0x0` (`call` at 0x10852a) before storing
  four-byte `0x0` at absolute `0x1ae7ac` (`mov` at 0x10852f). The concrete timer behavior is unknown
  beyond these sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1ae7ac` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1ae7ac` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1ae7ac] == 0x0` | `[0x1ae7ac] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The initial branch takes `[0x1ae7ac] != 0x0` and falls through for zero (`jne` at
  0x108553); its fall-through calls `_sysOutByte` with leading candidates `0x43` and `0x36` (`call`
  at 0x108567). Two later `_sysOutByte` calls both present `0x40` and unknown as leading candidates
  (`call` at 0x108575, `call` at 0x108588). `_sysIntEnablePIC` then presents leading candidate `0x0`
  (`call` at 0x108592), followed by a four-byte store of `0x1` at absolute `0x1ae7ac` (`mov` at
  0x108597). The computed timer value and units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1ae7ac` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1ae7b4` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1ae7ac` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1ae7ac] != 0x0` | `[0x1ae7ac] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. Loads four bytes from absolute `0x1ae7b4` (`mov` at 0x1085ab). The return carries
  `[0x1ae7b4]` in EAX (`ret` at 0x1085b3). The loaded field has no established units here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1ae7b4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1ae7b4]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The unsigned entry branch takes `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` and falls
  through for greater values (`jbe` at 0x1085c6); its taken route stores four-byte `[sp+4]` at
  absolute `0x1ae7b4` (`mov` at 0x1085d8). A later branch takes `[0x1ae7ac] == 0x0` and falls
  through for nonzero (`je` at 0x1085e5). The fall-through calls `_sysClkDisable` (`call` at
  0x1085e7) followed by `_sysClkEnable` (`call` at 0x1085ec), both with unknown candidates. The
  accepted field interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1ae7b4` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1ae7ac` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1ae7ac] == 0x0` | `[0x1ae7ac] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The first `_sysOutByte` call has leading candidates `0x70` and `0xc` (`call` at
  0x1085ff). `_sysInByte` then presents leading candidate `0x71` (`call` at 0x108609). The branch
  takes `[0x1ae7b8] == 0x0` and falls through for nonzero (`je` at 0x108618); its fall-through calls
  an unknown target with leading candidate `[0x1ae7bc]` (`call` at 0x108625). The callee behavior
  and indirect-call identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1ae7b8` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1ae7bc` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1ae7b8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1ae7bc]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1ae7b8] == 0x0` | `[0x1ae7b8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. Four-byte stores save `[sp+4]` at absolute `0x1ae7b8` (`mov` at 0x108641) and
  `[sp+8]` at absolute `0x1ae7bc` (`mov` at 0x108646). Another four-byte store sets absolute
  `0x1ae7c4` to `0x1` (`mov` at 0x10864c). The return carries EAX `0x0` (`ret` at 0x10865b). The
  stored-value roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1ae7b8` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1ae7bc` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1ae7c4` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The branch takes `[0x1ae7c0] == 0x0` and falls through for nonzero (`je` at
  0x108672). The fall-through calls `_sysOutByte` with leading candidates `0x70`, `0xb` (`call` at
  0x108678), then `0x71`, `0x2` (`call` at 0x108684). Calling `_sysIntDisablePIC` exposes leading
  candidate `0x8` (`call` at 0x10868e). A four-byte store then puts `0x0` at absolute `0x1ae7c0`
  (`mov` at 0x108693). The auxiliary-clock hardware interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1ae7c0` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1ae7c0` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1ae7c0] == 0x0` | `[0x1ae7c0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The first branch takes `[0x1ae7c0] != 0x0` and falls through for zero (`jne` at
  0x1086b5); its fall-through includes a four-byte load with unknown effective target (`mov` at
  0x1086c4). Later routes include `_sysOutByte` with leading candidates `0x70`, `0xb` (`call` at
  0x108724) and `0x71`, `0x42` (`call` at 0x108730). `_sysIntEnablePIC` presents leading candidate
  `0x8` (`call` at 0x10873a), followed by a four-byte store of `0x1` at absolute `0x1ae7c0` (`mov`
  at 0x10873f). The table-selected timing and register semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1ae7c0` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1ae7c8` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1ae7c0` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1086d6 | `_sysOutByte 0x1081d0` | `0x70` | `0xa` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x1086e0 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1086f1 | `_sysOutByte 0x1081d0` | `0x70` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x108703 | `_sysOutByte 0x1081d0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108724 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x108730 | `_sysOutByte 0x1081d0` | `0x71` | `0x42` | `?` | `?` | `?` | `?` |
| `call` at 0x10873a | `_sysIntEnablePIC 0x108428` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1086b5 | `[0x1ae7c0] != 0x0` | `[0x1ae7c0] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. Reads four bytes at absolute `0x1ae7c8` (`mov` at 0x10875b). EAX returns `[0x1ae7c8]`
  (`ret` at 0x108763). The auxiliary rate units cannot be established from this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1ae7c8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1ae7c8]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The range branch takes `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` and falls through for
  the complementary comparison (`ja` at 0x108778). A later route stores four-byte `[sp+4]` at
  absolute `0x1ae7c8` (`mov` at 0x1087a8). The next branch takes `[0x1ae7c0] == 0x0` and falls
  through for nonzero (`je` at 0x1087b5); its fall-through calls `_sysAuxClkDisable` (`call` at
  0x1087b7) and `_sysAuxClkEnable` (`call` at 0x1087bc) with unknown candidates. The table-match
  policy remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1ae7c8` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1ae7c0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1087b7 | `_sysAuxClkDisable 0x108668` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1087bc | `_sysAuxClkEnable 0x1086a8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x108778 | `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` | `[sp+4]-2 <= 0x1ffe (unsigned, 32-bit)` |
| `je` at 0x108782 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x10878a | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |
| `jne` at 0x10878e | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `je` at 0x1087b5 | `[0x1ae7c0] == 0x0` | `[0x1ae7c0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. Two `_intConnect` sites have leading candidates `0x100`, `0x108498`, `0x0` (`call` at
  0x10b867) and `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b). `_sysSerialHwInit2` follows with
  unknown candidates (`call` at 0x10b883). The remaining `_intConnect` sites begin `0x138`,
  `0x10bbc8`, `0x0` (`call` at 0x10b894) and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b8a8). EAX
  returns `ret(_intConnect)` (`ret` at 0x10b8b0). The interrupt-vector interpretation remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x178b60` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x178b60` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x178b60` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x178b60` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. Calling `_sysIntInitPIC` presents leading candidate entry `ebp` (`call` at 0x10b83b).
  A four-byte store puts `0x108398` at absolute `0x1d5568` (`mov` at 0x10b840). `_sysSerialHwInit`
  follows with unknown candidates (`call` at 0x10b84a). EAX returns `ret(_sysSerialHwInit)` (`ret`
  at 0x10b852). The stored pointer role and wider initialization contract are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d5568` | `0x108398` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b83b | `_sysIntInitPIC 0x1082f8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b84a | `_sysSerialHwInit 0x10b618` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b852 | `eax` | `ret(_sysSerialHwInit)` |

## `_sysReboot`

- `_sysReboot`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. The `lgdt` site has a six-byte load at absolute `0x0` (`lgdt` at 0x108235). The
  following return carries EAX `0x0` (`ret` at 0x108238). Whether execution returns after that
  descriptor load, or instead faults, is unknown without the runtime descriptor and privilege state.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `lgdt` at 0x108235 | load | 6 | `abs 0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108238 | `eax` | `0x0` |
