# MMSC SRAM and flash paths in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
No unresolved computed transfers were found in these spans.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and full listings](disasm/sram-flash-6.4-facts.txt) include all tool sections.

## `_sram_read`

- `_sram_read`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10c928 to 0x10ca08.
  Established. The two-byte read at absolute `0xa2000` (`mov` at 0x10c942) feeds a branch taken when
  `([0xa2000]:2 & 0xffff) == 0xaa55`, falling through for inequality (`je` at 0x10c94c). A later
  byte store has unknown effective target and value `0x0` (`mov` at 0x10c98c). Another byte store
  has both unknown target and unknown value (`mov` at 0x10c9ca). The `_sram_off` call has unknown
  candidates (`call` at 0x10c9ef). The memory-window mapping and integrity-check semantics are
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10c93a | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10c942 | load | 2 | `abs 0xa2000` | `-` |
| `cmp` at 0x10c958 | load | 4 | `abs 0x1aaf80` | `-` |
| `mov` at 0x10c968 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c981 | load | 4 | `abs 0xa2004` | `-` |
| `mov` at 0x10c98c | store | 1 | `(unknown)` | `0x0` |
| `mov` at 0x10c9a8 | store | 4 | `abs 0x1aaf80` | `0x0` |
| `mov` at 0x10c9b8 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10c9be | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9c4 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10c9c7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10c9ca | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10c9cf | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9d4 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10c9d7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10c9eb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10c935 | `_sram_on 0x10cac8` | `ebx` | `esi` | `edi` | `?` | `ebp` | `?` |
| `call` at 0x10c9ef | `_sram_off 0x10caf8` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10c94c | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `je` at 0x10c95f | `[0x1aaf80] == 0x0` | `[0x1aaf80] != 0x0` |
| `jbe` at 0x10c979 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `je` at 0x10c986 | `Z == 1` | `not (Z == 1)` |
| `jbe` at 0x10c997 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jne` at 0x10c9b4 | `(0x0 & 0x0) != 0x0` | `(0x0 & 0x0) == 0x0` |
| `jle` at 0x10c9c1 | `[sp+8] <= 0x0 (signed, 32-bit)` | `[sp+8] > 0x0 (signed, 32-bit)` |
| `jg` at 0x10c9d2 | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10c9e4 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10c9ff | `eax` | `?` |

## `_sram_write`

- `_sram_write`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10ca08 to 0x10cac8.
  Established. The branch takes `([0xa2000]:2 & 0xffff) == 0xaa55` and falls through for inequality
  (`je` at 0x10ca31). That fall-through writes two bytes containing `0xaa55` at absolute `0xa2000`
  (`mov` at 0x10ca33), then byte zero at `0xa2002` (`mov` at 0x10ca3c) and `0xa2003` (`mov` at
  0x10ca43). Later four-byte stores write zero (`mov` at 0x10ca6c) and unknown (`mov` at 0x10ca90)
  at absolute `0xa2004`. The bounds branch takes `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)`
  and falls through for larger sums (`jbe` at 0x10caa5). The storage format and physical reply
  requirements remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10ca13 | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10ca1f | store | 4 | `sp -0x8` | `ret(_sram_on)` |
| `mov` at 0x10ca27 | load | 2 | `abs 0xa2000` | `-` |
| `mov` at 0x10ca33 | store | 2 | `abs 0xa2000` | `0xaa55` |
| `mov` at 0x10ca3c | store | 1 | `abs 0xa2002` | `0x0` |
| `mov` at 0x10ca43 | store | 1 | `abs 0xa2003` | `0x0` |
| `mov` at 0x10ca4c | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ca52 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10ca55 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x10ca5c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ca5f | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca62 | store | 1 | `(unknown)` | `?` |
| `cmp` at 0x10ca67 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca6c | store | 4 | `abs 0xa2004` | `0x0` |
| `mov` at 0x10ca78 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10ca90 | store | 4 | `abs 0xa2004` | `?` |
| `mov` at 0x10ca93 | load | 4 | `sp +0xc` | `-` |
| `add` at 0x10ca96 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ca99 | store | 4 | `sp -0xc` | `0x1` |
| `mov` at 0x10caa7 | store | 4 | `sp -0xc` | `0xffffffff` |
| `mov` at 0x10caae | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10cab7 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10ca1a | `_sram_on 0x10cac8` | `ebx` | `esi` | `edi` | `0x0` | `?` | `ebp` |
| `call` at 0x10cab2 | `_sram_off 0x10caf8` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10ca31 | `([0xa2000]:2 & 0xffff) == 0xaa55` | `([0xa2000]:2 & 0xffff) != 0xaa55` |
| `jge` at 0x10ca58 | `[sp-12] >= [sp+8] (signed, 32-bit)` | `[sp-12] < [sp+8] (signed, 32-bit)` |
| `jg` at 0x10ca6a | `[sp+8] > ? (signed, 32-bit)` | `[sp+8] <= ? (signed, 32-bit)` |
| `jbe` at 0x10ca89 | `? <= 0x1ff7 (unsigned, 32-bit)` | `? > 0x1ff7 (unsigned, 32-bit)` |
| `jbe` at 0x10caa5 | `([sp+12] + [sp+8]) <= 0x1ff8 (unsigned, 32-bit)` | `([sp+12] + [sp+8]) > 0x1ff8 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cac3 | `eax` | `[sp-12]` |

## `_sram_on`

- `_sram_on`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cac8 to 0x10caf8.
  Established. Calls `_intLock` (`call` at 0x10cacc), then `_sysOutByte` with leading candidates
  `0x548`, `0x0` (`call` at 0x10cada) and `0x760`, `0xffffff81` (`call` at 0x10cae9). The exit
  retains `ret(_intLock)` in EAX (`ret` at 0x10caf6). The memory-selection effect of those output
  calls remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10caf0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cacc | `_intLock 0x170618` | `ebx` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x10cada | `_sysOutByte 0x1081d0` | `0x548` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cae9 | `_sysOutByte 0x1081d0` | `0x760` | `0xffffff81` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10caf6 | `eax` | `ret(_intLock)` |

## `_sram_off`

- `_sram_off`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10caf8 to 0x10cb28.
  Established. The `_sysOutByte` call presents leading candidates `0x540` and `0x0` (`call` at
  0x10cb06). `_intUnlock` has leading candidate `[sp+4]` (`call` at 0x10cb0f), and the exit carries
  `ret(_intUnlock)` in EAX (`ret` at 0x10cb1a). The mapping change implied by the symbol remains
  unknown without the output callee.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cafc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cb14 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cb06 | `_sysOutByte 0x1081d0` | `0x540` | `0x0` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10cb0f | `_intUnlock 0x170628` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cb1a | `eax` | `ret(_intUnlock)` |

## `_initFlash`

- `_initFlash`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cba4 to 0x10cc14.
  Established. Four-byte stores include `0x550` at `[sp+4] +0x0` (`mov` at 0x10cbaa), `0x8000` at
  `[sp+4] +0x10` (`mov` at 0x10cbc5), and `0xa0000` at `[sp+4] +0x24` (`mov` at 0x10cbe8). Other
  stored record values include `0x760` at `[sp+4] +0x18` (`mov` at 0x10cbd3), `0x540` at `[sp+4]
  +0x1c` (`mov` at 0x10cbda), and `0x548` at `[sp+4] +0x20` (`mov` at 0x10cbe1). EAX returns
  `[sp+4]` (`ret` at 0x10cc07). The configuration-field meanings are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cba7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cbaa | store | 4 | `[sp+4] +0x0` | `0x550` |
| `mov` at 0x10cbb0 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10cbb7 | store | 4 | `[sp+4] +0x8` | `0x2` |
| `mov` at 0x10cbbe | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10cbc5 | store | 4 | `[sp+4] +0x10` | `0x8000` |
| `mov` at 0x10cbcc | store | 4 | `[sp+4] +0x14` | `0x1` |
| `mov` at 0x10cbd3 | store | 4 | `[sp+4] +0x18` | `0x760` |
| `mov` at 0x10cbda | store | 4 | `[sp+4] +0x1c` | `0x540` |
| `mov` at 0x10cbe1 | store | 4 | `[sp+4] +0x20` | `0x548` |
| `mov` at 0x10cbe8 | store | 4 | `[sp+4] +0x24` | `0xa0000` |
| `mov` at 0x10cbef | store | 4 | `[sp+4] +0x28` | `0x800` |
| `mov` at 0x10cbf6 | store | 4 | `[sp+4] +0x2c` | `0x1aaf94` |
| `mov` at 0x10cbfd | store | 4 | `[sp+4] +0x30` | `0x1aafd4` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc07 | `eax` | `[sp+4]` |

## `_openFlashWindow`

- `_openFlashWindow`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc14 to 0x10cc94.
  Established. The first `_sysOutByte` call has leading candidates `[[sp+4]+32]` and `0xffffffff`
  (`call` at 0x10cc4b). The branch takes `[sp+12] != 0x40` and falls through for equality (`jne` at
  0x10cc5d). A later `_sysOutByte` call presents `[[sp+4]+24]` and unknown (`call` at 0x10cc6f).
  `_nanosleep` has leading candidates `0x1aaf84` and `0x0` (`call` at 0x10cc7e). The selected window
  and delay duration remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc1d | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10cc20 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cc26 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x10cc3f | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x10cc47 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10cc53 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cc56 | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x10cc59 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cc5f | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x10cc62 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10cc6b | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cc4b | `_sysOutByte 0x1081d0` | `[[sp+4]+32]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10cc6f | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cc7e | `_nanosleep 0x17a010` | `0x1aaf84` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10cc2e | `? <= ? (unsigned, 32-bit)` | `? > ? (unsigned, 32-bit)` |
| `jne` at 0x10cc5d | `[sp+12] != 0x40` | `[sp+12] == 0x40` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cc8e | `eax` | `?` |

## `_closeFlashWindow`

- `_closeFlashWindow`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cc94 to 0x10cd44.
  Established. Two `_sysOutByte` calls have leading candidate `[[sp+4]+24]` and an unknown second
  candidate (`call` at 0x10cca8, `call` at 0x10ccbe). A third uses `[[sp+4]+28]` and `0xffffffff`
  (`call` at 0x10cccc). `_nanosleep` has leading candidates `0x1aaf84` and `0x0` (`call` at
  0x10ccdb); its result is retained at the return (`ret` at 0x10cce6). The window-closing effect and
  timing remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cc98 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cc9b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10cc9e | load | 4 | `[[sp+4]+44] +0x0` | `-` |
| `mov` at 0x10cca4 | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10ccb0 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x10ccb3 | load | 4 | `[[sp+4]+44] +0x20` | `-` |
| `mov` at 0x10ccba | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10ccc8 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x10cce0 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cca8 | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x10ccbe | `_sysOutByte 0x1081d0` | `[[sp+4]+24]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cccc | `_sysOutByte 0x1081d0` | `[[sp+4]+28]` | `0xffffffff` | `?` | `?` | `?` | `?` |
| `call` at 0x10ccdb | `_nanosleep 0x17a010` | `0x1aaf84` | `0x0` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cce6 | `eax` | `ret(_nanosleep)` |

## `_clearFlashBlock`

- `_clearFlashBlock`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cd44 to 0x10cf84.
  Established. A loop body calls `_openFlashWindow` with leading candidates `[sp+4]`, `[sp+8]`, and
  `0x40` (`call` at 0x10cd6e). Byte stores at `ret(_openFlashWindow) +0x0` write `0x50` (`mov` at
  0x10cd78), `0x20` (`mov` at 0x10cda9), and `0xd0` (`mov` at 0x10cdda). A later `_openFlashWindow`
  call uses leading candidates `[sp+4]`, `[sp+8]`, and `0xff` (`call` at 0x10ce1e). The following
  byte load addresses `ret(_openFlashWindow) +0x0` (`mov` at 0x10ce34). The command meanings, device
  identity and completion rule are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cd4d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10cd50 | store | 1 | `sp -0xc` | `0x0` |
| `mov` at 0x10cd54 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10cd66 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10cd6a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cd78 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x50` |
| `mov` at 0x10cda9 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x20` |
| `mov` at 0x10cdda | store | 1 | `ret(_openFlashWindow) +0x0` | `0xd0` |
| `mov` at 0x10ce0b | load | 4 | `abs 0x1dae7c` | `-` |
| `mov` at 0x10ce16 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ce1a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ce34 | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10ce36 | store | 1 | `sp -0xc` | `[ret(_openFlashWindow)]:1` |
| `mov` at 0x10ce47 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10ce64 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x10ce94 | load | 4 | `abs 0x1dae7c` | `-` |
| `mov` at 0x10ce9c | load | 4 | `sp +0x8` | `-` |
| `inc` at 0x10cebf | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10cebf | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10cec2 | load | 4 | `sp -0x8` | `-` |
| `cmp` at 0x10cec5 | load | 4 | `abs 0x1dae80` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cd6e | `_openFlashWindow 0x10cc14` | `[sp+4]` | `[sp+8]` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10cd82 | `_nanosleep 0x17a010` | `0x1aaf8c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cd94 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cce7` | `?` | `?` | `?` | `?` |
| `call` at 0x10cda1 | `_ffscMsgN 0x11ee50` | `0x10cce7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cdb3 | `_nanosleep 0x17a010` | `0x1aaf8c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cdc5 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cce7` | `?` | `?` | `?` | `?` |
| `call` at 0x10cdd2 | `_ffscMsgN 0x11ee50` | `0x10cce7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cde4 | `_nanosleep 0x17a010` | `0x1aaf8c` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x10cdf6 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cce7` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce03 | `_ffscMsgN 0x11ee50` | `0x10cce7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce1e | `_openFlashWindow 0x10cc14` | `[sp+4]` | `[sp+8]` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10ce50 | `_ffscMsg 0x11edf0` | `0x10cce9` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce76 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cd0e` | `?` | `?` | `?` | `?` |
| `call` at 0x10ce83 | `_ffscMsgN 0x11ee50` | `0x10cd0e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10cea5 | `_ffscMsg 0x11edf0` | `0x10cd10` | `[sp+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10ceb7 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cd3d` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jl` at 0x10cd8c | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10cdbd | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10cdee | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10ce3b | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10ce41 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10ce45 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10ce6a | `(([sp-12]:1 & 0xff) & 0x20) != 0x0` | `(([sp-12]:1 & 0xff) & 0x20) == 0x0` |
| `jl` at 0x10ce6e | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10ceaf | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jg` at 0x10cecb | `g flags condition (operands unknown)` | `not (g flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10cedf | `eax` | `?` |

## `_openFlash`

- `_openFlash`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10cf84 to 0x10d0d4.
  Established. The first `_openFlashWindow` call presents leading candidates `[sp+4]`, `0x0`, and
  `0xff` (`call` at 0x10cfaf). Byte stores then write `0x90` (`mov` at 0x10cfb7) and `0xff` (`mov`
  at 0x10cfbf) at `ret(_openFlashWindow) +0x0`. The branch takes `([ret(_openFlashWindow)]:1 & 0xff)
  != 0x89` and falls through for equality (`jne` at 0x10cfc5). The fall-through comparison takes
  `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` and falls through for inequality (`je` at
  0x10cfca). The device identification and erase policy remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10cf8d | store | 4 | `sp -0x8` | `0x1` |
| `mov` at 0x10cf94 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10cf97 | store | 4 | `[sp+4] +0x4` | `0x1` |
| `mov` at 0x10cfa0 | store | 4 | `[sp+4] +0xc` | `0x0` |
| `mov` at 0x10cfb7 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10cfba | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10cfbc | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10cfbf | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10d01b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d027 | store | 1 | `ret(_openFlashWindow) +0x0` | `0x90` |
| `mov` at 0x10d02a | load | 1 | `ret(_openFlashWindow) +0x0` | `-` |
| `mov` at 0x10d02c | load | 1 | `ret(_openFlashWindow) +0x1` | `-` |
| `mov` at 0x10d02f | store | 1 | `ret(_openFlashWindow) +0x0` | `0xff` |
| `mov` at 0x10d03c | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10d043 | load | 4 | `sp +0x8` | `-` |
| `cmp` at 0x10d056 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d061 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d074 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d079 | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x10d08f | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d095 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d09d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d0b9 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10cfaf | `_openFlashWindow 0x10cc14` | `[sp+4]` | `0x0` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10cffa | `_ffscMsg 0x11edf0` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d01f | `_openFlashWindow 0x10cc14` | `[sp+4]` | `0x20` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d04e | `_ffscMsg 0x11edf0` | `0x10cee0` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d065 | `_fdprintf 0x172100` | `[sp+12]` | `0x10cef7` | `?` | `?` | `?` | `?` |
| `call` at 0x10d07d | `_clearFlashBlock 0x10cd44` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d0a1 | `_clearFlashBlock 0x10cd44` | `[sp+4]` | `?` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d0bd | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10cfc5 | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10cfca | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10d035 | `([ret(_openFlashWindow)]:1 & 0xff) != 0x89` | `([ret(_openFlashWindow)]:1 & 0xff) == 0x89` |
| `je` at 0x10d03a | `([ret(_openFlashWindow)+1]:1 & 0xff) == 0xa2` | `([ret(_openFlashWindow)+1]:1 & 0xff) != 0xa2` |
| `jne` at 0x10d047 | `[sp+8] != 0x2` | `[sp+8] == 0x2` |
| `jl` at 0x10d05a | `[sp+12] < 0x0 (signed, 32-bit)` | `[sp+12] >= 0x0 (signed, 32-bit)` |
| `jne` at 0x10d089 | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) != 0x0` | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) == 0x0` |
| `je` at 0x10d093 | `[sp-8] == 0x0` | `[sp-8] != 0x0` |
| `jne` at 0x10d0ad | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) != 0x0` | `(ret(_clearFlashBlock) & ret(_clearFlashBlock)) == 0x0` |
| `jle` at 0x10d0b7 | `? <= 0xf (signed, 32-bit)` | `? > 0xf (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d0cd | `eax` | `?` |

## `_closeFlash`

- `_closeFlash`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d0d4 to 0x10d0f4.
  Established. Calls `_closeFlashWindow` with leading candidate `[sp+4]` (`call` at 0x10d0dc). A
  four-byte store clears `[sp+4] +0x4` (`mov` at 0x10d0e1). The exit returns EAX zero (`ret` at
  0x10d0f0). The record flag and physical close operation remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d0d8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d0e1 | store | 4 | `[sp+4] +0x4` | `0x0` |
| `mov` at 0x10d0ea | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d0dc | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d0f0 | `eax` | `0x0` |

## `_readFlash`

- `_readFlash`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d0f4 to 0x10d234.
  Established. The first `_openFlashWindow` call has leading candidates `[sp+4]`, unknown, and
  `0xff` (`call` at 0x10d13b). The branch takes `(ret(_openFlashWindow) & ret(_openFlashWindow)) ==
  0x0` and falls through for nonzero (`je` at 0x10d147). After that fall-through, another branch
  takes `[sp+12] <= ? (unsigned, 32-bit)` and falls through for `[sp+12] > ? (unsigned, 32-bit)`
  (`jbe` at 0x10d152). Only the latter route reaches `_bcopy` with leading candidates
  `(ret(_openFlashWindow) + [sp-16])`, `[sp-8]`, and unknown (`call` at 0x10d15e). A
  four-byte store on the taken route writes `0xfffffff7` at `sp -0xc` (`mov` at 0x10d214). The
  actual read range and window mapping are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d0fd | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d100 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d103 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d106 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d109 | store | 4 | `sp -0x8` | `[sp+8]` |
| `mov` at 0x10d10c | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x10d113 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d11e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d121 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d12a | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x10d137 | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d14d | load | 4 | `sp -0x10` | `-` |
| `mov` at 0x10d159 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d166 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d169 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d16e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d171 | store | 4 | `[sp+4] +0xc` | `?` |
| `add` at 0x10d176 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d176 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d180 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d183 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d194 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1a3 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d1b0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1b3 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1b8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1bb | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `add` at 0x10d1c0 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d1c0 | store | 4 | `sp -0x8` | `([sp-8] + [[sp+4]+16])` |
| `mov` at 0x10d1cb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1ce | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d1df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1ee | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d1fb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d1fe | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d203 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d206 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d209 | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d214 | store | 4 | `sp -0xc` | `0xfffffff7` |
| `mov` at 0x10d21b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d224 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d13b | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d15e | `_bcopy 0x17529c` | `(ret(_openFlashWindow) + [sp-16])` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d198 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d1a8 | `_bcopy 0x17529c` | `ret(_openFlashWindow)` | `[sp-8]` | `[[sp+4]+16]` | `?` | `?` | `?` |
| `call` at 0x10d1e3 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0xff` | `?` | `?` | `?` |
| `call` at 0x10d1f3 | `_bcopy 0x17529c` | `?` | `[sp-8]` | `?` | `?` | `?` | `?` |
| `call` at 0x10d21f | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d147 | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jbe` at 0x10d152 | `[sp+12] <= ? (unsigned, 32-bit)` | `[sp+12] > ? (unsigned, 32-bit)` |
| `jb` at 0x10d17b | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jae` at 0x10d1c5 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d1c9 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d230 | `eax` | `[sp-12]` |

## `_writeFlashBytes`

- `_writeFlashBytes`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d234 to 0x10d324.
  Established. The unsigned branch takes `[sp+12] <= 0x0` and falls through for greater values
  (`jbe` at 0x10d255). Byte stores on the fall-through path include `0x50` at `[sp+4] +0x0` (`mov`
  at 0x10d25c, `mov` at 0x10d264) and `0x70` there (`mov` at 0x10d267). A later route writes
  another byte `0x50` at `[sp+4] +0x0` (`mov` at 0x10d284), followed by byte `0x40` at an unknown effective target (`mov` at 0x10d287) and byte `[[sp+8]]:1` at an unknown
  target (`mov` at 0x10d28f). Further byte stores include `0x50` (`mov` at 0x10d298) and `0x70`
  (`mov` at 0x10d29b) at `[sp+4] +0x0`. A later branch takes
  `(([[sp+4]]:1 & 0xff) & 0x10) == 0x0` and falls through for a nonzero expression (`je` at
  0x10d2c7); its taken route writes byte `0xff` at `[sp+4] +0x0` (`mov` at 0x10d2d4). Their command meanings and
  program-completion semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d23d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d242 | store | 4 | `sp -0xc` | `0x64` |
| `mov` at 0x10d24b | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10d252 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d25c | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d264 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d267 | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d26a | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d284 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d287 | store | 1 | `(unknown)` | `0x40` |
| `mov` at 0x10d28a | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d28d | load | 1 | `[sp+8] +0x0` | `-` |
| `mov` at 0x10d28f | store | 1 | `(unknown)` | `[[sp+8]]:1` |
| `mov` at 0x10d298 | store | 1 | `[sp+4] +0x0` | `0x50` |
| `mov` at 0x10d29b | store | 1 | `[sp+4] +0x0` | `0x70` |
| `mov` at 0x10d29e | load | 1 | `[sp+4] +0x0` | `-` |
| `mov` at 0x10d2d4 | store | 1 | `[sp+4] +0x0` | `0xff` |
| `mov` at 0x10d2d7 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d2d9 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d2dc | load | 1 | `[sp+8] +0x0` | `-` |
| `dec` at 0x10d2e7 | load | 4 | `sp -0xc` | `-` |
| `dec` at 0x10d2e7 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d2f4 | store | 4 | `sp -0xc` | `0x2710` |
| `inc` at 0x10d2fb | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10d2fb | store | 4 | `sp -0x8` | `?` |
| `inc` at 0x10d2ff | load | 4 | `sp +0x8` | `-` |
| `inc` at 0x10d2ff | store | 4 | `sp +0x8` | `?` |
| `mov` at 0x10d302 | load | 4 | `sp +0xc` | `-` |
| `cmp` at 0x10d305 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jbe` at 0x10d255 | `[sp+12] <= 0x0 (unsigned, 32-bit)` | `[sp+12] > 0x0 (unsigned, 32-bit)` |
| `jl` at 0x10d26e | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d274 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d278 | `Z == 0` | `not (Z == 0)` |
| `jl` at 0x10d2a2 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jne` at 0x10d2a8 | `? != 0xffffffff` | `? == 0xffffffff` |
| `jne` at 0x10d2ac | `Z == 0` | `not (Z == 0)` |
| `je` at 0x10d2c7 | `(([[sp+4]]:1 & 0xff) & 0x10) == 0x0` | `(([[sp+4]]:1 & 0xff) & 0x10) != 0x0` |
| `je` at 0x10d2e0 | `Z == 1` | `not (Z == 1)` |
| `je` at 0x10d2ea | `Z == 1` | `not (Z == 1)` |
| `jb` at 0x10d308 | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d319 | `eax` | `?` |

## `_writeFlash`

- `_writeFlash`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d324 to 0x10d514.
  Established. The `_openFlashWindow` call presents leading candidates `[sp+4]`, unknown, and `0x40`
  (`call` at 0x10d362); its result branch takes `(ret(_openFlashWindow) & ret(_openFlashWindow)) ==
  0x0` and falls through for nonzero (`je` at 0x10d36e). A following unsigned branch takes `[sp-8] <
  [sp+12]` and falls through otherwise (`jb` at 0x10d37a). The alternative `_writeFlashBytes` calls
  share leading candidates `(ret(_openFlashWindow) + [sp-12])`, `[sp+8]`, and `[sp+12]` (`call` at
  0x10d37f, `call` at 0x10d3b7). The physical write geometry remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d32d | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10d330 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d333 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x10d336 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10d339 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d344 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d347 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d34a | store | 4 | `sp -0x18` | `[[sp+4]+12]` |
| `mov` at 0x10d351 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10d358 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10d35e | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10d374 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x10d377 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d391 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d394 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d399 | store | 4 | `sp -0x14` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d39c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d39f | store | 4 | `[sp+4] +0xc` | `([sp+12] + [[sp+4]+12])` |
| `mov` at 0x10d3c9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3cc | load | 4 | `[sp+4] +0xc` | `-` |
| `add` at 0x10d3cf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d3d2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3d5 | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [sp-8])` |
| `sub` at 0x10d3d8 | load | 4 | `sp -0x8` | `-` |
| `add` at 0x10d3db | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10d3e4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d3e7 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d3f5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d414 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d417 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d41c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d41f | store | 4 | `[sp+4] +0xc` | `([[sp+4]+12] + [[sp+4]+16])` |
| `mov` at 0x10d42e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d431 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d43f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d45e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d461 | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x10d466 | store | 4 | `sp -0x14` | `?` |
| `mov` at 0x10d469 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d46c | store | 4 | `[sp+4] +0xc` | `?` |
| `mov` at 0x10d476 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d47a | store | 4 | `sp -0x1c` | `?` |
| `mov` at 0x10d482 | load | 4 | `sp -0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d362 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d37f | `_writeFlashBytes 0x10d234` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d3b7 | `_writeFlashBytes 0x10d234` | `(ret(_openFlashWindow) + [sp-12])` | `[sp+8]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10d3f9 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d406 | `_writeFlashBytes 0x10d234` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d443 | `_openFlashWindow 0x10cc14` | `[sp+4]` | `?` | `0x40` | `?` | `?` | `?` |
| `call` at 0x10d450 | `_writeFlashBytes 0x10d234` | `ret(_openFlashWindow)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d47d | `_closeFlashWindow 0x10cc94` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10d36e | `(ret(_openFlashWindow) & ret(_openFlashWindow)) == 0x0` | `(ret(_openFlashWindow) & ret(_openFlashWindow)) != 0x0` |
| `jb` at 0x10d37a | `[sp-8] < [sp+12] (unsigned, 32-bit)` | `[sp-8] >= [sp+12] (unsigned, 32-bit)` |
| `jne` at 0x10d38b | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jne` at 0x10d3c3 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jb` at 0x10d3e0 | `? < [[sp+4]+16] (unsigned, 32-bit)` | `? >= [[sp+4]+16] (unsigned, 32-bit)` |
| `jne` at 0x10d412 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `jae` at 0x10d428 | `? >= [[sp+4]+16] (unsigned, 32-bit)` | `? < [[sp+4]+16] (unsigned, 32-bit)` |
| `je` at 0x10d42c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x10d45c | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d493 | `eax` | `?` |

## `_flashtest`

- `_flashtest`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10d514 to 0x10d6b0.
  Established. The `_openFlash` call has leading candidates `sp-56`, `0x2`, and `0xffffffff` (`call`
  at 0x10d58c). Calls inside later loops include `_writeFlash` with leading candidates `sp-56`,
  unknown, and `0x80` (`call` at 0x10d5c3), and `_readFlash` with the same candidate triple (`call`
  at 0x10d61f). A byte store has unknown effective target and value `[sp-64]:1` (`mov` at 0x10d573).
  The hardware test pattern, transferred range and expected results are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10d51d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10d53d | store | 4 | `sp -0x3c` | `ret(_malloc)` |
| `mov` at 0x10d563 | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d56a | store | 4 | `sp -0x40` | `?` |
| `mov` at 0x10d56d | load | 1 | `sp -0x40` | `-` |
| `mov` at 0x10d570 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d573 | store | 1 | `(unknown)` | `[sp-64]:1` |
| `mov` at 0x10d5b9 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d5ce | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d5fa | store | 4 | `sp -0x2c` | `0x0` |
| `mov` at 0x10d615 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d62a | load | 4 | `sp -0x2c` | `-` |
| `mov` at 0x10d650 | load | 4 | `sp -0x3c` | `-` |
| `mov` at 0x10d653 | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10d659 | store | 4 | `sp -0x44` | `?` |
| `mov` at 0x10d676 | store | 4 | `sp -0x40` | `?` |
| `cmp` at 0x10d67d | load | 4 | `sp -0x44` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10d52c | `_ffscMsg 0x11edf0` | `0x10d494` | `[sp+4]` | `?` | `ebx` | `esi` | `edi` |
| `call` at 0x10d535 | `_malloc 0x17d318` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d57f | `_initFlash 0x10cba4` | `sp-56` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d58c | `_openFlash 0x10cf84` | `sp-56` | `0x2` | `0xffffffff` | `?` | `?` | `?` |
| `call` at 0x10d59c | `_ffscMsg 0x11edf0` | `0x10d4ae` | `ret(_openFlash)` | `?` | `?` | `?` | `?` |
| `call` at 0x10d5c3 | `_writeFlash 0x10d324` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d5d8 | `_ffscMsg 0x11edf0` | `0x10d4c4` | `?` | `[sp-44]` | `ret(_writeFlash)` | `?` | `?` |
| `call` at 0x10d61f | `_readFlash 0x10d0f4` | `sp-56` | `?` | `0x80` | `?` | `?` | `?` |
| `call` at 0x10d634 | `_ffscMsg 0x11edf0` | `0x10d4c4` | `?` | `[sp-44]` | `ret(_readFlash)` | `?` | `?` |
| `call` at 0x10d688 | `_ffscMsg 0x11edf0` | `0x10d4e7` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10d69e | `_ffscMsg 0x11edf0` | `0x10d4f8` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jge` at 0x10d544 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d579 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |
| `jl` at 0x10d5a6 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d5b0 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d5e2 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d5f0 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d5f4 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d605 | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `jl` at 0x10d60a | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d644 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jl` at 0x10d648 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10d64e | `0x0 >= ? (signed, 32-bit)` | `0x0 < ? (signed, 32-bit)` |
| `je` at 0x10d680 | `[sp-68] == ?` | `[sp-68] != ?` |
| `jl` at 0x10d697 | `? < ? (signed, 32-bit)` | `? >= ? (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10d6ac | `eax` | `ret(_ffscMsg)` |

## `_ceEnterFlashMode`

- `_ceEnterFlashMode`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x118ee8 to 0x118fc8.
  Established. Two `_bufClear` calls have leading candidates `[[sp+4]+112]` (`call` at 0x118f0a) and
  `[[sp+4]+116]` (`call` at 0x118f16). A four-byte store writes `0x6` at `[sp+4] +0x18` (`mov` at
  0x118f1e). The branch takes `(? & 0x40) == 0x0` and falls through for a nonzero expression (`je`
  at 0x118f2b). The fall-through includes a four-byte store of unknown value at `[sp+4] +0x3c`
  (`mov` at 0x118f47). The flash-mode flag interpretation remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x118eec | load | 4 | `sp +0x4` | `-` |
| `test` at 0x118eef | load | 1 | `abs 0x1db370` | `-` |
| `mov` at 0x118f06 | load | 4 | `[sp+4] +0x70` | `-` |
| `mov` at 0x118f12 | load | 4 | `[sp+4] +0x74` | `-` |
| `mov` at 0x118f1e | store | 4 | `[sp+4] +0x18` | `0x6` |
| `mov` at 0x118f25 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118f2d | load | 4 | `abs 0x1ef244` | `-` |
| `mov` at 0x118f38 | load | 4 | `[sp+4] +0x60` | `-` |
| `mov` at 0x118f41 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118f47 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118f4a | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118f52 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x118f58 | store | 4 | `[sp+4] +0x3c` | `?` |
| `mov` at 0x118f5d | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x118efe | `_ffscMsg 0x11edf0` | `0x118ec5` | `[sp+4]` | `ebx` | `ebp` | `?` | `?` |
| `call` at 0x118f0a | `_bufClear 0x10f448` | `[[sp+4]+112]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118f16 | `_bufClear 0x10f448` | `[[sp+4]+116]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x118f3c | `_fdprintf 0x172100` | `[[sp+4]+96]` | `0x11859d` | `[0x1ef244]` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x118ef6 | `(([0x1db370]:1 & 0xff) & 0x4) == 0x0` | `(([0x1db370]:1 & 0xff) & 0x4) != 0x0` |
| `je` at 0x118f2b | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |
| `jge` at 0x118f50 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x118f63 | `eax` | `0x0` |
