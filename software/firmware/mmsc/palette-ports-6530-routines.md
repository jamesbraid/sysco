# MMSC palette and port paths in IRIX 6.5.30

Read from `6.5.30/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.30).
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. Raw zero bytes at `0x14ec4f`
and `0x15beef` remain outside the decoded instructions. The unnamed targets `0x14eb7d`
and `0x14ebc6` are included in the `_mw8514RPal` listing but are not reached from
that entry. Their effects are not attributed to the callers. Statements below
concern sites reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records,
and omit implicit push/pop accesses. The full facts retain those accesses. Call
and return instructions remain in the listings without separate implicit stack
access rows.
The tables retain analysis limits for partial registers, self-subtraction and
unsupported instructions. An unknown result does not establish that the firmware
produces an indeterminate value.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/palette-ports-6530-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e810 to 0x14e878.
  Established. Three two-byte loads use `[[ebp+16]+12] +0x0` (`mov` at 0x14e821), `[[ebp+16]+12]
  +0x2` (`mov` at 0x14e834), and `[[ebp+16]+12] +0x4` (`mov` at 0x14e848). The first branch uses the
  flag-only condition `Z == 0` (`jne` at 0x14e81a). The software interrupts carry immediate `0x10`
  (`int` at 0x14e85e, `int` at 0x14e871), without establishing a service or hardware reply.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e812 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e815 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e81c | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e821 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14e834 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14e848 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14e860 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e863 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14e81a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14e827 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e82f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e83b | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e843 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e84f | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e857 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14e869 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e877 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e878 to 0x14e92c.
  Established. The byte input at I/O absolute `0x3da` (`in` at 0x14e8a4) follows the fall-through
  condition `([[ebp+16]]:1 & 0xff) == 0x0` (`jne` at 0x14e89e). The byte outputs have unknown I/O
  addresses, with unknown values at two sites (`out` at 0x14e8f7, `out` at 0x14e8fa) and `0x20` at
  another (`out` at 0x14e907). The intervening byte store also has unknown address and value (`mov`
  at 0x14e8ee).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e87d | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e880 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e883 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e899 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14e8a4 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14e8a9 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e8b5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8c8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8db | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8ee | store | 1 | `(unknown)` | `?` |
| `out` at 0x14e8f7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e8fa | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e907 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e889 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e892 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e89e | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14e8f3 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14e903 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14e91e | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e90f | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e92c to 0x14e9bc.
  Established. The first guard takes its branch when `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` (`jbe`
  at 0x14e93c). A second guard takes its branch when `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned,
  32-bit)` (`jae` at 0x14e942). The guarded path includes three two-byte stores with unknown
  destinations and values (`mov` at 0x14e969, `mov` at 0x14e983, `mov` at 0x14e99e). The repeat
  comparison keeps both operands unknown (`jle` at 0x14e9aa).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e930 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e933 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e936 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e94d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e950 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e969 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e983 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e99e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e93c | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e942 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e95a | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e963 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e974 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e97d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e98f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e998 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14e9aa | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e9b2 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9bc to 0x14e9e4.
  Established. At I/O absolute `0x3c0`, byte output values are `0x11` (`out` at 0x14e9c9), unknown
  (`out` at 0x14e9d8), and `0x20` (`out` at 0x14e9db). The earlier byte input reads I/O absolute
  `0x3da` (`in` at 0x14e9c2). The byte table load remains unresolved (`mov` at 0x14e9d6), so its
  index and value receive no inferred interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14e9c2 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14e9c9 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14e9ca | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e9d6 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14e9d8 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14e9db | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e9e0 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9e4 to 0x14ea98.
  Established. Taking the first branch when `[[ebp+16]] == 0x0` (`je` at 0x14e9f7) reaches the
  shared load at `ebp +0x8` (`mov` at 0x14ea0d). Falling through reaches the unnamed call to
  `0x14eb7d` (`call` at 0x14e9f9) and then jumps to the restore sequence (`jmp` at 0x14e9fe). The
  shared byte outputs include four unknown values at unknown I/O addresses (`out` at 0x14ea4a, `out`
  at 0x14ea4f, `out` at 0x14ea52, `out` at 0x14ea55).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e9f1 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14e9f4 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14ea0d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ea10 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ea14 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ea17 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ea1a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea2d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea36 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea39 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea3c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea4a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea4f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea52 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea55 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea72 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea75 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea78 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea7d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea82 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea85 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea88 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14e9f9 | `(unknown callee) 0x14eb7d` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14e9f7 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14ea23 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea29 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea34 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea5a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea8d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea6a | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea98 to 0x14eb7d.
  Established. The unnamed direct call to `0x14ebc6` (`call` at 0x14eaad) lies on the first branch
  fall-through, whose condition is `[[ebp+16]] != 0x0` (`je` at 0x14eaab). That call path jumps to
  restoration (`jmp` at 0x14eab2). The taken arm reaches a two-byte load at `[ebp+8] +0x16` (`mov`
  at 0x14eac7) and includes byte inputs at unknown I/O addresses (`in` at 0x14eaf2, `in` at
  0x14eaf5, `in` at 0x14eaf8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14eaa5 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14eaa8 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14eac4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eac7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eacb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eace | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ead1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14eae4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eaef | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eaf2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eaf5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eaf8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb0a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb1c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb2f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb4f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb52 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb55 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb58 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb5f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb66 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb6e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14eaad | `(unknown callee) 0x14ebc6` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14eaab | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14eada | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14eae0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eaeb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eb05 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb17 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb2a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb35 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb74 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb45 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ec38 to 0x14ec50.
  Established. A four-byte load at `ebp +0x10` (`mov` at 0x14ec45) separates the two byte outputs to
  I/O absolute `0x3c0`. Their values are `0x31` (`out` at 0x14ec44) and unknown (`out` at 0x14ec48).
  The byte input at I/O absolute `0x3da` precedes both (`in` at 0x14ec3d). Port register roles
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14ec3d | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14ec44 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14ec45 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14ec48 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ec4c | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea00 to 0x14ea98.
  Established. The failed upper-bound guard falls through when `[[ebp+16]+8] > 0xff (unsigned,
  32-bit)` (`jbe` at 0x14ea23), then jumps (`jmp` at 0x14ea25) to the assignment
  `eax = 0xffffffff` (`mov` at 0x14ea6b). Only after
  following both range guards (`jbe` at 0x14ea23, `jae` at 0x14ea29) does the arm-selection branch test
  `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14ea34). The eight byte outputs retain
  unknown I/O targets and values (`out` at 0x14ea4a, `out` at 0x14ea4f, `out` at 0x14ea52, `out` at
  0x14ea55, `out` at 0x14ea7d, `out` at 0x14ea82, `out` at 0x14ea85, `out` at 0x14ea88).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ea0d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ea10 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ea14 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ea17 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ea1a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea2d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea36 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea39 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea3c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea4a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea4f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea52 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea55 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea72 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea75 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea78 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea7d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea82 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea85 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea88 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ea23 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea29 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea34 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea5a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea8d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea6a | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14eab7 to 0x14ec38.
  Established. The selected fall-through arm has condition `(([[ebp+8]+22]:2 & 0xffff) & 0x100) ==
  0x0` (`jne` at 0x14eaeb). It includes a byte output (`out` at 0x14eaef), byte inputs (`in` at
  0x14eaf2, `in` at 0x14eaf5, `in` at 0x14eaf8), and two-byte stores (`mov` at 0x14eb0a, `mov` at
  0x14eb1c, `mov` at 0x14eb2f). Their effective targets remain unknown, as do output and stored
  values. The other arm is retained separately in the access table.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14eac4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eac7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eacb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eace | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ead1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14eae4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eaef | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eaf2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eaf5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eaf8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb0a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb1c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb2f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb4f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb52 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb55 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb58 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb5f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb66 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb6e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14eada | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14eae0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eaeb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eb05 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb17 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb2a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb35 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb74 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb45 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172e50 to 0x172ec0.
  Established. Four `_sysOutByte` calls carry candidate zero `0x3c8` at the first site (`call` at
  0x172e5d), then `0x3c9` at the later sites (`call` at 0x172e79, `call` at 0x172e95, `call` at
  0x172eb1). None has a resolved candidate one. The three repeat branches all use `N == 0` as the
  taken condition (`jns` at 0x172e6d, `jns` at 0x172e89, `jns` at 0x172ea5). Iteration counts remain
  unestablished.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x172e53 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172e6f | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x172e8b | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x172ea7 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172e5d | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x172e79 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172e95 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172eb1 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172e6d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172e89 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172ea5 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172ebb | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172ec0 to 0x172f40.
  Established. The `_sysOutByte` call presents candidate zero `0x3c7` (`call` at 0x172ed9). Three
  `_sysInByte` calls each present `0x3c9` in that slot (`call` at 0x172ef0, `call` at 0x172f0c,
  `call` at 0x172f28). Their following four-byte stores target `[sp+8] +0x0` (`mov` at 0x172efd),
  `[sp+12] +0x0` (`mov` at 0x172f19), and `[sp+16] +0x0` (`mov` at 0x172f32), with unknown stored
  values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172ec6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172ec9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x172ecc | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x172ecf | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x172efd | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x172f19 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x172f32 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172ed9 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172ef0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f0c | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f28 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172ee9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f05 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f21 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172f3f | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172f40 to 0x172fd0.
  Established. The signed guard is taken when `0x0 >= [sp+8] (signed, 32-bit)` (`jge` at 0x172f62),
  bypassing the body. The body includes three byte loads at unknown effective addresses (`movsx` at
  0x172f6f, `movsx` at 0x172f8b, `movsx` at 0x172fa7); its `_sysOutByte` call candidates include
  candidate zero `0x3c9` with unknown candidate one (`call` at 0x172f7b, `call` at 0x172f97, `call`
  at 0x172fb3). The repeat left operand stays unknown (`jl` at 0x172fbe).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172f46 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172f49 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172f4c | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172f6f | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172f8b | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172fa7 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172f56 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172f7b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f97 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172fb3 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172f62 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172f6d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f89 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172fa5 | `N == 0` | `not (N == 0)` |
| `jl` at 0x172fbe | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172fcb | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172fd0 to 0x173070.
  Established. Falling through the signed guard means `0x0 < [sp+8] (signed, 32-bit)` (`jge` at
  0x172ff2). That body reaches the `_sysInByte` sites with candidate zero `0x3c9` (`call` at
  0x173004, `call` at 0x173024, `call` at 0x173044). The three four-byte stores use unknown
  effective addresses and values (`mov` at 0x173011, `mov` at 0x173031, `mov` at 0x173051). The
  eventual return value is also unknown (`ret` at 0x173066).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172fd6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172fd9 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172fdc | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x173011 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173031 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173051 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172fe6 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x173004 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173024 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173044 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172ff2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172ffd | `N == 0` | `not (N == 0)` |
| `jns` at 0x17301d | `N == 0` | `not (N == 0)` |
| `jns` at 0x17303d | `N == 0` | `not (N == 0)` |
| `jl` at 0x173059 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173066 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be9c to 0x15bef0.
  Established. Four-byte stores include `[sp+4]` at `sp -0x20` (`mov` at 0x15bea8) and `[sp+16]` at
  `sp -0x14` (`mov` at 0x15beba). The indirect call operand loads four bytes at `[[0x1d323c]] +0xcc`
  (`call` at 0x15bed0); its leading candidates are `[[0x1d323c]]`, `0x4`, and `sp-32` (`call` at
  0x15bed0). The later unnamed direct call targets `0x166090` (`call` at 0x15bee2), without
  establishing either callee effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15bea5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15bea8 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15beab | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15beae | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15beb1 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15beb4 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15beb7 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15beba | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15bebd | load | 4 | `abs 0x1d323c` | `-` |
| `mov` at 0x15bec3 | load | 4 | `[0x1d323c] +0x0` | `-` |
| `call` at 0x15bed0 | load | 4 | `[[0x1d323c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15bed0 | `(unknown callee) (unknown target)` | `[[0x1d323c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15bee2 | `(unknown callee) 0x166090` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15bedb | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15beee | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.5.30/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be48 to 0x15be9c.
  Established. The four-byte load at absolute `0x1d323c` (`mov` at 0x15be69) is followed by another
  at `[0x1d323c] +0x0` (`mov` at 0x15be6f). The indirect call through `[[0x1d323c]] +0xcc` presents
  leading candidates `[[0x1d323c]]`, `0x3`, and `sp-32` (`call` at 0x15be7c). The branch is taken
  when `Z == 1` (`je` at 0x15be87), bypassing the unnamed direct call (`call` at 0x15be8e). Routine
  purpose remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15be51 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15be54 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15be57 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15be5a | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15be5d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15be60 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15be63 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15be66 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15be69 | load | 4 | `abs 0x1d323c` | `-` |
| `mov` at 0x15be6f | load | 4 | `[0x1d323c] +0x0` | `-` |
| `call` at 0x15be7c | load | 4 | `[[0x1d323c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15be7c | `(unknown callee) (unknown target)` | `[[0x1d323c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15be8e | `(unknown callee) 0x166090` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15be87 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15be9a | `eax` | `?` |
