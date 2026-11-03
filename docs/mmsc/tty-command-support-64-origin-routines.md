# MMSC TTY and command-support routines in the 6.4-origin image

Read from `6.4-origin/usr/cpu/firmware/mmscfw.bin`.
Image SHA-256: `b766e559f86078526916d8bd98bbb8079294ad774cce9177928873153c6fb09b`.
The x86 a.out link address is `0x108000`. The [load map](load-map.md) explains segment mapping.
Analysed spans have exclusive ends inferred from the next text symbol.
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

[Fact sheets and complete listings](disasm/tty-command-support-64-origin-facts.txt) include all tool sections.

## `_sysBusIntGen`

- `_sysBusIntGen`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108818 to 0x108828.
  Established. The immediate assignment places `0xffffffff` in `eax` (`mov` at 0x10881b), and that
  literal reaches the exit (`ret` at 0x108823). No explicit memory-access or call rows are reported. An
  interrupt-generation effect remains unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108823 | `eax` | `0xffffffff` |

## `_sysMailboxConnect`

- `_sysMailboxConnect`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108828 to 0x108838.
  Established. The literal `0xffffffff` is assigned to `eax` (`mov` at 0x10882b) and retained at the
  exit (`ret` at 0x108833); its tables contain no calls or explicit memory accesses. No
  mailbox-connection effect is established.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108833 | `eax` | `0xffffffff` |

## `_sysMailboxEnable`

- `_sysMailboxEnable`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108838 to 0x108848.
  Established. The assignment writes `0xffffffff` to `eax` (`mov` at 0x10883b). The exit reports the
  same literal (`ret` at 0x108843), with no calls or explicit access rows in this body. A mailbox-enabling
  role is unproved.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108843 | `eax` | `0xffffffff` |

## `_sysBusTas`

- `_sysBusTas`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x108848 to 0x108858.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x10884b). The `_vxTas` site presents
  candidate zero `[sp+4]` (`call` at 0x10884f), and its result expression `ret(_vxTas)` reaches the
  exit (`ret` at 0x108857). Bus or atomic-operation semantics are not inferred from the callee name.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10884b | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10884f | `_vxTas 0x16e150` | `[sp+4]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x108857 | `eax` | `ret(_vxTas)` |

## `_ttyDevCreate`

- `_ttyDevCreate`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x18fd08 to 0x18fdd8.
  Established. The `_malloc` site presents candidate zero `0xe4` (`call` at 0x18fd3d). A four-byte
  store through `ret(_malloc) +0xe0` writes `[sp+8]` (`mov` at 0x18fd62). Three indirect calls have
  unknown callees (`call` at 0x18fd76, `call` at 0x18fd89, `call` at 0x18fd9a). Their first
  candidate is `[sp+8]` at each site (`call` at 0x18fd76, `call` at 0x18fd89, `call` at 0x18fd9a).
  Neither callback identity nor allocated-record size is inferred.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x18fd0e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x18fd11 | load | 4 | `sp +0x10` | `-` |
| `cmp` at 0x18fd14 | load | 4 | `abs 0x1d95f8` | `-` |
| `mov` at 0x18fd51 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x18fd62 | store | 4 | `ret(_malloc) +0xe0` | `[sp+8]` |
| `mov` at 0x18fd68 | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x18fd73 | load | 4 | `[[sp+8]] +0x8` | `-` |
| `mov` at 0x18fd7b | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x18fd86 | load | 4 | `[[sp+8]] +0x8` | `-` |
| `mov` at 0x18fd8e | load | 4 | `[sp+8] +0x0` | `-` |
| `mov` at 0x18fd98 | load | 4 | `[[sp+8]] +0x0` | `-` |
| `mov` at 0x18fd9f | load | 4 | `abs 0x1d95f8` | `-` |
| `mov` at 0x18fda5 | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18fd22 | `_errnoSet 0x16fae8` | `0xc0001` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x18fd3d | `_malloc 0x17c878` | `0xe4` | `ebx` | `esi` | `edi` | `ebp` | `?` |
| `call` at 0x18fd56 | `_tyDevInit 0x16e160` | `ret(_malloc)` | `[sp+12]` | `[sp+16]` | `0x18fe38` | `?` | `?` |
| `call` at 0x18fd76 | `(unknown callee) (unknown target)` | `[sp+8]` | `0x1` | `0x16e860` | `ret(_malloc)` | `?` | `?` |
| `call` at 0x18fd89 | `(unknown callee) (unknown target)` | `[sp+8]` | `0x2` | `0x16e970` | `ret(_malloc)` | `?` | `?` |
| `call` at 0x18fd9a | `(unknown callee) (unknown target)` | `[sp+8]` | `0x1007` | `0x2` | `?` | `?` | `?` |
| `call` at 0x18fdaa | `_iosDevAdd 0x16f210` | `ret(_malloc)` | `[sp+4]` | `[0x1d95f8]` | `?` | `?` | `?` |
| `call` at 0x18fdb9 | `_free 0x17c898` | `ret(_malloc)` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jg` at 0x18fd1b | `[0x1d95f8] > 0x0 (signed, 32-bit)` | `[0x1d95f8] <= 0x0 (signed, 32-bit)` |
| `je` at 0x18fd49 | `(ret(_malloc) & ret(_malloc)) == 0x0` | `(ret(_malloc) & ret(_malloc)) != 0x0` |
| `jne` at 0x18fd60 | `(ret(_tyDevInit) & ret(_tyDevInit)) != 0x0` | `(ret(_tyDevInit) & ret(_tyDevInit)) == 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18fdcc | `eax` | `?` |

## `_ttyOpen`

- `_ttyOpen`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x18fdd8 to 0x18fde8.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x18fddb), and the exit retains `[sp+4]`
  in `eax` (`ret` at 0x18fde1). No call appears in this body. The returned expression alone does not
  establish an opened device.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x18fddb | load | 4 | `sp +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18fde1 | `eax` | `[sp+4]` |

## `_ttyStartup`

- `_ttyStartup`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x18fe38 to 0x18fe50.
  Established. Four-byte loads read through `[sp+4] +0xe0` (`mov` at 0x18fe3e), `[[sp+4]+224] +0x0`
  (`mov` at 0x18fe44), and `[[[sp+4]+224]] +0x4` (`mov` at 0x18fe47). The indirect call has an
  unknown callee and candidate zero `[[sp+4]+224]` (`call` at 0x18fe4a). The exit reports
  `ret(unknown)` (`ret` at 0x18fe4f). Startup behavior remains unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x18fe3b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x18fe3e | load | 4 | `[sp+4] +0xe0` | `-` |
| `mov` at 0x18fe44 | load | 4 | `[[sp+4]+224] +0x0` | `-` |
| `mov` at 0x18fe47 | load | 4 | `[[[sp+4]+224]] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x18fe4a | `(unknown callee) (unknown target)` | `[[sp+4]+224]` | `ebp` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x18fe4f | `eax` | `ret(unknown)` |

## `_bufAppendAddMsgs`

- `_bufAppendAddMsgs`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10eea8 to 0x10efd8.
  Established. The byte store at `([[sp+8]+16] + [[sp+8]+4]) +0x0` writes `0x0` (`mov` at 0x10eecd).
  The first `_bufWrite` site presents `[sp+4]`, `[[sp+8]+4]`, and unknown as leading candidates
  (`call` at 0x10ef5a). The second presents `[sp+4]`, `[[sp+8]+4]`, and `[sp+12]` (`call` at
  0x10efa4). A four-byte store at `sp +0xc` retains an unknown value (`sub` at 0x10ef77). No
  transferred extent or message boundary is assigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10eeb1 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10eeb9 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10eebc | load | 4 | `[sp+16] +0x10` | `-` |
| `mov` at 0x10eec7 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10eeca | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x10eecd | store | 1 | `([[sp+8]+16] + [[sp+8]+4]) +0x0` | `0x0` |
| `mov` at 0x10eed1 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10eed4 | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10eedb | load | 4 | `[sp+4] +0x20` | `-` |
| `mov` at 0x10eeed | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef0c | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef12 | store | 4 | `sp -0x8` | `[sp+4]+36` |
| `mov` at 0x10ef16 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef28 | load | 4 | `[sp+8] +0x4` | `-` |
| `add` at 0x10ef2b | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ef32 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x10ef40 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef4e | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ef52 | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10ef56 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef68 | load | 4 | `[sp+8] +0x4` | `-` |
| `sub` at 0x10ef77 | load | 4 | `sp +0xc` | `-` |
| `sub` at 0x10ef77 | store | 4 | `sp +0xc` | `?` |
| `mov` at 0x10ef7a | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef7d | load | 4 | `[sp+4] +0x18` | `-` |
| `mov` at 0x10ef84 | load | 4 | `sp +0x10` | `-` |
| `mov` at 0x10ef88 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10ef98 | load | 4 | `sp +0xc` | `-` |
| `mov` at 0x10ef9c | load | 4 | `[sp+8] +0x4` | `-` |
| `mov` at 0x10efa0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10efb2 | load | 4 | `[sp+8] +0x4` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10eee3 | `_strlen 0x172f14` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10eef1 | `_strncmp 0x1749b8` | `[[sp+8]+4]` | `([sp+4] + [[sp+4]+32])+36` | `ret(_strlen)` | `?` | `?` | `?` |
| `call` at 0x10eefe | `_strlen 0x172f14` | `([sp+4] + [[sp+4]+32])+36` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef1a | `_strstr 0x17e2c0` | `[[sp+8]+4]` | `[sp+4]+36` | `?` | `?` | `?` | `[sp+4]+36` |
| `call` at 0x10ef36 | `_strlen 0x172f14` | `[sp-8]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef5a | `_bufWrite 0x10fb78` | `[sp+4]` | `[[sp+8]+4]` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef6f | `_bufLeftAlign 0x10f838` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |
| `call` at 0x10ef8c | `_bufAppendMsgs 0x10f1c8` | `[sp+4]` | `[sp+16]` | `?` | `?` | `?` | `?` |
| `call` at 0x10efa4 | `_bufWrite 0x10fb78` | `[sp+4]` | `[[sp+8]+4]` | `[sp+12]` | `?` | `?` | `?` |
| `call` at 0x10efb9 | `_bufLeftAlign 0x10f838` | `[sp+8]` | `([[sp+8]+4] + ret(_bufWrite))` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10eec1 | `([[sp+16]+16] & [[sp+16]+16]) == 0x0` | `([[sp+16]+16] & [[sp+16]+16]) != 0x0` |
| `jle` at 0x10eed9 | `le flags condition (operands unknown)` | `not (le flags condition (operands unknown))` |
| `jne` at 0x10eefb | `(ret(_strncmp) & ret(_strncmp)) != 0x0` | `(ret(_strncmp) & ret(_strncmp)) == 0x0` |
| `jge` at 0x10ef0a | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `je` at 0x10ef26 | `(ret(_strstr) & ret(_strstr)) == 0x0` | `(ret(_strstr) & ret(_strstr)) != 0x0` |
| `jae` at 0x10ef30 | `ret(_strstr) >= ([[sp+8]+4] + [sp+12]) (unsigned, 32-bit)` | `ret(_strstr) < ([[sp+8]+4] + [sp+12]) (unsigned, 32-bit)` |
| `jge` at 0x10ef4c | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jl` at 0x10ef66 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |
| `jge` at 0x10ef82 | `ge flags condition (operands unknown)` | `not (ge flags condition (operands unknown))` |
| `jne` at 0x10ef96 | `(ret(_bufAppendMsgs) & ret(_bufAppendMsgs)) != 0x0` | `(ret(_bufAppendMsgs) & ret(_bufAppendMsgs)) == 0x0` |
| `jl` at 0x10efb0 | `l flags condition (operands unknown)` | `not (l flags condition (operands unknown))` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10efd6 | `eax` | `?` |

## `_bufAttachLog`

- `_bufAttachLog`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f418 to 0x10f438.
  Established. The branch falls through when `([sp+8] & [sp+8]) != 0x0` (`je` at 0x10f423), reaching
  the four-byte store of `[sp+8]` through `[sp+4] +0x30` (`mov` at 0x10f425). The exit retains
  `[sp+8]` in `eax` (`ret` at 0x10f42b). The target offset does not establish a log-pointer role.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f41b | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f41e | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x10f425 | store | 4 | `[sp+4] +0x30` | `[sp+8]` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x10f423 | `([sp+8] & [sp+8]) == 0x0` | `([sp+8] & [sp+8]) != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f42b | `eax` | `[sp+8]` |

## `_bufDetachLog`

- `_bufDetachLog`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10f4e8 to 0x10f558.
  Established. The four-byte load reads `sp +0x4` (`mov` at 0x10f4eb). The store through `[sp+4]
  +0x30` writes four-byte `0x0` (`mov` at 0x10f4ee). The exit reports `[sp+4]` (`ret` at 0x10f4f8).
  Post-return data decodes do not establish additional effects, and a log-detachment role remains
  unknown.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10f4eb | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x10f4ee | store | 4 | `[sp+4] +0x30` | `0x0` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10f4f8 | `eax` | `[sp+4]` |

## `_sysModel`

- `_sysModel`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7a8 to 0x10b7c8.
  Established. The assignment places literal `0x10b79f` in `eax` (`mov` at 0x10b7ab), which the exit
  reports (`ret` at 0x10b7b3). There are no reached calls or explicit memory accesses. The pointed-to content
  and model-name meaning are not established by this body.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7b3 | `eax` | `0x10b79f` |

## `_sysBspRev`

- `_sysBspRev`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b7c8 to 0x10b7d8.
  Established. The immediate assignment writes `0x10b7b4` to `eax` (`mov` at 0x10b7cb). The exit
  retains that address literal (`ret` at 0x10b7d3). The routine does not read the addressed bytes,
  so no revision string is inferred here.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|

| Branch instruction | Taken when | Falls through when |
|---|---|---|

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b7d3 | `eax` | `0x10b7b4` |

## `_cmdGRAPHICS_COMMAND`

- `_cmdGRAPHICS_COMMAND`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x117740 to 0x117cf0.
  Established. The `_msgWrite` site has leading candidates `[[0x1da974]+140]` and `[0x1da190]`
  (`call` at 0x1179ea). A four-byte store through `[[0x1da974]+140] +0x1c` writes `0x1` (`mov` at
  0x1179ac), while another through `[[0x1da974]+144] +0x1c` writes `0x6` (`mov` at 0x117a8a). The
  `_bcopy` site presents `ret(_strtol)`, `[sp-12]`, and unknown (`call` at 0x117ca1). A byte store
  at an unknown effective address retains an unknown value (`mov` at 0x117cb8). Graphics-command or
  protocol semantics remain unassigned.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x117749 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x11774c | store | 4 | `sp -0xc` | `0x0` |
| `mov` at 0x117753 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x117759 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x117761 | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x117767 | load | 4 | `abs 0x1eb6b4` | `-` |
| `mov` at 0x117782 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117787 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11778e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1177a0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1177cb | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x1177d0 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1177d7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117801 | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x117804 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117821 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117826 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11782d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117860 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117865 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11786c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117880 | load | 1 | `ret(_strtol) +0x0` | `-` |
| `mov` at 0x117898 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11789d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1178a4 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x1178c0 | load | 4 | `sp +0x4` | `-` |
| `mov` at 0x1178e9 | store | 4 | `sp -0xc` | `ret(_strtol)` |
| `mov` at 0x1178ec | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x1178ef | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117908 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11790d | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117914 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117930 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117953 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117958 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x11795f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117970 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117975 | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x11798a | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x11798f | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x117995 | load | 4 | `[[0x1da974]+140] +0x8` | `-` |
| `mov` at 0x1179a1 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x1179a6 | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x1179ac | store | 4 | `[[0x1da974]+140] +0x1c` | `0x1` |
| `test` at 0x1179b3 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x1179bc | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x1179c1 | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x1179c7 | load | 4 | `[[0x1da974]+140] +0x8` | `-` |
| `mov` at 0x1179d8 | load | 4 | `abs 0x1da190` | `-` |
| `mov` at 0x1179de | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x1179e3 | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x1179fb | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117a00 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a07 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a20 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117a26 | load | 4 | `[0x1da974] +0x8c` | `-` |
| `mov` at 0x117a2c | load | 4 | `[[0x1da974]+140] +0x1c` | `-` |
| `mov` at 0x117a34 | load | 4 | `[[0x1da974]+140] +0x1c` | `-` |
| `mov` at 0x117a4a | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117a4f | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a56 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117a70 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117a7f | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117a84 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117a8a | store | 4 | `[[0x1da974]+144] +0x1c` | `0x6` |
| `mov` at 0x117a91 | load | 4 | `abs 0x1da18c` | `-` |
| `mov` at 0x117a97 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117a9c | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117aaf | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117ab4 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117aba | load | 4 | `[[0x1da974]+144] +0x1c` | `-` |
| `mov` at 0x117ac2 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117ac7 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117acd | load | 4 | `[[0x1da974]+144] +0x1c` | `-` |
| `mov` at 0x117ade | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117ae3 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117ae9 | load | 4 | `[[0x1da974]+144] +0x1c` | `-` |
| `mov` at 0x117b05 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117b0a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b11 | load | 4 | `(unknown)` | `-` |
| `test` at 0x117b20 | load | 1 | `abs 0x1da698` | `-` |
| `mov` at 0x117b29 | load | 4 | `[[0x1da974]+144] +0x8` | `-` |
| `mov` at 0x117b41 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117b46 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117b4c | load | 4 | `[[0x1da974]+144] +0x8` | `-` |
| `mov` at 0x117b5c | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117b61 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117b67 | load | 4 | `[[0x1da974]+144] +0x8` | `-` |
| `mov` at 0x117b6b | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117b70 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b77 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117b90 | store | 4 | `sp -0x8` | `0x0` |
| `mov` at 0x117ba0 | load | 4 | `abs 0x1da974` | `-` |
| `mov` at 0x117ba5 | load | 4 | `[0x1da974] +0x90` | `-` |
| `mov` at 0x117bab | load | 4 | `[[0x1da974]+144] +0x8` | `-` |
| `mov` at 0x117bd2 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117bd7 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117bde | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117bfe | load | 4 | `sp -0x8` | `-` |
| `mov` at 0x117c01 | load | 1 | `[sp-8] +0x0` | `-` |
| `mov` at 0x117c1e | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117c23 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c2a | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c60 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117c65 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117c6c | load | 4 | `(unknown)` | `-` |
| `cmp` at 0x117c80 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117c86 | load | 1 | `ret(_strtol) +0x2` | `-` |
| `mov` at 0x117c8c | load | 1 | `ret(_strtol) +0x3` | `-` |
| `mov` at 0x117c9c | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117ca9 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117cb5 | load | 4 | `sp -0xc` | `-` |
| `mov` at 0x117cb8 | store | 1 | `(unknown)` | `?` |
| `mov` at 0x117cbb | load | 1 | `ret(_strtol) +0x1` | `-` |
| `mov` at 0x117cc9 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x117cce | load | 4 | `(unknown)` | `-` |
| `mov` at 0x117cd5 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x117775 | `_ffscMsg 0x11ebd0` | `0x11748c` | `[sp+8]` | `ebx` | `esi` | `edi` | `0x0` |
| `call` at 0x117792 | `_sprintf 0x171680` | `?` | `0x1174af` | `?` | `?` | `?` | `?` |
| `call` at 0x1177ab | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `ebx` | `esi` | `edi` |
| `call` at 0x1177be | `_ffscMsg 0x11ebd0` | `0x1174bc` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1177db | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x1177f7 | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x117814 | `_ffscMsg 0x11ebd0` | `0x1174ed` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117831 | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x117841 | `_ffscTestAddress 0x1259cc` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117853 | `_ffscMsg 0x11ebd0` | `0x117513` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117870 | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x11788b | `_ffscMsg 0x11ebd0` | `0x11753c` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1178a8 | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x1178cb | `_strtok_r 0x17e358` | `0x0` | `0x116c80` | `[sp+4]` | `?` | `?` | `?` |
| `call` at 0x1178e1 | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x1178fb | `_ffscMsg 0x11ebd0` | `0x117568` | `ret(_strtok_r)` | `?` | `?` | `?` | `ret(_strtol)` |
| `call` at 0x117918 | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x117934 | `_ffscTestAddress 0x1259cc` | `ret(_strtol)` | `?` | `?` | `?` | `ret(_strtol)` | `?` |
| `call` at 0x117946 | `_ffscMsg 0x11ebd0` | `0x117590` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117963 | `_sprintf 0x171680` | `?` | `0x116da9` | `?` | `?` | `?` | `?` |
| `call` at 0x11797c | `_msgClear 0x125ea0` | `[[0x1da974]+140]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117999 | `_sprintf 0x171680` | `[[[0x1da974]+140]+8]` | `0x1175bb` | `ret(_strtol)` | `?` | `?` | `?` |
| `call` at 0x1179d0 | `_ffscMsg 0x11ebd0` | `0x1175c0` | `[[[0x1da974]+140]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x1179ea | `_msgWrite 0x126680` | `[[0x1da974]+140]` | `[0x1da190]` | `?` | `?` | `?` | `?` |
| `call` at 0x117a0b | `_sprintf 0x171680` | `?` | `0x1175e3` | `?` | `?` | `?` | `?` |
| `call` at 0x117a3d | `_ffscMsg 0x11ebd0` | `0x1175ee` | `[[[0x1da974]+140]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x117a5a | `_sprintf 0x171680` | `?` | `0x1175e3` | `?` | `?` | `?` | `?` |
| `call` at 0x117a77 | `_msgClear 0x125ea0` | `[[0x1da974]+144]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117aa3 | `_msgRead 0x126160` | `[[0x1da974]+144]` | `[0x1da18c]` | `?` | `?` | `?` | `?` |
| `call` at 0x117ad6 | `_ffscMsg 0x11ebd0` | `0x117620` | `[[[0x1da974]+144]+28]` | `?` | `?` | `?` | `?` |
| `call` at 0x117b15 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117b32 | `_ffscMsg 0x11ebd0` | `0x117670` | `[[[0x1da974]+144]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117b50 | `_strncmp 0x1749b8` | `[[[0x1da974]+144]+8]` | `0x11769b` | `0x2` | `?` | `?` | `?` |
| `call` at 0x117b7b | `_strcpy 0x17dfdc` | `?` | `[[[0x1da974]+144]+8]` | `?` | `?` | `?` | `?` |
| `call` at 0x117bb2 | `_strtok_r 0x17e358` | `[[[0x1da974]+144]+8]+2` | `0x116c80` | `sp-8` | `?` | `?` | `?` |
| `call` at 0x117bc5 | `_ffscMsg 0x11ebd0` | `0x11769e` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117be2 | `_sprintf 0x171680` | `?` | `0x1176c4` | `?` | `?` | `?` | `?` |
| `call` at 0x117bf4 | `_strtol 0x17d9f8` | `ret(_strtok_r)` | `sp-8` | `0x0` | `?` | `?` | `?` |
| `call` at 0x117c11 | `_ffscMsg 0x11ebd0` | `0x1176d2` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117c2e | `_sprintf 0x171680` | `?` | `0x1176c4` | `?` | `?` | `?` | `?` |
| `call` at 0x117c41 | `_ffscTestAddress 0x1259cc` | `ret(_strtol)` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117c53 | `_ffscMsg 0x11ebd0` | `0x117701` | `ret(_strtok_r)` | `?` | `?` | `?` | `?` |
| `call` at 0x117c70 | `_sprintf 0x171680` | `?` | `0x1176c4` | `?` | `?` | `?` | `?` |
| `call` at 0x117ca1 | `_bcopy 0x1747fc` | `ret(_strtol)` | `[sp-12]` | `?` | `?` | `?` | `?` |
| `call` at 0x117cad | `_oobChecksumMsg 0x1270b4` | `[sp-12]` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x117cd9 | `_sprintf 0x171680` | `?` | `0x117733` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11775f | `[0x1d9b9c] == [[sp+8]+148]` | `[0x1d9b9c] != [[sp+8]+148]` |
| `je` at 0x11776d | `[0x1eb6b4] == [[sp+8]+148]` | `[0x1eb6b4] != [[sp+8]+148]` |
| `jne` at 0x1177b7 | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117808 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jne` at 0x11780c | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x11784b | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x117884 | `([ret(_strtol)]:1 & 0xff) == 0xa0` | `([ret(_strtol)]:1 & 0xff) != 0xa0` |
| `je` at 0x1178d7 | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` |
| `je` at 0x1178f3 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` |
| `je` at 0x11793e | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x1179ba | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x1179f4 | `(ret(_msgWrite) & ret(_msgWrite)) == 0x0` | `(ret(_msgWrite) & ret(_msgWrite)) != 0x0` |
| `je` at 0x117a32 | `[[[0x1da974]+140]+28] == 0xb` | `[[[0x1da974]+140]+28] != 0xb` |
| `jne` at 0x117aad | `(ret(_msgRead) & ret(_msgRead)) != 0x0` | `(ret(_msgRead) & ret(_msgRead)) == 0x0` |
| `je` at 0x117ac0 | `[[[0x1da974]+144]+28] == 0xb` | `[[[0x1da974]+144]+28] != 0xb` |
| `jne` at 0x117aef | `[[[0x1da974]+144]+28] != 0xa` | `[[[0x1da974]+144]+28] == 0xa` |
| `je` at 0x117b27 | `(([0x1da698]:1 & 0xff) & 0x20) == 0x0` | `(([0x1da698]:1 & 0xff) & 0x20) != 0x0` |
| `je` at 0x117b5a | `(ret(_strncmp) & ret(_strncmp)) == 0x0` | `(ret(_strncmp) & ret(_strncmp)) != 0x0` |
| `jne` at 0x117bbe | `(ret(_strtok_r) & ret(_strtok_r)) != 0x0` | `(ret(_strtok_r) & ret(_strtok_r)) == 0x0` |
| `jne` at 0x117c05 | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) != 0x0` | `(([[sp-8]]:1 & 0xff) & ([[sp-8]]:1 & 0xff)) == 0x0` |
| `jne` at 0x117c09 | `(ret(_strtol) & ret(_strtol)) != 0x0` | `(ret(_strtol) & ret(_strtol)) == 0x0` |
| `je` at 0x117c4b | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) == 0x0` | `(ret(_ffscTestAddress) & ret(_ffscTestAddress)) != 0x0` |
| `je` at 0x117c84 | `[sp-12] == 0x0` | `[sp-12] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x117ce9 | `eax` | `0x0` |

## `_cmdSTEAL`

- `_cmdSTEAL`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x116930 to 0x116b10.
  Established. The signed branch is taken when `[[sp+8]+32] > 0x1f3 (signed, 32-bit)` (`jg` at
  0x116988). The `_userChangeInputMode` site has candidate zero `[[0x1d9b9c]+132]` with unknown
  candidates one and two (`call` at 0x116a38); its result branch is taken when
  `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` (`je` at 0x116a42). Both exits
  report `0x0` (`ret` at 0x116973, `ret` at 0x116aa1), without establishing console ownership
  changes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x116933 | load | 4 | `sp +0x8` | `-` |
| `mov` at 0x116936 | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x11693e | load | 4 | `[sp+8] +0x10` | `-` |
| `mov` at 0x116959 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x11695e | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116965 | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116980 | load | 4 | `[sp+8] +0x20` | `-` |
| `cmp` at 0x1169a0 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x1169ad | load | 4 | `[sp+8] +0x94` | `-` |
| `cmp` at 0x1169b3 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x1169d0 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x1169d6 | load | 4 | `[0x1d9b9c] +0x84` | `-` |
| `mov` at 0x116a00 | load | 4 | `[0x1d9b9c] +0x18` | `-` |
| `mov` at 0x116a2c | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x116a31 | load | 4 | `[0x1d9b9c] +0x84` | `-` |
| `cmp` at 0x116a50 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x116a59 | load | 4 | `abs 0x1d9b9c` | `-` |
| `mov` at 0x116a87 | load | 4 | `abs 0x1ee564` | `-` |
| `mov` at 0x116a8c | load | 4 | `(unknown)` | `-` |
| `mov` at 0x116a93 | load | 4 | `(unknown)` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x11694c | `_ffscMsg 0x11ebd0` | `0x116170` | `[sp+8]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x116969 | `_sprintf 0x171680` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x1169c1 | `_ffscMsg 0x11ebd0` | `0x1168b0` | `[sp+8]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x1169e6 | `_ffscMsg 0x11ebd0` | `0x1168dd` | `[0x1d9b9c]` | `ebp` | `?` | `?` | `?` |
| `call` at 0x116a38 | `_userChangeInputMode 0x12aa48` | `[[0x1d9b9c]+132]` | `?` | `?` | `ebp` | `?` | `?` |
| `call` at 0x116a64 | `_ffscMsg 0x11ebd0` | `0x116902` | `[0x1d9b9c]` | `?` | `?` | `?` | `?` |
| `call` at 0x116a97 | `_sprintf 0x171680` | `?` | `0x11598f` | `0x1` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `je` at 0x11693c | `[[sp+8]+16] == 0x1` | `[[sp+8]+16] != 0x1` |
| `jne` at 0x116944 | `[[sp+8]+16] != 0x3` | `[[sp+8]+16] == 0x3` |
| `jg` at 0x116988 | `[[sp+8]+32] > 0x1f3 (signed, 32-bit)` | `[[sp+8]+32] <= 0x1f3 (signed, 32-bit)` |
| `je` at 0x1169a7 | `[0x1d9b9c] == 0x0` | `[0x1d9b9c] != 0x0` |
| `jne` at 0x1169b9 | `[0x1d9b9c] != [[sp+8]+148]` | `[0x1d9b9c] == [[sp+8]+148]` |
| `jne` at 0x1169de | `([[0x1d9b9c]+132] & [[0x1d9b9c]+132]) != 0x0` | `([[0x1d9b9c]+132] & [[0x1d9b9c]+132]) == 0x0` |
| `jne` at 0x116a06 | `[[0x1d9b9c]+24] != 0x9` | `[[0x1d9b9c]+24] == 0x9` |
| `je` at 0x116a42 | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) == 0x0` | `(ret(_userChangeInputMode) & ret(_userChangeInputMode)) != 0x0` |
| `je` at 0x116a57 | `[0x1d9b9c] == 0x0` | `[0x1d9b9c] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x116973 | `eax` | `0x0` |
| `ret` at 0x116aa1 | `eax` | `0x0` |

## `_kbdExt`

- `_kbdExt`, `6.4-origin/usr/cpu/firmware/mmscfw.bin`:
  Analysed span 0x10b168 to 0x10b228.
  Established. The byte load at an unresolved effective address has an unknown value (`mov` at
  0x10b1ac), which remains unknown in the byte store at `sp -0x8` (`mov` at 0x10b1af). The first
  `_tyIRd` site has unknown candidate zero and candidate one `0x1b` (`call` at 0x10b1c8). The later
  sites retain unknown candidates (`call` at 0x10b1eb, `call` at 0x10b20c). No extended-key encoding
  is inferred from the raw pushes.

| Instruction | Access | Width in bytes | Effective target | Stored value |
|---|---|---:|---|---|
| `mov` at 0x10b170 | load | 1 | `sp +0x4` | `-` |
| `mov` at 0x10b173 | load | 4 | `abs 0x1d2744` | `-` |
| `test` at 0x10b179 | load | 1 | `abs 0x1d2741` | `-` |
| `mov` at 0x10b186 | load | 4 | `abs 0x1d273c` | `-` |
| `movzx` at 0x10b192 | load | 2 | `abs 0x1d2742` | `-` |
| `mov` at 0x10b1ac | load | 1 | `(unknown)` | `-` |
| `mov` at 0x10b1af | store | 1 | `sp -0x8` | `?` |
| `cmp` at 0x10b1d0 | load | 4 | `abs 0x1d2738` | `-` |
| `movsx` at 0x10b1f3 | load | 1 | `sp -0x8` | `-` |

| Call instruction | Callee and target | Candidate 0 | Candidate 1 | Candidate 2 | Candidate 3 | Candidate 4 | Candidate 5 |
|---|---|---|---|---|---|---|---|
| `call` at 0x10b1c8 | `_tyIRd 0x16e970` | `?` | `0x1b` | `ebx` | `esi` | `?` | `ebp` |
| `call` at 0x10b1eb | `_tyIRd 0x16e970` | `?` | `?` | `?` | `?` | `?` | `?` |
| `call` at 0x10b20c | `_tyIRd 0x16e970` | `?` | `?` | `?` | `?` | `?` | `?` |

| Branch instruction | Taken when | Falls through when |
|---|---|---|
| `jne` at 0x10b180 | `(([0x1d2741]:1 & 0xff) & 0x2) != 0x0` | `(([0x1d2741]:1 & 0xff) & 0x2) == 0x0` |
| `je` at 0x10b1d7 | `[0x1d2738] == 0x0` | `[0x1d2738] != 0x0` |

| Return instruction | Register | Value |
|---|---|---|
| `ret` at 0x10b219 | `eax` | `?` |
