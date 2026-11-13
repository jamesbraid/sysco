# MMSC clocks and board setup in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/bsp-clocks-sg0002378-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The branch bypasses the indirect call when `[0x1abfdc] == 0x0` (`je` at 0x1084a2).
  The indirect callee is unknown, with candidate zero `[0x1abfe0]` (`call` at 0x1084af). Both
  absolute four-byte loads feeding that site are retained below (`mov` at 0x1084a4, `mov` at
  0x1084aa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1abfdc` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1abfe0` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1abfdc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1abfe0]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1abfdc] == 0x0` | `[0x1abfdc] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. After the `_sysHwInit2` call (`call` at 0x1084c3), four-byte stores place `[sp+4]` at
  absolute `0x1abfdc` (`mov` at 0x1084c8), `[sp+8]` at absolute `0x1abfe0` (`mov` at 0x1084ce), and
  `0x1` at absolute `0x1abfe8` (`mov` at 0x1084d4). The exit exposes `0x0` in `eax` (`ret` at
  0x1084e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1abfdc` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1abfe0` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1abfe8` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The zero-state branch skips the calls when `[0x1abfe4] == 0x0` (`je` at 0x108502). On
  fall-through, `_sysOutByte` sites present leading candidates `0x43`, `0x38` (`call` at 0x108508),
  then `0x40`, `0x0` (`call` at 0x108514), then `0x40`, `0x0` again (`call` at 0x108520). A
  four-byte store clears absolute `0x1abfe4` (`mov` at 0x10852f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1abfe4` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1abfe4` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1abfe4] == 0x0` | `[0x1abfe4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The branch skips the body when `[0x1abfe4] != 0x0` (`jne` at 0x108553). The first
  `_sysOutByte` site has leading candidates `0x43` and `0x36` (`call` at 0x108567). The two later
  sites each have candidate zero `0x40` and unknown candidate one (`call` at 0x108575, `call` at
  0x108588). The four-byte state store writes `0x1` to absolute `0x1abfe4` (`mov` at 0x108597).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1abfe4` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1abfec` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1abfe4` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1abfe4] != 0x0` | `[0x1abfe4] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. The sole explicit access loads four bytes from absolute `0x1abfec` (`mov` at
  0x1085ab). The return reports `[0x1abfec]` in `eax` (`ret` at 0x1085b3). Units and the field role
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1abfec` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1abfec]` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The acceptance branch is taken when `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` (`jbe`
  at 0x1085c6). That path writes `[sp+4]` as four bytes to absolute `0x1abfec` (`mov` at 0x1085d8).
  The subsequent branch skips both `_sysClkDisable` (`call` at 0x1085e7) and `_sysClkEnable` (`call`
  at 0x1085ec) when `[0x1abfe4] == 0x0` (`je` at 0x1085e5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1abfec` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1abfe4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1abfe4] == 0x0` | `[0x1abfe4] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The `_sysOutByte` site has leading candidates `0x70` and `0xc` (`call` at 0x1085ff),
  followed by `_sysInByte` with candidate zero `0x71` (`call` at 0x108609). The indirect call has
  unknown callee and candidate zero `[0x1abff4]` (`call` at 0x108625). The branch bypasses it when
  `[0x1abff0] == 0x0` (`je` at 0x108618).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1abff0` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1abff4` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1abff0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1abff4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1abff0] == 0x0` | `[0x1abff0] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. Three four-byte stores write `[sp+4]` to absolute `0x1abff0` (`mov` at 0x108641),
  `[sp+8]` to absolute `0x1abff4` (`mov` at 0x108646), and `0x1` to absolute `0x1abffc` (`mov` at
  0x10864c). No call sites appear in this span. These fields have unknown semantic roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1abff0` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1abff4` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1abffc` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The body is skipped when `[0x1abff8] == 0x0` (`je` at 0x108672); its two
  `_sysOutByte` sites present leading candidates `0x70`, `0xb` (`call` at 0x108678) and `0x71`,
  `0x2` (`call` at 0x108684). After the `_sysIntDisablePIC` call with candidate zero `0x8` (`call`
  at 0x10868e), a four-byte store clears absolute `0x1abff8` (`mov` at 0x108693).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1abff8` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1abff8` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1abff8] == 0x0` | `[0x1abff8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The indexed four-byte load and indexed byte load both have unknown effective
  addresses (`mov` at 0x1086c4, `or` at 0x1086f9). The conditional `_sysOutByte` site with candidate
  zero `0x71` has unknown candidate one (`call` at 0x108703). The later sites present `0x70`, `0xb`
  (`call` at 0x108724) and `0x71`, `0x42` (`call` at 0x108730). The final four-byte store writes
  `0x1` to absolute `0x1abff8` (`mov` at 0x10873f).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1abff8` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1ac000` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1abff8` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1abff8] != 0x0` | `[0x1abff8] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. The four-byte absolute load reads `0x1ac000` (`mov` at 0x10875b). The exit retains
  that loaded expression, `[0x1ac000]`, as its return value (`ret` at 0x108763). This body
  establishes no unit for the field.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1ac000` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1ac000]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The range branch is taken when `[sp+4]-2 >
  0x1ffe (unsigned, 32-bit)` (`ja` at 0x108778). The indexed four-byte comparison access remains
  unresolved (`cmp` at 0x10877c). The four-byte store at the match target writes `[sp+4]` to
  absolute `0x1ac000` (`mov` at 0x1087a8). The later `_sysAuxClkDisable` (`call` at 0x1087b7) and
  `_sysAuxClkEnable` (`call` at 0x1087bc) calls are bypassed when `[0x1abff8] == 0x0` (`je` at
  0x1087b5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1ac000` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1abff8` | `-` |

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
| `je` at 0x1087b5 | `[0x1abff8] == 0x0` | `[0x1abff8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. Four `_intConnect` sites have leading candidates `0x100`, `0x108498`, `0x0` (`call`
  at 0x10b867), `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b), `0x138`, `0x10bbc8`, `0x0` (`call`
  at 0x10b894), and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b8a8). Between the second and third is
  a `_sysSerialHwInit2` call with unknown candidates (`call` at 0x10b883).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x176398` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x176398` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x176398` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x176398` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. The `_sysIntInitPIC` call precedes a four-byte store of `0x108398` to absolute
  `0x1d2d30` (`call` at 0x10b83b, `mov` at 0x10b840). The subsequent `_sysSerialHwInit` call has
  unknown candidate values (`call` at 0x10b84a). The exit carries `ret(_sysSerialHwInit)` (`ret` at
  0x10b852).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d2d30` | `0x108398` |

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

- `_sysReboot`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108230 to 0x108240.
  Established. A six-byte access loads from absolute `0x0` through `lgdt` (`lgdt` at 0x108235). The
  following return exposes `0x0` in `eax` (`ret` at 0x108238). The name alone does not establish a
  reboot mechanism.

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
