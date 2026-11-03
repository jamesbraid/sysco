# MMSC palette and port paths in IRIX 6.5.5

Read from `6.5.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.5).
Image SHA-256: `18f3b67215ca20c15e41c100df7fb87280790b475f3b4081c9ffcdc141863ec9`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. The decoder leaves one raw zero
byte at `0x14ec2f` and another at `0x15becf`. The unnamed targets `0x14eb5d` and
`0x14eba6` appear in the `_mw8514RPal` listing but are not reached from that entry.
Their effects are not attributed to the callers. Statements below concern sites
reached by known control-flow edges.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack locations,
and omit implicit push/pop accesses. The full facts retain those accesses.
Call and return instructions remain in the listings without separate implicit stack
access rows. Unsupported instructions, partial-register operations and self-subtraction
can leave values or flags unknown in the analysis. These limits do not establish
indeterminate firmware behavior.
Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/palette-ports-655-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e7f0 to 0x14e858.
  Established. The four-byte pointer load uses `ebp +0x10` (`mov` at 0x14e7f2). A second four-byte
  load uses `[ebp+16] +0x4` (`mov` at 0x14e7f5), followed by the taken predicate `Z == 0` (`jne` at
  0x14e7fa). Two software interrupts use immediate `0x10` (`int` at 0x14e83e, `int` at 0x14e851).
  Their service meanings and replies remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e7f2 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e7f5 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e7fc | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e801 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14e814 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14e828 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14e840 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e843 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14e7fa | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14e807 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e80f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e81b | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e823 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e82f | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e837 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14e849 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e857 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e858 to 0x14e90c.
  Established. The guard falls through when `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` (`jbe` at
  0x14e869), reaching the jump to the error assignment (`jmp` at 0x14e86b). Taking both range
  guards (`jbe` at 0x14e869, `jae` at 0x14e872) instead reaches the body, which includes a byte
  input at I/O absolute `0x3da` (`in` at 0x14e884). The byte output values are unknown (`out` at
  0x14e8d7, `out` at 0x14e8da) and `0x20` (`out` at 0x14e8e7), all at unknown I/O addresses.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e85d | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e860 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e863 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e879 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14e884 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14e889 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e895 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8a8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8bb | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8ce | store | 1 | `(unknown)` | `?` |
| `out` at 0x14e8d7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e8da | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e8e7 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e869 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e872 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e87e | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14e8d3 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14e8e3 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14e8fe | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e8ef | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e90c to 0x14e99c.
  Established. The output stores in the loop are two-byte accesses at unknown effective addresses,
  with unknown values (`mov` at 0x14e949, `mov` at 0x14e963, `mov` at 0x14e97e). The earlier
  four-byte pointer load uses `[ebp+16] +0xc` (`mov` at 0x14e92d). A signed eight-bit repeat
  condition remains `? <= ?` on its taken edge (`jle` at 0x14e98a). No destination span or component
  encoding is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e910 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e913 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e916 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e92d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e930 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e949 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e963 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e97e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e91c | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e922 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e93a | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e943 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e954 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e95d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e96f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e978 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14e98a | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e992 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e99c to 0x14e9c4.
  Established. The byte input reads I/O absolute `0x3da` (`in` at 0x14e9a2). Byte outputs at I/O
  absolute `0x3c0` carry `0x11` (`out` at 0x14e9a9), unknown (`out` at 0x14e9b8), and `0x20` (`out`
  at 0x14e9bb). A byte load at an unknown effective address precedes the middle output (`mov` at
  0x14e9b6). The sequence does not establish port register roles.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14e9a2 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14e9a9 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14e9aa | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e9b6 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14e9b8 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14e9bb | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e9c0 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9c4 to 0x14ea78.
  Established. The zero predicate `[[ebp+16]] == 0x0` takes the branch into the shared continuation
  (`je` at 0x14e9d7), whose first four-byte load uses `ebp +0x8` (`mov` at 0x14e9ed); its
  fall-through calls unnamed target `0x14eb5d` (`call` at 0x14e9d9) and jumps to restoration (`jmp`
  at 0x14e9de). The shared byte outputs keep unknown I/O addresses and values (`out` at 0x14ea2a,
  `out` at 0x14ea2f, `out` at 0x14ea32, `out` at 0x14ea35).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e9d1 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14e9d4 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14e9ed | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14e9f0 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14e9f4 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e9f7 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e9fa | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea0d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea16 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea19 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea1c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea2a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea2f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea32 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea35 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea52 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea55 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea58 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea5d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea62 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea65 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea68 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14e9d9 | `(unknown callee) 0x14eb5d` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14e9d7 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14ea03 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea09 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea14 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea3a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea6d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea4a | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea78 to 0x14eb5d.
  Established. The first branch falls through when `[[ebp+16]] != 0x0` (`je` at 0x14ea8b), reaching
  the unnamed call to `0x14eba6` (`call` at 0x14ea8d) and a jump to restoration (`jmp` at 0x14ea92).
  Taking that branch reaches a shared two-byte load at `[ebp+8] +0x16` (`mov` at 0x14eaa7). The byte
  input addresses on the shared path remain unknown (`in` at 0x14ead2, `in` at 0x14ead5, `in` at
  0x14ead8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ea85 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ea88 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14eaa4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eaa7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eaab | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eaae | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14eab1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14eac4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eacf | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ead2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ead5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ead8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eaea | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eafc | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb0f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb2f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb32 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb35 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb38 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb3f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb46 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb4e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ea8d | `(unknown callee) 0x14eba6` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ea8b | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14eaba | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14eac0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eacb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eae5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eaf7 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb0a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb54 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb25 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ec18 to 0x14ec30.
  Established. The two byte outputs use I/O absolute `0x3c0`, with value `0x31` at the first (`out`
  at 0x14ec24) and unknown value at the second (`out` at 0x14ec28). A four-byte access at `ebp
  +0x10` separates them (`mov` at 0x14ec25). The prior byte input uses I/O absolute `0x3da`
  (`in` at 0x14ec1d). Their hardware meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14ec1d | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14ec24 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14ec25 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14ec28 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ec2c | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9e0 to 0x14ea78.
  Established. The upper-range guard takes its branch when `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)`
  (`jbe` at 0x14ea03). Arm selection follows the second guard following its edge when `[[ebp+16]+8] >=
  [[ebp+16]+4] (unsigned, 32-bit)` (`jae` at 0x14ea09). The selection branch tests `(([[ebp+8]+22]:2
  & 0xffff) & 0x100) != 0x0` on its taken edge (`jne` at 0x14ea14). Eight byte outputs retain
  unknown I/O targets and values (`out` at 0x14ea2a, `out` at 0x14ea2f, `out` at 0x14ea32, `out` at
  0x14ea35, `out` at 0x14ea5d, `out` at 0x14ea62, `out` at 0x14ea65, `out` at 0x14ea68).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e9ed | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14e9f0 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14e9f4 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e9f7 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e9fa | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea0d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea16 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea19 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea1c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea2a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea2f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea32 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea35 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea52 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea55 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea58 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea5d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea62 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea65 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea68 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ea03 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea09 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea14 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea3a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea6d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea4a | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea97 to 0x14ec18.
  Established. The arm-selection branch falls through when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) ==
  0x0` (`jne` at 0x14eacb). That arm includes the byte output (`out` at 0x14eacf), three byte inputs
  (`in` at 0x14ead2, `in` at 0x14ead5, `in` at 0x14ead8), and two-byte stores (`mov` at 0x14eaea,
  `mov` at 0x14eafc, `mov` at 0x14eb0f). Their effective addresses remain unknown. The output and
  stored values are also unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14eaa4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eaa7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eaab | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eaae | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14eab1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14eac4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eacf | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14ead2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ead5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14ead8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eaea | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eafc | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb0f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb2f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb32 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb35 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb38 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb3f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb46 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb4e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14eaba | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14eac0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eacb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eae5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eaf7 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb0a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb15 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb54 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb25 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172e30 to 0x172ea0.
  Established. The first `_sysOutByte` site presents candidate zero `0x3c8` (`call` at 0x172e3d).
  Later sites present `0x3c9` in that slot (`call` at 0x172e59, `call` at 0x172e75, `call` at
  0x172e91), while candidate one remains unknown. The first repeat branch uses the taken condition
  `N == 0` (`jns` at 0x172e4d); its final exit carries `0x0` in `eax` (`ret` at 0x172e9b).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x172e33 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172e4f | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x172e6b | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x172e87 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172e3d | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x172e59 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172e75 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172e91 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172e4d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172e69 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172e85 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172e9b | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172ea0 to 0x172f20.
  Established. The initial `_sysOutByte` call retains candidate zero `0x3c7` and unknown candidate
  one (`call` at 0x172eb9). Three `_sysInByte` calls each present candidate zero `0x3c9` (`call` at
  0x172ed0, `call` at 0x172eec, `call` at 0x172f08). Four-byte stores at `[sp+8] +0x0` (`mov` at
  0x172edd), `[sp+12] +0x0` (`mov` at 0x172ef9), and `[sp+16] +0x0` (`mov` at 0x172f12) have unknown
  values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172ea6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172ea9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x172eac | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x172eaf | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x172edd | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x172ef9 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x172f12 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172eb9 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172ed0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172eec | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f08 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172ec9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172ee5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f01 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172f1f | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172f20 to 0x172fb0.
  Established. The initial signed guard bypasses the body when its taken condition is `0x0 >= [sp+8]
  (signed, 32-bit)` (`jge` at 0x172f42). The body contains three unknown-address byte loads (`movsx`
  at 0x172f4f, `movsx` at 0x172f6b, `movsx` at 0x172f87); its three `_sysOutByte` sites have
  candidate zero `0x3c9` and unknown candidate one (`call` at 0x172f5b, `call` at 0x172f77, `call`
  at 0x172f93). The loop repeat condition keeps an unknown left operand (`jl` at 0x172f9e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172f26 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172f29 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172f2c | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172f4f | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172f6b | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172f87 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172f36 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172f5b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f77 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f93 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172f42 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172f4d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f69 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f85 | `N == 0` | `not (N == 0)` |
| `jl` at 0x172f9e | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172fab | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172fb0 to 0x173050.
  Established. Entry into the body follows the guard fall-through `0x0 < [sp+8] (signed, 32-bit)`
  (`jge` at 0x172fd2). It includes three `_sysInByte` calls with candidate zero `0x3c9` (`call` at
  0x172fe4, `call` at 0x173004, `call` at 0x173024). The three four-byte stores keep unknown
  effective addresses and values (`mov` at 0x172ff1, `mov` at 0x173011, `mov` at 0x173031). The
  return expression remains unknown (`ret` at 0x173046).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172fb6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172fb9 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172fbc | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x172ff1 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173011 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173031 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172fc6 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172fe4 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173004 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173024 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172fd2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172fdd | `N == 0` | `not (N == 0)` |
| `jns` at 0x172ffd | `N == 0` | `not (N == 0)` |
| `jns` at 0x17301d | `N == 0` | `not (N == 0)` |
| `jl` at 0x173039 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173046 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be7c to 0x15bed0.
  Established. A four-byte load from absolute `0x1d321c` (`mov` at 0x15be9d) precedes another at
  `[0x1d321c] +0x0` (`mov` at 0x15bea3). The indirect call operand loads four bytes from
  `[[0x1d321c]] +0xcc` and has leading candidates `[[0x1d321c]]`, `0x4`, and `sp-32` (`call` at
  0x15beb0); its target operation and the later unnamed direct call effect remain unknown (`call` at
  0x15bec2).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15be85 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15be88 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15be8b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15be8e | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15be91 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15be94 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15be97 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15be9a | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15be9d | load | 4 | `abs 0x1d321c` | `-` |
| `mov` at 0x15bea3 | load | 4 | `[0x1d321c] +0x0` | `-` |
| `call` at 0x15beb0 | load | 4 | `[[0x1d321c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15beb0 | `(unknown callee) (unknown target)` | `[[0x1d321c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15bec2 | `(unknown callee) 0x166070` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15bebb | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15bece | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.5.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be28 to 0x15be7c.
  Established. Four-byte stores copy entry stack values to local locations: `[sp+4]` to `sp -0x20`
  (`mov` at 0x15be34), `[sp+8]` to `sp -0x1c` (`mov` at 0x15be3a), `[sp+12]` to `sp -0x18` (`mov` at
  0x15be40), and `[sp+16]` to `sp -0x14` (`mov` at 0x15be46). The indirect call presents leading
  candidates `[[0x1d321c]]`, `0x3`, and `sp-32` (`call` at 0x15be5c). The branch taken condition `Z
  == 1` (`je` at 0x15be67) bypasses the unnamed direct call (`call` at 0x15be6e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15be31 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15be34 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15be37 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15be3a | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15be3d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15be40 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15be43 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15be46 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15be49 | load | 4 | `abs 0x1d321c` | `-` |
| `mov` at 0x15be4f | load | 4 | `[0x1d321c] +0x0` | `-` |
| `call` at 0x15be5c | load | 4 | `[[0x1d321c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15be5c | `(unknown callee) (unknown target)` | `[[0x1d321c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15be6e | `(unknown callee) 0x166070` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15be67 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15be7a | `eax` | `?` |
