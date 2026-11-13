# MMSC clock and initialization hooks in IRIX 6.5.5

Read from `6.5.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.5).
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred exclusive end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
The known-edge CFGs have no unresolved jump entries. Indirect call targets remain unknown.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-clock-hooks-6.5.5-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The four-byte comparison reads absolute `0x1afe34` (`cmp` at 0x10849b). The branch
  takes `[0x1afe34] == 0x0` and falls through for nonzero (`je` at 0x1084a2); its fall-through call
  presents leading candidate `[0x1afe38]`, with unknown callee and target (`call` at 0x1084af). EAX
  remains unknown at the merged return (`ret` at 0x1084b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1afe34` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1afe38` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1afe34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1afe38]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1afe34] == 0x0` | `[0x1afe34] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. Calling `_sysHwInit2` has leading candidates entry `ebx`, `esi`, and `ebp` (`call` at
  0x1084c3). Four-byte stores afterward put `[sp+4]` at absolute `0x1afe34` (`mov` at 0x1084c8),
  `[sp+8]` at absolute `0x1afe38` (`mov` at 0x1084ce), and `0x1` at absolute `0x1afe40` (`mov` at
  0x1084d4). The roles of these globals remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1afe34` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1afe38` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1afe40` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The entry decision takes `[0x1afe3c] == 0x0` and falls through for nonzero (`je` at
  0x108502). Three `_sysOutByte` calls on the fall-through begin `0x43`, `0x38` (`call` at
  0x108508), `0x40`, `0x0` (`call` at 0x108514), and `0x40`, `0x0` (`call` at 0x108520). A later
  four-byte store writes `0x0` at absolute `0x1afe3c` (`mov` at 0x10852f). The peripheral effects
  are unknown from the local call sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1afe3c` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1afe3c` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1afe3c] == 0x0` | `[0x1afe3c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The initial branch takes `[0x1afe3c] != 0x0` and falls through for zero (`jne` at
  0x108553). The fall-through reads a four-byte divisor at absolute `0x1afe44` (`idiv` at 0x10855b).
  It calls `_sysOutByte` with leading candidates `0x43`, `0x36` (`call` at 0x108567). Two later
  calls begin `0x40`, unknown (`call` at 0x108575, `call` at 0x108588). A four-byte store eventually
  sets absolute `0x1afe3c` to `0x1` (`mov` at 0x108597). The computed output and its units remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1afe3c` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1afe44` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1afe3c` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1afe3c] != 0x0` | `[0x1afe3c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. A four-byte load supplies the value at absolute `0x1afe44` (`mov` at 0x1085ab). EAX
  returns `[0x1afe44]` (`ret` at 0x1085b3). The field semantics are not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1afe44` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1afe44]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The range branch takes `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` and falls through for
  greater values (`jbe` at 0x1085c6). The taken route stores four-byte `[sp+4]` at absolute
  `0x1afe44` (`mov` at 0x1085d8). The subsequent branch takes `[0x1afe3c] == 0x0` and falls through
  for nonzero (`je` at 0x1085e5). That fall-through calls `_sysClkDisable` (`call` at 0x1085e7) then
  `_sysClkEnable` (`call` at 0x1085ec) with unknown candidates. The accepted field and rate policy
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1afe44` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1afe3c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1afe3c] == 0x0` | `[0x1afe3c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The `_sysOutByte` candidates begin `0x70` and `0xc` (`call` at 0x1085ff), followed by
  `_sysInByte` with leading candidate `0x71` (`call` at 0x108609). The later branch takes
  `[0x1afe48] == 0x0` and falls through for nonzero (`je` at 0x108618); its fall-through calls an
  unknown target with leading candidate `[0x1afe4c]` (`call` at 0x108625). The indirect callee identity is
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1afe48` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1afe4c` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1afe48` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1afe4c]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1afe48] == 0x0` | `[0x1afe48] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. The four-byte stores save `[sp+4]` at absolute `0x1afe48` (`mov` at 0x108641),
  `[sp+8]` at absolute `0x1afe4c` (`mov` at 0x108646), and `0x1` at absolute `0x1afe54` (`mov` at
  0x10864c). The static EAX return is `0x0` (`ret` at 0x10865b). These saved-value roles remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1afe48` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1afe4c` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1afe54` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The branch takes `[0x1afe50] == 0x0` and falls through for nonzero (`je` at
  0x108672); its fall-through includes `_sysOutByte` with leading pairs `0x70`, `0xb` (`call` at
  0x108678) and `0x71`, `0x2` (`call` at 0x108684). After `_sysIntDisablePIC` with leading candidate
  `0x8` (`call` at 0x10868e), a four-byte store writes `0x0` at absolute `0x1afe50` (`mov` at
  0x108693). The call effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1afe50` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1afe50` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1afe50] == 0x0` | `[0x1afe50] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The first branch takes `[0x1afe50] != 0x0` and falls through for zero (`jne` at
  0x1086b5). Later paths include `_sysOutByte` with leading candidates `0x70`, `0xb` (`call` at
  0x108724), then `0x71`, `0x42` (`call` at 0x108730). The `_sysIntEnablePIC` site begins `0x8`
  (`call` at 0x10873a), followed by a four-byte store of `0x1` at absolute `0x1afe50` (`mov` at
  0x10873f). The indexed state and callee behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1afe50` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1afe58` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1afe50` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1afe50] != 0x0` | `[0x1afe50] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. The four-byte load reads absolute `0x1afe58` (`mov` at 0x10875b). The corresponding
  EAX return is `[0x1afe58]` (`ret` at 0x108763). No units are established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1afe58` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1afe58]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The unsigned comparison takes `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` and falls
  through for lesser or equal values (`ja` at 0x108778). A later route writes four-byte `[sp+4]` at
  absolute `0x1afe58` (`mov` at 0x1087a8). The next branch takes `[0x1afe50] == 0x0` and falls
  through for nonzero (`je` at 0x1087b5). That fall-through calls `_sysAuxClkDisable` (`call` at
  0x1087b7) and `_sysAuxClkEnable` (`call` at 0x1087bc), both with unknown candidates. The indexed
  match semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1afe58` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1afe50` | `-` |

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
| `je` at 0x1087b5 | `[0x1afe50] == 0x0` | `[0x1afe50] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. The `_intConnect` call candidates start with `0x100`, `0x108498`, `0x0` (`call` at
  0x10b867) and `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b) at the first two sites.
  `_sysSerialHwInit2` then presents unknown candidates (`call` at 0x10b883). The last two
  `_intConnect` sites begin `0x138`, `0x10bbc8`, `0x0` (`call` at 0x10b894) and `0x108`, `0x10ae88`,
  `0x0` (`call` at 0x10b8a8). EAX returns the last `ret(_intConnect)` (`ret` at 0x10b8b0). Interrupt
  meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x17a1f0` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x17a1f0` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x17a1f0` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x17a1f0` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. The `_sysIntInitPIC` call presents leading candidate entry `ebp` (`call` at
  0x10b83b). A four-byte store puts `0x108398` at absolute `0x1d6c40` (`mov` at 0x10b840).
  `_sysSerialHwInit` follows with unknown candidates (`call` at 0x10b84a), and EAX returns
  `ret(_sysSerialHwInit)` (`ret` at 0x10b852). The pointer role is unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d6c40` | `0x108398` |

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

- `_sysReboot`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. The `lgdt` access loads six bytes from absolute `0x0` (`lgdt` at 0x108235). A static
  known edge reaches a return carrying EAX `0x0` (`ret` at 0x108238). Runtime faults or successful
  return depend on unknown descriptor contents and privilege state.

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
