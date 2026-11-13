# MMSC i8250 serial and port routines in IRIX 6.4

Read from `6.4/usr/cpu/firmware/mmscfw.bin` (IRIX 6.4).
Image SHA-256: `58ac958cb0c3fa09f807340ec3f2f3864e3c601b3fc5f057ae9a8100db18a098`.
The [load map](load-map.md) describes the x86 a.out mapping at `0x108000`.
The analysed spans use the next text symbol, or the next global text symbol across local
labels, as an inferred end and can include literals
after returns. Statements below concern sites reached by known control-flow edges.
Unresolved computed jumps make the effects incomplete for the affected routines.
I/O prefixes in the access table identify ports rather than memory addresses.

`sp` denotes the entry stack pointer and brackets denote loaded values. Call candidates
use the assumed cdecl32 convention with unknown arity. Unknown expressions remain unknown.
The access tables include explicit memory operands, including stack records. Implicit
stack operations appear in the full fact sheets. Candidate slots can contain saved registers,
local values or stale stack words. Global and record meanings remain unknown
unless the entry establishes them.

[Fact sheets and linear listings](disasm/i8250-serial-6.4-facts.txt) include all tool sections.
Linear decoding can treat literals as instructions or stop at undecodable bytes.
The listings do not establish instruction boundaries in those ranges.

## `_i8250CallbackInstall`

- `_i8250CallbackInstall`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1745bc to 0x17460c.
  Established. The first branch takes `[sp+8] == 0x1` and falls through for inequality (`je` at
  0x1745cf); its taken arm writes `[sp+12]` at `[sp+4] +0x4` (`mov` at 0x1745dc) and `[sp+16]` at
  `[sp+4] +0xc` (`mov` at 0x1745df), both as 4-byte values. A second branch takes `[sp+8] == 0x2` and falls
  through for inequality (`je` at 0x1745d4). That taken arm writes 4-byte values `[sp+12]` at `[sp+4] +0x8`
  (`mov` at 0x1745ec) and `[sp+16]` at `[sp+4] +0x10` (`mov` at 0x1745ef). The callback meanings of
  those fields are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1745c0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1745c3 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x1745c6 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x1745c9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1745dc | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x1745df | store | 4 | `[sp+4] +0xc` | `[sp+16]` |
| `mov` at 0x1745ec | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x1745ef | store | 4 | `[sp+4] +0x10` | `[sp+16]` |
| `mov` at 0x174601 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1745cf | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x1745d4 | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x174607 | `eax` | `?` |

## `_i8250HrdInit`

- `_i8250HrdInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17460c to 0x17466c.
  Established. An indirect call with unknown target has leading candidates `[[sp+4]+48]` and `0x0`
  (`call` at 0x17461c). The branch takes `[0x1d954c] != 0x0` and falls through for zero (`jne` at
  0x174625); its fall-through stores include 4-byte value `0x1746cc` at absolute `0x1d954c` (`mov` at
  0x174627) and 4-byte value `0x1745bc` at absolute `0x1d9554` (`mov` at 0x17463b). A 4-byte store writes
  `0x1d954c` at `[sp+4] +0x0` (`mov` at 0x174659). The hardware initialization role is unknown
  beyond these stores and the unresolved callee.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x174610 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x174615 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x174619 | load | 4 | `[sp+4] +0x1c` | `-` |
| `cmp` at 0x17461e | load | 4 | `abs 0x1d954c` | `-` |
| `mov` at 0x174627 | store | 4 | `abs 0x1d954c` | `0x1746cc` |
| `mov` at 0x174631 | store | 4 | `abs 0x1d9550` | `0x1749dc` |
| `mov` at 0x17463b | store | 4 | `abs 0x1d9554` | `0x1745bc` |
| `mov` at 0x174645 | store | 4 | `abs 0x1d9558` | `0x1749fc` |
| `mov` at 0x17464f | store | 4 | `abs 0x1d955c` | `0x174a3c` |
| `mov` at 0x174659 | store | 4 | `[sp+4] +0x0` | `0x1d954c` |
| `mov` at 0x17465f | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17461c | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x0` | `ebx` | `ebp` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x174625 | `[0x1d954c] != 0x0` | `[0x1d954c] == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x174665 | `eax` | `ret(unknown)` |

## `_i8250InitChannel`

- `_i8250InitChannel`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17466c to 0x1746cc.
  Established. Calls `_intLock` (`call` at 0x174674), followed by indirect calls whose leading
  candidate pairs include `[[sp+4]+32]`, `0x3` (`call` at 0x174684) and `[[sp+4]+40]`, `0xb` (`call`
  at 0x174692). A later indirect call presents `[[sp+4]+48]` and `0x1` (`call` at 0x1746ac). Each
  indirect target is unknown (`call` at 0x174684, `call` at 0x174692, `call` at 0x1746ac).
  `_intUnlock` has leading candidate `ret(_intLock)` (`call` at 0x1746b2), and the exit returns
  `ret(_intUnlock)` (`ret` at 0x1746bf). The channel configuration encoded by those calls remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x174671 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17467d | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x174681 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17468b | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17468f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x174697 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x17469b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x1746a5 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1746a9 | load | 4 | `[sp+4] +0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x174674 | `_intLock 0x170618` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x174684 | `(unknown callee) (unknown target)` | `[[sp+4]+32]` | `0x3` | `?` | `?` | `?` | `?` |
| `call` at 0x174692 | `(unknown callee) (unknown target)` | `[[sp+4]+40]` | `0xb` | `?` | `?` | `?` | `?` |
| `call` at 0x17469e | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1746ac | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x1746b2 | `_intUnlock 0x170628` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1746bf | `eax` | `ret(_intUnlock)` |

## `_i8250Int`

- `_i8250Int`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1748fc to 0x1749dc.
  Established. The first indirect call has leading candidate `[[sp+4]+52]` and an unknown target
  (`call` at 0x174910). A branch takes `[[sp+4]+8] == 0x0` and falls through for a nonzero value
  (`je` at 0x174935). Another indirect call has leading candidates `[[sp+4]+12]` and `sp-5` (`call`
  at 0x174982). The final loop decision takes `le flags condition (operands unknown)` and falls
  through for its negation (`jle` at 0x1749c3). The interrupt causes and callback identities remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x174904 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x174909 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17490d | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x174920 | load | 4 | `[sp+4] +0x24` | `-` |
| `cmp` at 0x174931 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x174937 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x17493b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x174949 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x17494d | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x17495c | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x174960 | load | 4 | `[sp+4] +0x18` | `-` |
| `cmp` at 0x174971 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x17497b | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x17497f | load | 4 | `[sp+4] +0x4` | `-` |
| `movsx` at 0x17498b | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x174990 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x17499e | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1749a2 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x1749aa | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1749ae | load | 4 | `[sp+4] +0x18` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x174910 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `ebx` | `esi` | `?` | `ebp` | `?` |
| `call` at 0x17493e | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x174950 | `(unknown callee) (unknown target)` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x174963 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x174982 | `(unknown callee) (unknown target)` | `[[sp+4]+12]` | `sp-5` | `?` | `?` | `?` | `?` |
| `call` at 0x1749a5 | `(unknown callee) (unknown target)` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1749b1 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17491e | `? != 0x6` | `? == 0x6` |
| `jne` at 0x17492f | `? != 0x4` | `? == 0x4` |
| `je` at 0x174935 | `[[sp+4]+8] == 0x0` | `[[sp+4]+8] != 0x0` |
| `jne` at 0x17496f | `? != 0x2` | `? == 0x2` |
| `je` at 0x174975 | `[[sp+4]+4] == 0x0` | `[[sp+4]+4] != 0x0` |
| `jne` at 0x174989 | `(ret(unknown) & ret(unknown)) != 0x0` | `(ret(unknown) & ret(unknown)) == 0x0` |
| `jne` at 0x1749bb | `(? & 0x1) != 0x0` | `(? & 0x1) == 0x0` |
| `jle` at 0x1749c3 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1749d1 | `eax` | `?` |

## `_i8250Ioctl`

- `_i8250Ioctl`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1746cc to 0x1748fc.
  Established. The unsigned branch takes `[sp+8]-4099 > 0x6` and falls through for `[sp+8]-4099 <=
  0x6` (`ja` at 0x1746e3). The taken route reaches an exit with EAX `0x47` (`ret` at 0x1748ec). The
  fall-through route reaches an unresolved computed transfer (`jmp` at 0x1746e9). The
  control-request role and effects behind that selector are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1746d5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1746d8 | load | 4 | `sp +0x8` | `-` |
| `jmp` at 0x1746e9 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1746e3 | `[sp+8]-4099 > 0x6 (unsigned, 32-bit)` | `[sp+8]-4099 <= 0x6 (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1748ec | `eax` | `0x47` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1746e9).

## `_i8250PRxChar`

- `_i8250PRxChar`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1749fc to 0x174a3c.
  Established. An indirect call with unknown target presents leading candidate `[[sp+4]+36]` (`call`
  at 0x174a0e). The branch takes `(? & 0x1) == 0x0` and falls through when that bit expression is
  nonzero (`je` at 0x174a15). The fall-through includes another indirect call with leading candidate
  `[[sp+4]+64]` (`call` at 0x174a1e) and a byte store of unknown value at `[sp+8] +0x0` (`mov` at
  0x174a20). The input-device meaning and merged result remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x174a01 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x174a04 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x174a07 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x174a0b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x174a17 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x174a1b | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x174a20 | store | 1 | `[sp+8] +0x0` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x174a0e | `(unknown callee) (unknown target)` | `[[sp+4]+36]` | `ebx` | `esi` | `ebp` | `?` | `?` |
| `call` at 0x174a1e | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x174a15 | `(? & 0x1) == 0x0` | `(? & 0x1) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x174a39 | `eax` | `?` |

## `_i8250PTxChar`

- `_i8250PTxChar`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x174a3c to 0x174a88.
  Established. A byte store saves `[sp+8]:1` at `sp -0x8` (`mov` at 0x174a49). The first indirect
  call has leading candidate `[[sp+4]+36]` (`call` at 0x174a53). The branch takes `(? & 0x40) ==
  0x0` and falls through for a nonzero expression (`je` at 0x174a5a). The other indirect call has
  leading candidate `[[sp+4]+64]` and unknown remaining candidates (`call` at 0x174a68). Both
  targets and the transmitted-value semantics remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x174a43 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x174a46 | load | 1 | `sp +0x8` | `-` |
| `mov` at 0x174a49 | store | 1 | `sp -0x8` | `[sp+8]:1` |
| `mov` at 0x174a4c | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x174a50 | load | 4 | `[sp+4] +0x18` | `-` |
| `movsx` at 0x174a5c | load | 1 | `sp -0x8` | `-` |
| `mov` at 0x174a61 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x174a65 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x174a81 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x174a53 | `(unknown callee) (unknown target)` | `[[sp+4]+36]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x174a68 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x174a5a | `(? & 0x40) == 0x0` | `(? & 0x40) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x174a87 | `eax` | `?` |

## `_i8250Startup`

- `_i8250Startup`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1749dc to 0x1749fc.
  Established. The branch takes `([[sp+4]+22]:2 & 0xffff) == 0x1` and falls through for inequality
  (`je` at 0x1749e7). Only the fall-through contains the indirect call, with leading candidates
  `[[sp+4]+48]` and `0x3` and an unknown target (`call` at 0x1749f2). The exit returns EAX zero
  (`ret` at 0x1749f9). The startup state represented by the compared halfword remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1749df | load | 4 | `sp +0x4` | `-` |
| `cmp` at 0x1749e2 | load | 2 | `[sp+4] +0x16` | `-` |
| `mov` at 0x1749eb | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1749ef | load | 4 | `[sp+4] +0x1c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1749f2 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `0x3` | `ebp` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1749e7 | `([[sp+4]+22]:2 & 0xffff) == 0x1` | `([[sp+4]+22]:2 & 0xffff) != 0x1` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1749f9 | `eax` | `0x0` |

## `_sysSerialChanGet`

- `_sysSerialChanGet`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b788 to 0x10b7a8.
  Established. Two 4-byte reads use `sp +0x4` (`mov` at 0x10b78b, `add` at 0x10b791). The resulting
  address expression is unknown at the EAX return (`ret` at 0x10b79e). The channel lookup role
  cannot be established from that unresolved value.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b78b | load | 4 | `sp +0x4` | `-` |
| `add` at 0x10b791 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b79e | `eax` | `?` |

## `_sysSerialHwInit`

- `_sysSerialHwInit`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b618 to 0x10b718.
  Established. A 4-byte store initializes `sp -0x8` to zero (`mov` at 0x10b621). Later 4-byte stores
  have unknown effective targets and values `0x1081d0` (`mov` at 0x10b6ca) and `0x1081b0` (`mov` at
  0x10b6d2). The `_i8250HrdInit` call has unknown candidates (`call` at 0x10b6e1). The signed branch
  takes `[sp-8] <= 0x5` and falls through for larger values (`jle` at 0x10b6ff). The channel-record
  addresses and physical initialization effects remain unknown after loop merging.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b621 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x10b63c | load | 2 | `(unknown)` | `-` |
| `mov` at 0x10b643 | store | 2 | `(unknown)` | `?` |
| `mov` at 0x10b648 | store | 2 | `(unknown)` | `0x0` |
| `movzx` at 0x10b64f | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b657 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b65b | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b65f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b663 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b667 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b66b | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b66f | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b674 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b678 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b67c | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b681 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b685 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b689 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b690 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b694 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b698 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6a0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6a4 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6a8 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6b0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6b4 | store | 4 | `(unknown)` | `?` |
| `movzx` at 0x10b6b8 | load | 2 | `(unknown)` | `-` |
| `add` at 0x10b6c2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x10b6c6 | store | 4 | `(unknown)` | `?` |
| `mov` at 0x10b6ca | store | 4 | `(unknown)` | `0x1081d0` |
| `mov` at 0x10b6d2 | store | 4 | `(unknown)` | `0x1081b0` |
| `mov` at 0x10b6db | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x10b6de | store | 4 | `sp -0x10` | `?` |
| `mov` at 0x10b6e9 | load | 4 | `sp -0x10` | `-` |
| `inc` at 0x10b6f5 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b6f5 | store | 4 | `sp -0x8` | `?` |
| `mov` at 0x10b6f8 | load | 4 | `sp -0xc` | `-` |
| `cmp` at 0x10b6fb | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b6e1 | `_i8250HrdInit 0x17460c` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jle` at 0x10b6ff | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b70e | `eax` | `ret(_i8250HrdInit)` |

## `_sysSerialHwInit2`

- `_sysSerialHwInit2`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b718 to 0x10b788.
  Established. The branch takes `Z == 1` and falls through for `not (Z == 1)` (`je` at 0x10b73c).
  The fall-through contains `_intConnect`, whose candidates begin unknown, `0x1748fc`, and unknown
  (`call` at 0x10b74f), followed by `_sysIntEnablePIC` with unknown candidates (`call` at 0x10b75f).
  The loop branch takes `[sp-8] <= 0x5 (signed, 32-bit)` and falls through for greater values (`jle`
  at 0x10b777). Which channel and interrupt line each pass configures is unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b721 | store | 4 | `sp -0x8` | `0x0` |
| `cmp` at 0x10b734 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b744 | load | 2 | `(unknown)` | `-` |
| `movzx` at 0x10b757 | load | 2 | `(unknown)` | `-` |
| `inc` at 0x10b770 | load | 4 | `sp -0x8` | `-` |
| `inc` at 0x10b770 | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x10b773 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b74f | `_intConnect 0x174a88` | `?` | `0x1748fc` | `?` | `?` | `?` | `?` |
| `call` at 0x10b75f | `_sysIntEnablePIC 0x108428` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10b73c | `Z == 1` | `not (Z == 1)` |
| `jle` at 0x10b777 | `[sp-8] <= 0x5 (signed, 32-bit)` | `[sp-8] > 0x5 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b782 | `eax` | `?` |

## `_sysInByte`

- `_sysInByte`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. Performs an I/O byte read with unknown effective port (`in` at 0x1081b9). The EAX
  return value is also unknown (`ret` at 0x1081bc). The role is a byte input primitive, with device
  selection unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081b0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081b9 | load | 1 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081bc | `eax` | `?` |

## `_sysInWord`

- `_sysInWord`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. Reads two bytes from an I/O port whose effective address is unknown (`in` at
  0x1081c9). EAX remains unknown at the exit (`ret` at 0x1081cd). The access establishes a word
  input primitive without identifying the peripheral.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081c0 | load | 4 | `sp +0x4` | `-` |
| `in` at 0x1081c9 | load | 2 | `I/O (unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081cd | `eax` | `?` |

## `_sysOutByte`

- `_sysOutByte`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. Writes one byte of unknown value to an unresolved I/O port (`out` at 0x1081d8). The
  return carries `[sp+8]` in EAX (`ret` at 0x1081db). The byte output operation is established,
  while its destination device and emitted value are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081d0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081d4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081d8 | store | 1 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081db | `eax` | `[sp+8]` |

## `_sysOutWord`

- `_sysOutWord`, `6.4/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. An I/O store transfers two bytes, with both effective port and stored value unknown
  (`out` at 0x1081e8). The exit retains `[sp+8]` in EAX (`ret` at 0x1081ec). This is a word output
  primitive whose device mapping remains unresolved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081e0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081e4 | load | 4 | `sp +0x8` | `-` |
| `out` at 0x1081e8 | store | 2 | `I/O (unknown)` | `?` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081ec | `eax` | `[sp+8]` |
