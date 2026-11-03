# MMSC palette and port paths in IRIX 6.5.6

Read from `6.5.6/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5.6).
Image SHA-256: `f832451fd668051393e1dadc7de9dfda7fa6bf29cbdb1af991f8c5379c6fe752`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
The unnamed targets at `0x14eb6d` and `0x14ebb6` are included in the
`_mw8514RPal` listing but are not reached from that entry. The tables describe
reachable sites within each analysed span without propagating callee effects.
The decoder leaves one residual byte at `0x14ec3f` and one at `0x15bedf`.
Both were read directly as `0x00` after the respective returns.

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

[Fact sheets and full listings](disasm/palette-ports-656-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e800 to 0x14e868.
  Established. A four-byte load at `ebp +0x10` (`mov` at 0x14e802) precedes another at `[ebp+16]
  +0x4` (`mov` at 0x14e805). The first branch takes its edge when `Z == 0` (`jne` at 0x14e80a).
  Later, software interrupts carry immediate `0x10` (`int` at 0x14e84e, `int` at 0x14e861), without
  a proven service or result interpretation.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e802 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e805 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e80c | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e811 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14e824 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14e838 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14e850 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e853 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14e80a | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14e817 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e81f | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e82b | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e833 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14e83f | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14e847 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14e859 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e867 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e868 to 0x14e91c.
  Established. Both range guards must take their edges to reach the body: `[[ebp+16]+8] <= 0xf
  (unsigned, 32-bit)` (`jbe` at 0x14e879) and `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)`
  (`jae` at 0x14e882). The body includes a byte input at I/O absolute `0x3da` (`in` at 0x14e894).
  Byte outputs retain unknown I/O targets, with unknown values (`out` at 0x14e8e7, `out` at
  0x14e8ea) and value `0x20` (`out` at 0x14e8f7).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e86d | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e870 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e873 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e889 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14e894 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14e899 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e8a5 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8b8 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8cb | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e8de | store | 1 | `(unknown)` | `?` |
| `out` at 0x14e8e7 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e8ea | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14e8f7 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e879 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e882 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e88e | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14e8e3 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14e8f3 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14e90e | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e8ff | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e91c to 0x14e9ac.
  Established. The four-byte load uses `[ebp+16] +0xc` (`mov` at 0x14e93d). Three two-byte
  stores have unknown effective targets and values (`mov` at 0x14e959, `mov` at 0x14e973, `mov` at
  0x14e98e). The loop branch takes its edge when `? <= ? (signed, 8-bit)` (`jle` at 0x14e99a).
  Neither operand nor the destination extent is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e920 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e923 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14e926 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14e93d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14e940 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14e959 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e973 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14e98e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14e92c | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14e932 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14e94a | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e953 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e964 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e96d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e97f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14e988 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14e99a | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e9a2 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9ac to 0x14e9d4.
  Established. The byte input at I/O absolute `0x3da` (`in` at 0x14e9b2) precedes three byte outputs
  at I/O absolute `0x3c0`. Their values are `0x11` (`out` at 0x14e9b9), unknown (`out` at 0x14e9c8),
  and `0x20` (`out` at 0x14e9cb). An intervening byte load retains an unknown effective address
  (`mov` at 0x14e9c6). No port register role is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14e9b2 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14e9b9 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14e9ba | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14e9c6 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14e9c8 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14e9cb | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14e9d0 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9d4 to 0x14ea88.
  Established. The first branch takes the shared path when `[[ebp+16]] == 0x0` (`je` at 0x14e9e7).
  That path starts with a four-byte load at `ebp +0x8` (`mov` at 0x14e9fd). The fall-through instead
  calls unnamed target `0x14eb6d` (`call` at 0x14e9e9) and jumps to restoration (`jmp` at 0x14e9ee).
  Four shared byte outputs retain unknown I/O targets and values (`out` at 0x14ea3a, `out` at
  0x14ea3f, `out` at 0x14ea42, `out` at 0x14ea45).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e9e1 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14e9e4 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14e9fd | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ea00 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ea04 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ea07 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ea0a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea1d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea26 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea29 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea2c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea3a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea3f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea42 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea45 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea62 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea65 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea68 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea6d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea72 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea75 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea78 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14e9e9 | `(unknown callee) 0x14eb6d` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14e9e7 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14ea13 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea19 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea24 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea4a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea7d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea5a | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea88 to 0x14eb6d.
  Established. A fall-through with `[[ebp+16]] != 0x0` (`je` at 0x14ea9b) reaches the unnamed direct
  call to `0x14ebb6` (`call` at 0x14ea9d) and restoration jump (`jmp` at 0x14eaa2). Taking that
  branch reaches a shared two-byte load at `[ebp+8] +0x16` (`mov` at 0x14eab7). The shared byte
  input targets remain unknown (`in` at 0x14eae2, `in` at 0x14eae5, `in` at 0x14eae8).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14ea95 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14ea98 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14eab4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eab7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eabb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eabe | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14eac1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ead4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eadf | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eae2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eae5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eae8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eafa | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb0c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb1f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb3f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb42 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb45 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb48 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb4f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb56 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb5e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14ea9d | `(unknown callee) 0x14ebb6` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14ea9b | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14eaca | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ead0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eadb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eaf5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb07 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb1a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb25 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb64 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb35 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ec28 to 0x14ec40.
  Established. Byte outputs at I/O absolute `0x3c0` write `0x31` (`out` at 0x14ec34) and an unknown
  value (`out` at 0x14ec38). A four-byte load at `ebp +0x10` separates them (`mov` at 0x14ec35). The
  byte input before those outputs reads I/O absolute `0x3da` (`in` at 0x14ec2d). Their hardware
  meanings remain unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14ec2d | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14ec34 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14ec35 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14ec38 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ec3c | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9f0 to 0x14ea88.
  Established. The failed upper-range guard falls through when `[[ebp+16]+8] > 0xff (unsigned,
  32-bit)` (`jbe` at 0x14ea13) and reaches a jump (`jmp` at 0x14ea15) to the assignment
  `eax = 0xffffffff` (`mov` at 0x14ea5b).
  Arm selection follows following both range guards (`jbe` at 0x14ea13, `jae` at 0x14ea19); its taken
  predicate is `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14ea24). Eight byte outputs
  keep unknown I/O addresses and values (`out` at 0x14ea3a, `out` at 0x14ea3f, `out` at 0x14ea42,
  `out` at 0x14ea45, `out` at 0x14ea6d, `out` at 0x14ea72, `out` at 0x14ea75, `out` at 0x14ea78).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14e9fd | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14ea00 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14ea04 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14ea07 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14ea0a | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ea1d | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14ea26 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea29 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea2c | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea3a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea3f | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea42 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea45 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14ea62 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea65 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14ea68 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14ea6d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea72 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea75 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14ea78 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14ea13 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ea19 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14ea24 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14ea4a | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14ea7d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14ea5a | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14eaa7 to 0x14ec28.
  Established. The selected fall-through predicate is `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0`
  (`jne` at 0x14eadb). That arm includes a byte output (`out` at 0x14eadf), three byte inputs (`in`
  at 0x14eae2, `in` at 0x14eae5, `in` at 0x14eae8), and two-byte stores (`mov` at 0x14eafa, `mov` at
  0x14eb0c, `mov` at 0x14eb1f). Their effective targets remain unknown, as do output and stored
  values. The alternative arm is retained separately.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14eab4 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14eab7 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14eabb | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14eabe | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14eac1 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14ead4 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14eadf | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eae2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eae5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eae8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eafa | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb0c | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb1f | store | 2 | `(unknown)` | `?` |
| `out` at 0x14eb3f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14eb42 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb45 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14eb48 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14eb4f | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb56 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14eb5e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14eaca | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14ead0 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14eadb | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14eaf5 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb07 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14eb1a | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14eb25 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14eb64 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14eb35 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172e40 to 0x172eb0.
  Established. The first `_sysOutByte` call presents candidate zero `0x3c8` (`call` at 0x172e4d). Later calls present `0x3c9` in that slot (`call` at 0x172e69, `call` at 0x172e85, `call` at
  0x172ea1). Candidate one remains unknown. Three repeat branches use taken condition `N == 0`
  (`jns` at 0x172e5d, `jns` at 0x172e79, `jns` at 0x172e95), without establishing counts. The exit
  reports `0x0` in `eax` (`ret` at 0x172eab).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x172e43 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172e5f | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x172e7b | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x172e97 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172e4d | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x172e69 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172e85 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172ea1 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172e5d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172e79 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172e95 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172eab | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172eb0 to 0x172f30.
  Established. The `_sysOutByte` site has candidate zero `0x3c7` and unknown candidate one (`call`
  at 0x172ec9). Three `_sysInByte` sites present candidate zero `0x3c9` (`call` at 0x172ee0, `call`
  at 0x172efc, `call` at 0x172f18). Four-byte stores at `[sp+8] +0x0` (`mov` at 0x172eed), `[sp+12]
  +0x0` (`mov` at 0x172f09), and `[sp+16] +0x0` (`mov` at 0x172f22) preserve unknown values.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172eb6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172eb9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x172ebc | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x172ebf | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x172eed | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x172f09 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x172f22 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172ec9 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172ee0 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172efc | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f18 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x172ed9 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172ef5 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f11 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172f2f | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172f30 to 0x172fc0.
  Established. The loop guard bypasses the body when `0x0 >= [sp+8] (signed, 32-bit)` on its taken
  edge (`jge` at 0x172f52). The body includes three byte loads at unknown effective addresses
  (`movsx` at 0x172f5f, `movsx` at 0x172f7b, `movsx` at 0x172f97). Later `_sysOutByte` calls present
  candidate zero `0x3c9` with unknown candidate one (`call` at 0x172f6b, `call` at 0x172f87, `call`
  at 0x172fa3). The repeat comparison keeps its left operand unknown (`jl` at 0x172fae).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172f36 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172f39 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172f3c | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x172f5f | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172f7b | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x172f97 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172f46 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172f6b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172f87 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172fa3 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172f52 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172f5d | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f79 | `N == 0` | `not (N == 0)` |
| `jns` at 0x172f95 | `N == 0` | `not (N == 0)` |
| `jl` at 0x172fae | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172fbb | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172fc0 to 0x173060.
  Established. The initial signed guard enters the body on fall-through when `0x0 < [sp+8] (signed,
  32-bit)` (`jge` at 0x172fe2). The three `_sysInByte` sites have candidate zero `0x3c9` (`call` at
  0x172ff4, `call` at 0x173014, `call` at 0x173034). Three four-byte stores have unknown addresses
  and values (`mov` at 0x173001, `mov` at 0x173021, `mov` at 0x173041). The return value remains
  unknown (`ret` at 0x173056).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172fc6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x172fc9 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x172fcc | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x173001 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173021 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x173041 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172fd6 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x172ff4 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173014 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x173034 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x172fe2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x172fed | `N == 0` | `not (N == 0)` |
| `jns` at 0x17300d | `N == 0` | `not (N == 0)` |
| `jns` at 0x17302d | `N == 0` | `not (N == 0)` |
| `jl` at 0x173049 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x173056 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be8c to 0x15bee0.
  Established. The pointer chain includes four-byte loads at absolute `0x1d322c` (`mov` at 0x15bead)
  and `[0x1d322c] +0x0` (`mov` at 0x15beb3). The indirect call operand loads four bytes at
  `[[0x1d322c]] +0xcc` and presents leading candidates `[[0x1d322c]]`, `0x4`, and `sp-32` (`call` at
  0x15bec0). The unnamed direct call to `0x166080` has unknown candidate values (`call` at
  0x15bed2). Neither callee effect is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15be95 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15be98 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15be9b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15be9e | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15bea1 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15bea4 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15bea7 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15beaa | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15bead | load | 4 | `abs 0x1d322c` | `-` |
| `mov` at 0x15beb3 | load | 4 | `[0x1d322c] +0x0` | `-` |
| `call` at 0x15bec0 | load | 4 | `[[0x1d322c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15bec0 | `(unknown callee) (unknown target)` | `[[0x1d322c]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15bed2 | `(unknown callee) 0x166080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15becb | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15bede | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.5.6/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be38 to 0x15be8c.
  Established. Four-byte local stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x15be44), `[sp+8]` to
  `sp -0x1c` (`mov` at 0x15be4a), `[sp+12]` to `sp -0x18` (`mov` at 0x15be50), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x15be56). The indirect call has leading candidates `[[0x1d322c]]`, `0x3`, and
  `sp-32` (`call` at 0x15be6c); its later branch takes the edge when `Z == 1` (`je` at 0x15be77),
  bypassing the unnamed direct call (`call` at 0x15be7e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x15be41 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x15be44 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15be47 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15be4a | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x15be4d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x15be50 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x15be53 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15be56 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15be59 | load | 4 | `abs 0x1d322c` | `-` |
| `mov` at 0x15be5f | load | 4 | `[0x1d322c] +0x0` | `-` |
| `call` at 0x15be6c | load | 4 | `[[0x1d322c]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15be6c | `(unknown callee) (unknown target)` | `[[0x1d322c]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15be7e | `(unknown callee) 0x166080` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15be77 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15be8a | `eax` | `?` |
