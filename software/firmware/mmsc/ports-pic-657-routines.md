# MMSC port and interrupt callers in the 6.5.7 image

Read from `6.5.7/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `6c3db7ffd7fb4cc43ed02146f0c5bea0e85338def2530689c9990f877f2ddedc`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Ten analysed spans have exclusive ends inferred from the next text symbol.
The four single-port wrappers and `_sysWait` instead use explicit 16-byte bounds
across local continuation symbols.
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

[Fact sheets and complete listings](disasm/ports-pic-657-facts.txt) include all tool sections.

## `_ns16550PollInput`

- `_ns16550PollInput`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a13c to 0x17a19c.
  Established. The first indirect site has unknown callee and candidate zero `[[sp+4]+72]` (`call`
  at 0x17a150). The branch is taken when `(? & ?) != 0x0` (`jne` at 0x17a15f),
  reaching the later call with candidate zero `[[sp+4]+48]` (`call` at 0x17a179). A one-byte unknown
  value is then stored at `[sp+8] +0x0` (`mov` at 0x17a183). Input status meaning and callback
  identities are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a143 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a146 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a149 | load | 4 | `[sp+4] +0x48` | `-` |
| `mov` at 0x17a14d | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17a155 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x17a158 | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x17a16c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a16f | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a172 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x17a176 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17a180 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17a183 | store | 1 | `[sp+8] +0x0` | `?` |
| `mov` at 0x17a18c | load | 4 | `sp -0xc` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17a150 | `(unknown callee) (unknown target)` | `[[sp+4]+72]` | `ebx` | `?` | `ebp` | `?` | `?` |
| `call` at 0x17a179 | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17a15f | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a192 | `eax` | `?` |

## `_ns16550PollOutput`

- `_ns16550PollOutput`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x17a0cc to 0x17a13c.
  Established. The first unknown callee receives candidate zero `[[sp+4]+72]` (`call` at 0x17a0e7).
  Two byte stores at `sp -0x5` and `sp -0x6` have unknown values (`mov` at 0x17a0d7, `mov` at
  0x17a0ec). The branch is taken when `(? & ?) != 0x0` (`jne` at 0x17a0f6),
  reaching another unknown callee with candidate zero `[[sp+4]+48]` and unknown candidate one
  (`call` at 0x17a11e). The output role is Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x17a0d4 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x17a0d7 | store | 1 | `sp -0x5` | `?` |
| `mov` at 0x17a0da | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a0dd | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a0e0 | load | 4 | `[sp+4] +0x48` | `-` |
| `mov` at 0x17a0e4 | load | 4 | `[sp+4] +0x28` | `-` |
| `mov` at 0x17a0ec | store | 1 | `sp -0x6` | `?` |
| `mov` at 0x17a0ef | load | 1 | `sp -0x6` | `-` |
| `mov` at 0x17a10c | load | 4 | `sp +0x4` | `-` |
| `movsx` at 0x17a10f | load | 1 | `sp -0x5` | `-` |
| `mov` at 0x17a114 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x17a117 | load | 4 | `[sp+4] +0x30` | `-` |
| `mov` at 0x17a11b | load | 4 | `[sp+4] +0x2c` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x17a0e7 | `(unknown callee) (unknown target)` | `[[sp+4]+72]` | `ebx` | `esi` | `?` | `ebp` | `?` |
| `call` at 0x17a11e | `(unknown callee) (unknown target)` | `[[sp+4]+48]` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x17a0f6 | `(? & ?) != 0x0` | `(? & ?) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x17a134 | `eax` | `?` |

## `_ns16550DummyCallback`

- `_ns16550DummyCallback`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1790ec to 0x17910c.
  Established. The assignment puts `0xffffffff` into `eax` (`mov` at 0x1790ef), and that literal
  reaches the exit (`ret` at 0x1790ff). No explicit access, call or conditional branch is reported.
  Callback meaning is not established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1790ff | `eax` | `0xffffffff` |

## `_sysInByte`

- `_sysInByte`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081b0 to 0x1081c0.
  Established. The entry reads four bytes at `sp +0x4` (`mov` at 0x1081b0); its one-byte I/O load
  has an unknown effective port (`in` at 0x1081b9), and the exit expression remains unknown (`ret`
  at 0x1081bc). The explicit bound includes the local return continuation. No specific input device
  is identified.

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

- `_sysInWord`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081c0 to 0x1081d0.
  Established. A four-byte entry load reads `sp +0x4` (`mov` at 0x1081c0). The I/O read width is two
  bytes, with the effective port unresolved (`in` at 0x1081c9). The local continuation reaches a
  return with unknown expression (`ret` at 0x1081cd). The port function is Unknown.

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

- `_sysOutByte`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081d0 to 0x1081e0.
  Established. Two four-byte loads read `sp +0x4` and `sp +0x8` (`mov` at 0x1081d0, `mov` at
  0x1081d4). The one-byte I/O store has unknown port and value (`out` at 0x1081d8). The return
  register retains `[sp+8]` (`ret` at 0x1081db). The local continuation is included, while hardware
  meaning stays Unknown.

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

- `_sysOutWord`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081e0 to 0x1081f0.
  Established. The explicit two-byte I/O store has an unknown port and value (`out` at 0x1081e8);
  its prior four-byte loads read `sp +0x4` and `sp +0x8` (`mov` at 0x1081e0, `mov` at 0x1081e4).
  The return carries `[sp+8]` (`ret` at 0x1081ec). No register identity follows from this body.

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

## `_sysInWordString`

- `_sysInWordString`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081f0 to 0x108210.
  Established. Entry loads read four-byte values at `sp +0x4`, `+0x8`, `+0xc` (`mov` at 0x1081f1,
  `mov` at 0x1081f5, `mov` at 0x1081f9). The repeated string instruction has a two-byte access
  classified unknown at an unknown target (`rep insw` at 0x1081fe); its exit value is unknown (`ret`
  at 0x108204). Transfer extent and memory-update behavior are not resolved by the tool.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1081f1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1081f5 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x1081f9 | load | 4 | `sp +0xc` | `-` |
| `rep insw` at 0x1081fe | unknown | 2 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108204 | `eax` | `?` |

## `_sysOutWordString`

- `_sysOutWordString`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108210 to 0x108230.
  Established. The three four-byte entry loads read `sp +0x4`, `+0x8`, `+0xc` (`mov` at 0x108211,
  `mov` at 0x108215, `mov` at 0x108219). The repeated string instruction is classified as an unknown
  two-byte access at `[sp+8] +0x0` (`rep outsw` at 0x10821e). The return expression remains unknown
  (`ret` at 0x108224). The access does not establish a complete list of output stores or a transfer
  extent.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x108211 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108215 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x108219 | load | 4 | `sp +0xc` | `-` |
| `rep outsw` at 0x10821e | unknown | 2 | `[sp+8] +0x0` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108224 | `eax` | `?` |

## `_sysIntInitPIC`

- `_sysIntInitPIC`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1082f8 to 0x108398.
  Established. The `_sysOutByte` sequence includes candidate pairs `0x20`, `0x11` (`call` at
  0x1082ff), `0x21`, `0x4` (`call` at 0x10831d), `0x21`, `0x1` (`call` at 0x108329), and
  `0xa0`, `0x11` (`call` at 0x108338). Later pairs are `0xa1`, `0x2` (`call` at 0x108360),
  `0xa1`, `0x1` (`call` at 0x10836f), `0x21`, `0xfffffffb` (`call` at 0x10837b), and
  `0xa1`, `0xffffffff` (`call` at 0x10838a). Two other sites have candidate zero `0x21` and
  `0xa1` with unknown second slots (`call` at 0x108311, `call` at 0x108351).
  Peripheral initialization semantics are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `movsx` at 0x108307 | load | 1 | `abs 0x1afe20` | `-` |
| `mov` at 0x108340 | load | 1 | `abs 0x1afe20` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1082ff | `_sysOutByte 0x1081d0` | `0x20` | `0x11` | `ebp` | `?` | `?` | `?` |
| `call` at 0x108311 | `_sysOutByte 0x1081d0` | `0x21` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10831d | `_sysOutByte 0x1081d0` | `0x21` | `0x4` | `?` | `?` | `?` | `?` |
| `call` at 0x108329 | `_sysOutByte 0x1081d0` | `0x21` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x108338 | `_sysOutByte 0x1081d0` | `0xa0` | `0x11` | `?` | `?` | `?` | `?` |
| `call` at 0x108351 | `_sysOutByte 0x1081d0` | `0xa1` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x108360 | `_sysOutByte 0x1081d0` | `0xa1` | `0x2` | `?` | `?` | `?` | `?` |
| `call` at 0x10836f | `_sysOutByte 0x1081d0` | `0xa1` | `0x1` | `?` | `?` | `?` | `?` |
| `call` at 0x10837b | `_sysOutByte 0x1081d0` | `0x21` | `0xfffffffb` | `?` | `?` | `?` | `?` |
| `call` at 0x10838a | `_sysOutByte 0x1081d0` | `0xa1` | `0xffffffff` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108392 | `eax` | `ret(_sysOutByte)` |

## `_sysIntEOI`

- `_sysIntEOI`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108398 to 0x1083b8.
  Established. The two `_sysOutByte` calls have leading pairs `0x20`, `0x20` and `0xa0`, `0x20`
  (`call` at 0x10839f, `call` at 0x1083ae). The last call-result expression reaches the exit (`ret`
  at 0x1083b6). No explicit memory access or conditional branch appears. Interrupt acknowledgment is
  not established in this caller.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10839f | `_sysOutByte 0x1081d0` | `0x20` | `0x20` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ae | `_sysOutByte 0x1081d0` | `0xa0` | `0x20` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1083b6 | `eax` | `ret(_sysOutByte)` |

## `_sysIntDisablePIC`

- `_sysIntDisablePIC`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1083b8 to 0x108428.
  Established. The signed branch takes the alternate arm when `[sp+4] > 0x7 (signed, 32-bit)` (`jg`
  at 0x1083c2). The two arms call `_sysInByte` with candidate zero `0x21` and `0xa1` respectively
  (`call` at 0x1083c6, `call` at 0x1083ed). Their shared `_sysOutByte` site has only unknown
  candidate slots (`call` at 0x10840c). The return is zero (`ret` at 0x108419). Mask semantics
  remain Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1083bc | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108413 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x1083c6 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1083ed | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10840c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x1083c2 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108419 | `eax` | `0x0` |

## `_sysIntEnablePIC`

- `_sysIntEnablePIC`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108428 to 0x108498.
  Established. The deciding branch falls through when `[sp+4] <= 0x7 (signed, 32-bit)` (`jg` at
  0x108432). The `_sysInByte` sites carry candidate zero `0x21` and `0xa1` (`call` at 0x108436,
  `call` at 0x10845d). The shared `_sysOutByte` call retains unknown candidate slots (`call` at
  0x10847c), and the exit reports zero (`ret` at 0x108489). Mask roles are Unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10842c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x108483 | load | 4 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x108436 | `_sysInByte 0x1081b0` | `0x21` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10845d | `_sysInByte 0x1081b0` | `0xa1` | `ebx` | `ebp` | `?` | `?` | `?` |
| `call` at 0x10847c | `_sysOutByte 0x1081d0` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x108432 | `[sp+4] > 0x7 (signed, 32-bit)` | `[sp+4] <= 0x7 (signed, 32-bit)` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108489 | `eax` | `0x0` |

## `_sysWait`

- `_sysWait`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1081a0 to 0x1081b0.
  Established. A one-byte I/O load reads absolute port `0x64` (`in` at 0x1081a7). The looping branch
  retains an unknown flags condition in both printed directions (`loopne` at 0x1081aa); its return
  expression is unknown (`ret` at 0x1081ac). The given span crosses the local continuation, without
  establishing a wait duration or device role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `in` at 0x1081a7 | load | 1 | `I/O abs 0x64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `loopne` at 0x1081aa | `oopne flags condition (operands unknown)` | `not (oopne flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1081ac | `eax` | `?` |

## `_sysClkRateGet`

- `_sysClkRateGet`, `6.5.7/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x1085a8 to 0x1085b8.
  Established. The four-byte load reads absolute `0x1afe64` (`mov` at 0x1085ab); its value
  expression `[0x1afe64]` is retained at the return (`ret` at 0x1085b3). There are no call or
  conditional branch rows, and the stored value has no proven rate unit.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x1085ab | load | 4 | `abs 0x1afe64` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x1085b3 | `eax` | `[0x1afe64]` |
