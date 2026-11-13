# MMSC VGA mode and pixel callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
The fifteen primary spans end at the next text symbol, with exclusive inferred ends.
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

[Fact sheets and complete listings](disasm/vga-mode-pixel-657-facts.txt) include all tool sections.

Four computed transfers remain unresolved. Selector-only effects are excluded.
The linear listings cross embedded tables and can miss instruction boundaries.
Six independent analyses follow the fifteen entry analyses in the retained facts:

| Independent span | Start | Exclusive end |
|---|---|---|
| `_vga_setmode` aligned tail | `0x172920` | `0x172c90` |
| `_vga_clear` aligned tail | `0x172d40` | `0x172e50` |
| `_vga_setcolor` aligned tail | `0x1730c0` | `0x173100` |
| `_vga_drawpixel` aligned tail | `0x173160` | `0x173260` |
| earlier `_mwGPX` target | `0x1573e8` | `0x1573f0` |
| earlier `_mwGPP` target | `0x157494` | `0x15749c` |

Each analysis starts with fresh state; its arguments, accesses and returns are not
merged into the primary caller state. The two earlier targets jump back into their
primary listings (`jmp` at 0x1573ea, `jmp` at 0x157496). Those out-of-span edges
remain outside the primary entry effects.
Raw final bytes complete three spans: `01` at `0x16a647` in `_mwSPX`, `00` at
`0x157493` in `_mwGPX`, and `00` at `0x1573e7` in `_mwGPM`.

## `_vga_setmode`

- `_vga_setmode`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1726c0 to 0x172c90.
  Established. Four-byte assignments copy `[0x1d69f0]` to absolute `0x1d69ec` and `[sp+4]` to
  `0x1d69f0` (`mov` at 0x1726e7, `mov` at 0x1726ec). One branch takes the later arm when the first
  stack value self-test is nonzero (`jne` at 0x172732). That arm stores `0x3d4`, `0x3d5`, `0x3da` at
  absolute `0x1d9998`, `0x1d999c`, `0x1d99a0` (`mov` at 0x172890, `mov` at 0x17289a, `mov` at
  0x1728a4); its unsigned range branch takes the shared path for `[sp+4]-1 > 0xb` (`ja` at
  0x1728d4). The other arm has an unresolved computed jump (`jmp` at 0x1728da). Mode meanings and
  selector-only effects are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1726c9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1726cc | load | 4 | `abs 0x1d69f4` | `-` |
| `mov` at 0x1726df | store | 4 | `sp -0x8` | `ret(_intLock)` |
| `mov` at 0x1726e2 | load | 4 | `abs 0x1d69f0` | `-` |
| `mov` at 0x1726e7 | store | 4 | `abs 0x1d69ec` | `[0x1d69f0]` |
| `mov` at 0x1726ec | store | 4 | `abs 0x1d69f0` | `[sp+4]` |
| `mov` at 0x1726f2 | load | 4 | `abs 0x1d99a0` | `-` |
| `cmp` at 0x17270f | load | 4 | `abs 0x1d69fc` | `-` |
| `cmp` at 0x172718 | load | 4 | `abs 0x1d69ec` | `-` |
| `mov` at 0x172783 | load | 4 | `abs 0x1d9d24` | `-` |
| `rep movsd` at 0x172794 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x172794 | load | 4 | `abs 0x1d9d28` | `-` |
| `mov` at 0x1727b4 | load | 4 | `abs 0x1d9d24` | `-` |
| `rep movsd` at 0x1727c5 | store | 4 | `(unknown)` | `?` |
| `rep movsd` at 0x1727c5 | load | 4 | `abs 0x1dbd28` | `-` |
| `cmp` at 0x1727c7 | load | 4 | `abs 0x1d99a4` | `-` |
| `mov` at 0x1727d0 | store | 4 | `abs 0x1d9998` | `0x3b4` |
| `mov` at 0x1727da | store | 4 | `abs 0x1d999c` | `0x3b5` |
| `mov` at 0x1727e4 | store | 4 | `abs 0x1d99a0` | `0x3ba` |
| `movzx` at 0x172810 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x172818 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x172820 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x172849 | load | 4 | `abs 0x1d99a0` | `-` |
| `mov` at 0x172870 | store | 1 | `(unknown)` | `0x20` |
| `mov` at 0x172874 | store | 1 | `(unknown)` | `0x1f` |
| `mov` at 0x172890 | store | 4 | `abs 0x1d9998` | `0x3d4` |
| `mov` at 0x17289a | store | 4 | `abs 0x1d999c` | `0x3d5` |
| `mov` at 0x1728a4 | store | 4 | `abs 0x1d99a0` | `0x3da` |
| `jmp` at 0x1728da | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x172c17 | load | 4 | `abs 0x1d69f8` | `-` |
| `movzx` at 0x172c24 | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x172c2c | load | 1 | `(unknown)` | `-` |
| `movzx` at 0x172c34 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x172c53 | load | 4 | `abs 0x1d99a0` | `-` |
| `mov` at 0x172c70 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1726d5 | `_initialize 0x1724e0` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x1726da | `_intLock 0x175148` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1726f8 | `_sysInByte 0x1081b0` | `[0x1d99a0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172707 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x172728 | `_set_regs 0x172340` | `0x1d65e4` | `0xa` | `?` | `?` | `?` | `?` |
| `call` at 0x17273f | `_set_regs 0x172340` | `0x1d641c` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x17274e | `_sysOutByte 0x1081d0` | `0x3ce` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x17275d | `_sysOutByte 0x1081d0` | `0x3cf` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x17276c | `_sysOutByte 0x1081d0` | `0x3c4` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x17277b | `_sysOutByte 0x1081d0` | `0x3c5` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x17279d | `_sysOutByte 0x1081d0` | `0x3c4` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x1727ac | `_sysOutByte 0x1081d0` | `0x3c5` | `0x8` | `?` | `?` | `?` | `?` |
| `call` at 0x1727f3 | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172806 | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172829 | `_vga_setpalette 0x172e50` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172841 | `_set_regs 0x172340` | `0x1d99a8` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x17284f | `_sysInByte 0x1081b0` | `[0x1d99a0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x17285e | `_sysOutByte 0x1081d0` | `0x3c0` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x1728b3 | `_sysInByte 0x1081b0` | `0x3cc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1728c6 | `_sysOutByte 0x1081d0` | `0x3c2` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172c3d | `_vga_setpalette 0x172e50` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172c4e | `_vga_clear 0x172ce0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172c59 | `_sysInByte 0x1081b0` | `[0x1d99a0]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172c68 | `_sysOutByte 0x1081d0` | `0x3c0` | `0x20` | `?` | `?` | `?` | `?` |
| `call` at 0x172c74 | `_intUnlock 0x175158` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1726d3 | `[0x1d69f4] != 0x0` | `[0x1d69f4] == 0x0` |
| `je` at 0x172716 | `[0x1d69fc] == 0x0` | `[0x1d69fc] != 0x0` |
| `jne` at 0x17271f | `[0x1d69ec] != 0xc` | `[0x1d69ec] == 0xc` |
| `jne` at 0x172732 | `([sp+4] & [sp+4]) != 0x0` | `([sp+4] & [sp+4]) == 0x0` |
| `jne` at 0x1727ce | `[0x1d99a4] != 0x0` | `[0x1d99a4] == 0x0` |
| `jle` at 0x172838 | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |
| `jne` at 0x17287b | `Z == 0` | `not (Z == 0)` |
| `ja` at 0x1728d4 | `[sp+4]-1 > 0xb (unsigned, 32-bit)` | `[sp+4]-1 <= 0xb (unsigned, 32-bit)` |
| `jne` at 0x172c1e | `[0x1d69f8] != 0x0` | `[0x1d69f8] == 0x0` |
| `jle` at 0x172c4c | `? <= 0xff (signed, 32-bit)` | `? > 0xff (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172c84 | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1728da).

## `_vga_clear`

- `_vga_clear`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172ce0 to 0x172e50.
  Established. The initial visible call is `_vga_screenoff`, with leading candidates `ebx`, `esi`,
  `edi` (`call` at 0x172ce6). The unsigned branch takes the shared-exit path for `? > 0xb` (`ja` at
  0x172cf4); its other arm reaches an unresolved computed transfer (`jmp` at 0x172cfa). The known
  shared path calls `_vga_setcolor` with candidate zero `0xf`, then `_vga_screenon` with unknown
  slots (`call` at 0x172e30, `call` at 0x172e38). The return is zero (`ret` at 0x172e48).
  Selector-only effects and screen-clearing behavior are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172ceb | load | 4 | `abs 0x1d69f0` | `-` |
| `jmp` at 0x172cfa | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172ce6 | `_vga_screenoff 0x173260` | `ebx` | `esi` | `edi` | `ebp` | `?` | `?` |
| `call` at 0x172e30 | `_vga_setcolor 0x173070` | `0xf` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172e38 | `_vga_screenon 0x1732a0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x172cf4 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172e48 | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x172cfa).

## `_vga_hasmode`

- `_vga_hasmode`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x172c90 to 0x172ce0.
  Established. The signed branch takes its later arm when `[sp+4] > 0x9` (`jg` at 0x172c9d). That
  arm's next branch is taken for `[sp+4] > 0xc` (`jg` at 0x172cb3); its fall-through calls
  `_get_perm` with candidate zero `ebp`, then `_et4000_test` with unknown slots (`call` at 0x172cb5,
  `call` at 0x172cba). Three returns report `0x1`, `ret(_et4000_test)`, and `0x0` at distinct sites
  (`ret` at 0x172ca7, `ret` at 0x172cc2, `ret` at 0x172cd5). A supported-mode catalogue is not
  established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x172c93 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x172cb5 | `_get_perm 0x171f90` | `ebp` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x172cba | `_et4000_test 0x172000` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x172c98 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jg` at 0x172c9d | `[sp+4] > 0x9 (signed, 32-bit)` | `[sp+4] <= 0x9 (signed, 32-bit)` |
| `jg` at 0x172cb3 | `[sp+4] > 0xc (signed, 32-bit)` | `[sp+4] <= 0xc (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x172ca7 | `eax` | `0x1` |
| `ret` at 0x172cc2 | `eax` | `ret(_et4000_test)` |
| `ret` at 0x172cd5 | `eax` | `0x0` |

## `_vga_setcolor`

- `_vga_setcolor`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173070 to 0x173100.
  Established. A four-byte load reads absolute `0x1d69f0` (`mov` at 0x173077). The unsigned branch
  takes its shared-exit arm when `? > 0xb` (`ja` at 0x173080). The other arm reaches an unresolved
  computed jump (`jmp` at 0x173082). The known exit reports zero (`ret` at 0x1730fe). Selector-only
  stores and hardware color semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173074 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173077 | load | 4 | `abs 0x1d69f0` | `-` |
| `jmp` at 0x173082 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1730f8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x173080 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1730fe | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x173082).

## `_vga_drawpixel`

- `_vga_drawpixel`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173100 to 0x173260.
  Established. Two four-byte entry loads read `sp +0x4` and `sp +0x8` (`mov` at 0x173105, `mov` at
  0x173108). The selector source is the four-byte absolute read at `0x1d69f0` (`mov` at 0x17310b).
  The unsigned branch takes its shared-exit arm for `? > 0xb` (`ja` at 0x173114), while its other
  arm reaches an unresolved computed transfer (`jmp` at 0x17311a). No calls or explicit stores are
  reported among the known-edge effects. The final return is zero (`ret` at 0x17325b). Pixel layout
  and selector-only effects remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173105 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x173108 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17310b | load | 4 | `abs 0x1d69f0` | `-` |
| `jmp` at 0x17311a | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x173114 | `? > 0xb (unsigned, 32-bit)` | `? <= 0xb (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17325b | `eax` | `0x0` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x17311a).

## `_vga_getxdim`

- `_vga_getxdim`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1732e0 to 0x1732f0.
  Established. A four-byte load reads absolute `0x1d9d10` (`mov` at 0x1732e3), and the expression
  `[0x1d9d10]` reaches the exit (`ret` at 0x1732eb). The body has no calls or conditional branches.
  A display-dimension role and unit remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1732e3 | load | 4 | `abs 0x1d9d10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1732eb | `eax` | `[0x1d9d10]` |

## `_vga_getydim`

- `_vga_getydim`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1732f0 to 0x173300.
  Established. The only explicit access reads four bytes at absolute `0x1d9d14` (`mov` at 0x1732f3);
  its return preserves `[0x1d9d14]` (`ret` at 0x1732fb). No condition or callee is present, and the
  value's meaning is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1732f3 | load | 4 | `abs 0x1d9d14` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1732fb | `eax` | `[0x1d9d14]` |

## `_vga_getcolors`

- `_vga_getcolors`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x173300 to 0x173310.
  Established. The exit carries `[0x1d9d18]` (`ret` at 0x17330b), from the four-byte absolute load
  at `0x1d9d18` (`mov` at 0x173303). Color-count semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x173303 | load | 4 | `abs 0x1d9d18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17330b | `eax` | `[0x1d9d18]` |

## `_mwSPX`

- `_mwSPX`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x16a360 to 0x16a648.
  Established. The I/O stores include two-byte `0xf01`, `0x1`, and `0xff08` at absolute port `0x3ce`
  (`out` at 0x16a4c4, `out` at 0x16a4e8, `out` at 0x16a4ee). Other two-byte stores at that port have
  unknown values (`out` at 0x16a4be, `out` at 0x16a4cf), and a further two-byte I/O store has
  unknown port and value (`out` at 0x16a4d8). A byte exchange has unresolved target and store value
  (`xchg` at 0x16a4da). The masked branch is taken when `(([[sp+4]+10]:1 & 0xff) & 0x10) != 0x0`
  (`jne` at 0x16a518). Pixel format, register identities and indirect callees remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x16a36a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16a36d | load | 1 | `[sp+4] +0x8` | `-` |
| `mov` at 0x16a375 | store | 1 | `sp -0x26` | `?` |
| `mov` at 0x16a37a | store | 1 | `sp -0x27` | `?` |
| `mov` at 0x16a37f | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x16a382 | load | 4 | `[[sp+4]+36] +0x0` | `-` |
| `mov` at 0x16a384 | store | 4 | `sp -0x38` | `[[[sp+4]+36]]` |
| `mov` at 0x16a387 | load | 4 | `[[sp+4]+36] +0x4` | `-` |
| `mov` at 0x16a38a | store | 4 | `sp -0x34` | `[[[sp+4]+36]+4]` |
| `mov` at 0x16a38d | load | 1 | `[sp+4] +0x8` | `-` |
| `mov` at 0x16a392 | store | 1 | `sp -0x28` | `?` |
| `mov` at 0x16a39b | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x16a39e | load | 4 | `[[sp+4]+32] +0x14` | `-` |
| `cmp` at 0x16a3a1 | load | 2 | `[[[sp+4]+32]+20] +0x2` | `-` |
| `mov` at 0x16a3ad | store | 4 | `sp -0x24` | `[[[sp+4]+32]+20]` |
| `cmp` at 0x16a3b0 | load | 1 | `sp -0x27` | `-` |
| `mov` at 0x16a3ba | load | 2 | `sp -0x38` | `-` |
| `cmp` at 0x16a3be | load | 2 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x16a3c4 | load | 2 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x16a3c8 | store | 2 | `sp -0x38` | `[[[sp+4]+32]+12]:2` |
| `mov` at 0x16a3cc | load | 2 | `[[sp+4]+32] +0xe` | `-` |
| `cmp` at 0x16a3d0 | load | 2 | `sp -0x36` | `-` |
| `mov` at 0x16a3d6 | load | 2 | `sp -0x36` | `-` |
| `cmp` at 0x16a3e7 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a3ed | store | 4 | `sp -0x24` | `?` |
| `mov` at 0x16a3f0 | store | 2 | `sp -0x36` | `?` |
| `mov` at 0x16a3f4 | load | 2 | `sp -0x34` | `-` |
| `cmp` at 0x16a3f8 | load | 2 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x16a3fe | load | 2 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x16a402 | store | 2 | `sp -0x34` | `[[[sp+4]+32]+16]:2` |
| `mov` at 0x16a406 | load | 2 | `[[sp+4]+32] +0x12` | `-` |
| `cmp` at 0x16a40a | load | 2 | `sp -0x32` | `-` |
| `mov` at 0x16a410 | load | 2 | `sp -0x32` | `-` |
| `mov` at 0x16a414 | store | 2 | `sp -0x32` | `?` |
| `mov` at 0x16a434 | load | 4 | `[[sp+4]+32] +0xc` | `-` |
| `mov` at 0x16a437 | store | 4 | `sp -0x38` | `[[[sp+4]+32]+12]` |
| `mov` at 0x16a43a | load | 4 | `[[sp+4]+32] +0x10` | `-` |
| `mov` at 0x16a43d | store | 4 | `sp -0x34` | `[[[sp+4]+32]+16]` |
| `mov` at 0x16a440 | store | 1 | `sp -0x27` | `0x1` |
| `mov` at 0x16a444 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x16a447 | load | 4 | `[sp+4] +0x30` | `-` |
| `movsx` at 0x16a44a | load | 2 | `[[sp+4]+48] +0x0` | `-` |
| `movsx` at 0x16a44d | load | 2 | `[[sp+4]+48] +0x2` | `-` |
| `mov` at 0x16a451 | load | 4 | `[sp+4] +0x1c` | `-` |
| `dec` at 0x16a454 | load | 1 | `[[sp+4]+28] +0x98` | `-` |
| `dec` at 0x16a454 | store | 1 | `[[sp+4]+28] +0x98` | `?` |
| `cmp` at 0x16a45a | load | 1 | `sp -0x27` | `-` |
| `cmp` at 0x16a460 | load | 2 | `sp -0x38` | `-` |
| `cmp` at 0x16a466 | load | 2 | `sp -0x36` | `-` |
| `cmp` at 0x16a46c | load | 2 | `sp -0x34` | `-` |
| `cmp` at 0x16a472 | load | 2 | `sp -0x32` | `-` |
| `cmp` at 0x16a478 | load | 1 | `sp -0x28` | `-` |
| `cmp` at 0x16a487 | load | 4 | `[[sp+4]+28] +0xa4` | `-` |
| `cmp` at 0x16a48f | load | 4 | `[[sp+4]+28] +0xac` | `-` |
| `call` at 0x16a499 | load | 4 | `[[sp+4]+28] +0xc0` | `-` |
| `mov` at 0x16a49f | load | 4 | `[[sp+4]+28] +0x18` | `-` |
| `mov` at 0x16a4a2 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x16a4af | load | 1 | `[sp+4] +0xc` | `-` |
| `mov` at 0x16a4b5 | load | 1 | `[sp+4] +0x2c` | `-` |
| `out` at 0x16a4be | store | 2 | `I/O abs 0x3ce` | `?` |
| `out` at 0x16a4c4 | store | 2 | `I/O abs 0x3ce` | `0xf01` |
| `mov` at 0x16a4c8 | load | 1 | `(unknown)` | `-` |
| `out` at 0x16a4cf | store | 2 | `I/O abs 0x3ce` | `?` |
| `mov` at 0x16a4d5 | load | 1 | `[sp+4] +0x10` | `-` |
| `out` at 0x16a4d8 | store | 2 | `I/O (unknown)` | `?` |
| `xchg` at 0x16a4da | load | 1 | `(unknown)` | `-` |
| `xchg` at 0x16a4da | store | 1 | `(unknown)` | `?` |
| `out` at 0x16a4e8 | store | 2 | `I/O abs 0x3ce` | `0x1` |
| `out` at 0x16a4ee | store | 2 | `I/O abs 0x3ce` | `0xff08` |
| `mov` at 0x16a4f0 | load | 4 | `[sp+4] +0x1c` | `-` |
| `inc` at 0x16a4f3 | load | 1 | `[[sp+4]+28] +0x98` | `-` |
| `inc` at 0x16a4f3 | store | 1 | `[[sp+4]+28] +0x98` | `?` |
| `test` at 0x16a4fd | load | 2 | `[[sp+4]+28] +0x16` | `-` |
| `call` at 0x16a505 | load | 4 | `[[sp+4]+28] +0xf8` | `-` |
| `test` at 0x16a514 | load | 1 | `[sp+4] +0xa` | `-` |
| `mov` at 0x16a51a | load | 1 | `[sp+4] +0x28` | `-` |
| `mov` at 0x16a526 | load | 4 | `sp -0x24` | `-` |
| `cmp` at 0x16a52f | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16a535 | load | 1 | `sp -0x26` | `-` |
| `mov` at 0x16a53b | store | 4 | `sp -0x24` | `?` |
| `cmp` at 0x16a53e | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a548 | store | 2 | `sp -0x20` | `?` |
| `mov` at 0x16a54c | store | 2 | `sp -0x1e` | `?` |
| `mov` at 0x16a550 | store | 2 | `sp -0x1c` | `?` |
| `mov` at 0x16a554 | store | 2 | `sp -0x1a` | `?` |
| `movsx` at 0x16a558 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a55c | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x16a55f | load | 2 | `sp -0x1e` | `-` |
| `cmp` at 0x16a563 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a569 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a56d | store | 2 | `sp -0x18` | `?` |
| `mov` at 0x16a571 | load | 2 | `sp -0x1a` | `-` |
| `cmp` at 0x16a575 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a57b | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a57f | store | 2 | `sp -0x16` | `?` |
| `mov` at 0x16a583 | load | 2 | `sp -0x20` | `-` |
| `cmp` at 0x16a587 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16a590 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16a59a | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16a5a0 | load | 2 | `sp -0x1c` | `-` |
| `cmp` at 0x16a5a4 | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16a5ad | load | 2 | `sp -0x18` | `-` |
| `mov` at 0x16a5b1 | store | 2 | `sp -0x2e` | `?` |
| `movsx` at 0x16a5b5 | load | 2 | `sp -0x16` | `-` |
| `mov` at 0x16a5b9 | store | 2 | `sp -0x2a` | `?` |
| `cmp` at 0x16a5bd | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16a5c3 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a5c7 | store | 2 | `sp -0x2c` | `?` |
| `movsx` at 0x16a5cb | load | 2 | `sp -0x20` | `-` |
| `cmp` at 0x16a5cf | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16a5d4 | load | 2 | `(unknown)` | `-` |
| `mov` at 0x16a5d7 | store | 2 | `sp -0x30` | `?` |
| `mov` at 0x16a5e5 | load | 4 | `sp -0x14` | `-` |
| `cmp` at 0x16a5e8 | load | 2 | `[sp-60] +0xa` | `-` |
| `movsx` at 0x16a5ee | load | 2 | `sp -0x1c` | `-` |
| `cmp` at 0x16a5f2 | load | 2 | `[sp-60] +0x8` | `-` |
| `cmp` at 0x16a5fa | load | 2 | `(unknown)` | `-` |
| `movsx` at 0x16a600 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16a604 | load | 2 | `sp -0x1a` | `-` |
| `cmp` at 0x16a614 | load | 2 | `(unknown)` | `-` |
| `cmp` at 0x16a620 | load | 1 | `sp -0x26` | `-` |
| `mov` at 0x16a638 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x16a499 | `(unknown callee) (unknown target)` | `0x16a4e0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16a505 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x16a5dc | `(unknown callee) 0x16a630` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x16a37d | `Z == 1` | `not (Z == 1)` |
| `je` at 0x16a395 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x16a3a7 | `([[[[sp+4]+32]+20]+2]:2 & 0xffff) == 0x7fff` | `([[[[sp+4]+32]+20]+2]:2 & 0xffff) != 0x7fff` |
| `je` at 0x16a3b4 | `([sp-39]:1 & 0xff) == 0x0` | `([sp-39]:1 & 0xff) != 0x0` |
| `jge` at 0x16a3c2 | `([sp-56]:2 & 0xffff) >= ([[[sp+4]+32]+12]:2 & 0xffff) (signed, 16-bit)` | `([sp-56]:2 & 0xffff) < ([[[sp+4]+32]+12]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x16a3d4 | `([[[sp+4]+32]+14]:2 & 0xffff) >= ([sp-54]:2 & 0xffff) (signed, 16-bit)` | `([[[sp+4]+32]+14]:2 & 0xffff) < ([sp-54]:2 & 0xffff) (signed, 16-bit)` |
| `je` at 0x16a3df | `([sp-54]:2 & 0xffff) == 0x7fff` | `([sp-54]:2 & 0xffff) != 0x7fff` |
| `jle` at 0x16a3eb | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x16a3fc | `([sp-52]:2 & 0xffff) <= ([[[sp+4]+32]+16]:2 & 0xffff) (signed, 16-bit)` | `([sp-52]:2 & 0xffff) > ([[[sp+4]+32]+16]:2 & 0xffff) (signed, 16-bit)` |
| `jle` at 0x16a40e | `([[[sp+4]+32]+18]:2 & 0xffff) <= ([sp-50]:2 & 0xffff) (signed, 16-bit)` | `([[[sp+4]+32]+18]:2 & 0xffff) > ([sp-50]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x16a41b | `? >= ? (signed, 16-bit)` | `? < ? (signed, 16-bit)` |
| `jge` at 0x16a424 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x16a45e | `([sp-39]:1 & 0xff) == 0x0` | `([sp-39]:1 & 0xff) != 0x0` |
| `jl` at 0x16a464 | `? < ([sp-56]:2 & 0xffff) (signed, 16-bit)` | `? >= ([sp-56]:2 & 0xffff) (signed, 16-bit)` |
| `jl` at 0x16a46a | `? < ([sp-54]:2 & 0xffff) (signed, 16-bit)` | `? >= ([sp-54]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x16a470 | `? >= ([sp-52]:2 & 0xffff) (signed, 16-bit)` | `? < ([sp-52]:2 & 0xffff) (signed, 16-bit)` |
| `jge` at 0x16a476 | `? >= ([sp-50]:2 & 0xffff) (signed, 16-bit)` | `? < ([sp-50]:2 & 0xffff) (signed, 16-bit)` |
| `jne` at 0x16a47c | `([sp-40]:1 & 0xff) != 0x0` | `([sp-40]:1 & 0xff) == 0x0` |
| `jb` at 0x16a48d | `? < [[[sp+4]+28]+164] (unsigned, 32-bit)` | `? >= [[[sp+4]+28]+164] (unsigned, 32-bit)` |
| `jbe` at 0x16a495 | `? <= [[[sp+4]+28]+172] (unsigned, 32-bit)` | `? > [[[sp+4]+28]+172] (unsigned, 32-bit)` |
| `je` at 0x16a4b3 | `([[sp+4]+12]:1 & 0xff) == 0x0` | `([[sp+4]+12]:1 & 0xff) != 0x0` |
| `js` at 0x16a4f9 | `N == 1` | `not (N == 1)` |
| `jg` at 0x16a4fb | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x16a503 | `(([[[sp+4]+28]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[[sp+4]+28]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jne` at 0x16a518 | `(([[sp+4]+10]:1 & 0xff) & 0x10) != 0x0` | `(([[sp+4]+10]:1 & 0xff) & 0x10) == 0x0` |
| `jge` at 0x16a533 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x16a539 | `([sp-38]:1 & 0xff) == 0x0` | `([sp-38]:1 & 0xff) != 0x0` |
| `jle` at 0x16a542 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x16a567 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x16a579 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jl` at 0x16a58b | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x16a594 | `Z == 0` | `not (Z == 0)` |
| `jge` at 0x16a59e | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jle` at 0x16a5a7 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jle` at 0x16a5c1 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jge` at 0x16a5d2 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x16a5ec | `([[sp-60]+10]:2 & 0xffff) != ?` | `([[sp-60]+10]:2 & 0xffff) == ?` |
| `jg` at 0x16a5f5 | `? > ([[sp-60]+8]:2 & 0xffff) (signed, 16-bit)` | `? <= ([[sp-60]+8]:2 & 0xffff) (signed, 16-bit)` |
| `je` at 0x16a5fe | `Z == 1` | `not (Z == 1)` |
| `jg` at 0x16a608 | `([sp-26]:2 & 0xffff) > ? (signed, 16-bit)` | `([sp-26]:2 & 0xffff) <= ? (signed, 16-bit)` |
| `jne` at 0x16a61a | `Z == 0` | `not (Z == 0)` |
| `jne` at 0x16a624 | `([sp-38]:1 & 0xff) != 0x0` | `([sp-38]:1 & 0xff) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x16a4dc | `eax` | `?` |
| `ret` at 0x16a512 | `eax` | `?` |

## `_mwGPX`

- `_mwGPX`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1573f0 to 0x157494.
  Established. The first two sign-test branches both take a target before the analysed entry when `N
  == 1` (`js` at 0x157401, `js` at 0x157405). An independent listing preserves that earlier target
  without supplying caller state. The two-byte I/O store writes `0x304` at absolute port `0x3ce`
  (`out` at 0x15743f). A one-byte I/O store writes `0x2` to an unknown port (`out` at 0x157453). Two
  later one-byte I/O stores have unknown ports and values (`out` at 0x15745b, `out` at 0x157463).
  Two indirect callees have only unknown candidate slots (`call` at 0x15741f, `call` at 0x15747d).
  Pixel-read semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1573f6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1573f9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1573fc | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x157407 | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x15740d | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x157413 | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x157413 | store | 1 | `[sp+12] +0x98` | `?` |
| `call` at 0x15741f | load | 4 | `[sp+12] +0xc4` | `-` |
| `mov` at 0x157427 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x15742a | load | 4 | `(unknown)` | `-` |
| `out` at 0x15743f | store | 2 | `I/O abs 0x3ce` | `0x304` |
| `mov` at 0x157442 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x15744b | load | 1 | `(unknown)` | `-` |
| `out` at 0x157453 | store | 1 | `I/O (unknown)` | `0x2` |
| `mov` at 0x157454 | load | 1 | `(unknown)` | `-` |
| `out` at 0x15745b | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x15745c | load | 1 | `(unknown)` | `-` |
| `out` at 0x157463 | store | 1 | `I/O (unknown)` | `?` |
| `mov` at 0x157464 | load | 1 | `(unknown)` | `-` |
| `inc` at 0x15746b | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x15746b | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x157475 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x15747d | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x15741f | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x15747d | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x157401 | `N == 1` | `not (N == 1)` |
| `js` at 0x157405 | `N == 1` | `not (N == 1)` |
| `jae` at 0x15740b | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x157411 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `js` at 0x157471 | `N == 1` | `not (N == 1)` |
| `jg` at 0x157473 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x15747b | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x157488 | `eax` | `[sp-20]` |

## `_ResetGUIPalette`

- `_ResetGUIPalette`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x132a78 to 0x132aa8.
  Established. The two visible calls to unnamed target `0x15be48` carry candidate prefixes `0x0`,
  `0x0`, `0xf`, `0x1b1058` and `0x0`, `ret(_QueryColors)`, `ret(_QueryColors)`, `0x1b10b2` (`call`
  at 0x132a86, `call` at 0x132a9c). `_QueryColors` itself has unknown candidate slots (`call` at
  0x132a8e). The exit expression is `ret(0x15be48)` (`ret` at 0x132aa4). No palette-reset behavior
  is proved by the caller alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x132a86 | `(unknown callee) 0x15be48` | `0x0` | `0x0` | `0xf` | `0x1b1058` | `ebp` | `?` |
| `call` at 0x132a8e | `_QueryColors 0x166060` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x132a9c | `(unknown callee) 0x15be48` | `0x0` | `ret(_QueryColors)` | `ret(_QueryColors)` | `0x1b10b2` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x132aa4 | `eax` | `ret(0x15be48)` |

## `_mwGPA`

- `_mwGPA`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15721c to 0x1572a8.
  Established. The unsigned branches are taken when `? >= ([[sp+12]+10]:2 & 0xffff)` and `? >=
  ([[sp+12]+12]:2 & 0xffff)` respectively (`jae` at 0x157237, `jae` at 0x15723d). A one-byte memory
  read remains unresolved (`mov` at 0x157269). Three indirect sites have unknown callees and unknown
  candidate slots (`call` at 0x157257, `call` at 0x15727e, `call` at 0x157299). The byte updates at
  `[sp+12] +0x98` have unknown stored values (`dec` at 0x15723f, `inc` at 0x15726c). Pixel storage
  and callback meanings are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x157222 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x157225 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x157228 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x157233 | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x157239 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x15723f | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x15723f | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x157245 | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x15724d | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x157257 | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x15725f | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x157262 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x157269 | load | 1 | `(unknown)` | `-` |
| `inc` at 0x15726c | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x15726c | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x157276 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x15727e | load | 4 | `[sp+12] +0xf8` | `-` |
| `call` at 0x157299 | load | 4 | `[sp+12] +0xc0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x157257 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x15727e | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x157299 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x15722d | `N == 1` | `not (N == 1)` |
| `js` at 0x157231 | `N == 1` | `not (N == 1)` |
| `jae` at 0x157237 | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x15723d | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x15724b | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x157253 | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `jb` at 0x15725d | `C == 1` | `not (C == 1)` |
| `js` at 0x157272 | `N == 1` | `not (N == 1)` |
| `jg` at 0x157274 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x15727c | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jae` at 0x157292 | `? >= ret(unknown) (unsigned, 32-bit)` | `? < ret(unknown) (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x157289 | `eax` | `?` |

## `_mwGPM`

- `_mwGPM`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15735c to 0x1573e8.
  Established. Two indirect callees have only unknown candidate slots (`call` at 0x157397, `call` at
  0x1573cf). The byte load at the indexed target is unresolved (`mov` at 0x1573ad), as is the
  subsequent byte read (`mov` at 0x1573b6). Two byte updates at `[sp+12] +0x98` retain unknown
  values (`dec` at 0x15737f, `inc` at 0x1573bd). The later masked test takes its skip arm when
  `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` (`je` at 0x1573cd). The bit-format role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x157362 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x157365 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x157368 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x157373 | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x157379 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x15737f | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x15737f | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x157385 | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x15738d | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x157397 | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x15739d | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x1573a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1573ad | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1573b6 | load | 1 | `(unknown)` | `-` |
| `inc` at 0x1573bd | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x1573bd | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x1573c7 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x1573cf | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x157397 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1573cf | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x15736d | `N == 1` | `not (N == 1)` |
| `js` at 0x157371 | `N == 1` | `not (N == 1)` |
| `jae` at 0x157377 | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x15737d | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x15738b | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x157393 | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `js` at 0x1573c3 | `N == 1` | `not (N == 1)` |
| `jg` at 0x1573c5 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x1573cd | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1573da | `eax` | `?` |

## `_mwGPP`

- `_mwGPP`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x15749c to 0x157540.
  Established. The first sign-test branch is taken for `N == 1` to a target before the analysed
  entry (`js` at 0x1574b0). The earlier target is retained as an independent listing with fresh
  state. The four-byte local update at `sp -0x14` writes `[sp-20]-4` (`sub` at 0x1574fc). Two
  indirect sites have unknown callees and candidate slots (`call` at 0x1574f3, `call` at 0x157527).
  The loop branch takes its back edge when `N == 0` (`jns` at 0x157512). The reported return
  expression is `[sp-32]` (`ret` at 0x157535). Pixel-plane semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1574a5 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1574a8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1574ab | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1574b6 | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x1574bc | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x1574c2 | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x1574c2 | store | 1 | `[sp+12] +0x98` | `?` |
| `mov` at 0x1574cd | load | 1 | `(unknown)` | `-` |
| `mov` at 0x1574d4 | store | 1 | `sp -0x1c` | `?` |
| `mov` at 0x1574da | store | 4 | `sp -0x18` | `?` |
| `movzx` at 0x1574dd | load | 2 | `[sp+12] +0x14` | `-` |
| `mov` at 0x1574ec | store | 4 | `sp -0x14` | `?` |
| `call` at 0x1574f3 | load | 4 | `[sp+12] +0xc4` | `-` |
| `mov` at 0x1574f9 | load | 4 | `sp -0x14` | `-` |
| `sub` at 0x1574fc | load | 4 | `sp -0x14` | `-` |
| `sub` at 0x1574fc | store | 4 | `sp -0x14` | `[sp-20]-4` |
| `mov` at 0x157500 | load | 4 | `[sp-20] +0x0` | `-` |
| `mov` at 0x157502 | load | 4 | `(unknown)` | `-` |
| `add` at 0x157505 | load | 4 | `sp -0x18` | `-` |
| `mov` at 0x157508 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x15750a | load | 1 | `sp -0x1c` | `-` |
| `inc` at 0x157515 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x157515 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x15751f | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x157527 | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1574f3 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x157527 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x1574b0 | `N == 1` | `not (N == 1)` |
| `js` at 0x1574b4 | `N == 1` | `not (N == 1)` |
| `jae` at 0x1574ba | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x1574c0 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jns` at 0x157512 | `N == 0` | `not (N == 0)` |
| `js` at 0x15751b | `N == 1` | `not (N == 1)` |
| `jg` at 0x15751d | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x157525 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x157535 | `eax` | `[sp-32]` |

## `_mwGP6`

- `_mwGP6`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1572a8 to 0x15735c.
  Established. A two-byte load reads an unresolved target (`movzx` at 0x1572f5). The branch takes
  its alternate path when `C == 1` (`jb` at 0x1572e9). That path's two-byte load also has an unknown
  effective address (`movzx` at 0x15733c). Four indirect calls have unknown callees and all
  candidate slots unknown (`call` at 0x1572e3, `call` at 0x15730b, `call` at 0x157325, `call` at
  0x157351). Three byte updates at `[sp+12] +0x98` retain unknown stored values (`dec` at 0x1572cb,
  `inc` at 0x1572f9, `inc` at 0x15733f). Both visible returns have unknown expressions (`ret` at
  0x157316, `ret` at 0x15735b). No pixel-width interpretation is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1572ae | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1572b1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1572b4 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x1572bf | load | 2 | `[sp+12] +0xa` | `-` |
| `cmp` at 0x1572c5 | load | 2 | `[sp+12] +0xc` | `-` |
| `dec` at 0x1572cb | load | 1 | `[sp+12] +0x98` | `-` |
| `dec` at 0x1572cb | store | 1 | `[sp+12] +0x98` | `?` |
| `cmp` at 0x1572d1 | load | 4 | `[sp+12] +0xa4` | `-` |
| `cmp` at 0x1572d9 | load | 4 | `[sp+12] +0xac` | `-` |
| `call` at 0x1572e3 | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x1572eb | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x1572ee | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x1572f5 | load | 2 | `(unknown)` | `-` |
| `inc` at 0x1572f9 | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x1572f9 | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x157303 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x15730b | load | 4 | `[sp+12] +0xf8` | `-` |
| `call` at 0x157325 | load | 4 | `[sp+12] +0xc0` | `-` |
| `mov` at 0x157330 | load | 4 | `[sp+12] +0x18` | `-` |
| `mov` at 0x157333 | load | 4 | `(unknown)` | `-` |
| `movzx` at 0x15733c | load | 2 | `(unknown)` | `-` |
| `inc` at 0x15733f | load | 1 | `[sp+12] +0x98` | `-` |
| `inc` at 0x15733f | store | 1 | `[sp+12] +0x98` | `?` |
| `test` at 0x157349 | load | 2 | `[sp+12] +0x16` | `-` |
| `call` at 0x157351 | load | 4 | `[sp+12] +0xf8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1572e3 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x15730b | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x157325 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x157351 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `js` at 0x1572b9 | `N == 1` | `not (N == 1)` |
| `js` at 0x1572bd | `N == 1` | `not (N == 1)` |
| `jae` at 0x1572c3 | `? >= ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+10]:2 & 0xffff) (unsigned, 16-bit)` |
| `jae` at 0x1572c9 | `? >= ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` | `? < ([[sp+12]+12]:2 & 0xffff) (unsigned, 16-bit)` |
| `jb` at 0x1572d7 | `? < [[sp+12]+164] (unsigned, 32-bit)` | `? >= [[sp+12]+164] (unsigned, 32-bit)` |
| `jbe` at 0x1572df | `? <= [[sp+12]+172] (unsigned, 32-bit)` | `? > [[sp+12]+172] (unsigned, 32-bit)` |
| `jb` at 0x1572e9 | `C == 1` | `not (C == 1)` |
| `js` at 0x1572ff | `N == 1` | `not (N == 1)` |
| `jg` at 0x157301 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x157309 | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |
| `jae` at 0x15731e | `? >= ret(unknown) (unsigned, 32-bit)` | `? < ret(unknown) (unsigned, 32-bit)` |
| `js` at 0x157345 | `N == 1` | `not (N == 1)` |
| `jg` at 0x157347 | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |
| `je` at 0x15734f | `(([[sp+12]+22]:2 & 0xffff) & 0x8) == 0x0` | `(([[sp+12]+22]:2 & 0xffff) & 0x8) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x157316 | `eax` | `?` |
| `ret` at 0x15735b | `eax` | `?` |
