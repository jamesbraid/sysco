# MMSC flash window routines in patch SG0002378

Read from `patches/SG0002378/usr/cpu/firmware/mmscfw.bin` (patch SG0002378).
Image SHA-256: `b1249e58cf4f00c252362da8ed0515e25bb94aaf096bb60a5c5082c9b6e4d342`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals or padding
after returns. Each end address is exclusive. Statements below concern sites reached by known
control-flow edges.
No unresolved jump transfers were reported in these spans.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/flash-window-sg0002378-facts.txt) include all tool sections.

I/O prefixes in the access tables identify ports rather than memory addresses.

## `_sram_on`

- `_sram_on`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cb38 to 0x10cb68.
  Established. Calling `_intLock` presents leading candidates entry `ebx` and `ebp` (`call` at
  0x10cb3c). The two `_sysOutByte` sites begin `0x548`, `0x0` (`call` at 0x10cb4a) and `0x760`,
  `0xffffff81` (`call` at 0x10cb59). EAX returns `ret(_intLock)` (`ret` at 0x10cb66). The
  mapped-window and interrupt effects are unknown from these calls alone.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cb60 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cb3c | `_intLock 0x1712d0` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10cb4a | `_sysOutByte 0x1081d0` | `0x548` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cb59 | `_sysOutByte 0x1081d0` | `0x760` | `0xffffff81` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb66 | `eax` | `ret(_intLock)` |

## `_sram_off`

- `_sram_off`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cb68 to 0x10cb98.
  Established. The `_sysOutByte` candidates begin `0x540` and `0x0` (`call` at 0x10cb76).
  `_intUnlock` follows with leading candidate `[sp+4]` (`call` at 0x10cb7f). The return carries EAX
  `ret(_intUnlock)` (`ret` at 0x10cb8a). The window-control meaning remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cb6c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cb84 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cb76 | `_sysOutByte 0x1081d0` | `0x540` | `0x0` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cb7f | `_intUnlock 0x1712e0` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb8a | `eax` | `ret(_intUnlock)` |

## `_enable_downloader`

- `_enable_downloader`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cb98 to 0x10cbd8.
  Established. Two `_sysOutByte` sites present leading candidates `0x70` and `0x7e` (`call` at
  0x10cba0, `call` at 0x10cbb8). Between them, `_sysInByte` presents leading candidate `0x71`
  (`call` at 0x10cbaa). The final `_sysOutByte` site begins `0x71` and unknown (`call` at 0x10cbca).
  The downloader-state effect is not established from the unresolved output value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cbcf | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cba0 | `_sysOutByte 0x1081d0` | `0x70` | `0x7e` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cbaa | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cbb8 | `_sysOutByte 0x1081d0` | `0x70` | `0x7e` | `?` | `?` | `?` | `?` |
| `call` at 0x10cbca | `_sysOutByte 0x1081d0` | `0x71` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cbd5 | `eax` | `ret(_sysOutByte)` |

## `_disable_downloader`

- `_disable_downloader`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cbd8 to 0x10cc14.
  Established. The first `_sysOutByte` call has leading candidates `0x70`, `0x7e` (`call` at
  0x10cbe0), followed by `_sysInByte` with leading candidate `0x71` (`call` at 0x10cbea). A second
  `_sysOutByte` site again begins `0x70`, `0x7e` (`call` at 0x10cbf8). The last call begins `0x71`,
  unknown (`call` at 0x10cc08). The numeric register and downloader meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc0d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cbe0 | `_sysOutByte 0x1081d0` | `0x70` | `0x7e` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cbea | `_sysInByte 0x1081b0` | `0x71` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cbf8 | `_sysOutByte 0x1081d0` | `0x70` | `0x7e` | `?` | `?` | `?` | `?` |
| `call` at 0x10cc08 | `_sysOutByte 0x1081d0` | `0x71` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc13 | `eax` | `ret(_sysOutByte)` |

## `_sysA20on`

- `_sysA20on`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108100 to 0x1081a0.
  Established. Byte I/O stores write `0xd1` at port `0x64` (`out` at 0x10810a), `0xdf` at port
  `0x60` (`out` at 0x108116), and `0xff` at port `0x64` (`out` at 0x108122). The comparison branch
  takes `[0x100000] != 0x1234567` and falls through for equality (`jne` at 0x10814b); its
  fall-through includes a byte I/O store of `0x2` at port `0x92` (`out` at 0x108152). After the
  intervening loop falls through, four-byte stores write `0x0` at absolute `0x0` (`mov` at
  0x108166), `0x0` at absolute `0x100000` (`mov` at 0x10816c), and `0x1234567` at absolute `0x0`
  (`mov` at 0x108172). The next branch takes `[0x100000] != 0x1234567` directly to the return
  (`jne` at 0x10817e); its equality fall-through writes four-byte `0xffffffff` at absolute
  `0x108190` (`mov` at 0x108180). The runtime address-alias and port effects remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `out` at 0x10810a | store | 1 | `I/O abs 0x64` | `0xd1` |
| `out` at 0x108116 | store | 1 | `I/O abs 0x60` | `0xdf` |
| `out` at 0x108122 | store | 1 | `I/O abs 0x64` | `0xff` |
| `mov` at 0x108133 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x108139 | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x10813f | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108145 | load | 4 | `abs 0x100000` | `-` |
| `out` at 0x108152 | store | 1 | `I/O abs 0x92` | `0x2` |
| `in` at 0x108156 | load | 1 | `I/O abs 0x92` | `-` |
| `mov` at 0x108166 | store | 4 | `abs 0x0` | `0x0` |
| `mov` at 0x10816c | store | 4 | `abs 0x100000` | `0x0` |
| `mov` at 0x108172 | store | 4 | `abs 0x0` | `0x1234567` |
| `cmp` at 0x108178 | load | 4 | `abs 0x100000` | `-` |
| `mov` at 0x108180 | store | 4 | `abs 0x108190` | `0xffffffff` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108100 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10810c | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108118 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108124 | `_sysWait 0x1081a0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10814b | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |
| `loope` at 0x10815a | `oope flags condition (operands unknown)` | `not (oope flags condition (operands unknown))` |
| `jne` at 0x10817e | `[0x100000] != 0x1234567` | `[0x100000] == 0x1234567` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10818a | `eax` | `0x0` |

## `_initFlash`

- `_initFlash`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc14 to 0x10cc84.
  Established. Four-byte stores include `0x550` at `[sp+4] +0x0` (`mov` at 0x10cc1a), `0x8000` at
  `[sp+4] +0x10` (`mov` at 0x10cc35), `0xa0000` at `[sp+4] +0x24` (`mov` at 0x10cc58), and `0x800`
  at `[sp+4] +0x28` (`mov` at 0x10cc5f). The return carries EAX `[sp+4]` (`ret` at 0x10cc77). The
  initialized field roles are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc17 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc1a | store | 4 | `[sp+4] +0x0` | `0x550` |
| `mov` at 0x10cc20 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10cc27 | store | 4 | `[sp+4] +0x8` | `0x2` |
| `mov` at 0x10cc2e | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10cc35 | store | 4 | `[sp+4] +0x10` | `0x8000` |
| `mov` at 0x10cc3c | store | 4 | `[sp+4] +0x14` | `0x1` |
| `mov` at 0x10cc43 | store | 4 | `[sp+4] +0x18` | `0x760` |
| `mov` at 0x10cc4a | store | 4 | `[sp+4] +0x1c` | `0x540` |
| `mov` at 0x10cc51 | store | 4 | `[sp+4] +0x20` | `0x548` |
| `mov` at 0x10cc58 | store | 4 | `[sp+4] +0x24` | `0xa0000` |
| `mov` at 0x10cc5f | store | 4 | `[sp+4] +0x28` | `0x800` |
| `mov` at 0x10cc66 | store | 4 | `[sp+4] +0x2c` | `0x1ac8a4` |
| `mov` at 0x10cc6d | store | 4 | `[sp+4] +0x30` | `0x1ac8e4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc77 | `eax` | `[sp+4]` |

## `_openFlashWindow`

- `_openFlashWindow`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc84 to 0x10cd04.
  Established. The first decision takes `? <= ? (unsigned, 32-bit)` and falls through for `? > ?
  (unsigned, 32-bit)` (`jbe` at 0x10cc9e); its taken route includes `_sysOutByte` with leading
  candidates `[[sp+4]+32]` and `0xffffffff` (`call` at 0x10ccbb). Another `_sysOutByte` site begins
  `[[sp+4]+24]` and unknown (`call` at 0x10ccdf). The final `_nanosleep` candidates begin `0x1ac894`
  and `0x0` (`call` at 0x10ccee). The effective flash mapping and delay units remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc8a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc8d | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10cc90 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cc96 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10ccaf | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10ccb7 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10ccc3 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10ccc6 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x10ccc9 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cccf | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10ccd2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10ccdb | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ccbb | `_sysOutByte 0x1081d0` | `[[sp+4]+32]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10ccdf | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ccee | `_nanosleep 0x17b920` | `0x1ac894` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10cc9e | `? <= ? (unsigned, 32-bit)` | `? > ? (unsigned, 32-bit)` |
| `jne` at 0x10cccd | `[sp+12] != 0x40` | `[sp+12] == 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10ccfe | `eax` | `?` |

## `_closeFlashWindow`

- `_closeFlashWindow`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cd04 to 0x10cdb4.
  Established. The first two `_sysOutByte` calls have leading candidates `[[sp+4]+24]` and unknown
  (`call` at 0x10cd18, `call` at 0x10cd2e). A third begins `[[sp+4]+28]` and `0xffffffff` (`call` at
  0x10cd3c). Calling `_nanosleep` then presents leading candidates `0x1ac894` and `0x0` (`call` at
  0x10cd4b). EAX returns `ret(_nanosleep)` (`ret` at 0x10cd56). The window-closing effect remains
  unproven.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cd08 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cd0b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cd0e | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10cd14 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10cd20 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cd23 | load | 4 | `[[sp+4]+44] +0x20` | `-` |
| `mov` at 0x10cd2a | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10cd38 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10cd50 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cd18 | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cd2e | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cd3c | `_sysOutByte 0x1081d0` | `[[sp+4]+28]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10cd4b | `_nanosleep 0x17b920` | `0x1ac894` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cd56 | `eax` | `ret(_nanosleep)` |

## `_clearFlashBlock`

- `_clearFlashBlock`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cdb4 to 0x10cff4.
  Established. The first `_openFlashWindow` site has leading candidates `[sp+4]`, `[sp+8]`, and
  `0x40` (`call` at 0x10cdde). Byte stores on the reached loop route write `0x50` (`mov` at
  0x10cde8), `0x20` (`mov` at 0x10ce19), and `0xd0` (`mov` at 0x10ce4a) at `ret(_openFlashWindow)
  +0x0`. A later branch takes `(([sp-12]:1 & 0xff) & 0x20) != 0x0` and falls through for zero (`jne`
  at 0x10ceda). The command-token interpretation and erase result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cdbd | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cdc0 | store | 1 | `sp -0xc` | `0x0` |
| `mov` at 0x10cdc4 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10cdd6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cdda | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cde8 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x50` |
| `mov` at 0x10ce19 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x20` |
| `mov` at 0x10ce4a | store | 1 | `ret(_openFlashWindow) +0x0` | `0xd0` |
| `mov` at 0x10ce7b | load | 4 | `abs 0x1dc7cc` | `-` |
| `mov` at 0x10ce86 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ce8a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cea4 | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10cea6 | store | 1 | `sp -0xc` | `[ret(_openFlashWindow)]:1` |
| `mov` at 0x10ceb7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ced4 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x10cf04 | load | 4 | `abs 0x1dc7cc` | `-` |
| `mov` at 0x10cf0c | load | 4 | `sp +0x8` | `-` |
| `inc` at 0x10cf2f | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10cf2f | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10cf32 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x10cf35 | load | 4 | `abs 0x1dc7d0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cdde | `_openFlashWindow 0x10cc84` | `[sp+4]` | `[sp+8]` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10cdf2 | `_nanosleep 0x17b920` | `0x1ac89c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce04 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cd57` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce11 | `_ffscMsgN 0x11f0a0` | `0x10cd57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce23 | `_nanosleep 0x17b920` | `0x1ac89c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce35 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cd57` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce42 | `_ffscMsgN 0x11f0a0` | `0x10cd57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce54 | `_nanosleep 0x17b920` | `0x1ac89c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce66 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cd57` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce73 | `_ffscMsgN 0x11f0a0` | `0x10cd57` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce8e | `_openFlashWindow 0x10cc84` | `[sp+4]` | `[sp+8]` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10cec0 | `_ffscMsg 0x11f040` | `0x10cd59` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10cee6 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cd7e` | `?` | `?` | `?` | `?` |
| `call` at 0x10cef3 | `_ffscMsgN 0x11f0a0` | `0x10cd7e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf15 | `_ffscMsg 0x11f040` | `0x10cd80` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10cf27 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cdad` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10cdfc | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10ce2d | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10ce5e | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10ceab | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10ceb1 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10ceb5 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10ceda | `(([sp-12]:1 & 0xff) & 0x20) != 0x0` | `(([sp-12]:1 & 0xff) & 0x20) == 0x0` |
| `jl` at 0x10cede | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10cf1f | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jg` at 0x10cf3b | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cf4f | `eax` | `?` |

## `_openFlash`

- `_openFlash`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cff4 to 0x10d144.
  Established. The first `_openFlashWindow` call begins `[sp+4]`, `0x0`, and `0xff` (`call` at
  0x10d01f). Subsequent byte stores put `0x90` (`mov` at 0x10d027) and `0xff` (`mov` at 0x10d02f) at
  `ret(_openFlashWindow) +0x0`. The comparison branch takes `([ret(_openFlashWindow)]:1 & 0xff) !=
  0x89` and falls through for equality (`jne` at 0x10d035). From that fall-through, the next
  branch reaches the second `_openFlashWindow` call only when
  `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` (`je` at 0x10d03a). That call presents leading
  candidates `[sp+4]`, `0x20`, and `0xff` (`call` at 0x10d08f). Byte stores then write `0x90`
  (`mov` at 0x10d097) and `0xff` (`mov` at 0x10d09f) at the corresponding call result plus zero.
  The device identity and clear policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cffd | store | 4 | `sp -0x8` | `0x1` |
| `mov` at 0x10d004 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d007 | store | 4 | `[sp+4] +0x4` | `0x1` |
| `mov` at 0x10d010 | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10d027 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10d02a | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10d02c | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10d02f | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10d08b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d097 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10d09a | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10d09c | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10d09f | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10d0ac | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10d0b3 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10d0c6 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d0d1 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d0e4 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d0e9 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10d0ff | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d105 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d10d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d129 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d01f | `_openFlashWindow 0x10cc84` | `[sp+4]` | `0x0` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d06a | `_ffscMsg 0x11f040` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d08f | `_openFlashWindow 0x10cc84` | `[sp+4]` | `0x20` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d0be | `_ffscMsg 0x11f040` | `0x10cf50` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d0d5 | `_fdprintf 0x172db8` | `[sp+12]` | `0x10cf67` | `?` | `?` | `?` | `?` |
| `call` at 0x10d0ed | `_clearFlashBlock 0x10cdb4` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d111 | `_clearFlashBlock 0x10cdb4` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d12d | `_closeFlashWindow 0x10cd04` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10d035 | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10d03a | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10d0a5 | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10d0aa | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10d0b7 | `[sp+8] != 0x2` | `[sp+8] == 0x2` |
| `jl` at 0x10d0ca | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jne` at 0x10d0f9 | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) != 0x0` | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) == 0x0` |
| `je` at 0x10d103 | `[sp-8] == 0x0` | `[sp-8] != 0x0` |
| `jne` at 0x10d11d | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) != 0x0` | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) == 0x0` |
| `jle` at 0x10d127 | `? <= 0xf (signed, 32-bit)` | `? > 0xf (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d13d | `eax` | `?` |

## `_closeFlash`

- `_closeFlash`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d144 to 0x10d164.
  Established. Calls `_closeFlashWindow` with leading candidate `[sp+4]` (`call` at 0x10d14c). A
  four-byte store then puts `0x0` at `[sp+4] +0x4` (`mov` at 0x10d151). EAX reaches the return as
  `0x0` (`ret` at 0x10d160). The cleared field meaning remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d148 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d151 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10d15a | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d14c | `_closeFlashWindow 0x10cd04` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d160 | `eax` | `0x0` |

## `_readFlash`

- `_readFlash`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d164 to 0x10d2a4.
  Established. The first `_openFlashWindow` candidates begin `[sp+4]`, unknown, and `0xff` (`call`
  at 0x10d1ab); its branch takes `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` and falls
  through for nonzero (`je` at 0x10d1b7). A later `_bcopy` site has leading candidates
  `ret(_openFlashWindow)`, `[sp-8]`, and `[[sp+4]+16]` (`call` at 0x10d218). The final
  `_closeFlashWindow` site begins `[sp+4]` (`call` at 0x10d28f). The chunk boundaries and physical
  read outcome remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d16d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d170 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d173 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d176 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d179 | store | 4 | `sp -0x8` | `[sp+8]` |
| `mov` at 0x10d17c | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10d183 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d18e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d191 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d19a | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x10d1a7 | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d1bd | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d1c9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d1d6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1d9 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1de | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1e1 | store | 4 | `[sp+4] +0xc` | `?` |
| `add` at 0x10d1e6 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d1e6 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d1f0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1f3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d204 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d213 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d220 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d223 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d228 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d22b | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `add` at 0x10d230 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d230 | store | 4 | `sp -0x8` | `([sp-8] + [[sp+4]+16])` |
| `mov` at 0x10d23b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d23e | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d24f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d25e | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d26b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d26e | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d273 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d276 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d279 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d284 | store | 4 | `sp -0xc` | `0xfffffff7` |
| `mov` at 0x10d28b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d294 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d1ab | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d1ce | `_bcopy 0x176bac` | `(ret(_openFlashWindow) + [sp-16])` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d208 | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d218 | `_bcopy 0x176bac` | `ret(_openFlashWindow)` | `[sp-8]` | `[[sp+4]+16]` | `?` | `?` | `?` |
| `call` at 0x10d253 | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d263 | `_bcopy 0x176bac` | `?` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d28f | `_closeFlashWindow 0x10cd04` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d1b7 | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jbe` at 0x10d1c2 | `[sp+12] <= ? (unsigned, 32-bit)` | `[sp+12] > ? (unsigned, 32-bit)` |
| `jb` at 0x10d1eb | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jae` at 0x10d235 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d239 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d2a0 | `eax` | `[sp-12]` |

## `_writeFlashBytes`

- `_writeFlashBytes`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d2a4 to 0x10d394.
  Established. The entry branch takes `[sp+12] <= 0x0 (unsigned, 32-bit)` and falls through for
  greater values (`jbe` at 0x10d2c5). The fall-through includes byte stores of `0x50` (`mov` at
  0x10d2cc, `mov` at 0x10d2d4) and `0x70` (`mov` at 0x10d2d7) at `[sp+4] +0x0`. A later route writes
  byte `0x40` to an unknown effective target (`mov` at 0x10d2f7), then `[[sp+8]]:1` to an unknown
  target (`mov` at 0x10d2ff). Another route writes byte `0xff` at `[sp+4] +0x0` (`mov` at 0x10d344).
  The token meanings and program result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d2ad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d2b2 | store | 4 | `sp -0xc` | `0x64` |
| `mov` at 0x10d2bb | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10d2c2 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d2cc | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d2d4 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d2d7 | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d2da | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d2f4 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d2f7 | store | 1 | `(unknown)` | `0x40` |
| `mov` at 0x10d2fa | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d2fd | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x10d2ff | store | 1 | `(unknown)` | `[[sp+8]]:1` |
| `mov` at 0x10d308 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d30b | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d30e | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d344 | store | 1 | `[sp+4] +0x0` | `0xff` |
| `mov` at 0x10d347 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d349 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d34c | load | 1 | `[sp+8] +0x0` | `-` |
| `dec` at 0x10d357 | load | 4 | `sp -0xc` | `-` |
| `dec` at 0x10d357 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d364 | store | 4 | `sp -0xc` | `0x2710` |
| `inc` at 0x10d36b | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10d36b | store | 4 | `sp -0x8` | `?` |
| `inc` at 0x10d36f | load | 4 | `sp +0x8` | `-` |
| `inc` at 0x10d36f | store | 4 | `sp +0x8` | `?` |
| `mov` at 0x10d372 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10d375 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10d2c5 | `[sp+12] <= 0x0 (unsigned, 32-bit)` | `[sp+12] > 0x0 (unsigned, 32-bit)` |
| `jl` at 0x10d2de | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d2e4 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d2e8 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10d312 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d318 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d31c | `Z == 0` | `not (Z == 0)` |
| `je` at 0x10d337 | `(([[sp+4]]:1 & 0xff) & 0x10) == 0x0` | `(([[sp+4]]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x10d350 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x10d35a | `Z == 1` | `not (Z == 1)` |
| `jb` at 0x10d378 | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d389 | `eax` | `?` |

## `_writeFlash`

- `_writeFlash`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d394 to 0x10d584.
  Established. Calling `_openFlashWindow` first presents leading candidates `[sp+4]`, unknown, and
  `0x40` (`call` at 0x10d3d2). One later `_writeFlashBytes` call begins `(ret(_openFlashWindow) +
  [sp-12])`, `[sp+8]`, and `[sp+12]` (`call` at 0x10d3ef). After the other
  `_writeFlashBytes` site (`call` at 0x10d427), the branch falls through for `(? & ?) == 0x0`
  (`jne` at 0x10d433). That fall-through includes a four-byte store that writes `([[sp+4]+12] + [sp-8])` at `[sp+4] +0xc` (`mov` at 0x10d445). The final
  `_closeFlashWindow` candidates begin `[sp+4]` (`call` at 0x10d4ed). The offset units and
  persistence behavior remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d39d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d3a0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3a3 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d3a6 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d3a9 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d3b4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3b7 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d3ba | store | 4 | `sp -0x18` | `[[sp+4]+12]` |
| `mov` at 0x10d3c1 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d3c8 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d3ce | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d3e4 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x10d3e7 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d401 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d404 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d409 | store | 4 | `sp -0x14` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d40c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d40f | store | 4 | `[sp+4] +0xc` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d439 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d43c | load | 4 | `[sp+4] +0xc` | `-` |
| `add` at 0x10d43f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d442 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d445 | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [sp-8])` |
| `sub` at 0x10d448 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d44b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d454 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d457 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d465 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d484 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d487 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d48c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d48f | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `mov` at 0x10d49e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d4a1 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d4af | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d4ce | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d4d1 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d4d6 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d4d9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d4dc | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d4e6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d4ea | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x10d4f2 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d3d2 | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d3ef | `_writeFlashBytes 0x10d2a4` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d427 | `_writeFlashBytes 0x10d2a4` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d469 | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d476 | `_writeFlashBytes 0x10d2a4` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d4b3 | `_openFlashWindow 0x10cc84` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d4c0 | `_writeFlashBytes 0x10d2a4` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d4ed | `_closeFlashWindow 0x10cd04` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d3de | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jb` at 0x10d3ea | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |
| `jne` at 0x10d3fb | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10d433 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jb` at 0x10d450 | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jne` at 0x10d482 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jae` at 0x10d498 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d49c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x10d4cc | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d503 | `eax` | `?` |

## `_flashtest`

- `_flashtest`, `patches/SG0002378/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d584 to 0x10d720.
  Established. The `_initFlash` candidates begin `sp-56` (`call` at 0x10d5ef). `_openFlash` then
  presents leading candidates `sp-56`, `0x2`, and `0xffffffff` (`call` at 0x10d5fc). Later loop
  routes include `_writeFlash` with leading candidates `sp-56`, unknown, `0x80` (`call` at 0x10d633)
  and `_readFlash` with the same leading candidates (`call` at 0x10d68f). The comparison branch
  takes `[sp-68] == ?` and falls through for inequality (`je` at 0x10d6f0). A successful hardware
  test is not established by these static sites.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d58d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d5ad | store | 4 | `sp -0x3c` | `ret(_malloc)` |
| `mov` at 0x10d5d3 | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d5da | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d5dd | load | 1 | `sp -0x40` | `-` |
| `mov` at 0x10d5e0 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d5e3 | store | 1 | `(unknown)` | `[sp-64]:1` |
| `mov` at 0x10d629 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d63e | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d66a | store | 4 | `sp -0x2c` | `0x0` |
| `mov` at 0x10d685 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d69a | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d6c0 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d6c3 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d6c9 | store | 4 | `sp -0x44` | `?` |
| `mov` at 0x10d6e6 | store | 4 | `sp -0x40` | `?` |
| `cmp` at 0x10d6ed | load | 4 | `sp -0x44` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d59c | `_ffscMsg 0x11f040` | `0x10d504` | `[sp+4]` | `?` | `ebx` | `esi` | `edi` |
| `call` at 0x10d5a5 | `_malloc 0x17ec28` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d5ef | `_initFlash 0x10cc14` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d5fc | `_openFlash 0x10cff4` | `sp-56` | `0x2` | `0xffffffff` | `?` | `?` | `?` |
| `call` at 0x10d60c | `_ffscMsg 0x11f040` | `0x10d51e` | `ret(_openFlash)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d633 | `_writeFlash 0x10d394` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d648 | `_ffscMsg 0x11f040` | `0x10d534` | `?` | `[sp-44]` | `ret(_writeFlash)` | `?` | `?` |
| `call` at 0x10d68f | `_readFlash 0x10d164` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d6a4 | `_ffscMsg 0x11f040` | `0x10d534` | `?` | `[sp-44]` | `ret(_readFlash)` | `?` | `?` |
| `call` at 0x10d6f8 | `_ffscMsg 0x11f040` | `0x10d557` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d70e | `_ffscMsg 0x11f040` | `0x10d568` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d5b4 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d5e9 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `jl` at 0x10d616 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d620 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d652 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d660 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d664 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d675 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d67a | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d6b4 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d6b8 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d6be | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `je` at 0x10d6f0 | `[sp-68] == ?` | `[sp-68] != ?` |
| `jl` at 0x10d707 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d71c | `eax` | `ret(_ffscMsg)` |
