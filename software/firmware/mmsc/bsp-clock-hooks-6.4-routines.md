# MMSC clock and initialization hooks in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
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

[Fact sheets and full listings](disasm/bsp-clock-hooks-6.4-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The branch takes `[0x1aa6cc] == 0x0` and falls through for nonzero (`je` at
  0x1084a2). On the fall-through, a four-byte load reads absolute `0x1aa6d0` (`mov` at 0x1084a4)
  before an indirect call with leading candidate `[0x1aa6d0]` (`call` at 0x1084af). That callee and
  its target remain unknown, as does the merged EAX return value (`ret` at 0x1084b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1aa6cc` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1aa6d0` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1aa6cc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1aa6d0]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1aa6cc] == 0x0` | `[0x1aa6cc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. The `_sysHwInit2` call presents leading candidates entry `ebx`, `esi`, and `ebp`
  (`call` at 0x1084c3). It is followed by four-byte stores of `[sp+4]` at absolute `0x1aa6cc` (`mov`
  at 0x1084c8), `[sp+8]` at absolute `0x1aa6d0` (`mov` at 0x1084ce), and `0x1` at absolute
  `0x1aa6d8` (`mov` at 0x1084d4). These stored values have unknown roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1aa6cc` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1aa6d0` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1aa6d8` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b7f8` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. A four-byte comparison reads absolute `0x1aa6d4` (`cmp` at 0x1084fb); its branch
  takes `[0x1aa6d4] == 0x0` and falls through for nonzero (`je` at 0x108502). The fall-through
  includes three `_sysOutByte` calls with leading candidates `0x43`, `0x38` (`call` at 0x108508),
  `0x40`, `0x0` (`call` at 0x108514), and `0x40`, `0x0` (`call` at 0x108520). A subsequent four-byte
  store writes `0x0` to absolute `0x1aa6d4` (`mov` at 0x10852f). The global field and callee effects
  are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1aa6d4` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1aa6d4` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1aa6d4] == 0x0` | `[0x1aa6d4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The branch takes `[0x1aa6d4] != 0x0` and falls through for zero (`jne` at 0x108553).
  The fall-through includes a four-byte divisor read at absolute `0x1aa6dc` (`idiv` at 0x10855b).
  Calling `_sysOutByte` first presents leading candidates `0x43` and `0x36` (`call` at 0x108567),
  while two later calls present `0x40` and unknown (`call` at 0x108575, `call` at 0x108588). A
  four-byte store then sets absolute `0x1aa6d4` to `0x1` (`mov` at 0x108597). The divisor units and
  output effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1aa6d4` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1aa6dc` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1aa6d4` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1aa6d4] != 0x0` | `[0x1aa6d4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. A four-byte load at absolute `0x1aa6dc` supplies the value (`mov` at 0x1085ab)
  returned as EAX `[0x1aa6dc]` (`ret` at 0x1085b3). The field interpretation is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1aa6dc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1aa6dc]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The entry decision takes `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` and falls through
  for greater values (`jbe` at 0x1085c6). The taken path writes four-byte `[sp+4]` at absolute
  `0x1aa6dc` (`mov` at 0x1085d8). A later decision takes `[0x1aa6d4] == 0x0` and falls through for
  nonzero (`je` at 0x1085e5). That fall-through includes `_sysClkDisable` (`call` at 0x1085e7) and
  `_sysClkEnable` (`call` at 0x1085ec), with unknown candidate slots at both calls. The update
  policy beyond these sites is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1aa6dc` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1aa6d4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1aa6d4] == 0x0` | `[0x1aa6d4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. Calls `_sysOutByte` with leading candidates `0x70` and `0xc` (`call` at 0x1085ff),
  then `_sysInByte` with leading candidate `0x71` (`call` at 0x108609). The later branch takes
  `[0x1aa6e0] == 0x0` and falls through for nonzero (`je` at 0x108618); its fall-through includes an
  unknown callee and target with leading candidate `[0x1aa6e4]` (`call` at 0x108625). Hardware
  effects of these callees are not established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1aa6e0` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1aa6e4` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1aa6e0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1aa6e4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1aa6e0] == 0x0` | `[0x1aa6e0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. Three four-byte stores put `[sp+4]` at absolute `0x1aa6e0` (`mov` at 0x108641),
  `[sp+8]` at absolute `0x1aa6e4` (`mov` at 0x108646), and `0x1` at absolute `0x1aa6ec` (`mov` at
  0x10864c). EAX is `0x0` at the return (`ret` at 0x10865b). The stored-value roles are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1aa6e0` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1aa6e4` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1aa6ec` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The comparison branch takes `[0x1aa6e8] == 0x0` and falls through for nonzero (`je`
  at 0x108672). On the fall-through, `_sysOutByte` presents leading candidates `0x70` and `0xb`
  (`call` at 0x108678), followed by `0x71` and `0x2` (`call` at 0x108684). `_sysIntDisablePIC` then
  has leading candidate `0x8` (`call` at 0x10868e). The subsequent four-byte store writes `0x0` at
  absolute `0x1aa6e8` (`mov` at 0x108693). The peripheral meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1aa6e8` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1aa6e8` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1aa6e8] == 0x0` | `[0x1aa6e8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. An initial branch takes `[0x1aa6e8] != 0x0` and falls through for zero (`jne` at
  0x1086b5). Later routes include `_sysOutByte` with leading candidates `0x70`, `0xa` (`call` at
  0x1086d6) and `_sysInByte` with leading candidate `0x71` (`call` at 0x1086e0). Further calls to
  `_sysOutByte` begin `0x70`, `0xb` (`call` at 0x108724) and `0x71`, `0x42` (`call` at 0x108730). A
  four-byte store writes `0x1` at absolute `0x1aa6e8` (`mov` at 0x10873f). The indexed values and
  callee effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1aa6e8` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1aa6f0` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1aa6e8` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1aa6e8] != 0x0` | `[0x1aa6e8] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. Loads four bytes at absolute `0x1aa6f0` (`mov` at 0x10875b), then returns that
  `[0x1aa6f0]` value in EAX (`ret` at 0x108763); its units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1aa6f0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1aa6f0]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The unsigned range branch takes `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` and falls
  through for lesser or equal values (`ja` at 0x108778). On a later path, a four-byte store writes
  `[sp+4]` at absolute `0x1aa6f0` (`mov` at 0x1087a8). The following branch takes `[0x1aa6e8] ==
  0x0` and falls through for nonzero (`je` at 0x1087b5); its fall-through calls `_sysAuxClkDisable`
  (`call` at 0x1087b7) then `_sysAuxClkEnable` (`call` at 0x1087bc), both with unknown candidates.
  The accepted table entries remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1aa6f0` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1aa6e8` | `-` |

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
| `je` at 0x1087b5 | `[0x1aa6e8] == 0x0` | `[0x1aa6e8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7f8 to 0x10b858.
  Established. The first two `_intConnect` calls present leading candidates `0x100`, `0x108498`,
  `0x0` (`call` at 0x10b807) and `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b81b). Calling
  `_sysSerialHwInit2` follows with unknown candidate values (`call` at 0x10b823). Two further
  `_intConnect` sites begin `0x138`, `0x10bb68`, `0x0` (`call` at 0x10b834) and `0x108`, `0x10ae88`,
  `0x0` (`call` at 0x10b848). The return carries EAX `ret(_intConnect)` (`ret` at 0x10b850). The
  connected handlers and interrupt semantics are not established from their numeric candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b807 | `_intConnect 0x174a88` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b81b | `_intConnect 0x174a88` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b823 | `_sysSerialHwInit2 0x10b718` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b834 | `_intConnect 0x174a88` | `0x138` | `0x10bb68` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b848 | `_intConnect 0x174a88` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b850 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7d8 to 0x10b7f8.
  Established. The `_sysIntInitPIC` call has leading candidate entry `ebp` (`call` at 0x10b7db). A
  four-byte store writes `0x108398` at absolute `0x1d1464` (`mov` at 0x10b7e0). The subsequent
  `_sysSerialHwInit` call has unknown candidates (`call` at 0x10b7ea), and its result reaches the
  return as EAX `ret(_sysSerialHwInit)` (`ret` at 0x10b7f2). The stored address role remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b7e0 | store | 4 | `abs 0x1d1464` | `0x108398` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b7db | `_sysIntInitPIC 0x1082f8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b7ea | `_sysSerialHwInit 0x10b618` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7f2 | `eax` | `ret(_sysSerialHwInit)` |

## `_sysReboot`

- `_sysReboot`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. The descriptor load accesses six bytes at absolute `0x0` (`lgdt` at 0x108235). EAX is
  statically `0x0` at the following return (`ret` at 0x108238). Runtime descriptor contents and
  privilege state, including whether the load faults, remain unknown.

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
