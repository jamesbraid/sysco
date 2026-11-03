# MMSC palette and port paths in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
This image has the x86 a.out link address `0x108000`. The [load map](load-map.md) explains segment mapping.
Most analysed spans end at the next text symbol, with exclusive inferred ends.
The `_mwVGAHWPal` and `_mwVGAHRPal` spans extend through shared continuations.
Those two given extents overlap the following entry rather than ending at its symbol.
They can include literals or padding after returns. The unnamed bodies at
`0x14a40d` and `0x14a456` lie in the `_mw8514RPal` listing but are not reached
from that entry. The caller entries establish call targets and candidates,
with callee effects unknown. Statements below concern sites reached by known
control-flow edges.
The decoder leaves one byte at `0x14a4df` and one at `0x15777f` undecoded.
Both bytes are `0x00` and follow returns.

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

[Fact sheets and full listings](disasm/palette-ports-64-facts.txt) include all tool sections.

## `_mwCGAPal`

- `_mwCGAPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a0a0 to 0x14a108.
  Established. Four-byte loads read `ebp +0x10` (`mov` at 0x14a0a2) and `[ebp+16] +0x4` (`mov` at
  0x14a0a5). The first branch is taken when `Z == 0` (`jne` at 0x14a0aa). Both software-interrupt
  sites have immediate `0x10` (`int` at 0x14a0ee, `int` at 0x14a101). Service meanings and replies
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a0a2 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a0a5 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a0ac | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a0b1 | load | 2 | `[[ebp+16]+12] +0x0` | `-` |
| `mov` at 0x14a0c4 | load | 2 | `[[ebp+16]+12] +0x2` | `-` |
| `mov` at 0x14a0d8 | load | 2 | `[[ebp+16]+12] +0x4` | `-` |
| `mov` at 0x14a0f0 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a0f3 | load | 4 | `[ebp+16] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x14a0aa | `Z == 0` | `not (Z == 0)` |
| `je` at 0x14a0b7 | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a0bf | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14a0cb | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a0d3 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x14a0df | `(? & 0x80) == 0x0` | `(? & 0x80) != 0x0` |
| `je` at 0x14a0e7 | `Z == 1` | `not (Z == 1)` |
| `jne` at 0x14a0f9 | `Z == 0` | `not (Z == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a107 | `eax` | `?` |

## `_mwEGAWPal`

- `_mwEGAWPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a108 to 0x14a1bc.
  Established. The upper-range guard takes its branch when `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)`
  (`jbe` at 0x14a119). Reaching the body also requires the next guard following its edge when
  `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` (`jae` at 0x14a122). Byte I/O includes input at
  absolute `0x3da` (`in` at 0x14a134), unknown output values (`out` at 0x14a187, `out` at 0x14a18a),
  and output value `0x20` (`out` at 0x14a197). All three output targets remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a10d | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a110 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a113 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a129 | load | 1 | `[ebp+16] +0x0` | `-` |
| `in` at 0x14a134 | load | 1 | `I/O abs 0x3da` | `-` |
| `mov` at 0x14a139 | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a145 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a158 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a16b | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a17e | store | 1 | `(unknown)` | `?` |
| `out` at 0x14a187 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a18a | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a197 | store | 1 | `I/O (unknown)` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14a119 | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14a122 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a12e | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jne` at 0x14a183 | `([[ebp+16]]:1 & 0xff) != 0x0` | `([[ebp+16]]:1 & 0xff) == 0x0` |
| `jle` at 0x14a193 | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |
| `jle` at 0x14a1ae | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a19f | `eax` | `?` |

## `_mwEGARPal`

- `_mwEGARPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a1bc to 0x14a24c.
  Established. The loop includes three two-byte stores with unknown effective addresses and values
  (`mov` at 0x14a1f9, `mov` at 0x14a213, `mov` at 0x14a22e); its prior four-byte load
  uses `[ebp+16] +0xc` (`mov` at 0x14a1dd). The repeat branch has taken condition `? <= ? (signed,
  8-bit)` (`jle` at 0x14a23a), leaving both operands and the extent unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a1c0 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a1c3 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a1c6 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a1dd | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a1e0 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a1f9 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a213 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a22e | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14a1cc | `[[ebp+16]+8] <= 0xf (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xf (unsigned, 32-bit)` |
| `jae` at 0x14a1d2 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a1ea | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14a1f3 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14a204 | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14a20d | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14a21f | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x14a228 | `Z == 0` | `not (Z == 0)` |
| `jle` at 0x14a23a | `? <= ? (signed, 8-bit)` | `? > ? (signed, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a242 | `eax` | `?` |

## `_mwEGAOscan`

- `_mwEGAOscan`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a24c to 0x14a274.
  Established. A byte input reads I/O absolute `0x3da` (`in` at 0x14a252). The following byte
  outputs to I/O absolute `0x3c0` write `0x11` (`out` at 0x14a259), unknown (`out` at 0x14a268), and
  `0x20` (`out` at 0x14a26b). The middle value follows a byte load at an unknown effective address
  (`mov` at 0x14a266). Hardware register roles remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14a252 | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14a259 | store | 1 | `I/O abs 0x3c0` | `0x11` |
| `mov` at 0x14a25a | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a266 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14a268 | store | 1 | `I/O abs 0x3c0` | `?` |
| `out` at 0x14a26b | store | 1 | `I/O abs 0x3c0` | `0x20` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a270 | `eax` | `?` |

## `_mwVGAHWPal`

- `_mwVGAHWPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a274 to 0x14a328.
  Established. The first branch enters the shared continuation when `[[ebp+16]] == 0x0` on its taken
  edge (`je` at 0x14a287). The continuation loads four bytes at `ebp +0x8` (`mov` at 0x14a29d). The
  branch fall-through instead calls unnamed target `0x14a40d` (`call` at 0x14a289) and jumps to
  restoration (`jmp` at 0x14a28e). Shared byte output sites retain unknown I/O addresses and values
  (`out` at 0x14a2da, `out` at 0x14a2df, `out` at 0x14a2e2, `out` at 0x14a2e5).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a281 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14a284 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14a29d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14a2a0 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14a2a4 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a2a7 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a2aa | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a2bd | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a2c6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a2c9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a2cc | load | 1 | `(unknown)` | `-` |
| `out` at 0x14a2da | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2df | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2e2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2e5 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14a302 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a305 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a308 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14a30d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a312 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a315 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a318 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14a289 | `(unknown callee) 0x14a40d` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14a287 | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14a2b3 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14a2b9 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a2c4 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14a2ea | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14a31d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a2fa | `eax` | `?` |

## `_mwVGAHRPal`

- `_mwVGAHRPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a328 to 0x14a40d.
  Established. Falling through the initial branch means `[[ebp+16]] != 0x0` (`je` at 0x14a33b),
  reaching the unnamed call to `0x14a456` (`call` at 0x14a33d) and restoration jump (`jmp` at
  0x14a342). Taking that branch reaches the shared two-byte load at `[ebp+8] +0x16` (`mov` at
  0x14a357). The shared byte inputs have unknown I/O targets (`in` at 0x14a382, `in` at 0x14a385,
  `in` at 0x14a388).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a335 | load | 4 | `ebp +0x10` | `-` |
| `cmp` at 0x14a338 | load | 4 | `[ebp+16] +0x0` | `-` |
| `mov` at 0x14a354 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14a357 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14a35b | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a35e | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a361 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a374 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14a37f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14a382 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a385 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a388 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14a39a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3ac | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3bf | store | 2 | `(unknown)` | `?` |
| `out` at 0x14a3df | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14a3e2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a3e5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a3e8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14a3ef | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3f6 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3fe | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x14a33d | `(unknown callee) 0x14a456` | `edi` | `esi` | `edx` | `ecx` | `ebx` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x14a33b | `[[ebp+16]] == 0x0` | `[[ebp+16]] != 0x0` |
| `jbe` at 0x14a36a | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14a370 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a37b | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14a395 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14a3a7 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14a3ba | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14a3c5 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14a404 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a3d5 | `eax` | `?` |

## `_mwVGAOscan`

- `_mwVGAOscan`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a4c8 to 0x14a4e0.
  Established. A byte output to I/O absolute `0x3c0` writes `0x31` (`out` at 0x14a4d4), followed by
  another with unknown value (`out` at 0x14a4d8). A four-byte load at `ebp +0x10` lies between them
  (`mov` at 0x14a4d5). The earlier byte input reads I/O absolute `0x3da` (`in` at 0x14a4cd). No port
  meaning is inferred from the routine name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x14a4cd | load | 1 | `I/O abs 0x3da` | `-` |
| `out` at 0x14a4d4 | store | 1 | `I/O abs 0x3c0` | `0x31` |
| `mov` at 0x14a4d5 | load | 4 | `ebp +0x10` | `-` |
| `out` at 0x14a4d8 | store | 1 | `I/O abs 0x3c0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a4dc | `eax` | `?` |

## `_mw8514WPal`

- `_mw8514WPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a290 to 0x14a328.
  Established. Failing the upper-range guard falls through when `[[ebp+16]+8] > 0xff (unsigned,
  32-bit)` (`jbe` at 0x14a2b3), reaching a jump (`jmp` at 0x14a2b5) to the
  assignment of `0xffffffff` (`mov` at 0x14a2fb). Arm selection follows following both range guards (`jbe` at 0x14a2b3, `jae` at 0x14a2b9); its branch is
  taken when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14a2c4). Both arms contain
  byte outputs at unknown I/O addresses with unknown values (`out` at 0x14a2da, `out` at 0x14a2df,
  `out` at 0x14a2e2, `out` at 0x14a2e5, `out` at 0x14a30d, `out` at 0x14a312, `out` at 0x14a315,
  `out` at 0x14a318).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a29d | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14a2a0 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14a2a4 | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a2a7 | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a2aa | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a2bd | load | 4 | `[ebp+16] +0xc` | `-` |
| `mov` at 0x14a2c6 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a2c9 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a2cc | load | 1 | `(unknown)` | `-` |
| `out` at 0x14a2da | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2df | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2e2 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a2e5 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x14a302 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a305 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x14a308 | load | 1 | `(unknown)` | `-` |
| `out` at 0x14a30d | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a312 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a315 | store | 1 | `I/O (unknown)` | `?` |
| `out` at 0x14a318 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14a2b3 | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14a2b9 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a2c4 | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jae` at 0x14a2ea | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14a31d | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a2fa | `eax` | `?` |

## `_mw8514RPal`

- `_mw8514RPal`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14a347 to 0x14a4c8.
  Established. The arm-selection fall-through is `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` (`jne`
  at 0x14a37b). It includes a byte output (`out` at 0x14a37f), byte inputs (`in` at 0x14a382, `in`
  at 0x14a385, `in` at 0x14a388), and two-byte stores (`mov` at 0x14a39a, `mov` at 0x14a3ac, `mov`
  at 0x14a3bf). Effective targets and output/store values remain unknown. The other arm is
  represented separately in the tables.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x14a354 | load | 4 | `ebp +0x8` | `-` |
| `mov` at 0x14a357 | load | 2 | `[ebp+8] +0x16` | `-` |
| `mov` at 0x14a35b | load | 4 | `ebp +0x10` | `-` |
| `mov` at 0x14a35e | load | 4 | `[ebp+16] +0x4` | `-` |
| `mov` at 0x14a361 | load | 4 | `[ebp+16] +0x8` | `-` |
| `mov` at 0x14a374 | load | 4 | `[ebp+16] +0xc` | `-` |
| `out` at 0x14a37f | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14a382 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a385 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a388 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14a39a | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3ac | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3bf | store | 2 | `(unknown)` | `?` |
| `out` at 0x14a3df | store | 1 | `I/O (unknown)` | `?` |
| `in` at 0x14a3e2 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a3e5 | load | 1 | `I/O (unknown)` | `-` |
| `in` at 0x14a3e8 | load | 1 | `I/O (unknown)` | `-` |
| `mov` at 0x14a3ef | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3f6 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x14a3fe | store | 2 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x14a36a | `[[ebp+16]+8] <= 0xff (unsigned, 32-bit)` | `[[ebp+16]+8] > 0xff (unsigned, 32-bit)` |
| `jae` at 0x14a370 | `[[ebp+16]+8] >= [[ebp+16]+4] (unsigned, 32-bit)` | `[[ebp+16]+8] < [[ebp+16]+4] (unsigned, 32-bit)` |
| `jne` at 0x14a37b | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` | `(([[ebp+8]+22]:2 & 0xffff) & 0x100) == 0x0` |
| `jne` at 0x14a395 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14a3a7 | `? != 0xfc` | `? == 0xfc` |
| `jne` at 0x14a3ba | `? != 0xfc` | `? == 0xfc` |
| `jae` at 0x14a3c5 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |
| `jae` at 0x14a404 | `? >= ? (unsigned, 8-bit)` | `? < ? (unsigned, 8-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x14a3d5 | `eax` | `?` |

## `_vga_setpalette`

- `_vga_setpalette`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e6e0 to 0x16e750.
  Established. The first `_sysOutByte` site presents candidate zero `0x3c8` (`call` at 0x16e6ed).
  Three later sites present `0x3c9` there (`call` at 0x16e709, `call` at 0x16e725, `call` at
  0x16e741), with unknown candidate one at all four. The repeat branches use taken condition `N ==
  0` (`jns` at 0x16e6fd, `jns` at 0x16e719, `jns` at 0x16e735). The final `eax` value is `0x0`
  (`ret` at 0x16e74b), without an established hardware effect.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x16e6e3 | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16e6ff | load | 1 | `sp +0x8` | `-` |
| `movsx` at 0x16e71b | load | 1 | `sp +0xc` | `-` |
| `movsx` at 0x16e737 | load | 1 | `sp +0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e6ed | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebp` | `?` | `?` | `?` |
| `call` at 0x16e709 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e725 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e741 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16e6fd | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e719 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e735 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e74b | `eax` | `0x0` |

## `_vga_getpalette`

- `_vga_getpalette`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e750 to 0x16e7d0.
  Established. The `_sysOutByte` call presents candidate zero `0x3c7` and unknown candidate one
  (`call` at 0x16e769). Each `_sysInByte` site has candidate zero `0x3c9` (`call` at 0x16e780,
  `call` at 0x16e79c, `call` at 0x16e7b8). Four-byte stores use `[sp+8] +0x0` (`mov` at 0x16e78d),
  `[sp+12] +0x0` (`mov` at 0x16e7a9), and `[sp+16] +0x0` (`mov` at 0x16e7c2). Their values remain
  unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e756 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16e759 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x16e75c | load | 4 | `sp +0x10` | `-` |
| `movsx` at 0x16e75f | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16e78d | store | 4 | `[sp+8] +0x0` | `?` |
| `mov` at 0x16e7a9 | store | 4 | `[sp+12] +0x0` | `?` |
| `mov` at 0x16e7c2 | store | 4 | `[sp+16] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e769 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16e780 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e79c | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e7b8 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jns` at 0x16e779 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e795 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e7b1 | `N == 0` | `not (N == 0)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e7cf | `eax` | `0x0` |

## `_vga_setpalvec`

- `_vga_setpalvec`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e7d0 to 0x16e860.
  Established. The initial signed guard takes its bypass edge when `0x0 >= [sp+8] (signed, 32-bit)`
  (`jge` at 0x16e7f2). The body includes unknown-address byte loads (`movsx` at 0x16e7ff, `movsx` at
  0x16e81b, `movsx` at 0x16e837); its three later `_sysOutByte` sites retain candidate zero `0x3c9`
  and unknown candidate one (`call` at 0x16e80b, `call` at 0x16e827, `call` at 0x16e843). The repeat
  comparison retains an unknown left operand (`jl` at 0x16e84e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e7d6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16e7d9 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16e7dc | load | 1 | `sp +0x4` | `-` |
| `movsx` at 0x16e7ff | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16e81b | load | 1 | `(unknown)` | `-` |
| `movsx` at 0x16e837 | load | 1 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e7e6 | `_sysOutByte 0x1081d0` | `0x3c8` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16e80b | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e827 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e843 | `_sysOutByte 0x1081d0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16e7f2 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16e7fd | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e819 | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e835 | `N == 0` | `not (N == 0)` |
| `jl` at 0x16e84e | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e85b | `eax` | `?` |

## `_vga_getpalvec`

- `_vga_getpalvec`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16e860 to 0x16e900.
  Established. The body is entered on the guard fall-through `0x0 < [sp+8] (signed, 32-bit)` (`jge`
  at 0x16e882). Three `_sysInByte` calls have candidate zero `0x3c9` (`call` at 0x16e894, `call` at
  0x16e8b4, `call` at 0x16e8d4). Three four-byte stores have unknown effective addresses and values
  (`mov` at 0x16e8a1, `mov` at 0x16e8c1, `mov` at 0x16e8e1). The exit expression remains unknown
  (`ret` at 0x16e8f6).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16e866 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x16e869 | load | 4 | `sp +0xc` | `-` |
| `movsx` at 0x16e86c | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x16e8a1 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16e8c1 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x16e8e1 | store | 4 | `(unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16e876 | `_sysOutByte 0x1081d0` | `0x3c7` | `?` | `ebx` | `esi` | `edi` | `ebp` |
| `call` at 0x16e894 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8b4 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16e8d4 | `_sysInByte 0x1081b0` | `0x3c9` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x16e882 | `0x0 >= [sp+8] (signed, 32-bit)` | `0x0 < [sp+8] (signed, 32-bit)` |
| `jns` at 0x16e88d | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e8ad | `N == 0` | `not (N == 0)` |
| `jns` at 0x16e8cd | `N == 0` | `not (N == 0)` |
| `jl` at 0x16e8e9 | `? < [sp+8] (signed, 32-bit)` | `? >= [sp+8] (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16e8f6 | `eax` | `?` |

## `_ReadPalette`

- `_ReadPalette`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15772c to 0x157780.
  Established. The load chain has four-byte loads at absolute `0x1cd9dc` (`mov` at 0x15774d) and
  `[0x1cd9dc] +0x0` (`mov` at 0x157753). The indirect call operand reads four bytes from
  `[[0x1cd9dc]] +0xcc`, with leading candidates `[[0x1cd9dc]]`, `0x4`, and `sp-32` (`call` at
  0x157760). The later unnamed direct call targets `0x161920` with unknown candidates (`call` at
  0x157772). Callee effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x157735 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x157738 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x15773b | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x15773e | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x157741 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x157744 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x157747 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x15774a | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x15774d | load | 4 | `abs 0x1cd9dc` | `-` |
| `mov` at 0x157753 | load | 4 | `[0x1cd9dc] +0x0` | `-` |
| `call` at 0x157760 | load | 4 | `[[0x1cd9dc]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x157760 | `(unknown callee) (unknown target)` | `[[0x1cd9dc]]` | `0x4` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x157772 | `(unknown callee) 0x161920` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x15776b | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15777e | `eax` | `?` |

## `_WritePalette`

- `_WritePalette`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1576d8 to 0x15772c.
  Established. Four-byte stores copy `[sp+4]` to `sp -0x20` (`mov` at 0x1576e4), `[sp+8]` to `sp
  -0x1c` (`mov` at 0x1576ea), `[sp+12]` to `sp -0x18` (`mov` at 0x1576f0), and `[sp+16]` to `sp
  -0x14` (`mov` at 0x1576f6). The indirect call presents leading candidates `[[0x1cd9dc]]`, `0x3`,
  and `sp-32` (`call` at 0x15770c); its later branch takes the edge when `Z == 1` (`je` at
  0x157717), bypassing the unnamed direct call (`call` at 0x15771e).

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1576e1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1576e4 | store | 4 | `sp -0x20` | `[sp+4]` |
| `mov` at 0x1576e7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1576ea | store | 4 | `sp -0x1c` | `[sp+8]` |
| `mov` at 0x1576ed | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1576f0 | store | 4 | `sp -0x18` | `[sp+12]` |
| `mov` at 0x1576f3 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1576f6 | store | 4 | `sp -0x14` | `[sp+16]` |
| `mov` at 0x1576f9 | load | 4 | `abs 0x1cd9dc` | `-` |
| `mov` at 0x1576ff | load | 4 | `[0x1cd9dc] +0x0` | `-` |
| `call` at 0x15770c | load | 4 | `[[0x1cd9dc]] +0xcc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15770c | `(unknown callee) (unknown target)` | `[[0x1cd9dc]]` | `0x3` | `sp-32` | `[sp+4]` | `[sp+8]` | `[sp+12]` |
| `call` at 0x15771e | `(unknown callee) 0x161920` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x157717 | `Z == 1` | `not (Z == 1)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x15772a | `eax` | `?` |
