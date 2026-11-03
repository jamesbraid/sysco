# MMSC clock and initialization hooks in IRIX 6.5.2

Read from `6.5.2/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.2).
Image SHA-256: `0a88bb7402fcda7f099ff2d3e5d2e6be9abaf99866c79c1f1be9b943274111b8`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
The known-edge CFGs have no unresolved jump entries. Indirect call targets remain unknown.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-clock-hooks-6.5.2-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. Reads four bytes at absolute `0x1ac0dc` for the comparison (`cmp` at 0x10849b); its
  decision takes `[0x1ac0dc] == 0x0` and falls through for nonzero (`je` at 0x1084a2). The
  fall-through call has unknown callee and target, with leading candidate `[0x1ac0e0]` (`call` at
  0x1084af). The indirect destination and its behavior remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1ac0dc` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1ac0e0` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1ac0dc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1ac0e0]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1ac0dc] == 0x0` | `[0x1ac0dc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. Following `_sysHwInit2` with leading candidates entry `ebx`, `esi`, and `ebp` (`call`
  at 0x1084c3), four-byte stores save `[sp+4]` at absolute `0x1ac0dc` (`mov` at 0x1084c8) and
  `[sp+8]` at absolute `0x1ac0e0` (`mov` at 0x1084ce). A third four-byte store writes `0x1` to
  absolute `0x1ac0e8` (`mov` at 0x1084d4). These three fields have unknown meanings.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1ac0dc` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1ac0e0` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1ac0e8` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The entry test takes `[0x1ac0e4] == 0x0` and falls through for nonzero (`je` at
  0x108502). The fall-through includes `_sysOutByte` with leading candidates `0x43`, `0x38` (`call`
  at 0x108508), then two sites beginning `0x40`, `0x0` (`call` at 0x108514, `call` at 0x108520).
  After `_sysIntDisablePIC` with leading candidate `0x0` (`call` at 0x10852a), a four-byte store
  clears absolute `0x1ac0e4` to `0x0` (`mov` at 0x10852f). The callee effects and saved flag
  interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1ac0e4` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1ac0e4` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1ac0e4] == 0x0` | `[0x1ac0e4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. A branch takes `[0x1ac0e4] != 0x0` and falls through for zero (`jne` at 0x108553). On
  the fall-through, `_sysOutByte` presents leading candidates `0x43` and `0x36` (`call` at
  0x108567). Two later `_sysOutByte` sites present `0x40` and unknown (`call` at 0x108575, `call` at
  0x108588). The path later sets absolute `0x1ac0e4` to four-byte `0x1` (`mov` at 0x108597). The
  unresolved output values have no established timing interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1ac0e4` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1ac0ec` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1ac0e4` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1ac0e4] != 0x0` | `[0x1ac0e4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. The four-byte read at absolute `0x1ac0ec` (`mov` at 0x1085ab) reaches the return as
  EAX `[0x1ac0ec]` (`ret` at 0x1085b3). The code does not establish the field units.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1ac0ec` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1ac0ec]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The accepted branch takes `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` and falls through
  for greater values (`jbe` at 0x1085c6); its taken route stores four-byte `[sp+4]` at absolute
  `0x1ac0ec` (`mov` at 0x1085d8). A later decision takes `[0x1ac0e4] == 0x0` and falls through for
  nonzero (`je` at 0x1085e5). The latter route calls `_sysClkDisable` (`call` at 0x1085e7), then
  `_sysClkEnable` (`call` at 0x1085ec), with unknown candidates. The rate contract remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1ac0ec` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1ac0e4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1ac0e4] == 0x0` | `[0x1ac0e4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. Calling `_sysOutByte` exposes leading candidates `0x70`, `0xc` (`call` at 0x1085ff),
  followed by `_sysInByte` with leading candidate `0x71` (`call` at 0x108609). A later decision
  takes `[0x1ac0f0] == 0x0` and falls through for nonzero (`je` at 0x108618). That fall-through
  calls an unknown target with leading candidate `[0x1ac0f4]` (`call` at 0x108625). The input/output
  purposes and callback identity remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1ac0f0` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1ac0f4` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1ac0f0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1ac0f4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1ac0f0] == 0x0` | `[0x1ac0f0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. Saves four-byte `[sp+4]` at absolute `0x1ac0f0` (`mov` at 0x108641) and `[sp+8]` at
  absolute `0x1ac0f4` (`mov` at 0x108646). The subsequent four-byte store puts `0x1` at absolute
  `0x1ac0fc` (`mov` at 0x10864c). The stored roles are unknown, while EAX is `0x0` at the return
  (`ret` at 0x10865b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1ac0f0` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1ac0f4` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1ac0fc` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The branch skips its call sequence when taken for `[0x1ac0f8] == 0x0` and falls
  through for nonzero (`je` at 0x108672). The fall-through calls `_sysOutByte` with leading
  candidates `0x70`, `0xb` (`call` at 0x108678), then `0x71`, `0x2` (`call` at 0x108684). A later
  four-byte store writes `0x0` at absolute `0x1ac0f8` (`mov` at 0x108693). The physical result of
  the calls remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1ac0f8` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1ac0f8` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1ac0f8] == 0x0` | `[0x1ac0f8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The initial branch takes `[0x1ac0f8] != 0x0` and falls through for zero (`jne` at
  0x1086b5); its fall-through includes an indexed four-byte load with unknown effective target
  (`mov` at 0x1086c4). Later paths include `_sysOutByte` with leading candidates `0x70`, `0xb`
  (`call` at 0x108724) and `0x71`, `0x42` (`call` at 0x108730). `_sysIntEnablePIC` then presents
  leading candidate `0x8` (`call` at 0x10873a), before a four-byte store sets absolute `0x1ac0f8` to
  `0x1` (`mov` at 0x10873f). The indexed values and callee effects stay unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1ac0f8` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1ac100` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1ac0f8` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1ac0f8] != 0x0` | `[0x1ac0f8] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. EAX receives a four-byte load from absolute `0x1ac100` (`mov` at 0x10875b). It
  returns `[0x1ac100]` (`ret` at 0x108763). The loaded value has no established auxiliary-clock
  units.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1ac100` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1ac100]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The range decision takes `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` and falls through for
  lesser or equal values (`ja` at 0x108778). A later route writes four-byte `[sp+4]` at absolute
  `0x1ac100` (`mov` at 0x1087a8). The next branch takes `[0x1ac0f8] == 0x0` and falls through for
  nonzero (`je` at 0x1087b5). The fall-through calls `_sysAuxClkDisable` (`call` at 0x1087b7) and
  `_sysAuxClkEnable` (`call` at 0x1087bc), each with unknown candidates. The indexed match criteria
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1ac100` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1ac0f8` | `-` |

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
| `je` at 0x1087b5 | `[0x1ac0f8] == 0x0` | `[0x1ac0f8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. Four `_intConnect` calls present leading triples `0x100`, `0x108498`, `0x0` (`call`
  at 0x10b867), `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b), `0x138`, `0x10bbc8`, `0x0` (`call`
  at 0x10b894), and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b8a8). Between the second and third
  sites, `_sysSerialHwInit2` has unknown candidates (`call` at 0x10b883). EAX returns the last
  `ret(_intConnect)` (`ret` at 0x10b8b0). The interrupt-vector semantics remain unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x176498` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x176498` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x176498` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x176498` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. Calls `_sysIntInitPIC` with leading candidate entry `ebp` (`call` at 0x10b83b), then
  stores four-byte `0x108398` at absolute `0x1d2e30` (`mov` at 0x10b840). `_sysSerialHwInit` follows
  with unknown candidates (`call` at 0x10b84a); its result is the EAX return value
  `ret(_sysSerialHwInit)` (`ret` at 0x10b852). The stored address role remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d2e30` | `0x108398` |

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

- `_sysReboot`, `6.5.2/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. A six-byte descriptor load addresses absolute `0x0` (`lgdt` at 0x108235). The next
  decoded return has EAX `0x0` (`ret` at 0x108238). Whether that return executes remains unknown
  because the descriptor contents, runtime privilege and fault state are not supplied.

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
