# MMSC clock and initialization hooks in IRIX 6.5.6

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
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

[Fact sheets and full listings](disasm/bsp-clock-hooks-6.5.6-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The comparison uses a four-byte read at absolute `0x1afe44` (`cmp` at 0x10849b); its
  branch takes `[0x1afe44] == 0x0` and falls through for nonzero (`je` at 0x1084a2). The
  fall-through includes an unknown callee and target, with leading candidate `[0x1afe48]` (`call` at
  0x1084af). The return EAX value is unknown (`ret` at 0x1084b4).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1afe44` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1afe48` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1afe44` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1afe48]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1afe44] == 0x0` | `[0x1afe44] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. The `_sysHwInit2` call has leading candidates entry `ebx`, `esi`, and `ebp` (`call`
  at 0x1084c3). Four-byte stores then write `[sp+4]` at absolute `0x1afe44` (`mov` at 0x1084c8),
  `[sp+8]` at absolute `0x1afe48` (`mov` at 0x1084ce), and `0x1` at absolute `0x1afe50` (`mov` at
  0x1084d4). The stored state meanings are unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1afe44` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1afe48` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1afe50` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The entry decision takes `[0x1afe4c] == 0x0` and falls through for nonzero (`je` at
  0x108502); its fall-through `_sysOutByte` calls have leading pairs `0x43`, `0x38` (`call` at
  0x108508), `0x40`, `0x0` (`call` at 0x108514), and `0x40`, `0x0` (`call` at 0x108520). A
  subsequent four-byte store puts `0x0` at absolute `0x1afe4c` (`mov` at 0x10852f). No timer
  interpretation is established from those call candidates.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1afe4c` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1afe4c` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1afe4c] == 0x0` | `[0x1afe4c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The branch takes `[0x1afe4c] != 0x0` and falls through for zero (`jne` at 0x108553).
  Along the fall-through, `_sysOutByte` first presents leading candidates `0x43` and `0x36` (`call`
  at 0x108567). The next two sites each begin `0x40` and unknown (`call` at 0x108575, `call` at
  0x108588). A later four-byte store sets absolute `0x1afe4c` to `0x1` (`mov` at 0x108597). The
  unknown values and callee effects remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1afe4c` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1afe54` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1afe4c` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1afe4c] != 0x0` | `[0x1afe4c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. The value comes from a four-byte read at absolute `0x1afe54` (`mov` at 0x1085ab). EAX
  carries `[0x1afe54]` at the return (`ret` at 0x1085b3). This body establishes no units for it.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1afe54` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1afe54]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The range decision takes `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` and falls through
  for greater values (`jbe` at 0x1085c6); its taken path stores four-byte `[sp+4]` at absolute
  `0x1afe54` (`mov` at 0x1085d8). The next branch takes `[0x1afe4c] == 0x0` and falls through for
  nonzero (`je` at 0x1085e5). That fall-through includes `_sysClkDisable` (`call` at 0x1085e7) and
  `_sysClkEnable` (`call` at 0x1085ec), with unknown candidates at both sites. The rate contract is
  not established here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1afe54` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1afe4c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1afe4c] == 0x0` | `[0x1afe4c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The first call presents `_sysOutByte` leading candidates `0x70` and `0xc` (`call` at
  0x1085ff). The next presents `_sysInByte` leading candidate `0x71` (`call` at 0x108609). A later
  branch takes `[0x1afe58] == 0x0` and falls through for nonzero (`je` at 0x108618); its
  fall-through includes an unknown target and callee with leading candidate `[0x1afe5c]` (`call` at
  0x108625). The call purposes remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1afe58` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1afe5c` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1afe58` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1afe5c]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1afe58] == 0x0` | `[0x1afe58] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. The stores write four-byte `[sp+4]` to absolute `0x1afe58` (`mov` at 0x108641) and
  `[sp+8]` to absolute `0x1afe5c` (`mov` at 0x108646). A further four-byte store writes `0x1` at
  absolute `0x1afe64` (`mov` at 0x10864c). EAX is statically `0x0` at the return (`ret` at
  0x10865b), while the field roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1afe58` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1afe5c` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1afe64` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The branch takes `[0x1afe60] == 0x0` and falls through for nonzero (`je` at
  0x108672). The fall-through has `_sysOutByte` leading pairs `0x70`, `0xb` (`call` at 0x108678) and
  `0x71`, `0x2` (`call` at 0x108684). `_sysIntDisablePIC` follows with leading candidate `0x8`
  (`call` at 0x10868e), before a four-byte store puts `0x0` at absolute `0x1afe60` (`mov` at
  0x108693). The peripheral outcome is unknown from these local sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1afe60` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1afe60` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1afe60] == 0x0` | `[0x1afe60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The entry branch takes `[0x1afe60] != 0x0` and falls through for zero (`jne` at
  0x1086b5). Later routes include `_sysOutByte` leading pairs `0x70`, `0xa` (`call` at 0x1086d6) and
  `0x70`, `0xb` (`call` at 0x108724). Another `_sysOutByte` site begins `0x71`, `0x42` (`call` at
  0x108730). A four-byte store writes `0x1` at absolute `0x1afe60` (`mov` at 0x10873f). The indexed
  values and resulting device behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1afe60` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1afe68` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1afe60` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1afe60] != 0x0` | `[0x1afe60] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. The four-byte read at absolute `0x1afe68` (`mov` at 0x10875b) reaches the return as
  EAX `[0x1afe68]` (`ret` at 0x108763). The numeric field lacks established units in this routine.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1afe68` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1afe68]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The entry branch takes `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` and falls through for
  lesser or equal values (`ja` at 0x108778). A later route stores four-byte `[sp+4]` at absolute
  `0x1afe68` (`mov` at 0x1087a8). The next decision takes `[0x1afe60] == 0x0` and falls through for
  nonzero (`je` at 0x1087b5). The fall-through includes `_sysAuxClkDisable` (`call` at 0x1087b7)
  followed by `_sysAuxClkEnable` (`call` at 0x1087bc), both with unknown candidate slots. The
  table-match interpretation remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1afe68` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1afe60` | `-` |

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
| `je` at 0x1087b5 | `[0x1afe60] == 0x0` | `[0x1afe60] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. The first two `_intConnect` leading triples are `0x100`, `0x108498`, `0x0` (`call` at
  0x10b867) and `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b). After `_sysSerialHwInit2` with
  unknown candidates (`call` at 0x10b883), two more `_intConnect` sites begin `0x138`, `0x10bbc8`,
  `0x0` (`call` at 0x10b894) and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b8a8). The last result
  reaches the return as EAX `ret(_intConnect)` (`ret` at 0x10b8b0). The interrupt associations
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x17a200` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x17a200` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x17a200` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x17a200` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. Calling `_sysIntInitPIC` presents leading candidate entry `ebp` (`call` at 0x10b83b).
  The following four-byte store writes `0x108398` at absolute `0x1d6c50` (`mov` at 0x10b840). The
  `_sysSerialHwInit` site has unknown candidates (`call` at 0x10b84a), and the return carries its
  `ret(_sysSerialHwInit)` result in EAX (`ret` at 0x10b852). The stored address meaning is
  unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d6c50` | `0x108398` |

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

- `_sysReboot`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. A six-byte load accesses absolute `0x0` (`lgdt` at 0x108235). The following static
  return carries EAX `0x0` (`ret` at 0x108238). Unknown descriptor contents, privilege state and
  possible faults prevent a runtime-return conclusion.

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
