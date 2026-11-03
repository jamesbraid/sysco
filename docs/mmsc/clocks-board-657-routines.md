# MMSC clock and board-setup callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
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

[Fact sheets and complete listings](disasm/clocks-board-657-facts.txt) include all tool sections.

## `_sysClkInt`

- `_sysClkInt`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108498 to 0x1084b8.
  Established. The branch is taken when `[0x1afe54] == 0x0`, past the indirect call (`je` at
  0x1084a2). On the fall-through, the unknown callee receives candidate zero `[0x1afe58]` (`call` at 0x1084af).
  The prior four-byte loads read absolute `0x1afe58` and `0x1afe54` (`mov` at 0x1084a4, `mov` at
  0x1084aa). Neither the call target nor a clock-service role is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10849b | load | 4 | `abs 0x1afe54` | `-` |
| `mov` at 0x1084a4 | load | 4 | `abs 0x1afe58` | `-` |
| `mov` at 0x1084aa | load | 4 | `abs 0x1afe54` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084af | `(unknown callee) (unknown target)` | `[0x1afe58]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1084a2 | `[0x1afe54] == 0x0` | `[0x1afe54] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084b4 | `eax` | `?` |

## `_sysClkConnect`

- `_sysClkConnect`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084b8 to 0x1084f8.
  Established. The `_sysHwInit2` call presents leading candidates `ebx`, `esi`, `ebp` (`call` at
  0x1084c3). Three four-byte assignments then write `[sp+4]` at absolute `0x1afe54`, `[sp+8]` at
  `0x1afe58`, and `0x1` at `0x1afe60` (`mov` at 0x1084c8, `mov` at 0x1084ce, `mov` at 0x1084d4). The
  common exit has value zero (`ret` at 0x1084e8). The stored fields have Unknown roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1084bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1084c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1084c8 | store | 4 | `abs 0x1afe54` | `[sp+4]` |
| `mov` at 0x1084ce | store | 4 | `abs 0x1afe58` | `[sp+8]` |
| `mov` at 0x1084d4 | store | 4 | `abs 0x1afe60` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1084c3 | `_sysHwInit2 0x10b858` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1084e8 | `eax` | `0x0` |

## `_sysClkDisable`

- `_sysClkDisable`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1084f8 to 0x108548.
  Established. The first branch falls through when `[0x1afe5c] != 0x0` (`je` at 0x108502). That arm
  includes `_sysOutByte` calls with leading pairs `0x43`, `0x38`, then `0x40`, `0x0` twice (`call`
  at 0x108508, `call` at 0x108514, `call` at 0x108520). `_sysIntDisablePIC` has candidate zero `0x0`
  (`call` at 0x10852a), before the four-byte zero assignment to absolute `0x1afe5c` (`mov` at
  0x10852f). Peripheral effects inside those callees are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1084fb | load | 4 | `abs 0x1afe5c` | `-` |
| `mov` at 0x10852f | store | 4 | `abs 0x1afe5c` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108508 | `_sysOutByte 0x1081d0` | `0x43` | `0x38` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108514 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x108520 | `_sysOutByte 0x1081d0` | `0x40` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10852a | `_sysIntDisablePIC 0x1083b8` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108502 | `[0x1afe5c] == 0x0` | `[0x1afe5c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10853c | `eax` | `?` |

## `_sysClkEnable`

- `_sysClkEnable`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108548 to 0x1085a8.
  Established. The `jne` falls through when `[0x1afe5c] == 0x0` (`jne` at 0x108553). The four-byte
  division operand is loaded at absolute `0x1afe64` (`idiv` at 0x10855b). `_sysOutByte` receives
  leading candidates `0x43`, `0x36` at its first site (`call` at 0x108567), and `0x40` with unknown
  second slots at its next two (`call` at 0x108575, `call` at 0x108588). The later four-byte store
  writes `0x1` at absolute `0x1afe5c` (`mov` at 0x108597). Frequency and divisor units remain
  Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10854c | load | 4 | `abs 0x1afe5c` | `-` |
| `idiv` at 0x10855b | load | 4 | `abs 0x1afe64` | `-` |
| `mov` at 0x108597 | store | 4 | `abs 0x1afe5c` | `0x1` |
| `mov` at 0x1085a1 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108567 | `_sysOutByte 0x1081d0` | `0x43` | `0x36` | `?` | `?` | `?` | `?` |
| `call` at 0x108575 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108588 | `_sysOutByte 0x1081d0` | `0x40` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108592 | `_sysIntEnablePIC 0x108428` | `0x0` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x108553 | `[0x1afe5c] != 0x0` | `[0x1afe5c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085a7 | `eax` | `?` |

## `_sysClkRateSet`

- `_sysClkRateSet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085b8 to 0x1085f8.
  Established. The unsigned range decision takes its assignment arm when `[sp+4]-19 <= 0x1221
  (unsigned, 32-bit)` (`jbe` at 0x1085c6). The four-byte store writes `[sp+4]` at absolute
  `0x1afe64` (`mov` at 0x1085d8). A nonzero `[0x1afe5c]` falls through the later branch (`je` at
  0x1085e5), reaching `_sysClkDisable` and `_sysClkEnable` with unknown candidates (`call` at
  0x1085e7, `call` at 0x1085ec). The numeric unit is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085bb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1085d8 | store | 4 | `abs 0x1afe64` | `[sp+4]` |
| `cmp` at 0x1085de | load | 4 | `abs 0x1afe5c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085e7 | `_sysClkDisable 0x1084f8` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1085ec | `_sysClkEnable 0x108548` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1085c6 | `[sp+4]-19 <= 0x1221 (unsigned, 32-bit)` | `[sp+4]-19 > 0x1221 (unsigned, 32-bit)` |
| `je` at 0x1085e5 | `[0x1afe5c] == 0x0` | `[0x1afe5c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085d0 | `eax` | `0xffffffff` |
| `ret` at 0x1085f6 | `eax` | `0x0` |

## `_sysAuxClkInt`

- `_sysAuxClkInt`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085f8 to 0x108638.
  Established. The first two calls are `_sysOutByte` with candidates `0x70`, `0xc`, and `_sysInByte`
  with candidate zero `0x71` (`call` at 0x1085ff, `call` at 0x108609). The later branch falls
  through for `[0x1afe68] != 0x0` (`je` at 0x108618). That path reaches an unknown callee with
  candidate zero `[0x1afe6c]` (`call` at 0x108625). These call sites do not identify a register or
  callback role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x108611 | load | 4 | `abs 0x1afe68` | `-` |
| `mov` at 0x10861a | load | 4 | `abs 0x1afe6c` | `-` |
| `mov` at 0x108620 | load | 4 | `abs 0x1afe68` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1085ff | `_sysOutByte 0x1081d0` | `0x70` | `0xc` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108609 | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108625 | `(unknown callee) (unknown target)` | `[0x1afe6c]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108618 | `[0x1afe68] == 0x0` | `[0x1afe68] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10862a | `eax` | `?` |

## `_sysAuxClkConnect`

- `_sysAuxClkConnect`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108638 to 0x108668.
  Established. This body writes four-byte `[sp+4]` to absolute `0x1afe68`, `[sp+8]` to `0x1afe6c`,
  and `0x1` to `0x1afe74` (`mov` at 0x108641, `mov` at 0x108646, `mov` at 0x10864c); its exit
  expression is zero (`ret` at 0x10865b). There are no conditional branch or call rows.
  Auxiliary-clock registration is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10863b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10863e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108641 | store | 4 | `abs 0x1afe68` | `[sp+4]` |
| `mov` at 0x108646 | store | 4 | `abs 0x1afe6c` | `[sp+8]` |
| `mov` at 0x10864c | store | 4 | `abs 0x1afe74` | `0x1` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10865b | `eax` | `0x0` |

## `_sysAuxClkDisable`

- `_sysAuxClkDisable`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108668 to 0x1086a8.
  Established. A zero `[0x1afe70]` takes the branch past the call sequence (`je` at 0x108672). The
  fall-through includes `_sysOutByte` with pairs `0x70`, `0xb` and `0x71`, `0x2` (`call` at
  0x108678, `call` at 0x108684), then `_sysIntDisablePIC` with candidate zero `0x8` (`call` at
  0x10868e). A four-byte zero is stored at absolute `0x1afe70` (`mov` at 0x108693). Hardware command
  meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x10866b | load | 4 | `abs 0x1afe70` | `-` |
| `mov` at 0x108693 | store | 4 | `abs 0x1afe70` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108678 | `_sysOutByte 0x1081d0` | `0x70` | `0xb` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108684 | `_sysOutByte 0x1081d0` | `0x71` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10868e | `_sysIntDisablePIC 0x1083b8` | `0x8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x108672 | `[0x1afe70] == 0x0` | `[0x1afe70] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1086a0 | `eax` | `?` |

## `_sysAuxClkEnable`

- `_sysAuxClkEnable`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1086a8 to 0x108758.
  Established. The initial nonzero test takes the exit arm for `[0x1afe70] != 0x0` (`jne` at
  0x1086b5). The indexed four-byte and one-byte access targets remain unknown (`mov` at 0x1086c4,
  `or` at 0x1086f9). Calls include `_sysOutByte` with `0x70`, `0xa` at two sites (`call` at
  0x1086d6, `call` at 0x1086f1), and `0x71`, unknown at another (`call` at 0x108703). A four-byte
  store writes `0x1` at absolute `0x1afe70` (`mov` at 0x10873f). The selector catalogue and timer
  behavior are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x1086ae | load | 4 | `abs 0x1afe70` | `-` |
| `mov` at 0x1086c4 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x1086ca | load | 4 | `abs 0x1afe78` | `-` |
| `or` at 0x1086f9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10873f | store | 4 | `abs 0x1afe70` | `0x1` |

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
| `jne` at 0x1086b5 | `[0x1afe70] != 0x0` | `[0x1afe70] == 0x0` |
| `jne` at 0x1086d0 | `Z == 0` | `not (Z == 0)` |
| `jbe` at 0x10871e | `be flags condition (operands unknown)` | `not (be flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108752 | `eax` | `?` |

## `_sysAuxClkRateGet`

- `_sysAuxClkRateGet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108758 to 0x108768.
  Established. The sole explicit load reads four bytes from absolute `0x1afe78` (`mov` at 0x10875b).
  The exit preserves `[0x1afe78]` in `eax` (`ret` at 0x108763). No call or condition is reported,
  and the value has no proven unit.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10875b | load | 4 | `abs 0x1afe78` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108763 | `eax` | `[0x1afe78]` |

## `_sysAuxClkRateSet`

- `_sysAuxClkRateSet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108768 to 0x1087c8.
  Established. The first branch takes its literal-return arm when `[sp+4]-2 > 0x1ffe (unsigned,
  32-bit)` (`ja` at 0x108778). The indexed comparison target remains unknown (`cmp` at 0x10877c).
  Another path writes `[sp+4]` as four bytes at absolute `0x1afe78` (`mov` at 0x1087a8). The later
  branch falls through for nonzero `[0x1afe70]` (`je` at 0x1087b5), reaching `_sysAuxClkDisable` and
  `_sysAuxClkEnable` (`call` at 0x1087b7, `call` at 0x1087bc). Accepted frequencies are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10876b | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10877c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1087a8 | store | 4 | `abs 0x1afe78` | `[sp+4]` |
| `cmp` at 0x1087ae | load | 4 | `abs 0x1afe70` | `-` |

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
| `je` at 0x1087b5 | `[0x1afe70] == 0x0` | `[0x1afe70] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108798 | `eax` | `0xffffffff` |
| `ret` at 0x1087c6 | `eax` | `0x0` |

## `_sysHwInit2`

- `_sysHwInit2`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b858 to 0x10b8b8.
  Established. The `_intConnect` calls present leading triples `0x100`, `0x108498`, `0x0` (`call`
  at 0x10b867), `0x140`, `0x1085f8`, `0x0` (`call` at 0x10b87b), `0x138`, `0x10bbc8`, `0x0`
  (`call` at 0x10b894), and `0x108`, `0x10ae88`, `0x0` (`call` at 0x10b8a8). Those candidate addresses are not
  themselves visible call targets. `_sysSerialHwInit2` has unknown slots (`call` at 0x10b883).
  Vector identities and registration effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b867 | `_intConnect 0x17a210` | `0x100` | `0x108498` | `0x0` | `ebp` | `?` | `?` |
| `call` at 0x10b87b | `_intConnect 0x17a210` | `0x140` | `0x1085f8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b883 | `_sysSerialHwInit2 0x10b758` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b894 | `_intConnect 0x17a210` | `0x138` | `0x10bbc8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x10b8a8 | `_intConnect 0x17a210` | `0x108` | `0x10ae88` | `0x0` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b8b0 | `eax` | `ret(_intConnect)` |

## `_sysHwInit`

- `_sysHwInit`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b838 to 0x10b858.
  Established. `_sysIntInitPIC` is called with candidate zero `ebp` (`call` at 0x10b83b). The
  four-byte store at absolute `0x1d6c60` then writes `0x108398` (`mov` at 0x10b840).
  `_sysSerialHwInit` has unknown candidates (`call` at 0x10b84a), and its result expression reaches
  the exit (`ret` at 0x10b852). The assigned pointer role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b840 | store | 4 | `abs 0x1d6c60` | `0x108398` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b83b | `_sysIntInitPIC 0x1082f8` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b84a | `_sysSerialHwInit 0x10b618` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b852 | `eax` | `ret(_sysSerialHwInit)` |

## `_sysNvRamGet`

- `_sysNvRamGet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087c8 to 0x1087d8.
  Established. The immediate assignment places `0xffffffff` in `eax` (`mov` at 0x1087cb), retained
  at the exit (`ret` at 0x1087d3). No explicit accesses, calls or conditional branches appear. No
  NVRAM read role is established.

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

- `_sysNvRamSet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1087d8 to 0x1087e8.
  Established. The body gives `eax` the literal `0xffffffff` (`mov` at 0x1087db); its return reports
  that literal (`ret` at 0x1087e3). The explicit-access and call sections are empty. An NVRAM write
  effect remains unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1087e3 | `eax` | `0xffffffff` |
