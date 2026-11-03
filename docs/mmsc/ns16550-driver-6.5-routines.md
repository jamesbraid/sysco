# MMSC NS16550 driver in IRIX 6.5

Read from `6.5/usr/cpu/firmware/mmscfw.bin` (IRIX 6.5).
Image SHA-256: `2017f743fb7360a160f4252b69569b7f9713d9efad82719cfb2fd82538e37ff8`.
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

[Fact sheets and full listings](disasm/ns16550-driver-6.5-facts.txt) include all tool sections.

## `_ns16550CallbackInstall`

- `_ns16550CallbackInstall`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x178aec to 0x178b60.
  Established. The branch takes `[sp+8] == 0x1` and falls through for inequality (`je` at 0x178afe);
  its taken arm writes four-byte `[sp+12]` at `[sp+4] +0x4` (`mov` at 0x178b12) and `[sp+16]` at
  `[sp-8] +0xc` (`mov` at 0x178b1b). The second branch takes `[sp+8] == 0x2` and falls through for
  inequality (`je` at 0x178b03); its taken arm writes four-byte `[sp+12]` at `[sp+4] +0x8` (`mov` at
  0x178b32) and `[sp+16]` at `[sp-8] +0x10` (`mov` at 0x178b3b). The callback-field meanings remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178af2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178af5 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x178af8 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x178b0c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x178b0f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x178b12 | store | 4 | `[sp+4] +0x4` | `[sp+12]` |
| `mov` at 0x178b15 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x178b18 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x178b1b | store | 4 | `[sp-8] +0xc` | `[sp+16]` |
| `mov` at 0x178b2c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x178b2f | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x178b32 | store | 4 | `[sp+4] +0x8` | `[sp+12]` |
| `mov` at 0x178b35 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x178b38 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x178b3b | store | 4 | `[sp-8] +0x10` | `[sp+16]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x178afe | `[sp+8] == 0x1` | `[sp+8] != 0x1` |
| `je` at 0x178b03 | `[sp+8] == 0x2` | `[sp+8] != 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178b5f | `eax` | `?` |

## `_ns16550DevInit`

- `_ns16550DevInit`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x177a5c to 0x177abc.
  Established. Four-byte stores write `0x1d554c` at `[sp+4] +0x0` (`mov` at 0x177a6f) and `0x177a3c`
  at `[sp+4] +0x4` (`mov` at 0x177a78) and `[sp+4] +0x8` (`mov` at 0x177a82). A two-byte store
  clears `[sp+4] +0x5c` (`mov` at 0x177a8c), and a four-byte store writes `0xf` at `[sp+4] +0x68`
  (`mov` at 0x177a95). The `_ns16550InitChannel` call has leading candidate `[sp+4]` (`call` at
  0x177aa0). The initialized device state and field interpretations are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x177a69 | store | 4 | `sp -0x8` | `ret(_intLock)` |
| `mov` at 0x177a6c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177a6f | store | 4 | `[sp+4] +0x0` | `0x1d554c` |
| `mov` at 0x177a75 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177a78 | store | 4 | `[sp+4] +0x4` | `0x177a3c` |
| `mov` at 0x177a7f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177a82 | store | 4 | `[sp+4] +0x8` | `0x177a3c` |
| `mov` at 0x177a89 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177a8c | store | 2 | `[sp+4] +0x5c` | `0x0` |
| `mov` at 0x177a92 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177a95 | store | 4 | `[sp+4] +0x68` | `0xf` |
| `mov` at 0x177a9c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177aa8 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x177a62 | `_intLock 0x173a98` | `?` | `ebp` | `?` | `?` | `?` | `?` |
| `call` at 0x177aa0 | `_ns16550InitChannel 0x177abc` | `[sp+4]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x177aac | `_intUnlock 0x173aa8` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x177ab7 | `eax` | `ret(_intUnlock)` |

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x177a3c to 0x177a5c.
  Established. Assigns EAX `0xffffffff` (`mov` at 0x177a3f), which reaches the exit (`ret` at
  0x177a4f). The constant-result body is established, while its callback service role remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x177a4f | `eax` | `0xffffffff` |

## `_ns16550Hup`

- `_ns16550Hup`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17803c to 0x17811c.
  Established. An indirect call with unknown target has leading candidates `[[sp+4]+52]` and `0x0`
  (`call` at 0x178057). Stores of unknown values include a byte at `[sp+4] +0x19` (`mov` at
  0x17806a), four bytes at `[sp+4] +0x1c` (`mov` at 0x17807b), and a byte at `[sp+4] +0x1a` (`mov`
  at 0x1780aa). The branch takes `([[sp+4]+92]:2 & 0xffff) != 0x2` and falls through for equality
  (`jne` at 0x1780d4). The hangup signaling and device-register mapping are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178048 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17804d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178050 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x178054 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17805c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17805f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178062 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17806a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17806d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178070 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178073 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17807b | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x17807e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178081 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178084 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x17808d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178090 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x178094 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17809c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17809f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780a2 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1780aa | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x1780ad | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780b0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780b3 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1780ba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780bd | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1780c1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1780c9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780cc | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x1780d6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780d9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780dc | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1780e3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1780e6 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1780ea | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178041 | `_intLock 0x173a98` | `ebx` | `esi` | `ebp` | `?` | `?` | `?` |
| `call` at 0x178057 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x178097 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1780c4 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1780ed | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1780f3 | `_intUnlock 0x173aa8` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x1780d4 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178114 | `eax` | `0x0` |

## `_ns16550InitChannel`

- `_ns16550InitChannel`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x177abc to 0x177dac.
  Established. The byte stores include `0x3` at `[sp+4] +0x1b` (`mov` at 0x177acc) and `0x7` at
  `[sp+4] +0x1a` (`mov` at 0x177b0d). The first indirect call has unknown target and leading
  candidates `[[sp+4]+64]` and `[sp-12]` (`call` at 0x177aff). The branch takes `[sp-8] == 0x0` and
  falls through for nonzero (`je` at 0x177d02). The merged exit retains `ret(unknown)` in EAX (`ret`
  at 0x177da8). The baud, interrupt and line-control interpretations remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x177ac3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ac6 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177ac9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177acc | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x177ad0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ad3 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177ad6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ad9 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177adc | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177adf | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177ae5 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177ae8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177aec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177aef | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177af2 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177af5 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x177af9 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177afc | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177b04 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b07 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177b0a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177b0d | store | 1 | `[sp+4] +0x1a` | `0x7` |
| `mov` at 0x177b11 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b14 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177b17 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b1a | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177b1d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b20 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x177b26 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177b29 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b2d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b30 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177b33 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b36 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x177b3a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177b3d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177b45 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b48 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177b4b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177b4e | store | 1 | `[sp+4] +0x1a` | `0x89` |
| `mov` at 0x177b52 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b55 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177b58 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b5b | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177b5e | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b61 | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x177b67 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177b6a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b6e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b71 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177b74 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b77 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x177b7b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177b7e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177b86 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b89 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177b8c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177b8f | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177b92 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177b95 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177b9c | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x177b9f | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177ba4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ba7 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177baa | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177bad | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x177bb1 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177bb4 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177bbc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177bbf | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177bc2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177bc5 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177bc8 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177bcb | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x177bce | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x177bd5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177be1 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177be4 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177be7 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177bea | store | 4 | `[sp-8] +0x20` | `[sp-12]` |
| `mov` at 0x177bed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177bf0 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177bf3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177bf6 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177bf9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177bfc | load | 4 | `[sp+4] +0x64` | `-` |
| `mov` at 0x177bff | store | 4 | `sp -0xc` | `0x1b4e81b5` |
| `mov` at 0x177c06 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c12 | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177c15 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c1b | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177c1e | store | 4 | `[sp-8] +0x24` | `?` |
| `mov` at 0x177c21 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c24 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177c27 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c2a | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177c2d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c30 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x177c33 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x177c36 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177c3b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c3e | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177c41 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c44 | load | 4 | `[sp+4] +0x54` | `-` |
| `mov` at 0x177c48 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177c4b | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177c53 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c56 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177c59 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c5c | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177c5f | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c62 | load | 4 | `[sp+4] +0x24` | `-` |
| `mov` at 0x177c65 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x177c68 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177c6d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c70 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177c73 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c76 | load | 4 | `[sp+4] +0x58` | `-` |
| `mov` at 0x177c7a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177c7d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177c85 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c88 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177c8b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177c8e | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177c91 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177c94 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177c9a | store | 4 | `sp -0xc` | `?` |
| `mov` at 0x177c9d | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177ca1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ca4 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177ca7 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177caa | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x177cae | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177cb1 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177cb9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177cbc | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177cbf | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177cc2 | store | 1 | `[sp+4] +0x19` | `0x3` |
| `mov` at 0x177cc6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177cc9 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177cce | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177cd1 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177cd4 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177cd7 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x177cdb | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177cde | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177ce6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ce9 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177cec | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177cef | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x177cf2 | store | 4 | `sp -0xc` | `[[sp+4]+104]` |
| `mov` at 0x177cf5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177cfb | store | 4 | `sp -0x8` | `?` |
| `cmp` at 0x177cfe | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d04 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d07 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177d0a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d0d | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x177d1c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d1f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177d22 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d25 | store | 4 | `[sp+4] +0x1c` | `0xb` |
| `mov` at 0x177d2c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d2f | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177d32 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d35 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177d38 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177d3b | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177d42 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x177d45 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177d48 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d4b | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x177d4e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d51 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177d54 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d57 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177d5a | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177d5d | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177d64 | store | 1 | `sp -0xc` | `?` |
| `mov` at 0x177d67 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177d6a | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d6d | store | 1 | `[sp+4] +0x19` | `[sp-12]:1` |
| `mov` at 0x177d70 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d73 | store | 4 | `sp -0x8` | `[sp+4]` |
| `mov` at 0x177d76 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d79 | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177d7c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177d7f | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x177d82 | store | 1 | `sp -0xc` | `?` |
| `movsx` at 0x177d85 | load | 1 | `sp -0xc` | `-` |
| `mov` at 0x177d8a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177d8d | store | 4 | `sp -0xc` | `[sp+4]` |
| `mov` at 0x177d90 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x177d93 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x177d97 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x177d9a | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177da2 | load | 4 | `sp -0x10` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x177aff | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177b40 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177b81 | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177bb7 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177c4e | `(unknown callee) (unknown target)` | `[[sp+4]+84]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177c80 | `(unknown callee) (unknown target)` | `[[sp+4]+88]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177cb4 | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `[sp-12]` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177ce1 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `[sp+4]` | `[sp+4]` | `?` |
| `call` at 0x177d9d | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `[sp+4]` | `[sp+4]` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x177d02 | `[sp-8] == 0x0` | `[sp-8] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x177da8 | `eax` | `ret(unknown)` |

## `_ns16550Int`

- `_ns16550Int`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17877c to 0x17895c.
  Established. The first indirect call has unknown target and leading candidate `[[sp+4]+60]`
  (`call` at 0x178790). A second indirect call presents leading candidates `[[sp+4]+52]` and `0x0`
  (`call` at 0x1787ae). The unsigned branch takes `? > 0xc` and falls through for values at most
  `0xc`, with its operand unknown (`ja` at 0x1787bc). The fall-through reaches an unresolved
  computed transfer (`jmp` at 0x1787c9). The interrupt selector and effects beyond its unknown edges
  remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178783 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178786 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178789 | load | 4 | `[sp+4] +0x3c` | `-` |
| `mov` at 0x17878d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17879c | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x17879f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1787a4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1787a7 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1787ab | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1787b3 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1787c2 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x17892c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17892f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178932 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x178939 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17893c | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x178940 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x178948 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178790 | `(unknown callee) (unknown target)` | `[[sp+4]+60]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x1787ae | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x178943 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x1787bc | `? > 0xc (unsigned, 32-bit)` | `? <= 0xc (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17894e | `eax` | `ret(unknown)` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x1787c9).

## `_ns16550IntEx`

- `_ns16550IntEx`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17876c to 0x17877c.
  Established. The exit preserves entry EAX (`ret` at 0x178772). The short body establishes no
  interrupt-extension role, so its purpose remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178772 | `eax` | `eax` |

## `_ns16550IntRd`

- `_ns16550IntRd`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17872c to 0x17876c.
  Established. An indirect call with unknown target presents leading candidate `[[sp+4]+48]` (`call`
  at 0x178740). A byte store puts an unknown value at `sp -0x5` (`mov` at 0x178745). Another
  indirect call has leading candidate `[[sp+4]+16]` and unknown remaining candidates (`call` at
  0x17875a); its result reaches EAX at the exit (`ret` at 0x178765). The receive callback and byte
  interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178733 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178736 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178739 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x17873d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x178745 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x178748 | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x17874b | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x178750 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178753 | load | 4 | `[sp+4] +0x10` | `-` |
| `mov` at 0x178757 | load | 4 | `[sp+4] +0x8` | `-` |
| `mov` at 0x17875f | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178740 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x17875a | `(unknown callee) (unknown target)` | `[[sp+4]+16]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178765 | `eax` | `ret(unknown)` |

## `_ns16550IntWr`

- `_ns16550IntWr`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1786ac to 0x17872c.
  Established. The first indirect call has unknown target and leading candidates `[[sp+4]+12]` and
  `sp-5` (`call` at 0x1786c4). The branch takes `ret(unknown) == 0xffffffff` and falls through for
  inequality (`je` at 0x1786ce); its taken path includes a byte store of unknown value at `[sp+4]
  +0x19` (`mov` at 0x1786fa). The fall-through instead calls an unknown target with leading
  candidate `[[sp+4]+48]` (`call` at 0x1786e2). The transmit and status-bit meanings remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1786b3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1786ba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1786bd | load | 4 | `[sp+4] +0xc` | `-` |
| `mov` at 0x1786c1 | load | 4 | `[sp+4] +0x4` | `-` |
| `mov` at 0x1786d0 | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x1786d3 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1786d8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1786db | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x1786df | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1786ec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1786ef | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1786f2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1786fa | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1786fd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178700 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178703 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x17870a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17870d | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x178711 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x178719 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1786c4 | `(unknown callee) (unknown target)` | `[[sp+4]+12]` | `sp-5` | `ebx` | `?` | `ebp` | `?` |
| `call` at 0x1786e2 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x178714 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x1786ce | `ret(unknown) == 0xffffffff` | `ret(unknown) != 0xffffffff` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17871f | `eax` | `ret(unknown)` |

## `_ns16550Ioctl`

- `_ns16550Ioctl`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17821c to 0x1786ac.
  Established. The unsigned branch takes `[sp+8]-4099 > 0xa` and falls through for `[sp+8]-4099 <=
  0xa` (`ja` at 0x178236); its taken default route reaches an exit with EAX `0x47` (`ret` at
  0x1786a5). The fall-through reaches the unresolved computed transfer (`jmp` at 0x178249). The
  linear listing loses alignment after the inline table, and no control-request effects behind its
  unknown edges are established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178227 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17822f | store | 4 | `sp -0x8` | `[sp+8]-4099` |
| `cmp` at 0x178232 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17823c | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x17823f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x178246 | store | 4 | `sp -0x8` | `?` |
| `jmp` at 0x178249 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `ja` at 0x178236 | `[sp+8]-4099 > 0xa (unsigned, 32-bit)` | `[sp+8]-4099 <= 0xa (unsigned, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1786a5 | `eax` | `0x47` |

Coverage excludes unresolved edges from the computed transfer (`jmp` at 0x178249).

## `_ns16550Open`

- `_ns16550Open`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17811c to 0x17821c.
  Established. The first indirect call has unknown target and leading candidate `[[sp+4]+76]`
  (`call` at 0x178131). The branch takes `(? & ?) == 0x0` and falls through for a nonzero expression
  (`je` at 0x178147); its fall-through calls `_intLock` with unknown candidates (`call` at 0x17814d)
  and includes byte stores of unknown values at `[sp+4] +0x19` (`mov` at 0x178163, `mov` at
  0x178174). The exit returns EAX zero (`ret` at 0x178214). The opening handshake and register-bit
  semantics are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178124 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178127 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17812a | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x17812e | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17813d | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x178140 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x178152 | store | 4 | `sp -0xc` | `ret(_intLock)` |
| `mov` at 0x178155 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178158 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17815b | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x178163 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x178166 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178169 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17816c | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x178174 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x178177 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17817a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17817d | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x178185 | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x178188 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17818b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17818e | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x178197 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17819a | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x17819e | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1781a6 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781a9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781ac | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1781b4 | store | 1 | `[sp+4] +0x1a` | `?` |
| `mov` at 0x1781b7 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781ba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781bd | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x1781c4 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781c7 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x1781cb | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1781d3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781d6 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x1781e0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781e3 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781e6 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1781ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1781f0 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x1781f4 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x1781fc | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178131 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `esi` | `?` | `?` | `ebp` |
| `call` at 0x17814d | `_intLock 0x173a98` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1781a1 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1781ce | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1781f7 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x178200 | `_intUnlock 0x173aa8` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x178147 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x1781de | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178214 | `eax` | `0x0` |

## `_ns16550OptsSet`

- `_ns16550OptsSet`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x177dac to 0x17803c.
  Established. The first branch takes `[sp+4] == 0x0` and falls through for nonzero (`je` at
  0x177db5). Alternative paths include byte stores of `0x0` (`mov` at 0x177e0f), `0x1` (`mov` at
  0x177e1f), `0x2` (`mov` at 0x177e2f), or `0x3` (`mov` at 0x177e3f) at `[sp+4] +0x1b`. A later
  indirect call has unknown target and leading candidates `[[sp+4]+52]` and `0x0` (`call` at
  0x177f0b). The merged return value is unknown (`ret` at 0x178034). The option encodings and
  physical line configuration remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `cmp` at 0x177db1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177db7 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177ddc | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177e0c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e0f | store | 1 | `[sp+4] +0x1b` | `0x0` |
| `mov` at 0x177e1c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e1f | store | 1 | `[sp+4] +0x1b` | `0x1` |
| `mov` at 0x177e2c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e2f | store | 1 | `[sp+4] +0x1b` | `0x2` |
| `mov` at 0x177e3c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e3f | store | 1 | `[sp+4] +0x1b` | `0x3` |
| `mov` at 0x177e4c | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177e56 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e59 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e5c | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177e64 | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x177e6c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e6f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177e72 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177e75 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x177e78 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177eac | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177eaf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177eb2 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177eba | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x177ecc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ecf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ed2 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177eda | store | 1 | `[sp+4] +0x1b` | `?` |
| `mov` at 0x177eec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177eef | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ef2 | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177ef5 | store | 1 | `[sp+4] +0x1b` | `[[sp+4]+27]:1` |
| `mov` at 0x177efc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f01 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f04 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x177f08 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177f10 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f13 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f16 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x177f1e | store | 4 | `[sp+4] +0x1c` | `?` |
| `mov` at 0x177f21 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f24 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f27 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177f2f | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x177f32 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177f3c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f3f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f42 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177f4a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x177f5c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f5f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f62 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177f6a | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x177f74 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f77 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f7a | load | 1 | `[sp+4] +0x1b` | `-` |
| `mov` at 0x177f81 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f84 | load | 4 | `[sp+4] +0x40` | `-` |
| `mov` at 0x177f88 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177f90 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f93 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177f96 | load | 4 | `[sp+4] +0x1c` | `-` |
| `mov` at 0x177f9f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fa2 | load | 4 | `[sp+4] +0x44` | `-` |
| `mov` at 0x177fa6 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177fae | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fb1 | store | 1 | `[sp+4] +0x1a` | `0x87` |
| `mov` at 0x177fb5 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fb8 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fbb | load | 1 | `[sp+4] +0x1a` | `-` |
| `mov` at 0x177fc2 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fc5 | load | 4 | `[sp+4] +0x38` | `-` |
| `mov` at 0x177fc9 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x177fd1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x177fdb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fde | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fe1 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x177fe9 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x177fec | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fef | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x177ff9 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177ffc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x177fff | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x178006 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178009 | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x17800d | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x17801e | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178021 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x178024 | store | 4 | `[sp+4] +0x68` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x177f0b | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `0x0` | `?` | `?` | `?` | `?` |
| `call` at 0x177f6d | `_intLock 0x173a98` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x177f8b | `(unknown callee) (unknown target)` | `[[sp+4]+64]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x177fa9 | `(unknown callee) (unknown target)` | `[[sp+4]+68]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x177fcc | `(unknown callee) (unknown target)` | `[[sp+4]+56]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x178010 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x178016 | `_intUnlock 0x173aa8` | `ret(_intLock)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x177db5 | `[sp+4] == 0x0` | `[sp+4] != 0x0` |
| `jne` at 0x177dc1 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x177de5 | `? == 0x4` | `? != 0x4` |
| `ja` at 0x177dea | `? > 0x4 (unsigned, 32-bit)` | `? <= 0x4 (unsigned, 32-bit)` |
| `je` at 0x177dee | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x177dff | `? == 0x8` | `? != 0x8` |
| `je` at 0x177e04 | `? == 0xc` | `? != 0xc` |
| `je` at 0x177e54 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x177e83 | `? == 0x40` | `? != 0x40` |
| `ja` at 0x177e88 | `? > 0x40 (unsigned, 32-bit)` | `? <= 0x40 (unsigned, 32-bit)` |
| `je` at 0x177e8c | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x177ea1 | `? == 0xc0` | `? != 0xc0` |
| `jne` at 0x177f3a | `(? & ?) != 0x0` | `(? & ?) == 0x0` |
| `je` at 0x177fd9 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `jne` at 0x177ff7 | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178034 | `eax` | `?` |

## `_ns16550PollInput`

- `_ns16550PollInput`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x178a8c to 0x178aec.
  Established. The first indirect call has unknown target and leading candidate `[[sp+4]+72]`
  (`call` at 0x178aa0). The branch takes `(? & ?) != 0x0` and falls through for zero (`jne` at
  0x178aaf); its taken route calls an unknown target with leading candidate `[[sp+4]+48]` (`call` at
  0x178ac9), then writes an unknown byte at `[sp+8] +0x0` (`mov` at 0x178ad3). The input-ready test
  and device-data interpretation are unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178a93 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178a96 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178a99 | load | 4 | `[sp+4] +0x48` | `-` |
| `mov` at 0x178a9d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x178aa5 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x178aa8 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x178abc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178abf | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178ac2 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x178ac6 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x178ad0 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x178ad3 | store | 1 | `[sp+8] +0x0` | `?` |
| `mov` at 0x178adc | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178aa0 | `(unknown callee) (unknown target)` | `[[sp+4]+72]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x178ac9 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x178aaf | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178ae2 | `eax` | `?` |

## `_ns16550PollOutput`

- `_ns16550PollOutput`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x178a1c to 0x178a8c.
  Established. An indirect call with unknown target presents leading candidate `[[sp+4]+72]` (`call`
  at 0x178a37). The branch takes `(? & ?) != 0x0` and falls through for zero (`jne` at 0x178a46);
  its taken route includes another indirect call with leading candidate `[[sp+4]+48]` and unknown
  remaining candidates (`call` at 0x178a6e). The polling-status and output-byte semantics remain
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178a24 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x178a27 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x178a2a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178a2d | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178a30 | load | 4 | `[sp+4] +0x48` | `-` |
| `mov` at 0x178a34 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x178a3c | store | 1 | `sp -0x6` | `?` |
| `mov` at 0x178a3f | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x178a5c | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x178a5f | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x178a64 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178a67 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x178a6b | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x178a37 | `(unknown callee) (unknown target)` | `[[sp+4]+72]` | `ebx` | `esi` | `?` | `ebp` | `?` |
| `call` at 0x178a6e | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x178a46 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178a84 | `eax` | `?` |

## `_ns16550TxStartup`

- `_ns16550TxStartup`, `6.5/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17895c to 0x178a1c.
  Established. The branch takes `([[sp+4]+92]:2 & 0xffff) != 0x2` and falls through for equality
  (`jne` at 0x17896e). Later alternative paths include byte stores of unknown value at `[sp+4]
  +0x19` (`mov` at 0x178991, `mov` at 0x1789c8, `mov` at 0x1789ea). The final indirect call has
  unknown target and leading candidate `[[sp+4]+52]` (`call` at 0x178a04). The transmit-start policy
  and interrupt-mask meaning remain unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x178963 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178966 | load | 2 | `[sp+4] +0x5c` | `-` |
| `mov` at 0x178974 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178977 | load | 4 | `[sp+4] +0x68` | `-` |
| `mov` at 0x178983 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178986 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x178989 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x178991 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x17899c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17899f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789a2 | load | 4 | `[sp+4] +0x4c` | `-` |
| `mov` at 0x1789a6 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x1789ae | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x1789b1 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x1789ba | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789bd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789c0 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1789c8 | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1789dc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789df | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789e2 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1789ea | store | 1 | `[sp+4] +0x19` | `?` |
| `mov` at 0x1789ed | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789f0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789f3 | load | 1 | `[sp+4] +0x19` | `-` |
| `mov` at 0x1789fa | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1789fd | load | 4 | `[sp+4] +0x34` | `-` |
| `mov` at 0x178a01 | load | 4 | `[sp+4] +0x2c` | `-` |
| `mov` at 0x178a09 | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1789a9 | `(unknown callee) (unknown target)` | `[[sp+4]+76]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x178a04 | `(unknown callee) (unknown target)` | `[[sp+4]+52]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17896e | `([[sp+4]+92]:2 & 0xffff) != 0x2` | `([[sp+4]+92]:2 & 0xffff) == 0x2` |
| `je` at 0x178981 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |
| `je` at 0x1789b8 | `(? & ?) == 0x0` | `(? & ?) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x178a0f | `eax` | `?` |
