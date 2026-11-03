# MMSC clock and board-setup callers in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
Some include literals and padding decoded linearly after returns. Only sites reached
by known control-flow edges contribute effects to the tables.
Repeated `ret(F)` labels can refer to distinct call results and do not prove equal addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Calls use
the assumed cdecl32 convention with unknown arity. Printed candidates can be local values,
saved registers or stale stack words. Unknown candidates and effective addresses stay unknown.
The access tables show explicit memory operands, including stack locations, and omit
implicit push/pop accesses. Full facts retain those accesses. Call and return instructions
remain listed without separate implicit stack access rows.
Unsupported instructions and partial-register tracking can leave results unresolved.
An analysis unknown does not establish indeterminate firmware behavior.
Field meanings, hardware roles, command semantics and callee effects remain unknown
unless explicitly established by an entry.

[Fact sheets and complete listings](disasm/clocks-board-64-origin-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The deciding branch falls through when `[0x1a9c2c] != 0x0` (`je` at 0x1084a2). The
  indirect call has an unknown callee and candidate zero `[0x1a9c30]` (`call` at 0x1084af). The exit
  value remains unknown (`ret` at 0x1084b4). No clock-callback identity is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1a9c2c` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1a9c30` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1a9c2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1a9c30]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1a9c2c] == 0x0` | `[0x1a9c2c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. Four-byte stores write `[sp+4]` at absolute `0x1a9c2c` (`mov` at 0x1084c8), `[sp+8]`
  at `0x1a9c30` (`mov` at 0x1084ce), and `0x1` at `0x1a9c38` (`mov` at 0x1084d4). The `_sysHwInit2`
  site presents candidate zero `ebx` (`call` at 0x1084c3). The exit reports `0x0` (`ret` at
  0x1084e8), without proving callback registration.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1a9c2c` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1a9c30` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1a9c38` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b7f8` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The `_sysOutByte` sites present leading pairs `0x43`, `0x38` (`call` at 0x108508),
  and `0x40`, `0x0` (`call` at 0x108514, `call` at 0x108520). A four-byte store at absolute
  `0x1a9c34` writes `0x0` (`mov` at 0x10852f). The `_sysIntDisablePIC` site presents candidate zero
  `0x0` (`call` at 0x10852a). Timer and interrupt effects remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1a9c34` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1a9c34` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1a9c34] == 0x0` | `[0x1a9c34] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The `_sysOutByte` site presents `0x43` and `0x36` as its leading candidates (`call`
  at 0x108567). The later sites have candidate zero `0x40` and unknown candidate one (`call` at
  0x108575, `call` at 0x108588). The four-byte store at absolute `0x1a9c34` writes `0x1` (`mov` at
  0x108597). No timer divisor or frequency is inferred from the raw division.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1a9c34` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1a9c3c` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1a9c34` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1a9c34] != 0x0` | `[0x1a9c34] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The range branch is taken when `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` (`jbe` at
  0x1085c6). A four-byte store at absolute `0x1a9c3c` writes `[sp+4]` (`mov` at 0x1085d8). The later
  branch falls through when `[0x1a9c34] != 0x0` (`je` at 0x1085e5), reaching `_sysClkDisable`
  (`call` at 0x1085e7) and `_sysClkEnable` (`call` at 0x1085ec), each with unknown candidates. No
  rate unit or timer outcome is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1a9c3c` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1a9c34` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1a9c34] == 0x0` | `[0x1a9c34] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The `_sysOutByte` site presents leading candidates `0x70` and `0xc` (`call` at
  0x1085ff). The `_sysInByte` site has candidate zero `0x71` (`call` at 0x108609). The indirect call
  has an unknown callee and candidate zero `[0x1a9c44]` (`call` at 0x108625). Neither a register
  role nor callback identity is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1a9c40` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1a9c44` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1a9c40` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1a9c44]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1a9c40] == 0x0` | `[0x1a9c40] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. The four-byte stores write `[sp+4]` at absolute `0x1a9c40` (`mov` at 0x108641),
  `[sp+8]` at `0x1a9c44` (`mov` at 0x108646), and `0x1` at `0x1a9c4c` (`mov` at 0x10864c). The exit
  retains `0x0` (`ret` at 0x10865b). Those assignments do not establish auxiliary-clock
  registration.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1a9c40` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1a9c44` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1a9c4c` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. The `_sysOutByte` sites have leading pairs `0x70`, `0xb` (`call` at 0x108678) and
  `0x71`, `0x2` (`call` at 0x108684). The `_sysIntDisablePIC` site presents candidate zero `0x8`
  (`call` at 0x10868e). A four-byte store at absolute `0x1a9c48` writes `0x0` (`mov` at 0x108693).
  No interrupt or clock-disabling effect is assigned from the names.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1a9c48` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1a9c48` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1a9c48] == 0x0` | `[0x1a9c48] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The `_sysOutByte` sites include leading pairs `0x70`, `0xa` (`call` at 0x1086d6,
  `call` at 0x1086f1), `0x70`, `0xb` (`call` at 0x108724), and `0x71`, `0x42` (`call` at 0x108730).
  The site with candidate zero `0x71` has unknown candidate one (`call` at 0x108703). A four-byte
  store at absolute `0x1a9c48` writes `0x1` (`mov` at 0x10873f). Timer encoding and frequency remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1a9c48` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1a9c50` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1a9c48` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1a9c48] != 0x0` | `[0x1a9c48] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. The four-byte load reads absolute `0x1a9c50` (`mov` at 0x10875b), and its expression
  `[0x1a9c50]` reaches the exit (`ret` at 0x108763). The body does not establish a rate unit.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1a9c50` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1a9c50]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The first branch is taken when `[sp+4]-2 > 0x1ffe (unsigned, 32-bit)` (`ja` at
  0x108778). A four-byte store at absolute `0x1a9c50` writes `[sp+4]` (`mov` at 0x1087a8). The
  `_sysAuxClkDisable` (`call` at 0x1087b7) and `_sysAuxClkEnable` (`call` at 0x1087bc) sites retain
  unknown candidate slots. No supported-frequency list is inferred from the raw indexed comparisons.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1a9c50` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1a9c48` | `-` |

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
| `je` at 0x1087b5 | `[0x1a9c48] == 0x0` | `[0x1a9c48] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7f8 to 0x10b858.
  Established. The `_intConnect` sites present leading triples `0x100`, `0x108498`, `0x0` (`call` at
  0x10b807), `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b81b), `0x138`, `0x10bb68`, `0x0` (`call` at
  0x10b834), and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b848). Those candidate addresses do not
  establish calls through them. The `_sysSerialHwInit2` site retains unknown candidates (`call` at
  0x10b823). Interrupt-vector meanings remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b807 | `_intConnect 0x173fe8` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b81b | `_intConnect 0x173fe8` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b823 | `_sysSerialHwInit2 0x10b718` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b834 | `_intConnect 0x173fe8` | `0x138` | `0x10bb68` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b848 | `_intConnect 0x173fe8` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b850 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7d8 to 0x10b7f8.
  Established. The four-byte store at absolute `0x1d09a4` writes `0x108398` (`mov` at 0x10b7e0). The
  `_sysIntInitPIC` call has candidate zero `ebp` (`call` at 0x10b7db). The `_sysSerialHwInit` site
  retains unknown candidates (`call` at 0x10b7ea), and its result expression `ret(_sysSerialHwInit)`
  reaches the exit (`ret` at 0x10b7f2). No board-initialization outcome is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b7e0 | store | 4 | `abs 0x1d09a4` | `0x108398` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b7db | `_sysIntInitPIC 0x1082f8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b7ea | `_sysSerialHwInit 0x10b618` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7f2 | `eax` | `ret(_sysSerialHwInit)` |

## `_sysNvRamGet`

- `_sysNvRamGet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087c8 to 0x1087d8.
  Established. The immediate assignment writes `0xffffffff` to `eax` (`mov` at 0x1087cb). The exit
  retains that literal (`ret` at 0x1087d3). The tables report no explicit accesses or calls, so no
  NVRAM read effect is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087d3 | `eax` | `0xffffffff` |

## `_sysNvRamSet`

- `_sysNvRamSet`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087d8 to 0x1087e8.
  Established. The assignment places `0xffffffff` in `eax` (`mov` at 0x1087db), which the exit
  reports (`ret` at 0x1087e3). No explicit access or call rows appear. An NVRAM write role remains
  unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087e3 | `eax` | `0xffffffff` |
