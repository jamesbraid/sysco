# MMSC palette and port paths in IRIX 6.4-origin

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4-origin).
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. Statements below concern sites
reached by known control-flow edges. The helper bodies at `0x14996d` and
`0x1499b6` appear in the `_mw8514RPal` listing but are unreachable from that
entry. Their instructions do not establish effects of either caller.
The undecoded final bytes at `0x149a3f` and `0x156cdf` are each `0x00`.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack locations,
and omit implicit push/pop accesses. The full facts retain those accesses.
Call and return instructions
remain in the listings without separate implicit stack access rows.
Unsupported instructions and partial-register operations can leave values or flags unknown.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/palette-ports-64-origin-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x149600 to 0x149668.
  Established. The two-byte loads use `[[ebp+16]+12] +0x0` (`mov` at 0x149611), `[[ebp+16]+12] +0x2`
  (`mov` at 0x149624), and `[[ebp+16]+12] +0x4` (`mov` at 0x149638). The first branch has taken
  predicate `Z == 0` (`jne` at 0x14960a). Software interrupts carry immediate `0x10` (`int` at
  0x14964e, `int` at 0x149661). Their service roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x149602 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149605 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14960c | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x149611 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x149624 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x149638 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x149650 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149653 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14960a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x149617 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14961f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14962b | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x149633 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14963f | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x149647 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x149659 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x149667 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x149668 to 0x14971c.
  Established. The upper-range guard falls through when `[[ebp+16]+8] > 0xf (unsigned, 32-bit)`
  (`jbe` at 0x149679), reaching the jump (`jmp` at 0x14967b) to assignment `eax = 0xffffffff` (`mov`
  at 0x149714). Taking both range guards (`jbe` at 0x149679, `jae` at 0x149682) reaches the body.
  Byte I/O includes input at absolute `0x3da` (`in` at 0x149694), unknown output values (`out` at
  0x1496e7, `out` at 0x1496ea), and output value `0x20` (`out` at 0x1496f7). All output addresses
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14966d | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149670 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x149673 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x149689 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x149694 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x149699 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x1496a5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1496b8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1496cb | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1496de | store | 1 | `(unknown)` | `?` |
| `out` at 0x1496e7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x1496ea | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x1496f7 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x149679 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x149682 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14968e | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x1496e3 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x1496f3 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14970e | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1496ff | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14971c to 0x1497ac.
  Established. The four-byte load uses `[ebp+16] +0xc` (`mov` at 0x14973d). Three loop stores write
  two bytes to unknown effective addresses with unknown values (`mov` at 0x149759, `mov` at
  0x149773, `mov` at 0x14978e). The repeat edge is taken when `? <= ? (signed, 8-bit)` (`jle` at
  0x14979a). No destination extent or data representation is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x149720 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149723 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x149726 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14973d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x149740 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149759 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x149773 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14978e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14972c | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x149732 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14974a | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x149753 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x149764 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14976d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14977f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x149788 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14979a | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1497a2 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1497ac to 0x1497d4.
  Established. One byte input reads I/O absolute `0x3da` (`in` at 0x1497b2). Three byte outputs to
  I/O absolute `0x3c0` write `0x11` (`out` at 0x1497b9), unknown (`out` at 0x1497c8), and `0x20`
  (`out` at 0x1497cb). The intervening byte load has an unknown effective address (`mov` at
  0x1497c6). These sites leave hardware register meanings unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x1497b2 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x1497b9 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x1497ba | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x1497c6 | load | 1 | `(unknown)` | `-` |
| `out` at 0x1497c8 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x1497cb | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1497d0 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1497d4 to 0x149888.
  Established. The first branch takes the shared path when `[[ebp+16]] == 0x0` (`je` at 0x1497e7),
  reaching a four-byte load at `ebp +0x8` (`mov` at 0x1497fd); its fall-through calls unnamed target
  `0x14996d` (`call` at 0x1497e9), then jumps to restoration (`jmp` at 0x1497ee). Four shared byte
  outputs retain unknown I/O addresses and values (`out` at 0x14983a, `out` at 0x14983f, `out` at
  0x149842, `out` at 0x149845).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1497e1 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x1497e4 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x1497fd | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x149800 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x149804 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149807 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14980a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14981d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x149826 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149829 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14982c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14983a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14983f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149842 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149845 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x149862 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149865 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149868 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14986d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149872 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149875 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149878 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1497e9 | `(unknown callee) 0x14996d` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1497e7 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x149813 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x149819 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x149824 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14984a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14987d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14985a | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x149888 to 0x14996d.
  Established. The initial branch falls through when `[[ebp+16]] != 0x0` (`je` at 0x14989b),
  reaching an unnamed call to `0x1499b6` (`call` at 0x14989d) and restoration jump (`jmp` at
  0x1498a2). Taking that branch reaches the shared two-byte load at `[ebp+8] +0x16` (`mov` at
  0x1498b7). The shared byte inputs keep unknown I/O targets (`in` at 0x1498e2, `in` at 0x1498e5,
  `in` at 0x1498e8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x149895 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x149898 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x1498b4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x1498b7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x1498bb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x1498be | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x1498c1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x1498d4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x1498df | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x1498e2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x1498e5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x1498e8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x1498fa | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14990c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14991f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14993f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x149942 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x149945 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x149948 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14994f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x149956 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14995e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14989d | `(unknown callee) 0x1499b6` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14989b | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x1498ca | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x1498d0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x1498db | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x1498f5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x149907 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14991a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x149925 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x149964 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x149935 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x149a28 to 0x149a40.
  Established. Two byte outputs use I/O absolute `0x3c0`, with value `0x31` (`out` at 0x149a34) and
  an unknown value (`out` at 0x149a38). A four-byte load at `ebp +0x10` lies between them (`mov` at
  0x149a35). The prior byte input reads I/O absolute `0x3da` (`in` at 0x149a2d). Port field
  roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x149a2d | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x149a34 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x149a35 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x149a38 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x149a3c | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1497f0 to 0x149888.
  Established. Failing the upper-range guard falls through when `[[ebp+16]+8] > 0xff (unsigned,
  32-bit)` (`jbe` at 0x149813), reaching the jump (`jmp` at 0x149815) to assignment `eax =
  0xffffffff` (`mov` at 0x14985b). Arm selection follows following both range guards (`jbe` at
  0x149813, `jae` at 0x149819), with taken predicate `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0`
  (`jne` at 0x149824). Eight byte outputs retain unknown I/O addresses and values (`out` at
  0x14983a, `out` at 0x14983f, `out` at 0x149842, `out` at 0x149845, `out` at 0x14986d, `out` at
  0x149872, `out` at 0x149875, `out` at 0x149878).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1497fd | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x149800 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x149804 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x149807 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14980a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14981d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x149826 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149829 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14982c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14983a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14983f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149842 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149845 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x149862 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149865 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x149868 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14986d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149872 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149875 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x149878 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x149813 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x149819 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x149824 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14984a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14987d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14985a | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1498a7 to 0x149a28.
  Established. The arm-selection branch falls through when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) ==
  0x0` (`jne` at 0x1498db). The selected arm includes a byte output (`out` at 0x1498df), byte inputs
  (`in` at 0x1498e2, `in` at 0x1498e5, `in` at 0x1498e8), and two-byte stores (`mov` at 0x1498fa,
  `mov` at 0x14990c, `mov` at 0x14991f). Their effective targets remain unknown, along with output
  and stored values. The other arm stays separate.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1498b4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x1498b7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x1498bb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x1498be | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x1498c1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x1498d4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x1498df | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x1498e2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x1498e5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x1498e8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x1498fa | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14990c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14991f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14993f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x149942 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x149945 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x149948 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14994f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x149956 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14995e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x1498ca | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x1498d0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x1498db | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x1498f5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x149907 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14991a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x149925 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x149964 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x149935 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16dc40 to 0x16dcb0.
  Established. The `_sysOutByte` sites present candidate zero `0x3c8` at the first call (`call` at
  0x16dc4d) and `0x3c9` at the next three (`call` at 0x16dc69, `call` at 0x16dc85, `call` at
  0x16dca1). Each retains unknown candidate one. The repeat branches use taken condition `N == 0`
  (`jns` at 0x16dc5d, `jns` at 0x16dc79, `jns` at 0x16dc95), without establishing iteration counts.
  The exit reports `0x0` in `eax` (`ret` at 0x16dcab).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x16dc43 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16dc5f | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x16dc7b | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x16dc97 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16dc4d | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16dc69 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dc85 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dca1 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16dc5d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dc79 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dc95 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16dcab | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16dcb0 to 0x16dd30.
  Established. The `_sysOutByte` call has candidate zero `0x3c7` and unknown candidate one (`call`
  at 0x16dcc9). Three `_sysInByte` calls present candidate zero `0x3c9` (`call` at 0x16dce0, `call`
  at 0x16dcfc, `call` at 0x16dd18). Four-byte stores through `[sp+8] +0x0` (`mov` at 0x16dced),
  `[sp+12] +0x0` (`mov` at 0x16dd09), and `[sp+16] +0x0` (`mov` at 0x16dd22) retain unknown values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16dcb6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16dcb9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x16dcbc | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x16dcbf | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16dced | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x16dd09 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x16dd22 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16dcc9 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16dce0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dcfc | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dd18 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16dcd9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dcf5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dd11 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16dd2f | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16dd30 to 0x16ddc0.
  Established. The signed guard bypasses the body on its taken condition `0x0 >= [sp+8] (signed,
  32-bit)` (`jge` at 0x16dd52). Byte loads within the body keep unknown effective addresses (`movsx`
  at 0x16dd5f, `movsx` at 0x16dd7b, `movsx` at 0x16dd97); its three later `_sysOutByte` sites
  present candidate zero `0x3c9` and unknown candidate one (`call` at 0x16dd6b, `call` at 0x16dd87,
  `call` at 0x16dda3). The repeat left operand remains unknown (`jl` at 0x16ddae).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16dd36 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16dd39 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16dd3c | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16dd5f | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16dd7b | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16dd97 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16dd46 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16dd6b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dd87 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16dda3 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16dd52 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16dd5d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dd79 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16dd95 | `N == 0` | `not (N == 0)` |
| `jl` at 0x16ddae | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16ddbb | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16ddc0 to 0x16de60.
  Established. Entry into the body uses the guard fall-through `0x0 < [sp+8] (signed, 32-bit)`
  (`jge` at 0x16dde2). Three `_sysInByte` sites have candidate zero `0x3c9` (`call` at 0x16ddf4,
  `call` at 0x16de14, `call` at 0x16de34). Three four-byte stores retain unknown effective addresses
  and values (`mov` at 0x16de01, `mov` at 0x16de21, `mov` at 0x16de41). The final return expression
  stays unknown (`ret` at 0x16de56).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16ddc6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16ddc9 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16ddcc | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16de01 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16de21 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16de41 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16ddd6 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16ddf4 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16de14 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16de34 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16dde2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16dded | `N == 0` | `not (N == 0)` |
| `jns` at 0x16de0d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16de2d | `N == 0` | `not (N == 0)` |
| `jl` at 0x16de49 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16de56 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x156c8c to 0x156ce0.
  Established. Two four-byte loads use absolute `0x1ccf1c` (`mov` at 0x156cad) and `[0x1ccf1c] +0x0`
  (`mov` at 0x156cb3). The indirect call operand reads four bytes at `[[0x1ccf1c]] +0xcc` and
  presents leading candidates `[[0x1ccf1c]]`, `0x4`, and `sp-32` (`call` at 0x156cc0). The unnamed
  direct call to `0x160e80` has unknown candidate values (`call` at 0x156cd2). Callee effects remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x156c95 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x156c98 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x156c9b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x156c9e | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x156ca1 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x156ca4 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x156ca7 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x156caa | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x156cad | load | 4 | `abs 0x1ccf1c` | `-` |
| `mov` at 0x156cb3 | load | 4 | `[0x1ccf1c] +0x0` | `-` |
| `call` at 0x156cc0 | load | 4 | `[[0x1ccf1c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x156cc0 | `(unknown callee) (unknown target)` | `[[0x1ccf1c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x156cd2 | `(unknown callee) 0x160e80` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x156ccb | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x156cde | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x156c38 to 0x156c8c.
  Established. Four-byte local stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x156c44), `[sp+8]` to
  `sp -0x1c` (`mov` at 0x156c4a), `[sp+12]` to `sp -0x18` (`mov` at 0x156c50), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x156c56). The indirect call presents leading candidates `[[0x1ccf1c]]`, `0x3`,
  and `sp-32` (`call` at 0x156c6c); its later branch takes the edge when `Z == 1` (`je` at
  0x156c77), bypassing the unnamed direct call (`call` at 0x156c7e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x156c41 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x156c44 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x156c47 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x156c4a | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x156c4d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x156c50 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x156c53 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x156c56 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x156c59 | load | 4 | `abs 0x1ccf1c` | `-` |
| `mov` at 0x156c5f | load | 4 | `[0x1ccf1c] +0x0` | `-` |
| `call` at 0x156c6c | load | 4 | `[[0x1ccf1c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x156c6c | `(unknown callee) (unknown target)` | `[[0x1ccf1c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x156c7e | `(unknown callee) 0x160e80` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x156c77 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x156c8a | `eax` | `?` |
