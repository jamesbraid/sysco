# MMSC palette and port callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Thirteen spans end at the next text symbol. `_mwVGAHWPal` uses an explicit 180-byte span
and `_mwVGAHRPal` an explicit 416-byte span through shared continuations. These given
extents overlap following entries.
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

[Fact sheets and complete listings](disasm/palette-ports-657-facts.txt) include all tool sections.

Raw final zero bytes at `0x14ec4f` and `0x15beef` complete the `_mwVGAOscan` and
`_ReadPalette` spans respectively. The unnamed targets `0x14eb7d` and `0x14ebc6`
are included in the listings but their effects are not reached by known entry edges.
The direct call sites remain listed without assuming the callees' effects or arguments
beyond their printed candidates. Interrupt-service meanings and runtime mode are unknown.

## `_mwCGAPal`

- `_mwCGAPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e810 to 0x14e878.
  Established. Three two-byte reads use `[[ebp+16]+12] +0x0`, `+0x2`, `+0x4` (`mov` at 0x14e821,
  `mov` at 0x14e834, `mov` at 0x14e848). Each intervening branch is taken when `(? & 0x80) == 0x0`
  (`je` at 0x14e827, `je` at 0x14e83b, `je` at 0x14e84f). The two interrupt instructions carry
  immediate `0x10` (`int` at 0x14e85e, `int` at 0x14e871). Their service semantics and the final
  return value are Unknown (`ret` at 0x14e877).

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

- `_mwEGAWPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e878 to 0x14e92c.
  Established. The first unsigned branch takes its continuation arm when `[[ebp+16]+8] <= 0xf`
  (`jbe` at 0x14e889). The next is taken when `[[ebp+16]+8] >= [[ebp+16]+4]` (`jae` at 0x14e892). A
  one-byte I/O load reads absolute port `0x3da` (`in` at 0x14e8a4). The first two one-byte I/O
  stores have unknown ports and values (`out` at 0x14e8f7, `out` at 0x14e8fa). The later one-byte
  store writes `0x20` to an unknown port (`out` at 0x14e907). The memory byte store is also
  unresolved (`mov` at 0x14e8ee). Palette format and hardware identities are Unknown.

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

- `_mwEGARPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e92c to 0x14e9bc.
  Established. The entry reads four bytes at `[ebp+16] +0x4`, `+0x8`, and `+0xc` (`mov` at 0x14e933,
  `mov` at 0x14e936, `mov` at 0x14e94d). The first range branch is taken for `[[ebp+16]+8] <= 0xf
  (unsigned, 32-bit)` (`jbe` at 0x14e93c). Three two-byte stores have unknown targets and values
  (`mov` at 0x14e969, `mov` at 0x14e983, `mov` at 0x14e99e). The loop comparison values remain
  unknown (`jle` at 0x14e9aa). The palette record interpretation is Unknown.

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

- `_mwEGAOscan`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9bc to 0x14e9e4.
  Established. The one-byte I/O sequence reads port `0x3da` (`in` at 0x14e9c2), then writes `0x11`,
  an unknown value, and `0x20` to port `0x3c0` (`out` at 0x14e9c9, `out` at 0x14e9d8, `out` at
  0x14e9db); its intervening byte load has an unresolved target (`mov` at 0x14e9d6). The exit value
  remains unknown (`ret` at 0x14e9e0). No display-control role is established.

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

- `_mwVGAHWPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14e9e4 to 0x14ea98.
  Established. A zero `[[ebp+16]]` takes the shared-continuation branch (`je` at 0x14e9f7). The
  other arm includes a visible unnamed call to `0x14eb7d` (`call` at 0x14e9f9). Shared code reads a
  two-byte value from `[ebp+8] +0x16` (`mov` at 0x14ea10). The branch takes the later arm when
  `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14ea34). Eight one-byte I/O stores have
  unknown ports and values (`out` at 0x14ea4a, `out` at 0x14ea4f, `out` at 0x14ea52, `out` at
  0x14ea55, `out` at 0x14ea7d, `out` at 0x14ea82, `out` at 0x14ea85, `out` at 0x14ea88). Palette
  encoding and call effects remain Unknown.

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

- `_mwVGAHRPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea98 to 0x14ec38.
  Established. The entry's zero test takes the shared continuation for `[[ebp+16]] == 0x0` (`je` at
  0x14eaab); its other arm has a visible unnamed call to `0x14ebc6` (`call` at 0x14eaad). The shared
  branch is taken when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14eaeb). Two
  one-byte I/O stores have unknown ports and values (`out` at 0x14eaef, `out` at 0x14eb4f). Six
  two-byte memory stores retain unknown targets and values (`mov` at 0x14eb0a, `mov` at 0x14eb1c,
  `mov` at 0x14eb2f, `mov` at 0x14eb5f, `mov` at 0x14eb66, `mov` at 0x14eb6e). The explicit span
  includes shared continuations and further unnamed bodies, whose unreached effects are excluded.
  Hardware role: Unknown.

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

- `_mwVGAOscan`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ec38 to 0x14ec50.
  Established. A one-byte I/O load reads absolute port `0x3da` (`in` at 0x14ec3d). The next stores
  write `0x31` and an unknown byte to port `0x3c0` (`out` at 0x14ec44, `out` at 0x14ec48). A
  four-byte load reads `ebp +0x10` between those stores (`mov` at 0x14ec45). The returned expression
  is unknown (`ret` at 0x14ec4c). The meaning of the output sequence is Unknown.

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

- `_mw8514WPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14ea00 to 0x14ea98.
  Established. Shared code takes the range continuation when `[[ebp+16]+8] <= 0xff` and then when
  `[[ebp+16]+8] >= [[ebp+16]+4]`, both unsigned comparisons (`jbe` at 0x14ea23, `jae` at 0x14ea29).
  The masked two-byte test takes its later arm for a nonzero `0x100` mask (`jne` at 0x14ea34). I/O
  stores include eight one-byte accesses with unknown ports and values (`out` at 0x14ea4a, `out` at
  0x14ea4f, `out` at 0x14ea52, `out` at 0x14ea55, `out` at 0x14ea7d, `out` at 0x14ea82, `out` at
  0x14ea85, `out` at 0x14ea88). The return remains unknown (`ret` at 0x14ea6a). No color-component
  encoding is established.

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

- `_mw8514RPal`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x14eab7 to 0x14ec38.
  Established. The shared-range branches take their continuations when `[[ebp+16]+8] <= 0xff` and
  `[[ebp+16]+8] >= [[ebp+16]+4]` as unsigned comparisons (`jbe` at 0x14eada, `jae` at 0x14eae0). The
  branch takes its later arm when `(([[ebp+8]+22]:2 & 0xffff) & 0x100) != 0x0` (`jne` at 0x14eaeb).
  Three one-byte I/O reads occur on each of those two arms, all at unknown ports (`in` at 0x14eaf2,
  `in` at 0x14eaf5, `in` at 0x14eaf8, `in` at 0x14eb52, `in` at 0x14eb55, `in` at 0x14eb58). Each
  arm has three unresolved two-byte memory stores (`mov` at 0x14eb0a, `mov` at 0x14eb1c, `mov` at
  0x14eb2f, `mov` at 0x14eb5f, `mov` at 0x14eb66, `mov` at 0x14eb6e). The later unnamed bodies are
  listed without attributing their effects to this entry. Component meanings remain Unknown.

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

- `_vga_setpalette`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172e50 to 0x172ec0.
  Established. The `_sysOutByte` sites carry candidate zero `0x3c8`, then `0x3c9` at three later
  sites, with candidate one unknown throughout (`call` at 0x172e5d, `call` at 0x172e79, `call` at
  0x172e95, `call` at 0x172eb1). Each intervening loop branch is taken when `N == 0` (`jns` at
  0x172e6d, `jns` at 0x172e89, `jns` at 0x172ea5). The exit reports zero (`ret` at 0x172ebb).
  Palette components and timing units are Unknown.

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

- `_vga_getpalette`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172ec0 to 0x172f40.
  Established. The first `_sysOutByte` call has candidate zero `0x3c7` and unknown candidate one
  (`call` at 0x172ed9). Three `_sysInByte` calls carry candidate zero `0x3c9` (`call` at 0x172ef0,
  `call` at 0x172f0c, `call` at 0x172f28). The four-byte stores at `[sp+8] +0x0`, `[sp+12] +0x0`,
  and `[sp+16] +0x0` have unknown values (`mov` at 0x172efd, `mov` at 0x172f19, `mov` at 0x172f32).
  The exit is zero (`ret` at 0x172f3f). Color meanings remain Unknown.

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

- `_vga_setpalvec`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172f40 to 0x172fd0.
  Established. The entry `_sysOutByte` candidate zero is `0x3c8` (`call` at 0x172f56). The branch
  takes the exit arm when `0x0 >= [sp+8] (signed, 32-bit)` (`jge` at 0x172f62). The loop contains
  three `_sysOutByte` sites with candidate zero `0x3c9` and unknown candidate one (`call` at
  0x172f7b, `call` at 0x172f97, `call` at 0x172fb3); its three one-byte memory loads have unresolved
  targets (`movsx` at 0x172f6f, `movsx` at 0x172f8b, `movsx` at 0x172fa7). The tracked return is
  unknown (`ret` at 0x172fcb). The complete transfer extent is not established.

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

- `_vga_getpalvec`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172fd0 to 0x173070.
  Established. The initial branch takes its exit route when `0x0 >= [sp+8] (signed, 32-bit)` (`jge`
  at 0x172ff2). The `_sysOutByte` site carries candidate zero `0x3c7` (`call` at 0x172fe6). Three
  `_sysInByte` sites carry candidate zero `0x3c9` (`call` at 0x173004, `call` at 0x173024, `call` at
  0x173044). Three four-byte stores have unknown targets and values (`mov` at 0x173011, `mov` at
  0x173031, `mov` at 0x173051). The loop branch takes the back edge when `? < [sp+8] (signed,
  32-bit)` (`jl` at 0x173059). Count and component meanings are Unknown.

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

- `_ReadPalette`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be9c to 0x15bef0.
  Established. The entry copies `[sp+4]`, `[sp+8]`, `[sp+12]`, `[sp+16]` into four-byte stack
  locations `sp -0x20`, `-0x1c`, `-0x18`, `-0x14` (`mov` at 0x15bea8, `mov` at 0x15beae, `mov` at
  0x15beb4, `mov` at 0x15beba); its indirect call has unknown callee and leading candidates
  `[[0x1d323c]]`, `0x4`, `sp-32` (`call` at 0x15bed0). A branch falls through for `not (Z == 1)`
  (`je` at 0x15bedb), reaching an unnamed call to `0x166090` with unknown candidate slots (`call` at
  0x15bee2). Palette-dispatch semantics are Unknown.

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

- `_WritePalette`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15be48 to 0x15be9c.
  Established. Four-byte stack stores copy `[sp+4]`, `[sp+8]`, `[sp+12]`, `[sp+16]` at `sp -0x20`,
  `-0x1c`, `-0x18`, `-0x14` respectively (`mov` at 0x15be54, `mov` at 0x15be5a, `mov` at 0x15be60,
  `mov` at 0x15be66). The indirect call has leading candidates `[[0x1d323c]]`, `0x3`, `sp-32` with
  an unknown callee (`call` at 0x15be7c). The later branch is taken when `Z == 1`, skipping the
  unnamed call to `0x166090` (`je` at 0x15be87, `call` at 0x15be8e). The command value and callee
  effects are Unknown.

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
